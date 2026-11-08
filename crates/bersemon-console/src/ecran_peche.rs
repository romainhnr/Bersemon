//! La pêche d'Ansgar (section 13.8) : le joueur lance la ligne, la relève
//! une fois prête, puis garde ou relâche le monstre pêché. Ansgar y prépare
//! aussi ses appâts et ses parfums, avec les matières de la ligne.

use bersemon_core::appat::Appat;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::monstre::Monstre;
use bersemon_core::parfum::Parfum;
use bersemon_core::partie::Partie;
use bersemon_presentation::atelier::{
    etat_de_l_atelier, nom_du_parfum, offre_d_appat, offre_de_parfum, recette_d_un_parfum_d_espece,
};
use bersemon_presentation::mise_en_valeur::sans_marques;
use bersemon_presentation::monstres::fiche_du_monstre;
use bersemon_presentation::peche::{etat_de_la_ligne, question_de_la_prise};

use crate::horloge::maintenant;
use crate::saisie::Entree;

/// Ce que propose l'écran, dans l'ordre des numéros.
enum EntreeDeLaPeche {
    Ligne(Choix, &'static str),
    Appats,
    Parfums,
}

/// `Some(None)` au retour vers le menu du camp. La ligne ne propose d'être
/// lancée que sans ligne à l'eau, et relevée qu'une fois prête ; un parfum
/// ne se prépare pas quand le héros en porte déjà un.
pub fn choisir(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    let maintenant = maintenant();
    let peche = partie.peche();
    println!();
    println!("Pêche d'Ansgar :");
    println!("{}", etat_de_la_ligne(partie, maintenant));
    println!("{}", etat_de_l_atelier(partie, contenu));
    let mut entrees = Vec::new();
    match peche.ligne() {
        None => entrees.push(EntreeDeLaPeche::Ligne(
            Choix::LancerLaLigne,
            "Lancer la ligne",
        )),
        Some(_) if peche.est_prete(maintenant) => {
            entrees.push(EntreeDeLaPeche::Ligne(
                Choix::ReleverLaLigne,
                "Relever la ligne",
            ));
        }
        Some(_) => {}
    }
    entrees.push(EntreeDeLaPeche::Appats);
    if partie.parfum_porte().is_none() {
        entrees.push(EntreeDeLaPeche::Parfums);
    }
    for (position, entree_du_menu) in entrees.iter().enumerate() {
        let libelle = match entree_du_menu {
            EntreeDeLaPeche::Ligne(_, libelle) => libelle,
            EntreeDeLaPeche::Appats => "Préparer un appât",
            EntreeDeLaPeche::Parfums => "Préparer un parfum",
        };
        println!("  {}. {libelle}", position + 1);
    }
    println!("  0. Retour au camp");
    let numero = entree.choix(0, entrees.len())?;
    if numero == 0 {
        return Some(None);
    }
    match entrees.remove(numero - 1) {
        EntreeDeLaPeche::Ligne(choix, _) => Some(Some(choix)),
        EntreeDeLaPeche::Appats => choisir_appat(contenu, entree),
        EntreeDeLaPeche::Parfums => choisir_parfum(partie, contenu, entree),
    }
}

/// Les trois appâts, avec leur recette : Ansgar refuse ce qui manque.
fn choisir_appat(contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    println!("Quel appât préparer ? Il ira dans le coffre.");
    for (position, appat) in Appat::TOUS.iter().enumerate() {
        println!(
            "  {}. {}",
            position + 1,
            offre_d_appat(*appat, contenu.atelier())
        );
    }
    println!("  0. Retour");
    let choix = entree.choix(0, Appat::TOUS.len())?;
    Some((choix > 0).then(|| Choix::PreparerAppat(Appat::TOUS[choix - 1])))
}

/// Le parfum de charogne, puis ceux des espèces que le joueur peut faire
/// préparer maintenant.
fn choisir_parfum(
    partie: &Partie,
    contenu: &ContenuV1,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    let atelier = contenu.atelier();
    let especes = partie.parfums_d_especes_possibles(contenu.especes(), atelier);
    println!(
        "Quel parfum préparer ? Vous le porterez jusqu'à votre prochaine expédition dans une zone."
    );
    println!("{}", recette_d_un_parfum_d_espece(atelier));
    let charogne = offre_de_parfum(&Parfum::Charogne, atelier)
        .unwrap_or_else(|| nom_du_parfum(&Parfum::Charogne));
    println!("  1. {charogne}");
    for (position, espece) in especes.iter().enumerate() {
        let parfum = Parfum::Espece((*espece).clone());
        let offre = offre_de_parfum(&parfum, atelier).unwrap_or_else(|| nom_du_parfum(&parfum));
        println!("  {}. {offre}", position + 2);
    }
    println!("  0. Retour");
    let choix = entree.choix(0, especes.len() + 1)?;
    Some(match choix {
        0 => None,
        1 => Some(Choix::PreparerParfumDeCharogne),
        _ => Some(Choix::PreparerParfumDEspece(
            especes[choix - 2].id().clone(),
        )),
    })
}

/// Le monstre pêché, avec sa fiche, que le joueur garde ou relâche.
pub fn prise(monstre: &Monstre, entree: &mut Entree) -> Option<Choix> {
    println!();
    println!("{}", fiche_du_monstre(monstre));
    println!("{}", sans_marques(&question_de_la_prise(monstre)));
    println!("  1. Garder");
    println!("  2. Relâcher");
    let choix = match entree.choix(1, 2)? {
        1 => Choix::GarderLaPrise,
        _ => Choix::RelacherLaPrise,
    };
    Some(choix)
}
