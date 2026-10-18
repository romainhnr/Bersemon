//! Commandes slash du bot. Les réponses ne sont visibles que du joueur qui
//! commande, dans un salon comme en message privé (section 2).
//!
//! Le commentaire de documentation d'une commande est la description que
//! Discord affiche au joueur.

use crate::ecrans;
use crate::joueurs::verrouiller;
use crate::{ContexteDeCommande, Erreur};

/// Ouvre votre partie de Bersemon.
#[poise::command(slash_command)]
pub async fn bersemon(contexte: ContexteDeCommande<'_>) -> Result<(), Erreur> {
    let donnees = contexte.data();
    let joueur = contexte.author().id.get();
    // Le verrou est relâché avant d'envoyer la réponse à Discord.
    let message = {
        let mut joueurs = verrouiller(&donnees.joueurs);
        let session = joueurs.session(joueur, &donnees.contenu);
        ecrans::message(session, &[], None, &donnees.contenu)
    };
    let reponse = poise::CreateReply::default()
        .content(message.texte)
        .components(message.composants)
        .ephemeral(true);
    contexte.send(reponse).await?;
    Ok(())
}
