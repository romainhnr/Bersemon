//! Clics sur les boutons et envois de formulaires : chacun remplace le
//! contenu du message par la suite de la partie. Le verrou des parties n'est
//! jamais tenu pendant un échange avec Discord.

use poise::serenity_prelude as serenity;

use bersemon_core::deroulement::Choix;

use crate::boutons::{Action, Bouton};
use crate::ecrans::{self, CHAMP_NOM, Message};
use crate::joueurs::{IdJoueur, Refus, verrouiller};
use crate::{DonneesDuBot, Erreur};

pub async fn bouton(
    contexte: &serenity::Context,
    donnees: &DonneesDuBot,
    composant: &serenity::ComponentInteraction,
) -> Result<(), Erreur> {
    let Some(bouton) = Bouton::lire(&composant.data.custom_id) else {
        return Ok(());
    };
    let joueur = composant.user.id.get();
    let reponse = match (&bouton.action, bouton.action.choix()) {
        (Action::OuvrirLeFormulaireDuNom, _) => {
            let mut joueurs = verrouiller(&donnees.joueurs);
            if joueurs.est_a_jour(joueur, bouton.version, &donnees.contenu) {
                serenity::CreateInteractionResponse::Modal(ecrans::formulaire_du_nom(
                    bouton.version,
                ))
            } else {
                let session = joueurs.session(joueur, &donnees.contenu);
                let message =
                    ecrans::message(session, &[], Some(&Refus::MessagePerime), &donnees.contenu);
                mise_a_jour(message)
            }
        }
        (_, Some(choix)) => repondre(donnees, joueur, bouton.version, choix),
        (_, None) => return Ok(()),
    };
    composant.create_response(contexte, reponse).await?;
    Ok(())
}

pub async fn formulaire(
    contexte: &serenity::Context,
    donnees: &DonneesDuBot,
    formulaire: &serenity::ModalInteraction,
) -> Result<(), Erreur> {
    let Some(bouton) = Bouton::lire(&formulaire.data.custom_id) else {
        return Ok(());
    };
    if bouton.action != Action::NomSaisi {
        return Ok(());
    }
    let nom = valeur_du_champ(&formulaire.data.components, CHAMP_NOM).unwrap_or_default();
    let joueur = formulaire.user.id.get();
    let reponse = repondre(donnees, joueur, bouton.version, Choix::NomDuHeros(nom));
    formulaire.create_response(contexte, reponse).await?;
    Ok(())
}

fn repondre(
    donnees: &DonneesDuBot,
    joueur: IdJoueur,
    version: u32,
    choix: Choix,
) -> serenity::CreateInteractionResponse {
    let mut joueurs = verrouiller(&donnees.joueurs);
    let (evenements, refus) = match joueurs.repondre(joueur, version, choix, &donnees.contenu) {
        Ok(evenements) => (evenements, None),
        Err(refus) => (Vec::new(), Some(refus)),
    };
    let session = joueurs.session(joueur, &donnees.contenu);
    mise_a_jour(ecrans::message(
        session,
        &evenements,
        refus.as_ref(),
        &donnees.contenu,
    ))
}

/// Remplace le texte et les boutons du message d'où vient l'interaction.
fn mise_a_jour(message: Message) -> serenity::CreateInteractionResponse {
    serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::new()
            .content(message.texte)
            .components(message.composants),
    )
}

fn valeur_du_champ(lignes: &[serenity::ActionRow], champ: &str) -> Option<String> {
    lignes
        .iter()
        .flat_map(|ligne| &ligne.components)
        .find_map(|composant| match composant {
            serenity::ActionRowComponent::InputText(texte) if texte.custom_id == champ => {
                texte.value.clone()
            }
            _ => None,
        })
}
