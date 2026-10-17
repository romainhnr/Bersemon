//! Interface console de Bersemon. Pour l'instant, elle propose un combat de
//! démonstration contre un monstre sauvage.

mod affichage;
mod demo;

use std::io::{self, BufRead, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use bersemon_core::aleatoire::GenerateurAleatoire;
use bersemon_core::combat::{Action, Combat, Cote};

fn main() {
    let mut aleatoire = GenerateurAleatoire::new(graine_depuis_horloge());
    let (joueur, adversaire) = demo::monstres(&mut aleatoire);
    let mut combat =
        Combat::new(joueur, adversaire).expect("les monstres de démonstration ont tous leurs PV");
    let mut lignes = io::stdin().lock().lines();

    println!("Bersemon : combat de démonstration, avec un contenu provisoire.");
    println!(
        "{} apparaît !",
        affichage::nom(combat.adversaire(), Cote::Adversaire)
    );
    while combat.issue().is_none() {
        println!();
        println!("{}", affichage::etat(&combat));
        let Some(index) = demander_attaque(&combat, &mut lignes) else {
            println!();
            println!("Saisie terminée : combat abandonné.");
            return;
        };
        match combat.jouer_tour(Action::Attaquer(index), &mut aleatoire) {
            Ok(evenements) => {
                for evenement in &evenements {
                    println!("{}", affichage::evenement(evenement, &combat));
                }
            }
            Err(erreur) => println!("Action impossible : {erreur}."),
        }
    }
}

/// Rend la position de l'attaque choisie, ou None si l'entrée est fermée.
fn demander_attaque(
    combat: &Combat,
    lignes: &mut impl Iterator<Item = io::Result<String>>,
) -> Option<usize> {
    let nombre = combat.joueur().attaques().liste().len();
    println!("{}", affichage::liste_attaques(combat.joueur()));
    loop {
        print!("Votre attaque (1 à {nombre}) : ");
        // Sans vidage explicite, l'invite resterait en mémoire tampon jusqu'au
        // prochain saut de ligne. Un échec ici n'empêche pas de jouer.
        let _ = io::stdout().flush();
        let ligne = lignes.next()?.ok()?;
        match ligne.trim().parse::<usize>() {
            Ok(choix) if (1..=nombre).contains(&choix) => return Some(choix - 1),
            _ => println!("Choix invalide : entrez un numéro de 1 à {nombre}."),
        }
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
