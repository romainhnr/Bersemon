//! Objets du joueur, rangés dans le sac, qu'il emporte en expédition
//! manuelle, ou dans le coffre du camp, où ils sont à l'abri (sections 13.3
//! et 14.4) : argent, soins, capsules d'attaque, capsules d'habileté,
//! ressources, pierres d'évolution, boosters et Éclats d'astre.

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::attaque::AttaqueId;
use crate::booster::Booster;
use crate::capsule::Capsule;
use crate::monstre::Monstre;
use crate::pierre::PierreDEvolution;
use crate::ressource::{Ressource, RessourceId};
use crate::soin::{EffetDuSoin, ErreurSoin, Soin, Soins, appliquer_soin, verifier_cible};

/// Le contenu du sac ou du coffre. Pas de limite de place pour l'instant
/// (section 13.3).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Inventaire {
    argent: u32,
    soins: Soins,
    /// Sans doublon : une capsule sert sans s'user (section 13.10).
    capsules: Vec<Capsule>,
    /// Elles se consomment, contrairement aux capsules d'attaque (section
    /// 5.9).
    capsules_d_habilete: u32,
    /// Dans l'ordre où elles ont été trouvées, sans nombre nul (section
    /// 14.3).
    ressources: Vec<(Ressource, u32)>,
    /// Dans l'ordre de `PierreDEvolution::TOUTES`, sans nombre nul (section
    /// 5.4).
    pierres: Vec<(PierreDEvolution, u32)>,
    /// Dans l'ordre de `Booster::TOUS`, sans nombre nul (section 14.3).
    boosters: Vec<(Booster, u32)>,
    /// Pour le service de Malvoise (section 13.7).
    eclats_d_astre: u32,
}

/// Ce que le joueur déplace entre le sac et le coffre du camp.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Objet {
    Argent(u32),
    Soins(Soin, u32),
    Capsule(AttaqueId),
    CapsulesDHabilete(u32),
    Ressources(RessourceId, u32),
    Pierres(PierreDEvolution, u32),
    Boosters(Booster, u32),
    EclatsDAstre(u32),
}

impl Inventaire {
    /// Reprise d'une sauvegarde ; les capsules en double ne sont gardées
    /// qu'une fois. Les autres soins que la potion s'ajoutent avec
    /// `avec_soins`.
    pub fn new(argent: u32, potions: u32, capsules: Vec<Capsule>) -> Self {
        let mut inventaire = Self {
            argent,
            soins: Soins::default(),
            capsules: Vec::new(),
            capsules_d_habilete: 0,
            ressources: Vec::new(),
            pierres: Vec::new(),
            boosters: Vec::new(),
            eclats_d_astre: 0,
        };
        inventaire.soins.ajouter(Soin::Potion, potions);
        for capsule in capsules {
            inventaire.ajouter_capsule(capsule);
        }
        inventaire
    }

    pub fn avec_soins(mut self, soin: Soin, nombre: u32) -> Self {
        self.soins.ajouter(soin, nombre);
        self
    }

    pub fn avec_capsules_d_habilete(mut self, nombre: u32) -> Self {
        self.ajouter_capsules_d_habilete(nombre);
        self
    }

    pub fn capsules_d_habilete(&self) -> u32 {
        self.capsules_d_habilete
    }

    pub fn ajouter_capsules_d_habilete(&mut self, nombre: u32) {
        self.capsules_d_habilete = self.capsules_d_habilete.saturating_add(nombre);
    }

    pub fn pierres(&self) -> &[(PierreDEvolution, u32)] {
        &self.pierres
    }

    pub fn nombre_de_pierres(&self, pierre: PierreDEvolution) -> u32 {
        self.pierres
            .iter()
            .find(|(presente, _)| *presente == pierre)
            .map_or(0, |(_, nombre)| *nombre)
    }

    pub fn ajouter_pierres(&mut self, pierre: PierreDEvolution, nombre: u32) {
        if nombre == 0 {
            return;
        }
        match self
            .pierres
            .iter_mut()
            .find(|(presente, _)| *presente == pierre)
        {
            Some((_, present)) => *present = present.saturating_add(nombre),
            None => {
                self.pierres.push((pierre, nombre));
                let rang = |pierre: &PierreDEvolution| {
                    PierreDEvolution::TOUTES.iter().position(|p| p == pierre)
                };
                self.pierres.sort_by_key(|(pierre, _)| rang(pierre));
            }
        }
    }

    pub fn avec_pierres(mut self, pierre: PierreDEvolution, nombre: u32) -> Self {
        self.ajouter_pierres(pierre, nombre);
        self
    }

    /// Rend `false` sans rien changer s'il n'y en a pas.
    pub(crate) fn utiliser_pierre(&mut self, pierre: PierreDEvolution) -> bool {
        self.retirer(&Objet::Pierres(pierre, 1)).is_ok()
    }

    pub fn boosters(&self) -> &[(Booster, u32)] {
        &self.boosters
    }

    pub fn nombre_de_boosters(&self, booster: Booster) -> u32 {
        self.boosters
            .iter()
            .find(|(present, _)| *present == booster)
            .map_or(0, |(_, nombre)| *nombre)
    }

    pub fn ajouter_boosters(&mut self, booster: Booster, nombre: u32) {
        if nombre == 0 {
            return;
        }
        match self
            .boosters
            .iter_mut()
            .find(|(present, _)| *present == booster)
        {
            Some((_, present)) => *present = present.saturating_add(nombre),
            None => {
                self.boosters.push((booster, nombre));
                let rang = |booster: &Booster| Booster::TOUS.iter().position(|b| b == booster);
                self.boosters.sort_by_key(|(booster, _)| rang(booster));
            }
        }
    }

    pub fn avec_boosters(mut self, booster: Booster, nombre: u32) -> Self {
        self.ajouter_boosters(booster, nombre);
        self
    }

    /// Rend `false` sans rien changer s'il n'y en a pas.
    pub(crate) fn utiliser_booster(&mut self, booster: Booster) -> bool {
        self.retirer(&Objet::Boosters(booster, 1)).is_ok()
    }

    pub fn eclats_d_astre(&self) -> u32 {
        self.eclats_d_astre
    }

    pub fn ajouter_eclats_d_astre(&mut self, nombre: u32) {
        self.eclats_d_astre = self.eclats_d_astre.saturating_add(nombre);
    }

    pub fn avec_eclats_d_astre(mut self, nombre: u32) -> Self {
        self.ajouter_eclats_d_astre(nombre);
        self
    }

    /// Rend `false` sans rien changer s'il n'y en a pas.
    pub(crate) fn utiliser_eclat_d_astre(&mut self) -> bool {
        self.retirer(&Objet::EclatsDAstre(1)).is_ok()
    }

    pub fn ressources(&self) -> &[(Ressource, u32)] {
        &self.ressources
    }

    pub fn nombre_de_ressources(&self, id: &RessourceId) -> u32 {
        self.ressources
            .iter()
            .find(|(ressource, _)| ressource.id() == id)
            .map_or(0, |(_, nombre)| *nombre)
    }

    pub fn ajouter_ressources(&mut self, ressource: Ressource, nombre: u32) {
        if nombre == 0 {
            return;
        }
        match self
            .ressources
            .iter_mut()
            .find(|(presente, _)| presente.id() == ressource.id())
        {
            Some((_, present)) => *present = present.saturating_add(nombre),
            None => self.ressources.push((ressource, nombre)),
        }
    }

    pub fn avec_ressources(mut self, ressource: Ressource, nombre: u32) -> Self {
        self.ajouter_ressources(ressource, nombre);
        self
    }

    /// Toutes celles de cette sorte, rendues avec leur nombre.
    pub(crate) fn retirer_toutes_les_ressources(
        &mut self,
        id: &RessourceId,
    ) -> Option<(Ressource, u32)> {
        let position = self
            .ressources
            .iter()
            .position(|(ressource, _)| ressource.id() == id)?;
        Some(self.ressources.remove(position))
    }

    /// Rend `false` sans rien changer s'il n'y en a pas.
    pub(crate) fn utiliser_capsule_d_habilete(&mut self) -> bool {
        if self.capsules_d_habilete == 0 {
            return false;
        }
        self.capsules_d_habilete -= 1;
        true
    }

    /// En pièces d'argent (section 14.1).
    pub fn argent(&self) -> u32 {
        self.argent
    }

    pub fn potions(&self) -> u32 {
        self.soins.nombre(Soin::Potion)
    }

    pub fn soins(&self) -> &Soins {
        &self.soins
    }

    pub fn capsules(&self) -> &[Capsule] {
        &self.capsules
    }

    pub fn est_vide(&self) -> bool {
        self.argent == 0
            && self.soins.est_vide()
            && self.capsules.is_empty()
            && self.capsules_d_habilete == 0
            && self.ressources.is_empty()
            && self.pierres.is_empty()
            && self.boosters.is_empty()
            && self.eclats_d_astre == 0
    }

    pub fn contient_capsule(&self, id: &AttaqueId) -> bool {
        self.capsules.iter().any(|capsule| capsule.id() == id)
    }

    pub fn ajouter_potions(&mut self, nombre: u32) {
        self.soins.ajouter(Soin::Potion, nombre);
    }

    pub fn ajouter_soins(&mut self, soin: Soin, nombre: u32) {
        self.soins.ajouter(soin, nombre);
    }

    pub fn ajouter_argent(&mut self, montant: u32) {
        self.argent = self.argent.saturating_add(montant);
    }

    /// Sans effet pour une capsule déjà là.
    pub fn ajouter_capsule(&mut self, capsule: Capsule) {
        if !self.contient_capsule(capsule.id()) {
            self.capsules.push(capsule);
        }
    }

    /// Tout le contenu de `autre` rejoint celui-ci.
    pub fn ajouter(&mut self, autre: Inventaire) {
        self.ajouter_argent(autre.argent);
        for (soin, nombre) in autre.soins.presents() {
            self.soins.ajouter(soin, nombre);
        }
        for capsule in autre.capsules {
            self.ajouter_capsule(capsule);
        }
        self.ajouter_capsules_d_habilete(autre.capsules_d_habilete);
        for (ressource, nombre) in autre.ressources {
            self.ajouter_ressources(ressource, nombre);
        }
        for (pierre, nombre) in autre.pierres {
            self.ajouter_pierres(pierre, nombre);
        }
        for (booster, nombre) in autre.boosters {
            self.ajouter_boosters(booster, nombre);
        }
        self.ajouter_eclats_d_astre(autre.eclats_d_astre);
    }

    /// Retire l'objet et le rend, seul dans un inventaire, sans rien changer
    /// s'il manque.
    pub(crate) fn retirer(&mut self, objet: &Objet) -> Result<Inventaire, ErreurObjet> {
        let mut retire = Inventaire::default();
        match objet {
            Objet::Argent(0)
            | Objet::Soins(_, 0)
            | Objet::CapsulesDHabilete(0)
            | Objet::Ressources(_, 0)
            | Objet::Pierres(_, 0)
            | Objet::Boosters(_, 0)
            | Objet::EclatsDAstre(0) => {
                return Err(ErreurObjet::QuantiteNulle);
            }
            Objet::Argent(montant) => {
                if *montant > self.argent {
                    return Err(ErreurObjet::Insuffisant);
                }
                self.argent -= montant;
                retire.argent = *montant;
            }
            Objet::Soins(soin, nombre) => {
                if !self.soins.retirer(*soin, *nombre) {
                    return Err(ErreurObjet::Insuffisant);
                }
                retire.soins.ajouter(*soin, *nombre);
            }
            Objet::Capsule(id) => {
                let position = self
                    .capsules
                    .iter()
                    .position(|capsule| capsule.id() == id)
                    .ok_or(ErreurObjet::CapsuleAbsente)?;
                retire.capsules.push(self.capsules.remove(position));
            }
            Objet::CapsulesDHabilete(nombre) => {
                if *nombre > self.capsules_d_habilete {
                    return Err(ErreurObjet::Insuffisant);
                }
                self.capsules_d_habilete -= nombre;
                retire.capsules_d_habilete = *nombre;
            }
            Objet::Ressources(id, nombre) => {
                if *nombre > self.nombre_de_ressources(id) {
                    return Err(ErreurObjet::Insuffisant);
                }
                let position = self
                    .ressources
                    .iter()
                    .position(|(ressource, _)| ressource.id() == id)
                    .expect("ressource présente");
                let (ressource, present) = &mut self.ressources[position];
                *present -= nombre;
                retire.ajouter_ressources(ressource.clone(), *nombre);
                if *present == 0 {
                    self.ressources.remove(position);
                }
            }
            Objet::Pierres(pierre, nombre) => {
                if *nombre > self.nombre_de_pierres(*pierre) {
                    return Err(ErreurObjet::Insuffisant);
                }
                let position = self
                    .pierres
                    .iter()
                    .position(|(presente, _)| presente == pierre)
                    .expect("pierre présente");
                self.pierres[position].1 -= nombre;
                if self.pierres[position].1 == 0 {
                    self.pierres.remove(position);
                }
                retire.ajouter_pierres(*pierre, *nombre);
            }
            Objet::Boosters(booster, nombre) => {
                if *nombre > self.nombre_de_boosters(*booster) {
                    return Err(ErreurObjet::Insuffisant);
                }
                let position = self
                    .boosters
                    .iter()
                    .position(|(present, _)| present == booster)
                    .expect("booster présent");
                self.boosters[position].1 -= nombre;
                if self.boosters[position].1 == 0 {
                    self.boosters.remove(position);
                }
                retire.ajouter_boosters(*booster, *nombre);
            }
            Objet::EclatsDAstre(nombre) => {
                if *nombre > self.eclats_d_astre {
                    return Err(ErreurObjet::Insuffisant);
                }
                self.eclats_d_astre -= nombre;
                retire.eclats_d_astre = *nombre;
            }
        }
        Ok(retire)
    }

    /// Au plus `montant`, et rend ce qui a été pris.
    pub(crate) fn prendre_argent(&mut self, montant: u32) -> u32 {
        let pris = montant.min(self.argent);
        self.argent -= pris;
        pris
    }

    /// Hors combat, entre deux étapes d'une expédition (section 14.4).
    pub fn utiliser_soin(
        &mut self,
        soin: Soin,
        monstre: &mut Monstre,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<EffetDuSoin, ErreurSoin> {
        if self.soins.nombre(soin) == 0 {
            return Err(ErreurSoin::Aucun(soin));
        }
        verifier_cible(soin, monstre)?;
        self.soins.retirer(soin, 1);
        Ok(appliquer_soin(soin, monstre, aleatoire))
    }

    /// Une défaite fait perdre tout le sac (section 9.3). Rend ce qui est
    /// perdu.
    pub fn perdre_tout(&mut self) -> Inventaire {
        std::mem::take(self)
    }

    /// Après un combat, qui a pu consommer des soins.
    pub(crate) fn definir_soins(&mut self, soins: Soins) {
        self.soins = soins;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurObjet {
    QuantiteNulle,
    /// Il n'y en a pas assez là d'où on veut les prendre.
    Insuffisant,
    CapsuleAbsente,
}

impl fmt::Display for ErreurObjet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurObjet::QuantiteNulle => write!(f, "il faut déplacer au moins un objet"),
            ErreurObjet::Insuffisant => write!(f, "il n'y en a pas autant"),
            ErreurObjet::CapsuleAbsente => write!(f, "cette capsule n'est pas là"),
        }
    }
}

impl std::error::Error for ErreurObjet {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::attaque::Categorie;
    use crate::donnees_de_test::{attaque, monstre, stats_uniformes};
    use crate::soin::soin_sans_effet;
    use crate::types::Type;

    /// 20 PV max au niveau 5.
    fn blesse(pv_restants: u32) -> Monstre {
        let coup = attaque("coup", Type::Normal, Categorie::Physique, 40, 100);
        let mut monstre = monstre(&[Type::Feu], stats_uniformes(50), 5, vec![coup]);
        monstre.subir_degats(20 - pv_restants);
        monstre
    }

    fn capsule(id: &str) -> Capsule {
        Capsule::new(attaque(id, Type::Normal, Categorie::Physique, 40, 100), 150)
    }

    #[test]
    fn une_potion_rend_de_40_a_60_pour_cent_des_pv_max() {
        let mut inventaire = Inventaire::default();
        inventaire.ajouter_potions(2);
        let mut monstre = blesse(1);
        assert_eq!(
            inventaire.utiliser_soin(Soin::Potion, &mut monstre, &mut AleatoireScripte::new([40])),
            Ok(EffetDuSoin::PvRendus(8))
        );
        assert_eq!(
            inventaire.utiliser_soin(Soin::Potion, &mut monstre, &mut AleatoireScripte::new([60])),
            Ok(EffetDuSoin::PvRendus(11))
        );
        assert_eq!(monstre.pv_actuels(), 20);
        assert_eq!(inventaire.potions(), 0);
    }

    #[test]
    fn soin_refuse_sans_rien_consommer() {
        let mut inventaire = Inventaire::default();
        let mut aleatoire = AleatoireScripte::new([]);
        assert_eq!(
            inventaire.utiliser_soin(Soin::Potion, &mut blesse(5), &mut aleatoire),
            Err(ErreurSoin::Aucun(Soin::Potion))
        );
        inventaire.ajouter_potions(1);
        assert_eq!(
            inventaire.utiliser_soin(Soin::Potion, &mut blesse(0), &mut aleatoire),
            Err(ErreurSoin::MonstreKo)
        );
        assert_eq!(inventaire.potions(), 1);
    }

    #[test]
    fn potion_perdue_sur_un_monstre_qui_a_tous_ses_pv() {
        let mut inventaire = Inventaire::default();
        inventaire.ajouter_potions(1);
        let mut monstre = blesse(20);
        assert!(soin_sans_effet(Soin::Potion, &monstre));
        assert_eq!(
            inventaire.utiliser_soin(Soin::Potion, &mut monstre, &mut AleatoireScripte::new([])),
            Ok(EffetDuSoin::PvRendus(0))
        );
        assert_eq!(inventaire.potions(), 0);
    }

    #[test]
    fn un_elixir_du_sac_releve_un_monstre_ko() {
        let mut inventaire = Inventaire::default().avec_soins(Soin::Elixir, 1);
        let mut monstre = blesse(0);
        assert_eq!(
            inventaire.utiliser_soin(Soin::Elixir, &mut monstre, &mut AleatoireScripte::new([])),
            Ok(EffetDuSoin::PvRendus(10))
        );
        assert!(inventaire.soins().est_vide());
    }

    #[test]
    fn une_defaite_fait_tout_perdre_et_dit_quoi() {
        let mut inventaire = Inventaire::new(30, 3, vec![capsule("a")]).avec_soins(Soin::Remede, 1);
        let perdu = inventaire.perdre_tout();
        assert!(inventaire.est_vide());
        assert_eq!(
            perdu,
            Inventaire::new(30, 3, vec![capsule("a")]).avec_soins(Soin::Remede, 1)
        );
    }

    #[test]
    fn retirer_un_objet_seulement_s_il_y_en_a_assez() {
        let mut inventaire = Inventaire::new(30, 3, vec![capsule("a"), capsule("a")]);
        assert_eq!(inventaire.capsules().len(), 1);
        assert_eq!(
            inventaire.retirer(&Objet::Soins(Soin::Potion, 4)),
            Err(ErreurObjet::Insuffisant)
        );
        assert_eq!(
            inventaire.retirer(&Objet::Soins(Soin::Elixir, 1)),
            Err(ErreurObjet::Insuffisant)
        );
        assert_eq!(
            inventaire.retirer(&Objet::Argent(0)),
            Err(ErreurObjet::QuantiteNulle)
        );
        assert_eq!(
            inventaire.retirer(&Objet::Capsule(AttaqueId::new("b"))),
            Err(ErreurObjet::CapsuleAbsente)
        );
        assert_eq!(
            inventaire.retirer(&Objet::Argent(10)),
            Ok(Inventaire::new(10, 0, Vec::new()))
        );
        let capsule_a = inventaire
            .retirer(&Objet::Capsule(AttaqueId::new("a")))
            .unwrap();
        assert_eq!(capsule_a.capsules().len(), 1);
        assert_eq!(inventaire, Inventaire::new(20, 3, Vec::new()));
        let mut coffre = Inventaire::new(5, 0, Vec::new());
        coffre.ajouter(capsule_a);
        assert_eq!(coffre, Inventaire::new(5, 0, vec![capsule("a")]));
    }

    #[test]
    fn les_pierres_se_rangent_dans_l_ordre_et_se_deplacent_par_lots() {
        let mut sac = Inventaire::default()
            .avec_pierres(PierreDEvolution::Lune, 1)
            .avec_pierres(PierreDEvolution::Braise, 2);
        assert_eq!(
            sac.pierres(),
            &[(PierreDEvolution::Braise, 2), (PierreDEvolution::Lune, 1)]
        );
        assert!(!sac.est_vide());
        assert_eq!(
            sac.retirer(&Objet::Pierres(PierreDEvolution::Lune, 2)),
            Err(ErreurObjet::Insuffisant)
        );
        let lot = sac
            .retirer(&Objet::Pierres(PierreDEvolution::Lune, 1))
            .unwrap();
        assert_eq!(lot.nombre_de_pierres(PierreDEvolution::Lune), 1);
        assert_eq!(sac.pierres(), &[(PierreDEvolution::Braise, 2)]);
        assert!(sac.utiliser_pierre(PierreDEvolution::Braise));
        assert!(sac.utiliser_pierre(PierreDEvolution::Braise));
        assert!(!sac.utiliser_pierre(PierreDEvolution::Braise));
        assert!(sac.est_vide());
        let mut coffre = Inventaire::default();
        coffre.ajouter(lot);
        assert_eq!(coffre.nombre_de_pierres(PierreDEvolution::Lune), 1);
    }
}
