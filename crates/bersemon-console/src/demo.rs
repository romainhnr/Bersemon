//! Contenu provisoire du combat de démonstration, en attendant le bestiaire
//! de la V1 (Q44) : ces espèces et ces attaques ne sont pas des décisions de
//! game design.

use bersemon_core::aleatoire::Aleatoire;
use bersemon_core::attaque::{Attaque, AttaqueId, AttaquesMonstre, Categorie};
use bersemon_core::espece::{Espece, EspeceId, Role};
use bersemon_core::monstre::Monstre;
use bersemon_core::niveau::Niveau;
use bersemon_core::stats::{Stats, StatsDeBase, ValeursInternes};
use bersemon_core::types::{Type, TypesMonstre};

/// Rend le monstre du joueur puis le monstre sauvage, tous deux au niveau 5.
pub fn monstres(aleatoire: &mut dyn Aleatoire) -> (Monstre, Monstre) {
    let salamandre = espece(
        "salamandre",
        "Salamandre",
        Type::Feu,
        Stats {
            pv: 45,
            attaque: 50,
            defense: 40,
            attaque_speciale: 60,
            defense_speciale: 50,
            vitesse: 65,
        },
    );
    let loup = espece(
        "loup",
        "Loup",
        Type::Normal,
        Stats {
            pv: 50,
            attaque: 60,
            defense: 45,
            attaque_speciale: 30,
            defense_speciale: 40,
            vitesse: 55,
        },
    );
    let attaques_salamandre = vec![
        attaque(
            "coup-de-queue",
            "Coup de queue",
            Type::Normal,
            Categorie::Physique,
            40,
            100,
        ),
        attaque(
            "crachat-de-braises",
            "Crachat de braises",
            Type::Feu,
            Categorie::Speciale,
            40,
            100,
        ),
        attaque(
            "flamme-vive",
            "Flamme vive",
            Type::Feu,
            Categorie::Speciale,
            70,
            75,
        ),
    ];
    let attaques_loup = vec![
        attaque(
            "griffure",
            "Griffure",
            Type::Normal,
            Categorie::Physique,
            40,
            100,
        ),
        attaque(
            "morsure",
            "Morsure",
            Type::Tenebres,
            Categorie::Physique,
            60,
            90,
        ),
    ];
    (
        creer(salamandre, attaques_salamandre, aleatoire),
        creer(loup, attaques_loup, aleatoire),
    )
}

fn espece(id: &str, nom: &str, type_espece: Type, base: StatsDeBase) -> Espece {
    Espece::new(
        EspeceId::new(id),
        nom,
        TypesMonstre::new(&[type_espece]).expect("types de démonstration valides"),
        Role::Offensif,
        base,
    )
}

fn attaque(
    id: &str,
    nom: &str,
    type_attaque: Type,
    categorie: Categorie,
    puissance: u16,
    precision: u8,
) -> Attaque {
    Attaque::new(
        AttaqueId::new(id),
        nom,
        type_attaque,
        categorie,
        puissance,
        precision,
    )
    .expect("attaque de démonstration valide")
}

fn creer(espece: Espece, attaques: Vec<Attaque>, aleatoire: &mut dyn Aleatoire) -> Monstre {
    Monstre::new(
        espece,
        Niveau::new(5).expect("niveau de démonstration valide"),
        ValeursInternes::tirer(aleatoire),
        AttaquesMonstre::new(attaques).expect("attaques de démonstration valides"),
        false,
    )
}
