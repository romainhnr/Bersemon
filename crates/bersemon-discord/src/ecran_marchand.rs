//! Malvoise sur Discord (section 14.2) : son étal, où l'on achète
//! soins, filets, capsules et pierres d'évolution, la vente des monstres, et leur
//! achat ou leur location. Les achats se font sans confirmation ; la vente
//! et le retour d'un monstre loué sont confirmés par le déroulement.

use bersemon_core::contenu::ContenuV1;
use bersemon_core::marchand::{
    EXPEDITIONS_DE_LOCATION, OffreDeMonstre, PRIX_CAPSULE_D_HABILETE, PRIX_PIERRE_D_EVOLUTION,
    prix_du_filet, prix_du_soin,
};
use bersemon_core::partie::Partie;
use bersemon_core::pierre::PierreDEvolution;
use bersemon_presentation::beherit::{FORGER_LE_BEHERIT, peut_forger_le_beherit};
use bersemon_presentation::filets::filets;
use bersemon_presentation::marchand::{
    capsules_en_vente, etal as texte_de_l_etal, location_en_cours, offre_d_achat,
    offre_de_location, offre_de_rachat, offre_de_ressources, ressources_a_vendre,
};
use bersemon_presentation::monstres::decrire_attaque;
use bersemon_presentation::partie::argent;
use bersemon_presentation::soins::{le_soin, soins};

use crate::composants::{BEHERIT, Cle, Vue};
use crate::ecran_camp::{menus_de_monstres_decrits, retour};
use crate::message::{
    DANGER, OptionDeMenu, PRINCIPAL, Question, Rangee, SECONDAIRE, bouton, menu, rangees,
};

/// Soins achetés d'un bouton : au plus 4 soins, soit 8 boutons, que
/// complètent les 2 filets en vente pour remplir deux rangées.
const LOTS_DE_SOINS: [u32; 2] = [1, 5];
/// Filets achetés d'un bouton : le lot que forge Gunther (section 13.5).
const LOT_DE_FILETS: u32 = 5;

pub fn etal(partie: &Partie, contenu: &ContenuV1) -> Question {
    if !partie.marchand_au_camp() {
        return sans_marchand();
    }
    let mut achats: Vec<_> = partie
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
        .collect();
    for filet in partie.filets_en_vente() {
        let Some(prix) = prix_du_filet(filet) else {
            continue;
        };
        let libelle = format!(
            "Acheter {} ({})",
            filets(filet, LOT_DE_FILETS),
            argent(LOT_DE_FILETS * prix)
        );
        achats.push(bouton(
            Cle::AcheterFilets(filet, LOT_DE_FILETS),
            &libelle,
            PRINCIPAL,
        ));
    }
    let mut lignes: Vec<Rangee> = rangees(achats);
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
    // Au plus une âme par zone : le menu reste loin de ses 25 options. Les
    // âmes des Apôtres s'y donnent ensemble, pour forger le Béhérit (section
    // 9.4).
    let mut ames: Vec<OptionDeMenu> = partie
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
    if peut_forger_le_beherit(partie, contenu) {
        ames.push(OptionDeMenu {
            libelle: FORGER_LE_BEHERIT.to_string(),
            valeur: BEHERIT.to_string(),
            description: Some("Les trois âmes seront perdues".to_string()),
        });
    }
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
        "{}\nMalvoise rend un monstre shiny contre un Philtre chromatique (vous en avez {}) : bouton \"Rendre shiny (Malvoise)\" de la gestion des monstres.",
        texte_de_l_etal(partie),
        partie.philtres_chromatiques()
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
    // La dernière rangée de l'étal est pleine : l'achat et la location de
    // monstres passent par la vente (section 14.2).
    lignes.extend(rangees(vec![
        bouton(Cle::Afficher(Vue::Achats), "Acheter ou louer", PRINCIPAL),
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

/// Les espèces communes et rares des zones menées au bout : 13 et 8 dans le
/// contenu, loin des 25 options d'un menu (section 14.2).
pub fn achats(partie: &Partie, contenu: &ContenuV1) -> Question {
    if !partie.marchand_au_camp() {
        return sans_marchand();
    }
    // Sans les types : ceux d'une espèce seulement rencontrée restent
    // cachés jusqu'à sa capture (section 5.6).
    let option = |offre: &OffreDeMonstre, libelle: String| OptionDeMenu {
        libelle,
        valeur: offre.espece.id().as_str().to_string(),
        description: None,
    };
    let mut lignes: Vec<Rangee> = Vec::new();
    let a_vendre = partie
        .offres_d_achat(contenu.zones(), contenu.especes())
        .iter()
        .map(|offre| option(offre, offre_d_achat(offre)))
        .collect();
    lignes.extend(menu(Cle::AcheterMonstre, "Acheter un monstre", a_vendre));
    let a_louer = partie
        .offres_de_location(contenu.zones(), contenu.piste())
        .iter()
        .map(|offre| option(offre, offre_de_location(offre)))
        .collect();
    lignes.extend(menu(Cle::LouerMonstre, "Louer un monstre", a_louer));
    let mut boutons = Vec::new();
    if let Some((_, monstre)) = partie.collection().monstre_loue() {
        boutons.push(bouton(
            Cle::RendreLeMonstreLoue,
            &format!("Rendre {} à Malvoise", monstre.nom()),
            DANGER,
        ));
    }
    boutons.extend([
        bouton(
            Cle::Afficher(Vue::Marchand),
            "Retour à Malvoise",
            SECONDAIRE,
        ),
        retour(),
    ]);
    lignes.extend(rangees(boutons));
    let location = location_en_cours(partie)
        .map(|ligne| format!("\n{ligne}"))
        .unwrap_or_default();
    Question {
        texte: format!(
            "Malvoise vend des monstres communs des zones que vous avez explorées, et loue des monstres rares pour {EXPEDITIONS_DE_LOCATION} expéditions : un monstre loué ne fait que combattre, puis repart. Vous avez {}.{location}",
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
