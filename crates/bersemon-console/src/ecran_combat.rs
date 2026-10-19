//! Questions posées pendant un combat et à sa suite : action du tour,
//! remplacement d'un monstre KO, capture des KO, attaque à oublier et
//! évolution.

use bersemon_core::attaque::Attaque;
use bersemon_core::collection::Collection;
use bersemon_core::combat::{Action, Combat, Cote};
use bersemon_core::deroulement::Choix;
use bersemon_core::espece::Espece;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::soin::{Soin, Soins};
use bersemon_presentation::partie::{places_pour_une_capture, proposition_d_evolution};
use bersemon_presentation::soins::{effet_du_soin, nom_du_soin};

use crate::affichage;
use crate::saisie::Entree;

/// Entrées du menu de combat, numérotées dans l'ordre de la liste.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Commande {
    Attaquer,
    Changer,
    LancerFilet,
    UtiliserSoin,
    Fuir,
}

/// Face à un Apôtre, le filet et la fuite ne sont pas proposés (section 9.4).
fn commandes(combat: &Combat) -> Vec<Commande> {
    let mut commandes = vec![Commande::Attaquer, Commande::Changer];
    if combat.filet_possible() {
        commandes.push(Commande::LancerFilet);
    }
    commandes.push(Commande::UtiliserSoin);
    if combat.fuite_possible() {
        commandes.push(Commande::Fuir);
    }
    commandes
}

/// Rend `None` si l'entrée est fermée. Le choix 0 d'un sous-menu ramène au
/// menu principal. Les types d'un adversaire ne s'affichent que si son
/// espèce est capturée (section 10.6).
pub fn choisir_action(combat: &Combat, partie: &Partie, entree: &mut Entree) -> Option<Choix> {
    println!();
    println!(
        "{}",
        bersemon_presentation::combat::etat(combat, partie.codex())
    );
    if combat.filet_possible() {
        println!("{}", places_pour_une_capture(partie.collection()));
    }
    let commandes = commandes(combat);
    loop {
        println!("{}", affichage::menu_action(combat, &commandes));
        let action = match commandes[entree.choix(1, commandes.len())? - 1] {
            Commande::Attaquer => {
                let monstre = combat.monstre(combat.actif(Cote::Joueur));
                println!("{}", affichage::liste_attaques(combat));
                let choix = entree.choix(0, monstre.attaques().liste().len())?;
                (choix > 0).then(|| Action::Attaquer(choix - 1))
            }
            Commande::Changer => {
                println!("{}", affichage::liste_equipe(combat, true));
                let choix = entree.choix(0, combat.equipe().len())?;
                (choix > 0).then(|| Action::Changer(choix - 1))
            }
            Commande::LancerFilet => Some(Action::LancerFilet),
            // La confirmation d'un soin perdu est demandée par le
            // déroulement (Q53).
            Commande::UtiliserSoin => match choisir_soin(combat.soins(), entree)? {
                Some(soin) => {
                    println!("Soigner quel monstre ?");
                    println!("{}", affichage::liste_equipe(combat, true));
                    let choix = entree.choix(0, combat.equipe().len())?;
                    (choix > 0).then(|| Action::UtiliserSoin(soin, choix - 1))
                }
                None => None,
            },
            Commande::Fuir => Some(Action::Fuir),
        };
        if let Some(action) = action {
            return Some(Choix::Combat(action));
        }
    }
}

/// Parmi ceux du sac, en combat ou entre deux étapes. `Some(None)` s'il
/// n'y en a aucun ou si le joueur annule.
pub fn choisir_soin(soins: &Soins, entree: &mut Entree) -> Option<Option<Soin>> {
    let presents: Vec<(Soin, u32)> = soins.presents().collect();
    if presents.is_empty() {
        println!("Vous n'avez aucun soin.");
        return Some(None);
    }
    println!("Quel soin utiliser ?");
    for (position, (soin, nombre)) in presents.iter().enumerate() {
        println!(
            "  {}. {} ({nombre}) : {}",
            position + 1,
            nom_du_soin(*soin),
            effet_du_soin(*soin)
        );
    }
    println!("  0. Retour");
    let choix = entree.choix(0, presents.len())?;
    Some((choix > 0).then(|| presents[choix - 1].0))
}

pub fn choisir_remplacant(combat: &Combat, entree: &mut Entree) -> Option<Choix> {
    println!();
    println!("Quel monstre envoyer ?");
    println!("{}", affichage::liste_equipe(combat, false));
    let choix = entree.choix(1, combat.equipe().len())?;
    Some(Choix::Remplacer(choix - 1))
}

/// Plusieurs monstres KO peuvent être capturés après une victoire (Q29).
pub fn choisir_capture(
    combat: &Combat,
    collection: &Collection,
    entree: &mut Entree,
) -> Option<Choix> {
    let capturables = combat.adversaires_capturables();
    println!();
    println!("Capturer un monstre KO ?");
    println!("{}", places_pour_une_capture(collection));
    println!("{}", affichage::liste_capturables(combat, &capturables));
    let choix = match entree.choix(0, capturables.len())? {
        0 => Choix::NePlusCapturer,
        choix => Choix::Capturer(capturables[choix - 1]),
    };
    Some(choix)
}

/// Le monstre connaît déjà 4 attaques : le joueur en oublie une pour
/// apprendre la nouvelle, ou y renonce (Q33).
/// Refusée, l'évolution sera proposée de nouveau au prochain niveau gagné
/// (section 5.4).
pub fn choisir_evolution(monstre: &Monstre, vers: &Espece, entree: &mut Entree) -> Option<Choix> {
    let choix = if entree.confirmation(&proposition_d_evolution(monstre, vers))? {
        Choix::Evoluer
    } else {
        Choix::NePasEvoluer
    };
    Some(choix)
}

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
