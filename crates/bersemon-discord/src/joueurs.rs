//! Parties des joueurs Discord, une par compte (Q60), gardées en mémoire et
//! chargées depuis leur sauvegarde à la première demande du joueur. Après un
//! redémarrage du bot, chacun reprend donc à sa dernière sauvegarde (section
//! 13.9).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};

use bersemon_core::aleatoire::{Aleatoire, GenerateurAleatoire};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{Attente, Choix, Contexte, Deroulement, ErreurChoix, Evenement};
use bersemon_core::echange::{ErreurEchange, Offre};
use bersemon_core::horodatage::Horodatage;
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::SauvegardeRepository;
use bersemon_presentation::echanges::{offre_acceptee, offre_recue, offre_refusee};
use bersemon_storage::FichierJson;

use crate::composants::Vue;
use crate::echanges::{
    ActionDEchange, AutreJoueur, Bourse, ErreurDOffre, MessagePrive, OffreEnAttente, Pseudos,
    TableauDesEchanges,
};
use crate::message::OPTIONS_PAR_MENU;
use crate::messages_prives::{Annonces, Retour, parties};

/// Identifiant du compte Discord.
pub type IdJoueur = u64;

pub struct Session {
    pub deroulement: Deroulement,
    /// Change à chaque choix accepté : les boutons d'un message qui n'est
    /// plus à jour portent une version dépassée.
    pub version: u32,
    /// Ce que montre le camp ; sans effet ailleurs.
    pub vue: Vue,
    /// Préparé à chaque lecture de la session quand la vue est celle des
    /// échanges (section 15.1).
    pub echanges: TableauDesEchanges,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Refus {
    MessagePerime,
    /// Le choix est gardé pour l'expliquer, par exemple un nom ou un surnom
    /// refusé.
    Choix {
        choix: Choix,
        erreur: ErreurChoix,
    },
    Echange(ErreurDOffre),
}

pub struct Joueurs {
    /// Une sauvegarde par compte : `<identifiant>.json`.
    dossier: PathBuf,
    sessions: HashMap<IdJoueur, Session>,
    /// Partagé par tous les joueurs, sous le même verrou que leurs parties.
    aleatoire: GenerateurAleatoire,
    /// Retours d'expéditions automatiques à annoncer par message privé.
    annonces: Annonces,
    bourse: Bourse,
    pseudos: Pseudos,
    /// Offres reçues, acceptées ou refusées, à annoncer par message privé.
    messages_prives: Vec<MessagePrive>,
}

/// Un incident pendant le traitement d'un choix ne doit pas bloquer tous les
/// joueurs : les parties restent utilisables.
pub fn verrouiller(joueurs: &Mutex<Joueurs>) -> MutexGuard<'_, Joueurs> {
    joueurs.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Joueurs {
    pub fn new(dossier: PathBuf, graine: u64) -> Self {
        Self {
            annonces: Annonces::new(&dossier),
            pseudos: Pseudos::charger(&dossier),
            dossier,
            sessions: HashMap::new(),
            aleatoire: GenerateurAleatoire::new(graine),
            bourse: Bourse::default(),
            messages_prives: Vec::new(),
        }
    }

    /// Au démarrage du bot : les parties ne sont chargées qu'à la demande
    /// de leur joueur, mais leurs expéditions automatiques doivent être
    /// annoncées dès leur retour, même pendant que le bot était éteint.
    pub fn preparer_les_annonces(&mut self, contenu: &ContenuV1) {
        self.annonces.charger(&self.dossier, contenu);
    }

    pub fn retours_echus(&self, maintenant: Horodatage) -> Vec<Retour> {
        self.annonces.echus(maintenant)
    }

    pub fn noter_annonce(&mut self, retour: &Retour) {
        self.annonces.noter_annonce(retour);
    }

    /// La version de départ est tirée au hasard, pour qu'un bouton d'avant un
    /// redémarrage du bot ne tombe pas, par chance, sur la même.
    fn charger(&mut self, joueur: IdJoueur, contenu: &ContenuV1) -> &mut Session {
        let dossier = &self.dossier;
        let aleatoire = &mut self.aleatoire;
        self.sessions.entry(joueur).or_insert_with(|| Session {
            deroulement: Deroulement::demarrer(depot(dossier, joueur).charger(contenu)),
            version: aleatoire.tirer(0..=u32::MAX),
            vue: Vue::Camp,
            echanges: TableauDesEchanges::default(),
        })
    }

    pub fn session(&mut self, joueur: IdJoueur, contenu: &ContenuV1) -> &Session {
        let vue = self.charger(joueur, contenu).vue;
        if let Vue::Echanges { avec, .. } = vue {
            let tableau = self.tableau_des_echanges(joueur, avec, contenu);
            self.charger(joueur, contenu).echanges = tableau;
        }
        self.charger(joueur, contenu)
    }

    /// Noté à chaque interaction du joueur.
    pub fn noter_pseudo(&mut self, joueur: IdJoueur, pseudo: &str) {
        self.pseudos.noter(joueur, pseudo);
    }

    pub fn prendre_les_messages_prives(&mut self) -> Vec<MessagePrive> {
        std::mem::take(&mut self.messages_prives)
    }

    /// Le joueur tel que les autres le voient, s'il a une partie.
    fn autre_joueur(&mut self, joueur: IdJoueur, contenu: &ContenuV1) -> Option<AutreJoueur> {
        let heros = partie_lisible(&self.charger(joueur, contenu).deroulement)?
            .heros()
            .nom()
            .as_str()
            .to_string();
        Some(AutreJoueur {
            id: joueur,
            heros,
            pseudo: self.pseudos.pseudo(joueur).map(str::to_string),
        })
    }

    /// Les parties de tous les joueurs du dossier sont chargées pour en
    /// connaître les héros : le bot sert un cercle d'amis, pas une foule.
    fn tableau_des_echanges(
        &mut self,
        joueur: IdJoueur,
        avec: Option<IdJoueur>,
        contenu: &ContenuV1,
    ) -> TableauDesEchanges {
        let mut joueurs: Vec<AutreJoueur> = parties(&self.dossier)
            .into_iter()
            .filter(|(autre, _)| *autre != joueur)
            .filter_map(|(autre, _)| self.autre_joueur(autre, contenu))
            .collect();
        joueurs.sort_by(|a, b| a.heros.cmp(&b.heros).then(a.id.cmp(&b.id)));
        joueurs.truncate(OPTIONS_PAR_MENU);
        let recue = self.bourse.recue(joueur).cloned();
        let recue =
            recue.and_then(|offre| Some((self.autre_joueur(offre.de, contenu)?, offre.offre)));
        let envoyee = self.bourse.envoyee(joueur).cloned();
        let envoyee =
            envoyee.and_then(|offre| Some((self.autre_joueur(offre.a, contenu)?, offre.offre)));
        let choisi = avec.and_then(|autre| {
            let qui = self.autre_joueur(autre, contenu)?;
            let partie = partie_lisible(&self.sessions.get(&autre)?.deroulement)?;
            Some((qui, partie.collection().clone()))
        });
        TableauDesEchanges {
            joueurs,
            recue,
            envoyee,
            choisi,
        }
    }

    /// Fait, retire, accepte ou refuse une offre, si le message affichait la
    /// version en cours de la partie. Le joueur reste sur l'écran des
    /// échanges, sans joueur choisi.
    pub fn echanger(
        &mut self,
        joueur: IdJoueur,
        version: u32,
        action: ActionDEchange,
        contenu: &ContenuV1,
    ) -> Result<Vec<Evenement>, Refus> {
        if !self.est_a_jour(joueur, version, contenu) {
            return Err(Refus::MessagePerime);
        }
        let resultat = match action {
            ActionDEchange::Offrir { a, cede, demande } => self
                .offrir(joueur, a, cede, demande, contenu)
                .map(|()| Vec::new()),
            ActionDEchange::Retirer => self
                .bourse
                .retirer_l_envoyee(joueur)
                .map(|_| Vec::new())
                .ok_or(ErreurDOffre::AucuneOffre),
            ActionDEchange::Refuser => self.refuser(joueur, contenu).map(|()| Vec::new()),
            ActionDEchange::Accepter => self.accepter(joueur, contenu),
        };
        let evenements = resultat.map_err(Refus::Echange)?;
        let session = self.charger(joueur, contenu);
        session.version = session.version.wrapping_add(1);
        session.vue = Vue::Echanges {
            avec: None,
            cede: None,
        };
        Ok(evenements)
    }

    fn offrir(
        &mut self,
        joueur: IdJoueur,
        a: IdJoueur,
        cede: bersemon_core::collection::Emplacement,
        demande: Option<bersemon_core::collection::Emplacement>,
        contenu: &ContenuV1,
    ) -> Result<(), ErreurDOffre> {
        if a == joueur {
            return Err(ErreurDOffre::AvecSoiMeme);
        }
        if self.bourse.envoyee(joueur).is_some() {
            return Err(ErreurDOffre::OffreDejaEnvoyee);
        }
        let destinataire = self
            .autre_joueur(a, contenu)
            .ok_or(ErreurDOffre::JoueurSansPartie)?;
        let expediteur = self
            .autre_joueur(joueur, contenu)
            .ok_or(ErreurDOffre::JoueurSansPartie)?;
        let donneur = self
            .sessions
            .get(&joueur)
            .and_then(|session| session.deroulement.partie_au_camp())
            .ok_or(ErreurDOffre::Echange(ErreurEchange::DonneurPasAuCamp))?;
        let receveur = self
            .sessions
            .get(&a)
            .and_then(|session| partie_lisible(&session.deroulement))
            .ok_or(ErreurDOffre::JoueurSansPartie)?;
        let offre = match demande {
            Some(demande) => Offre::echange(donneur, cede, receveur, demande),
            None => Offre::don(donneur, cede, receveur),
        }
        .map_err(ErreurDOffre::Echange)?;
        let texte = offre_recue(&expediteur.heros, expediteur.pseudo.as_deref(), &offre);
        self.bourse.deposer(OffreEnAttente {
            de: joueur,
            a: destinataire.id,
            offre,
        })?;
        self.messages_prives.push(MessagePrive { joueur: a, texte });
        Ok(())
    }

    fn refuser(&mut self, joueur: IdJoueur, contenu: &ContenuV1) -> Result<(), ErreurDOffre> {
        let offre = self
            .bourse
            .retirer_la_recue(joueur)
            .ok_or(ErreurDOffre::AucuneOffre)?;
        if let Some(moi) = self.autre_joueur(joueur, contenu) {
            self.messages_prives.push(MessagePrive {
                joueur: offre.de,
                texte: offre_refusee(&moi.heros),
            });
        }
        Ok(())
    }

    /// L'offre reste en attente si celui qui l'a faite n'est pas au camp ;
    /// elle est retirée si l'un des monstres a changé.
    fn accepter(
        &mut self,
        joueur: IdJoueur,
        contenu: &ContenuV1,
    ) -> Result<Vec<Evenement>, ErreurDOffre> {
        let offre = self
            .bourse
            .recue(joueur)
            .cloned()
            .ok_or(ErreurDOffre::AucuneOffre)?;
        self.charger(offre.de, contenu);
        let mut donneur = self
            .sessions
            .remove(&offre.de)
            .expect("session chargée juste avant");
        let resultat = match self.sessions.get_mut(&joueur) {
            Some(receveur) => Deroulement::conclure_offre(
                &mut donneur.deroulement,
                &mut receveur.deroulement,
                &offre.offre,
                &depot(&self.dossier, offre.de),
                &depot(&self.dossier, joueur),
            ),
            None => Err(ErreurEchange::ReceveurPasAuCamp),
        };
        if resultat.is_ok() {
            donneur.version = donneur.version.wrapping_add(1);
        }
        self.sessions.insert(offre.de, donneur);
        match resultat {
            Ok((_, du_receveur)) => {
                self.bourse.retirer_la_recue(joueur);
                if let Some(moi) = self.autre_joueur(joueur, contenu) {
                    self.messages_prives.push(MessagePrive {
                        joueur: offre.de,
                        texte: offre_acceptee(&moi.heros),
                    });
                }
                Ok(du_receveur)
            }
            Err(erreur) => {
                if erreur == ErreurEchange::OffrePerimee {
                    self.bourse.retirer_la_recue(joueur);
                }
                Err(ErreurDOffre::Echange(erreur))
            }
        }
    }

    /// Le composant affichait-il la version en cours de la partie ?
    pub fn est_a_jour(&mut self, joueur: IdJoueur, version: u32, contenu: &ContenuV1) -> bool {
        self.session(joueur, contenu).version == version
    }

    /// Applique le choix si le message affichait la version en cours de la
    /// partie. Un choix refusé ne change rien. L'heure vient de l'appelant,
    /// pour que les tests puissent la fixer.
    pub fn jouer(
        &mut self,
        joueur: IdJoueur,
        version: u32,
        choix: Choix,
        contenu: &ContenuV1,
        maintenant: Horodatage,
    ) -> Result<Vec<Evenement>, Refus> {
        if !self.est_a_jour(joueur, version, contenu) {
            return Err(Refus::MessagePerime);
        }
        let depot = depot(&self.dossier, joueur);
        let session = self
            .sessions
            .get_mut(&joueur)
            .expect("session chargée par est_a_jour");
        let mut contexte = Contexte {
            contenu,
            aleatoire: &mut self.aleatoire,
            depot: &depot,
            maintenant,
        };
        let evenements = session
            .deroulement
            .repondre(choix.clone(), &mut contexte)
            .map_err(|erreur| Refus::Choix {
                choix: choix.clone(),
                erreur,
            })?;
        session.version = session.version.wrapping_add(1);
        session.vue = vue_apres(session.vue, &choix);
        self.annonces.suivre(joueur, &evenements);
        Ok(evenements)
    }

    /// Change ce que montre le camp, sans toucher à la partie ni à sa
    /// version : les autres boutons du message restent valables.
    pub fn afficher(
        &mut self,
        joueur: IdJoueur,
        version: u32,
        vue: Vue,
        contenu: &ContenuV1,
    ) -> Result<(), Refus> {
        if !self.est_a_jour(joueur, version, contenu) {
            return Err(Refus::MessagePerime);
        }
        let session = self
            .sessions
            .get_mut(&joueur)
            .expect("session chargée par est_a_jour");
        session.vue = vue;
        Ok(())
    }
}

/// Le camp s'ouvre sur sa vue principale au retour d'expédition et au début
/// d'une partie, même après un suicide décidé depuis les options. La gestion
/// des monstres reste ouverte d'une action à l'autre, comme dans la console,
/// mais le monstre choisi est oublié : sa place a pu changer. Après le départ
/// ou l'accueil d'une expédition automatique, le joueur retrouve la liste des
/// expéditions.
fn vue_apres(vue: Vue, choix: &Choix) -> Vue {
    match (vue, choix) {
        (_, Choix::PartirEnExpedition(_)) | (_, Choix::Classe(_)) => Vue::Camp,
        (_, Choix::EnvoyerEnExpeditionAutomatique { .. })
        | (_, Choix::AccueillirExpeditionAutomatique(_)) => Vue::Expeditions,
        (Vue::Gestion(Some(_)), _) => Vue::Gestion(None),
        (vue, _) => vue,
    }
}

/// Une partie reprise après un redémarrage se lit avant que son joueur ne la
/// reprenne : ses héros et ses monstres sont ceux de sa sauvegarde.
fn partie_lisible(deroulement: &Deroulement) -> Option<&Partie> {
    match deroulement.attente() {
        Attente::Reprise(partie) => Some(partie),
        _ => deroulement.partie(),
    }
}

fn depot(dossier: &Path, joueur: IdJoueur) -> FichierJson {
    FichierJson::new(dossier.join(format!("{joueur}.json")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::collection::Emplacement;
    use bersemon_core::deroulement::{ActionDeGestion, Attente};
    use bersemon_core::heros::Classe;
    use bersemon_core::nom::ErreurNom;
    use bersemon_core::zone::ZoneId;
    use std::fs;
    use std::sync::atomic::{AtomicU32, Ordering};

    /// Un dossier par test, dans le dossier temporaire du système.
    fn dossier_de_test() -> PathBuf {
        static COMPTEUR: AtomicU32 = AtomicU32::new(0);
        let numero = COMPTEUR.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "bersemon-discord-test-{}-{numero}",
            std::process::id()
        ))
    }

    fn repondre(
        joueurs: &mut Joueurs,
        joueur: IdJoueur,
        choix: Choix,
        contenu: &ContenuV1,
    ) -> Result<Vec<Evenement>, Refus> {
        let version = joueurs.session(joueur, contenu).version;
        joueurs.jouer(
            joueur,
            version,
            choix,
            contenu,
            Horodatage::depuis_secondes(0),
        )
    }

    #[test]
    fn deux_joueurs_ont_chacun_leur_partie() {
        let contenu = ContenuV1::new();
        let dossier = dossier_de_test();
        let mut joueurs = Joueurs::new(dossier.clone(), 1);
        let nom = Choix::NomDuHeros("Aldric".to_string());
        repondre(&mut joueurs, 1, nom, &contenu).unwrap();
        assert_eq!(
            joueurs.session(1, &contenu).deroulement.attente(),
            Attente::Classe
        );
        assert_eq!(
            joueurs.session(2, &contenu).deroulement.attente(),
            Attente::NomDuHeros
        );
        let _ = fs::remove_dir_all(dossier);
    }

    #[test]
    fn un_bouton_d_une_version_depassee_est_refuse() {
        let contenu = ContenuV1::new();
        let dossier = dossier_de_test();
        let mut joueurs = Joueurs::new(dossier.clone(), 1);
        let version = joueurs.session(1, &contenu).version;
        let nom = Choix::NomDuHeros("Aldric".to_string());
        joueurs
            .jouer(
                1,
                version,
                nom.clone(),
                &contenu,
                Horodatage::depuis_secondes(0),
            )
            .unwrap();
        assert_eq!(
            joueurs.jouer(1, version, nom, &contenu, Horodatage::depuis_secondes(0)),
            Err(Refus::MessagePerime)
        );
        assert_eq!(
            joueurs.session(1, &contenu).deroulement.attente(),
            Attente::Classe
        );
        let _ = fs::remove_dir_all(dossier);
    }

    #[test]
    fn un_choix_refuse_garde_la_version() {
        let contenu = ContenuV1::new();
        let dossier = dossier_de_test();
        let mut joueurs = Joueurs::new(dossier.clone(), 1);
        let version = joueurs.session(1, &contenu).version;
        let vide = Choix::NomDuHeros("  ".to_string());
        assert_eq!(
            joueurs.jouer(
                1,
                version,
                vide.clone(),
                &contenu,
                Horodatage::depuis_secondes(0)
            ),
            Err(Refus::Choix {
                choix: vide,
                erreur: ErreurChoix::Nom(ErreurNom::Vide),
            })
        );
        assert_eq!(joueurs.session(1, &contenu).version, version);
        let _ = fs::remove_dir_all(dossier);
    }

    #[test]
    fn la_partie_est_reprise_depuis_la_sauvegarde_du_joueur() {
        let contenu = ContenuV1::new();
        let dossier = dossier_de_test();
        let mut joueurs = Joueurs::new(dossier.clone(), 1);
        for choix in [
            Choix::NomDuHeros("Aldric".to_string()),
            Choix::Classe(Classe::Mage),
        ] {
            repondre(&mut joueurs, 42, choix, &contenu).unwrap();
        }
        assert_eq!(
            joueurs.session(42, &contenu).deroulement.attente(),
            Attente::Camp
        );
        assert!(dossier.join("42.json").exists());

        // Comme après un redémarrage du bot.
        let mut apres = Joueurs::new(dossier.clone(), 2);
        assert!(matches!(
            apres.session(42, &contenu).deroulement.attente(),
            Attente::Reprise(partie) if partie.heros().nom().as_str() == "Aldric"
        ));
        let _ = fs::remove_dir_all(dossier);
    }

    /// Aldric, Mage, joue le compte 1 ; Ysolde, Templier, le compte 2.
    fn deux_joueurs(contenu: &ContenuV1) -> (Joueurs, PathBuf) {
        let dossier = dossier_de_test();
        let mut joueurs = Joueurs::new(dossier.clone(), 1);
        for (joueur, nom, classe) in [(1, "Aldric", Classe::Mage), (2, "Ysolde", Classe::Templier)]
        {
            repondre(
                &mut joueurs,
                joueur,
                Choix::NomDuHeros(nom.to_string()),
                contenu,
            )
            .unwrap();
            repondre(&mut joueurs, joueur, Choix::Classe(classe), contenu).unwrap();
        }
        (joueurs, dossier)
    }

    fn echanger(
        joueurs: &mut Joueurs,
        joueur: IdJoueur,
        action: ActionDEchange,
        contenu: &ContenuV1,
    ) -> Result<Vec<Evenement>, Refus> {
        let version = joueurs.session(joueur, contenu).version;
        joueurs.echanger(joueur, version, action, contenu)
    }

    fn ouvrir_les_echanges(joueurs: &mut Joueurs, joueur: IdJoueur, contenu: &ContenuV1) {
        let version = joueurs.session(joueur, contenu).version;
        let vue = Vue::Echanges {
            avec: None,
            cede: None,
        };
        joueurs.afficher(joueur, version, vue, contenu).unwrap();
    }

    fn premier_monstre(joueurs: &mut Joueurs, joueur: IdJoueur, contenu: &ContenuV1) -> String {
        let partie = joueurs
            .session(joueur, contenu)
            .deroulement
            .partie()
            .unwrap();
        partie.collection().equipe()[0]
            .espece()
            .id()
            .as_str()
            .to_string()
    }

    const ECHANGE_DES_PREMIERS: ActionDEchange = ActionDEchange::Offrir {
        a: 2,
        cede: Emplacement::Equipe(0),
        demande: Some(Emplacement::Equipe(0)),
    };

    #[test]
    fn un_echange_accepte_est_sauvegarde_et_annonce() {
        let contenu = ContenuV1::new();
        let (mut joueurs, dossier) = deux_joueurs(&contenu);
        joueurs.noter_pseudo(1, "aldric_du_nord");
        echanger(&mut joueurs, 1, ECHANGE_DES_PREMIERS, &contenu).unwrap();
        let messages = joueurs.prendre_les_messages_prives();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].joueur, 2);
        assert!(
            messages[0].texte.starts_with(
                "Aldric (aldric_du_nord) vous propose **Salamandre** contre votre **Mandragore**."
            ),
            "{}",
            messages[0].texte
        );

        // Ysolde voit l'offre sur son écran des échanges.
        ouvrir_les_echanges(&mut joueurs, 2, &contenu);
        let tableau = &joueurs.session(2, &contenu).echanges;
        assert_eq!(tableau.recue.as_ref().map(|(autre, _)| autre.id), Some(1));
        assert_eq!(tableau.joueurs.len(), 1);
        assert_eq!(tableau.joueurs[0].pseudo.as_deref(), Some("aldric_du_nord"));

        let version_d_aldric = joueurs.session(1, &contenu).version;
        let evenements = echanger(&mut joueurs, 2, ActionDEchange::Accepter, &contenu).unwrap();
        assert!(matches!(
            evenements.as_slice(),
            [Evenement::MonstreCede { .. }, Evenement::MonstreRecu { heros, .. }] if heros == "Aldric"
        ));
        assert_ne!(joueurs.session(1, &contenu).version, version_d_aldric);
        assert_eq!(premier_monstre(&mut joueurs, 1, &contenu), "mandragore");
        assert_eq!(premier_monstre(&mut joueurs, 2, &contenu), "salamandre");
        let messages = joueurs.prendre_les_messages_prives();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].joueur, 1);
        assert_eq!(messages[0].texte, "Ysolde a accepté votre offre.");
        assert_eq!(joueurs.session(2, &contenu).echanges.recue, None);

        // Les deux sauvegardes sont à jour.
        let mut apres = Joueurs::new(dossier.clone(), 2);
        repondre(&mut apres, 1, Choix::ReprendreLaPartie, &contenu).unwrap();
        let partie = apres.session(1, &contenu).deroulement.partie().unwrap();
        let mandragore = &partie.collection().equipe()[0];
        assert_eq!(
            mandragore.recu_de().map(|heros| heros.as_str()),
            Some("Ysolde")
        );
        let _ = fs::remove_dir_all(dossier);
    }

    #[test]
    fn une_offre_se_refuse_ou_se_retire() {
        let contenu = ContenuV1::new();
        let (mut joueurs, dossier) = deux_joueurs(&contenu);
        let soi_meme = ActionDEchange::Offrir {
            a: 1,
            cede: Emplacement::Equipe(0),
            demande: None,
        };
        assert_eq!(
            echanger(&mut joueurs, 1, soi_meme, &contenu),
            Err(Refus::Echange(ErreurDOffre::AvecSoiMeme))
        );
        // Un don laisserait Aldric sans monstre.
        let don = ActionDEchange::Offrir {
            a: 2,
            cede: Emplacement::Equipe(0),
            demande: None,
        };
        assert_eq!(
            echanger(&mut joueurs, 1, don, &contenu),
            Err(Refus::Echange(ErreurDOffre::Echange(
                ErreurEchange::DernierMonstre
            )))
        );
        echanger(&mut joueurs, 1, ECHANGE_DES_PREMIERS, &contenu).unwrap();
        assert_eq!(
            echanger(&mut joueurs, 1, ECHANGE_DES_PREMIERS, &contenu),
            Err(Refus::Echange(ErreurDOffre::OffreDejaEnvoyee))
        );
        echanger(&mut joueurs, 2, ActionDEchange::Refuser, &contenu).unwrap();
        let messages = joueurs.prendre_les_messages_prives();
        assert_eq!(
            messages.last().map(|message| message.texte.as_str()),
            Some("Ysolde a refusé votre offre.")
        );
        assert_eq!(
            echanger(&mut joueurs, 2, ActionDEchange::Accepter, &contenu),
            Err(Refus::Echange(ErreurDOffre::AucuneOffre))
        );
        echanger(&mut joueurs, 1, ECHANGE_DES_PREMIERS, &contenu).unwrap();
        echanger(&mut joueurs, 1, ActionDEchange::Retirer, &contenu).unwrap();
        assert_eq!(
            echanger(&mut joueurs, 2, ActionDEchange::Accepter, &contenu),
            Err(Refus::Echange(ErreurDOffre::AucuneOffre))
        );
        assert_eq!(premier_monstre(&mut joueurs, 1, &contenu), "salamandre");
        let _ = fs::remove_dir_all(dossier);
    }

    #[test]
    fn une_offre_attend_le_donneur_au_camp_et_se_retire_si_un_monstre_change() {
        let contenu = ContenuV1::new();
        let (mut joueurs, dossier) = deux_joueurs(&contenu);
        echanger(&mut joueurs, 1, ECHANGE_DES_PREMIERS, &contenu).unwrap();
        let partir = Choix::PartirEnExpedition(ZoneId::new("lisiere"));
        repondre(&mut joueurs, 1, partir, &contenu).unwrap();
        assert_eq!(
            echanger(&mut joueurs, 2, ActionDEchange::Accepter, &contenu),
            Err(Refus::Echange(ErreurDOffre::Echange(
                ErreurEchange::DonneurPasAuCamp
            )))
        );
        assert!(joueurs.bourse.recue(2).is_some());
        // Ysolde renomme sa Mandragore : l'offre n'est plus valable.
        joueurs.bourse.retirer_la_recue(2);
        let (mut joueurs_au_camp, autre_dossier) = deux_joueurs(&contenu);
        echanger(&mut joueurs_au_camp, 1, ECHANGE_DES_PREMIERS, &contenu).unwrap();
        let renommer = Choix::Gerer(ActionDeGestion::Renommer {
            emplacement: Emplacement::Equipe(0),
            surnom: "Racine".to_string(),
        });
        repondre(&mut joueurs_au_camp, 2, renommer, &contenu).unwrap();
        assert_eq!(
            echanger(&mut joueurs_au_camp, 2, ActionDEchange::Accepter, &contenu),
            Err(Refus::Echange(ErreurDOffre::Echange(
                ErreurEchange::OffrePerimee
            )))
        );
        assert!(joueurs_au_camp.bourse.recue(2).is_none());
        let _ = fs::remove_dir_all(dossier);
        let _ = fs::remove_dir_all(autre_dossier);
    }

    #[test]
    fn les_vues_du_camp_ne_changent_pas_la_version() {
        let contenu = ContenuV1::new();
        let dossier = dossier_de_test();
        let mut joueurs = Joueurs::new(dossier.clone(), 1);
        for choix in [
            Choix::NomDuHeros("Aldric".to_string()),
            Choix::Classe(Classe::Mage),
        ] {
            repondre(&mut joueurs, 1, choix, &contenu).unwrap();
        }
        let version = joueurs.session(1, &contenu).version;
        let choisi = Vue::Gestion(Some(Emplacement::Equipe(0)));
        joueurs.afficher(1, version, choisi, &contenu).unwrap();
        assert_eq!(joueurs.session(1, &contenu).vue, choisi);
        assert_eq!(joueurs.session(1, &contenu).version, version);
        assert_eq!(
            joueurs.afficher(1, version.wrapping_sub(1), Vue::Camp, &contenu),
            Err(Refus::MessagePerime)
        );

        // Après une action, la gestion reste ouverte sans monstre choisi.
        let renommer = Choix::Gerer(ActionDeGestion::Renommer {
            emplacement: Emplacement::Equipe(0),
            surnom: "Braise".to_string(),
        });
        repondre(&mut joueurs, 1, renommer, &contenu).unwrap();
        assert_eq!(joueurs.session(1, &contenu).vue, Vue::Gestion(None));

        let partir = Choix::PartirEnExpedition(ZoneId::new("lisiere"));
        repondre(&mut joueurs, 1, partir, &contenu).unwrap();
        assert_eq!(joueurs.session(1, &contenu).vue, Vue::Camp);
        let _ = fs::remove_dir_all(dossier);
    }

    #[test]
    fn le_suicide_efface_la_sauvegarde_et_la_partie_suivante_s_ouvre_au_camp() {
        let contenu = ContenuV1::new();
        let dossier = dossier_de_test();
        let mut joueurs = Joueurs::new(dossier.clone(), 1);
        for choix in [
            Choix::NomDuHeros("Aldric".to_string()),
            Choix::Classe(Classe::Mage),
        ] {
            repondre(&mut joueurs, 1, choix, &contenu).unwrap();
        }
        let sauvegarde = dossier.join("1.json");
        assert!(sauvegarde.exists());
        let version = joueurs.session(1, &contenu).version;
        joueurs
            .afficher(1, version, Vue::Options, &contenu)
            .unwrap();
        repondre(&mut joueurs, 1, Choix::SeSuicider, &contenu).unwrap();
        let evenements = repondre(&mut joueurs, 1, Choix::Oui, &contenu).unwrap();
        assert_eq!(evenements, vec![Evenement::Suicide]);
        assert!(!sauvegarde.exists());
        assert_eq!(
            joueurs.session(1, &contenu).deroulement.attente(),
            Attente::NomDuHeros
        );

        for choix in [
            Choix::NomDuHeros("Ysolde".to_string()),
            Choix::Classe(Classe::Templier),
        ] {
            repondre(&mut joueurs, 1, choix, &contenu).unwrap();
        }
        assert_eq!(joueurs.session(1, &contenu).vue, Vue::Camp);
        assert!(sauvegarde.exists());
        let _ = fs::remove_dir_all(dossier);
    }
}
