//! Commandes slash du bot. Les réponses ne sont visibles que du joueur qui
//! commande, dans un salon comme en message privé (section 2), sauf avec
//! l'option de test des messages publics.
//!
//! Le commentaire de documentation d'une commande est la description que
//! Discord affiche au joueur.

use crate::horloge::maintenant;
use crate::joueurs::verrouiller;
use crate::message;
use crate::{ContexteDeCommande, Erreur};

/// Ouvre votre partie de Bersemon.
#[poise::command(slash_command)]
pub async fn bersemon(contexte: ContexteDeCommande<'_>) -> Result<(), Erreur> {
    let donnees = contexte.data();
    let joueur = contexte.author().id.get();
    crate::interactions::noter_pseudo(donnees, contexte.author());
    // Le verrou est relâché avant d'envoyer la réponse à Discord.
    let affiche = {
        let mut joueurs = verrouiller(&donnees.joueurs);
        let session = joueurs.session(joueur, &donnees.contenu);
        message::message(session, &[], None, &donnees.contenu, maintenant())
    };
    let mut reponse = poise::CreateReply::default()
        .components(affiche.composants())
        .ephemeral(!donnees.messages_publics);
    if let Some(sprite) = affiche.piece_jointe() {
        reponse = reponse.attachment(sprite);
    }
    let reponse = reponse.content(affiche.texte);
    contexte.send(reponse).await?;
    Ok(())
}
