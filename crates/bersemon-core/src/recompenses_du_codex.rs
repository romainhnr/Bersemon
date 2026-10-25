//! Les récompenses de la complétion du codex (section 5.6) : toutes les 10
//! espèces capturées, puis au codex complet, le joueur réclame depuis le
//! codex une récompense, qui va dans le coffre.

use std::fmt;

use crate::codex::{Codex, EtatCodex};
use crate::equipement::ObjetDEquipement;
use crate::espece::{Espece, Rarete};
use crate::inventaire::Inventaire;

/// Les Apôtres et le Démiurge n'y comptent pas : ils ont leur carnet de
/// traque (section 5.6).
pub fn compte_au_codex(espece: &Espece) -> bool {
    !matches!(espece.rarete(), Rarete::Apotre | Rarete::Demiurge)
}

pub fn especes_au_codex(especes: &[Espece]) -> u32 {
    let nombre = especes
        .iter()
        .filter(|espece| compte_au_codex(espece))
        .count();
    u32::try_from(nombre).unwrap_or(u32::MAX)
}

/// Celles que le codex dit capturées : le monstre de départ, les captures,
/// les achats, la pêche et les petits du rite.
pub fn especes_capturees(codex: &Codex, especes: &[Espece]) -> u32 {
    let nombre = especes
        .iter()
        .filter(|espece| compte_au_codex(espece))
        .filter(|espece| codex.etat(espece.id()) == EtatCodex::Capturee)
        .count();
    u32::try_from(nombre).unwrap_or(u32::MAX)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PalierDuCodex {
    /// Ce nombre d'espèces capturées.
    Especes(u32),
    /// Toutes les espèces du codex, quel qu'en soit le nombre.
    Complet,
}

impl PalierDuCodex {
    pub fn seuil(self, especes_au_codex: u32) -> u32 {
        match self {
            PalierDuCodex::Especes(nombre) => nombre,
            PalierDuCodex::Complet => especes_au_codex,
        }
    }
}

/// Ce que rapporte un palier : des objets pour le coffre, et parfois un
/// objet d'équipement unique, qui ne se fabrique pas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecompenseDuCodex {
    pub palier: PalierDuCodex,
    pub objets: Inventaire,
    pub objet_unique: Option<ObjetDEquipement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErreurCodex {
    /// Aucun palier atteint qui n'ait déjà sa récompense.
    AucuneRecompense,
}

impl fmt::Display for ErreurCodex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurCodex::AucuneRecompense => write!(f, "aucune récompense du codex à réclamer"),
        }
    }
}

impl std::error::Error for ErreurCodex {}
