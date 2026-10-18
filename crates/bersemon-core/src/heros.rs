//! Héros du joueur (section 4). Dans la V1, il se résume à un nom et une
//! classe : il ne combat pas et n'a ni niveau, ni stats, ni équipement.

use crate::nom::Nom;
use crate::stats::{BonusDeStats, Stats};

/// Classes de la V1 (Q2). Templier et Assassin viendront après la V1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Classe {
    Guerrier,
    Mage,
}

impl Classe {
    pub const TOUTES: [Classe; 2] = [Classe::Guerrier, Classe::Mage];

    pub fn nom(self) -> &'static str {
        match self {
            Classe::Guerrier => "Guerrier",
            Classe::Mage => "Mage",
        }
    }

    /// +10 % dans une stat des monstres du joueur en combat (Q34).
    pub fn bonus(self) -> BonusDeStats {
        match self {
            Classe::Guerrier => Stats {
                attaque: 10,
                ..Stats::default()
            },
            Classe::Mage => Stats {
                attaque_speciale: 10,
                ..Stats::default()
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heros {
    nom: Nom,
    classe: Classe,
}

impl Heros {
    pub fn new(nom: Nom, classe: Classe) -> Self {
        Self { nom, classe }
    }

    pub fn nom(&self) -> &Nom {
        &self.nom
    }

    pub fn classe(&self) -> Classe {
        self.classe
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaque_classe_renforce_sa_stat() {
        assert_eq!(Classe::Guerrier.bonus().attaque, 10);
        assert_eq!(Classe::Guerrier.bonus().attaque_speciale, 0);
        assert_eq!(Classe::Mage.bonus().attaque_speciale, 10);
        assert_eq!(Classe::Mage.bonus().attaque, 0);
    }
}
