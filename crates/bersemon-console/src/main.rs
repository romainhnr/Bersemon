//! Interface console de Bersemon. Pour l'instant : création du héros, choix
//! du monstre de départ, camp de base et expéditions dans les zones de
//! démonstration.

mod affichage;
mod demo;
mod ecran_camp;
mod ecran_combat;
mod ecran_creation;
mod ecran_expedition;
mod saisie;

use std::time::{SystemTime, UNIX_EPOCH};

use bersemon_core::aleatoire::GenerateurAleatoire;
use bersemon_core::partie::Partie;

use crate::saisie::Entree;

fn main() {
    let mut aleatoire = GenerateurAleatoire::new(graine_depuis_horloge());
    let mut entree = Entree::depuis_le_clavier();
    let zones = demo::zones();
    let especes = demo::toutes_les_especes();

    println!("Bersemon : version de démonstration, avec un contenu provisoire.");
    let Some(heros) = ecran_creation::creer_heros(&mut entree) else {
        return;
    };
    let Some(depart) = ecran_creation::choisir_depart(&mut entree, &mut aleatoire) else {
        return;
    };
    let mut partie = Partie::nouvelle(heros, depart);

    loop {
        println!();
        println!(
            "Camp de base ({}, {}). {}.",
            partie.heros().nom().as_str(),
            partie.heros().classe().nom(),
            affichage::potions(partie.inventaire().potions())
        );
        println!("  1. Partir en expédition");
        println!("  2. Voir vos monstres");
        println!("  3. Gérer vos monstres");
        println!("  4. Consulter le codex");
        println!("  5. Quitter");
        let continuer = match entree.choix(1, 5) {
            Some(1) => ecran_expedition::partir(&mut partie, &zones, &mut aleatoire, &mut entree),
            Some(2) => {
                println!("{}", affichage::collection(partie.collection()));
                true
            }
            Some(3) => ecran_camp::gerer_monstres(partie.collection_mut(), &mut entree),
            Some(4) => {
                println!("{}", affichage::codex(partie.codex(), &especes));
                true
            }
            _ => false,
        };
        if !continuer {
            break;
        }
    }
    println!();
    println!("À bientôt.");
}

/// Une graine différente à chaque lancement suffit : le hasard d'un jeu n'a
/// pas besoin d'être imprévisible au sens cryptographique.
fn graine_depuis_horloge() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duree| duree.as_nanos() as u64)
        .unwrap_or(0)
}
