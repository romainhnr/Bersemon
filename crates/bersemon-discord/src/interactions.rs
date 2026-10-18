//! Clics sur les boutons, choix dans les menus et envois de formulaires :
//! chacun remplace le contenu du message par la suite de la partie. Le verrou
//! des parties n'est jamais tenu pendant un échange avec Discord.

use std::time::{SystemTime, UNIX_EPOCH};

use bersemon_core::horodatage::Horodatage;
use poise::serenity_prelude as serenity;

use crate::composants::{Commande, Composant};
use crate::formulaires::{self, CHAMP_TEXTE};
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
    let texte = valeur_du_champ(&formulaire.data.components, CHAMP_TEXTE).unwrap_or_default();
    let Some(commande) = lu.cle.commande(None, Some(&texte)) else {
        return Ok(());
    };
    let reponse = executer(donnees, formulaire.user.id.get(), lu.version, commande);
    formulaire.create_response(contexte, reponse).await?;
    Ok(())
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

/// L'heure du choix, que le noyau ne lit jamais lui-même (section 12.2).
fn maintenant() -> Horodatage {
    let secondes = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duree| duree.as_secs());
    Horodatage::depuis_secondes(secondes)
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
