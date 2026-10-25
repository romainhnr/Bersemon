//! Équipement du héros (section 4.3) : une arme, une armure et deux
//! talismans, fabriqués par Gunther. Chaque objet porté ajoute ses
//! pourcentages aux stats des monstres du joueur en combat, comme la classe.
//! Depuis l'étape 129, les armes et les armures sont propres à une classe :
//! elles donnent aussi au héros son niveau de combat, et l'arme ses
//! attaques (section 4.4).
//!
//! Le héros garde les objets qu'il possède : rien ne les lui fait perdre, et
//! il ne les change qu'au camp.

use std::fmt;

use crate::attaque::Attaque;
use crate::heros::Classe;
use crate::stats::{BonusDeStats, cumuler_bonus};

/// Identifiant stable d'un objet d'équipement, enregistré dans la sauvegarde.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjetDEquipementId(String);

impl ObjetDEquipementId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SorteDObjet {
    Arme,
    Armure,
    Talisman,
}

impl SorteDObjet {
    pub fn nom(self) -> &'static str {
        match self {
            SorteDObjet::Arme => "Arme",
            SorteDObjet::Armure => "Armure",
            SorteDObjet::Talisman => "Talisman",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjetDEquipement {
    id: ObjetDEquipementId,
    nom: String,
    sorte: SorteDObjet,
    bonus: BonusDeStats,
    /// Celle des armes et des armures ; un talisman va à toutes.
    classe: Option<Classe>,
    /// De 1 à 4 pour une arme ou une armure de classe, 0 pour un talisman.
    palier: u8,
    /// Celles que l'arme donne au héros quand il combat (section 4.4).
    attaques: Vec<Attaque>,
}

impl ObjetDEquipement {
    pub fn new(
        id: ObjetDEquipementId,
        nom: impl Into<String>,
        sorte: SorteDObjet,
        bonus: BonusDeStats,
    ) -> Self {
        Self {
            id,
            nom: nom.into(),
            sorte,
            bonus,
            classe: None,
            palier: 0,
            attaques: Vec::new(),
        }
    }

    /// Une arme ou une armure réservée à une classe, de ce palier.
    pub fn de_classe(mut self, classe: Classe, palier: u8) -> Self {
        self.classe = Some(classe);
        self.palier = palier;
        self
    }

    pub fn avec_attaques(mut self, attaques: Vec<Attaque>) -> Self {
        self.attaques = attaques;
        self
    }

    pub fn id(&self) -> &ObjetDEquipementId {
        &self.id
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }

    pub fn sorte(&self) -> SorteDObjet {
        self.sorte
    }

    /// Pourcentages ajoutés aux stats des monstres du joueur en combat.
    pub fn bonus(&self) -> BonusDeStats {
        self.bonus
    }

    pub fn classe(&self) -> Option<Classe> {
        self.classe
    }

    pub fn palier(&self) -> u8 {
        self.palier
    }

    pub fn attaques(&self) -> &[Attaque] {
        &self.attaques
    }

    /// Un héros ne porte que les armes et les armures de sa classe, et tous
    /// les talismans (section 4.3).
    pub fn convient_a(&self, classe: Classe) -> bool {
        self.classe.is_none_or(|sienne| sienne == classe)
    }
}

/// Là où le héros porte un objet. Les deux talismans ont chacun leur place,
/// pour que le joueur choisisse celui qu'il remplace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaceDEquipement {
    Arme,
    Armure,
    PremierTalisman,
    SecondTalisman,
}

impl PlaceDEquipement {
    pub const TOUTES: [PlaceDEquipement; 4] = [
        PlaceDEquipement::Arme,
        PlaceDEquipement::Armure,
        PlaceDEquipement::PremierTalisman,
        PlaceDEquipement::SecondTalisman,
    ];

    pub fn sorte(self) -> SorteDObjet {
        match self {
            PlaceDEquipement::Arme => SorteDObjet::Arme,
            PlaceDEquipement::Armure => SorteDObjet::Armure,
            PlaceDEquipement::PremierTalisman | PlaceDEquipement::SecondTalisman => {
                SorteDObjet::Talisman
            }
        }
    }

    fn index(self) -> usize {
        match self {
            PlaceDEquipement::Arme => 0,
            PlaceDEquipement::Armure => 1,
            PlaceDEquipement::PremierTalisman => 2,
            PlaceDEquipement::SecondTalisman => 3,
        }
    }
}

/// Les objets que possède le héros, dans l'ordre où il les a obtenus, et
/// ceux qu'il porte, chacun à sa place.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Equipement {
    possedes: Vec<ObjetDEquipement>,
    /// Positions dans `possedes`, une par place.
    portes: [Option<usize>; 4],
}

impl Equipement {
    /// Reprise d'une sauvegarde : chaque objet porté doit être possédé, de
    /// la sorte de sa place, et porté une seule fois.
    pub fn restaurer(
        possedes: Vec<ObjetDEquipement>,
        portes: &[(PlaceDEquipement, ObjetDEquipementId)],
    ) -> Result<Self, ErreurEquipement> {
        let mut equipement = Self::default();
        for objet in possedes {
            equipement.ajouter(objet)?;
        }
        for (place, id) in portes {
            if equipement.porte(*place).is_some() {
                return Err(ErreurEquipement::PlaceEnDouble(*place));
            }
            equipement.equiper(*place, id)?;
        }
        Ok(equipement)
    }

    pub fn possedes(&self) -> &[ObjetDEquipement] {
        &self.possedes
    }

    pub fn possede(&self, id: &ObjetDEquipementId) -> bool {
        self.position(id).is_some()
    }

    pub fn porte(&self, place: PlaceDEquipement) -> Option<&ObjetDEquipement> {
        self.portes[place.index()].map(|position| &self.possedes[position])
    }

    /// Les objets possédés qui peuvent aller à cette place : de sa sorte, et
    /// portés nulle part, ni à cette place ni à l'autre place de talisman.
    pub fn disponibles(&self, place: PlaceDEquipement) -> Vec<&ObjetDEquipement> {
        self.possedes
            .iter()
            .enumerate()
            .filter(|(position, objet)| {
                objet.sorte() == place.sorte() && !self.portes.contains(&Some(*position))
            })
            .map(|(_, objet)| objet)
            .collect()
    }

    /// Un objet fabriqué ne l'est qu'une fois (section 13.5).
    pub(crate) fn ajouter(&mut self, objet: ObjetDEquipement) -> Result<(), ErreurEquipement> {
        if self.possede(objet.id()) {
            return Err(ErreurEquipement::DejaPossede(objet.id().clone()));
        }
        self.possedes.push(objet);
        Ok(())
    }

    /// Remplace l'objet porté à cette place. Un talisman déjà porté à l'autre
    /// place y est refusé : les deux talismans sont différents.
    pub fn equiper(
        &mut self,
        place: PlaceDEquipement,
        id: &ObjetDEquipementId,
    ) -> Result<(), ErreurEquipement> {
        let position = self
            .position(id)
            .ok_or_else(|| ErreurEquipement::NonPossede(id.clone()))?;
        if self.possedes[position].sorte() != place.sorte() {
            return Err(ErreurEquipement::MauvaisePlace(place));
        }
        let ailleurs = PlaceDEquipement::TOUTES
            .iter()
            .any(|autre| *autre != place && self.portes[autre.index()] == Some(position));
        if ailleurs {
            return Err(ErreurEquipement::DejaPorte(id.clone()));
        }
        self.portes[place.index()] = Some(position);
        Ok(())
    }

    pub fn retirer(&mut self, place: PlaceDEquipement) -> Result<(), ErreurEquipement> {
        self.portes[place.index()]
            .take()
            .map(|_| ())
            .ok_or(ErreurEquipement::PlaceVide(place))
    }

    /// Reprise d'une sauvegarde d'avant les objets de classe (section 4.3) :
    /// chaque arme ou armure d'une autre classe devient celle de la classe
    /// du héros, de même sorte et de même palier, que donne `equivalent`, et
    /// reste portée si l'ancienne l'était. Deux anciens objets du même palier
    /// n'en donnent qu'un. Un objet sans équivalent est gardé tel quel.
    pub fn adapter_a_la_classe(
        possedes: Vec<ObjetDEquipement>,
        portes: Vec<(PlaceDEquipement, ObjetDEquipementId)>,
        classe: Classe,
        equivalent: impl Fn(SorteDObjet, u8) -> Option<ObjetDEquipement>,
    ) -> (
        Vec<ObjetDEquipement>,
        Vec<(PlaceDEquipement, ObjetDEquipementId)>,
    ) {
        let mut adaptes: Vec<ObjetDEquipement> = Vec::new();
        let mut remplacements: Vec<(ObjetDEquipementId, ObjetDEquipementId)> = Vec::new();
        for objet in possedes {
            let adapte = if objet.convient_a(classe) {
                objet.clone()
            } else {
                equivalent(objet.sorte(), objet.palier()).unwrap_or_else(|| objet.clone())
            };
            if adapte.id() != objet.id() {
                remplacements.push((objet.id().clone(), adapte.id().clone()));
            }
            if !adaptes.iter().any(|deja| deja.id() == adapte.id()) {
                adaptes.push(adapte);
            }
        }
        let portes = portes
            .into_iter()
            .map(|(place, id)| {
                let id = remplacements
                    .iter()
                    .find(|(ancien, _)| *ancien == id)
                    .map_or(id, |(_, nouveau)| nouveau.clone());
                (place, id)
            })
            .collect();
        (adaptes, portes)
    }

    /// Les pourcentages de tous les objets portés, additionnés.
    pub fn bonus(&self) -> BonusDeStats {
        PlaceDEquipement::TOUTES
            .iter()
            .filter_map(|place| self.porte(*place))
            .fold(BonusDeStats::default(), |total, objet| {
                cumuler_bonus(&total, &objet.bonus())
            })
    }

    fn position(&self, id: &ObjetDEquipementId) -> Option<usize> {
        self.possedes.iter().position(|objet| objet.id() == id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurEquipement {
    NonPossede(ObjetDEquipementId),
    DejaPossede(ObjetDEquipementId),
    /// L'objet n'est pas de la sorte de cette place.
    MauvaisePlace(PlaceDEquipement),
    /// Ce talisman est déjà porté à l'autre place.
    DejaPorte(ObjetDEquipementId),
    PlaceVide(PlaceDEquipement),
    /// Deux objets à la même place dans une sauvegarde.
    PlaceEnDouble(PlaceDEquipement),
    /// Une arme ou une armure d'une autre classe que celle du héros.
    AutreClasse(ObjetDEquipementId),
}

impl fmt::Display for ErreurEquipement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurEquipement::NonPossede(id) => write!(f, "objet non possédé : {}", id.as_str()),
            ErreurEquipement::DejaPossede(id) => {
                write!(f, "objet déjà possédé : {}", id.as_str())
            }
            ErreurEquipement::MauvaisePlace(place) => {
                write!(f, "objet qui ne va pas à cette place ({place:?})")
            }
            ErreurEquipement::DejaPorte(id) => write!(f, "objet déjà porté : {}", id.as_str()),
            ErreurEquipement::PlaceVide(place) => write!(f, "aucun objet porté ({place:?})"),
            ErreurEquipement::PlaceEnDouble(place) => {
                write!(f, "deux objets à la même place ({place:?})")
            }
            ErreurEquipement::AutreClasse(id) => {
                write!(f, "objet d'une autre classe : {}", id.as_str())
            }
        }
    }
}

impl std::error::Error for ErreurEquipement {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::Stats;

    fn objet(id: &str, sorte: SorteDObjet, bonus: BonusDeStats) -> ObjetDEquipement {
        ObjetDEquipement::new(ObjetDEquipementId::new(id), id, sorte, bonus)
    }

    fn couteau() -> ObjetDEquipement {
        objet(
            "couteau",
            SorteDObjet::Arme,
            Stats {
                attaque: 5,
                ..Stats::default()
            },
        )
    }

    fn croc() -> ObjetDEquipement {
        objet(
            "croc",
            SorteDObjet::Talisman,
            Stats {
                attaque: 5,
                ..Stats::default()
            },
        )
    }

    fn plume() -> ObjetDEquipement {
        objet(
            "plume",
            SorteDObjet::Talisman,
            Stats {
                vitesse: 5,
                ..Stats::default()
            },
        )
    }

    fn equipement() -> Equipement {
        let mut equipement = Equipement::default();
        for objet in [couteau(), croc(), plume()] {
            equipement.ajouter(objet).unwrap();
        }
        equipement
    }

    #[test]
    fn les_objets_portes_cumulent_leurs_bonus() {
        let mut equipement = equipement();
        assert_eq!(equipement.bonus(), BonusDeStats::default());
        equipement
            .equiper(PlaceDEquipement::Arme, couteau().id())
            .unwrap();
        equipement
            .equiper(PlaceDEquipement::PremierTalisman, croc().id())
            .unwrap();
        equipement
            .equiper(PlaceDEquipement::SecondTalisman, plume().id())
            .unwrap();
        let bonus = equipement.bonus();
        assert_eq!((bonus.attaque, bonus.vitesse, bonus.defense), (10, 5, 0));
        equipement.retirer(PlaceDEquipement::Arme).unwrap();
        assert_eq!(equipement.bonus().attaque, 5);
    }

    #[test]
    fn un_objet_ne_va_qu_a_une_place_de_sa_sorte() {
        let mut equipement = equipement();
        assert_eq!(
            equipement.equiper(PlaceDEquipement::Armure, couteau().id()),
            Err(ErreurEquipement::MauvaisePlace(PlaceDEquipement::Armure))
        );
        let inconnu = ObjetDEquipementId::new("inconnu");
        assert_eq!(
            equipement.equiper(PlaceDEquipement::Arme, &inconnu),
            Err(ErreurEquipement::NonPossede(inconnu))
        );
        assert_eq!(
            equipement.retirer(PlaceDEquipement::Armure),
            Err(ErreurEquipement::PlaceVide(PlaceDEquipement::Armure))
        );
    }

    #[test]
    fn les_deux_talismans_sont_differents() {
        let mut equipement = equipement();
        equipement
            .equiper(PlaceDEquipement::PremierTalisman, croc().id())
            .unwrap();
        assert_eq!(
            equipement.equiper(PlaceDEquipement::SecondTalisman, croc().id()),
            Err(ErreurEquipement::DejaPorte(croc().id().clone()))
        );
        let disponibles: Vec<_> = equipement
            .disponibles(PlaceDEquipement::SecondTalisman)
            .into_iter()
            .map(|objet| objet.id().as_str())
            .collect();
        assert_eq!(disponibles, ["plume"]);
        // Remis à sa propre place, il ne change rien.
        equipement
            .equiper(PlaceDEquipement::PremierTalisman, croc().id())
            .unwrap();
    }

    #[test]
    fn un_objet_ne_se_possede_qu_une_fois() {
        let mut equipement = equipement();
        assert_eq!(
            equipement.ajouter(couteau()),
            Err(ErreurEquipement::DejaPossede(couteau().id().clone()))
        );
        assert_eq!(equipement.possedes().len(), 3);
    }

    #[test]
    fn la_reprise_refuse_un_equipement_incoherent() {
        let portes = [
            (PlaceDEquipement::Arme, couteau().id().clone()),
            (PlaceDEquipement::SecondTalisman, plume().id().clone()),
        ];
        let equipement = Equipement::restaurer(vec![couteau(), plume()], &portes).unwrap();
        assert_eq!(
            equipement.porte(PlaceDEquipement::SecondTalisman),
            Some(&plume())
        );
        let en_double = [
            (PlaceDEquipement::Arme, couteau().id().clone()),
            (PlaceDEquipement::Arme, couteau().id().clone()),
        ];
        assert_eq!(
            Equipement::restaurer(vec![couteau()], &en_double),
            Err(ErreurEquipement::PlaceEnDouble(PlaceDEquipement::Arme))
        );
        assert!(Equipement::restaurer(vec![couteau(), couteau()], &[]).is_err());
    }

    fn arme(id: &str, classe: Classe, palier: u8) -> ObjetDEquipement {
        objet(id, SorteDObjet::Arme, BonusDeStats::default()).de_classe(classe, palier)
    }

    #[test]
    fn un_talisman_va_a_toutes_les_classes_une_arme_a_la_sienne() {
        let arme = arme("lame", Classe::Guerrier, 1);
        assert!(arme.convient_a(Classe::Guerrier));
        assert!(!arme.convient_a(Classe::Mage));
        assert!(
            Classe::TOUTES
                .iter()
                .all(|classe| croc().convient_a(*classe))
        );
    }

    #[test]
    fn les_anciens_objets_deviennent_ceux_de_la_classe_du_heros() {
        let equivalent = |sorte: SorteDObjet, palier: u8| match (sorte, palier) {
            (SorteDObjet::Arme, 1) => Some(arme("baton", Classe::Mage, 1)),
            _ => None,
        };
        let possedes = vec![
            arme("lame", Classe::Guerrier, 1),
            arme("couteau-de-classe", Classe::Assassin, 1),
            arme("epee", Classe::Guerrier, 2),
            croc(),
        ];
        let portes = vec![
            (
                PlaceDEquipement::Arme,
                ObjetDEquipementId::new("couteau-de-classe"),
            ),
            (PlaceDEquipement::PremierTalisman, croc().id().clone()),
        ];
        let (possedes, portes) =
            Equipement::adapter_a_la_classe(possedes, portes, Classe::Mage, equivalent);
        let ids: Vec<_> = possedes.iter().map(|objet| objet.id().as_str()).collect();
        // L'épée, sans équivalent, est gardée telle quelle.
        assert_eq!(ids, ["baton", "epee", "croc"]);
        assert_eq!(
            portes[0],
            (PlaceDEquipement::Arme, ObjetDEquipementId::new("baton"))
        );
        assert_eq!(
            portes[1],
            (PlaceDEquipement::PremierTalisman, croc().id().clone())
        );
    }
}
