//! Expédition sur Discord (section 9.2) : entre deux étapes, devant un
//! monstre rare pacifique, et quand une capture ne trouve pas de place.

use bersemon_core::collection::Collection;
use bersemon_core::combat::Cote;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_presentation::monstres::ligne_monstre;
use bersemon_presentation::partie::{
    collection as liste_des_monstres, place_manquante as texte_de_place_manquante,
};

use crate::composants::Cle;
use crate::ecran_camp::{menus_de_monstres, options_d_equipe};
use crate::message::{PRINCIPAL, Question, SECONDAIRE, bouton, menu, rangees};

/// Continuer, rentrer, soigner un monstre ou mettre un monstre en tête :
/// c'est lui qui commencera le prochain combat (Q55).
pub fn entre_deux_etapes(partie: &Partie) -> Question {
    let equipe = partie.collection().equipe();
    let potions = partie.inventaire().potions();
    let lignes: Vec<String> = equipe
        .iter()
        .map(|monstre| ligne_monstre(monstre, Cote::Joueur))
        .collect();
    let mut rangees_de_l_etape = rangees(vec![
        bouton(Cle::Continuer, "Continuer l'expédition", PRINCIPAL),
        bouton(Cle::Rentrer, "Rentrer au camp", SECONDAIRE),
    ]);
    if potions > 0 {
        let aptes = options_d_equipe(equipe)
            .into_iter()
            .zip(equipe)
            .filter(|(_, monstre)| !monstre.est_ko())
            .map(|(option, _)| option)
            .collect();
        rangees_de_l_etape.extend(menu(
            Cle::Potion,
            &format!("Utiliser une potion ({potions}) sur"),
            aptes,
        ));
    }
    let suivants = options_d_equipe(equipe).into_iter().skip(1).collect();
    rangees_de_l_etape.extend(menu(Cle::EnTete, "Mettre en tête de l'équipe", suivants));
    Question {
        texte: format!(
            "Votre équipe :\n{}\nPotions : {potions}\n\nQue faire ?",
            lignes.join("\n")
        ),
        rangees: rangees_de_l_etape,
    }
}

pub fn monstre_rare() -> Question {
    Question {
        texte: "Le prendre avec vous ?".to_string(),
        rangees: rangees(vec![
            bouton(Cle::Prendre, "Le prendre", PRINCIPAL),
            bouton(Cle::Laisser, "Le laisser", SECONDAIRE),
        ]),
    }
}

/// Le joueur relâche un de ses monstres pour garder la capture, ou relâche
/// la capture (Q12).
pub fn place_manquante(capture: &Monstre, collection: &Collection) -> Question {
    let mut lignes = menus_de_monstres(collection, Cle::Liberer, "Relâcher", |_| true);
    lignes.extend(rangees(vec![bouton(
        Cle::RelacherLaCapture,
        &format!("Relâcher {}", capture.nom()),
        SECONDAIRE,
    )]));
    Question {
        texte: format!(
            "{}\n{}",
            texte_de_place_manquante(capture),
            liste_des_monstres(collection)
        ),
        rangees: lignes,
    }
}
