//! Début de partie : création du héros et choix du monstre de départ
//! (sections 4.1 et 6).

use bersemon_core::aleatoire::Aleatoire;
use bersemon_core::espece::Espece;
use bersemon_core::heros::{Classe, Heros};
use bersemon_core::monstre::Monstre;
use bersemon_core::nom::Nom;
use bersemon_core::partie::creer_monstre_de_depart;

use crate::affichage;
use crate::saisie::Entree;

/// Rend `None` si l'entrée est fermée.
pub fn creer_heros(entree: &mut Entree) -> Option<Heros> {
    let nom = loop {
        let texte = entree.texte("Nom de votre héros : ")?;
        match Nom::new(&texte) {
            Ok(nom) => break nom,
            Err(erreur) => println!("Nom refusé : {erreur}."),
        }
    };
    println!("Classe de votre héros :");
    for (position, classe) in Classe::TOUTES.iter().enumerate() {
        println!("  {}. {}", position + 1, decrire_classe(*classe));
    }
    let choix = entree.choix(1, Classe::TOUTES.len())?;
    Some(Heros::new(nom, Classe::TOUTES[choix - 1]))
}

/// Rend `None` si l'entrée est fermée.
pub fn choisir_depart(
    especes: &[Espece],
    entree: &mut Entree,
    aleatoire: &mut dyn Aleatoire,
) -> Option<Monstre> {
    println!("Choisissez votre premier monstre :");
    println!("{}", affichage::liste_especes(especes));
    let choix = entree.choix(1, especes.len())?;
    let monstre = creer_monstre_de_depart(especes[choix - 1].clone(), aleatoire)
        .expect("monstre de départ créé au niveau 5");
    println!("{} vous accompagne désormais.", monstre.nom());
    Some(monstre)
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
