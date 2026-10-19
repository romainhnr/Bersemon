//! Noms et descriptions des monstres et de leurs attaques.

use std::fmt::Display;

use bersemon_core::attaque::{Attaque, Categorie};
use bersemon_core::combat::{Combat, Combattant, Cote};
use bersemon_core::espece::Espece;
use bersemon_core::experience::experience_requise;
use bersemon_core::monstre::Monstre;
use bersemon_core::niveau::Niveau;
use bersemon_core::stats::Stats;
use bersemon_core::statut::Statut;

use crate::mise_en_valeur::important;

/// Nom du monstre, suivi de "sauvage" pour un adversaire.
pub fn nom(monstre: &Monstre, cote: Cote) -> String {
    nommer(monstre, cote == Cote::Adversaire)
}

pub fn nom_sauvage(monstre: &Monstre) -> String {
    nom(monstre, Cote::Adversaire)
}

/// En gras sur Discord, dans le récit (section 10.6).
pub fn nom_en_valeur(monstre: &Monstre, cote: Cote) -> String {
    important(&nom(monstre, cote))
}

/// Comme `nom`, sauf qu'un Apôtre n'est pas dit sauvage (section 9.4).
pub fn nom_en_combat(combat: &Combat, combattant: Combattant) -> String {
    let sauvage = combattant.cote == Cote::Adversaire && !combat.est_contre_un_apotre();
    nommer(combat.monstre(combattant), sauvage)
}

fn nommer(monstre: &Monstre, sauvage: bool) -> String {
    let mut nom = monstre.nom().to_string();
    if sauvage {
        nom.push_str(" sauvage");
    }
    if monstre.est_shiny() {
        nom.push_str(" (shiny)");
    }
    nom
}

/// Nom, niveau, PV et état du monstre.
pub fn ligne_monstre(monstre: &Monstre, cote: Cote) -> String {
    ligne_monstre_nommee(monstre, &nom(monstre, cote))
}

/// Avec un surnom, le nom de l'espèce reste visible entre parenthèses.
pub fn ligne_monstre_nommee(monstre: &Monstre, nom: &str) -> String {
    let etat = if monstre.est_ko() {
        ", KO".to_string()
    } else {
        match monstre.statut() {
            Some(Statut::Poison) => ", poison".to_string(),
            Some(Statut::Paralysie) => ", paralysie".to_string(),
            None => String::new(),
        }
    };
    let espece = if monstre.surnom().is_some() {
        format!("{}, ", monstre.espece().nom())
    } else {
        String::new()
    };
    format!(
        "{nom} ({espece}niv. {}) : {}/{} PV{etat}",
        monstre.niveau().valeur(),
        monstre.pv_actuels(),
        monstre.stats().pv
    )
}

pub fn decrire_attaque(attaque: &Attaque) -> String {
    let categorie = match attaque.categorie() {
        Categorie::Physique => "physique",
        Categorie::Speciale => "spéciale",
    };
    let effet = match attaque.effet() {
        Some(effet) => {
            let statut = match effet.statut() {
                Statut::Poison => "poison",
                Statut::Paralysie => "paralysie",
            };
            format!(", {statut} {} %", effet.chance())
        }
        None => String::new(),
    };
    format!(
        "{} ({}, {categorie}, puissance {}, précision {} %{effet})",
        attaque.nom(),
        attaque.type_attaque().nom(),
        attaque.puissance(),
        attaque.precision()
    )
}

/// Sa ligne, ses types, ses stats et ses valeurs internes (section 13.2).
pub fn fiche_du_monstre(monstre: &Monstre) -> String {
    [
        ligne_monstre(monstre, Cote::Joueur),
        format!("XP : {}", progression_d_experience(monstre)),
        format!("Types : {}", types_de(monstre.espece())),
        format!("Stats : {}", decrire_stats(&monstre.stats())),
        format!(
            "Valeurs internes : {}",
            decrire_stats(monstre.valeurs_internes().valeurs())
        ),
    ]
    .join("\n")
}

/// L'XP totale du monstre et celle qu'il lui faut au total pour le niveau
/// suivant, « 130/216 » (section 7).
pub fn progression_d_experience(monstre: &Monstre) -> String {
    match Niveau::new(monstre.niveau().valeur() + 1) {
        Ok(suivant) => format!("{}/{}", monstre.experience(), experience_requise(suivant)),
        Err(_) => format!("{}, niveau maximum", monstre.experience()),
    }
}

/// Les six stats dans l'ordre du jeu.
pub fn decrire_stats<T: Display>(stats: &Stats<T>) -> String {
    format!(
        "PV {}, Attaque {}, Défense {}, Attaque spéciale {}, Défense spéciale {}, Vitesse {}",
        stats.pv,
        stats.attaque,
        stats.defense,
        stats.attaque_speciale,
        stats.defense_speciale,
        stats.vitesse
    )
}

/// Les attaques que le monstre connaît, numérotées à partir de 1.
pub fn attaques_connues(monstre: &Monstre) -> String {
    let mut lignes = vec![format!("Attaques de {} :", nom(monstre, Cote::Joueur))];
    for (position, attaque) in monstre.attaques().liste().iter().enumerate() {
        lignes.push(format!("  {}. {}", position + 1, decrire_attaque(attaque)));
    }
    lignes.join("\n")
}

/// Les types de l'espèce, dans l'ordre, séparés par des virgules.
pub fn types_de(espece: &Espece) -> String {
    let types: Vec<&str> = espece
        .types()
        .liste()
        .iter()
        .map(|type_monstre| type_monstre.nom())
        .collect();
    types.join(", ")
}
