//! La pêche d'Ansgar sur Discord (section 13.8) : l'état de la ligne, de
//! quoi la lancer ou la relever, puis le monstre pêché, gardé ou relâché.
//! Ansgar y prépare aussi ses appâts et ses parfums, dans des menus.

use bersemon_core::appat::Appat;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::monstre::Monstre;
use bersemon_core::parfum::Parfum;
use bersemon_core::partie::Partie;
use bersemon_core::peche::CanneAPeche;
use bersemon_presentation::atelier::{
    effet_de_l_appat, effet_du_parfum, etat_de_l_atelier, nom_de_l_appat, nom_du_parfum,
    offre_de_parfum, recette_d_un_parfum_d_espece,
};
use bersemon_presentation::expeditions_automatiques::duree;
use bersemon_presentation::forge::cout;
use bersemon_presentation::monstres::fiche_du_monstre;
use bersemon_presentation::peche::{
    cannes_possedees, description_de_la_canne, etat_de_la_ligne, question_de_la_prise,
};

use crate::composants::{Cle, PARFUM_DE_CHAROGNE};
use crate::ecran_camp::retour;
use crate::message::{
    OptionDeMenu, PRINCIPAL, Question, RANGEES_MAX, SECONDAIRE, SUCCES, bouton, menu, menus,
    rangees,
};

/// Une ligne à l'eau qui n'est pas encore prête ne propose que le retour.
/// Les parfums ne se proposent pas quand le héros en porte déjà un ; au
/// delà des rangées libres, les parfums d'espèces attendent.
pub fn peche(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> Question {
    if !partie.peche().pecheur_au_camp() {
        return Question {
            texte: "Personne ne pêche au camp.".to_string(),
            rangees: rangees(vec![retour()]),
        };
    }
    let mut boutons = Vec::new();
    let peche = partie.peche();
    match peche.ligne() {
        // Un bouton par canne fabriquée : 3 au plus (section 13.8).
        None => boutons.extend(peche.cannes().iter().map(|canne| {
            bouton(
                Cle::LancerAvec(*canne),
                &format!(
                    "Lancer : {} ({})",
                    canne.nom().to_lowercase(),
                    duree(partie.duree_de_la_ligne(*canne))
                ),
                PRINCIPAL,
            )
        })),
        Some(_) if peche.est_prete(maintenant) => {
            boutons.push(bouton(Cle::ReleverLaLigne, "Relever la ligne", SUCCES));
        }
        Some(_) => {}
    }
    if let Some(suivante) = CanneAPeche::TOUTES
        .into_iter()
        .find(|canne| !peche.cannes().contains(canne))
    {
        boutons.push(bouton(
            Cle::FabriquerCanne(suivante),
            &format!("Fabriquer : {}", suivante.nom().to_lowercase()),
            SECONDAIRE,
        ));
    }
    boutons.push(retour());
    let mut lignes = rangees(boutons);
    let atelier = contenu.atelier();
    let appats = Appat::TOUS
        .into_iter()
        .map(|appat| OptionDeMenu {
            libelle: format!("{} ({})", nom_de_l_appat(appat), effet_de_l_appat(appat)),
            valeur: appat.id().to_string(),
            description: Some(cout(atelier.cout_de_l_appat(appat))),
        })
        .collect();
    lignes.extend(menu(Cle::PreparerAppat, "Préparer un appât", appats));
    if partie.parfum_porte().is_none() {
        let charogne = Parfum::Charogne;
        let mut parfums = vec![OptionDeMenu {
            libelle: nom_du_parfum(&charogne),
            valeur: PARFUM_DE_CHAROGNE.to_string(),
            description: offre_de_parfum(&charogne, atelier),
        }];
        for espece in partie.parfums_d_especes_possibles(contenu.especes(), atelier) {
            let parfum = Parfum::Espece(espece.clone());
            parfums.push(OptionDeMenu {
                libelle: nom_du_parfum(&parfum),
                valeur: espece.id().as_str().to_string(),
                description: offre_de_parfum(&parfum, atelier),
            });
        }
        let libres = RANGEES_MAX.saturating_sub(lignes.len());
        lignes.extend(menus(
            Cle::PreparerParfum,
            "Préparer un parfum",
            parfums,
            libres,
        ));
    }
    // Les parfums d'espèces ne se proposent que possibles : leur recette
    // générale le dit.
    let parfum = match partie.parfum_porte() {
        Some(_) => String::new(),
        None => format!(
            "\nUn parfum attire des hordes ({}). {}",
            effet_du_parfum(&Parfum::Charogne),
            recette_d_un_parfum_d_espece(atelier)
        ),
    };
    Question {
        texte: format!(
            "Pêche d'Ansgar :\n{}\n{}{}\n{}{parfum}",
            etat_de_la_ligne(partie, maintenant),
            cannes_possedees(partie),
            CanneAPeche::TOUTES
                .into_iter()
                .find(|canne| !partie.peche().cannes().contains(canne))
                .map(|suivante| format!(
                    " Canne suivante : {}.",
                    description_de_la_canne(suivante, contenu.peche())
                ))
                .unwrap_or_default(),
            etat_de_l_atelier(partie, contenu)
        ),
        rangees: lignes,
    }
}

/// Le monstre pêché, avec sa fiche, que le joueur garde ou relâche.
pub fn prise(monstre: &Monstre) -> Question {
    Question {
        texte: format!(
            "{}\n\n{}",
            fiche_du_monstre(monstre),
            question_de_la_prise(monstre)
        ),
        rangees: rangees(vec![
            bouton(Cle::GarderLaPrise, "Garder", PRINCIPAL),
            bouton(Cle::RelacherLaPrise, "Relâcher", SECONDAIRE),
        ]),
    }
}
