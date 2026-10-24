//! Invitations et expéditions en groupe entre joueurs du bot (section 15.5),
//! gardées en mémoire : un redémarrage du bot les perd, et chacun reprend
//! alors à sa dernière sauvegarde, comme après une expédition seul. Les
//! règles sont dans le noyau (`bersemon_core::groupe`).

use bersemon_core::combat::{Combat, Membre};
use bersemon_core::deroulement::ErreurChoix;
use bersemon_core::groupe::ExpeditionEnGroupe;
use bersemon_core::horodatage::Horodatage;

use crate::joueurs::IdJoueur;

/// Ce que le joueur fait depuis l'écran des autres joueurs ou celui de la
/// descente ; les autres choix passent par les composants habituels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionDeGroupe {
    Inviter(IdJoueur),
    RetirerInvitation,
    AccepterInvitation,
    RefuserInvitation,
    /// Après 5 minutes sans réponse du compagnon.
    RamenerLeGroupe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurDInvitation {
    AvecSoiMeme,
    JoueurSansPartie,
    /// Une invitation à la fois par joueur, envoyée ou reçue.
    InvitationDejaEnCours,
    AutreDejaInvite,
    AucuneInvitation,
    /// L'un des deux est en duel ou en expédition en groupe.
    DejaOccupe,
    /// L'un des deux n'est pas au camp, ou une question l'y attend.
    PasAuCamp,
    PasEnGroupe,
    /// Refus du noyau : Tour fermée, équipe vide, attente trop courte.
    Choix(ErreurChoix),
}

/// Le refus d'une action d'invitation ou de groupe, vu par celui qui l'a
/// tentée.
pub fn erreur_d_invitation(erreur: &ErreurDInvitation) -> String {
    match erreur {
        ErreurDInvitation::AvecSoiMeme => "Vous ne pouvez pas vous inviter vous-même.".to_string(),
        ErreurDInvitation::JoueurSansPartie => {
            "Ce joueur n'a pas de partie sur le bot.".to_string()
        }
        ErreurDInvitation::InvitationDejaEnCours => {
            "Vous avez déjà un défi ou une invitation en attente : un seul à la fois.".to_string()
        }
        ErreurDInvitation::AutreDejaInvite => {
            "Ce joueur a déjà un défi ou une invitation en attente : réessayez plus tard."
                .to_string()
        }
        ErreurDInvitation::AucuneInvitation => {
            "Cette invitation n'est plus en attente.".to_string()
        }
        ErreurDInvitation::DejaOccupe => {
            "Un des deux joueurs est déjà en duel ou en expédition.".to_string()
        }
        ErreurDInvitation::PasAuCamp => {
            "Les deux joueurs doivent être au camp pour descendre ensemble : réessayez plus tard."
                .to_string()
        }
        ErreurDInvitation::PasEnGroupe => "Vous n'êtes pas en expédition en groupe.".to_string(),
        ErreurDInvitation::Choix(ErreurChoix::ZoneFermee) => {
            "Chacun doit avoir ouvert la Tour sans fond, en menant la Lisière au bout.".to_string()
        }
        ErreurDInvitation::Choix(ErreurChoix::EquipeVide) => {
            "Chaque joueur doit avoir au moins un monstre dans son équipe.".to_string()
        }
        ErreurDInvitation::Choix(ErreurChoix::AttenteTropCourte) => {
            "Pas encore : votre compagnon a 5 minutes pour jouer.".to_string()
        }
        ErreurDInvitation::Choix(erreur) => format!("Action impossible : {erreur}."),
    }
}

/// Une invitation en attente de réponse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invitation {
    pub de: IdJoueur,
    pub a: IdJoueur,
}

/// Celui qui invite est l'hôte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupeEnCours {
    pub joueurs: [IdJoueur; 2],
    pub expedition: ExpeditionEnGroupe,
}

impl GroupeEnCours {
    pub fn membre_de(&self, joueur: IdJoueur) -> Option<Membre> {
        if self.joueurs[0] == joueur {
            Some(Membre::Hote)
        } else if self.joueurs[1] == joueur {
            Some(Membre::Invite)
        } else {
            None
        }
    }

    pub fn joueur(&self, membre: Membre) -> IdJoueur {
        match membre {
            Membre::Hote => self.joueurs[0],
            Membre::Invite => self.joueurs[1],
        }
    }
}

/// Ce que montre l'écran de la descente, préparé par `Joueurs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupeVu {
    pub membre: Membre,
    /// Le nom du héros du compagnon.
    pub compagnon: String,
    pub combat: Option<Combat>,
    /// Ceux dont le groupe attend une réponse.
    pub attendus: Vec<Membre>,
    /// D'où court le délai d'absence (section 15.5).
    pub derniere_action: Horodatage,
}
