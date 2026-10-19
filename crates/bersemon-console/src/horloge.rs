//! L'heure système, que le noyau ne lit jamais lui-même : la console la lui
//! donne à chaque choix, et s'en sert pour afficher le temps restant des
//! expéditions automatiques (section 12.2).

use std::time::{SystemTime, UNIX_EPOCH};

use bersemon_core::horodatage::Horodatage;

pub fn maintenant() -> Horodatage {
    let secondes = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duree| duree.as_secs());
    Horodatage::depuis_secondes(secondes)
}
