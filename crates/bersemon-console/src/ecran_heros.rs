//! L'écran du héros (sections 4.3 et 4.5) : sa fiche, son équipement, ses
//! succès et le titre qu'il porte. La console n'affiche pas de sprite : elle
//! ne propose pas de couleur.

use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::partie::Partie;
use bersemon_presentation::equipement::fiche_du_heros;
use bersemon_presentation::succes::{ecran_des_succes, titre_du_heros, trophees_du_camp};

use crate::ecran_equipement;
use crate::saisie::Entree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntreeDuHeros {
    Equipement,
    Succes,
    Titre,
}

/// `Some(None)` au retour vers le menu du camp.
pub fn choisir(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    loop {
        println!();
        println!("{}", fiche_du_heros(partie.heros()));
        println!("{}", titre_du_heros(partie.heros(), contenu.succes()));
        // L'équipement vient de la forge (section 13.5).
        let mut entrees = Vec::new();
        if partie.forge().est_au_camp() || !partie.heros().equipement().possedes().is_empty() {
            entrees.push((EntreeDuHeros::Equipement, "Équipement du héros"));
        }
        entrees.push((EntreeDuHeros::Succes, "Succès et trophées du camp"));
        entrees.push((EntreeDuHeros::Titre, "Porter un titre"));
        for (position, (_, libelle)) in entrees.iter().enumerate() {
            println!("  {}. {libelle}", position + 1);
        }
        println!("  0. Retour au camp");
        let numero = entree.choix(0, entrees.len())?;
        if numero == 0 {
            return Some(None);
        }
        match entrees[numero - 1].0 {
            EntreeDuHeros::Equipement => {
                if let Some(choix) = ecran_equipement::choisir(partie, entree)? {
                    return Some(Some(choix));
                }
            }
            EntreeDuHeros::Succes => {
                println!();
                println!("{}", ecran_des_succes(partie, contenu.succes()));
                println!();
                println!("{}", trophees_du_camp(partie, contenu.succes()));
            }
            EntreeDuHeros::Titre => {
                if let Some(choix) = choisir_un_titre(partie, contenu, entree)? {
                    return Some(Some(choix));
                }
            }
        }
    }
}

/// Les titres des succès obtenus, et aucun titre.
fn choisir_un_titre(
    partie: &Partie,
    contenu: &ContenuV1,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    let titres: Vec<_> = contenu
        .succes()
        .iter()
        .filter(|un| partie.succes_obtenus().contains(un.id()))
        .collect();
    println!();
    if titres.is_empty() {
        println!("Aucun succès obtenu : vous n'avez pas encore de titre à porter.");
        return Some(None);
    }
    println!("Quel titre porter ?");
    for (position, un) in titres.iter().enumerate() {
        println!("  {}. {}", position + 1, un.titre());
    }
    let aucun = titres.len() + 1;
    println!("  {aucun}. Ne porter aucun titre");
    println!("  0. Retour");
    let numero = entree.choix(0, aucun)?;
    Some(match numero {
        0 => None,
        numero if numero == aucun => Some(Choix::PorterTitre(None)),
        numero => Some(Choix::PorterTitre(Some(titres[numero - 1].id().clone()))),
    })
}
