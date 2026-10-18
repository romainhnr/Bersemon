//! Expéditions automatiques (section 12) : départ, rappel, retour et durées.

use std::time::Duration;

use bersemon_core::expedition_automatique::{BilanExpeditionAutomatique, ExpeditionAutomatique};
use bersemon_core::monstre::Monstre;

use crate::partie::potions;

/// Arrondie à la minute supérieure, pour qu'un temps restant ne s'affiche
/// jamais à 0 avant le retour.
pub fn duree(duree: Duration) -> String {
    let minutes = duree.as_secs().div_ceil(60);
    match (minutes / 60, minutes % 60) {
        (0, 0) => "moins d'une minute".to_string(),
        (0, minutes) => format!("{minutes} min"),
        (heures, 0) => format!("{heures} h"),
        (heures, minutes) => format!("{heures} h {minutes:02}"),
    }
}

pub fn depart(expedition: &ExpeditionAutomatique) -> String {
    let monstres = expedition.monstres();
    format!(
        "{} {} pour {}, sans vous. Retour prévu dans {}.",
        noms(monstres),
        if monstres.len() > 1 {
            "partent"
        } else {
            "part"
        },
        expedition.zone().nom(),
        duree(expedition.depart().duree_jusqu_a(expedition.retour()))
    )
}

pub fn rappel(expedition: &ExpeditionAutomatique) -> String {
    let monstres = expedition.monstres();
    format!(
        "Vous rappelez l'expédition partie pour {} : {} {} au camp sans rien rapporter.",
        expedition.zone().nom(),
        noms(monstres),
        if monstres.len() > 1 {
            "rentrent"
        } else {
            "rentre"
        }
    )
}

pub fn question_de_rappel(expedition: &ExpeditionAutomatique) -> String {
    format!(
        "Rappeler l'expédition partie pour {} ? Elle ne rapportera rien.",
        expedition.zone().nom()
    )
}

/// Le bilan, une ligne par monstre ; l'XP gagnée suit dans son propre
/// récit.
pub fn retour(zone: &str, bilan: &BilanExpeditionAutomatique) -> String {
    let mut lignes = vec![format!(
        "L'expédition partie pour {zone} est de retour : elle a dû encaisser {} dégâts.",
        bilan.degats
    )];
    for rentre in &bilan.monstres {
        let ko = if rentre.ko { " et tombe KO" } else { "" };
        lignes.push(format!(
            "{} perd {} PV{ko}.",
            rentre.monstre.nom(),
            rentre.degats_subis
        ));
    }
    let conclusion = if !bilan.reussite {
        "Échec : tous vos monstres sont tombés KO, l'expédition ne rapporte rien.".to_string()
    } else if bilan.potions == 0 {
        "Réussite, mais vos monstres ne rapportent aucune potion.".to_string()
    } else {
        format!(
            "Réussite : vos monstres rapportent {}.",
            potions(bilan.potions)
        )
    };
    lignes.push(conclusion);
    lignes.push("Ils rentrent au camp et sont soignés.".to_string());
    lignes.join("\n")
}

/// "a", "a et b", "a, b et c".
fn noms(monstres: &[Monstre]) -> String {
    let noms: Vec<&str> = monstres.iter().map(Monstre::nom).collect();
    match noms.split_last() {
        None => String::new(),
        Some((dernier, [])) => dernier.to_string(),
        Some((dernier, autres)) => format!("{} et {dernier}", autres.join(", ")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durees_arrondies_a_la_minute_superieure() {
        assert_eq!(duree(Duration::from_secs(3_600)), "1 h");
        assert_eq!(duree(Duration::from_secs(5_400)), "1 h 30");
        assert_eq!(duree(Duration::from_secs(3_601)), "1 h 01");
        assert_eq!(duree(Duration::from_secs(2_700)), "45 min");
        assert_eq!(duree(Duration::from_secs(1)), "1 min");
        assert_eq!(duree(Duration::ZERO), "moins d'une minute");
    }
}
