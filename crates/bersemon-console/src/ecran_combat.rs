//! Combat contre un groupe sauvage, puis ses suites : XP, apprentissage
//! d'attaques, placement des captures et retour au camp.

use bersemon_core::aleatoire::Aleatoire;
use bersemon_core::attaque::Attaque;
use bersemon_core::collection::{Collection, Emplacement, Placement};
use bersemon_core::combat::{Action, Combat, Cote, ErreurCombat, Evenement, Phase};
use bersemon_core::heros::Heros;
use bersemon_core::monstre::Monstre;

use crate::affichage;
use crate::demo;
use crate::saisie::Entree;

/// Rend `false` si l'entrée a été fermée : la partie s'arrête alors.
pub fn combattre(
    heros: &Heros,
    collection: &mut Collection,
    aleatoire: &mut dyn Aleatoire,
    entree: &mut Entree,
) -> bool {
    let groupe = demo::groupe_sauvage(aleatoire);
    // Au camp, tous les monstres sont soignés : l'équipe peut combattre.
    let mut combat = Combat::new(collection.equipe().to_vec(), groupe)
        .expect("équipe soignée et groupe sauvage valide")
        .avec_bonus_du_joueur(heros.classe().bonus());
    println!();
    println!("{}", affichage::rencontre(&combat));
    let mut entree_ouverte =
        jouer(&mut combat, aleatoire, entree) && capturer_les_ko(&mut combat, entree);

    let mut bilan = combat.terminer(aleatoire);
    for (index, rapport) in bilan.experience.iter().enumerate() {
        let Some(rapport) = rapport else {
            continue;
        };
        let monstre = &mut bilan.equipe[index];
        println!();
        println!("{}", affichage::rapport_experience(monstre, rapport));
        for attaque in &rapport.attaques_en_attente {
            entree_ouverte = entree_ouverte && choisir_attaque_a_oublier(monstre, attaque, entree);
        }
    }
    collection
        .actualiser_equipe(bilan.equipe)
        .expect("le combat rend l'équipe qu'il a reçue");
    for capture in bilan.captures {
        entree_ouverte = placer_capture(collection, capture, entree_ouverte, entree);
    }

    // Retour au camp, où les monstres sont soignés, même après une défaite
    // (sections 9.3 et 13.1).
    collection.soigner_tout();
    println!();
    println!("Retour au camp : vos monstres sont soignés.");
    entree_ouverte
}

fn jouer(combat: &mut Combat, aleatoire: &mut dyn Aleatoire, entree: &mut Entree) -> bool {
    loop {
        let resultat = match combat.phase() {
            Phase::ChoixAction => {
                println!();
                println!("{}", affichage::etat(combat));
                let Some(action) = choisir_action(combat, entree) else {
                    return false;
                };
                combat.jouer_tour(action, aleatoire)
            }
            Phase::Remplacement => {
                println!();
                println!("Quel monstre envoyer ?");
                println!("{}", affichage::liste_equipe(combat, false));
                let Some(choix) = entree.choix(1, combat.equipe().len()) else {
                    return false;
                };
                combat.remplacer(choix - 1)
            }
            Phase::Termine(_) => return true,
        };
        afficher(resultat, combat);
    }
}

/// Plusieurs monstres KO peuvent être capturés après une victoire (Q29).
fn capturer_les_ko(combat: &mut Combat, entree: &mut Entree) -> bool {
    loop {
        let capturables = combat.adversaires_capturables();
        if capturables.is_empty() {
            return true;
        }
        println!();
        println!("Capturer un monstre KO ?");
        println!("{}", affichage::liste_capturables(combat, &capturables));
        match entree.choix(0, capturables.len()) {
            None => return false,
            Some(0) => return true,
            Some(choix) => {
                let resultat = combat.capturer_ko(capturables[choix - 1]);
                afficher(resultat, combat);
            }
        }
    }
}

/// Rend `None` si l'entrée est fermée. Le choix 0 d'un sous-menu ramène au
/// menu principal.
fn choisir_action(combat: &Combat, entree: &mut Entree) -> Option<Action> {
    loop {
        println!("{}", affichage::menu_action(combat));
        match entree.choix(1, 4)? {
            1 => {
                let monstre = combat.monstre(combat.actif(Cote::Joueur));
                println!("{}", affichage::liste_attaques(monstre));
                let choix = entree.choix(0, monstre.attaques().liste().len())?;
                if choix > 0 {
                    return Some(Action::Attaquer(choix - 1));
                }
            }
            2 => {
                println!("{}", affichage::liste_equipe(combat, true));
                let choix = entree.choix(0, combat.equipe().len())?;
                if choix > 0 {
                    return Some(Action::Changer(choix - 1));
                }
            }
            3 => return Some(Action::LancerFilet),
            _ => return Some(Action::Fuir),
        }
    }
}

/// Le monstre connaît déjà 4 attaques : le joueur en oublie une pour
/// apprendre la nouvelle, ou y renonce (Q33). Rend `false` si l'entrée est
/// fermée, le monstre renonçant alors à l'attaque.
fn choisir_attaque_a_oublier(
    monstre: &mut Monstre,
    nouvelle: &Attaque,
    entree: &mut Entree,
) -> bool {
    println!("{}", affichage::question_oubli(monstre, nouvelle));
    let choix = entree.choix(0, monstre.attaques().liste().len());
    let Some(position) = choix.filter(|&choix| choix > 0).map(|choix| choix - 1) else {
        println!("{}", affichage::renoncement(monstre, nouvelle));
        return choix.is_some();
    };
    let oubliee = monstre.attaques().liste()[position].clone();
    match monstre.remplacer_attaque(position, nouvelle.clone()) {
        Ok(()) => println!("{}", affichage::remplacement(monstre, &oubliee, nouvelle)),
        Err(erreur) => println!("Impossible d'apprendre cette attaque : {erreur}."),
    }
    true
}

/// Dans l'équipe s'il y a de la place, sinon au camp ; si tout est plein, le
/// joueur relâche un monstre ou la capture (Q12). Sans entrée, la capture est
/// relâchée faute de place. Rend `false` si l'entrée est fermée.
fn placer_capture(
    collection: &mut Collection,
    capture: Monstre,
    entree_ouverte: bool,
    entree: &mut Entree,
) -> bool {
    let capture = match collection.accueillir_capture(capture) {
        Placement::Equipe(index) => {
            println!("{} rejoint votre équipe.", collection.equipe()[index].nom());
            return entree_ouverte;
        }
        Placement::Camp(index) => {
            println!("{} part pour le camp.", collection.camp()[index].nom());
            return entree_ouverte;
        }
        Placement::PlaceManquante(capture) => capture,
    };
    if !entree_ouverte {
        return false;
    }
    loop {
        println!();
        println!(
            "Votre équipe et le camp sont pleins. Quel monstre relâcher pour garder {} ?",
            capture.nom()
        );
        println!("{}", affichage::collection(collection));
        println!("  0. Relâcher {}", capture.nom());
        let taille_equipe = collection.equipe().len();
        let total = taille_equipe + collection.camp().len();
        let Some(choix) = entree.choix(0, total) else {
            return false;
        };
        if choix == 0 {
            println!("Vous relâchez {}.", capture.nom());
            return true;
        }
        let emplacement = if choix <= taille_equipe {
            Emplacement::Equipe(choix - 1)
        } else {
            Emplacement::Camp(choix - 1 - taille_equipe)
        };
        let nom = collection
            .monstre(emplacement)
            .map(|monstre| monstre.nom().to_string())
            .unwrap_or_default();
        match entree.confirmation(&format!("Relâcher définitivement {nom} ?")) {
            None => return false,
            Some(false) => continue,
            Some(true) => {}
        }
        let nom_capture = capture.nom().to_string();
        match collection.relacher_pour_capture(emplacement, capture) {
            Ok(relache) => {
                println!(
                    "Vous relâchez {}. {nom_capture} prend sa place.",
                    relache.nom()
                );
                return true;
            }
            Err(erreur) => {
                println!("{}", affichage::erreur_collection(&erreur));
                return true;
            }
        }
    }
}

fn afficher(resultat: Result<Vec<Evenement>, ErreurCombat>, combat: &Combat) {
    match resultat {
        Ok(evenements) => {
            for evenement in &evenements {
                println!("{}", affichage::evenement(evenement, combat));
            }
        }
        Err(erreur) => println!("{}", affichage::erreur(&erreur)),
    }
}
