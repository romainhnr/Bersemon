//! Objets d'expérience (section 14.3) : au camp, ils font gagner de l'XP à
//! un monstre, ou un niveau entier, et se consomment. Malvoise les vend ;
//! achetés, ils vont au coffre, d'où ils servent.

use std::fmt;

use crate::collection::ErreurCollection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjetDExperience {
    Eclat,
    Fiole,
    Cristal,
    /// Un niveau entier, quel que soit le niveau du monstre.
    Coeur,
}

/// Ce que fait un objet d'expérience.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffetDExperience {
    Experience(u32),
    /// Juste assez d'XP pour le niveau suivant.
    Niveau,
}

impl ObjetDExperience {
    /// Dans l'ordre de docs/contenu-v1.md, section 54, et des affichages.
    pub const TOUS: [ObjetDExperience; 4] = [
        ObjetDExperience::Eclat,
        ObjetDExperience::Fiole,
        ObjetDExperience::Cristal,
        ObjetDExperience::Coeur,
    ];

    pub fn nom(self) -> &'static str {
        match self {
            ObjetDExperience::Eclat => "Éclat de mémoire",
            ObjetDExperience::Fiole => "Fiole de mémoire",
            ObjetDExperience::Cristal => "Cristal de mémoire",
            ObjetDExperience::Coeur => "Cœur de bataille",
        }
    }

    /// Valeurs proposées par Claude sans validation (Q123).
    pub fn effet(self) -> EffetDExperience {
        match self {
            ObjetDExperience::Eclat => EffetDExperience::Experience(1_000),
            ObjetDExperience::Fiole => EffetDExperience::Experience(5_000),
            ObjetDExperience::Cristal => EffetDExperience::Experience(20_000),
            ObjetDExperience::Coeur => EffetDExperience::Niveau,
        }
    }

    /// En pièces d'argent, chez Malvoise (hypothèse retenue, Q123).
    pub fn prix(self) -> u32 {
        match self {
            ObjetDExperience::Eclat => 100,
            ObjetDExperience::Fiole => 400,
            ObjetDExperience::Cristal => 1_200,
            ObjetDExperience::Coeur => 1_500,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurObjetDExperience {
    /// Ni le sac ni le coffre n'en contiennent.
    Aucun(ObjetDExperience),
    /// Au niveau 100, il ne gagne plus d'XP (section 7).
    NiveauMaximum,
    /// Un monstre loué ne reçoit rien (section 14.2).
    MonstreLoue,
    Collection(ErreurCollection),
}

impl fmt::Display for ErreurObjetDExperience {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurObjetDExperience::Aucun(objet) => write!(f, "aucun {}", objet.nom()),
            ErreurObjetDExperience::NiveauMaximum => write!(f, "niveau maximum"),
            ErreurObjetDExperience::MonstreLoue => write!(f, "monstre loué"),
            ErreurObjetDExperience::Collection(erreur) => write!(f, "{erreur}"),
        }
    }
}

impl std::error::Error for ErreurObjetDExperience {}
