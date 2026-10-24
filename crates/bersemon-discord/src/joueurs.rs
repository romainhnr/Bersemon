//! Parties des joueurs Discord, une par compte (Q60), gardées en mémoire et
//! chargées depuis leur sauvegarde à la première demande du joueur. Après un
//! redémarrage du bot, chacun reprend donc à sa dernière sauvegarde (section
//! 13.9).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};

use bersemon_core::aleatoire::{Aleatoire, GenerateurAleatoire};
use bersemon_core::combat::{Cote, Membre};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{Attente, Choix, Contexte, Deroulement, ErreurChoix, Evenement};
use bersemon_core::duel::Duel;
use bersemon_core::echange::{ErreurEchange, Offre};
use bersemon_core::groupe::{ContexteDeGroupe, ExpeditionEnGroupe};
use bersemon_core::horodatage::Horodatage;
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::SauvegardeRepository;
use bersemon_core::tour::RecordsDeLaTour;
use bersemon_presentation::duels::{defi_accepte, defi_recu, defi_refuse};
use bersemon_presentation::echanges::{offre_acceptee, offre_recue, offre_refusee};
use bersemon_presentation::groupe::{invitation_acceptee, invitation_recue, invitation_refusee};
use bersemon_storage::FichierJson;

use crate::composants::Vue;
use crate::duels::{ActionDeDuel, Defi, DuelEnCours, DuelVu, ErreurDeDefi};
use crate::echanges::{
    ActionDEchange, AutreJoueur, Bourse, ErreurDOffre, MessagePrive, OffreEnAttente, Pseudos,
    TableauDesEchanges,
};
use crate::groupes::{ActionDeGroupe, ErreurDInvitation, GroupeEnCours, GroupeVu, Invitation};
use crate::message::OPTIONS_PAR_MENU;
use crate::messages_prives::{Annonces, Retour, parties};

/// Identifiant du compte Discord.
pub type IdJoueur = u64;

/// Le classement des duels ne montre que les premiers (Q94).
const PLACES_AU_CLASSEMENT: usize = 10;

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
    /// Le duel du joueur, préparé à chaque lecture de la session : il
    /// remplace le camp à l'écran (section 15.3).
    pub duel: Option<DuelVu>,
    /// L'expédition en groupe du joueur, préparée de même (section 15.5).
    pub groupe: Option<GroupeVu>,
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
    Duel(ErreurDeDefi),
    Groupe(ErreurDInvitation),
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
    /// Offres et défis reçus, acceptés ou refusés, à annoncer par message
    /// privé.
    messages_prives: Vec<MessagePrive>,
    defis: Vec<Defi>,
    duels: Vec<DuelEnCours>,
    invitations: Vec<Invitation>,
    groupes: Vec<GroupeEnCours>,
    /// Ce que chaque joueur n'a pas encore vu des suites des choix de son
    /// compagnon, même après la fin de l'expédition en groupe.
    recits_en_attente: HashMap<IdJoueur, Vec<Evenement>>,
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
            defis: Vec::new(),
            duels: Vec::new(),
            invitations: Vec::new(),
            groupes: Vec::new(),
            recits_en_attente: HashMap::new(),
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
            duel: None,
            groupe: None,
        })
    }

    pub fn session(&mut self, joueur: IdJoueur, contenu: &ContenuV1) -> &Session {
        let vue = self.charger(joueur, contenu).vue;
        let avec = match vue {
            Vue::Echanges { avec, .. } => Some(avec),
            Vue::Classement | Vue::RecordsDeLaTour => Some(None),
            _ => None,
        };
        if let Some(avec) = avec {
            let tableau = self.tableau_des_echanges(joueur, avec, contenu);
            self.charger(joueur, contenu).echanges = tableau;
        }
        let duel = self.duel_vu(joueur, contenu);
        let groupe = self.groupe_vu(joueur, contenu);
        let session = self.charger(joueur, contenu);
        session.duel = duel;
        session.groupe = groupe;
        session
    }

    fn groupe_de(&self, joueur: IdJoueur) -> Option<(usize, Membre)> {
        self.groupes
            .iter()
            .enumerate()
            .find_map(|(position, groupe)| Some((position, groupe.membre_de(joueur)?)))
    }

    fn groupe_vu(&mut self, joueur: IdJoueur, contenu: &ContenuV1) -> Option<GroupeVu> {
        let (position, membre) = self.groupe_de(joueur)?;
        let autre = self.groupes[position].joueur(membre.autre());
        let compagnon = self.autre_joueur(autre, contenu)?.heros;
        let groupe = &self.groupes[position];
        let [hote, invite] = groupe
            .joueurs
            .map(|qui| self.sessions.get(&qui).map(|session| &session.deroulement));
        let attendus = match (hote, invite) {
            (Some(hote), Some(invite)) => groupe.expedition.attendus([hote, invite]),
            _ => Vec::new(),
        };
        Some(GroupeVu {
            membre,
            compagnon,
            combat: groupe.expedition.combat().cloned(),
            attendus,
            derniere_action: groupe.expedition.derniere_action(),
        })
    }

    /// Les suites des choix du compagnon, que le joueur n'a pas encore
    /// vues ; elles ne se montrent qu'une fois.
    pub fn prendre_le_recit_du_groupe(&mut self, joueur: IdJoueur) -> Vec<Evenement> {
        self.recits_en_attente.remove(&joueur).unwrap_or_default()
    }

    fn duel_de(&self, joueur: IdJoueur) -> Option<(usize, Cote)> {
        self.duels
            .iter()
            .enumerate()
            .find_map(|(position, duel)| Some((position, duel.cote_de(joueur)?)))
    }

    fn duel_vu(&mut self, joueur: IdJoueur, contenu: &ContenuV1) -> Option<DuelVu> {
        let (position, cote) = self.duel_de(joueur)?;
        let autre = self.duels[position].autre(cote);
        let heros = self.autre_joueur(joueur, contenu)?.heros;
        let adversaire = self.autre_joueur(autre, contenu)?.heros;
        Some(DuelVu {
            duel: self.duels[position].duel.clone(),
            cote,
            heros,
            adversaire,
        })
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
        let partie = partie_lisible(&self.charger(joueur, contenu).deroulement)?;
        let heros = partie.heros().nom().as_str().to_string();
        let palmares = partie.relations().palmares();
        let records = partie.records_de_la_tour();
        Some(AutreJoueur {
            id: joueur,
            heros,
            pseudo: self.pseudos.pseudo(joueur).map(str::to_string),
            palmares,
            records,
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
        let tous: Vec<AutreJoueur> = parties(&self.dossier)
            .into_iter()
            .filter_map(|(autre, _)| self.autre_joueur(autre, contenu))
            .collect();
        let classement = classer(&tous);
        let records_seul = classer_les_records(&tous, |records| records.seul);
        let records_a_deux = classer_les_records(&tous, |records| records.a_deux);
        let amis = partie_lisible(&self.charger(joueur, contenu).deroulement)
            .map(|partie| partie.relations().clone())
            .unwrap_or_default();
        let mut joueurs: Vec<AutreJoueur> = tous
            .into_iter()
            .filter(|autre| autre.id != joueur)
            .collect();
        // Les amis d'abord (section 15.4).
        joueurs.sort_by(|a, b| {
            amis.est_ami(b.id)
                .cmp(&amis.est_ami(a.id))
                .then(a.heros.cmp(&b.heros))
                .then(a.id.cmp(&b.id))
        });
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
        let defi_recu = self.defis.iter().find(|defi| defi.a == joueur).copied();
        let defi_recu = defi_recu.and_then(|defi| self.autre_joueur(defi.de, contenu));
        let defi_envoye = self.defis.iter().find(|defi| defi.de == joueur).copied();
        let defi_envoye = defi_envoye.and_then(|defi| self.autre_joueur(defi.a, contenu));
        let invitation_recue = self
            .invitations
            .iter()
            .find(|invitation| invitation.a == joueur)
            .copied();
        let invitation_recue =
            invitation_recue.and_then(|invitation| self.autre_joueur(invitation.de, contenu));
        let invitation_envoyee = self
            .invitations
            .iter()
            .find(|invitation| invitation.de == joueur)
            .copied();
        let invitation_envoyee =
            invitation_envoyee.and_then(|invitation| self.autre_joueur(invitation.a, contenu));
        TableauDesEchanges {
            joueurs,
            recue,
            envoyee,
            choisi,
            defi_recu,
            defi_envoye,
            invitation_recue,
            invitation_envoyee,
            classement,
            records_seul,
            records_a_deux,
        }
    }

    /// Ajoute ou retire un ami de la liste du joueur, qui doit être au camp
    /// (section 15.4).
    fn changer_d_ami(
        &mut self,
        joueur: IdJoueur,
        autre: IdJoueur,
        ami: bool,
        contenu: &ContenuV1,
    ) -> Result<Vec<Evenement>, ErreurDOffre> {
        if autre == joueur {
            return Err(ErreurDOffre::AvecSoiMeme);
        }
        if ami {
            self.autre_joueur(autre, contenu)
                .ok_or(ErreurDOffre::JoueurSansPartie)?;
        }
        let depot = depot(&self.dossier, joueur);
        self.charger(joueur, contenu)
            .deroulement
            .changer_d_ami(autre, ami, &depot)
            .ok_or(ErreurDOffre::AmitieInchangee)
    }

    /// Une fois le duel fini, sa victoire ou sa défaite rejoint le palmarès
    /// de chaque joueur, une seule fois (section 15.4).
    fn noter_la_fin_du_duel(&mut self, joueur: IdJoueur) {
        let Some((position, _)) = self.duel_de(joueur) else {
            return;
        };
        let en_cours = &mut self.duels[position];
        let Some(vainqueur) = en_cours.duel.vainqueur() else {
            return;
        };
        if en_cours.palmares_note {
            return;
        }
        en_cours.palmares_note = true;
        let joueurs = en_cours.joueurs;
        for (qui, cote) in joueurs.into_iter().zip([Cote::Joueur, Cote::Adversaire]) {
            let depot = depot(&self.dossier, qui);
            if let Some(session) = self.sessions.get_mut(&qui) {
                session.deroulement.noter_duel(cote == vainqueur, &depot);
            }
        }
    }

    /// Défie, retire ou répond à un défi, ou agit dans le duel en cours,
    /// si le message affichait la version en cours de la partie. Chaque
    /// changement d'un duel rend dépassés les messages des deux joueurs.
    pub fn agir_en_duel(
        &mut self,
        joueur: IdJoueur,
        version: u32,
        action: ActionDeDuel,
        contenu: &ContenuV1,
        maintenant: Horodatage,
    ) -> Result<Vec<Evenement>, Refus> {
        if !self.est_a_jour(joueur, version, contenu) {
            return Err(Refus::MessagePerime);
        }
        let concernes = match action {
            ActionDeDuel::Defier(a) => self.defier(joueur, a, contenu).map(|()| vec![joueur]),
            ActionDeDuel::RetirerDefi => self.retirer_defi(joueur).map(|()| vec![joueur]),
            ActionDeDuel::RefuserDefi => self.refuser_defi(joueur, contenu),
            ActionDeDuel::AccepterDefi => self.accepter_defi(joueur, contenu),
            ActionDeDuel::Abandonner => {
                self.dans_le_duel(joueur, |duel, cote| duel.abandonner(cote))
            }
            ActionDeDuel::ReclamerLaVictoire => self.dans_le_duel(joueur, |duel, cote| {
                duel.reclamer_la_victoire(cote, maintenant)
            }),
            ActionDeDuel::RevenirAuCamp => self.revenir_du_duel(joueur),
        }
        .map_err(Refus::Duel)?;
        self.noter_la_fin_du_duel(joueur);
        for concerne in concernes {
            let session = self.charger(concerne, contenu);
            session.version = session.version.wrapping_add(1);
        }
        // Le duel fini, le joueur retrouve le camp sur sa vue principale.
        if action == ActionDeDuel::RevenirAuCamp {
            self.charger(joueur, contenu).vue = Vue::Camp;
        }
        Ok(Vec::new())
    }

    fn defier(
        &mut self,
        joueur: IdJoueur,
        a: IdJoueur,
        contenu: &ContenuV1,
    ) -> Result<(), ErreurDeDefi> {
        if a == joueur {
            return Err(ErreurDeDefi::AvecSoiMeme);
        }
        if self.duel_de(joueur).is_some() || self.duel_de(a).is_some() {
            return Err(ErreurDeDefi::DejaEnDuel);
        }
        // Un défi ou une invitation à la fois par joueur, pour que l'écran
        // des autres joueurs tienne dans un message (section 15.5).
        if self.a_un_defi_ou_une_invitation(joueur) {
            return Err(ErreurDeDefi::DefiDejaEnCours);
        }
        if self.a_un_defi_ou_une_invitation(a) {
            return Err(ErreurDeDefi::AutreDejaDefie);
        }
        self.autre_joueur(a, contenu)
            .ok_or(ErreurDeDefi::JoueurSansPartie)?;
        let moi = self
            .autre_joueur(joueur, contenu)
            .ok_or(ErreurDeDefi::JoueurSansPartie)?;
        self.defis.push(Defi { de: joueur, a });
        self.messages_prives.push(MessagePrive {
            joueur: a,
            texte: defi_recu(&moi.heros, moi.pseudo.as_deref()),
        });
        Ok(())
    }

    fn retirer_defi(&mut self, joueur: IdJoueur) -> Result<(), ErreurDeDefi> {
        let position = self
            .defis
            .iter()
            .position(|defi| defi.de == joueur)
            .ok_or(ErreurDeDefi::AucunDefi)?;
        self.defis.remove(position);
        Ok(())
    }

    fn refuser_defi(
        &mut self,
        joueur: IdJoueur,
        contenu: &ContenuV1,
    ) -> Result<Vec<IdJoueur>, ErreurDeDefi> {
        let position = self
            .defis
            .iter()
            .position(|defi| defi.a == joueur)
            .ok_or(ErreurDeDefi::AucunDefi)?;
        let defi = self.defis.remove(position);
        if let Some(moi) = self.autre_joueur(joueur, contenu) {
            self.messages_prives.push(MessagePrive {
                joueur: defi.de,
                texte: defi_refuse(&moi.heros),
            });
        }
        Ok(vec![joueur])
    }

    /// Le duel commence si les deux joueurs sont au camp, chacun avec au
    /// moins un monstre dans son équipe ; sinon le défi reste en attente.
    fn accepter_defi(
        &mut self,
        joueur: IdJoueur,
        contenu: &ContenuV1,
    ) -> Result<Vec<IdJoueur>, ErreurDeDefi> {
        let defi = *self
            .defis
            .iter()
            .find(|defi| defi.a == joueur)
            .ok_or(ErreurDeDefi::AucunDefi)?;
        self.charger(defi.de, contenu);
        let au_camp = |qui: IdJoueur| {
            self.sessions
                .get(&qui)
                .and_then(|session| session.deroulement.partie_au_camp())
        };
        let (Some(premier), Some(second)) = (au_camp(defi.de), au_camp(joueur)) else {
            return Err(ErreurDeDefi::PasAuCamp);
        };
        let duel = Duel::entre(premier, second, contenu.zones(), contenu.piste())
            .map_err(|_| ErreurDeDefi::EquipeVide)?;
        self.defis.retain(|autre| *autre != defi);
        self.duels.push(DuelEnCours {
            joueurs: [defi.de, joueur],
            duel,
            revenus: [false; 2],
            palmares_note: false,
        });
        if let Some(moi) = self.autre_joueur(joueur, contenu) {
            self.messages_prives.push(MessagePrive {
                joueur: defi.de,
                texte: defi_accepte(&moi.heros),
            });
        }
        Ok(vec![defi.de, joueur])
    }

    /// Applique au duel du joueur ce qu'il y fait ; rend les deux joueurs
    /// du duel.
    fn dans_le_duel(
        &mut self,
        joueur: IdJoueur,
        agir: impl FnOnce(&mut Duel, Cote) -> Result<(), bersemon_core::duel::ErreurDuel>,
    ) -> Result<Vec<IdJoueur>, ErreurDeDefi> {
        let (position, cote) = self.duel_de(joueur).ok_or(ErreurDeDefi::PasEnDuel)?;
        let en_cours = &mut self.duels[position];
        agir(&mut en_cours.duel, cote).map_err(ErreurDeDefi::Duel)?;
        Ok(en_cours.joueurs.to_vec())
    }

    fn revenir_du_duel(&mut self, joueur: IdJoueur) -> Result<Vec<IdJoueur>, ErreurDeDefi> {
        let (position, cote) = self.duel_de(joueur).ok_or(ErreurDeDefi::PasEnDuel)?;
        let en_cours = &mut self.duels[position];
        if !en_cours.duel.est_fini() {
            return Err(ErreurDeDefi::DuelPasFini);
        }
        en_cours.revenus[match cote {
            Cote::Joueur => 0,
            Cote::Adversaire => 1,
        }] = true;
        if en_cours.revenus == [true; 2] {
            self.duels.remove(position);
        }
        Ok(vec![joueur])
    }

    /// Les choix du combat d'un joueur en duel vont au duel : action du tour,
    /// remplacement et ordre du héros.
    fn jouer_en_duel(
        &mut self,
        joueur: IdJoueur,
        choix: Choix,
        maintenant: Horodatage,
    ) -> Result<Vec<IdJoueur>, Refus> {
        let aleatoire = &mut self.aleatoire;
        let (position, cote) = match self
            .duels
            .iter()
            .enumerate()
            .find_map(|(position, duel)| Some((position, duel.cote_de(joueur)?)))
        {
            Some(trouve) => trouve,
            None => return Err(Refus::Duel(ErreurDeDefi::PasEnDuel)),
        };
        let en_cours = &mut self.duels[position];
        let resultat = match &choix {
            Choix::Combat(action) => en_cours.duel.choisir(cote, *action, maintenant, aleatoire),
            Choix::Remplacer(index) => en_cours.duel.remplacer(cote, *index, maintenant),
            Choix::DonnerOrdre => en_cours.duel.donner_ordre(cote),
            _ => {
                return Err(Refus::Choix {
                    choix,
                    erreur: ErreurChoix::HorsDePropos,
                });
            }
        };
        resultat.map_err(|erreur| Refus::Duel(ErreurDeDefi::Duel(erreur)))?;
        Ok(en_cours.joueurs.to_vec())
    }

    /// Invite, retire ou répond à une invitation, ou ramène le groupe au
    /// camp, si le message affichait la version en cours de la partie.
    /// Chaque changement d'une expédition en groupe rend dépassés les
    /// messages des deux joueurs (section 15.5).
    pub fn agir_en_groupe(
        &mut self,
        joueur: IdJoueur,
        version: u32,
        action: ActionDeGroupe,
        contenu: &ContenuV1,
        maintenant: Horodatage,
    ) -> Result<Vec<Evenement>, Refus> {
        if !self.est_a_jour(joueur, version, contenu) {
            return Err(Refus::MessagePerime);
        }
        let resultat = match action {
            ActionDeGroupe::Inviter(a) => self.inviter(joueur, a, contenu).map(|()| Vec::new()),
            ActionDeGroupe::RetirerInvitation => {
                self.retirer_invitation(joueur).map(|()| Vec::new())
            }
            ActionDeGroupe::RefuserInvitation => self
                .refuser_invitation(joueur, contenu)
                .map(|()| Vec::new()),
            ActionDeGroupe::AccepterInvitation => {
                self.accepter_invitation(joueur, contenu, maintenant)
            }
            ActionDeGroupe::RamenerLeGroupe => self.dans_le_groupe(
                joueur,
                contenu,
                maintenant,
                |expedition, membre, deroulements, contexte| {
                    expedition.ramener_au_camp(membre, deroulements, contexte)
                },
            ),
        };
        let evenements = resultat.map_err(Refus::Groupe)?;
        let session = self.charger(joueur, contenu);
        session.version = session.version.wrapping_add(1);
        Ok(evenements)
    }

    fn inviter(
        &mut self,
        joueur: IdJoueur,
        a: IdJoueur,
        contenu: &ContenuV1,
    ) -> Result<(), ErreurDInvitation> {
        if a == joueur {
            return Err(ErreurDInvitation::AvecSoiMeme);
        }
        if self.est_occupe(joueur) || self.est_occupe(a) {
            return Err(ErreurDInvitation::DejaOccupe);
        }
        if self.a_un_defi_ou_une_invitation(joueur) {
            return Err(ErreurDInvitation::InvitationDejaEnCours);
        }
        if self.a_un_defi_ou_une_invitation(a) {
            return Err(ErreurDInvitation::AutreDejaInvite);
        }
        self.autre_joueur(a, contenu)
            .ok_or(ErreurDInvitation::JoueurSansPartie)?;
        let moi = self
            .autre_joueur(joueur, contenu)
            .ok_or(ErreurDInvitation::JoueurSansPartie)?;
        self.invitations.push(Invitation { de: joueur, a });
        self.messages_prives.push(MessagePrive {
            joueur: a,
            texte: invitation_recue(&moi.heros, moi.pseudo.as_deref()),
        });
        Ok(())
    }

    /// Envoyé ou reçu, en attente de réponse.
    fn a_un_defi_ou_une_invitation(&self, joueur: IdJoueur) -> bool {
        self.defis
            .iter()
            .any(|defi| defi.de == joueur || defi.a == joueur)
            || self
                .invitations
                .iter()
                .any(|invitation| invitation.de == joueur || invitation.a == joueur)
    }

    /// En duel ou en expédition en groupe.
    fn est_occupe(&self, joueur: IdJoueur) -> bool {
        self.duel_de(joueur).is_some() || self.groupe_de(joueur).is_some()
    }

    fn retirer_invitation(&mut self, joueur: IdJoueur) -> Result<(), ErreurDInvitation> {
        let position = self
            .invitations
            .iter()
            .position(|invitation| invitation.de == joueur)
            .ok_or(ErreurDInvitation::AucuneInvitation)?;
        self.invitations.remove(position);
        Ok(())
    }

    fn refuser_invitation(
        &mut self,
        joueur: IdJoueur,
        contenu: &ContenuV1,
    ) -> Result<(), ErreurDInvitation> {
        let position = self
            .invitations
            .iter()
            .position(|invitation| invitation.a == joueur)
            .ok_or(ErreurDInvitation::AucuneInvitation)?;
        let invitation = self.invitations.remove(position);
        if let Some(moi) = self.autre_joueur(joueur, contenu) {
            self.messages_prives.push(MessagePrive {
                joueur: invitation.de,
                texte: invitation_refusee(&moi.heros),
            });
        }
        Ok(())
    }

    /// La descente commence si les deux joueurs sont au camp, sans question
    /// en attente, ont ouvert la Tour et ont au moins un monstre dans
    /// l'équipe ; sinon l'invitation reste en attente.
    fn accepter_invitation(
        &mut self,
        joueur: IdJoueur,
        contenu: &ContenuV1,
        maintenant: Horodatage,
    ) -> Result<Vec<Evenement>, ErreurDInvitation> {
        let invitation = *self
            .invitations
            .iter()
            .find(|invitation| invitation.a == joueur)
            .ok_or(ErreurDInvitation::AucuneInvitation)?;
        if self.est_occupe(invitation.de) || self.est_occupe(joueur) {
            return Err(ErreurDInvitation::DejaOccupe);
        }
        self.charger(invitation.de, contenu);
        let au_camp = |qui: IdJoueur| {
            self.sessions
                .get(&qui)
                .and_then(|session| session.deroulement.partie_au_camp())
                .is_some()
        };
        if !au_camp(invitation.de) || !au_camp(joueur) {
            return Err(ErreurDInvitation::PasAuCamp);
        }
        let mut hote = self
            .sessions
            .remove(&invitation.de)
            .expect("session chargée juste avant");
        let resultat = match self.sessions.get_mut(&joueur) {
            Some(invite) => {
                let depots = [
                    depot(&self.dossier, invitation.de),
                    depot(&self.dossier, joueur),
                ];
                let mut contexte = ContexteDeGroupe {
                    contenu,
                    aleatoire: &mut self.aleatoire,
                    depots: [&depots[0], &depots[1]],
                    maintenant,
                };
                let resultat = ExpeditionEnGroupe::partir(
                    [&mut hote.deroulement, &mut invite.deroulement],
                    &mut contexte,
                );
                if resultat.is_ok() {
                    invite.vue = Vue::Camp;
                }
                resultat
            }
            None => Err(ErreurChoix::HorsDePropos),
        };
        if resultat.is_ok() {
            hote.version = hote.version.wrapping_add(1);
            hote.vue = Vue::Camp;
        }
        self.sessions.insert(invitation.de, hote);
        let (expedition, [recit_de_l_hote, recit_de_l_invite]) =
            resultat.map_err(ErreurDInvitation::Choix)?;
        self.invitations.retain(|autre| *autre != invitation);
        self.groupes.push(GroupeEnCours {
            joueurs: [invitation.de, joueur],
            expedition,
        });
        self.recits_en_attente
            .entry(invitation.de)
            .or_default()
            .extend(recit_de_l_hote);
        if let Some(moi) = self.autre_joueur(joueur, contenu) {
            self.messages_prives.push(MessagePrive {
                joueur: invitation.de,
                texte: invitation_acceptee(&moi.heros),
            });
        }
        Ok(recit_de_l_invite)
    }

    /// Applique à l'expédition en groupe du joueur ce qu'il y fait, avec
    /// les deux déroulements ; rend ce que lui voit, garde pour l'autre ce
    /// qu'il n'a pas encore vu, et oublie l'expédition une fois chacun
    /// rentré au camp.
    fn dans_le_groupe(
        &mut self,
        joueur: IdJoueur,
        contenu: &ContenuV1,
        maintenant: Horodatage,
        agir: impl FnOnce(
            &mut ExpeditionEnGroupe,
            Membre,
            [&mut Deroulement; 2],
            &mut ContexteDeGroupe<'_>,
        ) -> Result<[Vec<Evenement>; 2], ErreurChoix>,
    ) -> Result<Vec<Evenement>, ErreurDInvitation> {
        let (position, membre) = self
            .groupe_de(joueur)
            .ok_or(ErreurDInvitation::PasEnGroupe)?;
        let joueurs = self.groupes[position].joueurs;
        let autre = self.groupes[position].joueur(membre.autre());
        self.charger(autre, contenu);
        self.charger(joueur, contenu);
        let mut session_de_l_autre = self
            .sessions
            .remove(&autre)
            .expect("session chargée juste avant");
        let session = self
            .sessions
            .get_mut(&joueur)
            .expect("session chargée juste avant");
        let depots = joueurs.map(|qui| depot(&self.dossier, qui));
        let mut contexte = ContexteDeGroupe {
            contenu,
            aleatoire: &mut self.aleatoire,
            depots: [&depots[0], &depots[1]],
            maintenant,
        };
        let deroulements = match membre {
            Membre::Hote => [
                &mut session.deroulement,
                &mut session_de_l_autre.deroulement,
            ],
            Membre::Invite => [
                &mut session_de_l_autre.deroulement,
                &mut session.deroulement,
            ],
        };
        let resultat = agir(
            &mut self.groupes[position].expedition,
            membre,
            deroulements,
            &mut contexte,
        );
        let termine = self.groupes[position]
            .expedition
            .est_terminee([&session.deroulement, &session_de_l_autre.deroulement]);
        if resultat.is_ok() {
            session_de_l_autre.version = session_de_l_autre.version.wrapping_add(1);
            if termine {
                session.vue = Vue::Camp;
                session_de_l_autre.vue = Vue::Camp;
            }
        }
        self.sessions.insert(autre, session_de_l_autre);
        let recits = resultat.map_err(ErreurDInvitation::Choix)?;
        let [du_joueur, de_l_autre] = match membre {
            Membre::Hote => recits,
            Membre::Invite => {
                let [hote, invite] = recits;
                [invite, hote]
            }
        };
        let mut vus = self.prendre_le_recit_du_groupe(joueur);
        vus.extend(du_joueur);
        self.recits_en_attente
            .entry(autre)
            .or_default()
            .extend(de_l_autre);
        if termine {
            self.groupes.remove(position);
        }
        Ok(vus)
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
            ActionDEchange::AjouterAmi(autre) => self.changer_d_ami(joueur, autre, true, contenu),
            ActionDEchange::RetirerAmi(autre) => self.changer_d_ami(joueur, autre, false, contenu),
        };
        let evenements = resultat.map_err(Refus::Echange)?;
        // Après un changement d'ami, le joueur choisi reste à l'écran.
        let avec = match action {
            ActionDEchange::AjouterAmi(autre) | ActionDEchange::RetirerAmi(autre) => Some(autre),
            _ => None,
        };
        let session = self.charger(joueur, contenu);
        session.version = session.version.wrapping_add(1);
        session.vue = Vue::Echanges { avec, cede: None };
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
        if self.groupe_de(joueur).is_some() {
            let evenements = self
                .dans_le_groupe(
                    joueur,
                    contenu,
                    maintenant,
                    |expedition, membre, deroulements, contexte| {
                        expedition.repondre(membre, choix.clone(), deroulements, contexte)
                    },
                )
                .map_err(|erreur| match erreur {
                    ErreurDInvitation::Choix(erreur) => Refus::Choix {
                        choix: choix.clone(),
                        erreur,
                    },
                    autre => Refus::Groupe(autre),
                })?;
            let session = self.charger(joueur, contenu);
            session.version = session.version.wrapping_add(1);
            session.vue = vue_apres(session.vue, &choix);
            return Ok(evenements);
        }
        if self.duel_de(joueur).is_some() {
            let concernes = self.jouer_en_duel(joueur, choix, maintenant)?;
            self.noter_la_fin_du_duel(joueur);
            for concerne in concernes {
                let session = self.charger(concerne, contenu);
                session.version = session.version.wrapping_add(1);
            }
            return Ok(Vec::new());
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
        (Vue::Entrainement, _) => Vue::Machines,
        // Le sacrifice est suivi du départ vers l'Éclipse (section 9.4).
        (Vue::Eclipse, _) => Vue::Camp,
        (vue, _) => vue,
    }
}

/// Une partie reprise après un redémarrage se lit avant que son joueur ne la
/// reprenne : ses héros et ses monstres sont ceux de sa sauvegarde.
/// Ceux qui ont livré au moins un duel, par victoires, puis par défaites,
/// les moins nombreuses d'abord, puis par nom du héros ; 10 au plus (section
/// 15.4, Q94).
fn classer(joueurs: &[AutreJoueur]) -> Vec<AutreJoueur> {
    let mut classes: Vec<AutreJoueur> = joueurs
        .iter()
        .filter(|joueur| joueur.palmares.duels() > 0)
        .cloned()
        .collect();
    classes.sort_by(|a, b| {
        b.palmares
            .victoires
            .cmp(&a.palmares.victoires)
            .then(a.palmares.defaites.cmp(&b.palmares.defaites))
            .then(a.heros.cmp(&b.heros))
            .then(a.id.cmp(&b.id))
    });
    classes.truncate(PLACES_AU_CLASSEMENT);
    classes
}

/// Ceux qui ont franchi au moins une étape de la Tour sans fond, du record
/// le plus profond au moins profond, puis par nom du héros ; 10 au plus
/// (section 9.5).
fn classer_les_records(
    joueurs: &[AutreJoueur],
    record: impl Fn(RecordsDeLaTour) -> u32,
) -> Vec<AutreJoueur> {
    let mut classes: Vec<AutreJoueur> = joueurs
        .iter()
        .filter(|joueur| record(joueur.records) > 0)
        .cloned()
        .collect();
    classes.sort_by(|a, b| {
        record(b.records)
            .cmp(&record(a.records))
            .then(a.heros.cmp(&b.heros))
            .then(a.id.cmp(&b.id))
    });
    classes.truncate(PLACES_AU_CLASSEMENT);
    classes
}

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

    fn en_duel(
        joueurs: &mut Joueurs,
        joueur: IdJoueur,
        action: ActionDeDuel,
        contenu: &ContenuV1,
    ) -> Result<Vec<Evenement>, Refus> {
        let version = joueurs.session(joueur, contenu).version;
        joueurs.agir_en_duel(
            joueur,
            version,
            action,
            contenu,
            Horodatage::depuis_secondes(0),
        )
    }

    #[test]
    fn un_defi_se_refuse_se_retire_ou_s_accepte() {
        let contenu = ContenuV1::new();
        let (mut joueurs, dossier) = deux_joueurs(&contenu);
        assert_eq!(
            en_duel(&mut joueurs, 1, ActionDeDuel::Defier(1), &contenu),
            Err(Refus::Duel(ErreurDeDefi::AvecSoiMeme))
        );
        en_duel(&mut joueurs, 1, ActionDeDuel::Defier(2), &contenu).unwrap();
        let messages = joueurs.prendre_les_messages_prives();
        assert_eq!(messages[0].joueur, 2);
        assert!(messages[0].texte.starts_with("Aldric vous défie en duel."));
        // Un défi à la fois, envoyé ou reçu.
        assert_eq!(
            en_duel(&mut joueurs, 2, ActionDeDuel::Defier(1), &contenu),
            Err(Refus::Duel(ErreurDeDefi::DefiDejaEnCours))
        );
        en_duel(&mut joueurs, 2, ActionDeDuel::RefuserDefi, &contenu).unwrap();
        assert_eq!(
            joueurs.prendre_les_messages_prives()[0].texte,
            "Ysolde a refusé votre défi."
        );
        en_duel(&mut joueurs, 1, ActionDeDuel::Defier(2), &contenu).unwrap();
        en_duel(&mut joueurs, 1, ActionDeDuel::RetirerDefi, &contenu).unwrap();
        assert_eq!(
            en_duel(&mut joueurs, 2, ActionDeDuel::AccepterDefi, &contenu),
            Err(Refus::Duel(ErreurDeDefi::AucunDefi))
        );

        // Aldric part en expédition : le défi attend son retour au camp.
        en_duel(&mut joueurs, 1, ActionDeDuel::Defier(2), &contenu).unwrap();
        let partir = Choix::PartirEnExpedition(ZoneId::new("lisiere"));
        repondre(&mut joueurs, 1, partir, &contenu).unwrap();
        assert_eq!(
            en_duel(&mut joueurs, 2, ActionDeDuel::AccepterDefi, &contenu),
            Err(Refus::Duel(ErreurDeDefi::PasAuCamp))
        );
        let _ = fs::remove_dir_all(dossier);
    }

    #[test]
    fn le_duel_remplace_le_camp_puis_chacun_y_revient() {
        let contenu = ContenuV1::new();
        let (mut joueurs, dossier) = deux_joueurs(&contenu);
        en_duel(&mut joueurs, 1, ActionDeDuel::Defier(2), &contenu).unwrap();
        joueurs.prendre_les_messages_prives();
        let version_d_aldric = joueurs.session(1, &contenu).version;
        en_duel(&mut joueurs, 2, ActionDeDuel::AccepterDefi, &contenu).unwrap();
        assert_ne!(joueurs.session(1, &contenu).version, version_d_aldric);
        assert_eq!(
            joueurs.prendre_les_messages_prives()[0].texte,
            "Ysolde a accepté votre défi : le duel commence. Ouvrez /bersemon."
        );
        let vu = joueurs.session(2, &contenu).duel.clone().unwrap();
        assert_eq!(
            (vu.heros.as_str(), vu.adversaire.as_str()),
            ("Ysolde", "Aldric")
        );
        assert_eq!(vu.cote, Cote::Adversaire);
        // En duel, les choix du camp sont refusés et ceux du combat vont au
        // duel.
        let partir = Choix::PartirEnExpedition(ZoneId::new("lisiere"));
        assert!(matches!(
            repondre(&mut joueurs, 1, partir, &contenu),
            Err(Refus::Choix { .. })
        ));
        repondre(&mut joueurs, 1, Choix::DonnerOrdre, &contenu).unwrap();
        let vu = joueurs.session(2, &contenu).duel.clone().unwrap();
        assert_eq!(vu.duel.recit(Cote::Adversaire).len(), 1);
        assert_eq!(
            en_duel(&mut joueurs, 1, ActionDeDuel::RevenirAuCamp, &contenu),
            Err(Refus::Duel(ErreurDeDefi::DuelPasFini))
        );
        en_duel(&mut joueurs, 2, ActionDeDuel::Abandonner, &contenu).unwrap();
        let vu = joueurs.session(1, &contenu).duel.clone().unwrap();
        assert_eq!(vu.duel.vainqueur(), Some(Cote::Joueur));
        en_duel(&mut joueurs, 1, ActionDeDuel::RevenirAuCamp, &contenu).unwrap();
        assert!(joueurs.session(1, &contenu).duel.is_none());
        assert!(joueurs.session(2, &contenu).duel.is_some());
        en_duel(&mut joueurs, 2, ActionDeDuel::RevenirAuCamp, &contenu).unwrap();
        assert!(joueurs.session(2, &contenu).duel.is_none());
        assert!(joueurs.duels.is_empty());
        // La partie n'a pas changé : le duel se jouait sur des copies.
        assert_eq!(
            joueurs.session(1, &contenu).deroulement.attente(),
            Attente::Camp
        );
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
