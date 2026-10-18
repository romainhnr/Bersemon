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
use bersemon_core::sauvegarde::SauvegardeRepository;
use bersemon_storage::FichierJson;

/// Identifiant du compte Discord.
pub type IdJoueur = u64;

pub struct Session {
    pub deroulement: Deroulement,
    /// Change à chaque choix accepté : les boutons d'un message qui n'est
    /// plus à jour portent une version dépassée.
    pub version: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Refus {
    MessagePerime,
    Choix(ErreurChoix),
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
        })
    }

    /// Le bouton affichait-il la version en cours de la partie ?
    pub fn est_a_jour(&mut self, joueur: IdJoueur, version: u32, contenu: &ContenuV1) -> bool {
        self.session(joueur, contenu).version == version
    }

    /// Applique le choix si le message affichait la version en cours de la
    /// partie. Un choix refusé ne change rien.
    pub fn repondre(
        &mut self,
        joueur: IdJoueur,
        version: u32,
        choix: Choix,
        contenu: &ContenuV1,
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
        };
        let evenements = session
            .deroulement
            .repondre(choix, &mut contexte)
            .map_err(Refus::Choix)?;
        session.version = session.version.wrapping_add(1);
        Ok(evenements)
    }
}

fn depot(dossier: &Path, joueur: IdJoueur) -> FichierJson {
    FichierJson::new(dossier.join(format!("{joueur}.json")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::deroulement::Attente;
    use bersemon_core::espece::EspeceId;
    use bersemon_core::heros::Classe;
    use bersemon_core::nom::ErreurNom;
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
        joueurs.repondre(joueur, version, choix, contenu)
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
        joueurs.repondre(1, version, nom.clone(), &contenu).unwrap();
        assert_eq!(
            joueurs.repondre(1, version, nom, &contenu),
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
            joueurs.repondre(1, version, vide, &contenu),
            Err(Refus::Choix(ErreurChoix::Nom(ErreurNom::Vide)))
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
}
