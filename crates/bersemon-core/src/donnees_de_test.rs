//! Fabriques pour les tests : espèces, attaques et monstres sans lien avec le
//! contenu du jeu, pour que l'équilibrage ne casse pas les tests des règles.

use crate::attaque::{Attaque, AttaqueId, AttaquesMonstre, Categorie};
use crate::espece::{Espece, EspeceId, Role};
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::stats::{Stats, StatsDeBase, ValeursInternes};
use crate::types::{Type, TypesMonstre};

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

/// Monstre aux valeurs internes nulles, non shiny, avec tous ses PV.
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
