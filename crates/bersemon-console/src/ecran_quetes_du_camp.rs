//! Les quêtes des personnages du camp (section 13.13) : écouter une demande,
//! remettre une quête.

use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::partie::Partie;
use bersemon_core::quete_du_camp::EtatDeLaChaine;
use bersemon_presentation::quetes_du_camp::{chaines, ecran_des_quetes_du_camp, peut_remettre};

use crate::saisie::Entree;

/// `Some(None)` au retour vers le menu du camp.
pub fn choisir(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    println!();
    println!("{}", ecran_des_quetes_du_camp(partie, contenu));
    let actions: Vec<(Choix, String)> = chaines(partie, contenu)
        .into_iter()
        .filter_map(|(personnage, etat)| match etat {
            EtatDeLaChaine::Proposee(_) => Some((
                Choix::EcouterQuete(personnage),
                format!("Écouter {}", personnage.nom()),
            )),
            EtatDeLaChaine::EnCours(_) if peut_remettre(partie, contenu, personnage) => Some((
                Choix::RemettreQuete(personnage),
                format!("Remettre à {}", personnage.nom()),
            )),
            _ => None,
        })
        .collect();
    for (position, (_, libelle)) in actions.iter().enumerate() {
        println!("  {}. {libelle}", position + 1);
    }
    println!("  0. Retour au camp");
    let numero = entree.choix(0, actions.len())?;
    Some((numero > 0).then(|| actions[numero - 1].0.clone()))
}
