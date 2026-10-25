//! Le cycle du jour et de la nuit (section 9.6) : 2 heures de jour puis 1
//! heure de nuit, en temps réel, les mêmes pour tous les joueurs. Il ne
//! joue que dans les expéditions manuelles des zones de la carte, qui
//! gardent le moment de leur départ.

use std::time::Duration;

use crate::horodatage::Horodatage;
use crate::types::Type;

const DUREE_DU_JOUR: u64 = 2 * 3_600;
const DUREE_DE_LA_NUIT: u64 = 3_600;
const DUREE_DU_CYCLE: u64 = DUREE_DU_JOUR + DUREE_DE_LA_NUIT;

/// De nuit, les monstres des groupes sauvages et les monstres rares ont ces
/// niveaux de plus que ceux tirés pour la zone.
pub const NIVEAUX_DE_PLUS_LA_NUIT: u8 = 2;
/// De nuit, l'XP des monstres vaincus et l'argent des coffres augmentent de
/// ce pourcentage.
pub const BONUS_DE_LA_NUIT: u32 = 25;
/// Les attaques des types renforcés au moment du combat font ce pourcentage
/// de dégâts en plus.
pub const BONUS_DES_TYPES_DU_MOMENT: u64 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JourOuNuit {
    Jour,
    Nuit,
}

impl JourOuNuit {
    /// Le cycle part de minuit UTC : le jour d'abord, puis la nuit
    /// (hypothèse retenue, Q109).
    pub fn a(maintenant: Horodatage) -> Self {
        if maintenant.secondes() % DUREE_DU_CYCLE < DUREE_DU_JOUR {
            JourOuNuit::Jour
        } else {
            JourOuNuit::Nuit
        }
    }

    /// Le temps qui reste avant que la nuit tombe, ou avant l'aube.
    pub fn duree_restante(maintenant: Horodatage) -> Duration {
        let dans_le_cycle = maintenant.secondes() % DUREE_DU_CYCLE;
        let fin = if dans_le_cycle < DUREE_DU_JOUR {
            DUREE_DU_JOUR
        } else {
            DUREE_DU_CYCLE
        };
        Duration::from_secs(fin - dans_le_cycle)
    }

    /// Les attaques de ces types font 20 % de dégâts en plus : Ténèbres,
    /// Spectre et Poison de nuit, Lumière, Feu et Plante de jour.
    pub fn renforce(self, type_attaque: Type) -> bool {
        match self {
            JourOuNuit::Jour => matches!(type_attaque, Type::Lumiere | Type::Feu | Type::Plante),
            JourOuNuit::Nuit => {
                matches!(type_attaque, Type::Tenebres | Type::Spectre | Type::Poison)
            }
        }
    }
}

/// Une espèce nocturne ne sort que la nuit, une diurne que le jour, dans les
/// groupes sauvages des zones ; les autres sortent à toute heure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Activite {
    Diurne,
    Nocturne,
}

impl Activite {
    pub fn sort(self, jour_ou_nuit: JourOuNuit) -> bool {
        match self {
            Activite::Diurne => jour_ou_nuit == JourOuNuit::Jour,
            Activite::Nocturne => jour_ou_nuit == JourOuNuit::Nuit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_l_heure(heures: u64, minutes: u64) -> Horodatage {
        Horodatage::depuis_secondes(heures * 3_600 + minutes * 60)
    }

    #[test]
    fn deux_heures_de_jour_puis_une_heure_de_nuit() {
        assert_eq!(JourOuNuit::a(a_l_heure(0, 0)), JourOuNuit::Jour);
        assert_eq!(JourOuNuit::a(a_l_heure(1, 59)), JourOuNuit::Jour);
        assert_eq!(JourOuNuit::a(a_l_heure(2, 0)), JourOuNuit::Nuit);
        assert_eq!(JourOuNuit::a(a_l_heure(2, 59)), JourOuNuit::Nuit);
        assert_eq!(JourOuNuit::a(a_l_heure(3, 0)), JourOuNuit::Jour);
        // Le cycle se répète sans fin : 23 h UTC est une heure de nuit.
        assert_eq!(
            JourOuNuit::a(a_l_heure(24 * 400 + 23, 30)),
            JourOuNuit::Nuit
        );
    }

    #[test]
    fn le_temps_restant_mene_au_moment_suivant() {
        assert_eq!(
            JourOuNuit::duree_restante(a_l_heure(0, 48)),
            Duration::from_secs(72 * 60)
        );
        assert_eq!(
            JourOuNuit::duree_restante(a_l_heure(2, 37)),
            Duration::from_secs(23 * 60)
        );
        let veille_de_l_aube = Horodatage::depuis_secondes(3 * 3_600 - 1);
        assert_eq!(
            JourOuNuit::duree_restante(veille_de_l_aube),
            Duration::from_secs(1)
        );
        let aube = veille_de_l_aube.plus(Duration::from_secs(1));
        assert_eq!(JourOuNuit::a(aube), JourOuNuit::Jour);
        assert_eq!(
            JourOuNuit::duree_restante(aube),
            Duration::from_secs(2 * 3_600)
        );
    }

    #[test]
    fn chaque_moment_renforce_trois_types() {
        let de_nuit: Vec<Type> = Type::TOUS
            .into_iter()
            .filter(|type_attaque| JourOuNuit::Nuit.renforce(*type_attaque))
            .collect();
        assert_eq!(de_nuit, vec![Type::Poison, Type::Spectre, Type::Tenebres]);
        let de_jour: Vec<Type> = Type::TOUS
            .into_iter()
            .filter(|type_attaque| JourOuNuit::Jour.renforce(*type_attaque))
            .collect();
        assert_eq!(de_jour, vec![Type::Feu, Type::Plante, Type::Lumiere]);
    }

    #[test]
    fn une_espece_nocturne_ne_sort_que_la_nuit() {
        assert!(Activite::Nocturne.sort(JourOuNuit::Nuit));
        assert!(!Activite::Nocturne.sort(JourOuNuit::Jour));
        assert!(Activite::Diurne.sort(JourOuNuit::Jour));
        assert!(!Activite::Diurne.sort(JourOuNuit::Nuit));
    }
}
