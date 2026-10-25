//! Format JSON de la sauvegarde, version 23, et sa conversion vers les types
//! du noyau. Les énumérations sont écrites avec des noms fixés ici, et non
//! avec les noms Rust, pour qu'un renommage dans le code ne casse pas les
//! sauvegardes existantes.
//!
//! Chaque structure porte le numéro de la version qui l'a introduite.

use serde::{Deserialize, Serialize};

use bersemon_core::ame::{MarchandId, MarchandMort};
use bersemon_core::amelioration::{Amelioration, Ameliorations, Chantier, Forge, Ouvrage};
use bersemon_core::attaque::{AttaqueId, AttaquesMonstre};
use bersemon_core::beherit::EtatDuBeherit;
use bersemon_core::booster::Booster;
use bersemon_core::catalogue::Catalogue;
use bersemon_core::codex::Codex;
use bersemon_core::collection::{Collection, Emplacement};
use bersemon_core::equipement::{
    Equipement, ObjetDEquipement, ObjetDEquipementId, PlaceDEquipement,
};
use bersemon_core::espece::{EspeceId, Rarete};
use bersemon_core::expedition_automatique::{ExpeditionAutomatique, Protection};
use bersemon_core::filet::Filet;
use bersemon_core::habilete::Habilete;
use bersemon_core::heros::{Classe, Heros};
use bersemon_core::horodatage::Horodatage;
use bersemon_core::humain::HumainId;
use bersemon_core::inventaire::Inventaire;
use bersemon_core::machines::{Entrainement, Machines, Pieges};
use bersemon_core::monstre::{EtatMonstre, Monstre};
use bersemon_core::niveau::Niveau;
use bersemon_core::nom::Nom;
use bersemon_core::partie::{AttaqueAChoisir, EtatPartie, Partie};
use bersemon_core::peche::Peche;
use bersemon_core::pierre::PierreDEvolution;
use bersemon_core::relations::{Palmares, Relations};
use bersemon_core::ressource::RessourceId;
use bersemon_core::rite::Rite;
use bersemon_core::sauvegarde::ErreurSauvegarde;
use bersemon_core::soin::Soin;
use bersemon_core::stats::{Stats, ValeursInternes};
use bersemon_core::statut::Statut;
use bersemon_core::tour::RecordsDeLaTour;
use bersemon_core::tutoriel::Conseil;
use bersemon_core::zone::{Zone, ZoneId};

/// À augmenter à chaque changement du format, avec une conversion depuis
/// les versions précédentes.
pub(crate) const VERSION: u64 = 28;

/// La version de la sauvegarde qui sépare le sac du coffre du camp.
const VERSION_DU_COFFRE: u64 = 7;

/// La version de la sauvegarde qui suit la piste à deux indices (section
/// 9.4).
const VERSION_DE_LA_PISTE_A_DEUX_INDICES: u64 = 10;

/// La version de la sauvegarde où Malvoise remplace le marchand (section
/// 14.2).
const VERSION_DE_MALVOISE: u64 = 11;

/// La version de la sauvegarde où Gunther, le forgeron, rejoint le camp
/// (section 13.4).
const VERSION_DE_GUNTHER: u64 = 14;

/// La version de la sauvegarde où les Apôtres ne se capturent plus : vaincus,
/// ils laissent leur âme (section 9.4).
const VERSION_DES_AMES_D_APOTRES: u64 = 19;

/// La version de la sauvegarde où chaque Apôtre vaincu laisse un Philtre
/// chromatique, alors appelé Éclat d'astre (section 14.3).
const VERSION_DES_PHILTRES_CHROMATIQUES: u64 = 20;

/// La version de la sauvegarde où Gunther construit les machines du camp
/// (section 13.6).
const VERSION_DES_MACHINES: u64 = 22;

/// La version de la sauvegarde où Malvoise loue des monstres (section 14.2).
const VERSION_DES_LOCATIONS: u64 = 24;

/// La version de la sauvegarde où Malvoise forge le Béhérit pourpre (section
/// 9.4).
const VERSION_DU_BEHERIT: u64 = 25;

/// La version de la sauvegarde qui garde les records de la Tour sans fond
/// (section 9.5).
const VERSION_DES_RECORDS: u64 = 26;

/// La version de la sauvegarde où les filets de capture s'épuisent (section
/// 11.2).
const VERSION_DES_FILETS: u64 = 27;

/// La version de la sauvegarde qui garde le Rite du sang mêlé en cours
/// (section 13.11).
const VERSION_DU_RITE: u64 = 28;

/// Ce que reçoit, au coffre, une partie qui reprend une sauvegarde
/// antérieure aux filets (hypothèse retenue, Q102).
const FILETS_DES_SAUVEGARDES_ANTERIEURES: u32 = 10;

/// Gunther est le prisonnier de cet humain : avant la version 14, une partie
/// qui l'a vaincu a Gunther au camp (section 13.4, Q85).
const GEOLIER_DE_GUNTHER: &str = "bourreau";

/// La version 2 ajoute la traque de l'Apôtre et le tutoriel, la version 3
/// les expéditions automatiques et les attaques à choisir à leur retour, la
/// version 4 l'argent, la version 5 le marchand, la version 6 les capsules
/// d'attaque, la version 7 le sac et le coffre du camp, la version 8
/// l'habileté de chaque monstre, la version 9 les humains vaincus ; la
/// version 10 suit la piste à deux indices ; la version 11 ajoute le
/// remède, la grande potion, l'élixir et les âmes recueillies, et Malvoise
/// remplace le marchand ; avec la version 12, une attaque à choisir peut
/// désigner un monstre de l'équipe, et s'ajoutent les âmes portées et à
/// retrouver, les capsules d'habileté et les ressources ; la version 13
/// ajoute les pierres d'évolution, la version 14 Gunther et les
/// améliorations du camp, la version 15 l'amélioration de l'équipe, la
/// version 16 l'équipement du héros et sa fabrication, la version 17 la
/// marque des monstres reçus d'un autre joueur, la version 18 la pêche
/// d'Ansgar ; avec la version 19, les Apôtres capturés deviennent leur âme,
/// sans changer la structure ; la version 20 ajoute les boosters et les
/// Philtres chromatiques, la version 21 les amis et le palmarès des duels, la
/// version 22 les machines du camp, la version 23 les charmes de garde et
/// les protections des expéditions automatiques, la version 24 la location
/// d'un monstre, la version 25 le Béhérit pourpre, la version 26 les records
/// de la Tour sans fond, la version 27 les filets de capture, la version 28
/// le Rite du sang mêlé. Une sauvegarde d'une version précédente se lit avec cette
/// structure : les champs ajoutés y prennent leur valeur de début de
/// partie.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SauvegardeV14 {
    pub(crate) version: u64,
    heros: HerosV1,
    equipe: Vec<MonstreV1>,
    camp: Vec<MonstreV1>,
    /// Jusqu'à la version 6 : tout ce que possédait le joueur, rangé dans le
    /// coffre du camp à la lecture (section 13.3). N'est plus écrit.
    #[serde(default, skip_serializing)]
    potions: u32,
    #[serde(default, skip_serializing)]
    argent: u32,
    #[serde(default, skip_serializing)]
    capsules: Vec<String>,
    #[serde(default)]
    sac: InventaireV7,
    #[serde(default)]
    coffre_du_camp: InventaireV7,
    /// Depuis la version 11 : Malvoise au camp.
    #[serde(default)]
    marchand_au_camp: bool,
    /// Identifiants des marchands morts dont l'âme est donnée à Malvoise ;
    /// avant la version 12, recueillies, ce qui revient au même (section
    /// 14.2).
    #[serde(default)]
    ames_recueillies: Vec<String>,
    /// Depuis la version 12 : âmes portées, pas encore données.
    #[serde(default)]
    ames_portees: Vec<String>,
    /// Depuis la version 12 : âmes à retrouver après une fuite ou une
    /// défaite.
    #[serde(default)]
    ames_a_retrouver: Vec<String>,
    codex: CodexV1,
    zones_explorees: Vec<String>,
    #[serde(default)]
    indices_trouves: usize,
    #[serde(default)]
    apotres_vaincus: Vec<String>,
    #[serde(default)]
    humains_vaincus: Vec<String>,
    #[serde(default)]
    conseils_vus: Vec<String>,
    #[serde(default)]
    expeditions_automatiques: Vec<ExpeditionAutomatiqueV3>,
    #[serde(default)]
    attaques_a_choisir: Vec<AttaqueAChoisirV3>,
    /// Depuis la version 14 (section 13.4).
    #[serde(default)]
    gunther_au_camp: bool,
    #[serde(default)]
    ameliorations: AmeliorationsV14,
    #[serde(default)]
    chantier: Option<ChantierV14>,
    /// Depuis la version 18 (section 13.8).
    #[serde(default)]
    ansgar_au_camp: bool,
    /// Secondes depuis le 1er janvier 1970 (UTC) : l'heure où la ligne sera
    /// prête.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ligne: Option<u64>,
    /// Le monstre pêché que le joueur n'a encore ni gardé ni relâché.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    prise: Option<MonstreV1>,
    /// Depuis la version 21 (section 15.4) : identifiants des comptes
    /// Discord des amis.
    #[serde(default)]
    amis: Vec<u64>,
    #[serde(default)]
    victoires_en_duel: u32,
    #[serde(default)]
    defaites_en_duel: u32,
    /// Depuis la version 22 (section 13.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pieges: Option<PiegesV22>,
    #[serde(default)]
    entrainement: Vec<EntrainementV22>,
    /// Depuis la version 25 (section 9.4) : "forge", "eclipse-ouverte" ou
    /// "demiurge-vaincu", absent tant que le Béhérit n'est pas forgé.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    beherit: Option<String>,
    /// Depuis la version 26 (section 9.5) : dernières étapes franchies
    /// dans la Tour sans fond, seul et à deux.
    #[serde(default)]
    record_seul_dans_la_tour: u32,
    #[serde(default)]
    record_a_deux_dans_la_tour: u32,
    /// Depuis la version 28 (section 13.11) : les parents confiés à
    /// Malvoise, absent sans rite en cours.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rite: Option<RiteV28>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RiteV28 {
    parents: Vec<MonstreV1>,
    /// Secondes depuis le 1er janvier 1970 (UTC).
    fin: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct AmeliorationsV14 {
    #[serde(default)]
    enclos: u8,
    #[serde(default)]
    filets: u8,
    /// Depuis la version 15.
    #[serde(default)]
    equipe: u8,
    /// Depuis la version 22 (section 13.6).
    #[serde(default)]
    pieges: u8,
    #[serde(default)]
    cour: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct PiegesV22 {
    zone: String,
    /// Secondes depuis le 1er janvier 1970 (UTC) : l'heure d'où part leur
    /// compte.
    depuis: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct EntrainementV22 {
    monstre: MonstreV1,
    /// Secondes depuis le 1er janvier 1970 (UTC).
    depuis: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChantierV14 {
    /// Depuis la version 16, absente pour un objet en fabrication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    amelioration: Option<String>,
    #[serde(default)]
    niveau: u8,
    /// Depuis la version 16 : l'objet que fabrique Gunther (section 13.5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    objet: Option<String>,
    /// Depuis la version 23 : un charme de garde, sans amélioration ni
    /// objet (section 13.5).
    #[serde(default)]
    charme: bool,
    /// Depuis la version 27 : un lot de filets (section 13.5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    filets: Option<FiletV27>,
    /// Secondes depuis le 1er janvier 1970 (UTC).
    fin: u64,
}

fn amelioration_en_texte(amelioration: Amelioration) -> &'static str {
    match amelioration {
        Amelioration::Enclos => "enclos",
        Amelioration::Filets => "filets",
        Amelioration::Equipe => "equipe",
        Amelioration::Pieges => "pieges",
        Amelioration::Cour => "cour",
    }
}

fn lire_amelioration(texte: &str) -> Result<Amelioration, ErreurSauvegarde> {
    Amelioration::TOUTES
        .into_iter()
        .find(|amelioration| amelioration_en_texte(*amelioration) == texte)
        .ok_or_else(|| ErreurSauvegarde::Incoherente(format!("amélioration inconnue : {texte}")))
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct InventaireV7 {
    #[serde(default)]
    argent: u32,
    #[serde(default)]
    potions: u32,
    /// Depuis la version 11 (section 14.4).
    #[serde(default)]
    remedes: u32,
    #[serde(default)]
    grandes_potions: u32,
    #[serde(default)]
    elixirs: u32,
    /// Identifiants de leurs attaques.
    #[serde(default)]
    capsules: Vec<String>,
    /// Depuis la version 12 (section 5.9).
    #[serde(default)]
    capsules_d_habilete: u32,
    /// Depuis la version 12 (section 14.3).
    #[serde(default)]
    ressources: Vec<RessourceV12>,
    /// Depuis la version 13 (section 5.4).
    #[serde(default)]
    pierres: Vec<PierreV13>,
    /// Depuis la version 20 (section 14.3).
    #[serde(default)]
    boosters: Vec<BoosterV20>,
    /// Les Philtres chromatiques, sous leur ancien nom d'Éclats d'astre,
    /// gardé dans le fichier pour relire les sauvegardes existantes.
    #[serde(default, rename = "eclats_d_astre")]
    philtres_chromatiques: u32,
    /// Depuis la version 23 (section 12.5).
    #[serde(default)]
    charmes_de_garde: u32,
    /// Depuis la version 27 (section 11.2).
    #[serde(default)]
    filets: Vec<FiletV27>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FiletV27 {
    filet: String,
    nombre: u32,
}

fn lire_filet(texte: &str) -> Result<Filet, ErreurSauvegarde> {
    Filet::depuis_id(texte)
        .ok_or_else(|| ErreurSauvegarde::Incoherente(format!("filet inconnu : {texte}")))
}

#[derive(Debug, Serialize, Deserialize)]
struct BoosterV20 {
    booster: String,
    nombre: u32,
}

fn booster_en_texte(booster: Booster) -> &'static str {
    match booster {
        Booster::FioleDeSang => "fiole-de-sang",
        Booster::MoelleDeBete => "moelle-de-bete",
        Booster::EcorceDeFer => "ecorce-de-fer",
        Booster::EncreDArcane => "encre-d-arcane",
        Booster::CendreBenite => "cendre-benite",
        Booster::PoudreDeVifArgent => "poudre-de-vif-argent",
    }
}

fn lire_booster(texte: &str) -> Result<Booster, ErreurSauvegarde> {
    Booster::TOUS
        .into_iter()
        .find(|booster| booster_en_texte(*booster) == texte)
        .ok_or_else(|| ErreurSauvegarde::Incoherente(format!("booster inconnu : {texte}")))
}

#[derive(Debug, Serialize, Deserialize)]
struct PierreV13 {
    pierre: String,
    nombre: u32,
}

fn pierre_en_texte(pierre: PierreDEvolution) -> &'static str {
    match pierre {
        PierreDEvolution::Braise => "braise",
        PierreDEvolution::Lune => "lune",
    }
}

fn lire_pierre(texte: &str) -> Result<PierreDEvolution, ErreurSauvegarde> {
    PierreDEvolution::TOUTES
        .into_iter()
        .find(|pierre| pierre_en_texte(*pierre) == texte)
        .ok_or_else(|| {
            ErreurSauvegarde::Incoherente(format!("pierre d'évolution inconnue : {texte}"))
        })
}

#[derive(Debug, Serialize, Deserialize)]
struct RessourceV12 {
    id: String,
    nombre: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExpeditionAutomatiqueV3 {
    zone: String,
    monstres: Vec<MonstreV1>,
    /// Secondes depuis le 1er janvier 1970 (UTC).
    depart: u64,
    /// Fixé au départ, et non recalculé à la lecture : un rééquilibrage des
    /// durées ne change pas une expédition déjà partie.
    retour: u64,
    /// Depuis la version 23 (section 12.5) : la position du monstre protégé
    /// parmi ceux envoyés, et le charme de garde.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    protege: Option<usize>,
    #[serde(default)]
    charme: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct AttaqueAChoisirV3 {
    /// Avant la version 12, toujours au camp, sous le nom
    /// `position_au_camp`.
    #[serde(alias = "position_au_camp")]
    position: usize,
    /// Depuis la version 12 : le monstre est revenu dans l'équipe (section
    /// 12.7).
    #[serde(default)]
    dans_l_equipe: bool,
    attaque: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HerosV1 {
    nom: String,
    classe: String,
    /// Depuis la version 16 : les objets possédés, dans l'ordre où ils ont
    /// été obtenus, et ceux qui sont portés (section 4.3).
    #[serde(default)]
    objets: Vec<String>,
    #[serde(default)]
    equipement_porte: EquipementPorteV16,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct EquipementPorteV16 {
    #[serde(default)]
    arme: Option<String>,
    #[serde(default)]
    armure: Option<String>,
    #[serde(default)]
    premier_talisman: Option<String>,
    #[serde(default)]
    second_talisman: Option<String>,
}

fn depuis_equipement(equipement: &Equipement) -> (Vec<String>, EquipementPorteV16) {
    let porte = |place| {
        equipement
            .porte(place)
            .map(|objet: &ObjetDEquipement| objet.id().as_str().to_string())
    };
    let objets = equipement
        .possedes()
        .iter()
        .map(|objet| objet.id().as_str().to_string())
        .collect();
    let portes = EquipementPorteV16 {
        arme: porte(PlaceDEquipement::Arme),
        armure: porte(PlaceDEquipement::Armure),
        premier_talisman: porte(PlaceDEquipement::PremierTalisman),
        second_talisman: porte(PlaceDEquipement::SecondTalisman),
    };
    (objets, portes)
}

fn vers_objet(id: &str, catalogue: &dyn Catalogue) -> Result<ObjetDEquipement, ErreurSauvegarde> {
    catalogue
        .objet_d_equipement(&ObjetDEquipementId::new(id))
        .ok_or_else(|| ErreurSauvegarde::Incoherente(format!("objet d'équipement inconnu : {id}")))
}

/// Un objet disparu du jeu rend la sauvegarde illisible, comme une espèce
/// disparue (section 13.9).
fn vers_equipement(
    objets: &[String],
    portes: EquipementPorteV16,
    catalogue: &dyn Catalogue,
) -> Result<Equipement, ErreurSauvegarde> {
    let possedes = objets
        .iter()
        .map(|id| vers_objet(id, catalogue))
        .collect::<Result<Vec<_>, _>>()?;
    let portes: Vec<_> = [
        (PlaceDEquipement::Arme, portes.arme),
        (PlaceDEquipement::Armure, portes.armure),
        (PlaceDEquipement::PremierTalisman, portes.premier_talisman),
        (PlaceDEquipement::SecondTalisman, portes.second_talisman),
    ]
    .into_iter()
    .filter_map(|(place, id)| id.map(|id| (place, ObjetDEquipementId::new(id))))
    .collect();
    Equipement::restaurer(possedes, &portes).map_err(incoherente)
}

#[derive(Debug, Serialize, Deserialize)]
struct MonstreV1 {
    espece: String,
    surnom: Option<String>,
    niveau: u8,
    experience: u32,
    valeurs_internes: ValeursV1,
    attaques: Vec<String>,
    shiny: bool,
    pv: u32,
    statut: Option<String>,
    /// Depuis la version 8 ; avant, le monstre reçoit la première habileté
    /// de son espèce (section 5.9).
    #[serde(default)]
    habilete: Option<String>,
    /// Depuis la version 17 : le héros qui l'a cédé (section 15.1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    recu_de: Option<String>,
    /// Depuis la version 24 : les expéditions qu'il reste à la location
    /// d'un monstre loué (section 14.2).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    location: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ValeursV1 {
    pv: u8,
    attaque: u8,
    defense: u8,
    attaque_speciale: u8,
    defense_speciale: u8,
    vitesse: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct CodexV1 {
    rencontrees: Vec<String>,
    capturees: Vec<String>,
}

pub(crate) fn depuis_partie(partie: &Partie) -> SauvegardeV14 {
    let collection = partie.collection();
    let (objets, equipement_porte) = depuis_equipement(partie.heros().equipement());
    SauvegardeV14 {
        version: VERSION,
        heros: HerosV1 {
            nom: partie.heros().nom().as_str().to_string(),
            classe: nom_de_classe(partie.heros().classe()).to_string(),
            objets,
            equipement_porte,
        },
        equipe: collection.equipe().iter().map(depuis_monstre).collect(),
        camp: collection.camp().iter().map(depuis_monstre).collect(),
        potions: 0,
        argent: 0,
        capsules: Vec::new(),
        sac: depuis_inventaire(partie.sac()),
        coffre_du_camp: depuis_inventaire(partie.coffre_du_camp()),
        marchand_au_camp: partie.marchand_au_camp(),
        ames_recueillies: ids_de_marchands(partie.ames_donnees()),
        ames_portees: ids_de_marchands(partie.ames_portees()),
        ames_a_retrouver: partie
            .ames_a_retrouver()
            .iter()
            .map(|marchand| marchand.as_str().to_string())
            .collect(),
        codex: CodexV1 {
            rencontrees: ids(partie.codex().rencontrees()),
            capturees: ids(partie.codex().capturees()),
        },
        zones_explorees: partie
            .zones_explorees()
            .iter()
            .map(|zone| zone.as_str().to_string())
            .collect(),
        indices_trouves: partie.indices_trouves(),
        apotres_vaincus: ids(partie.apotres_vaincus()),
        humains_vaincus: partie
            .humains_vaincus()
            .iter()
            .map(|humain| humain.as_str().to_string())
            .collect(),
        conseils_vus: partie
            .conseils_vus()
            .iter()
            .map(|conseil| nom_de_conseil(*conseil).to_string())
            .collect(),
        expeditions_automatiques: partie
            .expeditions_automatiques()
            .iter()
            .map(|expedition| ExpeditionAutomatiqueV3 {
                zone: expedition.zone().id().as_str().to_string(),
                monstres: expedition.monstres().iter().map(depuis_monstre).collect(),
                depart: expedition.depart().secondes(),
                retour: expedition.retour().secondes(),
                protege: expedition.protection().protege,
                charme: expedition.protection().charme,
            })
            .collect(),
        attaques_a_choisir: partie
            .attaques_a_choisir()
            .iter()
            .map(|a_choisir| {
                let (position, dans_l_equipe) = match a_choisir.emplacement {
                    Emplacement::Equipe(position) => (position, true),
                    Emplacement::Camp(position) => (position, false),
                };
                AttaqueAChoisirV3 {
                    position,
                    dans_l_equipe,
                    attaque: a_choisir.attaque.id().as_str().to_string(),
                }
            })
            .collect(),
        gunther_au_camp: partie.forge().est_au_camp(),
        ameliorations: AmeliorationsV14 {
            enclos: partie.forge().ameliorations().niveau(Amelioration::Enclos),
            filets: partie.forge().ameliorations().niveau(Amelioration::Filets),
            equipe: partie.forge().ameliorations().niveau(Amelioration::Equipe),
            pieges: partie.forge().ameliorations().niveau(Amelioration::Pieges),
            cour: partie.forge().ameliorations().niveau(Amelioration::Cour),
        },
        chantier: partie
            .forge()
            .chantier()
            .map(|chantier| match &chantier.ouvrage {
                Ouvrage::Amelioration {
                    amelioration,
                    niveau,
                } => ChantierV14 {
                    amelioration: Some(amelioration_en_texte(*amelioration).to_string()),
                    niveau: *niveau,
                    objet: None,
                    charme: false,
                    filets: None,
                    fin: chantier.fin.secondes(),
                },
                Ouvrage::Objet(objet) => ChantierV14 {
                    amelioration: None,
                    niveau: 0,
                    objet: Some(objet.id().as_str().to_string()),
                    charme: false,
                    filets: None,
                    fin: chantier.fin.secondes(),
                },
                Ouvrage::CharmeDeGarde => ChantierV14 {
                    amelioration: None,
                    niveau: 0,
                    objet: None,
                    charme: true,
                    filets: None,
                    fin: chantier.fin.secondes(),
                },
                Ouvrage::Filets { filet, nombre } => ChantierV14 {
                    amelioration: None,
                    niveau: 0,
                    objet: None,
                    charme: false,
                    filets: Some(FiletV27 {
                        filet: filet.id().to_string(),
                        nombre: *nombre,
                    }),
                    fin: chantier.fin.secondes(),
                },
            }),
        ansgar_au_camp: partie.peche().pecheur_au_camp(),
        ligne: partie.peche().ligne().map(Horodatage::secondes),
        prise: partie.peche().prise().map(depuis_monstre),
        amis: partie.relations().amis().to_vec(),
        victoires_en_duel: partie.relations().palmares().victoires,
        defaites_en_duel: partie.relations().palmares().defaites,
        pieges: partie.machines().pieges().map(|pieges| PiegesV22 {
            zone: pieges.zone().as_str().to_string(),
            depuis: pieges.depuis().secondes(),
        }),
        entrainement: partie
            .machines()
            .cour()
            .iter()
            .map(|entrainement| EntrainementV22 {
                monstre: depuis_monstre(entrainement.monstre()),
                depuis: entrainement.depuis().secondes(),
            })
            .collect(),
        beherit: beherit_en_texte(partie.beherit()).map(str::to_string),
        record_seul_dans_la_tour: partie.records_de_la_tour().seul,
        record_a_deux_dans_la_tour: partie.records_de_la_tour().a_deux,
        rite: partie.rite().map(|rite| RiteV28 {
            parents: rite.parents().iter().map(depuis_monstre).collect(),
            fin: rite.fin().secondes(),
        }),
    }
}

fn beherit_en_texte(etat: EtatDuBeherit) -> Option<&'static str> {
    match etat {
        EtatDuBeherit::Aucun => None,
        EtatDuBeherit::Forge => Some("forge"),
        EtatDuBeherit::EclipseOuverte => Some("eclipse-ouverte"),
        EtatDuBeherit::DemiurgeVaincu => Some("demiurge-vaincu"),
    }
}

fn lire_beherit(texte: Option<&str>) -> Result<EtatDuBeherit, ErreurSauvegarde> {
    [
        EtatDuBeherit::Aucun,
        EtatDuBeherit::Forge,
        EtatDuBeherit::EclipseOuverte,
        EtatDuBeherit::DemiurgeVaincu,
    ]
    .into_iter()
    .find(|etat| beherit_en_texte(*etat) == texte)
    .ok_or_else(|| ErreurSauvegarde::Incoherente(format!("Béhérit inconnu : {texte:?}")))
}

fn depuis_inventaire(inventaire: &Inventaire) -> InventaireV7 {
    InventaireV7 {
        argent: inventaire.argent(),
        potions: inventaire.potions(),
        remedes: inventaire.soins().nombre(Soin::Remede),
        grandes_potions: inventaire.soins().nombre(Soin::GrandePotion),
        elixirs: inventaire.soins().nombre(Soin::Elixir),
        capsules: inventaire
            .capsules()
            .iter()
            .map(|capsule| capsule.id().as_str().to_string())
            .collect(),
        capsules_d_habilete: inventaire.capsules_d_habilete(),
        ressources: inventaire
            .ressources()
            .iter()
            .map(|(ressource, nombre)| RessourceV12 {
                id: ressource.id().as_str().to_string(),
                nombre: *nombre,
            })
            .collect(),
        pierres: inventaire
            .pierres()
            .iter()
            .map(|(pierre, nombre)| PierreV13 {
                pierre: pierre_en_texte(*pierre).to_string(),
                nombre: *nombre,
            })
            .collect(),
        boosters: inventaire
            .boosters()
            .iter()
            .map(|(booster, nombre)| BoosterV20 {
                booster: booster_en_texte(*booster).to_string(),
                nombre: *nombre,
            })
            .collect(),
        philtres_chromatiques: inventaire.philtres_chromatiques(),
        charmes_de_garde: inventaire.charmes_de_garde(),
        filets: inventaire
            .filets()
            .presents()
            .map(|(filet, nombre)| FiletV27 {
                filet: filet.id().to_string(),
                nombre,
            })
            .collect(),
    }
}

fn vers_inventaire(
    inventaire: InventaireV7,
    catalogue: &dyn Catalogue,
) -> Result<Inventaire, ErreurSauvegarde> {
    let capsules = inventaire
        .capsules
        .into_iter()
        .map(|id| {
            catalogue
                .capsule(&AttaqueId::new(id.as_str()))
                .ok_or(ErreurSauvegarde::CapsuleInconnue(id))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut resultat = Inventaire::new(inventaire.argent, inventaire.potions, capsules)
        .avec_soins(Soin::Remede, inventaire.remedes)
        .avec_soins(Soin::GrandePotion, inventaire.grandes_potions)
        .avec_soins(Soin::Elixir, inventaire.elixirs)
        .avec_capsules_d_habilete(inventaire.capsules_d_habilete);
    // Une ressource disparue du jeu ne se vend plus : elle est oubliée,
    // comme une âme dont le marchand a disparu.
    for ressource in inventaire.ressources {
        if let Some(connue) = catalogue.ressource(&RessourceId::new(ressource.id)) {
            resultat.ajouter_ressources(connue, ressource.nombre);
        }
    }
    for pierre in inventaire.pierres {
        resultat.ajouter_pierres(lire_pierre(&pierre.pierre)?, pierre.nombre);
    }
    for booster in inventaire.boosters {
        resultat.ajouter_boosters(lire_booster(&booster.booster)?, booster.nombre);
    }
    resultat.ajouter_philtres_chromatiques(inventaire.philtres_chromatiques);
    resultat.ajouter_charmes_de_garde(inventaire.charmes_de_garde);
    for filet in inventaire.filets {
        resultat.ajouter_filets(lire_filet(&filet.filet)?, filet.nombre);
    }
    Ok(resultat)
}

pub(crate) fn vers_partie(
    mut sauvegarde: SauvegardeV14,
    catalogue: &dyn Catalogue,
) -> Result<Partie, ErreurSauvegarde> {
    let heros = Heros::new(
        nom(&sauvegarde.heros.nom)?,
        classe(&sauvegarde.heros.classe)?,
    )
    .avec_equipement(vers_equipement(
        &sauvegarde.heros.objets,
        std::mem::take(&mut sauvegarde.heros.equipement_porte),
        catalogue,
    )?);
    // Avant la version 24, aucun monstre n'est loué (section 14.2).
    if sauvegarde.version < VERSION_DES_LOCATIONS {
        for monstre in sauvegarde.equipe.iter_mut().chain(&mut sauvegarde.camp) {
            monstre.location = None;
        }
    }
    let mut equipe = vers_monstres(sauvegarde.equipe, catalogue)?;
    let mut camp = vers_monstres(sauvegarde.camp, catalogue)?;
    let mut attaques_a_choisir = sauvegarde
        .attaques_a_choisir
        .into_iter()
        .map(|a_choisir| vers_attaque_a_choisir(a_choisir, &equipe, &camp))
        .collect::<Result<Vec<_>, _>>()?;
    let mut partis = sauvegarde
        .expeditions_automatiques
        .into_iter()
        .map(|expedition| vers_monstres_partis(expedition, catalogue))
        .collect::<Result<Vec<_>, _>>()?;
    if sauvegarde.version < VERSION_DES_AMES_D_APOTRES {
        changer_les_apotres_en_ames(&mut equipe, &mut camp, &mut attaques_a_choisir, &mut partis);
    }
    let expeditions_automatiques = partis
        .into_iter()
        .map(|parti| {
            ExpeditionAutomatique::restaurer(parti.zone, parti.monstres, parti.depart, parti.retour)
                .and_then(|expedition| expedition.avec_protection(parti.protection))
                .map_err(incoherente)
        })
        .collect::<Result<Vec<_>, _>>()?;
    // Avant la version 22, ni pièges ni cour d'entraînement (section 13.6).
    if sauvegarde.version < VERSION_DES_MACHINES {
        sauvegarde.ameliorations.pieges = 0;
        sauvegarde.ameliorations.cour = 0;
        sauvegarde.pieges = None;
        sauvegarde.entrainement.clear();
    }
    // Avant la version 14, ni amélioration ni chantier : seul Gunther peut
    // être au camp, s'il est déjà libéré.
    if sauvegarde.version < VERSION_DE_GUNTHER {
        sauvegarde.gunther_au_camp = sauvegarde
            .humains_vaincus
            .iter()
            .any(|humain| humain == GEOLIER_DE_GUNTHER);
        sauvegarde.ameliorations = AmeliorationsV14::default();
        sauvegarde.chantier = None;
    }
    let gunther_au_camp = sauvegarde.gunther_au_camp;
    let ameliorations = Ameliorations::new(
        sauvegarde.ameliorations.enclos,
        sauvegarde.ameliorations.filets,
        sauvegarde.ameliorations.equipe,
    )
    .and_then(|ameliorations| {
        ameliorations.avec_machines(
            sauvegarde.ameliorations.pieges,
            sauvegarde.ameliorations.cour,
        )
    })
    .ok_or_else(|| {
        ErreurSauvegarde::Incoherente("niveau d'amélioration hors limites".to_string())
    })?;
    let chantier = sauvegarde
        .chantier
        .map(|chantier| {
            let ouvrage = match (
                chantier.amelioration,
                chantier.objet,
                chantier.charme,
                chantier.filets,
            ) {
                (Some(amelioration), None, false, None) => Ouvrage::Amelioration {
                    amelioration: lire_amelioration(&amelioration)?,
                    niveau: chantier.niveau,
                },
                (None, Some(objet), false, None) => Ouvrage::Objet(vers_objet(&objet, catalogue)?),
                (None, None, true, None) => Ouvrage::CharmeDeGarde,
                (None, None, false, Some(lot)) if lot.nombre > 0 => Ouvrage::Filets {
                    filet: lire_filet(&lot.filet)?,
                    nombre: lot.nombre,
                },
                _ => {
                    return Err(ErreurSauvegarde::Incoherente(
                        "travail de Gunther sans ouvrage, ou double".to_string(),
                    ));
                }
            };
            Ok(Chantier {
                ouvrage,
                fin: Horodatage::depuis_secondes(chantier.fin),
            })
        })
        .transpose()?;
    let forge = Forge::restaurer(gunther_au_camp, ameliorations, chantier).ok_or_else(|| {
        ErreurSauvegarde::Incoherente("améliorations sans Gunther au camp".to_string())
    })?;
    let prise = sauvegarde
        .prise
        .map(|prise| vers_monstre(prise, catalogue))
        .transpose()?;
    let peche = Peche::restaurer(
        sauvegarde.ansgar_au_camp,
        sauvegarde.ligne.map(Horodatage::depuis_secondes),
        prise,
    )
    .ok_or_else(|| ErreurSauvegarde::Incoherente("pêche sans Ansgar au camp".to_string()))?;
    let collection = Collection::restaurer(equipe, camp, ameliorations.tailles())
        .map_err(|erreur| ErreurSauvegarde::Incoherente(erreur.to_string()))?;
    // Avant la version 7, tout ce que possédait le joueur est rangé dans le
    // coffre du camp, à l'abri (section 13.3).
    let (sac, mut coffre_du_camp) = if sauvegarde.version < VERSION_DU_COFFRE {
        let ancien = InventaireV7 {
            argent: sauvegarde.argent,
            potions: sauvegarde.potions,
            capsules: sauvegarde.capsules,
            ..InventaireV7::default()
        };
        (Inventaire::default(), vers_inventaire(ancien, catalogue)?)
    } else {
        // Avant la version 27, les filets étaient illimités : rien n'en dit
        // le nombre.
        if sauvegarde.version < VERSION_DES_FILETS {
            sauvegarde.sac.filets.clear();
            sauvegarde.coffre_du_camp.filets.clear();
        }
        (
            vers_inventaire(std::mem::take(&mut sauvegarde.sac), catalogue)?,
            vers_inventaire(std::mem::take(&mut sauvegarde.coffre_du_camp), catalogue)?,
        )
    };
    // Une partie antérieure aux filets en trouve 10 au coffre (section
    // 11.2, Q102).
    if sauvegarde.version < VERSION_DES_FILETS {
        coffre_du_camp.ajouter_filets(Filet::Chanvre, FILETS_DES_SAUVEGARDES_ANTERIEURES);
    }
    let mut codex = Codex::default();
    for espece in &sauvegarde.codex.rencontrees {
        codex.noter_rencontre(&EspeceId::new(espece.as_str()));
    }
    for espece in &sauvegarde.codex.capturees {
        codex.noter_capture(&EspeceId::new(espece.as_str()));
    }
    // La victoire sur un Apôtre ouvre désormais sa fiche, comme une capture
    // (section 5.6).
    if sauvegarde.version < VERSION_DES_AMES_D_APOTRES {
        for apotre in &sauvegarde.apotres_vaincus {
            codex.noter_capture(&EspeceId::new(apotre.as_str()));
        }
    }
    let zones_explorees: Vec<ZoneId> = sauvegarde
        .zones_explorees
        .into_iter()
        .map(ZoneId::new)
        .collect();
    // Un Apôtre vaincu reste une espèce du jeu, comme un monstre capturé.
    let apotres_vaincus = sauvegarde
        .apotres_vaincus
        .into_iter()
        .map(|id| match catalogue.espece(&EspeceId::new(id.as_str())) {
            Some(espece) => Ok(espece.id().clone()),
            None => Err(ErreurSauvegarde::EspeceInconnue(id)),
        })
        .collect::<Result<Vec<_>, _>>()?;
    // Chaque Apôtre vaincu laisse désormais un Philtre chromatique : celui des
    // victoires passées attend dans le coffre (section 14.3, Q93).
    if sauvegarde.version < VERSION_DES_PHILTRES_CHROMATIQUES {
        coffre_du_camp.ajouter_philtres_chromatiques(apotres_vaincus.len() as u32);
    }
    // L'ancienne piste comptait trois indices : une partie qui en a trouvé
    // reprend au premier, qui ouvre le Marais ; il faut vaincre le Bourreau
    // pour ouvrir la Vallée (section 9.4).
    let indices_trouves = if sauvegarde.version < VERSION_DE_LA_PISTE_A_DEUX_INDICES {
        sauvegarde.indices_trouves.min(1)
    } else {
        sauvegarde.indices_trouves
    };
    // Un humain disparu du jeu ne gêne pas : il ne sert qu'à ne plus faire
    // revenir celui qui a été vaincu (section 10.7).
    let humains_vaincus = sauvegarde
        .humains_vaincus
        .into_iter()
        .map(HumainId::new)
        .collect();
    // Malvoise remplace le marchand, et arrive au bout de la première
    // expédition menée à son terme : une partie qui en a mené une l'a au
    // camp (section 14.2).
    let marchand_au_camp = if sauvegarde.version < VERSION_DE_MALVOISE {
        sauvegarde.marchand_au_camp || !zones_explorees.is_empty()
    } else {
        sauvegarde.marchand_au_camp
    };
    // Une âme dont le marchand a disparu du jeu ne vend plus rien : elle est
    // oubliée, comme un humain disparu.
    let ames_donnees = vers_marchands(&sauvegarde.ames_recueillies, catalogue);
    let ames_portees = vers_marchands(&sauvegarde.ames_portees, catalogue);
    let ames_a_retrouver = vers_marchands(&sauvegarde.ames_a_retrouver, catalogue)
        .iter()
        .map(|marchand| marchand.id().clone())
        .collect();
    // Une zone disparue du jeu est refusée par le noyau, comme des pièges
    // posés dans une zone inexplorée.
    let cour = sauvegarde
        .entrainement
        .into_iter()
        .map(|entrainement| {
            Ok(Entrainement::new(
                vers_monstre(entrainement.monstre, catalogue)?,
                Horodatage::depuis_secondes(entrainement.depuis),
            ))
        })
        .collect::<Result<Vec<_>, ErreurSauvegarde>>()?;
    let pieges = sauvegarde.pieges.map(|pieges| {
        Pieges::new(
            ZoneId::new(pieges.zone),
            Horodatage::depuis_secondes(pieges.depuis),
        )
    });
    let machines = Machines::restaurer(pieges, cour);
    let conseils_vus = sauvegarde
        .conseils_vus
        .iter()
        .map(String::as_str)
        .map(conseil)
        .collect::<Result<Vec<_>, _>>()?;
    // Avant la version 25, pas de Béhérit (section 9.4).
    let beherit = if sauvegarde.version < VERSION_DU_BEHERIT {
        EtatDuBeherit::Aucun
    } else {
        lire_beherit(sauvegarde.beherit.as_deref())?
    };
    // Avant la version 26, pas de Tour sans fond (section 9.5).
    let records_de_la_tour = if sauvegarde.version < VERSION_DES_RECORDS {
        RecordsDeLaTour::default()
    } else {
        RecordsDeLaTour {
            seul: sauvegarde.record_seul_dans_la_tour,
            a_deux: sauvegarde.record_a_deux_dans_la_tour,
        }
    };
    // Avant la version 28, pas de rite (section 13.11).
    let rite = match sauvegarde.rite {
        Some(rite) if sauvegarde.version >= VERSION_DU_RITE => {
            let parents: [Monstre; 2] = vers_monstres(rite.parents, catalogue)?
                .try_into()
                .map_err(|_| {
                    ErreurSauvegarde::Incoherente("rite sans ses deux parents".to_string())
                })?;
            Some(Rite::new(parents, Horodatage::depuis_secondes(rite.fin)))
        }
        _ => None,
    };
    Partie::restaurer(EtatPartie {
        heros,
        collection,
        sac,
        coffre_du_camp,
        marchand_au_camp,
        ames_donnees,
        ames_portees,
        ames_a_retrouver,
        codex,
        zones_explorees,
        indices_trouves,
        apotres_vaincus,
        humains_vaincus,
        conseils_vus,
        expeditions_automatiques,
        attaques_a_choisir,
        forge,
        peche,
        relations: Relations::restaurer(
            sauvegarde.amis,
            Palmares {
                victoires: sauvegarde.victoires_en_duel,
                defaites: sauvegarde.defaites_en_duel,
            },
        ),
        machines,
        beherit,
        records_de_la_tour,
        rite,
    })
    .map_err(incoherente)
}

fn ids_de_marchands(marchands: &[MarchandMort]) -> Vec<String> {
    marchands
        .iter()
        .map(|marchand| marchand.id().as_str().to_string())
        .collect()
}

fn vers_marchands(ids: &[String], catalogue: &dyn Catalogue) -> Vec<MarchandMort> {
    ids.iter()
        .filter_map(|id| catalogue.marchand_mort(&MarchandId::new(id.as_str())))
        .collect()
}

/// Une expédition automatique lue, avant d'être vérifiée par le noyau.
struct MonstresPartis {
    zone: Zone,
    monstres: Vec<Monstre>,
    protection: Protection,
    depart: Horodatage,
    retour: Horodatage,
}

fn vers_monstres_partis(
    expedition: ExpeditionAutomatiqueV3,
    catalogue: &dyn Catalogue,
) -> Result<MonstresPartis, ErreurSauvegarde> {
    let zone = catalogue
        .zone(&ZoneId::new(expedition.zone.as_str()))
        .ok_or(ErreurSauvegarde::ZoneInconnue(expedition.zone))?;
    Ok(MonstresPartis {
        zone,
        monstres: vers_monstres(expedition.monstres, catalogue)?,
        protection: Protection {
            protege: expedition.protege,
            charme: expedition.charme,
        },
        depart: Horodatage::depuis_secondes(expedition.depart),
        retour: Horodatage::depuis_secondes(expedition.retour),
    })
}

/// Avant la version 19, un Apôtre vaincu pouvait être capturé. Il quitte
/// l'équipe, le camp ou son expédition automatique, et le joueur garde son
/// âme, qui suit les Apôtres vaincus (section 9.4). Une attaque qu'il
/// attendait de choisir est oubliée, et les autres suivent la place de leur
/// monstre ; une expédition dont il partait seul disparaît. Un joueur qui
/// n'aurait plus aucun autre monstre garde ses Apôtres (hypothèse, Q92).
fn changer_les_apotres_en_ames(
    equipe: &mut Vec<Monstre>,
    camp: &mut Vec<Monstre>,
    attaques_a_choisir: &mut Vec<AttaqueAChoisir>,
    partis: &mut Vec<MonstresPartis>,
) {
    let est_apotre = |monstre: &Monstre| monstre.espece().rarete() == Rarete::Apotre;
    let autres = equipe
        .iter()
        .chain(camp.iter())
        .chain(partis.iter().flat_map(|parti| parti.monstres.iter()))
        .filter(|monstre| !est_apotre(monstre))
        .count();
    if autres == 0 {
        return;
    }
    // La nouvelle place de chaque monstre gardé, dans l'équipe ou au camp.
    let nouvelles_places = |monstres: &[Monstre]| -> Vec<Option<usize>> {
        let mut suivante = 0;
        monstres
            .iter()
            .map(|monstre| {
                if est_apotre(monstre) {
                    None
                } else {
                    suivante += 1;
                    Some(suivante - 1)
                }
            })
            .collect()
    };
    let dans_l_equipe = nouvelles_places(equipe);
    let au_camp = nouvelles_places(camp);
    attaques_a_choisir.retain_mut(|a_choisir| {
        let nouvelle = match a_choisir.emplacement {
            Emplacement::Equipe(position) => dans_l_equipe[position].map(Emplacement::Equipe),
            Emplacement::Camp(position) => au_camp[position].map(Emplacement::Camp),
        };
        match nouvelle {
            Some(emplacement) => {
                a_choisir.emplacement = emplacement;
                true
            }
            None => false,
        }
    });
    equipe.retain(|monstre| !est_apotre(monstre));
    camp.retain(|monstre| !est_apotre(monstre));
    for parti in partis.iter_mut() {
        parti.monstres.retain(|monstre| !est_apotre(monstre));
    }
    partis.retain(|parti| !parti.monstres.is_empty());
}

/// L'attaque se retrouve parmi celles que l'espèce du monstre apprend.
fn vers_attaque_a_choisir(
    a_choisir: AttaqueAChoisirV3,
    equipe: &[Monstre],
    camp: &[Monstre],
) -> Result<AttaqueAChoisir, ErreurSauvegarde> {
    let (emplacement, monstres) = if a_choisir.dans_l_equipe {
        (Emplacement::Equipe(a_choisir.position), equipe)
    } else {
        (Emplacement::Camp(a_choisir.position), camp)
    };
    let monstre = monstres.get(a_choisir.position).ok_or_else(|| {
        ErreurSauvegarde::Incoherente(format!(
            "attaque à choisir pour un monstre absent ({emplacement:?})"
        ))
    })?;
    let espece = monstre.espece();
    let attaque = espece
        .attaque_apprise(&AttaqueId::new(a_choisir.attaque.as_str()))
        .cloned()
        .ok_or_else(|| ErreurSauvegarde::AttaqueInconnue {
            espece: espece.id().as_str().to_string(),
            attaque: a_choisir.attaque,
        })?;
    Ok(AttaqueAChoisir {
        emplacement,
        attaque,
    })
}

fn depuis_monstre(monstre: &Monstre) -> MonstreV1 {
    let vi = monstre.valeurs_internes().valeurs();
    MonstreV1 {
        espece: monstre.espece().id().as_str().to_string(),
        surnom: monstre.surnom().map(|surnom| surnom.as_str().to_string()),
        niveau: monstre.niveau().valeur(),
        experience: monstre.experience(),
        valeurs_internes: ValeursV1 {
            pv: vi.pv,
            attaque: vi.attaque,
            defense: vi.defense,
            attaque_speciale: vi.attaque_speciale,
            defense_speciale: vi.defense_speciale,
            vitesse: vi.vitesse,
        },
        attaques: monstre
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str().to_string())
            .collect(),
        shiny: monstre.est_shiny(),
        pv: monstre.pv_actuels(),
        statut: monstre
            .statut()
            .map(|statut| nom_de_statut(statut).to_string()),
        habilete: monstre
            .habilete()
            .map(|habilete| nom_d_habilete(habilete).to_string()),
        recu_de: monstre.recu_de().map(|heros| heros.as_str().to_string()),
        location: monstre.expeditions_de_location(),
    }
}

fn vers_monstres(
    monstres: Vec<MonstreV1>,
    catalogue: &dyn Catalogue,
) -> Result<Vec<Monstre>, ErreurSauvegarde> {
    monstres
        .into_iter()
        .map(|monstre| vers_monstre(monstre, catalogue))
        .collect()
}

fn vers_monstre(
    monstre: MonstreV1,
    catalogue: &dyn Catalogue,
) -> Result<Monstre, ErreurSauvegarde> {
    let espece = catalogue
        .espece(&EspeceId::new(monstre.espece.as_str()))
        .ok_or_else(|| ErreurSauvegarde::EspeceInconnue(monstre.espece.clone()))?;
    // Une attaque hors de la liste de l'espèce vient d'une capsule de son
    // type (section 13.10).
    let attaques = monstre
        .attaques
        .iter()
        .map(|id| {
            let id_attaque = AttaqueId::new(id.as_str());
            let de_capsule = || {
                catalogue
                    .capsule(&id_attaque)
                    .filter(|capsule| capsule.convient_a(&espece))
                    .map(|capsule| capsule.attaque().clone())
            };
            espece
                .attaque_apprise(&id_attaque)
                .cloned()
                .or_else(de_capsule)
                .ok_or_else(|| ErreurSauvegarde::AttaqueInconnue {
                    espece: monstre.espece.clone(),
                    attaque: id.clone(),
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let attaques = AttaquesMonstre::new(attaques).map_err(incoherente)?;
    let niveau = Niveau::new(monstre.niveau).map_err(incoherente)?;
    let vi = monstre.valeurs_internes;
    let valeurs_internes = ValeursInternes::new(Stats {
        pv: vi.pv,
        attaque: vi.attaque,
        defense: vi.defense,
        attaque_speciale: vi.attaque_speciale,
        defense_speciale: vi.defense_speciale,
        vitesse: vi.vitesse,
    })
    .map_err(incoherente)?;
    let surnom = monstre.surnom.as_deref().map(nom).transpose()?;
    let statut = monstre.statut.as_deref().map(statut).transpose()?;
    let habilete = monstre.habilete.as_deref().map(habilete).transpose()?;
    let recu_de = monstre.recu_de.as_deref().map(nom).transpose()?;
    Monstre::restaurer(EtatMonstre {
        espece,
        surnom,
        niveau,
        experience: monstre.experience,
        valeurs_internes,
        attaques,
        shiny: monstre.shiny,
        pv_actuels: monstre.pv,
        statut,
        habilete,
        recu_de,
        location: monstre.location,
    })
    .map_err(incoherente)
}

fn ids(especes: &[EspeceId]) -> Vec<String> {
    especes
        .iter()
        .map(|espece| espece.as_str().to_string())
        .collect()
}

fn nom(texte: &str) -> Result<Nom, ErreurSauvegarde> {
    Nom::new(texte).map_err(incoherente)
}

fn nom_de_classe(classe: Classe) -> &'static str {
    match classe {
        Classe::Guerrier => "guerrier",
        Classe::Mage => "mage",
        Classe::Templier => "templier",
        Classe::Assassin => "assassin",
    }
}

fn classe(texte: &str) -> Result<Classe, ErreurSauvegarde> {
    match texte {
        "guerrier" => Ok(Classe::Guerrier),
        "mage" => Ok(Classe::Mage),
        "templier" => Ok(Classe::Templier),
        "assassin" => Ok(Classe::Assassin),
        autre => Err(ErreurSauvegarde::Incoherente(format!(
            "classe inconnue : {autre}"
        ))),
    }
}

fn nom_de_statut(statut: Statut) -> &'static str {
    match statut {
        Statut::Poison => "poison",
        Statut::Paralysie => "paralysie",
    }
}

fn statut(texte: &str) -> Result<Statut, ErreurSauvegarde> {
    match texte {
        "poison" => Ok(Statut::Poison),
        "paralysie" => Ok(Statut::Paralysie),
        autre => Err(ErreurSauvegarde::Incoherente(format!(
            "statut inconnu : {autre}"
        ))),
    }
}

fn nom_d_habilete(habilete: Habilete) -> &'static str {
    match habilete {
        Habilete::Ferocite => "ferocite",
        Habilete::Carapace => "carapace",
        Habilete::Sorcellerie => "sorcellerie",
        Habilete::Volonte => "volonte",
        Habilete::Celerite => "celerite",
        Habilete::Affinite => "affinite",
        Habilete::Instinct => "instinct",
        Habilete::Eclaireur => "eclaireur",
        Habilete::Traqueur => "traqueur",
        Habilete::Pillard => "pillard",
        Habilete::Fouineur => "fouineur",
    }
}

fn habilete(texte: &str) -> Result<Habilete, ErreurSauvegarde> {
    Habilete::TOUTES
        .into_iter()
        .find(|habilete| nom_d_habilete(*habilete) == texte)
        .ok_or_else(|| ErreurSauvegarde::Incoherente(format!("habileté inconnue : {texte}")))
}

fn nom_de_conseil(conseil: Conseil) -> &'static str {
    match conseil {
        Conseil::PremierCombat => "premier-combat",
        Conseil::PremierArretEntreDeuxEtapes => "premier-arret-entre-deux-etapes",
        Conseil::PremierRetourAuCamp => "premier-retour-au-camp",
    }
}

fn conseil(texte: &str) -> Result<Conseil, ErreurSauvegarde> {
    match texte {
        "premier-combat" => Ok(Conseil::PremierCombat),
        "premier-arret-entre-deux-etapes" => Ok(Conseil::PremierArretEntreDeuxEtapes),
        "premier-retour-au-camp" => Ok(Conseil::PremierRetourAuCamp),
        autre => Err(ErreurSauvegarde::Incoherente(format!(
            "conseil inconnu : {autre}"
        ))),
    }
}

fn incoherente(erreur: impl std::fmt::Display) -> ErreurSauvegarde {
    ErreurSauvegarde::Incoherente(erreur.to_string())
}
