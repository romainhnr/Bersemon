//! La météo des zones de la carte (section 9.7) : des créneaux de 30
//! minutes réelles, les mêmes pour tous les joueurs ; à chacun, chaque zone a
//! une chance sur 3 d'avoir une de ses météos. Le tirage ne dépend que du
//! créneau et de la zone, pour que tous voient le même ciel sans rien
//! sauvegarder.

use std::time::Duration;

use crate::aleatoire::{Aleatoire, GenerateurAleatoire};
use crate::horodatage::Horodatage;
use crate::types::Type;
use crate::zone::Zone;

const DUREE_D_UN_CRENEAU: u64 = 30 * 60;
/// Une chance sur 3 qu'un créneau ait une météo.
const CHANCES_D_UNE_METEO: u32 = 3;
/// Le temps restant ne se cherche pas au-delà d'une journée de créneaux.
const CRENEAUX_REGARDES: u64 = 48;

/// Les attaques des types renforcés par la météo font ce pourcentage de
/// dégâts en plus, après le bonus du moment du jour.
pub const BONUS_DE_LA_METEO: u64 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Meteo {
    Pluie,
    Tempete,
    Grele,
    /// Seulement dans un désert.
    TempeteDeSable,
    /// Seulement dans une zone de mer.
    Cyclone,
    Canicule,
}

impl Meteo {
    pub const TOUTES: [Meteo; 6] = [
        Meteo::Pluie,
        Meteo::Tempete,
        Meteo::Grele,
        Meteo::TempeteDeSable,
        Meteo::Cyclone,
        Meteo::Canicule,
    ];

    /// Deux types par météo, dont les attaques font 20 % de dégâts en plus.
    pub fn renforce(self, type_attaque: Type) -> bool {
        let types: [Type; 2] = match self {
            Meteo::Pluie => [Type::Eau, Type::Plante],
            Meteo::Tempete => [Type::Electrique, Type::Vol],
            Meteo::Grele => [Type::Glace, Type::Roche],
            Meteo::TempeteDeSable => [Type::Sol, Type::Roche],
            Meteo::Cyclone => [Type::Eau, Type::Vol],
            Meteo::Canicule => [Type::Feu, Type::Sol],
        };
        types.contains(&type_attaque)
    }

    /// La météo de la zone à cet instant ; `None` par temps calme, ou dans
    /// une zone sans météo. Tirages, sur un générateur propre au créneau et à
    /// la zone : la chance d'une météo, puis laquelle.
    pub fn de_la_zone(zone: &Zone, maintenant: Horodatage) -> Option<Meteo> {
        Self::du_creneau(zone, maintenant.secondes() / DUREE_D_UN_CRENEAU)
    }

    /// Jusqu'au changement de météo de la zone, sur les créneaux suivants
    /// s'ils tirent la même (hypothèse retenue, Q110), sans regarder plus
    /// d'une journée en avant.
    pub fn duree_restante(zone: &Zone, maintenant: Horodatage) -> Duration {
        let creneau = maintenant.secondes() / DUREE_D_UN_CRENEAU;
        let meteo = Self::du_creneau(zone, creneau);
        let suivants = (1..CRENEAUX_REGARDES)
            .take_while(|ecart| Self::du_creneau(zone, creneau + ecart) == meteo)
            .count() as u64;
        let fin = (creneau + 1 + suivants) * DUREE_D_UN_CRENEAU;
        Duration::from_secs(fin - maintenant.secondes())
    }

    fn du_creneau(zone: &Zone, creneau: u64) -> Option<Meteo> {
        let meteos = zone.meteos();
        if meteos.is_empty() {
            return None;
        }
        let mut aleatoire = GenerateurAleatoire::new(graine(creneau, zone.id().as_str()));
        if aleatoire.tirer(1..=CHANCES_D_UNE_METEO) != 1 {
            return None;
        }
        let position = aleatoire.tirer(0..=(meteos.len() - 1) as u32) as usize;
        Some(meteos[position])
    }
}

/// FNV-1a de l'identifiant de la zone, mêlé au créneau : deux zones n'ont pas
/// le même ciel au même moment, et la graine ne dépend ni de la plateforme ni
/// de la version de Rust.
fn graine(creneau: u64, zone: &str) -> u64 {
    let empreinte = zone
        .bytes()
        .fold(0xCBF2_9CE4_8422_2325_u64, |empreinte, octet| {
            (empreinte ^ u64::from(octet)).wrapping_mul(0x0100_0000_01B3)
        });
    empreinte ^ creneau.wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::donnees_de_test::{espece, parametres_coffre, parametres_expedition_automatique};
    use crate::niveau::Niveau;
    use crate::zone::ZoneId;

    fn zone(id: &str, meteos: Vec<Meteo>) -> Zone {
        let niveau = Niveau::new(5).unwrap();
        Zone::new(
            ZoneId::new(id),
            id,
            (niveau, niveau),
            4,
            1,
            vec![espece("loup")],
            vec![],
            parametres_expedition_automatique(),
            parametres_coffre(),
        )
        .unwrap()
        .avec_meteos(meteos)
    }

    fn a(creneau: u64) -> Horodatage {
        Horodatage::depuis_secondes(creneau * DUREE_D_UN_CRENEAU)
    }

    #[test]
    fn chaque_meteo_renforce_deux_types() {
        for meteo in Meteo::TOUTES {
            let renforces = Type::TOUS
                .into_iter()
                .filter(|type_attaque| meteo.renforce(*type_attaque))
                .count();
            assert_eq!(renforces, 2, "{meteo:?}");
        }
        assert!(Meteo::Pluie.renforce(Type::Eau));
        assert!(Meteo::Canicule.renforce(Type::Feu));
        assert!(!Meteo::Canicule.renforce(Type::Eau));
    }

    #[test]
    fn une_zone_sans_meteo_reste_au_calme() {
        let calme = zone("calme", vec![]);
        assert!((0..200).all(|creneau| Meteo::de_la_zone(&calme, a(creneau)).is_none()));
    }

    /// Sur une longue période, environ un créneau sur 3 a une météo, tirée
    /// parmi celles de la zone ; le tirage se refait à l'identique.
    #[test]
    fn un_creneau_sur_trois_tire_une_meteo_de_la_zone() {
        let meteos = vec![Meteo::Pluie, Meteo::Tempete, Meteo::Canicule];
        let lisiere = zone("lisiere", meteos.clone());
        let tirees: Vec<Option<Meteo>> = (0..3_000)
            .map(|creneau| Meteo::de_la_zone(&lisiere, a(creneau)))
            .collect();
        let avec_meteo = tirees.iter().flatten().count();
        assert!((900..1_100).contains(&avec_meteo), "{avec_meteo}");
        assert!(tirees.iter().flatten().all(|meteo| meteos.contains(meteo)));
        for meteo in meteos {
            assert!(tirees.contains(&Some(meteo)), "{meteo:?}");
        }
        // Le même créneau donne la même météo, du début à la fin.
        let fin_du_creneau = Horodatage::depuis_secondes(DUREE_D_UN_CRENEAU - 1);
        assert_eq!(
            Meteo::de_la_zone(&lisiere, fin_du_creneau),
            Meteo::de_la_zone(&lisiere, a(0))
        );
    }

    #[test]
    fn deux_zones_n_ont_pas_le_meme_ciel() {
        let meteos = vec![Meteo::Pluie, Meteo::Tempete, Meteo::Canicule];
        let lisiere = zone("lisiere", meteos.clone());
        let marais = zone("marais", meteos);
        let differents = (0..300)
            .filter(|creneau| {
                Meteo::de_la_zone(&lisiere, a(*creneau)) != Meteo::de_la_zone(&marais, a(*creneau))
            })
            .count();
        assert!(differents > 50, "{differents}");
    }

    /// Le temps restant court jusqu'au premier créneau où la météo change.
    #[test]
    fn le_temps_restant_va_jusqu_au_changement() {
        let lisiere = zone("lisiere", vec![Meteo::Pluie]);
        for creneau in 0..300 {
            let maintenant = Horodatage::depuis_secondes(creneau * DUREE_D_UN_CRENEAU + 600);
            let meteo = Meteo::de_la_zone(&lisiere, maintenant);
            let restant = Meteo::duree_restante(&lisiere, maintenant);
            let fin = maintenant.plus(restant);
            assert!(restant.as_secs() >= DUREE_D_UN_CRENEAU - 600);
            assert_eq!(
                Meteo::de_la_zone(&lisiere, Horodatage::depuis_secondes(fin.secondes() - 1)),
                meteo
            );
            assert_ne!(Meteo::de_la_zone(&lisiere, fin), meteo);
        }
    }
}
