//! Partie en cours : héros, camp, carte, codex et confirmations.

use bersemon_core::capsule::Capsule;
use bersemon_core::codex::{Codex, EtatCodex};
use bersemon_core::collection::Collection;
use bersemon_core::combat::Cote;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Confirmation;
use bersemon_core::espece::Espece;
use bersemon_core::heros::Classe;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::zone::Zone;

use crate::expeditions_automatiques::question_de_rappel;
use crate::monstres::{ligne_monstre, nom, types_de};

/// Équipe puis camp, numérotés à la suite : l'équipe de 1 à 3, le camp
/// ensuite. Le joueur désigne un monstre par ce numéro.
pub fn collection(collection: &Collection) -> String {
    let equipe = collection.equipe();
    let camp = collection.camp();
    let mut lignes = vec![format!(
        "Équipe ({}/{}) :",
        equipe.len(),
        Collection::TAILLE_EQUIPE
    )];
    for (index, monstre) in equipe.iter().enumerate() {
        lignes.push(format!(
            "  {}. {}",
            index + 1,
            ligne_monstre(monstre, Cote::Joueur)
        ));
    }
    lignes.push(format!(
        "Camp ({}/{}) :",
        camp.len(),
        Collection::TAILLE_CAMP
    ));
    if camp.is_empty() {
        lignes.push("  aucun monstre".to_string());
    }
    for (index, monstre) in camp.iter().enumerate() {
        lignes.push(format!(
            "  {}. {}",
            equipe.len() + index + 1,
            ligne_monstre(monstre, Cote::Joueur)
        ));
    }
    // Les monstres partis gardent leur place au camp (section 12.1).
    match collection.places_reservees() {
        0 => {}
        1 => lignes
            .push("En expédition automatique : 1 monstre, qui garde sa place au camp.".to_string()),
        absents => lignes.push(format!(
            "En expédition automatique : {absents} monstres, qui gardent leur place au camp."
        )),
    }
    lignes.join("\n")
}

pub fn potions(nombre: u32) -> String {
    if nombre > 1 {
        format!("{nombre} potions")
    } else {
        format!("{nombre} potion")
    }
}

/// L'argent se compte en pièces d'argent (section 14.1).
pub fn argent(montant: u32) -> String {
    if montant > 1 {
        format!("{montant} pièces d'argent")
    } else {
        format!("{montant} pièce d'argent")
    }
}

/// « la capsule d'attaque Brasier ».
pub fn capsule(capsule: &Capsule) -> String {
    format!("la capsule d'attaque {}", capsule.attaque().nom())
}

/// "a", "a et b", "a, b et c".
pub fn enumerer(elements: &[String]) -> String {
    match elements.split_last() {
        None => String::new(),
        Some((dernier, [])) => dernier.clone(),
        Some((dernier, autres)) => format!("{} et {dernier}", autres.join(", ")),
    }
}

/// Le bonus de la classe et son monstre de départ, que le joueur connaît
/// avant de choisir (section 6).
pub fn decrire_classe(classe: Classe, contenu: &ContenuV1) -> String {
    let bonus = match classe {
        Classe::Guerrier => "d'Attaque",
        Classe::Mage => "d'Attaque spéciale",
        Classe::Templier => "de Défense et de Défense spéciale",
    };
    format!(
        "{} : +10 % {bonus} pour vos monstres en combat ; premier monstre : {}",
        classe.nom(),
        contenu.espece_de_depart(classe).nom()
    )
}

pub fn partie_en_cours(partie: &Partie) -> String {
    format!(
        "Partie en cours : {}, {}.",
        partie.heros().nom().as_str(),
        partie.heros().classe().nom()
    )
}

pub fn en_tete_du_camp(partie: &Partie) -> String {
    format!(
        "Camp de base ({}, {}). {} et {}.",
        partie.heros().nom().as_str(),
        partie.heros().classe().nom(),
        potions(partie.inventaire().potions()),
        argent(partie.argent())
    )
}

/// Les niveaux d'une zone disent sa difficulté (section 9.1).
pub fn decrire_zone(partie: &Partie, zone: &Zone) -> String {
    let (minimum, maximum) = zone.niveaux();
    let exploree = if partie.est_exploree(zone.id()) {
        ", explorée"
    } else {
        ""
    };
    format!(
        "{} (niveaux {} à {}){exploree}",
        zone.nom(),
        minimum.valeur(),
        maximum.valeur()
    )
}

/// Une espèce inconnue reste masquée (section 5.6).
pub fn ligne_du_codex(codex: &Codex, espece: &Espece) -> String {
    match codex.etat(espece.id()) {
        EtatCodex::Inconnue => "???".to_string(),
        EtatCodex::Rencontree => format!("{} ({}) : rencontrée", espece.nom(), types_de(espece)),
        EtatCodex::Capturee => format!("{} ({}) : capturée", espece.nom(), types_de(espece)),
    }
}

/// Types, stats de base et attaques, montrés pour une espèce capturée
/// (section 5.6).
pub fn fiche_d_espece(espece: &Espece) -> [String; 3] {
    let base = espece.stats_de_base();
    let stats = format!(
        "Stats de base : PV {}, Attaque {}, Défense {}, Attaque spéciale {}, Défense spéciale {}, Vitesse {}",
        base.pv,
        base.attaque,
        base.defense,
        base.attaque_speciale,
        base.defense_speciale,
        base.vitesse
    );
    let attaques: Vec<String> = espece
        .apprentissage()
        .iter()
        .map(|apprise| {
            format!(
                "{} (niv. {})",
                apprise.attaque.nom(),
                apprise.niveau.valeur()
            )
        })
        .collect();
    [
        format!("Types : {}", types_de(espece)),
        stats,
        format!("Attaques : {}", attaques.join(", ")),
    ]
}

pub fn question_de_confirmation(confirmation: &Confirmation) -> String {
    match confirmation {
        Confirmation::RemplacerLaPartie(heros) => format!(
            "La partie en cours ({}) sera remplacée. Continuer ?",
            heros.nom().as_str()
        ),
        Confirmation::PotionPerdue(monstre) => format!(
            "{} a déjà tous ses PV : la potion serait perdue. L'utiliser quand même ?",
            nom(monstre, Cote::Joueur)
        ),
        Confirmation::Relacher(monstre) => {
            format!("Relâcher définitivement {} ?", monstre.nom())
        }
        Confirmation::RappelerLExpedition(expedition) => question_de_rappel(expedition),
        Confirmation::Vendre { monstre, prix } => format!(
            "Vendre définitivement {} au marchand pour {} ?",
            monstre.nom(),
            argent(*prix)
        ),
    }
}

/// L'équipe et le camp sont pleins (Q12).
pub fn place_manquante(capture: &Monstre) -> String {
    format!(
        "Votre équipe et le camp sont pleins. Quel monstre relâcher pour garder {} ?",
        capture.nom()
    )
}
