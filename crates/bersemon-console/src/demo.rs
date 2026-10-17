//! Contenu provisoire, en attendant le bestiaire et les zones de la V1 (Q44,
//! Q45) : ces espèces, leurs attaques, leurs rendements, les noms et les
//! niveaux des zones ne sont pas des décisions de game design.

use bersemon_core::aleatoire::Aleatoire;
use bersemon_core::attaque::{Attaque, AttaqueId, Categorie, EffetSecondaire};
use bersemon_core::catalogue::Catalogue;
use bersemon_core::espece::{AttaqueApprise, Espece, EspeceId, Role};
use bersemon_core::monstre::Monstre;
use bersemon_core::niveau::Niveau;
use bersemon_core::stats::{Stats, StatsDeBase, ValeursInternes};
use bersemon_core::statut::Statut;
use bersemon_core::types::{Type, TypesMonstre};
use bersemon_core::zone::{EspeceRare, Zone, ZoneId};

/// Espèces de démonstration, retrouvées par identifiant au chargement d'une
/// sauvegarde et listées dans l'ordre du codex.
pub struct CatalogueDemo(Vec<Espece>);

impl CatalogueDemo {
    pub fn especes(&self) -> &[Espece] {
        &self.0
    }
}

impl Catalogue for CatalogueDemo {
    fn espece(&self, id: &EspeceId) -> Option<Espece> {
        self.0.iter().find(|espece| espece.id() == id).cloned()
    }
}

pub fn catalogue() -> CatalogueDemo {
    CatalogueDemo(toutes_les_especes())
}

/// Le monstre de départ est au niveau 5 (section 6).
const NIVEAU_DE_DEPART: u8 = 5;

/// Les trois espèces proposées au départ.
pub fn especes_de_depart() -> Vec<Espece> {
    vec![salamandre(), crapaud(), mandragore()]
}

/// Valeurs internes tirées au hasard, jamais shiny (Q52).
pub fn monstre_de_depart(espece: Espece, aleatoire: &mut dyn Aleatoire) -> Monstre {
    Monstre::creer(
        espece,
        niveau(NIVEAU_DE_DEPART),
        ValeursInternes::tirer(aleatoire),
        false,
    )
    .expect("apprentissage de démonstration valide")
}

/// Toutes les espèces du jeu, dans l'ordre du codex.
fn toutes_les_especes() -> Vec<Espece> {
    vec![
        salamandre(),
        crapaud(),
        mandragore(),
        loup(),
        vipere(),
        corbeau(),
        cerf_blanc(),
        basilic(),
    ]
}

/// Les deux zones de la V1, de la plus facile à la plus difficile (section
/// 9.1).
pub fn zones() -> Vec<Zone> {
    let lisiere = Zone::new(
        ZoneId::new("lisiere"),
        "Lisière du Bois-Sombre",
        (niveau(3), niveau(6)),
        2,
        vec![loup(), vipere(), corbeau()],
        vec![EspeceRare {
            espece: cerf_blanc(),
            pacifique: true,
        }],
    )
    .expect("zone de démonstration valide");
    let marais = Zone::new(
        ZoneId::new("marais"),
        "Marais des Pendus",
        (niveau(7), niveau(11)),
        3,
        vec![loup(), vipere(), corbeau()],
        vec![EspeceRare {
            espece: basilic(),
            pacifique: false,
        }],
    )
    .expect("zone de démonstration valide");
    vec![lisiere, marais]
}

fn salamandre() -> Espece {
    espece(
        "salamandre",
        "Salamandre",
        &[Type::Feu],
        base(45, 50, 40, 60, 50, 65),
        62,
        vec![
            apprise(
                1,
                physique("coup-de-queue", "Coup de queue", Type::Normal, 40, 100),
            ),
            apprise(
                3,
                speciale(
                    "crachat-de-braises",
                    "Crachat de braises",
                    Type::Feu,
                    40,
                    100,
                ),
            ),
            apprise(5, speciale("flamme-vive", "Flamme vive", Type::Feu, 70, 75)),
            apprise(
                6,
                physique("morsure-ardente", "Morsure ardente", Type::Feu, 55, 95),
            ),
            apprise(
                6,
                physique("fouet-de-queue", "Fouet de queue", Type::Normal, 50, 100),
            ),
        ],
    )
}

fn crapaud() -> Espece {
    espece(
        "crapaud",
        "Crapaud",
        &[Type::Eau],
        base(50, 50, 55, 50, 50, 45),
        63,
        vec![
            apprise(
                1,
                physique("charge-gluante", "Charge gluante", Type::Normal, 40, 100),
            ),
            apprise(
                3,
                speciale("jet-de-vase", "Jet de vase", Type::Eau, 40, 100),
            ),
            apprise(
                6,
                speciale("crachat-amer", "Crachat amer", Type::Poison, 35, 100)
                    .avec_effet(effet(Statut::Poison, 30)),
            ),
        ],
    )
}

fn mandragore() -> Espece {
    espece(
        "mandragore",
        "Mandragore",
        &[Type::Plante],
        base(55, 45, 50, 55, 55, 40),
        64,
        vec![
            apprise(
                1,
                physique("coup-de-racine", "Coup de racine", Type::Normal, 40, 100),
            ),
            apprise(
                3,
                speciale(
                    "feuille-tranchante",
                    "Feuille tranchante",
                    Type::Plante,
                    40,
                    100,
                ),
            ),
            apprise(
                6,
                speciale("cri-strident", "Cri strident", Type::Normal, 40, 100)
                    .avec_effet(effet(Statut::Paralysie, 20)),
            ),
        ],
    )
}

fn loup() -> Espece {
    espece(
        "loup",
        "Loup",
        &[Type::Normal],
        base(50, 60, 45, 30, 40, 55),
        80,
        vec![
            apprise(1, physique("griffure", "Griffure", Type::Normal, 40, 100)),
            apprise(4, physique("morsure", "Morsure", Type::Tenebres, 60, 90)),
        ],
    )
}

fn vipere() -> Espece {
    espece(
        "vipere",
        "Vipère",
        &[Type::Poison],
        base(40, 50, 40, 45, 45, 60),
        75,
        vec![
            apprise(
                1,
                physique(
                    "morsure-venimeuse",
                    "Morsure venimeuse",
                    Type::Poison,
                    40,
                    100,
                )
                .avec_effet(effet(Statut::Poison, 30)),
            ),
            apprise(
                3,
                speciale(
                    "regard-petrifiant",
                    "Regard pétrifiant",
                    Type::Normal,
                    30,
                    100,
                )
                .avec_effet(effet(Statut::Paralysie, 30)),
            ),
        ],
    )
}

fn corbeau() -> Espece {
    espece(
        "corbeau",
        "Corbeau",
        &[Type::Vol],
        base(40, 55, 35, 40, 35, 70),
        58,
        vec![
            apprise(
                1,
                physique("coup-de-bec", "Coup de bec", Type::Vol, 35, 100),
            ),
            apprise(4, physique("pique", "Piqué", Type::Vol, 60, 90)),
        ],
    )
}

fn cerf_blanc() -> Espece {
    espece(
        "cerf-blanc",
        "Cerf blanc",
        &[Type::Normal, Type::Feerique],
        base(60, 60, 55, 50, 55, 70),
        110,
        vec![
            apprise(
                1,
                physique("coup-de-bois", "Coup de bois", Type::Normal, 40, 100),
            ),
            apprise(5, physique("ruade", "Ruade", Type::Normal, 60, 95)),
        ],
    )
}

fn basilic() -> Espece {
    espece(
        "basilic",
        "Basilic",
        &[Type::Poison, Type::Dragon],
        base(70, 65, 60, 70, 60, 55),
        140,
        vec![
            apprise(
                1,
                physique(
                    "morsure-venimeuse",
                    "Morsure venimeuse",
                    Type::Poison,
                    40,
                    100,
                )
                .avec_effet(effet(Statut::Poison, 30)),
            ),
            apprise(
                6,
                speciale(
                    "regard-petrifiant",
                    "Regard pétrifiant",
                    Type::Normal,
                    30,
                    100,
                )
                .avec_effet(effet(Statut::Paralysie, 30)),
            ),
            apprise(
                9,
                speciale("souffle-putride", "Souffle putride", Type::Dragon, 60, 100),
            ),
        ],
    )
}

fn niveau(valeur: u8) -> Niveau {
    Niveau::new(valeur).expect("niveau de démonstration valide")
}

fn base(
    pv: u16,
    attaque: u16,
    defense: u16,
    attaque_speciale: u16,
    defense_speciale: u16,
    vitesse: u16,
) -> StatsDeBase {
    Stats {
        pv,
        attaque,
        defense,
        attaque_speciale,
        defense_speciale,
        vitesse,
    }
}

fn espece(
    id: &str,
    nom: &str,
    types: &[Type],
    base: StatsDeBase,
    rendement: u16,
    apprentissage: Vec<AttaqueApprise>,
) -> Espece {
    Espece::new(
        EspeceId::new(id),
        nom,
        TypesMonstre::new(types).expect("types de démonstration valides"),
        Role::Offensif,
        base,
        rendement,
        apprentissage,
    )
}

fn apprise(niveau_appris: u8, attaque: Attaque) -> AttaqueApprise {
    AttaqueApprise {
        niveau: niveau(niveau_appris),
        attaque,
    }
}

fn physique(id: &str, nom: &str, type_attaque: Type, puissance: u16, precision: u8) -> Attaque {
    attaque(
        id,
        nom,
        type_attaque,
        Categorie::Physique,
        puissance,
        precision,
    )
}

fn speciale(id: &str, nom: &str, type_attaque: Type, puissance: u16, precision: u8) -> Attaque {
    attaque(
        id,
        nom,
        type_attaque,
        Categorie::Speciale,
        puissance,
        precision,
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
