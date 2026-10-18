//! Commandes slash du bot. Les réponses ne sont visibles que du joueur qui
//! commande, dans un salon comme en message privé (section 2).
//!
//! Le commentaire de documentation d'une commande est la description que
//! Discord affiche au joueur.

use crate::{ContexteDeCommande, Erreur};

/// Vérifie que Bersemon est en ligne.
#[poise::command(slash_command)]
pub async fn bersemon(contexte: ContexteDeCommande<'_>) -> Result<(), Erreur> {
    let reponse = poise::CreateReply::default()
        .content("Bersemon est en ligne. Le jeu arrivera bientôt sur Discord.")
        .ephemeral(true);
    contexte.send(reponse).await?;
    Ok(())
}
