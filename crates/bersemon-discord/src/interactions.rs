//! Clics sur les boutons, choix dans les menus et envois de formulaires :
//! chacun remplace le contenu du message par la suite de la partie. Le verrou
//! des parties n'est jamais tenu pendant un échange avec Discord.

use poise::serenity_prelude as serenity;

use crate::composants::{Commande, Composant};
use crate::formulaires::{self, CHAMP_TEXTE};
use crate::horloge::maintenant;
use crate::joueurs::{IdJoueur, Refus, verrouiller};
use crate::message::{self, Message};
use crate::{DonneesDuBot, Erreur};

pub async fn composant(
    contexte: &serenity::Context,
    donnees: &DonneesDuBot,
    composant: &serenity::ComponentInteraction,
) -> Result<(), Erreur> {
    let Some(lu) = Composant::lire(&composant.data.custom_id) else {
        return Ok(());
    };
    if est_a_un_autre_joueur(&composant.message, composant.user.id) {
        composant
            .create_response(contexte, reponse_a_un_autre_joueur())
            .await?;
        return Ok(());
    }
    let valeur = match &composant.data.kind {
        serenity::ComponentInteractionDataKind::StringSelect { values } => values.first(),
        _ => None,
    };
    let Some(commande) = lu.cle.commande(valeur.map(String::as_str), None) else {
        return Ok(());
    };
    let reponse = executer(donnees, composant.user.id.get(), lu.version, commande);
    composant.create_response(contexte, reponse).await?;
    Ok(())
}

pub async fn formulaire(
    contexte: &serenity::Context,
    donnees: &DonneesDuBot,
    formulaire: &serenity::ModalInteraction,
) -> Result<(), Erreur> {
    let Some(lu) = Composant::lire(&formulaire.data.custom_id) else {
        return Ok(());
    };
    let a_un_autre_joueur = formulaire
        .message
        .as_deref()
        .is_some_and(|message| est_a_un_autre_joueur(message, formulaire.user.id));
    if a_un_autre_joueur {
        formulaire
            .create_response(contexte, reponse_a_un_autre_joueur())
            .await?;
        return Ok(());
    }
    let texte = valeur_du_champ(&formulaire.data.components, CHAMP_TEXTE).unwrap_or_default();
    let Some(commande) = lu.cle.commande(None, Some(&texte)) else {
        return Ok(());
    };
    let reponse = executer(donnees, formulaire.user.id.get(), lu.version, commande);
    formulaire.create_response(contexte, reponse).await?;
    Ok(())
}

/// Avec l'option des messages publics, d'autres joueurs voient le message et
/// ses boutons : seul le joueur qui l'a ouvert, ou mis à jour, s'en sert.
/// Sans information sur son auteur, le message est laissé à qui clique.
fn est_a_un_autre_joueur(message: &serenity::Message, joueur: serenity::UserId) -> bool {
    let auteur = match message.interaction_metadata.as_deref() {
        Some(serenity::MessageInteractionMetadata::Command(commande)) => commande.user.id,
        Some(serenity::MessageInteractionMetadata::Component(composant)) => composant.user.id,
        Some(serenity::MessageInteractionMetadata::ModalSubmit(formulaire)) => formulaire.user.id,
        _ => return false,
    };
    auteur != joueur
}

/// Visible du seul joueur qui a cliqué ; le message n'est pas touché.
fn reponse_a_un_autre_joueur() -> serenity::CreateInteractionResponse {
    serenity::CreateInteractionResponse::Message(
        serenity::CreateInteractionResponseMessage::new()
            .content("Ce message est la partie d'un autre joueur : ouvrez la vôtre avec /bersemon.")
            .ephemeral(true),
    )
}

fn executer(
    donnees: &DonneesDuBot,
    joueur: IdJoueur,
    version: u32,
    commande: Commande,
) -> serenity::CreateInteractionResponse {
    let contenu = &donnees.contenu;
    let mut joueurs = verrouiller(&donnees.joueurs);
    let (evenements, refus) = match commande {
        Commande::Jouer(choix) => {
            match joueurs.jouer(joueur, version, choix, contenu, maintenant()) {
                Ok(evenements) => (evenements, None),
                Err(refus) => (Vec::new(), Some(refus)),
            }
        }
        Commande::Afficher(vue) => (
            Vec::new(),
            joueurs.afficher(joueur, version, vue, contenu).err(),
        ),
        Commande::Ouvrir(formulaire) => {
            if joueurs.est_a_jour(joueur, version, contenu) {
                let partie = joueurs.session(joueur, contenu).deroulement.partie();
                return serenity::CreateInteractionResponse::Modal(formulaires::formulaire(
                    version, formulaire, partie,
                ));
            }
            (Vec::new(), Some(Refus::MessagePerime))
        }
    };
    let session = joueurs.session(joueur, contenu);
    mise_a_jour(message::message(
        session,
        &evenements,
        refus.as_ref(),
        contenu,
        maintenant(),
    ))
}

/// Remplace le texte et les composants du message d'où vient l'interaction.
fn mise_a_jour(message: Message) -> serenity::CreateInteractionResponse {
    serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::new()
            .components(message.composants())
            .content(message.texte),
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
