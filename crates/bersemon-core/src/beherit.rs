//! Béhérit pourpre, Éclipse et Démiurge (section 9.4) : Malvoise forge le
//! Béhérit des âmes des Apôtres ; son premier usage réclame un sacrifice et
//! ouvre l'Éclipse, où attend le Démiurge.

use std::fmt;

use crate::collection::ErreurCollection;
use crate::traque::Apotre;
use crate::zone::Zone;

/// Le sacrifice coûte vraiment : un monstre de ce niveau ou plus (section
/// 9.4).
pub const NIVEAU_MINIMAL_DU_SACRIFICE: u8 = 30;

/// Ce que laisse le Démiurge vaincu, dans le sac (section 9.4).
pub const PHILTRES_DU_DEMIURGE: u32 = 3;

/// L'avancée du joueur vers le Démiurge, dans l'ordre.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum EtatDuBeherit {
    /// Les âmes des Apôtres, s'il en a, restent les siennes.
    #[default]
    Aucun,
    /// Forgé par Malvoise, il attend son sacrifice.
    Forge,
    /// Le sacrifice est fait : l'Éclipse reste ouverte jusqu'à la victoire.
    EclipseOuverte,
    /// L'Éclipse s'est refermée.
    DemiurgeVaincu,
}

impl EtatDuBeherit {
    /// Le joueur porte le Béhérit, que rien ne lui fait perdre.
    pub fn est_forge(self) -> bool {
        self != EtatDuBeherit::Aucun
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurBeherit {
    AucunMarchand,
    /// Il faut les âmes de tous les Apôtres.
    AmesManquantes,
    DejaForge,
    /// Pas encore forgé.
    AucunBeherit,
    /// Le sacrifice n'est demandé qu'une fois.
    SacrificeDejaFait,
    NiveauInsuffisant,
    /// Le sacrifice laisserait l'équipe vide : personne ne partirait.
    EquipeVide,
    Collection(ErreurCollection),
    /// Pas encore de sacrifice, ou le Démiurge est vaincu.
    EclipseFermee,
}

impl fmt::Display for ErreurBeherit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurBeherit::AucunMarchand => write!(f, "Malvoise n'est pas au camp"),
            ErreurBeherit::AmesManquantes => write!(f, "il faut les âmes de tous les Apôtres"),
            ErreurBeherit::DejaForge => write!(f, "le Béhérit est déjà forgé"),
            ErreurBeherit::AucunBeherit => write!(f, "le Béhérit n'est pas forgé"),
            ErreurBeherit::SacrificeDejaFait => write!(f, "le sacrifice est déjà fait"),
            ErreurBeherit::NiveauInsuffisant => write!(
                f,
                "le sacrifice exige un monstre de niveau {NIVEAU_MINIMAL_DU_SACRIFICE} ou plus"
            ),
            ErreurBeherit::EquipeVide => write!(f, "le sacrifice laisserait l'équipe vide"),
            ErreurBeherit::Collection(erreur) => write!(f, "{erreur}"),
            ErreurBeherit::EclipseFermee => write!(f, "l'Éclipse est fermée"),
        }
    }
}

impl std::error::Error for ErreurBeherit {}

/// Le lieu et le Démiurge, avec leurs textes (docs/contenu-v1.md, section
/// 23). L'Éclipse n'est pas une zone de la carte : elle n'a qu'une étape,
/// où attend le Démiurge, et ne compte jamais comme explorée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    eclipse: Zone,
    demiurge: Apotre,
    forge: String,
    sacrifice: String,
}

impl Invocation {
    /// `sacrifice` contient `{monstre}`, remplacé par le nom du monstre
    /// sacrifié.
    pub fn new(
        eclipse: Zone,
        demiurge: Apotre,
        forge: impl Into<String>,
        sacrifice: impl Into<String>,
    ) -> Self {
        Self {
            eclipse: eclipse.en_lieu_d_invocation(),
            demiurge,
            forge: forge.into(),
            sacrifice: sacrifice.into(),
        }
    }

    pub fn eclipse(&self) -> &Zone {
        &self.eclipse
    }

    pub fn demiurge(&self) -> &Apotre {
        &self.demiurge
    }

    /// Le récit de Malvoise qui forge le Béhérit.
    pub fn recit_de_la_forge(&self) -> &str {
        &self.forge
    }

    pub fn recit_du_sacrifice(&self, monstre: &str) -> String {
        self.sacrifice.replace("{monstre}", monstre)
    }
}
