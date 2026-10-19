//! Évolution des monstres (section 5.4) : un monstre change d'espèce, en
//! gardant ce qui lui est propre.
//!
//! L'évolution est toujours proposée au joueur, qui peut la refuser : ce
//! module dit si elle est possible et l'applique ; le déroulement la propose
//! après un combat, et la partie l'applique au camp.

use std::fmt;

use crate::attaque::{Attaque, AttaqueId};
use crate::catalogue::Catalogue;
use crate::collection::ErreurCollection;
use crate::espece::{Espece, EspeceId};
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::pierre::PierreDEvolution;

/// Ce qu'il faut à un monstre pour évoluer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionEvolution {
    /// Avoir atteint ce niveau.
    Niveau(Niveau),
    /// Recevoir cette pierre, au camp, à tout niveau.
    Pierre(PierreDEvolution),
}

/// L'évolution d'une espèce vers une autre, désignée par son identifiant :
/// chaque espèce reste ainsi légère, et une lignée se lit dans le catalogue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evolution {
    vers: EspeceId,
    condition: ConditionEvolution,
    /// L'attaque d'évolution, dans la liste d'apprentissage de l'espèce
    /// d'arrivée, apprise au moment d'évoluer.
    attaque: AttaqueId,
}

impl Evolution {
    pub fn new(vers: EspeceId, condition: ConditionEvolution, attaque: AttaqueId) -> Self {
        Self {
            vers,
            condition,
            attaque,
        }
    }

    pub fn vers(&self) -> &EspeceId {
        &self.vers
    }

    pub fn condition(&self) -> &ConditionEvolution {
        &self.condition
    }

    pub fn attaque(&self) -> &AttaqueId {
        &self.attaque
    }

    /// La pierre qu'il faut utiliser au camp, si c'est la condition.
    pub fn pierre(&self) -> Option<PierreDEvolution> {
        match self.condition {
            ConditionEvolution::Pierre(pierre) => Some(pierre),
            ConditionEvolution::Niveau(_) => None,
        }
    }

    /// Une condition de niveau se remplit au camp comme après un combat ;
    /// une pierre ne se donne qu'au camp, sur demande du joueur.
    fn est_remplie(&self, monstre: &Monstre) -> bool {
        match &self.condition {
            ConditionEvolution::Niveau(niveau) => monstre.niveau() >= *niveau,
            ConditionEvolution::Pierre(_) => false,
        }
    }
}

/// L'espèce en laquelle ce monstre peut évoluer maintenant, s'il en remplit
/// la condition, sans pierre.
pub fn evolution_possible(monstre: &Monstre, catalogue: &dyn Catalogue) -> Option<Espece> {
    let evolution = monstre.espece().evolution()?;
    if !evolution.est_remplie(monstre) {
        return None;
    }
    catalogue.espece(evolution.vers())
}

/// Ce qu'a fait une évolution, au-delà du changement d'espèce.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultatEvolution {
    /// L'attaque d'évolution, apprise sans rien oublier.
    pub attaque_apprise: Option<Attaque>,
    /// L'attaque d'évolution, quand le monstre en connaît déjà 4 : le joueur
    /// lui en fait oublier une ou y renonce, comme à la montée de niveau.
    pub attaque_en_attente: Option<Attaque>,
}

/// Fait évoluer le monstre vers `vers`, qui doit être l'évolution de son
/// espèce, et lui apprend l'attaque d'évolution s'il a la place et ne la
/// connaît pas déjà.
pub(crate) fn faire_evoluer(monstre: &mut Monstre, vers: Espece) -> ResultatEvolution {
    let attaque = monstre
        .espece()
        .evolution()
        .and_then(|evolution| vers.attaque_apprise(evolution.attaque()))
        .cloned();
    monstre.evoluer(vers);
    let mut resultat = ResultatEvolution {
        attaque_apprise: None,
        attaque_en_attente: None,
    };
    match attaque {
        Some(attaque) if !monstre.attaques().connait(attaque.id()) => {
            if monstre.attaques().est_complete() {
                resultat.attaque_en_attente = Some(attaque);
            } else if monstre.ajouter_attaque(attaque.clone()).is_ok() {
                resultat.attaque_apprise = Some(attaque);
            }
        }
        _ => {}
    }
    resultat
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurEvolution {
    Collection(ErreurCollection),
    /// Son espèce n'évolue pas, ou il n'en remplit pas la condition.
    ImpossiblePourLInstant,
    /// Ni le sac ni le coffre n'ont la pierre qu'il lui faut.
    PierreManquante(PierreDEvolution),
}

impl fmt::Display for ErreurEvolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurEvolution::Collection(erreur) => write!(f, "{erreur}"),
            ErreurEvolution::ImpossiblePourLInstant => {
                write!(f, "ce monstre ne peut pas évoluer pour l'instant")
            }
            ErreurEvolution::PierreManquante(pierre) => {
                write!(f, "il lui faut une {}", pierre.nom().to_lowercase())
            }
        }
    }
}

impl std::error::Error for ErreurEvolution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attaque::{AttaquesMonstre, Categorie};
    use crate::contenu::ContenuV1;
    use crate::donnees_de_test::attaque;
    use crate::habilete::Habilete;
    use crate::stats::ValeursInternes;
    use crate::types::Type;

    fn niveau(valeur: u8) -> Niveau {
        Niveau::new(valeur).unwrap()
    }

    fn salamandre(contenu: &ContenuV1, valeur: u8) -> Monstre {
        let espece = contenu.espece(&EspeceId::new("salamandre")).unwrap();
        Monstre::creer(espece, niveau(valeur), ValeursInternes::default(), false).unwrap()
    }

    #[test]
    fn possible_a_partir_du_niveau_de_la_condition() {
        let contenu = ContenuV1::new();
        assert_eq!(
            evolution_possible(&salamandre(&contenu, 15), &contenu),
            None
        );
        let vers = evolution_possible(&salamandre(&contenu, 16), &contenu).unwrap();
        assert_eq!(vers.id().as_str(), "lezard-de-braise");
        assert!(evolution_possible(&salamandre(&contenu, 40), &contenu).is_some());
        // Sa forme évoluée n'évolue pas encore (section 5.4).
        let mut lezard = salamandre(&contenu, 16);
        faire_evoluer(&mut lezard, vers);
        assert_eq!(evolution_possible(&lezard, &contenu), None);
    }

    #[test]
    fn garde_ce_qui_est_propre_au_monstre() {
        let contenu = ContenuV1::new();
        let valeurs = ValeursInternes::new(crate::stats::Stats {
            pv: 3,
            attaque: 7,
            defense: 11,
            attaque_speciale: 13,
            defense_speciale: 17,
            vitesse: 19,
        })
        .unwrap();
        let espece = contenu.espece(&EspeceId::new("salamandre")).unwrap();
        let mut monstre = Monstre::creer(espece, niveau(16), valeurs, false)
            .unwrap()
            .avec_habilete(Habilete::Sorcellerie);
        monstre.renommer(Some(crate::nom::Nom::new("Braise").unwrap()));
        monstre.subir_degats(10);
        let experience = monstre.experience();
        let pv_max = monstre.stats().pv;
        let pv = monstre.pv_actuels();
        let vers = evolution_possible(&monstre, &contenu).unwrap();
        faire_evoluer(&mut monstre, vers);
        assert_eq!(monstre.espece().id().as_str(), "lezard-de-braise");
        assert_eq!(monstre.nom(), "Braise");
        assert_eq!(monstre.niveau(), niveau(16));
        assert_eq!(monstre.experience(), experience);
        assert_eq!(monstre.valeurs_internes(), &valeurs);
        assert!(!monstre.est_shiny());
        // Seconde habileté de la Salamandre, seconde du Lézard de braise.
        assert_eq!(monstre.habilete(), Some(Habilete::Sorcellerie));
        assert_eq!(monstre.pv_actuels(), pv + monstre.stats().pv - pv_max);
    }

    #[test]
    fn un_shiny_le_reste() {
        let contenu = ContenuV1::new();
        let espece = contenu.espece(&EspeceId::new("salamandre")).unwrap();
        let mut monstre =
            Monstre::creer(espece, niveau(16), ValeursInternes::default(), true).unwrap();
        let vers = evolution_possible(&monstre, &contenu).unwrap();
        faire_evoluer(&mut monstre, vers);
        assert!(monstre.est_shiny());
        assert_eq!(monstre.valeurs_internes(), &ValeursInternes::maximales());
    }

    #[test]
    fn un_monstre_ko_le_reste() {
        let contenu = ContenuV1::new();
        let mut monstre = salamandre(&contenu, 16);
        monstre.subir_degats(u32::MAX);
        let vers = evolution_possible(&monstre, &contenu).unwrap();
        faire_evoluer(&mut monstre, vers);
        assert!(monstre.est_ko());
    }

    #[test]
    fn apprend_l_attaque_d_evolution_s_il_a_la_place() {
        let contenu = ContenuV1::new();
        let espece = contenu.espece(&EspeceId::new("salamandre")).unwrap();
        let attaques =
            AttaquesMonstre::new(vec![espece.apprentissage()[0].attaque.clone()]).unwrap();
        let mut monstre = Monstre::new(
            espece,
            niveau(16),
            ValeursInternes::default(),
            attaques,
            false,
        );
        let vers = evolution_possible(&monstre, &contenu).unwrap();
        let resultat = faire_evoluer(&mut monstre, vers);
        let apprise = resultat.attaque_apprise.unwrap();
        assert_eq!(apprise.id().as_str(), "jet-de-lave");
        assert_eq!(resultat.attaque_en_attente, None);
        assert!(monstre.attaques().connait(apprise.id()));
    }

    #[test]
    fn met_l_attaque_d_evolution_en_attente_s_il_en_connait_4() {
        let contenu = ContenuV1::new();
        let mut monstre = salamandre(&contenu, 16);
        assert!(monstre.attaques().est_complete());
        let avant = monstre.attaques().clone();
        let vers = evolution_possible(&monstre, &contenu).unwrap();
        let resultat = faire_evoluer(&mut monstre, vers);
        assert_eq!(resultat.attaque_apprise, None);
        assert_eq!(
            resultat
                .attaque_en_attente
                .map(|attaque| attaque.id().clone()),
            Some(AttaqueId::new("jet-de-lave"))
        );
        assert_eq!(monstre.attaques(), &avant);
    }

    #[test]
    fn n_apprend_pas_une_attaque_deja_connue() {
        let contenu = ContenuV1::new();
        let espece = contenu.espece(&EspeceId::new("salamandre")).unwrap();
        let vers = contenu.espece(&EspeceId::new("lezard-de-braise")).unwrap();
        let jet = vers
            .attaque_apprise(&AttaqueId::new("jet-de-lave"))
            .unwrap()
            .clone();
        let autre = attaque("autre", Type::Normal, Categorie::Physique, 40, 100);
        let attaques = AttaquesMonstre::new(vec![jet, autre]).unwrap();
        let mut monstre = Monstre::new(
            espece,
            niveau(16),
            ValeursInternes::default(),
            attaques,
            false,
        );
        let resultat = faire_evoluer(&mut monstre, vers);
        assert_eq!(resultat.attaque_apprise, None);
        assert_eq!(resultat.attaque_en_attente, None);
        assert_eq!(monstre.attaques().liste().len(), 2);
    }
}
