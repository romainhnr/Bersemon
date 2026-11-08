//! Textes de l'atelier d'Ansgar (section 13.8) : les appâts, lancés en
//! combat avant un filet (section 11.2), les parfums, portés jusqu'à la
//! prochaine expédition dans une zone (section 9.2), et les matières que
//! remonte la ligne. Communs à la console et au bot Discord ; textes dans
//! docs/contenu-v1.md, section 53.

use bersemon_core::appat::{Appat, Appats};
use bersemon_core::atelier::{AtelierDAnsgar, ErreurAtelier, RESSOURCES_DU_PARFUM_D_ESPECE};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::parfum::Parfum;
use bersemon_core::partie::Partie;
use bersemon_core::ressource::Ressource;

use crate::forge::cout;
use crate::mise_en_valeur::important;
use crate::partie::{enumerer, ressources};

/// "Appât de vers", en tête d'un libellé.
pub fn nom_de_l_appat(appat: Appat) -> &'static str {
    match appat {
        Appat::Vers => "Appât de vers",
        Appat::Chair => "Appât de chair",
        Appat::Ambre => "Appât d'ambre",
    }
}

/// "1 appât de vers", "2 appâts d'ambre".
pub fn appats(appat: Appat, nombre: u32) -> String {
    let (singulier, pluriel) = match appat {
        Appat::Vers => ("appât de vers", "appâts de vers"),
        Appat::Chair => ("appât de chair", "appâts de chair"),
        Appat::Ambre => ("appât d'ambre", "appâts d'ambre"),
    };
    if nombre > 1 {
        format!("{nombre} {pluriel}")
    } else {
        format!("{nombre} {singulier}")
    }
}

/// Ceux qu'il y a, du plus faible au plus fort.
pub fn liste_d_appats(reserve: &Appats) -> Vec<String> {
    reserve
        .presents()
        .map(|(appat, nombre)| appats(appat, nombre))
        .collect()
}

/// "prochain filet x1,25".
pub fn effet_de_l_appat(appat: Appat) -> &'static str {
    match appat {
        Appat::Vers => "prochain filet x1,25",
        Appat::Chair => "prochain filet x1,5",
        Appat::Ambre => "prochain filet x2",
    }
}

/// En combat, l'appât jeté au monstre adverse actif.
pub fn appat_lance(appat: Appat, cible: &str) -> String {
    let multiplicateur = match appat {
        Appat::Vers => "1,25",
        Appat::Chair => "1,5",
        Appat::Ambre => "2",
    };
    format!(
        "Vous jetez un {} à {cible}, qui s'en repaît : la chance de votre prochain filet est multipliée par {multiplicateur}.",
        nom_de_l_appat(appat).to_lowercase()
    )
}

/// "Parfum de charogne", "Parfum de Loup".
pub fn nom_du_parfum(parfum: &Parfum) -> String {
    match parfum {
        Parfum::Charogne => "Parfum de charogne".to_string(),
        Parfum::Espece(espece) => format!("Parfum de {}", espece.nom()),
    }
}

/// Ce que fait un parfum, pour l'écran d'Ansgar.
pub fn effet_du_parfum(parfum: &Parfum) -> String {
    let hordes =
        "groupes plus nombreux, plus de monstres rares, 3 niveaux de plus, plus de ressources";
    match parfum {
        Parfum::Charogne => hordes.to_string(),
        Parfum::Espece(espece) => {
            format!("{hordes}, et l'espèce {} là où elle vit", espece.nom())
        }
    }
}

/// "2 Vers de vase", "1 Vers de vase et 1 Chair de noyé".
fn recette(cout_de_la_recette: &[(Ressource, u32)]) -> String {
    let liste: Vec<String> = cout_de_la_recette
        .iter()
        .map(|(ressource, nombre)| format!("{nombre} {}", ressource.nom()))
        .collect();
    enumerer(&liste)
}

/// "Appât de vers (prochain filet x1,25) : 2 Vers de vase".
pub fn offre_d_appat(appat: Appat, atelier: &AtelierDAnsgar) -> String {
    format!(
        "{} ({}) : {}",
        nom_de_l_appat(appat),
        effet_de_l_appat(appat),
        recette(atelier.cout_de_l_appat(appat))
    )
}

/// "Parfum de charogne : 3 Chair de noyé et 2 Vers de vase".
pub fn offre_de_parfum(parfum: &Parfum, atelier: &AtelierDAnsgar) -> Option<String> {
    atelier
        .cout_du_parfum(parfum)
        .map(|cout_du_parfum| format!("{} : {}", nom_du_parfum(parfum), recette(&cout_du_parfum)))
}

/// La recette générale, puisque seuls les parfums d'espèces possibles sont
/// proposés.
pub fn recette_d_un_parfum_d_espece(atelier: &AtelierDAnsgar) -> String {
    let base = recette(atelier.base_du_parfum_d_espece());
    let base = if base.is_empty() {
        String::new()
    } else {
        format!("{base} et ")
    };
    format!(
        "Le parfum d'une espèce rencontrée demande {base}{RESSOURCES_DU_PARFUM_D_ESPECE} de la ressource qu'elle lâche."
    )
}

/// L'état de l'atelier, sous celui de la ligne : le parfum porté, ce que
/// font appâts et parfums, et les matières que le joueur possède.
pub fn etat_de_l_atelier(partie: &Partie, contenu: &ContenuV1) -> String {
    let mut lignes = vec![
        "Ansgar prépare des appâts, à lancer en combat avant un filet, et des parfums, qui attirent des hordes à votre prochaine expédition dans une zone.".to_string(),
    ];
    lignes.push(match partie.parfum_porte() {
        Some(parfum) => format!(
            "Vous portez le {} jusqu'à votre prochaine expédition dans une zone ({}).",
            nom_du_parfum(parfum),
            effet_du_parfum(parfum)
        ),
        None => "Vous ne portez aucun parfum.".to_string(),
    });
    let matieres: Vec<String> = contenu
        .peche()
        .matieres()
        .iter()
        .map(|(matiere, _)| {
            let nombre = partie
                .sac()
                .nombre_de_ressources(matiere.id())
                .saturating_add(partie.coffre_du_camp().nombre_de_ressources(matiere.id()));
            format!("{} ({nombre})", matiere.nom())
        })
        .collect();
    lignes.push(format!("Matières de la ligne : {}.", matieres.join(", ")));
    let appats = liste_d_appats(&{
        let mut tous = *partie.sac().appats();
        for (appat, nombre) in partie.coffre_du_camp().appats().presents() {
            tous.ajouter(appat, nombre);
        }
        tous
    });
    lignes.push(if appats.is_empty() {
        "Appâts : aucun.".to_string()
    } else {
        format!("Appâts, du sac et du coffre : {}.", appats.join(", "))
    });
    lignes.join("\n")
}

/// Avec la prise, les matières rangées dans le coffre.
pub fn matieres_pechees(matieres: &[(Ressource, u32)]) -> String {
    let liste: Vec<String> = matieres
        .iter()
        .map(|(matiere, nombre)| ressources(matiere, *nombre))
        .collect();
    format!(
        "La ligne remonte aussi : {}. Vous les rangez dans le coffre.",
        enumerer(&liste)
    )
}

pub fn appat_prepare(appat: Appat) -> String {
    format!(
        "Ansgar prépare un {}. Vous le rangez dans le coffre.",
        nom_de_l_appat(appat).to_lowercase()
    )
}

pub fn parfum_prepare(parfum: &Parfum) -> String {
    format!(
        "Ansgar mêle ses matières dans une fiole et vous en asperge : vous portez le {}, jusqu'à votre prochaine expédition dans une zone.",
        nom_du_parfum(parfum)
    )
}

/// Au départ de l'expédition.
pub fn parfum_repandu(parfum: &Parfum) -> String {
    let appel = match parfum {
        Parfum::Charogne => String::new(),
        Parfum::Espece(espece) => format!(", et l'odeur attire l'espèce {}", espece.nom()),
    };
    important(&format!(
        "Le {} se répand autour de vous : des hordes plus fortes approchent{appel}.",
        nom_du_parfum(parfum)
    ))
}

pub fn erreur_atelier(erreur: &ErreurAtelier) -> String {
    match erreur {
        ErreurAtelier::AucunPecheur => "Personne ne prépare d'appâts au camp.".to_string(),
        ErreurAtelier::RessourcesManquantes(manquantes) => {
            format!("Il vous manque : {}.", cout(manquantes))
        }
        ErreurAtelier::ParfumDejaPorte(parfum) => format!(
            "Vous portez déjà le {} : il se répandra à votre prochaine expédition dans une zone.",
            nom_du_parfum(parfum)
        ),
        ErreurAtelier::EspeceSansRessource(espece) => {
            format!("{} ne lâche rien : aucun parfum ne l'attire.", espece.nom())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noms_et_recettes() {
        let contenu = ContenuV1::new();
        let atelier = contenu.atelier();
        assert_eq!(appats(Appat::Ambre, 2), "2 appâts d'ambre");
        assert_eq!(
            offre_d_appat(Appat::Chair, atelier),
            "Appât de chair (prochain filet x1,5) : 1 Vers de vase et 1 Chair de noyé"
        );
        assert_eq!(
            offre_de_parfum(&Parfum::Charogne, atelier).as_deref(),
            Some("Parfum de charogne : 3 Chair de noyé et 2 Vers de vase")
        );
        assert_eq!(
            recette_d_un_parfum_d_espece(atelier),
            "Le parfum d'une espèce rencontrée demande 1 Ambre noir et 3 de la ressource qu'elle lâche."
        );
    }
}
