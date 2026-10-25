//! Filets de capture (section 11.2) : leurs noms, au singulier et au
//! pluriel, communs à la console et au bot Discord. Textes dans
//! docs/contenu-v1.md, section 26.

use bersemon_core::filet::{Filet, Filets};
use bersemon_core::marchand::prix_du_filet;

use crate::partie::argent;

/// "Filet de chanvre", en tête d'un libellé.
pub fn nom_du_filet(filet: Filet) -> &'static str {
    match filet {
        Filet::Chanvre => "Filet de chanvre",
        Filet::Leste => "Filet lesté",
        Filet::Chaines => "Filet de chaînes",
    }
}

/// "1 filet de chanvre", "2 filets lestés".
pub fn filets(filet: Filet, nombre: u32) -> String {
    let (singulier, pluriel) = match filet {
        Filet::Chanvre => ("filet de chanvre", "filets de chanvre"),
        Filet::Leste => ("filet lesté", "filets lestés"),
        Filet::Chaines => ("filet de chaînes", "filets de chaînes"),
    };
    if nombre > 1 {
        format!("{nombre} {pluriel}")
    } else {
        format!("{nombre} {singulier}")
    }
}

/// Ceux qu'il y a, du plus simple au plus efficace.
pub fn liste_de_filets(reserve: &Filets) -> Vec<String> {
    reserve
        .presents()
        .map(|(filet, nombre)| filets(filet, nombre))
        .collect()
}

/// L'effet d'une qualité : "chance de capture x1,5".
pub fn effet_du_filet(filet: Filet) -> &'static str {
    match filet {
        Filet::Chanvre => "chance de capture normale",
        Filet::Leste => "chance de capture x1,5",
        Filet::Chaines => "chance de capture x2",
    }
}

/// "Filet lesté (chance de capture x1,5) : 30 pièces d'argent", pour
/// ceux que vend Malvoise (section 14.2).
pub fn offre_de_filet(filet: Filet) -> Option<String> {
    prix_du_filet(filet).map(|prix| {
        format!(
            "{} ({}) : {}",
            nom_du_filet(filet),
            effet_du_filet(filet),
            argent(prix)
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noms_au_singulier_et_au_pluriel() {
        assert_eq!(filets(Filet::Chanvre, 1), "1 filet de chanvre");
        assert_eq!(filets(Filet::Leste, 3), "3 filets lestés");
        let mut reserve = Filets::default();
        reserve.ajouter(Filet::Chaines, 2);
        reserve.ajouter(Filet::Chanvre, 10);
        assert_eq!(
            liste_de_filets(&reserve),
            vec!["10 filets de chanvre", "2 filets de chaînes"]
        );
    }

    #[test]
    fn malvoise_ne_vend_pas_le_filet_de_chaines() {
        assert_eq!(
            offre_de_filet(Filet::Leste).as_deref(),
            Some("Filet lesté (chance de capture x1,5) : 30 pièces d'argent")
        );
        assert_eq!(offre_de_filet(Filet::Chaines), None);
    }
}
