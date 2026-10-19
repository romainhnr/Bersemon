//! Format JSON de la sauvegarde, version 11, et sa conversion vers les types
//! du noyau. Les énumérations sont écrites avec des noms fixés ici, et non
//! avec les noms Rust, pour qu'un renommage dans le code ne casse pas les
//! sauvegardes existantes.
//!
//! Chaque structure porte le numéro de la version qui l'a introduite.

use serde::{Deserialize, Serialize};

use bersemon_core::ame::MarchandId;
use bersemon_core::attaque::{AttaqueId, AttaquesMonstre};
use bersemon_core::catalogue::Catalogue;
use bersemon_core::codex::Codex;
use bersemon_core::collection::Collection;
use bersemon_core::espece::EspeceId;
use bersemon_core::expedition_automatique::ExpeditionAutomatique;
use bersemon_core::habilete::Habilete;
use bersemon_core::heros::{Classe, Heros};
use bersemon_core::horodatage::Horodatage;
use bersemon_core::humain::HumainId;
use bersemon_core::inventaire::Inventaire;
use bersemon_core::monstre::{EtatMonstre, Monstre};
use bersemon_core::niveau::Niveau;
use bersemon_core::nom::Nom;
use bersemon_core::partie::{AttaqueAChoisir, EtatPartie, Partie};
use bersemon_core::sauvegarde::ErreurSauvegarde;
use bersemon_core::soin::Soin;
use bersemon_core::stats::{Stats, ValeursInternes};
use bersemon_core::statut::Statut;
use bersemon_core::tutoriel::Conseil;
use bersemon_core::zone::ZoneId;

/// À augmenter à chaque changement du format, avec une conversion depuis
/// les versions précédentes.
pub(crate) const VERSION: u64 = 11;

/// La version de la sauvegarde qui sépare le sac du coffre du camp.
const VERSION_DU_COFFRE: u64 = 7;

/// La version de la sauvegarde qui suit la piste à deux indices (section
/// 9.4).
const VERSION_DE_LA_PISTE_A_DEUX_INDICES: u64 = 10;

/// La version de la sauvegarde où Malvoise remplace le marchand (section
/// 14.2).
const VERSION_DE_MALVOISE: u64 = 11;

/// La version 2 ajoute la traque de l'Apôtre et le tutoriel, la version 3
/// les expéditions automatiques et les attaques à choisir à leur retour, la
/// version 4 l'argent, la version 5 le marchand, la version 6 les capsules
/// d'attaque, la version 7 le sac et le coffre du camp, la version 8
/// l'habileté de chaque monstre, la version 9 les humains vaincus ; la
/// version 10 suit la piste à deux indices ; la version 11 ajoute le
/// remède, la grande potion, l'élixir et les âmes recueillies, et Malvoise
/// remplace le marchand. Une sauvegarde d'une version
/// précédente se lit avec cette structure : les champs ajoutés y prennent
/// leur valeur de début de partie.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SauvegardeV11 {
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
    /// Identifiants des marchands morts.
    #[serde(default)]
    ames_recueillies: Vec<String>,
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
}

#[derive(Debug, Serialize, Deserialize)]
struct AttaqueAChoisirV3 {
    position_au_camp: usize,
    attaque: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HerosV1 {
    nom: String,
    classe: String,
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

pub(crate) fn depuis_partie(partie: &Partie) -> SauvegardeV11 {
    let collection = partie.collection();
    SauvegardeV11 {
        version: VERSION,
        heros: HerosV1 {
            nom: partie.heros().nom().as_str().to_string(),
            classe: nom_de_classe(partie.heros().classe()).to_string(),
        },
        equipe: collection.equipe().iter().map(depuis_monstre).collect(),
        camp: collection.camp().iter().map(depuis_monstre).collect(),
        potions: 0,
        argent: 0,
        capsules: Vec::new(),
        sac: depuis_inventaire(partie.sac()),
        coffre_du_camp: depuis_inventaire(partie.coffre_du_camp()),
        marchand_au_camp: partie.marchand_au_camp(),
        ames_recueillies: partie
            .ames_recueillies()
            .iter()
            .map(|marchand| marchand.id().as_str().to_string())
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
            })
            .collect(),
        attaques_a_choisir: partie
            .attaques_a_choisir()
            .iter()
            .map(|a_choisir| AttaqueAChoisirV3 {
                position_au_camp: a_choisir.position_au_camp,
                attaque: a_choisir.attaque.id().as_str().to_string(),
            })
            .collect(),
    }
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
    Ok(
        Inventaire::new(inventaire.argent, inventaire.potions, capsules)
            .avec_soins(Soin::Remede, inventaire.remedes)
            .avec_soins(Soin::GrandePotion, inventaire.grandes_potions)
            .avec_soins(Soin::Elixir, inventaire.elixirs),
    )
}

pub(crate) fn vers_partie(
    sauvegarde: SauvegardeV11,
    catalogue: &dyn Catalogue,
) -> Result<Partie, ErreurSauvegarde> {
    let heros = Heros::new(
        nom(&sauvegarde.heros.nom)?,
        classe(&sauvegarde.heros.classe)?,
    );
    let equipe = vers_monstres(sauvegarde.equipe, catalogue)?;
    let camp = vers_monstres(sauvegarde.camp, catalogue)?;
    let attaques_a_choisir = sauvegarde
        .attaques_a_choisir
        .into_iter()
        .map(|a_choisir| vers_attaque_a_choisir(a_choisir, &camp))
        .collect::<Result<Vec<_>, _>>()?;
    let expeditions_automatiques = sauvegarde
        .expeditions_automatiques
        .into_iter()
        .map(|expedition| vers_expedition_automatique(expedition, catalogue))
        .collect::<Result<Vec<_>, _>>()?;
    let collection = Collection::restaurer(equipe, camp)
        .map_err(|erreur| ErreurSauvegarde::Incoherente(erreur.to_string()))?;
    // Avant la version 7, tout ce que possédait le joueur est rangé dans le
    // coffre du camp, à l'abri (section 13.3).
    let (sac, coffre_du_camp) = if sauvegarde.version < VERSION_DU_COFFRE {
        let ancien = InventaireV7 {
            argent: sauvegarde.argent,
            potions: sauvegarde.potions,
            capsules: sauvegarde.capsules,
            ..InventaireV7::default()
        };
        (Inventaire::default(), vers_inventaire(ancien, catalogue)?)
    } else {
        (
            vers_inventaire(sauvegarde.sac, catalogue)?,
            vers_inventaire(sauvegarde.coffre_du_camp, catalogue)?,
        )
    };
    let mut codex = Codex::default();
    for espece in &sauvegarde.codex.rencontrees {
        codex.noter_rencontre(&EspeceId::new(espece.as_str()));
    }
    for espece in &sauvegarde.codex.capturees {
        codex.noter_capture(&EspeceId::new(espece.as_str()));
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
    let ames_recueillies = sauvegarde
        .ames_recueillies
        .iter()
        .filter_map(|id| catalogue.marchand_mort(&MarchandId::new(id.as_str())))
        .collect();
    let conseils_vus = sauvegarde
        .conseils_vus
        .iter()
        .map(String::as_str)
        .map(conseil)
        .collect::<Result<Vec<_>, _>>()?;
    Partie::restaurer(EtatPartie {
        heros,
        collection,
        sac,
        coffre_du_camp,
        marchand_au_camp,
        ames_recueillies,
        codex,
        zones_explorees,
        indices_trouves,
        apotres_vaincus,
        humains_vaincus,
        conseils_vus,
        expeditions_automatiques,
        attaques_a_choisir,
    })
    .map_err(incoherente)
}

fn vers_expedition_automatique(
    expedition: ExpeditionAutomatiqueV3,
    catalogue: &dyn Catalogue,
) -> Result<ExpeditionAutomatique, ErreurSauvegarde> {
    let zone = catalogue
        .zone(&ZoneId::new(expedition.zone.as_str()))
        .ok_or(ErreurSauvegarde::ZoneInconnue(expedition.zone))?;
    let monstres = vers_monstres(expedition.monstres, catalogue)?;
    ExpeditionAutomatique::restaurer(
        zone,
        monstres,
        Horodatage::depuis_secondes(expedition.depart),
        Horodatage::depuis_secondes(expedition.retour),
    )
    .map_err(incoherente)
}

/// L'attaque se retrouve parmi celles que l'espèce du monstre apprend.
fn vers_attaque_a_choisir(
    a_choisir: AttaqueAChoisirV3,
    camp: &[Monstre],
) -> Result<AttaqueAChoisir, ErreurSauvegarde> {
    let monstre = camp.get(a_choisir.position_au_camp).ok_or_else(|| {
        ErreurSauvegarde::Incoherente(format!(
            "attaque à choisir pour un monstre absent du camp (position {})",
            a_choisir.position_au_camp
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
        position_au_camp: a_choisir.position_au_camp,
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
