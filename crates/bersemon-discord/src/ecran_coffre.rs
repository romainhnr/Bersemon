//! Le coffre du camp sur Discord (section 13.3) : un menu pour prendre dans
//! le coffre, un autre pour ranger ce que contient le sac, et de même pour
//! les filets, les pierres d'évolution, les boosters et les Éclats d'astre. Les
//! ressources ont leur propre écran : avec celles du Col du Givre-Noir, elles
//! ne tiennent plus dans un menu avec ces objets (étape 82).

use bersemon_core::inventaire::{Inventaire, Objet};
use bersemon_core::partie::Partie;
use bersemon_core::soin::Soin;
use bersemon_presentation::boosters::{boosters, eclats_d_astre};
use bersemon_presentation::filets::filets;
use bersemon_presentation::monstres::decrire_attaque;
use bersemon_presentation::partie::{
    argent, capsules_d_habilete as capsules_d_habilete_en_texte, charmes_de_garde, coffre_du_camp,
    pierres, ressources as ressources_en_texte,
};
use bersemon_presentation::soins::soins;

use crate::composants::{Cle, Vue, objet_en_texte};
use crate::ecran_camp::retour;
use crate::message::{
    OPTIONS_PAR_MENU, OptionDeMenu, PRINCIPAL, Question, Rangee, SECONDAIRE, bouton, menu, rangees,
};

/// Quantités proposées en plus de tout ce qu'il y en a. Un seul lot pour
/// les soins plus chers que la potion, qui s'achètent moins : avec l'argent
/// et toutes les capsules, le menu reste sous la limite de 25 options.
const LOTS_DE_POTIONS: [u32; 2] = [1, 5];
const LOTS_D_AUTRES_SOINS: [u32; 1] = [1];
const LOTS_D_ARGENT: [u32; 4] = [10, 50, 100, 500];
/// Le lot que vend Malvoise et que forge Gunther.
const LOT_DE_FILETS: u32 = 5;

pub fn coffre(partie: &Partie) -> Question {
    let mut lignes: Vec<Rangee> = Vec::new();
    // Une capsule d'attaque sert où qu'elle soit (section 13.3) : la
    // prendre dans le coffre ne ferait que l'exposer à une défaite.
    lignes.extend(menu(
        Cle::SortirDuCoffre,
        "Prendre dans le coffre",
        options(partie.coffre_du_camp(), false),
    ));
    lignes.extend(menu(
        Cle::RangerAuCoffre,
        "Ranger dans le coffre",
        options(partie.sac(), true),
    ));
    lignes.extend(menu(
        Cle::SortirRessourcesDuCoffre,
        "Prendre d'autres objets dans le coffre",
        options_d_objets(partie.coffre_du_camp()),
    ));
    lignes.extend(menu(
        Cle::RangerRessourcesAuCoffre,
        "Ranger d'autres objets dans le coffre",
        options_d_objets(partie.sac()),
    ));
    let mut boutons = Vec::new();
    if !partie.coffre_du_camp().ressources().is_empty() || !partie.sac().ressources().is_empty() {
        boutons.push(bouton(
            Cle::Afficher(Vue::Ressources),
            "Ressources",
            SECONDAIRE,
        ));
    }
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
/// l'argent ; puis chaque capsule, si on la propose.
fn options(inventaire: &Inventaire, avec_capsules: bool) -> Vec<OptionDeMenu> {
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
    // Ils se consomment, un par expédition : un lot d'un, puis tous
    // (section 12.5).
    let charmes = inventaire.charmes_de_garde();
    if charmes > 1 {
        options.push(option(Objet::CharmesDeGarde(1), charmes_de_garde(1), None));
    }
    if charmes > 0 {
        options.push(option(
            Objet::CharmesDeGarde(charmes),
            charmes_de_garde(charmes),
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
    for capsule in inventaire.capsules().iter().filter(|_| avec_capsules) {
        options.push(option(
            Objet::Capsule(capsule.id().clone()),
            format!("Capsule {}", capsule.attaque().nom()),
            Some(decrire_attaque(capsule.attaque())),
        ));
    }
    // Sans capsule, 16 options au plus ; un sac qui en porterait beaucoup
    // perdrait les dernières du menu, que "Tout ranger" range quand même.
    options.truncate(OPTIONS_PAR_MENU);
    options
}

/// Les ressources du sac et du coffre (section 14.3). Les mêmes clés que les
/// autres objets du coffre, qui sont sur un autre écran.
pub fn ressources(partie: &Partie) -> Question {
    let mut lignes: Vec<Rangee> = Vec::new();
    lignes.extend(menu(
        Cle::SortirRessourcesDuCoffre,
        "Prendre des ressources dans le coffre",
        options_de_ressources(partie.coffre_du_camp()),
    ));
    lignes.extend(menu(
        Cle::RangerRessourcesAuCoffre,
        "Ranger des ressources dans le coffre",
        options_de_ressources(partie.sac()),
    ));
    lignes.extend(rangees(vec![
        bouton(Cle::Afficher(Vue::Coffre), "Retour au coffre", SECONDAIRE),
        retour(),
    ]));
    Question {
        texte: coffre_du_camp(partie),
        rangees: lignes,
    }
}

/// Un lot de 5 ou tous pour chaque filet, une ou toutes pour chaque pierre,
/// tous ceux d'une sorte pour chaque booster, et tous les Éclats d'astre :
/// avec les trois filets, les deux pierres et les six boosters du contenu,
/// 17 options au plus (sections 5.4, 11.2, 13.7 et 14.3).
fn options_d_objets(inventaire: &Inventaire) -> Vec<OptionDeMenu> {
    let mut options = Vec::new();
    for (filet, nombre) in inventaire.filets().presents() {
        if nombre > LOT_DE_FILETS {
            options.push(option(
                Objet::Filets(filet, LOT_DE_FILETS),
                filets(filet, LOT_DE_FILETS),
                None,
            ));
        }
        options.push(option(
            Objet::Filets(filet, nombre),
            filets(filet, nombre),
            Some("Tout".to_string()),
        ));
    }
    for &(booster, nombre) in inventaire.boosters() {
        options.push(option(
            Objet::Boosters(booster, nombre),
            boosters(booster, nombre),
            Some("Tout".to_string()),
        ));
    }
    let eclats = inventaire.eclats_d_astre();
    if eclats > 0 {
        options.push(option(
            Objet::EclatsDAstre(eclats),
            eclats_d_astre(eclats),
            Some("Tout".to_string()),
        ));
    }
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
    options
}

/// Toutes celles d'une sorte pour chaque ressource : 17 sortes dans le
/// contenu.
fn options_de_ressources(inventaire: &Inventaire) -> Vec<OptionDeMenu> {
    let mut options: Vec<OptionDeMenu> = inventaire
        .ressources()
        .iter()
        .map(|(ressource, nombre)| {
            option(
                Objet::Ressources(ressource.id().clone(), *nombre),
                ressources_en_texte(ressource, *nombre),
                Some("Tout".to_string()),
            )
        })
        .collect();
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
