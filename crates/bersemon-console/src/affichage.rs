//! Mise en texte, pour la console, de l'état du jeu et des événements. Les
//! phrases évitent les accords en genre, que le nom d'un monstre ne permet pas
//! de connaître. Les numéros affichés au joueur commencent à 1, les positions
//! du noyau à 0.

use bersemon_core::attaque::{Attaque, Categorie};
use bersemon_core::codex::{Codex, EtatCodex};
use bersemon_core::collection::{Collection, ErreurCollection};
use bersemon_core::combat::{Combat, Combattant, Cote, ErreurCombat, Evenement, Issue};
use bersemon_core::espece::Espece;
use bersemon_core::inventaire::ErreurPotion;
use bersemon_core::monstre::{Monstre, RapportExperience};
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::ErreurSauvegarde;
use bersemon_core::statut::Statut;
use bersemon_core::types::Efficacite;
use bersemon_core::zone::Zone;

pub fn erreur(erreur: &ErreurCombat) -> String {
    match erreur {
        ErreurCombat::AttaqueInexistante(index) => {
            format!("Il n'y a pas d'attaque numéro {}.", index + 1)
        }
        ErreurCombat::MonstreInexistant(index) => {
            format!("Il n'y a pas de monstre numéro {}.", index + 1)
        }
        ErreurCombat::MonstreKo(index) => format!("Le monstre numéro {} est KO.", index + 1),
        ErreurCombat::MonstreDejaActif(index) => {
            format!("Le monstre numéro {} combat déjà.", index + 1)
        }
        ErreurCombat::AucunePotion => "Vous n'avez plus de potion.".to_string(),
        ErreurCombat::PvDejaPleins(index) => {
            format!("Le monstre numéro {} a déjà tous ses PV.", index + 1)
        }
        // Les autres erreurs ne viennent pas d'une saisie du joueur : la
        // console suit la phase du combat.
        autre => format!("Action impossible : {autre}."),
    }
}

pub fn erreur_collection(erreur: &ErreurCollection) -> String {
    match erreur {
        ErreurCollection::EmplacementInexistant(_) => "Ce monstre n'existe pas.".to_string(),
        ErreurCollection::EquipePleine => format!(
            "L'équipe compte déjà {} monstres.",
            Collection::TAILLE_EQUIPE
        ),
        ErreurCollection::CampPlein => {
            format!("Le camp est plein ({} monstres).", Collection::TAILLE_CAMP)
        }
        ErreurCollection::DernierMonstreDeLEquipe => {
            "L'équipe doit garder au moins un monstre.".to_string()
        }
        autre => format!("Action impossible : {autre}."),
    }
}

/// Nom du monstre, suivi de "sauvage" pour un adversaire.
pub fn nom(monstre: &Monstre, cote: Cote) -> String {
    let mut nom = monstre.nom().to_string();
    if cote == Cote::Adversaire {
        nom.push_str(" sauvage");
    }
    if monstre.est_shiny() {
        nom.push_str(" (shiny)");
    }
    nom
}

pub fn liste_especes(especes: &[Espece]) -> String {
    especes
        .iter()
        .enumerate()
        .map(|(position, espece)| {
            let types: Vec<&str> = espece
                .types()
                .liste()
                .iter()
                .map(|type_monstre| type_monstre.nom())
                .collect();
            format!(
                "  {}. {} ({})",
                position + 1,
                espece.nom(),
                types.join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Équipe puis camp, numérotés à la suite : l'équipe de 1 à 3, le camp
/// ensuite.
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
    lignes.join("\n")
}

pub fn rencontre(combat: &Combat) -> String {
    let noms: Vec<String> = combat
        .adversaires()
        .iter()
        .map(|monstre| nom(monstre, Cote::Adversaire))
        .collect();
    format!("Des monstres surgissent : {}.", noms.join(", "))
}

pub fn etat(combat: &Combat) -> String {
    let actif_joueur = combat.actif(Cote::Joueur);
    let mut lignes = vec!["Votre équipe :".to_string()];
    for (index, monstre) in combat.equipe().iter().enumerate() {
        let repere = if index == actif_joueur.index {
            ">"
        } else {
            " "
        };
        lignes.push(format!(
            "  {repere} {}",
            ligne_monstre(monstre, Cote::Joueur)
        ));
    }
    let adversaire = combat.actif(Cote::Adversaire);
    lignes.push(format!(
        "En face : {}",
        ligne_monstre(combat.monstre(adversaire), Cote::Adversaire)
    ));
    let en_attente = combat
        .adversaires()
        .iter()
        .enumerate()
        .filter(|&(index, monstre)| index > adversaire.index && !monstre.est_ko())
        .count();
    if en_attente > 0 {
        lignes.push(format!("  et {en_attente} autre(s) monstre(s) en attente"));
    }
    lignes.join("\n")
}

pub fn menu_action(combat: &Combat) -> String {
    format!(
        "Que faire ?\n  1. Attaquer\n  2. Changer de monstre\n  3. Lancer un filet ({} % de chances)\n  4. Utiliser une potion ({})\n  5. Fuir",
        combat.chance_de_capture(),
        combat.potions()
    )
}

pub fn liste_attaques(monstre: &Monstre) -> String {
    let mut lignes: Vec<String> = monstre
        .attaques()
        .liste()
        .iter()
        .enumerate()
        .map(|(position, attaque)| format!("  {}. {}", position + 1, decrire_attaque(attaque)))
        .collect();
    lignes.push("  0. Retour".to_string());
    lignes.join("\n")
}

pub fn liste_equipe(combat: &Combat, avec_retour: bool) -> String {
    let actif = combat.actif(Cote::Joueur).index;
    let mut lignes: Vec<String> = combat
        .equipe()
        .iter()
        .enumerate()
        .map(|(index, monstre)| {
            let au_combat = if index == actif && !monstre.est_ko() {
                " (au combat)"
            } else {
                ""
            };
            format!(
                "  {}. {}{au_combat}",
                index + 1,
                ligne_monstre(monstre, Cote::Joueur)
            )
        })
        .collect();
    if avec_retour {
        lignes.push("  0. Retour".to_string());
    }
    lignes.join("\n")
}

pub fn liste_capturables(combat: &Combat, capturables: &[usize]) -> String {
    let mut lignes: Vec<String> = capturables
        .iter()
        .enumerate()
        .map(|(position, &index)| {
            format!(
                "  {}. {}",
                position + 1,
                nom(&combat.adversaires()[index], Cote::Adversaire)
            )
        })
        .collect();
    lignes.push("  0. Aucun".to_string());
    lignes.join("\n")
}

pub fn evenement(evenement: &Evenement, combat: &Combat) -> String {
    let nom_de = |combattant: Combattant| nom(combat.monstre(combattant), combattant.cote);
    match evenement {
        Evenement::MonstreEnvoye(combattant) => match combattant.cote {
            Cote::Joueur => format!("Vous envoyez {}.", nom_de(*combattant)),
            Cote::Adversaire => format!("{} entre en combat.", nom_de(*combattant)),
        },
        Evenement::AttaqueLancee { lanceur, attaque } => {
            format!("{} utilise {}.", nom_de(*lanceur), attaque.nom())
        }
        Evenement::AttaqueRatee(lanceur) => format!("L'attaque de {} rate.", nom_de(*lanceur)),
        Evenement::SansEffet(cible) => format!("Cela n'affecte pas {}.", nom_de(*cible)),
        Evenement::Degats {
            cible,
            degats,
            critique,
            efficacite,
        } => {
            let critique = if *critique { "Coup critique ! " } else { "" };
            let efficacite = phrase_efficacite(*efficacite)
                .map(|phrase| format!("{phrase} "))
                .unwrap_or_default();
            format!("{critique}{efficacite}{} perd {degats} PV.", nom_de(*cible))
        }
        Evenement::StatutInflige { cible, statut } => match statut {
            Statut::Poison => format!("Le poison atteint {}.", nom_de(*cible)),
            Statut::Paralysie => format!("La paralysie frappe {}.", nom_de(*cible)),
        },
        Evenement::BloqueParLaParalysie(combattant) => {
            format!("La paralysie empêche {} d'attaquer.", nom_de(*combattant))
        }
        Evenement::DegatsDePoison { cible, degats } => {
            format!("Le poison fait perdre {degats} PV à {}.", nom_de(*cible))
        }
        Evenement::Ko(combattant) => format!("{} est KO.", nom_de(*combattant)),
        Evenement::FiletLance { cible, chance } => format!(
            "Vous lancez un filet sur {} ({chance} % de chances).",
            nom_de(*cible)
        ),
        Evenement::CaptureReussie(cible) => {
            format!("Capture réussie : {}.", combat.monstre(*cible).nom())
        }
        Evenement::CaptureRatee(cible) => format!("{} se dégage du filet.", nom_de(*cible)),
        Evenement::PotionUtilisee { cible, pv_rendus } => {
            format!("{} récupère {pv_rendus} PV.", nom_de(*cible))
        }
        Evenement::Fuite => "Vous prenez la fuite.".to_string(),
        Evenement::FinDuCombat(Issue::Victoire) => "Victoire !".to_string(),
        Evenement::FinDuCombat(Issue::Defaite) => {
            "Défaite : toute votre équipe est KO.".to_string()
        }
        Evenement::FinDuCombat(Issue::Fuite) => "Le combat est terminé.".to_string(),
    }
}

pub fn rapport_experience(monstre: &Monstre, rapport: &RapportExperience) -> String {
    let nom = nom(monstre, Cote::Joueur);
    let mut lignes = vec![format!("{nom} gagne {} XP.", rapport.experience_gagnee)];
    if rapport.niveau_apres > rapport.niveau_avant {
        lignes.push(format!(
            "{nom} passe au niveau {}.",
            rapport.niveau_apres.valeur()
        ));
    }
    for attaque in &rapport.attaques_apprises {
        lignes.push(format!("{nom} apprend {}.", attaque.nom()));
    }
    lignes.join("\n")
}

pub fn question_oubli(monstre: &Monstre, nouvelle: &Attaque) -> String {
    let mut lignes = vec![
        format!(
            "{} veut apprendre {}, mais connaît déjà 4 attaques.",
            nom(monstre, Cote::Joueur),
            decrire_attaque(nouvelle)
        ),
        "Quelle attaque oublier ?".to_string(),
    ];
    for (position, attaque) in monstre.attaques().liste().iter().enumerate() {
        lignes.push(format!("  {}. {}", position + 1, decrire_attaque(attaque)));
    }
    lignes.push(format!("  0. Renoncer à {}", nouvelle.nom()));
    lignes.join("\n")
}

pub fn renoncement(monstre: &Monstre, nouvelle: &Attaque) -> String {
    format!(
        "{} n'apprend pas {}.",
        nom(monstre, Cote::Joueur),
        nouvelle.nom()
    )
}

pub fn remplacement(monstre: &Monstre, oubliee: &Attaque, nouvelle: &Attaque) -> String {
    format!(
        "{} oublie {} et apprend {}.",
        nom(monstre, Cote::Joueur),
        oubliee.nom(),
        nouvelle.nom()
    )
}

/// Le détail technique, parfois en anglais, n'est pas montré au joueur.
pub fn erreur_sauvegarde(erreur: &ErreurSauvegarde) -> String {
    match erreur {
        ErreurSauvegarde::Acces(_) => "le fichier est inaccessible".to_string(),
        ErreurSauvegarde::Illisible(_) => {
            "le fichier est abîmé ou n'est pas une sauvegarde".to_string()
        }
        ErreurSauvegarde::VersionInconnue(_) => "elle vient d'une autre version du jeu".to_string(),
        ErreurSauvegarde::EspeceInconnue(espece) => {
            format!("elle cite une espèce qui n'existe plus ({espece})")
        }
        ErreurSauvegarde::AttaqueInconnue { attaque, .. } => {
            format!("elle cite une attaque qui n'existe plus ({attaque})")
        }
        ErreurSauvegarde::Incoherente(_) => {
            "ses données ne respectent pas les règles du jeu".to_string()
        }
    }
}

pub fn erreur_potion(erreur: &ErreurPotion) -> String {
    match erreur {
        ErreurPotion::AucunePotion => "Vous n'avez plus de potion.".to_string(),
        ErreurPotion::MonstreInexistant => "Ce monstre n'est pas dans l'équipe.".to_string(),
        ErreurPotion::MonstreKo => "Une potion ne relève pas un monstre KO.".to_string(),
        ErreurPotion::PvDejaPleins => "Ce monstre a déjà tous ses PV.".to_string(),
    }
}

pub fn nom_sauvage(monstre: &Monstre) -> String {
    nom(monstre, Cote::Adversaire)
}

pub fn potions(nombre: u32) -> String {
    if nombre > 1 {
        format!("{nombre} potions")
    } else {
        format!("{nombre} potion")
    }
}

/// Zones accessibles avec leurs niveaux, qui disent leur difficulté (section
/// 9.1).
pub fn carte(partie: &Partie, accessibles: &[&Zone]) -> String {
    let mut lignes = vec!["Carte :".to_string()];
    for (position, zone) in accessibles.iter().enumerate() {
        let (minimum, maximum) = zone.niveaux();
        let exploree = if partie.est_exploree(zone.id()) {
            ", explorée"
        } else {
            ""
        };
        lignes.push(format!(
            "  {}. {} (niveaux {} à {}){exploree}",
            position + 1,
            zone.nom(),
            minimum.valeur(),
            maximum.valeur()
        ));
    }
    lignes.join("\n")
}

pub fn equipe_en_expedition(partie: &Partie) -> String {
    let mut lignes = vec!["Votre équipe :".to_string()];
    for (index, monstre) in partie.collection().equipe().iter().enumerate() {
        lignes.push(format!(
            "  {}. {}",
            index + 1,
            ligne_monstre(monstre, Cote::Joueur)
        ));
    }
    lignes.push(format!("Potions : {}", partie.inventaire().potions()));
    lignes.join("\n")
}

/// Une espèce inconnue reste masquée ; une espèce capturée montre ses stats de
/// base et ses attaques (section 5.6).
pub fn codex(codex: &Codex, especes: &[Espece]) -> String {
    let mut lignes = vec!["Codex :".to_string()];
    for (position, espece) in especes.iter().enumerate() {
        let numero = position + 1;
        let types: Vec<&str> = espece
            .types()
            .liste()
            .iter()
            .map(|type_monstre| type_monstre.nom())
            .collect();
        match codex.etat(espece.id()) {
            EtatCodex::Inconnue => lignes.push(format!("  {numero}. ???")),
            EtatCodex::Rencontree => lignes.push(format!(
                "  {numero}. {} ({}) : rencontrée",
                espece.nom(),
                types.join(", ")
            )),
            EtatCodex::Capturee => {
                let base = espece.stats_de_base();
                lignes.push(format!(
                    "  {numero}. {} ({}) : capturée",
                    espece.nom(),
                    types.join(", ")
                ));
                lignes.push(format!(
                    "     Stats de base : PV {}, Attaque {}, Défense {}, Attaque spéciale {}, Défense spéciale {}, Vitesse {}",
                    base.pv,
                    base.attaque,
                    base.defense,
                    base.attaque_speciale,
                    base.defense_speciale,
                    base.vitesse
                ));
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
                lignes.push(format!("     Attaques : {}", attaques.join(", ")));
            }
        }
    }
    lignes.join("\n")
}

/// Avec un surnom, le nom de l'espèce reste visible entre parenthèses.
fn ligne_monstre(monstre: &Monstre, cote: Cote) -> String {
    let etat = if monstre.est_ko() {
        ", KO".to_string()
    } else {
        match monstre.statut() {
            Some(Statut::Poison) => ", poison".to_string(),
            Some(Statut::Paralysie) => ", paralysie".to_string(),
            None => String::new(),
        }
    };
    let espece = if monstre.surnom().is_some() {
        format!("{}, ", monstre.espece().nom())
    } else {
        String::new()
    };
    format!(
        "{} ({espece}niv. {}) : {}/{} PV{etat}",
        nom(monstre, cote),
        monstre.niveau().valeur(),
        monstre.pv_actuels(),
        monstre.stats().pv
    )
}

fn decrire_attaque(attaque: &Attaque) -> String {
    let categorie = match attaque.categorie() {
        Categorie::Physique => "physique",
        Categorie::Speciale => "spéciale",
    };
    let effet = match attaque.effet() {
        Some(effet) => {
            let statut = match effet.statut() {
                Statut::Poison => "poison",
                Statut::Paralysie => "paralysie",
            };
            format!(", {statut} {} %", effet.chance())
        }
        None => String::new(),
    };
    format!(
        "{} ({}, {categorie}, puissance {}, précision {} %{effet})",
        attaque.nom(),
        attaque.type_attaque().nom(),
        attaque.puissance(),
        attaque.precision()
    )
}

fn phrase_efficacite(efficacite: Efficacite) -> Option<&'static str> {
    match efficacite {
        Efficacite::TresForte => Some("C'est extrêmement efficace !"),
        Efficacite::Forte => Some("C'est très efficace !"),
        Efficacite::Normale | Efficacite::Nulle => None,
        Efficacite::Reduite => Some("Ce n'est pas très efficace."),
        Efficacite::TresReduite => Some("Ce n'est presque pas efficace."),
    }
}
