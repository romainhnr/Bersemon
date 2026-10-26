//! Zones annexes (section 9.9) : une par zone d'Apôtre, courte et riche en
//! coffres, ouverte par une carte trouvée dans un coffre de sa zone, une
//! fois l'humain gardien vaincu.

use crate::zone::{Zone, ZoneId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annexe {
    zone: Zone,
    /// La zone dont les coffres contiennent sa carte.
    de: ZoneId,
    /// Raconté quand la carte est trouvée.
    decouverte: String,
}

impl Annexe {
    pub fn new(zone: Zone, de: ZoneId, decouverte: impl Into<String>) -> Self {
        Self {
            zone,
            de,
            decouverte: decouverte.into(),
        }
    }

    pub fn zone(&self) -> &Zone {
        &self.zone
    }

    pub fn de(&self) -> &ZoneId {
        &self.de
    }

    pub fn decouverte(&self) -> &str {
        &self.decouverte
    }
}
