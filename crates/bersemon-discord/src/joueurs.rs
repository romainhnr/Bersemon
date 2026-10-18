//! Parties des joueurs Discord, une par compte (Q60), gardées en mémoire et
//! chargées depuis leur sauvegarde à la première demande du joueur. Après un
//! redémarrage du bot, chacun reprend donc à sa dernière sauvegarde (section
//! 13.9).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};

use bersemon_core::aleatoire::{Aleatoire, GenerateurAleatoire};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{Choix, Contexte, Deroulement, ErreurChoix, Evenement};
use bersemon_core::horodatage::Horodatage;
use bersemon_core::sauvegarde::SauvegardeRepository;
use bersemon_storage::FichierJson;

use crate::composants::Vue;

/// Identifiant du compte Discord.
pub type IdJoueur = u64;

pub struct Session {
    pub deroulement: Deroulement,
    /// Change à chaque choix accepté : les boutons d'un message qui n'est
    /// plus à jour portent une version dépassée.
    pub version: u32,
    /// Ce que montre le camp ; sans effet ailleurs.
    pub vue: Vue,
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
}

pub struct Joueurs {
    /// Une sauvegarde par compte : `<identifiant>.json`.
    dossier: PathBuf,
    sessions: HashMap<IdJoueur, Session>,
    /// Partagé par tous les joueurs, sous le même verrou que leurs parties.
    aleatoire: GenerateurAleatoire,
}

/// Un incident pendant le traitement d'un choix ne doit pas bloquer tous les
/// joueurs : les parties restent utilisables.
pub fn verrouiller(joueurs: &Mutex<Joueurs>) -> MutexGuard<'_, Joueurs> {
    joueurs.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Joueurs {
    pub fn new(dossier: PathBuf, graine: u64) -> Self {
        Self {
            dossier,
            sessions: HashMap::new(),
            aleatoire: GenerateurAleatoire::new(graine),
        }
    }

    /// La version de départ est tirée au hasard, pour qu'un bouton d'avant un
    /// redémarrage du bot ne tombe pas, par chance, sur la même.
    pub fn session(&mut self, joueur: IdJoueur, contenu: &ContenuV1) -> &Session {
        let dossier = &self.dossier;
        let aleatoire = &mut self.aleatoire;
        self.sessions.entry(joueur).or_insert_with(|| Session {
            deroulement: Deroulement::demarrer(depot(dossier, joueur).charger(contenu)),
            version: aleatoire.tirer(0..=u32::MAX),
            vue: Vue::Camp,
        })
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

/// Le camp s'ouvre sur sa vue principale au retour d'expédition. La gestion
/// des monstres reste ouverte d'une action à l'autre, comme dans la console,
/// mais le monstre choisi est oublié : sa place a pu changer.
fn vue_apres(vue: Vue, choix: &Choix) -> Vue {
    match (vue, choix) {
        (_, Choix::PartirEnExpedition(_)) => Vue::Camp,
        (Vue::Gestion(Some(_)), _) => Vue::Gestion(None),
        (vue, _) => vue,
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
    use bersemon_core::espece::EspeceId;
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
            Choix::MonstreDeDepart(EspeceId::new("salamandre")),
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

    #[test]
    fn les_vues_du_camp_ne_changent_pas_la_version() {
        let contenu = ContenuV1::new();
        let dossier = dossier_de_test();
        let mut joueurs = Joueurs::new(dossier.clone(), 1);
        for choix in [
            Choix::NomDuHeros("Aldric".to_string()),
            Choix::Classe(Classe::Mage),
            Choix::MonstreDeDepart(EspeceId::new("salamandre")),
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
}
