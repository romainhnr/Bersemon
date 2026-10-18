//! Expérience des monstres : courbe de niveau et gain après un combat
//! (section 7).

use crate::aleatoire::Aleatoire;
use crate::niveau::Niveau;

/// Fourchette du pourcentage appliqué au gain d'XP (Q32).
const POURCENTAGE_MIN: u32 = 90;
const POURCENTAGE_MAX: u32 = 110;

/// XP totale nécessaire pour atteindre ce niveau : n x n x n, la même courbe
/// pour toutes les espèces (Q10).
pub fn experience_requise(niveau: Niveau) -> u32 {
    let niveau = u32::from(niveau.valeur());
    niveau * niveau * niveau
}

/// XP rapportée par un monstre vaincu : rendement x niveau / 7, multiplié par
/// un pourcentage tiré de 90 % à 110 % (Q32). Aucun tirage si le gain de base
/// est nul.
pub fn tirer_gain_experience(
    rendement: u16,
    niveau_vaincu: Niveau,
    aleatoire: &mut dyn Aleatoire,
) -> u32 {
    let base = u32::from(rendement) * u32::from(niveau_vaincu.valeur()) / 7;
    if base == 0 {
        return 0;
    }
    base * aleatoire.tirer(POURCENTAGE_MIN..=POURCENTAGE_MAX) / 100
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;

    fn niveau(valeur: u8) -> Niveau {
        Niveau::new(valeur).unwrap()
    }

    #[test]
    fn courbe_cubique() {
        assert_eq!(experience_requise(niveau(1)), 1);
        assert_eq!(experience_requise(niveau(5)), 125);
        assert_eq!(experience_requise(niveau(6)), 216);
        assert_eq!(experience_requise(niveau(100)), 1_000_000);
    }

    #[test]
    fn exemples_de_reference_du_gain() {
        // Exemples consignés avec la décision Q32, rendement 64.
        let cas = [(5, 90, 40), (5, 110, 49), (50, 90, 411), (50, 110, 502)];
        for (niveau_vaincu, pourcentage, gain) in cas {
            let mut aleatoire = AleatoireScripte::new([pourcentage]);
            assert_eq!(
                tirer_gain_experience(64, niveau(niveau_vaincu), &mut aleatoire),
                gain,
                "niveau {niveau_vaincu}, pourcentage {pourcentage}"
            );
        }
    }

    #[test]
    fn rendement_nul_sans_tirage() {
        let mut aleatoire = AleatoireScripte::new([]);
        assert_eq!(tirer_gain_experience(0, niveau(50), &mut aleatoire), 0);
    }

    #[test]
    fn gain_maximal_sans_debordement() {
        let mut aleatoire = AleatoireScripte::new([110]);
        assert_eq!(
            tirer_gain_experience(u16::MAX, niveau(100), &mut aleatoire),
            u32::from(u16::MAX) * 100 / 7 * 110 / 100
        );
    }
}
