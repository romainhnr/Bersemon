//! Don et échange de monstres entre joueurs (section 15.1) : les offres,
//! leurs messages privés et leurs refus. Textes dans docs/contenu-v1.md,
//! section 13.

use bersemon_core::combat::Cote;
use bersemon_core::echange::{ErreurEchange, Offre};
use bersemon_core::monstre::Monstre;

use crate::monstres::{nom, nom_en_valeur};

/// "Ysolde (ysolde_42)", ou le nom du héros seul si le bot ne connaît pas
/// encore le pseudo du joueur.
pub fn joueur(heros: &str, pseudo: Option<&str>) -> String {
    match pseudo {
        Some(pseudo) => format!("{heros} ({pseudo})"),
        None => heros.to_string(),
    }
}

/// Le message privé qui prévient le joueur d'une offre reçue.
pub fn offre_recue(heros: &str, pseudo: Option<&str>, offre: &Offre) -> String {
    let qui = joueur(heros, pseudo);
    let cede = nom_en_valeur(&offre.cede.monstre, Cote::Joueur);
    let proposition = match &offre.demande {
        Some(demande) => format!(
            "{qui} vous propose {cede} contre votre {}.",
            nom_en_valeur(&demande.monstre, Cote::Joueur)
        ),
        None => format!("{qui} vous offre {cede}."),
    };
    format!("{proposition} Ouvrez /bersemon, au camp, pour répondre.")
}

pub fn offre_acceptee(heros: &str) -> String {
    format!("{heros} a accepté votre offre.")
}

pub fn offre_refusee(heros: &str) -> String {
    format!("{heros} a refusé votre offre.")
}

/// "Croc (Loup, niv. 7)" : dans une offre, les PV ne comptent pas, le
/// camp les soigne.
pub fn nom_et_niveau(monstre: &Monstre) -> String {
    let espece = if monstre.surnom().is_some() {
        format!("{}, ", monstre.espece().nom())
    } else {
        String::new()
    };
    format!(
        "{} ({espece}niv. {})",
        nom(monstre, Cote::Joueur),
        monstre.niveau().valeur()
    )
}

/// L'offre vue par celui qui la reçoit.
pub fn decrire_offre_recue(qui: &str, offre: &Offre) -> String {
    let cede = nom_et_niveau(&offre.cede.monstre);
    match &offre.demande {
        Some(demande) => format!(
            "{qui} vous propose {cede}, contre votre {}.",
            nom_et_niveau(&demande.monstre)
        ),
        None => format!("{qui} vous offre {cede}."),
    }
}

/// L'offre vue par celui qui l'a faite.
pub fn decrire_offre_envoyee(qui: &str, offre: &Offre) -> String {
    let cede = nom_et_niveau(&offre.cede.monstre);
    match &offre.demande {
        Some(demande) => format!(
            "Vous proposez à {qui} {cede} contre {}. En attente de sa réponse.",
            nom_et_niveau(&demande.monstre)
        ),
        None => format!("Vous offrez {cede} à {qui}. En attente de sa réponse."),
    }
}

pub fn erreur_echange(erreur: &ErreurEchange) -> String {
    match erreur {
        ErreurEchange::MonstreInexistant(_) => "Ce monstre n'est plus là.".to_string(),
        ErreurEchange::Apotre => {
            "Un Apôtre ne se donne ni ne s'échange : il reste à celui qui l'a vaincu.".to_string()
        }
        ErreurEchange::MonstreLoue => "Un monstre loué ne peut pas faire cela.".to_string(),
        ErreurEchange::DernierMonstre => "Vous ne pouvez pas donner votre dernier monstre.".to_string(),
        ErreurEchange::PlusDePlace => {
            "L'équipe et le camp de celui qui reçoit le don sont pleins.".to_string()
        }
        ErreurEchange::OffrePerimee => {
            "Cette offre n'est plus valable : un des monstres a changé ou changé de place. Elle est retirée.".to_string()
        }
        ErreurEchange::DonneurPasAuCamp => {
            "Le joueur qui vous a fait cette offre n'est pas au camp : réessayez plus tard.".to_string()
        }
        ErreurEchange::ReceveurPasAuCamp => {
            "Vous devez être au camp pour conclure cette offre.".to_string()
        }
    }
}
