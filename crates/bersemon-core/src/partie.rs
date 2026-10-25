//! Partie en cours : le héros et tout ce que le joueur possède ou sait. C'est
//! ce qu'une sauvegarde devra conserver.

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::ame::{MarchandId, MarchandMort};
use crate::amelioration::{
    Chantier, ErreurAmelioration, Forge, Ouvrage, PlanDAmelioration, PlanDeFabrication,
    PlanDesFilets, PlanDuCharme,
};
use crate::attaque::{Attaque, AttaqueId, ErreurAttaquesMonstre};
use crate::beherit::{
    ErreurBeherit, EtatDuBeherit, Invocation, NIVEAU_MINIMAL_DU_SACRIFICE, PHILTRES_DU_DEMIURGE,
};
use crate::booster::{Booster, ErreurBooster};
use crate::capsule::{Capsule, ErreurApprentissage, attaques_a_apprendre};
use crate::catalogue::Catalogue;
use crate::codex::Codex;
use crate::collection::{Collection, Emplacement, ErreurCollection, Placement};
use crate::combat::{BilanCombat, Combat, Cote, ErreurCombat, Issue, ReglesDuMembre};
use crate::equipement::{ErreurEquipement, ObjetDEquipementId, PlaceDEquipement};
use crate::espece::{Espece, EspeceId, Rarete};
use crate::evolution::{
    Circonstances, ErreurEvolution, ResultatEvolution, evolution_possible, faire_evoluer,
};
use crate::expedition::{ErreurExpedition, Expedition, RencontreDeLAme};
use crate::expedition_automatique::{
    BilanExpeditionAutomatique, ErreurExpeditionAutomatique, Estimation, ExpeditionAutomatique,
    MONSTRES_MAX, MONSTRES_MIN, Protection, verifier_protection,
};
use crate::filet::{Filet, Filets};
use crate::habilete::{
    BONUS_DU_PILLARD, CHANCES_DU_FOUINEUR, ErreurChangementDHabilete, Habilete, autre_habilete,
};
use crate::heros::Heros;
use crate::horodatage::Horodatage;
use crate::humain::{Humain, HumainId, IaHumain};
use crate::inventaire::{ErreurObjet, Inventaire, Objet};
use crate::machines::{ErreurMachine, Machines, reste_avant_le_plafond};
use crate::marchand::{
    EXPEDITIONS_DE_LOCATION, ErreurMarchand, OffreDeMonstre, PRIX_CAPSULE_D_HABILETE,
    PRIX_PIERRE_D_EVOLUTION, prix_de_revente, prix_du_filet, prix_du_soin,
};
use crate::monstre::{Monstre, RapportExperience, tirer_shiny, tirer_valeurs_internes};
use crate::niveau::Niveau;
use crate::nom::Nom;
use crate::obeissance::limite_d_obeissance;
use crate::peche::{ErreurPeche, ParametresPeche, Peche, Prise};
use crate::pierre::PierreDEvolution;
use crate::relations::Relations;
use crate::ressource::{Ressource, RessourceId};
use crate::soin::{EffetDuSoin, ErreurSoin, Soin};
use crate::stats::ValeursInternes;
use crate::tour::{RecordsDeLaTour, TourSansFond};
use crate::traque::{Apotre, Indice, Piste};
use crate::tutoriel::Conseil;
use crate::zone::{Zone, ZoneId};

/// Un monstre sauvage battu lâche sa ressource 1 fois sur 3 (section 14.3,
/// Q82).
const CHANCES_DE_RESSOURCE: u32 = 3;

/// Le monstre de départ est au niveau 5 (section 6).
const NIVEAU_DE_DEPART: u8 = 5;

/// 1 chance sur 512 d'être shiny, valeurs internes et habileté tirées au
/// hasard, comme un monstre sauvage (sections 5.9 et 6). Tirages : shiny,
/// valeurs internes s'il n'est pas shiny, puis habileté.
/// Échoue si l'espèce n'apprend aucune attaque jusqu'au niveau 5.
pub fn creer_monstre_de_depart(
    espece: Espece,
    aleatoire: &mut dyn Aleatoire,
) -> Result<Monstre, ErreurAttaquesMonstre> {
    let shiny = tirer_shiny(aleatoire);
    let valeurs_internes = tirer_valeurs_internes(shiny, aleatoire);
    let niveau = Niveau::new(NIVEAU_DE_DEPART).expect("niveau de départ valide");
    Monstre::creer(espece, niveau, valeurs_internes, shiny)
        .map(|monstre| monstre.tirer_son_habilete(aleatoire))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Partie {
    heros: Heros,
    collection: Collection,
    /// Ce que le joueur emporte en expédition manuelle, et qu'une défaite
    /// lui fait perdre (section 9.3).
    sac: Inventaire,
    /// À l'abri : ce qui est gagné au camp y va (section 13.3).
    coffre_du_camp: Inventaire,
    /// Malvoise, le nécromancien, tient l'étal du camp une fois arrivé
    /// (section 14.2).
    marchand_au_camp: bool,
    /// Chaque âme donnée ajoute à son étal l'objet de son marchand.
    ames_donnees: Vec<MarchandMort>,
    /// Recueillies et pas encore données : à part du sac, aucune défaite ne
    /// les fait perdre (section 14.2).
    ames_portees: Vec<MarchandMort>,
    /// Après une fuite ou une défaite face à leur gardien : la prochaine
    /// expédition dans leur zone les rencontre à coup sûr.
    ames_a_retrouver: Vec<MarchandId>,
    codex: Codex,
    zones_explorees: Vec<ZoneId>,
    /// Indices trouvés sur la piste de l'Apôtre (section 9.4).
    indices_trouves: usize,
    apotres_vaincus: Vec<EspeceId>,
    /// Un humain vaincu ne revient plus (section 10.7).
    humains_vaincus: Vec<HumainId>,
    conseils_vus: Vec<Conseil>,
    /// Une au plus par zone, dans l'ordre des départs (section 12.1).
    expeditions_automatiques: Vec<ExpeditionAutomatique>,
    /// Dans l'ordre où le joueur les choisira. Tant qu'il en reste, le
    /// déroulement n'accepte aucune autre action au camp : les positions
    /// qu'elles désignent ne bougent donc pas.
    attaques_a_choisir: Vec<AttaqueAChoisir>,
    /// Gunther et les améliorations du camp (section 13.4).
    forge: Forge,
    /// Ansgar et sa ligne (section 13.8).
    peche: Peche,
    /// Amis et palmarès des duels, sur le bot Discord (section 15.4).
    relations: Relations,
    /// Les pièges et la cour d'entraînement (section 13.6).
    machines: Machines,
    /// Le Béhérit pourpre et l'Éclipse (section 9.4).
    beherit: EtatDuBeherit,
    /// Dernières étapes franchies dans la Tour sans fond (section 9.5).
    records_de_la_tour: RecordsDeLaTour,
}

/// Attaque qu'un monstre rentré d'expédition automatique veut apprendre en
/// plus des 4 qu'il connaît : le joueur lui en fait oublier une ou y renonce
/// (sections 7 et 12.7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttaqueAChoisir {
    /// Là où le monstre est allé à son retour (section 12.7).
    pub emplacement: Emplacement,
    pub attaque: Attaque,
}

/// Un monstre repris de la cour d'entraînement (section 13.6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepriseDEntrainement {
    /// Copie prise à la reprise, XP comprise.
    pub monstre: Monstre,
    pub rapport: RapportExperience,
    /// Dans l'équipe s'il y avait de la place, sinon au camp.
    pub emplacement: Emplacement,
}

/// Tout ce qu'une sauvegarde conserve d'une partie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtatPartie {
    pub heros: Heros,
    pub collection: Collection,
    pub sac: Inventaire,
    pub coffre_du_camp: Inventaire,
    pub marchand_au_camp: bool,
    pub ames_donnees: Vec<MarchandMort>,
    pub ames_portees: Vec<MarchandMort>,
    pub ames_a_retrouver: Vec<MarchandId>,
    pub codex: Codex,
    pub zones_explorees: Vec<ZoneId>,
    pub indices_trouves: usize,
    pub apotres_vaincus: Vec<EspeceId>,
    pub humains_vaincus: Vec<HumainId>,
    pub conseils_vus: Vec<Conseil>,
    pub expeditions_automatiques: Vec<ExpeditionAutomatique>,
    pub attaques_a_choisir: Vec<AttaqueAChoisir>,
    pub forge: Forge,
    pub peche: Peche,
    pub relations: Relations,
    pub machines: Machines,
    pub beherit: EtatDuBeherit,
    pub records_de_la_tour: RecordsDeLaTour,
}

/// Sauvegarde lisible mais contraire aux règles des expéditions
/// automatiques (section 12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurRestaurationPartie {
    /// Le camp n'a pas la place de garder celle des monstres partis.
    Collection(ErreurCollection),
    ZoneNonExploree(ZoneId),
    ExpeditionEnDouble(ZoneId),
    /// Aucun monstre à cet emplacement.
    AttaqueAChoisirSansMonstre(Emplacement),
    /// La même capsule dans le sac et dans le coffre.
    CapsuleEnDouble(AttaqueId),
    /// Ni monstre au camp, ni monstre en expédition automatique.
    AucunMonstre,
    /// Un monstre pêché attend que le joueur le garde ou le relâche, mais
    /// l'équipe et le camp sont pleins.
    PriseSansPlace,
    /// Des pièges posés sans être construits ou dans une zone inexplorée,
    /// ou plus de monstres à l'entraînement que de places dans la cour.
    MachinesIncoherentes,
    /// Plus d'un monstre loué, ou un monstre loué pour seul monstre (section
    /// 14.2).
    LocationIncoherente,
}

impl fmt::Display for ErreurRestaurationPartie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurRestaurationPartie::Collection(erreur) => write!(f, "{erreur}"),
            ErreurRestaurationPartie::ZoneNonExploree(zone) => write!(
                f,
                "expédition automatique dans une zone non explorée : {}",
                zone.as_str()
            ),
            ErreurRestaurationPartie::ExpeditionEnDouble(zone) => write!(
                f,
                "deux expéditions automatiques dans la zone {}",
                zone.as_str()
            ),
            ErreurRestaurationPartie::AttaqueAChoisirSansMonstre(emplacement) => write!(
                f,
                "attaque à choisir pour un monstre absent ({emplacement:?})"
            ),
            ErreurRestaurationPartie::CapsuleEnDouble(capsule) => {
                write!(f, "capsule obtenue deux fois : {}", capsule.as_str())
            }
            ErreurRestaurationPartie::AucunMonstre => write!(f, "aucun monstre"),
            ErreurRestaurationPartie::PriseSansPlace => {
                write!(f, "monstre pêché sans place dans l'équipe ni au camp")
            }
            ErreurRestaurationPartie::MachinesIncoherentes => {
                write!(f, "machines du camp incohérentes")
            }
            ErreurRestaurationPartie::LocationIncoherente => {
                write!(f, "location de monstre incohérente")
            }
        }
    }
}

impl std::error::Error for ErreurRestaurationPartie {}

/// Les filets de chanvre du héros au départ (section 11.2).
const FILETS_DE_DEPART: u32 = 10;

impl Partie {
    /// Le monstre de départ compte comme capturé dans le codex (Q54).
    /// Le héros part avec 10 filets de chanvre dans le sac (section 11.2).
    pub fn nouvelle(heros: Heros, depart: Monstre) -> Self {
        let mut codex = Codex::default();
        codex.noter_capture(depart.espece().id());
        Self {
            heros,
            collection: Collection::new(depart),
            sac: Inventaire::default().avec_filets(Filet::Chanvre, FILETS_DE_DEPART),
            coffre_du_camp: Inventaire::default(),
            marchand_au_camp: false,
            ames_donnees: Vec::new(),
            ames_portees: Vec::new(),
            ames_a_retrouver: Vec::new(),
            codex,
            zones_explorees: Vec::new(),
            indices_trouves: 0,
            apotres_vaincus: Vec::new(),
            humains_vaincus: Vec::new(),
            forge: Forge::default(),
            peche: Peche::default(),
            relations: Relations::default(),
            machines: Machines::default(),
            beherit: EtatDuBeherit::Aucun,
            records_de_la_tour: RecordsDeLaTour::default(),
            conseils_vus: Vec::new(),
            expeditions_automatiques: Vec::new(),
            attaques_a_choisir: Vec::new(),
        }
    }

    pub fn heros(&self) -> &Heros {
        &self.heros
    }

    pub fn collection(&self) -> &Collection {
        &self.collection
    }

    /// Gestion des monstres au camp (section 13.2).
    pub fn collection_mut(&mut self) -> &mut Collection {
        &mut self.collection
    }

    pub fn sac(&self) -> &Inventaire {
        &self.sac
    }

    pub fn coffre_du_camp(&self) -> &Inventaire {
        &self.coffre_du_camp
    }

    /// Au camp, l'argent du sac et celui du coffre paient ensemble (section
    /// 13.3).
    pub fn argent(&self) -> u32 {
        self.sac
            .argent()
            .saturating_add(self.coffre_du_camp.argent())
    }

    /// Celles du sac, puis celles du coffre : au camp, elles servent toutes
    /// (section 13.3).
    pub fn capsules(&self) -> impl Iterator<Item = &Capsule> {
        self.sac
            .capsules()
            .iter()
            .chain(self.coffre_du_camp.capsules())
    }

    pub fn possede_capsule(&self, id: &AttaqueId) -> bool {
        self.sac.contient_capsule(id) || self.coffre_du_camp.contient_capsule(id)
    }

    /// Du sac vers le coffre, au camp seulement (section 13.3).
    pub fn ranger_au_coffre(&mut self, objet: &Objet) -> Result<Inventaire, ErreurObjet> {
        let range = self.sac.retirer(objet)?;
        self.coffre_du_camp.ajouter(range.clone());
        Ok(range)
    }

    /// Du coffre vers le sac, au camp seulement (section 13.3).
    pub fn sortir_du_coffre(&mut self, objet: &Objet) -> Result<Inventaire, ErreurObjet> {
        let sorti = self.coffre_du_camp.retirer(objet)?;
        self.sac.ajouter(sorti.clone());
        Ok(sorti)
    }

    /// Tout le sac rejoint le coffre. Rend ce qui a été rangé.
    pub fn tout_ranger_au_coffre(&mut self) -> Inventaire {
        let range = self.sac.perdre_tout();
        self.coffre_du_camp.ajouter(range.clone());
        range
    }

    pub fn marchand_au_camp(&self) -> bool {
        self.marchand_au_camp
    }

    /// Malvoise, rencontré au bout de la première expédition menée au bout,
    /// rentre au camp avec le joueur et y reste (section 14.2).
    pub(crate) fn accueillir_malvoise(&mut self) {
        self.marchand_au_camp = true;
    }

    pub fn forge(&self) -> &Forge {
        &self.forge
    }

    /// Libéré à la défaite du Bourreau, Gunther rentre au camp avec le
    /// joueur et y reste (section 13.4).
    pub(crate) fn accueillir_gunther(&mut self) {
        self.forge.accueillir_gunther();
    }

    pub fn peche(&self) -> &Peche {
        &self.peche
    }

    pub fn machines(&self) -> &Machines {
        &self.machines
    }

    /// Le niveau jusqu'auquel la cour entraîne : le niveau maximal de la
    /// zone ouverte la plus avancée (section 13.6).
    pub fn plafond_d_entrainement(&self, zones: &[Zone], piste: &Piste) -> Niveau {
        let premier = Niveau::new(Niveau::MIN).expect("le niveau minimal est valide");
        self.zone_la_plus_avancee(zones, piste)
            .map_or(premier, |zone| zone.niveaux().1)
    }

    /// Au camp : les pièges se posent dans une zone explorée. Déjà posés
    /// ailleurs, ils doivent d'abord être relevés (section 13.6).
    pub fn poser_les_pieges(
        &mut self,
        zone: &Zone,
        maintenant: Horodatage,
    ) -> Result<(), ErreurMachine> {
        let regles = self
            .forge
            .ameliorations()
            .pieges()
            .ok_or(ErreurMachine::NonConstruite)?;
        if !self.est_exploree(zone.id()) {
            return Err(ErreurMachine::ZoneNonExploree);
        }
        if let Some(pieges) = self.machines.pieges() {
            if pieges.zone() == zone.id() {
                return Err(ErreurMachine::DejaPosesIci);
            }
            if pieges.stock(&regles, maintenant) > 0 {
                return Err(ErreurMachine::PiegesARelever);
            }
        }
        self.machines.poser(zone.id().clone(), maintenant);
        Ok(())
    }

    /// Au camp : les ressources prises par les pièges, posés dans `zone`,
    /// vont dans le coffre (section 13.6). Les tirages sont ceux de
    /// `Machines::relever`.
    pub fn relever_les_pieges(
        &mut self,
        zone: &Zone,
        maintenant: Horodatage,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<Vec<(Ressource, u32)>, ErreurMachine> {
        let regles = self
            .forge
            .ameliorations()
            .pieges()
            .ok_or(ErreurMachine::NonConstruite)?;
        let prises = self
            .machines
            .relever(&regles, zone, maintenant, aleatoire)?;
        for (ressource, nombre) in &prises {
            self.coffre_du_camp
                .ajouter_ressources(ressource.clone(), *nombre);
        }
        Ok(prises)
    }

    /// Au camp : le monstre quitte l'équipe ou le camp pour la cour, comme
    /// pour une expédition automatique, et garde sa place au camp (sections
    /// 12.1 et 13.6). Refusé s'il est déjà à son plafond.
    pub fn mettre_a_l_entrainement(
        &mut self,
        emplacement: Emplacement,
        plafond: Niveau,
        maintenant: Horodatage,
    ) -> Result<(), ErreurMachine> {
        let regles = self
            .forge
            .ameliorations()
            .cour()
            .ok_or(ErreurMachine::NonConstruite)?;
        if self.machines.cour().len() >= regles.places {
            return Err(ErreurMachine::CourPleine);
        }
        let monstre = self
            .collection
            .monstre(emplacement)
            .ok_or(ErreurMachine::Collection(
                ErreurCollection::EmplacementInexistant(emplacement),
            ))?;
        if reste_avant_le_plafond(monstre, plafond) == 0 {
            return Err(ErreurMachine::PlafondAtteint);
        }
        let monstre = self
            .collection
            .retirer_pour_expedition(&[emplacement])
            .map_err(ErreurMachine::Collection)?
            .remove(0);
        self.machines.entrer(monstre, maintenant);
        Ok(())
    }

    /// Au camp : le monstre en `position` dans la cour reçoit l'XP gagnée,
    /// puis rejoint l'équipe s'il y reste une place, sinon le camp, comme au
    /// retour d'une expédition automatique ; ses attaques en attente
    /// s'ajoutent aux attaques à choisir (sections 12.7 et 13.6).
    pub fn reprendre_de_l_entrainement(
        &mut self,
        position: usize,
        plafond: Niveau,
        maintenant: Horodatage,
    ) -> Result<RepriseDEntrainement, ErreurMachine> {
        let regles = self
            .forge
            .ameliorations()
            .cour()
            .ok_or(ErreurMachine::NonConstruite)?;
        let entrainement = self.machines.sortir(position)?;
        let gain = entrainement.experience(&regles, plafond, maintenant);
        let mut monstre = entrainement.monstre().clone();
        let rapport = monstre.gagner_experience(gain);
        let emplacement = self.collection.accueillir_retour(vec![monstre.clone()])[0];
        for attaque in &rapport.attaques_en_attente {
            self.attaques_a_choisir.push(AttaqueAChoisir {
                emplacement,
                attaque: attaque.clone(),
            });
        }
        Ok(RepriseDEntrainement {
            monstre,
            rapport,
            emplacement,
        })
    }

    pub fn relations(&self) -> &Relations {
        &self.relations
    }

    /// Les amis et le palmarès se tiennent depuis le bot Discord (section
    /// 15.4).
    pub fn relations_mut(&mut self) -> &mut Relations {
        &mut self.relations
    }

    /// Sauvé dans le Marais, Ansgar rentre au camp avec le joueur et y reste
    /// (section 13.8).
    pub(crate) fn accueillir_ansgar(&mut self) {
        self.peche.accueillir_ansgar();
    }

    /// Gratuite et sans confirmation, une ligne à la fois. Rend l'heure où
    /// elle sera prête (section 13.8).
    pub fn lancer_la_ligne(
        &mut self,
        parametres: &ParametresPeche,
        maintenant: Horodatage,
    ) -> Result<Horodatage, ErreurPeche> {
        self.peche.lancer(parametres.duree(), maintenant)
    }

    /// Relève la ligne prête, si un monstre pêché trouverait sa place dans
    /// l'équipe ou au camp, et rend la prise : un monstre, rencontré au
    /// codex, attend que le joueur le garde ou le relâche ; un objet va dans
    /// le coffre (section 13.8). `zone` est la plus avancée que le joueur a
    /// ouverte. Tirages : ceux de `ParametresPeche::tirer_prise`, seulement
    /// si la ligne se relève.
    pub fn relever_la_ligne(
        &mut self,
        parametres: &ParametresPeche,
        zone: &Zone,
        maintenant: Horodatage,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<Prise, ErreurPeche> {
        self.peche.verifier_releve(maintenant)?;
        if !self.collection.a_de_la_place() {
            return Err(ErreurPeche::PlaceManquante);
        }
        let prise = parametres.tirer_prise(zone, aleatoire);
        self.peche.relever();
        match &prise {
            Prise::Monstre(monstre) => {
                self.codex.noter_rencontre(monstre.espece().id());
                self.peche.garder_en_attente(Monstre::clone(monstre));
            }
            Prise::Potion => self.coffre_du_camp.ajouter_potions(1),
            Prise::Argent(argent) => self.coffre_du_camp.ajouter_argent(*argent),
            Prise::Pierre(pierre) => self.coffre_du_camp.ajouter_pierres(*pierre, 1),
            Prise::CapsuleDHabilete => self.coffre_du_camp.ajouter_capsules_d_habilete(1),
            Prise::Booster(booster) => self.coffre_du_camp.ajouter_boosters(*booster, 1),
            Prise::PhiltreChromatique => self.coffre_du_camp.ajouter_philtres_chromatiques(1),
        }
        Ok(prise)
    }

    /// Le monstre pêché compte comme capturé et rejoint l'équipe s'il y a de
    /// la place, sinon le camp ; la place a été vérifiée en relevant la
    /// ligne. `None` sans monstre pêché.
    pub fn garder_la_prise(&mut self) -> Option<Placement> {
        let prise = self.peche.prendre_la_prise()?;
        self.codex.noter_capture(prise.espece().id());
        Some(self.collection.accueillir_capture(prise))
    }

    /// `None` sans monstre pêché.
    pub fn relacher_la_prise(&mut self) -> Option<Monstre> {
        self.peche.prendre_la_prise()
    }

    /// La zone ouverte de plus haut niveau, où la pêche prend ses monstres
    /// et son argent (section 13.8).
    pub fn zone_la_plus_avancee<'a>(&self, zones: &'a [Zone], piste: &Piste) -> Option<&'a Zone> {
        self.zones_accessibles(zones, piste)
            .into_iter()
            .max_by_key(|zone| zone.niveaux().1)
    }

    /// Ce qu'il manque, du sac et du coffre réunis, pour payer ce coût.
    pub fn ressources_manquantes(&self, cout: &[(Ressource, u32)]) -> Vec<(Ressource, u32)> {
        cout.iter()
            .filter_map(|(ressource, nombre)| {
                let presentes = self
                    .sac
                    .nombre_de_ressources(ressource.id())
                    .saturating_add(self.coffre_du_camp.nombre_de_ressources(ressource.id()));
                (presentes < *nombre).then(|| (ressource.clone(), nombre - presentes))
            })
            .collect()
    }

    /// Au camp, après confirmation : les ressources sont prises au
    /// lancement, du sac d'abord, puis du coffre, et Gunther se met au
    /// travail (section 13.4).
    pub fn lancer_amelioration(
        &mut self,
        plan: &PlanDAmelioration,
        maintenant: Horodatage,
    ) -> Result<Chantier, ErreurAmelioration> {
        self.verifier_amelioration(plan)?;
        self.payer_ressources(plan.cout());
        Ok(self.forge.lancer(plan, maintenant))
    }

    /// Du sac d'abord, puis du coffre, comme l'argent (section 13.3). Le
    /// coût est vérifié par l'appelant.
    fn payer_ressources(&mut self, cout: &[(Ressource, u32)]) {
        for (ressource, nombre) in cout {
            let du_sac = (*nombre).min(self.sac.nombre_de_ressources(ressource.id()));
            for (inventaire, quantite) in [
                (&mut self.sac, du_sac),
                (&mut self.coffre_du_camp, nombre - du_sac),
            ] {
                if quantite > 0 {
                    inventaire
                        .retirer(&Objet::Ressources(ressource.id().clone(), quantite))
                        .expect("ressources vérifiées");
                }
            }
        }
    }

    /// Refuse ce que refuserait `lancer_amelioration`, sans rien changer.
    pub fn verifier_amelioration(
        &self,
        plan: &PlanDAmelioration,
    ) -> Result<(), ErreurAmelioration> {
        self.forge.verifier_lancement(plan)?;
        let manquantes = self.ressources_manquantes(plan.cout());
        if !manquantes.is_empty() {
            return Err(ErreurAmelioration::RessourcesManquantes(manquantes));
        }
        Ok(())
    }

    /// Au camp, après confirmation, comme une amélioration : Gunther ne fait
    /// qu'un travail à la fois (section 13.5).
    pub fn lancer_fabrication(
        &mut self,
        plan: &PlanDeFabrication,
        maintenant: Horodatage,
    ) -> Result<Chantier, ErreurAmelioration> {
        self.verifier_fabrication(plan)?;
        self.payer_ressources(plan.cout());
        Ok(self.forge.lancer_fabrication(plan, maintenant))
    }

    /// Au camp, après confirmation : un charme de garde, aussi souvent que
    /// le joueur le veut, un travail à la fois (section 13.5).
    pub fn lancer_charme(
        &mut self,
        plan: &PlanDuCharme,
        maintenant: Horodatage,
    ) -> Result<Chantier, ErreurAmelioration> {
        self.verifier_charme(plan)?;
        self.payer_ressources(plan.cout());
        Ok(self.forge.lancer_charme(plan, maintenant))
    }

    /// Refuse ce que refuserait `lancer_charme`, sans rien changer.
    pub fn verifier_charme(&self, plan: &PlanDuCharme) -> Result<(), ErreurAmelioration> {
        self.forge.verifier_charme()?;
        let manquantes = self.ressources_manquantes(plan.cout());
        if !manquantes.is_empty() {
            return Err(ErreurAmelioration::RessourcesManquantes(manquantes));
        }
        Ok(())
    }

    /// Au camp, après confirmation : un lot de filets, aussi souvent que le
    /// joueur le veut, un travail à la fois (section 13.5).
    pub fn lancer_filets(
        &mut self,
        plan: &PlanDesFilets,
        maintenant: Horodatage,
    ) -> Result<Chantier, ErreurAmelioration> {
        self.verifier_filets(plan)?;
        self.payer_ressources(plan.cout());
        Ok(self.forge.lancer_filets(plan, maintenant))
    }

    /// Refuse ce que refuserait `lancer_filets`, sans rien changer.
    pub fn verifier_filets(&self, plan: &PlanDesFilets) -> Result<(), ErreurAmelioration> {
        self.forge.verifier_filets()?;
        let manquantes = self.ressources_manquantes(plan.cout());
        if !manquantes.is_empty() {
            return Err(ErreurAmelioration::RessourcesManquantes(manquantes));
        }
        Ok(())
    }

    /// Ceux du sac et du coffre, de chaque qualité.
    pub fn filets(&self) -> Filets {
        let mut filets = *self.sac.filets();
        for (filet, nombre) in self.coffre_du_camp.filets().presents() {
            filets.ajouter(filet, nombre);
        }
        filets
    }

    /// Ceux du sac et du coffre : au camp, ils servent tous (section 12.5).
    pub fn charmes_de_garde(&self) -> u32 {
        self.sac
            .charmes_de_garde()
            .saturating_add(self.coffre_du_camp.charmes_de_garde())
    }

    /// Refuse ce que refuserait `lancer_fabrication`, sans rien changer.
    pub fn verifier_fabrication(&self, plan: &PlanDeFabrication) -> Result<(), ErreurAmelioration> {
        self.forge
            .verifier_fabrication(plan, self.heros.equipement())?;
        let manquantes = self.ressources_manquantes(plan.cout());
        if !manquantes.is_empty() {
            return Err(ErreurAmelioration::RessourcesManquantes(manquantes));
        }
        Ok(())
    }

    /// Au camp, le joueur constate le travail fini : le niveau compte, et un
    /// enclos agrandi accueille aussitôt plus de monstres (section 13.4) ;
    /// un objet fabriqué rejoint l'équipement du héros, sans être porté
    /// (section 13.5).
    pub fn constater_travail(
        &mut self,
        maintenant: Horodatage,
    ) -> Result<Chantier, ErreurAmelioration> {
        let chantier = self.forge.achever(maintenant)?;
        match &chantier.ouvrage {
            Ouvrage::Amelioration { .. } => self
                .collection
                .definir_tailles(self.forge.ameliorations().tailles())
                .expect("une amélioration ne réduit aucune place"),
            // Un objet déjà possédé ne se fabrique pas : rien à ajouter.
            Ouvrage::Objet(objet) => {
                let _ = self.heros.equipement_mut().ajouter(objet.clone());
            }
            // Comme tout ce qui est gagné au camp (section 13.3).
            Ouvrage::CharmeDeGarde => self.coffre_du_camp.ajouter_charmes_de_garde(1),
            Ouvrage::Filets { filet, nombre } => {
                self.coffre_du_camp.ajouter_filets(*filet, *nombre)
            }
        }
        Ok(chantier)
    }

    /// Au camp seulement : l'objet remplace celui de cette place (section
    /// 4.3).
    pub fn equiper(
        &mut self,
        place: PlaceDEquipement,
        objet: &ObjetDEquipementId,
    ) -> Result<(), ErreurEquipement> {
        self.heros.equipement_mut().equiper(place, objet)
    }

    pub fn retirer_equipement(&mut self, place: PlaceDEquipement) -> Result<(), ErreurEquipement> {
        self.heros.equipement_mut().retirer(place)
    }

    pub fn ames_donnees(&self) -> &[MarchandMort] {
        &self.ames_donnees
    }

    pub fn ames_portees(&self) -> &[MarchandMort] {
        &self.ames_portees
    }

    pub fn ames_a_retrouver(&self) -> &[MarchandId] {
        &self.ames_a_retrouver
    }

    /// Portée ou déjà donnée à Malvoise.
    pub fn a_recueilli_l_ame(&self, marchand: &MarchandId) -> bool {
        self.ames_donnees
            .iter()
            .chain(&self.ames_portees)
            .any(|recueillie| recueillie.id() == marchand)
    }

    /// Acquise dès la victoire et portée à part du sac : une défaite ne la
    /// fait pas perdre (section 14.2).
    pub(crate) fn recueillir_ame(&mut self, marchand: MarchandMort) {
        self.ames_a_retrouver.retain(|id| id != marchand.id());
        if !self.a_recueilli_l_ame(marchand.id()) {
            self.ames_portees.push(marchand);
        }
    }

    /// Après une fuite ou une défaite face à son gardien (section 14.2).
    pub(crate) fn noter_ame_a_retrouver(&mut self, marchand: &MarchandId) {
        if !self.ames_a_retrouver.contains(marchand) && !self.a_recueilli_l_ame(marchand) {
            self.ames_a_retrouver.push(marchand.clone());
        }
    }

    /// Au camp, chez Malvoise, sans confirmation : l'âme rejoint son crâne
    /// et l'objet de son marchand, son étal (section 14.2). Rend le
    /// marchand.
    pub fn donner_ame(&mut self, marchand: &MarchandId) -> Result<MarchandMort, ErreurMarchand> {
        self.verifier_marchand()?;
        let position = self
            .ames_portees
            .iter()
            .position(|portee| portee.id() == marchand)
            .ok_or(ErreurMarchand::AmeNonPortee)?;
        let donnee = self.ames_portees.remove(position);
        self.ames_donnees.push(donnee.clone());
        Ok(donnee)
    }

    /// Ce que vend Malvoise, dans l'ordre de `Soin::TOUS` : la potion, puis
    /// l'objet de chaque marchand dont l'âme lui est donnée (section 14.2).
    pub fn soins_en_vente(&self) -> Vec<Soin> {
        Soin::TOUS
            .into_iter()
            .filter(|soin| {
                *soin == Soin::Potion
                    || self
                        .ames_donnees
                        .iter()
                        .any(|marchand| marchand.soin() == *soin)
            })
            .collect()
    }

    /// Malvoise vend le filet de chanvre et le filet lesté (section 14.2).
    pub fn filets_en_vente(&self) -> Vec<Filet> {
        if !self.marchand_au_camp {
            return Vec::new();
        }
        Filet::TOUS
            .into_iter()
            .filter(|filet| prix_du_filet(*filet).is_some())
            .collect()
    }

    /// Au camp, sans confirmation ; les filets vont dans le coffre, comme
    /// les soins (sections 13.3 et 14.2). Rend le prix payé.
    pub fn acheter_filets(&mut self, filet: Filet, nombre: u32) -> Result<u32, ErreurMarchand> {
        self.verifier_marchand()?;
        let prix = prix_du_filet(filet).ok_or(ErreurMarchand::FiletIndisponible(filet))?;
        if nombre == 0 {
            return Err(ErreurMarchand::QuantiteNulle);
        }
        let prix = nombre.saturating_mul(prix);
        self.payer(prix)?;
        self.coffre_du_camp.ajouter_filets(filet, nombre);
        Ok(prix)
    }

    /// Au camp, sans confirmation ; les soins vont dans le coffre (sections
    /// 13.3 et 14.2). Rend le prix payé.
    pub fn acheter_soins(&mut self, soin: Soin, nombre: u32) -> Result<u32, ErreurMarchand> {
        self.verifier_marchand()?;
        if !self.soins_en_vente().contains(&soin) {
            return Err(ErreurMarchand::SoinIndisponible(soin));
        }
        if nombre == 0 {
            return Err(ErreurMarchand::QuantiteNulle);
        }
        let prix = nombre.saturating_mul(prix_du_soin(soin));
        self.payer(prix)?;
        self.coffre_du_camp.ajouter_soins(soin, nombre);
        Ok(prix)
    }

    /// Le monstre que la vente retirerait et son prix, sans rien vendre :
    /// de quoi demander confirmation au joueur. Refuse ce que refuserait
    /// `vendre`.
    pub fn estimer_vente(
        &self,
        emplacement: Emplacement,
    ) -> Result<(Monstre, u32), ErreurMarchand> {
        self.verifier_marchand()?;
        let monstre = self
            .collection
            .clone()
            .relacher(emplacement)
            .map_err(ErreurMarchand::Collection)?;
        let prix = prix_de_revente(&monstre);
        Ok((monstre, prix))
    }

    /// Définitive et soumise aux règles du relâcher : jamais le dernier
    /// monstre (section 14.2). La confirmation revient au déroulement.
    /// L'argent va dans le coffre. Rend le monstre vendu et son prix.
    pub fn vendre(&mut self, emplacement: Emplacement) -> Result<(Monstre, u32), ErreurMarchand> {
        self.verifier_marchand()?;
        let monstre = self
            .collection
            .relacher(emplacement)
            .map_err(ErreurMarchand::Collection)?;
        let prix = prix_de_revente(&monstre);
        self.coffre_du_camp.ajouter_argent(prix);
        Ok((monstre, prix))
    }

    /// Au camp, sans confirmation ; la capsule va dans le coffre (sections
    /// 13.3 et 14.2). Rend le prix payé.
    pub fn acheter_capsule(&mut self, capsule: &Capsule) -> Result<u32, ErreurMarchand> {
        self.verifier_marchand()?;
        if self.possede_capsule(capsule.id()) {
            return Err(ErreurMarchand::CapsuleDejaPossedee);
        }
        self.payer(capsule.prix())?;
        self.coffre_du_camp.ajouter_capsule(capsule.clone());
        Ok(capsule.prix())
    }

    /// Au camp, sans confirmation ; les capsules vont dans le coffre
    /// (sections 5.9 et 13.3). Rend le prix payé.
    pub fn acheter_capsules_d_habilete(&mut self, nombre: u32) -> Result<u32, ErreurMarchand> {
        self.verifier_marchand()?;
        if nombre == 0 {
            return Err(ErreurMarchand::QuantiteNulle);
        }
        let prix = nombre.saturating_mul(PRIX_CAPSULE_D_HABILETE);
        self.payer(prix)?;
        self.coffre_du_camp.ajouter_capsules_d_habilete(nombre);
        Ok(prix)
    }

    /// Celles du sac et du coffre : au camp, elles servent toutes.
    pub fn capsules_d_habilete(&self) -> u32 {
        self.sac
            .capsules_d_habilete()
            .saturating_add(self.coffre_du_camp.capsules_d_habilete())
    }

    /// Le monstre et l'habileté que lui donnerait une capsule d'habileté,
    /// sans rien changer : de quoi demander confirmation. Refuse ce que
    /// refuserait `changer_d_habilete`.
    pub fn estimer_changement_d_habilete(
        &self,
        emplacement: Emplacement,
    ) -> Result<(Monstre, Habilete), ErreurChangementDHabilete> {
        let monstre = self
            .collection
            .monstre_a_soi(emplacement)
            .map_err(ErreurChangementDHabilete::Collection)?;
        let nouvelle =
            autre_habilete(monstre).ok_or(ErreurChangementDHabilete::AucuneAutreHabilete)?;
        if self.capsules_d_habilete() == 0 {
            return Err(ErreurChangementDHabilete::AucuneCapsule);
        }
        Ok((monstre.clone(), nouvelle))
    }

    /// Au camp, après confirmation : le monstre passe à l'autre habileté de
    /// son espèce, et une capsule d'habileté est consommée, du sac d'abord,
    /// puis du coffre (section 5.9). Rend l'ancienne habileté et la
    /// nouvelle.
    pub fn changer_d_habilete(
        &mut self,
        emplacement: Emplacement,
    ) -> Result<(Option<Habilete>, Habilete), ErreurChangementDHabilete> {
        let (monstre, nouvelle) = self.estimer_changement_d_habilete(emplacement)?;
        if !self.sac.utiliser_capsule_d_habilete() {
            self.coffre_du_camp.utiliser_capsule_d_habilete();
        }
        self.collection
            .monstre_a_modifier(emplacement)
            .expect("emplacement vérifié")
            .changer_d_habilete(nouvelle);
        Ok((monstre.habilete(), nouvelle))
    }

    /// Ce que ce monstre peut apprendre au camp, gratuitement (section
    /// 13.10).
    pub fn attaques_a_apprendre(
        &self,
        emplacement: Emplacement,
    ) -> Result<Vec<Attaque>, ErreurCollection> {
        let monstre = self
            .collection
            .monstre(emplacement)
            .ok_or(ErreurCollection::EmplacementInexistant(emplacement))?;
        // Un monstre loué n'apprend rien d'une capsule (section 14.2).
        if monstre.est_loue() {
            return Ok(attaques_a_apprendre(monstre, std::iter::empty()));
        }
        Ok(attaques_a_apprendre(monstre, self.capsules()))
    }

    /// Le monstre apprend l'attaque ; s'il en connaît déjà 4, il oublie
    /// celle en position `oubliee`, que l'interface a fait choisir au joueur
    /// (section 13.10). Rend l'attaque oubliée.
    pub fn apprendre_attaque(
        &mut self,
        emplacement: Emplacement,
        attaque: &AttaqueId,
        oubliee: Option<usize>,
    ) -> Result<Option<Attaque>, ErreurApprentissage> {
        let nouvelle = self
            .attaques_a_apprendre(emplacement)
            .map_err(ErreurApprentissage::Collection)?
            .into_iter()
            .find(|possible| possible.id() == attaque)
            .ok_or(ErreurApprentissage::AttaqueInaccessible)?;
        let monstre = self.collection.monstre_a_modifier(emplacement).ok_or(
            ErreurApprentissage::Collection(ErreurCollection::EmplacementInexistant(emplacement)),
        )?;
        match (monstre.attaques().est_complete(), oubliee) {
            (true, None) => Err(ErreurApprentissage::OubliManquant),
            (false, Some(_)) => Err(ErreurApprentissage::OubliInutile),
            (false, None) => monstre
                .ajouter_attaque(nouvelle)
                .map(|()| None)
                .map_err(ErreurApprentissage::Attaque),
            (true, Some(position)) => {
                let ancienne = monstre.attaques().liste().get(position).cloned().ok_or(
                    ErreurApprentissage::Attaque(ErreurAttaquesMonstre::PositionInexistante(
                        position,
                    )),
                )?;
                monstre
                    .remplacer_attaque(position, nouvelle)
                    .map(|()| Some(ancienne))
                    .map_err(ErreurApprentissage::Attaque)
            }
        }
    }

    /// Le monstre, l'espèce en laquelle il évoluerait au camp et la pierre
    /// que l'évolution consommerait, sans rien changer : de quoi demander
    /// confirmation. Refuse ce que refuserait `faire_evoluer`.
    pub fn estimer_evolution(
        &self,
        emplacement: Emplacement,
        catalogue: &dyn Catalogue,
    ) -> Result<(Monstre, Espece, Option<PierreDEvolution>), ErreurEvolution> {
        let monstre = self
            .collection
            .monstre_a_soi(emplacement)
            .map_err(ErreurEvolution::Collection)?;
        let evolution = monstre.espece().evolution();
        let pierre = evolution.and_then(|evolution| evolution.pierre());
        let vers = match pierre {
            Some(pierre) => {
                if self.nombre_de_pierres(pierre) == 0 {
                    return Err(ErreurEvolution::PierreManquante(pierre));
                }
                evolution.and_then(|evolution| catalogue.espece(evolution.vers()))
            }
            None => {
                let au_camp = Circonstances {
                    zone: None,
                    apotres_vaincus: &self.apotres_vaincus,
                };
                evolution_possible(monstre, &au_camp, catalogue)
            }
        };
        let vers = vers.ok_or(ErreurEvolution::ImpossiblePourLInstant)?;
        Ok((monstre.clone(), vers, pierre))
    }

    /// Celles du sac et du coffre : au camp, elles servent toutes (section
    /// 5.4).
    pub fn nombre_de_pierres(&self, pierre: PierreDEvolution) -> u32 {
        self.sac
            .nombre_de_pierres(pierre)
            .saturating_add(self.coffre_du_camp.nombre_de_pierres(pierre))
    }

    /// Au camp, sans confirmation ; les pierres vont dans le coffre (section
    /// 5.4). Rend le prix payé.
    pub fn acheter_pierres(
        &mut self,
        pierre: PierreDEvolution,
        nombre: u32,
    ) -> Result<u32, ErreurMarchand> {
        self.verifier_marchand()?;
        if nombre == 0 {
            return Err(ErreurMarchand::QuantiteNulle);
        }
        let prix = nombre.saturating_mul(PRIX_PIERRE_D_EVOLUTION);
        self.payer(prix)?;
        self.coffre_du_camp.ajouter_pierres(pierre, nombre);
        Ok(prix)
    }

    /// Trouvée dans un coffre, elle va dans le sac (section 5.4).
    pub fn trouver_pierre(&mut self, pierre: PierreDEvolution) {
        self.sac.ajouter_pierres(pierre, 1);
    }

    /// Ceux du sac et du coffre : au camp, ils servent tous (section 14.3).
    pub fn nombre_de_boosters(&self, booster: Booster) -> u32 {
        self.sac
            .nombre_de_boosters(booster)
            .saturating_add(self.coffre_du_camp.nombre_de_boosters(booster))
    }

    /// Trouvé dans un coffre, il va dans le sac (section 14.3).
    pub fn trouver_booster(&mut self, booster: Booster) {
        self.sac.ajouter_boosters(booster, 1);
    }

    /// Au camp, sans confirmation (section 14.3) : le booster est consommé,
    /// du sac d'abord, puis du coffre. Refusé, sans rien consommer, si la
    /// valeur interne visée est déjà à 31. Rend le monstre d'avant.
    pub fn donner_booster(
        &mut self,
        emplacement: Emplacement,
        booster: Booster,
    ) -> Result<Monstre, ErreurBooster> {
        let monstre = self
            .collection
            .monstre_a_soi(emplacement)
            .map_err(ErreurBooster::Collection)?
            .clone();
        if *monstre.valeurs_internes().valeurs().de(booster.stat()) >= ValeursInternes::MAX {
            return Err(ErreurBooster::ValeurMaximale(booster));
        }
        if !self.sac.utiliser_booster(booster) && !self.coffre_du_camp.utiliser_booster(booster) {
            return Err(ErreurBooster::Aucun(booster));
        }
        self.collection
            .monstre_a_modifier(emplacement)
            .expect("emplacement vérifié")
            .recevoir_booster(booster);
        Ok(monstre)
    }

    /// Ceux du sac et du coffre : au camp, ils servent tous (section 13.7).
    pub fn philtres_chromatiques(&self) -> u32 {
        self.sac
            .philtres_chromatiques()
            .saturating_add(self.coffre_du_camp.philtres_chromatiques())
    }

    /// Le monstre que Malvoise rendrait shiny, sans rien changer : de quoi
    /// demander confirmation. Refuse ce que refuserait `rendre_shiny`.
    pub fn estimer_shiny(&self, emplacement: Emplacement) -> Result<Monstre, ErreurMarchand> {
        self.verifier_marchand()?;
        let monstre = self
            .collection
            .monstre_a_soi(emplacement)
            .map_err(ErreurMarchand::Collection)?;
        if monstre.est_shiny() {
            return Err(ErreurMarchand::DejaShiny);
        }
        if self.philtres_chromatiques() == 0 {
            return Err(ErreurMarchand::AucunPhiltreChromatique);
        }
        Ok(monstre.clone())
    }

    /// Au camp, chez Malvoise, après confirmation (section 13.7) : le
    /// monstre devient shiny, et un Philtre chromatique est consommé, du sac
    /// d'abord, puis du coffre. Rend le monstre d'avant.
    pub fn rendre_shiny(&mut self, emplacement: Emplacement) -> Result<Monstre, ErreurMarchand> {
        let monstre = self.estimer_shiny(emplacement)?;
        if !self.sac.utiliser_philtre_chromatique() {
            self.coffre_du_camp.utiliser_philtre_chromatique();
        }
        self.collection
            .monstre_a_modifier(emplacement)
            .expect("emplacement vérifié")
            .devenir_shiny();
        Ok(monstre)
    }

    /// Au camp, après confirmation (section 5.4) : la forme évoluée compte
    /// comme capturée au codex, et la pierre éventuelle est consommée, du sac
    /// d'abord, puis du coffre. Une attaque d'évolution en attente rejoint
    /// les attaques à choisir. Rend le monstre d'avant l'évolution.
    pub fn faire_evoluer(
        &mut self,
        emplacement: Emplacement,
        catalogue: &dyn Catalogue,
    ) -> Result<(Monstre, ResultatEvolution), ErreurEvolution> {
        let (avant, vers, pierre) = self.estimer_evolution(emplacement, catalogue)?;
        if let Some(pierre) = pierre {
            if !self.sac.utiliser_pierre(pierre) {
                self.coffre_du_camp.utiliser_pierre(pierre);
            }
        }
        self.codex.noter_capture(vers.id());
        let monstre = self
            .collection
            .monstre_a_modifier(emplacement)
            .expect("emplacement vérifié");
        let resultat = faire_evoluer(monstre, vers);
        if let Some(attaque) = &resultat.attaque_en_attente {
            self.attaques_a_choisir.push(AttaqueAChoisir {
                emplacement,
                attaque: attaque.clone(),
            });
        }
        Ok((avant, resultat))
    }

    /// Les monstres de l'équipe puis du camp qui peuvent évoluer au camp, y
    /// compris par une pierre que le joueur a, et en quelle espèce (section
    /// 5.4).
    pub fn evolutions_possibles(&self, catalogue: &dyn Catalogue) -> Vec<(Emplacement, Espece)> {
        let equipe = self
            .collection
            .equipe()
            .iter()
            .enumerate()
            .map(|(index, monstre)| (Emplacement::Equipe(index), monstre));
        let camp = self
            .collection
            .camp()
            .iter()
            .enumerate()
            .map(|(index, monstre)| (Emplacement::Camp(index), monstre));
        equipe
            .chain(camp)
            .filter_map(|(emplacement, _)| {
                let (_, vers, _) = self.estimer_evolution(emplacement, catalogue).ok()?;
                Some((emplacement, vers))
            })
            .collect()
    }

    /// Les espèces communes sauvages, à leur forme de base, des zones menées
    /// au bout, au niveau minimal de la première zone où chacune se
    /// rencontre, dans l'ordre des zones (section 14.2, Q98). `especes`
    /// sert à reconnaître les formes évoluées sauvages.
    pub fn offres_d_achat(&self, zones: &[Zone], especes: &[Espece]) -> Vec<OffreDeMonstre> {
        let est_evoluee = |id: &EspeceId| {
            especes.iter().any(|espece| {
                espece
                    .evolution()
                    .is_some_and(|evolution| evolution.vers() == id)
            })
        };
        let mut offres: Vec<OffreDeMonstre> = Vec::new();
        for zone in zones.iter().filter(|zone| self.est_exploree(zone.id())) {
            for espece in zone.especes() {
                let deja = offres.iter().any(|offre| offre.espece.id() == espece.id());
                if !deja && espece.rarete() == Rarete::Commune && !est_evoluee(espece.id()) {
                    offres.push(OffreDeMonstre::a_l_achat(espece.clone(), zone.niveaux().0));
                }
            }
        }
        offres
    }

    /// Les espèces rares des zones menées au bout, monstre caché excepté, au
    /// niveau maximal de la zone ouverte la plus avancée (section 14.2).
    pub fn offres_de_location(&self, zones: &[Zone], piste: &Piste) -> Vec<OffreDeMonstre> {
        let Some(niveau) = self
            .zone_la_plus_avancee(zones, piste)
            .map(|zone| zone.niveaux().1)
        else {
            return Vec::new();
        };
        let mut offres: Vec<OffreDeMonstre> = Vec::new();
        for zone in zones.iter().filter(|zone| self.est_exploree(zone.id())) {
            for rare in zone.rares() {
                if !offres
                    .iter()
                    .any(|offre| offre.espece.id() == rare.espece.id())
                {
                    offres.push(OffreDeMonstre::a_la_location(rare.espece.clone(), niveau));
                }
            }
        }
        offres
    }

    /// Au camp, chez Malvoise, sans confirmation (section 14.2) : le monstre
    /// est tiré comme un monstre sauvage, puis rejoint l'équipe s'il y reste
    /// une place, sinon le camp ; son espèce passe capturée au codex. Rend le
    /// monstre, sa place et le prix payé.
    pub fn acheter_monstre(
        &mut self,
        espece: &EspeceId,
        zones: &[Zone],
        especes: &[Espece],
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<(Monstre, Placement, u32), ErreurMarchand> {
        self.verifier_marchand()?;
        let offre = self
            .offres_d_achat(zones, especes)
            .into_iter()
            .find(|offre| offre.espece.id() == espece)
            .ok_or(ErreurMarchand::MonstreIndisponible)?;
        if !self.collection.a_de_la_place() {
            return Err(ErreurMarchand::PlusDePlace);
        }
        self.payer(offre.prix)?;
        let monstre = monstre_de_malvoise(&offre, aleatoire);
        self.codex.noter_capture(monstre.espece().id());
        let placement = self.collection.accueillir_capture(monstre.clone());
        Ok((monstre, placement, offre.prix))
    }

    /// Au camp, chez Malvoise, sans confirmation (section 14.2) : comme un
    /// achat, mais un seul à la fois, pour 3 expéditions manuelles, et son
    /// espèce passe seulement rencontrée au codex (Q98).
    pub fn louer_monstre(
        &mut self,
        espece: &EspeceId,
        zones: &[Zone],
        piste: &Piste,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<(Monstre, Placement, u32), ErreurMarchand> {
        self.verifier_marchand()?;
        if self.collection.monstre_loue().is_some() {
            return Err(ErreurMarchand::DejaUnMonstreLoue);
        }
        let offre = self
            .offres_de_location(zones, piste)
            .into_iter()
            .find(|offre| offre.espece.id() == espece)
            .ok_or(ErreurMarchand::MonstreIndisponible)?;
        if !self.collection.a_de_la_place() {
            return Err(ErreurMarchand::PlusDePlace);
        }
        self.payer(offre.prix)?;
        let monstre = monstre_de_malvoise(&offre, aleatoire).louer(EXPEDITIONS_DE_LOCATION);
        self.codex.noter_rencontre(monstre.espece().id());
        let placement = self.collection.accueillir_capture(monstre.clone());
        Ok((monstre, placement, offre.prix))
    }

    /// Au camp, après confirmation, sans remboursement (section 14.2).
    pub fn rendre_le_monstre_loue(&mut self) -> Result<Monstre, ErreurMarchand> {
        let (emplacement, monstre) = self
            .collection
            .rendre_le_monstre_loue()
            .ok_or(ErreurMarchand::AucunMonstreLoue)?;
        self.oublier_les_attaques_a_choisir(emplacement);
        Ok(monstre)
    }

    /// Au retour au camp d'une expédition manuelle, quelle qu'en soit
    /// l'issue : une expédition de plus pour la location ; à la dernière, le
    /// monstre loué repart chez Malvoise, rendu ici (section 14.2).
    pub(crate) fn decompter_location(&mut self) -> Option<Monstre> {
        let (emplacement, monstre) = self.collection.decompter_location()?;
        self.oublier_les_attaques_a_choisir(emplacement);
        Some(monstre)
    }

    /// Le monstre de cet emplacement est parti : ses attaques à choisir sont
    /// oubliées, et celles des monstres qui le suivaient suivent leur place.
    fn oublier_les_attaques_a_choisir(&mut self, parti: Emplacement) {
        self.attaques_a_choisir
            .retain(|a_choisir| a_choisir.emplacement != parti);
        for a_choisir in &mut self.attaques_a_choisir {
            a_choisir.emplacement = match (a_choisir.emplacement, parti) {
                (Emplacement::Equipe(index), Emplacement::Equipe(depart)) if index > depart => {
                    Emplacement::Equipe(index - 1)
                }
                (Emplacement::Camp(index), Emplacement::Camp(depart)) if index > depart => {
                    Emplacement::Camp(index - 1)
                }
                (emplacement, _) => emplacement,
            };
        }
    }

    /// Une évolution après un combat, sur l'équipe du bilan.
    pub(crate) fn noter_evolution(&mut self, espece: &EspeceId) {
        self.codex.noter_capture(espece);
    }

    fn verifier_marchand(&self) -> Result<(), ErreurMarchand> {
        if self.marchand_au_camp {
            Ok(())
        } else {
            Err(ErreurMarchand::AucunMarchand)
        }
    }

    /// Avec l'argent du sac d'abord, puis celui du coffre (section 13.3).
    fn payer(&mut self, prix: u32) -> Result<(), ErreurMarchand> {
        let argent = self.argent();
        if prix > argent {
            return Err(ErreurMarchand::ArgentInsuffisant { prix, argent });
        }
        let du_sac = self.sac.prendre_argent(prix);
        self.coffre_du_camp.prendre_argent(prix - du_sac);
        Ok(())
    }

    pub fn codex(&self) -> &Codex {
        &self.codex
    }

    pub fn est_exploree(&self, zone: &ZoneId) -> bool {
        self.zones_explorees.contains(zone)
    }

    pub fn zones_explorees(&self) -> &[ZoneId] {
        &self.zones_explorees
    }

    pub fn indices_trouves(&self) -> usize {
        self.indices_trouves
    }

    pub fn apotres_vaincus(&self) -> &[EspeceId] {
        &self.apotres_vaincus
    }

    /// L'âme de chaque Apôtre vaincu, dans l'ordre des victoires : à part
    /// du sac et du coffre, rien ne la fait perdre, jusqu'à ce que Malvoise
    /// en forge le Béhérit pourpre (section 9.4).
    pub fn ames_d_apotres(&self) -> &[EspeceId] {
        if self.beherit.est_forge() {
            &[]
        } else {
            &self.apotres_vaincus
        }
    }

    pub fn beherit(&self) -> EtatDuBeherit {
        self.beherit
    }

    /// Le carnet de traque du codex : les Apôtres abattus, dans l'ordre des
    /// victoires, puis le Démiurge une fois vaincu. Rien de ceux qui restent,
    /// ni leur nombre (section 5.6).
    pub fn carnet_de_traque<'a>(
        &self,
        piste: &'a Piste,
        invocation: &'a Invocation,
    ) -> Vec<&'a Apotre> {
        let mut carnet: Vec<&Apotre> = self
            .apotres_vaincus
            .iter()
            .filter_map(|id| piste.apotres().find(|apotre| apotre.espece().id() == id))
            .collect();
        if self.beherit == EtatDuBeherit::DemiurgeVaincu {
            carnet.push(invocation.demiurge());
        }
        carnet
    }

    /// Ce que refuserait `forger_le_beherit`, sans rien changer : de quoi
    /// demander confirmation.
    pub fn verifier_la_forge_du_beherit(&self, piste: &Piste) -> Result<(), ErreurBeherit> {
        if !self.marchand_au_camp {
            return Err(ErreurBeherit::AucunMarchand);
        }
        if self.beherit.est_forge() {
            return Err(ErreurBeherit::DejaForge);
        }
        if !piste.apotres().all(|apotre| self.est_vaincu(apotre)) {
            return Err(ErreurBeherit::AmesManquantes);
        }
        Ok(())
    }

    /// Au camp, chez Malvoise, après confirmation : les âmes de tous les
    /// Apôtres sont consommées (section 9.4).
    pub fn forger_le_beherit(&mut self, piste: &Piste) -> Result<(), ErreurBeherit> {
        self.verifier_la_forge_du_beherit(piste)?;
        self.beherit = EtatDuBeherit::Forge;
        Ok(())
    }

    /// Le monstre que le Béhérit prendrait, sans rien changer : de quoi
    /// demander confirmation. Refuse ce que refuserait `sacrifier`.
    pub fn estimer_sacrifice(&self, emplacement: Emplacement) -> Result<Monstre, ErreurBeherit> {
        match self.beherit {
            EtatDuBeherit::Aucun => return Err(ErreurBeherit::AucunBeherit),
            EtatDuBeherit::Forge => {}
            EtatDuBeherit::EclipseOuverte | EtatDuBeherit::DemiurgeVaincu => {
                return Err(ErreurBeherit::SacrificeDejaFait);
            }
        }
        let monstre = self
            .collection
            .monstre_a_soi(emplacement)
            .map_err(ErreurBeherit::Collection)?;
        if monstre.niveau().valeur() < NIVEAU_MINIMAL_DU_SACRIFICE {
            return Err(ErreurBeherit::NiveauInsuffisant);
        }
        if matches!(emplacement, Emplacement::Equipe(_)) && self.collection.equipe().len() == 1 {
            return Err(ErreurBeherit::EquipeVide);
        }
        // Les règles du relâcher valent aussi : jamais le dernier monstre.
        self.collection
            .clone()
            .relacher(emplacement)
            .map_err(ErreurBeherit::Collection)?;
        Ok(monstre.clone())
    }

    /// Les monstres que le Béhérit accepterait, équipe puis camp.
    pub fn monstres_sacrifiables(&self) -> Vec<Emplacement> {
        let equipe = (0..self.collection.equipe().len()).map(Emplacement::Equipe);
        let camp = (0..self.collection.camp().len()).map(Emplacement::Camp);
        equipe
            .chain(camp)
            .filter(|emplacement| self.estimer_sacrifice(*emplacement).is_ok())
            .collect()
    }

    /// Après confirmation, la première fois que le joueur part vers
    /// l'Éclipse : le monstre est perdu pour toujours, et l'Éclipse reste
    /// ouverte jusqu'à la victoire (section 9.4). Rend le monstre sacrifié.
    pub fn sacrifier(&mut self, emplacement: Emplacement) -> Result<Monstre, ErreurBeherit> {
        self.estimer_sacrifice(emplacement)?;
        let monstre = self
            .collection
            .relacher(emplacement)
            .map_err(ErreurBeherit::Collection)?;
        self.oublier_les_attaques_a_choisir(emplacement);
        self.beherit = EtatDuBeherit::EclipseOuverte;
        Ok(monstre)
    }

    /// Une expédition d'une seule étape, où attend le Démiurge, une fois le
    /// sacrifice fait et tant qu'il n'est pas vaincu (section 9.4).
    pub fn partir_vers_l_eclipse(
        &self,
        invocation: &Invocation,
    ) -> Result<Expedition, ErreurExpedition> {
        if self.beherit != EtatDuBeherit::EclipseOuverte {
            return Err(ErreurExpedition::ZoneFermee);
        }
        if self.collection.equipe().is_empty() {
            return Err(ErreurExpedition::EquipeVide);
        }
        Ok(Expedition::partir(invocation.eclipse().clone())
            .vers_l_antre(invocation.demiurge().clone()))
    }

    /// La Tour sans fond s'ouvre quand la Lisière est menée au bout
    /// (section 9.5).
    pub fn tour_ouverte(&self, tour: &TourSansFond) -> bool {
        self.est_exploree(tour.ouverte_par())
    }

    /// Une descente seul dans la Tour sans fond (section 9.5).
    pub fn descendre_dans_la_tour(
        &self,
        tour: &TourSansFond,
        zones: &[Zone],
    ) -> Result<Expedition, ErreurExpedition> {
        self.preparer_la_descente(tour, zones, false)
    }

    /// Les coffres de la Tour contiennent les capsules des zones que le
    /// joueur a menées au bout, et qu'il n'a pas encore (section 9.5).
    pub(crate) fn preparer_la_descente(
        &self,
        tour: &TourSansFond,
        zones: &[Zone],
        a_deux: bool,
    ) -> Result<Expedition, ErreurExpedition> {
        if !self.tour_ouverte(tour) {
            return Err(ErreurExpedition::ZoneFermee);
        }
        if self.collection.equipe().is_empty() {
            return Err(ErreurExpedition::EquipeVide);
        }
        let capsules = zones
            .iter()
            .filter(|zone| self.est_exploree(zone.id()))
            .flat_map(|zone| zone.coffre().capsules().iter().cloned())
            .collect();
        let possedees = self
            .capsules()
            .map(|capsule| capsule.id().clone())
            .collect();
        Ok(
            Expedition::descendre(tour.clone(), capsules, a_deux)
                .avec_capsules_possedees(possedees),
        )
    }

    pub fn records_de_la_tour(&self) -> RecordsDeLaTour {
        self.records_de_la_tour
    }

    /// Garde la dernière étape franchie si elle bat le record, seul ou à
    /// deux (section 9.5). Rend vrai pour un nouveau record.
    pub(crate) fn noter_record_de_la_tour(&mut self, etapes: u32, a_deux: bool) -> bool {
        let record = if a_deux {
            &mut self.records_de_la_tour.a_deux
        } else {
            &mut self.records_de_la_tour.seul
        };
        if etapes > *record {
            *record = etapes;
            true
        } else {
            false
        }
    }

    /// Ses deux formes s'ouvrent au codex ; il laisse 3 Philtres chromatiques dans
    /// le sac, et l'Éclipse se referme (section 9.4).
    pub(crate) fn noter_demiurge_vaincu(&mut self, demiurge: &Apotre) {
        if self.beherit != EtatDuBeherit::DemiurgeVaincu {
            self.beherit = EtatDuBeherit::DemiurgeVaincu;
            self.sac.ajouter_philtres_chromatiques(PHILTRES_DU_DEMIURGE);
        }
        self.codex.noter_capture(demiurge.espece().id());
        if let Some(seconde) = demiurge.seconde_forme() {
            self.codex.noter_capture(seconde.id());
        }
    }

    pub fn est_vaincu(&self, apotre: &Apotre) -> bool {
        self.apotres_vaincus.contains(apotre.espece().id())
    }

    pub fn humains_vaincus(&self) -> &[HumainId] {
        &self.humains_vaincus
    }

    pub fn conseils_vus(&self) -> &[Conseil] {
        &self.conseils_vus
    }

    /// Reconstitue une partie depuis une sauvegarde : chaque morceau a déjà
    /// été revérifié par sa propre restauration. Il reste à vérifier ce qui
    /// les lie : les monstres partis gardent leur place au camp, une seule
    /// expédition automatique par zone explorée, chaque attaque à choisir
    /// désigne un monstre de l'équipe ou du camp, une capsule n'est obtenue qu'une fois, et
    /// le joueur a au moins un monstre, au camp ou parti.
    pub fn restaurer(etat: EtatPartie) -> Result<Self, ErreurRestaurationPartie> {
        if let Some(capsule) = etat
            .sac
            .capsules()
            .iter()
            .find(|capsule| etat.coffre_du_camp.contient_capsule(capsule.id()))
        {
            return Err(ErreurRestaurationPartie::CapsuleEnDouble(
                capsule.id().clone(),
            ));
        }
        let mut collection = etat.collection;
        collection
            .definir_tailles(etat.forge.ameliorations().tailles())
            .map_err(ErreurRestaurationPartie::Collection)?;
        let ameliorations = etat.forge.ameliorations();
        let cour = ameliorations.cour().map_or(0, |regles| regles.places);
        let pieges_valides = etat.machines.pieges().is_none_or(|pieges| {
            ameliorations.pieges().is_some() && etat.zones_explorees.contains(pieges.zone())
        });
        if etat.machines.cour().len() > cour || !pieges_valides {
            return Err(ErreurRestaurationPartie::MachinesIncoherentes);
        }
        // Un monstre à l'entraînement garde sa place au camp, comme un
        // monstre parti (section 13.6).
        let partis = etat
            .expeditions_automatiques
            .iter()
            .map(|expedition| expedition.monstres().len())
            .sum::<usize>()
            + etat.machines.cour().len();
        collection
            .definir_places_reservees(partis)
            .map_err(ErreurRestaurationPartie::Collection)?;
        if collection.nombre_de_monstres() == 0 {
            return Err(ErreurRestaurationPartie::AucunMonstre);
        }
        let loues = collection
            .equipe()
            .iter()
            .chain(collection.camp())
            .filter(|monstre| monstre.est_loue())
            .count();
        if loues > 1 || (loues == 1 && collection.nombre_de_monstres_a_soi() == 0) {
            return Err(ErreurRestaurationPartie::LocationIncoherente);
        }
        for (position, expedition) in etat.expeditions_automatiques.iter().enumerate() {
            let zone = expedition.zone().id();
            if !etat.zones_explorees.contains(zone) {
                return Err(ErreurRestaurationPartie::ZoneNonExploree(zone.clone()));
            }
            let precedentes = &etat.expeditions_automatiques[..position];
            if precedentes.iter().any(|autre| autre.zone().id() == zone) {
                return Err(ErreurRestaurationPartie::ExpeditionEnDouble(zone.clone()));
            }
        }
        if etat.peche.prise().is_some() && !collection.a_de_la_place() {
            return Err(ErreurRestaurationPartie::PriseSansPlace);
        }
        for a_choisir in &etat.attaques_a_choisir {
            if collection.monstre(a_choisir.emplacement).is_none() {
                return Err(ErreurRestaurationPartie::AttaqueAChoisirSansMonstre(
                    a_choisir.emplacement,
                ));
            }
        }
        Ok(Self {
            heros: etat.heros,
            collection,
            sac: etat.sac,
            coffre_du_camp: etat.coffre_du_camp,
            marchand_au_camp: etat.marchand_au_camp,
            ames_donnees: etat.ames_donnees,
            ames_portees: etat.ames_portees,
            ames_a_retrouver: etat.ames_a_retrouver,
            codex: etat.codex,
            zones_explorees: etat.zones_explorees,
            indices_trouves: etat.indices_trouves,
            apotres_vaincus: etat.apotres_vaincus,
            humains_vaincus: etat.humains_vaincus,
            conseils_vus: etat.conseils_vus,
            expeditions_automatiques: etat.expeditions_automatiques,
            attaques_a_choisir: etat.attaques_a_choisir,
            forge: etat.forge,
            peche: etat.peche,
            relations: etat.relations,
            machines: etat.machines,
            beherit: etat.beherit,
            records_de_la_tour: etat.records_de_la_tour,
        })
    }

    /// Les zones ouvertes par la piste de l'Apôtre (section 9.1), dans
    /// l'ordre de la carte.
    pub fn zones_accessibles<'a>(&self, zones: &'a [Zone], piste: &Piste) -> Vec<&'a Zone> {
        zones
            .iter()
            .filter(|zone| piste.zone_ouverte(zone.id(), self.indices_trouves))
            .collect()
    }

    /// Un monstre reçu d'un autre joueur porte le nom du héros qui l'a cédé,
    /// et son espèce passe capturée au codex (section 15.1).
    pub(crate) fn preparer_monstre_recu(&mut self, mut monstre: Monstre, de: &Nom) -> Monstre {
        monstre.marquer_recu(de.clone());
        self.codex.noter_capture(monstre.espece().id());
        monstre
    }

    /// Niveau jusqu'auquel ses monstres reçus d'un autre joueur obéissent
    /// (section 15.2).
    pub fn limite_d_obeissance(&self, zones: &[Zone], piste: &Piste) -> u8 {
        limite_d_obeissance(self.zones_accessibles(zones, piste))
    }

    /// Une fois tous les indices trouvés, une expédition dans la zone de
    /// l'Apôtre s'achève par le combat contre lui, tant qu'il n'est pas
    /// vaincu (section 9.4). Elle peut trouver l'âme du marchand mort de la
    /// zone, tant qu'elle n'est pas recueillie : à son étape s'il en a une,
    /// à coup sûr si elle est à retrouver, sinon à 10 % des étapes (section
    /// 14.2). Ansgar l'attend à son étape tant qu'il n'est pas au camp
    /// (section 9.2). Ses coffres ne contiennent que des
    /// capsules que le joueur n'a pas encore (section 13.10). Une équipe vide
    /// ne part pas (section 13.2).
    pub fn partir_en_expedition(
        &self,
        zone: &Zone,
        piste: &Piste,
    ) -> Result<Expedition, ErreurExpedition> {
        if !piste.zone_ouverte(zone.id(), self.indices_trouves) {
            return Err(ErreurExpedition::ZoneFermee);
        }
        if self.collection.equipe().is_empty() {
            return Err(ErreurExpedition::EquipeVide);
        }
        let possedees = self
            .capsules()
            .map(|capsule| capsule.id().clone())
            .collect();
        let mut expedition = Expedition::partir(zone.clone()).avec_capsules_possedees(possedees);
        if let Some(marchand) = zone
            .marchand_mort()
            .filter(|marchand| !self.a_recueilli_l_ame(marchand.id()))
        {
            let rencontre = match marchand.etape() {
                Some(etape) => RencontreDeLAme::AEtape(etape),
                None if self.ames_a_retrouver.contains(marchand.id()) => {
                    RencontreDeLAme::ARetrouver
                }
                None => RencontreDeLAme::Tiree,
            };
            expedition = expedition.avec_ame(marchand.clone(), rencontre);
        }
        if let Some(pecheur) = zone.pecheur().filter(|_| !self.peche.pecheur_au_camp()) {
            expedition = expedition.avec_pecheur(pecheur.clone());
        }
        if let Some(apotre) =
            piste.apotre_en_attente(zone.id(), self.indices_trouves, &self.apotres_vaincus)
        {
            return Ok(expedition.vers_l_antre(apotre.clone()));
        }
        if let Some(humain) = zone
            .humain()
            .filter(|humain| !self.humains_vaincus.contains(humain.id()))
        {
            expedition = expedition.contre_l_humain(humain.clone());
        }
        Ok(expedition)
    }

    /// Combat contre un groupe sauvage, avec l'équipe, le bonus et l'ordre du
    /// héros et les soins du sac. Les espèces du groupe entrent dans le
    /// codex comme rencontrées.
    pub fn preparer_combat(&mut self, groupe: Vec<Monstre>) -> Result<Combat, ErreurCombat> {
        let especes: Vec<_> = groupe
            .iter()
            .map(|monstre| monstre.espece().id().clone())
            .collect();
        let combat = self.combat_contre(groupe)?;
        for espece in &especes {
            self.codex.noter_rencontre(espece);
        }
        Ok(combat)
    }

    fn combat_contre(&self, groupe: Vec<Monstre>) -> Result<Combat, ErreurCombat> {
        Ok(Combat::new(self.collection.equipe().to_vec(), groupe)?
            .avec_bonus_du_joueur(self.heros.bonus_en_combat())
            .avec_bonus_de_role_du_cote(Cote::Joueur)
            .avec_ordre(self.heros.classe().ordre())
            .avec_filets(self.forge.ameliorations().facteur_des_filets())
            .avec_soins(*self.sac.soins())
            .avec_reserve_de_filets(*self.sac.filets()))
    }

    /// Combat d'une expédition en groupe : l'équipe de l'hôte, puis celle de
    /// l'invité, chacune avec le bonus, l'ordre, la limite d'obéissance, les
    /// soins et les filets de son joueur (section 15.5). Le gardien de la
    /// Tour se combat sans fuite, ni filet, ni capture (section 9.5).
    pub(crate) fn preparer_combat_en_groupe(
        hote: &Partie,
        invite: &Partie,
        groupe: Vec<Monstre>,
        contre_un_gardien: bool,
        zones: &[Zone],
        piste: &Piste,
    ) -> Result<Combat, ErreurCombat> {
        let combat = Combat::en_groupe(
            hote.collection.equipe().to_vec(),
            invite.collection.equipe().to_vec(),
            groupe,
        )?
        .avec_bonus_du_joueur(hote.heros.bonus_en_combat())
        .avec_bonus_de_role_du_cote(Cote::Joueur)
        .avec_ordre(hote.heros.classe().ordre())
        .avec_filets(hote.forge.ameliorations().facteur_des_filets())
        .avec_soins(*hote.sac.soins())
        .avec_reserve_de_filets(*hote.sac.filets())
        .avec_limite_d_obeissance(hote.limite_d_obeissance(zones, piste))
        .avec_regles_de_l_invite(ReglesDuMembre {
            bonus: invite.heros.bonus_en_combat(),
            ordre: invite.heros.classe().ordre(),
            limite_d_obeissance: invite.limite_d_obeissance(zones, piste),
            soins: *invite.sac.soins(),
            filets: *invite.sac.filets(),
            facteur_des_filets: invite.forge.ameliorations().facteur_des_filets(),
        });
        Ok(if contre_un_gardien {
            combat.contre_un_gardien()
        } else {
            combat
        })
    }

    /// Comme `preparer_combat`, sans fuite ni filet (section 9.4). Les
    /// formes du Démiurge se succèdent : la seconde reste cachée, et n'entre
    /// au codex qu'en apparaissant (Q99).
    pub fn preparer_combat_contre_apotre(
        &mut self,
        formes: Vec<Monstre>,
    ) -> Result<Combat, ErreurCombat> {
        let premiere = formes.first().map(|forme| forme.espece().id().clone());
        let deux_phases = formes.len() > 1;
        let combat = self.combat_contre(formes)?.contre_un_apotre();
        if let Some(premiere) = premiere {
            self.codex.noter_rencontre(&premiere);
        }
        Ok(if deux_phases {
            combat.en_deux_phases()
        } else {
            combat
        })
    }

    /// Contre un marchand relevé d'entre les morts : fuite possible, mais ni
    /// filet ni capture, et il n'entre pas au codex (section 14.2).
    pub fn preparer_combat_sans_capture(
        &mut self,
        groupe: Vec<Monstre>,
    ) -> Result<Combat, ErreurCombat> {
        Ok(Combat::new(self.collection.equipe().to_vec(), groupe)?
            .avec_bonus_du_joueur(self.heros.bonus_en_combat())
            .avec_bonus_de_role_du_cote(Cote::Joueur)
            .avec_ordre(self.heros.classe().ordre())
            .avec_soins(*self.sac.soins())
            .avec_reserve_de_filets(*self.sac.filets())
            .sans_capture())
    }

    /// Ni fuite ni filet contre un humain (section 10.7). Ses monstres entrent
    /// au codex comme rencontrés.
    pub fn preparer_combat_contre_humain(
        &mut self,
        equipe: Vec<Monstre>,
        ia: IaHumain,
    ) -> Result<Combat, ErreurCombat> {
        self.preparer_combat(equipe)
            .map(|combat| combat.contre_un_humain(ia))
    }

    /// Un humain vaincu ne revient plus ; son argent va dans le sac (section
    /// 10.7). Rend cet argent.
    pub fn vaincre_humain(&mut self, humain: &Humain) -> u32 {
        if !self.humains_vaincus.contains(humain.id()) {
            self.humains_vaincus.push(humain.id().clone());
        }
        self.sac.ajouter_argent(humain.argent());
        humain.argent()
    }

    /// Après la victoire, l'Apôtre ne réapparaît plus ; il ne se capture
    /// pas, mais sa fiche s'ouvre au codex et le joueur garde son âme
    /// (section 9.4). Il laisse aussi un Philtre chromatique, qui va dans le sac
    /// (section 14.3).
    pub fn noter_apotre_vaincu(&mut self, apotre: &Apotre) {
        if !self.est_vaincu(apotre) {
            self.apotres_vaincus.push(apotre.espece().id().clone());
            self.sac.ajouter_philtres_chromatiques(1);
        }
        self.codex.noter_capture(apotre.espece().id());
    }

    /// Rend `true` si le conseil n'avait encore jamais été montré : c'est le
    /// moment de l'afficher (section 6).
    pub fn noter_conseil_vu(&mut self, conseil: Conseil) -> bool {
        if self.conseils_vus.contains(&conseil) {
            return false;
        }
        self.conseils_vus.push(conseil);
        true
    }

    /// Reprend l'équipe et les soins à la fin du combat, inscrit les
    /// captures au codex et les rend, à placer ensuite avec
    /// `Collection::accueillir_capture`. Une défaite fait perdre tout le sac,
    /// rendu à côté des captures (section 9.3). Les choix d'attaques en
    /// attente se font sur l'équipe du bilan, avant cet appel.
    pub fn conclure_combat(
        &mut self,
        bilan: BilanCombat,
    ) -> Result<(Vec<Monstre>, Inventaire), ErreurCollection> {
        self.collection.actualiser_equipe(bilan.equipe)?;
        self.sac.definir_soins(bilan.soins);
        self.sac.definir_filets(bilan.filets);
        let perdu = if bilan.issue == Some(Issue::Defaite) {
            self.sac.perdre_tout()
        } else {
            Inventaire::default()
        };
        for capture in &bilan.captures {
            self.codex.noter_capture(capture.espece().id());
        }
        Ok((bilan.captures, perdu))
    }

    /// Un monstre aperçu, comme un monstre rare qui n'attaque pas.
    pub fn noter_rencontre(&mut self, monstre: &Monstre) {
        self.codex.noter_rencontre(monstre.espece().id());
    }

    /// Monstre rare non hostile que le joueur choisit de prendre (section
    /// 9.2) : il compte comme capturé.
    pub fn prendre_monstre_rare(&mut self, monstre: Monstre) -> Placement {
        self.codex.noter_capture(monstre.espece().id());
        self.collection.accueillir_capture(monstre)
    }

    /// Des filets trouvés en expédition vont dans le sac (section 11.2).
    pub(crate) fn trouver_filets(&mut self, filet: Filet, nombre: u32) {
        self.sac.ajouter_filets(filet, nombre);
    }

    /// Ce que contient un coffre trouvé en expédition va dans le sac (section
    /// 13.3). Un Pillard de l'équipe qui n'est pas KO ajoute 50 % à son
    /// argent (section 5.9). Rend cet argent en plus.
    pub fn ouvrir_coffre(&mut self, potions: u32, argent: u32, capsule: Option<Capsule>) -> u32 {
        let argent_du_pillard = if self.membre_debout(Habilete::Pillard).is_some() {
            argent * BONUS_DU_PILLARD / 100
        } else {
            0
        };
        self.sac.ajouter_potions(potions);
        self.sac.ajouter_argent(argent + argent_du_pillard);
        if let Some(capsule) = capsule {
            self.sac.ajouter_capsule(capsule);
        }
        argent_du_pillard
    }

    /// Une capsule d'habileté trouvée dans un coffre va dans le sac (section
    /// 5.9).
    pub fn trouver_capsule_d_habilete(&mut self) {
        self.sac.ajouter_capsules_d_habilete(1);
    }

    /// Après un combat gagné, un Fouineur de l'équipe qui n'est pas KO trouve
    /// une potion 1 fois sur 4 ; deux Fouineurs n'en valent qu'un (section
    /// 5.9). La potion va dans le sac. Rend le Fouineur qui l'a trouvée.
    /// Tirage, seulement s'il y a un Fouineur : sa chance.
    pub fn fouiller_apres_une_victoire(
        &mut self,
        aleatoire: &mut dyn Aleatoire,
    ) -> Option<Monstre> {
        let fouineur = self.membre_debout(Habilete::Fouineur)?.clone();
        if aleatoire.tirer(1..=CHANCES_DU_FOUINEUR) != 1 {
            return None;
        }
        self.sac.ajouter_potions(1);
        Some(fouineur)
    }

    /// Après un combat gagné, chaque monstre sauvage mis KO et non capturé
    /// lâche sa ressource 1 fois sur 3 ; elle va dans le sac (section 14.3).
    /// Rend ce qui est trouvé, dans l'ordre. Tirages : un par vaincu dont
    /// l'espèce a une ressource, dans l'ordre du groupe.
    pub fn recolter_ressources(
        &mut self,
        vaincus: &[Monstre],
        aleatoire: &mut dyn Aleatoire,
    ) -> Vec<Ressource> {
        let mut trouvees = Vec::new();
        for ressource in vaincus
            .iter()
            .filter_map(|vaincu| vaincu.espece().ressource())
        {
            if aleatoire.tirer(1..=CHANCES_DE_RESSOURCE) == 1 {
                self.sac.ajouter_ressources(ressource.clone(), 1);
                trouvees.push(ressource.clone());
            }
        }
        trouvees
    }

    /// Au camp, chez Malvoise, sans confirmation : toutes les ressources de
    /// cette sorte, celles du sac puis celles du coffre ; l'argent va dans le
    /// coffre (section 14.3). Rend la ressource, leur nombre et le prix.
    pub fn vendre_ressources(
        &mut self,
        id: &RessourceId,
    ) -> Result<(Ressource, u32, u32), ErreurMarchand> {
        self.verifier_marchand()?;
        let du_sac = self.sac.retirer_toutes_les_ressources(id);
        let du_coffre = self.coffre_du_camp.retirer_toutes_les_ressources(id);
        let (ressource, nombre) = match (du_sac, du_coffre) {
            (Some((ressource, sac)), Some((_, coffre))) => (ressource, sac + coffre),
            (Some(seules), None) | (None, Some(seules)) => seules,
            (None, None) => return Err(ErreurMarchand::RessourceAbsente),
        };
        let prix = nombre.saturating_mul(ressource.prix());
        self.coffre_du_camp.ajouter_argent(prix);
        Ok((ressource, nombre, prix))
    }

    /// Le premier monstre de l'équipe qui a cette habileté et n'est pas KO.
    fn membre_debout(&self, habilete: Habilete) -> Option<&Monstre> {
        self.collection
            .equipe()
            .iter()
            .find(|monstre| !monstre.est_ko() && monstre.a_l_habilete(habilete))
    }

    /// Soin du sac, utilisé entre deux étapes d'une expédition (section
    /// 14.4).
    pub fn utiliser_soin(
        &mut self,
        soin: Soin,
        index_equipe: usize,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<EffetDuSoin, ErreurSoin> {
        let monstre = self
            .collection
            .membre_de_l_equipe_mut(index_equipe)
            .ok_or(ErreurSoin::MonstreInexistant)?;
        self.sac.utiliser_soin(soin, monstre, aleatoire)
    }

    pub fn expeditions_automatiques(&self) -> &[ExpeditionAutomatique] {
        &self.expeditions_automatiques
    }

    pub fn attaques_a_choisir(&self) -> &[AttaqueAChoisir] {
        &self.attaques_a_choisir
    }

    /// La prochaine attaque à choisir, avec le monstre qui veut l'apprendre.
    pub fn attaque_a_choisir(&self) -> Option<(&Monstre, &Attaque)> {
        let a_choisir = self.attaques_a_choisir.first()?;
        let monstre = self.collection.monstre(a_choisir.emplacement)?;
        Some((monstre, &a_choisir.attaque))
    }

    /// Le joueur a fait oublier une attaque au monstre, ou renoncé à la
    /// nouvelle : on passe à la suivante.
    pub(crate) fn attaque_choisie(&mut self) {
        if !self.attaques_a_choisir.is_empty() {
            self.attaques_a_choisir.remove(0);
        }
    }

    pub fn expedition_automatique(&self, zone: &ZoneId) -> Option<&ExpeditionAutomatique> {
        self.expeditions_automatiques
            .iter()
            .find(|expedition| expedition.zone().id() == zone)
    }

    /// Durée et chances de réussite, montrées avant le départ (section
    /// 12.4). Refuse ce que refuserait `envoyer_en_expedition_automatique`.
    pub fn estimer_expedition_automatique(
        &self,
        zone: &Zone,
        emplacements: &[Emplacement],
        protection: Protection,
    ) -> Result<Estimation, ErreurExpeditionAutomatique> {
        self.verifier_depart(zone, emplacements)?;
        self.verifier_protection(protection, emplacements.len())?;
        let monstres: Vec<Monstre> = emplacements
            .iter()
            .filter_map(|emplacement| self.collection.monstre(*emplacement).cloned())
            .collect();
        let parametres = zone.expedition_automatique();
        Ok(Estimation {
            duree: parametres.duree_pour(&monstres),
            chances: parametres.chances_de_reussite(&monstres, protection),
        })
    }

    /// Les monstres quittent l'équipe ou le camp et gardent leur place au
    /// camp, où ils reviendront ; l'équipe peut rester vide (section 12.1).
    pub fn envoyer_en_expedition_automatique(
        &mut self,
        zone: &Zone,
        emplacements: &[Emplacement],
        maintenant: Horodatage,
    ) -> Result<(), ErreurExpeditionAutomatique> {
        self.envoyer_en_expedition_automatique_avec(
            zone,
            emplacements,
            Protection::default(),
            maintenant,
        )
    }

    /// Comme `envoyer_en_expedition_automatique`, avec un monstre protégé,
    /// désigné par sa position dans `emplacements`, et peut-être un charme
    /// de garde, pris du sac d'abord, puis du coffre, et consommé au départ
    /// (section 12.5).
    pub fn envoyer_en_expedition_automatique_avec(
        &mut self,
        zone: &Zone,
        emplacements: &[Emplacement],
        protection: Protection,
        maintenant: Horodatage,
    ) -> Result<(), ErreurExpeditionAutomatique> {
        self.verifier_depart(zone, emplacements)?;
        self.verifier_protection(protection, emplacements.len())?;
        let partis = self
            .collection
            .retirer_pour_expedition(emplacements)
            .map_err(ErreurExpeditionAutomatique::Collection)?;
        if protection.charme && !self.sac.utiliser_charme_de_garde() {
            self.coffre_du_camp.utiliser_charme_de_garde();
        }
        self.expeditions_automatiques
            .push(ExpeditionAutomatique::partir(
                zone.clone(),
                partis,
                protection,
                maintenant,
            ));
        Ok(())
    }

    fn verifier_protection(
        &self,
        protection: Protection,
        nombre_de_monstres: usize,
    ) -> Result<(), ErreurExpeditionAutomatique> {
        verifier_protection(protection, nombre_de_monstres)?;
        if protection.charme && self.charmes_de_garde() == 0 {
            return Err(ErreurExpeditionAutomatique::AucunCharme);
        }
        Ok(())
    }

    /// Avant le terme, les monstres reviennent sans dégâts ni récompense,
    /// dans l'équipe s'il y a de la place, sinon au camp ; une expédition
    /// rentrée s'accueille (section 12.7). Rend leurs places, dans l'ordre de
    /// l'expédition.
    pub fn rappeler_expedition_automatique(
        &mut self,
        zone: &ZoneId,
        maintenant: Horodatage,
    ) -> Result<Vec<Emplacement>, ErreurExpeditionAutomatique> {
        let position = self.position_de_l_expedition(zone)?;
        if self.expeditions_automatiques[position].est_rentree(maintenant) {
            return Err(ErreurExpeditionAutomatique::DejaRentree);
        }
        let monstres = self.expeditions_automatiques.remove(position).rappeler();
        Ok(self.collection.accueillir_retour(monstres))
    }

    /// Tire le résultat, range les potions et l'argent dans le coffre, et
    /// ramène les monstres dans l'équipe s'il y a de la place, sinon au camp
    /// (sections 12.3, 12.6, 12.7 et 13.3). Les
    /// attaques en attente dans les rapports d'XP s'ajoutent aux attaques à
    /// choisir.
    pub fn accueillir_expedition_automatique(
        &mut self,
        zone: &ZoneId,
        maintenant: Horodatage,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<BilanExpeditionAutomatique, ErreurExpeditionAutomatique> {
        let position = self.position_de_l_expedition(zone)?;
        if !self.expeditions_automatiques[position].est_rentree(maintenant) {
            return Err(ErreurExpeditionAutomatique::PasEncoreRentree);
        }
        let expedition = self.expeditions_automatiques.remove(position);
        let places = self
            .collection
            .places_du_retour(expedition.monstres().len());
        let (bilan, monstres) = expedition.resoudre(&places, aleatoire);
        self.coffre_du_camp.ajouter_potions(bilan.potions);
        self.coffre_du_camp.ajouter_argent(bilan.argent);
        self.collection.accueillir_retour(monstres);
        for rentre in &bilan.monstres {
            let attaques = rentre
                .rapport
                .iter()
                .flat_map(|rapport| &rapport.attaques_en_attente);
            for attaque in attaques {
                self.attaques_a_choisir.push(AttaqueAChoisir {
                    emplacement: rentre.emplacement,
                    attaque: attaque.clone(),
                });
            }
        }
        Ok(bilan)
    }

    fn verifier_depart(
        &self,
        zone: &Zone,
        emplacements: &[Emplacement],
    ) -> Result<(), ErreurExpeditionAutomatique> {
        if !self.est_exploree(zone.id()) {
            return Err(ErreurExpeditionAutomatique::ZoneNonExploree);
        }
        if self.expedition_automatique(zone.id()).is_some() {
            return Err(ErreurExpeditionAutomatique::DejaEnCours);
        }
        if !(MONSTRES_MIN..=MONSTRES_MAX).contains(&emplacements.len()) {
            return Err(ErreurExpeditionAutomatique::NombreDeMonstres(
                emplacements.len(),
            ));
        }
        self.collection
            .verifier_depart(emplacements)
            .map_err(ErreurExpeditionAutomatique::Collection)
    }

    fn position_de_l_expedition(
        &self,
        zone: &ZoneId,
    ) -> Result<usize, ErreurExpeditionAutomatique> {
        self.expeditions_automatiques
            .iter()
            .position(|expedition| expedition.zone().id() == zone)
            .ok_or(ErreurExpeditionAutomatique::Inexistante)
    }

    /// Tous les monstres sont soignés (section 13.1) ; ce qui a été trouvé
    /// reste dans le sac. Une expédition menée jusqu'au bout sans défaite
    /// rend la zone explorée et, si l'indice suivant de la piste se trouve
    /// dans cette zone, le révèle et le rend (section 9.4).
    pub fn rentrer_au_camp<'p>(
        &mut self,
        expedition: &Expedition,
        piste: &'p Piste,
    ) -> Option<&'p Indice> {
        self.collection.soigner_tout();
        // L'Éclipse ne compte jamais comme explorée (section 9.4).
        if !expedition.est_reussie() || expedition.zone().est_un_lieu_d_invocation() {
            return None;
        }
        let zone = expedition.zone().id();
        if !self.est_exploree(zone) {
            self.zones_explorees.push(zone.clone());
        }
        let indice = piste
            .indice_suivant(self.indices_trouves, &self.apotres_vaincus)
            .filter(|indice| &indice.zone == zone)?;
        self.indices_trouves += 1;
        Some(indice)
    }
}

/// Tiré comme un monstre sauvage : shiny, valeurs internes, puis habileté
/// (section 14.2).
fn monstre_de_malvoise(offre: &OffreDeMonstre, aleatoire: &mut dyn Aleatoire) -> Monstre {
    let shiny = tirer_shiny(aleatoire);
    let valeurs_internes = tirer_valeurs_internes(shiny, aleatoire);
    // Malvoise propose des espèces de zones, qui s'y créent dès leur
    // niveau minimal : elles connaissent une attaque à tout niveau au-delà.
    Monstre::creer(offre.espece.clone(), offre.niveau, valeurs_internes, shiny)
        .expect("espèce d'une zone, créée au-delà de son niveau minimal")
        .tirer_son_habilete(aleatoire)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::ame::Gardien;
    use crate::attaque::Categorie;
    use crate::codex::EtatCodex;
    use crate::collection::Tailles;
    use crate::combat::Action;
    use crate::donnees_de_test::{attaque, espece, zone};
    use crate::espece::EspeceId;
    use crate::expedition::EvenementExpedition;
    use crate::heros::Classe;
    use crate::nom::Nom;
    use crate::stats::Stats;
    use crate::traque::Chapitre;
    use crate::types::Type;
    use std::time::Duration;

    fn creer(id: &str, niveau: u8) -> Monstre {
        Monstre::creer(
            espece(id),
            Niveau::new(niveau).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    /// Sans les filets du départ, pour que le sac soit vide.
    fn partie() -> Partie {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier);
        let mut partie = Partie::nouvelle(heros, creer("depart", 5));
        partie.sac = Inventaire::default();
        partie
    }

    #[test]
    fn le_heros_part_avec_10_filets_de_chanvre() {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier);
        let partie = Partie::nouvelle(heros, creer("depart", 5));
        assert_eq!(partie.sac().filets().nombre(Filet::Chanvre), 10);
        assert!(partie.coffre_du_camp().est_vide());
    }

    #[test]
    fn les_vaincus_lachent_leur_ressource_que_malvoise_rachete() {
        let croc = Ressource::new(RessourceId::new("croc"), "Croc", 5);
        let loup = Monstre::creer(
            espece("loup").avec_ressource(croc.clone()),
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        let mut partie = partie();
        // Un tirage par vaincu dont l'espèce a une ressource : 1 la lâche.
        let vaincus = [loup.clone(), creer("rat", 5), loup.clone(), loup];
        let trouvees = partie.recolter_ressources(&vaincus, &mut AleatoireScripte::new([1, 2, 1]));
        assert_eq!(trouvees, vec![croc.clone(), croc.clone()]);
        assert_eq!(partie.sac().nombre_de_ressources(croc.id()), 2);
        assert_eq!(
            partie.vendre_ressources(croc.id()),
            Err(ErreurMarchand::AucunMarchand)
        );
        partie.accueillir_malvoise();
        partie
            .ranger_au_coffre(&Objet::Ressources(croc.id().clone(), 1))
            .unwrap();
        // Celles du sac et du coffre, d'un coup ; l'argent va au coffre.
        assert_eq!(
            partie.vendre_ressources(croc.id()),
            Ok((croc.clone(), 2, 10))
        );
        assert!(partie.sac().est_vide());
        assert_eq!(partie.coffre_du_camp().argent(), 10);
        assert_eq!(
            partie.vendre_ressources(croc.id()),
            Err(ErreurMarchand::RessourceAbsente)
        );
    }

    #[test]
    fn une_capsule_d_habilete_donne_l_autre_habilete_de_l_espece() {
        let mut partie = partie();
        let espece = espece("loup").avec_habiletes(vec![Habilete::Pillard, Habilete::Fouineur]);
        let loup = Monstre::creer(
            espece,
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        partie.prendre_monstre_rare(loup);
        let loup = Emplacement::Equipe(1);
        assert_eq!(
            partie.changer_d_habilete(loup),
            Err(ErreurChangementDHabilete::AucuneCapsule)
        );
        // Le monstre de départ n'a qu'une habileté possible, ou aucune.
        assert_eq!(
            partie.estimer_changement_d_habilete(Emplacement::Equipe(0)),
            Err(ErreurChangementDHabilete::AucuneAutreHabilete)
        );
        assert_eq!(
            partie.acheter_capsules_d_habilete(1),
            Err(ErreurMarchand::AucunMarchand)
        );
        partie.accueillir_malvoise();
        partie.coffre_du_camp.ajouter_argent(450);
        assert_eq!(partie.acheter_capsules_d_habilete(2), Ok(400));
        partie.trouver_capsule_d_habilete();
        assert_eq!(partie.capsules_d_habilete(), 3);
        // Le sac d'abord, puis le coffre.
        assert_eq!(
            partie.changer_d_habilete(loup),
            Ok((Some(Habilete::Pillard), Habilete::Fouineur))
        );
        assert_eq!(partie.sac().capsules_d_habilete(), 0);
        assert_eq!(
            partie.changer_d_habilete(loup),
            Ok((Some(Habilete::Fouineur), Habilete::Pillard))
        );
        assert_eq!(partie.coffre_du_camp().capsules_d_habilete(), 1);
        assert!(partie.collection().equipe()[1].a_l_habilete(Habilete::Pillard));
    }

    fn loup_aux_valeurs(pv: u8) -> Monstre {
        let valeurs = ValeursInternes::new(Stats {
            pv,
            ..Stats::default()
        })
        .unwrap();
        Monstre::creer(espece("loup"), Niveau::new(50).unwrap(), valeurs, false).unwrap()
    }

    #[test]
    fn un_booster_ajoute_5_a_une_valeur_interne_jusqu_a_31() {
        let mut partie = partie();
        partie.prendre_monstre_rare(loup_aux_valeurs(24));
        let loup = Emplacement::Equipe(1);
        assert_eq!(
            partie.donner_booster(loup, Booster::FioleDeSang),
            Err(ErreurBooster::Aucun(Booster::FioleDeSang))
        );
        partie.trouver_booster(Booster::FioleDeSang);
        partie
            .coffre_du_camp
            .ajouter_boosters(Booster::FioleDeSang, 2);
        assert_eq!(partie.nombre_de_boosters(Booster::FioleDeSang), 3);
        // Les PV actuels suivent les PV max : 24 de plus en valeur interne,
        // au niveau 50, font 2 PV de plus par tranche de 5.
        partie
            .collection
            .monstre_a_modifier(loup)
            .unwrap()
            .subir_degats(10);
        let avant = partie.collection().monstre(loup).unwrap().clone();
        assert_eq!(
            partie.donner_booster(loup, Booster::FioleDeSang),
            Ok(avant.clone())
        );
        let apres = partie.collection().monstre(loup).unwrap().clone();
        assert_eq!(apres.valeurs_internes().valeurs().pv, 29);
        assert_eq!(apres.valeurs_internes().valeurs().attaque, 0);
        let gain = apres.stats().pv - avant.stats().pv;
        assert!(gain > 0);
        assert_eq!(apres.pv_actuels(), avant.pv_actuels() + gain);
        // Le sac d'abord.
        assert_eq!(partie.sac().nombre_de_boosters(Booster::FioleDeSang), 0);
        // 29 passe à 31, le surplus est perdu ; puis le booster est refusé
        // sans être consommé.
        partie.donner_booster(loup, Booster::FioleDeSang).unwrap();
        assert_eq!(
            partie
                .collection()
                .monstre(loup)
                .unwrap()
                .valeurs_internes()
                .valeurs()
                .pv,
            31
        );
        assert_eq!(
            partie.donner_booster(loup, Booster::FioleDeSang),
            Err(ErreurBooster::ValeurMaximale(Booster::FioleDeSang))
        );
        assert_eq!(partie.nombre_de_boosters(Booster::FioleDeSang), 1);
        assert_eq!(
            partie.donner_booster(Emplacement::Camp(0), Booster::FioleDeSang),
            Err(ErreurBooster::Collection(
                ErreurCollection::EmplacementInexistant(Emplacement::Camp(0))
            ))
        );
    }

    #[test]
    fn malvoise_rend_un_monstre_shiny_contre_un_philtre_chromatique() {
        let mut partie = partie();
        partie.prendre_monstre_rare(loup_aux_valeurs(3));
        let loup = Emplacement::Equipe(1);
        assert_eq!(
            partie.estimer_shiny(loup),
            Err(ErreurMarchand::AucunMarchand)
        );
        partie.accueillir_malvoise();
        assert_eq!(
            partie.rendre_shiny(loup),
            Err(ErreurMarchand::AucunPhiltreChromatique)
        );
        partie.sac.ajouter_philtres_chromatiques(1);
        partie.coffre_du_camp.ajouter_philtres_chromatiques(1);
        assert_eq!(partie.philtres_chromatiques(), 2);
        let avant = partie.collection().monstre(loup).unwrap().clone();
        assert_eq!(partie.estimer_shiny(loup), Ok(avant.clone()));
        assert_eq!(partie.rendre_shiny(loup), Ok(avant.clone()));
        let apres = partie.collection().monstre(loup).unwrap();
        assert!(apres.est_shiny());
        assert_eq!(apres.valeurs_internes(), &ValeursInternes::maximales());
        assert_eq!(apres.pv_actuels(), apres.stats().pv);
        assert_eq!(apres.niveau(), avant.niveau());
        // Le sac d'abord.
        assert_eq!(partie.sac().philtres_chromatiques(), 0);
        assert_eq!(partie.rendre_shiny(loup), Err(ErreurMarchand::DejaShiny));
        assert_eq!(partie.philtres_chromatiques(), 1);
        // Un shiny n'a plus besoin de booster.
        partie.trouver_booster(Booster::PoudreDeVifArgent);
        assert_eq!(
            partie.donner_booster(loup, Booster::PoudreDeVifArgent),
            Err(ErreurBooster::ValeurMaximale(Booster::PoudreDeVifArgent))
        );
    }

    #[test]
    fn un_pillard_debout_ajoute_50_pour_cent_a_l_argent_d_un_coffre() {
        let mut partie = partie();
        partie.prendre_monstre_rare(creer("loup", 5).avec_habilete(Habilete::Pillard));
        assert_eq!(partie.ouvrir_coffre(1, 15, None), 7);
        assert_eq!(partie.sac().argent(), 22);
        // KO, il ne pille plus.
        let mut equipe = partie.collection().equipe().to_vec();
        equipe[1].subir_degats(u32::MAX);
        partie.collection_mut().actualiser_equipe(equipe).unwrap();
        assert_eq!(partie.ouvrir_coffre(0, 10, None), 0);
        assert_eq!(partie.sac().argent(), 32);
    }

    #[test]
    fn l_humain_attend_au_bout_de_sa_zone_jusqu_a_sa_defaite() {
        let brigand = Humain::new(
            HumainId::new("brigand"),
            "Brigand",
            vec![(espece("loup"), Niveau::new(5).unwrap())],
            60,
            "",
            "",
        )
        .unwrap();
        let bois = zone("bois").avec_humain(brigand.clone());
        let (_, piste) = carte();
        let mut partie = partie();
        let expedition = partie.partir_en_expedition(&bois, &piste).unwrap();
        assert_eq!(expedition.humain(), Some(&brigand));
        assert_eq!(partie.vaincre_humain(&brigand), 60);
        assert_eq!(partie.sac().argent(), 60);
        assert_eq!(partie.humains_vaincus(), &[HumainId::new("brigand")]);
        let expedition = partie.partir_en_expedition(&bois, &piste).unwrap();
        assert_eq!(expedition.humain(), None);
    }

    #[test]
    fn un_fouineur_debout_trouve_une_potion_une_fois_sur_4() {
        let mut partie = partie();
        // Sans Fouineur, aucun tirage.
        assert_eq!(
            partie.fouiller_apres_une_victoire(&mut AleatoireScripte::new([])),
            None
        );
        partie.prendre_monstre_rare(creer("loup", 5).avec_habilete(Habilete::Fouineur));
        assert_eq!(
            partie.fouiller_apres_une_victoire(&mut AleatoireScripte::new([2])),
            None
        );
        let trouvee = partie.fouiller_apres_une_victoire(&mut AleatoireScripte::new([1]));
        assert_eq!(
            trouvee.map(|fouineur| fouineur.nom().to_string()),
            Some("loup".to_string())
        );
        assert_eq!(partie.sac().potions(), 1);
    }

    #[test]
    fn le_monstre_de_depart_est_capture_au_codex() {
        let partie = partie();
        assert_eq!(
            partie.codex().etat(&EspeceId::new("depart")),
            EtatCodex::Capturee
        );
        assert_eq!(partie.collection().equipe().len(), 1);
    }

    /// Indices dans le bois, puis deux fois dans le marais, où attend
    /// l'Apôtre ; la zone des cimes n'est sur aucune piste.
    fn carte() -> (Vec<Zone>, Piste) {
        let zones = vec![zone("bois"), zone("marais"), zone("cimes")];
        let indice = |zone: &str| Indice {
            zone: ZoneId::new(zone),
            texte: String::new(),
        };
        let apotre = Apotre::new(espece("baron"), &zones[1], "", "").unwrap();
        let chapitre = Chapitre::new(
            vec![indice("bois"), indice("marais"), indice("marais")],
            apotre,
        )
        .unwrap();
        (zones, Piste::new(vec![chapitre]).unwrap())
    }

    fn ids(zones: &[&Zone]) -> Vec<String> {
        zones
            .iter()
            .map(|zone| zone.id().as_str().to_string())
            .collect()
    }

    /// Expédition de 4 étapes, toutes des coffres.
    fn expedition_complete(partie: &Partie, zone: &Zone, piste: &Piste) -> Expedition {
        let mut expedition = partie.partir_en_expedition(zone, piste).unwrap();
        for _ in 0..4 {
            assert!(matches!(
                expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 100])),
                Some(EvenementExpedition::Coffre { .. })
            ));
        }
        expedition
    }

    #[test]
    fn les_indices_ouvrent_la_piste_jusqu_a_l_antre() {
        let (zones, piste) = carte();
        let mut partie = partie();
        assert_eq!(ids(&partie.zones_accessibles(&zones, &piste)), vec!["bois"]);

        let expedition = expedition_complete(&partie, &zones[0], &piste);
        assert_eq!(
            partie.rentrer_au_camp(&expedition, &piste),
            piste.indices().next()
        );
        assert!(partie.est_exploree(&ZoneId::new("bois")));
        assert_eq!(
            ids(&partie.zones_accessibles(&zones, &piste)),
            vec!["bois", "marais"]
        );
        // L'indice suivant est dans le marais : le bois n'en donne plus.
        let expedition = expedition_complete(&partie, &zones[0], &piste);
        assert_eq!(partie.rentrer_au_camp(&expedition, &piste), None);

        for numero in 2..=3 {
            let expedition = expedition_complete(&partie, &zones[1], &piste);
            assert!(partie.rentrer_au_camp(&expedition, &piste).is_some());
            assert_eq!(partie.indices_trouves(), numero);
        }
        assert_eq!(
            partie.partir_en_expedition(&zones[2], &piste),
            Err(ErreurExpedition::ZoneFermee)
        );
    }

    #[test]
    fn un_coffre_trouve_en_expedition_remplit_le_sac() {
        let (zones, piste) = carte();
        let mut partie = partie();
        partie.ouvrir_coffre(2, 10, Some(capsule("a", Type::Normal)));
        assert_eq!(
            partie.sac(),
            &Inventaire::new(10, 2, vec![capsule("a", Type::Normal)])
        );
        assert!(partie.coffre_du_camp().est_vide());
        assert_eq!(partie.argent(), 10);
        // Ce qui a été trouvé reste dans le sac au retour (section 13.3).
        let expedition = expedition_complete(&partie, &zones[0], &piste);
        partie.rentrer_au_camp(&expedition, &piste);
        assert_eq!(partie.sac().argent(), 10);
        assert!(partie.coffre_du_camp().est_vide());
    }

    #[test]
    fn ranger_et_sortir_du_coffre_au_camp() {
        let mut partie = partie();
        partie.ouvrir_coffre(3, 40, Some(capsule("a", Type::Normal)));
        assert_eq!(
            partie.ranger_au_coffre(&Objet::Soins(Soin::Potion, 4)),
            Err(ErreurObjet::Insuffisant)
        );
        assert_eq!(
            partie.ranger_au_coffre(&Objet::Argent(30)),
            Ok(Inventaire::new(30, 0, Vec::new()))
        );
        assert_eq!(partie.sac().argent(), 10);
        assert_eq!(partie.coffre_du_camp().argent(), 30);
        assert_eq!(
            partie.sortir_du_coffre(&Objet::Argent(5)),
            Ok(Inventaire::new(5, 0, Vec::new()))
        );
        let range = partie.tout_ranger_au_coffre();
        assert_eq!(
            range,
            Inventaire::new(15, 3, vec![capsule("a", Type::Normal)])
        );
        assert!(partie.sac().est_vide());
        assert_eq!(
            partie.coffre_du_camp(),
            &Inventaire::new(40, 3, vec![capsule("a", Type::Normal)])
        );
        // Au camp, une capsule sert où qu'elle soit.
        assert!(partie.possede_capsule(&AttaqueId::new("a")));
    }

    #[test]
    fn l_ame_se_trouve_sans_malvoise_tant_qu_elle_n_est_pas_recueillie() {
        let (mut zones, piste) = carte();
        let bertrand = MarchandMort::new(
            MarchandId::new("bertrand"),
            "Bertrand",
            Soin::Remede,
            Gardien::MonstreDeLaZone,
            "",
            "",
        )
        .unwrap();
        zones[0] = zones[0].clone().avec_marchand_mort(bertrand.clone());
        // Le monstre du groupe, ou celui qui garde Bertrand, tire son shiny
        // et ses valeurs internes.
        let tirages = [71, 2, 0, 0, 0, 0, 0, 0];
        let etape = |partie: &Partie| {
            let mut expedition = partie.partir_en_expedition(&zones[0], &piste).unwrap();
            expedition.etape_suivante(&mut AleatoireScripte::new(tirages))
        };
        let mut partie = partie();
        assert!(matches!(
            etape(&partie),
            Some(EvenementExpedition::Ame { .. })
        ));
        partie.recueillir_ame(bertrand.clone());
        partie.recueillir_ame(bertrand);
        assert_eq!(partie.ames_portees().len(), 1);
        assert!(matches!(
            etape(&partie),
            Some(EvenementExpedition::Combat(_))
        ));
        // Portée, l'âme n'ajoute encore rien à l'étal.
        assert_eq!(partie.soins_en_vente(), vec![Soin::Potion]);
        let bertrand = MarchandId::new("bertrand");
        assert_eq!(
            partie.donner_ame(&bertrand),
            Err(ErreurMarchand::AucunMarchand)
        );
        partie.accueillir_malvoise();
        assert_eq!(partie.donner_ame(&bertrand).unwrap().nom(), "Bertrand");
        assert!(partie.ames_portees().is_empty());
        assert_eq!(partie.ames_donnees().len(), 1);
        assert_eq!(partie.soins_en_vente(), vec![Soin::Potion, Soin::Remede]);
        assert_eq!(
            partie.donner_ame(&bertrand),
            Err(ErreurMarchand::AmeNonPortee)
        );
    }

    #[test]
    fn une_ame_a_retrouver_revient_a_une_etape_tiree() {
        let (mut zones, piste) = carte();
        let grete = MarchandMort::new(
            MarchandId::new("grete"),
            "Grete",
            Soin::GrandePotion,
            Gardien::MonstreDeLaZone,
            "",
            "",
        )
        .unwrap();
        zones[0] = zones[0].clone().avec_marchand_mort(grete.clone());
        let mut partie = partie();
        partie.noter_ame_a_retrouver(grete.id());
        partie.noter_ame_a_retrouver(grete.id());
        assert_eq!(partie.ames_a_retrouver().len(), 1);
        let mut expedition = partie.partir_en_expedition(&zones[0], &piste).unwrap();
        // L'étape de l'âme, tirée parmi celles qui précèdent la dernière (1 à
        // 3 sur 4), puis la première étape, un coffre : sa nature et ses
        // potions.
        let mut aleatoire = AleatoireScripte::new([2, 81, 1, 100]);
        assert!(matches!(
            expedition.etape_suivante(&mut aleatoire),
            Some(EvenementExpedition::Coffre { .. })
        ));
        drop(aleatoire);
        // À la deuxième, sans tirage de sa nature : seulement son gardien.
        let mut aleatoire = AleatoireScripte::new([2, 0, 0, 0, 0, 0, 0]);
        assert!(matches!(
            expedition.etape_suivante(&mut aleatoire),
            Some(EvenementExpedition::Ame { .. })
        ));
        drop(aleatoire);
        partie.recueillir_ame(grete);
        assert!(partie.ames_a_retrouver().is_empty());
    }

    /// Le marchand au camp et 50 pièces d'argent dans le coffre.
    fn partie_chez_le_marchand() -> Partie {
        let mut partie = partie();
        partie.collection_mut().accueillir_capture(creer("loup", 3));
        partie.accueillir_malvoise();
        partie.coffre_du_camp.ajouter_argent(50);
        partie
    }

    #[test]
    fn rien_ne_s_echange_sans_marchand() {
        let mut partie = partie();
        partie.coffre_du_camp.ajouter_argent(50);
        partie.collection_mut().accueillir_capture(creer("loup", 3));
        assert_eq!(
            partie.acheter_soins(Soin::Potion, 1),
            Err(ErreurMarchand::AucunMarchand)
        );
        assert_eq!(
            partie.vendre(Emplacement::Equipe(1)),
            Err(ErreurMarchand::AucunMarchand)
        );
        assert_eq!(partie.collection().equipe().len(), 2);
    }

    #[test]
    fn acheter_des_potions_dans_la_limite_de_son_argent() {
        let mut partie = partie_chez_le_marchand();
        assert_eq!(
            partie.acheter_soins(Soin::Potion, 0),
            Err(ErreurMarchand::QuantiteNulle)
        );
        assert_eq!(
            partie.acheter_soins(Soin::Potion, 3),
            Err(ErreurMarchand::ArgentInsuffisant {
                prix: 60,
                argent: 50
            })
        );
        assert_eq!(partie.coffre_du_camp().potions(), 0);
        assert_eq!(partie.acheter_soins(Soin::Potion, 2), Ok(40));
        assert_eq!(partie.coffre_du_camp().potions(), 2);
        assert_eq!(partie.argent(), 10);
    }

    #[test]
    fn malvoise_vend_les_filets_de_chanvre_et_lestes_qui_vont_au_coffre() {
        let mut partie = partie();
        assert!(partie.filets_en_vente().is_empty());
        assert_eq!(
            partie.acheter_filets(Filet::Chanvre, 1),
            Err(ErreurMarchand::AucunMarchand)
        );
        let mut partie = partie_chez_le_marchand();
        assert_eq!(partie.filets_en_vente(), vec![Filet::Chanvre, Filet::Leste]);
        assert_eq!(
            partie.acheter_filets(Filet::Chaines, 1),
            Err(ErreurMarchand::FiletIndisponible(Filet::Chaines))
        );
        assert_eq!(
            partie.acheter_filets(Filet::Leste, 0),
            Err(ErreurMarchand::QuantiteNulle)
        );
        assert_eq!(
            partie.acheter_filets(Filet::Leste, 2),
            Err(ErreurMarchand::ArgentInsuffisant {
                prix: 60,
                argent: 50
            })
        );
        partie.ouvrir_coffre(0, 15, None);
        assert_eq!(partie.acheter_filets(Filet::Chanvre, 2), Ok(20));
        assert_eq!(partie.acheter_filets(Filet::Leste, 1), Ok(30));
        // Le sac paie d'abord, comme pour les soins.
        assert_eq!(partie.sac().argent(), 0);
        assert_eq!(partie.coffre_du_camp().argent(), 15);
        assert!(partie.sac().filets().est_vide());
        assert_eq!(partie.coffre_du_camp().filets().nombre(Filet::Chanvre), 2);
        assert_eq!(partie.coffre_du_camp().filets().nombre(Filet::Leste), 1);
        assert_eq!(partie.filets().nombre(Filet::Leste), 1);
    }

    #[test]
    fn le_sac_paie_avant_le_coffre_et_les_gains_vont_au_coffre() {
        let mut partie = partie_chez_le_marchand();
        partie.ouvrir_coffre(0, 15, None);
        assert_eq!(partie.argent(), 65);
        assert_eq!(partie.acheter_soins(Soin::Potion, 1), Ok(20));
        assert_eq!(partie.sac().argent(), 0);
        assert_eq!(partie.coffre_du_camp().argent(), 45);
        assert_eq!(partie.coffre_du_camp().potions(), 1);
        partie.vendre(Emplacement::Equipe(1)).unwrap();
        assert!(partie.sac().est_vide());
        assert!(partie.coffre_du_camp().argent() > 45);
    }

    #[test]
    fn vendre_un_monstre_comme_on_le_relache() {
        let mut partie = partie_chez_le_marchand();
        // Un Loup de niveau 3, d'espèce commune et de base 300 : 3 x 300 / 50.
        let (estime, prix) = partie.estimer_vente(Emplacement::Equipe(1)).unwrap();
        assert_eq!(prix, 18);
        assert_eq!(partie.collection().equipe().len(), 2);
        assert_eq!(partie.vendre(Emplacement::Equipe(1)), Ok((estime, 18)));
        assert_eq!(partie.argent(), 68);
        assert_eq!(partie.collection().equipe().len(), 1);
        let dernier = Err(ErreurMarchand::Collection(ErreurCollection::DernierMonstre));
        assert_eq!(partie.estimer_vente(Emplacement::Equipe(0)), dernier);
        assert_eq!(partie.vendre(Emplacement::Equipe(0)), dernier);
        assert_eq!(partie.argent(), 68);
    }

    fn capsule(id: &str, type_attaque: Type) -> Capsule {
        Capsule::new(attaque(id, type_attaque, Categorie::Physique, 40, 100), 150)
    }

    #[test]
    fn acheter_une_capsule_une_seule_fois() {
        let mut partie = partie_chez_le_marchand();
        let coup = capsule("grand-coup", Type::Normal);
        assert_eq!(
            partie.acheter_capsule(&coup),
            Err(ErreurMarchand::ArgentInsuffisant {
                prix: 150,
                argent: 50
            })
        );
        partie.coffre_du_camp.ajouter_argent(150);
        assert_eq!(partie.acheter_capsule(&coup), Ok(150));
        assert_eq!(partie.argent(), 50);
        assert_eq!(
            partie.acheter_capsule(&coup),
            Err(ErreurMarchand::CapsuleDejaPossedee)
        );
        assert_eq!(partie.coffre_du_camp().capsules(), &[coup]);
    }

    #[test]
    fn apprendre_au_camp_une_attaque_de_capsule() {
        let mut partie = partie_chez_le_marchand();
        partie.coffre_du_camp.ajouter_argent(1_000);
        let loup = Emplacement::Equipe(1);
        // Les espèces de test sont de type Normal.
        partie.acheter_capsule(&capsule("feu", Type::Feu)).unwrap();
        assert!(partie.attaques_a_apprendre(loup).unwrap().is_empty());
        assert_eq!(
            partie.apprendre_attaque(loup, &AttaqueId::new("feu"), None),
            Err(ErreurApprentissage::AttaqueInaccessible)
        );
        for id in ["a", "b", "c", "d"] {
            partie.acheter_capsule(&capsule(id, Type::Normal)).unwrap();
        }
        assert_eq!(partie.attaques_a_apprendre(loup).unwrap().len(), 4);
        assert_eq!(
            partie.apprendre_attaque(loup, &AttaqueId::new("a"), Some(0)),
            Err(ErreurApprentissage::OubliInutile)
        );
        for id in ["a", "b", "c"] {
            assert_eq!(
                partie.apprendre_attaque(loup, &AttaqueId::new(id), None),
                Ok(None)
            );
        }
        // Il connaît coup, a, b et c : il doit en oublier une.
        assert_eq!(
            partie.apprendre_attaque(loup, &AttaqueId::new("d"), None),
            Err(ErreurApprentissage::OubliManquant)
        );
        assert_eq!(
            partie.apprendre_attaque(loup, &AttaqueId::new("d"), Some(4)),
            Err(ErreurApprentissage::Attaque(
                ErreurAttaquesMonstre::PositionInexistante(4)
            ))
        );
        let oubliee = partie
            .apprendre_attaque(loup, &AttaqueId::new("d"), Some(0))
            .unwrap()
            .unwrap();
        assert_eq!(oubliee.id().as_str(), "coup");
        // Coup, appris par son espèce au niveau 1, se réapprend.
        let a_apprendre = partie.attaques_a_apprendre(loup).unwrap();
        assert_eq!(a_apprendre.len(), 1);
        assert_eq!(a_apprendre[0].id().as_str(), "coup");
        assert_eq!(
            partie.attaques_a_apprendre(Emplacement::Camp(0)),
            Err(ErreurCollection::EmplacementInexistant(Emplacement::Camp(
                0
            )))
        );
    }

    #[test]
    fn un_coffre_ne_contient_que_des_capsules_nouvelles() {
        let (zones, piste) = carte();
        let mut zones = zones;
        let niveau = Niveau::new(5).unwrap();
        zones[0] = Zone::new(
            ZoneId::new("bois"),
            "bois",
            (niveau, niveau),
            4,
            1,
            vec![espece("loup")],
            vec![],
            crate::donnees_de_test::parametres_expedition_automatique(),
            crate::expedition::ParametresCoffre::new(
                10..=10,
                vec![capsule("a", Type::Normal), capsule("b", Type::Normal)],
            )
            .unwrap(),
        )
        .unwrap();
        let mut partie = partie();
        partie
            .coffre_du_camp
            .ajouter_capsule(capsule("a", Type::Normal));
        let mut expedition = partie.partir_en_expedition(&zones[0], &piste).unwrap();
        // Un coffre, où seule la capsule b est nouvelle.
        let Some(EvenementExpedition::Coffre {
            capsule: Some(trouvee),
            ..
        }) = expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 20, 100]))
        else {
            panic!("un coffre avec une capsule était attendu");
        };
        assert_eq!(trouvee.id().as_str(), "b");
    }

    #[test]
    fn rentrer_avant_la_fin_ne_donne_pas_d_indice() {
        let (zones, piste) = carte();
        let mut partie = partie();
        let mut expedition = partie.partir_en_expedition(&zones[0], &piste).unwrap();
        expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 100]));
        assert_eq!(partie.rentrer_au_camp(&expedition, &piste), None);
        assert!(!partie.est_exploree(&ZoneId::new("bois")));
        assert_eq!(partie.indices_trouves(), 0);
    }

    #[test]
    fn l_apotre_attend_au_bout_de_l_expedition_tant_qu_il_n_est_pas_vaincu() {
        let (zones, piste) = carte();
        let mut partie = partie();
        partie.indices_trouves = 3;
        let mut expedition = partie.partir_en_expedition(&zones[1], &piste).unwrap();
        for _ in 0..3 {
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 100]));
        }
        let Some(EvenementExpedition::Apotre(apotre)) =
            expedition.etape_suivante(&mut AleatoireScripte::new([2]))
        else {
            panic!("l'Apôtre devait attendre à la dernière étape");
        };
        let combat = partie.preparer_combat_contre_apotre(apotre).unwrap();
        assert!(combat.est_contre_un_apotre());
        assert!(!combat.est_en_deux_phases());

        let baron = piste.apotres().next().unwrap();
        partie.noter_apotre_vaincu(baron);
        partie.noter_apotre_vaincu(baron);
        assert_eq!(partie.apotres_vaincus(), &[EspeceId::new("baron")]);
        let expedition = expedition_complete(&partie, &zones[1], &piste);
        assert_eq!(partie.rentrer_au_camp(&expedition, &piste), None);
    }

    #[test]
    fn chaque_conseil_n_est_montre_qu_une_fois() {
        let mut partie = partie();
        assert!(partie.noter_conseil_vu(Conseil::PremierCombat));
        assert!(!partie.noter_conseil_vu(Conseil::PremierCombat));
        assert!(partie.noter_conseil_vu(Conseil::PremierRetourAuCamp));
        assert_eq!(
            partie.conseils_vus(),
            &[Conseil::PremierCombat, Conseil::PremierRetourAuCamp]
        );
    }

    #[test]
    fn monstre_de_depart_au_niveau_5() {
        // Pas shiny : ses valeurs internes sont tirées.
        let tirages = [2, 0, 1, 2, 3, 4, 5];
        let depart =
            creer_monstre_de_depart(espece("depart"), &mut AleatoireScripte::new(tirages)).unwrap();
        assert_eq!(depart.niveau(), Niveau::new(5).unwrap());
        assert!(!depart.est_shiny());
        assert_eq!(depart.valeurs_internes().valeurs().vitesse, 5);
    }

    #[test]
    fn un_monstre_de_depart_shiny_a_31_partout_sans_tirage() {
        let depart =
            creer_monstre_de_depart(espece("depart"), &mut AleatoireScripte::new([1])).unwrap();
        assert!(depart.est_shiny());
        assert_eq!(depart.valeurs_internes(), &ValeursInternes::maximales());
    }

    #[test]
    fn combat_note_les_rencontres_et_les_captures_et_rend_les_potions() {
        let mut partie = partie();
        partie.ouvrir_coffre(2, 0, None);
        partie.trouver_filets(Filet::Chanvre, 10);
        let mut loup = creer("loup", 5);
        loup.subir_degats(loup.stats().pv - 2);
        let mut combat = partie.preparer_combat(vec![loup]).unwrap();
        assert_eq!(combat.soins().nombre(Soin::Potion), 2);
        assert_eq!(
            partie.codex().etat(&EspeceId::new("loup")),
            EtatCodex::Rencontree
        );
        combat
            .jouer_tour(
                Action::LancerFilet(Filet::Chanvre),
                &mut AleatoireScripte::new([90]),
            )
            .unwrap();
        let bilan = combat.terminer(&mut AleatoireScripte::new([100]));
        let (captures, perdu) = partie.conclure_combat(bilan).unwrap();
        assert_eq!(captures.len(), 1);
        assert!(perdu.est_vide());
        // Le filet lancé est consommé (section 11.2).
        assert_eq!(partie.sac().filets().nombre(Filet::Chanvre), 9);
        assert_eq!(
            partie.codex().etat(&EspeceId::new("loup")),
            EtatCodex::Capturee
        );
        assert_eq!(partie.sac().potions(), 2);
    }

    #[test]
    fn une_defaite_fait_perdre_tout_le_sac_mais_pas_le_coffre() {
        let mut partie = partie();
        partie.ouvrir_coffre(3, 25, Some(capsule("a", Type::Normal)));
        partie.coffre_du_camp.ajouter_argent(100);
        let combat = partie.preparer_combat(vec![creer("loup", 50)]).unwrap();
        let mut bilan = combat.terminer(&mut AleatoireScripte::new([]));
        bilan.issue = Some(Issue::Defaite);
        let (_, perdu) = partie.conclure_combat(bilan).unwrap();
        assert_eq!(
            perdu,
            Inventaire::new(25, 3, vec![capsule("a", Type::Normal)])
        );
        assert!(partie.sac().est_vide());
        assert_eq!(partie.coffre_du_camp().argent(), 100);
    }

    #[test]
    fn soin_entre_deux_etapes() {
        let mut partie = partie();
        partie.ouvrir_coffre(2, 0, None);
        assert_eq!(
            partie.utiliser_soin(Soin::Potion, 3, &mut AleatoireScripte::new([])),
            Err(ErreurSoin::MonstreInexistant)
        );
        // Sur un monstre qui a tous ses PV, la potion est perdue (Q53).
        assert_eq!(
            partie.utiliser_soin(Soin::Potion, 0, &mut AleatoireScripte::new([])),
            Ok(EffetDuSoin::PvRendus(0))
        );
        assert_eq!(partie.sac().potions(), 1);
        let mut equipe = partie.collection().equipe().to_vec();
        equipe[0].subir_degats(10);
        partie.collection_mut().actualiser_equipe(equipe).unwrap();
        assert_eq!(
            partie.utiliser_soin(Soin::Potion, 0, &mut AleatoireScripte::new([50])),
            Ok(EffetDuSoin::PvRendus(10))
        );
        assert_eq!(partie.sac().potions(), 0);
    }

    #[test]
    fn le_marchand_ne_vend_que_les_soins_debloques() {
        let mut partie = partie_chez_le_marchand();
        assert_eq!(partie.soins_en_vente(), vec![Soin::Potion]);
        assert_eq!(
            partie.acheter_soins(Soin::Elixir, 1),
            Err(ErreurMarchand::SoinIndisponible(Soin::Elixir))
        );
        assert_eq!(partie.argent(), 50);
    }

    #[test]
    fn prendre_un_monstre_rare_le_capture() {
        let mut partie = partie();
        let placement = partie.prendre_monstre_rare(creer("cerf-blanc", 5));
        assert_eq!(placement, Placement::Equipe(1));
        assert_eq!(
            partie.codex().etat(&EspeceId::new("cerf-blanc")),
            EtatCodex::Capturee
        );
    }

    /// Le monstre de départ et un loup de niveau 5, de 20 PV chacun, avec
    /// le bois déjà exploré.
    fn partie_prete_pour_le_bois() -> Partie {
        let mut partie = partie();
        partie.collection_mut().accueillir_capture(creer("loup", 5));
        partie.zones_explorees.push(ZoneId::new("bois"));
        partie
    }

    fn a(secondes: u64) -> Horodatage {
        Horodatage::depuis_secondes(secondes)
    }

    #[test]
    fn un_charme_de_garde_part_avec_un_monstre_protege() {
        let bois = zone("bois");
        let mut partie = partie();
        partie.collection_mut().accueillir_capture(creer("loup", 5));
        partie.zones_explorees.push(ZoneId::new("bois"));
        let deux = [Emplacement::Equipe(0), Emplacement::Equipe(1)];
        let avec_charme = Protection {
            protege: Some(1),
            charme: true,
        };
        assert_eq!(
            partie.envoyer_en_expedition_automatique_avec(&bois, &deux, avec_charme, a(0)),
            Err(ErreurExpeditionAutomatique::AucunCharme)
        );
        assert_eq!(
            partie.estimer_expedition_automatique(
                &bois,
                &deux[..1],
                Protection {
                    protege: Some(0),
                    charme: false
                }
            ),
            Err(ErreurExpeditionAutomatique::ProtectionImpossible)
        );
        partie.sac.ajouter_charmes_de_garde(1);
        partie.coffre_du_camp.ajouter_charmes_de_garde(1);
        assert_eq!(partie.charmes_de_garde(), 2);
        partie
            .envoyer_en_expedition_automatique_avec(&bois, &deux, avec_charme, a(0))
            .unwrap();
        // Le sac d'abord.
        assert_eq!(partie.sac().charmes_de_garde(), 0);
        assert_eq!(partie.charmes_de_garde(), 1);
        let expedition = partie.expedition_automatique(&ZoneId::new("bois")).unwrap();
        assert_eq!(expedition.protection(), avec_charme);
    }

    #[test]
    fn gunther_forge_un_charme_de_garde_qui_va_au_coffre() {
        let ressource = |id: &str| Ressource::new(RessourceId::new(id), id, 5);
        let plan =
            PlanDuCharme::new(vec![(ressource("os"), 3)], Duration::from_secs(1_800)).unwrap();
        let mut partie = partie();
        assert_eq!(
            partie.verifier_charme(&plan),
            Err(ErreurAmelioration::AucunArtisan)
        );
        partie.forge.accueillir_gunther();
        assert!(matches!(
            partie.verifier_charme(&plan),
            Err(ErreurAmelioration::RessourcesManquantes(_))
        ));
        partie.coffre_du_camp.ajouter_ressources(ressource("os"), 7);
        let chantier = partie.lancer_charme(&plan, a(0)).unwrap();
        assert_eq!(chantier.ouvrage, Ouvrage::CharmeDeGarde);
        assert_eq!(chantier.fin, a(1_800));
        assert_eq!(
            partie
                .coffre_du_camp()
                .nombre_de_ressources(&RessourceId::new("os")),
            4
        );
        partie.constater_travail(a(1_800)).unwrap();
        assert_eq!(partie.coffre_du_camp().charmes_de_garde(), 1);
        // Il se forge aussi souvent que le joueur le veut.
        assert!(partie.lancer_charme(&plan, a(2_000)).is_ok());
    }

    #[test]
    fn gunther_forge_un_lot_de_filets_qui_va_au_coffre() {
        let ressource = |id: &str| Ressource::new(RessourceId::new(id), id, 5);
        let plan = PlanDesFilets::new(
            Filet::Chaines,
            5,
            vec![(ressource("fer"), 4)],
            Duration::from_secs(3_600),
        )
        .unwrap();
        let mut partie = partie();
        assert_eq!(
            partie.verifier_filets(&plan),
            Err(ErreurAmelioration::AucunArtisan)
        );
        partie.forge.accueillir_gunther();
        assert!(matches!(
            partie.verifier_filets(&plan),
            Err(ErreurAmelioration::RessourcesManquantes(_))
        ));
        partie
            .coffre_du_camp
            .ajouter_ressources(ressource("fer"), 9);
        let chantier = partie.lancer_filets(&plan, a(0)).unwrap();
        assert_eq!(
            chantier.ouvrage,
            Ouvrage::Filets {
                filet: Filet::Chaines,
                nombre: 5
            }
        );
        assert_eq!(chantier.fin, a(3_600));
        assert_eq!(
            partie
                .coffre_du_camp()
                .nombre_de_ressources(&RessourceId::new("fer")),
            5
        );
        // Un travail à la fois.
        assert_eq!(
            partie.verifier_filets(&plan),
            Err(ErreurAmelioration::ChantierEnCours)
        );
        partie.constater_travail(a(3_600)).unwrap();
        assert_eq!(partie.coffre_du_camp().filets().nombre(Filet::Chaines), 5);
        assert!(partie.lancer_filets(&plan, a(4_000)).is_ok());
    }

    #[test]
    fn une_expedition_automatique_part_dans_une_zone_exploree() {
        let bois = zone("bois");
        let mut partie = partie();
        partie.collection_mut().accueillir_capture(creer("loup", 5));
        let loup = [Emplacement::Equipe(1)];
        assert_eq!(
            partie.envoyer_en_expedition_automatique(&bois, &loup, a(0)),
            Err(ErreurExpeditionAutomatique::ZoneNonExploree)
        );
        partie.zones_explorees.push(ZoneId::new("bois"));
        assert_eq!(
            partie.envoyer_en_expedition_automatique(&bois, &[], a(0)),
            Err(ErreurExpeditionAutomatique::NombreDeMonstres(0))
        );
        // Toute l'équipe peut partir : elle reste alors vide (section 12.1).
        assert!(
            partie
                .estimer_expedition_automatique(
                    &bois,
                    &[Emplacement::Equipe(0), Emplacement::Equipe(1)],
                    Protection::default()
                )
                .is_ok()
        );

        // 20 PV contre 10 à 40 dégâts : 10 totaux sur 31 laissent le loup
        // debout.
        let estimation = partie
            .estimer_expedition_automatique(&bois, &loup, Protection::default())
            .unwrap();
        assert_eq!(estimation.duree, Duration::from_secs(3_600));
        assert_eq!(estimation.chances.pourcentage(), 32);

        partie
            .envoyer_en_expedition_automatique(&bois, &loup, a(0))
            .unwrap();
        assert_eq!(partie.collection().equipe().len(), 1);
        assert_eq!(partie.collection().places_reservees(), 1);
        let expedition = partie.expedition_automatique(&ZoneId::new("bois")).unwrap();
        assert_eq!(expedition.retour(), a(3_600));
        assert_eq!(
            partie.estimer_expedition_automatique(
                &bois,
                &[Emplacement::Equipe(0)],
                Protection::default()
            ),
            Err(ErreurExpeditionAutomatique::DejaEnCours)
        );
    }

    #[test]
    fn toute_l_equipe_part_et_la_laisse_vide() {
        let (_, piste) = carte();
        let mut partie = partie_prete_pour_le_bois();
        partie
            .envoyer_en_expedition_automatique(
                &zone("bois"),
                &[Emplacement::Equipe(0), Emplacement::Equipe(1)],
                a(0),
            )
            .unwrap();
        assert!(partie.collection().equipe().is_empty());
        assert!(partie.collection().camp().is_empty());
        assert_eq!(partie.collection().places_reservees(), 2);
        assert_eq!(
            partie.partir_en_expedition(&zone("bois"), &piste),
            Err(ErreurExpedition::EquipeVide)
        );
        // Les monstres partis comptent : la partie se recharge.
        assert_eq!(Partie::restaurer(etat(&partie)), Ok(partie.clone()));
    }

    #[test]
    fn accueillir_une_expedition_rentree() {
        let mut partie = partie_prete_pour_le_bois();
        partie
            .envoyer_en_expedition_automatique(&zone("bois"), &[Emplacement::Equipe(1)], a(0))
            .unwrap();
        let bois = ZoneId::new("bois");
        assert_eq!(
            partie.accueillir_expedition_automatique(
                &bois,
                a(3_599),
                &mut AleatoireScripte::new([])
            ),
            Err(ErreurExpeditionAutomatique::PasEncoreRentree)
        );
        // 15 dégâts sur 20 PV : réussite, 100 XP, une potion et 20 pièces
        // d'argent.
        let bilan = partie
            .accueillir_expedition_automatique(
                &bois,
                a(3_600),
                &mut AleatoireScripte::new([15, 100, 1]),
            )
            .unwrap();
        assert!(bilan.reussite);
        // L'équipe a de la place : le monstre la rejoint (section 12.7).
        assert_eq!(bilan.monstres[0].emplacement, Emplacement::Equipe(1));
        assert_eq!(partie.collection().equipe()[1].experience(), 125 + 100);
        assert!(partie.collection().camp().is_empty());
        assert_eq!(partie.collection().places_reservees(), 0);
        // Ce qui est gagné au camp va dans le coffre (section 13.3) : 20
        // pièces d'argent, et 10 % de plus pour le Loup, offensif, debout.
        assert_eq!(partie.coffre_du_camp().potions(), 1);
        assert_eq!(partie.coffre_du_camp().argent(), 22);
        assert!(partie.sac().est_vide());
        assert!(partie.expeditions_automatiques().is_empty());
        assert_eq!(
            partie.accueillir_expedition_automatique(
                &bois,
                a(3_600),
                &mut AleatoireScripte::new([])
            ),
            Err(ErreurExpeditionAutomatique::Inexistante)
        );
    }

    #[test]
    fn rappeler_une_expedition_avant_son_terme() {
        let mut partie = partie_prete_pour_le_bois();
        partie
            .envoyer_en_expedition_automatique(&zone("bois"), &[Emplacement::Equipe(1)], a(0))
            .unwrap();
        let bois = ZoneId::new("bois");
        assert_eq!(
            partie.rappeler_expedition_automatique(&bois, a(3_600)),
            Err(ErreurExpeditionAutomatique::DejaRentree)
        );
        assert_eq!(
            partie.rappeler_expedition_automatique(&bois, a(10)),
            Ok(vec![Emplacement::Equipe(1)])
        );
        assert!(partie.expeditions_automatiques().is_empty());
        assert!(partie.collection().camp().is_empty());
        assert_eq!(partie.collection().equipe()[1].experience(), 125);
        assert_eq!(partie.collection().places_reservees(), 0);
        assert!(partie.coffre_du_camp().est_vide());
    }

    fn etat(partie: &Partie) -> EtatPartie {
        EtatPartie {
            heros: partie.heros().clone(),
            collection: partie.collection().clone(),
            sac: partie.sac().clone(),
            coffre_du_camp: partie.coffre_du_camp().clone(),
            marchand_au_camp: partie.marchand_au_camp(),
            ames_donnees: partie.ames_donnees().to_vec(),
            ames_portees: partie.ames_portees().to_vec(),
            ames_a_retrouver: partie.ames_a_retrouver().to_vec(),
            codex: partie.codex().clone(),
            zones_explorees: partie.zones_explorees().to_vec(),
            indices_trouves: partie.indices_trouves(),
            apotres_vaincus: partie.apotres_vaincus().to_vec(),
            humains_vaincus: partie.humains_vaincus().to_vec(),
            conseils_vus: partie.conseils_vus().to_vec(),
            expeditions_automatiques: partie.expeditions_automatiques().to_vec(),
            attaques_a_choisir: partie.attaques_a_choisir().to_vec(),
            peche: partie.peche().clone(),
            forge: partie.forge().clone(),
            relations: partie.relations().clone(),
            machines: partie.machines().clone(),
            beherit: partie.beherit(),
            records_de_la_tour: partie.records_de_la_tour(),
        }
    }

    #[test]
    fn restaurer_reverifie_les_expeditions_automatiques() {
        let mut partie = partie_prete_pour_le_bois();
        partie
            .envoyer_en_expedition_automatique(&zone("bois"), &[Emplacement::Equipe(1)], a(0))
            .unwrap();
        // Rechargée telle quelle, la partie garde l'expédition et la place
        // réservée au camp.
        assert_eq!(Partie::restaurer(etat(&partie)), Ok(partie.clone()));

        let mut sans_exploration = etat(&partie);
        sans_exploration.zones_explorees.clear();
        assert_eq!(
            Partie::restaurer(sans_exploration),
            Err(ErreurRestaurationPartie::ZoneNonExploree(ZoneId::new(
                "bois"
            )))
        );

        let mut en_double = etat(&partie);
        let copie = en_double.expeditions_automatiques[0].clone();
        en_double.expeditions_automatiques.push(copie);
        assert_eq!(
            Partie::restaurer(en_double),
            Err(ErreurRestaurationPartie::ExpeditionEnDouble(ZoneId::new(
                "bois"
            )))
        );

        let mut sans_monstre = etat(&partie);
        sans_monstre.attaques_a_choisir.push(AttaqueAChoisir {
            emplacement: Emplacement::Camp(0),
            attaque: creer("loup", 5).attaques().liste()[0].clone(),
        });
        assert_eq!(
            Partie::restaurer(sans_monstre),
            Err(ErreurRestaurationPartie::AttaqueAChoisirSansMonstre(
                Emplacement::Camp(0)
            ))
        );

        // Le monstre parti a besoin de sa place dans un camp déjà plein.
        let mut camp_plein = etat(&partie);
        let equipe = camp_plein.collection.equipe().to_vec();
        let camp = (0..Tailles::DE_DEPART.camp)
            .map(|_| creer("loup", 5))
            .collect();
        camp_plein.collection = Collection::restaurer(equipe, camp, Tailles::DE_DEPART).unwrap();
        assert_eq!(
            Partie::restaurer(camp_plein),
            Err(ErreurRestaurationPartie::Collection(
                ErreurCollection::CampPlein
            ))
        );

        let mut capsule_en_double = etat(&partie);
        capsule_en_double
            .sac
            .ajouter_capsule(capsule("a", Type::Normal));
        capsule_en_double
            .coffre_du_camp
            .ajouter_capsule(capsule("a", Type::Normal));
        assert_eq!(
            Partie::restaurer(capsule_en_double),
            Err(ErreurRestaurationPartie::CapsuleEnDouble(AttaqueId::new(
                "a"
            )))
        );
    }

    /// Malvoise au camp, la Lisière et le Marais menés au bout et la Vallée
    /// ouverte (niveaux 14 à 20), de quoi payer (section 14.2).
    fn chez_malvoise() -> (Partie, crate::contenu::ContenuV1) {
        let contenu = crate::contenu::ContenuV1::new();
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier);
        let depart = Monstre::creer(
            contenu.espece(&EspeceId::new("crapaud")).unwrap(),
            Niveau::new(12).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        let mut partie = Partie::nouvelle(heros, depart);
        partie.marchand_au_camp = true;
        partie.zones_explorees = vec![ZoneId::new("lisiere"), ZoneId::new("marais")];
        partie.indices_trouves = 2;
        partie.coffre_du_camp.ajouter_argent(5_000);
        (partie, contenu)
    }

    /// Ni shiny, des valeurs internes de 1 à 6, la première habileté.
    const TIRAGES_D_UN_MONSTRE: [u32; 8] = [2, 1, 2, 3, 4, 5, 6, 0];

    #[test]
    fn le_carnet_de_traque_suit_l_ordre_des_victoires() {
        let (mut partie, contenu) = chez_malvoise();
        let piste = contenu.piste();
        let invocation = contenu.invocation();
        assert!(partie.carnet_de_traque(piste, invocation).is_empty());
        let apotres: Vec<&Apotre> = piste.apotres().collect();
        // Dans un autre ordre que celui de la piste, pour vérifier que le
        // carnet suit les victoires.
        partie.noter_apotre_vaincu(apotres[1]);
        partie.noter_apotre_vaincu(apotres[0]);
        let noms = |partie: &Partie| -> Vec<String> {
            partie
                .carnet_de_traque(piste, invocation)
                .iter()
                .map(|apotre| apotre.espece().nom().to_string())
                .collect()
        };
        assert_eq!(noms(&partie), vec!["Abbesse écorchée", "Baron pendu"]);
        partie.noter_apotre_vaincu(apotres[2]);
        partie.noter_demiurge_vaincu(invocation.demiurge());
        assert_eq!(
            noms(&partie),
            vec![
                "Abbesse écorchée",
                "Baron pendu",
                "Margrave de givre",
                "Démiurge"
            ]
        );
    }

    #[test]
    fn malvoise_vend_les_communes_et_loue_les_rares_des_zones_explorees() {
        let (partie, contenu) = chez_malvoise();
        let offres = partie.offres_d_achat(contenu.zones(), contenu.especes());
        let achats: Vec<(&str, u8, u32)> = offres
            .iter()
            .map(|offre| {
                (
                    offre.espece.id().as_str(),
                    offre.niveau.valeur(),
                    offre.prix,
                )
            })
            .collect();
        // 4 fois la revente : niveau x total des stats / 50 x rareté.
        assert_eq!(
            achats,
            vec![
                ("loup", 3, 68),
                ("corbeau", 3, 68),
                ("vipere", 3, 68),
                ("feu-follet", 8, 204),
                ("armure-hantee", 8, 208),
            ]
        );
        let offres = partie.offres_de_location(contenu.zones(), contenu.piste());
        let locations: Vec<(&str, u8, u32)> = offres
            .iter()
            .map(|offre| {
                (
                    offre.espece.id().as_str(),
                    offre.niveau.valeur(),
                    offre.prix,
                )
            })
            .collect();
        assert_eq!(
            locations,
            vec![("cerf-blanc", 20, 320), ("basilic", 20, 320)]
        );
    }

    #[test]
    fn les_formes_evoluees_sauvages_du_col_ne_se_vendent_pas() {
        let (mut partie, contenu) = chez_malvoise();
        partie.zones_explorees = vec![ZoneId::new("col-du-givre-noir")];
        let offres = partie.offres_d_achat(contenu.zones(), contenu.especes());
        let achats: Vec<&str> = offres
            .iter()
            .map(|offre| offre.espece.id().as_str())
            .collect();
        assert_eq!(
            achats,
            vec![
                "wendigo",
                "troll-des-cavernes",
                "harpie-des-orages",
                "gargouille"
            ]
        );
    }

    #[test]
    fn un_monstre_achete_est_au_joueur() {
        let (mut partie, contenu) = chez_malvoise();
        let mut aleatoire = AleatoireScripte::new(TIRAGES_D_UN_MONSTRE);
        let (monstre, placement, prix) = partie
            .acheter_monstre(
                &EspeceId::new("loup"),
                contenu.zones(),
                contenu.especes(),
                &mut aleatoire,
            )
            .unwrap();
        assert_eq!(prix, 68);
        assert_eq!(partie.argent(), 5_000 - 68);
        assert_eq!(placement, Placement::Equipe(1));
        assert_eq!(monstre.niveau().valeur(), 3);
        assert!(!monstre.est_shiny() && !monstre.est_loue());
        assert_eq!(monstre.valeurs_internes().valeurs().vitesse, 6);
        assert_eq!(
            partie.codex().etat(&EspeceId::new("loup")),
            crate::codex::EtatCodex::Capturee
        );
        assert_eq!(
            partie.acheter_monstre(
                &EspeceId::new("cerf-blanc"),
                contenu.zones(),
                contenu.especes(),
                &mut AleatoireScripte::new([]),
            ),
            Err(ErreurMarchand::MonstreIndisponible)
        );
    }

    #[test]
    fn un_monstre_loue_ne_fait_que_combattre_puis_repart() {
        let (mut partie, contenu) = chez_malvoise();
        let louer = |partie: &mut Partie| {
            partie.louer_monstre(
                &EspeceId::new("cerf-blanc"),
                contenu.zones(),
                contenu.piste(),
                &mut AleatoireScripte::new(TIRAGES_D_UN_MONSTRE),
            )
        };
        let (monstre, placement, prix) = louer(&mut partie).unwrap();
        assert_eq!((placement, prix), (Placement::Equipe(1), 320));
        assert_eq!(monstre.expeditions_de_location(), Some(3));
        assert_eq!(
            partie.codex().etat(&EspeceId::new("cerf-blanc")),
            crate::codex::EtatCodex::Rencontree
        );
        assert_eq!(
            partie.louer_monstre(
                &EspeceId::new("basilic"),
                contenu.zones(),
                contenu.piste(),
                &mut AleatoireScripte::new([]),
            ),
            Err(ErreurMarchand::DejaUnMonstreLoue)
        );
        let loue = Emplacement::Equipe(1);
        let refus = ErreurCollection::MonstreLoue;
        assert_eq!(
            partie.vendre(loue),
            Err(ErreurMarchand::Collection(refus.clone()))
        );
        assert_eq!(
            partie.collection_mut().renommer(loue, None),
            Err(refus.clone())
        );
        assert_eq!(
            partie.estimer_shiny(loue),
            Err(ErreurMarchand::Collection(refus.clone()))
        );
        assert_eq!(
            partie.donner_booster(loue, Booster::FioleDeSang),
            Err(ErreurBooster::Collection(refus.clone()))
        );
        assert_eq!(
            partie.estimer_evolution(loue, &contenu),
            Err(ErreurEvolution::Collection(refus.clone()))
        );
        // Le monstre loué ne compte pas : le Crapaud reste le dernier.
        assert_eq!(
            partie.vendre(Emplacement::Equipe(0)),
            Err(ErreurMarchand::Collection(ErreurCollection::DernierMonstre))
        );
        // Trois retours d'expédition manuelle, puis il repart.
        assert_eq!(partie.decompter_location(), None);
        assert_eq!(partie.decompter_location(), None);
        let parti = partie.decompter_location().unwrap();
        assert_eq!(parti.espece().id().as_str(), "cerf-blanc");
        assert_eq!(partie.collection().equipe().len(), 1);
        assert_eq!(partie.decompter_location(), None);
        // Une nouvelle location, rendue plus tôt, sans remboursement.
        louer(&mut partie).unwrap();
        assert_eq!(
            partie
                .rendre_le_monstre_loue()
                .unwrap()
                .espece()
                .id()
                .as_str(),
            "cerf-blanc"
        );
        assert_eq!(partie.argent(), 5_000 - 2 * 320);
        assert_eq!(
            partie.rendre_le_monstre_loue(),
            Err(ErreurMarchand::AucunMonstreLoue)
        );
    }

    #[test]
    fn le_beherit_se_forge_des_ames_de_tous_les_apotres() {
        let (mut partie, contenu) = chez_malvoise();
        let piste = contenu.piste();
        assert_eq!(
            partie.forger_le_beherit(piste),
            Err(ErreurBeherit::AmesManquantes)
        );
        partie.apotres_vaincus = piste
            .apotres()
            .map(|apotre| apotre.espece().id().clone())
            .collect();
        assert_eq!(partie.ames_d_apotres().len(), 3);
        partie.marchand_au_camp = false;
        assert_eq!(
            partie.forger_le_beherit(piste),
            Err(ErreurBeherit::AucunMarchand)
        );
        partie.marchand_au_camp = true;
        assert_eq!(partie.forger_le_beherit(piste), Ok(()));
        assert!(partie.ames_d_apotres().is_empty());
        assert_eq!(
            partie.forger_le_beherit(piste),
            Err(ErreurBeherit::DejaForge)
        );
    }

    #[test]
    fn le_beherit_ne_prend_qu_un_monstre_de_niveau_30_qui_ne_vide_pas_l_equipe() {
        let (mut partie, contenu) = chez_malvoise();
        let creer = |id: &str, niveau: u8| {
            Monstre::creer(
                contenu.espece(&EspeceId::new(id)).unwrap(),
                Niveau::new(niveau).unwrap(),
                ValeursInternes::default(),
                false,
            )
            .unwrap()
        };
        let seul = Emplacement::Equipe(0);
        assert_eq!(
            partie.estimer_sacrifice(seul),
            Err(ErreurBeherit::AucunBeherit)
        );
        partie.beherit = EtatDuBeherit::Forge;
        // Le Crapaud de départ, de niveau 12, est trop faible.
        assert_eq!(
            partie.estimer_sacrifice(seul),
            Err(ErreurBeherit::NiveauInsuffisant)
        );
        partie.collection = Collection::new(creer("crapaud", 40));
        assert_eq!(
            partie.estimer_sacrifice(seul),
            Err(ErreurBeherit::EquipeVide)
        );
        partie
            .collection
            .accueillir_capture(creer("cerf-blanc", 36).louer(3));
        assert_eq!(
            partie.estimer_sacrifice(Emplacement::Equipe(1)),
            Err(ErreurBeherit::Collection(ErreurCollection::MonstreLoue))
        );
        partie.collection.accueillir_capture(creer("loup", 30));
        assert_eq!(
            partie.monstres_sacrifiables(),
            vec![seul, Emplacement::Equipe(2)]
        );
        let sacrifie = partie.sacrifier(seul).unwrap();
        assert_eq!(sacrifie.espece().id().as_str(), "crapaud");
        assert_eq!(partie.beherit(), EtatDuBeherit::EclipseOuverte);
        assert_eq!(
            partie.estimer_sacrifice(Emplacement::Equipe(1)),
            Err(ErreurBeherit::SacrificeDejaFait)
        );
        let expedition = partie.partir_vers_l_eclipse(contenu.invocation()).unwrap();
        assert_eq!(expedition.nombre_etapes(), 1);
    }
}
