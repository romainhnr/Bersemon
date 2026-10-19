//! Partie en cours : héros, camp, carte, codex et confirmations.

use bersemon_core::capsule::Capsule;
use bersemon_core::codex::{Codex, EtatCodex};
use bersemon_core::collection::Collection;
use bersemon_core::combat::Cote;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{Annonce, Confirmation};
use bersemon_core::espece::Espece;
use bersemon_core::expedition::Expedition;
use bersemon_core::heros::Classe;
use bersemon_core::inventaire::Inventaire;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::zone::Zone;

use crate::expeditions_automatiques::question_de_rappel;
use crate::monstres::{decrire_stats, ligne_monstre, nom, types_de};
use crate::soins::{liste_de_soins, question_du_soin_perdu};

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
    // Au camp, l'équipe peut être vide (section 13.2).
    if equipe.is_empty() {
        lignes.push("  aucun monstre".to_string());
    }
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

/// "22 pièces d'argent, dont 7 grâce à l'habileté Pillard" (section 5.9).
pub fn argent_et_part_du_pillard(montant: u32, part_du_pillard: u32) -> String {
    if part_du_pillard == 0 {
        argent(montant)
    } else {
        format!(
            "{}, dont {part_du_pillard} grâce à l'habileté Pillard",
            argent(montant)
        )
    }
}

/// « la capsule d'attaque Brasier ».
pub fn capsule(capsule: &Capsule) -> String {
    format!("la capsule d'attaque {}", capsule.attaque().nom())
}

/// « 2 potions, 30 pièces d'argent et la capsule d'attaque Brasier », ou
/// « vide ».
pub fn contenu(inventaire: &Inventaire) -> String {
    let mut elements = liste_de_soins(inventaire.soins());
    if inventaire.argent() > 0 {
        elements.push(argent(inventaire.argent()));
    }
    elements.extend(inventaire.capsules().iter().map(capsule));
    if elements.is_empty() {
        "vide".to_string()
    } else {
        enumerer(&elements)
    }
}

/// L'écran du coffre du camp (section 13.3).
pub fn coffre_du_camp(partie: &Partie) -> String {
    format!(
        "Ce qui est rangé dans le coffre du camp est à l'abri. Le sac part avec vous en expédition : une défaite le fait perdre.\nSac : {}.\nCoffre du camp : {}.{}",
        contenu(partie.sac()),
        contenu(partie.coffre_du_camp()),
        ligne_des_ames(partie)
    )
}

/// Le sac en expédition : une défaite le ferait perdre (section 9.3).
pub fn sac_en_expedition(partie: &Partie) -> String {
    let sac = partie.sac();
    let sac = if sac.est_vide() {
        "Sac : vide.".to_string()
    } else {
        format!("Sac : {}. Une défaite le ferait perdre.", contenu(sac))
    };
    format!("{sac}{}", ligne_des_ames(partie))
}

/// Les âmes portées, à part du sac : rien ne les fait perdre (section
/// 14.2). Une ligne qui commence par un saut de ligne, ou rien.
pub fn ligne_des_ames(partie: &Partie) -> String {
    let noms: Vec<String> = partie
        .ames_portees()
        .iter()
        .map(|marchand| marchand.nom().to_string())
        .collect();
    if noms.is_empty() {
        String::new()
    } else {
        format!(
            "\nÂmes portées, que rien ne vous fait perdre : {}.",
            enumerer(&noms)
        )
    }
}

/// En haut de chaque écran d'expédition (section 9.2).
pub fn compteur_d_etapes(expedition: &Expedition) -> String {
    format!(
        "[Étape : {}/{}]",
        expedition.etapes_faites(),
        expedition.nombre_etapes()
    )
}

/// Le texte et le bouton d'une annonce (section 9.2).
pub fn annonce(annonce: Annonce) -> (&'static str, &'static str) {
    match annonce {
        Annonce::Combat => ("Le combat vous attend.", "Combattre"),
        Annonce::Indice => ("Vous avez trouvé un indice.", "Lire l'indice"),
    }
}

/// Les places de l'équipe et du camp, rappelées quand le joueur peut
/// capturer. Le camp compte les places des monstres partis (section 12.1).
pub fn places_pour_une_capture(collection: &Collection) -> String {
    format!(
        "Place pour vos captures : équipe {}/{}, camp {}/{}.",
        collection.equipe().len(),
        Collection::TAILLE_EQUIPE,
        collection.camp().len() + collection.places_reservees(),
        Collection::TAILLE_CAMP
    )
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
        Classe::Assassin => "de Vitesse",
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

/// Le sac, que le joueur emporte, et le coffre, à l'abri (section 13.3).
pub fn en_tete_du_camp(partie: &Partie) -> String {
    format!(
        "Camp de base ({}, {}).\nSac : {}. Coffre du camp : {}.{}",
        partie.heros().nom().as_str(),
        partie.heros().classe().nom(),
        contenu(partie.sac()),
        contenu(partie.coffre_du_camp()),
        ligne_des_ames(partie)
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

/// Une espèce inconnue reste masquée ; ses types n'apparaissent qu'à sa
/// capture (section 5.6).
pub fn ligne_du_codex(codex: &Codex, espece: &Espece) -> String {
    match codex.etat(espece.id()) {
        EtatCodex::Inconnue => "???".to_string(),
        EtatCodex::Rencontree => format!("{} : rencontrée", espece.nom()),
        EtatCodex::Capturee => format!("{} ({}) : capturée", espece.nom(), types_de(espece)),
    }
}

/// Les types d'une espèce capturée, « ? » sinon (section 10.6).
pub fn types_connus(codex: &Codex, espece: &Espece) -> String {
    if codex.etat(espece.id()) == EtatCodex::Capturee {
        types_de(espece)
    } else {
        "?".to_string()
    }
}

/// Types, habiletés possibles, stats de base et attaques, montrés pour une
/// espèce capturée (sections 5.6 et 5.9).
pub fn fiche_d_espece(espece: &Espece) -> [String; 4] {
    let base = espece.stats_de_base();
    let total = u32::from(base.pv)
        + u32::from(base.attaque)
        + u32::from(base.defense)
        + u32::from(base.attaque_speciale)
        + u32::from(base.defense_speciale)
        + u32::from(base.vitesse);
    let stats = format!("Stats de base : {} (total {total})", decrire_stats(base));
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
    let habiletes: Vec<String> = espece
        .habiletes()
        .iter()
        .map(|habilete| habilete.nom().to_string())
        .collect();
    let habiletes = if habiletes.is_empty() {
        "aucune".to_string()
    } else {
        enumerer(&habiletes)
    };
    [
        format!("Types : {}", types_de(espece)),
        format!("Habiletés possibles : {habiletes}"),
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
        Confirmation::SoinPerdu { soin, monstre } => {
            question_du_soin_perdu(*soin, &nom(monstre, Cote::Joueur))
        }
        Confirmation::Relacher(monstre) => {
            format!("Relâcher définitivement {} ?", monstre.nom())
        }
        Confirmation::RappelerLExpedition(expedition) => question_de_rappel(expedition),
        Confirmation::Vendre { monstre, prix } => format!(
            "Vendre définitivement {} à Malvoise pour {} ?",
            monstre.nom(),
            argent(*prix)
        ),
        Confirmation::Suicide(heros) => format!(
            "Vous suicider ? Votre partie sera effacée pour de bon : {}, vos monstres, votre sac, votre coffre, votre codex et vos expéditions automatiques en cours.",
            heros.nom().as_str()
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
