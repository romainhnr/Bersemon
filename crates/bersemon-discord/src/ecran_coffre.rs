//! Le coffre du camp sur Discord (section 13.3) : un menu pour prendre dans
//! le coffre, un autre pour ranger ce que contient le sac, et de même pour
//! les ressources et les pierres d'évolution.

use bersemon_core::inventaire::{Inventaire, Objet};
use bersemon_core::partie::Partie;
use bersemon_core::soin::Soin;
use bersemon_presentation::monstres::decrire_attaque;
use bersemon_presentation::partie::{
    argent, capsules_d_habilete as capsules_d_habilete_en_texte, coffre_du_camp, pierres,
    ressources,
};
use bersemon_presentation::soins::soins;

use crate::composants::{Cle, objet_en_texte};
use crate::ecran_camp::retour;
use crate::message::{
    OPTIONS_PAR_MENU, OptionDeMenu, PRINCIPAL, Question, Rangee, bouton, menu, rangees,
};

/// Quantités proposées en plus de tout ce qu'il y en a. Un seul lot pour
/// les soins plus chers que la potion, qui s'achètent moins : avec l'argent
/// et toutes les capsules, le menu reste sous la limite de 25 options.
const LOTS_DE_POTIONS: [u32; 2] = [1, 5];
const LOTS_D_AUTRES_SOINS: [u32; 1] = [1];
const LOTS_D_ARGENT: [u32; 4] = [10, 50, 100, 500];

pub fn coffre(partie: &Partie) -> Question {
    let mut lignes: Vec<Rangee> = Vec::new();
    lignes.extend(menu(
        Cle::SortirDuCoffre,
        "Prendre dans le coffre",
        options(partie.coffre_du_camp()),
    ));
    lignes.extend(menu(
        Cle::RangerAuCoffre,
        "Ranger dans le coffre",
        options(partie.sac()),
    ));
    lignes.extend(menu(
        Cle::SortirRessourcesDuCoffre,
        "Prendre des ressources ou des pierres dans le coffre",
        options_de_ressources(partie.coffre_du_camp()),
    ));
    lignes.extend(menu(
        Cle::RangerRessourcesAuCoffre,
        "Ranger des ressources ou des pierres dans le coffre",
        options_de_ressources(partie.sac()),
    ));
    let mut boutons = Vec::new();
    if !partie.sac().est_vide() {
        boutons.push(bouton(
            Cle::ToutRanger,
            "Tout ranger dans le coffre",
            PRINCIPAL,
        ));
    }
    boutons.push(retour());
    lignes.extend(rangees(boutons));
    Question {
        texte: coffre_du_camp(partie),
        rangees: lignes,
    }
}

/// Les lots plus petits que ce qu'il y a, puis le tout, pour chaque soin et
/// l'argent ; puis chaque capsule.
fn options(inventaire: &Inventaire) -> Vec<OptionDeMenu> {
    let mut options = Vec::new();
    for (soin, disponible) in inventaire.soins().presents() {
        let lots: &[u32] = if soin == Soin::Potion {
            &LOTS_DE_POTIONS
        } else {
            &LOTS_D_AUTRES_SOINS
        };
        for &nombre in lots.iter().filter(|&&nombre| nombre < disponible) {
            options.push(option(
                Objet::Soins(soin, nombre),
                soins(soin, nombre),
                None,
            ));
        }
        options.push(option(
            Objet::Soins(soin, disponible),
            soins(soin, disponible),
            Some("Tout".to_string()),
        ));
    }
    let argent_disponible = inventaire.argent();
    for montant in LOTS_D_ARGENT
        .into_iter()
        .filter(|&montant| montant < argent_disponible)
    {
        options.push(option(Objet::Argent(montant), argent(montant), None));
    }
    if argent_disponible > 0 {
        options.push(option(
            Objet::Argent(argent_disponible),
            argent(argent_disponible),
            Some("Tout".to_string()),
        ));
    }
    // Elles se consomment : un lot d'une, puis toutes (section 5.9).
    let capsules_d_habilete = inventaire.capsules_d_habilete();
    if capsules_d_habilete > 1 {
        options.push(option(
            Objet::CapsulesDHabilete(1),
            capsules_d_habilete_en_texte(1),
            None,
        ));
    }
    if capsules_d_habilete > 0 {
        options.push(option(
            Objet::CapsulesDHabilete(capsules_d_habilete),
            capsules_d_habilete_en_texte(capsules_d_habilete),
            Some("Tout".to_string()),
        ));
    }
    for capsule in inventaire.capsules() {
        options.push(option(
            Objet::Capsule(capsule.id().clone()),
            format!("Capsule {}", capsule.attaque().nom()),
            Some(decrire_attaque(capsule.attaque())),
        ));
    }
    // Avec les neuf capsules d'attaque du contenu, tous les soins et les
    // capsules d'habileté, la limite est tout juste atteinte ; elle est tenue
    // au cas où il y en aurait davantage, aux dépens des dernières capsules
    // d'attaque.
    options.truncate(OPTIONS_PAR_MENU);
    options
}

/// Une, puis toutes, pour chaque sorte de ressource et de pierre : avec les
/// neuf ressources et les deux pierres du contenu, 22 options au plus
/// (sections 5.4 et 14.3).
fn options_de_ressources(inventaire: &Inventaire) -> Vec<OptionDeMenu> {
    let mut options = Vec::new();
    for &(pierre, nombre) in inventaire.pierres() {
        if nombre > 1 {
            options.push(option(Objet::Pierres(pierre, 1), pierres(pierre, 1), None));
        }
        options.push(option(
            Objet::Pierres(pierre, nombre),
            pierres(pierre, nombre),
            Some("Tout".to_string()),
        ));
    }
    for (ressource, nombre) in inventaire.ressources() {
        let id = ressource.id().clone();
        if *nombre > 1 {
            options.push(option(
                Objet::Ressources(id.clone(), 1),
                ressources(ressource, 1),
                None,
            ));
        }
        options.push(option(
            Objet::Ressources(id, *nombre),
            ressources(ressource, *nombre),
            Some("Tout".to_string()),
        ));
    }
    options.truncate(OPTIONS_PAR_MENU);
    options
}

fn option(objet: Objet, libelle: String, description: Option<String>) -> OptionDeMenu {
    OptionDeMenu {
        libelle,
        valeur: objet_en_texte(&objet),
        description,
    }
}
