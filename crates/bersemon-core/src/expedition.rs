//! Expédition manuelle dans une zone (section 9.2) : une suite d'étapes,
//! chacune avec son événement.

use std::fmt;
use std::ops::RangeInclusive;

use crate::aleatoire::Aleatoire;
use crate::expedition_automatique::ErreurParametres;
use crate::monstre::Monstre;
use crate::traque::Apotre;
use crate::zone::{MonstreRare, Zone, tirer_entre};

/// Une expédition compte de 4 à 6 étapes (Q37).
const ETAPES_MIN: u32 = 4;
const ETAPES_MAX: u32 = 6;
/// Un coffre contient 1 à 2 potions (Q39).
const POTIONS_MIN: u32 = 1;
const POTIONS_MAX: u32 = 2;

/// Contenu des coffres propre à chaque zone (section 14.5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParametresCoffre {
    argent: RangeInclusive<u32>,
}

impl ParametresCoffre {
    pub fn new(argent: RangeInclusive<u32>) -> Result<Self, ErreurParametres> {
        if argent.is_empty() {
            return Err(ErreurParametres::FourchetteInversee);
        }
        Ok(Self { argent })
    }

    pub fn argent(&self) -> &RangeInclusive<u32> {
        &self.argent
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvenementExpedition {
    Combat(Vec<Monstre>),
    /// Les potions rejoignent aussitôt celles du joueur ; l'argent ne lui est
    /// acquis qu'au retour au camp (section 9.3).
    Coffre {
        potions: u32,
        argent: u32,
    },
    MonstreRare(MonstreRare),
    /// Combat, sans fuite, contre l'Apôtre (section 9.4).
    Apotre(Monstre),
    /// Le marchand rencontré se réfugie aussitôt au camp (section 14.2).
    Marchand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurExpedition {
    ZoneFermee,
}

impl fmt::Display for ErreurExpedition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurExpedition::ZoneFermee => write!(f, "cette zone n'est pas encore sur la carte"),
        }
    }
}

impl std::error::Error for ErreurExpedition {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expedition {
    zone: Zone,
    /// Inconnu du joueur : il ne sait pas combien d'étapes il reste (Q37).
    nombre_etapes: u32,
    etapes_faites: u32,
    defaite: bool,
    /// L'Apôtre qui attend à la dernière étape, une fois son antre localisé.
    apotre: Option<Apotre>,
    /// Somme des coffres ouverts, perdue en cas de défaite (section 9.3).
    argent_trouve: u32,
    /// Tant qu'aucun marchand n'est au camp, une étape peut tirer sa
    /// rencontre, une fois au plus (section 9.2).
    marchand_possible: bool,
}

impl Expedition {
    /// Un tirage : le nombre d'étapes. Le départ passe par
    /// `Partie::partir_en_expedition`, qui vérifie que la zone est ouverte et
    /// place l'Apôtre au bout de l'expédition quand il le faut.
    pub(crate) fn partir(zone: Zone, aleatoire: &mut dyn Aleatoire) -> Self {
        Self {
            zone,
            nombre_etapes: aleatoire.tirer(ETAPES_MIN..=ETAPES_MAX),
            etapes_faites: 0,
            defaite: false,
            apotre: None,
            argent_trouve: 0,
            marchand_possible: false,
        }
    }

    pub(crate) fn vers_l_antre(mut self, apotre: Apotre) -> Self {
        self.apotre = Some(apotre);
        self
    }

    pub(crate) fn avec_marchand_possible(mut self) -> Self {
        self.marchand_possible = true;
        self
    }

    pub fn zone(&self) -> &Zone {
        &self.zone
    }

    pub fn etapes_faites(&self) -> u32 {
        self.etapes_faites
    }

    /// Plus d'étape à faire : après la dernière, ou après une défaite.
    pub fn est_terminee(&self) -> bool {
        self.defaite || self.etapes_faites >= self.nombre_etapes
    }

    /// Menée jusqu'à sa dernière étape sans défaite : la zone compte alors
    /// comme explorée (sections 9.2 et 9.3).
    pub fn est_reussie(&self) -> bool {
        !self.defaite && self.etapes_faites >= self.nombre_etapes
    }

    /// Une défaite renvoie au camp et met fin à l'expédition (section 9.3).
    pub fn noter_defaite(&mut self) {
        self.defaite = true;
    }

    pub fn argent_trouve(&self) -> u32 {
        self.argent_trouve
    }

    /// Ce que le joueur garde au retour au camp : rien après une défaite
    /// (section 9.3).
    pub fn argent_rapporte(&self) -> u32 {
        if self.defaite { 0 } else { self.argent_trouve }
    }

    /// Passe à l'étape suivante et tire son événement ; `None` si
    /// l'expédition est terminée. Tirages : nature de l'événement (combat
    /// 80 %, coffre 15 %, monstre rare 5 %, Q38 ; tant que le marchand peut
    /// être rencontré, combat 70 % et marchand 10 %, Q70), puis ceux de son
    /// contenu : pour un coffre, ses potions puis son argent.
    /// Une zone sans monstre rare donne un combat à la place. L'Apôtre
    /// remplace l'événement de la dernière étape, sans tirage de sa nature
    /// (Q57).
    pub fn etape_suivante(&mut self, aleatoire: &mut dyn Aleatoire) -> Option<EvenementExpedition> {
        if self.est_terminee() {
            return None;
        }
        self.etapes_faites += 1;
        let derniere = self.etapes_faites == self.nombre_etapes;
        if let Some(apotre) = self.apotre.as_ref().filter(|_| derniere) {
            return Some(EvenementExpedition::Apotre(apotre.generer(aleatoire)));
        }
        let evenement = match aleatoire.tirer(1..=100) {
            71..=80 if self.marchand_possible => {
                self.marchand_possible = false;
                EvenementExpedition::Marchand
            }
            1..=80 => EvenementExpedition::Combat(self.zone.generer_groupe(aleatoire)),
            81..=95 => self.tirer_un_coffre(aleatoire),
            _ => match self.zone.generer_monstre_rare(aleatoire) {
                Some(rare) => EvenementExpedition::MonstreRare(rare),
                None => EvenementExpedition::Combat(self.zone.generer_groupe(aleatoire)),
            },
        };
        Some(evenement)
    }

    fn tirer_un_coffre(&mut self, aleatoire: &mut dyn Aleatoire) -> EvenementExpedition {
        let potions = aleatoire.tirer(POTIONS_MIN..=POTIONS_MAX);
        let fourchette = self.zone.coffre().argent();
        let argent = tirer_entre(*fourchette.start(), *fourchette.end(), aleatoire);
        self.argent_trouve = self.argent_trouve.saturating_add(argent);
        EvenementExpedition::Coffre { potions, argent }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::donnees_de_test::{espece, parametres_coffre, parametres_expedition_automatique};
    use crate::niveau::Niveau;
    use crate::traque::Apotre;
    use crate::zone::{EspeceRare, ZoneId};

    /// Niveau 5 fixe et groupes d'un monstre : un monstre ne tire que son
    /// shiny et ses 6 valeurs internes.
    fn zone(avec_rare: bool) -> Zone {
        let rares = if avec_rare {
            vec![EspeceRare {
                espece: espece("cerf-blanc"),
                pacifique: true,
            }]
        } else {
            Vec::new()
        };
        let niveau = Niveau::new(5).unwrap();
        Zone::new(
            ZoneId::new("bois"),
            "Bois",
            (niveau, niveau),
            1,
            vec![espece("loup")],
            rares,
            parametres_expedition_automatique(),
            parametres_coffre(),
        )
        .unwrap()
    }

    const MONSTRE: [u32; 7] = [2, 0, 0, 0, 0, 0, 0];

    #[test]
    fn evenements_selon_le_tirage() {
        let mut expedition = Expedition::partir(zone(true), &mut AleatoireScripte::new([6]));
        let mut combat = vec![80];
        combat.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(combat)),
            Some(EvenementExpedition::Combat(groupe)) if groupe.len() == 1
        ));
        assert_eq!(
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 2])),
            Some(EvenementExpedition::Coffre {
                potions: 2,
                argent: 10
            })
        );
        let mut rare = vec![96];
        rare.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(rare)),
            Some(EvenementExpedition::MonstreRare(rare)) if rare.pacifique
        ));
        assert_eq!(expedition.etapes_faites(), 3);
    }

    #[test]
    fn sans_monstre_rare_la_zone_donne_un_combat() {
        let mut expedition = Expedition::partir(zone(false), &mut AleatoireScripte::new([4]));
        let mut tirages = vec![100];
        tirages.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(tirages)),
            Some(EvenementExpedition::Combat(_))
        ));
    }

    #[test]
    fn reussie_apres_la_derniere_etape() {
        let mut expedition = Expedition::partir(zone(false), &mut AleatoireScripte::new([4]));
        for _ in 0..4 {
            assert!(!expedition.est_terminee());
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 1]));
        }
        assert!(expedition.est_terminee());
        assert!(expedition.est_reussie());
        assert_eq!(
            expedition.etape_suivante(&mut AleatoireScripte::new([])),
            None
        );
    }

    #[test]
    fn l_apotre_attend_a_la_derniere_etape() {
        let apotre = Apotre::new(espece("baron"), &zone(false), "", "").unwrap();
        let mut expedition =
            Expedition::partir(zone(false), &mut AleatoireScripte::new([4])).vers_l_antre(apotre);
        for _ in 0..3 {
            assert!(matches!(
                expedition.etape_suivante(&mut AleatoireScripte::new([81, 1])),
                Some(EvenementExpedition::Coffre { .. })
            ));
        }
        // Seul le shiny de l'Apôtre est tiré.
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new([2])),
            Some(EvenementExpedition::Apotre(apotre)) if apotre.niveau() == Niveau::new(7).unwrap()
        ));
        assert!(expedition.est_terminee());
    }

    #[test]
    fn une_defaite_termine_l_expedition_sans_la_reussir() {
        let mut expedition = Expedition::partir(zone(false), &mut AleatoireScripte::new([4]));
        for _ in 0..4 {
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 1]));
        }
        expedition.noter_defaite();
        assert!(expedition.est_terminee());
        assert!(!expedition.est_reussie());
    }

    #[test]
    fn l_argent_des_coffres_est_tire_apres_les_potions() {
        let niveau = Niveau::new(5).unwrap();
        let zone = Zone::new(
            ZoneId::new("bois"),
            "Bois",
            (niveau, niveau),
            1,
            vec![espece("loup")],
            Vec::new(),
            parametres_expedition_automatique(),
            ParametresCoffre::new(10..=25).unwrap(),
        )
        .unwrap();
        let mut expedition = Expedition::partir(zone, &mut AleatoireScripte::new([4]));
        assert_eq!(
            expedition.etape_suivante(&mut AleatoireScripte::new([90, 1, 25])),
            Some(EvenementExpedition::Coffre {
                potions: 1,
                argent: 25
            })
        );
        expedition.etape_suivante(&mut AleatoireScripte::new([90, 2, 10]));
        assert_eq!(expedition.argent_trouve(), 35);
        assert_eq!(expedition.argent_rapporte(), 35);
    }

    #[test]
    fn une_defaite_fait_perdre_l_argent_trouve() {
        let mut expedition = Expedition::partir(zone(false), &mut AleatoireScripte::new([4]));
        expedition.etape_suivante(&mut AleatoireScripte::new([81, 1]));
        assert_eq!(expedition.argent_rapporte(), 10);
        expedition.noter_defaite();
        assert_eq!(expedition.argent_trouve(), 10);
        assert_eq!(expedition.argent_rapporte(), 0);
    }

    #[test]
    fn le_marchand_se_rencontre_une_fois_a_la_place_d_un_combat() {
        let mut expedition = Expedition::partir(zone(false), &mut AleatoireScripte::new([6]))
            .avec_marchand_possible();
        let mut combat = vec![70];
        combat.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(combat)),
            Some(EvenementExpedition::Combat(_))
        ));
        assert_eq!(
            expedition.etape_suivante(&mut AleatoireScripte::new([71])),
            Some(EvenementExpedition::Marchand)
        );
        // Il ne se rencontre qu'une fois.
        let mut combat = vec![80];
        combat.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(combat)),
            Some(EvenementExpedition::Combat(_))
        ));
    }

    #[test]
    fn sans_marchand_possible_le_tirage_donne_un_combat() {
        let mut expedition = Expedition::partir(zone(false), &mut AleatoireScripte::new([4]));
        let mut combat = vec![75];
        combat.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(combat)),
            Some(EvenementExpedition::Combat(_))
        ));
    }

    #[test]
    fn fourchette_d_argent_inversee_refusee() {
        assert_eq!(
            ParametresCoffre::new(RangeInclusive::new(2, 1)),
            Err(ErreurParametres::FourchetteInversee)
        );
    }
}
