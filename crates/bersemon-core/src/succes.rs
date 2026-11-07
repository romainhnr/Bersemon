//! Succès du joueur (section 4.5) : chacun donne un titre que le héros peut
//! porter ; les plus grands exploits, un trophée du camp ; quatre, une
//! couleur du héros, et les trophées réunis, la couleur royale. Les succès ne
//! donnent rien d'autre : ni objet, ni bonus, ni argent.

use std::fmt;

use crate::amelioration::Amelioration;
use crate::beherit::EtatDuBeherit;
use crate::equipement::PlaceDEquipement;
use crate::espece::{Espece, EspeceId};
use crate::humain::HumainId;
use crate::partie::Partie;
use crate::quete_du_camp::Personnage;
use crate::recompenses_du_codex::especes_capturees;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SuccesId(String);

impl SuccesId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Les couleurs du héros, qui ne changent que son sprite, sur Discord.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CouleurDuHeros {
    Cendre,
    Sang,
    Abime,
    Nacre,
    /// Réservée au joueur qui a réuni tous les trophées du camp ; elle
    /// couronne le héros.
    Royale,
}

impl CouleurDuHeros {
    pub const TOUTES: [CouleurDuHeros; 5] = [
        CouleurDuHeros::Cendre,
        CouleurDuHeros::Sang,
        CouleurDuHeros::Abime,
        CouleurDuHeros::Nacre,
        CouleurDuHeros::Royale,
    ];

    pub fn nom(self) -> &'static str {
        match self {
            CouleurDuHeros::Cendre => "Cendre",
            CouleurDuHeros::Sang => "Sang",
            CouleurDuHeros::Abime => "Abîme",
            CouleurDuHeros::Nacre => "Nacre",
            CouleurDuHeros::Royale => "Royale",
        }
    }

    /// Celui des sauvegardes, des menus et des sections des sprites.
    pub fn identifiant(self) -> &'static str {
        match self {
            CouleurDuHeros::Cendre => "cendre",
            CouleurDuHeros::Sang => "sang",
            CouleurDuHeros::Abime => "abime",
            CouleurDuHeros::Nacre => "nacre",
            CouleurDuHeros::Royale => "royale",
        }
    }

    pub fn depuis_identifiant(identifiant: &str) -> Option<Self> {
        Self::TOUTES
            .into_iter()
            .find(|couleur| couleur.identifiant() == identifiant)
    }
}

/// Ce que demande un succès, lu sur l'état de la partie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionDeSucces {
    /// Une expédition menée au bout, dans une zone de la carte.
    ExpeditionMenee,
    /// Au moins ce nombre d'espèces capturées au codex (section 5.6).
    EspecesCapturees(u32),
    /// Toutes les espèces du codex, quel qu'en soit le nombre.
    CodexComplet,
    /// Au moins ce nombre de monstres shiny, ensemble dans l'équipe et au
    /// camp.
    MonstresShiny(usize),
    HerosShiny,
    /// Un monstre de l'équipe ou du camp à ce niveau, ou au-delà.
    Niveau(u8),
    HumainVaincu(HumainId),
    ApotreAbattu(EspeceId),
    DemiurgeAbattu,
    LegendaireCapture(EspeceId),
    PaliersDeLaFosse(u8),
    /// Le record de la Tour sans fond, seul (section 9.5).
    EtapesDeLaTour(u32),
    DuelsGagnes(u32),
    /// L'argent du sac et du coffre réunis.
    Argent(u32),
    /// Chaque amélioration du camp à son niveau maximal (section 13.4).
    AmeliorationsAuMaximum,
    /// Une arme et une armure de ce palier, ou plus, portées.
    PalierDEquipement(u8),
    AnnexesOuvertes(usize),
    /// Toute la chaîne de quêtes du personnage (section 13.13).
    QuetesAccomplies(Personnage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Succes {
    id: SuccesId,
    titre: String,
    /// Ce qu'il demande, dit au joueur : "abattre le Baron pendu".
    description: String,
    condition: ConditionDeSucces,
    /// La phrase du trophée qu'il ajoute au camp, pour les plus grands
    /// exploits.
    trophee: Option<String>,
    couleur: Option<CouleurDuHeros>,
}

impl Succes {
    pub fn new(id: &str, titre: &str, description: &str, condition: ConditionDeSucces) -> Self {
        Self {
            id: SuccesId::new(id),
            titre: titre.to_string(),
            description: description.to_string(),
            condition,
            trophee: None,
            couleur: None,
        }
    }

    pub fn avec_trophee(mut self, trophee: &str) -> Self {
        self.trophee = Some(trophee.to_string());
        self
    }

    pub fn avec_couleur(mut self, couleur: CouleurDuHeros) -> Self {
        self.couleur = Some(couleur);
        self
    }

    pub fn id(&self) -> &SuccesId {
        &self.id
    }

    pub fn titre(&self) -> &str {
        &self.titre
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn condition(&self) -> &ConditionDeSucces {
        &self.condition
    }

    pub fn trophee(&self) -> Option<&str> {
        self.trophee.as_deref()
    }

    pub fn couleur(&self) -> Option<CouleurDuHeros> {
        self.couleur
    }

    /// D'après l'état de la partie ; les espèces servent à compter le codex.
    pub fn est_atteint(&self, partie: &Partie, especes: &[Espece]) -> bool {
        let collection = partie.collection();
        let monstres = || collection.equipe().iter().chain(collection.camp());
        match &self.condition {
            ConditionDeSucces::ExpeditionMenee => !partie.zones_explorees().is_empty(),
            ConditionDeSucces::EspecesCapturees(nombre) => {
                especes_capturees(partie.codex(), especes) >= *nombre
            }
            ConditionDeSucces::CodexComplet => {
                let total = crate::recompenses_du_codex::especes_au_codex(especes);
                total > 0 && especes_capturees(partie.codex(), especes) >= total
            }
            ConditionDeSucces::MonstresShiny(nombre) => {
                monstres().filter(|monstre| monstre.est_shiny()).count() >= *nombre
            }
            ConditionDeSucces::HerosShiny => partie.heros().est_shiny(),
            ConditionDeSucces::Niveau(niveau) => {
                monstres().any(|monstre| monstre.niveau().valeur() >= *niveau)
            }
            ConditionDeSucces::HumainVaincu(humain) => partie.humains_vaincus().contains(humain),
            ConditionDeSucces::ApotreAbattu(apotre) => partie.apotres_vaincus().contains(apotre),
            ConditionDeSucces::DemiurgeAbattu => partie.beherit() == EtatDuBeherit::DemiurgeVaincu,
            ConditionDeSucces::LegendaireCapture(espece) => partie.legendaire_capture(espece),
            ConditionDeSucces::PaliersDeLaFosse(paliers) => {
                partie.arene().paliers_vaincus >= *paliers
            }
            ConditionDeSucces::EtapesDeLaTour(etapes) => {
                partie.records_de_la_tour().seul >= *etapes
            }
            ConditionDeSucces::DuelsGagnes(victoires) => {
                partie.relations().palmares().victoires >= *victoires
            }
            ConditionDeSucces::Argent(montant) => {
                partie
                    .sac()
                    .argent()
                    .saturating_add(partie.coffre_du_camp().argent())
                    >= *montant
            }
            ConditionDeSucces::AmeliorationsAuMaximum => {
                Amelioration::TOUTES.into_iter().all(|amelioration| {
                    partie.forge().ameliorations().niveau(amelioration) >= amelioration.niveau_max()
                })
            }
            ConditionDeSucces::PalierDEquipement(palier) => {
                let equipement = partie.heros().equipement();
                [PlaceDEquipement::Arme, PlaceDEquipement::Armure]
                    .into_iter()
                    .all(|place| {
                        equipement
                            .porte(place)
                            .is_some_and(|objet| objet.palier() >= *palier)
                    })
            }
            ConditionDeSucces::AnnexesOuvertes(nombre) => {
                partie.decouvertes().annexes_ouvertes.len() >= *nombre
            }
            ConditionDeSucces::QuetesAccomplies(personnage) => {
                partie.quetes_du_camp().service_ameliore(*personnage)
            }
        }
    }
}

/// Les couleurs que le joueur peut faire porter à son héros : celles de ses
/// succès, et la royale quand tous les trophées sont réunis.
pub fn couleurs_debloquees(obtenus: &[SuccesId], succes: &[Succes]) -> Vec<CouleurDuHeros> {
    let obtenu = |un: &Succes| obtenus.contains(un.id());
    let mut couleurs: Vec<CouleurDuHeros> = Vec::new();
    for couleur in CouleurDuHeros::TOUTES {
        let debloquee = match couleur {
            CouleurDuHeros::Royale => tous_les_trophees(obtenus, succes),
            _ => succes
                .iter()
                .any(|un| un.couleur() == Some(couleur) && obtenu(un)),
        };
        if debloquee {
            couleurs.push(couleur);
        }
    }
    couleurs
}

/// Les succès qui ont un trophée, tous obtenus.
pub fn tous_les_trophees(obtenus: &[SuccesId], succes: &[Succes]) -> bool {
    let mut trophees = succes.iter().filter(|un| un.trophee().is_some()).peekable();
    trophees.peek().is_some() && trophees.all(|un| obtenus.contains(un.id()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurSucces {
    /// Le titre d'un succès que le joueur n'a pas.
    NonObtenu,
    CouleurBloquee,
}

impl fmt::Display for ErreurSucces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurSucces::NonObtenu => write!(f, "succès pas encore obtenu"),
            ErreurSucces::CouleurBloquee => write!(f, "couleur pas encore débloquée"),
        }
    }
}

impl std::error::Error for ErreurSucces {}
