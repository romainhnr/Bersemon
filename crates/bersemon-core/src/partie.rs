//! Partie en cours : le héros et tout ce que le joueur possède ou sait. C'est
//! ce qu'une sauvegarde devra conserver.

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::ame::{MarchandId, MarchandMort};
use crate::amelioration::{
    Chantier, ErreurAmelioration, Forge, Ouvrage, PlanDAmelioration, PlanDeFabrication,
};
use crate::attaque::{Attaque, AttaqueId, ErreurAttaquesMonstre};
use crate::booster::{Booster, ErreurBooster};
use crate::capsule::{Capsule, ErreurApprentissage, attaques_a_apprendre};
use crate::catalogue::Catalogue;
use crate::codex::Codex;
use crate::collection::{Collection, Emplacement, ErreurCollection, Placement};
use crate::combat::{BilanCombat, Combat, ErreurCombat, Issue};
use crate::equipement::{ErreurEquipement, ObjetDEquipementId, PlaceDEquipement};
use crate::espece::{Espece, EspeceId};
use crate::evolution::{
    Circonstances, ErreurEvolution, ResultatEvolution, evolution_possible, faire_evoluer,
};
use crate::expedition::{ErreurExpedition, Expedition, RencontreDeLAme};
use crate::expedition_automatique::{
    BilanExpeditionAutomatique, ErreurExpeditionAutomatique, Estimation, ExpeditionAutomatique,
    MONSTRES_MAX, MONSTRES_MIN,
};
use crate::habilete::{
    BONUS_DU_PILLARD, CHANCES_DU_FOUINEUR, ErreurChangementDHabilete, Habilete, autre_habilete,
};
use crate::heros::Heros;
use crate::horodatage::Horodatage;
use crate::humain::{Humain, HumainId, IaHumain};
use crate::inventaire::{ErreurObjet, Inventaire, Objet};
use crate::marchand::{
    ErreurMarchand, PRIX_CAPSULE_D_HABILETE, PRIX_PIERRE_D_EVOLUTION, prix_de_revente, prix_du_soin,
};
use crate::monstre::{Monstre, tirer_shiny, tirer_valeurs_internes};
use crate::niveau::Niveau;
use crate::nom::Nom;
use crate::obeissance::limite_d_obeissance;
use crate::peche::{ErreurPeche, ParametresPeche, Peche, Prise};
use crate::pierre::PierreDEvolution;
use crate::relations::Relations;
use crate::ressource::{Ressource, RessourceId};
use crate::soin::{EffetDuSoin, ErreurSoin, Soin};
use crate::stats::ValeursInternes;
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
        }
    }
}

impl std::error::Error for ErreurRestaurationPartie {}

impl Partie {
    /// Le monstre de départ compte comme capturé dans le codex (Q54).
    pub fn nouvelle(heros: Heros, depart: Monstre) -> Self {
        let mut codex = Codex::default();
        codex.noter_capture(depart.espece().id());
        Self {
            heros,
            collection: Collection::new(depart),
            sac: Inventaire::default(),
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
            Prise::EclatDAstre => self.coffre_du_camp.ajouter_eclats_d_astre(1),
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
        let monstre =
            self.collection
                .monstre(emplacement)
                .ok_or(ErreurChangementDHabilete::Collection(
                    ErreurCollection::EmplacementInexistant(emplacement),
                ))?;
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
            .monstre(emplacement)
            .ok_or(ErreurEvolution::Collection(
                ErreurCollection::EmplacementInexistant(emplacement),
            ))?;
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
            .monstre(emplacement)
            .ok_or(ErreurBooster::Collection(
                ErreurCollection::EmplacementInexistant(emplacement),
            ))?
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
    pub fn eclats_d_astre(&self) -> u32 {
        self.sac
            .eclats_d_astre()
            .saturating_add(self.coffre_du_camp.eclats_d_astre())
    }

    /// Le monstre que Malvoise rendrait shiny, sans rien changer : de quoi
    /// demander confirmation. Refuse ce que refuserait `rendre_shiny`.
    pub fn estimer_shiny(&self, emplacement: Emplacement) -> Result<Monstre, ErreurMarchand> {
        self.verifier_marchand()?;
        let monstre = self
            .collection
            .monstre(emplacement)
            .ok_or(ErreurMarchand::Collection(
                ErreurCollection::EmplacementInexistant(emplacement),
            ))?;
        if monstre.est_shiny() {
            return Err(ErreurMarchand::DejaShiny);
        }
        if self.eclats_d_astre() == 0 {
            return Err(ErreurMarchand::AucunEclatDAstre);
        }
        Ok(monstre.clone())
    }

    /// Au camp, chez Malvoise, après confirmation (section 13.7) : le
    /// monstre devient shiny, et un Éclat d'astre est consommé, du sac
    /// d'abord, puis du coffre. Rend le monstre d'avant.
    pub fn rendre_shiny(&mut self, emplacement: Emplacement) -> Result<Monstre, ErreurMarchand> {
        let monstre = self.estimer_shiny(emplacement)?;
        if !self.sac.utiliser_eclat_d_astre() {
            self.coffre_du_camp.utiliser_eclat_d_astre();
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
    /// du sac et du coffre, rien ne la fait perdre. Elles serviront à
    /// affronter le Démiurge (section 9.4, Q92).
    pub fn ames_d_apotres(&self) -> &[EspeceId] {
        &self.apotres_vaincus
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
        let partis = etat
            .expeditions_automatiques
            .iter()
            .map(|expedition| expedition.monstres().len())
            .sum();
        collection
            .definir_places_reservees(partis)
            .map_err(ErreurRestaurationPartie::Collection)?;
        if collection.nombre_de_monstres() == 0 {
            return Err(ErreurRestaurationPartie::AucunMonstre);
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
        let combat = Combat::new(self.collection.equipe().to_vec(), groupe)?
            .avec_bonus_du_joueur(self.heros.bonus_en_combat())
            .avec_ordre(self.heros.classe().ordre())
            .avec_filets(self.forge.ameliorations().facteur_des_filets())
            .avec_soins(*self.sac.soins());
        for espece in &especes {
            self.codex.noter_rencontre(espece);
        }
        Ok(combat)
    }

    /// Comme `preparer_combat`, sans fuite ni filet (section 9.4).
    pub fn preparer_combat_contre_apotre(
        &mut self,
        apotre: Monstre,
    ) -> Result<Combat, ErreurCombat> {
        self.preparer_combat(vec![apotre])
            .map(Combat::contre_un_apotre)
    }

    /// Contre un marchand relevé d'entre les morts : fuite possible, mais ni
    /// filet ni capture, et il n'entre pas au codex (section 14.2).
    pub fn preparer_combat_sans_capture(
        &mut self,
        groupe: Vec<Monstre>,
    ) -> Result<Combat, ErreurCombat> {
        Ok(Combat::new(self.collection.equipe().to_vec(), groupe)?
            .avec_bonus_du_joueur(self.heros.bonus_en_combat())
            .avec_ordre(self.heros.classe().ordre())
            .avec_soins(*self.sac.soins())
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
    /// (section 9.4). Il laisse aussi un Éclat d'astre, qui va dans le sac
    /// (section 14.3).
    pub fn noter_apotre_vaincu(&mut self, apotre: &Apotre) {
        if !self.est_vaincu(apotre) {
            self.apotres_vaincus.push(apotre.espece().id().clone());
            self.sac.ajouter_eclats_d_astre(1);
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
    ) -> Result<Estimation, ErreurExpeditionAutomatique> {
        self.verifier_depart(zone, emplacements)?;
        let monstres: Vec<Monstre> = emplacements
            .iter()
            .filter_map(|emplacement| self.collection.monstre(*emplacement).cloned())
            .collect();
        let parametres = zone.expedition_automatique();
        Ok(Estimation {
            duree: parametres.duree_pour(&monstres),
            chances: parametres.chances_de_reussite(&monstres),
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
        self.verifier_depart(zone, emplacements)?;
        let partis = self
            .collection
            .retirer_pour_expedition(emplacements)
            .map_err(ErreurExpeditionAutomatique::Collection)?;
        self.expeditions_automatiques
            .push(ExpeditionAutomatique::partir(
                zone.clone(),
                partis,
                maintenant,
            ));
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
        if !expedition.est_reussie() {
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

    fn partie() -> Partie {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier);
        Partie::nouvelle(heros, creer("depart", 5))
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
    fn malvoise_rend_un_monstre_shiny_contre_un_eclat_d_astre() {
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
            Err(ErreurMarchand::AucunEclatDAstre)
        );
        partie.sac.ajouter_eclats_d_astre(1);
        partie.coffre_du_camp.ajouter_eclats_d_astre(1);
        assert_eq!(partie.eclats_d_astre(), 2);
        let avant = partie.collection().monstre(loup).unwrap().clone();
        assert_eq!(partie.estimer_shiny(loup), Ok(avant.clone()));
        assert_eq!(partie.rendre_shiny(loup), Ok(avant.clone()));
        let apres = partie.collection().monstre(loup).unwrap();
        assert!(apres.est_shiny());
        assert_eq!(apres.valeurs_internes(), &ValeursInternes::maximales());
        assert_eq!(apres.pv_actuels(), apres.stats().pv);
        assert_eq!(apres.niveau(), avant.niveau());
        // Le sac d'abord.
        assert_eq!(partie.sac().eclats_d_astre(), 0);
        assert_eq!(partie.rendre_shiny(loup), Err(ErreurMarchand::DejaShiny));
        assert_eq!(partie.eclats_d_astre(), 1);
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
        let mut loup = creer("loup", 5);
        loup.subir_degats(loup.stats().pv - 2);
        let mut combat = partie.preparer_combat(vec![loup]).unwrap();
        assert_eq!(combat.soins().nombre(Soin::Potion), 2);
        assert_eq!(
            partie.codex().etat(&EspeceId::new("loup")),
            EtatCodex::Rencontree
        );
        combat
            .jouer_tour(Action::LancerFilet, &mut AleatoireScripte::new([90]))
            .unwrap();
        let bilan = combat.terminer(&mut AleatoireScripte::new([100]));
        let (captures, perdu) = partie.conclure_combat(bilan).unwrap();
        assert_eq!(captures.len(), 1);
        assert!(perdu.est_vide());
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
                    &[Emplacement::Equipe(0), Emplacement::Equipe(1)]
                )
                .is_ok()
        );

        // 20 PV contre 10 à 40 dégâts : 10 totaux sur 31 laissent le loup
        // debout.
        let estimation = partie.estimer_expedition_automatique(&bois, &loup).unwrap();
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
            partie.estimer_expedition_automatique(&bois, &[Emplacement::Equipe(0)]),
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
        // Ce qui est gagné au camp va dans le coffre (section 13.3).
        assert_eq!(partie.coffre_du_camp().potions(), 1);
        assert_eq!(partie.coffre_du_camp().argent(), 20);
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
}
