//! Parfums (section 9.2) : Ansgar les prépare au camp, avec les matières de
//! sa ligne et, pour attirer une espèce, la ressource qu'elle lâche (section
//! 13.8). Le héros le porte aussitôt, jusqu'à sa prochaine expédition dans
//! une zone, où il attire des hordes : des groupes plus nombreux, plus de
//! monstres rares, des monstres plus forts, qui lâchent plus souvent leur
//! ressource.

use crate::espece::Espece;

/// Chaque groupe sauvage compte un monstre de plus, au plus comme au moins.
pub const MONSTRES_DE_PLUS: u8 = 1;
/// Points pris aux combats pour les monstres rares, sur 100 : 5 % passent à
/// 15 % dans une zone de la carte.
pub const POINTS_DE_RARES_EN_PLUS: u32 = 10;
/// Niveaux de plus des monstres sauvages, après ceux de la nuit.
pub const NIVEAUX_DE_PLUS: u8 = 3;
/// Un monstre d'une espèce commune lâche sa ressource 1 fois sur 2 au lieu
/// de 1 sur 3 ; celui d'une espèce rare, à coup sûr.
pub const CHANCES_DE_RESSOURCE_COMMUNE: u32 = 2;
/// Chaque monstre d'un groupe est de l'espèce attirée 1 fois sur 2, si elle
/// vit dans la zone.
pub const CHANCES_DE_L_ESPECE_ATTIREE: u32 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Parfum {
    /// Sans espèce : les hordes seules.
    Charogne,
    /// Les hordes, et cette espèce, dans une zone où elle vit.
    Espece(Espece),
}

impl Parfum {
    /// L'espèce qu'il attire en plus des hordes.
    pub fn espece_attiree(&self) -> Option<&Espece> {
        match self {
            Parfum::Charogne => None,
            Parfum::Espece(espece) => Some(espece),
        }
    }
}
