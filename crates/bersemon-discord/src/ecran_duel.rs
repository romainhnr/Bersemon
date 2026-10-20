//! Duel en temps réel sur Discord (section 15.3) : le combat vu par le
//! joueur, avec les composants du combat ordinaire, puis l'attente de
//! l'autre joueur et la fin du duel. Le duel remplace le camp à l'écran.

use bersemon_core::combat::Phase;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::partie::Partie;
use bersemon_presentation::combat::etat;
use bersemon_presentation::duels::{en_attente, issue_du_duel, raconter};

use crate::composants::Cle;
use crate::duels::DuelVu;
use crate::ecran_combat;
use crate::message::{DANGER, PRINCIPAL, Question, SECONDAIRE, SUCCES, bouton, rangees};

pub fn question(vu: &DuelVu, partie: &Partie, maintenant: Horodatage) -> Question {
    let combat = vu.duel.vu_par(vu.cote);
    let mut paragraphes = vec![format!("Duel contre {}.", vu.adversaire)];
    let recit = raconter(vu.duel.recit(vu.cote), &combat, &vu.heros, &vu.adversaire);
    if !recit.is_empty() {
        paragraphes.push(recit);
    }
    let abandonner = || bouton(Cle::Abandonner, "Abandonner", DANGER);
    if let Some(issue) = issue_du_duel(&combat, &vu.adversaire) {
        paragraphes.push(issue);
        return Question {
            texte: paragraphes.join("\n\n"),
            rangees: rangees(vec![bouton(
                Cle::RevenirDuDuel,
                "Retour au camp",
                SECONDAIRE,
            )]),
        };
    }
    if vu.duel.attend_l_autre(vu.cote) {
        paragraphes.push(etat(&combat, partie.codex()));
        paragraphes.push(en_attente(&vu.adversaire));
        let mut boutons = vec![bouton(Cle::Actualiser, "Actualiser", PRINCIPAL)];
        if vu.duel.forfait_possible(vu.cote, maintenant) {
            boutons.push(bouton(
                Cle::ReclamerLaVictoire,
                "Réclamer la victoire",
                SUCCES,
            ));
        }
        boutons.push(abandonner());
        return Question {
            texte: paragraphes.join("\n\n"),
            rangees: rangees(boutons),
        };
    }
    let mut question = match combat.phase() {
        Phase::Remplacement => ecran_combat::remplacement(&combat, partie.codex()),
        _ => ecran_combat::action(&combat, partie),
    };
    paragraphes.push(question.texte);
    question.rangees.extend(rangees(vec![abandonner()]));
    Question {
        texte: paragraphes.join("\n\n"),
        rangees: question.rangees,
    }
}
