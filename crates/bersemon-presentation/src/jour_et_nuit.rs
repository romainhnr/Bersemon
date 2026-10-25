//! Les textes du cycle du jour et de la nuit (section 9.6, docs/contenu-v1.md,
//! section 31).

use bersemon_core::horodatage::Horodatage;
use bersemon_core::jour_et_nuit::{Activite, JourOuNuit};

use crate::expeditions_automatiques::duree;

pub fn nom(jour_ou_nuit: JourOuNuit) -> &'static str {
    match jour_ou_nuit {
        JourOuNuit::Jour => "Jour",
        JourOuNuit::Nuit => "Nuit",
    }
}

/// Au camp et sur la carte : "Nuit : l'aube dans 23 min."
pub fn moment_present(maintenant: Horodatage) -> String {
    let restant = duree(JourOuNuit::duree_restante(maintenant));
    match JourOuNuit::a(maintenant) {
        JourOuNuit::Jour => format!("Jour : la nuit tombe dans {restant}."),
        JourOuNuit::Nuit => format!("Nuit : l'aube dans {restant}."),
    }
}

/// Après le départ dans une zone de la carte.
pub fn au_depart(jour_ou_nuit: JourOuNuit) -> &'static str {
    match jour_ou_nuit {
        JourOuNuit::Jour => "Il fait jour. Les créatures de la nuit se terrent.",
        JourOuNuit::Nuit => {
            "La nuit est tombée. Les bêtes rôdent plus fortes, et d'autres sortent de l'ombre."
        }
    }
}

/// "de nuit", "de jour".
pub fn complement(jour_ou_nuit: JourOuNuit) -> &'static str {
    match jour_ou_nuit {
        JourOuNuit::Jour => "de jour",
        JourOuNuit::Nuit => "de nuit",
    }
}

/// À la fiche d'une espèce rencontrée.
pub fn activite(activite: Activite) -> &'static str {
    match activite {
        Activite::Diurne => "Ne sort que le jour.",
        Activite::Nocturne => "Ne sort que la nuit.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_moment_present_dit_le_temps_avant_le_suivant() {
        let a = |minutes: u64| Horodatage::depuis_secondes(minutes * 60);
        assert_eq!(moment_present(a(48)), "Jour : la nuit tombe dans 1 h 12.");
        assert_eq!(moment_present(a(157)), "Nuit : l'aube dans 23 min.");
        assert_eq!(moment_present(a(0)), "Jour : la nuit tombe dans 2 h.");
    }
}
