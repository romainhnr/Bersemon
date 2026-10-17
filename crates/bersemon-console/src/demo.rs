//! Contenu provisoire du combat de démonstration, en attendant le bestiaire
//! de la V1 (Q44) : ces espèces, leurs attaques et leurs rendements ne sont
//! pas des décisions de game design.

use bersemon_core::aleatoire::Aleatoire;
use bersemon_core::attaque::{Attaque, AttaqueId, Categorie, EffetSecondaire};
use bersemon_core::espece::{AttaqueApprise, Espece, EspeceId, Role};
use bersemon_core::monstre::{Monstre, tirer_shiny};
use bersemon_core::niveau::Niveau;
use bersemon_core::stats::{Stats, StatsDeBase, ValeursInternes};
use bersemon_core::statut::Statut;
use bersemon_core::types::{Type, TypesMonstre};

/// Rend l'équipe du joueur puis le groupe sauvage, tous au niveau 5. La
/// Salamandre apprend deux attaques au niveau 6, la seconde sans place libre,
/// pour montrer le choix de l'attaque à oublier.
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
        62,
        vec![
            apprise(
                1,
                attaque(
                    "coup-de-queue",
                    "Coup de queue",
                    Type::Normal,
                    Categorie::Physique,
                    40,
                    100,
                ),
            ),
            apprise(
                3,
                attaque(
                    "crachat-de-braises",
                    "Crachat de braises",
                    Type::Feu,
                    Categorie::Speciale,
                    40,
                    100,
                ),
            ),
            apprise(
                5,
                attaque(
                    "flamme-vive",
                    "Flamme vive",
                    Type::Feu,
                    Categorie::Speciale,
                    70,
                    75,
                ),
            ),
            apprise(
                6,
                attaque(
                    "morsure-ardente",
                    "Morsure ardente",
                    Type::Feu,
                    Categorie::Physique,
                    55,
                    95,
                ),
            ),
            apprise(
                6,
                attaque(
                    "fouet-de-queue",
                    "Fouet de queue",
                    Type::Normal,
                    Categorie::Physique,
                    50,
                    100,
                ),
            ),
        ],
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
        58,
        vec![
            apprise(
                1,
                attaque(
                    "coup-de-bec",
                    "Coup de bec",
                    Type::Vol,
                    Categorie::Physique,
                    35,
                    100,
                ),
            ),
            apprise(
                4,
                attaque("pique", "Piqué", Type::Vol, Categorie::Physique, 60, 90),
            ),
            apprise(
                6,
                attaque(
                    "cri-percant",
                    "Cri perçant",
                    Type::Normal,
                    Categorie::Speciale,
                    40,
                    100,
                )
                .avec_effet(effet(Statut::Paralysie, 10)),
            ),
        ],
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
        80,
        vec![
            apprise(
                1,
                attaque(
                    "griffure",
                    "Griffure",
                    Type::Normal,
                    Categorie::Physique,
                    40,
                    100,
                ),
            ),
            apprise(
                4,
                attaque(
                    "morsure",
                    "Morsure",
                    Type::Tenebres,
                    Categorie::Physique,
                    60,
                    90,
                ),
            ),
        ],
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
        75,
        vec![
            apprise(
                1,
                attaque(
                    "morsure-venimeuse",
                    "Morsure venimeuse",
                    Type::Poison,
                    Categorie::Physique,
                    40,
                    100,
                )
                .avec_effet(effet(Statut::Poison, 30)),
            ),
            apprise(
                3,
                attaque(
                    "regard-petrifiant",
                    "Regard pétrifiant",
                    Type::Normal,
                    Categorie::Speciale,
                    30,
                    100,
                )
                .avec_effet(effet(Statut::Paralysie, 30)),
            ),
        ],
    );

    let equipe = vec![
        creer(salamandre, false, aleatoire),
        creer(corbeau, false, aleatoire),
    ];
    let loup_shiny = tirer_shiny(aleatoire);
    let vipere_shiny = tirer_shiny(aleatoire);
    let groupe = vec![
        creer(loup, loup_shiny, aleatoire),
        creer(vipere, vipere_shiny, aleatoire),
    ];
    (equipe, groupe)
}

fn espece(
    id: &str,
    nom: &str,
    type_espece: Type,
    base: StatsDeBase,
    rendement: u16,
    apprentissage: Vec<AttaqueApprise>,
) -> Espece {
    Espece::new(
        EspeceId::new(id),
        nom,
        TypesMonstre::new(&[type_espece]).expect("types de démonstration valides"),
        Role::Offensif,
        base,
        rendement,
        apprentissage,
    )
}

fn apprise(niveau: u8, attaque: Attaque) -> AttaqueApprise {
    AttaqueApprise {
        niveau: Niveau::new(niveau).expect("niveau de démonstration valide"),
        attaque,
    }
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

fn creer(espece: Espece, shiny: bool, aleatoire: &mut dyn Aleatoire) -> Monstre {
    Monstre::creer(
        espece,
        Niveau::new(5).expect("niveau de démonstration valide"),
        ValeursInternes::tirer(aleatoire),
        shiny,
    )
    .expect("apprentissage de démonstration valide")
}
