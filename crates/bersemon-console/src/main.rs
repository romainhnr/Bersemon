//! Interface console de Bersemon. Pour l'instant, elle propose un combat de
//! démonstration contre un groupe de monstres sauvages.

mod affichage;
mod demo;

use std::io::{self, BufRead, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use bersemon_core::aleatoire::GenerateurAleatoire;
use bersemon_core::attaque::Attaque;
use bersemon_core::combat::{Action, Combat, Cote, ErreurCombat, Evenement, Phase};
use bersemon_core::monstre::Monstre;

fn main() {
    let mut aleatoire = GenerateurAleatoire::new(graine_depuis_horloge());
    let (equipe, groupe) = demo::rencontre(&mut aleatoire);
    let mut combat = Combat::new(equipe, groupe).expect("la rencontre de démonstration est valide");
    let mut lignes = io::stdin().lock().lines();

    println!("Bersemon : combat de démonstration, avec un contenu provisoire.");
    println!("{}", affichage::rencontre(&combat));
    loop {
        let resultat = match combat.phase() {
            Phase::ChoixAction => {
                println!();
                println!("{}", affichage::etat(&combat));
                let Some(action) = choisir_action(&combat, &mut lignes) else {
                    break;
                };
                combat.jouer_tour(action, &mut aleatoire)
            }
            Phase::Remplacement => {
                println!();
                println!("Quel monstre envoyer ?");
                println!("{}", affichage::liste_equipe(&combat, false));
                let nombre = combat.equipe().len();
                let Some(choix) = lire_choix(&mut lignes, 1, nombre) else {
                    break;
                };
                combat.remplacer(choix - 1)
            }
            Phase::Termine(_) => break,
        };
        afficher(resultat, &combat);
    }

    // Plusieurs monstres KO peuvent être capturés après une victoire (Q29).
    loop {
        let capturables = combat.adversaires_capturables();
        if capturables.is_empty() {
            break;
        }
        println!();
        println!("Capturer un monstre KO ?");
        println!("{}", affichage::liste_capturables(&combat, &capturables));
        match lire_choix(&mut lignes, 0, capturables.len()) {
            Some(choix) if choix > 0 => {
                let resultat = combat.capturer_ko(capturables[choix - 1]);
                afficher(resultat, &combat);
            }
            _ => break,
        }
    }

    let mut bilan = combat.terminer(&mut aleatoire);
    for (index, rapport) in bilan.experience.iter().enumerate() {
        let Some(rapport) = rapport else {
            continue;
        };
        let monstre = &mut bilan.equipe[index];
        println!();
        println!("{}", affichage::rapport_experience(monstre, rapport));
        for attaque in &rapport.attaques_en_attente {
            choisir_attaque_a_oublier(monstre, attaque, &mut lignes);
        }
    }
    println!();
    println!("{}", affichage::bilan(&bilan));
}

/// Le monstre connaît déjà 4 attaques : le joueur en oublie une pour
/// apprendre la nouvelle, ou y renonce (Q33).
fn choisir_attaque_a_oublier(
    monstre: &mut Monstre,
    nouvelle: &Attaque,
    lignes: &mut impl Iterator<Item = io::Result<String>>,
) {
    println!("{}", affichage::question_oubli(monstre, nouvelle));
    let nombre = monstre.attaques().liste().len();
    let choix = lire_choix(lignes, 0, nombre).unwrap_or(0);
    if choix == 0 {
        println!("{}", affichage::renoncement(monstre, nouvelle));
        return;
    }
    let oubliee = monstre.attaques().liste()[choix - 1].clone();
    match monstre.remplacer_attaque(choix - 1, nouvelle.clone()) {
        Ok(()) => println!("{}", affichage::remplacement(monstre, &oubliee, nouvelle)),
        Err(erreur) => println!("Impossible d'apprendre cette attaque : {erreur}."),
    }
}

/// Rend `None` si l'entrée est fermée. Le choix 0 d'un sous-menu ramène au
/// menu principal.
fn choisir_action(
    combat: &Combat,
    lignes: &mut impl Iterator<Item = io::Result<String>>,
) -> Option<Action> {
    loop {
        println!("{}", affichage::menu_action(combat));
        match lire_choix(lignes, 1, 4)? {
            1 => {
                let monstre = combat.monstre(combat.actif(Cote::Joueur));
                println!("{}", affichage::liste_attaques(monstre));
                let choix = lire_choix(lignes, 0, monstre.attaques().liste().len())?;
                if choix > 0 {
                    return Some(Action::Attaquer(choix - 1));
                }
            }
            2 => {
                println!("{}", affichage::liste_equipe(combat, true));
                let choix = lire_choix(lignes, 0, combat.equipe().len())?;
                if choix > 0 {
                    return Some(Action::Changer(choix - 1));
                }
            }
            3 => return Some(Action::LancerFilet),
            _ => return Some(Action::Fuir),
        }
    }
}

/// Redemande tant que la saisie n'est pas un nombre de `min` à `max`. Rend
/// `None` si l'entrée est fermée.
fn lire_choix(
    lignes: &mut impl Iterator<Item = io::Result<String>>,
    min: usize,
    max: usize,
) -> Option<usize> {
    loop {
        print!("Votre choix ({min} à {max}) : ");
        // Sans vidage explicite, l'invite resterait en mémoire tampon jusqu'au
        // prochain saut de ligne. Un échec ici n'empêche pas de jouer.
        let _ = io::stdout().flush();
        let ligne = lignes.next()?.ok()?;
        match ligne.trim().parse::<usize>() {
            Ok(choix) if (min..=max).contains(&choix) => return Some(choix),
            _ => println!("Choix invalide : entrez un numéro de {min} à {max}."),
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

/// Une graine différente à chaque lancement suffit : le hasard d'un jeu n'a
/// pas besoin d'être imprévisible au sens cryptographique.
fn graine_depuis_horloge() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duree| duree.as_nanos() as u64)
        .unwrap_or(0)
}
