//! Les textes des succès, des titres, des trophées et des couleurs du héros
//! (section 4.5, docs/contenu-v1.md, section 51).

use bersemon_core::heros::Heros;
use bersemon_core::partie::Partie;
use bersemon_core::succes::{CouleurDuHeros, ErreurSucces, Succes};

/// Au-delà, l'annonce compte les succès pour tenir dans un message Discord.
const SUCCES_DETAILLES_AU_PLUS: usize = 3;

/// Sous la fiche du héros : "Titre porté : Fléau du Baron."
pub fn titre_du_heros(heros: &Heros, succes: &[Succes]) -> String {
    match heros
        .titre()
        .and_then(|titre| succes.iter().find(|un| un.id() == titre))
    {
        Some(un) => format!("Titre porté : {}.", un.titre()),
        None => "Aucun titre porté.".to_string(),
    }
}

/// Sur Discord, où se voit le sprite : "Couleur : Cendre."
pub fn couleur_du_heros(heros: &Heros) -> String {
    match heros.couleur() {
        Some(couleur) => format!("Couleur : {}.", couleur.nom()),
        None => "Couleur d'origine.".to_string(),
    }
}

/// "Aldric, Fléau du Baron", ou "Aldric" sans titre porté.
pub fn nom_et_titre(heros: &Heros, succes: &[Succes]) -> String {
    let titre = heros
        .titre()
        .and_then(|titre| succes.iter().find(|un| un.id() == titre));
    match titre {
        Some(un) => format!("{}, {}", heros.nom().as_str(), un.titre()),
        None => heros.nom().as_str().to_string(),
    }
}

pub fn succes_obtenus(succes: &[Succes], tous_les_trophees: bool) -> String {
    let mut lignes: Vec<String> = if succes.len() > SUCCES_DETAILLES_AU_PLUS {
        vec![format!(
            "{} succès obtenus : voyez les succès, sur l'écran du héros.",
            succes.len()
        )]
    } else {
        succes.iter().map(succes_obtenu).collect()
    };
    if tous_les_trophees {
        lignes.push(
            "Tous les trophées du camp sont réunis : la couleur royale vous revient.".to_string(),
        );
    }
    lignes.join("\n")
}

fn succes_obtenu(un: &Succes) -> String {
    let mut texte = format!(
        "Succès obtenu : {} ({}). Vous pouvez porter ce titre.",
        un.titre(),
        un.description()
    );
    if let Some(trophee) = un.trophee() {
        texte.push_str(&format!(" Un nouveau trophée orne le camp : {trophee}"));
    }
    if let Some(couleur) = un.couleur() {
        texte.push_str(&format!(" Nouvelle couleur du héros : {}.", couleur.nom()));
    }
    texte
}

pub fn titre_porte(succes: Option<&Succes>) -> String {
    match succes {
        Some(un) => format!("Vous portez désormais le titre {}.", un.titre()),
        None => "Vous ne portez plus de titre.".to_string(),
    }
}

pub fn couleur_portee(couleur: Option<CouleurDuHeros>) -> String {
    match couleur {
        Some(couleur) => format!("Votre héros prend la couleur {}.", couleur.nom()),
        None => "Votre héros reprend sa couleur d'origine.".to_string(),
    }
}

pub fn erreur_succes(erreur: &ErreurSucces) -> String {
    match erreur {
        ErreurSucces::NonObtenu => "Ce succès n'est pas encore obtenu.".to_string(),
        ErreurSucces::CouleurBloquee => "Cette couleur n'est pas encore débloquée.".to_string(),
    }
}

/// "Succès : 12/36. Trophées du camp : 5/22."
pub fn bilan_des_succes(partie: &Partie, succes: &[Succes]) -> String {
    let obtenu = |un: &Succes| partie.succes_obtenus().contains(un.id());
    let trophees: Vec<&Succes> = succes.iter().filter(|un| un.trophee().is_some()).collect();
    format!(
        "Succès : {}/{}. Trophées du camp : {}/{}.",
        succes.iter().filter(|un| obtenu(un)).count(),
        succes.len(),
        trophees.iter().filter(|un| obtenu(un)).count(),
        trophees.len()
    )
}

/// Une ligne par succès, obtenu ou non : "Fléau du Baron : abattre le
/// Baron pendu (obtenu)".
pub fn lignes_des_succes(partie: &Partie, succes: &[Succes]) -> Vec<String> {
    succes
        .iter()
        .map(|un| {
            let obtenu = if partie.succes_obtenus().contains(un.id()) {
                " (obtenu)"
            } else {
                ""
            };
            format!("{} : {}{obtenu}", un.titre(), un.description())
        })
        .collect()
}

/// L'écran des succès de la console : le bilan, puis chaque succès.
pub fn ecran_des_succes(partie: &Partie, succes: &[Succes]) -> String {
    let mut lignes = vec![bilan_des_succes(partie, succes)];
    lignes.extend(lignes_des_succes(partie, succes));
    lignes.join("\n")
}

/// Les trophées réunis, dans l'ordre des succès.
pub fn trophees_du_camp(partie: &Partie, succes: &[Succes]) -> String {
    let reunis: Vec<&str> = succes
        .iter()
        .filter(|un| partie.succes_obtenus().contains(un.id()))
        .filter_map(Succes::trophee)
        .collect();
    if reunis.is_empty() {
        return "Aucun trophée n'orne encore le camp.".to_string();
    }
    let total = succes.iter().filter(|un| un.trophee().is_some()).count();
    let mut lignes = vec![format!("Trophées du camp ({}/{total}) :", reunis.len())];
    lignes.extend(reunis.into_iter().map(str::to_string));
    lignes.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::contenu::ContenuV1;
    use bersemon_core::heros::Classe;
    use bersemon_core::nom::Nom;
    use bersemon_core::succes::SuccesId;

    #[test]
    fn le_titre_suit_le_nom_du_heros() {
        let contenu = ContenuV1::new();
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier);
        assert_eq!(nom_et_titre(&heros, contenu.succes()), "Aldric");
        let heros = heros.avec_apparence(Some(SuccesId::new("fleau-du-baron")), None);
        assert_eq!(
            nom_et_titre(&heros, contenu.succes()),
            "Aldric, Fléau du Baron"
        );
    }

    /// Au-delà de 3 succès d'un coup, l'annonce les compte.
    #[test]
    fn l_annonce_detaille_trois_succes_au_plus() {
        let contenu = ContenuV1::new();
        let succes = contenu.succes();
        let deicide = succes
            .iter()
            .find(|un| un.id().as_str() == "deicide")
            .unwrap()
            .clone();
        let texte = succes_obtenus(std::slice::from_ref(&deicide), false);
        assert!(
            texte.starts_with("Succès obtenu : Déicide (abattre le Démiurge)."),
            "{texte}"
        );
        assert!(texte.contains("Un nouveau trophée orne le camp"), "{texte}");
        assert!(
            texte.ends_with("Nouvelle couleur du héros : Cendre."),
            "{texte}"
        );
        let texte = succes_obtenus(&succes[..5], true);
        assert!(texte.starts_with("5 succès obtenus"), "{texte}");
        assert!(
            texte.ends_with("la couleur royale vous revient."),
            "{texte}"
        );
    }
}
