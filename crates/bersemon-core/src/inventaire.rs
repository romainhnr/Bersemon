//! Objets que porte le joueur. Dans la V1, uniquement des potions de soin
//! (section 14.4).

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::monstre::Monstre;

/// Fourchette du pourcentage de PV max rendu par une potion.
const POURCENTAGE_MIN: u32 = 40;
const POURCENTAGE_MAX: u32 = 60;

/// Le joueur porte toutes ses potions : pas de limite de sac ni de coffre de
/// stockage dans la V1.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Inventaire {
    potions: u32,
}

impl Inventaire {
    pub fn potions(&self) -> u32 {
        self.potions
    }

    pub fn ajouter_potions(&mut self, nombre: u32) {
        self.potions = self.potions.saturating_add(nombre);
    }

    /// Hors combat, entre deux étapes d'une expédition. Rend les PV rendus.
    pub fn utiliser_potion(
        &mut self,
        monstre: &mut Monstre,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<u32, ErreurPotion> {
        if self.potions == 0 {
            return Err(ErreurPotion::AucunePotion);
        }
        verifier_cible(monstre)?;
        self.potions -= 1;
        Ok(appliquer_potion(monstre, aleatoire))
    }

    /// Une défaite fait perdre tous les objets portés (section 9.3).
    pub fn perdre_tout(&mut self) {
        self.potions = 0;
    }

    /// Après un combat, qui a pu consommer des potions.
    pub(crate) fn definir_potions(&mut self, potions: u32) {
        self.potions = potions;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurPotion {
    AucunePotion,
    MonstreInexistant,
    MonstreKo,
    PvDejaPleins,
}

impl fmt::Display for ErreurPotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurPotion::AucunePotion => write!(f, "aucune potion"),
            ErreurPotion::MonstreInexistant => write!(f, "ce monstre n'est pas dans l'équipe"),
            ErreurPotion::MonstreKo => write!(f, "une potion ne relève pas un monstre KO"),
            ErreurPotion::PvDejaPleins => write!(f, "le monstre a déjà tous ses PV"),
        }
    }
}

impl std::error::Error for ErreurPotion {}

/// Une potion ne relève pas un monstre KO (section 14.4) et n'est pas
/// gaspillée sur un monstre qui a tous ses PV (Q53).
pub fn verifier_cible(monstre: &Monstre) -> Result<(), ErreurPotion> {
    if monstre.est_ko() {
        Err(ErreurPotion::MonstreKo)
    } else if monstre.pv_actuels() >= monstre.stats().pv {
        Err(ErreurPotion::PvDejaPleins)
    } else {
        Ok(())
    }
}

/// Rend un pourcentage des PV max tiré de 40 % à 60 % (un tirage). Rend les
/// PV réellement rendus.
pub(crate) fn appliquer_potion(monstre: &mut Monstre, aleatoire: &mut dyn Aleatoire) -> u32 {
    let pourcentage = aleatoire.tirer(POURCENTAGE_MIN..=POURCENTAGE_MAX);
    let soin = monstre.stats().pv * pourcentage / 100;
    monstre.soigner(soin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::attaque::Categorie;
    use crate::donnees_de_test::{attaque, monstre, stats_uniformes};
    use crate::types::Type;

    /// 20 PV max au niveau 5.
    fn blesse(pv_restants: u32) -> Monstre {
        let coup = attaque("coup", Type::Normal, Categorie::Physique, 40, 100);
        let mut monstre = monstre(&[Type::Feu], stats_uniformes(50), 5, vec![coup]);
        monstre.subir_degats(20 - pv_restants);
        monstre
    }

    #[test]
    fn une_potion_rend_de_40_a_60_pour_cent_des_pv_max() {
        let mut inventaire = Inventaire::default();
        inventaire.ajouter_potions(2);
        let mut monstre = blesse(1);
        assert_eq!(
            inventaire.utiliser_potion(&mut monstre, &mut AleatoireScripte::new([40])),
            Ok(8)
        );
        assert_eq!(
            inventaire.utiliser_potion(&mut monstre, &mut AleatoireScripte::new([60])),
            Ok(11)
        );
        assert_eq!(monstre.pv_actuels(), 20);
        assert_eq!(inventaire.potions(), 0);
    }

    #[test]
    fn potion_refusee_sans_rien_consommer() {
        let mut inventaire = Inventaire::default();
        let mut aleatoire = AleatoireScripte::new([]);
        assert_eq!(
            inventaire.utiliser_potion(&mut blesse(5), &mut aleatoire),
            Err(ErreurPotion::AucunePotion)
        );
        inventaire.ajouter_potions(1);
        assert_eq!(
            inventaire.utiliser_potion(&mut blesse(0), &mut aleatoire),
            Err(ErreurPotion::MonstreKo)
        );
        assert_eq!(
            inventaire.utiliser_potion(&mut blesse(20), &mut aleatoire),
            Err(ErreurPotion::PvDejaPleins)
        );
        assert_eq!(inventaire.potions(), 1);
    }

    #[test]
    fn une_defaite_fait_tout_perdre() {
        let mut inventaire = Inventaire::default();
        inventaire.ajouter_potions(3);
        inventaire.perdre_tout();
        assert_eq!(inventaire.potions(), 0);
    }
}
