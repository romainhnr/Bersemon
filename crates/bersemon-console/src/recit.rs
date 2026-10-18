//! Mise en page, pour la console, du récit des événements : une ligne vide
//! avant chaque nouveau paragraphe.

use bersemon_core::deroulement::Evenement;
use bersemon_presentation::mise_en_valeur::sans_marques;
use bersemon_presentation::recit::raconter as recits;

/// Les marques de mise en valeur, faites pour Discord, sont retirées : la
/// console reste en texte simple (section 2).
pub fn raconter(evenements: &[Evenement]) {
    for (evenement, recit) in evenements.iter().zip(recits(evenements)) {
        if recit.nouveau_paragraphe {
            println!();
        }
        println!("{}", sans_marques(&recit.texte));
        // L'introduction est séparée de la question qui la suit.
        if matches!(evenement, Evenement::Introduction(_)) {
            println!();
        }
    }
}
