//! Atelier d'Ansgar (section 13.8) : avec les matières que remonte sa ligne
//! et les ressources que lâchent les monstres, Ansgar prépare au camp, sur
//! le moment, des appâts et des parfums.

use std::fmt;

use crate::appat::Appat;
use crate::espece::Espece;
use crate::parfum::Parfum;
use crate::ressource::Ressource;

/// Le parfum d'une espèce demande ce nombre de la ressource qu'elle lâche,
/// en plus de sa base (docs/contenu-v1.md, section 53).
pub const RESSOURCES_DU_PARFUM_D_ESPECE: u32 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtelierDAnsgar {
    /// Dans l'ordre de `Appat::TOUS`.
    appats: Vec<(Appat, Vec<(Ressource, u32)>)>,
    parfum_de_charogne: Vec<(Ressource, u32)>,
    /// Ce qu'il faut, en plus de la ressource de l'espèce.
    base_du_parfum_d_espece: Vec<(Ressource, u32)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurParametresAtelier {
    /// Chaque appât a sa recette, une seule fois.
    RecettesDAppatsIncompletes,
    /// Une recette qui ne coûte rien, ou un nombre nul.
    CoutInvalide,
}

impl fmt::Display for ErreurParametresAtelier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurParametresAtelier::RecettesDAppatsIncompletes => {
                write!(f, "chaque appât doit avoir une recette, une seule")
            }
            ErreurParametresAtelier::CoutInvalide => write!(f, "recette vide ou nombre nul"),
        }
    }
}

impl std::error::Error for ErreurParametresAtelier {}

impl AtelierDAnsgar {
    pub fn new(
        appats: Vec<(Appat, Vec<(Ressource, u32)>)>,
        parfum_de_charogne: Vec<(Ressource, u32)>,
        base_du_parfum_d_espece: Vec<(Ressource, u32)>,
    ) -> Result<Self, ErreurParametresAtelier> {
        let ordre: Vec<Appat> = appats.iter().map(|(appat, _)| *appat).collect();
        if ordre != Appat::TOUS {
            return Err(ErreurParametresAtelier::RecettesDAppatsIncompletes);
        }
        let cout_valide = |cout: &[(Ressource, u32)]| {
            !cout.is_empty() && cout.iter().all(|(_, nombre)| *nombre > 0)
        };
        let couts = appats
            .iter()
            .map(|(_, cout)| cout.as_slice())
            .chain([parfum_de_charogne.as_slice()]);
        // La base du parfum d'une espèce peut être vide : sa ressource suffit
        // alors à le payer.
        if !couts.into_iter().all(cout_valide)
            || base_du_parfum_d_espece
                .iter()
                .any(|(_, nombre)| *nombre == 0)
        {
            return Err(ErreurParametresAtelier::CoutInvalide);
        }
        Ok(Self {
            appats,
            parfum_de_charogne,
            base_du_parfum_d_espece,
        })
    }

    pub fn cout_de_l_appat(&self, appat: Appat) -> &[(Ressource, u32)] {
        self.appats
            .iter()
            .find(|(recette, _)| *recette == appat)
            .map(|(_, cout)| cout.as_slice())
            .expect("chaque appât a sa recette, vérifié par AtelierDAnsgar::new")
    }

    pub fn cout_du_parfum_de_charogne(&self) -> &[(Ressource, u32)] {
        &self.parfum_de_charogne
    }

    pub fn base_du_parfum_d_espece(&self) -> &[(Ressource, u32)] {
        &self.base_du_parfum_d_espece
    }

    /// `None` pour une espèce qui ne lâche pas de ressource : rien ne peut
    /// l'attirer.
    pub fn cout_du_parfum(&self, parfum: &Parfum) -> Option<Vec<(Ressource, u32)>> {
        match parfum {
            Parfum::Charogne => Some(self.parfum_de_charogne.clone()),
            Parfum::Espece(espece) => {
                let ressource = espece.ressource()?;
                let mut cout = self.base_du_parfum_d_espece.clone();
                cout.push((ressource.clone(), RESSOURCES_DU_PARFUM_D_ESPECE));
                Some(cout)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurAtelier {
    /// Ansgar n'est pas encore au camp.
    AucunPecheur,
    /// Il manque ces ressources, du sac et du coffre réunis.
    RessourcesManquantes(Vec<(Ressource, u32)>),
    /// Le héros porte déjà ce parfum, jusqu'à sa prochaine expédition dans
    /// une zone. En boîte, comme l'espèce qui suit, pour que l'erreur reste
    /// légère.
    ParfumDejaPorte(Box<Parfum>),
    /// Cette espèce ne lâche aucune ressource.
    EspeceSansRessource(Box<Espece>),
}

impl fmt::Display for ErreurAtelier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurAtelier::AucunPecheur => write!(f, "personne ne prépare d'appâts au camp"),
            ErreurAtelier::RessourcesManquantes(_) => write!(f, "il manque des ressources"),
            ErreurAtelier::ParfumDejaPorte(_) => write!(f, "le héros porte déjà un parfum"),
            ErreurAtelier::EspeceSansRessource(espece) => {
                write!(f, "{} ne lâche aucune ressource", espece.nom())
            }
        }
    }
}

impl std::error::Error for ErreurAtelier {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::donnees_de_test::espece;
    use crate::ressource::RessourceId;

    fn matiere(id: &str) -> Ressource {
        Ressource::new(RessourceId::new(id), id, 5)
    }

    fn atelier() -> AtelierDAnsgar {
        AtelierDAnsgar::new(
            Appat::TOUS
                .into_iter()
                .map(|appat| (appat, vec![(matiere("vers"), 2)]))
                .collect(),
            vec![(matiere("chair"), 3)],
            vec![(matiere("ambre"), 1)],
        )
        .unwrap()
    }

    #[test]
    fn chaque_appat_a_sa_recette() {
        assert_eq!(
            AtelierDAnsgar::new(
                vec![(Appat::Vers, vec![(matiere("vers"), 2)])],
                vec![(matiere("chair"), 3)],
                Vec::new(),
            ),
            Err(ErreurParametresAtelier::RecettesDAppatsIncompletes)
        );
        assert_eq!(
            AtelierDAnsgar::new(
                Appat::TOUS
                    .into_iter()
                    .map(|appat| (appat, vec![(matiere("vers"), 0)]))
                    .collect(),
                vec![(matiere("chair"), 3)],
                Vec::new(),
            ),
            Err(ErreurParametresAtelier::CoutInvalide)
        );
        assert_eq!(atelier().cout_de_l_appat(Appat::Ambre)[0].1, 2);
    }

    #[test]
    fn le_parfum_d_une_espece_demande_sa_ressource() {
        let atelier = atelier();
        let croc = matiere("croc");
        let loup = espece("loup").avec_ressource(croc.clone());
        assert_eq!(
            atelier.cout_du_parfum(&Parfum::Espece(loup)),
            Some(vec![(matiere("ambre"), 1), (croc, 3)])
        );
        assert_eq!(
            atelier.cout_du_parfum(&Parfum::Espece(espece("elfe"))),
            None
        );
        assert_eq!(
            atelier.cout_du_parfum(&Parfum::Charogne),
            Some(vec![(matiere("chair"), 3)])
        );
    }
}
