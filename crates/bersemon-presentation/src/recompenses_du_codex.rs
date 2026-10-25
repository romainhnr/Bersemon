//! Les récompenses de la complétion du codex (section 5.6) : la progression
//! montrée au codex, la récompense à réclamer et son récit. Textes dans
//! docs/contenu-v1.md, section 30.

use bersemon_core::contenu::ContenuV1;
use bersemon_core::partie::Partie;
use bersemon_core::recompenses_du_codex::{
    ErreurCodex, PalierDuCodex, RecompenseDuCodex, especes_au_codex, especes_capturees,
};

use crate::equipement::objet_et_bonus;
use crate::partie::contenu as objets;

/// "10 espèces capturées", "le codex complet".
pub fn palier(palier: PalierDuCodex) -> String {
    match palier {
        PalierDuCodex::Especes(nombre) => format!("{nombre} espèces capturées"),
        PalierDuCodex::Complet => "le codex complet".to_string(),
    }
}

/// "10 filets lestés et 300 pièces d'argent", et l'objet unique.
pub fn recompense(recompense: &RecompenseDuCodex) -> String {
    match &recompense.objet_unique {
        None => objets(&recompense.objets),
        Some(objet) => format!(
            "{} et le talisman {}",
            objets(&recompense.objets),
            objet_et_bonus(objet)
        ),
    }
}

/// Sous la liste du codex : le compte des espèces capturées, puis la
/// récompense à réclamer, ou celle du prochain palier.
pub fn progression_du_codex(partie: &Partie, contenu: &ContenuV1) -> String {
    let especes = contenu.especes();
    let recompenses = contenu.recompenses_du_codex();
    let compte = format!(
        "Espèces capturées : {} sur {}.",
        especes_capturees(partie.codex(), especes),
        especes_au_codex(especes)
    );
    if let Some(a_reclamer) = partie.recompense_a_reclamer(recompenses, especes) {
        return format!(
            "{compte} Récompense à réclamer pour {} : {}.",
            palier(a_reclamer.palier),
            recompense(a_reclamer)
        );
    }
    match partie.prochaine_recompense(recompenses, especes) {
        Some(prochaine) => format!(
            "{compte} Prochaine récompense, {} : {}.",
            au_palier(prochaine.palier),
            recompense(prochaine)
        ),
        None => format!("{compte} Toutes les récompenses du codex sont réclamées."),
    }
}

/// "à 10 espèces capturées", "au codex complet".
fn au_palier(palier: PalierDuCodex) -> String {
    match palier {
        PalierDuCodex::Especes(nombre) => format!("à {nombre} espèces capturées"),
        PalierDuCodex::Complet => "au codex complet".to_string(),
    }
}

/// Le libellé de l'action, dans la console comme sur un bouton Discord.
pub fn reclamer(recompense: &RecompenseDuCodex) -> String {
    format!("Réclamer la récompense : {}", palier(recompense.palier))
}

pub fn recompense_reclamee(recompense: &RecompenseDuCodex) -> String {
    let intitule = match recompense.palier {
        PalierDuCodex::Especes(nombre) => {
            format!("Récompense du codex, à {nombre} espèces capturées")
        }
        PalierDuCodex::Complet => "Récompense du codex complet".to_string(),
    };
    let au_coffre = format!("{intitule}, au coffre : {}.", objets(&recompense.objets));
    match &recompense.objet_unique {
        None => au_coffre,
        Some(objet) => format!(
            "{au_coffre} Le talisman {} rejoint votre équipement.",
            objet_et_bonus(objet)
        ),
    }
}

pub fn erreur_codex(erreur: &ErreurCodex) -> String {
    match erreur {
        ErreurCodex::AucuneRecompense => {
            "Aucune récompense du codex à réclamer pour l'instant.".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_recompenses_se_disent_avec_leur_palier() {
        let contenu = ContenuV1::new();
        let recompenses = contenu.recompenses_du_codex();
        assert_eq!(
            recompense_reclamee(&recompenses[0]),
            "Récompense du codex, à 10 espèces capturées, au coffre : 10 filets lestés et 300 pièces d'argent."
        );
        assert_eq!(
            reclamer(&recompenses[0]),
            "Réclamer la récompense : 10 espèces capturées"
        );
        let complet = recompenses.last().unwrap();
        assert!(
            recompense_reclamee(complet).starts_with(
                "Récompense du codex complet, au coffre : 3 Philtres chromatiques. Le talisman Sceau du bestiaire ("
            ),
            "{}",
            recompense_reclamee(complet)
        );
    }
}
