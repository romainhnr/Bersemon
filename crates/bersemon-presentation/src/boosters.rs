//! Textes des boosters et du service de Malvoise qui rend un monstre shiny
//! (sections 13.7 et 14.3, docs/contenu-v1.md, section 17).

use bersemon_core::booster::{Booster, GAIN_D_UN_BOOSTER};
use bersemon_core::combat::Cote;
use bersemon_core::monstre::Monstre;
use bersemon_core::stats::{Stat, ValeursInternes};

use crate::monstres::nom_en_valeur;

/// Après l'âme d'un Apôtre vaincu (section 9.4).
pub const ECLAT_RAMASSE: &str =
    "Dans la poussière de sa dépouille luit un Éclat d'astre. Vous le ramassez.";

/// Le libellé du service, à l'étal de Malvoise.
pub const SERVICE_SHINY: &str = "Rendre un monstre shiny (1 Éclat d'astre)";

pub fn nom_de_stat(stat: Stat) -> &'static str {
    match stat {
        Stat::Pv => "PV",
        Stat::Attaque => "Attaque",
        Stat::Defense => "Défense",
        Stat::AttaqueSpeciale => "Attaque spéciale",
        Stat::DefenseSpeciale => "Défense spéciale",
        Stat::Vitesse => "Vitesse",
    }
}

/// « de PV », « d'Attaque ».
fn de_la_stat(stat: Stat) -> String {
    let nom = nom_de_stat(stat);
    if nom.starts_with('A') {
        format!("d'{nom}")
    } else {
        format!("de {nom}")
    }
}

fn au_pluriel(booster: Booster) -> &'static str {
    match booster {
        Booster::FioleDeSang => "Fioles de sang",
        Booster::MoelleDeBete => "Moelles de bête",
        Booster::EcorceDeFer => "Écorces de fer",
        Booster::EncreDArcane => "Encres d'arcane",
        Booster::CendreBenite => "Cendres bénites",
        Booster::PoudreDeVifArgent => "Poudres de vif-argent",
    }
}

/// « 1 Fiole de sang », « 2 Fioles de sang ».
pub fn boosters(booster: Booster, nombre: u32) -> String {
    if nombre > 1 {
        format!("{nombre} {}", au_pluriel(booster))
    } else {
        format!("{nombre} {}", booster.nom())
    }
}

/// « 1 Éclat d'astre », « 2 Éclats d'astre ».
pub fn eclats_d_astre(nombre: u32) -> String {
    if nombre > 1 {
        format!("{nombre} Éclats d'astre")
    } else {
        format!("{nombre} Éclat d'astre")
    }
}

/// « +5 à la valeur interne de PV, 31 au plus. »
pub fn effet_du_booster(booster: Booster) -> String {
    format!(
        "+{GAIN_D_UN_BOOSTER} à la valeur interne {}, {} au plus.",
        de_la_stat(booster.stat()),
        ValeursInternes::MAX
    )
}

pub fn booster_donne(booster: Booster, avant: &Monstre, apres: &Monstre) -> String {
    let stat = booster.stat();
    format!(
        "{} : valeur interne {} de {} à {}.",
        nom_en_valeur(apres, Cote::Joueur),
        de_la_stat(stat),
        avant.valeurs_internes().valeurs().de(stat),
        apres.valeurs_internes().valeurs().de(stat)
    )
}

pub fn refus_valeur_maximale(booster: Booster) -> String {
    format!(
        "La valeur interne {} de ce monstre est déjà à {}.",
        de_la_stat(booster.stat()),
        ValeursInternes::MAX
    )
}

pub fn refus_sans_booster(booster: Booster) -> String {
    format!("Vous n'avez pas de {}.", booster.nom())
}

pub fn question_shiny(monstre: &Monstre) -> String {
    format!(
        "Malvoise rendra {} shiny contre un Éclat d'astre : ses valeurs internes passeront toutes à {}. Confirmer ?",
        monstre.nom(),
        ValeursInternes::MAX
    )
}

pub fn monstre_rendu_shiny(monstre: &Monstre) -> String {
    let nom = nom_en_valeur(monstre, Cote::Joueur);
    format!(
        "Malvoise broie l'Éclat d'astre au-dessus de {nom}, dont la robe prend un reflet nouveau. {nom} est désormais shiny."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noms_et_effets() {
        assert_eq!(boosters(Booster::CendreBenite, 2), "2 Cendres bénites");
        assert_eq!(boosters(Booster::FioleDeSang, 1), "1 Fiole de sang");
        assert_eq!(eclats_d_astre(3), "3 Éclats d'astre");
        assert_eq!(
            effet_du_booster(Booster::FioleDeSang),
            "+5 à la valeur interne de PV, 31 au plus."
        );
        assert_eq!(
            effet_du_booster(Booster::EncreDArcane),
            "+5 à la valeur interne d'Attaque spéciale, 31 au plus."
        );
    }
}
