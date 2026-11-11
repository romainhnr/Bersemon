//! Butin des monstres sauvages (section 14.3) : un monstre sauvage vaincu,
//! mis KO et non capturé, lâche toujours un objet, après une victoire. Son
//! objet commun, la ressource de son espèce, ou, s'il en a un, son butin
//! rare, dont la chance est de 1, 5, 15 ou 30 % ; l'objet commun a le reste.
//! Le butin se tire au début du combat, pour que l'habileté Flair puisse le
//! montrer.

use crate::aleatoire::Aleatoire;
use crate::booster::Booster;
use crate::espece::{Espece, Rarete};
use crate::habilete::Habilete;
use crate::monstre::Monstre;
use crate::ressource::Ressource;
use crate::stats::Stat;

/// Les seules chances possibles d'un butin rare, en pourcentage.
pub const CHANCES_DU_BUTIN_RARE: [u32; 4] = [1, 5, 15, 30];
/// Chance du butin rare d'une espèce commune, et d'une espèce rare
/// (hypothèses retenues, Q123).
pub const CHANCE_DU_BUTIN_RARE_COMMUN: u32 = 5;
pub const CHANCE_DU_BUTIN_RARE_D_UNE_RARE: u32 = 15;
/// L'habileté Fortune, comme un parfum, double la chance du butin rare.
pub const MULTIPLICATEUR_DE_FORTUNE: u32 = 2;
/// L'habileté Dépeceur double l'objet lâché.
pub const MULTIPLICATEUR_DU_DEPECEUR: u32 = 2;

/// Le butin rare d'une espèce : un booster, et sa chance sur 100.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButinRare {
    booster: Booster,
    chance: u32,
}

impl ButinRare {
    /// `None` pour une chance qui n'est pas parmi `CHANCES_DU_BUTIN_RARE`.
    pub fn new(booster: Booster, chance: u32) -> Option<Self> {
        CHANCES_DU_BUTIN_RARE
            .contains(&chance)
            .then_some(Self { booster, chance })
    }

    /// Le booster de sa meilleure stat de base, la première à égalité :
    /// 5 % pour une espèce commune, 15 % pour une rare (hypothèses retenues,
    /// Q123). Ni les Apôtres ni le Démiurge, qui ne lâchent rien.
    pub fn de_l_espece(espece: &Espece) -> Option<Self> {
        let chance = match espece.rarete() {
            Rarete::Commune => CHANCE_DU_BUTIN_RARE_COMMUN,
            Rarete::Rare => CHANCE_DU_BUTIN_RARE_D_UNE_RARE,
            Rarete::Apotre | Rarete::Demiurge => return None,
        };
        let base = espece.stats_de_base();
        let meilleure = Stat::TOUTES.into_iter().fold(Stat::Pv, |meilleure, stat| {
            if base.de(stat) > base.de(meilleure) {
                stat
            } else {
                meilleure
            }
        });
        let booster = Booster::TOUS
            .into_iter()
            .find(|booster| booster.stat() == meilleure)
            .expect("un booster par stat");
        Self::new(booster, chance)
    }

    pub fn booster(&self) -> Booster {
        self.booster
    }

    pub fn chance(&self) -> u32 {
        self.chance
    }
}

/// Ce qu'un monstre vaincu lâchera, et en combien d'exemplaires.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Butin {
    Ressource(Ressource, u32),
    Booster(Booster, u32),
}

impl Butin {
    pub fn nombre(&self) -> u32 {
        match self {
            Butin::Ressource(_, nombre) | Butin::Booster(_, nombre) => *nombre,
        }
    }
}

/// Les habiletés d'équipe qui jouent sur le butin, chez les monstres
/// debout au début du combat ; elles ne se cumulent pas (section 5.9).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HabiletesDuButin {
    pub depeceur: bool,
    pub fortune: bool,
    pub flair: bool,
}

impl HabiletesDuButin {
    pub fn de_l_equipe<'a>(equipe: impl IntoIterator<Item = &'a Monstre>) -> Self {
        let mut habiletes = Self::default();
        for monstre in equipe.into_iter().filter(|monstre| !monstre.est_ko()) {
            habiletes.depeceur |= monstre.a_l_habilete(Habilete::Depeceur);
            habiletes.fortune |= monstre.a_l_habilete(Habilete::Fortune);
            habiletes.flair |= monstre.a_l_habilete(Habilete::Flair);
        }
        habiletes
    }
}

/// Le butin d'un monstre sauvage, tiré au début du combat : `None` pour une
/// espèce qui ne lâche rien. La chance du butin rare double avec Fortune,
/// et encore avec un parfum, 100 % au plus ; Dépeceur double l'objet.
/// Tirage : un, de 1 à 100, pour une espèce qui a un butin rare.
pub fn tirer_le_butin(
    monstre: &Monstre,
    habiletes: HabiletesDuButin,
    parfum: bool,
    aleatoire: &mut dyn Aleatoire,
) -> Option<Butin> {
    let espece = monstre.espece();
    let ressource = espece.ressource()?;
    let nombre = if habiletes.depeceur {
        MULTIPLICATEUR_DU_DEPECEUR
    } else {
        1
    };
    if let Some(rare) = espece.butin_rare() {
        let mut chance = rare.chance();
        if habiletes.fortune {
            chance *= MULTIPLICATEUR_DE_FORTUNE;
        }
        if parfum {
            chance *= MULTIPLICATEUR_DE_FORTUNE;
        }
        if aleatoire.tirer(1..=100) <= chance.min(100) {
            return Some(Butin::Booster(rare.booster(), nombre));
        }
    }
    Some(Butin::Ressource(ressource.clone(), nombre))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::catalogue::Catalogue;
    use crate::contenu::ContenuV1;
    use crate::espece::EspeceId;
    use crate::niveau::Niveau;
    use crate::stats::ValeursInternes;

    fn creer(contenu: &ContenuV1, id: &str) -> Monstre {
        Monstre::creer(
            contenu.espece(&EspeceId::new(id)).unwrap(),
            Niveau::new(10).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    #[test]
    fn seules_quatre_chances_de_butin_rare() {
        assert!(ButinRare::new(Booster::FioleDeSang, 5).is_some());
        assert!(ButinRare::new(Booster::FioleDeSang, 10).is_none());
    }

    /// Le Loup, d'attaque de base 62, la meilleure de ses stats, lâche une
    /// Moelle de bête 5 fois sur 100, sinon son Croc.
    #[test]
    fn un_vaincu_lache_toujours_un_objet() {
        let contenu = ContenuV1::new();
        let loup = creer(&contenu, "loup");
        let rare = loup.espece().butin_rare().unwrap();
        assert_eq!((rare.booster(), rare.chance()), (Booster::MoelleDeBete, 5));
        let sans = HabiletesDuButin::default();
        assert_eq!(
            tirer_le_butin(&loup, sans, false, &mut AleatoireScripte::new([5])),
            Some(Butin::Booster(Booster::MoelleDeBete, 1))
        );
        assert!(matches!(
            tirer_le_butin(&loup, sans, false, &mut AleatoireScripte::new([6])),
            Some(Butin::Ressource(_, 1))
        ));
        // Fortune et un parfum : 20 %. Dépeceur : deux exemplaires.
        let toutes = HabiletesDuButin {
            depeceur: true,
            fortune: true,
            flair: false,
        };
        assert_eq!(
            tirer_le_butin(&loup, toutes, true, &mut AleatoireScripte::new([20])),
            Some(Butin::Booster(Booster::MoelleDeBete, 2))
        );
        assert!(matches!(
            tirer_le_butin(&loup, toutes, true, &mut AleatoireScripte::new([21])),
            Some(Butin::Ressource(_, 2))
        ));
        // Une espèce sans ressource ne lâche rien, sans tirage.
        let salamandre = creer(&contenu, "salamandre");
        assert_eq!(
            tirer_le_butin(&salamandre, sans, false, &mut AleatoireScripte::new([])),
            None
        );
    }
}
