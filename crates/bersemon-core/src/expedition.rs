//! Expédition manuelle dans une zone (section 9.2) : une suite d'étapes,
//! chacune avec son événement.

use crate::aleatoire::Aleatoire;
use crate::monstre::Monstre;
use crate::zone::{MonstreRare, Zone};

/// Une expédition compte de 4 à 6 étapes (Q37).
const ETAPES_MIN: u32 = 4;
const ETAPES_MAX: u32 = 6;
/// Un coffre contient 1 à 2 potions (Q39).
const POTIONS_MIN: u32 = 1;
const POTIONS_MAX: u32 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvenementExpedition {
    Combat(Vec<Monstre>),
    Coffre { potions: u32 },
    MonstreRare(MonstreRare),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expedition {
    zone: Zone,
    /// Inconnu du joueur : il ne sait pas combien d'étapes il reste (Q37).
    nombre_etapes: u32,
    etapes_faites: u32,
    defaite: bool,
}

impl Expedition {
    /// Un tirage : le nombre d'étapes.
    pub fn partir(zone: Zone, aleatoire: &mut dyn Aleatoire) -> Self {
        Self {
            zone,
            nombre_etapes: aleatoire.tirer(ETAPES_MIN..=ETAPES_MAX),
            etapes_faites: 0,
            defaite: false,
        }
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

    /// Passe à l'étape suivante et tire son événement ; `None` si
    /// l'expédition est terminée. Tirages : nature de l'événement (combat
    /// 80 %, coffre 15 %, monstre rare 5 %, Q38), puis ceux de son contenu.
    /// Une zone sans monstre rare donne un combat à la place.
    pub fn etape_suivante(&mut self, aleatoire: &mut dyn Aleatoire) -> Option<EvenementExpedition> {
        if self.est_terminee() {
            return None;
        }
        self.etapes_faites += 1;
        let evenement = match aleatoire.tirer(1..=100) {
            1..=80 => EvenementExpedition::Combat(self.zone.generer_groupe(aleatoire)),
            81..=95 => EvenementExpedition::Coffre {
                potions: aleatoire.tirer(POTIONS_MIN..=POTIONS_MAX),
            },
            _ => match self.zone.generer_monstre_rare(aleatoire) {
                Some(rare) => EvenementExpedition::MonstreRare(rare),
                None => EvenementExpedition::Combat(self.zone.generer_groupe(aleatoire)),
            },
        };
        Some(evenement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::donnees_de_test::espece;
    use crate::niveau::Niveau;
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
            Some(EvenementExpedition::Coffre { potions: 2 })
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
    fn une_defaite_termine_l_expedition_sans_la_reussir() {
        let mut expedition = Expedition::partir(zone(false), &mut AleatoireScripte::new([4]));
        for _ in 0..4 {
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 1]));
        }
        expedition.noter_defaite();
        assert!(expedition.est_terminee());
        assert!(!expedition.est_reussie());
    }
}
