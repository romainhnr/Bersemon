//! Expédition sur Discord (section 9.2) : entre deux étapes, devant un
//! monstre rare pacifique, quand une capture ne trouve pas de place, et pour
//! une annonce.

use bersemon_core::collection::Collection;
use bersemon_core::combat::Cote;
use bersemon_core::deroulement::Annonce;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_presentation::monstres::{ligne_du_heros, ligne_monstre};
use bersemon_presentation::partie::{
    annonce as texte_d_annonce, place_manquante as texte_de_place_manquante, sac_en_expedition,
};

use bersemon_core::inventaire::Objet;
use bersemon_presentation::sac::{question_du_sac_plein, sortes_a_jeter};

use crate::composants::{Cle, objet_en_texte};
use crate::ecran_camp::{liste_des_monstres, menus_de_monstres, options_d_equipe};
use crate::ecran_combat::options_de_soins;
use crate::message::{
    OPTIONS_PAR_MENU, OptionDeMenu, PRINCIPAL, Question, SECONDAIRE, bouton, menu, rangees,
};

/// Continuer, rentrer, soigner un monstre ou mettre un monstre en tête :
/// c'est lui qui commencera le prochain combat (Q55). Le contenu du sac est
/// rappelé : une défaite le ferait perdre (section 9.3).
pub fn entre_deux_etapes(partie: &Partie) -> Question {
    let equipe = partie.collection().equipe();
    let heros = partie.heros().combattant();
    let mut lignes: Vec<String> = equipe
        .iter()
        .map(|monstre| ligne_monstre(monstre, Cote::Joueur))
        .collect();
    // Le héros suit l'équipe : il combat quand elle est KO (section 4.4).
    lignes.push(ligne_du_heros(&heros));
    let mut rangees_de_l_etape = rangees(vec![
        bouton(Cle::Continuer, "Continuer l'expédition", PRINCIPAL),
        bouton(Cle::Rentrer, "Rentrer au camp", SECONDAIRE),
    ]);
    rangees_de_l_etape.extend(menu(
        Cle::Soin,
        "Utiliser un soin",
        options_de_soins(partie.sac().soins(), equipe, Some((equipe.len(), &heros))),
    ));
    let suivants = options_d_equipe(equipe).into_iter().skip(1).collect();
    rangees_de_l_etape.extend(menu(Cle::EnTete, "Mettre en tête de l'équipe", suivants));
    Question {
        texte: format!(
            "Votre équipe :\n{}\n{}\n\nQue faire ?",
            lignes.join("\n"),
            sac_en_expedition(partie)
        ),
        rangees: rangees_de_l_etape,
    }
}

/// À la fin d'une étape où le sac déborde : un menu de ce qu'il contient,
/// un autre pour les ressources ; chaque choix jette un exemplaire, jusqu'à
/// revenir à la capacité du sac (section 13.3). Au-delà de 25 sortes, les
/// dernières attendent qu'un exemplaire des premières soit jeté.
pub fn sac_plein(partie: &Partie) -> Question {
    let (ressources, autres): (Vec<_>, Vec<_>) = sortes_a_jeter(partie.sac())
        .into_iter()
        .partition(|sorte| matches!(sorte.unite, Objet::Ressources(..)));
    let options = |sortes: Vec<bersemon_presentation::sac::SorteAJeter>| -> Vec<OptionDeMenu> {
        sortes
            .into_iter()
            .take(OPTIONS_PAR_MENU)
            .map(|sorte| OptionDeMenu {
                libelle: sorte.libelle,
                valeur: objet_en_texte(&sorte.unite),
                description: Some("En jeter un".to_string()),
            })
            .collect()
    };
    let mut lignes = Vec::new();
    lignes.extend(menu(Cle::Jeter, "Jeter un objet", options(autres)));
    lignes.extend(menu(
        Cle::JeterUneRessource,
        "Jeter une ressource",
        options(ressources),
    ));
    Question {
        texte: format!(
            "{}
{}",
            question_du_sac_plein(partie),
            sac_en_expedition(partie)
        ),
        rangees: lignes,
    }
}

/// Un seul bouton, pour que le joueur ne rate pas l'événement en enchaînant
/// les actions (section 9.2).
pub fn annonce(annonce: Annonce) -> Question {
    let (texte, libelle) = texte_d_annonce(annonce);
    Question {
        texte: texte.to_string(),
        rangees: rangees(vec![bouton(Cle::Poursuivre, libelle, PRINCIPAL)]),
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
