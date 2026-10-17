//! Chance de capture d'un monstre sauvage au filet (section 11.2).

use crate::monstre::Monstre;
use crate::niveau::Niveau;

/// Pourcentage de 0 à 100 : PV perdus / PV max x facteur de niveau (Q28). Un
/// monstre KO est capturé à coup sûr ; un monstre à PV pleins, jamais.
pub fn calculer_chance_capture(monstre_actif: &Monstre, sauvage: &Monstre) -> u32 {
    if sauvage.est_ko() {
        return 100;
    }
    let pv_max = u64::from(sauvage.stats().pv);
    let pv_perdus = pv_max.saturating_sub(u64::from(sauvage.pv_actuels()));
    let (numerateur, denominateur) = facteur_de_niveau(monstre_actif.niveau(), sauvage.niveau());
    let chance = pv_perdus * 100 * numerateur / (pv_max * denominateur);
    // Au plus 100, donc tient dans un u32.
    chance.min(100) as u32
}

/// Niveau du monstre actif divisé par celui du sauvage, borné entre x0,5 et
/// x2. Gardé sous forme de fraction pour rester en calcul entier.
fn facteur_de_niveau(actif: Niveau, sauvage: Niveau) -> (u64, u64) {
    let actif = u64::from(actif.valeur());
    let sauvage = u64::from(sauvage.valeur());
    if actif > 2 * sauvage {
        (2, 1)
    } else if 2 * actif < sauvage {
        (1, 2)
    } else {
        (actif, sauvage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attaque::Categorie;
    use crate::donnees_de_test::{attaque, monstre, stats_uniformes};
    use crate::stats::Stats;
    use crate::types::Type;

    /// Base 50 et valeurs internes nulles : 20 PV au niveau 5. Au niveau 10,
    /// une base de PV nulle donne aussi 20 PV.
    fn monstre_a_20_pv(niveau: u8, pv_restants: u32) -> Monstre {
        let base = if niveau == 5 {
            stats_uniformes(50)
        } else {
            Stats {
                pv: 0,
                ..stats_uniformes(50)
            }
        };
        let coup = attaque("coup", Type::Normal, Categorie::Physique, 40, 100);
        let mut monstre = monstre(&[Type::Feu], base, niveau, vec![coup]);
        assert_eq!(monstre.stats().pv, 20);
        monstre.subir_degats(20 - pv_restants);
        monstre
    }

    #[test]
    fn exemples_de_reference() {
        // Exemples consignés avec la décision Q28.
        let cas = [
            (5, 5, 20, 0),
            (5, 5, 10, 50),
            (5, 5, 2, 90),
            (10, 5, 15, 50),
            (10, 5, 10, 100),
            (5, 10, 2, 45),
        ];
        for (niveau_actif, niveau_sauvage, pv_restants, chance) in cas {
            let actif = monstre_a_20_pv(niveau_actif, 20);
            let sauvage = monstre_a_20_pv(niveau_sauvage, pv_restants);
            assert_eq!(
                calculer_chance_capture(&actif, &sauvage),
                chance,
                "actif niv. {niveau_actif}, sauvage niv. {niveau_sauvage} à {pv_restants} PV"
            );
        }
    }

    #[test]
    fn monstre_ko_capture_a_coup_sur() {
        let actif = monstre_a_20_pv(5, 20);
        let sauvage = monstre_a_20_pv(5, 0);
        assert_eq!(calculer_chance_capture(&actif, &sauvage), 100);
    }
}
