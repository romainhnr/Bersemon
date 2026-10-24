//! Tour sans fond (section 9.5) : un lieu hors de la carte, aux étapes sans
//! fin, où chaque étape recrée sa zone selon la profondeur. C'est aussi le
//! seul lieu de l'expédition en groupe (section 15.5).

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::capsule::Capsule;
use crate::espece::Espece;
use crate::expedition::ParametresCoffre;
use crate::expedition_automatique::ParametresExpeditionAutomatique;
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::zone::{EspeceRare, Zone, ZoneId, creer_monstre_sauvage, tirer_entre};

/// À l'étape n, les monstres sont au niveau 4 + 2 x n, 100 au plus.
const NIVEAU_DE_DEPART: u32 = 4;
const NIVEAUX_PAR_ETAPE: u32 = 2;
/// Un gardien toutes les 10 étapes, 3 niveaux au-dessus de l'étape.
pub const ETAPES_ENTRE_DEUX_GARDIENS: u32 = 10;
const NIVEAUX_DU_GARDIEN: u32 = 3;
/// L'argent d'un coffre, de 5 x n à 8 x n.
const ARGENT_MIN_PAR_ETAPE: u32 = 5;
const ARGENT_MAX_PAR_ETAPE: u32 = 8;
/// Seul, des groupes de 1 à 3 monstres ; à deux, de 2 à 4 (section 9.5).
const GROUPE_SEUL: (u8, u8) = (1, 3);
const GROUPE_A_DEUX: (u8, u8) = (2, 4);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurTour {
    /// Les espèces de la Tour viennent des zones de la carte.
    AucuneZone,
}

impl fmt::Display for ErreurTour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurTour::AucuneZone => write!(f, "la Tour sans fond a besoin des zones de la carte"),
        }
    }
}

impl std::error::Error for ErreurTour {}

/// Les espèces d'une zone de la carte, qui descendent dans la Tour dès que
/// le niveau de l'étape atteint son niveau minimal.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Etage {
    niveau_min: Niveau,
    especes: Vec<Espece>,
    rares: Vec<EspeceRare>,
}

/// Records de profondeur d'un joueur : la dernière étape franchie, seul et
/// à deux (section 9.5).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RecordsDeLaTour {
    pub seul: u32,
    pub a_deux: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TourSansFond {
    id: ZoneId,
    nom: String,
    /// Mener cette zone au bout ouvre la Tour : la Lisière.
    ouverte_par: ZoneId,
    etages: Vec<Etage>,
    cache: EspeceRare,
    /// Ne sert jamais : la Tour ne se fait pas en expédition automatique.
    expedition_automatique: ParametresExpeditionAutomatique,
    entree: String,
    gardien: String,
    repos: String,
}

impl TourSansFond {
    /// Les espèces viennent de `zones`, dans leur ordre ; la première zone
    /// ouvre la Tour. `gardien` contient `{monstre}`, remplacé par le nom
    /// du gardien.
    pub fn new(
        id: ZoneId,
        nom: impl Into<String>,
        zones: &[Zone],
        cache: EspeceRare,
        entree: impl Into<String>,
        gardien: impl Into<String>,
        repos: impl Into<String>,
    ) -> Result<Self, ErreurTour> {
        let premiere = zones.first().ok_or(ErreurTour::AucuneZone)?;
        let etages = zones
            .iter()
            .map(|zone| Etage {
                niveau_min: zone.niveaux().0,
                especes: zone.especes().to_vec(),
                rares: zone.rares().to_vec(),
            })
            .collect();
        Ok(Self {
            id,
            nom: nom.into(),
            ouverte_par: premiere.id().clone(),
            etages,
            cache,
            expedition_automatique: premiere.expedition_automatique().clone(),
            entree: entree.into(),
            gardien: gardien.into(),
            repos: repos.into(),
        })
    }

    pub fn id(&self) -> &ZoneId {
        &self.id
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }

    pub fn ouverte_par(&self) -> &ZoneId {
        &self.ouverte_par
    }

    pub fn recit_de_l_entree(&self) -> &str {
        &self.entree
    }

    pub fn recit_du_gardien(&self, monstre: &str) -> String {
        self.gardien.replace("{monstre}", monstre)
    }

    pub fn recit_du_repos(&self) -> &str {
        &self.repos
    }

    pub fn niveau_de_l_etape(etape: u32) -> Niveau {
        niveau_plafonne(NIVEAU_DE_DEPART + NIVEAUX_PAR_ETAPE * etape)
    }

    pub fn a_un_gardien(etape: u32) -> bool {
        etape > 0 && etape % ETAPES_ENTRE_DEUX_GARDIENS == 0
    }

    /// La zone d'une étape : son niveau, les espèces des zones atteintes,
    /// chacune une fois, et ses coffres. `capsules` : celles des zones que
    /// le joueur a menées au bout.
    pub(crate) fn zone_de_l_etape(&self, etape: u32, a_deux: bool, capsules: Vec<Capsule>) -> Zone {
        let niveau = Self::niveau_de_l_etape(etape);
        let mut especes: Vec<Espece> = Vec::new();
        let mut rares: Vec<EspeceRare> = Vec::new();
        for etage in self.etages_atteints(niveau) {
            for espece in &etage.especes {
                if !especes.iter().any(|deja| deja.id() == espece.id()) {
                    especes.push(espece.clone());
                }
            }
            for rare in &etage.rares {
                if !rares
                    .iter()
                    .any(|deja| deja.espece.id() == rare.espece.id())
                {
                    rares.push(rare.clone());
                }
            }
        }
        let (taille_min, taille_max) = if a_deux { GROUPE_A_DEUX } else { GROUPE_SEUL };
        let coffre = ParametresCoffre::new(
            ARGENT_MIN_PAR_ETAPE * etape..=ARGENT_MAX_PAR_ETAPE * etape,
            capsules,
        )
        .expect("fourchette d'argent croissante");
        // Chaque espèce apprend une attaque au niveau 1 et vit dans une zone
        // dont le niveau minimal est atteint : elle se crée à ce niveau.
        Zone::new(
            self.id.clone(),
            self.nom.clone(),
            (niveau, niveau),
            1,
            taille_max,
            especes,
            rares,
            self.expedition_automatique.clone(),
            coffre,
        )
        .and_then(|zone| zone.avec_monstre_cache(self.cache.clone()))
        .map(|zone| zone.avec_taille_min_groupe(taille_min))
        .expect("zone de la Tour conforme aux règles")
    }

    /// Un monstre rare hostile des zones atteintes, au niveau de l'étape + 3,
    /// 100 au plus. Tirages : l'espèce, s'il y en a plusieurs, puis ceux
    /// d'un monstre sauvage. Sans rare hostile, une espèce commune.
    pub(crate) fn generer_gardien(&self, etape: u32, aleatoire: &mut dyn Aleatoire) -> Monstre {
        let niveau = Self::niveau_de_l_etape(etape);
        let mut candidats: Vec<&Espece> = Vec::new();
        for etage in self.etages_atteints(niveau) {
            for rare in etage.rares.iter().filter(|rare| !rare.pacifique) {
                if !candidats.iter().any(|deja| deja.id() == rare.espece.id()) {
                    candidats.push(&rare.espece);
                }
            }
        }
        if candidats.is_empty() {
            candidats = self
                .etages_atteints(niveau)
                .flat_map(|etage| etage.especes.iter())
                .collect();
        }
        let position = tirer_entre(0, (candidats.len() - 1) as u32, aleatoire) as usize;
        let niveau = niveau_plafonne(u32::from(niveau.valeur()) + NIVEAUX_DU_GARDIEN);
        creer_monstre_sauvage(candidats[position], niveau, aleatoire)
    }

    fn etages_atteints(&self, niveau: Niveau) -> impl Iterator<Item = &Etage> {
        self.etages
            .iter()
            .filter(move |etage| etage.niveau_min <= niveau)
    }
}

fn niveau_plafonne(valeur: u32) -> Niveau {
    let valeur = valeur.min(u32::from(Niveau::MAX));
    Niveau::new(valeur as u8).expect("niveau entre 1 et 100")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::donnees_de_test::{espece, parametres_coffre, parametres_expedition_automatique};

    fn zone(id: &str, niveau_min: u8, communes: &[&str], rares: &[(&str, bool)]) -> Zone {
        Zone::new(
            ZoneId::new(id),
            id,
            (
                Niveau::new(niveau_min).unwrap(),
                Niveau::new(niveau_min + 4).unwrap(),
            ),
            5,
            2,
            communes.iter().map(|id| espece(id)).collect(),
            rares
                .iter()
                .map(|(id, pacifique)| EspeceRare {
                    espece: espece(id),
                    pacifique: *pacifique,
                })
                .collect(),
            parametres_expedition_automatique(),
            parametres_coffre(),
        )
        .unwrap()
    }

    fn tour() -> TourSansFond {
        let zones = [
            zone("bois", 3, &["loup", "corbeau"], &[("cerf-blanc", true)]),
            zone("marais", 8, &["vipere", "loup"], &[("basilic", false)]),
            zone("vallee", 14, &["squelette"], &[("vouivre", false)]),
        ];
        TourSansFond::new(
            ZoneId::new("tour"),
            "Tour",
            &zones,
            EspeceRare {
                espece: espece("schnoz"),
                pacifique: true,
            },
            "entrée",
            "{monstre} garde l'escalier.",
            "repos",
        )
        .unwrap()
    }

    fn ids(especes: &[Espece]) -> Vec<&str> {
        especes.iter().map(|espece| espece.id().as_str()).collect()
    }

    #[test]
    fn le_niveau_monte_de_2_par_etape_jusqu_a_100() {
        let niveau = |etape| TourSansFond::niveau_de_l_etape(etape).valeur();
        assert_eq!(
            (niveau(1), niveau(5), niveau(10), niveau(20)),
            (6, 14, 24, 44)
        );
        assert_eq!((niveau(47), niveau(48), niveau(500)), (98, 100, 100));
    }

    #[test]
    fn un_gardien_toutes_les_10_etapes() {
        let gardiens: Vec<u32> = (0..=35)
            .filter(|etape| TourSansFond::a_un_gardien(*etape))
            .collect();
        assert_eq!(gardiens, vec![10, 20, 30]);
    }

    #[test]
    fn les_zones_atteintes_donnent_leurs_especes_une_fois() {
        let tour = tour();
        let etape_1 = tour.zone_de_l_etape(1, false, Vec::new());
        assert_eq!(ids(etape_1.especes()), vec!["loup", "corbeau"]);
        assert_eq!(etape_1.niveaux().0.valeur(), 6);
        assert_eq!(etape_1.id().as_str(), "tour");
        // Niveau 8 : le marais est atteint, le loup n'y compte qu'une fois.
        let etape_2 = tour.zone_de_l_etape(2, false, Vec::new());
        assert_eq!(ids(etape_2.especes()), vec!["loup", "corbeau", "vipere"]);
        let rares: Vec<&str> = etape_2
            .rares()
            .iter()
            .map(|rare| rare.espece.id().as_str())
            .collect();
        assert_eq!(rares, vec!["cerf-blanc", "basilic"]);
        assert!(etape_2.monstre_cache().is_some());
        assert_eq!(*etape_2.coffre().argent(), 10..=16);
    }

    #[test]
    fn a_deux_les_groupes_comptent_un_monstre_de_plus() {
        let tour = tour();
        assert_eq!(
            tour.zone_de_l_etape(1, false, Vec::new())
                .tailles_de_groupe(),
            (1, 3)
        );
        assert_eq!(
            tour.zone_de_l_etape(1, true, Vec::new())
                .tailles_de_groupe(),
            (2, 4)
        );
    }

    #[test]
    fn le_gardien_est_un_rare_hostile_trois_niveaux_au_dessus() {
        let tour = tour();
        // Niveau 24 : basilic et vouivre ; le tirage 1 donne la vouivre,
        // puis son shiny et ses valeurs internes, puis son habileté.
        let gardien =
            tour.generer_gardien(10, &mut AleatoireScripte::new([1, 2, 0, 0, 0, 0, 0, 0]));
        assert_eq!(gardien.espece().id().as_str(), "vouivre");
        assert_eq!(gardien.niveau().valeur(), 27);
        assert_eq!(
            tour.recit_du_gardien("Vouivre"),
            "Vouivre garde l'escalier."
        );
    }
}
