//! Marchand réfugié au camp (section 14.2) : il vend des potions et rachète
//! les monstres. Ses prix sont fixes, et non tirés au hasard, pour être
//! connus avant l'échange (section 14.5).

use std::fmt;

use crate::collection::ErreurCollection;
use crate::espece::Rarete;
use crate::monstre::Monstre;

/// En pièces d'argent.
pub const PRIX_DE_LA_POTION: u32 = 20;

/// Le prix de revente d'un shiny est doublé.
const MULTIPLICATEUR_SHINY: u32 = 2;

/// Pièces d'argent par niveau, selon la rareté de l'espèce.
fn prix_par_niveau(rarete: Rarete) -> u32 {
    match rarete {
        Rarete::Commune => 5,
        Rarete::Rare => 15,
        Rarete::Apotre => 40,
    }
}

/// Niveau multiplié par le prix de la rareté de l'espèce, doublé pour un
/// shiny.
pub fn prix_de_revente(monstre: &Monstre) -> u32 {
    let prix = u32::from(monstre.niveau().valeur()) * prix_par_niveau(monstre.espece().rarete());
    if monstre.est_shiny() {
        prix * MULTIPLICATEUR_SHINY
    } else {
        prix
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurMarchand {
    /// Aucun marchand ne s'est encore réfugié au camp (section 14.2).
    AucunMarchand,
    ArgentInsuffisant {
        prix: u32,
        argent: u32,
    },
    QuantiteNulle,
    /// La revente suit les règles du relâcher.
    Collection(ErreurCollection),
}

impl fmt::Display for ErreurMarchand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurMarchand::AucunMarchand => write!(f, "aucun marchand au camp"),
            ErreurMarchand::ArgentInsuffisant { prix, argent } => write!(
                f,
                "il faut {prix} pièces d'argent, le joueur n'en a que {argent}"
            ),
            ErreurMarchand::QuantiteNulle => write!(f, "il faut acheter au moins un objet"),
            ErreurMarchand::Collection(erreur) => write!(f, "{erreur}"),
        }
    }
}

impl std::error::Error for ErreurMarchand {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::donnees_de_test::espece;
    use crate::niveau::Niveau;
    use crate::stats::ValeursInternes;

    fn monstre(rarete: Rarete, niveau: u8, shiny: bool) -> Monstre {
        Monstre::creer(
            espece("loup").avec_rarete(rarete),
            Niveau::new(niveau).unwrap(),
            ValeursInternes::default(),
            shiny,
        )
        .unwrap()
    }

    #[test]
    fn prix_selon_le_niveau_et_la_rarete() {
        assert_eq!(prix_de_revente(&monstre(Rarete::Commune, 5, false)), 25);
        assert_eq!(prix_de_revente(&monstre(Rarete::Rare, 5, false)), 75);
        assert_eq!(prix_de_revente(&monstre(Rarete::Apotre, 15, false)), 600);
    }

    #[test]
    fn un_shiny_se_revend_le_double() {
        assert_eq!(prix_de_revente(&monstre(Rarete::Commune, 5, true)), 50);
    }
}
