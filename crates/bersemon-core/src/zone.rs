//! Zones d'expédition (section 9) : les espèces qu'on y rencontre, leurs
//! niveaux et la taille des groupes sauvages.
//!
//! Comme ailleurs dans le noyau, aucun tirage n'est fait quand son issue est
//! certaine (une seule espèce possible, un seul niveau, groupes d'un seul
//! monstre).

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::espece::{Espece, EspeceId};
use crate::expedition::ParametresCoffre;
use crate::expedition_automatique::ParametresExpeditionAutomatique;
use crate::monstre::{Monstre, tirer_shiny};
use crate::niveau::Niveau;
use crate::stats::ValeursInternes;

/// Un monstre rare sur 20 est le monstre caché, dans toute zone qui en a un
/// (section 9.2).
pub const CHANCES_MONSTRE_CACHE: u32 = 20;

/// Identifiant stable d'une zone, enregistré dans la sauvegarde pour savoir
/// quelles zones sont explorées.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ZoneId(String);

impl ZoneId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Espèce rare d'une zone : selon l'espèce, le monstre attaque ou se laisse
/// prendre (section 9.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EspeceRare {
    pub espece: Espece,
    pub pacifique: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonstreRare {
    pub monstre: Monstre,
    pub pacifique: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurZone {
    NiveauxInverses,
    /// Une expédition compte au moins une étape.
    AucuneEtape,
    GroupeVide,
    AucuneEspece,
    /// L'espèce n'apprend aucune attaque, ou en apprend une en double,
    /// jusqu'à ce niveau de la zone.
    EspeceInvalide {
        espece: EspeceId,
        niveau: Niveau,
    },
}

impl fmt::Display for ErreurZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurZone::NiveauxInverses => {
                write!(f, "le niveau minimal dépasse le niveau maximal")
            }
            ErreurZone::AucuneEtape => write!(f, "une expédition compte au moins une étape"),
            ErreurZone::GroupeVide => write!(f, "un groupe sauvage compte au moins un monstre"),
            ErreurZone::AucuneEspece => write!(f, "une zone doit avoir au moins une espèce"),
            ErreurZone::EspeceInvalide { espece, niveau } => write!(
                f,
                "l'espèce {} ne peut pas être créée au niveau {}",
                espece.as_str(),
                niveau.valeur()
            ),
        }
    }
}

impl std::error::Error for ErreurZone {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zone {
    id: ZoneId,
    nom: String,
    niveau_min: Niveau,
    niveau_max: Niveau,
    /// Fixe et connu du joueur ; il suit la difficulté (section 9.2).
    etapes: u32,
    taille_max_groupe: u8,
    especes: Vec<Espece>,
    rares: Vec<EspeceRare>,
    /// Il remplace parfois un monstre rare (section 9.2).
    cache: Option<EspeceRare>,
    expedition_automatique: ParametresExpeditionAutomatique,
    coffre: ParametresCoffre,
}

impl Zone {
    /// Vérifie que chaque espèce, rares comprises, peut être créée à chaque
    /// niveau de la zone : la génération des monstres ne peut ensuite plus
    /// échouer.
    ///
    /// Tous les paramètres sont obligatoires, pour qu'aucune zone n'oublie
    /// une règle ; les regrouper ne servirait qu'à contourner la limite de
    /// clippy.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ZoneId,
        nom: impl Into<String>,
        niveaux: (Niveau, Niveau),
        etapes: u32,
        taille_max_groupe: u8,
        especes: Vec<Espece>,
        rares: Vec<EspeceRare>,
        expedition_automatique: ParametresExpeditionAutomatique,
        coffre: ParametresCoffre,
    ) -> Result<Self, ErreurZone> {
        let (niveau_min, niveau_max) = niveaux;
        if niveau_min > niveau_max {
            return Err(ErreurZone::NiveauxInverses);
        }
        if etapes == 0 {
            return Err(ErreurZone::AucuneEtape);
        }
        if taille_max_groupe == 0 {
            return Err(ErreurZone::GroupeVide);
        }
        if especes.is_empty() {
            return Err(ErreurZone::AucuneEspece);
        }
        let toutes = especes.iter().chain(rares.iter().map(|rare| &rare.espece));
        for espece in toutes {
            verifier_espece(espece, niveau_min, niveau_max)?;
        }
        Ok(Self {
            id,
            nom: nom.into(),
            niveau_min,
            niveau_max,
            etapes,
            taille_max_groupe,
            especes,
            rares,
            cache: None,
            expedition_automatique,
            coffre,
        })
    }

    /// Comme les autres espèces, le monstre caché doit pouvoir être créé à
    /// chaque niveau de la zone.
    pub fn avec_monstre_cache(mut self, cache: EspeceRare) -> Result<Self, ErreurZone> {
        verifier_espece(&cache.espece, self.niveau_min, self.niveau_max)?;
        self.cache = Some(cache);
        Ok(self)
    }

    pub fn id(&self) -> &ZoneId {
        &self.id
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }

    /// La difficulté d'une zone, affichée sur la carte, se lit à ses niveaux
    /// (section 9.1).
    pub fn niveaux(&self) -> (Niveau, Niveau) {
        (self.niveau_min, self.niveau_max)
    }

    /// Le nombre d'étapes de chaque expédition manuelle (section 9.2).
    pub fn etapes(&self) -> u32 {
        self.etapes
    }

    pub fn taille_max_groupe(&self) -> u8 {
        self.taille_max_groupe
    }

    pub fn especes(&self) -> &[Espece] {
        &self.especes
    }

    pub fn rares(&self) -> &[EspeceRare] {
        &self.rares
    }

    pub fn monstre_cache(&self) -> Option<&EspeceRare> {
        self.cache.as_ref()
    }

    /// Durée, dégâts et récompenses d'une expédition automatique (section
    /// 12.8).
    pub fn expedition_automatique(&self) -> &ParametresExpeditionAutomatique {
        &self.expedition_automatique
    }

    /// Ce que contiennent ses coffres, en plus des potions (section 14.5).
    pub fn coffre(&self) -> &ParametresCoffre {
        &self.coffre
    }

    /// Tirages : taille du groupe, puis pour chaque monstre ceux de
    /// `generer_monstre`.
    pub fn generer_groupe(&self, aleatoire: &mut dyn Aleatoire) -> Vec<Monstre> {
        let taille = tirer_entre(1, u32::from(self.taille_max_groupe), aleatoire);
        (0..taille)
            .map(|_| {
                let espece = &self.especes[tirer_position(self.especes.len(), aleatoire)];
                self.generer_monstre(espece, aleatoire)
            })
            .collect()
    }

    /// `None` si la zone n'a pas de monstre rare, ou seulement un monstre
    /// caché que le tirage n'a pas donné. Tirages : la chance du monstre
    /// caché, si la zone en a un ; à défaut de lui, l'espèce rare ; puis ceux
    /// de `generer_monstre`.
    pub fn generer_monstre_rare(&self, aleatoire: &mut dyn Aleatoire) -> Option<MonstreRare> {
        let rare = match &self.cache {
            Some(cache) if aleatoire.tirer(1..=CHANCES_MONSTRE_CACHE) == 1 => cache,
            _ if self.rares.is_empty() => return None,
            _ => &self.rares[tirer_position(self.rares.len(), aleatoire)],
        };
        Some(MonstreRare {
            monstre: self.generer_monstre(&rare.espece, aleatoire),
            pacifique: rare.pacifique,
        })
    }

    /// Tirages : niveau, shiny, puis valeurs internes.
    fn generer_monstre(&self, espece: &Espece, aleatoire: &mut dyn Aleatoire) -> Monstre {
        let valeur = tirer_entre(
            u32::from(self.niveau_min.valeur()),
            u32::from(self.niveau_max.valeur()),
            aleatoire,
        );
        // Le tirage reste entre deux niveaux valides de la zone.
        let niveau = Niveau::new(valeur as u8).expect("niveau compris dans la zone");
        let shiny = tirer_shiny(aleatoire);
        let valeurs_internes = ValeursInternes::tirer(aleatoire);
        Monstre::creer(espece.clone(), niveau, valeurs_internes, shiny)
            .expect("espèce vérifiée à chaque niveau par Zone::new")
    }
}

/// L'espèce doit apprendre au moins une attaque, et aucune en double, jusqu'à
/// chacun de ces niveaux.
fn verifier_espece(
    espece: &Espece,
    niveau_min: Niveau,
    niveau_max: Niveau,
) -> Result<(), ErreurZone> {
    for valeur in niveau_min.valeur()..=niveau_max.valeur() {
        let niveau = Niveau::new(valeur).map_err(|_| ErreurZone::NiveauxInverses)?;
        Monstre::creer(espece.clone(), niveau, ValeursInternes::default(), false).map_err(
            |_| ErreurZone::EspeceInvalide {
                espece: espece.id().clone(),
                niveau,
            },
        )?;
    }
    Ok(())
}

/// Aucun tirage quand la plage n'a qu'une valeur.
pub(crate) fn tirer_entre(min: u32, max: u32, aleatoire: &mut dyn Aleatoire) -> u32 {
    if min == max {
        min
    } else {
        aleatoire.tirer(min..=max)
    }
}

fn tirer_position(nombre: usize, aleatoire: &mut dyn Aleatoire) -> usize {
    tirer_entre(0, (nombre - 1) as u32, aleatoire) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::attaque::Categorie;
    use crate::donnees_de_test::{
        attaque, espece, parametres_coffre, parametres_expedition_automatique, stats_uniformes,
    };
    use crate::espece::{AttaqueApprise, Role};
    use crate::types::{Type, TypesMonstre};

    fn niveau(valeur: u8) -> Niveau {
        Niveau::new(valeur).unwrap()
    }

    fn zone(
        niveaux: (u8, u8),
        taille: u8,
        especes: Vec<Espece>,
        rares: Vec<EspeceRare>,
    ) -> Result<Zone, ErreurZone> {
        Zone::new(
            ZoneId::new("bois"),
            "Bois",
            (niveau(niveaux.0), niveau(niveaux.1)),
            4,
            taille,
            especes,
            rares,
            parametres_expedition_automatique(),
            parametres_coffre(),
        )
    }

    #[test]
    fn groupe_tire_taille_espece_niveau_shiny_et_valeurs_internes() {
        let zone = zone((3, 6), 2, vec![espece("loup"), espece("vipere")], vec![]).unwrap();
        let mut aleatoire = AleatoireScripte::new([
            2, // deux monstres
            1, 4, 512, 0, 0, 0, 0, 0, 0, // vipère niveau 4, non shiny
            0, 6, 1, 31, 31, 31, 31, 31, 31, // loup niveau 6, shiny
        ]);
        let groupe = zone.generer_groupe(&mut aleatoire);
        assert_eq!(groupe.len(), 2);
        assert_eq!(groupe[0].espece().id().as_str(), "vipere");
        assert_eq!(groupe[0].niveau(), niveau(4));
        assert!(!groupe[0].est_shiny());
        assert_eq!(groupe[1].espece().id().as_str(), "loup");
        assert_eq!(groupe[1].niveau(), niveau(6));
        assert!(groupe[1].est_shiny());
    }

    #[test]
    fn aucun_tirage_quand_tout_est_certain() {
        // Une seule espèce, un seul niveau, un monstre par groupe : seuls le
        // shiny et les valeurs internes sont tirés.
        let zone = zone((5, 5), 1, vec![espece("loup")], vec![]).unwrap();
        let mut aleatoire = AleatoireScripte::new([2, 0, 0, 0, 0, 0, 0]);
        assert_eq!(zone.generer_groupe(&mut aleatoire).len(), 1);
    }

    #[test]
    fn monstre_rare_selon_son_espece() {
        let rare = EspeceRare {
            espece: espece("cerf-blanc"),
            pacifique: true,
        };
        let zone = zone((5, 5), 1, vec![espece("loup")], vec![rare]).unwrap();
        let mut aleatoire = AleatoireScripte::new([2, 0, 0, 0, 0, 0, 0]);
        let rare = zone.generer_monstre_rare(&mut aleatoire).unwrap();
        assert_eq!(rare.monstre.espece().id().as_str(), "cerf-blanc");
        assert!(rare.pacifique);
        let sans_rare = self::zone((5, 5), 1, vec![espece("loup")], vec![]).unwrap();
        assert_eq!(
            sans_rare.generer_monstre_rare(&mut AleatoireScripte::new([])),
            None
        );
    }

    fn cache() -> EspeceRare {
        EspeceRare {
            espece: espece("schnoz"),
            pacifique: true,
        }
    }

    #[test]
    fn le_monstre_cache_remplace_un_monstre_rare_sur_20() {
        let rare = EspeceRare {
            espece: espece("cerf-blanc"),
            pacifique: true,
        };
        let zone = zone((5, 5), 1, vec![espece("loup")], vec![rare])
            .unwrap()
            .avec_monstre_cache(cache())
            .unwrap();
        // 1 sur 20 : le monstre caché, puis son shiny et ses valeurs internes.
        let mut aleatoire = AleatoireScripte::new([1, 2, 0, 0, 0, 0, 0, 0]);
        let monstre = zone.generer_monstre_rare(&mut aleatoire).unwrap();
        assert_eq!(monstre.monstre.espece().id().as_str(), "schnoz");
        // Au-delà, le seul monstre rare de la zone, sans tirage de position.
        let mut aleatoire = AleatoireScripte::new([2, 2, 0, 0, 0, 0, 0, 0]);
        let monstre = zone.generer_monstre_rare(&mut aleatoire).unwrap();
        assert_eq!(monstre.monstre.espece().id().as_str(), "cerf-blanc");
    }

    #[test]
    fn sans_autre_monstre_rare_le_monstre_cache_peut_manquer() {
        let zone = zone((5, 5), 1, vec![espece("loup")], vec![])
            .unwrap()
            .avec_monstre_cache(cache())
            .unwrap();
        assert_eq!(
            zone.generer_monstre_rare(&mut AleatoireScripte::new([20])),
            None
        );
        let mut aleatoire = AleatoireScripte::new([1, 2, 0, 0, 0, 0, 0, 0]);
        assert!(zone.generer_monstre_rare(&mut aleatoire).is_some());
    }

    #[test]
    fn monstre_cache_invalide_refuse() {
        let tardif = EspeceRare {
            espece: Espece::new(
                EspeceId::new("tardif"),
                "Tardif",
                TypesMonstre::new(&[Type::Feu]).unwrap(),
                Role::Soutien,
                stats_uniformes(50),
                64,
                vec![AttaqueApprise {
                    niveau: niveau(6),
                    attaque: attaque("coup", Type::Normal, Categorie::Physique, 40, 100),
                }],
            ),
            pacifique: true,
        };
        let zone = zone((5, 7), 1, vec![espece("loup")], vec![]).unwrap();
        assert_eq!(
            zone.avec_monstre_cache(tardif),
            Err(ErreurZone::EspeceInvalide {
                espece: EspeceId::new("tardif"),
                niveau: niveau(5),
            })
        );
    }

    #[test]
    fn zone_invalide_refusee() {
        assert_eq!(
            zone((6, 3), 1, vec![espece("loup")], vec![]),
            Err(ErreurZone::NiveauxInverses)
        );
        assert_eq!(
            zone((3, 6), 0, vec![espece("loup")], vec![]),
            Err(ErreurZone::GroupeVide)
        );
        assert_eq!(
            zone((3, 6), 1, vec![], vec![]),
            Err(ErreurZone::AucuneEspece)
        );
    }

    #[test]
    fn espece_sans_attaque_au_niveau_minimal_refusee() {
        // L'espèce n'apprend sa première attaque qu'au niveau 4.
        let tardive = Espece::new(
            EspeceId::new("tardive"),
            "Tardive",
            TypesMonstre::new(&[Type::Feu]).unwrap(),
            Role::Offensif,
            stats_uniformes(50),
            64,
            vec![AttaqueApprise {
                niveau: niveau(4),
                attaque: attaque("coup", Type::Normal, Categorie::Physique, 40, 100),
            }],
        );
        assert_eq!(
            zone((3, 6), 1, vec![tardive], vec![]),
            Err(ErreurZone::EspeceInvalide {
                espece: EspeceId::new("tardive"),
                niveau: niveau(3),
            })
        );
    }
}
