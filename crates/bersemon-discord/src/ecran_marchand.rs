//! Malvoise sur Discord (section 14.2) : son étal, où l'on achète soins,
//! filets, capsules et pierres d'évolution et où se lit l'état du Rite du
//! sang mêlé, la vente des monstres, et leur achat ou leur location. Les achats se font sans confirmation ; la vente
//! et le retour d'un monstre loué sont confirmés par le déroulement.

use bersemon_core::contenu::ContenuV1;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::marchand::{
    EXPEDITIONS_DE_LOCATION, OffreDeMonstre, PRIX_CAPSULE_D_HABILETE, prix_de_la_pierre,
    prix_du_filet, prix_du_soin, ressources_d_un_monstre,
};
use bersemon_core::objet_d_experience::ObjetDExperience;
use bersemon_core::partie::Partie;
use bersemon_core::pierre::PierreDEvolution;
use bersemon_presentation::beherit::{FORGER_LE_BEHERIT, peut_forger_le_beherit};
use bersemon_presentation::filets::filets;
use bersemon_presentation::marchand::{
    capsules_en_vente, capsules_obtenues, etal as texte_de_l_etal, location_en_cours,
    offre_d_achat, offre_de_location, offre_de_rachat, offre_de_ressources,
    offre_de_transformation, ressources_a_vendre,
};
use bersemon_presentation::monstres::decrire_attaque;
use bersemon_presentation::partie::argent;
use bersemon_presentation::recit::effet_d_un_objet_d_experience;
use bersemon_presentation::rite::{etat_du_rite, service_du_rite};
use bersemon_presentation::soins::{effet_du_soin, le_soin, soins};

use crate::composants::{
    BEHERIT, Cle, Vue, lot_de_soins_en_texte, objet_d_experience_en_valeur, pierre_en_texte,
};
use crate::ecran_camp::{menus_de_monstres_decrits, retour};
use crate::message::{
    Bouton, DANGER, OptionDeMenu, PRINCIPAL, Question, RANGEES_MAX, Rangee, SECONDAIRE, bouton,
    menu, menus, rangees,
};

/// Soins achetés par 1 ou par 5 ; l'élixir et le grand élixir, chers, par 1
/// seulement (section 14.4). Avec la panacée, leurs boutons et ceux des
/// filets dépasseraient deux rangées : les soins se choisissent dans un
/// menu.
const LOTS_DE_SOINS: [u32; 2] = [1, 5];
/// Filets achetés d'un bouton : le lot que forge Gunther (section 13.5).
const LOT_DE_FILETS: u32 = 5;

pub fn etal(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> Question {
    if !partie.marchand_au_camp() {
        return sans_marchand();
    }
    let lots_de_soins: Vec<OptionDeMenu> = partie
        .soins_en_vente()
        .into_iter()
        .flat_map(|soin| {
            let lots = if soin.releve() {
                &LOTS_DE_SOINS[..1]
            } else {
                &LOTS_DE_SOINS[..]
            };
            lots.iter().map(move |nombre| (soin, *nombre))
        })
        .map(|(soin, nombre)| OptionDeMenu {
            libelle: format!(
                "{} : {}",
                soins(soin, nombre),
                argent(nombre * prix_du_soin(soin))
            ),
            valeur: lot_de_soins_en_texte(soin, nombre),
            description: Some(effet_du_soin(soin).to_string()),
        })
        .collect();
    let mut achats = Vec::new();
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
    // Plus de 25 capsules depuis l'étape 165 : elles ont leur écran, que
    // ce bouton ouvre, à côté des filets.
    if !capsules_en_vente(partie, contenu.capsules()).is_empty() {
        achats.push(bouton(
            Cle::Afficher(Vue::Capsules),
            "Capsules d'attaque",
            PRINCIPAL,
        ));
    }
    let mut lignes: Vec<Rangee> = rangees(achats);
    // Les neuf objets d'évolution, pierres et Chaîne de lien, ont leur
    // menu, un par achat (section 5.4).
    let mut objets_d_evolution: Vec<OptionDeMenu> = PierreDEvolution::TOUTES
        .into_iter()
        .map(|pierre| OptionDeMenu {
            libelle: format!("{} : {}", pierre.nom(), argent(prix_de_la_pierre(pierre))),
            valeur: pierre_en_texte(pierre).to_string(),
            description: None,
        })
        .collect();
    // Les objets d'expérience partagent le menu : l'étal n'a plus de
    // rangée libre (section 14.3).
    objets_d_evolution.extend(
        ObjetDExperience::TOUS
            .into_iter()
            .map(|objet| OptionDeMenu {
                libelle: format!("{} : {}", objet.nom(), argent(objet.prix())),
                valeur: objet_d_experience_en_valeur(objet),
                description: Some(effet_d_un_objet_d_experience(objet)),
            }),
    );
    lignes.extend(menu(Cle::AcheterSoins, "Acheter des soins", lots_de_soins));
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
            description: Some("Les âmes des Apôtres seront perdues".to_string()),
        });
    }
    lignes.extend(menu(Cle::DonnerAme, "Donner une âme à Malvoise", ames));
    let libelle = format!(
        "Acheter une capsule d'habileté ({})",
        argent(PRIX_CAPSULE_D_HABILETE)
    );
    lignes.extend(menu(
        Cle::AcheterObjetDEvolution,
        "Acheter un objet d'évolution ou d'expérience",
        objets_d_evolution,
    ));
    let mut boutons = vec![bouton(Cle::AcheterCapsuleDHabilete, &libelle, PRINCIPAL)];
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
    // Le rite se lance aussi depuis la gestion des monstres ; l'étal en dit
    // l'état (section 13.11).
    let rite = etat_du_rite(partie, maintenant).unwrap_or_else(|| {
        format!(
            "{} : bouton \"Rite du sang mêlé (Malvoise)\" de la gestion des monstres.",
            service_du_rite()
        )
    });
    let texte = format!(
        "{}\nMalvoise rend un monstre shiny contre un Philtre chromatique (vous en avez {}) : bouton \"Rendre shiny (Malvoise)\" de la gestion des monstres.\n{rite}",
        texte_de_l_etal(partie),
        partie.philtres_chromatiques()
    );
    Question {
        texte,
        rangees: lignes,
    }
}

/// Les capsules que le joueur n'a pas encore, en menus de 25 au plus, avec
/// leur prix et l'attaque qu'elles enseignent (section 14.2).
pub fn capsules(partie: &Partie, contenu: &ContenuV1) -> Question {
    if !partie.marchand_au_camp() {
        return sans_marchand();
    }
    let options = capsules_en_vente(partie, contenu.capsules())
        .into_iter()
        .map(|capsule| OptionDeMenu {
            libelle: format!("{} : {}", capsule.attaque().nom(), argent(capsule.prix())),
            valeur: capsule.id().as_str().to_string(),
            description: Some(decrire_attaque(capsule.attaque())),
        })
        .collect();
    let mut lignes = menus(
        Cle::AcheterCapsule,
        "Acheter une capsule d'attaque",
        options,
        RANGEES_MAX - 1,
    );
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
            "Malvoise vend les capsules d'attaque, qui servent sans s'user. Vous avez {}.\n{}",
            argent(partie.argent()),
            capsules_obtenues(partie)
        ),
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
    // Au-delà de 25 sortes, plusieurs menus, dans les rangées que laissent
    // les monstres et les boutons ; les sortes au-delà attendent qu'une
    // autre soit vendue.
    let ressources = ressources_a_vendre(partie)
        .into_iter()
        .map(|(ressource, nombre)| OptionDeMenu {
            libelle: offre_de_ressources(&ressource, nombre),
            valeur: ressource.id().as_str().to_string(),
            description: Some("Toutes celles du sac et du coffre".to_string()),
        })
        .collect();
    let restantes = RANGEES_MAX.saturating_sub(lignes.len() + 1);
    lignes.extend(menus(
        Cle::VendreRessources,
        "Vendre des ressources",
        ressources,
        restantes,
    ));
    // La dernière rangée de l'étal est pleine : l'achat et la location de
    // monstres passent par la vente (section 14.2).
    lignes.extend(rangees(vec![
        bouton(Cle::Afficher(Vue::Achats), "Acheter ou louer", PRINCIPAL),
        bouton(
            Cle::Afficher(Vue::Transformation),
            "Changer en ressources",
            DANGER,
        ),
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

/// Les monstres dont Malvoise tirerait des ressources, avec ce qu'il en
/// tirerait ; la confirmation est demandée par le déroulement (section
/// 14.2).
pub fn transformation(partie: &Partie, contenu: &ContenuV1) -> Question {
    if !partie.marchand_au_camp() {
        return sans_marchand();
    }
    let collection = partie.collection();
    let especes = contenu.especes();
    let mut lignes = menus_de_monstres_decrits(
        collection,
        Cle::Transformer,
        "Changer",
        |emplacement| {
            collection.monstre(emplacement).is_some_and(|monstre| {
                !monstre.est_loue() && ressources_d_un_monstre(monstre, especes).is_some()
            })
        },
        |monstre| offre_de_transformation(monstre, especes),
    );
    lignes.truncate(RANGEES_MAX - 1);
    lignes.extend(rangees(vec![
        bouton(Cle::Afficher(Vue::Vente), "Retour à la vente", SECONDAIRE),
        retour(),
    ]));
    let texte = if lignes.len() > 1 {
        "Malvoise change un monstre en ressources de sa famille : 2, plus 1 tous les 10 niveaux. Le monstre ne revient pas.".to_string()
    } else {
        "Malvoise ne tirerait rien de vos monstres : leur famille ne lâche pas de ressource."
            .to_string()
    };
    Question {
        texte,
        rangees: lignes,
    }
}

/// Les espèces communes des zones explorées, en vente : plus de 75 depuis
/// l'étape 208, en menus de 25 sur toutes les rangées sauf celle des
/// boutons ; la location a son propre écran (section 14.2).
pub fn achats(partie: &Partie, contenu: &ContenuV1) -> Question {
    if !partie.marchand_au_camp() {
        return sans_marchand();
    }
    let mut lignes: Vec<Rangee> = Vec::new();
    let a_vendre = partie
        .offres_d_achat(contenu.zones(), contenu.especes())
        .iter()
        .map(|offre| option_de_monstre(offre, offre_d_achat(offre)))
        .collect();
    lignes.extend(menus(
        Cle::AcheterMonstre,
        "Acheter un monstre",
        a_vendre,
        RANGEES_MAX - 1,
    ));
    let mut boutons = Vec::new();
    if !partie
        .offres_de_location(contenu.zones(), contenu.piste())
        .is_empty()
    {
        boutons.push(bouton(
            Cle::Afficher(Vue::Locations),
            "Louer un monstre",
            PRINCIPAL,
        ));
    }
    boutons.extend(boutons_du_monstre_loue(partie));
    Question {
        texte: format!(
            "Malvoise vend des monstres communs des zones que vous avez explorées. Vous avez {}.{}",
            argent(partie.argent()),
            ligne_de_location(partie)
        ),
        rangees: lignes.into_iter().chain(rangees(boutons)).collect(),
    }
}

/// Les espèces rares des zones explorées, à louer : plus de 25 depuis
/// l'étape 208, en menus de 25 (section 14.2).
pub fn locations(partie: &Partie, contenu: &ContenuV1) -> Question {
    if !partie.marchand_au_camp() {
        return sans_marchand();
    }
    let mut lignes: Vec<Rangee> = Vec::new();
    let a_louer = partie
        .offres_de_location(contenu.zones(), contenu.piste())
        .iter()
        .map(|offre| option_de_monstre(offre, offre_de_location(offre)))
        .collect();
    lignes.extend(menus(
        Cle::LouerMonstre,
        "Louer un monstre",
        a_louer,
        RANGEES_MAX - 1,
    ));
    let mut boutons = vec![bouton(
        Cle::Afficher(Vue::Achats),
        "Acheter un monstre",
        PRINCIPAL,
    )];
    boutons.extend(boutons_du_monstre_loue(partie));
    Question {
        texte: format!(
            "Malvoise loue des monstres rares des zones que vous avez explorées pour {EXPEDITIONS_DE_LOCATION} expéditions : un monstre loué ne fait que combattre, puis repart. Vous avez {}.{}",
            argent(partie.argent()),
            ligne_de_location(partie)
        ),
        rangees: lignes.into_iter().chain(rangees(boutons)).collect(),
    }
}

/// Sans les types : ceux d'une espèce seulement rencontrée restent cachés
/// jusqu'à sa capture (section 5.6).
fn option_de_monstre(offre: &OffreDeMonstre, libelle: String) -> OptionDeMenu {
    OptionDeMenu {
        libelle,
        valeur: offre.espece.id().as_str().to_string(),
        description: None,
    }
}

/// Le rendu du monstre loué, s'il y en a un, puis les retours, communs aux
/// achats et aux locations.
fn boutons_du_monstre_loue(partie: &Partie) -> Vec<Bouton> {
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
    boutons
}

fn ligne_de_location(partie: &Partie) -> String {
    location_en_cours(partie)
        .map(|ligne| format!("\n{ligne}"))
        .unwrap_or_default()
}

/// Une vue restée d'avant le marchand, ou ouverte par un message dépassé.
fn sans_marchand() -> Question {
    Question {
        texte: "Malvoise ne s'est pas encore installé au camp.".to_string(),
        rangees: rangees(vec![retour()]),
    }
}
