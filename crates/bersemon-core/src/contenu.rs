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

use crate::ame::{Gardien, MarchandId, MarchandMort};
use crate::amelioration::{
    Amelioration, PlanDAmelioration, PlanDeFabrication, PlanDesFilets, PlanDuCharme,
};
use crate::attaque::{Attaque, AttaqueId, Categorie, EffetSecondaire};
use crate::beherit::Invocation;
use crate::booster::Booster;
use crate::capsule::Capsule;
use crate::catalogue::Catalogue;
use crate::equipement::{ObjetDEquipement, ObjetDEquipementId, SorteDObjet};
use crate::espece::{AttaqueApprise, Espece, EspeceId, Rarete, Role};
use crate::evolution::{ConditionEvolution, Evolution};
use crate::expedition::ParametresCoffre;
use crate::expedition_automatique::ParametresExpeditionAutomatique;
use crate::filet::Filet;
use crate::habilete::Habilete;
use crate::heros::Classe;
use crate::humain::{Humain, HumainId, IaHumain};
use crate::inventaire::Inventaire;
use crate::jour_et_nuit::{Activite, JourOuNuit};
use crate::meteo::Meteo;
use crate::niveau::Niveau;
use crate::peche::{ParametresPeche, Pecheur};
use crate::pierre::PierreDEvolution;
use crate::recompenses_du_codex::{PalierDuCodex, RecompenseDuCodex};
use crate::ressource::{Ressource, RessourceId};
use crate::soin::Soin;
use crate::stats::Stats;
use crate::statut::Statut;
use crate::tour::TourSansFond;
use crate::traque::{Apotre, Chapitre, Indice, Piste};
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

const INDICE_2: &str = "le bourreau vaincu a parlé : son maître, le Baron pendu, s'est retiré au fond de la Vallée des Brumes, au-delà du Marais, là où les morts ne dorment pas.";

const INDICE_3: &str = "parmi les reliques du Baron, une lettre scellée du même sceau, signée de l'Abbesse : à l'est, l'Abbaye des Cendres brûle sans jamais s'éteindre. Une autre Apôtre y attend.";

const INDICE_4: &str = "l'inquisiteur vaincu a parlé : l'Abbesse écorchée se terre sous le chœur, dans la crypte où elle a livré ses sœurs au bûcher.";

const INDICE_5: &str = "sous l'autel de l'Abbesse, une plaque de fer gravée du même sceau : au nord, le Col du Givre-Noir, où un margrave a mené son armée mourir dans la neige. Un troisième Apôtre y règne.";

const INDICE_6: &str = "le porte-étendard vaincu a parlé : le Margrave attend au sommet, dans la forteresse où ses soldats ont gelé à sa place.";

/// Au bout de la première expédition menée au bout, quand le joueur porte
/// l'âme de Bertrand (section 14.2).
pub const MALVOISE_ET_L_AME: &str = "Sur le chemin du camp, un homme en robe noire vous attend, un crâne posé au creux du bras. \"Je t'ai vu recueillir cette âme. Elle m'intéresse, si tu ne sais pas quoi en faire. Je suis Malvoise, et les marchands morts sur ces routes ont encore des choses à vendre.\" Il vous suit jusqu'au camp et s'y installe.";

/// Au bout de la première expédition menée au bout, sans âme.
pub const MALVOISE_SANS_AME: &str = "Sur le chemin du camp, un homme en robe noire vous attend, un crâne posé au creux du bras. \"Je suis Malvoise. Les marchands meurent vite sur ces routes : rapporte-moi leur âme, et je te vendrai ce qu'ils vendaient de leur vivant.\" Il vous suit jusqu'au camp et s'y installe.";

/// Bertrand est abattu à l'étape 4 de la Lisière par le brigand qui attend
/// à l'étape 5 (section 14.2).
const ETAPE_DE_BERTRAND: u32 = 4;

// La bête qui attaque à la place de Bertrand ou garde Ulrich n'est pas
// nommée : le combat la présente aussitôt, et son nom ne dit pas son genre.
const AVANT_BERTRAND: &str = "Un cri déchire la Lisière : un brigand abat un colporteur sous vos yeux, lui arrache sa bourse et s'enfuit vers la sortie du bois. Une bête qui le suivait se tourne vers vous.";

// Le joueur ne connaît pas encore Malvoise (section 14.2).
const APRES_BERTRAND: &str = "L'âme de Bertrand, colporteur, flotte au-dessus de son corps. Vous la recueillez, sans trop savoir ce que c'est.";

const AVANT_GRETE: &str = "Une marchande titube vers vous entre les roseaux, la peau verdie, sa carriole renversée derrière elle. Elle n'est plus tout à fait morte.";

const APRES_GRETE: &str = "Le corps de Grete s'effondre enfin. Son âme flotte au-dessus de la vase : vous la recueillez pour Malvoise.";

const AVANT_ULRICH: &str = "Au pied d'un gibet, un alchimiste pend au bout d'une corde, ses fioles brisées dans l'herbe. Un monstre garde le corps et se jette sur vous.";

const APRES_ULRICH: &str =
    "L'âme d'Ulrich, alchimiste, s'échappe enfin de la corde. Vous la recueillez pour Malvoise.";

const AVANT_LE_BRIGAND: &str = "Au sortir de la Lisière, un brigand vous barre la route, un loup et un corbeau à ses côtés. Ses bêtes ont faim, et lui aussi.";

const APRES_LE_BRIGAND: &str =
    "Le brigand détale dans les fourrés, abandonnant ses bêtes et sa bourse.";

const AVANT_LE_BOURREAU: &str = "Au cœur du Marais, sous un gibet chargé de cordes, un bourreau encagoulé aiguise sa hache. Trois bêtes attendent ses ordres. Il sert le Baron pendu, et il ne vous laissera pas passer.";

const APRES_LE_BOURREAU: &str = "Le bourreau s'effondre dans la vase, sa hache brisée.";

/// Ansgar attend à l'étape 2 du Marais (docs/contenu-v1.md, section 15).
const ETAPE_D_ANSGAR: u32 = 2;

const AVANT_ANSGAR: &str = "Au bord d'une mare noire, un vieil homme s'arc-boute sur sa canne. Ce qu'il vient de ferrer surgit de l'eau et se jette sur vous.";

const APRES_ANSGAR: &str = "Le vieil homme reprend son souffle : Ansgar, pêcheur, chassé de sa cabane par ce qui remonte des eaux du Marais. Il vous suit jusqu'au camp et y tend ses lignes.";

// Après la victoire sur le Bourreau (section 13.4).
const LIBERATION_DE_GUNTHER: &str = "Derrière le gibet, un homme enchaîné relève la tête : Gunther, forgeron, que le bourreau gardait pour forger ses chaînes. Libéré, il vous suit jusqu'au camp et y installe sa forge.";

const AVANT_L_INQUISITEUR: &str = "Sur le parvis de l'abbaye, un inquisiteur en robe rouge vous attend, entouré de trois bêtes. Il brûle au nom de l'Abbesse, et il compte bien vous brûler aussi.";

const APRES_L_INQUISITEUR: &str =
    "L'inquisiteur s'effondre sur les dalles, son livre de prières en cendres.";

const AVANT_LE_PORTE_ETENDARD: &str = "Au milieu du col, un soldat couvert de givre tient encore l'étendard d'une armée morte. Trois bêtes sortent de la tempête à son signal.";

const APRES_LE_PORTE_ETENDARD: &str =
    "Le porte-étendard tombe à genoux ; l'étendard gelé se brise dans la neige.";

const AVANT_LE_MARGRAVE: &str = "Au sommet, sur un trône de glace entouré de soldats figés, le Margrave de givre se lève. Apôtre du Démiurge, il a offert son armée au froid. Il n'y aura pas de fuite.";

const EPILOGUE_DU_MARGRAVE: &str = "Le Margrave de givre se brise sur son trône. Le même sceau, une main ouverte, est gravé dans la glace. Trois Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

const AVANT_L_ABBESSE: &str = "Au fond de la crypte, une silhouette sans peau prie devant un autel de chair. L'Abbesse écorchée, Apôtre du Démiurge, se retourne lentement. Il n'y aura pas de fuite.";

const EPILOGUE_DE_L_ABBESSE: &str = "L'Abbesse écorchée s'effondre sur l'autel. Le même sceau, une main ouverte, est gravé dans sa chair. Deux Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

/// Après l'épilogue, le joueur recueille l'âme de l'Apôtre vaincu (section
/// 9.4).
/// Le carnet de traque du codex : une ligne par Apôtre abattu, puis le
/// Démiurge (section 5.6).
const CARNET_DU_BARON: &str = "Baron pendu, abattu dans la Vallée des Brumes.";
const CARNET_DE_L_ABBESSE: &str = "Abbesse écorchée, abattue dans l'Abbaye des Cendres.";
const CARNET_DU_MARGRAVE: &str = "Margrave de givre, abattu dans le Col du Givre-Noir.";
const CARNET_DU_DEMIURGE: &str = "Démiurge, abattu dans l'Éclipse.";

const AME_DU_BARON: &str =
    "L'âme du Baron pendu s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DE_L_ABBESSE: &str =
    "L'âme de l'Abbesse écorchée s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DU_MARGRAVE: &str =
    "L'âme du Margrave de givre s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

/// L'Éclipse et le Démiurge, section 23 de docs/contenu-v1.md (étapes 88 à
/// 90).
const FORGE_DU_BEHERIT: &str = "Malvoise dépose les trois âmes au creux de ses mains. Elles se tordent, se mêlent et durcissent en un œuf de pierre pourpre, où des yeux et une bouche s'ouvrent et se referment. \"Le Béhérit pourpre. Il t'ouvrira la porte du Démiurge, mais il réclamera son dû.\"";

// Tour sans fond, section 25 de docs/contenu-v1.md (étape 96).
const ENTREE_DANS_LA_TOUR: &str = "Au milieu des ruines se dresse le sommet d'une tour engloutie. Ses marches s'enfoncent dans la terre, et nul n'en a trouvé le fond ; plus on descend, plus les bêtes sont terribles.";
const AVANT_LE_GARDIEN: &str = "Les marches débouchent sur un palier plus large que les autres. Au centre, {monstre} garde l'escalier qui s'enfonce plus bas. Il n'y aura pas de fuite.";
const REPOS_DANS_LA_TOUR: &str = "Derrière le gardien, une source claire coule entre les pierres. Vos monstres y boivent et reprennent toutes leurs forces.";

const SACRIFICE: &str = "Le Béhérit pourpre s'ouvre en un cri. {monstre} vous est arraché et disparaît dans une lumière rouge. Le ciel s'assombrit : l'Éclipse commence.";

const AVANT_LE_DEMIURGE: &str = "Sous un soleil noir, la terre s'ouvre sur un abîme de chair. Une silhouette ailée descend lentement : le Démiurge, maître des Apôtres. Il n'y aura pas de fuite.";

/// Quand la première forme du Démiurge tombe, à la place de l'arrivée d'un
/// nouvel adversaire.
pub const METAMORPHOSE_DU_DEMIURGE: &str =
    "Le Démiurge s'effondre, puis sa chair se déchire : sa forme véritable s'en arrache, immense.";

const EPILOGUE_DU_DEMIURGE: &str = "Le Démiurge se disloque dans un hurlement qui fait trembler le ciel. Le soleil noir pâlit, et le jour revient sur le royaume. Votre vengeance est accomplie ; mais d'autres choses rôdent encore au bord des bois.";

/// Les Philtres chromatiques ramassés après l'épilogue du Démiurge.
pub const RECOMPENSE_DU_DEMIURGE: &str = "Dans les cendres du Démiurge reposent trois Philtres chromatiques, aux reflets changeants. Vous les ramassez.";

const AVANT_LE_BARON: &str = "Au fond de la Vallée, la brume se déchire sur un gibet dressé contre le ciel. Le Baron pendu, Apôtre du Démiurge, s'y balance encore, puis ouvre les yeux. Il n'y aura pas de fuite.";

const EPILOGUE: &str = "Le Baron pendu s'effondre au pied de son gibet. Sur sa poitrine, un sceau gravé d'une main ouverte : le signe du Démiurge. D'autres Apôtres portent ce sceau, au-delà des terres connues. Votre traque ne fait que commencer.";

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
    plans_d_amelioration: Vec<PlanDAmelioration>,
    plans_de_fabrication: Vec<PlanDeFabrication>,
    plan_du_charme: PlanDuCharme,
    plans_des_filets: Vec<PlanDesFilets>,
    peche: ParametresPeche,
    invocation: Invocation,
    tour: TourSansFond,
    recompenses_du_codex: Vec<RecompenseDuCodex>,
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
        // Au service du Baron pendu, il garde le Marais (section 10.7).
        let bourreau = Humain::new(
            HumainId::new("bourreau"),
            "Bourreau",
            vec![
                (trouver("armure-hantee"), niveau(13)),
                (trouver("feu-follet"), niveau(13)),
                (trouver("basilic"), niveau(14)),
            ],
            150,
            AVANT_LE_BOURREAU,
            APRES_LE_BOURREAU,
        )
        .expect("Bourreau conforme aux règles")
        .avec_ia(IaHumain::Relevee)
        .liberant_gunther(LIBERATION_DE_GUNTHER);
        // Un marchand mort par zone, dont le joueur donne l'âme à Malvoise
        // (section 14.2).
        let marchand = |id: &str, nom: &str, soin, gardien, avant: &str, apres: &str| {
            MarchandMort::new(MarchandId::new(id), nom, soin, gardien, avant, apres)
                .unwrap_or_else(|_| panic!("{nom} conforme aux règles"))
        };
        let bertrand = marchand(
            "bertrand",
            "Bertrand",
            Soin::Remede,
            Gardien::Bete {
                espece: trouver("loup"),
                niveau: niveau(7),
            },
            AVANT_BERTRAND,
            APRES_BERTRAND,
        )
        .a_l_etape(ETAPE_DE_BERTRAND);
        let grete = marchand(
            "grete",
            "Grete",
            Soin::GrandePotion,
            Gardien::MortVivant {
                espece: creer_espece(&MARCHAND_ZOMBIE),
                niveau: niveau(13),
            },
            AVANT_GRETE,
            APRES_GRETE,
        );
        let ulrich = marchand(
            "ulrich",
            "Ulrich",
            Soin::Elixir,
            Gardien::MonstreDeLaZone,
            AVANT_ULRICH,
            APRES_ULRICH,
        );
        // Aux prises avec un Noyé, au niveau maximal du Marais (section
        // 9.2).
        let ansgar = Pecheur::new(
            ETAPE_D_ANSGAR,
            trouver("noye"),
            niveau(13),
            AVANT_ANSGAR,
            APRES_ANSGAR,
        )
        .expect("Ansgar conforme aux règles");
        let lisiere = Zone::new(
            ZoneId::new("lisiere"),
            "Lisière du Bois-Sombre",
            (niveau(3), niveau(7)),
            5,
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
        .map(|zone| {
            zone.avec_humain(brigand)
                .avec_marchand_mort(bertrand)
                .avec_meteos(METEOS_COMMUNES.to_vec())
        })
        .expect("Lisière conforme aux règles");
        let marais = Zone::new(
            ZoneId::new("marais"),
            "Marais des Pendus",
            (niveau(8), niveau(13)),
            5,
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
        .map(|zone| {
            zone.avec_humain(bourreau)
                .avec_marchand_mort(grete)
                .avec_pecheur(ansgar)
                .avec_meteos(METEOS_COMMUNES.to_vec())
        })
        .expect("Marais conforme aux règles");
        let vallee = Zone::new(
            ZoneId::new("vallee"),
            "Vallée des Brumes",
            (niveau(14), niveau(20)),
            5,
            3,
            vec![
                trouver("fee-noire"),
                trouver("squelette"),
                trouver("armure-hantee"),
            ],
            vec![
                EspeceRare {
                    espece: trouver("elfe"),
                    pacifique: true,
                },
                EspeceRare {
                    espece: trouver("vouivre"),
                    pacifique: false,
                },
            ],
            expedition_automatique(3, 90..=220, 700..=1300, 1..=3, 80..=150),
            coffre(50..=90, capsules_de("vallee")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_marchand_mort(ulrich).avec_meteos(vec![
                Meteo::Pluie,
                Meteo::Tempete,
                Meteo::Grele,
            ])
        })
        .expect("Vallée conforme aux règles");
        // Au service de l'Abbesse, il garde l'Abbaye (section 9.4).
        let inquisiteur = Humain::new(
            HumainId::new("inquisiteur"),
            "Inquisiteur",
            vec![
                (trouver("gargouille"), niveau(27)),
                (trouver("goule"), niveau(27)),
                (trouver("chimere"), niveau(28)),
            ],
            300,
            AVANT_L_INQUISITEUR,
            APRES_L_INQUISITEUR,
        )
        .expect("Inquisiteur conforme aux règles")
        .avec_ia(IaHumain::Relevee);
        // Ouverte par l'indice que laisse le Baron pendu (section 9.4).
        let abbaye = Zone::new(
            ZoneId::new("abbaye"),
            "Abbaye des Cendres",
            (niveau(21), niveau(27)),
            5,
            3,
            vec![
                trouver("squelette"),
                trouver("gargouille"),
                trouver("goule"),
                trouver("chauve-souris-de-crypte"),
                trouver("cloche-hurlante"),
            ],
            vec![
                EspeceRare {
                    espece: trouver("chimere"),
                    pacifique: false,
                },
                EspeceRare {
                    espece: trouver("ange-dechu"),
                    pacifique: true,
                },
                EspeceRare {
                    espece: trouver("seraphin-de-vitrail"),
                    pacifique: true,
                },
            ],
            expedition_automatique(4, 150..=320, 1300..=2200, 1..=3, 120..=220),
            coffre(90..=150, capsules_de("abbaye")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(inquisiteur)
                .avec_meteos(METEOS_COMMUNES.to_vec())
        })
        .expect("Abbaye conforme aux règles");
        // Au service du Margrave, il garde le Col (section 9.4).
        let porte_etendard = Humain::new(
            HumainId::new("porte-etendard"),
            "Porte-étendard",
            vec![
                (trouver("wendigo"), niveau(36)),
                (trouver("troll-des-cavernes"), niveau(36)),
                (trouver("wyrm-de-givre"), niveau(37)),
            ],
            400,
            AVANT_LE_PORTE_ETENDARD,
            APRES_LE_PORTE_ETENDARD,
        )
        .expect("Porte-étendard conforme aux règles")
        .avec_ia(IaHumain::Relevee);
        // Ouvert par l'indice que laisse l'Abbesse ; ses neuf niveaux mènent
        // aux stades 3 du niveau 36 (sections 5.4 et 9.4).
        let col = Zone::new(
            ZoneId::new("col-du-givre-noir"),
            "Col du Givre-Noir",
            (niveau(28), niveau(36)),
            5,
            3,
            vec![
                trouver("wendigo"),
                trouver("troll-des-cavernes"),
                trouver("harpie-des-orages"),
                trouver("gargouille"),
                trouver("warg"),
                trouver("chevalier-noir"),
            ],
            vec![
                EspeceRare {
                    espece: trouver("wyrm-de-givre"),
                    pacifique: false,
                },
                EspeceRare {
                    espece: trouver("sylphe-des-neiges"),
                    pacifique: true,
                },
            ],
            expedition_automatique(5, 220..=450, 2200..=3400, 1..=3, 180..=320),
            coffre(140..=220, capsules_de("col-du-givre-noir")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(porte_etendard)
                .avec_meteos(vec![Meteo::Grele, Meteo::Tempete])
        })
        .expect("Col conforme aux règles");
        // Le Baron pendu attend au fond de la Vallée, que le second indice,
        // donné par le Bourreau, ouvre. Premier Apôtre, il combat au niveau
        // maximal de la Vallée, 20, et ne connaît qu'une attaque Obscur
        // (section 9.4, étape 117).
        let baron = Apotre::new(trouver("baron-pendu"), &vallee, AVANT_LE_BARON, EPILOGUE)
            .and_then(|baron| baron.au_niveau_maximal_de_sa_zone(&vallee))
            .expect("Baron pendu conforme aux règles")
            .avec_recit_de_l_ame(AME_DU_BARON)
            .avec_ligne_du_carnet(CARNET_DU_BARON);
        let indice = |zone: &Zone, texte: &str| Indice {
            zone: zone.id().clone(),
            texte: texte.to_string(),
        };
        let chapitre_du_baron = Chapitre::new(
            vec![indice(&lisiere, INDICE_1), indice(&marais, INDICE_2)],
            baron,
        )
        .expect("chapitre du Baron conforme aux règles");
        // Le troisième indice se trouve au bout de l'expédition qui abat le
        // Baron, dans la Vallée, et ouvre l'Abbaye (section 9.4).
        let abbesse = Apotre::new(
            trouver("abbesse-ecorchee"),
            &abbaye,
            AVANT_L_ABBESSE,
            EPILOGUE_DE_L_ABBESSE,
        )
        .expect("Abbesse écorchée conforme aux règles")
        .avec_recit_de_l_ame(AME_DE_L_ABBESSE)
        .avec_ligne_du_carnet(CARNET_DE_L_ABBESSE);
        let chapitre_de_l_abbesse = Chapitre::new(
            vec![indice(&vallee, INDICE_3), indice(&abbaye, INDICE_4)],
            abbesse,
        )
        .expect("chapitre de l'Abbesse conforme aux règles");
        // Le cinquième indice se trouve au bout de l'expédition qui abat
        // l'Abbesse, dans l'Abbaye, et ouvre le Col (section 9.4).
        let margrave = Apotre::new(
            trouver("margrave-de-givre"),
            &col,
            AVANT_LE_MARGRAVE,
            EPILOGUE_DU_MARGRAVE,
        )
        .expect("Margrave de givre conforme aux règles")
        .avec_recit_de_l_ame(AME_DU_MARGRAVE)
        .avec_ligne_du_carnet(CARNET_DU_MARGRAVE);
        let chapitre_du_margrave = Chapitre::new(
            vec![indice(&abbaye, INDICE_5), indice(&col, INDICE_6)],
            margrave,
        )
        .expect("chapitre du Margrave conforme aux règles");
        let piste = Piste::new(vec![
            chapitre_du_baron,
            chapitre_de_l_abbesse,
            chapitre_du_margrave,
        ])
        .expect("piste conforme aux règles");
        let ressource = |id: &str| {
            especes
                .iter()
                .filter_map(Espece::ressource)
                .find(|ressource| ressource.id().as_str() == id)
                .cloned()
                .unwrap_or_else(|| panic!("ressource {id} absente du contenu"))
        };
        let plans_d_amelioration = PLANS_D_AMELIORATION
            .iter()
            .map(|(amelioration, niveau, cout, heures)| {
                let cout = cout
                    .iter()
                    .map(|(id, nombre)| (ressource(id), *nombre))
                    .collect();
                PlanDAmelioration::new(
                    *amelioration,
                    *niveau,
                    cout,
                    Duration::from_secs(heures * 3_600),
                )
                .expect("plan d'amélioration conforme aux règles")
            })
            .collect();
        let plans_de_fabrication = PLANS_DE_FABRICATION
            .iter()
            .map(|fiche| {
                let cout = fiche
                    .cout
                    .iter()
                    .map(|(id, nombre)| (ressource(id), *nombre))
                    .collect();
                PlanDeFabrication::new(
                    creer_objet(fiche),
                    cout,
                    Duration::from_secs(fiche.heures * 3_600),
                )
                .expect("plan de fabrication conforme aux règles")
            })
            .collect();
        // Le Démiurge attend seul dans l'Éclipse, deux niveaux au-dessus de
        // ce lieu d'une étape, comme un Apôtre (section 9.4). Ses paramètres
        // d'expédition automatique et de coffre ne servent jamais.
        let eclipse = Zone::new(
            ZoneId::new("eclipse"),
            "Éclipse",
            (niveau(40), niveau(40)),
            1,
            1,
            vec![trouver("demiurge")],
            Vec::new(),
            expedition_automatique(5, 220..=450, 2200..=3400, 1..=3, 180..=320),
            coffre(140..=220, Vec::new()),
        )
        .expect("Éclipse conforme aux règles");
        let demiurge = Apotre::new(
            trouver("demiurge"),
            &eclipse,
            AVANT_LE_DEMIURGE,
            EPILOGUE_DU_DEMIURGE,
        )
        .and_then(|apotre| apotre.avec_seconde_forme(trouver("demiurge-eveille")))
        .expect("Démiurge conforme aux règles")
        .avec_ligne_du_carnet(CARNET_DU_DEMIURGE);
        let invocation = Invocation::new(eclipse, demiurge, FORGE_DU_BEHERIT, SACRIFICE);
        let zones = vec![lisiere, marais, vallee, abbaye, col];
        let tour = TourSansFond::new(
            ZoneId::new("tour-sans-fond"),
            "Tour sans fond",
            &zones,
            schnoz(),
            ENTREE_DANS_LA_TOUR,
            AVANT_LE_GARDIEN,
            REPOS_DANS_LA_TOUR,
        )
        .expect("Tour sans fond conforme aux règles");
        let peche = ParametresPeche::new(
            Duration::from_secs(HEURES_DE_PECHE * 3_600),
            ESPECES_PECHEES.iter().map(|id| trouver(id)).collect(),
            trouver(ESPECE_PECHEE_RARE),
            &zones,
        )
        .expect("pêche conforme aux règles");
        let plan_du_charme = PlanDuCharme::new(
            COUT_DU_CHARME
                .iter()
                .map(|(id, nombre)| (ressource(id), *nombre))
                .collect(),
            Duration::from_secs(MINUTES_DU_CHARME * 60),
        )
        .expect("plan du charme conforme aux règles");
        let plans_des_filets = PLANS_DES_FILETS
            .iter()
            .map(|(filet, cout, minutes)| {
                PlanDesFilets::new(
                    *filet,
                    FILETS_PAR_LOT,
                    cout.iter()
                        .map(|(id, nombre)| (ressource(id), *nombre))
                        .collect(),
                    Duration::from_secs(minutes * 60),
                )
                .expect("plan des filets conforme aux règles")
            })
            .collect();
        Self {
            especes,
            zones,
            piste,
            capsules,
            plans_d_amelioration,
            plans_de_fabrication,
            plan_du_charme,
            plans_des_filets,
            recompenses_du_codex: creer_recompenses_du_codex(),
            peche,
            invocation,
            tour,
        }
    }

    /// Le lieu hors de la carte où l'on descend le plus loin possible,
    /// seul ou à deux (section 9.5).
    pub fn tour(&self) -> &TourSansFond {
        &self.tour
    }

    /// L'Éclipse et le Démiurge, que le Béhérit pourpre fait venir (section
    /// 9.4).
    pub fn invocation(&self) -> &Invocation {
        &self.invocation
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

    /// Ce que coûte chaque niveau d'amélioration du camp (section 13.4), dans
    /// l'ordre de docs/contenu-v1.md, section 9.
    pub fn plans_d_amelioration(&self) -> &[PlanDAmelioration] {
        &self.plans_d_amelioration
    }

    pub fn plan_d_amelioration(
        &self,
        amelioration: Amelioration,
        niveau: u8,
    ) -> Option<&PlanDAmelioration> {
        self.plans_d_amelioration
            .iter()
            .find(|plan| plan.amelioration() == amelioration && plan.niveau() == niveau)
    }

    /// Les objets d'équipement que Gunther fabrique (section 13.5), dans
    /// l'ordre de docs/contenu-v1.md, section 11.
    pub fn plans_de_fabrication(&self) -> &[PlanDeFabrication] {
        &self.plans_de_fabrication
    }

    /// Le charme de garde, que Gunther forge à l'unité (section 13.5).
    pub fn plan_du_charme(&self) -> &PlanDuCharme {
        &self.plan_du_charme
    }

    /// Les lots de filets que Gunther forge, du plus simple au plus
    /// efficace (section 13.5).
    pub fn plans_des_filets(&self) -> &[PlanDesFilets] {
        &self.plans_des_filets
    }

    /// Dans l'ordre des paliers (section 5.6).
    pub fn recompenses_du_codex(&self) -> &[RecompenseDuCodex] {
        &self.recompenses_du_codex
    }

    pub fn plan_des_filets(&self, filet: Filet) -> Option<&PlanDesFilets> {
        self.plans_des_filets
            .iter()
            .find(|plan| plan.filet() == filet)
    }

    pub fn plan_de_fabrication(&self, objet: &ObjetDEquipementId) -> Option<&PlanDeFabrication> {
        self.plans_de_fabrication
            .iter()
            .find(|plan| plan.objet().id() == objet)
    }

    /// Toutes les capsules d'attaque, que le marchand vend (section 14.2),
    /// dans l'ordre de docs/contenu-v1.md.
    /// Ce que remonte la ligne d'Ansgar (section 13.8).
    pub fn peche(&self) -> &ParametresPeche {
        &self.peche
    }

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

    fn ressource(&self, id: &RessourceId) -> Option<Ressource> {
        self.especes
            .iter()
            .filter_map(Espece::ressource)
            .find(|ressource| ressource.id() == id)
            .cloned()
    }

    /// Un objet fabriqué, ou un objet unique des récompenses du codex.
    fn objet_d_equipement(&self, id: &ObjetDEquipementId) -> Option<ObjetDEquipement> {
        self.plan_de_fabrication(id)
            .map(|plan| plan.objet().clone())
            .or_else(|| {
                self.recompenses_du_codex
                    .iter()
                    .filter_map(|recompense| recompense.objet_unique.as_ref())
                    .find(|objet| objet.id() == id)
                    .cloned()
            })
    }

    fn marchand_mort(&self, id: &MarchandId) -> Option<MarchandMort> {
        self.zones
            .iter()
            .filter_map(Zone::marchand_mort)
            .find(|marchand| marchand.id() == id)
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
const ATTAQUES: [FicheAttaque; 133] = [
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
    // Attaques des monstres de la Vallée des Brumes.
    fiche(
        "bourdonnement",
        "Bourdonnement",
        Type::Insecte,
        Speciale,
        40,
        100,
    ),
    fiche("malefice", "Maléfice", Type::Tenebres, Speciale, 65, 100),
    fiche(
        "poussiere-d-elfe",
        "Poussière d'elfe",
        Type::Feerique,
        Speciale,
        40,
        100,
    ),
    fiche("bourrasque", "Bourrasque", Type::Vol, Speciale, 55, 100),
    fiche(
        "frappe-d-os",
        "Frappe d'os",
        Type::Combat,
        Physique,
        40,
        100,
    ),
    fiche(
        "furie-draconique",
        "Furie draconique",
        Type::Dragon,
        Physique,
        85,
        90,
    ),
    // Attaques Obscur du Baron pendu.
    fiche(
        "etreinte-obscure",
        "Étreinte obscure",
        Type::Obscur,
        Physique,
        80,
        95,
    ),
    avec_effet(
        fiche(
            "hurlement-des-abysses",
            "Hurlement des abysses",
            Type::Obscur,
            Speciale,
            80,
            100,
        ),
        Paralysie,
        10,
    ),
    // Attaques d'évolution (section 5.4).
    fiche("jet-de-lave", "Jet de lave", Type::Feu, Speciale, 85, 90),
    fiche("trombe", "Trombe", Type::Eau, Physique, 85, 90),
    avec_effet(
        fiche(
            "hurlement-de-mandragore",
            "Hurlement de mandragore",
            Type::Plante,
            Speciale,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "griffe-d-ombre",
        "Griffe d'ombre",
        Type::Tenebres,
        Physique,
        80,
        95,
    ),
    fiche("croc-noir", "Croc noir", Type::Tenebres, Physique, 75, 95),
    fiche("bec-funeste", "Bec funeste", Type::Vol, Physique, 75, 95),
    avec_effet(
        fiche("venin-noir", "Venin noir", Type::Poison, Speciale, 75, 100),
        Poison,
        30,
    ),
    fiche("nuee", "Nuée", Type::Insecte, Speciale, 75, 100),
    fiche(
        "flamme-funebre",
        "Flamme funèbre",
        Type::Spectre,
        Speciale,
        80,
        100,
    ),
    fiche(
        "rayon-de-lune",
        "Rayon de lune",
        Type::Feerique,
        Speciale,
        85,
        100,
    ),
    fiche(
        "taille-d-acier",
        "Taille d'acier",
        Type::Acier,
        Physique,
        85,
        90,
    ),
    fiche(
        "epee-des-morts",
        "Épée des morts",
        Type::Combat,
        Physique,
        85,
        95,
    ),
    fiche(
        "regard-de-mort",
        "Regard de mort",
        Type::Poison,
        Speciale,
        85,
        90,
    ),
    // Abbaye des Cendres (étape 45).
    fiche(
        "griffe-de-pierre",
        "Griffe de pierre",
        Type::Roche,
        Physique,
        50,
        100,
    ),
    fiche(
        "chute-de-pierres",
        "Chute de pierres",
        Type::Roche,
        Physique,
        75,
        90,
    ),
    fiche(
        "festin-macabre",
        "Festin macabre",
        Type::Tenebres,
        Physique,
        80,
        90,
    ),
    avec_effet(
        fiche("ultrason", "Ultrason", Type::Normal, Speciale, 70, 100),
        Paralysie,
        10,
    ),
    fiche(
        "onde-psychique",
        "Onde psychique",
        Type::Psychique,
        Speciale,
        65,
        100,
    ),
    fiche("jugement", "Jugement", Type::Psychique, Speciale, 90, 90),
    fiche("bucher", "Bûcher", Type::Feu, Speciale, 90, 85),
    // Attaques Obscur de l'Abbesse écorchée (étape 46).
    fiche(
        "flagellation",
        "Flagellation",
        Type::Obscur,
        Physique,
        85,
        95,
    ),
    avec_effet(
        fiche(
            "psaume-impie",
            "Psaume impie",
            Type::Obscur,
            Speciale,
            90,
            95,
        ),
        Paralysie,
        20,
    ),
    // Attaque d'évolution de la Reine des ronces (étape 47).
    avec_effet(
        fiche(
            "ronces-maudites",
            "Ronces maudites",
            Type::Plante,
            Speciale,
            85,
            90,
        ),
        Poison,
        20,
    ),
    // Attaques du Col du Givre-Noir et du Margrave de givre (étape 79).
    avec_effet(
        fiche(
            "croc-de-givre",
            "Croc de givre",
            Type::Glace,
            Physique,
            60,
            100,
        ),
        Paralysie,
        10,
    ),
    fiche("avalanche", "Avalanche", Type::Glace, Physique, 85, 90),
    fiche(
        "souffle-glacial",
        "Souffle glacial",
        Type::Glace,
        Speciale,
        70,
        100,
    ),
    avec_effet(
        fiche("blizzard", "Blizzard", Type::Glace, Speciale, 90, 85),
        Paralysie,
        10,
    ),
    fiche("secousse", "Secousse", Type::Sol, Physique, 60, 100),
    fiche("seisme", "Séisme", Type::Sol, Physique, 90, 90),
    fiche("eboulement", "Éboulement", Type::Roche, Physique, 90, 85),
    avec_effet(
        fiche(
            "etincelle",
            "Étincelle",
            Type::Electrique,
            Speciale,
            45,
            100,
        ),
        Paralysie,
        10,
    ),
    avec_effet(
        fiche("foudre", "Foudre", Type::Electrique, Speciale, 90, 85),
        Paralysie,
        10,
    ),
    avec_effet(
        fiche(
            "onde-de-choc",
            "Onde de choc",
            Type::Electrique,
            Speciale,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    avec_effet(
        fiche(
            "linceul-de-givre",
            "Linceul de givre",
            Type::Obscur,
            Speciale,
            95,
            90,
        ),
        Paralysie,
        20,
    ),
    // Attaques d'évolution des stades 3 (étape 81).
    fiche(
        "souffle-de-cendre",
        "Souffle de cendre",
        Type::Feu,
        Speciale,
        95,
        90,
    ),
    avec_effet(
        fiche(
            "deluge-toxique",
            "Déluge toxique",
            Type::Poison,
            Physique,
            90,
            90,
        ),
        Poison,
        30,
    ),
    avec_effet(
        fiche(
            "racines-du-gibet",
            "Racines du gibet",
            Type::Spectre,
            Physique,
            85,
            95,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "griffes-spectrales",
        "Griffes spectrales",
        Type::Spectre,
        Physique,
        85,
        95,
    ),
    fiche(
        "dechiquetage",
        "Déchiquetage",
        Type::Combat,
        Physique,
        90,
        90,
    ),
    // Attaques du Démiurge (étape 90).
    avec_effet(
        fiche(
            "marque-du-sacrifice",
            "Marque du sacrifice",
            Type::Obscur,
            Physique,
            95,
            95,
        ),
        Poison,
        30,
    ),
    avec_effet(
        fiche(
            "soleil-noir",
            "Soleil noir",
            Type::Obscur,
            Speciale,
            100,
            90,
        ),
        Paralysie,
        20,
    ),
    // Types Son et Lumière, dans l'Abbaye des Cendres (étape 94).
    fiche("glas", "Glas", Type::Son, Speciale, 50, 100),
    avec_effet(
        fiche("tocsin", "Tocsin", Type::Son, Speciale, 75, 100),
        Paralysie,
        10,
    ),
    avec_effet(
        fiche(
            "carillon-funebre",
            "Carillon funèbre",
            Type::Son,
            Speciale,
            90,
            85,
        ),
        Paralysie,
        10,
    ),
    fiche(
        "lueur-sacree",
        "Lueur sacrée",
        Type::Lumiere,
        Speciale,
        50,
        100,
    ),
    fiche(
        "lance-de-lumiere",
        "Lance de lumière",
        Type::Lumiere,
        Physique,
        80,
        95,
    ),
    fiche(
        "aube-eclatante",
        "Aube éclatante",
        Type::Lumiere,
        Speciale,
        90,
        90,
    ),
    // Attaques d'œuf : elles ne s'obtiennent que par le Rite du sang mêlé
    // (docs/contenu-v1.md, section 28).
    fiche(
        "souffle-des-forges",
        "Souffle des forges",
        Type::Feu,
        Speciale,
        80,
        100,
    ),
    fiche("ressac", "Ressac", Type::Eau, Physique, 80, 100),
    avec_effet(
        fiche(
            "seve-maudite",
            "Sève maudite",
            Type::Plante,
            Speciale,
            75,
            100,
        ),
        Poison,
        20,
    ),
    fiche(
        "griffes-de-minuit",
        "Griffes de minuit",
        Type::Tenebres,
        Physique,
        80,
        100,
    ),
    fiche(
        "assaut-de-meute",
        "Assaut de meute",
        Type::Normal,
        Physique,
        80,
        100,
    ),
    fiche(
        "plongee-funebre",
        "Plongée funèbre",
        Type::Vol,
        Physique,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "etreinte-de-l-aspic",
            "Étreinte de l'aspic",
            Type::Poison,
            Physique,
            75,
            100,
        ),
        Poison,
        30,
    ),
    fiche(
        "lueur-des-marais",
        "Lueur des marais",
        Type::Spectre,
        Speciale,
        80,
        100,
    ),
    fiche(
        "coup-de-heaume",
        "Coup de heaume",
        Type::Acier,
        Physique,
        80,
        100,
    ),
    fiche(
        "essaim-maudit",
        "Essaim maudit",
        Type::Insecte,
        Speciale,
        80,
        100,
    ),
    fiche(
        "danse-macabre",
        "Danse macabre",
        Type::Combat,
        Physique,
        80,
        100,
    ),
    fiche(
        "brame-sacre",
        "Brame sacré",
        Type::Feerique,
        Speciale,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "souffle-du-basilic",
            "Souffle du basilic",
            Type::Dragon,
            Speciale,
            75,
            100,
        ),
        Poison,
        20,
    ),
    fiche(
        "sortilege-sylvestre",
        "Sortilège sylvestre",
        Type::Feerique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "pique-draconique",
        "Piqué draconique",
        Type::Dragon,
        Physique,
        80,
        100,
    ),
    fiche(
        "pluie-d-etoiles",
        "Pluie d'étoiles",
        Type::Stellaire,
        Speciale,
        80,
        100,
    ),
    fiche(
        "plongeon-de-granit",
        "Plongeon de granit",
        Type::Roche,
        Physique,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "griffes-putrides",
            "Griffes putrides",
            Type::Poison,
            Physique,
            75,
            100,
        ),
        Poison,
        30,
    ),
    fiche("vol-nocturne", "Vol nocturne", Type::Vol, Physique, 80, 100),
    fiche(
        "souffle-triple",
        "Souffle triple",
        Type::Dragon,
        Speciale,
        80,
        100,
    ),
    fiche(
        "litanie-des-dechus",
        "Litanie des déchus",
        Type::Psychique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "appel-des-profondeurs",
        "Appel des profondeurs",
        Type::Eau,
        Speciale,
        80,
        100,
    ),
    fiche(
        "morsure-des-tourbieres",
        "Morsure des tourbières",
        Type::Tenebres,
        Physique,
        80,
        100,
    ),
    avec_effet(
        fiche("saignee", "Saignée", Type::Poison, Physique, 75, 100),
        Poison,
        30,
    ),
    fiche(
        "galop-des-eaux",
        "Galop des eaux",
        Type::Eau,
        Physique,
        80,
        100,
    ),
    fiche(
        "griffes-de-famine",
        "Griffes de famine",
        Type::Glace,
        Physique,
        80,
        100,
    ),
    fiche(
        "coup-de-massue",
        "Coup de massue",
        Type::Sol,
        Physique,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "oeil-de-l-orage",
            "Œil de l'orage",
            Type::Electrique,
            Speciale,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "souffle-de-l-hiver",
        "Souffle de l'hiver",
        Type::Glace,
        Speciale,
        80,
        100,
    ),
    fiche(
        "valse-des-neiges",
        "Valse des neiges",
        Type::Feerique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "volee-de-cloches",
        "Volée de cloches",
        Type::Son,
        Speciale,
        80,
        100,
    ),
    fiche(
        "rosace-ardente",
        "Rosace ardente",
        Type::Lumiere,
        Speciale,
        80,
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
const CAPSULES: [FicheCapsule; 15] = [
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
    FicheCapsule {
        attaque: "malefice",
        prix: 300,
        zone: "vallee",
    },
    FicheCapsule {
        attaque: "bourrasque",
        prix: 300,
        zone: "vallee",
    },
    FicheCapsule {
        attaque: "furie-draconique",
        prix: 400,
        zone: "vallee",
    },
    FicheCapsule {
        attaque: "bucher",
        prix: 450,
        zone: "abbaye",
    },
    FicheCapsule {
        attaque: "festin-macabre",
        prix: 400,
        zone: "abbaye",
    },
    FicheCapsule {
        attaque: "ultrason",
        prix: 400,
        zone: "abbaye",
    },
    FicheCapsule {
        attaque: "blizzard",
        prix: 500,
        zone: "col-du-givre-noir",
    },
    FicheCapsule {
        attaque: "seisme",
        prix: 450,
        zone: "col-du-givre-noir",
    },
    FicheCapsule {
        attaque: "onde-de-choc",
        prix: 450,
        zone: "col-du-givre-noir",
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
    /// Section 8 de docs/contenu-v1.md.
    evolution: Option<FicheEvolution>,
}

struct FicheEvolution {
    vers: &'static str,
    condition: FicheCondition,
    /// Dans la liste d'apprentissage de l'espèce d'arrivée.
    attaque: &'static str,
}

enum FicheCondition {
    Niveau(u8),
    Pierre(PierreDEvolution),
    /// Identifiant de la zone et niveau.
    Lieu(&'static str, u8),
    /// Identifiant de l'Apôtre et niveau.
    ApotreVaincu(&'static str, u8),
    /// Niveau, de jour ou de nuit (section 9.6).
    Moment(JourOuNuit, u8),
    /// Niveau, sous une météo (section 9.7).
    Meteo(Meteo, u8),
}

const fn par_pierre(
    vers: &'static str,
    pierre: PierreDEvolution,
    attaque: &'static str,
) -> Option<FicheEvolution> {
    Some(FicheEvolution {
        vers,
        condition: FicheCondition::Pierre(pierre),
        attaque,
    })
}

const fn sous_condition(
    vers: &'static str,
    condition: FicheCondition,
    attaque: &'static str,
) -> Option<FicheEvolution> {
    Some(FicheEvolution {
        vers,
        condition,
        attaque,
    })
}

const fn par_niveau(
    vers: &'static str,
    niveau: u8,
    attaque: &'static str,
) -> Option<FicheEvolution> {
    Some(FicheEvolution {
        vers,
        condition: FicheCondition::Niveau(niveau),
        attaque,
    })
}

const fn par_niveau_sous_la_meteo(
    vers: &'static str,
    niveau: u8,
    meteo: Meteo,
    attaque: &'static str,
) -> Option<FicheEvolution> {
    Some(FicheEvolution {
        vers,
        condition: FicheCondition::Meteo(meteo, niveau),
        attaque,
    })
}

const fn par_niveau_au_moment(
    vers: &'static str,
    niveau: u8,
    jour_ou_nuit: JourOuNuit,
    attaque: &'static str,
) -> Option<FicheEvolution> {
    Some(FicheEvolution {
        vers,
        condition: FicheCondition::Moment(jour_ou_nuit, niveau),
        attaque,
    })
}

/// Grete relevée d'entre les morts : hors du codex, elle n'est pas parmi
/// les espèces (docs/contenu-v1.md, section 4).
const MARCHAND_ZOMBIE: FicheEspece = FicheEspece {
    id: "marchand-zombie",
    nom: "Marchand zombie",
    types: &[Type::Spectre, Type::Poison],
    role: Role::Offensif,
    stats_de_base: [60, 70, 60, 40, 55, 35],
    rendement_experience: 95,
    rarete: Rarete::Commune,
    habiletes: &[],
    apprentissage: &[
        (1, "coup-de-garde"),
        (1, "morsure-venimeuse"),
        (1, "charge-spectrale"),
    ],
    evolution: None,
};

/// Section 2 de docs/contenu-v1.md, dans l'ordre du codex : les monstres de
/// départ, les communs, les rares, l'Apôtre, le monstre caché, puis les
/// formes évoluées de la section 8.
const ESPECES: [FicheEspece; 56] = [
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
        evolution: par_niveau("lezard-de-braise", 16, "jet-de-lave"),
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
        evolution: par_niveau("crapaud-geant", 16, "trombe"),
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
            (7, "secousse"),
            (10, "ronces"),
            (14, "spores-putrides"),
        ],
        evolution: par_niveau("mandragore-ancienne", 16, "hurlement-de-mandragore"),
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
        evolution: par_niveau("chat-des-ombres", 16, "griffe-d-ombre"),
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
        evolution: par_niveau_au_moment("warg", 16, JourOuNuit::Nuit, "croc-noir"),
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
        evolution: par_niveau_au_moment("corbeau-des-gibets", 16, JourOuNuit::Nuit, "bec-funeste"),
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
        evolution: par_niveau_au_moment("aspic", 16, JourOuNuit::Jour, "venin-noir"),
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
        evolution: par_pierre(
            "lanterne-des-morts",
            PierreDEvolution::Braise,
            "flamme-funebre",
        ),
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
        evolution: sous_condition(
            "chevalier-noir",
            FicheCondition::Lieu("vallee", 18),
            "taille-d-acier",
        ),
    },
    FicheEspece {
        id: "fee-noire",
        nom: "Fée noire",
        types: &[Type::Insecte, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [40, 35, 40, 65, 50, 75],
        rendement_experience: 72,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Celerite],
        apprentissage: &[(1, "bourdonnement"), (4, "regard-funeste"), (8, "malefice")],
        evolution: par_niveau_sous_la_meteo("fee-sorciere", 12, Meteo::Tempete, "nuee"),
    },
    FicheEspece {
        id: "squelette",
        nom: "Squelette",
        types: &[Type::Spectre, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [50, 80, 60, 30, 45, 55],
        rendement_experience: 92,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "frappe-d-os"),
            (8, "lame-rouillee"),
            (11, "charge-spectrale"),
        ],
        evolution: sous_condition(
            "chevalier-squelette",
            FicheCondition::ApotreVaincu("baron-pendu", 20),
            "epee-des-morts",
        ),
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
        evolution: par_pierre("cerf-de-lune", PierreDEvolution::Lune, "rayon-de-lune"),
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
        evolution: par_niveau_sous_la_meteo("basilic-royal", 22, Meteo::Canicule, "regard-de-mort"),
    },
    FicheEspece {
        id: "elfe",
        nom: "Elfe",
        types: &[Type::Feerique, Type::Vol],
        role: Role::Soutien,
        stats_de_base: [55, 40, 50, 85, 80, 90],
        rendement_experience: 150,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[
            (1, "poussiere-d-elfe"),
            (5, "bourrasque"),
            (9, "lumiere-d-aube"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "vouivre",
        nom: "Vouivre",
        types: &[Type::Dragon, Type::Vol],
        role: Role::Offensif,
        stats_de_base: [70, 80, 60, 80, 55, 55],
        rendement_experience: 170,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "souffle-draconique"),
            (8, "pique"),
            (12, "furie-draconique"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "baron-pendu",
        nom: "Baron pendu",
        types: &[Type::Obscur],
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
            (18, "etreinte-obscure"),
            (21, "hurlement-des-abysses"),
        ],
        evolution: None,
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
        evolution: None,
    },
    // Premières évolutions par niveau, section 8 de docs/contenu-v1.md.
    FicheEspece {
        id: "lezard-de-braise",
        nom: "Lézard de braise",
        types: &[Type::Feu],
        role: Role::Offensif,
        stats_de_base: [60, 50, 55, 92, 70, 83],
        rendement_experience: 86,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Affinite, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "souffle-chaud"),
            (3, "crachat-de-braises"),
            (7, "souffle-draconique"),
            (10, "flamme-vive"),
            (14, "fournaise"),
            (16, "jet-de-lave"),
            (16, "souffle-putride"),
        ],
        evolution: par_niveau("wyverne-de-cendre", 36, "souffle-de-cendre"),
    },
    FicheEspece {
        id: "crapaud-geant",
        nom: "Crapaud géant",
        types: &[Type::Eau],
        role: Role::Offensif,
        stats_de_base: [72, 88, 70, 45, 58, 67],
        rendement_experience: 88,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "charge-gluante"),
            (3, "plongeon"),
            (7, "coup-de-langue"),
            (10, "langue-venimeuse"),
            (14, "lame-de-fond"),
            (16, "trombe"),
            (16, "deferlante"),
        ],
        evolution: par_niveau("crapaud-roi", 36, "deluge-toxique"),
    },
    FicheEspece {
        id: "mandragore-ancienne",
        nom: "Mandragore ancienne",
        types: &[Type::Plante],
        role: Role::Soutien,
        stats_de_base: [90, 58, 90, 58, 84, 30],
        rendement_experience: 89,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-racine"),
            (3, "feuille-tranchante"),
            (7, "secousse"),
            (10, "ronces"),
            (14, "spores-putrides"),
            (16, "hurlement-de-mandragore"),
            (16, "lianes-etrangleuses"),
        ],
        evolution: par_niveau("arbre-pendu", 36, "racines-du-gibet"),
    },
    FicheEspece {
        id: "chat-des-ombres",
        nom: "Chat des ombres",
        types: &[Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [52, 92, 45, 50, 48, 113],
        rendement_experience: 86,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Traqueur],
        apprentissage: &[
            (1, "griffure"),
            (3, "coup-sournois"),
            (7, "feulement"),
            (10, "morsure"),
            (14, "coup-de-grace"),
            (16, "griffe-d-ombre"),
            (16, "malefice"),
        ],
        evolution: par_niveau("cait-sith", 36, "griffes-spectrales"),
    },
    FicheEspece {
        id: "warg",
        nom: "Warg",
        types: &[Type::Normal, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [65, 80, 58, 38, 52, 75],
        rendement_experience: 98,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Traqueur, Habilete::Fouineur],
        apprentissage: &[
            (1, "griffure"),
            (4, "morsure"),
            (9, "curee"),
            (16, "croc-noir"),
            (16, "coup-sournois"),
        ],
        evolution: par_niveau("loup-garou", 36, "dechiquetage"),
    },
    FicheEspece {
        id: "corbeau-des-gibets",
        nom: "Corbeau des gibets",
        types: &[Type::Vol, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [55, 72, 48, 58, 48, 90],
        rendement_experience: 92,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Pillard, Habilete::Eclaireur],
        apprentissage: &[
            (1, "coup-de-bec"),
            (4, "pique"),
            (8, "regard-funeste"),
            (16, "bec-funeste"),
            (16, "bourrasque"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "aspic",
        nom: "Aspic",
        types: &[Type::Poison],
        role: Role::Offensif,
        stats_de_base: [52, 64, 52, 64, 58, 80],
        rendement_experience: 95,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Instinct],
        apprentissage: &[
            (1, "morsure-venimeuse"),
            (3, "regard-petrifiant"),
            (9, "etreinte"),
            (16, "venin-noir"),
            (16, "croc-de-venin"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "fee-sorciere",
        nom: "Fée-sorcière",
        types: &[Type::Insecte, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [50, 44, 50, 84, 64, 96],
        rendement_experience: 100,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Celerite],
        apprentissage: &[
            (1, "bourdonnement"),
            (4, "regard-funeste"),
            (8, "malefice"),
            (12, "nuee"),
            (12, "sentence"),
        ],
        evolution: par_niveau_sous_la_meteo(
            "reine-des-ronces",
            25,
            Meteo::Pluie,
            "ronces-maudites",
        ),
    },
    FicheEspece {
        id: "basilic-royal",
        nom: "Basilic royal",
        types: &[Type::Poison, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [90, 88, 80, 92, 75, 70],
        rendement_experience: 238,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "morsure-venimeuse"),
            (8, "regard-petrifiant"),
            (10, "souffle-putride"),
            (13, "croc-de-venin"),
            (22, "regard-de-mort"),
            (22, "furie-draconique"),
        ],
        evolution: None,
    },
    // Évolutions par pierre (étape 39).
    FicheEspece {
        id: "lanterne-des-morts",
        nom: "Lanterne des morts",
        types: &[Type::Feu, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [58, 38, 52, 90, 78, 96],
        rendement_experience: 119,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Eclaireur, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "lueur-trompeuse"),
            (1, "crachat-de-braises"),
            (1, "flamme-funebre"),
            (1, "brasier"),
            (10, "flamme-vive"),
            (12, "frisson-d-outre-tombe"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "cerf-de-lune",
        nom: "Cerf de lune",
        types: &[Type::Normal, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [85, 75, 75, 82, 88, 95],
        rendement_experience: 210,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[
            (1, "coup-de-bois"),
            (1, "rayon-de-lune"),
            (5, "ruade"),
            (8, "lumiere-d-aube"),
        ],
        evolution: None,
    },
    // Évolutions par lieu et par événement (étape 40).
    FicheEspece {
        id: "chevalier-noir",
        nom: "Chevalier noir",
        types: &[Type::Acier, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [75, 95, 105, 38, 62, 38],
        rendement_experience: 133,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "coup-de-garde"),
            (8, "lame-rouillee"),
            (11, "charge-spectrale"),
            (18, "taille-d-acier"),
            (18, "frisson-d-outre-tombe"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "chevalier-squelette",
        nom: "Chevalier squelette",
        types: &[Type::Spectre, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [62, 100, 75, 38, 56, 69],
        rendement_experience: 128,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "frappe-d-os"),
            (8, "lame-rouillee"),
            (11, "charge-spectrale"),
            (20, "epee-des-morts"),
            (20, "coup-de-garde"),
        ],
        evolution: None,
    },
    // Abbaye des Cendres, section 10 de docs/contenu-v1.md (étape 45).
    FicheEspece {
        id: "gargouille",
        nom: "Gargouille",
        types: &[Type::Roche, Type::Vol],
        role: Role::Soutien,
        stats_de_base: [60, 80, 90, 30, 50, 40],
        rendement_experience: 105,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "griffe-de-pierre"),
            (10, "pique"),
            (22, "chute-de-pierres"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "goule",
        nom: "Goule",
        types: &[Type::Poison, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [65, 78, 55, 40, 45, 57],
        rendement_experience: 102,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Instinct, Habilete::Ferocite],
        apprentissage: &[
            (1, "morsure"),
            (1, "morsure-venimeuse"),
            (18, "croc-de-venin"),
            (24, "festin-macabre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "chauve-souris-de-crypte",
        nom: "Chauve-souris de crypte",
        types: &[Type::Vol, Type::Poison],
        role: Role::Offensif,
        stats_de_base: [50, 55, 45, 55, 45, 90],
        rendement_experience: 100,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Eclaireur],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "cri-strident"),
            (8, "morsure-venimeuse"),
            (22, "ultrason"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "chimere",
        nom: "Chimère",
        types: &[Type::Feu, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [75, 80, 65, 85, 60, 55],
        rendement_experience: 185,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "crachat-de-braises"),
            (1, "souffle-draconique"),
            (15, "flamme-vive"),
            (25, "furie-draconique"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "ange-dechu",
        nom: "Ange déchu",
        types: &[Type::Psychique, Type::Vol],
        role: Role::Soutien,
        stats_de_base: [70, 50, 60, 95, 90, 55],
        rendement_experience: 180,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[(1, "onde-psychique"), (12, "bourrasque"), (24, "jugement")],
        evolution: None,
    },
    // Deuxième Apôtre (étape 46).
    FicheEspece {
        id: "abbesse-ecorchee",
        nom: "Abbesse écorchée",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [90, 75, 80, 100, 90, 75],
        rendement_experience: 280,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "lamentation"),
            (8, "malefice"),
            (15, "hurlement-des-abysses"),
            (20, "bucher"),
            (25, "flagellation"),
            (28, "psaume-impie"),
        ],
        evolution: None,
    },
    // Deuxième évolution de la Fée-sorcière (étape 47).
    FicheEspece {
        id: "reine-des-ronces",
        nom: "Reine des ronces",
        types: &[Type::Insecte, Type::Plante],
        role: Role::Offensif,
        stats_de_base: [70, 55, 65, 110, 85, 95],
        rendement_experience: 140,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Celerite],
        apprentissage: &[
            (1, "bourdonnement"),
            (4, "regard-funeste"),
            (8, "malefice"),
            (12, "nuee"),
            (12, "sentence"),
            (25, "ronces-maudites"),
            (25, "lianes-etrangleuses"),
        ],
        evolution: None,
    },
    // Monstres aquatiques, qui ne se trouvent qu'à la pêche, section 15 de
    // docs/contenu-v1.md (étape 62).
    FicheEspece {
        id: "noye",
        nom: "Noyé",
        types: &[Type::Eau, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [60, 55, 55, 70, 60, 40],
        rendement_experience: 95,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "plongeon"),
            (1, "lueur-trompeuse"),
            (12, "deferlante"),
            (18, "frisson-d-outre-tombe"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "anguille-des-tourbieres",
        nom: "Anguille des tourbières",
        types: &[Type::Eau, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [50, 70, 45, 55, 45, 85],
        rendement_experience: 95,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Traqueur],
        apprentissage: &[
            (1, "plongeon"),
            (1, "morsure"),
            (8, "etreinte"),
            (16, "lame-de-fond"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "sangsue-geante",
        nom: "Sangsue géante",
        types: &[Type::Eau, Type::Poison],
        role: Role::Soutien,
        stats_de_base: [75, 55, 70, 40, 65, 35],
        rendement_experience: 92,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Instinct],
        apprentissage: &[
            (1, "charge-gluante"),
            (1, "morsure-venimeuse"),
            (15, "croc-de-venin"),
            (20, "deferlante"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "kelpie",
        nom: "Kelpie",
        types: &[Type::Eau, Type::Feerique],
        role: Role::Offensif,
        stats_de_base: [75, 85, 65, 65, 60, 70],
        rendement_experience: 180,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Ferocite, Habilete::Affinite],
        apprentissage: &[
            (1, "plongeon"),
            (1, "ruade"),
            (15, "lame-de-fond"),
            (25, "trombe"),
        ],
        evolution: None,
    },
    // Col du Givre-Noir et troisième Apôtre, section 21 de docs/contenu-v1.md
    // (étape 79).
    FicheEspece {
        id: "wendigo",
        nom: "Wendigo",
        types: &[Type::Glace, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [60, 85, 50, 45, 50, 70],
        rendement_experience: 110,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Traqueur],
        apprentissage: &[
            (1, "croc-de-givre"),
            (1, "charge-spectrale"),
            (20, "frisson-d-outre-tombe"),
            (30, "avalanche"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "troll-des-cavernes",
        nom: "Troll des cavernes",
        types: &[Type::Sol, Type::Roche],
        role: Role::Soutien,
        stats_de_base: [90, 85, 80, 30, 50, 30],
        rendement_experience: 112,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "griffe-de-pierre"),
            (1, "secousse"),
            (22, "chute-de-pierres"),
            (32, "eboulement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "harpie-des-orages",
        nom: "Harpie des orages",
        types: &[Type::Vol, Type::Electrique],
        role: Role::Offensif,
        stats_de_base: [55, 50, 45, 80, 50, 80],
        rendement_experience: 108,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Eclaireur],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "etincelle"),
            (20, "bourrasque"),
            (30, "foudre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "wyrm-de-givre",
        nom: "Wyrm de givre",
        types: &[Type::Dragon, Type::Glace],
        role: Role::Offensif,
        stats_de_base: [80, 85, 70, 85, 65, 60],
        rendement_experience: 195,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "souffle-draconique"),
            (1, "croc-de-givre"),
            (20, "souffle-glacial"),
            (30, "furie-draconique"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "sylphe-des-neiges",
        nom: "Sylphe des neiges",
        types: &[Type::Glace, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [65, 45, 60, 95, 90, 90],
        rendement_experience: 190,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[
            (1, "poussiere-d-elfe"),
            (1, "souffle-glacial"),
            (15, "lumiere-d-aube"),
            (30, "rayon-de-lune"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "margrave-de-givre",
        nom: "Margrave de givre",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [95, 100, 90, 95, 85, 65],
        rendement_experience: 300,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "charge-spectrale"),
            (10, "taille-d-acier"),
            (18, "etreinte-obscure"),
            (25, "blizzard"),
            (30, "hurlement-des-abysses"),
            (36, "linceul-de-givre"),
        ],
        evolution: None,
    },
    // Stades 3, au niveau 36 (étape 81).
    FicheEspece {
        id: "wyverne-de-cendre",
        nom: "Wyverne de cendre",
        types: &[Type::Feu, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [75, 68, 68, 110, 80, 90],
        rendement_experience: 120,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Affinite, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "souffle-chaud"),
            (3, "crachat-de-braises"),
            (7, "souffle-draconique"),
            (10, "flamme-vive"),
            (14, "fournaise"),
            (16, "jet-de-lave"),
            (16, "souffle-putride"),
            (36, "souffle-de-cendre"),
            (36, "furie-draconique"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "crapaud-roi",
        nom: "Crapaud-roi",
        types: &[Type::Eau, Type::Poison],
        role: Role::Offensif,
        stats_de_base: [90, 105, 85, 55, 70, 75],
        rendement_experience: 123,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "charge-gluante"),
            (3, "plongeon"),
            (7, "coup-de-langue"),
            (10, "langue-venimeuse"),
            (14, "lame-de-fond"),
            (16, "trombe"),
            (16, "deferlante"),
            (36, "deluge-toxique"),
            (36, "croc-de-venin"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "arbre-pendu",
        nom: "Arbre-pendu",
        types: &[Type::Plante, Type::Spectre],
        role: Role::Soutien,
        stats_de_base: [110, 70, 110, 70, 100, 30],
        rendement_experience: 124,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-racine"),
            (3, "feuille-tranchante"),
            (7, "secousse"),
            (10, "ronces"),
            (14, "spores-putrides"),
            (16, "hurlement-de-mandragore"),
            (16, "lianes-etrangleuses"),
            (36, "racines-du-gibet"),
            (36, "frisson-d-outre-tombe"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "cait-sith",
        nom: "Cait sith",
        types: &[Type::Tenebres, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [65, 112, 55, 65, 58, 135],
        rendement_experience: 120,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Traqueur],
        apprentissage: &[
            (1, "griffure"),
            (3, "coup-sournois"),
            (7, "feulement"),
            (10, "morsure"),
            (14, "coup-de-grace"),
            (16, "griffe-d-ombre"),
            (16, "malefice"),
            (36, "griffes-spectrales"),
            (36, "charge-spectrale"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "loup-garou",
        nom: "Loup-garou",
        types: &[Type::Combat, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [85, 110, 75, 45, 65, 100],
        rendement_experience: 137,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Traqueur, Habilete::Fouineur],
        apprentissage: &[
            (1, "griffure"),
            (4, "morsure"),
            (9, "curee"),
            (16, "croc-noir"),
            (16, "coup-sournois"),
            (36, "dechiquetage"),
            (36, "griffe-d-ombre"),
        ],
        evolution: None,
    },
    // Les deux formes du Démiurge, section 23 de docs/contenu-v1.md (étape
    // 90).
    FicheEspece {
        id: "demiurge",
        nom: "Démiurge",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [95, 100, 90, 100, 90, 85],
        rendement_experience: 320,
        rarete: Rarete::Demiurge,
        habiletes: &[Habilete::Ferocite, Habilete::Volonte],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (10, "hurlement-des-abysses"),
            (20, "flagellation"),
            (40, "marque-du-sacrifice"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "demiurge-eveille",
        nom: "Démiurge éveillé",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [110, 115, 100, 120, 100, 95],
        rendement_experience: 380,
        rarete: Rarete::Demiurge,
        habiletes: &[Habilete::Sorcellerie, Habilete::Carapace],
        apprentissage: &[
            (1, "hurlement-des-abysses"),
            (1, "psaume-impie"),
            (1, "marque-du-sacrifice"),
            (1, "soleil-noir"),
        ],
        evolution: None,
    },
    // Types Son et Lumière, dans l'Abbaye des Cendres (étape 94).
    FicheEspece {
        id: "cloche-hurlante",
        nom: "Cloche hurlante",
        types: &[Type::Acier, Type::Son],
        role: Role::Soutien,
        stats_de_base: [65, 50, 80, 65, 75, 25],
        rendement_experience: 108,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-garde"),
            (1, "glas"),
            (16, "tocsin"),
            (24, "carillon-funebre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "seraphin-de-vitrail",
        nom: "Séraphin de vitrail",
        types: &[Type::Lumiere],
        role: Role::Offensif,
        stats_de_base: [65, 50, 60, 105, 80, 60],
        rendement_experience: 180,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "lueur-sacree"),
            (12, "lumiere-d-aube"),
            (18, "lance-de-lumiere"),
            (24, "aube-eclatante"),
        ],
        evolution: None,
    },
];

/// Section 15 de docs/contenu-v1.md : la ligne est prête 2 h après avoir
/// été lancée.
const HEURES_DE_PECHE: u64 = 2;
const ESPECES_PECHEES: [&str; 3] = ["noye", "anguille-des-tourbieres", "sangsue-geante"];
const ESPECE_PECHEE_RARE: &str = "kelpie";

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
    let espece = Espece::new(
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
    .avec_habiletes(fiche.habiletes.to_vec());
    let espece = match &fiche.evolution {
        Some(evolution) => espece.avec_evolution(creer_evolution(evolution)),
        None => espece,
    };
    let base = base_de_la_famille(fiche.id);
    let espece = match ATTAQUES_D_OEUF.iter().find(|(espece, _)| *espece == base) {
        Some((_, attaque)) => espece.avec_attaque_d_oeuf(creer_attaque(attaque)),
        None => espece,
    };
    let espece = match ACTIVITES.iter().find(|(espece, _)| *espece == fiche.id) {
        Some((_, activite)) => espece.avec_activite(*activite),
        None => espece,
    };
    match RESSOURCES.iter().find(|(espece, _, _)| *espece == fiche.id) {
        Some((_, id, nom)) => {
            let prix = match fiche.rarete {
                Rarete::Rare => PRIX_D_UNE_RESSOURCE_RARE,
                Rarete::Commune | Rarete::Apotre | Rarete::Demiurge => PRIX_D_UNE_RESSOURCE_COMMUNE,
            };
            espece.avec_ressource(Ressource::new(RessourceId::new(*id), *nom, prix))
        }
        None => espece,
    }
}

fn creer_evolution(fiche: &FicheEvolution) -> Evolution {
    let condition = match fiche.condition {
        FicheCondition::Niveau(valeur) => ConditionEvolution::Niveau(niveau(valeur)),
        FicheCondition::Pierre(pierre) => ConditionEvolution::Pierre(pierre),
        FicheCondition::Lieu(zone, valeur) => ConditionEvolution::Lieu {
            zone: ZoneId::new(zone),
            niveau: niveau(valeur),
        },
        FicheCondition::ApotreVaincu(apotre, valeur) => ConditionEvolution::ApotreVaincu {
            apotre: EspeceId::new(apotre),
            niveau: niveau(valeur),
        },
        FicheCondition::Moment(jour_ou_nuit, valeur) => ConditionEvolution::Moment {
            jour_ou_nuit,
            niveau: niveau(valeur),
        },
        FicheCondition::Meteo(meteo, valeur) => ConditionEvolution::Meteo {
            meteo,
            niveau: niveau(valeur),
        },
    };
    Evolution::new(
        EspeceId::new(fiche.vers),
        condition,
        AttaqueId::new(fiche.attaque),
    )
}

/// Section 9 de docs/contenu-v1.md : amélioration, niveau atteint,
/// ressources et durée en heures.
type FichePlan = (Amelioration, u8, &'static [(&'static str, u32)], u64);

/// Section 9 de docs/contenu-v1.md, puis les machines de la section 19.
const PLANS_D_AMELIORATION: [FichePlan; 16] = [
    (
        Amelioration::Enclos,
        1,
        &[
            ("croc-de-loup", 5),
            ("plume-de-corbeau", 5),
            ("ecaille-de-vipere", 5),
        ],
        1,
    ),
    (
        Amelioration::Enclos,
        2,
        &[("cendre-de-feu-follet", 6), ("fragment-d-armure", 6)],
        2,
    ),
    (
        Amelioration::Enclos,
        3,
        &[("aile-de-fee-noire", 6), ("os-de-squelette", 6)],
        4,
    ),
    (
        Amelioration::Filets,
        1,
        &[("croc-de-loup", 4), ("ecaille-de-vipere", 4)],
        1,
    ),
    (
        Amelioration::Filets,
        2,
        &[("cendre-de-feu-follet", 5), ("fragment-d-armure", 5)],
        3,
    ),
    (
        Amelioration::Filets,
        3,
        &[("aile-de-fee-noire", 5), ("os-de-squelette", 5)],
        6,
    ),
    (
        Amelioration::Equipe,
        1,
        &[
            ("croc-de-basilic", 1),
            ("ecaille-de-vouivre", 1),
            ("os-de-squelette", 8),
            ("aile-de-fee-noire", 8),
        ],
        8,
    ),
    (
        Amelioration::Sac,
        1,
        &[("plume-de-corbeau", 6), ("ecaille-de-vipere", 6)],
        1,
    ),
    (
        Amelioration::Sac,
        2,
        &[("cendre-de-feu-follet", 6), ("fragment-d-armure", 6)],
        3,
    ),
    (
        Amelioration::Sac,
        3,
        &[("aile-de-fee-noire", 6), ("os-de-squelette", 6)],
        6,
    ),
    (
        Amelioration::Pieges,
        1,
        &[("croc-de-loup", 5), ("ecaille-de-vipere", 5)],
        1,
    ),
    (
        Amelioration::Pieges,
        2,
        &[("cendre-de-feu-follet", 6), ("fragment-d-armure", 6)],
        3,
    ),
    (
        Amelioration::Pieges,
        3,
        &[
            ("aile-de-fee-noire", 6),
            ("os-de-squelette", 6),
            ("ecaille-de-vouivre", 1),
        ],
        6,
    ),
    (
        Amelioration::Cour,
        1,
        &[("croc-de-loup", 6), ("plume-de-corbeau", 6)],
        2,
    ),
    (
        Amelioration::Cour,
        2,
        &[("fragment-d-armure", 6), ("os-de-squelette", 6)],
        4,
    ),
    (
        Amelioration::Cour,
        3,
        &[("eclat-de-gargouille", 8), ("dent-de-goule", 6)],
        8,
    ),
];

/// Section 20 de docs/contenu-v1.md : le charme de garde, un par travail.
const COUT_DU_CHARME: [(&str, u32); 2] = [("os-de-squelette", 3), ("fragment-d-armure", 3)];
const MINUTES_DU_CHARME: u64 = 30;

/// Section 26 de docs/contenu-v1.md : chaque lot de 5 filets, son coût et
/// sa durée en minutes.
const FILETS_PAR_LOT: u32 = 5;
type FicheDesFilets = (Filet, &'static [(&'static str, u32)], u64);
const PLANS_DES_FILETS: [FicheDesFilets; 3] = [
    (
        Filet::Chanvre,
        &[("croc-de-loup", 3), ("plume-de-corbeau", 3)],
        30,
    ),
    (
        Filet::Leste,
        &[("fragment-d-armure", 3), ("os-de-squelette", 3)],
        60,
    ),
    (
        Filet::Chaines,
        &[("eclat-de-gargouille", 3), ("pierre-de-troll", 2)],
        120,
    ),
];

/// Section 11 de docs/contenu-v1.md. Les bonus sont en pourcentage, dans
/// l'ordre Attaque, Défense, Attaque spéciale, Défense spéciale, Vitesse :
/// l'équipement ne donne pas de PV, que le combat ne majore pas.
struct FicheObjet {
    id: &'static str,
    nom: &'static str,
    sorte: SorteDObjet,
    bonus: [u8; 5],
    cout: &'static [(&'static str, u32)],
    heures: u64,
}

const PLANS_DE_FABRICATION: [FicheObjet; 14] = [
    FicheObjet {
        id: "couteau-de-chasse",
        nom: "Couteau de chasse",
        sorte: SorteDObjet::Arme,
        bonus: [5, 0, 0, 0, 0],
        cout: &[("croc-de-loup", 6), ("ecaille-de-vipere", 4)],
        heures: 1,
    },
    FicheObjet {
        id: "baton-d-if",
        nom: "Bâton d'if",
        sorte: SorteDObjet::Arme,
        bonus: [0, 0, 5, 0, 0],
        cout: &[("plume-de-corbeau", 6), ("ecaille-de-vipere", 4)],
        heures: 1,
    },
    FicheObjet {
        id: "epee-du-pendu",
        nom: "Épée du pendu",
        sorte: SorteDObjet::Arme,
        bonus: [10, 0, 0, 0, 0],
        cout: &[
            ("fragment-d-armure", 6),
            ("os-de-squelette", 6),
            ("croc-de-basilic", 1),
        ],
        heures: 3,
    },
    FicheObjet {
        id: "grimoire-des-brumes",
        nom: "Grimoire des brumes",
        sorte: SorteDObjet::Arme,
        bonus: [0, 0, 10, 0, 0],
        cout: &[
            ("cendre-de-feu-follet", 6),
            ("aile-de-fee-noire", 6),
            ("croc-de-basilic", 1),
        ],
        heures: 3,
    },
    FicheObjet {
        id: "espadon-des-cendres",
        nom: "Espadon des cendres",
        sorte: SorteDObjet::Arme,
        bonus: [15, 0, 0, 0, 0],
        cout: &[
            ("eclat-de-gargouille", 8),
            ("dent-de-goule", 6),
            ("crin-de-chimere", 1),
        ],
        heures: 6,
    },
    FicheObjet {
        id: "encensoir-maudit",
        nom: "Encensoir maudit",
        sorte: SorteDObjet::Arme,
        bonus: [0, 0, 15, 0, 0],
        cout: &[
            ("aile-de-chauve-souris", 8),
            ("dent-de-goule", 6),
            ("crin-de-chimere", 1),
        ],
        heures: 6,
    },
    FicheObjet {
        id: "brigandine-de-cuir",
        nom: "Brigandine de cuir",
        sorte: SorteDObjet::Armure,
        bonus: [0, 5, 0, 5, 0],
        cout: &[("croc-de-loup", 6), ("plume-de-corbeau", 6)],
        heures: 2,
    },
    FicheObjet {
        id: "cotte-de-mailles",
        nom: "Cotte de mailles",
        sorte: SorteDObjet::Armure,
        bonus: [0, 10, 0, 10, 0],
        cout: &[
            ("fragment-d-armure", 8),
            ("os-de-squelette", 4),
            ("ecaille-de-vouivre", 1),
        ],
        heures: 4,
    },
    FicheObjet {
        id: "plates-de-gargouille",
        nom: "Plates de gargouille",
        sorte: SorteDObjet::Armure,
        bonus: [0, 15, 0, 15, 0],
        cout: &[
            ("eclat-de-gargouille", 10),
            ("os-de-squelette", 6),
            ("crin-de-chimere", 1),
        ],
        heures: 8,
    },
    FicheObjet {
        id: "croc-tresse",
        nom: "Croc tressé",
        sorte: SorteDObjet::Talisman,
        bonus: [5, 0, 0, 0, 0],
        cout: &[("croc-de-loup", 8)],
        heures: 1,
    },
    FicheObjet {
        id: "plume-de-corbeau-sertie",
        nom: "Plume de corbeau sertie",
        sorte: SorteDObjet::Talisman,
        bonus: [0, 0, 0, 0, 5],
        cout: &[("plume-de-corbeau", 8)],
        heures: 1,
    },
    FicheObjet {
        id: "lanterne-de-feu-follet",
        nom: "Lanterne de feu follet",
        sorte: SorteDObjet::Talisman,
        bonus: [0, 0, 5, 0, 0],
        cout: &[("cendre-de-feu-follet", 8)],
        heures: 2,
    },
    FicheObjet {
        id: "os-grave",
        nom: "Os gravé",
        sorte: SorteDObjet::Talisman,
        bonus: [0, 5, 0, 0, 0],
        cout: &[("os-de-squelette", 8)],
        heures: 2,
    },
    FicheObjet {
        id: "aile-de-fee-sertie",
        nom: "Aile de fée sertie",
        sorte: SorteDObjet::Talisman,
        bonus: [0, 0, 0, 5, 0],
        cout: &[("aile-de-fee-noire", 8)],
        heures: 2,
    },
];

/// Section 30 de docs/contenu-v1.md : la récompense du codex complet, qui ne
/// se fabrique pas.
const SCEAU_DU_BESTIAIRE: FicheObjet = FicheObjet {
    id: "sceau-du-bestiaire",
    nom: "Sceau du bestiaire",
    sorte: SorteDObjet::Talisman,
    bonus: [5, 5, 5, 5, 5],
    cout: &[],
    heures: 0,
};

/// Section 30 de docs/contenu-v1.md : une récompense toutes les 10 espèces
/// capturées, puis au codex complet.
fn creer_recompenses_du_codex() -> Vec<RecompenseDuCodex> {
    let recompense = |palier, objets| RecompenseDuCodex {
        palier,
        objets,
        objet_unique: None,
    };
    let boosters = Booster::TOUS
        .into_iter()
        .fold(Inventaire::default(), |objets, booster| {
            objets.avec_boosters(booster, 1)
        });
    vec![
        recompense(
            PalierDuCodex::Especes(10),
            Inventaire::new(300, 0, Vec::new()).avec_filets(Filet::Leste, 10),
        ),
        recompense(
            PalierDuCodex::Especes(20),
            Inventaire::new(500, 0, Vec::new()).avec_filets(Filet::Chaines, 5),
        ),
        recompense(
            PalierDuCodex::Especes(30),
            Inventaire::default().avec_philtres_chromatiques(1),
        ),
        recompense(
            PalierDuCodex::Especes(40),
            Inventaire::default().avec_capsules_d_habilete(3),
        ),
        recompense(PalierDuCodex::Especes(50), boosters),
        RecompenseDuCodex {
            palier: PalierDuCodex::Complet,
            objets: Inventaire::default().avec_philtres_chromatiques(3),
            objet_unique: Some(creer_objet(&SCEAU_DU_BESTIAIRE)),
        },
    ]
}

fn creer_objet(fiche: &FicheObjet) -> ObjetDEquipement {
    let [
        attaque,
        defense,
        attaque_speciale,
        defense_speciale,
        vitesse,
    ] = fiche.bonus;
    ObjetDEquipement::new(
        ObjetDEquipementId::new(fiche.id),
        fiche.nom,
        fiche.sorte,
        Stats {
            pv: 0,
            attaque,
            defense,
            attaque_speciale,
            defense_speciale,
            vitesse,
        },
    )
}

/// Ce que Malvoise donne d'une ressource, selon la rareté de son espèce
/// (section 14.3, Q82).
const PRIX_D_UNE_RESSOURCE_COMMUNE: u32 = 5;
const PRIX_D_UNE_RESSOURCE_RARE: u32 = 15;

/// Espèce, identifiant et nom de chaque ressource : seules les espèces
/// sauvages qui se combattent en ont une (section 14.3).
const RESSOURCES: [(&str, &str, &str); 20] = [
    ("loup", "croc-de-loup", "Croc de loup"),
    ("corbeau", "plume-de-corbeau", "Plume de corbeau"),
    ("vipere", "ecaille-de-vipere", "Écaille de vipère"),
    ("feu-follet", "cendre-de-feu-follet", "Cendre de feu follet"),
    ("armure-hantee", "fragment-d-armure", "Fragment d'armure"),
    ("fee-noire", "aile-de-fee-noire", "Aile de fée noire"),
    ("squelette", "os-de-squelette", "Os de squelette"),
    ("basilic", "croc-de-basilic", "Croc de basilic"),
    ("vouivre", "ecaille-de-vouivre", "Écaille de vouivre"),
    ("gargouille", "eclat-de-gargouille", "Éclat de gargouille"),
    ("goule", "dent-de-goule", "Dent de goule"),
    (
        "chauve-souris-de-crypte",
        "aile-de-chauve-souris",
        "Aile de chauve-souris",
    ),
    ("chimere", "crin-de-chimere", "Crin de chimère"),
    ("wendigo", "poil-de-wendigo", "Poil de wendigo"),
    ("troll-des-cavernes", "pierre-de-troll", "Pierre de troll"),
    ("harpie-des-orages", "plume-de-harpie", "Plume de harpie"),
    ("wyrm-de-givre", "ecaille-de-wyrm", "Écaille de wyrm"),
    // Formes évoluées sauvages du Col : la ressource de leur forme de base
    // (section 5.4).
    ("warg", "croc-de-loup", "Croc de loup"),
    ("chevalier-noir", "fragment-d-armure", "Fragment d'armure"),
    ("cloche-hurlante", "eclat-de-bronze", "Éclat de bronze"),
];

/// Section 32 de docs/contenu-v1.md : les météos de la Lisière, du Marais
/// et de l'Abbaye (section 9.7). Le cyclone et la tempête de sable attendent
/// une zone de mer et un désert.
const METEOS_COMMUNES: [Meteo; 3] = [Meteo::Pluie, Meteo::Tempete, Meteo::Canicule];

/// Section 31 de docs/contenu-v1.md : les espèces qui ne sortent, dans les
/// groupes sauvages des zones, que la nuit ou que le jour (section 9.6).
/// Chaque zone en garde au moins deux à chaque moment.
const ACTIVITES: [(&str, Activite); 11] = [
    ("loup", Activite::Nocturne),
    ("feu-follet", Activite::Nocturne),
    ("fee-noire", Activite::Nocturne),
    ("goule", Activite::Nocturne),
    ("chauve-souris-de-crypte", Activite::Nocturne),
    ("troll-des-cavernes", Activite::Nocturne),
    ("wendigo", Activite::Nocturne),
    ("warg", Activite::Nocturne),
    ("vipere", Activite::Diurne),
    ("cloche-hurlante", Activite::Diurne),
    ("harpie-des-orages", Activite::Diurne),
];

/// Section 28 de docs/contenu-v1.md : l'attaque d'œuf de chaque famille
/// d'évolution, par sa forme de base (section 13.11).
const ATTAQUES_D_OEUF: [(&str, &str); 32] = [
    ("salamandre", "souffle-des-forges"),
    ("crapaud", "ressac"),
    ("mandragore", "seve-maudite"),
    ("chat-noir", "griffes-de-minuit"),
    ("loup", "assaut-de-meute"),
    ("corbeau", "plongee-funebre"),
    ("vipere", "etreinte-de-l-aspic"),
    ("feu-follet", "lueur-des-marais"),
    ("armure-hantee", "coup-de-heaume"),
    ("fee-noire", "essaim-maudit"),
    ("squelette", "danse-macabre"),
    ("cerf-blanc", "brame-sacre"),
    ("basilic", "souffle-du-basilic"),
    ("elfe", "sortilege-sylvestre"),
    ("vouivre", "pique-draconique"),
    ("schnoz", "pluie-d-etoiles"),
    ("gargouille", "plongeon-de-granit"),
    ("goule", "griffes-putrides"),
    ("chauve-souris-de-crypte", "vol-nocturne"),
    ("chimere", "souffle-triple"),
    ("ange-dechu", "litanie-des-dechus"),
    ("noye", "appel-des-profondeurs"),
    ("anguille-des-tourbieres", "morsure-des-tourbieres"),
    ("sangsue-geante", "saignee"),
    ("kelpie", "galop-des-eaux"),
    ("wendigo", "griffes-de-famine"),
    ("troll-des-cavernes", "coup-de-massue"),
    ("harpie-des-orages", "oeil-de-l-orage"),
    ("wyrm-de-givre", "souffle-de-l-hiver"),
    ("sylphe-des-neiges", "valse-des-neiges"),
    ("cloche-hurlante", "volee-de-cloches"),
    ("seraphin-de-vitrail", "rosace-ardente"),
];

/// La forme de base de la famille de cette espèce : celle qu'aucune autre
/// ne précède (section 5.4).
fn base_de_la_famille(id: &str) -> &str {
    let mut forme = id;
    while let Some(precedente) = ESPECES.iter().find(|fiche| {
        fiche
            .evolution
            .as_ref()
            .is_some_and(|evolution| evolution.vers == forme)
    }) {
        forme = precedente.id;
    }
    forme
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

    #[test]
    fn chaque_espece_sauvage_combattue_a_sa_ressource() {
        let contenu = ContenuV1::new();
        for (espece, id, _) in RESSOURCES {
            let ressource = contenu.espece(&EspeceId::new(espece)).unwrap();
            let ressource = ressource.ressource().expect("ressource attendue");
            assert_eq!(ressource.id().as_str(), id);
            assert_eq!(contenu.ressource(ressource.id()).as_ref(), Some(ressource));
        }
        let prix = |espece: &str| {
            let espece = contenu.espece(&EspeceId::new(espece)).unwrap();
            espece.ressource().map(Ressource::prix)
        };
        assert_eq!(prix("loup"), Some(5));
        assert_eq!(prix("vouivre"), Some(15));
        // Ni monstre de départ, ni espèce pacifique, ni Apôtre.
        for sans in ["salamandre", "cerf-blanc", "schnoz", "baron-pendu"] {
            assert_eq!(prix(sans), None, "{sans}");
        }
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
                .chances_de_reussite(
                    &monstres,
                    crate::expedition_automatique::Protection::default(),
                )
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
            let d_oeuf = ATTAQUES_D_OEUF
                .iter()
                .any(|&(_, attaque)| attaque == fiche.id);
            assert!(
                apprise || en_capsule || d_oeuf,
                "attaque {} apprise par aucune espèce ni capsule, ni d'œuf",
                fiche.id
            );
        }
    }

    #[test]
    fn chaque_famille_a_son_attaque_d_oeuf_de_son_type_et_d_elle_seule() {
        let contenu = ContenuV1::new();
        let especes = contenu.especes();
        let mut vues = Vec::new();
        for espece in especes {
            if matches!(espece.rarete(), Rarete::Apotre | Rarete::Demiurge) {
                assert!(espece.attaque_d_oeuf().is_none(), "{}", espece.nom());
                continue;
            }
            let base = crate::rite::forme_de_base(espece, especes);
            let attaque = espece.attaque_d_oeuf().expect("attaque d'œuf");
            assert_eq!(base.attaque_d_oeuf(), Some(attaque), "{}", espece.nom());
            assert!(
                base.types().liste().contains(&attaque.type_attaque()),
                "{} : {}",
                base.nom(),
                attaque.nom()
            );
            // Ni par niveau, ni par capsule : par le rite seulement.
            assert!(
                especes
                    .iter()
                    .all(|autre| autre.attaque_apprise(attaque.id()).is_none()),
                "{}",
                attaque.nom()
            );
            assert!(contenu.capsule(attaque.id()).is_none(), "{}", attaque.nom());
            if base.id() == espece.id() {
                vues.push(attaque.id().clone());
            }
        }
        let mut uniques = vues.clone();
        uniques.sort_by(|a, b| a.as_str().cmp(b.as_str()));
        uniques.dedup();
        assert_eq!(vues.len(), 32);
        assert_eq!(uniques.len(), 32);
    }

    #[test]
    fn les_recompenses_du_codex_suivent_les_paliers_et_le_sceau_ne_se_fabrique_pas() {
        use crate::recompenses_du_codex::{PalierDuCodex, especes_au_codex};

        let contenu = ContenuV1::new();
        let au_codex = especes_au_codex(contenu.especes());
        assert_eq!(au_codex, 51);
        let seuils: Vec<u32> = contenu
            .recompenses_du_codex()
            .iter()
            .map(|recompense| recompense.palier.seuil(au_codex))
            .collect();
        assert_eq!(seuils, vec![10, 20, 30, 40, 50, 51]);
        let complet = contenu.recompenses_du_codex().last().unwrap();
        assert_eq!(complet.palier, PalierDuCodex::Complet);
        let sceau = complet.objet_unique.as_ref().unwrap();
        assert!(contenu.plan_de_fabrication(sceau.id()).is_none());
        assert_eq!(contenu.objet_d_equipement(sceau.id()).as_ref(), Some(sceau));
    }

    #[test]
    fn chaque_capsule_sert_a_une_espece_qui_ne_l_apprend_pas() {
        let contenu = ContenuV1::new();
        assert_eq!(contenu.capsules().len(), 15);
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
        assert_eq!(
            ids(&contenu.zones()[2]),
            vec!["malefice", "bourrasque", "furie-draconique"]
        );
        assert_eq!(
            ids(&contenu.zones()[3]),
            vec!["bucher", "festin-macabre", "ultrason"]
        );
        assert_eq!(
            ids(&contenu.zones()[4]),
            vec!["blizzard", "seisme", "onde-de-choc"]
        );
    }

    #[test]
    fn contenu_conforme_aux_regles() {
        let contenu = ContenuV1::new();
        assert_eq!(contenu.especes().len(), 56);
        assert_eq!(contenu.zones().len(), 5);
        assert_eq!(contenu.piste().indices().count(), 6);
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

    /// Section 25 de docs/contenu-v1.md (étape 96).
    #[test]
    fn la_tour_sans_fond_descend_par_les_zones_de_la_carte() {
        let contenu = ContenuV1::new();
        let tour = contenu.tour();
        assert_eq!(tour.id().as_str(), "tour-sans-fond");
        assert_eq!(tour.ouverte_par().as_str(), "lisiere");
        assert!(contenu.zone(tour.id()).is_none());
        let especes = |etape: u32| -> Vec<String> {
            tour.zone_de_l_etape(etape, false, Vec::new())
                .especes()
                .iter()
                .map(|espece| espece.id().as_str().to_string())
                .collect()
        };
        assert_eq!(especes(1), vec!["loup", "corbeau", "vipere"]);
        // Niveau 24 à l'étape 10 : jusqu'à l'Abbaye, sans le Col.
        let etape_10 = especes(10);
        assert!(etape_10.contains(&"cloche-hurlante".to_string()));
        assert!(!etape_10.contains(&"wendigo".to_string()));
        assert_eq!(
            etape_10
                .iter()
                .filter(|id| id.as_str() == "squelette")
                .count(),
            1
        );
        assert!(especes(12).contains(&"warg".to_string()));
        assert!(tour.recit_du_gardien("Basilic").contains("Basilic garde"));
    }

    /// Section 24 de docs/contenu-v1.md (étape 94).
    #[test]
    fn l_abbaye_accueille_les_types_son_et_lumiere() {
        let contenu = ContenuV1::new();
        let abbaye = &contenu.zones()[3];
        let cloche = abbaye
            .especes()
            .iter()
            .find(|espece| espece.id().as_str() == "cloche-hurlante")
            .unwrap();
        assert_eq!(cloche.types().liste(), &[Type::Acier, Type::Son]);
        let seraphin = abbaye
            .rares()
            .iter()
            .find(|rare| rare.espece.id().as_str() == "seraphin-de-vitrail")
            .unwrap();
        assert!(seraphin.pacifique);
        assert_eq!(seraphin.espece.types().liste(), &[Type::Lumiere]);
        assert_eq!(abbaye.rares().len(), 3);
        let bronze = cloche.ressource().unwrap();
        assert_eq!((bronze.nom(), bronze.prix()), ("Éclat de bronze", 5));
        assert_eq!(seraphin.espece.ressource(), None);
    }

    #[test]
    fn la_rarete_suit_la_place_de_l_espece_dans_le_jeu() {
        let contenu = ContenuV1::new();
        // Le Kelpie est le monstre rare de la pêche (section 13.8).
        let rares: Vec<&EspeceId> = contenu
            .zones()
            .iter()
            .flat_map(|zone| zone.rares().iter().chain(zone.monstre_cache()))
            .map(|rare| rare.espece.id())
            .chain([contenu.peche().rare().id()])
            .collect();
        let apotres: Vec<&EspeceId> = contenu
            .piste()
            .apotres()
            .map(|apotre| apotre.espece().id())
            .collect();
        let demiurge = contenu.invocation().demiurge();
        let formes_du_demiurge = [demiurge.espece()]
            .into_iter()
            .chain(demiurge.seconde_forme())
            .map(Espece::id)
            .collect::<Vec<_>>();
        for espece in contenu.especes() {
            // Une forme évoluée a la rareté de sa forme de base (section 5.4).
            let base = contenu.especes().iter().find(|base| {
                base.evolution()
                    .is_some_and(|evolution| evolution.vers() == espece.id())
            });
            let attendue = if let Some(base) = base {
                base.rarete()
            } else if apotres.contains(&espece.id()) {
                Rarete::Apotre
            } else if formes_du_demiurge.contains(&espece.id()) {
                Rarete::Demiurge
            } else if rares.contains(&espece.id()) {
                Rarete::Rare
            } else {
                Rarete::Commune
            };
            assert_eq!(espece.rarete(), attendue, "{}", espece.id().as_str());
        }
    }

    #[test]
    fn chaque_niveau_d_amelioration_a_son_plan() {
        let contenu = ContenuV1::new();
        for amelioration in Amelioration::TOUTES {
            for niveau in 1..=amelioration.niveau_max() {
                let plan = contenu
                    .plan_d_amelioration(amelioration, niveau)
                    .unwrap_or_else(|| panic!("{} {niveau} sans plan", amelioration.nom()));
                // Seules des ressources de monstres sauvages, que Malvoise
                // rachète, se dépensent (section 13.4).
                for (ressource, _) in plan.cout() {
                    assert_eq!(contenu.ressource(ressource.id()).as_ref(), Some(ressource));
                }
            }
        }
        assert_eq!(contenu.plans_d_amelioration().len(), 16);
    }

    #[test]
    fn les_objets_d_equipement_suivent_la_section_11() {
        let contenu = ContenuV1::new();
        let plans = contenu.plans_de_fabrication();
        let par_sorte = |sorte| {
            plans
                .iter()
                .filter(|plan| plan.objet().sorte() == sorte)
                .count()
        };
        assert_eq!(
            (
                par_sorte(SorteDObjet::Arme),
                par_sorte(SorteDObjet::Armure),
                par_sorte(SorteDObjet::Talisman)
            ),
            (6, 3, 5)
        );
        let ids: std::collections::HashSet<_> =
            plans.iter().map(|plan| plan.objet().id()).collect();
        assert_eq!(ids.len(), plans.len());
        for plan in plans {
            let objet = plan.objet();
            // Pas de PV : le combat ne majore pas les PV (section 4.3).
            assert_eq!(objet.bonus().pv, 0, "{}", objet.nom());
            assert_ne!(objet.bonus(), Stats::default(), "{}", objet.nom());
            let heures = plan.duree().as_secs() / 3_600;
            assert!((1..=8).contains(&heures), "{}", objet.nom());
            for (ressource, _) in plan.cout() {
                assert_eq!(contenu.ressource(ressource.id()).as_ref(), Some(ressource));
            }
            assert_eq!(contenu.objet_d_equipement(objet.id()).as_ref(), Some(objet));
        }
        let couteau = contenu
            .plan_de_fabrication(&ObjetDEquipementId::new("couteau-de-chasse"))
            .unwrap();
        assert_eq!(couteau.objet().bonus().attaque, 5);
        assert_eq!(couteau.cout().len(), 2);
    }

    #[test]
    fn chaque_evolution_mene_a_une_forme_conforme() {
        let contenu = ContenuV1::new();
        let mut evoluees = 0;
        let mut sauvages = Vec::new();
        for base in contenu.especes() {
            let Some(evolution) = base.evolution() else {
                continue;
            };
            evoluees += 1;
            let vers = contenu
                .espece(evolution.vers())
                .unwrap_or_else(|| panic!("évolution de {} absente", base.id().as_str()));
            let nom = vers.id().as_str();
            assert!(
                vers.attaque_apprise(evolution.attaque()).is_some(),
                "{nom} n'apprend pas son attaque d'évolution"
            );
            for apprise in base.apprentissage() {
                assert!(
                    vers.apprentissage().contains(apprise),
                    "{nom} n'apprend pas {}",
                    apprise.attaque.id().as_str()
                );
            }
            assert_eq!(vers.habiletes(), base.habiletes(), "{nom}");
            assert_eq!(vers.role(), base.role(), "{nom}");
            // Seules les formes évoluées sauvages du Col lâchent une
            // ressource, celle de leur forme de base (section 5.4).
            let sauvage =
                |zone: &Zone| zone.especes().iter().any(|espece| espece.id() == vers.id());
            let zones_sauvages: Vec<&str> = contenu
                .zones()
                .iter()
                .filter(|zone| sauvage(zone))
                .map(|zone| zone.id().as_str())
                .collect();
            if zones_sauvages.is_empty() {
                assert_eq!(vers.ressource(), None, "{nom}");
            } else {
                assert_eq!(zones_sauvages, vec!["col-du-givre-noir"], "{nom}");
                assert_eq!(vers.ressource(), base.ressource(), "{nom}");
                sauvages.push(nom.to_string());
            }
            assert_eq!(
                vers.rendement_experience(),
                base.rendement_experience() * 14 / 10,
                "{nom}"
            );
            assert!(
                vers.total_des_stats_de_base() > base.total_des_stats_de_base(),
                "{nom}"
            );
        }
        assert_eq!(evoluees, 19);
        assert_eq!(sauvages, vec!["warg", "chevalier-noir"]);
    }

    #[test]
    fn chaque_apotre_laisse_son_ame() {
        let contenu = ContenuV1::new();
        let recits: Vec<&str> = contenu
            .piste()
            .apotres()
            .map(Apotre::recit_de_l_ame)
            .collect();
        assert_eq!(recits.len(), 3);
        assert!(recits[0].starts_with("L'âme du Baron pendu"));
        assert!(recits[1].starts_with("L'âme de l'Abbesse écorchée"));
        assert!(recits[2].starts_with("L'âme du Margrave de givre"));
    }

    #[test]
    fn la_peche_remonte_des_monstres_aquatiques_qui_ne_vivent_nulle_part_ailleurs() {
        let contenu = ContenuV1::new();
        let peche = contenu.peche();
        assert_eq!(peche.duree(), Duration::from_secs(7_200));
        // Ansgar attend à l'étape 2 du Marais, et nulle part ailleurs.
        let etapes: Vec<Option<u32>> = contenu
            .zones()
            .iter()
            .map(|zone| zone.pecheur().map(Pecheur::etape))
            .collect();
        assert_eq!(etapes, vec![None, Some(2), None, None, None]);
        let ids: Vec<&str> = peche
            .especes()
            .iter()
            .chain([peche.rare()])
            .map(|espece| espece.id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec![
                "noye",
                "anguille-des-tourbieres",
                "sangsue-geante",
                "kelpie"
            ]
        );
        for espece in peche.especes().iter().chain([peche.rare()]) {
            assert!(espece.types().liste().contains(&Type::Eau));
            assert!(espece.ressource().is_none());
            assert!(espece.evolution().is_none());
            assert_eq!(contenu.espece(espece.id()).as_ref(), Some(espece));
            for zone in contenu.zones() {
                let ailleurs = zone
                    .especes()
                    .iter()
                    .chain(zone.rares().iter().map(|rare| &rare.espece));
                assert!(ailleurs.into_iter().all(|autre| autre.id() != espece.id()));
            }
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
    fn le_brigand_et_le_bourreau_attendent_au_bout_de_la_lisiere_et_du_marais() {
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
        let bourreau = contenu.zones()[1].humain().unwrap();
        assert_eq!(
            (bourreau.id().as_str(), bourreau.argent()),
            ("bourreau", 150)
        );
        let niveaux: Vec<u8> = bourreau
            .generer_equipe()
            .iter()
            .map(|monstre| monstre.niveau().valeur())
            .collect();
        // Le maximum du Marais et un niveau de plus, comme les humains
        // suivants (étape 117).
        assert_eq!(niveaux, vec![13, 13, 14]);
        assert_eq!(brigand.ia(), IaHumain::Simple);
        assert_eq!(bourreau.ia(), IaHumain::Relevee);
        assert!(contenu.zones()[2].humain().is_none());
    }

    /// Toutes les espèces du Marais résistent à la Plante ou la frappent
    /// fort : la famille de la Mandragore apprend une attaque Sol au niveau
    /// 7 (étape 117).
    #[test]
    fn la_famille_de_la_mandragore_apprend_secousse_au_niveau_7() {
        let contenu = ContenuV1::new();
        for id in ["mandragore", "mandragore-ancienne", "arbre-pendu"] {
            let espece = contenu.espece(&EspeceId::new(id)).unwrap();
            let au_niveau_7: Vec<&str> = espece
                .apprentissage()
                .iter()
                .filter(|apprise| apprise.niveau == niveau(7))
                .map(|apprise| apprise.attaque.id().as_str())
                .collect();
            assert_eq!(au_niveau_7, vec!["secousse"], "{id}");
        }
        let mandragore = creer(&contenu, "mandragore", 7);
        assert!(mandragore.attaques().connait(&AttaqueId::new("secousse")));
    }

    /// Section 31 de docs/contenu-v1.md : chaque zone garde au moins deux
    /// espèces à chaque moment (section 9.6).
    #[test]
    fn chaque_zone_a_ses_especes_de_jour_et_de_nuit() {
        fn trier(mut ids: Vec<&str>) -> Vec<&str> {
            ids.sort_unstable();
            ids
        }
        let contenu = ContenuV1::new();
        let attendu: [(&str, &[&str], &[&str]); 5] = [
            ("lisiere", &["corbeau", "vipere"], &["loup", "corbeau"]),
            (
                "marais",
                &["armure-hantee", "vipere", "corbeau"],
                &["feu-follet", "armure-hantee", "corbeau"],
            ),
            (
                "vallee",
                &["squelette", "armure-hantee"],
                &["fee-noire", "squelette", "armure-hantee"],
            ),
            (
                "abbaye",
                &["squelette", "gargouille", "cloche-hurlante"],
                &[
                    "squelette",
                    "gargouille",
                    "goule",
                    "chauve-souris-de-crypte",
                ],
            ),
            (
                "col-du-givre-noir",
                &["harpie-des-orages", "gargouille", "chevalier-noir"],
                &[
                    "wendigo",
                    "troll-des-cavernes",
                    "gargouille",
                    "warg",
                    "chevalier-noir",
                ],
            ),
        ];
        for (id, de_jour, de_nuit) in attendu {
            let zone = contenu.zone(&ZoneId::new(id)).unwrap();
            let ids = |jour_ou_nuit| -> Vec<&str> {
                zone.especes_qui_sortent(Some(jour_ou_nuit))
                    .iter()
                    .map(|espece| espece.id().as_str())
                    .collect()
            };
            let (jour, nuit) = (ids(JourOuNuit::Jour), ids(JourOuNuit::Nuit));
            assert_eq!(
                trier(jour.clone()),
                trier(de_jour.to_vec()),
                "{id}, de jour"
            );
            assert_eq!(
                trier(nuit.clone()),
                trier(de_nuit.to_vec()),
                "{id}, de nuit"
            );
            assert!(jour.len() >= 2 && nuit.len() >= 2, "{id}");
        }
    }

    /// Section 32 de docs/contenu-v1.md ; le cyclone et la tempête de sable
    /// attendent une zone de mer et un désert (section 9.7).
    #[test]
    fn chaque_zone_a_ses_meteos() {
        use Meteo::*;
        let contenu = ContenuV1::new();
        let attendu: [(&str, &[Meteo]); 5] = [
            ("lisiere", &[Pluie, Tempete, Canicule]),
            ("marais", &[Pluie, Tempete, Canicule]),
            ("vallee", &[Pluie, Tempete, Grele]),
            ("abbaye", &[Pluie, Tempete, Canicule]),
            ("col-du-givre-noir", &[Grele, Tempete]),
        ];
        for (id, meteos) in attendu {
            let zone = contenu.zone(&ZoneId::new(id)).unwrap();
            assert_eq!(zone.meteos(), meteos, "{id}");
        }
        assert!(contenu.zones().iter().all(|zone| {
            !zone.meteos().contains(&Cyclone) && !zone.meteos().contains(&TempeteDeSable)
        }));
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
    fn le_baron_pendu_obscur_attend_au_bout_de_la_vallee_au_niveau_20() {
        let contenu = ContenuV1::new();
        let apotre = contenu.piste().apotres().next().unwrap();
        assert_eq!(apotre.niveau(), niveau(20));
        assert_eq!(apotre.zone().as_str(), "vallee");
        assert_eq!(apotre.espece().types().liste(), &[Type::Obscur]);
        // Les quatre dernières de sa liste jusqu'au niveau 20, dont une seule
        // attaque Obscur : Hurlement des abysses vient au niveau 21.
        let baron = creer(&contenu, "baron-pendu", 20);
        let ids: Vec<&str> = baron
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec![
                "lamentation",
                "sentence",
                "trappe-du-gibet",
                "etreinte-obscure"
            ]
        );
    }

    #[test]
    fn l_abbesse_obscure_attend_au_bout_de_l_abbaye_au_niveau_29() {
        let contenu = ContenuV1::new();
        let abbesse = contenu.piste().apotres().nth(1).unwrap();
        assert_eq!(abbesse.niveau(), niveau(29));
        assert_eq!(abbesse.zone().as_str(), "abbaye");
        assert_eq!(abbesse.espece().types().liste(), &[Type::Obscur]);
        assert_eq!(abbesse.espece().rarete(), Rarete::Apotre);
        let monstre = creer(&contenu, "abbesse-ecorchee", 29);
        let ids: Vec<&str> = monstre
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec![
                "hurlement-des-abysses",
                "bucher",
                "flagellation",
                "psaume-impie"
            ]
        );
        let inquisiteur = contenu.zones()[3].humain().unwrap();
        assert_eq!(inquisiteur.id().as_str(), "inquisiteur");
        assert_eq!(inquisiteur.argent(), 300);
    }

    #[test]
    fn le_margrave_obscur_attend_au_bout_du_col_au_niveau_38() {
        let contenu = ContenuV1::new();
        let margrave = contenu.piste().apotres().nth(2).unwrap();
        assert_eq!(margrave.niveau(), niveau(38));
        assert_eq!(margrave.zone().as_str(), "col-du-givre-noir");
        assert_eq!(margrave.espece().types().liste(), &[Type::Obscur]);
        assert_eq!(margrave.espece().rarete(), Rarete::Apotre);
        let monstre = creer(&contenu, "margrave-de-givre", 38);
        let ids: Vec<&str> = monstre
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec![
                "etreinte-obscure",
                "blizzard",
                "hurlement-des-abysses",
                "linceul-de-givre"
            ]
        );
        let col = &contenu.zones()[4];
        assert_eq!(col.niveaux().1, niveau(36));
        let porte_etendard = col.humain().unwrap();
        assert_eq!(porte_etendard.id().as_str(), "porte-etendard");
        assert_eq!(porte_etendard.argent(), 400);
        // Les stades 3 du niveau 36 s'atteignent dans le Col (section 5.4).
        let wyverne = contenu
            .espece(&EspeceId::new("lezard-de-braise"))
            .and_then(|espece| espece.evolution().cloned())
            .unwrap();
        assert_eq!(wyverne.vers().as_str(), "wyverne-de-cendre");
        assert_eq!(
            wyverne.condition(),
            &ConditionEvolution::Niveau(col.niveaux().1)
        );
    }

    #[test]
    fn chaque_apotre_et_le_demiurge_ont_leur_ligne_au_carnet() {
        let contenu = ContenuV1::new();
        for apotre in contenu
            .piste()
            .apotres()
            .chain([contenu.invocation().demiurge()])
        {
            assert!(
                apotre.ligne_du_carnet().starts_with(apotre.espece().nom()),
                "{}",
                apotre.espece().nom()
            );
        }
    }

    #[test]
    fn le_demiurge_attend_seul_dans_l_eclipse_sous_deux_formes_au_niveau_42() {
        let contenu = ContenuV1::new();
        let invocation = contenu.invocation();
        let eclipse = invocation.eclipse();
        assert!(eclipse.est_un_lieu_d_invocation());
        assert_eq!(eclipse.etapes(), 1);
        assert!(contenu.zones().iter().all(|zone| zone.id() != eclipse.id()));
        let demiurge = invocation.demiurge();
        assert_eq!(demiurge.niveau(), niveau(42));
        assert_eq!(demiurge.espece().id().as_str(), "demiurge");
        assert_eq!(
            demiurge.seconde_forme().map(|espece| espece.id().as_str()),
            Some("demiurge-eveille")
        );
        let attaques = |id: &str| -> Vec<String> {
            creer(&contenu, id, 42)
                .attaques()
                .liste()
                .iter()
                .map(|attaque| attaque.id().as_str().to_string())
                .collect()
        };
        assert_eq!(
            attaques("demiurge"),
            vec![
                "etreinte-obscure",
                "hurlement-des-abysses",
                "flagellation",
                "marque-du-sacrifice"
            ]
        );
        assert_eq!(
            attaques("demiurge-eveille"),
            vec![
                "hurlement-des-abysses",
                "psaume-impie",
                "marque-du-sacrifice",
                "soleil-noir"
            ]
        );
        assert!(
            invocation
                .recit_du_sacrifice("Loup")
                .starts_with("Le Béhérit pourpre s'ouvre en un cri. Loup vous est arraché")
        );
    }

    #[test]
    fn la_piste_commence_dans_la_lisiere() {
        let contenu = ContenuV1::new();
        let zones: Vec<&str> = contenu
            .piste()
            .indices()
            .map(|indice| indice.zone.as_str())
            .collect();
        assert_eq!(
            zones,
            vec![
                "lisiere",
                "marais",
                "vallee",
                "abbaye",
                "abbaye",
                "col-du-givre-noir"
            ]
        );
    }
}
