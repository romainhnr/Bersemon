//! L'Éclipse sur Discord (section 9.4) : avant le premier départ, le choix
//! du monstre que réclame le Béhérit ; ensuite, le départ.

use bersemon_core::beherit::EtatDuBeherit;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::partie::Partie;
use bersemon_presentation::beherit::{
    DEMANDE_DE_SACRIFICE, PARTIR_VERS_L_ECLIPSE, destination_de_l_eclipse,
};

use crate::composants::Cle;
use crate::ecran_camp::{menus_de_monstres, retour};
use crate::message::{PRINCIPAL, Question, Rangee, bouton, rangees};

pub fn eclipse(partie: &Partie, contenu: &ContenuV1) -> Question {
    let Some(destination) = destination_de_l_eclipse(partie, contenu) else {
        return Question {
            texte: "L'Éclipse est fermée.".to_string(),
            rangees: rangees(vec![retour()]),
        };
    };
    let mut lignes: Vec<Rangee> = Vec::new();
    let texte = if partie.beherit() == EtatDuBeherit::Forge {
        // Les monstres que le Béhérit refuserait ne sont pas proposés ; le
        // sacrifice est confirmé par le déroulement.
        let sacrifiables = partie.monstres_sacrifiables();
        lignes.extend(menus_de_monstres(
            partie.collection(),
            Cle::Sacrifier,
            "Sacrifier",
            |emplacement| sacrifiables.contains(&emplacement),
        ));
        let aucun = if sacrifiables.is_empty() {
            "\nAucun de vos monstres ne convient."
        } else {
            ""
        };
        format!("{destination}.\n{DEMANDE_DE_SACRIFICE}{aucun}")
    } else {
        lignes.extend(rangees(vec![bouton(
            Cle::PartirVersLEclipse,
            PARTIR_VERS_L_ECLIPSE,
            PRINCIPAL,
        )]));
        format!("{destination}.")
    };
    lignes.extend(rangees(vec![retour()]));
    Question {
        texte,
        rangees: lignes,
    }
}
