//! Le coffre du camp sur Discord (section 13.3) : un menu pour prendre dans
//! le coffre, un autre pour ranger ce que contient le sac.

use bersemon_core::inventaire::{Inventaire, Objet};
use bersemon_core::partie::Partie;
use bersemon_presentation::monstres::decrire_attaque;
use bersemon_presentation::partie::{argent, coffre_du_camp, potions};

use crate::composants::{Cle, objet_en_texte};
use crate::ecran_camp::retour;
use crate::message::{
    OPTIONS_PAR_MENU, OptionDeMenu, PRINCIPAL, Question, Rangee, bouton, menu, rangees,
};

/// Quantités proposées en plus de tout ce qu'il y en a.
const LOTS_DE_POTIONS: [u32; 2] = [1, 5];
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

/// Les lots plus petits que ce qu'il y a, puis le tout, pour les potions et
/// l'argent ; puis chaque capsule.
fn options(inventaire: &Inventaire) -> Vec<OptionDeMenu> {
    let mut options = Vec::new();
    let potions_disponibles = inventaire.potions();
    for nombre in LOTS_DE_POTIONS
        .into_iter()
        .filter(|&nombre| nombre < potions_disponibles)
    {
        options.push(option(Objet::Potions(nombre), potions(nombre), None));
    }
    if potions_disponibles > 0 {
        options.push(option(
            Objet::Potions(potions_disponibles),
            potions(potions_disponibles),
            Some("Toutes".to_string()),
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
    for capsule in inventaire.capsules() {
        options.push(option(
            Objet::Capsule(capsule.id().clone()),
            format!("Capsule {}", capsule.attaque().nom()),
            Some(decrire_attaque(capsule.attaque())),
        ));
    }
    // Avec les six capsules du contenu, la limite n'est pas atteinte ; elle
    // est tenue au cas où il y en aurait davantage.
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
