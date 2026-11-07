//! Quêtes des personnages du camp (section 13.13) : Malvoise, Gunther et
//! Ansgar ont chacun une chaîne de quêtes, à faire dans l'ordre. Une quête
//! demande de rapporter des ressources, ou d'explorer une zone pour un objet
//! unique, gardé par un monstre. La dernière de chaque chaîne améliore le
//! service du personnage.

use std::fmt;
use std::time::Duration;

use crate::aleatoire::Aleatoire;
use crate::espece::Espece;
use crate::inventaire::Inventaire;
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::ressource::Ressource;
use crate::zone::{ZoneId, creer_monstre_sauvage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Personnage {
    Malvoise,
    Gunther,
    Ansgar,
}

impl Personnage {
    /// Dans l'ordre de leur arrivée au camp, qui est celui des affichages.
    pub const TOUS: [Personnage; 3] = [
        Personnage::Malvoise,
        Personnage::Gunther,
        Personnage::Ansgar,
    ];

    pub fn nom(self) -> &'static str {
        match self {
            Personnage::Malvoise => "Malvoise",
            Personnage::Gunther => "Gunther",
            Personnage::Ansgar => "Ansgar",
        }
    }

    /// Celui des sauvegardes et des composants du bot.
    pub fn identifiant(self) -> &'static str {
        match self {
            Personnage::Malvoise => "malvoise",
            Personnage::Gunther => "gunther",
            Personnage::Ansgar => "ansgar",
        }
    }

    pub fn depuis_identifiant(identifiant: &str) -> Option<Self> {
        Self::TOUS
            .into_iter()
            .find(|personnage| personnage.identifiant() == identifiant)
    }
}

/// Le service amélioré dure 25 % de moins (section 13.13).
pub fn raccourcir(duree: Duration) -> Duration {
    duree * 3 / 4
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueteId(String);

impl QueteId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Un objet unique, qui ne sert qu'à sa quête et porte son identifiant. Il
/// va dans le sac, y prend une place et se perd avec lui (section 13.13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjetDeQuete {
    quete: QueteId,
    nom: String,
}

impl ObjetDeQuete {
    pub fn new(quete: QueteId, nom: &str) -> Self {
        Self {
            quete,
            nom: nom.to_string(),
        }
    }

    pub fn quete(&self) -> &QueteId {
        &self.quete
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }
}

/// Ce qui ouvre une quête, une fois la précédente de sa chaîne accomplie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OuvertureDeQuete {
    /// Une zone de la carte ouverte.
    Zone(ZoneId),
    /// Une zone annexe ouverte (section 9.9).
    Annexe(ZoneId),
}

/// L'objet unique d'une zone et son gardien, qui l'attend à une étape fixée
/// de chaque expédition dans la zone, tant que le joueur ne l'a pas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exploration {
    zone: ZoneId,
    nom_de_la_zone: String,
    etape: u32,
    objet: ObjetDeQuete,
    gardien: Espece,
    niveau: Niveau,
    rencontre: String,
    prise: String,
}

impl Exploration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        zone: ZoneId,
        nom_de_la_zone: &str,
        etape: u32,
        objet: ObjetDeQuete,
        gardien: Espece,
        niveau: Niveau,
        rencontre: &str,
        prise: &str,
    ) -> Self {
        Self {
            zone,
            nom_de_la_zone: nom_de_la_zone.to_string(),
            etape,
            objet,
            gardien,
            niveau,
            rencontre: rencontre.to_string(),
            prise: prise.to_string(),
        }
    }

    pub fn zone(&self) -> &ZoneId {
        &self.zone
    }

    pub fn nom_de_la_zone(&self) -> &str {
        &self.nom_de_la_zone
    }

    pub fn etape(&self) -> u32 {
        self.etape
    }

    pub fn objet(&self) -> &ObjetDeQuete {
        &self.objet
    }

    pub fn gardien(&self) -> &Espece {
        &self.gardien
    }

    pub fn niveau(&self) -> Niveau {
        self.niveau
    }

    /// Avant le combat contre le gardien.
    pub fn rencontre(&self) -> &str {
        &self.rencontre
    }

    /// Après la victoire, quand l'objet va dans le sac.
    pub fn prise(&self) -> &str {
        &self.prise
    }

    /// Tirages : ceux d'un monstre sauvage (valeurs internes, shiny,
    /// habileté).
    pub fn generer_gardien(&self, aleatoire: &mut dyn Aleatoire) -> Monstre {
        creer_monstre_sauvage(&self.gardien, self.niveau, aleatoire)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DemandeDeQuete {
    /// Ce nombre de ressources d'une sorte, prises du sac d'abord, puis du
    /// coffre.
    Rapporter { ressource: Ressource, nombre: u32 },
    /// En boîte, pour que la demande reste légère.
    Explorer(Box<Exploration>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quete {
    id: QueteId,
    personnage: Personnage,
    titre: String,
    ouverture: OuvertureDeQuete,
    demande: DemandeDeQuete,
    /// Elle va au coffre, à la remise.
    recompense: Inventaire,
    /// Ce que dit le personnage quand le joueur l'écoute.
    texte_de_la_demande: String,
    /// Ce qu'il dit à la remise.
    texte_de_la_remise: String,
    /// La dernière de la chaîne améliore le service du personnage.
    ameliore_le_service: bool,
}

impl Quete {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: QueteId,
        personnage: Personnage,
        titre: &str,
        ouverture: OuvertureDeQuete,
        demande: DemandeDeQuete,
        recompense: Inventaire,
        texte_de_la_demande: &str,
        texte_de_la_remise: &str,
    ) -> Self {
        Self {
            id,
            personnage,
            titre: titre.to_string(),
            ouverture,
            demande,
            recompense,
            texte_de_la_demande: texte_de_la_demande.to_string(),
            texte_de_la_remise: texte_de_la_remise.to_string(),
            ameliore_le_service: false,
        }
    }

    pub fn qui_ameliore_le_service(mut self) -> Self {
        self.ameliore_le_service = true;
        self
    }

    pub fn id(&self) -> &QueteId {
        &self.id
    }

    pub fn personnage(&self) -> Personnage {
        self.personnage
    }

    pub fn titre(&self) -> &str {
        &self.titre
    }

    pub fn ouverture(&self) -> &OuvertureDeQuete {
        &self.ouverture
    }

    pub fn demande(&self) -> &DemandeDeQuete {
        &self.demande
    }

    pub fn recompense(&self) -> &Inventaire {
        &self.recompense
    }

    pub fn texte_de_la_demande(&self) -> &str {
        &self.texte_de_la_demande
    }

    pub fn texte_de_la_remise(&self) -> &str {
        &self.texte_de_la_remise
    }

    pub fn ameliore_le_service(&self) -> bool {
        self.ameliore_le_service
    }

    /// L'objet unique qu'elle demande, pour une quête d'exploration.
    pub fn exploration(&self) -> Option<&Exploration> {
        match &self.demande {
            DemandeDeQuete::Explorer(exploration) => Some(exploration),
            DemandeDeQuete::Rapporter { .. } => None,
        }
    }
}

/// L'avancée des chaînes : les quêtes écoutées, en cours, et les quêtes
/// accomplies, dans l'ordre de leur remise. La sauvegarde la garde.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EtatDesQuetes {
    pub en_cours: Vec<QueteId>,
    pub accomplies: Vec<QueteId>,
    /// Les personnages dont la chaîne est achevée.
    pub services_ameliores: Vec<Personnage>,
}

impl EtatDesQuetes {
    pub fn est_en_cours(&self, quete: &QueteId) -> bool {
        self.en_cours.contains(quete)
    }

    pub fn est_accomplie(&self, quete: &QueteId) -> bool {
        self.accomplies.contains(quete)
    }

    pub fn service_ameliore(&self, personnage: Personnage) -> bool {
        self.services_ameliores.contains(&personnage)
    }

    /// Les quêtes accomplies de ce personnage, parmi celles du contenu.
    pub fn accomplies_de(&self, personnage: Personnage, quetes: &[Quete]) -> usize {
        quetes
            .iter()
            .filter(|quete| quete.personnage() == personnage && self.est_accomplie(quete.id()))
            .count()
    }
}

/// Où en est la chaîne d'un personnage au camp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtatDeLaChaine<'q> {
    /// Le personnage n'est pas au camp.
    Absent,
    /// Sa prochaine quête n'est pas encore ouverte.
    RienPourLInstant,
    /// Ouverte : le joueur peut l'écouter.
    Proposee(&'q Quete),
    EnCours(&'q Quete),
    Achevee,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurQuete {
    PersonnageAbsent(Personnage),
    /// Rien à écouter, ou rien à remettre.
    RienADemander(Personnage),
    /// Il manque ce nombre de ces ressources. En boîte, comme l'objet, pour
    /// que l'erreur reste légère.
    RessourcesManquantes(Box<Ressource>, u32),
    ObjetManquant(Box<ObjetDeQuete>),
}

impl fmt::Display for ErreurQuete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurQuete::PersonnageAbsent(personnage) => {
                write!(f, "{} n'est pas au camp", personnage.nom())
            }
            ErreurQuete::RienADemander(personnage) => {
                write!(f, "{} n'a rien à vous demander", personnage.nom())
            }
            ErreurQuete::RessourcesManquantes(ressource, nombre) => {
                write!(f, "il manque {nombre} {}", ressource.nom())
            }
            ErreurQuete::ObjetManquant(objet) => write!(f, "objet manquant : {}", objet.nom()),
        }
    }
}

impl std::error::Error for ErreurQuete {}

/// Ce que rapporte une quête remise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemiseDeQuete {
    pub quete: Quete,
    /// Le personnage rend désormais son service plus vite.
    pub service_ameliore: bool,
}
