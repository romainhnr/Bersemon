//! Machines du camp (section 13.6), construites par Gunther comme des
//! améliorations : les pièges, qui prennent des ressources dans une zone
//! explorée, et la cour d'entraînement, où des monstres gagnent de l'XP. Les
//! deux comptent sur l'horloge réelle.
//!
//! Les rendements de chaque niveau sont des règles, fixées ici ; les coûts et
//! les durées de construction sont du contenu (docs/contenu-v1.md, section
//! 19).

use std::fmt;
use std::time::Duration;

use crate::aleatoire::Aleatoire;
use crate::collection::ErreurCollection;
use crate::experience::experience_requise;
use crate::habilete::{BONUS_D_INSTINCT, Habilete, majorer};
use crate::horodatage::Horodatage;
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::ressource::Ressource;
use crate::zone::{Zone, ZoneId, tirer_entre};

/// Au niveau 3 des pièges, une ressource sur 10 est la ressource rare de la
/// zone, s'il en existe une.
const UNE_RARE_SUR: u32 = 10;
const SECONDES_PAR_HEURE: u64 = 3_600;

/// Ce que donnent les pièges à un niveau.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReglesDesPieges {
    /// Le temps qu'il faut pour prendre une ressource.
    pub intervalle: Duration,
    /// Au-delà, les pièges ne prennent plus rien.
    pub stock_maximal: u32,
    /// La ressource rare de la zone peut tomber.
    pub avec_rares: bool,
}

/// `None` au niveau 0 : les pièges ne sont pas construits.
pub fn regles_des_pieges(niveau: u8) -> Option<ReglesDesPieges> {
    let heures = |heures: u64| Duration::from_secs(heures * SECONDES_PAR_HEURE);
    match niveau {
        0 => None,
        1 => Some(ReglesDesPieges {
            intervalle: heures(2),
            stock_maximal: 6,
            avec_rares: false,
        }),
        2 => Some(ReglesDesPieges {
            intervalle: heures(1),
            stock_maximal: 12,
            avec_rares: false,
        }),
        _ => Some(ReglesDesPieges {
            intervalle: heures(1),
            stock_maximal: 24,
            avec_rares: true,
        }),
    }
}

/// Ce que donne la cour d'entraînement à un niveau.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReglesDeLaCour {
    pub places: usize,
    pub experience_par_heure: u32,
}

/// `None` au niveau 0 : la cour n'est pas construite.
pub fn regles_de_la_cour(niveau: u8) -> Option<ReglesDeLaCour> {
    match niveau {
        0 => None,
        1 => Some(ReglesDeLaCour {
            places: 1,
            experience_par_heure: 50,
        }),
        2 => Some(ReglesDeLaCour {
            places: 2,
            experience_par_heure: 50,
        }),
        _ => Some(ReglesDeLaCour {
            places: 3,
            experience_par_heure: 100,
        }),
    }
}

/// Les pièges posés dans une zone, et l'heure d'où part leur compte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pieges {
    zone: ZoneId,
    depuis: Horodatage,
}

impl Pieges {
    pub fn new(zone: ZoneId, depuis: Horodatage) -> Self {
        Self { zone, depuis }
    }

    pub fn zone(&self) -> &ZoneId {
        &self.zone
    }

    pub fn depuis(&self) -> Horodatage {
        self.depuis
    }

    /// Les ressources prises à cette heure, au plus le stock maximal.
    pub fn stock(&self, regles: &ReglesDesPieges, maintenant: Horodatage) -> u32 {
        let ecoule = self.depuis.duree_jusqu_a(maintenant).as_secs();
        let prises = ecoule / regles.intervalle.as_secs().max(1);
        u32::try_from(prises)
            .unwrap_or(u32::MAX)
            .min(regles.stock_maximal)
    }

    /// Le temps qu'il reste avant la prochaine ressource ; `None` quand les
    /// pièges sont pleins.
    pub fn prochaine(&self, regles: &ReglesDesPieges, maintenant: Horodatage) -> Option<Duration> {
        if self.stock(regles, maintenant) >= regles.stock_maximal {
            return None;
        }
        let intervalle = regles.intervalle.as_secs().max(1);
        let ecoule = self.depuis.duree_jusqu_a(maintenant).as_secs();
        Some(Duration::from_secs(intervalle - ecoule % intervalle))
    }

    /// Vide le stock et le rend. Le compte reprend où il en était, sauf si
    /// les pièges étaient pleins : il repart alors de maintenant (Q95).
    fn vider(&mut self, regles: &ReglesDesPieges, maintenant: Horodatage) -> u32 {
        let stock = self.stock(regles, maintenant);
        if stock >= regles.stock_maximal {
            self.depuis = maintenant;
        } else {
            self.depuis = self.depuis.plus(regles.intervalle.saturating_mul(stock));
        }
        stock
    }
}

/// Un monstre dans la cour, et l'heure où il y est entré.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entrainement {
    monstre: Monstre,
    depuis: Horodatage,
}

impl Entrainement {
    pub fn new(monstre: Monstre, depuis: Horodatage) -> Self {
        Self { monstre, depuis }
    }

    pub fn monstre(&self) -> &Monstre {
        &self.monstre
    }

    pub fn depuis(&self) -> Horodatage {
        self.depuis
    }

    /// L'XP gagnée jusqu'à cette heure, comptée à la seconde près, avec les
    /// 20 % d'Instinct, sans dépasser l'XP du niveau `plafond` (section
    /// 13.6).
    pub fn experience(
        &self,
        regles: &ReglesDeLaCour,
        plafond: Niveau,
        maintenant: Horodatage,
    ) -> u32 {
        let secondes = self.depuis.duree_jusqu_a(maintenant).as_secs();
        let brute = u64::from(regles.experience_par_heure) * secondes / SECONDES_PAR_HEURE;
        let brute = u32::try_from(brute).unwrap_or(u32::MAX);
        let gain = if self.monstre.a_l_habilete(Habilete::Instinct) {
            majorer(brute, BONUS_D_INSTINCT)
        } else {
            brute
        };
        gain.min(reste_avant_le_plafond(&self.monstre, plafond))
    }
}

/// L'XP qu'il peut encore gagner à l'entraînement ; nulle à son plafond.
pub fn reste_avant_le_plafond(monstre: &Monstre, plafond: Niveau) -> u32 {
    experience_requise(plafond).saturating_sub(monstre.experience())
}

/// L'état des machines : les pièges posés, s'ils le sont, et les monstres
/// à l'entraînement, dans l'ordre où ils y sont entrés.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Machines {
    pieges: Option<Pieges>,
    cour: Vec<Entrainement>,
}

impl Machines {
    pub fn restaurer(pieges: Option<Pieges>, cour: Vec<Entrainement>) -> Self {
        Self { pieges, cour }
    }

    pub fn pieges(&self) -> Option<&Pieges> {
        self.pieges.as_ref()
    }

    pub fn cour(&self) -> &[Entrainement] {
        &self.cour
    }

    pub(crate) fn poser(&mut self, zone: ZoneId, maintenant: Horodatage) {
        self.pieges = Some(Pieges::new(zone, maintenant));
    }

    /// Vide les pièges et tire chaque ressource prise : 1 fois sur 10 la
    /// ressource rare de la zone, si les règles le permettent et qu'il en
    /// existe une, sinon une de ses ressources communes. Tirages, pour
    /// chaque ressource : la chance de la rare, s'il peut y en avoir une, et
    /// laquelle s'il y en a plusieurs ; sinon, laquelle des communes. Rend
    /// les ressources, rangées par sorte dans l'ordre de leur première
    /// prise.
    pub(crate) fn relever(
        &mut self,
        regles: &ReglesDesPieges,
        zone: &Zone,
        maintenant: Horodatage,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<Vec<(Ressource, u32)>, ErreurMachine> {
        let pieges = self.pieges.as_mut().ok_or(ErreurMachine::PiegesNonPoses)?;
        if pieges.zone() != zone.id() {
            return Err(ErreurMachine::ZoneDesPieges);
        }
        if pieges.stock(regles, maintenant) == 0 {
            return Err(ErreurMachine::PiegesVides);
        }
        let communes = ressources_communes(zone);
        let rares: Vec<Ressource> = if regles.avec_rares {
            ressources_rares(zone)
        } else {
            Vec::new()
        };
        let stock = pieges.vider(regles, maintenant);
        let mut prises: Vec<(Ressource, u32)> = Vec::new();
        for _ in 0..stock {
            let rare = !rares.is_empty() && aleatoire.tirer(1..=UNE_RARE_SUR) == 1;
            let parmi = if rare { &rares } else { &communes };
            let Some(derniere) = parmi.len().checked_sub(1) else {
                continue;
            };
            let ressource = &parmi[tirer_entre(0, derniere as u32, aleatoire) as usize];
            match prises
                .iter_mut()
                .find(|(prise, _)| prise.id() == ressource.id())
            {
                Some((_, nombre)) => *nombre += 1,
                None => prises.push((ressource.clone(), 1)),
            }
        }
        Ok(prises)
    }

    pub(crate) fn entrer(&mut self, monstre: Monstre, maintenant: Horodatage) {
        self.cour.push(Entrainement::new(monstre, maintenant));
    }

    pub(crate) fn sortir(&mut self, position: usize) -> Result<Entrainement, ErreurMachine> {
        if position >= self.cour.len() {
            return Err(ErreurMachine::AucunMonstreALaPlace(position));
        }
        Ok(self.cour.remove(position))
    }
}

/// Celles des espèces communes de la zone, sans doublon, dans l'ordre des
/// espèces.
fn ressources_communes(zone: &Zone) -> Vec<Ressource> {
    let mut ressources: Vec<Ressource> = Vec::new();
    for ressource in zone
        .especes()
        .iter()
        .filter_map(|espece| espece.ressource())
    {
        if !ressources.iter().any(|autre| autre.id() == ressource.id()) {
            ressources.push(ressource.clone());
        }
    }
    ressources
}

/// Celles des espèces rares de la zone qui en lâchent une : un monstre
/// pacifique ne se combat pas et n'en a pas (section 14.3).
fn ressources_rares(zone: &Zone) -> Vec<Ressource> {
    zone.rares()
        .iter()
        .filter_map(|rare| rare.espece.ressource().cloned())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurMachine {
    /// Gunther ne l'a pas encore construite.
    NonConstruite,
    /// Les pièges se posent dans une zone explorée (section 9.2).
    ZoneNonExploree,
    DejaPosesIci,
    PiegesNonPoses,
    /// Les pièges ont un stock à relever avant d'être déplacés.
    PiegesARelever,
    /// Les pièges sont posés dans une autre zone que celle donnée.
    ZoneDesPieges,
    PiegesVides,
    CourPleine,
    /// Le monstre a déjà l'XP du niveau maximal de la zone ouverte la plus
    /// avancée.
    PlafondAtteint,
    AucunMonstreALaPlace(usize),
    Collection(ErreurCollection),
}

impl fmt::Display for ErreurMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurMachine::NonConstruite => write!(f, "machine pas encore construite"),
            ErreurMachine::ZoneNonExploree => write!(f, "zone pas encore explorée"),
            ErreurMachine::DejaPosesIci => write!(f, "les pièges sont déjà posés dans cette zone"),
            ErreurMachine::PiegesNonPoses => write!(f, "les pièges ne sont posés nulle part"),
            ErreurMachine::PiegesARelever => write!(f, "les pièges ont un stock à relever"),
            ErreurMachine::ZoneDesPieges => write!(f, "les pièges sont posés dans une autre zone"),
            ErreurMachine::PiegesVides => write!(f, "les pièges sont vides"),
            ErreurMachine::CourPleine => write!(f, "la cour d'entraînement est pleine"),
            ErreurMachine::PlafondAtteint => {
                write!(f, "le monstre a atteint son plafond d'entraînement")
            }
            ErreurMachine::AucunMonstreALaPlace(position) => {
                write!(f, "aucun monstre à la place {position} de la cour")
            }
            ErreurMachine::Collection(erreur) => write!(f, "{erreur}"),
        }
    }
}

impl std::error::Error for ErreurMachine {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::contenu::ContenuV1;
    use crate::stats::ValeursInternes;

    fn a(secondes: u64) -> Horodatage {
        Horodatage::depuis_secondes(secondes)
    }

    fn zone(id: &str) -> Zone {
        ContenuV1::new()
            .zones()
            .iter()
            .find(|zone| zone.id().as_str() == id)
            .unwrap()
            .clone()
    }

    #[test]
    fn les_pieges_se_remplissent_jusqu_au_stock_maximal() {
        let regles = regles_des_pieges(1).unwrap();
        let pieges = Pieges::new(ZoneId::new("lisiere"), a(1_000));
        assert_eq!(pieges.stock(&regles, a(1_000)), 0);
        assert_eq!(
            pieges.prochaine(&regles, a(1_000)),
            Some(Duration::from_secs(7_200))
        );
        assert_eq!(pieges.stock(&regles, a(1_000 + 7_199)), 0);
        assert_eq!(pieges.stock(&regles, a(1_000 + 7_200)), 1);
        assert_eq!(pieges.stock(&regles, a(1_000 + 7_200 * 6)), 6);
        assert_eq!(pieges.stock(&regles, a(1_000 + 7_200 * 9)), 6);
        assert_eq!(pieges.prochaine(&regles, a(1_000 + 7_200 * 9)), None);
        // Une horloge reculée ne prend rien.
        assert_eq!(pieges.stock(&regles, a(0)), 0);
    }

    #[test]
    fn apres_une_releve_le_compte_reprend_sauf_si_les_pieges_etaient_pleins() {
        let regles = regles_des_pieges(2).unwrap();
        let lisiere = zone("lisiere");
        let mut machines = Machines::default();
        machines.poser(lisiere.id().clone(), a(0));
        assert_eq!(
            machines.relever(&regles, &lisiere, a(3_599), &mut AleatoireScripte::new([])),
            Err(ErreurMachine::PiegesVides)
        );
        // Deux ressources et demie : la demie est gardée. Lisière : Croc de
        // loup, Plume de corbeau, Écaille de vipère.
        let prises = machines
            .relever(
                &regles,
                &lisiere,
                a(9_000),
                &mut AleatoireScripte::new([0, 0]),
            )
            .unwrap();
        assert_eq!(prises.len(), 1);
        assert_eq!(prises[0].0.id().as_str(), "croc-de-loup");
        assert_eq!(prises[0].1, 2);
        assert_eq!(machines.pieges().unwrap().depuis(), a(7_200));
        // Pleins : le compte repart de la relève.
        let tirages: Vec<u32> = (0..12).map(|rang| rang % 3).collect();
        let prises = machines
            .relever(
                &regles,
                &lisiere,
                a(100_000),
                &mut AleatoireScripte::new(tirages),
            )
            .unwrap();
        assert_eq!(prises.iter().map(|(_, nombre)| nombre).sum::<u32>(), 12);
        assert_eq!(prises.len(), 3);
        assert_eq!(machines.pieges().unwrap().depuis(), a(100_000));
        assert_eq!(
            machines.relever(
                &regles,
                &zone("marais"),
                a(200_000),
                &mut AleatoireScripte::new([])
            ),
            Err(ErreurMachine::ZoneDesPieges)
        );
    }

    #[test]
    fn au_niveau_3_une_ressource_sur_10_est_rare() {
        let regles = regles_des_pieges(3).unwrap();
        let marais = zone("marais");
        let mut machines = Machines::default();
        machines.poser(marais.id().clone(), a(0));
        // Deux ressources : la première rare (1), le Croc de basilic, seul
        // rare du Marais à en lâcher une, sans tirage de laquelle ; la
        // seconde commune (2), la première des communes (0).
        let prises = machines
            .relever(
                &regles,
                &marais,
                a(7_200),
                &mut AleatoireScripte::new([1, 2, 0]),
            )
            .unwrap();
        let ids: Vec<&str> = prises
            .iter()
            .map(|(ressource, _)| ressource.id().as_str())
            .collect();
        assert_eq!(ids, vec!["croc-de-basilic", "cendre-de-feu-follet"]);
        // Dans la Lisière, aucune rare : aucun tirage de la chance.
        let lisiere = zone("lisiere");
        machines.poser(lisiere.id().clone(), a(0));
        let prises = machines
            .relever(&regles, &lisiere, a(3_600), &mut AleatoireScripte::new([2]))
            .unwrap();
        assert_eq!(prises[0].0.id().as_str(), "ecaille-de-vipere");
    }

    #[test]
    fn l_entrainement_rapporte_une_xp_horaire_plafonnee() {
        let regles = regles_de_la_cour(1).unwrap();
        let loup = Monstre::creer(
            crate::donnees_de_test::espece("loup"),
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        let entrainement = Entrainement::new(loup.clone(), a(0));
        let plafond = Niveau::new(7).unwrap();
        // 50 XP par heure, à la seconde près.
        assert_eq!(entrainement.experience(&regles, plafond, a(1_800)), 25);
        assert_eq!(entrainement.experience(&regles, plafond, a(3_600 * 3)), 150);
        // Au niveau 5 (125 XP), le niveau 7 (343 XP) est à 218 XP.
        assert_eq!(
            entrainement.experience(&regles, plafond, a(3_600 * 100)),
            218
        );
        assert_eq!(reste_avant_le_plafond(&loup, Niveau::new(5).unwrap()), 0);
    }

    #[test]
    fn regles_par_niveau() {
        assert_eq!(regles_des_pieges(0), None);
        assert_eq!(regles_de_la_cour(0), None);
        assert_eq!(regles_de_la_cour(3).unwrap().places, 3);
        assert!(regles_des_pieges(3).unwrap().avec_rares);
    }
}
