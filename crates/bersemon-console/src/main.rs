//! Interface console de Bersemon : création du héros, introduction et choix
//! du monstre de départ, camp de base, expéditions avec la traque de
//! l'Apôtre, tutoriel et sauvegarde automatique.

mod affichage;
mod ecran_camp;
mod ecran_combat;
mod ecran_creation;
mod ecran_expedition;
mod saisie;
mod tutoriel;

use std::time::{SystemTime, UNIX_EPOCH};

use bersemon_core::aleatoire::{Aleatoire, GenerateurAleatoire};
use bersemon_core::contenu::{ContenuV1, INTRODUCTION};
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::SauvegardeRepository;
use bersemon_storage::{FichierJson, emplacement_par_defaut};

use crate::saisie::Entree;

fn main() {
    let mut aleatoire = GenerateurAleatoire::new(graine_depuis_horloge());
    let mut entree = Entree::depuis_le_clavier();
    let contenu = ContenuV1::new();
    let depot = FichierJson::new(emplacement_par_defaut());

    println!("Bersemon");
    println!("Sauvegarde : {}", depot.chemin().display());
    let Some(mut partie) = demarrer(&depot, &contenu, &mut aleatoire, &mut entree) else {
        println!();
        println!("À bientôt.");
        return;
    };

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
            Some(1) => {
                // Quitter en pleine expédition ramène à la dernière
                // sauvegarde (section 13.9) : on ne sauvegarde qu'au retour.
                let revenu =
                    ecran_expedition::partir(&mut partie, &contenu, &mut aleatoire, &mut entree);
                if revenu {
                    sauvegarder(&depot, &partie);
                }
                revenu
            }
            Some(2) => {
                println!("{}", affichage::collection(partie.collection()));
                true
            }
            Some(3) => ecran_camp::gerer_monstres(&mut partie, &depot, &mut entree),
            Some(4) => {
                println!("{}", affichage::codex(partie.codex(), contenu.especes()));
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

/// Reprend la partie sauvegardée ou en crée une nouvelle. Rien n'est effacé
/// sans l'accord du joueur (section 13.9, Q56). Rend `None` si le joueur
/// quitte.
fn demarrer(
    depot: &FichierJson,
    contenu: &ContenuV1,
    aleatoire: &mut dyn Aleatoire,
    entree: &mut Entree,
) -> Option<Partie> {
    match depot.charger(contenu) {
        Ok(Some(partie)) => loop {
            println!();
            println!(
                "Partie en cours : {}, {}.",
                partie.heros().nom().as_str(),
                partie.heros().classe().nom()
            );
            println!("  1. Continuer cette partie");
            println!("  2. Commencer une nouvelle partie");
            println!("  3. Quitter");
            match entree.choix(1, 3)? {
                1 => return Some(partie),
                2 => {
                    let question = format!(
                        "La partie en cours ({}) sera remplacée. Continuer ?",
                        partie.heros().nom().as_str()
                    );
                    if entree.confirmation(&question)? {
                        break;
                    }
                }
                _ => return None,
            }
        },
        Ok(None) => {}
        Err(erreur) => {
            println!();
            println!(
                "La sauvegarde ne peut pas être chargée : {}.",
                affichage::erreur_sauvegarde(&erreur)
            );
            println!("  1. Commencer une nouvelle partie, qui la remplacera");
            println!("  2. Quitter sans rien changer");
            if entree.choix(1, 2)? != 1 {
                return None;
            }
        }
    }
    let heros = ecran_creation::creer_heros(entree)?;
    // L'introduction mène du choix de la classe à celui du premier monstre
    // (section 6).
    println!();
    println!("{INTRODUCTION}");
    println!();
    let depart = ecran_creation::choisir_depart(contenu.especes_de_depart(), entree, aleatoire)?;
    let partie = Partie::nouvelle(heros, depart);
    sauvegarder(depot, &partie);
    Some(partie)
}

/// Un échec est signalé sans interrompre la partie.
fn sauvegarder(depot: &dyn SauvegardeRepository, partie: &Partie) {
    if let Err(erreur) = depot.enregistrer(partie) {
        println!(
            "Sauvegarde impossible : {}.",
            affichage::erreur_sauvegarde(&erreur)
        );
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
