//! Format JSON de la sauvegarde, version 1, et sa conversion vers les types
//! du noyau. Les énumérations sont écrites avec des noms fixés ici, et non
//! avec les noms Rust, pour qu'un renommage dans le code ne casse pas les
//! sauvegardes existantes.

use serde::{Deserialize, Serialize};

use bersemon_core::attaque::{AttaqueId, AttaquesMonstre};
use bersemon_core::catalogue::Catalogue;
use bersemon_core::codex::Codex;
use bersemon_core::collection::Collection;
use bersemon_core::espece::EspeceId;
use bersemon_core::heros::{Classe, Heros};
use bersemon_core::inventaire::Inventaire;
use bersemon_core::monstre::{EtatMonstre, Monstre};
use bersemon_core::niveau::Niveau;
use bersemon_core::nom::Nom;
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::ErreurSauvegarde;
use bersemon_core::stats::{Stats, ValeursInternes};
use bersemon_core::statut::Statut;
use bersemon_core::zone::ZoneId;

/// À augmenter à chaque changement du format, avec une conversion depuis
/// les versions précédentes.
pub(crate) const VERSION: u64 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SauvegardeV1 {
    pub(crate) version: u64,
    heros: HerosV1,
    equipe: Vec<MonstreV1>,
    camp: Vec<MonstreV1>,
    potions: u32,
    codex: CodexV1,
    zones_explorees: Vec<String>,
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

pub(crate) fn depuis_partie(partie: &Partie) -> SauvegardeV1 {
    let collection = partie.collection();
    SauvegardeV1 {
        version: VERSION,
        heros: HerosV1 {
            nom: partie.heros().nom().as_str().to_string(),
            classe: nom_de_classe(partie.heros().classe()).to_string(),
        },
        equipe: collection.equipe().iter().map(depuis_monstre).collect(),
        camp: collection.camp().iter().map(depuis_monstre).collect(),
        potions: partie.inventaire().potions(),
        codex: CodexV1 {
            rencontrees: ids(partie.codex().rencontrees()),
            capturees: ids(partie.codex().capturees()),
        },
        zones_explorees: partie
            .zones_explorees()
            .iter()
            .map(|zone| zone.as_str().to_string())
            .collect(),
    }
}

pub(crate) fn vers_partie(
    sauvegarde: SauvegardeV1,
    catalogue: &dyn Catalogue,
) -> Result<Partie, ErreurSauvegarde> {
    let heros = Heros::new(
        nom(&sauvegarde.heros.nom)?,
        classe(&sauvegarde.heros.classe)?,
    );
    let equipe = vers_monstres(sauvegarde.equipe, catalogue)?;
    let camp = vers_monstres(sauvegarde.camp, catalogue)?;
    let collection = Collection::restaurer(equipe, camp)
        .map_err(|erreur| ErreurSauvegarde::Incoherente(erreur.to_string()))?;
    let mut inventaire = Inventaire::default();
    inventaire.ajouter_potions(sauvegarde.potions);
    let mut codex = Codex::default();
    for espece in &sauvegarde.codex.rencontrees {
        codex.noter_rencontre(&EspeceId::new(espece.as_str()));
    }
    for espece in &sauvegarde.codex.capturees {
        codex.noter_capture(&EspeceId::new(espece.as_str()));
    }
    let zones = sauvegarde
        .zones_explorees
        .into_iter()
        .map(ZoneId::new)
        .collect();
    Ok(Partie::restaurer(
        heros, collection, inventaire, codex, zones,
    ))
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
    let attaques = monstre
        .attaques
        .iter()
        .map(|id| {
            espece
                .attaque_apprise(&AttaqueId::new(id.as_str()))
                .cloned()
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
    }
}

fn classe(texte: &str) -> Result<Classe, ErreurSauvegarde> {
    match texte {
        "guerrier" => Ok(Classe::Guerrier),
        "mage" => Ok(Classe::Mage),
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

fn incoherente(erreur: impl std::fmt::Display) -> ErreurSauvegarde {
    ErreurSauvegarde::Incoherente(erreur.to_string())
}
