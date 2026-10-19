//! Contrat de sauvegarde d'une partie (section 13.9). Le noyau ne sait pas où
//! ni sous quelle forme la partie est écrite : un crate de stockage
//! implémente ce trait, en JSON pour la V1 puis avec SQLite.

use std::fmt;

use crate::catalogue::Catalogue;
use crate::partie::Partie;

pub trait SauvegardeRepository {
    /// `Ok(None)` s'il n'existe aucune sauvegarde. Les espèces sont
    /// retrouvées dans le catalogue d'après leur identifiant.
    fn charger(&self, catalogue: &dyn Catalogue) -> Result<Option<Partie>, ErreurSauvegarde>;

    fn enregistrer(&self, partie: &Partie) -> Result<(), ErreurSauvegarde>;

    /// Après le suicide du héros (section 13.9). Sans sauvegarde, il n'y a
    /// rien à faire.
    fn supprimer(&self) -> Result<(), ErreurSauvegarde>;
}

/// Le détail technique reste en texte, pour que le contrat ne dépende pas de
/// la façon dont la sauvegarde est stockée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurSauvegarde {
    /// Lecture ou écriture impossible.
    Acces(String),
    /// Contenu qui n'a pas la forme d'une sauvegarde.
    Illisible(String),
    VersionInconnue(u64),
    EspeceInconnue(String),
    /// Zone d'une expédition automatique en cours (section 12).
    ZoneInconnue(String),
    /// Capsule d'attaque obtenue (section 13.10).
    CapsuleInconnue(String),
    AttaqueInconnue {
        espece: String,
        attaque: String,
    },
    /// Contenu lisible mais contraire aux règles, comme un niveau hors limites.
    Incoherente(String),
}

impl fmt::Display for ErreurSauvegarde {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurSauvegarde::Acces(detail) => write!(f, "accès impossible ({detail})"),
            ErreurSauvegarde::Illisible(detail) => write!(f, "fichier illisible ({detail})"),
            ErreurSauvegarde::VersionInconnue(version) => {
                write!(f, "version de sauvegarde inconnue : {version}")
            }
            ErreurSauvegarde::EspeceInconnue(espece) => {
                write!(f, "espèce inconnue : {espece}")
            }
            ErreurSauvegarde::ZoneInconnue(zone) => write!(f, "zone inconnue : {zone}"),
            ErreurSauvegarde::CapsuleInconnue(capsule) => {
                write!(f, "capsule d'attaque inconnue : {capsule}")
            }
            ErreurSauvegarde::AttaqueInconnue { espece, attaque } => {
                write!(f, "l'espèce {espece} n'apprend pas l'attaque {attaque}")
            }
            ErreurSauvegarde::Incoherente(detail) => write!(f, "données incohérentes ({detail})"),
        }
    }
}

impl std::error::Error for ErreurSauvegarde {}
