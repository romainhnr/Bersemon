//! Défis et duels en cours entre joueurs du bot (section 15.3), gardés en
//! mémoire : un redémarrage du bot les perd, sans conséquence puisque le
//! duel se joue sur des copies. Les règles du duel sont dans le noyau
//! (`bersemon_core::duel`).

use bersemon_core::combat::Cote;
use bersemon_core::duel::{Duel, ErreurDuel};
use bersemon_presentation::erreurs::erreur_de_combat;

use crate::joueurs::IdJoueur;

/// Ce que le joueur fait depuis l'écran des autres joueurs ou celui du
/// duel ; les actions du combat passent par les composants du combat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionDeDuel {
    Defier(IdJoueur),
    RetirerDefi,
    AccepterDefi,
    RefuserDefi,
    Abandonner,
    ReclamerLaVictoire,
    /// Après la fin du duel.
    RevenirAuCamp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurDeDefi {
    AvecSoiMeme,
    JoueurSansPartie,
    /// Un défi à la fois par joueur, envoyé ou reçu.
    DefiDejaEnCours,
    AutreDejaDefie,
    AucunDefi,
    /// Le joueur ou l'autre est déjà en duel.
    DejaEnDuel,
    /// L'un des deux n'est pas au camp, ou une question l'y attend.
    PasAuCamp,
    /// L'un des deux n'a aucun monstre dans son équipe.
    EquipeVide,
    PasEnDuel,
    /// On ne revient au camp qu'après la fin du duel, ou en abandonnant.
    DuelPasFini,
    Duel(ErreurDuel),
}

/// Le refus d'une action de défi ou de duel, vu par celui qui l'a tentée.
pub fn erreur_de_defi(erreur: &ErreurDeDefi) -> String {
    match erreur {
        ErreurDeDefi::AvecSoiMeme => "Vous ne pouvez pas vous défier vous-même.".to_string(),
        ErreurDeDefi::JoueurSansPartie => "Ce joueur n'a pas de partie sur le bot.".to_string(),
        ErreurDeDefi::DefiDejaEnCours => {
            "Vous avez déjà un défi en attente : un seul à la fois.".to_string()
        }
        ErreurDeDefi::AutreDejaDefie => {
            "Ce joueur a déjà un défi en attente : réessayez plus tard.".to_string()
        }
        ErreurDeDefi::AucunDefi => "Ce défi n'est plus en attente.".to_string(),
        ErreurDeDefi::DejaEnDuel => "Un des deux joueurs est déjà en duel.".to_string(),
        ErreurDeDefi::PasAuCamp => {
            "Les deux joueurs doivent être au camp pour commencer le duel : réessayez plus tard."
                .to_string()
        }
        ErreurDeDefi::EquipeVide => {
            "Chaque joueur doit avoir au moins un monstre dans son équipe.".to_string()
        }
        ErreurDeDefi::PasEnDuel => "Vous n'êtes pas en duel.".to_string(),
        ErreurDeDefi::DuelPasFini => {
            "Le duel n'est pas fini : abandonnez pour revenir au camp.".to_string()
        }
        ErreurDeDefi::Duel(ErreurDuel::Combat(erreur)) => erreur_de_combat(erreur),
        ErreurDeDefi::Duel(ErreurDuel::ActionDejaChoisie) => {
            "Votre action de ce tour est déjà choisie.".to_string()
        }
        ErreurDeDefi::Duel(ErreurDuel::ForfaitImpossible) => {
            "Pas encore : votre adversaire a 5 minutes pour jouer.".to_string()
        }
    }
}

/// Un défi en attente de réponse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Defi {
    pub de: IdJoueur,
    pub a: IdJoueur,
}

/// Le joueur qui a lancé le défi est le premier côté du combat (Q90).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuelEnCours {
    pub joueurs: [IdJoueur; 2],
    pub duel: Duel,
    /// Après la fin, chacun revient au camp quand il veut ; le duel est
    /// oublié quand les deux y sont revenus.
    pub revenus: [bool; 2],
    /// L'issue est notée au palmarès des deux joueurs dès la fin du duel,
    /// une seule fois (section 15.4).
    pub palmares_note: bool,
}

impl DuelEnCours {
    /// Le côté de ce joueur, s'il est de ce duel et ne l'a pas quitté.
    pub fn cote_de(&self, joueur: IdJoueur) -> Option<Cote> {
        if self.joueurs[0] == joueur && !self.revenus[0] {
            Some(Cote::Joueur)
        } else if self.joueurs[1] == joueur && !self.revenus[1] {
            Some(Cote::Adversaire)
        } else {
            None
        }
    }

    pub fn autre(&self, cote: Cote) -> IdJoueur {
        match cote {
            Cote::Joueur => self.joueurs[1],
            Cote::Adversaire => self.joueurs[0],
        }
    }
}

/// Ce que montre l'écran du duel, préparé par `Joueurs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuelVu {
    pub duel: Duel,
    pub cote: Cote,
    /// Le nom du héros du joueur, puis celui de l'autre.
    pub heros: String,
    pub adversaire: String,
}
