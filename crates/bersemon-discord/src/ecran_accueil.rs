//! Début de partie sur Discord : reprise de la partie du compte ou nouvelle
//! partie, création du héros, dont la classe donne le monstre de départ.

use bersemon_core::contenu::ContenuV1;
use bersemon_core::heros::Classe;
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::ErreurSauvegarde;
use bersemon_presentation::erreurs::erreur_sauvegarde;
use bersemon_presentation::partie::{decrire_classe, partie_en_cours};

use crate::composants::Cle;
use crate::message::{PRINCIPAL, Question, SECONDAIRE, bouton, rangees};

pub fn reprise(partie: &Partie) -> Question {
    Question {
        texte: partie_en_cours(partie),
        rangees: rangees(vec![
            bouton(Cle::ReprendreLaPartie, "Continuer cette partie", PRINCIPAL),
            bouton(
                Cle::NouvellePartie,
                "Commencer une nouvelle partie",
                SECONDAIRE,
            ),
        ]),
    }
}

/// Sans clic du joueur, la sauvegarde reste telle quelle (Q56).
pub fn sauvegarde_illisible(erreur: &ErreurSauvegarde) -> Question {
    Question {
        texte: format!(
            "Votre sauvegarde ne peut pas être chargée : {}.",
            erreur_sauvegarde(erreur)
        ),
        rangees: rangees(vec![bouton(
            Cle::NouvellePartie,
            "Commencer une nouvelle partie, qui la remplacera",
            SECONDAIRE,
        )]),
    }
}

pub fn nom_du_heros() -> Question {
    Question {
        texte: "Bienvenue dans Bersemon. Pour commencer, nommez votre héros.".to_string(),
        rangees: rangees(vec![bouton(Cle::Nommer, "Nommer votre héros", PRINCIPAL)]),
    }
}

pub fn classe(contenu: &ContenuV1) -> Question {
    let lignes: Vec<String> = Classe::TOUTES
        .iter()
        .map(|classe| decrire_classe(*classe, contenu))
        .collect();
    let boutons = Classe::TOUTES
        .iter()
        .map(|classe| bouton(Cle::Classe(*classe), classe.nom(), PRINCIPAL))
        .collect();
    Question {
        texte: format!("Classe de votre héros :\n{}", lignes.join("\n")),
        rangees: rangees(boutons),
    }
}
