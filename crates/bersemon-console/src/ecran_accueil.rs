//! Début de partie : reprise de la sauvegarde ou nouvelle partie, création
//! du héros et choix du monstre de départ (sections 4.1, 6 et 13.9).

use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::heros::Classe;
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::ErreurSauvegarde;

use crate::affichage;
use crate::saisie::Entree;

/// Rend `None` si le joueur quitte ou que l'entrée est fermée.
pub fn reprise(partie: &Partie, entree: &mut Entree) -> Option<Choix> {
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
        1 => Some(Choix::ReprendreLaPartie),
        2 => Some(Choix::NouvellePartie),
        _ => None,
    }
}

/// Rien n'est effacé sans l'accord du joueur (Q56). Rend `None` si le joueur
/// quitte ou que l'entrée est fermée.
pub fn sauvegarde_illisible(erreur: &ErreurSauvegarde, entree: &mut Entree) -> Option<Choix> {
    println!();
    println!(
        "La sauvegarde ne peut pas être chargée : {}.",
        affichage::erreur_sauvegarde(erreur)
    );
    println!("  1. Commencer une nouvelle partie, qui la remplacera");
    println!("  2. Quitter sans rien changer");
    (entree.choix(1, 2)? == 1).then_some(Choix::NouvellePartie)
}

pub fn nom_du_heros(entree: &mut Entree) -> Option<Choix> {
    entree.texte("Nom de votre héros : ").map(Choix::NomDuHeros)
}

pub fn classe(entree: &mut Entree) -> Option<Choix> {
    println!("Classe de votre héros :");
    for (position, classe) in Classe::TOUTES.iter().enumerate() {
        println!("  {}. {}", position + 1, decrire_classe(*classe));
    }
    let choix = entree.choix(1, Classe::TOUTES.len())?;
    Some(Choix::Classe(Classe::TOUTES[choix - 1]))
}

pub fn monstre_de_depart(contenu: &ContenuV1, entree: &mut Entree) -> Option<Choix> {
    let especes = contenu.especes_de_depart();
    println!("Choisissez votre premier monstre :");
    println!("{}", affichage::liste_especes(especes));
    let choix = entree.choix(1, especes.len())?;
    Some(Choix::MonstreDeDepart(especes[choix - 1].id().clone()))
}

fn decrire_classe(classe: Classe) -> String {
    let stat = match classe {
        Classe::Guerrier => "Attaque",
        Classe::Mage => "Attaque spéciale",
    };
    format!(
        "{} : +10 % d'{stat} pour vos monstres en combat",
        classe.nom()
    )
}
