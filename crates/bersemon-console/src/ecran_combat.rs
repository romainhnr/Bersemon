//! Questions posées pendant un combat et à sa suite : action du tour,
//! remplacement d'un monstre KO, capture des KO et attaque à oublier.

use bersemon_core::attaque::Attaque;
use bersemon_core::combat::{Action, Combat, Cote};
use bersemon_core::deroulement::Choix;
use bersemon_core::monstre::Monstre;

use crate::affichage;
use crate::saisie::Entree;

/// Entrées du menu de combat, numérotées dans l'ordre de la liste.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Commande {
    Attaquer,
    Changer,
    LancerFilet,
    UtiliserPotion,
    Fuir,
}

/// Face à un Apôtre, le filet et la fuite ne sont pas proposés (section 9.4).
fn commandes(combat: &Combat) -> Vec<Commande> {
    let mut commandes = vec![Commande::Attaquer, Commande::Changer];
    if combat.filet_possible() {
        commandes.push(Commande::LancerFilet);
    }
    commandes.push(Commande::UtiliserPotion);
    if combat.fuite_possible() {
        commandes.push(Commande::Fuir);
    }
    commandes
}

/// Rend `None` si l'entrée est fermée. Le choix 0 d'un sous-menu ramène au
/// menu principal.
pub fn choisir_action(combat: &Combat, entree: &mut Entree) -> Option<Choix> {
    println!();
    println!("{}", affichage::etat(combat));
    let commandes = commandes(combat);
    loop {
        println!("{}", affichage::menu_action(combat, &commandes));
        let action = match commandes[entree.choix(1, commandes.len())? - 1] {
            Commande::Attaquer => {
                let monstre = combat.monstre(combat.actif(Cote::Joueur));
                println!("{}", affichage::liste_attaques(monstre));
                let choix = entree.choix(0, monstre.attaques().liste().len())?;
                (choix > 0).then(|| Action::Attaquer(choix - 1))
            }
            Commande::Changer => {
                println!("{}", affichage::liste_equipe(combat, true));
                let choix = entree.choix(0, combat.equipe().len())?;
                (choix > 0).then(|| Action::Changer(choix - 1))
            }
            Commande::LancerFilet => Some(Action::LancerFilet),
            // La confirmation d'une potion perdue est demandée par le
            // déroulement (Q53).
            Commande::UtiliserPotion => {
                println!("Soigner quel monstre ?");
                println!("{}", affichage::liste_equipe(combat, true));
                let choix = entree.choix(0, combat.equipe().len())?;
                (choix > 0).then(|| Action::UtiliserPotion(choix - 1))
            }
            Commande::Fuir => Some(Action::Fuir),
        };
        if let Some(action) = action {
            return Some(Choix::Combat(action));
        }
    }
}

pub fn choisir_remplacant(combat: &Combat, entree: &mut Entree) -> Option<Choix> {
    println!();
    println!("Quel monstre envoyer ?");
    println!("{}", affichage::liste_equipe(combat, false));
    let choix = entree.choix(1, combat.equipe().len())?;
    Some(Choix::Remplacer(choix - 1))
}

/// Plusieurs monstres KO peuvent être capturés après une victoire (Q29).
pub fn choisir_capture(combat: &Combat, entree: &mut Entree) -> Option<Choix> {
    let capturables = combat.adversaires_capturables();
    println!();
    println!("Capturer un monstre KO ?");
    println!("{}", affichage::liste_capturables(combat, &capturables));
    let choix = match entree.choix(0, capturables.len())? {
        0 => Choix::NePlusCapturer,
        choix => Choix::Capturer(capturables[choix - 1]),
    };
    Some(choix)
}

/// Le monstre connaît déjà 4 attaques : le joueur en oublie une pour
/// apprendre la nouvelle, ou y renonce (Q33).
pub fn choisir_attaque_a_oublier(
    monstre: &Monstre,
    nouvelle: &Attaque,
    entree: &mut Entree,
) -> Option<Choix> {
    println!("{}", affichage::question_oubli(monstre, nouvelle));
    let choix = match entree.choix(0, monstre.attaques().liste().len())? {
        0 => Choix::RenoncerALAttaque,
        choix => Choix::OublierAttaque(choix - 1),
    };
    Some(choix)
}
