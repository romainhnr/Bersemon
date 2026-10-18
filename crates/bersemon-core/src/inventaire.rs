//! Objets du joueur, rangés dans le sac, qu'il emporte en expédition
//! manuelle, ou dans le coffre du camp, où ils sont à l'abri (sections 13.3
//! et 14.4) : argent, potions et capsules d'attaque.

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::attaque::AttaqueId;
use crate::capsule::Capsule;
use crate::monstre::Monstre;

/// Fourchette du pourcentage de PV max rendu par une potion.
const POURCENTAGE_MIN: u32 = 40;
const POURCENTAGE_MAX: u32 = 60;

/// Le contenu du sac ou du coffre. Pas de limite de place pour l'instant
/// (section 13.3).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Inventaire {
    argent: u32,
    potions: u32,
    /// Sans doublon : une capsule sert sans s'user (section 13.10).
    capsules: Vec<Capsule>,
}

/// Ce que le joueur déplace entre le sac et le coffre du camp.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Objet {
    Argent(u32),
    Potions(u32),
    Capsule(AttaqueId),
}

impl Inventaire {
    /// Reprise d'une sauvegarde ; les capsules en double ne sont gardées
    /// qu'une fois.
    pub fn new(argent: u32, potions: u32, capsules: Vec<Capsule>) -> Self {
        let mut inventaire = Self {
            argent,
            potions,
            capsules: Vec::new(),
        };
        for capsule in capsules {
            inventaire.ajouter_capsule(capsule);
        }
        inventaire
    }

    /// En pièces d'argent (section 14.1).
    pub fn argent(&self) -> u32 {
        self.argent
    }

    pub fn potions(&self) -> u32 {
        self.potions
    }

    pub fn capsules(&self) -> &[Capsule] {
        &self.capsules
    }

    pub fn est_vide(&self) -> bool {
        self.argent == 0 && self.potions == 0 && self.capsules.is_empty()
    }

    pub fn contient_capsule(&self, id: &AttaqueId) -> bool {
        self.capsules.iter().any(|capsule| capsule.id() == id)
    }

    pub fn ajouter_potions(&mut self, nombre: u32) {
        self.potions = self.potions.saturating_add(nombre);
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
        self.ajouter_potions(autre.potions);
        for capsule in autre.capsules {
            self.ajouter_capsule(capsule);
        }
    }

    /// Retire l'objet et le rend, seul dans un inventaire, sans rien changer
    /// s'il manque.
    pub(crate) fn retirer(&mut self, objet: &Objet) -> Result<Inventaire, ErreurObjet> {
        let mut retire = Inventaire::default();
        match objet {
            Objet::Argent(0) | Objet::Potions(0) => return Err(ErreurObjet::QuantiteNulle),
            Objet::Argent(montant) => {
                if *montant > self.argent {
                    return Err(ErreurObjet::Insuffisant);
                }
                self.argent -= montant;
                retire.argent = *montant;
            }
            Objet::Potions(nombre) => {
                if *nombre > self.potions {
                    return Err(ErreurObjet::Insuffisant);
                }
                self.potions -= nombre;
                retire.potions = *nombre;
            }
            Objet::Capsule(id) => {
                let position = self
                    .capsules
                    .iter()
                    .position(|capsule| capsule.id() == id)
                    .ok_or(ErreurObjet::CapsuleAbsente)?;
                retire.capsules.push(self.capsules.remove(position));
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

    /// Une défaite fait perdre tout le sac (section 9.3). Rend ce qui est
    /// perdu.
    pub fn perdre_tout(&mut self) -> Inventaire {
        std::mem::take(self)
    }

    /// Après un combat, qui a pu consommer des potions.
    pub(crate) fn definir_potions(&mut self, potions: u32) {
        self.potions = potions;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurPotion {
    AucunePotion,
    MonstreInexistant,
    MonstreKo,
}

impl fmt::Display for ErreurPotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurPotion::AucunePotion => write!(f, "aucune potion"),
            ErreurPotion::MonstreInexistant => write!(f, "ce monstre n'est pas dans l'équipe"),
            ErreurPotion::MonstreKo => write!(f, "une potion ne relève pas un monstre KO"),
        }
    }
}

impl std::error::Error for ErreurPotion {}

/// Une potion ne relève pas un monstre KO (section 14.4). Sur un monstre qui
/// a tous ses PV, elle est permise mais perdue : demander confirmation au
/// joueur revient à l'interface (`potion_sans_effet`, Q53).
pub fn verifier_cible(monstre: &Monstre) -> Result<(), ErreurPotion> {
    if monstre.est_ko() {
        Err(ErreurPotion::MonstreKo)
    } else {
        Ok(())
    }
}

/// Le monstre a tous ses PV : une potion serait consommée sans effet.
pub fn potion_sans_effet(monstre: &Monstre) -> bool {
    monstre.pv_actuels() >= monstre.stats().pv
}

/// Rend un pourcentage des PV max tiré de 40 % à 60 % (un tirage, sauf si
/// le monstre a déjà tous ses PV). Rend les PV réellement rendus.
pub(crate) fn appliquer_potion(monstre: &mut Monstre, aleatoire: &mut dyn Aleatoire) -> u32 {
    if potion_sans_effet(monstre) {
        return 0;
    }
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

    fn capsule(id: &str) -> Capsule {
        Capsule::new(attaque(id, Type::Normal, Categorie::Physique, 40, 100), 150)
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
        assert_eq!(inventaire.potions(), 1);
    }

    #[test]
    fn potion_perdue_sur_un_monstre_qui_a_tous_ses_pv() {
        let mut inventaire = Inventaire::default();
        inventaire.ajouter_potions(1);
        let mut monstre = blesse(20);
        assert!(potion_sans_effet(&monstre));
        assert_eq!(
            inventaire.utiliser_potion(&mut monstre, &mut AleatoireScripte::new([])),
            Ok(0)
        );
        assert_eq!(inventaire.potions(), 0);
    }

    #[test]
    fn une_defaite_fait_tout_perdre_et_dit_quoi() {
        let mut inventaire = Inventaire::new(30, 3, vec![capsule("a")]);
        let perdu = inventaire.perdre_tout();
        assert!(inventaire.est_vide());
        assert_eq!(perdu, Inventaire::new(30, 3, vec![capsule("a")]));
    }

    #[test]
    fn retirer_un_objet_seulement_s_il_y_en_a_assez() {
        let mut inventaire = Inventaire::new(30, 3, vec![capsule("a"), capsule("a")]);
        assert_eq!(inventaire.capsules().len(), 1);
        assert_eq!(
            inventaire.retirer(&Objet::Potions(4)),
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
}
