//! Combat : rencontre, état des deux camps, événements du tour et suites
//! (XP, attaques apprises ou oubliées).

use bersemon_core::attaque::Attaque;
use bersemon_core::combat::{Combat, Combattant, Cote, Evenement, Issue};
use bersemon_core::monstre::{Monstre, RapportExperience};
use bersemon_core::statut::Statut;
use bersemon_core::types::Efficacite;

use crate::monstres::{decrire_attaque, ligne_monstre, ligne_monstre_nommee, nom, nom_en_combat};

pub fn rencontre(combat: &Combat) -> String {
    let noms: Vec<String> = (0..combat.adversaires().len())
        .map(|index| {
            nom_en_combat(
                combat,
                Combattant {
                    cote: Cote::Adversaire,
                    index,
                },
            )
        })
        .collect();
    format!("Des monstres surgissent : {}.", noms.join(", "))
}

/// L'équipe, avec un repère devant le monstre actif, puis le monstre adverse
/// actif et ceux qui attendent.
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
        ligne_monstre_nommee(
            combat.monstre(adversaire),
            &nom_en_combat(combat, adversaire)
        )
    ));
    let en_attente = combat
        .adversaires()
        .iter()
        .enumerate()
        .filter(|&(index, monstre)| index > adversaire.index && !monstre.est_ko())
        .count();
    if en_attente > 0 {
        lignes.push(format!("  et {en_attente} autre(s) monstre(s) en attente"));
    }
    lignes.join("\n")
}

pub fn evenement(evenement: &Evenement, combat: &Combat) -> String {
    let nom_de = |combattant: Combattant| nom_en_combat(combat, combattant);
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
        Evenement::FiletLance { cible, chance } => format!(
            "Vous lancez un filet sur {} ({chance} % de chances).",
            nom_de(*cible)
        ),
        Evenement::CaptureReussie(cible) => {
            format!("Capture réussie : {}.", combat.monstre(*cible).nom())
        }
        Evenement::CaptureRatee(cible) => format!("{} se dégage du filet.", nom_de(*cible)),
        Evenement::PotionUtilisee { cible, pv_rendus } => {
            soin_de_potion(&nom_de(*cible), *pv_rendus)
        }
        Evenement::Fuite => "Vous prenez la fuite.".to_string(),
        Evenement::FinDuCombat(Issue::Victoire) => "Victoire !".to_string(),
        Evenement::FinDuCombat(Issue::Defaite) => {
            "Défaite : toute votre équipe est KO.".to_string()
        }
        Evenement::FinDuCombat(Issue::Fuite) => "Le combat est terminé.".to_string(),
    }
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

pub fn soin_de_potion(nom: &str, pv_rendus: u32) -> String {
    if pv_rendus == 0 {
        format!("La potion est sans effet sur {nom}.")
    } else {
        format!("{nom} récupère {pv_rendus} PV.")
    }
}

pub fn rapport_experience(monstre: &Monstre, rapport: &RapportExperience) -> String {
    let nom = nom(monstre, Cote::Joueur);
    let mut lignes = vec![format!("{nom} gagne {} XP.", rapport.experience_gagnee)];
    if rapport.niveau_apres > rapport.niveau_avant {
        lignes.push(format!(
            "{nom} passe au niveau {}.",
            rapport.niveau_apres.valeur()
        ));
    }
    for attaque in &rapport.attaques_apprises {
        lignes.push(format!("{nom} apprend {}.", attaque.nom()));
    }
    lignes.join("\n")
}

/// Le monstre connaît déjà 4 attaques (Q33).
pub fn attaque_de_trop(monstre: &Monstre, nouvelle: &Attaque) -> String {
    format!(
        "{} veut apprendre {}, mais connaît déjà 4 attaques.",
        nom(monstre, Cote::Joueur),
        decrire_attaque(nouvelle)
    )
}

pub fn renoncement(monstre: &Monstre, nouvelle: &Attaque) -> String {
    format!(
        "{} n'apprend pas {}.",
        nom(monstre, Cote::Joueur),
        nouvelle.nom()
    )
}

pub fn remplacement(monstre: &Monstre, oubliee: &Attaque, nouvelle: &Attaque) -> String {
    format!(
        "{} oublie {} et apprend {}.",
        nom(monstre, Cote::Joueur),
        oubliee.nom(),
        nouvelle.nom()
    )
}
