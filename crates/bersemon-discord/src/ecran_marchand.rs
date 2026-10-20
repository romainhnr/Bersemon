//! Malvoise sur Discord (section 14.2) : son étal, où l'on achète
//! soins, capsules et pierres d'évolution, et la vente des monstres. Les achats se
//! font sans confirmation ; la vente est confirmée par le déroulement.

use bersemon_core::contenu::ContenuV1;
use bersemon_core::marchand::{PRIX_CAPSULE_D_HABILETE, PRIX_PIERRE_D_EVOLUTION, prix_du_soin};
use bersemon_core::partie::Partie;
use bersemon_core::pierre::PierreDEvolution;
use bersemon_presentation::marchand::{
    capsules_en_vente, etal as texte_de_l_etal, offre_de_rachat, offre_de_ressources,
    ressources_a_vendre,
};
use bersemon_presentation::monstres::decrire_attaque;
use bersemon_presentation::partie::argent;
use bersemon_presentation::soins::{le_soin, soins};

use crate::composants::{Cle, Vue};
use crate::ecran_camp::{menus_de_monstres_decrits, retour};
use crate::message::{
    DANGER, OptionDeMenu, PRINCIPAL, Question, Rangee, SECONDAIRE, bouton, menu, rangees,
};

/// Soins achetés d'un bouton : au plus 4 soins, soit 8 boutons sur deux
/// rangées.
const LOTS_DE_SOINS: [u32; 2] = [1, 5];

pub fn etal(partie: &Partie, contenu: &ContenuV1) -> Question {
    if !partie.marchand_au_camp() {
        return sans_marchand();
    }
    let mut lignes: Vec<Rangee> = rangees(
        partie
            .soins_en_vente()
            .into_iter()
            .flat_map(|soin| LOTS_DE_SOINS.map(|nombre| (soin, nombre)))
            .map(|(soin, nombre)| {
                let libelle = format!(
                    "Acheter {} ({})",
                    soins(soin, nombre),
                    argent(nombre * prix_du_soin(soin))
                );
                bouton(Cle::AcheterSoins(soin, nombre), &libelle, PRINCIPAL)
            })
            .collect(),
    );
    let capsules = capsules_en_vente(partie, contenu.capsules())
        .into_iter()
        .map(|capsule| OptionDeMenu {
            libelle: format!("{} : {}", capsule.attaque().nom(), argent(capsule.prix())),
            valeur: capsule.id().as_str().to_string(),
            description: Some(decrire_attaque(capsule.attaque())),
        })
        .collect();
    lignes.extend(menu(
        Cle::AcheterCapsule,
        "Acheter une capsule d'attaque",
        capsules,
    ));
    // Au plus une âme par zone : le menu reste loin de ses 25 options.
    let ames = partie
        .ames_portees()
        .iter()
        .map(|marchand| OptionDeMenu {
            libelle: format!("Âme de {}", marchand.nom()),
            valeur: marchand.id().as_str().to_string(),
            description: Some(format!(
                "Malvoise vendra {}",
                le_soin(marchand.soin()).to_lowercase()
            )),
        })
        .collect();
    lignes.extend(menu(Cle::DonnerAme, "Donner une âme à Malvoise", ames));
    let libelle = format!(
        "Acheter une capsule d'habileté ({})",
        argent(PRIX_CAPSULE_D_HABILETE)
    );
    // Cinq boutons : la dernière rangée est pleine (section 5.4).
    let mut boutons = vec![bouton(Cle::AcheterCapsuleDHabilete, &libelle, PRINCIPAL)];
    for pierre in PierreDEvolution::TOUTES {
        let libelle = format!(
            "Acheter une {} ({})",
            pierre.nom(),
            argent(PRIX_PIERRE_D_EVOLUTION)
        );
        boutons.push(bouton(Cle::AcheterPierre(pierre), &libelle, PRINCIPAL));
    }
    boutons.extend([
        bouton(
            Cle::Afficher(Vue::Vente),
            "Vendre monstres et ressources",
            DANGER,
        ),
        retour(),
    ]);
    lignes.extend(rangees(boutons));
    // La dernière rangée est pleine : le service se rend depuis la gestion
    // des monstres (section 13.7).
    let texte = format!(
        "{}\nMalvoise rend un monstre shiny contre un Éclat d'astre (vous en avez {}) : bouton \"Rendre shiny (Malvoise)\" de la gestion des monstres.",
        texte_de_l_etal(partie),
        partie.eclats_d_astre()
    );
    Question {
        texte,
        rangees: lignes,
    }
}

/// Le prix de chaque monstre est dans les menus.
pub fn vente(partie: &Partie) -> Question {
    if !partie.marchand_au_camp() {
        return sans_marchand();
    }
    let mut lignes = menus_de_monstres_decrits(
        partie.collection(),
        Cle::Vendre,
        "Vendre",
        |_| true,
        offre_de_rachat,
    );
    // Neuf sortes au plus : le menu reste loin de ses 25 options.
    let ressources = ressources_a_vendre(partie)
        .into_iter()
        .map(|(ressource, nombre)| OptionDeMenu {
            libelle: offre_de_ressources(&ressource, nombre),
            valeur: ressource.id().as_str().to_string(),
            description: Some("Toutes celles du sac et du coffre".to_string()),
        })
        .collect();
    lignes.extend(menu(
        Cle::VendreRessources,
        "Vendre des ressources",
        ressources,
    ));
    lignes.extend(rangees(vec![
        bouton(
            Cle::Afficher(Vue::Marchand),
            "Retour à Malvoise",
            SECONDAIRE,
        ),
        retour(),
    ]));
    Question {
        texte: format!(
            "Malvoise rachète vos monstres et vos ressources : choisissez ce que vous vendez, son prix est indiqué dans le menu. Vous avez {}.",
            argent(partie.argent())
        ),
        rangees: lignes,
    }
}

/// Une vue restée d'avant le marchand, ou ouverte par un message dépassé.
fn sans_marchand() -> Question {
    Question {
        texte: "Malvoise ne s'est pas encore installé au camp.".to_string(),
        rangees: rangees(vec![retour()]),
    }
}
