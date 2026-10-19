//! Mise en page, pour la console, des listes et des menus numérotés. Les
//! phrases du jeu viennent de `bersemon_presentation`. Les numéros affichés au
//! joueur commencent à 1, les positions du noyau à 0.

use bersemon_core::attaque::Attaque;
use bersemon_core::codex::{Codex, EtatCodex};
use bersemon_core::combat::{Combat, Combattant, Cote};
use bersemon_core::espece::Espece;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::zone::Zone;
use bersemon_presentation::combat::{attaque_de_trop, attaque_en_combat};
use bersemon_presentation::monstres::{decrire_attaque, ligne_monstre, nom_en_combat};
use bersemon_presentation::partie::{
    decrire_zone, fiche_d_espece, ligne_du_codex, sac_en_expedition,
};

use crate::ecran_combat::Commande;

pub fn menu_action(combat: &Combat, commandes: &[Commande]) -> String {
    let mut lignes = vec!["Que faire ?".to_string()];
    for (position, commande) in commandes.iter().enumerate() {
        let libelle = match commande {
            Commande::Attaquer => "Attaquer".to_string(),
            Commande::Changer => "Changer de monstre".to_string(),
            Commande::LancerFilet => format!(
                "Lancer un filet ({} % de chances)",
                combat.chance_de_capture()
            ),
            Commande::UtiliserPotion => format!("Utiliser une potion ({})", combat.potions()),
            Commande::Fuir => "Fuir".to_string(),
        };
        lignes.push(format!("  {}. {libelle}", position + 1));
    }
    lignes.join("\n")
}

/// Chaque attaque du monstre actif, avec ses dégâts estimés contre le
/// monstre adverse actif (section 10.6).
pub fn liste_attaques(combat: &Combat) -> String {
    let monstre = combat.monstre(combat.actif(Cote::Joueur));
    let mut lignes: Vec<String> = monstre
        .attaques()
        .liste()
        .iter()
        .enumerate()
        .map(|(position, attaque)| {
            let ligne =
                attaque_en_combat(combat, position).unwrap_or_else(|| decrire_attaque(attaque));
            format!("  {}. {ligne}", position + 1)
        })
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
                nom_en_combat(
                    combat,
                    Combattant {
                        cote: Cote::Adversaire,
                        index,
                    },
                )
            )
        })
        .collect();
    lignes.push("  0. Ne pas capturer".to_string());
    lignes.join("\n")
}

pub fn question_oubli(monstre: &Monstre, nouvelle: &Attaque) -> String {
    let mut lignes = vec![
        attaque_de_trop(monstre, nouvelle),
        "Quelle attaque oublier ?".to_string(),
    ];
    for (position, attaque) in monstre.attaques().liste().iter().enumerate() {
        lignes.push(format!("  {}. {}", position + 1, decrire_attaque(attaque)));
    }
    lignes.push(format!("  0. Renoncer à {}", nouvelle.nom()));
    lignes.join("\n")
}

pub fn carte(partie: &Partie, accessibles: &[&Zone]) -> String {
    let mut lignes = vec!["Carte :".to_string()];
    for (position, zone) in accessibles.iter().enumerate() {
        lignes.push(format!(
            "  {}. {}",
            position + 1,
            decrire_zone(partie, zone)
        ));
    }
    lignes.join("\n")
}

pub fn equipe_en_expedition(partie: &Partie) -> String {
    let mut lignes = vec!["Votre équipe :".to_string()];
    for (index, monstre) in partie.collection().equipe().iter().enumerate() {
        lignes.push(format!(
            "  {}. {}",
            index + 1,
            ligne_monstre(monstre, Cote::Joueur)
        ));
    }
    lignes.push(sac_en_expedition(partie));
    lignes.join("\n")
}

/// Une espèce capturée montre aussi ses stats de base et ses attaques
/// (section 5.6).
pub fn codex(codex: &Codex, especes: &[Espece]) -> String {
    let mut lignes = vec!["Codex :".to_string()];
    for (position, espece) in especes.iter().enumerate() {
        lignes.push(format!(
            "  {}. {}",
            position + 1,
            ligne_du_codex(codex, espece)
        ));
        if codex.etat(espece.id()) == EtatCodex::Capturee {
            for ligne in fiche_d_espece(espece) {
                lignes.push(format!("     {ligne}"));
            }
        }
    }
    lignes.join("\n")
}
