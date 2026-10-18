//! Messages du bot : ce qui vient de se passer, puis la question en cours
//! avec ses boutons. Les expéditions, les combats et la gestion des monstres
//! arrivent à l'étape 12 ; d'ici là, le camp n'affiche qu'un résumé.

use bersemon_core::contenu::{ContenuV1, INTRODUCTION};
use bersemon_core::deroulement::{Attente, Confirmation, ErreurChoix, Evenement};
use bersemon_core::heros::Classe;
use bersemon_core::monstre::Monstre;
use bersemon_core::nom::Nom;
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::ErreurSauvegarde;
use poise::serenity_prelude as serenity;

use crate::boutons::{Action, Bouton};
use crate::joueurs::{Refus, Session};

/// Champ du formulaire où le joueur écrit le nom de son héros.
pub const CHAMP_NOM: &str = "nom";

/// Un message Discord compte au plus 2000 caractères.
const LONGUEUR_MAX: usize = 2000;

pub struct Message {
    pub texte: String,
    pub composants: Vec<serenity::CreateActionRow>,
}

/// Ce qui vient de se passer, un éventuel refus, puis la question en cours.
pub fn message(
    session: &Session,
    evenements: &[Evenement],
    refus: Option<&Refus>,
    contenu: &ContenuV1,
) -> Message {
    let mut paragraphes: Vec<String> = evenements.iter().filter_map(raconter).collect();
    if let Some(refus) = refus {
        paragraphes.push(expliquer(refus));
    }
    let (question, boutons) = question(session, contenu);
    paragraphes.push(question);
    let composants = if boutons.is_empty() {
        Vec::new()
    } else {
        vec![serenity::CreateActionRow::Buttons(boutons)]
    };
    Message {
        texte: limiter(paragraphes.join("\n\n")),
        composants,
    }
}

/// Formulaire du nom du héros, ouvert par son bouton.
pub fn formulaire_du_nom(version: u32) -> serenity::CreateModal {
    let longueur_max = u16::try_from(Nom::LONGUEUR_MAX).unwrap_or(u16::MAX);
    let champ =
        serenity::CreateInputText::new(serenity::InputTextStyle::Short, "Nom du héros", CHAMP_NOM)
            .min_length(1)
            .max_length(longueur_max);
    let identifiant = Bouton {
        version,
        action: Action::NomSaisi,
    }
    .identifiant();
    serenity::CreateModal::new(identifiant, "Votre héros")
        .components(vec![serenity::CreateActionRow::InputText(champ)])
}

/// Seuls les événements de l'accueil peuvent survenir à cette étape.
fn raconter(evenement: &Evenement) -> Option<String> {
    match evenement {
        Evenement::Introduction => Some(INTRODUCTION.to_string()),
        Evenement::PartieCommencee(depart) => {
            Some(format!("{} vous accompagne désormais.", depart.nom()))
        }
        Evenement::SauvegardeImpossible(erreur) => Some(format!(
            "Sauvegarde impossible : {}.",
            erreur_sauvegarde(erreur)
        )),
        _ => None,
    }
}

fn expliquer(refus: &Refus) -> String {
    match refus {
        Refus::MessagePerime => {
            "Ce message n'était plus à jour : voici où en est votre partie.".to_string()
        }
        Refus::Choix(ErreurChoix::Nom(erreur)) => format!("Nom refusé : {erreur}."),
        Refus::Choix(erreur) => format!("Action impossible : {erreur}."),
    }
}

fn question(session: &Session, contenu: &ContenuV1) -> (String, Vec<serenity::CreateButton>) {
    let bouton = |action: Action, libelle: &str, style: serenity::ButtonStyle| {
        let identifiant = Bouton {
            version: session.version,
            action,
        }
        .identifiant();
        serenity::CreateButton::new(identifiant)
            .label(libelle)
            .style(style)
    };
    let principal = serenity::ButtonStyle::Primary;
    let secondaire = serenity::ButtonStyle::Secondary;
    match session.deroulement.attente() {
        Attente::Reprise(partie) => (
            format!(
                "Partie en cours : {}, {}.",
                partie.heros().nom().as_str(),
                partie.heros().classe().nom()
            ),
            vec![
                bouton(
                    Action::ReprendreLaPartie,
                    "Continuer cette partie",
                    principal,
                ),
                bouton(
                    Action::NouvellePartie,
                    "Commencer une nouvelle partie",
                    secondaire,
                ),
            ],
        ),
        // Sans clic du joueur, la sauvegarde reste telle quelle (Q56).
        Attente::SauvegardeIllisible(erreur) => (
            format!(
                "Votre sauvegarde ne peut pas être chargée : {}.",
                erreur_sauvegarde(erreur)
            ),
            vec![bouton(
                Action::NouvellePartie,
                "Commencer une nouvelle partie, qui la remplacera",
                secondaire,
            )],
        ),
        Attente::NomDuHeros => (
            "Bienvenue dans Bersemon. Pour commencer, nommez votre héros.".to_string(),
            vec![bouton(
                Action::OuvrirLeFormulaireDuNom,
                "Nommer votre héros",
                principal,
            )],
        ),
        Attente::Classe => {
            let lignes: Vec<String> = Classe::TOUTES.iter().map(|c| decrire_classe(*c)).collect();
            let boutons = Classe::TOUTES
                .iter()
                .map(|classe| bouton(Action::Classe(*classe), classe.nom(), principal))
                .collect();
            (
                format!("Classe de votre héros :\n{}", lignes.join("\n")),
                boutons,
            )
        }
        Attente::MonstreDeDepart => {
            let especes = contenu.especes_de_depart();
            let lignes: Vec<String> = especes
                .iter()
                .map(|espece| {
                    let types: Vec<&str> = espece
                        .types()
                        .liste()
                        .iter()
                        .map(|type_monstre| type_monstre.nom())
                        .collect();
                    format!("{} ({})", espece.nom(), types.join(", "))
                })
                .collect();
            let boutons = especes
                .iter()
                .map(|espece| {
                    let action = Action::MonstreDeDepart(espece.id().clone());
                    bouton(action, espece.nom(), principal)
                })
                .collect();
            (
                format!("Choisissez votre premier monstre :\n{}", lignes.join("\n")),
                boutons,
            )
        }
        Attente::Confirmation(confirmation) => (
            question_de_confirmation(confirmation),
            vec![
                bouton(Action::Oui, "Oui", serenity::ButtonStyle::Danger),
                bouton(Action::Non, "Non", secondaire),
            ],
        ),
        Attente::Camp => (camp(session), Vec::new()),
        _ => (
            "Cette partie du jeu arrive bientôt sur Discord.".to_string(),
            Vec::new(),
        ),
    }
}

fn question_de_confirmation(confirmation: &Confirmation) -> String {
    match confirmation {
        Confirmation::RemplacerLaPartie(heros) => format!(
            "La partie en cours ({}) sera remplacée. Continuer ?",
            heros.nom().as_str()
        ),
        Confirmation::PotionPerdue(monstre) => format!(
            "{} a déjà tous ses PV : la potion serait perdue. L'utiliser quand même ?",
            monstre.nom()
        ),
        Confirmation::Relacher(monstre) => {
            format!("Relâcher définitivement {} ?", monstre.nom())
        }
    }
}

fn camp(session: &Session) -> String {
    let Some(partie) = session.deroulement.partie() else {
        return String::new();
    };
    let equipe: Vec<String> = partie
        .collection()
        .equipe()
        .iter()
        .map(ligne_monstre)
        .collect();
    format!(
        "{}\nÉquipe :\n{}\n\nLes expéditions, les combats et la gestion des monstres arrivent bientôt sur Discord.",
        en_tete_du_camp(partie),
        equipe.join("\n")
    )
}

fn en_tete_du_camp(partie: &Partie) -> String {
    let potions = partie.inventaire().potions();
    let pluriel = if potions > 1 { "s" } else { "" };
    format!(
        "Camp de base ({}, {}). {potions} potion{pluriel}.",
        partie.heros().nom().as_str(),
        partie.heros().classe().nom()
    )
}

fn ligne_monstre(monstre: &Monstre) -> String {
    format!(
        "{} (niv. {}) : {}/{} PV",
        monstre.nom(),
        monstre.niveau().valeur(),
        monstre.pv_actuels(),
        monstre.stats().pv
    )
}

fn decrire_classe(classe: Classe) -> String {
    let stat = match classe {
        Classe::Guerrier => "Attaque",
        Classe::Mage => "Attaque spéciale",
    };
    format!(
        "{} : +10 % d'{stat} pour vos monstres en combat",
        classe.nom()
    )
}

/// Le détail technique, parfois en anglais, n'est pas montré au joueur.
fn erreur_sauvegarde(erreur: &ErreurSauvegarde) -> String {
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

/// Coupe un texte trop long pour Discord plutôt que de voir le message
/// refusé.
fn limiter(texte: String) -> String {
    if texte.chars().count() <= LONGUEUR_MAX {
        return texte;
    }
    let mut coupe: String = texte.chars().take(LONGUEUR_MAX - 3).collect();
    coupe.push_str("...");
    coupe
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::aleatoire::GenerateurAleatoire;
    use bersemon_core::deroulement::{Choix, Contexte, Deroulement};
    use bersemon_storage::FichierJson;

    fn session(deroulement: Deroulement) -> Session {
        Session {
            deroulement,
            version: 7,
        }
    }

    #[test]
    fn premier_message_d_un_nouveau_joueur() {
        let contenu = ContenuV1::new();
        let message = message(
            &session(Deroulement::demarrer(Ok(None))),
            &[],
            None,
            &contenu,
        );
        assert!(message.texte.starts_with("Bienvenue dans Bersemon."));
        assert_eq!(message.composants.len(), 1);
    }

    #[test]
    fn l_introduction_et_le_choix_du_monstre_tiennent_dans_un_message() {
        let contenu = ContenuV1::new();
        let mut deroulement = Deroulement::demarrer(Ok(None));
        // Le nom et la classe ne tirent rien au hasard et ne sauvegardent
        // pas : le dépôt reste inutilisé.
        let mut aleatoire = GenerateurAleatoire::new(0);
        let depot = FichierJson::new(std::env::temp_dir().join("bersemon-inutilise.json"));
        let mut contexte = Contexte {
            contenu: &contenu,
            aleatoire: &mut aleatoire,
            depot: &depot,
        };
        deroulement
            .repondre(Choix::NomDuHeros("Aldric".to_string()), &mut contexte)
            .unwrap();
        let evenements = deroulement
            .repondre(Choix::Classe(Classe::Mage), &mut contexte)
            .unwrap();
        let message = message(&session(deroulement), &evenements, None, &contenu);
        assert!(message.texte.starts_with(INTRODUCTION));
        assert!(message.texte.contains("Choisissez votre premier monstre"));
        assert!(message.texte.chars().count() < LONGUEUR_MAX);
    }

    #[test]
    fn un_texte_trop_long_est_coupe() {
        let texte = "x".repeat(LONGUEUR_MAX + 10);
        let coupe = limiter(texte);
        assert_eq!(coupe.chars().count(), LONGUEUR_MAX);
        assert!(coupe.ends_with("..."));
    }

    #[test]
    fn un_refus_est_explique_avant_la_question() {
        let contenu = ContenuV1::new();
        let message = message(
            &session(Deroulement::demarrer(Ok(None))),
            &[],
            Some(&Refus::MessagePerime),
            &contenu,
        );
        assert!(message.texte.starts_with("Ce message n'était plus à jour"));
    }
}
