//! Espèces de monstres : ce que tous les monstres d'une même espèce ont en
//! commun (section 5.1).

use crate::stats::StatsDeBase;
use crate::types::TypesMonstre;

/// Identifiant stable d'une espèce. C'est lui, et non les données de
/// l'espèce, qui est enregistré dans une sauvegarde : rééquilibrer une espèce
/// s'applique ainsi aux monstres déjà capturés.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EspeceId(String);

impl EspeceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Rôle d'une espèce, en plus de ses types. Son effet en jeu reste à définir
/// (Q19). Hypothèse du code : le rôle est porté par l'espèce, pas par chaque
/// monstre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Soutien,
    Offensif,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Espece {
    id: EspeceId,
    nom: String,
    types: TypesMonstre,
    role: Role,
    stats_de_base: StatsDeBase,
}

impl Espece {
    pub fn new(
        id: EspeceId,
        nom: impl Into<String>,
        types: TypesMonstre,
        role: Role,
        stats_de_base: StatsDeBase,
    ) -> Self {
        Self {
            id,
            nom: nom.into(),
            types,
            role,
            stats_de_base,
        }
    }

    pub fn id(&self) -> &EspeceId {
        &self.id
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }

    pub fn types(&self) -> &TypesMonstre {
        &self.types
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn stats_de_base(&self) -> &StatsDeBase {
        &self.stats_de_base
    }
}
