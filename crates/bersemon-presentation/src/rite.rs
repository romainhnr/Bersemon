//! Le Rite du sang mêlé de Malvoise (section 13.11) : son service à l'étal,
//! la confirmation, son état et la naissance du petit. Textes dans
//! docs/contenu-v1.md, section 28.

use bersemon_core::combat::Cote;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::partie::Partie;
use bersemon_core::rite::{
    ApercuDuRite, DUREE_DU_RITE, ErreurRite, NIVEAU_DU_PETIT, NaissanceDuRite, PRIX_DU_RITE,
    RESSOURCES_DU_RITE,
};

use crate::erreurs::{erreur_collection, erreur_marchand};
use crate::expeditions_automatiques::duree;
use crate::forge::cout;
use crate::monstres::{nom, nom_en_valeur};
use crate::partie::argent;

pub const RITE_DU_SANG_MELE: &str = "Rite du sang mêlé";

/// Le service, à l'étal, avec ce qu'il coûte et dure.
pub fn service_du_rite() -> String {
    format!(
        "{RITE_DU_SANG_MELE} : {RESSOURCES_DU_RITE} ressources de l'espèce du petit et {}, {}",
        argent(PRIX_DU_RITE),
        duree(DUREE_DU_RITE)
    )
}

/// "Croc de loup (3) et 50 pièces d'argent", ou l'argent seul pour une
/// espèce qui ne lâche pas de ressource.
fn prix(apercu: &ApercuDuRite) -> String {
    if apercu.ressources.is_empty() {
        argent(apercu.prix)
    } else {
        format!("{} et {}", cout(&apercu.ressources), argent(apercu.prix))
    }
}

pub fn question_du_rite(apercu: &ApercuDuRite) -> String {
    let [premier, second] = &apercu.parents;
    format!(
        "Confier {} et {} à Malvoise pour le {RITE_DU_SANG_MELE} ? Il prendra {}. Dans {}, le petit naîtra, de l'espèce {}, au niveau {NIVEAU_DU_PETIT}, avec la meilleure valeur interne de ses parents pour chaque stat. D'ici là, les deux restent auprès de Malvoise.",
        nom(premier, Cote::Joueur),
        nom(second, Cote::Joueur),
        prix(apercu),
        duree(apercu.duree),
        apercu.petit.nom()
    )
}

pub fn rite_lance(apercu: &ApercuDuRite) -> String {
    let [premier, second] = &apercu.parents;
    format!(
        "Malvoise trace un cercle de sang autour de {} et {}, et entonne le {RITE_DU_SANG_MELE}. Le petit naîtra dans {}.",
        nom_en_valeur(premier, Cote::Joueur),
        nom_en_valeur(second, Cote::Joueur),
        duree(apercu.duree)
    )
}

pub fn naissance(naissance: &NaissanceDuRite) -> String {
    let [premier, second] = &naissance.parents;
    format!(
        "Le cercle de sang se referme et le petit en naît : {}, au niveau {}. {} et {} vous rejoignent.",
        nom_en_valeur(&naissance.petit, Cote::Joueur),
        naissance.petit.niveau().valeur(),
        nom_en_valeur(premier, Cote::Joueur),
        nom_en_valeur(second, Cote::Joueur)
    )
}

/// Le rite en cours, à l'étal : ses parents et le temps qui reste.
pub fn etat_du_rite(partie: &Partie, maintenant: Horodatage) -> Option<String> {
    let rite = partie.rite()?;
    let [premier, second] = rite.parents();
    let fin = if rite.est_fini(maintenant) {
        "achevé, le petit attend.".to_string()
    } else {
        format!(
            "achevé dans {}.",
            duree(maintenant.duree_jusqu_a(rite.fin()))
        )
    };
    Some(format!(
        "{RITE_DU_SANG_MELE} : {} et {}, {fin}",
        nom(premier, Cote::Joueur),
        nom(second, Cote::Joueur)
    ))
}

/// Au camp, quand le petit attend d'être accueilli.
pub fn rite_acheve(partie: &Partie, maintenant: Horodatage) -> Option<String> {
    partie
        .rite()
        .filter(|rite| rite.est_fini(maintenant))
        .map(|_| format!("Le {RITE_DU_SANG_MELE} est achevé : le petit attend auprès de Malvoise."))
}

pub fn erreur_rite(erreur: &ErreurRite) -> String {
    match erreur {
        ErreurRite::AucunMarchand => {
            erreur_marchand(&bersemon_core::marchand::ErreurMarchand::AucunMarchand)
        }
        ErreurRite::RiteEnCours => {
            "Malvoise accomplit déjà un rite : un seul à la fois.".to_string()
        }
        ErreurRite::FamillesDifferentes => {
            "Le rite n'unit que deux monstres d'une même famille d'évolution.".to_string()
        }
        ErreurRite::AucunePlacePourLePetit => {
            "Il faut une place libre au camp pour le petit.".to_string()
        }
        ErreurRite::RessourcesManquantes(manquantes) => {
            format!("Il vous manque {}.", cout(manquantes))
        }
        ErreurRite::ArgentInsuffisant { prix, argent } => erreur_marchand(
            &bersemon_core::marchand::ErreurMarchand::ArgentInsuffisant {
                prix: *prix,
                argent: *argent,
            },
        ),
        ErreurRite::AucunRite => "Aucun rite n'est en cours.".to_string(),
        ErreurRite::PasEncoreFini(_) => "Le rite n'est pas encore achevé.".to_string(),
        ErreurRite::Collection(erreur) => erreur_collection(erreur),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::contenu::ContenuV1;
    use bersemon_core::espece::EspeceId;
    use bersemon_core::monstre::Monstre;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;

    #[test]
    fn la_question_dit_le_prix_la_duree_et_le_petit() {
        let contenu = ContenuV1::new();
        let creer = |id: &str, surnom: &str| {
            let mut monstre = Monstre::creer(
                contenu.espece(&EspeceId::new(id)).unwrap(),
                Niveau::new(10).unwrap(),
                Default::default(),
                false,
            )
            .unwrap();
            monstre.renommer(Some(Nom::new(surnom).unwrap()));
            monstre
        };
        let loup = contenu.espece(&EspeceId::new("loup")).unwrap();
        let apercu = ApercuDuRite {
            parents: [creer("loup", "Croc"), creer("warg", "Bise")],
            ressources: vec![(loup.ressource().unwrap().clone(), 3)],
            petit: loup,
            prix: 50,
            duree: DUREE_DU_RITE,
        };
        assert_eq!(
            question_du_rite(&apercu),
            "Confier Croc et Bise à Malvoise pour le Rite du sang mêlé ? Il prendra Croc de loup (3) et 50 pièces d'argent. Dans 2 h, le petit naîtra, de l'espèce Loup, au niveau 1, avec la meilleure valeur interne de ses parents pour chaque stat. D'ici là, les deux restent auprès de Malvoise."
        );
        assert_eq!(
            service_du_rite(),
            "Rite du sang mêlé : 3 ressources de l'espèce du petit et 50 pièces d'argent, 2 h"
        );
    }
}
