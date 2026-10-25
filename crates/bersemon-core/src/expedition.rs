//! Expédition manuelle dans une zone (section 9.2) : une suite d'étapes,
//! chacune avec son événement.

use std::fmt;
use std::ops::RangeInclusive;

use crate::aleatoire::Aleatoire;
use crate::ame::MarchandMort;
use crate::attaque::AttaqueId;
use crate::booster::Booster;
use crate::capsule::Capsule;
use crate::expedition_automatique::ErreurParametres;
use crate::filet::Filet;
use crate::humain::Humain;
use crate::jour_et_nuit::{BONUS_DE_LA_NUIT, JourOuNuit};
use crate::monstre::Monstre;
use crate::peche::Pecheur;
use crate::pierre::PierreDEvolution;
use crate::tour::TourSansFond;
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
/// puis 5 chances par pierre. Le même tirage donne ensuite un booster, 1
/// chance par booster, de 21 à 26 (section 14.3, Q93).
const CHANCE_DE_CAPSULE_D_HABILETE: u32 = 10;
const CHANCE_PAR_PIERRE: u32 = 5;
const PREMIER_TIRAGE_DES_BOOSTERS: u32 = 21;
/// Le même tirage donne des filets : 2 filets de chanvre de 27 à 41, un
/// filet lesté de 42 à 46 (docs/contenu-v1.md, section 26).
const TIRAGES_DES_FILETS: [(std::ops::RangeInclusive<u32>, Filet, u32); 2] =
    [(27..=41, Filet::Chanvre, 2), (42..=46, Filet::Leste, 1)];

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
        booster: Option<Booster>,
        /// Des filets de capture, et leur nombre (section 11.2).
        filets: Option<(Filet, u32)>,
    },
    MonstreRare(MonstreRare),
    /// Combat, ni fuite ni filet ni capture, contre le gardien de la Tour
    /// sans fond, toutes les 10 étapes (section 9.5).
    Gardien(Monstre),
    /// Combat, sans fuite, contre l'Apôtre ; le Démiurge a deux formes, la
    /// seconde après la première (section 9.4).
    Apotre(Vec<Monstre>),
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
    /// Dans la Tour sans fond, de quoi recréer la zone de chaque étape
    /// (section 9.5). En boîte, pour que l'expédition reste légère.
    descente: Option<Box<Descente>>,
    /// Le moment de son départ, qu'elle garde jusqu'au retour ; `None` hors
    /// des zones de la carte, où le cycle ne joue pas (section 9.6).
    jour_ou_nuit: Option<JourOuNuit>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Descente {
    tour: TourSansFond,
    /// Celles des zones que le joueur a menées au bout.
    capsules: Vec<Capsule>,
    a_deux: bool,
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
            descente: None,
            jour_ou_nuit: None,
        }
    }

    /// Dans la Tour sans fond, des étapes sans fin, chacune avec sa zone
    /// (section 9.5). Le départ passe par `Partie::descendre_dans_la_tour`.
    pub(crate) fn descendre(tour: TourSansFond, capsules: Vec<Capsule>, a_deux: bool) -> Self {
        let zone = tour.zone_de_l_etape(1, a_deux, capsules.clone());
        Self {
            descente: Some(Box::new(Descente {
                tour,
                capsules,
                a_deux,
            })),
            nombre_etapes: u32::MAX,
            ..Self::partir(zone)
        }
    }

    /// La Tour sans fond, si l'expédition y descend.
    pub fn tour(&self) -> Option<&TourSansFond> {
        self.descente.as_ref().map(|descente| &descente.tour)
    }

    /// Les étapes de la Tour sans fond n'ont pas de fin (section 9.5).
    pub fn est_sans_fin(&self) -> bool {
        self.descente.is_some()
    }

    /// Les étapes dont l'événement est résolu sans défaite : celle d'une
    /// défaite n'est pas franchie (section 9.5).
    pub fn etapes_franchies(&self) -> u32 {
        self.etapes_faites.saturating_sub(u32::from(self.defaite))
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

    /// Le départ dans une zone de la carte fixe le moment de toute
    /// l'expédition (section 9.6).
    pub(crate) fn avec_jour_ou_nuit(mut self, jour_ou_nuit: JourOuNuit) -> Self {
        self.jour_ou_nuit = Some(jour_ou_nuit);
        self
    }

    /// `None` dans la Tour sans fond et dans l'Éclipse, hors du cycle.
    pub fn jour_ou_nuit(&self) -> Option<JourOuNuit> {
        self.jour_ou_nuit
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

    /// Celui de la zone, montré au joueur (section 9.2) ; sans fin dans la
    /// Tour sans fond (`est_sans_fin`).
    pub fn nombre_etapes(&self) -> u32 {
        self.nombre_etapes
    }

    /// Plus d'étape à faire : après la dernière, ou après une défaite.
    pub fn est_terminee(&self) -> bool {
        self.defaite || self.etapes_faites >= self.nombre_etapes
    }

    /// Menée jusqu'à sa dernière étape sans défaite : la zone compte alors
    /// comme explorée (sections 9.2 et 9.3). Jamais dans la Tour sans fond.
    pub fn est_reussie(&self) -> bool {
        !self.defaite && !self.est_sans_fin() && self.etapes_faites >= self.nombre_etapes
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
    /// d'habileté, sa pierre ou son booster ; pour une âme, ceux
    /// de son adversaire. L'âme attendue à cette étape remplace l'événement,
    /// sans tirage de sa nature.
    /// Une zone sans monstre rare donne un combat à la place. L'Apôtre
    /// remplace l'événement de la dernière étape, sans tirage de sa nature
    /// (Q57), comme l'humain de la zone, sans aucun tirage (section 10.7).
    /// Ansgar remplace celui de son étape, avant l'âme, avec les tirages de
    /// son adversaire (section 9.2). Dans la Tour sans fond, le gardien
    /// remplace celui des étapes multiples de 10, avec ses seuls tirages
    /// (section 9.5).
    pub fn etape_suivante(&mut self, aleatoire: &mut dyn Aleatoire) -> Option<EvenementExpedition> {
        if self.est_terminee() {
            return None;
        }
        if self.etapes_faites == 0 {
            self.placer_l_ame_a_retrouver(aleatoire);
        }
        self.etapes_faites += 1;
        // Dans la Tour, la zone suit la profondeur ; le gardien remplace
        // l'événement, sans tirage de sa nature (section 9.5).
        if let Some(descente) = &self.descente {
            let etape = self.etapes_faites;
            self.zone =
                descente
                    .tour
                    .zone_de_l_etape(etape, descente.a_deux, descente.capsules.clone());
            if TourSansFond::a_un_gardien(etape) {
                return Some(EvenementExpedition::Gardien(
                    descente.tour.generer_gardien(etape, aleatoire),
                ));
            }
        }
        let derniere = self.etapes_faites == self.nombre_etapes;
        if let Some(apotre) = self.apotre.as_ref().filter(|_| derniere) {
            return Some(EvenementExpedition::Apotre(
                apotre.generer_ses_formes(aleatoire),
            ));
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
            1..=80 => {
                EvenementExpedition::Combat(self.zone.generer_groupe(self.jour_ou_nuit, aleatoire))
            }
            81..=95 => self.tirer_un_coffre(aleatoire),
            _ => match self.zone.generer_monstre_rare(self.jour_ou_nuit, aleatoire) {
                Some(rare) => EvenementExpedition::MonstreRare(rare),
                None => EvenementExpedition::Combat(
                    self.zone.generer_groupe(self.jour_ou_nuit, aleatoire),
                ),
            },
        };
        Some(evenement)
    }

    /// En groupe, l'expédition de l'invité suit celle de l'hôte, qui tire
    /// les événements : elle passe à l'étape suivante sans tirage, avec la
    /// zone de cette étape (section 15.5).
    pub(crate) fn suivre_l_etape(&mut self) {
        self.etapes_faites += 1;
        if let Some(descente) = &self.descente {
            self.zone = descente.tour.zone_de_l_etape(
                self.etapes_faites,
                descente.a_deux,
                descente.capsules.clone(),
            );
        }
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

    /// En groupe, chacun tire aussi le sien quand l'étape de l'hôte donne
    /// un coffre (section 15.5).
    pub(crate) fn tirer_un_coffre(&mut self, aleatoire: &mut dyn Aleatoire) -> EvenementExpedition {
        let potions = aleatoire.tirer(POTIONS_MIN..=POTIONS_MAX);
        let fourchette = self.zone.coffre().argent();
        let mut argent = tirer_entre(*fourchette.start(), *fourchette.end(), aleatoire);
        // L'argent de la nuit, avant celui de l'habileté Pillard (section
        // 9.6).
        if self.jour_ou_nuit == Some(JourOuNuit::Nuit) {
            argent = argent * (100 + BONUS_DE_LA_NUIT) / 100;
        }
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
        let booster = tirage
            .checked_sub(PREMIER_TIRAGE_DES_BOOSTERS)
            .and_then(|rang| Booster::TOUS.get(rang as usize).copied());
        let filets = TIRAGES_DES_FILETS
            .iter()
            .find(|(tirages, _, _)| tirages.contains(&tirage))
            .map(|(_, filet, nombre)| (*filet, *nombre));
        EvenementExpedition::Coffre {
            potions,
            argent,
            capsule,
            capsule_d_habilete,
            pierre,
            booster,
            filets,
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
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 2, 47])),
            Some(EvenementExpedition::Coffre {
                potions: 2,
                argent: 10,
                capsule: None,
                capsule_d_habilete: false,
                pierre: None,
                booster: None,
                filets: None
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

    /// De nuit, l'argent des coffres augmente de 25 %, et les monstres ont
    /// 2 niveaux de plus (section 9.6).
    #[test]
    fn la_nuit_les_coffres_rapportent_plus_et_les_monstres_sont_plus_forts() {
        let mut de_nuit = Expedition::partir(zone(true)).avec_jour_ou_nuit(JourOuNuit::Nuit);
        assert_eq!(de_nuit.jour_ou_nuit(), Some(JourOuNuit::Nuit));
        assert!(matches!(
            de_nuit.etape_suivante(&mut AleatoireScripte::new([81, 2, 47])),
            Some(EvenementExpedition::Coffre { argent: 12, .. })
        ));
        let mut combat = vec![80];
        combat.extend(MONSTRE);
        assert!(matches!(
            de_nuit.etape_suivante(&mut AleatoireScripte::new(combat)),
            Some(EvenementExpedition::Combat(groupe))
                if groupe[0].niveau() == Niveau::new(7).unwrap()
        ));
        let mut de_jour = Expedition::partir(zone(true)).avec_jour_ou_nuit(JourOuNuit::Jour);
        assert!(matches!(
            de_jour.etape_suivante(&mut AleatoireScripte::new([81, 2, 47])),
            Some(EvenementExpedition::Coffre { argent: 10, .. })
        ));
        assert_eq!(Expedition::partir(zone(true)).jour_ou_nuit(), None);
    }

    /// Section 9.5 : des étapes sans fin, le gardien à l'étape 10, et
    /// l'étape d'une défaite qui n'est pas franchie.
    #[test]
    fn dans_la_tour_le_gardien_attend_a_l_etape_10() {
        let tour = TourSansFond::new(
            crate::zone::ZoneId::new("tour"),
            "Tour",
            &[zone(true)],
            EspeceRare {
                espece: espece("schnoz"),
                pacifique: true,
            },
            "",
            "",
            "",
        )
        .unwrap();
        let mut expedition = Expedition::descendre(tour, Vec::new(), false);
        assert!(expedition.est_sans_fin());
        assert_eq!(expedition.zone().id().as_str(), "tour");
        for etape in 1..10 {
            // Coffre : potions, argent (de 5 à 8 fois l'étape), puis le
            // tirage des capsules d'habileté, pierres et boosters.
            assert!(matches!(
                expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 5 * etape, 100])),
                Some(EvenementExpedition::Coffre { argent, .. }) if argent == 5 * etape
            ));
        }
        // Sans rare hostile, une espèce commune : seuls son shiny et ses
        // valeurs internes sont tirés.
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new(MONSTRE)),
            Some(EvenementExpedition::Gardien(gardien))
                if gardien.espece().id().as_str() == "loup" && gardien.niveau().valeur() == 27
        ));
        assert!(!expedition.est_terminee());
        assert_eq!(expedition.etapes_franchies(), 10);
        expedition.noter_defaite();
        assert!(expedition.est_terminee() && !expedition.est_reussie());
        assert_eq!(expedition.etapes_franchies(), 9);
    }

    /// Docs/contenu-v1.md, section 26 : le tirage du coffre donne 2 filets
    /// de chanvre de 27 à 41, un filet lesté de 42 à 46.
    #[test]
    fn un_coffre_sur_5_contient_des_filets() {
        let filets = |tirage: u32| {
            let mut expedition = Expedition::partir(zone(false));
            match expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, tirage])) {
                Some(EvenementExpedition::Coffre { filets, .. }) => filets,
                autre => panic!("{autre:?}"),
            }
        };
        assert_eq!(filets(26), None);
        assert_eq!(filets(27), Some((Filet::Chanvre, 2)));
        assert_eq!(filets(41), Some((Filet::Chanvre, 2)));
        assert_eq!(filets(42), Some((Filet::Leste, 1)));
        assert_eq!(filets(46), Some((Filet::Leste, 1)));
        assert_eq!(filets(47), None);
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
            Some(EvenementExpedition::Apotre(formes))
                if formes.len() == 1 && formes[0].niveau() == Niveau::new(7).unwrap()
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
                pierre: None,
                booster: None,
                filets: None
            })
        );
        // Le même tirage donne une pierre, de 11 à 20.
        for (tirage, pierre) in [
            (11, Some(PierreDEvolution::Braise)),
            (15, Some(PierreDEvolution::Braise)),
            (16, Some(PierreDEvolution::Lune)),
            (20, Some(PierreDEvolution::Lune)),
            (21, None),
            (100, None),
        ] {
            let mut expedition = Expedition::partir(zone.clone());
            assert!(matches!(
                expedition.etape_suivante(&mut AleatoireScripte::new([90, 1, 25, tirage])),
                Some(EvenementExpedition::Coffre { capsule_d_habilete: false, pierre: trouvee, .. })
                    if trouvee == pierre
            ));
        }
        // Puis un booster, de 21 à 26, un par booster (section 14.3).
        for (tirage, booster) in [
            (20, None),
            (21, Some(Booster::FioleDeSang)),
            (22, Some(Booster::MoelleDeBete)),
            (26, Some(Booster::PoudreDeVifArgent)),
            (27, None),
        ] {
            let mut expedition = Expedition::partir(zone.clone());
            assert!(matches!(
                expedition.etape_suivante(&mut AleatoireScripte::new([90, 1, 25, tirage])),
                Some(EvenementExpedition::Coffre { capsule_d_habilete: false, booster: trouve, .. })
                    if trouve == booster
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
