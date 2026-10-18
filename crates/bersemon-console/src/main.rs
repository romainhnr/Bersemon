//! Interface console de Bersemon. Le déroulement de la partie vient du noyau
//! (`bersemon_core::deroulement`) : la console pose ses questions, avec leurs
//! sous-menus, lui transmet les choix du joueur et raconte ce qui s'est
//! passé.

mod affichage;
mod ecran_accueil;
mod ecran_camp;
mod ecran_combat;
mod ecran_expedition;
mod recit;
mod saisie;

use std::time::{SystemTime, UNIX_EPOCH};

use bersemon_core::aleatoire::GenerateurAleatoire;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{Attente, Choix, Confirmation, Contexte, Deroulement};
use bersemon_core::sauvegarde::SauvegardeRepository;
use bersemon_storage::{FichierJson, emplacement_par_defaut};

use crate::ecran_camp::MenuDuCamp;
use crate::saisie::Entree;

/// Au camp et en expédition, une partie est en jeu.
const PARTIE_EN_JEU: &str = "une partie est en jeu";

fn main() {
    let mut aleatoire = GenerateurAleatoire::new(graine_depuis_horloge());
    let mut entree = Entree::depuis_le_clavier();
    let contenu = ContenuV1::new();
    let depot = FichierJson::new(emplacement_par_defaut());

    println!("Bersemon");
    println!("Sauvegarde : {}", depot.chemin().display());
    let mut deroulement = Deroulement::demarrer(depot.charger(&contenu));
    let mut menu = MenuDuCamp::default();
    while let Some(choix) = demander(&deroulement, &contenu, &mut menu, &mut entree) {
        let mut contexte = Contexte {
            contenu: &contenu,
            aleatoire: &mut aleatoire,
            depot: &depot,
        };
        match deroulement.repondre(choix.clone(), &mut contexte) {
            Ok(evenements) => recit::raconter(&evenements),
            Err(erreur) => println!("{}", affichage::erreur_de_choix(&choix, &erreur)),
        }
    }
    println!();
    println!("À bientôt.");
}

/// La question posée, avec ses sous-menus. Rend `None` quand le joueur
/// quitte ou que l'entrée est fermée : quitter en pleine expédition ramène à
/// la dernière sauvegarde (section 13.9).
fn demander(
    deroulement: &Deroulement,
    contenu: &ContenuV1,
    menu: &mut MenuDuCamp,
    entree: &mut Entree,
) -> Option<Choix> {
    match deroulement.attente() {
        Attente::Reprise(partie) => ecran_accueil::reprise(partie, entree),
        Attente::SauvegardeIllisible(erreur) => ecran_accueil::sauvegarde_illisible(erreur, entree),
        Attente::NomDuHeros => ecran_accueil::nom_du_heros(entree),
        Attente::Classe => ecran_accueil::classe(entree),
        Attente::MonstreDeDepart => ecran_accueil::monstre_de_depart(contenu, entree),
        Attente::Camp => {
            let partie = deroulement.partie().expect(PARTIE_EN_JEU);
            ecran_camp::choisir(partie, contenu, menu, entree)
        }
        Attente::EntreDeuxEtapes => {
            let partie = deroulement.partie().expect(PARTIE_EN_JEU);
            ecran_expedition::entre_deux_etapes(partie, entree)
        }
        Attente::MonstreRare(_) => ecran_expedition::monstre_rare(entree),
        Attente::ActionDeCombat(combat) => ecran_combat::choisir_action(combat, entree),
        Attente::Remplacement(combat) => ecran_combat::choisir_remplacant(combat, entree),
        Attente::CaptureDesKo(combat) => ecran_combat::choisir_capture(combat, entree),
        Attente::OubliAttaque { monstre, nouvelle } => {
            ecran_combat::choisir_attaque_a_oublier(monstre, nouvelle, entree)
        }
        Attente::PlaceManquante(capture) => {
            let collection = deroulement.partie().expect(PARTIE_EN_JEU).collection();
            ecran_expedition::place_manquante(capture, collection, entree)
        }
        Attente::Confirmation(confirmation) => confirmer(confirmation, entree),
    }
}

fn confirmer(confirmation: &Confirmation, entree: &mut Entree) -> Option<Choix> {
    let question = match confirmation {
        Confirmation::RemplacerLaPartie(heros) => format!(
            "La partie en cours ({}) sera remplacée. Continuer ?",
            heros.nom().as_str()
        ),
        Confirmation::PotionPerdue(monstre) => affichage::potion_perdue(monstre),
        Confirmation::Relacher(monstre) => format!("Relâcher définitivement {} ?", monstre.nom()),
    };
    let reponse = if entree.confirmation(&question)? {
        Choix::Oui
    } else {
        Choix::Non
    };
    Some(reponse)
}

/// Une graine différente à chaque lancement suffit : le hasard d'un jeu n'a
/// pas besoin d'être imprévisible au sens cryptographique.
fn graine_depuis_horloge() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duree| duree.as_nanos() as u64)
        .unwrap_or(0)
}
