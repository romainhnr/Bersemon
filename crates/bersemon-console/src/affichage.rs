//! Mise en texte, pour la console, de l'état et des événements d'un combat.

use bersemon_core::attaque::{Attaque, Categorie};
use bersemon_core::combat::{Combat, Cote, Evenement, Issue};
use bersemon_core::monstre::Monstre;
use bersemon_core::types::Efficacite;

pub fn nom(monstre: &Monstre, cote: Cote) -> String {
    let nom_espece = monstre.espece().nom();
    match cote {
        Cote::Joueur => nom_espece.to_string(),
        Cote::Adversaire => format!("{nom_espece} sauvage"),
    }
}

pub fn etat(combat: &Combat) -> String {
    format!(
        "{}\n{}",
        ligne_monstre(combat.joueur(), Cote::Joueur),
        ligne_monstre(combat.adversaire(), Cote::Adversaire)
    )
}

pub fn liste_attaques(monstre: &Monstre) -> String {
    monstre
        .attaques()
        .liste()
        .iter()
        .enumerate()
        .map(|(position, attaque)| format!("  {}. {}", position + 1, decrire_attaque(attaque)))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn evenement(evenement: &Evenement, combat: &Combat) -> String {
    let nom_cote = |cote: Cote| match cote {
        Cote::Joueur => nom(combat.joueur(), cote),
        Cote::Adversaire => nom(combat.adversaire(), cote),
    };
    match evenement {
        Evenement::AttaqueLancee { cote, attaque } => {
            format!("{} utilise {}.", nom_cote(*cote), attaque.nom())
        }
        Evenement::AttaqueRatee { cote } => format!("L'attaque de {} rate.", nom_cote(*cote)),
        Evenement::SansEffet { cible } => format!("Cela n'affecte pas {}.", nom_cote(*cible)),
        Evenement::Degats {
            cible,
            degats,
            critique,
            efficacite,
        } => {
            let critique = if *critique { "Coup critique ! " } else { "" };
            let efficacite = phrase_efficacite(*efficacite)
                .map(|phrase| format!("{phrase} "))
                .unwrap_or_default();
            format!(
                "{critique}{efficacite}{} perd {degats} PV.",
                nom_cote(*cible)
            )
        }
        Evenement::Ko { cote } => format!("{} est KO.", nom_cote(*cote)),
        Evenement::FinDuCombat {
            issue: Issue::Victoire,
        } => "Victoire !".to_string(),
        Evenement::FinDuCombat {
            issue: Issue::Defaite,
        } => "Défaite : votre monstre est KO.".to_string(),
    }
}

fn ligne_monstre(monstre: &Monstre, cote: Cote) -> String {
    format!(
        "{} (niv. {}) : {}/{} PV",
        nom(monstre, cote),
        monstre.niveau().valeur(),
        monstre.pv_actuels(),
        monstre.stats().pv
    )
}

fn decrire_attaque(attaque: &Attaque) -> String {
    let categorie = match attaque.categorie() {
        Categorie::Physique => "physique",
        Categorie::Speciale => "spéciale",
    };
    format!(
        "{} ({}, {categorie}, puissance {}, précision {} %)",
        attaque.nom(),
        attaque.type_attaque().nom(),
        attaque.puissance(),
        attaque.precision()
    )
}

fn phrase_efficacite(efficacite: Efficacite) -> Option<&'static str> {
    match efficacite {
        Efficacite::TresForte => Some("C'est extrêmement efficace !"),
        Efficacite::Forte => Some("C'est très efficace !"),
        Efficacite::Normale | Efficacite::Nulle => None,
        Efficacite::Reduite => Some("Ce n'est pas très efficace."),
        Efficacite::TresReduite => Some("Ce n'est presque pas efficace."),
    }
}
