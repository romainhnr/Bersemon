//! Boosters (section 14.3) : chacun ajoute 5 à une valeur interne d'un
//! monstre, sans dépasser 31, au camp, et se consomme.

use std::fmt;

use crate::collection::ErreurCollection;
use crate::stats::Stat;

/// Ce qu'un booster ajoute à la valeur interne de sa stat.
pub const GAIN_D_UN_BOOSTER: u8 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Booster {
    FioleDeSang,
    MoelleDeBete,
    EcorceDeFer,
    EncreDArcane,
    CendreBenite,
    PoudreDeVifArgent,
}

impl Booster {
    /// Dans l'ordre des stats, qui est celui de docs/contenu-v1.md, section
    /// 17, des tirages et des affichages.
    pub const TOUS: [Booster; 6] = [
        Booster::FioleDeSang,
        Booster::MoelleDeBete,
        Booster::EcorceDeFer,
        Booster::EncreDArcane,
        Booster::CendreBenite,
        Booster::PoudreDeVifArgent,
    ];

    pub fn nom(self) -> &'static str {
        match self {
            Booster::FioleDeSang => "Fiole de sang",
            Booster::MoelleDeBete => "Moelle de bête",
            Booster::EcorceDeFer => "Écorce de fer",
            Booster::EncreDArcane => "Encre d'arcane",
            Booster::CendreBenite => "Cendre bénite",
            Booster::PoudreDeVifArgent => "Poudre de vif-argent",
        }
    }

    pub fn stat(self) -> Stat {
        match self {
            Booster::FioleDeSang => Stat::Pv,
            Booster::MoelleDeBete => Stat::Attaque,
            Booster::EcorceDeFer => Stat::Defense,
            Booster::EncreDArcane => Stat::AttaqueSpeciale,
            Booster::CendreBenite => Stat::DefenseSpeciale,
            Booster::PoudreDeVifArgent => Stat::Vitesse,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurBooster {
    /// Ni le sac ni le coffre n'en contiennent.
    Aucun(Booster),
    /// La valeur interne visée est déjà à 31 : le booster ne serait pas
    /// consommé pour rien.
    ValeurMaximale(Booster),
    Collection(ErreurCollection),
}

impl fmt::Display for ErreurBooster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurBooster::Aucun(booster) => write!(f, "aucun booster {}", booster.nom()),
            ErreurBooster::ValeurMaximale(booster) => write!(
                f,
                "la valeur interne visée par {} est déjà au maximum",
                booster.nom()
            ),
            ErreurBooster::Collection(erreur) => write!(f, "{erreur}"),
        }
    }
}

impl std::error::Error for ErreurBooster {}
