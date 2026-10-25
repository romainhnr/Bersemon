//! Les textes de la météo (section 9.7, docs/contenu-v1.md, section 32).

use bersemon_core::horodatage::Horodatage;
use bersemon_core::meteo::Meteo;
use bersemon_core::zone::Zone;

use crate::expeditions_automatiques::duree;

/// "pluie", "tempête de sable" : en minuscules, au fil d'une phrase.
pub fn nom(meteo: Meteo) -> &'static str {
    match meteo {
        Meteo::Pluie => "pluie",
        Meteo::Tempete => "tempête",
        Meteo::Grele => "grêle",
        Meteo::TempeteDeSable => "tempête de sable",
        Meteo::Cyclone => "cyclone",
        Meteo::Canicule => "canicule",
    }
}

/// Après le départ, et le texte du moment du jour.
pub fn au_depart(meteo: Meteo) -> &'static str {
    match meteo {
        Meteo::Pluie => "Une pluie froide tombe sans relâche.",
        Meteo::Tempete => "L'orage gronde : le ciel se déchire d'éclairs.",
        Meteo::Grele => "La grêle crépite sur les pierres et les armures.",
        Meteo::TempeteDeSable => "Le sable vole, cinglant comme une lame.",
        Meteo::Cyclone => "Le cyclone hurle, et les vagues montent à l'assaut des côtes.",
        Meteo::Canicule => "Une chaleur écrasante pèse sur la terre.",
    }
}

/// Sur la carte : "pluie, encore 42 min" ; `None` par temps calme.
pub fn meteo_de_la_zone(zone: &Zone, maintenant: Horodatage) -> Option<String> {
    let meteo = Meteo::de_la_zone(zone, maintenant)?;
    Some(format!(
        "{}, encore {}",
        nom(meteo),
        duree(Meteo::duree_restante(zone, maintenant))
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::contenu::ContenuV1;

    #[test]
    fn la_carte_dit_la_meteo_et_sa_duree() {
        let contenu = ContenuV1::new();
        let lisiere = &contenu.zones()[0];
        let (maintenant, meteo) = (0..)
            .map(|creneau| Horodatage::depuis_secondes(creneau * 30 * 60 + 18 * 60))
            .find_map(|maintenant| {
                Meteo::de_la_zone(lisiere, maintenant).map(|meteo| (maintenant, meteo))
            })
            .unwrap();
        let texte = meteo_de_la_zone(lisiere, maintenant).unwrap();
        assert!(
            texte.starts_with(&format!("{}, encore ", nom(meteo))),
            "{texte}"
        );
        // 12 minutes au moins : la fin du créneau, ou plus si la même suit.
        let restant = Meteo::duree_restante(lisiere, maintenant);
        assert!(restant.as_secs() >= 12 * 60);
        assert!(texte.ends_with(&duree(restant)), "{texte}");
    }
}
