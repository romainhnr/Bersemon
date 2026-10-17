//! Contenu provisoire du combat de démonstration, en attendant le bestiaire
//! de la V1 (Q44) : ces espèces et ces attaques ne sont pas des décisions de
//! game design.

use bersemon_core::aleatoire::Aleatoire;
use bersemon_core::attaque::{Attaque, AttaqueId, AttaquesMonstre, Categorie, EffetSecondaire};
use bersemon_core::espece::{Espece, EspeceId, Role};
use bersemon_core::monstre::{Monstre, tirer_shiny};
use bersemon_core::niveau::Niveau;
use bersemon_core::stats::{Stats, StatsDeBase, ValeursInternes};
use bersemon_core::statut::Statut;
use bersemon_core::types::{Type, TypesMonstre};

/// Rend l'équipe du joueur puis le groupe sauvage, tous au niveau 5.
pub fn rencontre(aleatoire: &mut dyn Aleatoire) -> (Vec<Monstre>, Vec<Monstre>) {
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
    let corbeau = espece(
        "corbeau",
        "Corbeau",
        Type::Vol,
        Stats {
            pv: 40,
            attaque: 55,
            defense: 35,
            attaque_speciale: 40,
            defense_speciale: 35,
            vitesse: 70,
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
    let vipere = espece(
        "vipere",
        "Vipère",
        Type::Poison,
        Stats {
            pv: 40,
            attaque: 50,
            defense: 40,
            attaque_speciale: 45,
            defense_speciale: 45,
            vitesse: 60,
        },
    );

    let equipe = vec![
        creer(
            salamandre,
            vec![
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
            ],
            false,
            aleatoire,
        ),
        creer(
            corbeau,
            vec![
                attaque(
                    "coup-de-bec",
                    "Coup de bec",
                    Type::Vol,
                    Categorie::Physique,
                    35,
                    100,
                ),
                attaque("pique", "Piqué", Type::Vol, Categorie::Physique, 60, 90),
            ],
            false,
            aleatoire,
        ),
    ];
    let loup_shiny = tirer_shiny(aleatoire);
    let vipere_shiny = tirer_shiny(aleatoire);
    let groupe = vec![
        creer(
            loup,
            vec![
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
            ],
            loup_shiny,
            aleatoire,
        ),
        creer(
            vipere,
            vec![
                attaque(
                    "morsure-venimeuse",
                    "Morsure venimeuse",
                    Type::Poison,
                    Categorie::Physique,
                    40,
                    100,
                )
                .avec_effet(effet(Statut::Poison, 30)),
                attaque(
                    "regard-petrifiant",
                    "Regard pétrifiant",
                    Type::Normal,
                    Categorie::Speciale,
                    30,
                    100,
                )
                .avec_effet(effet(Statut::Paralysie, 30)),
            ],
            vipere_shiny,
            aleatoire,
        ),
    ];
    (equipe, groupe)
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

fn effet(statut: Statut, chance: u8) -> EffetSecondaire {
    EffetSecondaire::new(statut, chance).expect("effet de démonstration valide")
}

fn creer(
    espece: Espece,
    attaques: Vec<Attaque>,
    shiny: bool,
    aleatoire: &mut dyn Aleatoire,
) -> Monstre {
    Monstre::new(
        espece,
        Niveau::new(5).expect("niveau de démonstration valide"),
        ValeursInternes::tirer(aleatoire),
        AttaquesMonstre::new(attaques).expect("attaques de démonstration valides"),
        shiny,
    )
}
