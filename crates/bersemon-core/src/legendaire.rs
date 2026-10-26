//! Zones légendaires (section 9.10) : après la chute du Démiurge, des zones
//! au niveau 100, gardées chacune par un légendaire, capturé d'office une
//! fois vaincu.

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::espece::{Espece, EspeceId};
use crate::monstre::{Monstre, tirer_shiny};
use crate::niveau::Niveau;
use crate::stats::ValeursInternes;
use crate::zone::Zone;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurLegendaire {
    /// L'espèce n'apprend aucune attaque, ou en apprend une en double,
    /// jusqu'au niveau du légendaire.
    EspeceInvalide(EspeceId),
}

impl fmt::Display for ErreurLegendaire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurLegendaire::EspeceInvalide(espece) => write!(
                f,
                "l'espèce {} ne peut pas être créée au niveau du légendaire",
                espece.as_str()
            ),
        }
    }
}

impl std::error::Error for ErreurLegendaire {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Legendaire {
    espece: Espece,
    niveau: Niveau,
    avant_combat: String,
    /// Raconté quand il rejoint le joueur, après la victoire.
    capture: String,
}

impl Legendaire {
    /// Vérifie que l'espèce peut être créée à ce niveau : la génération ne
    /// peut ensuite plus échouer.
    pub fn new(
        espece: Espece,
        niveau: Niveau,
        avant_combat: impl Into<String>,
        capture: impl Into<String>,
    ) -> Result<Self, ErreurLegendaire> {
        Monstre::creer(espece.clone(), niveau, ValeursInternes::maximales(), false)
            .map_err(|_| ErreurLegendaire::EspeceInvalide(espece.id().clone()))?;
        Ok(Self {
            espece,
            niveau,
            avant_combat: avant_combat.into(),
            capture: capture.into(),
        })
    }

    pub fn espece(&self) -> &Espece {
        &self.espece
    }

    pub fn niveau(&self) -> Niveau {
        self.niveau
    }

    pub fn avant_combat(&self) -> &str {
        &self.avant_combat
    }

    pub fn capture(&self) -> &str {
        &self.capture
    }

    /// Valeurs internes fixes de 31, comme un Apôtre, et 1 chance sur 512
    /// d'être shiny (section 9.10). Tirages : le shiny, puis l'habileté.
    pub fn generer(&self, aleatoire: &mut dyn Aleatoire) -> Monstre {
        let shiny = tirer_shiny(aleatoire);
        Monstre::creer(
            self.espece.clone(),
            self.niveau,
            ValeursInternes::maximales(),
            shiny,
        )
        .expect("espèce vérifiée au niveau du légendaire")
        .tirer_son_habilete(aleatoire)
    }
}

/// Une zone où le légendaire attend à la dernière étape, tant qu'il n'est
/// pas capturé.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoneLegendaire {
    zone: Zone,
    legendaire: Legendaire,
}

impl ZoneLegendaire {
    pub fn new(zone: Zone, legendaire: Legendaire) -> Self {
        Self { zone, legendaire }
    }

    pub fn zone(&self) -> &Zone {
        &self.zone
    }

    pub fn legendaire(&self) -> &Legendaire {
        &self.legendaire
    }
}
