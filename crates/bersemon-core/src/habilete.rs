//! Habiletés des monstres (section 5.9) : un bonus passif propre à chaque
//! monstre, tiré parmi les habiletés possibles de son espèce.
//!
//! Ce module donne leurs valeurs ; chaque règle applique la sienne là où
//! elle s'exerce : stats et XP au combat, dégâts, capture, coffres,
//! expéditions automatiques.

use crate::aleatoire::Aleatoire;
use crate::espece::Espece;
use crate::stats::{BonusDeStats, Stats};
use crate::zone::tirer_entre;

/// Pourcentage des habiletés de stats, qui s'ajoute au bonus de la classe.
const BONUS_DE_STAT: u8 = 10;
/// Dégâts en plus, en pourcentage, des attaques d'Affinité qui ont le bonus
/// de type.
pub(crate) const BONUS_D_AFFINITE: u64 = 20;
/// XP en plus, en pourcentage, pour un monstre qui a Instinct.
pub(crate) const BONUS_D_INSTINCT: u32 = 20;
/// Durée en moins, en pourcentage, d'une expédition automatique avec un
/// Éclaireur.
pub(crate) const REDUCTION_D_ECLAIREUR: u64 = 20;
/// Chance de capture x1,25, gardée en fraction pour rester en calcul entier.
pub(crate) const FACTEUR_DU_TRAQUEUR: (u64, u64) = (5, 4);
/// Argent en plus, en pourcentage, avec un Pillard.
pub(crate) const BONUS_DU_PILLARD: u32 = 50;
/// Un Fouineur trouve une potion après une victoire 1 fois sur 4.
pub(crate) const CHANCES_DU_FOUINEUR: u32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Habilete {
    Ferocite,
    Carapace,
    Sorcellerie,
    Volonte,
    Celerite,
    Affinite,
    Instinct,
    Eclaireur,
    Traqueur,
    Pillard,
    Fouineur,
}

impl Habilete {
    pub const TOUTES: [Habilete; 11] = [
        Habilete::Ferocite,
        Habilete::Carapace,
        Habilete::Sorcellerie,
        Habilete::Volonte,
        Habilete::Celerite,
        Habilete::Affinite,
        Habilete::Instinct,
        Habilete::Eclaireur,
        Habilete::Traqueur,
        Habilete::Pillard,
        Habilete::Fouineur,
    ];

    /// Nom de l'habileté tel qu'il s'écrit dans le jeu, commun à toutes les
    /// interfaces.
    pub fn nom(self) -> &'static str {
        match self {
            Habilete::Ferocite => "Férocité",
            Habilete::Carapace => "Carapace",
            Habilete::Sorcellerie => "Sorcellerie",
            Habilete::Volonte => "Volonté",
            Habilete::Celerite => "Célérité",
            Habilete::Affinite => "Affinité",
            Habilete::Instinct => "Instinct",
            Habilete::Eclaireur => "Éclaireur",
            Habilete::Traqueur => "Traqueur",
            Habilete::Pillard => "Pillard",
            Habilete::Fouineur => "Fouineur",
        }
    }

    /// Pourcentage ajouté à une stat en combat ; nul pour les habiletés qui
    /// ne touchent pas aux stats.
    pub fn bonus_de_stats(self) -> BonusDeStats {
        let bonus = BONUS_DE_STAT;
        match self {
            Habilete::Ferocite => Stats {
                attaque: bonus,
                ..Stats::default()
            },
            Habilete::Carapace => Stats {
                defense: bonus,
                ..Stats::default()
            },
            Habilete::Sorcellerie => Stats {
                attaque_speciale: bonus,
                ..Stats::default()
            },
            Habilete::Volonte => Stats {
                defense_speciale: bonus,
                ..Stats::default()
            },
            Habilete::Celerite => Stats {
                vitesse: bonus,
                ..Stats::default()
            },
            Habilete::Affinite
            | Habilete::Instinct
            | Habilete::Eclaireur
            | Habilete::Traqueur
            | Habilete::Pillard
            | Habilete::Fouineur => Stats::default(),
        }
    }
}

/// Parmi les habiletés possibles de l'espèce ; `None` pour une espèce qui
/// n'en a pas. Aucun tirage quand l'espèce n'en a qu'une.
pub(crate) fn tirer_habilete(espece: &Espece, aleatoire: &mut dyn Aleatoire) -> Option<Habilete> {
    let possibles = espece.habiletes();
    if possibles.is_empty() {
        return None;
    }
    let position = tirer_entre(0, (possibles.len() - 1) as u32, aleatoire) as usize;
    Some(possibles[position])
}

/// `valeur` augmentée de `pourcentage`, arrondie à l'entier inférieur.
pub(crate) fn majorer(valeur: u32, pourcentage: u32) -> u32 {
    let majoree = u64::from(valeur) * (100 + u64::from(pourcentage)) / 100;
    u32::try_from(majoree).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::donnees_de_test::espece;

    #[test]
    fn seules_les_habiletes_de_stats_donnent_un_bonus_de_stats() {
        assert_eq!(Habilete::Ferocite.bonus_de_stats().attaque, 10);
        assert_eq!(Habilete::Celerite.bonus_de_stats().vitesse, 10);
        assert_eq!(Habilete::Instinct.bonus_de_stats(), BonusDeStats::default());
    }

    #[test]
    fn habilete_tiree_parmi_celles_de_l_espece() {
        let deux = espece("loup").avec_habiletes(vec![Habilete::Traqueur, Habilete::Fouineur]);
        assert_eq!(
            tirer_habilete(&deux, &mut AleatoireScripte::new([1])),
            Some(Habilete::Fouineur)
        );
        // Une seule possible : aucun tirage.
        let une = espece("loup").avec_habiletes(vec![Habilete::Pillard]);
        assert_eq!(
            tirer_habilete(&une, &mut AleatoireScripte::new([])),
            Some(Habilete::Pillard)
        );
        assert_eq!(
            tirer_habilete(&espece("loup"), &mut AleatoireScripte::new([])),
            None
        );
    }

    #[test]
    fn majorer_arrondit_a_l_entier_inferieur() {
        assert_eq!(majorer(45, 20), 54);
        assert_eq!(majorer(7, 50), 10);
        assert_eq!(majorer(u32::MAX, 50), u32::MAX);
    }
}
