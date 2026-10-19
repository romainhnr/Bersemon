//! Contenu de la V1, tel que validé dans docs/contenu-v1.md : attaques,
//! espèces, zones, piste de l'Apôtre et textes de l'histoire, puis, après la
//! V1, capsules d'attaque. Toute modification se reporte d'abord dans ce
//! document.
//!
//! Ce contenu vit dans le noyau pour être partagé tel quel par la console et
//! le futur bot Discord. Les règles, elles, n'en dépendent pas : elles le
//! reçoivent de l'appelant, comme le hasard.

use std::ops::RangeInclusive;
use std::time::Duration;

use crate::attaque::{Attaque, AttaqueId, Categorie, EffetSecondaire};
use crate::capsule::Capsule;
use crate::catalogue::Catalogue;
use crate::espece::{AttaqueApprise, Espece, EspeceId, Rarete, Role};
use crate::expedition::ParametresCoffre;
use crate::expedition_automatique::ParametresExpeditionAutomatique;
use crate::habilete::Habilete;
use crate::heros::Classe;
use crate::humain::{Humain, HumainId};
use crate::niveau::Niveau;
use crate::stats::Stats;
use crate::statut::Statut;
use crate::traque::{Apotre, Indice, Piste};
use crate::tutoriel::Conseil;
use crate::types::{Type, TypesMonstre};
use crate::zone::{EspeceRare, Zone, ZoneId};

use Categorie::{Physique, Speciale};
use Statut::{Paralysie, Poison};

/// Affichée après le choix de la classe (section 6), suivie du paragraphe
/// propre à la classe, `fin_de_l_introduction`.
pub const INTRODUCTION: &str = "Le royaume s'est éteint sous un ciel de cendre. Sur les routes que les hommes ont désertées, les bêtes ont repris leurs droits, et des choses plus anciennes rôdent au bord des bois.

Ces choses ont un nom : les Apôtres. Autrefois des hommes, ils ont sacrifié ce qu'ils aimaient pour une chair de monstre, et ils servent un maître que nul n'a vu en face, le Démiurge. Une nuit, ses Apôtres ont fondu sur les vôtres. Vous êtes le seul à vous être relevé.

Vous atteignez le camp de base, dernier feu allumé avant les terres sauvages. Ceux qui survivent ici ne le doivent pas à leur seule lame : ils se lient aux créatures, les capturent au filet et les mènent au combat. Pour atteindre le Démiurge, il faudra abattre ses Apôtres, un par un.";

/// Le gardien confie au héros le monstre attitré de sa classe (Q67).
pub fn fin_de_l_introduction(classe: Classe) -> &'static str {
    match classe {
        Classe::Guerrier => {
            "Le gardien du camp jauge votre lame, puis ouvre une cage : un crapaud massif s'en extrait, la peau couverte de cicatrices. Il frappe fort et ne recule pas. Il partagera votre route."
        }
        Classe::Mage => {
            "Le gardien du camp remarque la cendre sur vos doigts, puis ouvre une cage : une salamandre s'en glisse, et l'air tremble autour d'elle. Ses flammes répondront aux vôtres. Elle partagera votre route."
        }
        Classe::Templier => {
            "Le gardien du camp observe votre bouclier, puis ouvre une cage : une mandragore en sort à pas lents, ses racines tressées comme une armure. Elle encaisse ce qui en abattrait d'autres. Elle partagera votre route."
        }
        Classe::Assassin => {
            "Le gardien du camp remarque la lame cachée dans votre manche, puis ouvre une cage : un chat noir s'en glisse sans un bruit, les yeux luisants. Il frappe avant qu'on le voie, mais ne supporte pas les coups. Il partagera votre route."
        }
    }
}

// Chaque indice s'affiche après son numéro, d'où la minuscule initiale.
const INDICE_1: &str = "au bout de la Lisière, des cordes neuves pendent encore à un chêne mort. Les traces d'une procession s'enfoncent vers le sud, là où la forêt se noie : le Marais des Pendus. Quelque chose y règne.";

const INDICE_2: &str = "des noyés flottent entre les roseaux, tous tournés vers le même point de la brume, comme s'ils écoutaient un appel.";

const INDICE_3: &str = "un gibet se dresse sur une île de vase, plus haut que les arbres. À son pied, la terre est retournée par des pas trop lourds pour un homme. L'antre de l'Apôtre est là.";

/// Le marchand se réfugie aussitôt au camp, même si l'expédition finit mal
/// (section 14.2).
pub const RENCONTRE_DU_MARCHAND: &str = "Un marchand ambulant surgit de la brume, sa carriole chargée de fioles. Les routes ne sont plus sûres pour lui : il part se réfugier au camp, où vous le retrouverez.";

const AVANT_LE_BRIGAND: &str = "Au sortir de la Lisière, un brigand vous barre la route, un loup et un corbeau à ses côtés. Ses bêtes ont faim, et lui aussi.";

const APRES_LE_BRIGAND: &str =
    "Le brigand détale dans les fourrés, abandonnant ses bêtes et sa bourse.";

const AVANT_LE_BARON: &str = "Sous le gibet, une silhouette se déplie lentement, la corde encore au cou. Le Baron pendu, Apôtre du Démiurge, vous a senti venir. Il n'y aura pas de fuite.";

const EPILOGUE: &str = "Le Baron pendu s'effondre dans la vase. Sur sa poitrine, un sceau gravé d'une main ouverte : le signe du Démiurge. D'autres Apôtres portent ce sceau, au-delà des terres connues. Votre traque ne fait que commencer.";

pub fn texte_du_conseil(conseil: Conseil) -> &'static str {
    match conseil {
        Conseil::PremierCombat => {
            "Conseil : affaiblissez un monstre avant de lancer un filet, car plus il a perdu de PV, plus la capture a de chances de réussir. Fuir réussit toujours, mais le monstre adverse frappe une dernière fois."
        }
        Conseil::PremierArretEntreDeuxEtapes => {
            "Conseil : vos monstres ne récupèrent leurs PV qu'au camp. Entre deux étapes, soignez-les avec une potion, changez l'ordre de votre équipe, ou rentrez avant qu'il ne soit trop tard : une défaite vous fait perdre tout votre sac, mais pas ce qui est rangé dans le coffre du camp."
        }
        Conseil::PremierRetourAuCamp => {
            "Conseil : au camp, vos monstres sont soignés et la partie est sauvegardée. Trois monstres vous accompagnent ; les autres attendent ici, et vous pouvez les échanger à tout moment."
        }
    }
}

/// Les monstres de départ, un par classe, sont les premières espèces du
/// codex.
const NOMBRE_DE_DEPARTS: usize = 4;

pub struct ContenuV1 {
    especes: Vec<Espece>,
    zones: Vec<Zone>,
    piste: Piste,
    capsules: Vec<Capsule>,
}

impl ContenuV1 {
    /// Panique si le contenu enfreint une règle du noyau : c'est une erreur
    /// de programmation, que les tests de ce module détectent.
    pub fn new() -> Self {
        let especes: Vec<Espece> = ESPECES.iter().map(creer_espece).collect();
        let capsules: Vec<Capsule> = CAPSULES
            .iter()
            .map(|fiche| Capsule::new(creer_attaque(fiche.attaque), fiche.prix))
            .collect();
        let capsules_de = |zone: &str| {
            CAPSULES
                .iter()
                .zip(&capsules)
                .filter(|(fiche, _)| fiche.zone == zone)
                .map(|(_, capsule)| capsule.clone())
                .collect()
        };
        let trouver = |id: &str| {
            especes
                .iter()
                .find(|espece| espece.id().as_str() == id)
                .cloned()
                .unwrap_or_else(|| panic!("espèce {id} absente du contenu"))
        };
        // Pacifique, il se rencontre dans toutes les zones (section 9.2).
        let schnoz = || EspeceRare {
            espece: trouver("schnoz"),
            pacifique: true,
        };
        // Il attend au bout de la Lisière tant qu'il n'est pas vaincu
        // (section 10.7).
        let brigand = Humain::new(
            HumainId::new("brigand"),
            "Brigand",
            vec![
                (trouver("loup"), niveau(7)),
                (trouver("corbeau"), niveau(8)),
            ],
            60,
            AVANT_LE_BRIGAND,
            APRES_LE_BRIGAND,
        )
        .expect("brigand conforme aux règles");
        let lisiere = Zone::new(
            ZoneId::new("lisiere"),
            "Lisière du Bois-Sombre",
            (niveau(3), niveau(7)),
            3,
            2,
            vec![trouver("loup"), trouver("corbeau"), trouver("vipere")],
            vec![EspeceRare {
                espece: trouver("cerf-blanc"),
                pacifique: true,
            }],
            expedition_automatique(1, 15..=60, 100..=200, 0..=1, 15..=30),
            coffre(10..=25, capsules_de("lisiere")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| zone.avec_humain(brigand))
        .expect("Lisière conforme aux règles");
        let marais = Zone::new(
            ZoneId::new("marais"),
            "Marais des Pendus",
            (niveau(8), niveau(13)),
            4,
            3,
            vec![
                trouver("feu-follet"),
                trouver("armure-hantee"),
                trouver("vipere"),
                trouver("corbeau"),
            ],
            vec![EspeceRare {
                espece: trouver("basilic"),
                pacifique: false,
            }],
            expedition_automatique(2, 40..=120, 300..=600, 1..=2, 40..=80),
            coffre(25..=50, capsules_de("marais")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .expect("Marais conforme aux règles");
        let baron = Apotre::new(trouver("baron-pendu"), &marais, AVANT_LE_BARON, EPILOGUE)
            .expect("Baron pendu conforme aux règles");
        let indice = |zone: &Zone, texte: &str| Indice {
            zone: zone.id().clone(),
            texte: texte.to_string(),
        };
        let piste = Piste::new(
            vec![
                indice(&lisiere, INDICE_1),
                indice(&marais, INDICE_2),
                indice(&marais, INDICE_3),
            ],
            baron,
        )
        .expect("piste conforme aux règles");
        Self {
            especes,
            zones: vec![lisiere, marais],
            piste,
            capsules,
        }
    }

    /// Toutes les espèces, dans l'ordre du codex.
    pub fn especes(&self) -> &[Espece] {
        &self.especes
    }

    pub fn especes_de_depart(&self) -> &[Espece] {
        &self.especes[..NOMBRE_DE_DEPARTS]
    }

    /// Chaque classe a son monstre de départ, qui reprend son profil
    /// (section 6).
    pub fn espece_de_depart(&self, classe: Classe) -> &Espece {
        let id = match classe {
            Classe::Guerrier => "crapaud",
            Classe::Mage => "salamandre",
            Classe::Templier => "mandragore",
            Classe::Assassin => "chat-noir",
        };
        self.especes_de_depart()
            .iter()
            .find(|espece| espece.id().as_str() == id)
            .expect("chaque classe a son monstre de départ dans le contenu")
    }

    /// De la plus facile à la plus difficile, dans l'ordre de la carte.
    pub fn zones(&self) -> &[Zone] {
        &self.zones
    }

    pub fn piste(&self) -> &Piste {
        &self.piste
    }

    /// Toutes les capsules d'attaque, que le marchand vend (section 14.2),
    /// dans l'ordre de docs/contenu-v1.md.
    pub fn capsules(&self) -> &[Capsule] {
        &self.capsules
    }
}

impl Default for ContenuV1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Catalogue for ContenuV1 {
    fn espece(&self, id: &EspeceId) -> Option<Espece> {
        self.especes
            .iter()
            .find(|espece| espece.id() == id)
            .cloned()
    }

    fn zone(&self, id: &ZoneId) -> Option<Zone> {
        self.zones.iter().find(|zone| zone.id() == id).cloned()
    }

    fn capsule(&self, id: &AttaqueId) -> Option<Capsule> {
        self.capsules
            .iter()
            .find(|capsule| capsule.id() == id)
            .cloned()
    }
}

struct FicheAttaque {
    id: &'static str,
    nom: &'static str,
    type_attaque: Type,
    categorie: Categorie,
    puissance: u16,
    precision: u8,
    effet: Option<(Statut, u8)>,
}

const fn fiche(
    id: &'static str,
    nom: &'static str,
    type_attaque: Type,
    categorie: Categorie,
    puissance: u16,
    precision: u8,
) -> FicheAttaque {
    FicheAttaque {
        id,
        nom,
        type_attaque,
        categorie,
        puissance,
        precision,
        effet: None,
    }
}

const fn avec_effet(fiche: FicheAttaque, statut: Statut, chance: u8) -> FicheAttaque {
    FicheAttaque {
        effet: Some((statut, chance)),
        ..fiche
    }
}

/// Section 1 de docs/contenu-v1.md, dans le même ordre.
const ATTAQUES: [FicheAttaque; 46] = [
    fiche(
        "souffle-chaud",
        "Souffle chaud",
        Type::Normal,
        Speciale,
        40,
        100,
    ),
    fiche(
        "crachat-de-braises",
        "Crachat de braises",
        Type::Feu,
        Speciale,
        40,
        100,
    ),
    fiche(
        "souffle-draconique",
        "Souffle draconique",
        Type::Dragon,
        Speciale,
        60,
        100,
    ),
    fiche("flamme-vive", "Flamme vive", Type::Feu, Speciale, 70, 90),
    fiche("fournaise", "Fournaise", Type::Feu, Speciale, 90, 85),
    fiche(
        "charge-gluante",
        "Charge gluante",
        Type::Normal,
        Physique,
        40,
        100,
    ),
    fiche("plongeon", "Plongeon", Type::Eau, Physique, 40, 100),
    avec_effet(
        fiche(
            "coup-de-langue",
            "Coup de langue",
            Type::Normal,
            Physique,
            55,
            95,
        ),
        Paralysie,
        10,
    ),
    avec_effet(
        fiche(
            "langue-venimeuse",
            "Langue venimeuse",
            Type::Poison,
            Physique,
            50,
            100,
        ),
        Poison,
        30,
    ),
    fiche("lame-de-fond", "Lame de fond", Type::Eau, Physique, 80, 85),
    fiche(
        "coup-de-racine",
        "Coup de racine",
        Type::Normal,
        Physique,
        40,
        100,
    ),
    fiche(
        "feuille-tranchante",
        "Feuille tranchante",
        Type::Plante,
        Speciale,
        40,
        100,
    ),
    avec_effet(
        fiche(
            "cri-strident",
            "Cri strident",
            Type::Normal,
            Speciale,
            45,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche("ronces", "Ronces", Type::Plante, Physique, 65, 95),
    avec_effet(
        fiche(
            "spores-putrides",
            "Spores putrides",
            Type::Poison,
            Speciale,
            55,
            100,
        ),
        Poison,
        30,
    ),
    fiche("griffure", "Griffure", Type::Normal, Physique, 40, 100),
    fiche("morsure", "Morsure", Type::Tenebres, Physique, 60, 90),
    fiche("curee", "Curée", Type::Normal, Physique, 75, 85),
    fiche("coup-de-bec", "Coup de bec", Type::Vol, Physique, 35, 100),
    fiche("pique", "Piqué", Type::Vol, Physique, 60, 90),
    avec_effet(
        fiche(
            "regard-funeste",
            "Regard funeste",
            Type::Tenebres,
            Speciale,
            50,
            100,
        ),
        Paralysie,
        10,
    ),
    avec_effet(
        fiche(
            "morsure-venimeuse",
            "Morsure venimeuse",
            Type::Poison,
            Physique,
            40,
            100,
        ),
        Poison,
        30,
    ),
    avec_effet(
        fiche(
            "regard-petrifiant",
            "Regard pétrifiant",
            Type::Normal,
            Speciale,
            30,
            100,
        ),
        Paralysie,
        30,
    ),
    fiche("etreinte", "Étreinte", Type::Normal, Physique, 60, 95),
    avec_effet(
        fiche(
            "lueur-trompeuse",
            "Lueur trompeuse",
            Type::Spectre,
            Speciale,
            40,
            100,
        ),
        Paralysie,
        10,
    ),
    fiche(
        "frisson-d-outre-tombe",
        "Frisson d'outre-tombe",
        Type::Spectre,
        Speciale,
        65,
        100,
    ),
    fiche(
        "coup-de-garde",
        "Coup de garde",
        Type::Acier,
        Physique,
        40,
        100,
    ),
    avec_effet(
        fiche(
            "lame-rouillee",
            "Lame rouillée",
            Type::Acier,
            Physique,
            65,
            95,
        ),
        Poison,
        20,
    ),
    fiche(
        "charge-spectrale",
        "Charge spectrale",
        Type::Spectre,
        Physique,
        70,
        90,
    ),
    fiche(
        "coup-de-bois",
        "Coup de bois",
        Type::Normal,
        Physique,
        40,
        100,
    ),
    fiche("ruade", "Ruade", Type::Normal, Physique, 60, 95),
    fiche(
        "lumiere-d-aube",
        "Lumière d'aube",
        Type::Feerique,
        Speciale,
        65,
        100,
    ),
    fiche(
        "souffle-putride",
        "Souffle putride",
        Type::Dragon,
        Speciale,
        70,
        95,
    ),
    avec_effet(
        fiche(
            "croc-de-venin",
            "Croc de venin",
            Type::Poison,
            Physique,
            75,
            90,
        ),
        Poison,
        30,
    ),
    avec_effet(
        fiche(
            "corde-du-gibet",
            "Corde du gibet",
            Type::Spectre,
            Physique,
            60,
            100,
        ),
        Paralysie,
        20,
    ),
    avec_effet(
        fiche(
            "lamentation",
            "Lamentation",
            Type::Spectre,
            Speciale,
            70,
            100,
        ),
        Paralysie,
        10,
    ),
    fiche("sentence", "Sentence", Type::Tenebres, Speciale, 80, 95),
    fiche(
        "trappe-du-gibet",
        "Trappe du gibet",
        Type::Tenebres,
        Physique,
        90,
        85,
    ),
    // Attaques des capsules, qu'aucune espèce n'apprend (section 6).
    fiche("brasier", "Brasier", Type::Feu, Speciale, 70, 100),
    fiche("deferlante", "Déferlante", Type::Eau, Physique, 65, 100),
    avec_effet(
        fiche(
            "lianes-etrangleuses",
            "Lianes étrangleuses",
            Type::Plante,
            Physique,
            55,
            100,
        ),
        Paralysie,
        20,
    ),
    // Attaques du Chat noir, monstre de l'Assassin.
    fiche(
        "coup-sournois",
        "Coup sournois",
        Type::Tenebres,
        Physique,
        40,
        100,
    ),
    avec_effet(
        fiche("feulement", "Feulement", Type::Normal, Speciale, 40, 100),
        Paralysie,
        20,
    ),
    fiche(
        "coup-de-grace",
        "Coup de grâce",
        Type::Tenebres,
        Physique,
        85,
        90,
    ),
    // Attaques du Schnoz, monstre caché.
    fiche(
        "eclat-astral",
        "Éclat astral",
        Type::Stellaire,
        Speciale,
        40,
        100,
    ),
    fiche(
        "rayon-astral",
        "Rayon astral",
        Type::Stellaire,
        Speciale,
        75,
        100,
    ),
];

struct FicheCapsule {
    attaque: &'static str,
    prix: u32,
    /// Identifiant de la zone dont les coffres la contiennent.
    zone: &'static str,
}

/// Section 6 de docs/contenu-v1.md, dans le même ordre.
const CAPSULES: [FicheCapsule; 6] = [
    FicheCapsule {
        attaque: "brasier",
        prix: 150,
        zone: "lisiere",
    },
    FicheCapsule {
        attaque: "deferlante",
        prix: 150,
        zone: "lisiere",
    },
    FicheCapsule {
        attaque: "lianes-etrangleuses",
        prix: 150,
        zone: "lisiere",
    },
    FicheCapsule {
        attaque: "croc-de-venin",
        prix: 200,
        zone: "marais",
    },
    FicheCapsule {
        attaque: "frisson-d-outre-tombe",
        prix: 200,
        zone: "marais",
    },
    FicheCapsule {
        attaque: "sentence",
        prix: 300,
        zone: "marais",
    },
];

struct FicheEspece {
    id: &'static str,
    nom: &'static str,
    types: &'static [Type],
    role: Role,
    /// PV, Attaque, Défense, Attaque spéciale, Défense spéciale, Vitesse.
    stats_de_base: [u16; 6],
    rendement_experience: u16,
    /// Fixe le prix de revente (section 14.5).
    rarete: Rarete,
    /// Section 7 de docs/contenu-v1.md, dans le même ordre.
    habiletes: &'static [Habilete],
    /// Niveau d'apprentissage et identifiant de l'attaque.
    apprentissage: &'static [(u8, &'static str)],
}

/// Section 2 de docs/contenu-v1.md, dans l'ordre du codex : les monstres de
/// départ, les communs, les rares, l'Apôtre, puis le monstre caché.
const ESPECES: [FicheEspece; 13] = [
    FicheEspece {
        id: "salamandre",
        nom: "Salamandre",
        types: &[Type::Feu],
        role: Role::Offensif,
        stats_de_base: [45, 40, 40, 70, 55, 65],
        rendement_experience: 62,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Affinite, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "souffle-chaud"),
            (3, "crachat-de-braises"),
            (7, "souffle-draconique"),
            (10, "flamme-vive"),
            (14, "fournaise"),
        ],
    },
    FicheEspece {
        id: "crapaud",
        nom: "Crapaud",
        types: &[Type::Eau],
        role: Role::Offensif,
        stats_de_base: [55, 68, 55, 35, 45, 55],
        rendement_experience: 63,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "charge-gluante"),
            (3, "plongeon"),
            (7, "coup-de-langue"),
            (10, "langue-venimeuse"),
            (14, "lame-de-fond"),
        ],
    },
    FicheEspece {
        id: "mandragore",
        nom: "Mandragore",
        types: &[Type::Plante],
        role: Role::Soutien,
        stats_de_base: [70, 45, 70, 45, 65, 25],
        rendement_experience: 64,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-racine"),
            (3, "feuille-tranchante"),
            (7, "cri-strident"),
            (10, "ronces"),
            (14, "spores-putrides"),
        ],
    },
    FicheEspece {
        id: "chat-noir",
        nom: "Chat noir",
        types: &[Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [40, 72, 35, 40, 38, 90],
        rendement_experience: 62,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Traqueur],
        apprentissage: &[
            (1, "griffure"),
            (3, "coup-sournois"),
            (7, "feulement"),
            (10, "morsure"),
            (14, "coup-de-grace"),
        ],
    },
    FicheEspece {
        id: "loup",
        nom: "Loup",
        types: &[Type::Normal],
        role: Role::Offensif,
        stats_de_base: [50, 62, 45, 30, 40, 58],
        rendement_experience: 70,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Traqueur, Habilete::Fouineur],
        apprentissage: &[(1, "griffure"), (4, "morsure"), (9, "curee")],
    },
    FicheEspece {
        id: "corbeau",
        nom: "Corbeau",
        types: &[Type::Vol, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [42, 57, 38, 45, 38, 72],
        rendement_experience: 66,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Pillard, Habilete::Eclaireur],
        apprentissage: &[(1, "coup-de-bec"), (4, "pique"), (8, "regard-funeste")],
    },
    FicheEspece {
        id: "vipere",
        nom: "Vipère",
        types: &[Type::Poison],
        role: Role::Offensif,
        stats_de_base: [40, 50, 40, 50, 45, 63],
        rendement_experience: 68,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Instinct],
        apprentissage: &[
            (1, "morsure-venimeuse"),
            (3, "regard-petrifiant"),
            (9, "etreinte"),
        ],
    },
    FicheEspece {
        id: "feu-follet",
        nom: "Feu follet",
        types: &[Type::Feu, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [45, 30, 40, 70, 60, 75],
        rendement_experience: 85,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Eclaireur, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "lueur-trompeuse"),
            (1, "crachat-de-braises"),
            (10, "flamme-vive"),
            (12, "frisson-d-outre-tombe"),
        ],
    },
    FicheEspece {
        id: "armure-hantee",
        nom: "Armure hantée",
        types: &[Type::Acier, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [60, 75, 85, 30, 50, 30],
        rendement_experience: 95,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "coup-de-garde"),
            (8, "lame-rouillee"),
            (11, "charge-spectrale"),
        ],
    },
    FicheEspece {
        id: "cerf-blanc",
        nom: "Cerf blanc",
        types: &[Type::Normal, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [70, 60, 60, 65, 70, 75],
        rendement_experience: 150,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[(1, "coup-de-bois"), (5, "ruade"), (8, "lumiere-d-aube")],
    },
    FicheEspece {
        id: "basilic",
        nom: "Basilic",
        types: &[Type::Poison, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [75, 70, 65, 75, 60, 55],
        rendement_experience: 170,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "morsure-venimeuse"),
            (8, "regard-petrifiant"),
            (10, "souffle-putride"),
            (13, "croc-de-venin"),
        ],
    },
    FicheEspece {
        id: "baron-pendu",
        nom: "Baron pendu",
        types: &[Type::Tenebres, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [85, 85, 75, 90, 80, 65],
        rendement_experience: 250,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "corde-du-gibet"),
            (5, "lamentation"),
            (10, "sentence"),
            (15, "trappe-du-gibet"),
        ],
    },
    FicheEspece {
        id: "schnoz",
        nom: "Schnoz",
        types: &[Type::Stellaire],
        role: Role::Soutien,
        stats_de_base: [60, 50, 60, 90, 90, 100],
        rendement_experience: 200,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Pillard, Habilete::Fouineur],
        apprentissage: &[
            (1, "eclat-astral"),
            (1, "lueur-trompeuse"),
            (10, "rayon-astral"),
        ],
    },
];

fn creer_espece(fiche: &FicheEspece) -> Espece {
    let [
        pv,
        attaque,
        defense,
        attaque_speciale,
        defense_speciale,
        vitesse,
    ] = fiche.stats_de_base;
    let apprentissage = fiche
        .apprentissage
        .iter()
        .map(|&(niveau_appris, id)| AttaqueApprise {
            niveau: niveau(niveau_appris),
            attaque: creer_attaque(id),
        })
        .collect();
    Espece::new(
        EspeceId::new(fiche.id),
        fiche.nom,
        TypesMonstre::new(fiche.types).expect("types du contenu valides"),
        fiche.role,
        Stats {
            pv,
            attaque,
            defense,
            attaque_speciale,
            defense_speciale,
            vitesse,
        },
        fiche.rendement_experience,
        apprentissage,
    )
    .avec_rarete(fiche.rarete)
    .avec_habiletes(fiche.habiletes.to_vec())
}

fn creer_attaque(id: &str) -> Attaque {
    let fiche = ATTAQUES
        .iter()
        .find(|fiche| fiche.id == id)
        .unwrap_or_else(|| panic!("attaque {id} absente du contenu"));
    let attaque = Attaque::new(
        AttaqueId::new(fiche.id),
        fiche.nom,
        fiche.type_attaque,
        fiche.categorie,
        fiche.puissance,
        fiche.precision,
    )
    .expect("attaque du contenu valide");
    match fiche.effet {
        Some((statut, chance)) => attaque
            .avec_effet(EffetSecondaire::new(statut, chance).expect("effet du contenu valide")),
        None => attaque,
    }
}

fn niveau(valeur: u8) -> Niveau {
    Niveau::new(valeur).expect("niveau du contenu valide")
}

/// Expéditions automatiques de la section 3 de docs/contenu-v1.md.
fn expedition_automatique(
    heures: u64,
    degats: RangeInclusive<u32>,
    experience: RangeInclusive<u32>,
    potions: RangeInclusive<u32>,
    argent: RangeInclusive<u32>,
) -> ParametresExpeditionAutomatique {
    ParametresExpeditionAutomatique::new(
        Duration::from_secs(heures * 3_600),
        degats,
        experience,
        potions,
        argent,
    )
    .expect("expédition automatique conforme aux règles")
}

/// Coffres de la section 3 de docs/contenu-v1.md.
fn coffre(argent: RangeInclusive<u32>, capsules: Vec<Capsule>) -> ParametresCoffre {
    ParametresCoffre::new(argent, capsules).expect("coffre conforme aux règles")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monstre::Monstre;
    use crate::stats::ValeursInternes;

    fn creer(contenu: &ContenuV1, id: &str, valeur: u8) -> Monstre {
        let espece = contenu.espece(&EspeceId::new(id)).unwrap();
        Monstre::creer(espece, niveau(valeur), ValeursInternes::default(), false).unwrap()
    }

    /// Monstre aux valeurs internes de 15, la moyenne des tirages.
    fn creer_moyen(contenu: &ContenuV1, id: &str, valeur: u8) -> Monstre {
        let espece = contenu.espece(&EspeceId::new(id)).unwrap();
        let moyennes = ValeursInternes::new(Stats {
            pv: 15,
            attaque: 15,
            defense: 15,
            attaque_speciale: 15,
            defense_speciale: 15,
            vitesse: 15,
        })
        .unwrap();
        Monstre::creer(espece, niveau(valeur), moyennes, false).unwrap()
    }

    /// Repères de la section 12.8 de docs/gamedesign.md.
    #[test]
    fn reperes_des_expeditions_automatiques() {
        let contenu = ContenuV1::new();
        let chances = |zone: usize, equipe: &[(&str, u8)]| {
            let monstres: Vec<Monstre> = equipe
                .iter()
                .map(|&(id, valeur)| creer_moyen(&contenu, id, valeur))
                .collect();
            contenu.zones()[zone]
                .expedition_automatique()
                .chances_de_reussite(&monstres)
                .pourcentage()
        };
        let salamandre = ("salamandre", 8);
        assert_eq!(chances(0, &[salamandre]), 23);
        assert_eq!(chances(0, &[salamandre, ("loup", 6)]), 71);
        assert_eq!(chances(0, &[salamandre, ("loup", 6), ("vipere", 5)]), 100);
        let salamandre = ("salamandre", 12);
        assert_eq!(chances(1, &[salamandre]), 0);
        assert_eq!(chances(1, &[salamandre, ("crapaud", 11)]), 34);
        assert_eq!(
            chances(1, &[salamandre, ("crapaud", 11), ("mandragore", 10)]),
            77
        );
    }

    #[test]
    fn identifiants_uniques() {
        for (position, fiche) in ATTAQUES.iter().enumerate() {
            assert!(
                ATTAQUES[position + 1..]
                    .iter()
                    .all(|autre| autre.id != fiche.id),
                "attaque {} en double",
                fiche.id
            );
        }
        for (position, fiche) in ESPECES.iter().enumerate() {
            assert!(
                ESPECES[position + 1..]
                    .iter()
                    .all(|autre| autre.id != fiche.id),
                "espèce {} en double",
                fiche.id
            );
        }
    }

    #[test]
    fn chaque_attaque_est_apprise_par_une_espece_ou_une_capsule() {
        for fiche in &ATTAQUES {
            let apprise = ESPECES.iter().any(|espece| {
                espece
                    .apprentissage
                    .iter()
                    .any(|&(_, attaque)| attaque == fiche.id)
            });
            let en_capsule = CAPSULES.iter().any(|capsule| capsule.attaque == fiche.id);
            assert!(
                apprise || en_capsule,
                "attaque {} apprise par aucune espèce ni capsule",
                fiche.id
            );
        }
    }

    #[test]
    fn chaque_capsule_sert_a_une_espece_qui_ne_l_apprend_pas() {
        let contenu = ContenuV1::new();
        assert_eq!(contenu.capsules().len(), 6);
        for capsule in contenu.capsules() {
            let utile = contenu.especes().iter().any(|espece| {
                capsule.convient_a(espece) && espece.attaque_apprise(capsule.id()).is_none()
            });
            assert!(utile, "capsule {} inutile", capsule.id().as_str());
            assert_eq!(contenu.capsule(capsule.id()).as_ref(), Some(capsule));
        }
    }

    #[test]
    fn les_coffres_de_chaque_zone_ont_leurs_capsules() {
        let contenu = ContenuV1::new();
        let ids = |zone: &Zone| -> Vec<String> {
            zone.coffre()
                .capsules()
                .iter()
                .map(|capsule| capsule.id().as_str().to_string())
                .collect()
        };
        assert_eq!(
            ids(&contenu.zones()[0]),
            vec!["brasier", "deferlante", "lianes-etrangleuses"]
        );
        assert_eq!(
            ids(&contenu.zones()[1]),
            vec!["croc-de-venin", "frisson-d-outre-tombe", "sentence"]
        );
    }

    #[test]
    fn contenu_conforme_aux_regles() {
        let contenu = ContenuV1::new();
        assert_eq!(contenu.especes().len(), 13);
        assert_eq!(contenu.zones().len(), 2);
        assert_eq!(contenu.piste().indices().len(), 3);
        // Toute espèce rencontrée en zone se retrouve au chargement d'une
        // sauvegarde.
        for zone in contenu.zones() {
            let rares = zone
                .rares()
                .iter()
                .chain(zone.monstre_cache())
                .map(|rare| &rare.espece);
            for espece in zone.especes().iter().chain(rares) {
                assert_eq!(contenu.espece(espece.id()).as_ref(), Some(espece));
            }
        }
    }

    #[test]
    fn la_rarete_suit_la_place_de_l_espece_dans_le_jeu() {
        let contenu = ContenuV1::new();
        let rares: Vec<&EspeceId> = contenu
            .zones()
            .iter()
            .flat_map(|zone| zone.rares().iter().chain(zone.monstre_cache()))
            .map(|rare| rare.espece.id())
            .collect();
        let apotre = contenu.piste().apotre().espece().id();
        for espece in contenu.especes() {
            let attendue = if espece.id() == apotre {
                Rarete::Apotre
            } else if rares.contains(&espece.id()) {
                Rarete::Rare
            } else {
                Rarete::Commune
            };
            assert_eq!(espece.rarete(), attendue, "{}", espece.id().as_str());
        }
    }

    #[test]
    fn le_schnoz_seul_stellaire_se_cache_dans_chaque_zone() {
        let contenu = ContenuV1::new();
        for zone in contenu.zones() {
            let cache = zone.monstre_cache().unwrap();
            assert_eq!(cache.espece.id().as_str(), "schnoz");
            assert!(cache.pacifique);
        }
        let stellaires: Vec<&str> = contenu
            .especes()
            .iter()
            .filter(|espece| espece.types().liste().contains(&Type::Stellaire))
            .map(|espece| espece.id().as_str())
            .collect();
        assert_eq!(stellaires, vec!["schnoz"]);
    }

    #[test]
    fn chaque_espece_a_deux_habiletes_et_chaque_habilete_sert() {
        let contenu = ContenuV1::new();
        for espece in contenu.especes() {
            let habiletes = espece.habiletes();
            assert_eq!(habiletes.len(), 2, "{}", espece.id().as_str());
            assert_ne!(habiletes[0], habiletes[1], "{}", espece.id().as_str());
        }
        for habilete in Habilete::TOUTES {
            assert!(
                contenu
                    .especes()
                    .iter()
                    .any(|espece| espece.habiletes().contains(&habilete)),
                "{habilete:?} sans espèce"
            );
        }
    }

    #[test]
    fn le_brigand_attend_au_bout_de_la_lisiere_avec_un_loup_et_un_corbeau() {
        let contenu = ContenuV1::new();
        let brigand = contenu.zones()[0].humain().unwrap();
        assert_eq!(brigand.id().as_str(), "brigand");
        assert_eq!(brigand.argent(), 60);
        let equipe: Vec<(String, u8)> = brigand
            .generer_equipe()
            .iter()
            .map(|monstre| {
                (
                    monstre.espece().id().as_str().to_string(),
                    monstre.niveau().valeur(),
                )
            })
            .collect();
        assert_eq!(
            equipe,
            vec![("loup".to_string(), 7), ("corbeau".to_string(), 8)]
        );
        assert!(contenu.zones()[1].humain().is_none());
    }

    #[test]
    fn chaque_classe_a_son_monstre_de_depart() {
        let contenu = ContenuV1::new();
        let ids: Vec<&str> = Classe::TOUTES
            .iter()
            .map(|classe| contenu.espece_de_depart(*classe).id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec!["crapaud", "salamandre", "mandragore", "chat-noir"]
        );
    }

    #[test]
    fn chaque_depart_connait_une_attaque_normal_et_une_de_son_type_au_niveau_5() {
        let contenu = ContenuV1::new();
        let ids: Vec<&str> = contenu
            .especes_de_depart()
            .iter()
            .map(|espece| espece.id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec!["salamandre", "crapaud", "mandragore", "chat-noir"]
        );
        for espece in contenu.especes_de_depart() {
            let monstre = creer(&contenu, espece.id().as_str(), 5);
            let types: Vec<Type> = monstre
                .attaques()
                .liste()
                .iter()
                .map(Attaque::type_attaque)
                .collect();
            assert_eq!(types, vec![Type::Normal, espece.types().liste()[0]]);
        }
    }

    #[test]
    fn le_baron_pendu_connait_ses_quatre_attaques_au_niveau_15() {
        let contenu = ContenuV1::new();
        let apotre = contenu.piste().apotre();
        assert_eq!(apotre.niveau(), niveau(15));
        assert_eq!(apotre.zone().as_str(), "marais");
        let baron = creer(&contenu, "baron-pendu", 15);
        assert_eq!(baron.attaques().liste().len(), 4);
    }

    #[test]
    fn la_piste_commence_dans_la_lisiere() {
        let contenu = ContenuV1::new();
        let zones: Vec<&str> = contenu
            .piste()
            .indices()
            .iter()
            .map(|indice| indice.zone.as_str())
            .collect();
        assert_eq!(zones, vec!["lisiere", "marais", "marais"]);
    }
}
