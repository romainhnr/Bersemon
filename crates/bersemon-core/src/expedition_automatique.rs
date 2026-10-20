//! Expéditions automatiques (section 12) : des monstres envoyés seuls dans
//! une zone déjà explorée, qui reviennent après une durée réelle.
//!
//! Les monstres partent du camp, donc avec tous leurs PV, et y sont soignés à
//! leur retour : les dégâts ne servent qu'à décider qui tombe KO et si
//! l'expédition réussit. Le résultat n'est tiré qu'à l'accueil de
//! l'expédition (section 12.7).

use std::fmt;
use std::ops::RangeInclusive;
use std::time::Duration;

use crate::aleatoire::Aleatoire;
use crate::collection::{Emplacement, ErreurCollection};
use crate::espece::Role;
use crate::habilete::{
    BONUS_D_INSTINCT, BONUS_DU_PILLARD, Habilete, REDUCTION_D_ECLAIREUR, majorer,
};
use crate::horodatage::Horodatage;
use crate::monstre::{Monstre, RapportExperience};
use crate::zone::{Zone, ZoneId, tirer_entre};

/// Une expédition automatique emmène de 1 à 3 monstres (section 12.1).
pub const MONSTRES_MIN: usize = 1;
pub const MONSTRES_MAX: usize = 3;

/// Chaque monstre en plus du premier réduit la durée de 25 % de la durée de
/// base (Q7).
const REDUCTION_PAR_MONSTRE_EN_PLUS: u64 = 25;

/// Chaque monstre de soutien envoyé avec le monstre protégé lui donne 25 %
/// de protection, un charme de garde 50 %, 100 % au plus (section 12.5).
const PROTECTION_PAR_SOUTIEN: u32 = 25;
pub const PROTECTION_DU_CHARME: u32 = 50;
const PROTECTION_MAXIMALE: u32 = 100;
/// Chaque monstre offensif rentré debout d'une réussite ajoute 10 % à
/// l'argent rapporté (section 12.6).
const BONUS_PAR_OFFENSIF: u32 = 10;

/// Ce que le joueur choisit au départ pour protéger un de ses monstres
/// (section 12.5) : sa position parmi les monstres envoyés, et un charme de
/// garde. Par défaut, aucune protection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Protection {
    pub protege: Option<usize>,
    pub charme: bool,
}

impl Protection {
    /// La part de ses dégâts que le monstre protégé reporte sur les autres :
    /// 25 % par autre monstre de soutien, 50 % avec un charme, 100 % au
    /// plus. Nulle sans monstre protégé.
    pub fn pourcentage(&self, monstres: &[Monstre]) -> u32 {
        let Some(protege) = self.protege else {
            return 0;
        };
        let soutiens = monstres
            .iter()
            .enumerate()
            .filter(|(position, monstre)| {
                *position != protege && monstre.espece().role() == Role::Soutien
            })
            .count() as u32;
        let charme = if self.charme { PROTECTION_DU_CHARME } else { 0 };
        (soutiens * PROTECTION_PAR_SOUTIEN + charme).min(PROTECTION_MAXIMALE)
    }

    /// Le monstre protégé et sa protection, si elle n'est pas nulle.
    fn effective(&self, monstres: &[Monstre]) -> Option<(usize, u32)> {
        let pourcentage = self.pourcentage(monstres);
        self.protege
            .filter(|_| pourcentage > 0)
            .map(|protege| (protege, pourcentage))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurParametres {
    DureeNulle,
    /// Le minimum d'une fourchette dépasse son maximum.
    FourchetteInversee,
}

impl fmt::Display for ErreurParametres {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurParametres::DureeNulle => write!(f, "la durée de base doit être positive"),
            ErreurParametres::FourchetteInversee => {
                write!(f, "le minimum d'une fourchette dépasse son maximum")
            }
        }
    }
}

impl std::error::Error for ErreurParametres {}

/// Valeurs propres à chaque zone (section 12.8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParametresExpeditionAutomatique {
    duree_de_base: Duration,
    degats: RangeInclusive<u32>,
    experience: RangeInclusive<u32>,
    potions: RangeInclusive<u32>,
    argent: RangeInclusive<u32>,
}

impl ParametresExpeditionAutomatique {
    /// La durée de base vaut pour un seul monstre.
    pub fn new(
        duree_de_base: Duration,
        degats: RangeInclusive<u32>,
        experience: RangeInclusive<u32>,
        potions: RangeInclusive<u32>,
        argent: RangeInclusive<u32>,
    ) -> Result<Self, ErreurParametres> {
        if duree_de_base.as_secs() == 0 {
            return Err(ErreurParametres::DureeNulle);
        }
        let fourchettes = [&degats, &experience, &potions, &argent];
        if fourchettes.iter().any(|fourchette| fourchette.is_empty()) {
            return Err(ErreurParametres::FourchetteInversee);
        }
        Ok(Self {
            duree_de_base,
            degats,
            experience,
            potions,
            argent,
        })
    }

    pub fn duree_de_base(&self) -> Duration {
        self.duree_de_base
    }

    pub fn degats(&self) -> &RangeInclusive<u32> {
        &self.degats
    }

    /// Gain de chaque monstre non KO après une réussite (section 12.6).
    pub fn experience(&self) -> &RangeInclusive<u32> {
        &self.experience
    }

    pub fn potions(&self) -> &RangeInclusive<u32> {
        &self.potions
    }

    pub fn argent(&self) -> &RangeInclusive<u32> {
        &self.argent
    }

    /// Pour 1 à 3 monstres : 100 %, 75 % puis 50 % de la durée de base (Q7).
    /// La durée est fixe, pour être connue avant le départ (section 12.2).
    pub fn duree(&self, nombre_de_monstres: usize) -> Duration {
        let en_plus = nombre_de_monstres.saturating_sub(1) as u64;
        let pourcentage = 100u64.saturating_sub(REDUCTION_PAR_MONSTRE_EN_PLUS * en_plus);
        Duration::from_secs(self.duree_de_base.as_secs().saturating_mul(pourcentage) / 100)
    }

    /// La durée pour ces monstres, réduite encore de 20 % si l'un d'eux est
    /// Éclaireur ; deux Éclaireurs n'en valent qu'un (section 5.9).
    pub fn duree_pour(&self, monstres: &[Monstre]) -> Duration {
        let duree = self.duree(monstres.len());
        if monstres
            .iter()
            .any(|monstre| monstre.a_l_habilete(Habilete::Eclaireur))
        {
            Duration::from_secs(duree.as_secs() * (100 - REDUCTION_D_ECLAIREUR) / 100)
        } else {
            duree
        }
    }

    /// Calcul exact : chaque total de la fourchette de dégâts est aussi
    /// probable que les autres, et l'on compte ceux qui laissent un monstre
    /// debout, protection comprise (sections 12.4 et 12.5). Les fourchettes
    /// du contenu ne comptent que quelques centaines de valeurs.
    pub fn chances_de_reussite(&self, monstres: &[Monstre], protection: Protection) -> Chances {
        let pv = pv_max(monstres);
        let protection = protection.effective(monstres);
        let favorables = self
            .degats
            .clone()
            .filter(|&total| est_reussie(&repartir_degats(total, &pv, protection), &pv))
            .count();
        Chances {
            favorables: favorables as u64,
            possibles: u64::from(self.degats.end() - self.degats.start()) + 1,
        }
    }
}

/// Chances de réussite exactes, sur l'ensemble des tirages possibles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chances {
    pub favorables: u64,
    pub possibles: u64,
}

impl Chances {
    /// Arrondi à l'entier inférieur : 100 % veut dire une réussite certaine.
    pub fn pourcentage(self) -> u64 {
        self.favorables * 100 / self.possibles
    }
}

/// Ce que le joueur voit avant le départ, avec la difficulté de la zone et
/// sa fourchette de dégâts (section 12.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Estimation {
    pub duree: Duration,
    pub chances: Chances,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurExpeditionAutomatique {
    /// Aucune expédition manuelle n'y a été menée jusqu'au bout (section
    /// 12.1).
    ZoneNonExploree,
    /// Une seule expédition automatique à la fois par zone.
    DejaEnCours,
    NombreDeMonstres(usize),
    Collection(ErreurCollection),
    /// Aucune expédition automatique en cours dans cette zone.
    Inexistante,
    /// Un monstre ne se protège que s'il part avec au moins un autre, qui
    /// reprend sa part (section 12.5, Q96).
    ProtectionImpossible,
    /// Un charme sert à protéger un monstre désigné.
    CharmeSansProtege,
    /// Ni le sac ni le coffre ne contiennent de charme de garde.
    AucunCharme,
    PasEncoreRentree,
    /// Une expédition rentrée s'accueille, elle ne se rappelle plus.
    DejaRentree,
}

impl fmt::Display for ErreurExpeditionAutomatique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurExpeditionAutomatique::ZoneNonExploree => {
                write!(f, "la zone n'a pas encore été explorée")
            }
            ErreurExpeditionAutomatique::DejaEnCours => {
                write!(
                    f,
                    "une expédition automatique est déjà en cours dans cette zone"
                )
            }
            ErreurExpeditionAutomatique::NombreDeMonstres(nombre) => write!(
                f,
                "{nombre} monstres envoyés, il en faut de {MONSTRES_MIN} à {MONSTRES_MAX}"
            ),
            ErreurExpeditionAutomatique::Collection(erreur) => write!(f, "{erreur}"),
            ErreurExpeditionAutomatique::Inexistante => {
                write!(f, "aucune expédition automatique dans cette zone")
            }
            ErreurExpeditionAutomatique::ProtectionImpossible => {
                write!(f, "un monstre ne se protège qu'avec au moins un autre")
            }
            ErreurExpeditionAutomatique::CharmeSansProtege => {
                write!(f, "un charme protège un monstre désigné")
            }
            ErreurExpeditionAutomatique::AucunCharme => write!(f, "aucun charme de garde"),
            ErreurExpeditionAutomatique::PasEncoreRentree => {
                write!(f, "l'expédition n'est pas encore rentrée")
            }
            ErreurExpeditionAutomatique::DejaRentree => {
                write!(f, "l'expédition est déjà rentrée")
            }
        }
    }
}

impl std::error::Error for ErreurExpeditionAutomatique {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpeditionAutomatique {
    zone: Zone,
    monstres: Vec<Monstre>,
    protection: Protection,
    depart: Horodatage,
    retour: Horodatage,
}

impl ExpeditionAutomatique {
    /// Le départ passe par `Partie::envoyer_en_expedition_automatique`, qui
    /// vérifie la zone et les monstres envoyés.
    pub(crate) fn partir(
        zone: Zone,
        monstres: Vec<Monstre>,
        protection: Protection,
        maintenant: Horodatage,
    ) -> Self {
        let duree = zone.expedition_automatique().duree_pour(&monstres);
        Self {
            zone,
            monstres,
            protection,
            depart: maintenant,
            retour: maintenant.plus(duree),
        }
    }

    /// Reprise d'une sauvegarde : l'heure de retour reste celle fixée au
    /// départ.
    pub fn restaurer(
        zone: Zone,
        monstres: Vec<Monstre>,
        depart: Horodatage,
        retour: Horodatage,
    ) -> Result<Self, ErreurExpeditionAutomatique> {
        if !(MONSTRES_MIN..=MONSTRES_MAX).contains(&monstres.len()) {
            return Err(ErreurExpeditionAutomatique::NombreDeMonstres(
                monstres.len(),
            ));
        }
        Ok(Self {
            zone,
            monstres,
            protection: Protection::default(),
            depart,
            retour,
        })
    }

    /// Reprise d'une sauvegarde : le monstre protégé doit être parmi ceux
    /// envoyés, avec au moins un autre, et un charme ne va qu'avec lui.
    pub fn avec_protection(
        mut self,
        protection: Protection,
    ) -> Result<Self, ErreurExpeditionAutomatique> {
        verifier_protection(protection, self.monstres.len())?;
        self.protection = protection;
        Ok(self)
    }

    pub fn protection(&self) -> Protection {
        self.protection
    }

    pub fn zone(&self) -> &Zone {
        &self.zone
    }

    /// Dans l'ordre où le joueur les a choisis.
    pub fn monstres(&self) -> &[Monstre] {
        &self.monstres
    }

    pub fn depart(&self) -> Horodatage {
        self.depart
    }

    /// Fixée au départ : un rééquilibrage des durées ne change pas une
    /// expédition déjà partie.
    pub fn retour(&self) -> Horodatage {
        self.retour
    }

    pub fn est_rentree(&self, maintenant: Horodatage) -> bool {
        maintenant >= self.retour
    }

    pub fn temps_restant(&self, maintenant: Horodatage) -> Duration {
        maintenant.duree_jusqu_a(self.retour)
    }

    /// Rappel avant le terme : les monstres rentrent sans dégâts ni
    /// récompense (section 12.7).
    pub(crate) fn rappeler(self) -> Vec<Monstre> {
        self.monstres
    }

    /// Rend le bilan et les monstres, XP comprise, dans l'ordre de
    /// l'expédition ; `places` dit où chacun va, dans le même ordre (section
    /// 12.7). Tirages : total des dégâts,
    /// puis, après une réussite, le gain d'XP, le nombre de potions et
    /// l'argent. Instinct augmente l'XP de son monstre, et Pillard l'argent
    /// s'il rentre debout (section 5.9), comme chaque monstre offensif
    /// (section 12.6). Le monstre protégé reporte une part de ses dégâts sur
    /// les autres (section 12.5).
    pub(crate) fn resoudre(
        self,
        places: &[Emplacement],
        aleatoire: &mut dyn Aleatoire,
    ) -> (BilanExpeditionAutomatique, Vec<Monstre>) {
        let parametres = self.zone.expedition_automatique();
        let degats = tirer_dans(parametres.degats(), aleatoire);
        let pv = pv_max(&self.monstres);
        let protection = self.protection.effective(&self.monstres);
        let subis = repartir_degats(degats, &pv, protection);
        let reussite = est_reussie(&subis, &pv);
        let (experience, potions, mut argent) = if reussite {
            let experience = tirer_dans(parametres.experience(), aleatoire);
            let potions = tirer_dans(parametres.potions(), aleatoire);
            (
                experience,
                potions,
                tirer_dans(parametres.argent(), aleatoire),
            )
        } else {
            (0, 0, 0)
        };

        let mut monstres = self.monstres;
        let pillard_debout = monstres.iter().enumerate().any(|(index, monstre)| {
            subis[index] < pv[index] && monstre.a_l_habilete(Habilete::Pillard)
        });
        let argent_du_pillard = if pillard_debout {
            argent * BONUS_DU_PILLARD / 100
        } else {
            0
        };
        let offensifs_debout = monstres
            .iter()
            .enumerate()
            .filter(|(index, monstre)| {
                subis[*index] < pv[*index] && monstre.espece().role() == Role::Offensif
            })
            .count() as u32;
        // Les pourcentages s'ajoutent à celui de Pillard, tous comptés sur
        // l'argent tiré (Q96).
        let argent_des_offensifs = argent * offensifs_debout * BONUS_PAR_OFFENSIF / 100;
        argent += argent_du_pillard + argent_des_offensifs;
        let mut rentres = Vec::with_capacity(monstres.len());
        for (index, monstre) in monstres.iter_mut().enumerate() {
            let ko = subis[index] >= pv[index];
            // Le même gain, reçu en entier par chaque monstre debout, comme
            // l'XP d'un monstre vaincu (sections 7 et 12.6).
            let gain = if monstre.a_l_habilete(Habilete::Instinct) {
                majorer(experience, BONUS_D_INSTINCT)
            } else {
                experience
            };
            let rapport = (reussite && !ko).then(|| monstre.gagner_experience(gain));
            rentres.push(MonstreRentre {
                monstre: monstre.clone(),
                degats_subis: subis[index],
                ko,
                rapport,
                emplacement: places[index],
            });
        }
        let bilan = BilanExpeditionAutomatique {
            zone: self.zone.id().clone(),
            degats,
            reussite,
            potions,
            argent,
            argent_du_pillard,
            argent_des_offensifs,
            protection,
            monstres: rentres,
        };
        (bilan, monstres)
    }
}

/// Ce que le joueur découvre en accueillant l'expédition (section 12.7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BilanExpeditionAutomatique {
    pub zone: ZoneId,
    /// Total tiré dans la fourchette de la zone.
    pub degats: u32,
    /// Au moins un monstre est resté debout (section 12.3).
    pub reussite: bool,
    /// Ajoutées à celles du joueur ; aucune après un échec.
    pub potions: u32,
    /// Ajouté à celui du joueur ; rien après un échec (section 12.6). Il
    /// comprend `argent_du_pillard`.
    pub argent: u32,
    /// La part due à l'habileté Pillard (section 5.9).
    pub argent_du_pillard: u32,
    /// La part due aux monstres offensifs rentrés debout (section 12.6).
    pub argent_des_offensifs: u32,
    /// La position du monstre protégé et sa protection, si elle n'était pas
    /// nulle (section 12.5).
    pub protection: Option<(usize, u32)>,
    /// Dans l'ordre de l'expédition.
    pub monstres: Vec<MonstreRentre>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonstreRentre {
    /// Copie prise au retour, XP comprise.
    pub monstre: Monstre,
    pub degats_subis: u32,
    pub ko: bool,
    /// `None` pour un monstre KO ou après un échec. Les attaques en attente
    /// se choisissent comme après un combat (section 12.7).
    pub rapport: Option<RapportExperience>,
    /// Dans l'équipe s'il y avait de la place, sinon au camp (section 12.7).
    pub emplacement: Emplacement,
}

fn pv_max(monstres: &[Monstre]) -> Vec<u32> {
    monstres.iter().map(|monstre| monstre.stats().pv).collect()
}

fn tirer_dans(fourchette: &RangeInclusive<u32>, aleatoire: &mut dyn Aleatoire) -> u32 {
    tirer_entre(*fourchette.start(), *fourchette.end(), aleatoire)
}

/// Vérifie qu'un monstre protégé part avec au moins un autre, et qu'un
/// charme ne va qu'avec un monstre protégé (section 12.5).
pub(crate) fn verifier_protection(
    protection: Protection,
    nombre_de_monstres: usize,
) -> Result<(), ErreurExpeditionAutomatique> {
    match protection.protege {
        Some(protege) if protege >= nombre_de_monstres || nombre_de_monstres < 2 => {
            Err(ErreurExpeditionAutomatique::ProtectionImpossible)
        }
        None if protection.charme => Err(ErreurExpeditionAutomatique::CharmeSansProtege),
        _ => Ok(()),
    }
}

/// Dégâts subis par chaque monstre, dans l'ordre. Le total est réparti à
/// parts égales entre les monstres encore debout, en divisions entières. Un
/// monstre dont la part atteint ses PV tombe KO et n'encaisse que ses PV : le
/// reste est réparti de nouveau entre les autres (section 12.3). Le monstre
/// protégé, tant qu'un autre est debout, reporte sur eux le pourcentage de
/// sa part (section 12.5).
fn repartir_degats(total: u32, pv: &[u32], protection: Option<(usize, u32)>) -> Vec<u32> {
    let mut subis = vec![0; pv.len()];
    let mut debout: Vec<usize> = (0..pv.len()).collect();
    let mut restant = total;
    while !debout.is_empty() {
        let parts = parts(restant, &debout, protection);
        let tombes: Vec<usize> = debout
            .iter()
            .zip(&parts)
            .filter(|(index, part)| pv[**index] <= **part)
            .map(|(index, _)| *index)
            .collect();
        if tombes.is_empty() {
            for (index, part) in debout.into_iter().zip(parts) {
                subis[index] = part;
            }
            break;
        }
        for &index in &tombes {
            subis[index] = pv[index];
            // Les PV des monstres tombés ne dépassent pas leurs parts, dont
            // la somme ne dépasse pas ce qui restait.
            restant -= pv[index];
        }
        debout.retain(|index| !tombes.contains(index));
    }
    subis
}

/// La part de chaque monstre debout, dans l'ordre de `debout` : à parts
/// égales, puis le monstre protégé, s'il est debout avec d'autres, leur
/// reporte le pourcentage de la sienne, partagé à parts égales. Les restes
/// des divisions sont perdus.
fn parts(restant: u32, debout: &[usize], protection: Option<(usize, u32)>) -> Vec<u32> {
    let nombre = debout.len() as u32;
    let part = restant / nombre;
    let mut parts = vec![part; debout.len()];
    let protege = protection.and_then(|(protege, pourcentage)| {
        let position = debout.iter().position(|index| *index == protege)?;
        (nombre > 1).then_some((position, pourcentage))
    });
    if let Some((position, pourcentage)) = protege {
        let reportee = part * pourcentage / 100;
        let par_autre = reportee / (nombre - 1);
        for (rang, part) in parts.iter_mut().enumerate() {
            if rang == position {
                *part -= reportee;
            } else {
                *part += par_autre;
            }
        }
    }
    parts
}

fn est_reussie(subis: &[u32], pv: &[u32]) -> bool {
    subis.iter().zip(pv).any(|(subis, pv)| subis < pv)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::donnees_de_test::{espece, zone};
    use crate::niveau::Niveau;
    use crate::stats::ValeursInternes;

    /// Places de retour quelconques : la résolution les recopie seulement.
    const PLACES: [Emplacement; 2] = [Emplacement::Equipe(2), Emplacement::Camp(4)];

    fn parametres(degats: RangeInclusive<u32>) -> ParametresExpeditionAutomatique {
        ParametresExpeditionAutomatique::new(
            Duration::from_secs(3_600),
            degats,
            100..=200,
            0..=1,
            15..=30,
        )
        .unwrap()
    }

    /// Monstre de base 50 en PV, aux valeurs internes nulles : 20 PV au
    /// niveau 5, 30 au niveau 10.
    fn monstre(niveau: u8) -> Monstre {
        Monstre::creer(
            espece("loup"),
            Niveau::new(niveau).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    /// Une heure, 10 à 40 dégâts, 100 à 200 XP, 0 à 1 potion et 20 pièces
    /// d'argent.
    fn bois() -> Zone {
        zone("bois")
    }

    #[test]
    fn parametres_invalides_refuses() {
        assert_eq!(
            ParametresExpeditionAutomatique::new(Duration::ZERO, 1..=2, 1..=2, 0..=1, 0..=1),
            Err(ErreurParametres::DureeNulle)
        );
        assert_eq!(
            ParametresExpeditionAutomatique::new(
                Duration::from_secs(60),
                1..=2,
                RangeInclusive::new(2, 1),
                0..=1,
                0..=1
            ),
            Err(ErreurParametres::FourchetteInversee)
        );
        assert_eq!(
            ParametresExpeditionAutomatique::new(
                Duration::from_secs(60),
                1..=2,
                1..=2,
                0..=1,
                RangeInclusive::new(2, 1)
            ),
            Err(ErreurParametres::FourchetteInversee)
        );
    }

    #[test]
    fn chaque_monstre_en_plus_reduit_la_duree_de_25_pourcents() {
        let parametres = parametres(10..=40);
        assert_eq!(parametres.duree(1), Duration::from_secs(3_600));
        assert_eq!(parametres.duree(2), Duration::from_secs(2_700));
        assert_eq!(parametres.duree(3), Duration::from_secs(1_800));
    }

    #[test]
    fn degats_repartis_a_parts_egales_sans_ko() {
        assert_eq!(repartir_degats(41, &[30, 30], None), vec![20, 20]);
        assert_eq!(repartir_degats(1, &[30, 30], None), vec![0, 0]);
    }

    /// Exemple de la section 12.3 : une Salamandre de 26 PV et un Loup de 16.
    #[test]
    fn le_surplus_d_un_monstre_ko_passe_aux_autres() {
        let pv = [26, 16];
        assert_eq!(repartir_degats(40, &pv, None), vec![24, 16]);
        assert!(est_reussie(&repartir_degats(40, &pv, None), &pv));
        assert_eq!(repartir_degats(50, &pv, None), vec![26, 16]);
        assert!(!est_reussie(&repartir_degats(50, &pv, None), &pv));
    }

    #[test]
    fn report_en_cascade() {
        // 60 en trois parts de 20 : le monstre de 10 PV tombe, il reste 50
        // en deux parts de 25 ; celui de 25 PV tombe, le dernier en subit 25.
        assert_eq!(repartir_degats(60, &[10, 25, 40], None), vec![10, 25, 25]);
    }

    #[test]
    fn chances_exactes_sur_toute_la_fourchette() {
        let parametres = parametres(10..=40);
        // Un monstre de 20 PV seul résiste aux totaux de 10 à 19.
        let seul = parametres.chances_de_reussite(&[monstre(5)], Protection::default());
        assert_eq!(
            seul,
            Chances {
                favorables: 10,
                possibles: 31,
            }
        );
        assert_eq!(seul.pourcentage(), 32);
        // À deux (20 et 30 PV), seuls les totaux de 50 et plus échouent.
        let duo = parametres.chances_de_reussite(&[monstre(5), monstre(10)], Protection::default());
        assert_eq!(duo.pourcentage(), 100);
    }

    #[test]
    fn le_retour_est_fixe_au_depart() {
        let depart = Horodatage::depuis_secondes(10_000);
        let expedition = ExpeditionAutomatique::partir(
            bois(),
            vec![monstre(5), monstre(5)],
            Protection::default(),
            depart,
        );
        assert_eq!(expedition.retour(), Horodatage::depuis_secondes(12_700));
        let avant = Horodatage::depuis_secondes(12_699);
        assert!(!expedition.est_rentree(avant));
        assert_eq!(expedition.temps_restant(avant), Duration::from_secs(1));
        assert!(expedition.est_rentree(expedition.retour()));
        assert_eq!(
            expedition.temps_restant(Horodatage::depuis_secondes(20_000)),
            Duration::ZERO
        );
    }

    #[test]
    fn une_reussite_rapporte_l_xp_aux_monstres_debout_et_des_potions() {
        let depart = Horodatage::depuis_secondes(0);
        let expedition = ExpeditionAutomatique::partir(
            bois(),
            vec![monstre(5), monstre(10)],
            Protection::default(),
            depart,
        );
        // 30 dégâts en deux parts de 15 : personne ne tombe.
        let (bilan, monstres) =
            expedition.resoudre(&PLACES, &mut AleatoireScripte::new([30, 150, 1]));
        assert!(bilan.reussite);
        assert_eq!(bilan.degats, 30);
        assert_eq!(bilan.potions, 1);
        // 20 pièces d'argent, et 10 % de plus par Loup, offensif, debout.
        assert_eq!(bilan.argent, 24);
        assert_eq!(bilan.argent_des_offensifs, 4);
        assert_eq!(monstres.len(), 2);
        for (index, rentre) in bilan.monstres.iter().enumerate() {
            assert_eq!(rentre.degats_subis, 15);
            assert!(!rentre.ko);
            assert_eq!(rentre.emplacement, PLACES[index]);
            let rapport = rentre.rapport.as_ref().unwrap();
            assert_eq!(rapport.experience_gagnee, 150);
            assert_eq!(rentre.monstre, monstres[index]);
        }
        assert_eq!(monstres[0].experience(), 125 + 150);
    }

    #[test]
    fn un_monstre_ko_ne_gagne_pas_d_xp() {
        let depart = Horodatage::depuis_secondes(0);
        let expedition = ExpeditionAutomatique::partir(
            bois(),
            vec![monstre(5), monstre(10)],
            Protection::default(),
            depart,
        );
        // 40 en deux parts de 20 : le monstre de 20 PV tombe, l'autre subit
        // ses 20 dégâts et reste debout avec 10 PV.
        let (bilan, monstres) =
            expedition.resoudre(&PLACES, &mut AleatoireScripte::new([40, 100, 0]));
        assert!(bilan.reussite);
        assert!(bilan.monstres[0].ko);
        assert_eq!(bilan.monstres[0].rapport, None);
        assert_eq!(monstres[0].experience(), 125);
        assert!(!bilan.monstres[1].ko);
        assert_eq!(bilan.monstres[1].degats_subis, 20);
        assert_eq!(monstres[1].experience(), 1_000 + 100);
    }

    #[test]
    fn un_echec_ne_rapporte_rien_et_ne_tire_ni_xp_ni_potions() {
        let depart = Horodatage::depuis_secondes(0);
        let expedition =
            ExpeditionAutomatique::partir(bois(), vec![monstre(5)], Protection::default(), depart);
        let (bilan, monstres) = expedition.resoudre(&PLACES, &mut AleatoireScripte::new([25]));
        assert!(!bilan.reussite);
        assert_eq!(bilan.potions, 0);
        assert_eq!(bilan.argent, 0);
        assert!(bilan.monstres[0].ko);
        assert_eq!(bilan.monstres[0].rapport, None);
        // Les monstres reviennent soignés : ils n'ont jamais perdu de PV.
        assert!(!monstres[0].est_ko());
    }

    #[test]
    fn un_eclaireur_raccourcit_l_expedition_de_20_pour_cent() {
        let parametres = parametres(10..=40);
        let eclaireur = monstre(5).avec_habilete(Habilete::Eclaireur);
        // 45 minutes à deux, puis 20 % de moins : 36 minutes.
        assert_eq!(
            parametres.duree_pour(&[monstre(5), eclaireur.clone()]),
            Duration::from_secs(2_160)
        );
        // Deux Éclaireurs n'en valent qu'un.
        assert_eq!(
            parametres.duree_pour(&[eclaireur.clone(), eclaireur]),
            Duration::from_secs(2_160)
        );
        assert_eq!(
            parametres.duree_pour(&[monstre(5), monstre(5)]),
            Duration::from_secs(2_700)
        );
    }

    #[test]
    fn instinct_et_pillard_au_retour_d_une_reussite() {
        let depart = Horodatage::depuis_secondes(0);
        let monstres = vec![
            monstre(5).avec_habilete(Habilete::Instinct),
            monstre(10).avec_habilete(Habilete::Pillard),
        ];
        let expedition =
            ExpeditionAutomatique::partir(bois(), monstres, Protection::default(), depart);
        let (bilan, _) = expedition.resoudre(&PLACES, &mut AleatoireScripte::new([30, 150, 1]));
        // 150 XP, et 20 % de plus pour Instinct.
        let gains: Vec<u32> = bilan
            .monstres
            .iter()
            .map(|rentre| rentre.rapport.as_ref().unwrap().experience_gagnee)
            .collect();
        assert_eq!(gains, vec![180, 150]);
        // 20 pièces d'argent, 50 % de plus pour le Pillard debout, et 10 %
        // par Loup, offensif, debout.
        assert_eq!((bilan.argent, bilan.argent_du_pillard), (34, 10));
        assert_eq!(bilan.argent_des_offensifs, 4);
    }

    #[test]
    fn le_monstre_protege_reporte_une_part_de_ses_degats() {
        // L'exemple de la section 12.5 : un Loup protégé à 25 %, une
        // Mandragore et une Salamandre ; chacun devait subir 20.
        let protege = Some((0, 25));
        assert_eq!(
            repartir_degats(60, &[20, 30, 26], protege),
            vec![15, 22, 22]
        );
        // Sans protection, le Loup tombait.
        assert_eq!(repartir_degats(60, &[20, 30, 26], None), vec![20, 20, 20]);
        // À 100 %, il ne subit rien tant qu'un autre est debout ; seul, il
        // subit ce qui reste.
        assert_eq!(repartir_degats(25, &[20, 30], Some((0, 100))), vec![0, 24]);
        assert_eq!(repartir_degats(40, &[20, 30], Some((0, 100))), vec![10, 30]);
        assert_eq!(repartir_degats(70, &[20, 30], Some((0, 100))), vec![20, 30]);
        assert_eq!(repartir_degats(45, &[20, 30], Some((0, 100))), vec![15, 30]);
    }

    #[test]
    fn la_protection_vient_des_soutiens_et_du_charme() {
        let soutien = || {
            Monstre::creer(
                crate::donnees_de_test::espece_de_soutien("mandragore"),
                Niveau::new(5).unwrap(),
                ValeursInternes::default(),
                false,
            )
            .unwrap()
        };
        let monstres = vec![monstre(5), soutien(), soutien()];
        let protection = |protege, charme| Protection { protege, charme };
        assert_eq!(protection(None, false).pourcentage(&monstres), 0);
        assert_eq!(protection(Some(0), false).pourcentage(&monstres), 50);
        assert_eq!(protection(Some(0), true).pourcentage(&monstres), 100);
        // Le protégé ne compte pas dans sa propre protection.
        assert_eq!(protection(Some(1), false).pourcentage(&monstres), 25);
        assert_eq!(protection(Some(1), true).pourcentage(&monstres), 75);
        // Les chances en tiennent compte.
        let parametres = parametres(10..=60);
        let seuls = parametres.chances_de_reussite(&monstres[..2], Protection::default());
        let proteges = parametres.chances_de_reussite(&monstres[..2], protection(Some(1), true));
        assert!(proteges.favorables >= seuls.favorables);
        assert_eq!(
            verifier_protection(protection(Some(0), false), 1),
            Err(ErreurExpeditionAutomatique::ProtectionImpossible)
        );
        assert_eq!(
            verifier_protection(protection(None, true), 2),
            Err(ErreurExpeditionAutomatique::CharmeSansProtege)
        );
        assert_eq!(verifier_protection(protection(Some(1), true), 2), Ok(()));
    }

    #[test]
    fn le_bilan_dit_la_protection() {
        let depart = Horodatage::depuis_secondes(0);
        let expedition = ExpeditionAutomatique::partir(
            bois(),
            vec![monstre(5), monstre(10)],
            Protection {
                protege: Some(0),
                charme: true,
            },
            depart,
        );
        // 30 dégâts : 15 chacun, dont 7 que le premier reporte sur l'autre.
        let (bilan, _) = expedition.resoudre(&PLACES, &mut AleatoireScripte::new([30, 150, 1]));
        assert_eq!(bilan.protection, Some((0, 50)));
        assert_eq!(bilan.monstres[0].degats_subis, 8);
        assert_eq!(bilan.monstres[1].degats_subis, 22);
    }

    #[test]
    fn un_pillard_ko_ne_rapporte_rien_de_plus() {
        let depart = Horodatage::depuis_secondes(0);
        let monstres = vec![monstre(5).avec_habilete(Habilete::Pillard), monstre(10)];
        let expedition =
            ExpeditionAutomatique::partir(bois(), monstres, Protection::default(), depart);
        // 40 dégâts : le Pillard, à 20 PV, tombe.
        let (bilan, _) = expedition.resoudre(&PLACES, &mut AleatoireScripte::new([40, 100, 0]));
        assert!(bilan.reussite && bilan.monstres[0].ko);
        // Seul le Loup debout, offensif, ajoute ses 10 %.
        assert_eq!((bilan.argent, bilan.argent_du_pillard), (22, 0));
    }
}
