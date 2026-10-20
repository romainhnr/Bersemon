//! La pêche d'Ansgar sur Discord (section 13.8) : l'état de la ligne, de
//! quoi la lancer ou la relever, puis le monstre pêché, gardé ou relâché.

use bersemon_core::horodatage::Horodatage;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_presentation::monstres::fiche_du_monstre;
use bersemon_presentation::peche::{etat_de_la_ligne, question_de_la_prise};

use crate::composants::Cle;
use crate::ecran_camp::retour;
use crate::message::{PRINCIPAL, Question, SECONDAIRE, SUCCES, bouton, rangees};

/// Une ligne à l'eau qui n'est pas encore prête ne propose que le retour.
pub fn peche(partie: &Partie, maintenant: Horodatage) -> Question {
    if !partie.peche().pecheur_au_camp() {
        return Question {
            texte: "Personne ne pêche au camp.".to_string(),
            rangees: rangees(vec![retour()]),
        };
    }
    let mut boutons = Vec::new();
    match partie.peche().ligne() {
        None => boutons.push(bouton(Cle::LancerLaLigne, "Lancer la ligne", PRINCIPAL)),
        Some(_) if partie.peche().est_prete(maintenant) => {
            boutons.push(bouton(Cle::ReleverLaLigne, "Relever la ligne", SUCCES));
        }
        Some(_) => {}
    }
    boutons.push(retour());
    Question {
        texte: format!("Pêche d'Ansgar :\n{}", etat_de_la_ligne(partie, maintenant)),
        rangees: rangees(boutons),
    }
}

/// Le monstre pêché, avec sa fiche, que le joueur garde ou relâche.
pub fn prise(monstre: &Monstre) -> Question {
    Question {
        texte: format!(
            "{}\n\n{}",
            fiche_du_monstre(monstre),
            question_de_la_prise(monstre)
        ),
        rangees: rangees(vec![
            bouton(Cle::GarderLaPrise, "Garder", PRINCIPAL),
            bouton(Cle::RelacherLaPrise, "Relâcher", SECONDAIRE),
        ]),
    }
}
