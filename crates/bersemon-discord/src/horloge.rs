//! L'heure système, que le noyau ne lit jamais lui-même : le bot la lui donne
//! à chaque choix, s'en sert pour afficher les expéditions automatiques et
//! pour savoir quand annoncer leur retour (section 12.2).

use std::time::{SystemTime, UNIX_EPOCH};

use bersemon_core::horodatage::Horodatage;

pub fn maintenant() -> Horodatage {
    let secondes = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duree| duree.as_secs());
    Horodatage::depuis_secondes(secondes)
}
