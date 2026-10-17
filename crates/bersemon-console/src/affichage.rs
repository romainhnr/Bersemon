//! Mise en texte, pour la console, de l'état et des événements d'un combat.
//! Les phrases évitent les accords en genre, que le nom de l'espèce ne
//! permet pas de connaître.

use bersemon_core::attaque::{Attaque, Categorie};
use bersemon_core::combat::{
    BilanCombat, Combat, Combattant, Cote, ErreurCombat, Evenement, Issue,
};
use bersemon_core::monstre::Monstre;
use bersemon_core::statut::Statut;
use bersemon_core::types::Efficacite;

/// Les positions du noyau commencent à 0, les numéros affichés au joueur à 1.
pub fn erreur(erreur: &ErreurCombat) -> String {
    match erreur {
        ErreurCombat::AttaqueInexistante(index) => {
            format!("Il n'y a pas d'attaque numéro {}.", index + 1)
        }
        ErreurCombat::MonstreInexistant(index) => {
            format!("Il n'y a pas de monstre numéro {}.", index + 1)
        }
        ErreurCombat::MonstreKo(index) => format!("Le monstre numéro {} est KO.", index + 1),
        ErreurCombat::MonstreDejaActif(index) => {
            format!("Le monstre numéro {} combat déjà.", index + 1)
        }
        // Les autres erreurs ne viennent pas d'une saisie du joueur : la
        // console suit la phase du combat.
        autre => format!("Action impossible : {autre}."),
    }
}

pub fn nom(monstre: &Monstre, cote: Cote) -> String {
    let mut nom = monstre.espece().nom().to_string();
    if cote == Cote::Adversaire {
        nom.push_str(" sauvage");
    }
    if monstre.est_shiny() {
        nom.push_str(" (shiny)");
    }
    nom
}

pub fn rencontre(combat: &Combat) -> String {
    let noms: Vec<String> = combat
        .adversaires()
        .iter()
        .map(|monstre| monstre.espece().nom().to_string())
        .collect();
    format!("Des monstres sauvages surgissent : {}.", noms.join(", "))
}

pub fn etat(combat: &Combat) -> String {
    let actif_joueur = combat.actif(Cote::Joueur);
    let mut lignes = vec!["Votre équipe :".to_string()];
    for (index, monstre) in combat.equipe().iter().enumerate() {
        let repere = if index == actif_joueur.index {
            ">"
        } else {
            " "
        };
        lignes.push(format!(
            "  {repere} {}",
            ligne_monstre(monstre, Cote::Joueur)
        ));
    }
    let adversaire = combat.actif(Cote::Adversaire);
    lignes.push(format!(
        "En face : {}",
        ligne_monstre(combat.monstre(adversaire), Cote::Adversaire)
    ));
    let en_reserve = combat
        .adversaires()
        .iter()
        .enumerate()
        .filter(|&(index, monstre)| index > adversaire.index && !monstre.est_ko())
        .count();
    if en_reserve > 0 {
        lignes.push(format!("  et {en_reserve} autre(s) monstre(s) en attente"));
    }
    lignes.join("\n")
}

pub fn menu_action(combat: &Combat) -> String {
    format!(
        "Que faire ?\n  1. Attaquer\n  2. Changer de monstre\n  3. Lancer un filet ({} % de chances)\n  4. Fuir",
        combat.chance_de_capture()
    )
}

pub fn liste_attaques(monstre: &Monstre) -> String {
    let mut lignes: Vec<String> = monstre
        .attaques()
        .liste()
        .iter()
        .enumerate()
        .map(|(position, attaque)| format!("  {}. {}", position + 1, decrire_attaque(attaque)))
        .collect();
    lignes.push("  0. Retour".to_string());
    lignes.join("\n")
}

pub fn liste_equipe(combat: &Combat, avec_retour: bool) -> String {
    let actif = combat.actif(Cote::Joueur).index;
    let mut lignes: Vec<String> = combat
        .equipe()
        .iter()
        .enumerate()
        .map(|(index, monstre)| {
            let au_combat = if index == actif && !monstre.est_ko() {
                " (au combat)"
            } else {
                ""
            };
            format!(
                "  {}. {}{au_combat}",
                index + 1,
                ligne_monstre(monstre, Cote::Joueur)
            )
        })
        .collect();
    if avec_retour {
        lignes.push("  0. Retour".to_string());
    }
    lignes.join("\n")
}

pub fn liste_capturables(combat: &Combat, capturables: &[usize]) -> String {
    let mut lignes: Vec<String> = capturables
        .iter()
        .enumerate()
        .map(|(position, &index)| {
            format!(
                "  {}. {}",
                position + 1,
                nom(&combat.adversaires()[index], Cote::Adversaire)
            )
        })
        .collect();
    lignes.push("  0. Aucun".to_string());
    lignes.join("\n")
}

pub fn evenement(evenement: &Evenement, combat: &Combat) -> String {
    let nom_de = |combattant: Combattant| nom(combat.monstre(combattant), combattant.cote);
    match evenement {
        Evenement::MonstreEnvoye(combattant) => match combattant.cote {
            Cote::Joueur => format!("Vous envoyez {}.", nom_de(*combattant)),
            Cote::Adversaire => format!("{} entre en combat.", nom_de(*combattant)),
        },
        Evenement::AttaqueLancee { lanceur, attaque } => {
            format!("{} utilise {}.", nom_de(*lanceur), attaque.nom())
        }
        Evenement::AttaqueRatee(lanceur) => format!("L'attaque de {} rate.", nom_de(*lanceur)),
        Evenement::SansEffet(cible) => format!("Cela n'affecte pas {}.", nom_de(*cible)),
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
            format!("{critique}{efficacite}{} perd {degats} PV.", nom_de(*cible))
        }
        Evenement::StatutInflige { cible, statut } => match statut {
            Statut::Poison => format!("Le poison atteint {}.", nom_de(*cible)),
            Statut::Paralysie => format!("La paralysie frappe {}.", nom_de(*cible)),
        },
        Evenement::BloqueParLaParalysie(combattant) => {
            format!("La paralysie empêche {} d'attaquer.", nom_de(*combattant))
        }
        Evenement::DegatsDePoison { cible, degats } => {
            format!("Le poison fait perdre {degats} PV à {}.", nom_de(*cible))
        }
        Evenement::Ko(combattant) => format!("{} est KO.", nom_de(*combattant)),
        Evenement::FiletLance { cible, chance } => {
            format!(
                "Vous lancez un filet sur {} ({chance} % de chances).",
                nom_de(*cible)
            )
        }
        Evenement::CaptureReussie(cible) => format!(
            "Capture réussie : {}.",
            combat.monstre(*cible).espece().nom()
        ),
        Evenement::CaptureRatee(cible) => format!("{} se dégage du filet.", nom_de(*cible)),
        Evenement::Fuite => "Vous prenez la fuite.".to_string(),
        Evenement::FinDuCombat(Issue::Victoire) => "Victoire !".to_string(),
        Evenement::FinDuCombat(Issue::Defaite) => {
            "Défaite : toute votre équipe est KO.".to_string()
        }
        Evenement::FinDuCombat(Issue::Fuite) => "Le combat est terminé.".to_string(),
    }
}

pub fn bilan(bilan: &BilanCombat) -> String {
    let issue = match bilan.issue {
        Some(Issue::Victoire) => "victoire",
        Some(Issue::Defaite) => "défaite",
        Some(Issue::Fuite) => "fuite",
        None => "combat interrompu",
    };
    let mut lignes = vec![format!("Bilan : {issue}.")];
    for monstre in &bilan.equipe {
        lignes.push(format!("  {}", ligne_monstre(monstre, Cote::Joueur)));
    }
    if !bilan.captures.is_empty() {
        lignes.push("Monstres capturés :".to_string());
        for capture in &bilan.captures {
            lignes.push(format!("  {}", ligne_monstre(capture, Cote::Joueur)));
        }
    }
    lignes.join("\n")
}

fn ligne_monstre(monstre: &Monstre, cote: Cote) -> String {
    let etat = if monstre.est_ko() {
        ", KO".to_string()
    } else {
        match monstre.statut() {
            Some(Statut::Poison) => ", poison".to_string(),
            Some(Statut::Paralysie) => ", paralysie".to_string(),
            None => String::new(),
        }
    };
    format!(
        "{} (niv. {}) : {}/{} PV{etat}",
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

fn phrase_efficacite(efficacite: Efficacite) -> Option<&'static str> {
    match efficacite {
        Efficacite::TresForte => Some("C'est extrêmement efficace !"),
        Efficacite::Forte => Some("C'est très efficace !"),
        Efficacite::Normale | Efficacite::Nulle => None,
        Efficacite::Reduite => Some("Ce n'est pas très efficace."),
        Efficacite::TresReduite => Some("Ce n'est presque pas efficace."),
    }
}
