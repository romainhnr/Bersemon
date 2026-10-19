//! Marchand réfugié au camp (section 14.2) : il vend des soins et des
//! capsules d'attaque, et rachète les monstres. Ses prix sont fixes, et non
//! tirés au hasard, pour être connus avant l'échange (section 14.5).

use std::fmt;

use crate::collection::ErreurCollection;
use crate::espece::Rarete;
use crate::monstre::Monstre;
use crate::soin::Soin;

/// En pièces d'argent (sections 14.4 et 14.5).
pub fn prix_du_soin(soin: Soin) -> u32 {
    match soin {
        Soin::Potion => 20,
        Soin::Remede => 25,
        Soin::GrandePotion => 60,
        Soin::Elixir => 150,
    }
}

/// Le niveau multiplié par le total des stats de base est divisé par ce
/// nombre (section 14.5).
const DIVISEUR_DE_REVENTE: u32 = 50;
/// Le prix de revente d'un shiny est doublé.
const MULTIPLICATEUR_SHINY: u32 = 2;

fn multiplicateur_de_rarete(rarete: Rarete) -> u32 {
    match rarete {
        Rarete::Commune => 1,
        Rarete::Rare => 2,
        Rarete::Apotre => 4,
    }
}

/// Niveau x total des stats de base / 50, en division entière, puis
/// multiplié selon la rareté de l'espèce et doublé pour un shiny.
pub fn prix_de_revente(monstre: &Monstre) -> u32 {
    let espece = monstre.espece();
    let prix = u32::from(monstre.niveau().valeur()) * espece.total_des_stats_de_base()
        / DIVISEUR_DE_REVENTE
        * multiplicateur_de_rarete(espece.rarete());
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
    /// Le marchand ne vend ce soin qu'une fois l'âme de son marchand
    /// recueillie (section 14.2).
    SoinIndisponible(Soin),
    /// Une capsule sert sans s'user : en racheter une ne sert à rien.
    CapsuleDejaPossedee,
    /// Le joueur ne porte pas l'âme de ce marchand (section 14.2).
    AmeNonPortee,
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
            ErreurMarchand::SoinIndisponible(soin) => {
                write!(f, "le marchand ne vend pas encore ce soin : {soin:?}")
            }
            ErreurMarchand::CapsuleDejaPossedee => write!(f, "cette capsule est déjà obtenue"),
            ErreurMarchand::AmeNonPortee => write!(f, "le joueur ne porte pas cette âme"),
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

    /// Les espèces de test ont 300 de stats de base au total.
    #[test]
    fn prix_selon_le_niveau_le_total_des_stats_et_la_rarete() {
        assert_eq!(prix_de_revente(&monstre(Rarete::Commune, 5, false)), 30);
        assert_eq!(prix_de_revente(&monstre(Rarete::Rare, 5, false)), 60);
        assert_eq!(prix_de_revente(&monstre(Rarete::Apotre, 15, false)), 360);
    }

    #[test]
    fn un_shiny_se_revend_le_double() {
        assert_eq!(prix_de_revente(&monstre(Rarete::Commune, 5, true)), 60);
    }
}
