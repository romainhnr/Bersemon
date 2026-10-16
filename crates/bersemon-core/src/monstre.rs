//! Monstre : un individu d'une espèce, avec son niveau, ses valeurs internes
//! et ses PV du moment (section 5).

use crate::espece::Espece;
use crate::niveau::Niveau;
use crate::stats::{Stats, ValeursInternes, calculer_stats};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monstre {
    espece: Espece,
    niveau: Niveau,
    valeurs_internes: ValeursInternes,
    shiny: bool,
    pv_actuels: u32,
}

impl Monstre {
    /// Crée un monstre avec tous ses PV. Le tirage du shiny se fait à la
    /// rencontre, avec une probabilité encore à fixer : il est donc reçu ici
    /// déjà décidé.
    pub fn new(
        espece: Espece,
        niveau: Niveau,
        valeurs_internes: ValeursInternes,
        shiny: bool,
    ) -> Self {
        let mut monstre = Self {
            espece,
            niveau,
            valeurs_internes,
            shiny,
            pv_actuels: 0,
        };
        monstre.soigner_completement();
        monstre
    }

    pub fn espece(&self) -> &Espece {
        &self.espece
    }

    pub fn niveau(&self) -> Niveau {
        self.niveau
    }

    pub fn valeurs_internes(&self) -> &ValeursInternes {
        &self.valeurs_internes
    }

    /// Recalculées à chaque appel plutôt que mémorisées, pour qu'elles
    /// suivent sans risque d'oubli un changement de niveau ou d'espèce.
    pub fn stats(&self) -> Stats<u32> {
        calculer_stats(
            self.espece.stats_de_base(),
            &self.valeurs_internes,
            self.niveau,
        )
    }

    pub fn pv_actuels(&self) -> u32 {
        self.pv_actuels
    }

    /// Un monstre à 0 PV est KO ; il n'y a pas de mort définitive (section
    /// 5.7).
    pub fn est_ko(&self) -> bool {
        self.pv_actuels == 0
    }

    pub fn est_shiny(&self) -> bool {
        self.shiny
    }

    /// Les PV ne descendent pas sous 0 : le surplus de dégâts est perdu.
    pub fn subir_degats(&mut self, degats: u32) {
        self.pv_actuels = self.pv_actuels.saturating_sub(degats);
    }

    /// Rend tous les PV, même à un monstre KO, comme au retour au camp
    /// (section 13.1).
    pub fn soigner_completement(&mut self) {
        self.pv_actuels = self.stats().pv;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::espece::{EspeceId, Role};
    use crate::stats::StatsDeBase;
    use crate::types::{Type, TypesMonstre};

    fn espece_de_test() -> Espece {
        let base: StatsDeBase = Stats {
            pv: 50,
            attaque: 50,
            defense: 50,
            attaque_speciale: 50,
            defense_speciale: 50,
            vitesse: 50,
        };
        Espece::new(
            EspeceId::new("espece-de-test"),
            "Espèce de test",
            TypesMonstre::new(&[Type::Feu]).unwrap(),
            Role::Offensif,
            base,
        )
    }

    fn monstre_de_test(niveau: u8) -> Monstre {
        let vi = ValeursInternes::new(Stats {
            pv: 31,
            attaque: 31,
            defense: 31,
            attaque_speciale: 31,
            defense_speciale: 31,
            vitesse: 31,
        })
        .unwrap();
        Monstre::new(espece_de_test(), Niveau::new(niveau).unwrap(), vi, false)
    }

    #[test]
    fn nouveau_monstre_a_tous_ses_pv() {
        let monstre = monstre_de_test(5);
        assert_eq!(monstre.stats().pv, 21);
        assert_eq!(monstre.pv_actuels(), 21);
        assert!(!monstre.est_ko());
    }

    #[test]
    fn stats_calculees_depuis_espece_niveau_et_valeurs_internes() {
        let monstre = monstre_de_test(100);
        assert_eq!(monstre.stats().pv, 241);
        assert_eq!(monstre.stats().attaque, 136);
    }

    #[test]
    fn degats_partiels_puis_ko_sans_pv_negatifs() {
        let mut monstre = monstre_de_test(5);
        monstre.subir_degats(20);
        assert_eq!(monstre.pv_actuels(), 1);
        assert!(!monstre.est_ko());
        monstre.subir_degats(50);
        assert_eq!(monstre.pv_actuels(), 0);
        assert!(monstre.est_ko());
    }

    #[test]
    fn soin_complet_releve_un_monstre_ko() {
        let mut monstre = monstre_de_test(5);
        monstre.subir_degats(u32::MAX);
        monstre.soigner_completement();
        assert_eq!(monstre.pv_actuels(), 21);
        assert!(!monstre.est_ko());
    }

    #[test]
    fn shiny_conserve() {
        let vi = *monstre_de_test(5).valeurs_internes();
        let monstre = Monstre::new(espece_de_test(), Niveau::new(5).unwrap(), vi, true);
        assert!(monstre.est_shiny());
    }
}
