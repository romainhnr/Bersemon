//! Expédition en groupe dans la Tour sans fond (section 15.5) : invitations
//! et attente du compagnon. Textes dans docs/contenu-v1.md, section 25.

use crate::echanges::joueur;

pub const INVITER_DANS_LA_TOUR: &str = "Inviter dans la Tour";
pub const ACCEPTER_L_INVITATION: &str = "Accepter l'invitation";
pub const REFUSER_L_INVITATION: &str = "Refuser l'invitation";
pub const RETIRER_L_INVITATION: &str = "Retirer l'invitation";
pub const RAMENER_LE_GROUPE: &str = "Ramener le groupe au camp";

/// Le message privé qui prévient le joueur d'une invitation reçue.
pub fn invitation_recue(heros: &str, pseudo: Option<&str>) -> String {
    format!(
        "{} vous invite à descendre dans la Tour sans fond. Ouvrez /bersemon, au camp, pour répondre.",
        joueur(heros, pseudo)
    )
}

pub fn invitation_acceptee(heros: &str) -> String {
    format!("{heros} a accepté votre invitation : la descente commence. Ouvrez /bersemon.")
}

pub fn invitation_refusee(heros: &str) -> String {
    format!("{heros} a refusé votre invitation dans la Tour sans fond.")
}

/// Sur l'écran des autres joueurs, l'invitation reçue.
pub fn vous_invite(heros: &str) -> String {
    format!("{heros} vous invite à descendre dans la Tour sans fond.")
}

/// Sur l'écran des autres joueurs, l'invitation envoyée.
pub fn invitation_envoyee(heros: &str) -> String {
    format!("Vous avez invité {heros} dans la Tour sans fond. En attente de sa réponse.")
}

/// En tête de chaque écran de l'expédition en groupe.
pub fn descente_avec(heros: &str) -> String {
    format!("Descente avec {heros}.")
}

pub fn en_attente_du_compagnon(heros: &str) -> String {
    format!("En attente de {heros}.")
}

pub fn vous_attendez(heros: &str) -> String {
    format!("Vous voulez continuer : vous attendez {heros}.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textes_de_l_invitation() {
        assert_eq!(
            invitation_recue("Ysolde", Some("ysolde")),
            "Ysolde (ysolde) vous invite à descendre dans la Tour sans fond. Ouvrez /bersemon, au camp, pour répondre."
        );
        assert_eq!(
            invitation_envoyee("Aldric"),
            "Vous avez invité Aldric dans la Tour sans fond. En attente de sa réponse."
        );
    }
}
