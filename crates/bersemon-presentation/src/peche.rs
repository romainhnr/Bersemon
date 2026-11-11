//! Textes de la pêche au camp (section 13.8, docs/contenu-v1.md, section
//! 15).

use std::time::Duration;

use bersemon_core::horodatage::Horodatage;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::peche::{CanneAPeche, ErreurPeche, ParametresPeche, Prise};

use crate::expeditions_automatiques::duree;
use crate::mise_en_valeur::important;
use crate::monstres::nom_sauvage;
use crate::partie::argent;

pub fn lancement(delai: Duration) -> String {
    format!(
        "Vous lancez la ligne. Elle sera prête à relever dans {}.",
        duree(delai)
    )
}

/// Un objet est déjà rangé dans le coffre ; un monstre attend la réponse
/// du joueur.
pub fn prise(prise: &Prise) -> String {
    match prise {
        Prise::Monstre(monstre) => format!(
            "Au bout de la ligne : {}, de niveau {} !",
            important(&nom_sauvage(monstre)),
            monstre.niveau().valeur()
        ),
        Prise::Potion => objet_remonte("une potion"),
        Prise::Argent(montant) => objet_remonte(&format!(
            "une bourse noyée, qui contient {}",
            argent(*montant)
        )),
        Prise::Pierre(pierre) => objet_remonte(&format!("une {}", pierre.nom())),
        Prise::CapsuleDHabilete => objet_remonte("une capsule d'habileté"),
        Prise::Booster(booster) => objet_remonte(&format!("une {}", booster.nom())),
        Prise::PhiltreChromatique => {
            "Vous remontez une fiole scellée : un Philtre chromatique. Vous le rangez dans le coffre."
                .to_string()
        }
    }
}

/// Tous les objets à remonter sont féminins, sauf le Philtre chromatique.
fn objet_remonte(objet: &str) -> String {
    format!("Vous remontez {objet}. Vous la rangez dans le coffre.")
}

/// La question posée tant que le joueur n'a ni gardé ni relâché le monstre
/// pêché.
pub fn question_de_la_prise(monstre: &Monstre) -> String {
    format!(
        "Garder {} ? Sinon, la prise retourne à l'eau.",
        important(&nom_sauvage(monstre))
    )
}

pub fn prise_relachee(monstre: &Monstre) -> String {
    format!("{} retourne à l'eau.", important(&nom_sauvage(monstre)))
}

/// La ligne à l'eau et son temps restant, prête, ou aucune ligne.
pub fn etat_de_la_ligne(partie: &Partie, maintenant: Horodatage) -> String {
    match partie.peche().ligne() {
        None => "Aucune ligne à l'eau.".to_string(),
        Some(_) if partie.peche().est_prete(maintenant) => {
            "La ligne est prête à relever.".to_string()
        }
        Some(fin) => format!(
            "La ligne est à l'eau, avec la {} : prête dans {}.",
            partie
                .peche()
                .canne_de_la_ligne()
                .unwrap_or(CanneAPeche::Argent)
                .nom()
                .to_lowercase(),
            duree(maintenant.duree_jusqu_a(fin))
        ),
    }
}

/// Au camp, quand la ligne attend d'être relevée.
pub fn ligne_prete(partie: &Partie, maintenant: Horodatage) -> Option<String> {
    partie
        .peche()
        .est_prete(maintenant)
        .then(|| "La ligne d'Ansgar est prête à relever.".to_string())
}

pub fn erreur_peche(erreur: &ErreurPeche) -> String {
    match erreur {
        ErreurPeche::AucunPecheur => "Personne ne pêche au camp.".to_string(),
        ErreurPeche::LigneDejaALEau => {
            "La ligne est déjà à l'eau : une seule à la fois.".to_string()
        }
        ErreurPeche::AucuneLigne => "Aucune ligne à l'eau : lancez-en une.".to_string(),
        ErreurPeche::PasEncorePrete(_) => "La ligne n'est pas encore prête.".to_string(),
        ErreurPeche::PlaceManquante => {
            "Votre équipe et votre camp sont pleins : faites de la place avant de relever la ligne."
                .to_string()
        }
        ErreurPeche::CanneManquante(canne) => format!(
            "Il vous faut d'abord une {} : Ansgar vous aide à la fabriquer.",
            canne.nom().to_lowercase()
        ),
        ErreurPeche::CannePrecedenteManquante(canne) => format!(
            "Il faut d'abord fabriquer la {}.",
            canne.nom().to_lowercase()
        ),
        ErreurPeche::CanneDejaFabriquee => "Vous avez déjà cette canne.".to_string(),
        ErreurPeche::PrixDeLaCanne { argent, ressources } => {
            let mut manque = Vec::new();
            if *argent > 0 {
                manque.push(crate::partie::argent(*argent));
            }
            if !ressources.is_empty() {
                manque.push(crate::forge::cout(ressources));
            }
            format!("Il vous manque {}.", manque.join(" et "))
        }
    }
}

/// "Canne en fer : 1 h, 300 pièces d'argent et Fragment d'armure (5)".
pub fn description_de_la_canne(canne: CanneAPeche, parametres: &ParametresPeche) -> String {
    let (prix, ressources) = parametres.prix_de_la_canne(canne);
    let mut cout = vec![argent(prix)];
    if !ressources.is_empty() {
        cout.push(crate::forge::cout(ressources));
    }
    format!(
        "{} : {}, {}",
        canne.nom(),
        duree(canne.duree()),
        cout.join(" et ")
    )
}

pub fn canne_fabriquee(canne: CanneAPeche) -> String {
    format!(
        "Ansgar vous aide à fabriquer une {} : la ligne reste {} à l'eau, et remonte des prises plus rares que celle d'avant.",
        canne.nom().to_lowercase(),
        duree(canne.duree())
    )
}

/// "Cannes : Canne en bois, Canne en fer." ou "Aucune canne : fabriquez-en
/// une avec Ansgar."
pub fn cannes_possedees(partie: &Partie) -> String {
    let cannes = partie.peche().cannes();
    if cannes.is_empty() {
        return "Aucune canne : fabriquez-en une avec Ansgar.".to_string();
    }
    let noms: Vec<&str> = cannes.iter().map(|canne| canne.nom()).collect();
    format!("Cannes : {}.", noms.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::pierre::PierreDEvolution;

    #[test]
    fn les_objets_remontes_vont_au_coffre() {
        assert_eq!(
            prise(&Prise::Potion),
            "Vous remontez une potion. Vous la rangez dans le coffre."
        );
        assert_eq!(
            prise(&Prise::Argent(30)),
            "Vous remontez une bourse noyée, qui contient 30 pièces d'argent. Vous la rangez dans le coffre."
        );
        assert_eq!(
            prise(&Prise::Pierre(PierreDEvolution::Lune)),
            "Vous remontez une Pierre de lune. Vous la rangez dans le coffre."
        );
    }

    #[test]
    fn la_ligne_dit_son_temps_restant() {
        assert_eq!(
            lancement(Duration::from_secs(7_200)),
            "Vous lancez la ligne. Elle sera prête à relever dans 2 h."
        );
    }
}
