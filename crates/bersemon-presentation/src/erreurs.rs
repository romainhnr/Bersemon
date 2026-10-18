//! Choix refusés et sauvegardes impossibles, expliqués au joueur.

use bersemon_core::capsule::ErreurApprentissage;
use bersemon_core::collection::{Collection, ErreurCollection};
use bersemon_core::combat::ErreurCombat;
use bersemon_core::deroulement::{ActionDeGestion, Choix, ErreurChoix};
use bersemon_core::inventaire::ErreurPotion;
use bersemon_core::marchand::ErreurMarchand;
use bersemon_core::sauvegarde::ErreurSauvegarde;

use crate::expeditions_automatiques::erreur_expedition_automatique;
use crate::partie::argent;

/// Le choix refusé dit de quel nom il s'agissait : celui du héros ou un
/// surnom.
pub fn erreur_de_choix(choix: &Choix, erreur: &ErreurChoix) -> String {
    match erreur {
        ErreurChoix::Nom(erreur) => match choix {
            Choix::Gerer(ActionDeGestion::Renommer { .. }) => {
                format!("Surnom refusé : {erreur}.")
            }
            _ => format!("Nom refusé : {erreur}."),
        },
        ErreurChoix::Collection(erreur) => erreur_collection(erreur),
        ErreurChoix::Potion(erreur) => erreur_potion(erreur),
        ErreurChoix::Combat(erreur) => erreur_de_combat(erreur),
        ErreurChoix::Attaque(erreur) => {
            format!("Impossible d'apprendre cette attaque : {erreur}.")
        }
        ErreurChoix::ExpeditionAutomatique(erreur) => erreur_expedition_automatique(erreur),
        ErreurChoix::Marchand(erreur) => erreur_marchand(erreur),
        ErreurChoix::CapsuleInconnue(_) => "Le marchand ne vend pas cette capsule.".to_string(),
        ErreurChoix::Apprentissage(erreur) => erreur_apprentissage(erreur),
        autre => format!("Action impossible : {autre}."),
    }
}

pub fn erreur_marchand(erreur: &ErreurMarchand) -> String {
    match erreur {
        ErreurMarchand::AucunMarchand => {
            "Aucun marchand ne s'est encore installé au camp.".to_string()
        }
        ErreurMarchand::ArgentInsuffisant {
            prix,
            argent: reste,
        } => format!(
            "Il faut {}, et vous n'en avez que {}.",
            argent(*prix),
            reste
        ),
        ErreurMarchand::QuantiteNulle => "Choisissez au moins une potion.".to_string(),
        ErreurMarchand::CapsuleDejaPossedee => {
            "Vous avez déjà cette capsule : elle sert sans s'user.".to_string()
        }
        ErreurMarchand::Collection(erreur) => erreur_collection(erreur),
    }
}

pub fn erreur_apprentissage(erreur: &ErreurApprentissage) -> String {
    match erreur {
        ErreurApprentissage::Collection(erreur) => erreur_collection(erreur),
        ErreurApprentissage::AttaqueInaccessible => {
            "Ce monstre ne peut pas apprendre cette attaque au camp.".to_string()
        }
        ErreurApprentissage::OubliManquant => {
            "Ce monstre connaît déjà 4 attaques : choisissez celle à oublier.".to_string()
        }
        ErreurApprentissage::OubliInutile => {
            "Ce monstre a encore de la place : il n'a rien à oublier.".to_string()
        }
        ErreurApprentissage::Attaque(erreur) => {
            format!("Impossible d'apprendre cette attaque : {erreur}.")
        }
    }
}

/// Les positions sont données au joueur à partir de 1.
pub fn erreur_de_combat(erreur: &ErreurCombat) -> String {
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
        // Les autres erreurs ne viennent pas d'une saisie du joueur :
        // l'interface suit la phase du combat.
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
        ErreurCollection::EmplacementEnDouble(_) => "Ce monstre est déjà choisi.".to_string(),
        autre => format!("Action impossible : {autre}."),
    }
}

pub fn erreur_potion(erreur: &ErreurPotion) -> String {
    match erreur {
        ErreurPotion::AucunePotion => "Vous n'avez plus de potion.".to_string(),
        ErreurPotion::MonstreInexistant => "Ce monstre n'est pas dans l'équipe.".to_string(),
        ErreurPotion::MonstreKo => "Une potion ne relève pas un monstre KO.".to_string(),
    }
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
        ErreurSauvegarde::ZoneInconnue(zone) => {
            format!("elle cite une zone qui n'existe plus ({zone})")
        }
        ErreurSauvegarde::CapsuleInconnue(capsule) => {
            format!("elle cite une capsule d'attaque qui n'existe plus ({capsule})")
        }
        ErreurSauvegarde::AttaqueInconnue { attaque, .. } => {
            format!("elle cite une attaque qui n'existe plus ({attaque})")
        }
        ErreurSauvegarde::Incoherente(_) => {
            "ses données ne respectent pas les règles du jeu".to_string()
        }
    }
}
