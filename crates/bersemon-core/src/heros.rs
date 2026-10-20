//! Héros du joueur (section 4) : un nom, une classe et son équipement. Il
//! ne combat pas et n'a ni niveau ni stats.

use crate::equipement::Equipement;
use crate::nom::Nom;
use crate::stats::{BonusDeStats, Stats, cumuler_bonus};

/// Guerrier et Mage dans la V1 (Q2), puis le Templier et l'Assassin
/// (section 4.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Classe {
    Guerrier,
    Mage,
    Templier,
    Assassin,
}

impl Classe {
    pub const TOUTES: [Classe; 4] = [
        Classe::Guerrier,
        Classe::Mage,
        Classe::Templier,
        Classe::Assassin,
    ];

    pub fn nom(self) -> &'static str {
        match self {
            Classe::Guerrier => "Guerrier",
            Classe::Mage => "Mage",
            Classe::Templier => "Templier",
            Classe::Assassin => "Assassin",
        }
    }

    /// +10 % dans une stat des monstres du joueur en combat (Q34), dans les
    /// deux défenses pour le Templier (section 4.2).
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
            Classe::Templier => Stats {
                defense: 10,
                defense_speciale: 10,
                ..Stats::default()
            },
            Classe::Assassin => Stats {
                vitesse: 10,
                ..Stats::default()
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heros {
    nom: Nom,
    classe: Classe,
    equipement: Equipement,
}

impl Heros {
    pub fn new(nom: Nom, classe: Classe) -> Self {
        Self {
            nom,
            classe,
            equipement: Equipement::default(),
        }
    }

    /// Reprise d'une sauvegarde (section 13.9).
    pub fn avec_equipement(mut self, equipement: Equipement) -> Self {
        self.equipement = equipement;
        self
    }

    pub fn nom(&self) -> &Nom {
        &self.nom
    }

    pub fn classe(&self) -> Classe {
        self.classe
    }

    pub fn equipement(&self) -> &Equipement {
        &self.equipement
    }

    pub(crate) fn equipement_mut(&mut self) -> &mut Equipement {
        &mut self.equipement
    }

    /// Bonus des monstres du joueur en combat : celui de la classe, plus
    /// celui des objets portés (sections 4.2 et 4.3).
    pub fn bonus_en_combat(&self) -> BonusDeStats {
        cumuler_bonus(&self.classe.bonus(), &self.equipement.bonus())
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
        let templier = Classe::Templier.bonus();
        assert_eq!((templier.defense, templier.defense_speciale), (10, 10));
        assert_eq!((templier.attaque, templier.attaque_speciale), (0, 0));
        let assassin = Classe::Assassin.bonus();
        assert_eq!(assassin.vitesse, 10);
        assert_eq!((assassin.attaque, assassin.defense), (0, 0));
    }
}
