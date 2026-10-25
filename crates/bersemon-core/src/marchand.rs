//! Marchand réfugié au camp (section 14.2) : il vend des soins et des
//! capsules d'attaque, et rachète les monstres. Ses prix sont fixes, et non
//! tirés au hasard, pour être connus avant l'échange (section 14.5).

use std::fmt;

use crate::collection::ErreurCollection;
use crate::espece::{Espece, Rarete};
use crate::filet::Filet;
use crate::monstre::Monstre;
use crate::niveau::Niveau;
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

/// En pièces d'argent ; le filet de chaînes ne s'achète pas
/// (docs/contenu-v1.md, section 26).
pub fn prix_du_filet(filet: Filet) -> Option<u32> {
    match filet {
        Filet::Chanvre => Some(10),
        Filet::Leste => Some(30),
        Filet::Chaines => None,
    }
}

/// En pièces d'argent (section 5.9, Q81).
pub const PRIX_CAPSULE_D_HABILETE: u32 = 200;

/// En pièces d'argent, pour chaque pierre (section 5.4, Q84).
pub const PRIX_PIERRE_D_EVOLUTION: u32 = 300;

/// Le niveau multiplié par le total des stats de base est divisé par ce
/// nombre (section 14.5).
const DIVISEUR_DE_REVENTE: u32 = 50;
/// Le prix de revente d'un shiny est doublé.
const MULTIPLICATEUR_SHINY: u32 = 2;

fn multiplicateur_de_rarete(rarete: Rarete) -> u32 {
    match rarete {
        Rarete::Commune => 1,
        Rarete::Rare => 2,
        Rarete::Apotre | Rarete::Demiurge => 4,
    }
}

/// Un monstre acheté coûte 4 fois sa revente, hors shiny (section 14.2).
const MULTIPLICATEUR_D_ACHAT: u32 = 4;

/// Une location vaut pour ce nombre d'expéditions manuelles (section 14.2).
pub const EXPEDITIONS_DE_LOCATION: u8 = 3;

/// Une espèce que Malvoise vend ou loue, au niveau du monstre qu'il
/// confiera, et son prix (section 14.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffreDeMonstre {
    pub espece: Espece,
    pub niveau: Niveau,
    pub prix: u32,
}

impl OffreDeMonstre {
    /// 4 fois la revente d'un monstre de cette espèce à ce niveau, hors
    /// shiny.
    pub(crate) fn a_l_achat(espece: Espece, niveau: Niveau) -> Self {
        let prix = prix_d_une_espece(&espece, niveau) * MULTIPLICATEUR_D_ACHAT;
        Self {
            espece,
            niveau,
            prix,
        }
    }

    /// La revente d'un monstre de cette espèce à ce niveau, hors shiny.
    pub(crate) fn a_la_location(espece: Espece, niveau: Niveau) -> Self {
        let prix = prix_d_une_espece(&espece, niveau);
        Self {
            espece,
            niveau,
            prix,
        }
    }
}

fn prix_d_une_espece(espece: &Espece, niveau: Niveau) -> u32 {
    u32::from(niveau.valeur()) * espece.total_des_stats_de_base() / DIVISEUR_DE_REVENTE
        * multiplicateur_de_rarete(espece.rarete())
}

/// Niveau x total des stats de base / 50, en division entière, puis
/// multiplié selon la rareté de l'espèce et doublé pour un shiny.
pub fn prix_de_revente(monstre: &Monstre) -> u32 {
    let prix = prix_d_une_espece(monstre.espece(), monstre.niveau());
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
    /// Malvoise ne vend pas le filet de chaînes (section 14.2).
    FiletIndisponible(Filet),
    /// Une capsule sert sans s'user : en racheter une ne sert à rien.
    CapsuleDejaPossedee,
    /// Le joueur ne porte pas l'âme de ce marchand (section 14.2).
    AmeNonPortee,
    /// Ni le sac ni le coffre n'en contiennent (section 14.3).
    RessourceAbsente,
    /// La revente suit les règles du relâcher.
    Collection(ErreurCollection),
    /// Malvoise ne rend shiny un monstre que contre un Éclat d'astre
    /// (section 13.7).
    AucunEclatDAstre,
    DejaShiny,
    /// Malvoise ne vend ni ne loue cette espèce pour l'instant (section
    /// 14.2).
    MonstreIndisponible,
    /// L'équipe et le camp sont pleins.
    PlusDePlace,
    /// Un seul monstre loué à la fois.
    DejaUnMonstreLoue,
    AucunMonstreLoue,
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
            ErreurMarchand::FiletIndisponible(filet) => {
                write!(f, "le marchand ne vend pas ce filet : {filet:?}")
            }
            ErreurMarchand::CapsuleDejaPossedee => write!(f, "cette capsule est déjà obtenue"),
            ErreurMarchand::AmeNonPortee => write!(f, "le joueur ne porte pas cette âme"),
            ErreurMarchand::RessourceAbsente => write!(f, "aucune ressource de cette sorte"),
            ErreurMarchand::Collection(erreur) => write!(f, "{erreur}"),
            ErreurMarchand::AucunEclatDAstre => write!(f, "aucun Éclat d'astre"),
            ErreurMarchand::DejaShiny => write!(f, "ce monstre est déjà shiny"),
            ErreurMarchand::MonstreIndisponible => {
                write!(f, "Malvoise ne propose pas cette espèce")
            }
            ErreurMarchand::PlusDePlace => write!(f, "l'équipe et le camp sont pleins"),
            ErreurMarchand::DejaUnMonstreLoue => write!(f, "un monstre est déjà loué"),
            ErreurMarchand::AucunMonstreLoue => write!(f, "aucun monstre loué"),
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
