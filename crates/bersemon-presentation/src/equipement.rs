//! L'équipement du héros (section 4.3) : ses objets, leurs bonus et ce
//! qu'il porte.

use bersemon_core::equipement::{Equipement, ErreurEquipement, ObjetDEquipement, PlaceDEquipement};
use bersemon_core::heros::Heros;
use bersemon_core::stats::BonusDeStats;

use crate::monstres::{decrire_attaque, decrire_stats};
use crate::partie::enumerer;

/// "d'Attaque", "de Défense" : le nom de la stat, avec son article.
fn stats_renforcees(bonus: &BonusDeStats) -> Vec<(u8, &'static str)> {
    [
        (bonus.pv, "de PV"),
        (bonus.attaque, "d'Attaque"),
        (bonus.defense, "de Défense"),
        (bonus.attaque_speciale, "d'Attaque spéciale"),
        (bonus.defense_speciale, "de Défense spéciale"),
        (bonus.vitesse, "de Vitesse"),
    ]
    .into_iter()
    .filter(|(pour_cent, _)| *pour_cent > 0)
    .collect()
}

/// "+5 % d'Attaque", ou "+5 % de Défense et de Défense spéciale" quand les
/// stats renforcées le sont autant.
pub fn decrire_bonus(bonus: &BonusDeStats) -> String {
    let stats = stats_renforcees(bonus);
    match stats.first() {
        None => "aucun bonus".to_string(),
        Some((premier, _)) if stats.iter().all(|(pour_cent, _)| pour_cent == premier) => {
            let noms: Vec<String> = stats.iter().map(|(_, nom)| nom.to_string()).collect();
            format!("+{premier} % {}", enumerer(&noms))
        }
        Some(_) => {
            let parties: Vec<String> = stats
                .iter()
                .map(|(pour_cent, nom)| format!("+{pour_cent} % {nom}"))
                .collect();
            enumerer(&parties)
        }
    }
}

/// "Couteau de chasse (+5 % d'Attaque)".
pub fn objet_et_bonus(objet: &ObjetDEquipement) -> String {
    format!("{} ({})", objet.nom(), decrire_bonus(&objet.bonus()))
}

pub fn nom_de_place(place: PlaceDEquipement) -> &'static str {
    match place {
        PlaceDEquipement::Arme => "Arme",
        PlaceDEquipement::Armure => "Armure",
        PlaceDEquipement::PremierTalisman => "Premier talisman",
        PlaceDEquipement::SecondTalisman => "Second talisman",
    }
}

/// "Arme : Couteau de chasse (+5 % d'Attaque)", ou "Arme : aucune".
pub fn place_et_objet(equipement: &Equipement, place: PlaceDEquipement) -> String {
    let objet = match equipement.porte(place) {
        Some(objet) => objet_et_bonus(objet),
        None => match place {
            PlaceDEquipement::Arme | PlaceDEquipement::Armure => "aucune".to_string(),
            PlaceDEquipement::PremierTalisman | PlaceDEquipement::SecondTalisman => {
                "aucun".to_string()
            }
        },
    };
    format!("{} : {objet}", nom_de_place(place))
}

/// Le héros quand il combat, après ses monstres (section 4.4) : sa classe,
/// son type, son niveau de combat, ses PV, ses stats et les attaques de son
/// arme.
pub fn fiche_du_heros(heros: &Heros) -> String {
    let combattant = heros.combattant();
    let mut lignes = vec![
        format!(
            "{}, {}, Humain, niveau de combat {} : {}/{} PV.",
            heros.nom().as_str(),
            heros.classe().nom(),
            combattant.niveau().valeur(),
            combattant.pv_actuels(),
            combattant.stats().pv
        ),
        format!("Stats en combat : {}.", decrire_stats(&combattant.stats())),
        "Attaques :".to_string(),
    ];
    for attaque in combattant.attaques().liste() {
        lignes.push(format!("  {}", decrire_attaque(attaque)));
    }
    lignes.push(
        "Il combat quand tous vos monstres sont KO. Son niveau vaut 5, plus 5 par palier de son arme et de son armure.".to_string(),
    );
    lignes.join("\n")
}

/// "Taille, Frappe brutale" : celles qu'une arme donne au héros (section
/// 4.4).
pub fn attaques_de_l_objet(objet: &ObjetDEquipement) -> Option<String> {
    let noms: Vec<String> = objet
        .attaques()
        .iter()
        .map(|attaque| attaque.nom().to_string())
        .collect();
    (!noms.is_empty()).then(|| enumerer(&noms))
}

/// Ce que le héros porte, place par place, le bonus total des objets portés
/// et ce qu'il possède sans le porter.
pub fn etat_de_l_equipement(equipement: &Equipement) -> String {
    let mut lignes = vec!["Équipement du héros :".to_string()];
    for place in PlaceDEquipement::TOUTES {
        lignes.push(format!("  {}", place_et_objet(equipement, place)));
    }
    let total = equipement.bonus();
    if total != BonusDeStats::default() {
        lignes.push(format!(
            "Bonus de l'équipement pour vos monstres en combat : {}.",
            decrire_bonus(&total)
        ));
    }
    let portes: Vec<_> = PlaceDEquipement::TOUTES
        .iter()
        .filter_map(|place| equipement.porte(*place))
        .collect();
    let ranges: Vec<String> = equipement
        .possedes()
        .iter()
        .filter(|objet| !portes.contains(objet))
        .map(objet_et_bonus)
        .collect();
    if equipement.possedes().is_empty() {
        lignes.push("Vous ne possédez aucun objet : Gunther peut en forger.".to_string());
    } else if !ranges.is_empty() {
        lignes.push(format!("Non portés : {}.", ranges.join(", ")));
    }
    lignes.join("\n")
}

pub fn erreur_equipement(erreur: &ErreurEquipement) -> String {
    match erreur {
        ErreurEquipement::NonPossede(_) => {
            "Vous ne possédez pas cet objet : Gunther peut le forger.".to_string()
        }
        ErreurEquipement::DejaPossede(_) => "Vous possédez déjà cet objet.".to_string(),
        ErreurEquipement::MauvaisePlace(place) => {
            format!(
                "Cet objet ne va pas à cette place : {}.",
                nom_de_place(*place)
            )
        }
        ErreurEquipement::DejaPorte(_) => {
            "Vous portez déjà ce talisman : les deux talismans sont différents.".to_string()
        }
        ErreurEquipement::PlaceVide(place) => {
            format!("{} : vous ne portez rien.", nom_de_place(*place))
        }
        ErreurEquipement::PlaceEnDouble(place) => {
            format!("Deux objets à la même place : {}.", nom_de_place(*place))
        }
        ErreurEquipement::AutreClasse(_) => "Cet objet est réservé à une autre classe.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::stats::Stats;

    #[test]
    fn le_bonus_se_dit_en_une_phrase() {
        let arme = Stats {
            attaque: 5,
            ..Stats::default()
        };
        assert_eq!(decrire_bonus(&arme), "+5 % d'Attaque");
        let armure = Stats {
            defense: 10,
            defense_speciale: 10,
            ..Stats::default()
        };
        assert_eq!(
            decrire_bonus(&armure),
            "+10 % de Défense et de Défense spéciale"
        );
        let melange = Stats {
            attaque: 15,
            vitesse: 5,
            ..Stats::default()
        };
        assert_eq!(
            decrire_bonus(&melange),
            "+15 % d'Attaque et +5 % de Vitesse"
        );
    }

    #[test]
    fn un_heros_sans_objet_est_invite_a_la_forge() {
        assert_eq!(
            etat_de_l_equipement(&Equipement::default()),
            "Équipement du héros :\n  Arme : aucune\n  Armure : aucune\n  Premier talisman : aucun\n  Second talisman : aucun\nVous ne possédez aucun objet : Gunther peut en forger."
        );
    }
}
