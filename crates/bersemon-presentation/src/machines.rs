//! Textes des machines de Gunther (section 13.6, docs/contenu-v1.md, section
//! 19) : les pièges et la cour d'entraînement.

use std::time::Duration;

use bersemon_core::combat::Cote;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::machines::{ErreurMachine, ReglesDeLaCour, ReglesDesPieges};
use bersemon_core::monstre::Monstre;
use bersemon_core::niveau::Niveau;
use bersemon_core::partie::Partie;
use bersemon_core::ressource::Ressource;
use bersemon_core::zone::Zone;

use crate::expeditions_automatiques::duree;
use crate::monstres::nom_en_valeur;
use crate::partie::{enumerer, ressources};

/// "1 ressource toutes les 2 h, stock de 6".
pub fn effet_des_pieges(regles: &ReglesDesPieges) -> String {
    let rythme = if regles.intervalle <= Duration::from_secs(3_600) {
        "1 ressource par heure".to_string()
    } else {
        format!("1 ressource toutes les {}", duree(regles.intervalle))
    };
    let rares = if regles.avec_rares {
        ", ressource rare 1 fois sur 10"
    } else {
        ""
    };
    format!("{rythme}, stock de {}{rares}", regles.stock_maximal)
}

/// "1 place, 50 XP par heure".
pub fn effet_de_la_cour(regles: &ReglesDeLaCour) -> String {
    let places = if regles.places > 1 {
        format!("{} places", regles.places)
    } else {
        "1 place".to_string()
    };
    format!("{places}, {} XP par heure", regles.experience_par_heure)
}

/// La ligne des pièges, sur l'écran des machines ; rien s'ils ne sont pas
/// construits.
pub fn etat_des_pieges(partie: &Partie, zones: &[Zone], maintenant: Horodatage) -> Option<String> {
    let niveau = partie
        .forge()
        .ameliorations()
        .niveau(bersemon_core::amelioration::Amelioration::Pieges);
    let regles = partie.forge().ameliorations().pieges()?;
    let Some(pieges) = partie.machines().pieges() else {
        return Some(format!(
            "Pièges (niveau {niveau}, {}) : posés nulle part.",
            effet_des_pieges(&regles)
        ));
    };
    let zone = zones
        .iter()
        .find(|zone| zone.id() == pieges.zone())
        .map_or(pieges.zone().as_str(), Zone::nom);
    let stock = pieges.stock(&regles, maintenant);
    let suite = match pieges.prochaine(&regles, maintenant) {
        Some(attente) => format!("une de plus dans {}", duree(attente)),
        None => "pleins".to_string(),
    };
    Some(format!(
        "Pièges (niveau {niveau}, {}) dans {zone} : {stock}/{} ressources, {suite}.",
        effet_des_pieges(&regles),
        regles.stock_maximal
    ))
}

/// Les lignes de la cour, sur l'écran des machines ; rien si elle n'est pas
/// construite.
pub fn etat_de_la_cour(partie: &Partie, plafond: Niveau, maintenant: Horodatage) -> Option<String> {
    let niveau = partie
        .forge()
        .ameliorations()
        .niveau(bersemon_core::amelioration::Amelioration::Cour);
    let regles = partie.forge().ameliorations().cour()?;
    let mut lignes = vec![format!(
        "Cour d'entraînement (niveau {niveau}, {}), jusqu'au niveau {} : {}/{} occupées.",
        effet_de_la_cour(&regles),
        plafond.valeur(),
        partie.machines().cour().len(),
        regles.places
    )];
    for entrainement in partie.machines().cour() {
        lignes.push(format!(
            "  {} : {} XP gagnée depuis {}.",
            nom_en_valeur(entrainement.monstre(), Cote::Joueur),
            entrainement.experience(&regles, plafond, maintenant),
            duree(entrainement.depuis().duree_jusqu_a(maintenant))
        ));
    }
    Some(lignes.join("\n"))
}

/// L'écran des machines, depuis la forge.
pub fn machines(
    partie: &Partie,
    zones: &[Zone],
    plafond: Niveau,
    maintenant: Horodatage,
) -> String {
    let lignes: Vec<String> = [
        etat_des_pieges(partie, zones, maintenant),
        etat_de_la_cour(partie, plafond, maintenant),
    ]
    .into_iter()
    .flatten()
    .collect();
    if lignes.is_empty() {
        "Gunther n'a encore construit aucune machine : les pièges et la cour d'entraînement se construisent à la forge.".to_string()
    } else {
        format!("Machines de Gunther.\n{}", lignes.join("\n"))
    }
}

/// Au camp, quand les pièges sont pleins.
pub fn avis_des_pieges(partie: &Partie, maintenant: Horodatage) -> Option<&'static str> {
    let regles = partie.forge().ameliorations().pieges()?;
    let pieges = partie.machines().pieges()?;
    (pieges.prochaine(&regles, maintenant).is_none())
        .then_some("Les pièges de Gunther sont pleins.")
}

pub fn pieges_poses(zone: &str) -> String {
    format!("Gunther pose ses pièges dans {zone}.")
}

pub fn pieges_releves(prises: &[(Ressource, u32)]) -> String {
    let liste: Vec<String> = prises
        .iter()
        .map(|(ressource, nombre)| ressources(ressource, *nombre))
        .collect();
    format!(
        "Vous relevez les pièges : {}. Elles vont dans le coffre.",
        enumerer(&liste)
    )
}

pub fn entree_a_l_entrainement(monstre: &Monstre) -> String {
    format!(
        "{} rejoint la cour d'entraînement.",
        nom_en_valeur(monstre, Cote::Joueur)
    )
}

pub fn sortie_de_l_entrainement(monstre: &Monstre, experience: u32) -> String {
    format!(
        "{} quitte la cour d'entraînement avec {experience} XP.",
        nom_en_valeur(monstre, Cote::Joueur)
    )
}

pub fn erreur_machine(erreur: &ErreurMachine) -> String {
    match erreur {
        ErreurMachine::NonConstruite => {
            "Gunther n'a pas encore construit cette machine.".to_string()
        }
        ErreurMachine::ZoneNonExploree => {
            "Les pièges se posent dans une zone explorée, menée jusqu'au bout.".to_string()
        }
        ErreurMachine::DejaPosesIci => "Les pièges sont déjà posés dans cette zone.".to_string(),
        ErreurMachine::PiegesNonPoses => "Les pièges ne sont posés nulle part.".to_string(),
        ErreurMachine::PiegesARelever => {
            "Relevez d'abord les pièges avant de les déplacer.".to_string()
        }
        ErreurMachine::ZoneDesPieges => "Les pièges sont posés ailleurs.".to_string(),
        ErreurMachine::PiegesVides => "Les pièges sont vides.".to_string(),
        ErreurMachine::CourPleine => "La cour d'entraînement est pleine.".to_string(),
        ErreurMachine::PlafondAtteint => {
            "Ce monstre est déjà à son plafond d'entraînement : le niveau maximal de la zone ouverte la plus avancée.".to_string()
        }
        ErreurMachine::AucunMonstreALaPlace(_) => {
            "Ce monstre n'est plus dans la cour d'entraînement.".to_string()
        }
        ErreurMachine::Collection(erreur) => crate::erreurs::erreur_collection(erreur),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::machines::{regles_de_la_cour, regles_des_pieges};

    #[test]
    fn effets_par_niveau() {
        assert_eq!(
            effet_des_pieges(&regles_des_pieges(1).unwrap()),
            "1 ressource toutes les 2 h, stock de 6"
        );
        assert_eq!(
            effet_des_pieges(&regles_des_pieges(3).unwrap()),
            "1 ressource par heure, stock de 24, ressource rare 1 fois sur 10"
        );
        assert_eq!(
            effet_de_la_cour(&regles_de_la_cour(1).unwrap()),
            "1 place, 50 XP par heure"
        );
        assert_eq!(
            effet_de_la_cour(&regles_de_la_cour(3).unwrap()),
            "3 places, 100 XP par heure"
        );
    }
}
