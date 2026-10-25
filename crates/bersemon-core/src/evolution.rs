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
use crate::jour_et_nuit::JourOuNuit;
use crate::meteo::Meteo;
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::pierre::PierreDEvolution;
use crate::zone::ZoneId;

/// Ce qu'il faut à un monstre pour évoluer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionEvolution {
    /// Avoir atteint ce niveau.
    Niveau(Niveau),
    /// Recevoir cette pierre, au camp, à tout niveau.
    Pierre(PierreDEvolution),
    /// Gagner un niveau dans cette zone, en expédition manuelle, à partir de
    /// ce niveau.
    Lieu { zone: ZoneId, niveau: Niveau },
    /// Avoir atteint ce niveau une fois cet Apôtre vaincu.
    ApotreVaincu { apotre: EspeceId, niveau: Niveau },
    /// Avoir atteint ce niveau, après un combat d'une expédition dans une
    /// zone de la carte, partie à ce moment (section 9.6).
    Moment {
        jour_ou_nuit: JourOuNuit,
        niveau: Niveau,
    },
    /// Avoir atteint ce niveau, après un combat d'une expédition partie sous
    /// cette météo (section 9.7).
    Meteo { meteo: Meteo, niveau: Niveau },
}

/// Où en est le monstre quand on se demande s'il peut évoluer.
#[derive(Debug, Clone, Copy)]
pub struct Circonstances<'a> {
    /// La zone de l'expédition manuelle en cours ; `None` au camp.
    pub zone: Option<&'a ZoneId>,
    pub apotres_vaincus: &'a [EspeceId],
    /// Le moment de l'expédition en cours ; `None` au camp et hors du cycle,
    /// dans la Tour sans fond (section 9.6).
    pub jour_ou_nuit: Option<JourOuNuit>,
    /// La météo de l'expédition en cours ; `None` au camp, par temps calme
    /// et hors des zones de la carte (section 9.7).
    pub meteo: Option<Meteo>,
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
            ConditionEvolution::Niveau(_)
            | ConditionEvolution::Lieu { .. }
            | ConditionEvolution::ApotreVaincu { .. }
            | ConditionEvolution::Moment { .. }
            | ConditionEvolution::Meteo { .. } => None,
        }
    }

    /// Une condition de niveau ou d'Apôtre vaincu se remplit au camp comme
    /// après un combat, une condition de lieu seulement après un combat dans
    /// sa zone, une condition de moment ou de météo seulement après un combat
    /// d'une expédition partie à ce moment ou sous cette météo ; une pierre
    /// ne se donne qu'au camp, sur demande du joueur.
    fn est_remplie(&self, monstre: &Monstre, circonstances: &Circonstances<'_>) -> bool {
        match &self.condition {
            ConditionEvolution::Niveau(niveau) => monstre.niveau() >= *niveau,
            ConditionEvolution::Pierre(_) => false,
            ConditionEvolution::Lieu { zone, niveau } => {
                circonstances.zone == Some(zone) && monstre.niveau() >= *niveau
            }
            ConditionEvolution::ApotreVaincu { apotre, niveau } => {
                circonstances.apotres_vaincus.contains(apotre) && monstre.niveau() >= *niveau
            }
            ConditionEvolution::Moment {
                jour_ou_nuit,
                niveau,
            } => circonstances.jour_ou_nuit == Some(*jour_ou_nuit) && monstre.niveau() >= *niveau,
            ConditionEvolution::Meteo { meteo, niveau } => {
                circonstances.meteo == Some(*meteo) && monstre.niveau() >= *niveau
            }
        }
    }
}

/// L'espèce en laquelle ce monstre peut évoluer maintenant, s'il en remplit
/// la condition, sans pierre.
pub fn evolution_possible(
    monstre: &Monstre,
    circonstances: &Circonstances<'_>,
    catalogue: &dyn Catalogue,
) -> Option<Espece> {
    let evolution = monstre.espece().evolution()?;
    if !evolution.est_remplie(monstre, circonstances) {
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

    const AU_CAMP: Circonstances<'static> = Circonstances {
        zone: None,
        apotres_vaincus: &[],
        jour_ou_nuit: None,
        meteo: None,
    };

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
            evolution_possible(&salamandre(&contenu, 15), &AU_CAMP, &contenu),
            None
        );
        let vers = evolution_possible(&salamandre(&contenu, 16), &AU_CAMP, &contenu).unwrap();
        assert_eq!(vers.id().as_str(), "lezard-de-braise");
        assert!(evolution_possible(&salamandre(&contenu, 40), &AU_CAMP, &contenu).is_some());
        // Sa forme évoluée n'évolue pas encore (section 5.4).
        let mut lezard = salamandre(&contenu, 16);
        faire_evoluer(&mut lezard, vers);
        assert_eq!(evolution_possible(&lezard, &AU_CAMP, &contenu), None);
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
        let vers = evolution_possible(&monstre, &AU_CAMP, &contenu).unwrap();
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
        let vers = evolution_possible(&monstre, &AU_CAMP, &contenu).unwrap();
        faire_evoluer(&mut monstre, vers);
        assert!(monstre.est_shiny());
        assert_eq!(monstre.valeurs_internes(), &ValeursInternes::maximales());
    }

    #[test]
    fn un_monstre_ko_le_reste() {
        let contenu = ContenuV1::new();
        let mut monstre = salamandre(&contenu, 16);
        monstre.subir_degats(u32::MAX);
        let vers = evolution_possible(&monstre, &AU_CAMP, &contenu).unwrap();
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
        let vers = evolution_possible(&monstre, &AU_CAMP, &contenu).unwrap();
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
        let vers = evolution_possible(&monstre, &AU_CAMP, &contenu).unwrap();
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

    fn creer(contenu: &ContenuV1, id: &str, valeur: u8) -> Monstre {
        let espece = contenu.espece(&EspeceId::new(id)).unwrap();
        Monstre::creer(espece, niveau(valeur), ValeursInternes::default(), false).unwrap()
    }

    #[test]
    fn l_armure_hantee_n_evolue_que_dans_la_vallee() {
        let contenu = ContenuV1::new();
        let vallee = ZoneId::new("vallee");
        let marais = ZoneId::new("marais");
        let dans = |zone| Circonstances {
            zone: Some(zone),
            apotres_vaincus: &[],
            jour_ou_nuit: None,
            meteo: None,
        };
        let armure = creer(&contenu, "armure-hantee", 18);
        let vers = evolution_possible(&armure, &dans(&vallee), &contenu).unwrap();
        assert_eq!(vers.id().as_str(), "chevalier-noir");
        assert_eq!(evolution_possible(&armure, &dans(&marais), &contenu), None);
        assert_eq!(evolution_possible(&armure, &AU_CAMP, &contenu), None);
        let jeune = creer(&contenu, "armure-hantee", 17);
        assert_eq!(evolution_possible(&jeune, &dans(&vallee), &contenu), None);
    }

    #[test]
    fn le_squelette_evolue_une_fois_le_baron_vaincu() {
        let contenu = ContenuV1::new();
        let baron = [EspeceId::new("baron-pendu")];
        let apres_le_baron = Circonstances {
            zone: None,
            apotres_vaincus: &baron,
            jour_ou_nuit: None,
            meteo: None,
        };
        let squelette = creer(&contenu, "squelette", 20);
        assert_eq!(evolution_possible(&squelette, &AU_CAMP, &contenu), None);
        let vers = evolution_possible(&squelette, &apres_le_baron, &contenu).unwrap();
        assert_eq!(vers.id().as_str(), "chevalier-squelette");
        let jeune = creer(&contenu, "squelette", 19);
        assert_eq!(evolution_possible(&jeune, &apres_le_baron, &contenu), None);
    }

    /// Le Loup évolue de nuit, la Vipère de jour, après un combat d'une
    /// expédition partie à ce moment ; jamais au camp (section 9.6).
    #[test]
    fn une_evolution_de_nuit_attend_une_expedition_de_nuit() {
        let contenu = ContenuV1::new();
        let lisiere = ZoneId::new("lisiere");
        let en_expedition = |jour_ou_nuit| Circonstances {
            zone: Some(&lisiere),
            apotres_vaincus: &[],
            jour_ou_nuit: Some(jour_ou_nuit),
            meteo: None,
        };
        let loup = creer(&contenu, "loup", 16);
        assert_eq!(evolution_possible(&loup, &AU_CAMP, &contenu), None);
        assert_eq!(
            evolution_possible(&loup, &en_expedition(JourOuNuit::Jour), &contenu),
            None
        );
        let vers = evolution_possible(&loup, &en_expedition(JourOuNuit::Nuit), &contenu).unwrap();
        assert_eq!(vers.id().as_str(), "warg");
        let jeune = creer(&contenu, "loup", 15);
        assert_eq!(
            evolution_possible(&jeune, &en_expedition(JourOuNuit::Nuit), &contenu),
            None
        );
        let corbeau = creer(&contenu, "corbeau", 16);
        let vers =
            evolution_possible(&corbeau, &en_expedition(JourOuNuit::Nuit), &contenu).unwrap();
        assert_eq!(vers.id().as_str(), "corbeau-des-gibets");
        let vipere = creer(&contenu, "vipere", 16);
        assert_eq!(
            evolution_possible(&vipere, &en_expedition(JourOuNuit::Nuit), &contenu),
            None
        );
        let vers = evolution_possible(&vipere, &en_expedition(JourOuNuit::Jour), &contenu).unwrap();
        assert_eq!(vers.id().as_str(), "aspic");
    }

    #[test]
    fn une_pierre_ne_se_remplit_jamais_seule() {
        let contenu = ContenuV1::new();
        let cerf = creer(&contenu, "cerf-blanc", 50);
        assert_eq!(evolution_possible(&cerf, &AU_CAMP, &contenu), None);
        assert_eq!(
            cerf.espece().evolution().and_then(Evolution::pierre),
            Some(PierreDEvolution::Lune)
        );
    }

    /// La Fée noire évolue sous la tempête au niveau 12, la Fée-sorcière
    /// sous la pluie au niveau 25 ; jamais au camp (section 9.7).
    #[test]
    fn la_fee_sorciere_evolue_une_deuxieme_fois_au_niveau_25() {
        let contenu = ContenuV1::new();
        let vallee = ZoneId::new("vallee");
        let sous = |meteo| Circonstances {
            zone: Some(&vallee),
            apotres_vaincus: &[],
            jour_ou_nuit: Some(JourOuNuit::Nuit),
            meteo: Some(meteo),
        };
        let mut fee = creer(&contenu, "fee-noire", 25);
        assert_eq!(evolution_possible(&fee, &AU_CAMP, &contenu), None);
        assert_eq!(
            evolution_possible(&fee, &sous(Meteo::Pluie), &contenu),
            None
        );
        let vers = evolution_possible(&fee, &sous(Meteo::Tempete), &contenu).unwrap();
        faire_evoluer(&mut fee, vers);
        assert_eq!(fee.espece().id().as_str(), "fee-sorciere");
        assert_eq!(
            evolution_possible(&fee, &sous(Meteo::Tempete), &contenu),
            None
        );
        let vers = evolution_possible(&fee, &sous(Meteo::Pluie), &contenu).unwrap();
        assert_eq!(vers.id().as_str(), "reine-des-ronces");
        faire_evoluer(&mut fee, vers);
        assert_eq!(
            evolution_possible(&fee, &sous(Meteo::Pluie), &contenu),
            None
        );
        let fee_sorciere = creer(&contenu, "fee-sorciere", 24);
        assert_eq!(
            evolution_possible(&fee_sorciere, &sous(Meteo::Pluie), &contenu),
            None
        );
    }

    #[test]
    fn le_basilic_evolue_sous_la_canicule() {
        let contenu = ContenuV1::new();
        let marais = ZoneId::new("marais");
        let sous = |meteo| Circonstances {
            zone: Some(&marais),
            apotres_vaincus: &[],
            jour_ou_nuit: Some(JourOuNuit::Jour),
            meteo,
        };
        let basilic = creer(&contenu, "basilic", 22);
        assert_eq!(evolution_possible(&basilic, &sous(None), &contenu), None);
        assert_eq!(
            evolution_possible(&basilic, &sous(Some(Meteo::Pluie)), &contenu),
            None
        );
        let vers = evolution_possible(&basilic, &sous(Some(Meteo::Canicule)), &contenu).unwrap();
        assert_eq!(vers.id().as_str(), "basilic-royal");
    }
}
