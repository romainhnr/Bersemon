//! Effets de statut de la V1, avec les règles actuelles de Pokémon (section
//! 10.5).

use crate::types::{Type, TypesMonstre};

/// Un paralysé a 1 chance sur 4 de ne pas pouvoir attaquer.
pub(crate) const CHANCES_BLOCAGE_PARALYSIE: u32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Statut {
    Poison,
    Paralysie,
}

impl Statut {
    /// Les types Poison et Acier ne peuvent pas être empoisonnés, le type
    /// Électrique ne peut pas être paralysé.
    pub fn epargne(self, types: &TypesMonstre) -> bool {
        let immunises: &[Type] = match self {
            Statut::Poison => &[Type::Poison, Type::Acier],
            Statut::Paralysie => &[Type::Electrique],
        };
        types
            .liste()
            .iter()
            .any(|type_monstre| immunises.contains(type_monstre))
    }
}

/// Perte de PV du poison à la fin d'un tour : 1/8 des PV max, au moins 1.
pub(crate) fn degats_de_poison(pv_max: u32) -> u32 {
    (pv_max / 8).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn types(liste: &[Type]) -> TypesMonstre {
        TypesMonstre::new(liste).unwrap()
    }

    #[test]
    fn immunites_au_poison() {
        assert!(Statut::Poison.epargne(&types(&[Type::Poison])));
        assert!(Statut::Poison.epargne(&types(&[Type::Feu, Type::Acier])));
        assert!(!Statut::Poison.epargne(&types(&[Type::Feu])));
    }

    #[test]
    fn immunite_a_la_paralysie() {
        assert!(Statut::Paralysie.epargne(&types(&[Type::Eau, Type::Electrique])));
        assert!(!Statut::Paralysie.epargne(&types(&[Type::Poison])));
    }

    #[test]
    fn poison_retire_un_huitieme_des_pv_max() {
        assert_eq!(degats_de_poison(20), 2);
        assert_eq!(degats_de_poison(241), 30);
        assert_eq!(degats_de_poison(7), 1);
    }
}
