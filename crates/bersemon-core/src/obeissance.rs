//! Obéissance des monstres reçus d'un autre joueur (section 15.2) : au-dessus
//! de sa limite, un tel monstre ignore parfois l'ordre d'attaquer.

use crate::monstre::Monstre;
use crate::zone::Zone;

/// Marge au-dessus du niveau maximal de la zone la plus avancée ouverte.
const MARGE_D_OBEISSANCE: u8 = 5;
/// Chances de désobéir, en pourcentage, par niveau au-dessus de la limite.
const POUR_CENT_PAR_NIVEAU: u32 = 10;
const POUR_CENT_MAXIMUM: u32 = 50;

/// Le niveau jusqu'auquel un monstre reçu obéit : 12 avec la seule Lisière.
/// Sans zone ouverte, la marge seule.
pub fn limite_d_obeissance<'a>(zones_ouvertes: impl IntoIterator<Item = &'a Zone>) -> u8 {
    zones_ouvertes
        .into_iter()
        .map(|zone| zone.niveaux().1.valeur())
        .max()
        .unwrap_or(0)
        .saturating_add(MARGE_D_OBEISSANCE)
}

/// Pourcentage de chances que le monstre ignore l'ordre d'attaquer : nul
/// pour un monstre qui n'a pas été reçu ou qui ne dépasse pas la limite.
pub fn chances_de_desobeir(monstre: &Monstre, limite: u8) -> u32 {
    if monstre.recu_de().is_none() {
        return 0;
    }
    let ecart = u32::from(monstre.niveau().valeur().saturating_sub(limite));
    (ecart * POUR_CENT_PAR_NIVEAU).min(POUR_CENT_MAXIMUM)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attaque::Categorie;
    use crate::contenu::ContenuV1;
    use crate::donnees_de_test::{attaque, monstre, stats_uniformes};
    use crate::nom::Nom;
    use crate::types::Type;
    use crate::zone::ZoneId;

    #[test]
    fn la_limite_suit_la_zone_la_plus_avancee() {
        let contenu = ContenuV1::new();
        let zone = |id: &str| {
            contenu
                .zones()
                .iter()
                .find(|zone| zone.id() == &ZoneId::new(id))
                .unwrap()
        };
        assert_eq!(limite_d_obeissance([zone("lisiere")]), 12);
        assert_eq!(limite_d_obeissance([zone("lisiere"), zone("marais")]), 18);
        assert_eq!(limite_d_obeissance([zone("vallee"), zone("lisiere")]), 25);
        assert_eq!(limite_d_obeissance([zone("abbaye")]), 32);
        assert_eq!(limite_d_obeissance([]), 5);
    }

    #[test]
    fn seul_un_monstre_recu_au_dessus_de_sa_limite_desobeit() {
        let mut monstre = monstre(
            &[Type::Feu],
            stats_uniformes(50),
            20,
            vec![attaque("coup", Type::Normal, Categorie::Physique, 40, 100)],
        );
        assert_eq!(chances_de_desobeir(&monstre, 12), 0);
        monstre.marquer_recu(Nom::new("Ysolde").unwrap());
        assert_eq!(chances_de_desobeir(&monstre, 20), 0);
        assert_eq!(chances_de_desobeir(&monstre, 19), 10);
        assert_eq!(chances_de_desobeir(&monstre, 17), 30);
        assert_eq!(chances_de_desobeir(&monstre, 12), 50);
    }
}
