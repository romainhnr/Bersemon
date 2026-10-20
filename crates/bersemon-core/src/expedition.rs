//! Expédition manuelle dans une zone (section 9.2) : une suite d'étapes,
//! chacune avec son événement.

use std::fmt;
use std::ops::RangeInclusive;

use crate::aleatoire::Aleatoire;
use crate::ame::MarchandMort;
use crate::attaque::AttaqueId;
use crate::capsule::Capsule;
use crate::expedition_automatique::ErreurParametres;
use crate::humain::Humain;
use crate::monstre::Monstre;
use crate::peche::Pecheur;
use crate::pierre::PierreDEvolution;
use crate::traque::Apotre;
use crate::zone::{MonstreRare, Zone, tirer_entre};

/// Un coffre contient 1 à 2 potions (Q39).
const POTIONS_MIN: u32 = 1;
const POTIONS_MAX: u32 = 2;
/// Un coffre sur 5 contient une capsule d'attaque (Q69).
const CHANCE_DE_CAPSULE: u32 = 20;
/// Un coffre sur 10 contient une capsule d'habileté (section 5.9, Q81), et
/// un autre sur 10 une pierre d'évolution, chacune pour moitié (section 5.4,
/// Q84) : un même tirage sur 100 décide des deux, 1 à 10 pour la capsule,
/// puis 5 chances par pierre.
const CHANCE_DE_CAPSULE_D_HABILETE: u32 = 10;
const CHANCE_PAR_PIERRE: u32 = 5;

/// Contenu des coffres propre à chaque zone (section 14.5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParametresCoffre {
    argent: RangeInclusive<u32>,
    capsules: Vec<Capsule>,
}

impl ParametresCoffre {
    /// `capsules` : celles que ses coffres peuvent contenir (section 13.10).
    pub fn new(
        argent: RangeInclusive<u32>,
        capsules: Vec<Capsule>,
    ) -> Result<Self, ErreurParametres> {
        if argent.is_empty() {
            return Err(ErreurParametres::FourchetteInversee);
        }
        Ok(Self { argent, capsules })
    }

    pub fn argent(&self) -> &RangeInclusive<u32> {
        &self.argent
    }

    pub fn capsules(&self) -> &[Capsule] {
        &self.capsules
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvenementExpedition {
    Combat(Vec<Monstre>),
    /// Son contenu va dans le sac, et se perd avec lui en cas de défaite
    /// (section 9.3).
    Coffre {
        potions: u32,
        argent: u32,
        capsule: Option<Capsule>,
        capsule_d_habilete: bool,
        pierre: Option<PierreDEvolution>,
    },
    MonstreRare(MonstreRare),
    /// Combat, sans fuite, contre l'Apôtre (section 9.4).
    Apotre(Monstre),
    /// Combat, ni fuite ni filet, contre un humain et son équipe (section
    /// 10.7).
    Humain {
        humain: Humain,
        equipe: Vec<Monstre>,
    },
    /// Combat contre ce qui tue le marchand mort de la zone, ou garde son
    /// corps : la victoire recueille son âme (section 14.2). En boîte, pour
    /// que l'événement reste léger.
    Ame {
        marchand: Box<MarchandMort>,
        adversaire: Monstre,
    },
    /// Combat ordinaire contre ce qu'Ansgar vient de ferrer : la victoire
    /// l'installe au camp (section 9.2).
    Pecheur {
        pecheur: Box<Pecheur>,
        adversaire: Monstre,
    },
}

/// Quand l'âme du marchand mort de la zone se rencontre (section 14.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RencontreDeLAme {
    /// 10 % des étapes, à la place d'un combat.
    Tiree,
    /// À coup sûr, à cette étape.
    AEtape(u32),
    /// Après une fuite ou une défaite : à coup sûr, à une étape tirée au
    /// départ.
    ARetrouver,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurExpedition {
    ZoneFermee,
    /// Toute l'équipe est partie en expédition automatique, ou le joueur l'a
    /// vidée (section 13.2).
    EquipeVide,
}

impl fmt::Display for ErreurExpedition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurExpedition::ZoneFermee => write!(f, "cette zone n'est pas encore sur la carte"),
            ErreurExpedition::EquipeVide => write!(f, "l'équipe est vide"),
        }
    }
}

impl std::error::Error for ErreurExpedition {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expedition {
    zone: Zone,
    /// Fixé par la zone et connu du joueur (section 9.2).
    nombre_etapes: u32,
    etapes_faites: u32,
    defaite: bool,
    /// L'Apôtre qui attend à la dernière étape, une fois son antre localisé.
    apotre: Option<Apotre>,
    /// L'humain qui attend à la dernière étape tant qu'il n'est pas vaincu
    /// (section 10.7).
    humain: Option<Humain>,
    /// Le marchand mort de la zone, tant que son âme peut être trouvée, et
    /// l'étape où elle se rencontre ; une fois au plus (section 14.2).
    ame: Option<(MarchandMort, RencontreDeLAme)>,
    /// Ansgar, tant qu'il n'est pas au camp ; une fois au plus.
    pecheur: Option<Pecheur>,
    /// Un coffre ne contient qu'une capsule que le joueur n'a pas encore.
    capsules_possedees: Vec<AttaqueId>,
    /// Pour qu'un autre coffre de la même expédition ne la contienne pas.
    capsules_trouvees: Vec<Capsule>,
}

impl Expedition {
    /// Le départ passe par `Partie::partir_en_expedition`, qui vérifie que
    /// la zone est ouverte et place l'Apôtre au bout de l'expédition quand il
    /// le faut.
    pub(crate) fn partir(zone: Zone) -> Self {
        Self {
            nombre_etapes: zone.etapes(),
            zone,
            etapes_faites: 0,
            defaite: false,
            apotre: None,
            humain: None,
            ame: None,
            pecheur: None,
            capsules_possedees: Vec::new(),
            capsules_trouvees: Vec::new(),
        }
    }

    /// L'Apôtre qui attend au bout de l'expédition, s'il y en a un.
    pub fn apotre(&self) -> Option<&Apotre> {
        self.apotre.as_ref()
    }

    pub(crate) fn vers_l_antre(mut self, apotre: Apotre) -> Self {
        self.apotre = Some(apotre);
        self
    }

    pub(crate) fn contre_l_humain(mut self, humain: Humain) -> Self {
        self.humain = Some(humain);
        self
    }

    /// Celui qui attend au bout de cette expédition.
    pub fn humain(&self) -> Option<&Humain> {
        self.humain.as_ref()
    }

    pub(crate) fn avec_ame(mut self, marchand: MarchandMort, rencontre: RencontreDeLAme) -> Self {
        self.ame = Some((marchand, rencontre));
        self
    }

    pub(crate) fn avec_pecheur(mut self, pecheur: Pecheur) -> Self {
        self.pecheur = Some(pecheur);
        self
    }

    pub(crate) fn avec_capsules_possedees(mut self, capsules: Vec<AttaqueId>) -> Self {
        self.capsules_possedees = capsules;
        self
    }

    pub fn zone(&self) -> &Zone {
        &self.zone
    }

    pub fn etapes_faites(&self) -> u32 {
        self.etapes_faites
    }

    /// Celui de la zone, montré au joueur (section 9.2).
    pub fn nombre_etapes(&self) -> u32 {
        self.nombre_etapes
    }

    /// Plus d'étape à faire : après la dernière, ou après une défaite.
    pub fn est_terminee(&self) -> bool {
        self.defaite || self.etapes_faites >= self.nombre_etapes
    }

    /// Menée jusqu'à sa dernière étape sans défaite : la zone compte alors
    /// comme explorée (sections 9.2 et 9.3).
    pub fn est_reussie(&self) -> bool {
        !self.defaite && self.etapes_faites >= self.nombre_etapes
    }

    /// Une défaite renvoie au camp et met fin à l'expédition (section 9.3).
    pub fn noter_defaite(&mut self) {
        self.defaite = true;
    }

    /// Passe à l'étape suivante et tire son événement ; `None` si
    /// l'expédition est terminée. Tirages : à la première étape, celle d'une
    /// âme à retrouver, parmi celles qui précèdent la dernière ; puis la
    /// nature de l'événement (combat 80 %, coffre 15 %, monstre rare 5 %,
    /// Q38 ; tant que l'âme du marchand de la zone peut être tirée, combat
    /// 70 % et âme 10 %, section 14.2), puis ceux de son contenu : pour un
    /// coffre, ses potions, son argent, sa capsule d'attaque puis sa capsule
    /// d'habileté ; pour une âme, ceux
    /// de son adversaire. L'âme attendue à cette étape remplace l'événement,
    /// sans tirage de sa nature.
    /// Une zone sans monstre rare donne un combat à la place. L'Apôtre
    /// remplace l'événement de la dernière étape, sans tirage de sa nature
    /// (Q57), comme l'humain de la zone, sans aucun tirage (section 10.7).
    /// Ansgar remplace celui de son étape, avant l'âme, avec les tirages de
    /// son adversaire (section 9.2).
    pub fn etape_suivante(&mut self, aleatoire: &mut dyn Aleatoire) -> Option<EvenementExpedition> {
        if self.est_terminee() {
            return None;
        }
        if self.etapes_faites == 0 {
            self.placer_l_ame_a_retrouver(aleatoire);
        }
        self.etapes_faites += 1;
        let derniere = self.etapes_faites == self.nombre_etapes;
        if let Some(apotre) = self.apotre.as_ref().filter(|_| derniere) {
            return Some(EvenementExpedition::Apotre(apotre.generer(aleatoire)));
        }
        if let Some(humain) = self.humain.as_ref().filter(|_| derniere) {
            return Some(EvenementExpedition::Humain {
                humain: humain.clone(),
                equipe: humain.generer_equipe(),
            });
        }
        let etape = self.etapes_faites;
        if let Some(pecheur) = self.pecheur.take_if(|pecheur| pecheur.etape() == etape) {
            let adversaire = pecheur.generer_adversaire(aleatoire);
            return Some(EvenementExpedition::Pecheur {
                pecheur: Box::new(pecheur),
                adversaire,
            });
        }
        if matches!(&self.ame, Some((_, RencontreDeLAme::AEtape(attendue))) if *attendue == etape) {
            return Some(self.rencontrer_l_ame(aleatoire));
        }
        let tirable = matches!(&self.ame, Some((_, RencontreDeLAme::Tiree)));
        let evenement = match aleatoire.tirer(1..=100) {
            71..=80 if tirable => self.rencontrer_l_ame(aleatoire),
            1..=80 => EvenementExpedition::Combat(self.zone.generer_groupe(aleatoire)),
            81..=95 => self.tirer_un_coffre(aleatoire),
            _ => match self.zone.generer_monstre_rare(aleatoire) {
                Some(rare) => EvenementExpedition::MonstreRare(rare),
                None => EvenementExpedition::Combat(self.zone.generer_groupe(aleatoire)),
            },
        };
        Some(evenement)
    }

    /// Une âme à retrouver se rencontre à une étape tirée parmi celles qui
    /// précèdent la dernière, réservée aux combats de fin de zone (section
    /// 14.2), sans celle d'Ansgar.
    fn placer_l_ame_a_retrouver(&mut self, aleatoire: &mut dyn Aleatoire) {
        let derniere_possible = self.nombre_etapes.saturating_sub(1).max(1);
        let etape_d_ansgar = self
            .pecheur
            .as_ref()
            .map(Pecheur::etape)
            .filter(|etape| *etape <= derniere_possible && derniere_possible > 1);
        if let Some((_, rencontre)) = &mut self.ame {
            if *rencontre == RencontreDeLAme::ARetrouver {
                let etape = match etape_d_ansgar {
                    // Les étapes après la sienne se décalent d'un cran.
                    Some(reservee) => {
                        let tiree = aleatoire.tirer(1..=derniere_possible - 1);
                        if tiree >= reservee { tiree + 1 } else { tiree }
                    }
                    None => aleatoire.tirer(1..=derniere_possible),
                };
                *rencontre = RencontreDeLAme::AEtape(etape);
            }
        }
    }

    /// Tirages : ceux de l'adversaire.
    fn rencontrer_l_ame(&mut self, aleatoire: &mut dyn Aleatoire) -> EvenementExpedition {
        let (marchand, _) = self.ame.take().expect("âme attendue");
        let adversaire = marchand.generer_adversaire(&self.zone, aleatoire);
        EvenementExpedition::Ame {
            marchand: Box::new(marchand),
            adversaire,
        }
    }

    fn tirer_un_coffre(&mut self, aleatoire: &mut dyn Aleatoire) -> EvenementExpedition {
        let potions = aleatoire.tirer(POTIONS_MIN..=POTIONS_MAX);
        let fourchette = self.zone.coffre().argent();
        let argent = tirer_entre(*fourchette.start(), *fourchette.end(), aleatoire);
        let capsule = self.tirer_une_capsule(aleatoire);
        if let Some(capsule) = &capsule {
            self.capsules_trouvees.push(capsule.clone());
        }
        let tirage = aleatoire.tirer(1..=100);
        let capsule_d_habilete = tirage <= CHANCE_DE_CAPSULE_D_HABILETE;
        let pierre = tirage
            .checked_sub(CHANCE_DE_CAPSULE_D_HABILETE + 1)
            .map(|rang| (rang / CHANCE_PAR_PIERRE) as usize)
            .and_then(|rang| PierreDEvolution::TOUTES.get(rang).copied());
        EvenementExpedition::Coffre {
            potions,
            argent,
            capsule,
            capsule_d_habilete,
            pierre,
        }
    }

    /// Parmi les capsules de la zone que le joueur n'a ni obtenues, ni
    /// trouvées pendant cette expédition. Tirages, s'il en reste : la
    /// chance d'en trouver une, puis laquelle.
    fn tirer_une_capsule(&self, aleatoire: &mut dyn Aleatoire) -> Option<Capsule> {
        let nouvelles: Vec<&Capsule> = self
            .zone
            .coffre()
            .capsules()
            .iter()
            .filter(|capsule| {
                !self.capsules_possedees.contains(capsule.id())
                    && !self
                        .capsules_trouvees
                        .iter()
                        .any(|trouvee| trouvee.id() == capsule.id())
            })
            .collect();
        if nouvelles.is_empty() || aleatoire.tirer(1..=100) > CHANCE_DE_CAPSULE {
            return None;
        }
        let position = tirer_entre(0, (nouvelles.len() - 1) as u32, aleatoire) as usize;
        Some(nouvelles[position].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::ame::{Gardien, MarchandId};
    use crate::donnees_de_test::{espece, parametres_coffre, parametres_expedition_automatique};
    use crate::niveau::Niveau;
    use crate::soin::Soin;
    use crate::traque::Apotre;
    use crate::zone::{EspeceRare, ZoneId};

    /// Niveau 5 fixe et groupes d'un monstre : un monstre ne tire que son
    /// shiny et ses 6 valeurs internes.
    fn zone(avec_rare: bool) -> Zone {
        let rares = if avec_rare {
            vec![EspeceRare {
                espece: espece("cerf-blanc"),
                pacifique: true,
            }]
        } else {
            Vec::new()
        };
        let niveau = Niveau::new(5).unwrap();
        Zone::new(
            ZoneId::new("bois"),
            "Bois",
            (niveau, niveau),
            4,
            1,
            vec![espece("loup")],
            rares,
            parametres_expedition_automatique(),
            parametres_coffre(),
        )
        .unwrap()
    }

    const MONSTRE: [u32; 7] = [2, 0, 0, 0, 0, 0, 0];

    #[test]
    fn evenements_selon_le_tirage() {
        let mut expedition = Expedition::partir(zone(true));
        let mut combat = vec![80];
        combat.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(combat)),
            Some(EvenementExpedition::Combat(groupe)) if groupe.len() == 1
        ));
        assert_eq!(
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 2, 21])),
            Some(EvenementExpedition::Coffre {
                potions: 2,
                argent: 10,
                capsule: None,
                capsule_d_habilete: false,
                pierre: None
            })
        );
        let mut rare = vec![96];
        rare.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(rare)),
            Some(EvenementExpedition::MonstreRare(rare)) if rare.pacifique
        ));
        assert_eq!(expedition.etapes_faites(), 3);
    }

    #[test]
    fn sans_monstre_rare_la_zone_donne_un_combat() {
        let mut expedition = Expedition::partir(zone(false));
        let mut tirages = vec![100];
        tirages.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(tirages)),
            Some(EvenementExpedition::Combat(_))
        ));
    }

    #[test]
    fn reussie_apres_la_derniere_etape() {
        let mut expedition = Expedition::partir(zone(false));
        for _ in 0..4 {
            assert!(!expedition.est_terminee());
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 100]));
        }
        assert!(expedition.est_terminee());
        assert!(expedition.est_reussie());
        assert_eq!(
            expedition.etape_suivante(&mut AleatoireScripte::new([])),
            None
        );
    }

    #[test]
    fn l_apotre_attend_a_la_derniere_etape() {
        let apotre = Apotre::new(espece("baron"), &zone(false), "", "").unwrap();
        let mut expedition = Expedition::partir(zone(false)).vers_l_antre(apotre);
        for _ in 0..3 {
            assert!(matches!(
                expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 100])),
                Some(EvenementExpedition::Coffre { .. })
            ));
        }
        // Seul le shiny de l'Apôtre est tiré.
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new([2])),
            Some(EvenementExpedition::Apotre(apotre)) if apotre.niveau() == Niveau::new(7).unwrap()
        ));
        assert!(expedition.est_terminee());
    }

    #[test]
    fn l_humain_attend_a_la_derniere_etape_sans_tirage() {
        let niveau = Niveau::new(5).unwrap();
        let brigand = Humain::new(
            crate::humain::HumainId::new("brigand"),
            "Brigand",
            vec![(espece("loup"), niveau)],
            60,
            "",
            "",
        )
        .unwrap();
        let mut expedition = Expedition::partir(zone(false)).contre_l_humain(brigand);
        for _ in 0..3 {
            assert!(matches!(
                expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 100])),
                Some(EvenementExpedition::Coffre { .. })
            ));
        }
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new([])),
            Some(EvenementExpedition::Humain { humain, equipe })
                if humain.nom() == "Brigand" && equipe.len() == 1
        ));
        assert!(expedition.est_terminee());
    }

    #[test]
    fn une_defaite_termine_l_expedition_sans_la_reussir() {
        let mut expedition = Expedition::partir(zone(false));
        for _ in 0..4 {
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 100]));
        }
        expedition.noter_defaite();
        assert!(expedition.est_terminee());
        assert!(!expedition.est_reussie());
    }

    #[test]
    fn l_argent_des_coffres_est_tire_apres_les_potions() {
        let niveau = Niveau::new(5).unwrap();
        let zone = Zone::new(
            ZoneId::new("bois"),
            "Bois",
            (niveau, niveau),
            4,
            1,
            vec![espece("loup")],
            Vec::new(),
            parametres_expedition_automatique(),
            ParametresCoffre::new(10..=25, Vec::new()).unwrap(),
        )
        .unwrap();
        let mut expedition = Expedition::partir(zone.clone());
        assert_eq!(
            expedition.etape_suivante(&mut AleatoireScripte::new([90, 1, 25, 10])),
            Some(EvenementExpedition::Coffre {
                potions: 1,
                argent: 25,
                capsule: None,
                capsule_d_habilete: true,
                pierre: None
            })
        );
        // Le même tirage donne une pierre, de 11 à 20.
        for (tirage, pierre) in [
            (11, Some(PierreDEvolution::Braise)),
            (15, Some(PierreDEvolution::Braise)),
            (16, Some(PierreDEvolution::Lune)),
            (20, Some(PierreDEvolution::Lune)),
            (21, None),
        ] {
            let mut expedition = Expedition::partir(zone.clone());
            assert!(matches!(
                expedition.etape_suivante(&mut AleatoireScripte::new([90, 1, 25, tirage])),
                Some(EvenementExpedition::Coffre { capsule_d_habilete: false, pierre: trouvee, .. })
                    if trouvee == pierre
            ));
        }
    }

    #[test]
    fn le_nombre_d_etapes_est_celui_de_la_zone() {
        let expedition = Expedition::partir(zone(false));
        assert_eq!(expedition.nombre_etapes(), 4);
        assert_eq!(expedition.etapes_faites(), 0);
    }

    #[test]
    fn l_ame_se_trouve_une_fois_a_la_place_d_un_combat() {
        let bertrand = MarchandMort::new(
            MarchandId::new("bertrand"),
            "Bertrand",
            Soin::Remede,
            Gardien::MonstreDeLaZone,
            "",
            "",
        )
        .unwrap();
        let mut expedition =
            Expedition::partir(zone(false)).avec_ame(bertrand, RencontreDeLAme::Tiree);
        let mut combat = vec![70];
        combat.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(combat)),
            Some(EvenementExpedition::Combat(_))
        ));
        // Le loup qui tue Bertrand est au niveau 5, le seul de la zone.
        let mut ame = vec![71];
        ame.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(ame)),
            Some(EvenementExpedition::Ame { marchand, .. }) if marchand.nom() == "Bertrand"
        ));
        // Il ne se rencontre qu'une fois.
        let mut combat = vec![80];
        combat.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(combat)),
            Some(EvenementExpedition::Combat(_))
        ));
    }

    #[test]
    fn l_ame_attendue_a_son_etape_remplace_l_evenement_sans_tirage() {
        let bertrand = MarchandMort::new(
            MarchandId::new("bertrand"),
            "Bertrand",
            Soin::Remede,
            Gardien::MonstreDeLaZone,
            "",
            "",
        )
        .unwrap();
        let mut expedition =
            Expedition::partir(zone(false)).avec_ame(bertrand, RencontreDeLAme::AEtape(2));
        // À la première étape, le tirage de 71 à 80 reste un combat.
        let mut combat = vec![71];
        combat.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(combat)),
            Some(EvenementExpedition::Combat(_))
        ));
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(MONSTRE)),
            Some(EvenementExpedition::Ame { .. })
        ));
    }

    #[test]
    fn sans_ame_possible_le_tirage_donne_un_combat() {
        let mut expedition = Expedition::partir(zone(false));
        let mut combat = vec![75];
        combat.extend(MONSTRE);
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(combat)),
            Some(EvenementExpedition::Combat(_))
        ));
    }

    #[test]
    fn fourchette_d_argent_inversee_refusee() {
        assert_eq!(
            ParametresCoffre::new(RangeInclusive::new(2, 1), Vec::new()),
            Err(ErreurParametres::FourchetteInversee)
        );
    }

    fn capsule(id: &str) -> Capsule {
        Capsule::new(
            crate::donnees_de_test::attaque(
                id,
                crate::types::Type::Normal,
                crate::attaque::Categorie::Physique,
                40,
                100,
            ),
            150,
        )
    }

    /// Zone dont les coffres contiennent 10 pièces d'argent et peut-être une
    /// des capsules a, b et c.
    fn zone_a_capsules() -> Zone {
        let niveau = Niveau::new(5).unwrap();
        Zone::new(
            ZoneId::new("bois"),
            "Bois",
            (niveau, niveau),
            4,
            1,
            vec![espece("loup")],
            Vec::new(),
            parametres_expedition_automatique(),
            ParametresCoffre::new(10..=10, vec![capsule("a"), capsule("b"), capsule("c")]).unwrap(),
        )
        .unwrap()
    }

    fn capsule_du_coffre(evenement: Option<EvenementExpedition>) -> Option<String> {
        match evenement {
            Some(EvenementExpedition::Coffre { capsule, .. }) => {
                capsule.map(|capsule| capsule.id().as_str().to_string())
            }
            autre => panic!("coffre attendu : {autre:?}"),
        }
    }

    #[test]
    fn une_capsule_nouvelle_dans_un_coffre_sur_cinq() {
        let mut expedition = Expedition::partir(zone_a_capsules())
            .avec_capsules_possedees(vec![AttaqueId::new("a")]);
        // Au-delà de 20, pas de capsule.
        assert_eq!(
            capsule_du_coffre(
                expedition.etape_suivante(&mut AleatoireScripte::new([90, 1, 21, 100]))
            ),
            None
        );
        // Parmi b et c, la seconde.
        assert_eq!(
            capsule_du_coffre(
                expedition.etape_suivante(&mut AleatoireScripte::new([90, 1, 20, 1, 100]))
            ),
            Some("c".to_string())
        );
        // Seule b reste : aucun tirage de position.
        assert_eq!(
            capsule_du_coffre(
                expedition.etape_suivante(&mut AleatoireScripte::new([90, 1, 1, 100]))
            ),
            Some("b".to_string())
        );
        // Plus rien de nouveau : ni chance ni position tirées.
        assert_eq!(
            capsule_du_coffre(expedition.etape_suivante(&mut AleatoireScripte::new([90, 1, 100]))),
            None
        );
    }

    fn ansgar() -> Pecheur {
        Pecheur::new(2, espece("noye"), Niveau::new(5).unwrap(), "", "").unwrap()
    }

    #[test]
    fn ansgar_attend_a_son_etape_sans_tirage_de_nature() {
        let mut expedition = Expedition::partir(zone(false)).avec_pecheur(ansgar());
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 100])),
            Some(EvenementExpedition::Coffre { .. })
        ));
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(MONSTRE)),
            Some(EvenementExpedition::Pecheur { adversaire, .. })
                if adversaire.espece().id().as_str() == "noye"
        ));
        // Une fois au plus.
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 100])),
            Some(EvenementExpedition::Coffre { .. })
        ));
    }

    #[test]
    fn l_ame_a_retrouver_laisse_son_etape_a_ansgar() {
        let ulrich = MarchandMort::new(
            MarchandId::new("ulrich"),
            "Ulrich",
            Soin::Elixir,
            Gardien::MonstreDeLaZone,
            "",
            "",
        )
        .unwrap();
        // Parmi les étapes 1 et 3, la seconde : le tirage de 2 saute celle
        // d'Ansgar.
        let mut expedition = Expedition::partir(zone(false))
            .avec_ame(ulrich, RencontreDeLAme::ARetrouver)
            .avec_pecheur(ansgar());
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new([2, 81, 1, 100])),
            Some(EvenementExpedition::Coffre { .. })
        ));
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(MONSTRE)),
            Some(EvenementExpedition::Pecheur { .. })
        ));
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(MONSTRE)),
            Some(EvenementExpedition::Ame { .. })
        ));
    }
}
