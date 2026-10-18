//! Mise en page, pour la console, du récit des événements : une ligne vide
//! avant chaque nouveau paragraphe.

use bersemon_core::deroulement::Evenement;
use bersemon_presentation::recit::raconter as recits;

pub fn raconter(evenements: &[Evenement]) {
    for (evenement, recit) in evenements.iter().zip(recits(evenements)) {
        if recit.nouveau_paragraphe {
            println!();
        }
        println!("{}", recit.texte);
        // L'introduction est séparée de la question qui la suit.
        if *evenement == Evenement::Introduction {
            println!();
        }
    }
}
