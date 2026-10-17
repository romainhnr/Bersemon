//! Monstre : un individu d'une espèce, avec son niveau, ses valeurs internes,
//! ses attaques, ses PV et son statut du moment (section 5).

use crate::aleatoire::Aleatoire;
use crate::attaque::AttaquesMonstre;
use crate::espece::Espece;
use crate::niveau::Niveau;
use crate::stats::{Stats, ValeursInternes, calculer_stats};
use crate::statut::Statut;

/// Un monstre sauvage sur 512 est shiny (section 5.1).
pub const CHANCES_SHINY: u32 = 512;

/// Tirage du shiny d'un monstre sauvage, fait à la rencontre.
pub fn tirer_shiny(aleatoire: &mut dyn Aleatoire) -> bool {
    aleatoire.tirer(1..=CHANCES_SHINY) == 1
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monstre {
    espece: Espece,
    niveau: Niveau,
    valeurs_internes: ValeursInternes,
    attaques: AttaquesMonstre,
    shiny: bool,
    pv_actuels: u32,
    statut: Option<Statut>,
}

impl Monstre {
    /// Crée un monstre avec tous ses PV et sans statut. Le shiny est reçu
    /// déjà décidé : il ne se tire que pour un monstre sauvage
    /// (`tirer_shiny`).
    pub fn new(
        espece: Espece,
        niveau: Niveau,
        valeurs_internes: ValeursInternes,
        attaques: AttaquesMonstre,
        shiny: bool,
    ) -> Self {
        let mut monstre = Self {
            espece,
            niveau,
            valeurs_internes,
            attaques,
            shiny,
            pv_actuels: 0,
            statut: None,
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

    pub fn attaques(&self) -> &AttaquesMonstre {
        &self.attaques
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

    pub fn statut(&self) -> Option<Statut> {
        self.statut
    }

    /// Un monstre KO, déjà atteint d'un statut ou immunisé ne peut pas en
    /// recevoir un nouveau (section 10.5).
    pub fn peut_subir(&self, statut: Statut) -> bool {
        !self.est_ko() && self.statut.is_none() && !statut.epargne(self.espece.types())
    }

    /// Rend `false`, sans rien changer, si le monstre ne peut pas subir ce
    /// statut.
    pub fn infliger_statut(&mut self, statut: Statut) -> bool {
        let inflige = self.peut_subir(statut);
        if inflige {
            self.statut = Some(statut);
        }
        inflige
    }

    /// Les PV ne descendent pas sous 0 : le surplus de dégâts est perdu.
    pub fn subir_degats(&mut self, degats: u32) {
        self.pv_actuels = self.pv_actuels.saturating_sub(degats);
    }

    /// Rend tous les PV et guérit le statut, même d'un monstre KO, comme au
    /// retour au camp (section 13.1).
    pub fn soigner_completement(&mut self) {
        self.pv_actuels = self.stats().pv;
        self.statut = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attaque::{Attaque, AttaqueId, Categorie};
    use crate::espece::{EspeceId, Role};
    use crate::stats::StatsDeBase;
    use crate::types::{Type, TypesMonstre};

    fn attaques_de_test() -> AttaquesMonstre {
        let attaque = Attaque::new(
            AttaqueId::new("attaque-de-test"),
            "Attaque de test",
            Type::Normal,
            Categorie::Physique,
            40,
            100,
        )
        .unwrap();
        AttaquesMonstre::new(vec![attaque]).unwrap()
    }

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
        Monstre::new(
            espece_de_test(),
            Niveau::new(niveau).unwrap(),
            vi,
            attaques_de_test(),
            false,
        )
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
        let monstre = Monstre::new(
            espece_de_test(),
            Niveau::new(5).unwrap(),
            vi,
            attaques_de_test(),
            true,
        );
        assert!(monstre.est_shiny());
    }

    #[test]
    fn un_seul_statut_a_la_fois() {
        let mut monstre = monstre_de_test(5);
        assert!(monstre.infliger_statut(Statut::Paralysie));
        assert!(!monstre.infliger_statut(Statut::Poison));
        assert_eq!(monstre.statut(), Some(Statut::Paralysie));
    }

    #[test]
    fn un_monstre_ko_ne_recoit_pas_de_statut() {
        let mut monstre = monstre_de_test(5);
        monstre.subir_degats(u32::MAX);
        assert!(!monstre.infliger_statut(Statut::Poison));
        assert_eq!(monstre.statut(), None);
    }

    #[test]
    fn soin_complet_guerit_le_statut() {
        let mut monstre = monstre_de_test(5);
        monstre.infliger_statut(Statut::Poison);
        monstre.soigner_completement();
        assert_eq!(monstre.statut(), None);
    }

    #[test]
    fn shiny_sur_un_tirage_sur_512() {
        use crate::aleatoire::AleatoireScripte;
        assert!(tirer_shiny(&mut AleatoireScripte::new([1])));
        assert!(!tirer_shiny(&mut AleatoireScripte::new([512])));
    }
}
