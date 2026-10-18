//! Fabriques pour les tests : espèces, attaques et monstres sans lien avec le
//! contenu du jeu, pour que l'équilibrage ne casse pas les tests des règles.

use std::time::Duration;

use crate::attaque::{Attaque, AttaqueId, AttaquesMonstre, Categorie};
use crate::espece::{AttaqueApprise, Espece, EspeceId, Role};
use crate::expedition::ParametresCoffre;
use crate::expedition_automatique::ParametresExpeditionAutomatique;
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::stats::{Stats, StatsDeBase, ValeursInternes};
use crate::types::{Type, TypesMonstre};
use crate::zone::{Zone, ZoneId};

pub(crate) fn stats_uniformes(valeur: u16) -> StatsDeBase {
    Stats {
        pv: valeur,
        attaque: valeur,
        defense: valeur,
        attaque_speciale: valeur,
        defense_speciale: valeur,
        vitesse: valeur,
    }
}

pub(crate) fn attaque(
    id: &str,
    type_attaque: Type,
    categorie: Categorie,
    puissance: u16,
    precision: u8,
) -> Attaque {
    Attaque::new(
        AttaqueId::new(id),
        id,
        type_attaque,
        categorie,
        puissance,
        precision,
    )
    .unwrap()
}

/// Rendement d'XP des espèces de test : 70 x 5 / 7 donne 50 XP de base pour
/// un monstre vaincu au niveau 5.
pub(crate) const RENDEMENT_DE_TEST: u16 = 70;

/// Espèce Normal de base 50, qui connaît un coup dès le niveau 1 : elle peut
/// être créée à tout niveau.
pub(crate) fn espece(id: &str) -> Espece {
    Espece::new(
        EspeceId::new(id),
        id,
        TypesMonstre::new(&[Type::Normal]).unwrap(),
        Role::Offensif,
        stats_uniformes(50),
        RENDEMENT_DE_TEST,
        vec![AttaqueApprise {
            niveau: Niveau::new(1).unwrap(),
            attaque: attaque("coup", Type::Normal, Categorie::Physique, 40, 100),
        }],
    )
}

/// Une heure, 10 à 40 dégâts, 100 à 200 XP, 0 à 1 potion et toujours 20
/// pièces d'argent, sans tirage.
pub(crate) fn parametres_expedition_automatique() -> ParametresExpeditionAutomatique {
    ParametresExpeditionAutomatique::new(
        Duration::from_secs(3_600),
        10..=40,
        100..=200,
        0..=1,
        20..=20,
    )
    .unwrap()
}

/// Toujours 10 pièces d'argent par coffre, sans tirage.
pub(crate) fn parametres_coffre() -> ParametresCoffre {
    ParametresCoffre::new(10..=10).unwrap()
}

/// Zone de niveau 5 sans monstre rare, où vivent des loups en groupes d'un
/// seul monstre.
pub(crate) fn zone(id: &str) -> Zone {
    let niveau = Niveau::new(5).unwrap();
    Zone::new(
        ZoneId::new(id),
        id,
        (niveau, niveau),
        1,
        vec![espece("loup")],
        vec![],
        parametres_expedition_automatique(),
        parametres_coffre(),
    )
    .unwrap()
}

/// Monstre aux valeurs internes nulles, non shiny, avec tous ses PV, d'une
/// espèce qui n'apprend aucune attaque en montant de niveau.
pub(crate) fn monstre(
    types: &[Type],
    base: StatsDeBase,
    niveau: u8,
    attaques: Vec<Attaque>,
) -> Monstre {
    let espece = Espece::new(
        EspeceId::new("espece-de-test"),
        "Espèce de test",
        TypesMonstre::new(types).unwrap(),
        Role::Offensif,
        base,
        RENDEMENT_DE_TEST,
        Vec::new(),
    );
    let valeurs_internes = ValeursInternes::new(Stats {
        pv: 0,
        attaque: 0,
        defense: 0,
        attaque_speciale: 0,
        defense_speciale: 0,
        vitesse: 0,
    })
    .unwrap();
    Monstre::new(
        espece,
        Niveau::new(niveau).unwrap(),
        valeurs_internes,
        AttaquesMonstre::new(attaques).unwrap(),
        false,
    )
}
