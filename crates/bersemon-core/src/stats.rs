//! Stats des monstres et leur calcul selon le niveau (section 5.8).

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::niveau::Niveau;

/// Une valeur pour chacune des six stats (Q16).
///
/// Une attaque physique oppose l'Attaque à la Défense, une attaque spéciale
/// l'Attaque spéciale à la Défense spéciale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stats<T> {
    pub pv: T,
    pub attaque: T,
    pub defense: T,
    pub attaque_speciale: T,
    pub defense_speciale: T,
    pub vitesse: T,
}

/// Une des six stats, pour désigner celle que vise un booster (section
/// 14.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stat {
    Pv,
    Attaque,
    Defense,
    AttaqueSpeciale,
    DefenseSpeciale,
    Vitesse,
}

impl Stat {
    /// Dans l'ordre des champs de `Stats`.
    pub const TOUTES: [Stat; 6] = [
        Stat::Pv,
        Stat::Attaque,
        Stat::Defense,
        Stat::AttaqueSpeciale,
        Stat::DefenseSpeciale,
        Stat::Vitesse,
    ];
}

impl<T> Stats<T> {
    pub fn de(&self, stat: Stat) -> &T {
        match stat {
            Stat::Pv => &self.pv,
            Stat::Attaque => &self.attaque,
            Stat::Defense => &self.defense,
            Stat::AttaqueSpeciale => &self.attaque_speciale,
            Stat::DefenseSpeciale => &self.defense_speciale,
            Stat::Vitesse => &self.vitesse,
        }
    }

    fn de_a_modifier(&mut self, stat: Stat) -> &mut T {
        match stat {
            Stat::Pv => &mut self.pv,
            Stat::Attaque => &mut self.attaque,
            Stat::Defense => &mut self.defense,
            Stat::AttaqueSpeciale => &mut self.attaque_speciale,
            Stat::DefenseSpeciale => &mut self.defense_speciale,
            Stat::Vitesse => &mut self.vitesse,
        }
    }
}

/// Stats propres à une espèce, à partir desquelles on calcule celles de
/// chaque monstre. u16 suffit largement et garantit que le calcul ne
/// déborde pas en u32.
pub type StatsDeBase = Stats<u16>;

/// Pourcentage ajouté à chaque stat, comme le bonus de la classe du héros
/// (section 4.2).
pub type BonusDeStats = Stats<u8>;

/// Les pourcentages s'additionnent : la classe et une habileté qui renforcent
/// la même stat donnent +20 % (section 5.9).
pub fn cumuler_bonus(premier: &BonusDeStats, second: &BonusDeStats) -> BonusDeStats {
    Stats {
        pv: premier.pv.saturating_add(second.pv),
        attaque: premier.attaque.saturating_add(second.attaque),
        defense: premier.defense.saturating_add(second.defense),
        attaque_speciale: premier
            .attaque_speciale
            .saturating_add(second.attaque_speciale),
        defense_speciale: premier
            .defense_speciale
            .saturating_add(second.defense_speciale),
        vitesse: premier.vitesse.saturating_add(second.vitesse),
    }
}

/// Chaque stat est augmentée de son pourcentage de bonus, arrondi à l'entier
/// inférieur.
pub fn appliquer_bonus(stats: &Stats<u32>, bonus: &BonusDeStats) -> Stats<u32> {
    let majorer = |valeur: u32, pourcentage: u8| {
        let majoree = u64::from(valeur) * (100 + u64::from(pourcentage)) / 100;
        u32::try_from(majoree).unwrap_or(u32::MAX)
    };
    Stats {
        pv: majorer(stats.pv, bonus.pv),
        attaque: majorer(stats.attaque, bonus.attaque),
        defense: majorer(stats.defense, bonus.defense),
        attaque_speciale: majorer(stats.attaque_speciale, bonus.attaque_speciale),
        defense_speciale: majorer(stats.defense_speciale, bonus.defense_speciale),
        vitesse: majorer(stats.vitesse, bonus.vitesse),
    }
}

/// Valeurs internes d'un monstre, de 0 à 31 pour chaque stat (Q17) : elles
/// rendent chaque monstre unique. Par défaut, toutes nulles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ValeursInternes(Stats<u8>);

impl ValeursInternes {
    pub const MAX: u8 = 31;

    pub fn new(valeurs: Stats<u8>) -> Result<Self, ErreurValeursInternes> {
        let toutes = [
            valeurs.pv,
            valeurs.attaque,
            valeurs.defense,
            valeurs.attaque_speciale,
            valeurs.defense_speciale,
            valeurs.vitesse,
        ];
        match toutes.into_iter().find(|&valeur| valeur > Self::MAX) {
            Some(valeur) => Err(ErreurValeursInternes { valeur }),
            None => Ok(Self(valeurs)),
        }
    }

    /// 31 partout : celles d'un shiny et d'un Apôtre (sections 5.1 et 9.4).
    pub fn maximales() -> Self {
        let max = Self::MAX;
        Self(Stats {
            pv: max,
            attaque: max,
            defense: max,
            attaque_speciale: max,
            defense_speciale: max,
            vitesse: max,
        })
    }

    /// Tire chaque valeur uniformément de 0 à 31, dans l'ordre des champs de
    /// `Stats`.
    pub fn tirer(aleatoire: &mut dyn Aleatoire) -> Self {
        let mut tirer_une = || aleatoire.tirer(0..=u32::from(Self::MAX)) as u8;
        Self(Stats {
            pv: tirer_une(),
            attaque: tirer_une(),
            defense: tirer_une(),
            attaque_speciale: tirer_une(),
            defense_speciale: tirer_une(),
            vitesse: tirer_une(),
        })
    }

    pub fn valeurs(&self) -> &Stats<u8> {
        &self.0
    }

    /// Pour chaque stat, la plus haute des deux : celles du petit du Rite du
    /// sang mêlé (section 13.11).
    pub fn meilleures(&self, autre: &Self) -> Self {
        let (a, b) = (&self.0, &autre.0);
        Self(Stats {
            pv: a.pv.max(b.pv),
            attaque: a.attaque.max(b.attaque),
            defense: a.defense.max(b.defense),
            attaque_speciale: a.attaque_speciale.max(b.attaque_speciale),
            defense_speciale: a.defense_speciale.max(b.defense_speciale),
            vitesse: a.vitesse.max(b.vitesse),
        })
    }

    /// Ajoute `gain` à la valeur de cette stat, sans dépasser 31, comme un
    /// booster (section 14.3).
    pub fn augmenter(&mut self, stat: Stat, gain: u8) {
        let valeur = self.0.de_a_modifier(stat);
        *valeur = valeur.saturating_add(gain).min(Self::MAX);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurValeursInternes {
    pub valeur: u8,
}

impl fmt::Display for ErreurValeursInternes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "valeur interne {} hors de la plage 0 à {}",
            self.valeur,
            ValeursInternes::MAX
        )
    }
}

impl std::error::Error for ErreurValeursInternes {}

/// Formule de Pokémon sans EV ni nature (Q17), en divisions entières :
/// PV = (2 x base + VI) x niveau / 100 + niveau + 10,
/// autre stat = (2 x base + VI) x niveau / 100 + 5.
pub fn calculer_stats(
    base: &StatsDeBase,
    valeurs_internes: &ValeursInternes,
    niveau: Niveau,
) -> Stats<u32> {
    let vi = valeurs_internes.valeurs();
    let niveau = u32::from(niveau.valeur());
    let part_du_niveau = |base: u16, vi: u8| (2 * u32::from(base) + u32::from(vi)) * niveau / 100;
    Stats {
        pv: part_du_niveau(base.pv, vi.pv) + niveau + 10,
        attaque: part_du_niveau(base.attaque, vi.attaque) + 5,
        defense: part_du_niveau(base.defense, vi.defense) + 5,
        attaque_speciale: part_du_niveau(base.attaque_speciale, vi.attaque_speciale) + 5,
        defense_speciale: part_du_niveau(base.defense_speciale, vi.defense_speciale) + 5,
        vitesse: part_du_niveau(base.vitesse, vi.vitesse) + 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;

    fn uniformes<T: Copy>(valeur: T) -> Stats<T> {
        Stats {
            pv: valeur,
            attaque: valeur,
            defense: valeur,
            attaque_speciale: valeur,
            defense_speciale: valeur,
            vitesse: valeur,
        }
    }

    fn calculer(base: u16, vi: u8, niveau: u8) -> Stats<u32> {
        calculer_stats(
            &uniformes(base),
            &ValeursInternes::new(uniformes(vi)).unwrap(),
            Niveau::new(niveau).unwrap(),
        )
    }

    #[test]
    fn exemples_de_reference_base_50() {
        // Exemples consignés avec la décision Q17.
        let cas = [
            (5, 0, 20, 10),
            (5, 31, 21, 11),
            (100, 0, 210, 105),
            (100, 31, 241, 136),
        ];
        for (niveau, vi, pv, autre) in cas {
            let stats = calculer(50, vi, niveau);
            assert_eq!(stats.pv, pv, "niveau {niveau}, VI {vi}");
            assert_eq!(stats.vitesse, autre, "niveau {niveau}, VI {vi}");
        }
    }

    #[test]
    fn chaque_stat_utilise_sa_base_et_sa_valeur_interne() {
        let base = Stats {
            pv: 100,
            attaque: 80,
            defense: 60,
            attaque_speciale: 40,
            defense_speciale: 20,
            vitesse: 10,
        };
        let vi = ValeursInternes::new(Stats {
            pv: 31,
            attaque: 0,
            defense: 10,
            attaque_speciale: 20,
            defense_speciale: 30,
            vitesse: 5,
        })
        .unwrap();
        let stats = calculer_stats(&base, &vi, Niveau::new(50).unwrap());
        assert_eq!(
            stats,
            Stats {
                pv: 175,
                attaque: 85,
                defense: 70,
                attaque_speciale: 55,
                defense_speciale: 40,
                vitesse: 17,
            }
        );
    }

    #[test]
    fn bases_maximales_sans_debordement() {
        let stats = calculer(u16::MAX, 31, 100);
        assert_eq!(stats.pv, 131_101 + 110);
    }

    #[test]
    fn valeurs_internes_refusent_plus_de_31() {
        assert!(ValeursInternes::new(uniformes(31)).is_ok());
        let mut valeurs = uniformes(0);
        valeurs.vitesse = 32;
        assert_eq!(
            ValeursInternes::new(valeurs),
            Err(ErreurValeursInternes { valeur: 32 })
        );
    }

    #[test]
    fn bonus_de_dix_pour_cent_arrondi_a_l_inferieur() {
        let stats = Stats {
            attaque: 55,
            defense: 105,
            ..uniformes(20)
        };
        let bonus = Stats {
            attaque: 10,
            defense: 10,
            ..Stats::default()
        };
        let majorees = appliquer_bonus(&stats, &bonus);
        assert_eq!(majorees.attaque, 60);
        assert_eq!(majorees.defense, 115);
        assert_eq!(majorees.vitesse, 20);
    }

    #[test]
    fn valeurs_internes_tirees_dans_l_ordre_des_stats() {
        let mut aleatoire = AleatoireScripte::new([1, 2, 3, 4, 5, 31]);
        let vi = ValeursInternes::tirer(&mut aleatoire);
        assert_eq!(
            vi.valeurs(),
            &Stats {
                pv: 1,
                attaque: 2,
                defense: 3,
                attaque_speciale: 4,
                defense_speciale: 5,
                vitesse: 31,
            }
        );
    }
}
