//! L'équipement du héros (section 4.3) : au camp, le joueur porte, change ou
//! retire ses objets, sans confirmation.

use bersemon_core::deroulement::Choix;
use bersemon_core::equipement::PlaceDEquipement;
use bersemon_core::partie::Partie;
use bersemon_presentation::equipement::{etat_de_l_equipement, nom_de_place, objet_et_bonus};

use crate::saisie::Entree;

/// `Some(None)` au retour vers le menu du camp.
pub fn choisir(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let equipement = partie.heros().equipement();
    loop {
        println!();
        println!("{}", etat_de_l_equipement(equipement));
        for (position, place) in PlaceDEquipement::TOUTES.iter().enumerate() {
            println!("  {}. {}", position + 1, nom_de_place(*place));
        }
        println!("  0. Retour au camp");
        let numero = entree.choix(0, PlaceDEquipement::TOUTES.len())?;
        if numero == 0 {
            return Some(None);
        }
        let place = PlaceDEquipement::TOUTES[numero - 1];
        let mut choix: Vec<(Choix, String)> = equipement
            .disponibles(place)
            .into_iter()
            .map(|objet| {
                (
                    Choix::Equiper {
                        place,
                        objet: objet.id().clone(),
                    },
                    format!("Porter : {}", objet_et_bonus(objet)),
                )
            })
            .collect();
        if let Some(porte) = equipement.porte(place) {
            choix.push((
                Choix::RetirerEquipement(place),
                format!("Retirer : {}", porte.nom()),
            ));
        }
        println!();
        if choix.is_empty() {
            println!("{} : aucun objet à porter.", nom_de_place(place));
            continue;
        }
        println!("{} :", nom_de_place(place));
        for (position, (_, libelle)) in choix.iter().enumerate() {
            println!("  {}. {libelle}", position + 1);
        }
        println!("  0. Retour");
        let numero = entree.choix(0, choix.len())?;
        if numero > 0 {
            return Some(Some(choix.swap_remove(numero - 1).0));
        }
    }
}
