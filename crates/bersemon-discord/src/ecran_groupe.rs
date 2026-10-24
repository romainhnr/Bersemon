//! Expédition en groupe sur Discord (section 15.5) : le combat commun, vu
//! par celui qui agit avec les composants du combat ordinaire, et l'attente
//! du compagnon. Les autres questions sont celles d'une expédition seul,
//! sous l'en-tête de la descente.

use bersemon_core::combat::Phase;
use bersemon_core::deroulement::Attente;
use bersemon_core::groupe::DELAI_D_ABSENCE;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::partie::Partie;
use bersemon_presentation::combat::etat;
use bersemon_presentation::groupe::{
    RAMENER_LE_GROUPE, descente_avec, en_attente_du_compagnon, veut_continuer, vous_attendez,
};

use crate::composants::Cle;
use crate::ecran_combat;
use crate::groupes::GroupeVu;
use crate::message::{DANGER, PRINCIPAL, Question, SECONDAIRE, bouton, rangees};

/// En tête de chaque écran de la descente ; le joueur sait aussi si son
/// compagnon veut continuer.
pub fn entete(vu: &GroupeVu, attente: Attente<'_>) -> String {
    let mut texte = descente_avec(&vu.compagnon);
    if attente == Attente::EntreDeuxEtapes
        && vu.combat.is_none()
        && !vu.attendus.contains(&vu.membre.autre())
    {
        texte.push('\n');
        texte.push_str(&veut_continuer(&vu.compagnon));
    }
    texte
}

/// Le combat commun et l'attente du compagnon ; `None` pour une question
/// de l'expédition du joueur.
pub fn question(
    vu: &GroupeVu,
    partie: &Partie,
    attente: Attente<'_>,
    maintenant: Horodatage,
) -> Option<Question> {
    let mut paragraphes = vec![descente_avec(&vu.compagnon)];
    let mut boutons = Vec::new();
    if let Some(combat) = &vu.combat {
        if vu.attendus.contains(&vu.membre) {
            let question = match combat.phase() {
                Phase::Remplacement => ecran_combat::remplacement(combat, partie.codex()),
                Phase::Termine(_) => ecran_combat::capture(combat, partie.collection()),
                Phase::ChoixAction => ecran_combat::action(combat, partie),
            };
            paragraphes.push(question.texte);
            return Some(Question {
                texte: paragraphes.join("\n\n"),
                rangees: question.rangees,
            });
        }
        paragraphes.push(etat(combat, partie.codex()));
        paragraphes.push(en_attente_du_compagnon(&vu.compagnon));
        boutons.push(bouton(Cle::Actualiser, "Actualiser", PRINCIPAL));
    } else if attente == Attente::AttenteDuGroupe {
        paragraphes.push(vous_attendez(&vu.compagnon));
        boutons.push(bouton(Cle::Actualiser, "Actualiser", PRINCIPAL));
        boutons.push(bouton(Cle::Rentrer, "Rentrer au camp", SECONDAIRE));
    } else {
        return None;
    }
    if peut_ramener_le_groupe(vu, maintenant) {
        boutons.push(bouton(Cle::RamenerLeGroupe, RAMENER_LE_GROUPE, DANGER));
    }
    Some(Question {
        texte: paragraphes.join("\n\n"),
        rangees: rangees(boutons),
    })
}

/// Celui qui attend son compagnon depuis 5 minutes (section 15.5).
fn peut_ramener_le_groupe(vu: &GroupeVu, maintenant: Horodatage) -> bool {
    !vu.attendus.contains(&vu.membre)
        && vu.attendus.contains(&vu.membre.autre())
        && maintenant.secondes() >= vu.derniere_action.secondes() + DELAI_D_ABSENCE
}
