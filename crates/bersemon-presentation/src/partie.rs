//! Partie en cours : héros, camp, carte, codex et confirmations.

use bersemon_core::beherit::EtatDuBeherit;
use bersemon_core::capsule::Capsule;
use bersemon_core::catalogue::Catalogue;
use bersemon_core::codex::{Codex, EtatCodex};
use bersemon_core::collection::Collection;
use bersemon_core::combat::Cote;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{Annonce, Confirmation};
use bersemon_core::espece::{Espece, EspeceId};
use bersemon_core::evolution::ConditionEvolution;
use bersemon_core::expedition::Expedition;
use bersemon_core::habilete::Habilete;
use bersemon_core::heros::Classe;
use bersemon_core::inventaire::Inventaire;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::pierre::PierreDEvolution;
use bersemon_core::recompenses_du_codex::compte_au_codex;
use bersemon_core::ressource::Ressource;
use bersemon_core::traque::Apotre;
use bersemon_core::zone::Zone;

use crate::boosters::{boosters, philtres_chromatiques, question_shiny};
use crate::expeditions_automatiques::question_de_rappel;
use crate::filets::liste_de_filets;
use crate::forge::{
    question_de_fabrication, question_de_lancement, question_des_filets, question_du_charme,
};
use crate::jour_et_nuit;
use crate::meteo;
use crate::monstres::{
    decrire_attaque, decrire_stats, effet_d_habilete, ligne_monstre, nom, types_de,
};
use crate::soins::{liste_de_soins, question_du_soin_perdu};

/// Équipe puis camp, numérotés à la suite : l'équipe de 1 à 3, le camp
/// ensuite. Le joueur désigne un monstre par ce numéro.
pub fn collection(collection: &Collection) -> String {
    let equipe = collection.equipe();
    let camp = collection.camp();
    let mut lignes = vec![format!(
        "Équipe ({}/{}) :",
        equipe.len(),
        collection.tailles().equipe
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
        collection.tailles().camp
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
    // Les monstres partis en expédition automatique, ceux de la cour et du
    // rite gardent leur place au camp, comme le petit à naître (sections
    // 12.1, 13.6 et 13.11).
    match collection.places_reservees() {
        0 => {}
        1 => lignes
            .push("1 place du camp est gardée, pour un monstre absent ou à naître.".to_string()),
        gardees => lignes.push(format!(
            "{gardees} places du camp sont gardées, pour des monstres absents ou à naître."
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

/// "1 charme de garde", "2 charmes de garde" (section 12.5).
pub fn charmes_de_garde(nombre: u32) -> String {
    if nombre > 1 {
        format!("{nombre} charmes de garde")
    } else {
        format!("{nombre} charme de garde")
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

/// "Croc de loup" ou "Croc de loup (3)" : le nombre suit le nom, qui ne se
/// met pas facilement au pluriel (section 14.3).
pub fn ressources(ressource: &Ressource, nombre: u32) -> String {
    if nombre > 1 {
        format!("{} ({nombre})", ressource.nom())
    } else {
        ressource.nom().to_string()
    }
}

/// Elles se consomment, d'où leur nombre (section 5.9).
pub fn capsules_d_habilete(nombre: u32) -> String {
    if nombre > 1 {
        format!("{nombre} capsules d'habileté")
    } else {
        format!("{nombre} capsule d'habileté")
    }
}

/// « 1 Pierre de braise », « 2 Pierres de lune » (section 5.4).
pub fn pierres(pierre: PierreDEvolution, nombre: u32) -> String {
    let nom = pierre.nom();
    if nombre > 1 {
        format!("{nombre} {}", nom.replacen("Pierre", "Pierres", 1))
    } else {
        format!("{nombre} {nom}")
    }
}

/// « 2 potions, 30 pièces d'argent et la capsule d'attaque Brasier », ou
/// « vide ».
pub fn contenu(inventaire: &Inventaire) -> String {
    contenu_detaille(inventaire, true)
}

/// Les ressources se comptent par sortes quand `ressources_detaillees` est
/// faux : "17 sortes de ressources".
fn contenu_detaille(inventaire: &Inventaire, ressources_detaillees: bool) -> String {
    let mut elements = liste_de_soins(inventaire.soins());
    elements.extend(liste_de_filets(inventaire.filets()));
    if inventaire.argent() > 0 {
        elements.push(argent(inventaire.argent()));
    }
    // Regroupées, pour que le camp tienne dans un message Discord quand le
    // joueur les a toutes (étape 82).
    match inventaire.capsules() {
        [] => {}
        [seule] => elements.push(capsule(seule)),
        plusieurs => {
            let noms: Vec<&str> = plusieurs
                .iter()
                .map(|capsule| capsule.attaque().nom())
                .collect();
            elements.push(format!("les capsules d'attaque ({})", noms.join(", ")));
        }
    }
    if inventaire.capsules_d_habilete() > 0 {
        elements.push(capsules_d_habilete(inventaire.capsules_d_habilete()));
    }
    let sortes = inventaire.ressources().len();
    if ressources_detaillees {
        elements.extend(
            inventaire
                .ressources()
                .iter()
                .map(|(ressource, nombre)| ressources(ressource, *nombre)),
        );
    } else if sortes > 1 {
        elements.push(format!("{sortes} sortes de ressources"));
    } else if sortes == 1 {
        elements.push("1 sorte de ressource".to_string());
    }
    elements.extend(
        inventaire
            .pierres()
            .iter()
            .map(|(pierre, nombre)| pierres(*pierre, *nombre)),
    );
    elements.extend(
        inventaire
            .boosters()
            .iter()
            .map(|(booster, nombre)| boosters(*booster, *nombre)),
    );
    if inventaire.philtres_chromatiques() > 0 {
        elements.push(philtres_chromatiques(inventaire.philtres_chromatiques()));
    }
    if inventaire.charmes_de_garde() > 0 {
        elements.push(charmes_de_garde(inventaire.charmes_de_garde()));
    }
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

/// Pour Discord, quand l'écran du coffre ne tient plus dans un message : les
/// ressources s'y comptent par sortes, détaillées à l'écran des ressources
/// (section 13.3).
pub fn coffre_du_camp_abrege(partie: &Partie) -> String {
    format!(
        "Ce qui est rangé dans le coffre du camp est à l'abri. Le sac part avec vous en expédition : une défaite le fait perdre.\nSac : {}.\nCoffre du camp : {}.{}",
        contenu_detaille(partie.sac(), false),
        contenu_detaille(partie.coffre_du_camp(), false),
        ligne_des_ames(partie)
    )
}

/// Pour Discord, l'écran des ressources quand le coffre détaillé ne tient
/// plus dans un message : les ressources seules, du sac puis du coffre.
pub fn ressources_du_sac_et_du_coffre(partie: &Partie) -> String {
    let liste = |inventaire: &Inventaire| {
        let elements: Vec<String> = inventaire
            .ressources()
            .iter()
            .map(|(ressource, nombre)| ressources(ressource, *nombre))
            .collect();
        if elements.is_empty() {
            "aucune".to_string()
        } else {
            enumerer(&elements)
        }
    };
    format!(
        "Ressources du sac : {}.\nRessources du coffre du camp : {}.",
        liste(partie.sac()),
        liste(partie.coffre_du_camp())
    )
}

/// Le sac en expédition : une défaite le ferait perdre (section 9.3).
pub fn sac_en_expedition(partie: &Partie) -> String {
    let sac = partie.sac();
    let sac = if sac.est_vide() {
        "Sac : vide.".to_string()
    } else {
        format!(
            "Sac ({}) : {}. Une défaite le ferait perdre.",
            crate::sac::places(partie),
            contenu(sac)
        )
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

/// En haut de chaque écran d'expédition (section 9.2), sans total dans la
/// Tour sans fond (section 9.5), suivi du moment et de la météo de
/// l'expédition dans une zone de la carte (sections 9.6 et 9.7) :
/// "[Étape : 2/5] Nuit, pluie".
pub fn compteur_d_etapes(expedition: &Expedition) -> String {
    let compteur = if expedition.est_sans_fin() {
        format!("[Étape : {}]", expedition.etapes_faites())
    } else {
        format!(
            "[Étape : {}/{}]",
            expedition.etapes_faites(),
            expedition.nombre_etapes()
        )
    };
    let compteur = match expedition.jour_ou_nuit() {
        Some(jour_ou_nuit) => format!("{compteur} {}", jour_et_nuit::nom(jour_ou_nuit)),
        None => compteur,
    };
    match expedition.meteo() {
        Some(meteo) => format!("{compteur}, {}", meteo::nom(meteo)),
        None => compteur,
    }
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
        collection.tailles().equipe,
        collection.camp().len() + collection.places_reservees(),
        collection.tailles().camp
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
        "Camp de base ({}, {}).\nSac ({}) : {}. Coffre du camp : {}.{}",
        partie.heros().nom().as_str(),
        partie.heros().classe().nom(),
        crate::sac::places(partie),
        contenu(partie.sac()),
        contenu(partie.coffre_du_camp()),
        ligne_des_ames(partie)
    )
}

/// Pour Discord : les ressources s'y comptent par sortes, pour que le camp
/// tienne dans un message quand le joueur a de tout (section 13.3). Leur
/// détail est à l'écran du coffre.
pub fn en_tete_abrege_du_camp(partie: &Partie) -> String {
    format!(
        "Camp de base ({}, {}).\nSac ({}) : {}. Coffre du camp : {}.{}",
        partie.heros().nom().as_str(),
        partie.heros().classe().nom(),
        crate::sac::places(partie),
        contenu_detaille(partie.sac(), false),
        contenu_detaille(partie.coffre_du_camp(), false),
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

/// Les espèces de la liste du codex, avec leur position dans le catalogue.
/// Les Apôtres et le Démiurge n'y sont pas : ils n'apparaissent qu'au
/// carnet de traque, une fois abattus, pour ne rien dévoiler de ceux qui
/// restent ni de leur nombre (section 5.6).
pub fn especes_du_codex(especes: &[Espece]) -> impl Iterator<Item = (usize, &Espece)> {
    especes
        .iter()
        .enumerate()
        .filter(|(_, espece)| compte_au_codex(espece))
}

pub const TITRE_DU_CARNET: &str = "Carnet de traque :";

/// "1. Baron pendu, abattu dans la Vallée des Brumes."
pub fn entree_du_carnet(numero: usize, apotre: &Apotre) -> String {
    format!("{numero}. {}", apotre.ligne_du_carnet())
}

/// Le carnet de traque, une entrée par victoire, comme le héros les écrit ;
/// vide avant le premier Apôtre abattu (section 5.6).
pub fn carnet_de_traque(partie: &Partie, contenu: &ContenuV1) -> Vec<String> {
    let carnet = partie.carnet_de_traque(contenu.piste(), contenu.invocation());
    if carnet.is_empty() {
        return Vec::new();
    }
    let mut lignes = vec![TITRE_DU_CARNET.to_string()];
    lignes.extend(
        carnet
            .iter()
            .enumerate()
            .map(|(position, apotre)| format!("  {}", entree_du_carnet(position + 1, apotre))),
    );
    lignes
}

/// Les âmes des Apôtres vaincus, au camp : rien ne les fait perdre (section
/// 9.4). `None` avant la première victoire sur un Apôtre.
/// Une fois forgé, le Béhérit pourpre remplace leurs âmes (section 9.4).
pub fn ames_d_apotres(partie: &Partie, catalogue: &dyn Catalogue) -> Option<String> {
    match partie.beherit() {
        EtatDuBeherit::Aucun => {}
        EtatDuBeherit::Forge => {
            return Some("Béhérit pourpre : il attend son sacrifice.".to_string());
        }
        EtatDuBeherit::EclipseOuverte => {
            return Some("Béhérit pourpre : il vous ouvre l'Éclipse.".to_string());
        }
        EtatDuBeherit::DemiurgeVaincu => {
            return Some("Béhérit pourpre : l'Éclipse s'est refermée.".to_string());
        }
    }
    let noms: Vec<String> = partie
        .ames_d_apotres()
        .iter()
        .filter_map(|id| catalogue.espece(id))
        .map(|espece| espece.nom().to_string())
        .collect();
    (!noms.is_empty()).then(|| format!("Âmes d'Apôtres : {}.", enumerer(&noms)))
}

/// Les types d'une espèce capturée, « ? » sinon (section 10.6).
pub fn types_connus(codex: &Codex, espece: &Espece) -> String {
    if codex.etat(espece.id()) == EtatCodex::Capturee {
        types_de(espece)
    } else {
        "?".to_string()
    }
}

/// Types, habiletés possibles, stats de base, attaques et évolution, montrés
/// pour une espèce capturée ou un Apôtre abattu (sections 5.4, 5.6 et 5.9).
/// Elle ne dévoile ni le nom d'une évolution pas encore capturée, ni la
/// condition qui nomme un Apôtre pas encore abattu (Q103).
pub fn fiche_d_espece(
    espece: &Espece,
    codex: &Codex,
    apotres_vaincus: &[EspeceId],
    contenu: &ContenuV1,
) -> Vec<String> {
    let base = espece.stats_de_base();
    let total = u32::from(base.pv)
        + u32::from(base.attaque)
        + u32::from(base.defense)
        + u32::from(base.attaque_speciale)
        + u32::from(base.defense_speciale)
        + u32::from(base.vitesse);
    let stats = format!("Stats de base : {} (total {total})", decrire_stats(base));
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
    let mut fiche = vec![
        format!("Types : {}", types_de(espece)),
        format!("Habiletés possibles : {habiletes}"),
        stats,
        "Attaques apprises :".to_string(),
    ];
    // L'attaque d'évolution est dans la liste de l'espèce d'arrivée : on la
    // retrouve par l'espèce qui évolue vers celle-ci.
    let attaque_d_evolution = contenu.especes().iter().find_map(|avant| {
        avant
            .evolution()
            .filter(|evolution| evolution.vers() == espece.id())
            .map(|evolution| evolution.attaque())
    });
    for apprise in espece.apprentissage() {
        let evolution = if attaque_d_evolution == Some(apprise.attaque.id()) {
            " (évolution)"
        } else {
            ""
        };
        fiche.push(format!(
            "  niv. {}{evolution} : {}",
            apprise.niveau.valeur(),
            decrire_attaque(&apprise.attaque)
        ));
    }
    let evolution = espece.evolution().and_then(|evolution| {
        let vers = contenu.espece(evolution.vers())?;
        Some((vers, evolution.condition()))
    });
    if let Some((vers, condition)) = evolution {
        let nom = if codex.etat(vers.id()) == EtatCodex::Capturee {
            vers.nom()
        } else {
            "???"
        };
        let cachee = matches!(
            condition,
            ConditionEvolution::ApotreVaincu { apotre, .. } if !apotres_vaincus.contains(apotre)
        );
        if cachee {
            fiche.push(format!("Évolution : {nom}, condition inconnue"));
        } else {
            fiche.push(format!(
                "Évolution : {nom} {}",
                condition_d_evolution(condition, contenu)
            ));
        }
    }
    // Celles que l'espèce apprend déjà d'elle-même ne s'y répètent pas. Le
    // marchand les vend toutes : les nommer ne dévoile rien (section 13.10).
    let capsules: Vec<String> = contenu
        .capsules()
        .iter()
        .filter(|capsule| capsule.convient_a(espece))
        .filter(|capsule| espece.attaque_apprise(capsule.id()).is_none())
        .map(|capsule| format!("  {}", decrire_attaque(capsule.attaque())))
        .collect();
    if capsules.is_empty() {
        fiche.push("Capsules d'attaque possibles : aucune".to_string());
    } else {
        fiche.push("Capsules d'attaque possibles :".to_string());
        fiche.extend(capsules);
    }
    // La même pour toute la famille, que seul le Rite du sang mêlé
    // transmet (section 13.11).
    if let Some(attaque) = espece.attaque_d_oeuf() {
        fiche.push(format!(
            "Attaque d'œuf (Rite du sang mêlé) : {}",
            decrire_attaque(attaque)
        ));
    }
    if let Some(activite) = espece.activite() {
        fiche.push(jour_et_nuit::activite(activite).to_string());
    }
    fiche
}

/// Après le nom de l'espèce d'arrivée : "Lézard de braise au niveau 16".
pub fn condition_d_evolution(condition: &ConditionEvolution, catalogue: &dyn Catalogue) -> String {
    match condition {
        ConditionEvolution::Niveau(niveau) => format!("au niveau {}", niveau.valeur()),
        ConditionEvolution::Pierre(pierre) => {
            format!("avec une {}", pierre.nom().to_lowercase())
        }
        ConditionEvolution::Lieu { zone, niveau } => {
            let zone = catalogue
                .zone(zone)
                .map_or_else(|| zone.as_str().to_string(), |zone| zone.nom().to_string());
            format!(
                "dès le niveau {}, en gagnant un niveau dans la zone {zone}",
                niveau.valeur()
            )
        }
        ConditionEvolution::ApotreVaincu { apotre, niveau } => {
            let apotre = catalogue.espece(apotre).map_or_else(
                || apotre.as_str().to_string(),
                |apotre| apotre.nom().to_string(),
            );
            format!(
                "dès le niveau {}, une fois l'Apôtre {apotre} vaincu",
                niveau.valeur()
            )
        }
        ConditionEvolution::Moment {
            jour_ou_nuit,
            niveau,
        } => format!(
            "dès le niveau {}, après un combat {}",
            niveau.valeur(),
            jour_et_nuit::complement(*jour_ou_nuit)
        ),
        ConditionEvolution::Meteo { meteo, niveau } => format!(
            "dès le niveau {}, après un combat {}",
            niveau.valeur(),
            meteo::sous(*meteo)
        ),
    }
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
        Confirmation::ChangerDHabilete { monstre, nouvelle } => format!(
            "Utiliser une capsule d'habileté sur {} ? Son habileté {} sera remplacée par {} : {}.",
            monstre.nom(),
            monstre.habilete().map_or("actuelle", Habilete::nom),
            nouvelle.nom(),
            effet_d_habilete(*nouvelle)
        ),
        Confirmation::LancerAmelioration(plan) => question_de_lancement(plan),
        Confirmation::LancerFabrication(plan) => question_de_fabrication(plan),
        Confirmation::FabriquerFilets(plan) => question_des_filets(plan),
        Confirmation::LancerLeRite(apercu) => crate::rite::question_du_rite(apercu),
        Confirmation::Evoluer {
            monstre,
            vers,
            pierre,
        } => {
            let pierre = pierre
                .map(|pierre| format!(" Elle consomme une {}.", pierre.nom().to_lowercase()))
                .unwrap_or_default();
            format!(
                "Faire évoluer {} en {} ? L'évolution est définitive.{pierre}",
                monstre.nom(),
                vers.nom()
            )
        }
        Confirmation::RendreShiny(monstre) => question_shiny(monstre),
        Confirmation::ForgerLeBeherit => "Donner à Malvoise les âmes des trois Apôtres pour forger le Béhérit pourpre ? Elles seront perdues.".to_string(),
        Confirmation::Sacrifier(monstre) => format!(
            "Sacrifier {} au Béhérit pourpre ? Il sera perdu pour toujours.",
            monstre.nom()
        ),
        Confirmation::RendreLeMonstreLoue(monstre) => format!(
            "Rendre {} à Malvoise ? La location n'est pas remboursée.",
            monstre.nom()
        ),
        Confirmation::FabriquerCharme(plan) => question_du_charme(plan),
        Confirmation::Suicide(heros) => format!(
            "Vous suicider ? Votre partie sera effacée pour de bon : {}, vos monstres, votre sac, votre coffre, votre codex et vos expéditions automatiques en cours.",
            heros.nom().as_str()
        ),
    }
}

/// Après un combat ; le joueur peut la repousser (section 5.4).
pub fn proposition_d_evolution(monstre: &Monstre, vers: &Espece) -> String {
    format!(
        "{} peut évoluer en {}. Faire évoluer {} ?",
        monstre.nom(),
        vers.nom(),
        monstre.nom()
    )
}

/// L'équipe et le camp sont pleins (Q12).
pub fn place_manquante(capture: &Monstre) -> String {
    format!(
        "Votre équipe et le camp sont pleins. Quel monstre relâcher pour garder {} ?",
        capture.nom()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::espece::EspeceId;
    use bersemon_core::heros::Heros;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::stats::ValeursInternes;

    /// Le compteur d'étapes dit le moment de l'expédition, partie ici à 2 h
    /// UTC, de nuit, et sa météo s'il ne fait pas calme (sections 9.6 et
    /// 9.7).
    #[test]
    fn le_compteur_dit_le_moment_et_la_meteo_de_l_expedition() {
        use bersemon_core::aleatoire::GenerateurAleatoire;
        use bersemon_core::catalogue::Catalogue;
        use bersemon_core::deroulement::{Choix, Contexte, Deroulement};
        use bersemon_core::heros::Classe;
        use bersemon_core::horodatage::Horodatage;
        use bersemon_core::sauvegarde::{ErreurSauvegarde, SauvegardeRepository};
        use bersemon_core::zone::ZoneId;

        struct DepotMuet;
        impl SauvegardeRepository for DepotMuet {
            fn charger(&self, _: &dyn Catalogue) -> Result<Option<Partie>, ErreurSauvegarde> {
                Ok(None)
            }
            fn enregistrer(&self, _: &Partie) -> Result<(), ErreurSauvegarde> {
                Ok(())
            }
            fn supprimer(&self) -> Result<(), ErreurSauvegarde> {
                Ok(())
            }
        }

        let contenu = ContenuV1::new();
        let mut aleatoire = GenerateurAleatoire::new(1);
        let mut deroulement = Deroulement::demarrer(Ok(None));
        for choix in [
            Choix::NomDuHeros("Aldric".to_string()),
            Choix::Classe(Classe::Mage),
            Choix::PartirEnExpedition(ZoneId::new("lisiere")),
        ] {
            let mut contexte = Contexte {
                contenu: &contenu,
                aleatoire: &mut aleatoire,
                depot: &DepotMuet,
                maintenant: Horodatage::depuis_secondes(2 * 3_600),
            };
            // L'introduction et les conseils attendent d'être lus.
            while deroulement.repondre(choix.clone(), &mut contexte).is_err() {
                deroulement
                    .repondre(Choix::Poursuivre, &mut contexte)
                    .unwrap();
            }
        }
        let expedition = deroulement.expedition().unwrap();
        let attendu = match expedition.meteo() {
            Some(meteo) => format!("[Étape : 1/5] Nuit, {}", meteo::nom(meteo)),
            None => "[Étape : 1/5] Nuit".to_string(),
        };
        assert_eq!(compteur_d_etapes(expedition), attendu);
        assert_eq!(
            expedition.meteo(),
            bersemon_core::meteo::Meteo::de_la_zone(
                &contenu.zones()[0],
                Horodatage::depuis_secondes(2 * 3_600)
            )
        );
    }

    #[test]
    fn aucun_apotre_au_codex_avant_d_etre_abattu() {
        let contenu = ContenuV1::new();
        let salamandre = Monstre::creer(
            contenu.espece(&EspeceId::new("salamandre")).unwrap(),
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
        let partie = Partie::nouvelle(heros, salamandre);
        assert_eq!(ames_d_apotres(&partie, &contenu), None);
        assert!(carnet_de_traque(&partie, &contenu).is_empty());
        let liste: Vec<&str> = especes_du_codex(contenu.especes())
            .map(|(_, espece)| espece.id().as_str())
            .collect();
        for cache in [
            "baron-pendu",
            "abbesse-ecorchee",
            "demiurge",
            "demiurge-eveille",
        ] {
            assert!(!liste.contains(&cache), "{cache}");
        }
        assert!(liste.contains(&"salamandre"));
        assert_eq!(
            liste.len(),
            contenu.especes().len() - contenu.piste().apotres().count() - 2
        );
        let mut partie = partie;
        let baron = contenu.piste().apotres().next().unwrap();
        partie.noter_apotre_vaincu(baron);
        assert_eq!(
            carnet_de_traque(&partie, &contenu),
            vec![
                "Carnet de traque :",
                "  1. Baron pendu, abattu dans la Vallée des Brumes."
            ]
        );
    }

    #[test]
    fn l_en_tete_abrege_compte_les_ressources_par_sortes() {
        use bersemon_core::filet::Filet;
        use bersemon_core::ressource::{Ressource, RessourceId};
        let ressource = |id: &str, nom: &str| Ressource::new(RessourceId::new(id), nom, 5);
        let sac = Inventaire::default()
            .avec_filets(Filet::Leste, 2)
            .avec_ressources(ressource("os", "Os de squelette"), 3)
            .avec_ressources(ressource("croc", "Croc de loup"), 1);
        assert_eq!(
            contenu(&sac),
            "2 filets lestés, Os de squelette (3) et Croc de loup"
        );
        assert_eq!(
            contenu_detaille(&sac, false),
            "2 filets lestés et 2 sortes de ressources"
        );
        let une = Inventaire::default().avec_ressources(ressource("os", "Os de squelette"), 3);
        assert_eq!(contenu_detaille(&une, false), "1 sorte de ressource");
    }
}
