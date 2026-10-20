//! Héros du joueur (section 4) : un nom, une classe et son équipement. Il
//! ne combat pas et n'a ni niveau ni stats, mais donne un ordre par combat.

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

    pub fn ordre(self) -> Ordre {
        match self {
            Classe::Guerrier => Ordre::CriDeGuerre,
            Classe::Mage => Ordre::Incantation,
            Classe::Templier => Ordre::BouclierDeFoi,
            Classe::Assassin => Ordre::PasDeLOmbre,
        }
    }
}

/// Action de soutien que le héros donne en combat, une fois par combat, au
/// monstre actif (section 4.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordre {
    CriDeGuerre,
    Incantation,
    BouclierDeFoi,
    PasDeLOmbre,
}

/// Les ordres renforcent la stat du bonus de classe.
const POURCENTAGE_D_ORDRE: u8 = 20;

impl Ordre {
    pub fn nom(self) -> &'static str {
        match self {
            Ordre::CriDeGuerre => "Cri de guerre",
            Ordre::Incantation => "Incantation",
            Ordre::BouclierDeFoi => "Bouclier de foi",
            Ordre::PasDeLOmbre => "Pas de l'ombre",
        }
    }

    /// Bonus du monstre qui reçoit l'ordre, jusqu'à la fin du combat.
    pub fn bonus(self) -> BonusDeStats {
        let pourcentage = POURCENTAGE_D_ORDRE;
        match self {
            Ordre::CriDeGuerre => Stats {
                attaque: pourcentage,
                ..Stats::default()
            },
            Ordre::Incantation => Stats {
                attaque_speciale: pourcentage,
                ..Stats::default()
            },
            Ordre::BouclierDeFoi => Stats {
                defense: pourcentage,
                defense_speciale: pourcentage,
                ..Stats::default()
            },
            Ordre::PasDeLOmbre => Stats {
                vitesse: pourcentage,
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

    #[test]
    fn l_ordre_renforce_la_stat_de_classe_de_20_pour_cent() {
        for classe in Classe::TOUTES {
            let classe_bonus = classe.bonus();
            let ordre = classe.ordre().bonus();
            let doubler = |valeur: u8| valeur * 2;
            assert_eq!(
                ordre,
                Stats {
                    pv: doubler(classe_bonus.pv),
                    attaque: doubler(classe_bonus.attaque),
                    defense: doubler(classe_bonus.defense),
                    attaque_speciale: doubler(classe_bonus.attaque_speciale),
                    defense_speciale: doubler(classe_bonus.defense_speciale),
                    vitesse: doubler(classe_bonus.vitesse),
                },
                "{}",
                classe.nom()
            );
        }
    }
}
