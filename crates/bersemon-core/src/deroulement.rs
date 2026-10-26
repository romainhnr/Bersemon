//! Déroulement d'une partie, commun à la console et au bot Discord : il
//! enchaîne les règles du noyau d'une décision du joueur à la suivante.
//!
//! Il n'affiche rien et ne lit rien. Il dit ce qu'il attend (`Attente`),
//! reçoit une décision complète (`Choix`), applique les règles et rend ce qui
//! s'est passé (`Evenement`), que l'interface met en forme. La navigation
//! (sous-menus, retour en arrière, carte, liste des monstres, codex) reste à
//! l'interface. Un choix refusé ne change rien.
//!
//! Une expédition en cours ne vit qu'en mémoire : la partie n'est sauvegardée
//! qu'au camp (section 13.9).

use std::collections::VecDeque;
use std::fmt;
use std::time::Duration;

use crate::aleatoire::Aleatoire;
use crate::ame::{Gardien, MarchandId, MarchandMort};
use crate::amelioration::{
    Amelioration, Chantier, ErreurAmelioration, PlanDAmelioration, PlanDeFabrication,
    PlanDesFilets, PlanDuCharme,
};
use crate::arene::{AdversaireDeLArene, ErreurArene, PALIERS_DE_LA_FOSSE};
use crate::attaque::{Attaque, AttaqueId, ErreurAttaquesMonstre};
use crate::beherit::ErreurBeherit;
use crate::booster::{Booster, ErreurBooster};
use crate::capsule::{Capsule, ErreurApprentissage};
use crate::collection::{Emplacement, ErreurCollection, Placement, TriDuCamp};
use crate::combat::{self, Action, BilanCombat, Combat, Cote, ErreurCombat, Issue, Phase};
use crate::contenu::ContenuV1;
use crate::echange::{ErreurEchange, Offre, conclure_offre};
use crate::equipement::{ErreurEquipement, ObjetDEquipement, ObjetDEquipementId, PlaceDEquipement};
use crate::espece::{Espece, EspeceId};
use crate::evolution::{Circonstances, ErreurEvolution, evolution_possible, faire_evoluer};
use crate::expedition::{ErreurExpedition, EvenementExpedition, Expedition, Tresor};
use crate::expedition_automatique::{
    BilanExpeditionAutomatique, ErreurExpeditionAutomatique, ExpeditionAutomatique, Protection,
};
use crate::filet::Filet;
use crate::habilete::{ErreurChangementDHabilete, Habilete};
use crate::heros::{Classe, Heros, Ordre};
use crate::horodatage::Horodatage;
use crate::inventaire::{ErreurObjet, Inventaire, Objet};
use crate::jour_et_nuit::JourOuNuit;
use crate::machines::ErreurMachine;
use crate::marchand::ErreurMarchand;
use crate::meteo::Meteo;
use crate::monstre::{Monstre, RapportExperience, tirer_shiny};
use crate::nom::{ErreurNom, Nom};
use crate::partie::{Partie, RecompenseObtenue, creer_monstre_de_depart};
use crate::peche::{ErreurPeche, Pecheur, Prise};
use crate::pierre::PierreDEvolution;
use crate::recompenses_du_codex::{ErreurCodex, RecompenseDuCodex};
use crate::relations::IdJoueur;
use crate::ressource::{Ressource, RessourceId};
use crate::rite::{ApercuDuRite, ErreurRite, NaissanceDuRite};
use crate::sauvegarde::{ErreurSauvegarde, SauvegardeRepository};
use crate::soin::{EffetDuSoin, ErreurSoin, Soin, Soins, soin_sans_effet, verifier_cible};
use crate::traque::Piste;
use crate::tutoriel::Conseil;
use crate::zone::{Zone, ZoneId};

/// En expédition, l'équipe garde toujours un monstre en état de combattre
/// entre deux combats : sinon une défaite l'aurait ramenée au camp.
const EQUIPE_APTE: &str = "en expédition, l'équipe a un monstre en état de combattre";

/// Ce que le déroulement reçoit de l'extérieur à chaque décision. Il n'en
/// garde rien : un déroulement se range ainsi facilement, un par joueur.
pub struct Contexte<'a> {
    pub contenu: &'a ContenuV1,
    pub aleatoire: &'a mut dyn Aleatoire,
    pub depot: &'a dyn SauvegardeRepository,
    /// L'heure du choix, que le noyau ne lit jamais lui-même, pour les
    /// expéditions automatiques (section 12.2).
    pub maintenant: Horodatage,
}

/// Décision complète du joueur, sans étape de navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choix {
    ReprendreLaPartie,
    /// Remplace la partie sauvegardée, après confirmation (Q56).
    NouvellePartie,
    NomDuHeros(String),
    /// La classe donne aussi le monstre de départ (section 6).
    Classe(Classe),
    PartirEnExpedition(ZoneId),
    /// Les monstres choisis, de 1 à 3, partent seuls (section 12.1).
    /// L'interface a montré auparavant la durée et les chances de réussite
    /// (`Partie::estimer_expedition_automatique`, section 12.4).
    EnvoyerEnExpeditionAutomatique {
        zone: ZoneId,
        monstres: Vec<Emplacement>,
        /// Le monstre protégé, par sa position dans `monstres`, et le charme
        /// de garde (section 12.5).
        protection: Protection,
    },
    /// Une expédition rentrée, accueillie au camp (section 12.7).
    AccueillirExpeditionAutomatique(ZoneId),
    /// Avant son terme, après confirmation (section 12.7).
    RappelerExpeditionAutomatique(ZoneId),
    /// Au camp, chez le marchand, sans confirmation : un soin de
    /// `Partie::soins_en_vente` (section 14.2).
    AcheterSoins(Soin, u32),
    /// Au camp, chez Malvoise, sans confirmation : un filet de
    /// `Partie::filets_en_vente` (section 14.2).
    AcheterFilets(Filet, u32),
    /// Au camp, chez le marchand, après confirmation, comme un relâcher
    /// (section 14.2).
    Vendre(Emplacement),
    /// Au camp, chez le marchand, sans confirmation : une capsule de
    /// `ContenuV1::capsules` (section 14.2).
    AcheterCapsule(AttaqueId),
    /// Au camp, chez Malvoise, sans confirmation : une âme que le joueur
    /// porte (section 14.2).
    DonnerAme(MarchandId),
    /// Au camp, chez Malvoise, sans confirmation : une espèce de
    /// `Partie::offres_d_achat` (section 14.2).
    AcheterMonstre(EspeceId),
    /// Au camp, chez Malvoise, sans confirmation : une espèce de
    /// `Partie::offres_de_location` (section 14.2).
    LouerMonstre(EspeceId),
    /// Au camp, après confirmation, sans remboursement (section 14.2).
    RendreLeMonstreLoue,
    /// Au camp, chez Malvoise, après confirmation : les âmes des Apôtres
    /// deviennent le Béhérit pourpre (section 9.4).
    ForgerLeBeherit,
    /// Au camp, après confirmation, le premier départ vers l'Éclipse : le
    /// Béhérit prend ce monstre pour toujours, et l'expédition part
    /// (section 9.4).
    Sacrifier(Emplacement),
    /// Au camp, chez Malvoise, sans confirmation (section 5.9).
    AcheterCapsulesDHabilete(u32),
    /// Au camp, chez Malvoise, sans confirmation (section 5.4).
    AcheterPierres(PierreDEvolution, u32),
    /// Au camp, chez Malvoise, sans confirmation : toutes les ressources de
    /// cette sorte, du sac et du coffre (section 14.3).
    VendreRessources(RessourceId),
    /// Au camp, après confirmation : une capsule d'habileté fait passer ce
    /// monstre à l'autre habileté de son espèce (section 5.9).
    ChangerDHabilete(Emplacement),
    /// Au camp, après confirmation, pour un monstre qui remplit la condition
    /// de son évolution (section 5.4).
    FaireEvoluer(Emplacement),
    /// Au camp, sans confirmation : le booster augmente une valeur interne
    /// de ce monstre (section 14.3).
    DonnerBooster {
        emplacement: Emplacement,
        booster: Booster,
    },
    /// Au camp, chez Malvoise, après confirmation : un Philtre chromatique rend ce
    /// monstre shiny (section 13.7).
    RendreShiny(Emplacement),
    /// Au camp, chez Malvoise, après confirmation : un Philtre chromatique
    /// rend le héros shiny (sections 4.1 et 13.7).
    RendreLeHerosShiny,
    /// Au camp, après confirmation : Gunther construit le niveau suivant de
    /// cette amélioration (section 13.4).
    LancerAmelioration(Amelioration),
    /// Au camp, après confirmation : Gunther forge cet objet d'équipement
    /// (section 13.5).
    LancerFabrication(ObjetDEquipementId),
    /// Au camp, après confirmation : Gunther forge un charme de garde
    /// (section 13.5).
    FabriquerCharme,
    /// Au camp, après confirmation : Gunther forge un lot de filets de
    /// cette qualité (section 13.5).
    FabriquerFilets(Filet),
    /// Au camp : le joueur constate le travail fini de Gunther.
    ConstaterTravail,
    /// Au camp, sans confirmation : Ansgar tend une ligne (section 13.8).
    LancerLaLigne,
    /// Au camp, une fois la ligne prête.
    ReleverLaLigne,
    /// Répond au monstre pêché : il rejoint l'équipe ou le camp.
    GarderLaPrise,
    /// Répond au monstre pêché, sans confirmation : il retourne à l'eau.
    RelacherLaPrise,
    /// Au camp, sans confirmation : les pièges se posent dans cette zone
    /// explorée, après avoir été relevés s'ils ont un stock (section 13.6).
    PoserLesPieges(ZoneId),
    /// Au camp : leur stock va dans le coffre.
    ReleverLesPieges,
    /// Au camp, sans confirmation : le monstre entre dans la cour
    /// d'entraînement.
    MettreALEntrainement(Emplacement),
    /// Au camp : le monstre à cette position de la cour en sort, avec l'XP
    /// gagnée.
    ReprendreDeLEntrainement(usize),
    /// Au camp, chez Malvoise, après confirmation : ces deux monstres d'une
    /// même famille lui sont confiés pour le Rite du sang mêlé (section
    /// 13.11).
    LancerLeRite {
        premier: Emplacement,
        second: Emplacement,
    },
    /// Au camp, le rite fini : le petit naît, et ses parents reviennent.
    AccueillirLePetit,
    /// Au camp, depuis le codex, sans confirmation : la récompense du
    /// premier palier atteint et pas encore réclamé (section 5.6).
    ReclamerRecompenseDuCodex,
    /// Au camp, sans confirmation : l'objet remplace celui de cette place
    /// (section 4.3).
    Equiper {
        place: PlaceDEquipement,
        objet: ObjetDEquipementId,
    },
    /// Au camp, sans confirmation.
    RetirerEquipement(PlaceDEquipement),
    /// Au camp : du sac vers le coffre (section 13.3).
    RangerAuCoffre(Objet),
    /// Au camp : du coffre vers le sac (section 13.3).
    SortirDuCoffre(Objet),
    /// Au camp : tout le sac rejoint le coffre.
    ToutRangerAuCoffre,
    /// Au camp, après confirmation : la sauvegarde est effacée et une
    /// nouvelle partie commence (section 13.9).
    SeSuicider,
    /// Au camp, gratuitement : une attaque de `Partie::attaques_a_apprendre`.
    /// S'il en connaît déjà 4, le monstre oublie celle en position
    /// `oubliee`, que l'interface a fait choisir au joueur (section 13.10).
    ApprendreAttaque {
        emplacement: Emplacement,
        attaque: AttaqueId,
        oubliee: Option<usize>,
    },
    /// Au camp, toutes les actions ; entre deux étapes, seulement l'ordre de
    /// l'équipe (Q55).
    Gerer(ActionDeGestion),
    ContinuerLExpedition,
    /// Le soin et la position, dans l'équipe, du monstre à soigner entre
    /// deux étapes (section 14.4).
    UtiliserSoin(Soin, usize),
    RentrerAuCamp,
    /// Quand le sac déborde, à la fin d'une étape : l'objet jeté est perdu
    /// (section 13.3).
    Jeter(Objet),
    PrendreLeMonstreRare,
    LaisserLeMonstreRare,
    Combat(Action),
    /// En combat, avant l'action du tour : le héros donne son ordre au
    /// monstre actif, une fois par combat (section 4.2).
    DonnerOrdre,
    /// Position, dans l'équipe, du monstre envoyé à la place du monstre KO.
    Remplacer(usize),
    /// Position, dans le groupe adverse, du monstre KO à capturer.
    Capturer(usize),
    NePlusCapturer,
    /// Position de l'attaque oubliée pour apprendre la nouvelle.
    OublierAttaque(usize),
    RenoncerALAttaque,
    /// Répond à l'évolution proposée après un combat (section 5.4).
    Evoluer,
    NePasEvoluer,
    RelacherPourLaCapture(Emplacement),
    RelacherLaCapture,
    /// Répond à une annonce : combattre, ou lire l'indice (section 9.2).
    Poursuivre,
    /// Au camp, affronter le prochain palier de la Fosse aux damnés
    /// (section 9.8) : son adversaire est présenté d'abord.
    EntrerDansLaFosse,
    /// Dans la Fosse aux damnés, le héros jette son arme : c'est une défaite,
    /// sans conséquence (section 9.8).
    Abandonner,
    Oui,
    Non,
}

/// Ce qu'annonce un texte suivi d'un bouton, pour que le joueur ne le rate
/// pas en enchaînant les actions (section 9.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Annonce {
    /// Un humain de fin de zone ou un Apôtre : bouton "Combattre".
    Combat,
    /// Au bout d'une expédition qui révèle un indice : bouton "Lire
    /// l'indice".
    Indice,
}

/// Gestion des monstres (section 13.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionDeGestion {
    ChangerLOrdre {
        de: usize,
        vers: usize,
    },
    EnvoyerAuCamp(usize),
    FaireVenir(usize),
    Echanger {
        equipe: usize,
        camp: usize,
    },
    /// Un surnom vide efface le surnom.
    Renommer {
        emplacement: Emplacement,
        surnom: String,
    },
    /// Définitif, après confirmation.
    Relacher(Emplacement),
    /// L'ordre du camp change pour de bon.
    TrierLeCamp(TriDuCamp),
}

/// La question posée au joueur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attente<'a> {
    Reprise(&'a Partie),
    SauvegardeIllisible(&'a ErreurSauvegarde),
    NomDuHeros,
    Classe,
    Camp,
    EntreDeuxEtapes,
    /// Monstre rare pacifique, que le joueur prend ou laisse (section 9.2).
    MonstreRare(&'a Monstre),
    ActionDeCombat(&'a Combat),
    Remplacement(&'a Combat),
    /// Après une victoire, parmi `Combat::adversaires_capturables` (Q29).
    CaptureDesKo(&'a Combat),
    OubliAttaque {
        monstre: &'a Monstre,
        nouvelle: &'a Attaque,
    },
    /// Après un combat, le joueur fait évoluer ce monstre ou repousse son
    /// évolution (section 5.4).
    Evolution {
        monstre: &'a Monstre,
        vers: &'a Espece,
    },
    /// L'équipe et le camp sont pleins : le joueur relâche un de ses monstres
    /// pour garder la capture, ou relâche la capture (Q12).
    PlaceManquante(&'a Monstre),
    /// Au camp, le monstre pêché, que le joueur garde ou relâche (section
    /// 13.8).
    Prise(&'a Monstre),
    /// Le joueur poursuit par `Choix::Poursuivre`.
    Annonce(Annonce),
    Confirmation(&'a Confirmation),
    /// En expédition en groupe, le combat commun, que mène l'expédition en
    /// groupe (`groupe::ExpeditionEnGroupe`, section 15.5).
    CombatDeGroupe,
    /// En expédition en groupe, le joueur veut continuer et attend l'autre ;
    /// il peut encore rentrer au camp.
    AttenteDuGroupe,
    /// À la fin d'une étape, le sac déborde de `Partie::exces_du_sac`
    /// objets : le joueur en jette par `Choix::Jeter` (section 13.3).
    SacPlein,
    /// L'adversaire du prochain palier de la Fosse aux damnés : le joueur le
    /// combat par `Choix::Poursuivre`, ou revient au camp par
    /// `Choix::RentrerAuCamp` (section 9.8).
    AdversaireDeLArene {
        palier: u8,
        adversaire: &'a AdversaireDeLArene,
    },
}

/// Ce que le joueur doit confirmer avant que son choix s'applique.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Confirmation {
    /// Une nouvelle partie remplace celle de ce héros (Q56).
    RemplacerLaPartie(Heros),
    /// Le soin serait sans effet sur ce monstre, et perdu (Q53, section
    /// 14.4).
    SoinPerdu {
        soin: Soin,
        monstre: Monstre,
    },
    Relacher(Monstre),
    /// Les monstres rentreraient sans rien rapporter (section 12.7).
    RappelerLExpedition(ExpeditionAutomatique),
    /// La vente est définitive (section 14.2).
    Vendre {
        monstre: Monstre,
        prix: u32,
    },
    /// La partie de ce héros serait effacée pour de bon (section 13.9).
    Suicide(Heros),
    /// Une capsule d'habileté serait consommée (section 5.9).
    ChangerDHabilete {
        monstre: Monstre,
        nouvelle: Habilete,
    },
    /// Les ressources du plan seront prises (section 13.4). En boîte, pour
    /// que la confirmation reste légère.
    LancerAmelioration(Box<PlanDAmelioration>),
    /// Les ressources du plan seront prises (section 13.5).
    LancerFabrication(Box<PlanDeFabrication>),
    /// Les ressources du charme seront prises (section 13.5).
    FabriquerCharme(Box<PlanDuCharme>),
    FabriquerFilets(Box<PlanDesFilets>),
    /// Les ressources et l'argent du rite seraient pris, et les parents
    /// resteraient auprès de Malvoise jusqu'à sa fin (section 13.11).
    LancerLeRite(Box<ApercuDuRite>),
    /// L'évolution est définitive, et consomme la pierre éventuelle (section
    /// 5.4).
    Evoluer {
        monstre: Monstre,
        vers: Espece,
        pierre: Option<PierreDEvolution>,
    },
    /// Un Philtre chromatique serait consommé (section 13.7).
    RendreShiny(Monstre),
    /// Un Philtre chromatique serait consommé pour le héros (section 13.7).
    RendreLeHerosShiny,
    /// La location ne serait pas remboursée (section 14.2).
    RendreLeMonstreLoue(Monstre),
    /// Les âmes des Apôtres seraient perdues (section 9.4).
    ForgerLeBeherit,
    /// Le monstre serait perdu pour toujours.
    Sacrifier(Monstre),
}

/// Ce qui s'est passé, dans l'ordre. Les monstres et les combats sont des
/// copies prises sur le moment, pour que l'interface puisse les nommer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evenement {
    /// Après le choix de la classe, dont dépend son dernier paragraphe
    /// (section 6).
    Introduction(Classe),
    /// Le palier de la Fosse aux damnés est vaincu pour la première fois,
    /// avec ce qu'il a rapporté (section 9.8).
    PalierDeLaFosseVaincu {
        palier: u8,
        adversaire: String,
        recompense: RecompenseObtenue,
        conquise: bool,
    },
    /// Le héros est tombé dans la Fosse, ou a jeté son arme.
    DefaiteDansLaFosse {
        abandon: bool,
    },
    PartieCommencee(Monstre),
    /// Le héros est né shiny, 1 fois sur 512 (section 4.1).
    HerosShiny,
    /// Nom de la zone.
    Depart(String),
    /// Le moment que garde une expédition dans une zone de la carte, dit
    /// après le départ (section 9.6).
    JourOuNuitAuDepart(JourOuNuit),
    /// La météo qu'elle garde, s'il n'y fait pas calme (section 9.7).
    MeteoAuDepart(Meteo),
    /// Numéro de l'étape et nombre d'étapes de l'expédition (section 9.2),
    /// sans total dans la Tour sans fond (section 9.5).
    Etape {
        numero: u32,
        total: Option<u32>,
    },
    /// Le récit de l'entrée dans la Tour sans fond, au départ (section 9.5).
    EntreeDansLaTour(String),
    /// Le récit du gardien de la Tour, avant le combat annoncé.
    GardienDeLaTour(String),
    /// Après la victoire sur le gardien : tous les monstres sont soignés.
    ReposDansLaTour(String),
    /// Au retour au camp, la dernière étape franchie bat le record.
    NouveauRecordDeLaTour(u32),
    /// L'argent et la capsule ne seront acquis qu'au retour au camp
    /// (section 9.3).
    Coffre {
        potions: u32,
        /// Avec `argent_du_pillard`, la part due à l'habileté Pillard
        /// (section 5.9).
        argent: u32,
        argent_du_pillard: u32,
        capsule: Option<Capsule>,
        /// Elle va dans le sac (section 5.9).
        capsule_d_habilete: bool,
        /// Elle va dans le sac (section 5.4).
        pierre: Option<PierreDEvolution>,
        /// Il va dans le sac (section 14.3).
        booster: Option<Booster>,
        /// Ils vont dans le sac (section 11.2).
        filets: Option<(Filet, u32)>,
        /// Dans une zone annexe, il va dans le sac (section 9.9).
        tresor: Option<Tresor>,
    },
    MonstreRarePacifique(Monstre),
    MonstreRareHostile(Monstre),
    MonstreRareLaisse,
    RecitDeLApotre(String),
    /// La carte d'une zone annexe, trouvée dans un coffre : son nom et le
    /// texte de sa découverte (section 9.9).
    AnnexeDecouverte {
        nom: String,
        texte: String,
    },
    /// Le texte qui présente le légendaire avant le combat (section 9.10).
    LegendaireRencontre(String),
    /// Vaincu, il rejoint le joueur : le texte de sa capture.
    LegendaireCapture(String),
    /// Le texte qui présente l'humain avant le combat (section 10.7).
    HumainRencontre(String),
    /// Son texte après la victoire, et l'argent mis dans le sac.
    HumainVaincu {
        texte: String,
        argent: u32,
    },
    /// Le texte de sa libération : Gunther rentre au camp avec le joueur
    /// (section 13.4).
    GuntherLibere(String),
    /// Le texte du marchand mort trouvé, avant le combat (section 14.2).
    AmeRencontree(String),
    /// Son texte après la victoire : le joueur porte désormais l'âme.
    AmeRecueillie(String),
    /// Le texte d'Ansgar aux prises avec ce qu'il a ferré, avant le combat
    /// (section 9.2).
    PecheurRencontre(String),
    /// Son texte après la victoire : Ansgar s'installe au camp.
    PecheurSauve(String),
    Rencontre(Combat),
    /// Événements d'un tour, avec le combat qui permet de les mettre en texte.
    Combat {
        evenements: Vec<combat::Evenement>,
        combat: Combat,
    },
    Experience {
        monstre: Monstre,
        rapport: RapportExperience,
    },
    /// Après une victoire, grâce à l'habileté Fouineur de ce monstre ; la
    /// potion va dans le sac (section 5.9).
    PotionTrouvee(Monstre),
    /// Après une victoire, lâchées par les monstres sauvages mis KO, une par
    /// monstre ; elles vont dans le sac (section 14.3).
    RessourcesTrouvees(Vec<Ressource>),
    /// Rachetées par Malvoise ; l'argent va dans le coffre.
    RessourcesVendues {
        ressource: Ressource,
        nombre: u32,
        prix: u32,
    },
    AttaqueRemplacee {
        monstre: Monstre,
        oubliee: Attaque,
        nouvelle: Attaque,
    },
    AttaqueAbandonnee {
        monstre: Monstre,
        nouvelle: Attaque,
    },
    /// Le monstre avant et après son évolution, et l'attaque d'évolution
    /// apprise sans rien oublier ; sinon, son choix suit (section 5.4).
    Evolution {
        avant: Monstre,
        apres: Monstre,
        attaque: Option<Attaque>,
    },
    /// Le joueur a repoussé l'évolution de ce monstre.
    EvolutionRepoussee(Monstre),
    /// Au camp, sans rien oublier ; sinon `AttaqueRemplacee` (section
    /// 13.10).
    AttaqueApprise {
        monstre: Monstre,
        attaque: Attaque,
    },
    RejointLEquipe(Monstre),
    PartPourLeCamp(Monstre),
    CaptureRelachee(Monstre),
    RelachePourLaCapture {
        relache: Monstre,
        capture: Monstre,
    },
    Epilogue(String),
    /// Après l'épilogue : le récit de l'âme de l'Apôtre vaincu, que le
    /// joueur garde (section 9.4).
    AmeDApotreRecueillie(String),
    /// Nom de la zone (section 13.6).
    PiegesPoses(String),
    /// Ce qu'ils avaient pris, rangé dans le coffre, par sorte.
    PiegesReleves(Vec<(Ressource, u32)>),
    EntreeALEntrainement(Monstre),
    /// Les parents confiés à Malvoise, ce qu'ils ont coûté et le petit
    /// attendu (section 13.11).
    RiteLance(Box<ApercuDuRite>),
    /// Le petit né du rite, et ses parents revenus.
    PetitNe(Box<NaissanceDuRite>),
    /// Ce qu'un palier du codex a rapporté, rangé au coffre (section 5.6).
    RecompenseDuCodex(Box<RecompenseDuCodex>),
    /// Suivi de l'XP gagnée, s'il en a gagné.
    SortieDeLEntrainement {
        monstre: Monstre,
        experience: u32,
        emplacement: Emplacement,
    },
    /// Après l'âme : le Philtre chromatique que laisse l'Apôtre, qui va dans le sac
    /// (section 14.3).
    PhiltreChromatiqueRamasse,
    /// Toute l'équipe est KO : retour au camp, tout le sac perdu (section
    /// 9.3).
    Defaite {
        sac_perdu: Inventaire,
    },
    /// En combat ; `monstre` est celui qui reçoit l'ordre (section 4.2).
    OrdreDonne {
        heros: String,
        ordre: Ordre,
        monstre: Monstre,
    },
    /// Entre deux étapes ; `monstre` est le monstre soigné.
    SoinUtilise {
        soin: Soin,
        monstre: Monstre,
        effet: EffetDuSoin,
    },
    /// Nom de la zone menée jusqu'au bout.
    ZoneExploree(String),
    /// Ce que le joueur a jeté de son sac, qui débordait (section 13.3).
    ObjetsJetes(Inventaire),
    RetourAuCamp,
    Indice {
        numero: usize,
        texte: String,
    },
    /// Nom de la zone apparue sur la carte.
    NouvelleZone(String),
    Conseil(Conseil),
    Relache(Monstre),
    /// Copie de l'expédition qui vient de partir.
    ExpeditionAutomatiquePartie(ExpeditionAutomatique),
    /// Copie de l'expédition rappelée, avant le retour de ses monstres, et
    /// la place de chacun, dans l'ordre de l'expédition (section 12.7).
    ExpeditionAutomatiqueRappelee {
        expedition: ExpeditionAutomatique,
        places: Vec<Emplacement>,
    },
    /// Suivi d'un événement `Experience` par monstre qui en a gagné.
    ExpeditionAutomatiqueRentree {
        /// Nom de la zone.
        zone: String,
        bilan: BilanExpeditionAutomatique,
    },
    /// Au bout de la première expédition menée au bout, avant le retour au
    /// camp, selon que le joueur porte une âme ou non (section 14.2).
    MalvoiseRencontre {
        avec_une_ame: bool,
    },
    CapsulesDHabileteAchetees {
        nombre: u32,
        prix: u32,
    },
    PierresAchetees {
        pierre: PierreDEvolution,
        nombre: u32,
        prix: u32,
    },
    /// Gunther se met au travail, sur ce plan, jusqu'à la fin du chantier
    /// (section 13.4). Le plan est en boîte, comme dans la confirmation.
    AmeliorationLancee {
        plan: Box<PlanDAmelioration>,
        chantier: Chantier,
    },
    /// Gunther se met à forger cet objet (section 13.5).
    FabricationLancee {
        plan: Box<PlanDeFabrication>,
        chantier: Chantier,
    },
    CharmeLance {
        plan: Box<PlanDuCharme>,
        chantier: Chantier,
    },
    FiletsLances {
        plan: Box<PlanDesFilets>,
        chantier: Chantier,
    },
    /// Le niveau construit compte désormais, ou l'objet forgé rejoint
    /// l'équipement du héros.
    TravailConstate(Chantier),
    /// La ligne d'Ansgar sera prête à l'heure `fin`, dans `delai` (section
    /// 13.8).
    LigneLancee {
        fin: Horodatage,
        delai: Duration,
    },
    /// Ce qu'a remonté la ligne : un objet est déjà dans le coffre, un
    /// monstre attend que le joueur le garde ou le relâche.
    PrisePechee(Prise),
    /// Le monstre pêché retourne à l'eau.
    PriseRelachee(Monstre),
    /// L'objet porté à cette place, et celui qu'il remplace.
    ObjetEquipe {
        place: PlaceDEquipement,
        objet: ObjetDEquipement,
        remplace: Option<ObjetDEquipement>,
    },
    ObjetRetire {
        place: PlaceDEquipement,
        objet: ObjetDEquipement,
    },
    /// Le monstre après le changement.
    HabileteChangee {
        monstre: Monstre,
        ancienne: Option<Habilete>,
    },
    /// Le monstre avant et après le booster (section 14.3).
    BoosterDonne {
        booster: Booster,
        avant: Monstre,
        apres: Monstre,
    },
    /// Le monstre, désormais shiny (section 13.7).
    MonstreRenduShiny(Monstre),
    /// Le héros, désormais shiny (section 13.7).
    HerosRenduShiny,
    /// Nom du marchand dont l'âme est donnée, et le soin que Malvoise vend
    /// désormais.
    AmeDonnee {
        marchand: String,
        soin: Soin,
    },
    /// Prix payé, en pièces d'argent.
    SoinsAchetes {
        soin: Soin,
        nombre: u32,
        prix: u32,
    },
    /// Prix payé, en pièces d'argent.
    FiletsAchetes {
        filet: Filet,
        nombre: u32,
        prix: u32,
    },
    MonstreVendu {
        monstre: Monstre,
        prix: u32,
    },
    /// Chez Malvoise, avec la place qu'il a prise (section 14.2).
    MonstreAchete {
        monstre: Monstre,
        prix: u32,
        dans_l_equipe: bool,
    },
    /// Confié par Malvoise pour 3 expéditions (section 14.2).
    MonstreLoue {
        monstre: Monstre,
        prix: u32,
        dans_l_equipe: bool,
    },
    /// Rendu plus tôt à Malvoise.
    MonstreLoueRendu(Monstre),
    /// Au retour de sa dernière expédition, il repart chez Malvoise.
    LocationFinie(Monstre),
    /// Le récit de Malvoise qui forge le Béhérit pourpre (section 9.4).
    BeheritForge(String),
    /// Le récit du sacrifice, qui ouvre l'Éclipse.
    Sacrifice(String),
    /// Après l'épilogue du Démiurge, ses Philtres chromatiques
    /// (`contenu::RECOMPENSE_DU_DEMIURGE`).
    RecompenseDuDemiurge,
    /// À un autre joueur, par don ou échange ; `heros` est le nom de son
    /// héros (section 15.1).
    MonstreCede {
        heros: String,
        monstre: Monstre,
    },
    /// D'un autre joueur, déjà marqué du nom de son héros.
    MonstreRecu {
        heros: String,
        monstre: Monstre,
    },
    CapsuleAchetee {
        capsule: Capsule,
        prix: u32,
    },
    /// Ce qui est passé du sac au coffre (section 13.3).
    RangeAuCoffre(Inventaire),
    /// Ce qui est passé du coffre au sac.
    SortiDuCoffre(Inventaire),
    /// La partie continue : la sauvegarde sera retentée plus tard.
    SauvegardeImpossible(ErreurSauvegarde),
    /// La partie est effacée ; un nouveau héros est à créer (section 13.9).
    Suicide,
    /// L'ancienne sauvegarde reste jusqu'à la création du nouveau héros, qui
    /// la remplacera.
    EffacementImpossible(ErreurSauvegarde),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurChoix {
    /// Le choix ne répond pas à la question posée.
    HorsDePropos,
    Nom(ErreurNom),
    ZoneInconnue(ZoneId),
    ZoneFermee,
    /// Pas d'expédition manuelle sans monstre dans l'équipe (section 13.2).
    EquipeVide,
    /// Ni avec un sac qui déborde de tant d'objets (section 13.3).
    SacTropPlein {
        exces: u32,
    },
    Objet(ErreurObjet),
    Collection(ErreurCollection),
    Soin(ErreurSoin),
    Combat(ErreurCombat),
    Attaque(ErreurAttaquesMonstre),
    ExpeditionAutomatique(ErreurExpeditionAutomatique),
    Marchand(ErreurMarchand),
    /// Le marchand ne vend pas cette capsule.
    CapsuleInconnue(AttaqueId),
    Apprentissage(ErreurApprentissage),
    Habilete(ErreurChangementDHabilete),
    Booster(ErreurBooster),
    Evolution(ErreurEvolution),
    Amelioration(ErreurAmelioration),
    /// Le contenu n'a pas de plan pour ce niveau ou cet objet.
    PlanInconnu,
    Equipement(ErreurEquipement),
    Peche(ErreurPeche),
    Machine(ErreurMachine),
    Rite(ErreurRite),
    Codex(ErreurCodex),
    Beherit(ErreurBeherit),
    /// En groupe, c'est à l'autre joueur d'agir (section 15.5).
    AuTourDeLAutre,
    /// En groupe, l'autre joueur a encore le temps de jouer.
    AttenteTropCourte,
    Arene(ErreurArene),
}

impl fmt::Display for ErreurChoix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurChoix::HorsDePropos => write!(f, "ce choix ne répond pas à la question"),
            ErreurChoix::Nom(erreur) => write!(f, "{erreur}"),
            ErreurChoix::ZoneInconnue(zone) => write!(f, "zone inconnue : {}", zone.as_str()),
            ErreurChoix::ZoneFermee => write!(f, "cette zone n'est pas encore sur la carte"),
            ErreurChoix::EquipeVide => write!(f, "l'équipe est vide"),
            ErreurChoix::SacTropPlein { exces } => write!(f, "le sac a {exces} objets de trop"),
            ErreurChoix::Objet(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Collection(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Soin(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Combat(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Attaque(erreur) => write!(f, "{erreur}"),
            ErreurChoix::ExpeditionAutomatique(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Marchand(erreur) => write!(f, "{erreur}"),
            ErreurChoix::CapsuleInconnue(capsule) => {
                write!(f, "capsule inconnue : {}", capsule.as_str())
            }
            ErreurChoix::Apprentissage(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Habilete(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Booster(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Evolution(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Beherit(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Amelioration(erreur) => write!(f, "{erreur}"),
            ErreurChoix::PlanInconnu => write!(f, "aucun plan pour ce niveau ou cet objet"),
            ErreurChoix::Equipement(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Peche(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Machine(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Rite(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Codex(erreur) => write!(f, "{erreur}"),
            ErreurChoix::AuTourDeLAutre => write!(f, "c'est à l'autre joueur d'agir"),
            ErreurChoix::AttenteTropCourte => write!(f, "l'autre joueur a encore le temps"),
            ErreurChoix::Arene(erreur) => write!(f, "{erreur}"),
        }
    }
}

impl std::error::Error for ErreurChoix {}

pub struct Deroulement {
    etat: Etat,
    /// Choix suspendu à la confirmation du joueur.
    a_confirmer: Option<(Choix, Confirmation)>,
}

enum Etat {
    Reprise(Box<Partie>),
    SauvegardeIllisible(ErreurSauvegarde),
    NomDuHeros,
    Classe(Nom),
    EnJeu(Box<EnJeu>),
}

/// Ce que devient le déroulement après un choix.
#[derive(Debug, PartialEq, Eq)]
enum Reponse {
    Faite,
    /// En boîte : une confirmation copie un monstre ou une expédition, bien
    /// plus lourds que les autres réponses.
    AConfirmer(Box<Confirmation>),
    /// L'expédition est finie : il reste à rentrer au camp.
    RetourAuCamp,
    /// Après le suicide du héros : un nouveau héros est à créer.
    NouvellePartie,
}

impl Reponse {
    fn a_confirmer(confirmation: Confirmation) -> Self {
        Self::AConfirmer(Box::new(confirmation))
    }
}

impl Deroulement {
    /// Commence selon ce qu'a donné le chargement de la sauvegarde : reprise
    /// d'une partie, création d'un héros, ou nouvelle partie proposée à la
    /// place d'une sauvegarde illisible (section 13.9).
    pub fn demarrer(chargement: Result<Option<Partie>, ErreurSauvegarde>) -> Self {
        let etat = match chargement {
            Ok(Some(partie)) => Etat::Reprise(Box::new(partie)),
            Ok(None) => Etat::NomDuHeros,
            Err(erreur) => Etat::SauvegardeIllisible(erreur),
        };
        Self {
            etat,
            a_confirmer: None,
        }
    }

    /// La partie en jeu, pour les affichages qui ne changent rien.
    pub fn partie(&self) -> Option<&Partie> {
        match &self.etat {
            Etat::EnJeu(jeu) => Some(&jeu.partie),
            _ => None,
        }
    }

    /// La partie, si le joueur est au camp sans question en attente : seule
    /// une telle partie cède ou reçoit un monstre (section 15.1).
    pub fn partie_au_camp(&self) -> Option<&Partie> {
        if self.attente() != Attente::Camp {
            return None;
        }
        self.partie()
    }

    fn partie_au_camp_mut(&mut self) -> Option<&mut Partie> {
        if self.attente() != Attente::Camp {
            return None;
        }
        match &mut self.etat {
            Etat::EnJeu(jeu) => Some(&mut jeu.partie),
            _ => None,
        }
    }

    /// Conclut l'offre faite par `donneur` à `receveur`, tous deux au camp,
    /// puis sauvegarde la partie du receveur avant celle du donneur : un
    /// incident entre les deux dupliquerait le monstre plutôt que de le
    /// perdre (Q89). Rend ce que chacun a vécu, ou ne change rien si l'offre
    /// n'est plus valable.
    pub fn conclure_offre(
        donneur: &mut Deroulement,
        receveur: &mut Deroulement,
        offre: &Offre,
        depot_du_donneur: &dyn SauvegardeRepository,
        depot_du_receveur: &dyn SauvegardeRepository,
    ) -> Result<(Vec<Evenement>, Vec<Evenement>), ErreurEchange> {
        let partie_du_donneur = donneur
            .partie_au_camp_mut()
            .ok_or(ErreurEchange::DonneurPasAuCamp)?;
        let partie_du_receveur = receveur
            .partie_au_camp_mut()
            .ok_or(ErreurEchange::ReceveurPasAuCamp)?;
        let conclusion = conclure_offre(partie_du_donneur, partie_du_receveur, offre)?;
        let recu = |partie: &Partie, emplacement: Emplacement| {
            partie
                .collection()
                .monstre(emplacement)
                .expect("monstre placé par conclure_offre")
                .clone()
        };
        let heros_du_donneur = partie_du_donneur.heros().nom().as_str().to_string();
        let heros_du_receveur = partie_du_receveur.heros().nom().as_str().to_string();
        let mut du_donneur = vec![Evenement::MonstreCede {
            heros: heros_du_receveur.clone(),
            monstre: offre.cede.monstre.clone(),
        }];
        let mut du_receveur = Vec::new();
        if let Some(demande) = &offre.demande {
            du_receveur.push(Evenement::MonstreCede {
                heros: heros_du_donneur.clone(),
                monstre: demande.monstre.clone(),
            });
        }
        du_receveur.push(Evenement::MonstreRecu {
            heros: heros_du_donneur,
            monstre: recu(partie_du_receveur, conclusion.chez_le_receveur),
        });
        if let Some(emplacement) = conclusion.chez_le_donneur {
            du_donneur.push(Evenement::MonstreRecu {
                heros: heros_du_receveur,
                monstre: recu(partie_du_donneur, emplacement),
            });
        }
        sauvegarder(partie_du_receveur, depot_du_receveur, &mut du_receveur);
        sauvegarder(partie_du_donneur, depot_du_donneur, &mut du_donneur);
        Ok((du_donneur, du_receveur))
    }

    /// Au camp, sans question en attente : ajoute ou retire cet ami, puis
    /// sauvegarde (section 15.4). Rend ce qu'a donné la sauvegarde, ou
    /// `None` hors du camp ou si rien n'a changé.
    pub fn changer_d_ami(
        &mut self,
        joueur: IdJoueur,
        ami: bool,
        depot: &dyn SauvegardeRepository,
    ) -> Option<Vec<Evenement>> {
        let partie = self.partie_au_camp_mut()?;
        let relations = partie.relations_mut();
        let change = if ami {
            relations.ajouter_ami(joueur)
        } else {
            relations.retirer_ami(joueur)
        };
        if !change {
            return None;
        }
        let mut evenements = Vec::new();
        sauvegarder(partie, depot, &mut evenements);
        Some(evenements)
    }

    /// À la fin d'un duel, que le joueur a livré depuis le camp : sa
    /// victoire ou sa défaite rejoint son palmarès, puis la partie est
    /// sauvegardée (section 15.4). `None` si le joueur n'est plus au camp.
    pub fn noter_duel(
        &mut self,
        victoire: bool,
        depot: &dyn SauvegardeRepository,
    ) -> Option<Vec<Evenement>> {
        let partie = self.partie_au_camp_mut()?;
        partie.relations_mut().noter_duel(victoire);
        let mut evenements = Vec::new();
        sauvegarder(partie, depot, &mut evenements);
        Some(evenements)
    }

    /// À la fin d'un combat d'arène contre un autre joueur, livré depuis le
    /// camp : la cote monte ou baisse du gain, puis la partie est
    /// sauvegardée (section 15.6). `None` si le joueur n'est plus au camp.
    pub fn noter_combat_d_arene(
        &mut self,
        victoire: bool,
        gain: u32,
        depot: &dyn SauvegardeRepository,
    ) -> Option<Vec<Evenement>> {
        let partie = self.partie_au_camp_mut()?;
        partie.noter_combat_d_arene(victoire, gain);
        let mut evenements = Vec::new();
        sauvegarder(partie, depot, &mut evenements);
        Some(evenements)
    }

    /// `None` hors expédition.
    pub fn expedition(&self) -> Option<&Expedition> {
        match &self.etat {
            Etat::EnJeu(jeu) => jeu.expedition.as_ref().map(|en_cours| &en_cours.expedition),
            _ => None,
        }
    }

    fn en_jeu_mut(&mut self) -> Option<&mut EnJeu> {
        match &mut self.etat {
            Etat::EnJeu(jeu) => Some(jeu),
            _ => None,
        }
    }

    /// En groupe, au camp et sans question en attente : le départ dans la
    /// Tour sans fond, sans étape ; l'expédition en groupe tire la première
    /// (section 15.5).
    pub(crate) fn partir_en_groupe(
        &mut self,
        expedition: Expedition,
        evenements: &mut Vec<Evenement>,
    ) -> Result<(), ErreurChoix> {
        if self.attente() != Attente::Camp {
            return Err(ErreurChoix::HorsDePropos);
        }
        let jeu = self.en_jeu_mut().ok_or(ErreurChoix::HorsDePropos)?;
        evenements.push(Evenement::Depart(expedition.zone().nom().to_string()));
        if let Some(tour) = expedition.tour() {
            evenements.push(Evenement::EntreeDansLaTour(
                tour.recit_de_l_entree().to_string(),
            ));
        }
        jeu.expedition = Some(EnExpedition {
            expedition,
            moment: Moment::AttenteDuGroupe,
            en_groupe: true,
            ame: None,
            pecheur: None,
        });
        Ok(())
    }

    /// L'expédition de ce joueur, que l'expédition en groupe fait avancer.
    pub(crate) fn expedition_mut(&mut self) -> Option<&mut Expedition> {
        let jeu = self.en_jeu_mut()?;
        jeu.expedition
            .as_mut()
            .map(|en_cours| &mut en_cours.expedition)
    }

    /// En groupe : l'événement de l'étape, que l'hôte a tiré, avec le coffre
    /// de ce joueur ; il attendait de continuer.
    pub(crate) fn recevoir_l_etape(
        &mut self,
        evenement: EvenementExpedition,
        contexte: &mut Contexte<'_>,
    ) -> Vec<Evenement> {
        let mut evenements = Vec::new();
        let Some(jeu) = self.en_jeu_mut() else {
            return evenements;
        };
        let Some(en_cours) = jeu
            .expedition
            .as_mut()
            .filter(|en_cours| matches!(en_cours.moment, Moment::AttenteDuGroupe))
        else {
            return evenements;
        };
        let reponse =
            en_cours.traiter_l_evenement(&mut jeu.partie, evenement, contexte, &mut evenements);
        if reponse == Reponse::RetourAuCamp {
            jeu.rentrer(contexte, &mut evenements);
        }
        evenements
    }

    /// En groupe : la part de ce joueur dans le bilan du combat commun,
    /// lue comme après un combat seul (section 15.5).
    pub(crate) fn recevoir_le_bilan(
        &mut self,
        bilan: BilanCombat,
        contexte: &mut Contexte<'_>,
    ) -> Vec<Evenement> {
        let mut evenements = Vec::new();
        let Some(jeu) = self.en_jeu_mut() else {
            return evenements;
        };
        let Some(en_cours) = jeu.expedition.as_mut() else {
            return evenements;
        };
        let Moment::CombatDeGroupe { contre_un_gardien } = en_cours.moment else {
            return evenements;
        };
        let fin = FinDeCombat {
            issue: bilan.issue,
            contre_un_apotre: false,
            contre_un_humain: false,
            contre_un_gardien,
            contre_un_legendaire: false,
            sac_perdu: Inventaire::default(),
        };
        let reponse =
            en_cours.lire_le_bilan(&mut jeu.partie, bilan, fin, 0, contexte, &mut evenements);
        if reponse == Reponse::RetourAuCamp {
            jeu.rentrer(contexte, &mut evenements);
        }
        evenements
    }

    /// En groupe : la rencontre du combat commun, que l'expédition en
    /// groupe vient de préparer, comme au début d'un combat seul.
    pub(crate) fn engager_en_groupe(&mut self, combat: &Combat) -> Vec<Evenement> {
        let mut evenements = vec![Evenement::Rencontre(combat.clone())];
        if let Some(jeu) = self.en_jeu_mut() {
            montrer_conseil(&mut jeu.partie, Conseil::PremierCombat, &mut evenements);
        }
        evenements
    }

    /// En groupe, le joueur attend l'autre pour continuer.
    pub(crate) fn veut_continuer(&self) -> bool {
        self.attente() == Attente::AttenteDuGroupe
    }

    /// En groupe, quand le groupe rentre : seulement entre deux étapes,
    /// qu'il veuille continuer ou non. `None` s'il a encore à répondre.
    pub(crate) fn rentrer_avec_le_groupe(
        &mut self,
        contexte: &mut Contexte<'_>,
    ) -> Option<Vec<Evenement>> {
        if !matches!(
            self.attente(),
            Attente::EntreDeuxEtapes | Attente::AttenteDuGroupe
        ) {
            return None;
        }
        let jeu = self.en_jeu_mut()?;
        let mut evenements = Vec::new();
        jeu.rentrer(contexte, &mut evenements);
        Some(evenements)
    }

    /// Le combat que le joueur engagera par `Choix::Poursuivre`, pendant
    /// `Annonce::Combat` : contre un humain, un Apôtre ou un gardien de la
    /// Tour. Les interfaces y voient qui l'attend (section 5.1).
    pub fn combat_annonce(&self) -> Option<&Combat> {
        if self.a_confirmer.is_some() {
            return None;
        }
        let Etat::EnJeu(jeu) = &self.etat else {
            return None;
        };
        match &jeu.expedition.as_ref()?.moment {
            Moment::AnnonceDeCombat(combat) if jeu.arene.is_none() => Some(combat),
            _ => None,
        }
    }

    pub fn attente(&self) -> Attente<'_> {
        if let Some((_, confirmation)) = &self.a_confirmer {
            return Attente::Confirmation(confirmation);
        }
        match &self.etat {
            Etat::Reprise(partie) => Attente::Reprise(partie),
            Etat::SauvegardeIllisible(erreur) => Attente::SauvegardeIllisible(erreur),
            Etat::NomDuHeros => Attente::NomDuHeros,
            Etat::Classe(_) => Attente::Classe,
            Etat::EnJeu(jeu) => jeu.attente(),
        }
    }

    /// Applique le choix et rend ce qui s'est passé jusqu'à la question
    /// suivante.
    pub fn repondre(
        &mut self,
        choix: Choix,
        contexte: &mut Contexte<'_>,
    ) -> Result<Vec<Evenement>, ErreurChoix> {
        let mut evenements = Vec::new();
        let (choix, confirme) = match self.a_confirmer.take() {
            None => (choix, false),
            Some((suspendu, _)) if choix == Choix::Oui => (suspendu, true),
            Some(_) if choix == Choix::Non => return Ok(evenements),
            suspendu => {
                self.a_confirmer = suspendu;
                return Err(ErreurChoix::HorsDePropos);
            }
        };
        let reponse = match &mut self.etat {
            Etat::EnJeu(jeu) => jeu.appliquer(&choix, confirme, contexte, &mut evenements)?,
            _ => self.accueillir(&choix, confirme, contexte, &mut evenements)?,
        };
        match reponse {
            Reponse::AConfirmer(confirmation) => self.a_confirmer = Some((choix, *confirmation)),
            Reponse::NouvellePartie => self.etat = Etat::NomDuHeros,
            Reponse::Faite | Reponse::RetourAuCamp => {}
        }
        Ok(evenements)
    }

    /// Reprise et création du héros, qui reçoit le monstre attitré de sa
    /// classe (section 6).
    fn accueillir(
        &mut self,
        choix: &Choix,
        confirme: bool,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        let suivant = match (&self.etat, choix) {
            (Etat::Reprise(partie), Choix::ReprendreLaPartie) => {
                // La traque compte désormais 13 Apôtres (section 9.4).
                let mut partie = Partie::clone(partie);
                partie.reprendre_les_ames(contexte.contenu.piste());
                EnJeu::au_camp(partie)
            }
            (Etat::Reprise(partie), Choix::NouvellePartie) => {
                if !confirme {
                    let heros = partie.heros().clone();
                    return Ok(Reponse::a_confirmer(Confirmation::RemplacerLaPartie(heros)));
                }
                Etat::NomDuHeros
            }
            (Etat::SauvegardeIllisible(_), Choix::NouvellePartie) => Etat::NomDuHeros,
            (Etat::NomDuHeros, Choix::NomDuHeros(texte)) => {
                Etat::Classe(Nom::new(texte).map_err(ErreurChoix::Nom)?)
            }
            (Etat::Classe(nom), Choix::Classe(classe)) => {
                evenements.push(Evenement::Introduction(*classe));
                let espece = contexte.contenu.espece_de_depart(*classe);
                let depart = creer_monstre_de_depart(espece.clone(), &mut *contexte.aleatoire)
                    .expect("les monstres de départ du contenu existent au niveau 5");
                evenements.push(Evenement::PartieCommencee(depart.clone()));
                // Le héros tire son shiny après son monstre de départ, comme
                // un monstre sauvage (section 4.1).
                let shiny = tirer_shiny(&mut *contexte.aleatoire);
                if shiny {
                    evenements.push(Evenement::HerosShiny);
                }
                let heros = Heros::new(nom.clone(), *classe).avec_shiny(shiny);
                let partie = Partie::nouvelle(heros, depart);
                sauvegarder(&partie, contexte.depot, evenements);
                EnJeu::au_camp(partie)
            }
            _ => return Err(ErreurChoix::HorsDePropos),
        };
        self.etat = suivant;
        Ok(Reponse::Faite)
    }
}

/// Après un combat, seul un monstre qui a gagné un niveau se voit proposer
/// son évolution : un refus vaut ainsi jusqu'au niveau suivant (section 5.4).
fn evolution_a_proposer(
    bilan: &BilanCombat,
    index: usize,
    circonstances: &Circonstances<'_>,
    contexte: &Contexte<'_>,
) -> Option<Espece> {
    let rapport = bilan.experience.get(index)?.as_ref()?;
    // Un monstre loué n'évolue pas (section 14.2).
    if rapport.niveau_apres <= rapport.niveau_avant || bilan.equipe[index].est_loue() {
        return None;
    }
    evolution_possible(&bilan.equipe[index], circonstances, contexte.contenu)
}

fn zone_du_contenu<'c>(contenu: &'c ContenuV1, id: &ZoneId) -> Result<&'c Zone, ErreurChoix> {
    contenu
        .zones()
        .iter()
        .find(|zone| zone.id() == id)
        .ok_or_else(|| ErreurChoix::ZoneInconnue(id.clone()))
}

/// La sauvegarde est automatique (section 13.9) ; un échec n'arrête pas la
/// partie.
fn sauvegarder(partie: &Partie, depot: &dyn SauvegardeRepository, evenements: &mut Vec<Evenement>) {
    if let Err(erreur) = depot.enregistrer(partie) {
        evenements.push(Evenement::SauvegardeImpossible(erreur));
    }
}

fn montrer_conseil(partie: &mut Partie, conseil: Conseil, evenements: &mut Vec<Evenement>) {
    if partie.noter_conseil_vu(conseil) {
        evenements.push(Evenement::Conseil(conseil));
    }
}

struct EnJeu {
    partie: Partie,
    /// `None` au camp.
    expedition: Option<EnExpedition>,
    /// Le palier de la Fosse aux damnés en cours, depuis le camp.
    arene: Option<PalierEnCours>,
}

/// Le palier de la Fosse aux damnés affronté : son adversaire, présenté
/// avant le combat (section 9.8).
struct PalierEnCours {
    palier: u8,
    adversaire: AdversaireDeLArene,
    combat: Option<Combat>,
}

impl EnJeu {
    fn au_camp(partie: Partie) -> Etat {
        Etat::EnJeu(Box::new(Self {
            partie,
            expedition: None,
            arene: None,
        }))
    }

    fn attente(&self) -> Attente<'_> {
        if let Some(en_cours) = &self.arene {
            return match &en_cours.combat {
                None => Attente::AdversaireDeLArene {
                    palier: en_cours.palier,
                    adversaire: &en_cours.adversaire,
                },
                Some(combat) => Attente::ActionDeCombat(combat),
            };
        }
        match &self.expedition {
            None => match self.partie.attaque_a_choisir() {
                Some((monstre, nouvelle)) => Attente::OubliAttaque { monstre, nouvelle },
                None => match self.partie.peche().prise() {
                    Some(prise) => Attente::Prise(prise),
                    None => Attente::Camp,
                },
            },
            Some(en_cours) => en_cours.attente(),
        }
    }

    fn appliquer(
        &mut self,
        choix: &Choix,
        confirme: bool,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        if self.arene.is_some() {
            return self.combattre_dans_la_fosse(choix, contexte, evenements);
        }
        let reponse = match &mut self.expedition {
            None => return self.choisir_au_camp(choix, confirme, contexte, evenements),
            Some(en_cours) => {
                en_cours.appliquer(&mut self.partie, choix, confirme, contexte, evenements)?
            }
        };
        if reponse == Reponse::RetourAuCamp {
            self.rentrer(contexte, evenements);
            return Ok(Reponse::Faite);
        }
        Ok(reponse)
    }

    /// Devant l'adversaire, combattre ou revenir ; en combat, attaquer ou
    /// abandonner. À la fin, une victoire donne la récompense du palier et
    /// se sauvegarde ; une défaite ne coûte rien (section 9.8).
    fn combattre_dans_la_fosse(
        &mut self,
        choix: &Choix,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        let en_cours = self.arene.as_mut().expect("un palier en cours");
        let Some(combat) = &mut en_cours.combat else {
            match choix {
                Choix::Poursuivre => {
                    let combat = self
                        .partie
                        .preparer_combat_dans_la_fosse(&en_cours.adversaire)
                        .map_err(ErreurChoix::Combat)?;
                    en_cours.combat = Some(combat);
                }
                Choix::RentrerAuCamp => self.arene = None,
                _ => return Err(ErreurChoix::HorsDePropos),
            }
            return Ok(Reponse::Faite);
        };
        let tour = match choix {
            Choix::Combat(action @ Action::Attaquer(_)) => {
                combat.jouer_tour(*action, &mut *contexte.aleatoire)
            }
            Choix::Abandonner => combat.abandonner(Cote::Joueur),
            _ => return Err(ErreurChoix::HorsDePropos),
        }
        .map_err(ErreurChoix::Combat)?;
        evenements.push(Evenement::Combat {
            evenements: tour,
            combat: combat.clone(),
        });
        let Phase::Termine(issue) = combat.phase() else {
            return Ok(Reponse::Faite);
        };
        let en_cours = self.arene.take().expect("un palier en cours");
        if issue == Issue::Victoire {
            let arme = contexte
                .contenu
                .arme_du_champion(self.partie.heros().classe());
            let recompense = self.partie.vaincre_le_palier(
                en_cours.palier,
                &en_cours.adversaire,
                arme,
                &mut *contexte.aleatoire,
            );
            evenements.push(Evenement::PalierDeLaFosseVaincu {
                palier: en_cours.palier,
                adversaire: en_cours.adversaire.nom().to_string(),
                recompense,
                conquise: en_cours.palier == PALIERS_DE_LA_FOSSE,
            });
            sauvegarder(&self.partie, contexte.depot, evenements);
        } else {
            evenements.push(Evenement::DefaiteDansLaFosse {
                abandon: *choix == Choix::Abandonner,
            });
        }
        Ok(Reponse::Faite)
    }

    fn choisir_au_camp(
        &mut self,
        choix: &Choix,
        confirme: bool,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        // Les attaques à choisir après une expédition automatique passent
        // avant toute autre action (section 12.7).
        if self.partie.attaque_a_choisir().is_some() {
            return self.choisir_attaque(choix, contexte, evenements);
        }
        // Le monstre pêché aussi, pour que sa place ne puisse pas être prise
        // entre-temps (section 13.8).
        if self.partie.peche().prise().is_some() {
            return self.choisir_prise(choix, contexte, evenements);
        }
        match choix {
            Choix::EntrerDansLaFosse => {
                let (palier, adversaire) = self
                    .partie
                    .palier_a_affronter(contexte.contenu.fosse())
                    .map_err(ErreurChoix::Arene)?;
                self.arene = Some(PalierEnCours {
                    palier,
                    adversaire,
                    combat: None,
                });
                Ok(Reponse::Faite)
            }
            Choix::PartirEnExpedition(id) if id == contexte.contenu.invocation().eclipse().id() => {
                let expedition = self.partir_vers_l_eclipse(contexte)?;
                self.lancer(expedition, contexte, evenements);
                Ok(Reponse::Faite)
            }
            Choix::PartirEnExpedition(id) if id == contexte.contenu.tour().id() => {
                let expedition = self
                    .partie
                    .descendre_dans_la_tour(contexte.contenu.tour(), contexte.contenu.zones())
                    .map_err(|erreur| match erreur {
                        ErreurExpedition::ZoneFermee => ErreurChoix::ZoneFermee,
                        ErreurExpedition::EquipeVide => ErreurChoix::EquipeVide,
                        ErreurExpedition::SacTropPlein { exces } => {
                            ErreurChoix::SacTropPlein { exces }
                        }
                    })?;
                self.lancer(expedition, contexte, evenements);
                Ok(Reponse::Faite)
            }
            Choix::PartirEnExpedition(id) => {
                let contenu = contexte.contenu;
                let annexe = contenu
                    .annexes()
                    .iter()
                    .find(|annexe| annexe.zone().id() == id);
                let legendaire = contenu
                    .zones_legendaires()
                    .iter()
                    .find(|legendaire| legendaire.zone().id() == id);
                let (zone, depart) = match (annexe, legendaire) {
                    (Some(annexe), _) => (annexe.zone(), self.partie.partir_vers_l_annexe(annexe)),
                    (None, Some(legendaire)) => (
                        legendaire.zone(),
                        self.partie.partir_vers_la_zone_legendaire(legendaire),
                    ),
                    (None, None) => {
                        let zone = zone_du_contenu(contenu, id)?;
                        // Un coffre peut y contenir la carte de son annexe
                        // (section 9.9).
                        let carte = self.partie.annexe_a_decouvrir(zone, contenu.annexes());
                        let depart = self.partie.partir_en_expedition(zone, contenu.piste()).map(
                            |expedition| match carte {
                                Some(carte) => expedition.avec_carte(carte),
                                None => expedition,
                            },
                        );
                        (zone, depart)
                    }
                };
                let expedition = depart
                    .map_err(|erreur| match erreur {
                        ErreurExpedition::ZoneFermee => ErreurChoix::ZoneFermee,
                        ErreurExpedition::EquipeVide => ErreurChoix::EquipeVide,
                        ErreurExpedition::SacTropPlein { exces } => {
                            ErreurChoix::SacTropPlein { exces }
                        }
                    })?
                    .avec_jour_ou_nuit(JourOuNuit::a(contexte.maintenant))
                    .avec_meteo(Meteo::de_la_zone(zone, contexte.maintenant));
                self.lancer(expedition, contexte, evenements);
                Ok(Reponse::Faite)
            }
            Choix::ForgerLeBeherit => {
                let piste = contexte.contenu.piste();
                if !confirme {
                    self.partie
                        .verifier_la_forge_du_beherit(piste)
                        .map_err(ErreurChoix::Beherit)?;
                    return Ok(Reponse::a_confirmer(Confirmation::ForgerLeBeherit));
                }
                self.partie
                    .forger_le_beherit(piste)
                    .map_err(ErreurChoix::Beherit)?;
                evenements.push(Evenement::BeheritForge(
                    contexte
                        .contenu
                        .invocation()
                        .recit_de_la_forge()
                        .to_string(),
                ));
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::Sacrifier(emplacement) => {
                if !confirme {
                    let monstre = self
                        .partie
                        .estimer_sacrifice(*emplacement)
                        .map_err(ErreurChoix::Beherit)?;
                    return Ok(Reponse::a_confirmer(Confirmation::Sacrifier(monstre)));
                }
                let monstre = self
                    .partie
                    .sacrifier(*emplacement)
                    .map_err(ErreurChoix::Beherit)?;
                evenements.push(Evenement::Sacrifice(
                    contexte
                        .contenu
                        .invocation()
                        .recit_du_sacrifice(monstre.nom()),
                ));
                // Le sacrifice est définitif, même si le joueur quitte avant
                // le retour de l'expédition qu'il ouvre.
                sauvegarder(&self.partie, contexte.depot, evenements);
                let expedition = self.partir_vers_l_eclipse(contexte)?;
                self.lancer(expedition, contexte, evenements);
                Ok(Reponse::Faite)
            }
            Choix::Gerer(action) => {
                let reponse = gerer(&mut self.partie, action, confirme, evenements)?;
                if reponse == Reponse::Faite {
                    sauvegarder(&self.partie, contexte.depot, evenements);
                }
                Ok(reponse)
            }
            Choix::EnvoyerEnExpeditionAutomatique {
                zone,
                monstres,
                protection,
            } => {
                let zone = zone_du_contenu(contexte.contenu, zone)?;
                self.partie
                    .envoyer_en_expedition_automatique_avec(
                        zone,
                        monstres,
                        *protection,
                        contexte.maintenant,
                    )
                    .map_err(ErreurChoix::ExpeditionAutomatique)?;
                if let Some(expedition) = self.partie.expedition_automatique(zone.id()) {
                    evenements.push(Evenement::ExpeditionAutomatiquePartie(expedition.clone()));
                }
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::RappelerExpeditionAutomatique(zone) => {
                let expedition = self.expedition_automatique(zone)?;
                if !confirme {
                    // Une expédition déjà rentrée est refusée sans demander
                    // de confirmation.
                    if expedition.est_rentree(contexte.maintenant) {
                        return Err(ErreurChoix::ExpeditionAutomatique(
                            ErreurExpeditionAutomatique::DejaRentree,
                        ));
                    }
                    return Ok(Reponse::a_confirmer(Confirmation::RappelerLExpedition(
                        expedition,
                    )));
                }
                let places = self
                    .partie
                    .rappeler_expedition_automatique(zone, contexte.maintenant)
                    .map_err(ErreurChoix::ExpeditionAutomatique)?;
                evenements.push(Evenement::ExpeditionAutomatiqueRappelee { expedition, places });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::AccueillirExpeditionAutomatique(zone) => {
                let expedition = self.expedition_automatique(zone)?;
                let bilan = self
                    .partie
                    .accueillir_expedition_automatique(
                        zone,
                        contexte.maintenant,
                        &mut *contexte.aleatoire,
                    )
                    .map_err(ErreurChoix::ExpeditionAutomatique)?;
                let experience: Vec<Evenement> = bilan
                    .monstres
                    .iter()
                    .filter_map(|rentre| {
                        let rapport = rentre.rapport.clone()?;
                        Some(Evenement::Experience {
                            monstre: rentre.monstre.clone(),
                            rapport,
                        })
                    })
                    .collect();
                evenements.push(Evenement::ExpeditionAutomatiqueRentree {
                    zone: expedition.zone().nom().to_string(),
                    bilan,
                });
                evenements.extend(experience);
                // Tout de suite, pour que quitter le jeu ne permette pas de
                // tirer de nouveau le résultat (section 12.7).
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::AcheterFilets(filet, nombre) => {
                let prix = self
                    .partie
                    .acheter_filets(*filet, *nombre)
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::FiletsAchetes {
                    filet: *filet,
                    nombre: *nombre,
                    prix,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::AcheterSoins(soin, nombre) => {
                let prix = self
                    .partie
                    .acheter_soins(*soin, *nombre)
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::SoinsAchetes {
                    soin: *soin,
                    nombre: *nombre,
                    prix,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::Vendre(emplacement) => {
                if !confirme {
                    // Une vente impossible, comme celle du dernier monstre de
                    // l'équipe, est refusée sans demander de confirmation.
                    let (monstre, prix) = self
                        .partie
                        .estimer_vente(*emplacement)
                        .map_err(ErreurChoix::Marchand)?;
                    return Ok(Reponse::a_confirmer(Confirmation::Vendre { monstre, prix }));
                }
                let (monstre, prix) = self
                    .partie
                    .vendre(*emplacement)
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::MonstreVendu { monstre, prix });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::RangerAuCoffre(objet) => {
                let range = self
                    .partie
                    .ranger_au_coffre(objet)
                    .map_err(ErreurChoix::Objet)?;
                evenements.push(Evenement::RangeAuCoffre(range));
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::SortirDuCoffre(objet) => {
                let sorti = self
                    .partie
                    .sortir_du_coffre(objet)
                    .map_err(ErreurChoix::Objet)?;
                evenements.push(Evenement::SortiDuCoffre(sorti));
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::ToutRangerAuCoffre => {
                let range = self.partie.tout_ranger_au_coffre();
                evenements.push(Evenement::RangeAuCoffre(range));
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::AcheterCapsule(id) => {
                let capsule = contexte
                    .contenu
                    .capsules()
                    .iter()
                    .find(|capsule| capsule.id() == id)
                    .ok_or_else(|| ErreurChoix::CapsuleInconnue(id.clone()))?;
                let prix = self
                    .partie
                    .acheter_capsule(capsule)
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::CapsuleAchetee {
                    capsule: capsule.clone(),
                    prix,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::VendreRessources(id) => {
                let (ressource, nombre, prix) = self
                    .partie
                    .vendre_ressources(id)
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::RessourcesVendues {
                    ressource,
                    nombre,
                    prix,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::AcheterPierres(pierre, nombre) => {
                let prix = self
                    .partie
                    .acheter_pierres(*pierre, *nombre)
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::PierresAchetees {
                    pierre: *pierre,
                    nombre: *nombre,
                    prix,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::AcheterCapsulesDHabilete(nombre) => {
                let prix = self
                    .partie
                    .acheter_capsules_d_habilete(*nombre)
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::CapsulesDHabileteAchetees {
                    nombre: *nombre,
                    prix,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::ChangerDHabilete(emplacement) => {
                if !confirme {
                    // Un changement impossible est refusé sans demander de
                    // confirmation.
                    let (monstre, nouvelle) = self
                        .partie
                        .estimer_changement_d_habilete(*emplacement)
                        .map_err(ErreurChoix::Habilete)?;
                    return Ok(Reponse::a_confirmer(Confirmation::ChangerDHabilete {
                        monstre,
                        nouvelle,
                    }));
                }
                let (ancienne, _) = self
                    .partie
                    .changer_d_habilete(*emplacement)
                    .map_err(ErreurChoix::Habilete)?;
                let monstre = self
                    .partie
                    .collection()
                    .monstre(*emplacement)
                    .cloned()
                    .expect("le monstre vient de changer d'habileté");
                evenements.push(Evenement::HabileteChangee { monstre, ancienne });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::DonnerBooster {
                emplacement,
                booster,
            } => {
                let avant = self
                    .partie
                    .donner_booster(*emplacement, *booster)
                    .map_err(ErreurChoix::Booster)?;
                let apres = self
                    .partie
                    .collection()
                    .monstre(*emplacement)
                    .cloned()
                    .expect("le monstre vient de recevoir le booster");
                evenements.push(Evenement::BoosterDonne {
                    booster: *booster,
                    avant,
                    apres,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::AcheterMonstre(espece) => {
                let (monstre, placement, prix) = self
                    .partie
                    .acheter_monstre(
                        espece,
                        contexte.contenu.zones(),
                        contexte.contenu.especes(),
                        contexte.aleatoire,
                    )
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::MonstreAchete {
                    monstre,
                    prix,
                    dans_l_equipe: matches!(placement, Placement::Equipe(_)),
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::LouerMonstre(espece) => {
                let (monstre, placement, prix) = self
                    .partie
                    .louer_monstre(
                        espece,
                        contexte.contenu.zones(),
                        contexte.contenu.piste(),
                        contexte.aleatoire,
                    )
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::MonstreLoue {
                    monstre,
                    prix,
                    dans_l_equipe: matches!(placement, Placement::Equipe(_)),
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::RendreLeMonstreLoue => {
                if !confirme {
                    let monstre = self
                        .partie
                        .collection()
                        .monstre_loue()
                        .map(|(_, monstre)| monstre.clone())
                        .ok_or(ErreurChoix::Marchand(ErreurMarchand::AucunMonstreLoue))?;
                    return Ok(Reponse::a_confirmer(Confirmation::RendreLeMonstreLoue(
                        monstre,
                    )));
                }
                let monstre = self
                    .partie
                    .rendre_le_monstre_loue()
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::MonstreLoueRendu(monstre));
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::RendreShiny(emplacement) => {
                if !confirme {
                    // Un service impossible est refusé sans demander de
                    // confirmation.
                    let monstre = self
                        .partie
                        .estimer_shiny(*emplacement)
                        .map_err(ErreurChoix::Marchand)?;
                    return Ok(Reponse::a_confirmer(Confirmation::RendreShiny(monstre)));
                }
                self.partie
                    .rendre_shiny(*emplacement)
                    .map_err(ErreurChoix::Marchand)?;
                let monstre = self
                    .partie
                    .collection()
                    .monstre(*emplacement)
                    .cloned()
                    .expect("le monstre vient d'être rendu shiny");
                evenements.push(Evenement::MonstreRenduShiny(monstre));
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::RendreLeHerosShiny => {
                self.partie
                    .verifier_shiny_du_heros()
                    .map_err(ErreurChoix::Marchand)?;
                if !confirme {
                    return Ok(Reponse::a_confirmer(Confirmation::RendreLeHerosShiny));
                }
                self.partie
                    .rendre_le_heros_shiny()
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::HerosRenduShiny);
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::LancerAmelioration(amelioration) => {
                let niveau = self
                    .partie
                    .forge()
                    .ameliorations()
                    .niveau(*amelioration)
                    .saturating_add(1);
                if niveau > amelioration.niveau_max() {
                    return Err(ErreurChoix::Amelioration(ErreurAmelioration::NiveauMaximal));
                }
                let plan = contexte
                    .contenu
                    .plan_d_amelioration(*amelioration, niveau)
                    .ok_or(ErreurChoix::PlanInconnu)?;
                if !confirme {
                    // Un lancement impossible est refusé sans demander de
                    // confirmation.
                    self.partie
                        .verifier_amelioration(plan)
                        .map_err(ErreurChoix::Amelioration)?;
                    return Ok(Reponse::a_confirmer(Confirmation::LancerAmelioration(
                        Box::new(plan.clone()),
                    )));
                }
                let chantier = self
                    .partie
                    .lancer_amelioration(plan, contexte.maintenant)
                    .map_err(ErreurChoix::Amelioration)?;
                evenements.push(Evenement::AmeliorationLancee {
                    plan: Box::new(plan.clone()),
                    chantier,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::LancerFabrication(objet) => {
                let plan = contexte
                    .contenu
                    .plan_de_fabrication(objet)
                    .ok_or(ErreurChoix::PlanInconnu)?;
                if !confirme {
                    // Une fabrication impossible est refusée sans demander de
                    // confirmation.
                    self.partie
                        .verifier_fabrication(plan)
                        .map_err(ErreurChoix::Amelioration)?;
                    return Ok(Reponse::a_confirmer(Confirmation::LancerFabrication(
                        Box::new(plan.clone()),
                    )));
                }
                let chantier = self
                    .partie
                    .lancer_fabrication(plan, contexte.maintenant)
                    .map_err(ErreurChoix::Amelioration)?;
                evenements.push(Evenement::FabricationLancee {
                    plan: Box::new(plan.clone()),
                    chantier,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::FabriquerFilets(filet) => {
                let plan = contexte
                    .contenu
                    .plan_des_filets(*filet)
                    .ok_or(ErreurChoix::PlanInconnu)?;
                if !confirme {
                    // Un travail impossible est refusé sans demander de
                    // confirmation.
                    self.partie
                        .verifier_filets(plan)
                        .map_err(ErreurChoix::Amelioration)?;
                    return Ok(Reponse::a_confirmer(Confirmation::FabriquerFilets(
                        Box::new(plan.clone()),
                    )));
                }
                let chantier = self
                    .partie
                    .lancer_filets(plan, contexte.maintenant)
                    .map_err(ErreurChoix::Amelioration)?;
                evenements.push(Evenement::FiletsLances {
                    plan: Box::new(plan.clone()),
                    chantier,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::FabriquerCharme => {
                let plan = contexte.contenu.plan_du_charme();
                if !confirme {
                    // Un travail impossible est refusé sans demander de
                    // confirmation.
                    self.partie
                        .verifier_charme(plan)
                        .map_err(ErreurChoix::Amelioration)?;
                    return Ok(Reponse::a_confirmer(Confirmation::FabriquerCharme(
                        Box::new(plan.clone()),
                    )));
                }
                let chantier = self
                    .partie
                    .lancer_charme(plan, contexte.maintenant)
                    .map_err(ErreurChoix::Amelioration)?;
                evenements.push(Evenement::CharmeLance {
                    plan: Box::new(plan.clone()),
                    chantier,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::Equiper { place, objet } => {
                let remplace = self.partie.heros().equipement().porte(*place).cloned();
                self.partie
                    .equiper(*place, objet)
                    .map_err(ErreurChoix::Equipement)?;
                let objet = self
                    .partie
                    .heros()
                    .equipement()
                    .porte(*place)
                    .cloned()
                    .expect("l'objet vient d'être porté");
                // Remettre l'objet déjà porté ne change rien à annoncer.
                if remplace.as_ref() != Some(&objet) {
                    evenements.push(Evenement::ObjetEquipe {
                        place: *place,
                        objet,
                        remplace,
                    });
                    sauvegarder(&self.partie, contexte.depot, evenements);
                }
                Ok(Reponse::Faite)
            }
            Choix::RetirerEquipement(place) => {
                let objet = self
                    .partie
                    .heros()
                    .equipement()
                    .porte(*place)
                    .cloned()
                    .ok_or(ErreurChoix::Equipement(ErreurEquipement::PlaceVide(*place)))?;
                self.partie
                    .retirer_equipement(*place)
                    .map_err(ErreurChoix::Equipement)?;
                evenements.push(Evenement::ObjetRetire {
                    place: *place,
                    objet,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::ConstaterTravail => {
                let chantier = self
                    .partie
                    .constater_travail(contexte.maintenant)
                    .map_err(ErreurChoix::Amelioration)?;
                evenements.push(Evenement::TravailConstate(chantier));
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::LancerLaLigne => {
                let fin = self
                    .partie
                    .lancer_la_ligne(contexte.contenu.peche(), contexte.maintenant)
                    .map_err(ErreurChoix::Peche)?;
                evenements.push(Evenement::LigneLancee {
                    fin,
                    delai: contexte.maintenant.duree_jusqu_a(fin),
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::ReleverLaLigne => {
                let contenu = contexte.contenu;
                let zone = self
                    .partie
                    .zone_la_plus_avancee(contenu.zones(), contenu.piste())
                    .expect("la première zone est ouverte dès le départ");
                let prise = self
                    .partie
                    .relever_la_ligne(
                        contenu.peche(),
                        zone,
                        contexte.maintenant,
                        &mut *contexte.aleatoire,
                    )
                    .map_err(ErreurChoix::Peche)?;
                evenements.push(Evenement::PrisePechee(prise));
                // Tout de suite, pour que quitter le jeu ne permette pas de
                // tirer de nouveau la prise.
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::PoserLesPieges(id) => {
                let contenu = contexte.contenu;
                let zone = contenu
                    .zones()
                    .iter()
                    .find(|zone| zone.id() == id)
                    .ok_or_else(|| ErreurChoix::ZoneInconnue(id.clone()))?;
                if !self.partie.est_exploree(zone.id()) {
                    return Err(ErreurChoix::Machine(ErreurMachine::ZoneNonExploree));
                }
                // Déplacés, les pièges sont d'abord relevés.
                let pieges = self.partie.machines().pieges().cloned();
                if let Some(pieges) = pieges.filter(|pieges| pieges.zone() != zone.id()) {
                    let ancienne = contenu
                        .zones()
                        .iter()
                        .find(|zone| zone.id() == pieges.zone())
                        .ok_or_else(|| ErreurChoix::ZoneInconnue(pieges.zone().clone()))?;
                    match self.partie.relever_les_pieges(
                        ancienne,
                        contexte.maintenant,
                        &mut *contexte.aleatoire,
                    ) {
                        Ok(prises) => evenements.push(Evenement::PiegesReleves(prises)),
                        Err(ErreurMachine::PiegesVides) => {}
                        Err(erreur) => return Err(ErreurChoix::Machine(erreur)),
                    }
                }
                self.partie
                    .poser_les_pieges(zone, contexte.maintenant)
                    .map_err(ErreurChoix::Machine)?;
                evenements.push(Evenement::PiegesPoses(zone.nom().to_string()));
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::ReleverLesPieges => {
                let contenu = contexte.contenu;
                let pieges = self
                    .partie
                    .machines()
                    .pieges()
                    .ok_or(ErreurChoix::Machine(ErreurMachine::PiegesNonPoses))?;
                let zone = contenu
                    .zones()
                    .iter()
                    .find(|zone| zone.id() == pieges.zone())
                    .ok_or_else(|| ErreurChoix::ZoneInconnue(pieges.zone().clone()))?;
                let prises = self
                    .partie
                    .relever_les_pieges(zone, contexte.maintenant, &mut *contexte.aleatoire)
                    .map_err(ErreurChoix::Machine)?;
                evenements.push(Evenement::PiegesReleves(prises));
                // Tout de suite, pour que quitter le jeu ne permette pas de
                // tirer de nouveau les ressources.
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::MettreALEntrainement(emplacement) => {
                let contenu = contexte.contenu;
                let plafond = self
                    .partie
                    .plafond_d_entrainement(contenu.zones(), contenu.piste());
                let monstre = self.partie.collection().monstre(*emplacement).cloned();
                self.partie
                    .mettre_a_l_entrainement(*emplacement, plafond, contexte.maintenant)
                    .map_err(ErreurChoix::Machine)?;
                evenements.push(Evenement::EntreeALEntrainement(
                    monstre.expect("le monstre vient d'entrer dans la cour"),
                ));
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::ReprendreDeLEntrainement(position) => {
                let contenu = contexte.contenu;
                let plafond = self
                    .partie
                    .plafond_d_entrainement(contenu.zones(), contenu.piste());
                let reprise = self
                    .partie
                    .reprendre_de_l_entrainement(*position, plafond, contexte.maintenant)
                    .map_err(ErreurChoix::Machine)?;
                evenements.push(Evenement::SortieDeLEntrainement {
                    monstre: reprise.monstre.clone(),
                    experience: reprise.rapport.experience_gagnee,
                    emplacement: reprise.emplacement,
                });
                if reprise.rapport.experience_gagnee > 0 {
                    evenements.push(Evenement::Experience {
                        monstre: reprise.monstre,
                        rapport: reprise.rapport,
                    });
                }
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::LancerLeRite { premier, second } => {
                let especes = contexte.contenu.especes();
                if !confirme {
                    // Un rite impossible est refusé sans demander de
                    // confirmation.
                    let apercu = self
                        .partie
                        .preparer_le_rite(*premier, *second, especes)
                        .map_err(ErreurChoix::Rite)?;
                    return Ok(Reponse::a_confirmer(Confirmation::LancerLeRite(Box::new(
                        apercu,
                    ))));
                }
                let apercu = self
                    .partie
                    .lancer_le_rite(*premier, *second, especes, contexte.maintenant)
                    .map_err(ErreurChoix::Rite)?;
                evenements.push(Evenement::RiteLance(Box::new(apercu)));
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::AccueillirLePetit => {
                let naissance = self
                    .partie
                    .accueillir_le_petit(
                        contexte.contenu.especes(),
                        contexte.maintenant,
                        &mut *contexte.aleatoire,
                    )
                    .map_err(ErreurChoix::Rite)?;
                evenements.push(Evenement::PetitNe(Box::new(naissance)));
                // Tout de suite, pour que quitter le jeu ne permette pas de
                // tirer de nouveau le petit.
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::ReclamerRecompenseDuCodex => {
                let contenu = contexte.contenu;
                let recompense = self
                    .partie
                    .reclamer_recompense_du_codex(contenu.recompenses_du_codex(), contenu.especes())
                    .map_err(ErreurChoix::Codex)?;
                evenements.push(Evenement::RecompenseDuCodex(Box::new(recompense)));
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::FaireEvoluer(emplacement) => {
                if !confirme {
                    // Une évolution impossible est refusée sans demander de
                    // confirmation.
                    let (monstre, vers, pierre) = self
                        .partie
                        .estimer_evolution(*emplacement, contexte.contenu)
                        .map_err(ErreurChoix::Evolution)?;
                    return Ok(Reponse::a_confirmer(Confirmation::Evoluer {
                        monstre,
                        vers,
                        pierre,
                    }));
                }
                let (avant, resultat) = self
                    .partie
                    .faire_evoluer(*emplacement, contexte.contenu)
                    .map_err(ErreurChoix::Evolution)?;
                let apres = self
                    .partie
                    .collection()
                    .monstre(*emplacement)
                    .cloned()
                    .expect("le monstre vient d'évoluer");
                evenements.push(Evenement::Evolution {
                    avant,
                    apres,
                    attaque: resultat.attaque_apprise,
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::DonnerAme(marchand) => {
                let donnee = self
                    .partie
                    .donner_ame(marchand)
                    .map_err(ErreurChoix::Marchand)?;
                evenements.push(Evenement::AmeDonnee {
                    marchand: donnee.nom().to_string(),
                    soin: donnee.soin(),
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::ApprendreAttaque {
                emplacement,
                attaque,
                oubliee,
            } => {
                let ancienne = self
                    .partie
                    .apprendre_attaque(*emplacement, attaque, *oubliee)
                    .map_err(ErreurChoix::Apprentissage)?;
                let monstre = self
                    .partie
                    .collection()
                    .monstre(*emplacement)
                    .cloned()
                    .expect("le monstre vient d'apprendre l'attaque");
                let nouvelle = monstre
                    .attaques()
                    .liste()
                    .iter()
                    .find(|connue| connue.id() == attaque)
                    .cloned()
                    .expect("le monstre connaît l'attaque apprise");
                evenements.push(match ancienne {
                    Some(oubliee) => Evenement::AttaqueRemplacee {
                        monstre,
                        oubliee,
                        nouvelle,
                    },
                    None => Evenement::AttaqueApprise {
                        monstre,
                        attaque: nouvelle,
                    },
                });
                sauvegarder(&self.partie, contexte.depot, evenements);
                Ok(Reponse::Faite)
            }
            Choix::SeSuicider => {
                if !confirme {
                    let heros = self.partie.heros().clone();
                    return Ok(Reponse::a_confirmer(Confirmation::Suicide(heros)));
                }
                if let Err(erreur) = contexte.depot.supprimer() {
                    evenements.push(Evenement::EffacementImpossible(erreur));
                }
                evenements.push(Evenement::Suicide);
                Ok(Reponse::NouvellePartie)
            }
            _ => Err(ErreurChoix::HorsDePropos),
        }
    }

    fn expedition_automatique(&self, zone: &ZoneId) -> Result<ExpeditionAutomatique, ErreurChoix> {
        self.partie
            .expedition_automatique(zone)
            .cloned()
            .ok_or(ErreurChoix::ExpeditionAutomatique(
                ErreurExpeditionAutomatique::Inexistante,
            ))
    }

    /// Comme après un combat (Q33), sur un monstre rentré dans l'équipe ou
    /// au camp. Chaque
    /// choix est sauvegardé (section 12.7).
    fn choisir_attaque(
        &mut self,
        choix: &Choix,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        let Some(a_choisir) = self.partie.attaques_a_choisir().first().cloned() else {
            return Err(ErreurChoix::HorsDePropos);
        };
        let emplacement = a_choisir.emplacement;
        let monstre = self
            .partie
            .collection_mut()
            .monstre_a_modifier(emplacement)
            .ok_or(ErreurChoix::Collection(
                ErreurCollection::EmplacementInexistant(emplacement),
            ))?;
        match choix {
            Choix::OublierAttaque(position) => {
                let oubliee = monstre.attaques().liste().get(*position).cloned().ok_or(
                    ErreurChoix::Attaque(ErreurAttaquesMonstre::PositionInexistante(*position)),
                )?;
                monstre
                    .remplacer_attaque(*position, a_choisir.attaque.clone())
                    .map_err(ErreurChoix::Attaque)?;
                evenements.push(Evenement::AttaqueRemplacee {
                    monstre: monstre.clone(),
                    oubliee,
                    nouvelle: a_choisir.attaque,
                });
            }
            Choix::RenoncerALAttaque => evenements.push(Evenement::AttaqueAbandonnee {
                monstre: monstre.clone(),
                nouvelle: a_choisir.attaque,
            }),
            _ => return Err(ErreurChoix::HorsDePropos),
        }
        self.partie.attaque_choisie();
        sauvegarder(&self.partie, contexte.depot, evenements);
        Ok(Reponse::Faite)
    }

    /// Le monstre pêché rejoint l'équipe ou le camp, ou retourne à l'eau ;
    /// chaque choix est sauvegardé (section 13.8).
    fn choisir_prise(
        &mut self,
        choix: &Choix,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        match choix {
            Choix::GarderLaPrise => {
                let placement = self
                    .partie
                    .garder_la_prise()
                    .ok_or(ErreurChoix::HorsDePropos)?;
                let sans_place = annoncer_placement(&self.partie, placement, evenements);
                debug_assert!(sans_place.is_none(), "place vérifiée en relevant la ligne");
            }
            Choix::RelacherLaPrise => {
                let prise = self
                    .partie
                    .relacher_la_prise()
                    .ok_or(ErreurChoix::HorsDePropos)?;
                evenements.push(Evenement::PriseRelachee(prise));
            }
            _ => return Err(ErreurChoix::HorsDePropos),
        }
        sauvegarder(&self.partie, contexte.depot, evenements);
        Ok(Reponse::Faite)
    }

    /// Soin de tous les monstres, indice éventuel, nouvelle zone, puis
    /// sauvegarde (sections 9.4, 13.1 et 13.9).
    /// Le premier événement de l'expédition suit aussitôt le départ.
    fn lancer(
        &mut self,
        expedition: Expedition,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) {
        evenements.push(Evenement::Depart(expedition.zone().nom().to_string()));
        if let Some(jour_ou_nuit) = expedition.jour_ou_nuit() {
            evenements.push(Evenement::JourOuNuitAuDepart(jour_ou_nuit));
        }
        if let Some(meteo) = expedition.meteo() {
            evenements.push(Evenement::MeteoAuDepart(meteo));
        }
        if let Some(tour) = expedition.tour() {
            evenements.push(Evenement::EntreeDansLaTour(
                tour.recit_de_l_entree().to_string(),
            ));
        }
        let mut en_cours = EnExpedition {
            expedition,
            moment: Moment::EntreDeuxEtapes,
            en_groupe: false,
            ame: None,
            pecheur: None,
        };
        let reponse = en_cours.etape_suivante(&mut self.partie, contexte, evenements);
        let reponse = en_cours.annoncer_l_indice(&self.partie, contexte.contenu.piste(), reponse);
        self.expedition = Some(en_cours);
        if reponse == Reponse::RetourAuCamp {
            self.rentrer(contexte, evenements);
        }
    }

    fn partir_vers_l_eclipse(&self, contexte: &Contexte<'_>) -> Result<Expedition, ErreurChoix> {
        self.partie
            .partir_vers_l_eclipse(contexte.contenu.invocation())
            .map_err(|erreur| match erreur {
                ErreurExpedition::ZoneFermee => ErreurChoix::Beherit(ErreurBeherit::EclipseFermee),
                ErreurExpedition::EquipeVide => ErreurChoix::EquipeVide,
                ErreurExpedition::SacTropPlein { exces } => ErreurChoix::SacTropPlein { exces },
            })
    }

    fn rentrer(&mut self, contexte: &mut Contexte<'_>, evenements: &mut Vec<Evenement>) {
        let Some(en_cours) = self.expedition.take() else {
            return;
        };
        let piste = contexte.contenu.piste();
        let zones = contexte.contenu.zones();
        let avant = self.partie.zones_accessibles(zones, piste).len();
        let indice = self.partie.rentrer_au_camp(&en_cours.expedition, piste);
        let location_finie = self.partie.decompter_location();
        // L'Éclipse ne compte jamais comme explorée (section 9.4).
        let lieu_d_invocation = en_cours.expedition.zone().est_un_lieu_d_invocation();
        if en_cours.expedition.est_reussie() && !lieu_d_invocation {
            evenements.push(Evenement::ZoneExploree(
                en_cours.expedition.zone().nom().to_string(),
            ));
        }
        // La première expédition menée au bout ne peut être que dans la
        // Lisière, seule zone ouverte avant elle ; Malvoise rentre au camp
        // avec le joueur (section 14.2).
        if en_cours.expedition.est_reussie() && !self.partie.marchand_au_camp() {
            self.partie.accueillir_malvoise();
            evenements.push(Evenement::MalvoiseRencontre {
                avec_une_ame: !self.partie.ames_portees().is_empty(),
            });
        }
        evenements.push(Evenement::RetourAuCamp);
        if en_cours.expedition.est_sans_fin() {
            let franchies = en_cours.expedition.etapes_franchies();
            if self
                .partie
                .noter_record_de_la_tour(franchies, en_cours.en_groupe)
            {
                evenements.push(Evenement::NouveauRecordDeLaTour(franchies));
            }
        }
        if let Some(monstre) = location_finie {
            evenements.push(Evenement::LocationFinie(monstre));
        }
        if let Some(indice) = indice {
            evenements.push(Evenement::Indice {
                numero: self.partie.indices_trouves(),
                texte: indice.texte.clone(),
            });
        }
        if let Some(nouvelle) = self.partie.zones_accessibles(zones, piste).get(avant) {
            evenements.push(Evenement::NouvelleZone(nouvelle.nom().to_string()));
        }
        montrer_conseil(&mut self.partie, Conseil::PremierRetourAuCamp, evenements);
        sauvegarder(&self.partie, contexte.depot, evenements);
    }
}

/// Une action refusée par la collection ne change rien.
fn gerer(
    partie: &mut Partie,
    action: &ActionDeGestion,
    confirme: bool,
    evenements: &mut Vec<Evenement>,
) -> Result<Reponse, ErreurChoix> {
    let collection = partie.collection_mut();
    match action {
        ActionDeGestion::ChangerLOrdre { de, vers } => {
            collection.deplacer_dans_l_equipe(*de, *vers)
        }
        ActionDeGestion::EnvoyerAuCamp(index) => collection.envoyer_au_camp(*index),
        ActionDeGestion::FaireVenir(index) => collection.faire_venir(*index),
        ActionDeGestion::Echanger { equipe, camp } => collection.echanger(*equipe, *camp),
        ActionDeGestion::Renommer {
            emplacement,
            surnom,
        } => {
            let surnom = if surnom.trim().is_empty() {
                None
            } else {
                Some(Nom::new(surnom).map_err(ErreurChoix::Nom)?)
            };
            collection.renommer(*emplacement, surnom)
        }
        ActionDeGestion::Relacher(emplacement) => {
            if !confirme {
                // Un relâcher impossible, comme celui du dernier monstre de
                // l'équipe, est refusé sans demander de confirmation.
                let monstre = collection
                    .clone()
                    .relacher(*emplacement)
                    .map_err(ErreurChoix::Collection)?;
                return Ok(Reponse::a_confirmer(Confirmation::Relacher(monstre)));
            }
            collection
                .relacher(*emplacement)
                .map(|relache| evenements.push(Evenement::Relache(relache)))
        }
        ActionDeGestion::TrierLeCamp(tri) => {
            collection.trier_le_camp(*tri);
            Ok(())
        }
    }
    .map_err(ErreurChoix::Collection)?;
    Ok(Reponse::Faite)
}

struct EnExpedition {
    expedition: Expedition,
    moment: Moment,
    /// Dans la Tour sans fond, à deux : les étapes et le combat viennent de
    /// l'expédition en groupe (section 15.5).
    en_groupe: bool,
    /// Le marchand mort dont le combat en cours peut recueillir l'âme.
    ame: Option<MarchandMort>,
    /// Ansgar, que le combat en cours peut sauver.
    pecheur: Option<Pecheur>,
}

enum Moment {
    EntreDeuxEtapes,
    MonstreRare(Monstre),
    /// Le combat contre un humain de fin de zone ou un Apôtre, prêt à
    /// commencer quand le joueur le veut (section 9.2).
    AnnonceDeCombat(Combat),
    Combat(Combat),
    OubliAttaque(OubliEnCours),
    Evolution(EvolutionEnCours),
    PlaceManquante(CaptureSansPlace),
    /// L'expédition est finie et révèle un indice au retour (section 9.2).
    AnnonceDIndice,
    /// En groupe, le combat commun est en cours.
    CombatDeGroupe {
        contre_un_gardien: bool,
    },
    /// En groupe, le joueur veut continuer et attend l'autre.
    AttenteDuGroupe,
    /// À la fin de l'étape, le sac déborde : le joueur jette des objets
    /// avant de continuer, ou de rentrer après la dernière étape (section
    /// 13.3).
    SacPlein {
        puis_rentrer: bool,
    },
}

/// Ce qui reste à régler une fois les suites d'un combat terminées.
#[derive(Debug, Clone)]
struct FinDeCombat {
    issue: Option<Issue>,
    contre_un_apotre: bool,
    contre_un_humain: bool,
    contre_un_gardien: bool,
    contre_un_legendaire: bool,
    /// Le contenu du sac, perdu après une défaite (section 9.3).
    sac_perdu: Inventaire,
}

/// Lecture du bilan d'un combat : un monstre veut apprendre une attaque de
/// plus que les 4 qu'il connaît (Q33).
struct OubliEnCours {
    bilan: BilanCombat,
    fin: FinDeCombat,
    /// Position du monstre dans l'équipe du bilan.
    monstre: usize,
    nouvelle: Attaque,
    /// Attaques qu'il voudra apprendre ensuite.
    suivantes: VecDeque<Attaque>,
    /// L'attaque d'évolution : après son choix, le bilan passe au monstre
    /// suivant, sans proposer d'autre évolution.
    apres_evolution: bool,
}

/// Lecture du bilan d'un combat : l'évolution d'un monstre est proposée
/// (section 5.4).
struct EvolutionEnCours {
    bilan: BilanCombat,
    fin: FinDeCombat,
    /// Position du monstre dans l'équipe du bilan.
    monstre: usize,
    vers: Espece,
}

/// Capture qui ne trouve pas de place, et celles qui restent à placer.
struct CaptureSansPlace {
    capture: Monstre,
    restantes: VecDeque<Monstre>,
    /// `None` pour un monstre rare pris sans combat.
    fin: Option<FinDeCombat>,
}

impl EnExpedition {
    fn attente(&self) -> Attente<'_> {
        match &self.moment {
            Moment::EntreDeuxEtapes => Attente::EntreDeuxEtapes,
            Moment::MonstreRare(monstre) => Attente::MonstreRare(monstre),
            Moment::AnnonceDeCombat(_) => Attente::Annonce(Annonce::Combat),
            Moment::AnnonceDIndice => Attente::Annonce(Annonce::Indice),
            Moment::CombatDeGroupe { .. } => Attente::CombatDeGroupe,
            Moment::AttenteDuGroupe => Attente::AttenteDuGroupe,
            Moment::SacPlein { .. } => Attente::SacPlein,
            Moment::Combat(combat) => match combat.phase() {
                Phase::ChoixAction => Attente::ActionDeCombat(combat),
                Phase::Remplacement => Attente::Remplacement(combat),
                Phase::Termine(_) => Attente::CaptureDesKo(combat),
            },
            Moment::OubliAttaque(oubli) => Attente::OubliAttaque {
                monstre: &oubli.bilan.equipe[oubli.monstre],
                nouvelle: &oubli.nouvelle,
            },
            Moment::Evolution(evolution) => Attente::Evolution {
                monstre: &evolution.bilan.equipe[evolution.monstre],
                vers: &evolution.vers,
            },
            Moment::PlaceManquante(attente) => Attente::PlaceManquante(&attente.capture),
        }
    }

    /// Chaque fonction appelée remet le moment en place si le choix est
    /// refusé.
    fn appliquer(
        &mut self,
        partie: &mut Partie,
        choix: &Choix,
        confirme: bool,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        let reponse = match std::mem::replace(&mut self.moment, Moment::EntreDeuxEtapes) {
            Moment::EntreDeuxEtapes => {
                self.entre_deux_etapes(partie, choix, confirme, contexte, evenements)
            }
            Moment::MonstreRare(monstre) => self.monstre_rare(partie, monstre, choix, evenements),
            Moment::AnnonceDeCombat(combat) => {
                if *choix != Choix::Poursuivre {
                    self.moment = Moment::AnnonceDeCombat(combat);
                    return Err(ErreurChoix::HorsDePropos);
                }
                Ok(self.engager(partie, combat, evenements))
            }
            Moment::Combat(combat) => {
                self.combattre(partie, combat, choix, confirme, contexte, evenements)
            }
            Moment::OubliAttaque(oubli) => self.oublier(partie, oubli, choix, contexte, evenements),
            Moment::Evolution(evolution) => {
                self.evoluer(partie, evolution, choix, contexte, evenements)
            }
            Moment::PlaceManquante(attente) => {
                self.faire_place(partie, attente, choix, confirme, evenements)
            }
            Moment::AnnonceDIndice => {
                if *choix != Choix::Poursuivre {
                    self.moment = Moment::AnnonceDIndice;
                    return Err(ErreurChoix::HorsDePropos);
                }
                return Ok(Reponse::RetourAuCamp);
            }
            // Le combat commun se joue dans l'expédition en groupe.
            moment @ Moment::CombatDeGroupe { .. } => {
                self.moment = moment;
                return Err(ErreurChoix::HorsDePropos);
            }
            Moment::AttenteDuGroupe => {
                if *choix != Choix::RentrerAuCamp {
                    self.moment = Moment::AttenteDuGroupe;
                    return Err(ErreurChoix::HorsDePropos);
                }
                return Ok(Reponse::RetourAuCamp);
            }
            Moment::SacPlein { puis_rentrer } => {
                self.vider_le_sac(partie, puis_rentrer, choix, evenements)
            }
        }?;
        Ok(self.annoncer_l_indice(partie, contexte.contenu.piste(), reponse))
    }

    /// Une expédition menée au bout qui révèle un indice l'annonce d'abord
    /// par un bouton ; le retour au camp attend le joueur (section 9.2).
    fn annoncer_l_indice(&mut self, partie: &Partie, piste: &Piste, reponse: Reponse) -> Reponse {
        let indice_a_lire = reponse == Reponse::RetourAuCamp
            && self.expedition.est_reussie()
            && piste
                .indice_suivant(partie.indices_trouves(), partie.apotres_vaincus())
                .is_some_and(|indice| &indice.zone == self.expedition.zone().id());
        if indice_a_lire {
            self.moment = Moment::AnnonceDIndice;
            Reponse::Faite
        } else {
            reponse
        }
    }

    /// Entre deux étapes, le moment reste le même tant que le joueur ne
    /// continue ni ne rentre (section 9.2).
    fn entre_deux_etapes(
        &mut self,
        partie: &mut Partie,
        choix: &Choix,
        confirme: bool,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        match choix {
            // En groupe, continuer attend l'accord de l'autre (section 15.5).
            Choix::ContinuerLExpedition if self.en_groupe => {
                self.moment = Moment::AttenteDuGroupe;
                Ok(Reponse::Faite)
            }
            Choix::ContinuerLExpedition => Ok(self.etape_suivante(partie, contexte, evenements)),
            Choix::UtiliserSoin(soin, index) => {
                let cible = partie.cible_du_soin(*index);
                if let Some(confirmation) =
                    soin_a_confirmer(*soin, partie.sac().soins(), cible.as_ref(), confirme)
                {
                    return Ok(Reponse::a_confirmer(confirmation));
                }
                let effet = partie
                    .utiliser_soin(*soin, *index, &mut *contexte.aleatoire)
                    .map_err(ErreurChoix::Soin)?;
                evenements.push(Evenement::SoinUtilise {
                    soin: *soin,
                    monstre: partie
                        .cible_du_soin(*index)
                        .expect("la cible vient d'être soignée"),
                    effet,
                });
                Ok(Reponse::Faite)
            }
            Choix::Gerer(ActionDeGestion::ChangerLOrdre { de, vers }) => {
                partie
                    .collection_mut()
                    .deplacer_dans_l_equipe(*de, *vers)
                    .map_err(ErreurChoix::Collection)?;
                Ok(Reponse::Faite)
            }
            Choix::RentrerAuCamp => Ok(Reponse::RetourAuCamp),
            _ => Err(ErreurChoix::HorsDePropos),
        }
    }

    /// L'argent ne prend pas de place : il ne se jette pas.
    fn vider_le_sac(
        &mut self,
        partie: &mut Partie,
        puis_rentrer: bool,
        choix: &Choix,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        self.moment = Moment::SacPlein { puis_rentrer };
        let Choix::Jeter(objet) = choix else {
            return Err(ErreurChoix::HorsDePropos);
        };
        if matches!(objet, Objet::Argent(_)) {
            return Err(ErreurChoix::HorsDePropos);
        }
        let jete = partie.jeter(objet).map_err(ErreurChoix::Objet)?;
        evenements.push(Evenement::ObjetsJetes(jete));
        Ok(self.apres_l_etape(partie, evenements))
    }

    fn monstre_rare(
        &mut self,
        partie: &mut Partie,
        monstre: Monstre,
        choix: &Choix,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        match choix {
            Choix::PrendreLeMonstreRare => {
                let placement = partie.prendre_monstre_rare(monstre);
                match annoncer_placement(partie, placement, evenements) {
                    Some(capture) => {
                        self.moment = Moment::PlaceManquante(CaptureSansPlace {
                            capture,
                            restantes: VecDeque::new(),
                            fin: None,
                        });
                        Ok(Reponse::Faite)
                    }
                    None => Ok(self.apres_l_etape(partie, evenements)),
                }
            }
            Choix::LaisserLeMonstreRare => {
                evenements.push(Evenement::MonstreRareLaisse);
                Ok(self.apres_l_etape(partie, evenements))
            }
            _ => {
                self.moment = Moment::MonstreRare(monstre);
                Err(ErreurChoix::HorsDePropos)
            }
        }
    }

    fn combattre(
        &mut self,
        partie: &mut Partie,
        mut combat: Combat,
        choix: &Choix,
        confirme: bool,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        if *choix == Choix::DonnerOrdre {
            let resultat = combat.donner_ordre();
            let reponse = match resultat {
                Ok((ordre, cible)) => {
                    evenements.push(Evenement::OrdreDonne {
                        heros: partie.heros().nom().as_str().to_string(),
                        ordre,
                        monstre: combat.monstre(cible).clone(),
                    });
                    Ok(Reponse::Faite)
                }
                Err(erreur) => Err(ErreurChoix::Combat(erreur)),
            };
            self.moment = Moment::Combat(combat);
            return reponse;
        }
        let resultat = match (combat.phase(), choix) {
            (Phase::ChoixAction, Choix::Combat(action)) => {
                if let Action::UtiliserSoin(soin, index) = *action {
                    let confirmation = soin_a_confirmer(
                        soin,
                        combat.soins(),
                        combat.combattant_du_joueur(index),
                        confirme,
                    );
                    if let Some(confirmation) = confirmation {
                        self.moment = Moment::Combat(combat);
                        return Ok(Reponse::a_confirmer(confirmation));
                    }
                }
                combat.jouer_tour(*action, &mut *contexte.aleatoire)
            }
            (Phase::Remplacement, Choix::Remplacer(index)) => combat.remplacer(*index),
            (Phase::Termine(_), Choix::Capturer(index)) => combat.capturer_ko(*index),
            (Phase::Termine(_), Choix::NePlusCapturer) => Ok(Vec::new()),
            _ => {
                self.moment = Moment::Combat(combat);
                return Err(ErreurChoix::HorsDePropos);
            }
        };
        let tour = match resultat {
            Ok(tour) => tour,
            Err(erreur) => {
                self.moment = Moment::Combat(combat);
                return Err(ErreurChoix::Combat(erreur));
            }
        };
        // La seconde forme du Démiurge entre au codex en apparaissant (Q99).
        for evenement in &tour {
            if let combat::Evenement::Metamorphose(forme) = evenement {
                partie.noter_rencontre(combat.monstre(*forme));
            }
        }
        if !tour.is_empty() {
            evenements.push(Evenement::Combat {
                evenements: tour,
                combat: combat.clone(),
            });
        }
        // Vaincu, le légendaire est capturé d'office, sans filet (section
        // 9.10).
        if combat.est_contre_un_legendaire() {
            for index in combat.adversaires_capturables() {
                if let Ok(capture) = combat.capturer_ko(index) {
                    evenements.push(Evenement::Combat {
                        evenements: capture,
                        combat: combat.clone(),
                    });
                }
            }
        }
        // Après une victoire, le joueur capture les monstres KO qu'il veut
        // avant que le combat se termine (Q29).
        let fini = *choix == Choix::NePlusCapturer
            || (matches!(combat.phase(), Phase::Termine(_))
                && combat.adversaires_capturables().is_empty());
        if !fini {
            self.moment = Moment::Combat(combat);
            return Ok(Reponse::Faite);
        }
        let fin = FinDeCombat {
            issue: None,
            contre_un_apotre: combat.est_contre_un_apotre(),
            contre_un_humain: combat.est_contre_un_humain(),
            contre_un_gardien: combat.est_contre_un_gardien(),
            contre_un_legendaire: combat.est_contre_un_legendaire(),
            sac_perdu: Inventaire::default(),
        };
        let bilan = combat.terminer(&mut *contexte.aleatoire);
        let fin = FinDeCombat {
            issue: bilan.issue,
            ..fin
        };
        Ok(self.lire_le_bilan(partie, bilan, fin, 0, contexte, evenements))
    }

    /// XP de chaque monstre, dans l'ordre de l'équipe, avec les attaques à
    /// apprendre ; puis l'équipe revient dans la partie et les captures sont
    /// placées.
    fn lire_le_bilan(
        &mut self,
        partie: &mut Partie,
        bilan: BilanCombat,
        fin: FinDeCombat,
        depuis: usize,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Reponse {
        for index in depuis..bilan.experience.len() {
            let Some(rapport) = bilan.experience[index].clone() else {
                continue;
            };
            evenements.push(Evenement::Experience {
                monstre: bilan.equipe[index].clone(),
                rapport: rapport.clone(),
            });
            let mut attaques: VecDeque<Attaque> = rapport.attaques_en_attente.into();
            if let Some(nouvelle) = attaques.pop_front() {
                self.moment = Moment::OubliAttaque(OubliEnCours {
                    bilan,
                    fin,
                    monstre: index,
                    nouvelle,
                    suivantes: attaques,
                    apres_evolution: false,
                });
                return Reponse::Faite;
            }
            let circonstances = Circonstances {
                zone: Some(self.expedition.zone().id()),
                apotres_vaincus: partie.apotres_vaincus(),
                jour_ou_nuit: self.expedition.jour_ou_nuit(),
                meteo: self.expedition.meteo(),
            };
            if let Some(vers) = evolution_a_proposer(&bilan, index, &circonstances, contexte) {
                self.moment = Moment::Evolution(EvolutionEnCours {
                    bilan,
                    fin,
                    monstre: index,
                    vers,
                });
                return Reponse::Faite;
            }
        }
        let vaincus = bilan.vaincus_sauvages.clone();
        let (captures, sac_perdu) = partie
            .conclure_combat(bilan)
            .expect("le combat rend l'équipe qu'il a reçue");
        if fin.issue == Some(Issue::Victoire) {
            if let Some(fouineur) = partie.fouiller_apres_une_victoire(&mut *contexte.aleatoire) {
                evenements.push(Evenement::PotionTrouvee(fouineur));
            }
            let ressources = partie.recolter_ressources(&vaincus, &mut *contexte.aleatoire);
            if !ressources.is_empty() {
                evenements.push(Evenement::RessourcesTrouvees(ressources));
            }
        }
        let fin = FinDeCombat { sac_perdu, ..fin };
        self.placer(partie, captures.into(), Some(fin), evenements)
    }

    fn oublier(
        &mut self,
        partie: &mut Partie,
        mut oubli: OubliEnCours,
        choix: &Choix,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        let monstre = &mut oubli.bilan.equipe[oubli.monstre];
        match choix {
            Choix::OublierAttaque(position) => {
                let Some(oubliee) = monstre.attaques().liste().get(*position).cloned() else {
                    self.moment = Moment::OubliAttaque(oubli);
                    return Err(ErreurChoix::Attaque(
                        ErreurAttaquesMonstre::PositionInexistante(*position),
                    ));
                };
                if let Err(erreur) = monstre.remplacer_attaque(*position, oubli.nouvelle.clone()) {
                    self.moment = Moment::OubliAttaque(oubli);
                    return Err(ErreurChoix::Attaque(erreur));
                }
                evenements.push(Evenement::AttaqueRemplacee {
                    monstre: monstre.clone(),
                    oubliee,
                    nouvelle: oubli.nouvelle.clone(),
                });
            }
            Choix::RenoncerALAttaque => evenements.push(Evenement::AttaqueAbandonnee {
                monstre: monstre.clone(),
                nouvelle: oubli.nouvelle.clone(),
            }),
            _ => {
                self.moment = Moment::OubliAttaque(oubli);
                return Err(ErreurChoix::HorsDePropos);
            }
        }
        if let Some(nouvelle) = oubli.suivantes.pop_front() {
            oubli.nouvelle = nouvelle;
            self.moment = Moment::OubliAttaque(oubli);
            return Ok(Reponse::Faite);
        }
        if !oubli.apres_evolution {
            let circonstances = Circonstances {
                zone: Some(self.expedition.zone().id()),
                apotres_vaincus: partie.apotres_vaincus(),
                jour_ou_nuit: self.expedition.jour_ou_nuit(),
                meteo: self.expedition.meteo(),
            };
            let vers = evolution_a_proposer(&oubli.bilan, oubli.monstre, &circonstances, contexte);
            if let Some(vers) = vers {
                self.moment = Moment::Evolution(EvolutionEnCours {
                    bilan: oubli.bilan,
                    fin: oubli.fin,
                    monstre: oubli.monstre,
                    vers,
                });
                return Ok(Reponse::Faite);
            }
        }
        let suivant = oubli.monstre + 1;
        Ok(self.lire_le_bilan(
            partie,
            oubli.bilan,
            oubli.fin,
            suivant,
            contexte,
            evenements,
        ))
    }

    /// Une évolution acceptée peut encore demander de choisir l'attaque à
    /// oublier ; repoussée, elle sera proposée au prochain niveau gagné.
    fn evoluer(
        &mut self,
        partie: &mut Partie,
        mut evolution: EvolutionEnCours,
        choix: &Choix,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        let monstre = &mut evolution.bilan.equipe[evolution.monstre];
        match choix {
            Choix::Evoluer => {
                let avant = monstre.clone();
                partie.noter_evolution(evolution.vers.id());
                let resultat = faire_evoluer(monstre, evolution.vers);
                evenements.push(Evenement::Evolution {
                    avant,
                    apres: monstre.clone(),
                    attaque: resultat.attaque_apprise,
                });
                if let Some(nouvelle) = resultat.attaque_en_attente {
                    self.moment = Moment::OubliAttaque(OubliEnCours {
                        bilan: evolution.bilan,
                        fin: evolution.fin,
                        monstre: evolution.monstre,
                        nouvelle,
                        suivantes: VecDeque::new(),
                        apres_evolution: true,
                    });
                    return Ok(Reponse::Faite);
                }
            }
            Choix::NePasEvoluer => {
                evenements.push(Evenement::EvolutionRepoussee(monstre.clone()));
            }
            _ => {
                self.moment = Moment::Evolution(evolution);
                return Err(ErreurChoix::HorsDePropos);
            }
        }
        let suivant = evolution.monstre + 1;
        Ok(self.lire_le_bilan(
            partie,
            evolution.bilan,
            evolution.fin,
            suivant,
            contexte,
            evenements,
        ))
    }

    /// Dans l'équipe, sinon au camp ; si tout est plein, le joueur choisit
    /// (Q12).
    fn placer(
        &mut self,
        partie: &mut Partie,
        mut restantes: VecDeque<Monstre>,
        fin: Option<FinDeCombat>,
        evenements: &mut Vec<Evenement>,
    ) -> Reponse {
        while let Some(capture) = restantes.pop_front() {
            let placement = partie.collection_mut().accueillir_capture(capture);
            if let Some(capture) = annoncer_placement(partie, placement, evenements) {
                self.moment = Moment::PlaceManquante(CaptureSansPlace {
                    capture,
                    restantes,
                    fin,
                });
                return Reponse::Faite;
            }
        }
        self.conclure(partie, fin, evenements)
    }

    fn faire_place(
        &mut self,
        partie: &mut Partie,
        attente: CaptureSansPlace,
        choix: &Choix,
        confirme: bool,
        evenements: &mut Vec<Evenement>,
    ) -> Result<Reponse, ErreurChoix> {
        match choix {
            Choix::RelacherLaCapture => {
                evenements.push(Evenement::CaptureRelachee(attente.capture));
            }
            Choix::RelacherPourLaCapture(emplacement) => {
                let Some(monstre) = partie.collection().monstre(*emplacement).cloned() else {
                    self.moment = Moment::PlaceManquante(attente);
                    return Err(ErreurChoix::Collection(
                        ErreurCollection::EmplacementInexistant(*emplacement),
                    ));
                };
                if !confirme {
                    self.moment = Moment::PlaceManquante(attente);
                    return Ok(Reponse::a_confirmer(Confirmation::Relacher(monstre)));
                }
                match partie
                    .collection_mut()
                    .relacher_pour_capture(*emplacement, attente.capture.clone())
                {
                    Ok(relache) => evenements.push(Evenement::RelachePourLaCapture {
                        relache,
                        capture: attente.capture,
                    }),
                    Err(erreur) => {
                        self.moment = Moment::PlaceManquante(attente);
                        return Err(ErreurChoix::Collection(erreur));
                    }
                }
            }
            _ => {
                self.moment = Moment::PlaceManquante(attente);
                return Err(ErreurChoix::HorsDePropos);
            }
        }
        Ok(self.placer(partie, attente.restantes, attente.fin, evenements))
    }

    /// L'Apôtre vaincu ne réapparaît plus et l'épilogue suit ; un humain
    /// vaincu non plus, et son argent va dans le sac ; une victoire recueille
    /// l'âme du marchand mort, que la fuite ou la défaite laisse à retrouver,
    /// ou installe Ansgar au camp ; une défaite met fin à l'expédition
    /// (sections 9.2, 9.3, 9.4, 10.7 et 14.2).
    fn conclure(
        &mut self,
        partie: &mut Partie,
        fin: Option<FinDeCombat>,
        evenements: &mut Vec<Evenement>,
    ) -> Reponse {
        if let Some(fin) = fin {
            if fin.contre_un_apotre && fin.issue == Some(Issue::Victoire) {
                let apotre = self
                    .expedition
                    .apotre()
                    .expect("un combat contre un Apôtre suit l'antre de l'expédition")
                    .clone();
                evenements.push(Evenement::Epilogue(apotre.epilogue().to_string()));
                // Le Démiurge ne laisse pas d'âme, mais ses Philtres chromatiques
                // (section 9.4).
                if self.expedition.zone().est_un_lieu_d_invocation() {
                    partie.noter_demiurge_vaincu(&apotre);
                    evenements.push(Evenement::RecompenseDuDemiurge);
                } else {
                    partie.noter_apotre_vaincu(&apotre);
                    evenements.push(Evenement::AmeDApotreRecueillie(
                        apotre.recit_de_l_ame().to_string(),
                    ));
                    evenements.push(Evenement::PhiltreChromatiqueRamasse);
                }
            }
            // Capturé, le légendaire ne revient plus (section 9.10).
            if fin.contre_un_legendaire && fin.issue == Some(Issue::Victoire) {
                if let Some(legendaire) = self.expedition.legendaire().cloned() {
                    partie.noter_legendaire_capture(legendaire.espece().id());
                    evenements.push(Evenement::LegendaireCapture(
                        legendaire.capture().to_string(),
                    ));
                }
            }
            // Le repos qui suit le gardien de la Tour soigne tout (section
            // 9.5).
            if fin.contre_un_gardien && fin.issue == Some(Issue::Victoire) {
                partie.collection_mut().soigner_tout();
                partie.soigner_le_heros();
                if let Some(tour) = self.expedition.tour() {
                    evenements.push(Evenement::ReposDansLaTour(
                        tour.recit_du_repos().to_string(),
                    ));
                }
            }
            let humain = self.expedition.humain().cloned();
            if let Some(humain) =
                humain.filter(|_| fin.contre_un_humain && fin.issue == Some(Issue::Victoire))
            {
                let argent = partie.vaincre_humain(&humain);
                evenements.push(Evenement::HumainVaincu {
                    texte: humain.apres_victoire().to_string(),
                    argent,
                });
                if let Some(texte) = humain.liberation_de_gunther() {
                    partie.accueillir_gunther();
                    evenements.push(Evenement::GuntherLibere(texte.to_string()));
                }
            }
            // Une fuite ou une défaite laisse Ansgar à sauver à l'expédition
            // suivante, à la même étape.
            if let Some(pecheur) = self.pecheur.take() {
                if fin.issue == Some(Issue::Victoire) {
                    partie.accueillir_ansgar();
                    evenements.push(Evenement::PecheurSauve(
                        pecheur.apres_victoire().to_string(),
                    ));
                }
            }
            // Une fuite ou une défaite laisse l'âme à retrouver.
            if let Some(marchand) = self.ame.take() {
                if fin.issue == Some(Issue::Victoire) {
                    evenements.push(Evenement::AmeRecueillie(
                        marchand.apres_victoire().to_string(),
                    ));
                    partie.recueillir_ame(marchand);
                } else {
                    partie.noter_ame_a_retrouver(marchand.id());
                }
            }
            if fin.issue == Some(Issue::Defaite) {
                self.expedition.noter_defaite();
                evenements.push(Evenement::Defaite {
                    sac_perdu: fin.sac_perdu,
                });
            }
        }
        self.apres_l_etape(partie, evenements)
    }

    /// Un sac qui déborde se vide d'abord : le joueur jette des objets
    /// jusqu'à revenir à sa capacité (section 13.3). Après une défaite, le
    /// sac est perdu, et rien ne déborde.
    fn apres_l_etape(&mut self, partie: &mut Partie, evenements: &mut Vec<Evenement>) -> Reponse {
        let terminee = self.expedition.est_terminee();
        if partie.exces_du_sac() > 0 {
            self.moment = Moment::SacPlein {
                puis_rentrer: terminee,
            };
            return Reponse::Faite;
        }
        if terminee {
            return Reponse::RetourAuCamp;
        }
        self.moment = Moment::EntreDeuxEtapes;
        montrer_conseil(partie, Conseil::PremierArretEntreDeuxEtapes, evenements);
        Reponse::Faite
    }

    fn etape_suivante(
        &mut self,
        partie: &mut Partie,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Reponse {
        let Some(evenement) = self.expedition.etape_suivante(&mut *contexte.aleatoire) else {
            return Reponse::RetourAuCamp;
        };
        self.traiter_l_evenement(partie, evenement, contexte, evenements)
    }

    fn traiter_l_evenement(
        &mut self,
        partie: &mut Partie,
        evenement: EvenementExpedition,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Reponse {
        evenements.push(Evenement::Etape {
            numero: self.expedition.etapes_faites(),
            total: (!self.expedition.est_sans_fin()).then_some(self.expedition.nombre_etapes()),
        });
        if self.en_groupe {
            if let Some(reponse) = self.preparer_le_combat_de_groupe(partie, &evenement, evenements)
            {
                return reponse;
            }
        }
        let contenu = contexte.contenu;
        let limite = partie.limite_d_obeissance(contenu.zones(), contenu.piste());
        match evenement {
            EvenementExpedition::Combat(groupe) => {
                let combat = partie
                    .preparer_combat(groupe)
                    .expect(EQUIPE_APTE)
                    .avec_limite_d_obeissance(limite);
                let combat = self.sous_le_cycle(combat);
                evenements.push(Evenement::Rencontre(combat.clone()));
                self.engager(partie, combat, evenements)
            }
            EvenementExpedition::Coffre {
                potions,
                argent,
                capsule,
                capsule_d_habilete,
                pierre,
                booster,
                filets,
                tresor,
                carte,
            } => {
                let argent_du_pillard = partie.ouvrir_coffre(potions, argent, capsule.clone());
                if let Some(tresor) = tresor {
                    partie.trouver_tresor(tresor);
                }
                if let Some((filet, nombre)) = filets {
                    partie.trouver_filets(filet, nombre);
                }
                if capsule_d_habilete {
                    partie.trouver_capsule_d_habilete();
                }
                if let Some(pierre) = pierre {
                    partie.trouver_pierre(pierre);
                }
                if let Some(booster) = booster {
                    partie.trouver_booster(booster);
                }
                evenements.push(Evenement::Coffre {
                    potions,
                    argent: argent + argent_du_pillard,
                    argent_du_pillard,
                    capsule,
                    capsule_d_habilete,
                    pierre,
                    booster,
                    filets,
                    tresor,
                });
                if let Some(annexe) = carte.and_then(|id| {
                    contenu
                        .annexes()
                        .iter()
                        .find(|annexe| annexe.zone().id() == &id)
                }) {
                    partie.ouvrir_l_annexe(annexe.zone().id().clone());
                    evenements.push(Evenement::AnnexeDecouverte {
                        nom: annexe.zone().nom().to_string(),
                        texte: annexe.decouverte().to_string(),
                    });
                }
                self.apres_l_etape(partie, evenements)
            }
            EvenementExpedition::Ame {
                marchand,
                adversaire,
            } => {
                evenements.push(Evenement::AmeRencontree(
                    marchand.avant_combat().to_string(),
                ));
                let combat = match marchand.gardien() {
                    Gardien::MonstreDeLaZone | Gardien::Bete { .. } => {
                        partie.preparer_combat(vec![adversaire])
                    }
                    Gardien::MortVivant { .. } => {
                        partie.preparer_combat_sans_capture(vec![adversaire])
                    }
                }
                .expect(EQUIPE_APTE)
                .avec_limite_d_obeissance(limite);
                let combat = self.sous_le_cycle(combat);
                evenements.push(Evenement::Rencontre(combat.clone()));
                self.ame = Some(*marchand);
                self.engager(partie, combat, evenements)
            }
            EvenementExpedition::Pecheur {
                pecheur,
                adversaire,
            } => {
                evenements.push(Evenement::PecheurRencontre(
                    pecheur.avant_combat().to_string(),
                ));
                let combat = partie
                    .preparer_combat(vec![adversaire])
                    .expect(EQUIPE_APTE)
                    .avec_limite_d_obeissance(limite);
                let combat = self.sous_le_cycle(combat);
                evenements.push(Evenement::Rencontre(combat.clone()));
                self.pecheur = Some(*pecheur);
                self.engager(partie, combat, evenements)
            }
            EvenementExpedition::MonstreRare(rare) => {
                partie.noter_rencontre(&rare.monstre);
                if rare.pacifique {
                    evenements.push(Evenement::MonstreRarePacifique(rare.monstre.clone()));
                    self.moment = Moment::MonstreRare(rare.monstre);
                    return Reponse::Faite;
                }
                evenements.push(Evenement::MonstreRareHostile(rare.monstre.clone()));
                let combat = partie
                    .preparer_combat(vec![rare.monstre])
                    .expect(EQUIPE_APTE)
                    .avec_limite_d_obeissance(limite);
                let combat = self.sous_le_cycle(combat);
                evenements.push(Evenement::Rencontre(combat.clone()));
                self.engager(partie, combat, evenements)
            }
            EvenementExpedition::Apotre(formes) => {
                // Le récit de l'Apôtre tient lieu d'annonce de la rencontre.
                let recit = self
                    .expedition
                    .apotre()
                    .map(|apotre| apotre.avant_combat().to_string())
                    .unwrap_or_default();
                evenements.push(Evenement::RecitDeLApotre(recit));
                let combat = partie
                    .preparer_combat_contre_apotre(formes)
                    .expect(EQUIPE_APTE)
                    .avec_limite_d_obeissance(limite);
                let combat = self.sous_la_meteo(combat);
                self.moment = Moment::AnnonceDeCombat(combat);
                Reponse::Faite
            }
            EvenementExpedition::Legendaire(legendaire) => {
                // Comme pour l'Apôtre, son texte annonce la rencontre.
                let recit = self
                    .expedition
                    .legendaire()
                    .map(|attendu| attendu.avant_combat().to_string())
                    .unwrap_or_default();
                evenements.push(Evenement::LegendaireRencontre(recit));
                let combat = partie
                    .preparer_combat(vec![legendaire])
                    .expect(EQUIPE_APTE)
                    .contre_un_legendaire()
                    .avec_limite_d_obeissance(limite);
                let combat = self.sous_la_meteo(combat);
                self.moment = Moment::AnnonceDeCombat(combat);
                Reponse::Faite
            }
            EvenementExpedition::Gardien(gardien) => {
                // Comme pour l'humain, son texte annonce la rencontre.
                let recit = self
                    .expedition
                    .tour()
                    .map(|tour| tour.recit_du_gardien(gardien.nom()))
                    .unwrap_or_default();
                evenements.push(Evenement::GardienDeLaTour(recit));
                let combat = partie
                    .preparer_combat(vec![gardien])
                    .expect(EQUIPE_APTE)
                    .contre_un_gardien()
                    .avec_limite_d_obeissance(limite);
                self.moment = Moment::AnnonceDeCombat(combat);
                Reponse::Faite
            }
            EvenementExpedition::Humain { humain, equipe } => {
                // Comme pour l'Apôtre, son texte annonce la rencontre.
                evenements.push(Evenement::HumainRencontre(
                    humain.avant_combat().to_string(),
                ));
                let combat = partie
                    .preparer_combat_contre_humain(equipe, humain.generer_heros(), humain.ia())
                    .expect(EQUIPE_APTE)
                    .avec_limite_d_obeissance(limite);
                let combat = self.sous_la_meteo(combat);
                self.moment = Moment::AnnonceDeCombat(combat);
                Reponse::Faite
            }
        }
    }

    /// En groupe, un combat attend l'expédition en groupe, qui le mène : le
    /// joueur note ses adversaires au codex et voit le récit de la rencontre
    /// (section 15.5). `None` pour un événement sans combat.
    fn preparer_le_combat_de_groupe(
        &mut self,
        partie: &mut Partie,
        evenement: &EvenementExpedition,
        evenements: &mut Vec<Evenement>,
    ) -> Option<Reponse> {
        let (adversaires, contre_un_gardien) = match evenement {
            EvenementExpedition::Combat(groupe) => (groupe.clone(), false),
            EvenementExpedition::MonstreRare(rare) if !rare.pacifique => {
                evenements.push(Evenement::MonstreRareHostile(rare.monstre.clone()));
                (vec![rare.monstre.clone()], false)
            }
            EvenementExpedition::Gardien(gardien) => {
                if let Some(tour) = self.expedition.tour() {
                    evenements.push(Evenement::GardienDeLaTour(
                        tour.recit_du_gardien(gardien.nom()),
                    ));
                }
                (vec![gardien.clone()], true)
            }
            _ => return None,
        };
        for adversaire in &adversaires {
            partie.noter_rencontre(adversaire);
        }
        self.moment = Moment::CombatDeGroupe { contre_un_gardien };
        Some(Reponse::Faite)
    }

    /// Les combats d'une expédition dans une zone de la carte suivent son
    /// moment ; pas ceux de fin de zone, contre un humain ou un Apôtre
    /// (section 9.6). Tous suivent sa météo (section 9.7).
    fn sous_le_cycle(&self, combat: Combat) -> Combat {
        let combat = self.sous_la_meteo(combat);
        match self.expedition.jour_ou_nuit() {
            Some(jour_ou_nuit) => combat.avec_jour_ou_nuit(jour_ou_nuit),
            None => combat,
        }
    }

    /// Les combats de fin de zone suivent la météo, mais pas le moment
    /// (sections 9.6 et 9.7).
    fn sous_la_meteo(&self, combat: Combat) -> Combat {
        match self.expedition.meteo() {
            Some(meteo) => combat.avec_meteo(meteo),
            None => combat,
        }
    }

    fn engager(
        &mut self,
        partie: &mut Partie,
        combat: Combat,
        evenements: &mut Vec<Evenement>,
    ) -> Reponse {
        montrer_conseil(partie, Conseil::PremierCombat, evenements);
        self.moment = Moment::Combat(combat);
        Reponse::Faite
    }
}

/// Un soin sans effet serait perdu : le joueur confirme d'abord (Q53,
/// section 14.4). Les autres cas sont laissés aux règles des soins, qui les
/// refusent ou les appliquent.
fn soin_a_confirmer(
    soin: Soin,
    soins: &Soins,
    cible: Option<&Monstre>,
    confirme: bool,
) -> Option<Confirmation> {
    let monstre = cible?;
    let perdu = !confirme
        && soins.nombre(soin) > 0
        && verifier_cible(soin, monstre).is_ok()
        && soin_sans_effet(soin, monstre);
    perdu.then(|| Confirmation::SoinPerdu {
        soin,
        monstre: monstre.clone(),
    })
}

/// Rend la capture qui ne trouve pas de place.
fn annoncer_placement(
    partie: &Partie,
    placement: Placement,
    evenements: &mut Vec<Evenement>,
) -> Option<Monstre> {
    let collection = partie.collection();
    match placement {
        Placement::Equipe(index) => {
            evenements.push(Evenement::RejointLEquipe(
                collection.equipe()[index].clone(),
            ));
            None
        }
        Placement::Camp(index) => {
            evenements.push(Evenement::PartPourLeCamp(collection.camp()[index].clone()));
            None
        }
        Placement::PlaceManquante(capture) => Some(*capture),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::{AleatoireScripte, GenerateurAleatoire};
    use crate::ame::MarchandId;
    use crate::amelioration::Forge;
    use crate::beherit::EtatDuBeherit;
    use crate::catalogue::Catalogue;
    use crate::codex::EtatCodex;
    use crate::espece::EspeceId;
    use crate::habilete::Habilete;
    use crate::humain::HumainId;
    use crate::monstre::rayon_chromatique;
    use crate::niveau::Niveau;
    use crate::partie::EtatPartie;
    use crate::peche::Peche;
    use crate::stats::ValeursInternes;
    use crate::types::{Efficacite, efficacite};
    use std::cell::RefCell;

    /// Garde chaque sauvegarde ; en panne, refuse d'écrire.
    #[derive(Default)]
    struct DepotMemoire {
        sauvegardes: RefCell<Vec<Partie>>,
        en_panne: bool,
    }

    impl SauvegardeRepository for DepotMemoire {
        fn charger(&self, _: &dyn Catalogue) -> Result<Option<Partie>, ErreurSauvegarde> {
            Ok(self.sauvegardes.borrow().last().cloned())
        }

        fn enregistrer(&self, partie: &Partie) -> Result<(), ErreurSauvegarde> {
            if self.en_panne {
                return Err(ErreurSauvegarde::Acces("panne".to_string()));
            }
            self.sauvegardes.borrow_mut().push(partie.clone());
            Ok(())
        }

        fn supprimer(&self) -> Result<(), ErreurSauvegarde> {
            if self.en_panne {
                return Err(ErreurSauvegarde::Acces("panne".to_string()));
            }
            self.sauvegardes.borrow_mut().clear();
            Ok(())
        }
    }

    /// Réponses données sans tirage aléatoire.
    fn repondre_sans_tirage(
        deroulement: &mut Deroulement,
        choix: Choix,
        contenu: &ContenuV1,
        depot: &DepotMemoire,
    ) -> Result<Vec<Evenement>, ErreurChoix> {
        let mut aleatoire = AleatoireScripte::new([]);
        let mut contexte = Contexte {
            contenu,
            aleatoire: &mut aleatoire,
            depot,
            maintenant: Horodatage::depuis_secondes(0),
        };
        deroulement.repondre(choix, &mut contexte)
    }

    /// Aldric, Mage, avec une Salamandre de niveau 5 et un Loup de niveau 3.
    fn partie(contenu: &ContenuV1) -> Partie {
        let creer = |id: &str, niveau: u8| {
            let espece = contenu.espece(&EspeceId::new(id)).unwrap();
            Monstre::creer(
                espece,
                Niveau::new(niveau).unwrap(),
                ValeursInternes::default(),
                false,
            )
            .unwrap()
        };
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
        let mut partie = Partie::nouvelle(heros, creer("salamandre", 5));
        partie.prendre_monstre_rare(creer("loup", 3));
        partie
    }

    fn etat_de(partie: &Partie) -> EtatPartie {
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
            rite: partie.rite().cloned(),
            paliers_du_codex: partie.paliers_du_codex().to_vec(),
            arene: partie.arene(),
            decouvertes: partie.decouvertes().clone(),
        }
    }

    #[test]
    fn creation_du_heros_avec_le_monstre_attitre_de_sa_classe() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = Deroulement::demarrer(Ok(None));
        assert_eq!(deroulement.attente(), Attente::NomDuHeros);
        assert!(matches!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::NomDuHeros(String::new()),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::Nom(_))
        ));
        let nom = Choix::NomDuHeros("Aldric".to_string());
        repondre_sans_tirage(&mut deroulement, nom, &contenu, &depot).unwrap();
        assert_eq!(deroulement.attente(), Attente::Classe);

        // Shiny : ses valeurs internes ne se tirent pas ; puis son habileté,
        // la première des deux de la Mandragore ; puis le shiny du héros
        // (section 4.1).
        let mut aleatoire = AleatoireScripte::new([1, 0, 512]);
        let mut contexte = Contexte {
            contenu: &contenu,
            aleatoire: &mut aleatoire,
            depot: &depot,
            maintenant: Horodatage::depuis_secondes(0),
        };
        let evenements = deroulement
            .repondre(Choix::Classe(Classe::Templier), &mut contexte)
            .unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::Introduction(Classe::Templier), Evenement::PartieCommencee(depart)]
                if depart.est_shiny() && depart.espece().id().as_str() == "mandragore"
        ));
        assert_eq!(deroulement.attente(), Attente::Camp);
        let partie = deroulement.partie().unwrap();
        assert_eq!(partie.heros().classe(), Classe::Templier);
        assert!(!partie.heros().est_shiny());
        // Né shiny, le monstre de départ connaît le Rayon chromatique
        // (section 5.1).
        let depart = &partie.collection().equipe()[0];
        assert!(depart.attaques().connait(rayon_chromatique().id()));
        // La partie est sauvegardée dès sa création (section 13.9).
        assert_eq!(depot.sauvegardes.borrow().len(), 1);
    }

    /// 1 fois sur 512, le héros naît shiny ; il ne change que de couleur
    /// (section 4.1).
    #[test]
    fn le_heros_peut_naitre_shiny() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = Deroulement::demarrer(Ok(None));
        let nom = Choix::NomDuHeros("Aldric".to_string());
        repondre_sans_tirage(&mut deroulement, nom, &contenu, &depot).unwrap();
        // Le monstre de départ : non shiny, ses six valeurs internes, son
        // habileté ; puis le héros, shiny.
        let mut aleatoire = AleatoireScripte::new([512, 0, 0, 0, 0, 0, 0, 0, 1]);
        let mut contexte = Contexte {
            contenu: &contenu,
            aleatoire: &mut aleatoire,
            depot: &depot,
            maintenant: Horodatage::depuis_secondes(0),
        };
        let evenements = deroulement
            .repondre(Choix::Classe(Classe::Guerrier), &mut contexte)
            .unwrap();
        assert!(matches!(
            &evenements[..],
            [
                Evenement::Introduction(Classe::Guerrier),
                Evenement::PartieCommencee(depart),
                Evenement::HerosShiny
            ] if !depart.est_shiny()
        ));
        let heros = deroulement.partie().unwrap().heros();
        assert!(heros.est_shiny());
        let combattant = heros.combattant();
        assert!(combattant.est_shiny());
        // Ni valeurs internes à 31, ni Rayon chromatique.
        let ordinaire = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier).combattant();
        assert_eq!(combattant.stats(), ordinaire.stats());
        assert_eq!(combattant.attaques(), ordinaire.attaques());
    }

    #[test]
    fn une_nouvelle_partie_remplace_la_sauvegarde_apres_confirmation() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let partie = partie(&contenu);
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie.clone())));
        assert_eq!(deroulement.attente(), Attente::Reprise(&partie));

        repondre_sans_tirage(&mut deroulement, Choix::NouvellePartie, &contenu, &depot).unwrap();
        assert_eq!(
            deroulement.attente(),
            Attente::Confirmation(&Confirmation::RemplacerLaPartie(partie.heros().clone()))
        );
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot),
            Err(ErreurChoix::HorsDePropos)
        );
        repondre_sans_tirage(&mut deroulement, Choix::Non, &contenu, &depot).unwrap();
        assert_eq!(deroulement.attente(), Attente::Reprise(&partie));

        repondre_sans_tirage(&mut deroulement, Choix::NouvellePartie, &contenu, &depot).unwrap();
        repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert_eq!(deroulement.attente(), Attente::NomDuHeros);
    }

    #[test]
    fn le_suicide_efface_la_sauvegarde_apres_confirmation() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let partie = partie(&contenu);
        depot.enregistrer(&partie).unwrap();
        let mut deroulement = au_camp(partie.clone(), &contenu, &depot);

        repondre_sans_tirage(&mut deroulement, Choix::SeSuicider, &contenu, &depot).unwrap();
        assert_eq!(
            deroulement.attente(),
            Attente::Confirmation(&Confirmation::Suicide(partie.heros().clone()))
        );
        repondre_sans_tirage(&mut deroulement, Choix::Non, &contenu, &depot).unwrap();
        assert_eq!(deroulement.attente(), Attente::Camp);
        assert_eq!(depot.charger(&contenu), Ok(Some(partie)));

        repondre_sans_tirage(&mut deroulement, Choix::SeSuicider, &contenu, &depot).unwrap();
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert_eq!(evenements, vec![Evenement::Suicide]);
        assert_eq!(deroulement.attente(), Attente::NomDuHeros);
        assert!(deroulement.partie().is_none());
        assert_eq!(depot.charger(&contenu), Ok(None));
    }

    #[test]
    fn un_effacement_impossible_n_empeche_pas_de_recommencer() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire {
            en_panne: true,
            ..DepotMemoire::default()
        };
        let mut deroulement = au_camp(partie(&contenu), &contenu, &depot);
        repondre_sans_tirage(&mut deroulement, Choix::SeSuicider, &contenu, &depot).unwrap();
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert_eq!(
            evenements,
            vec![
                Evenement::EffacementImpossible(ErreurSauvegarde::Acces("panne".to_string())),
                Evenement::Suicide,
            ]
        );
        assert_eq!(deroulement.attente(), Attente::NomDuHeros);
    }

    #[test]
    fn une_sauvegarde_illisible_propose_une_nouvelle_partie_sans_confirmation() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let erreur = ErreurSauvegarde::Illisible("vide".to_string());
        let mut deroulement = Deroulement::demarrer(Err(erreur.clone()));
        assert_eq!(deroulement.attente(), Attente::SauvegardeIllisible(&erreur));
        repondre_sans_tirage(&mut deroulement, Choix::NouvellePartie, &contenu, &depot).unwrap();
        assert_eq!(deroulement.attente(), Attente::NomDuHeros);
    }

    #[test]
    fn gestion_au_camp_sauvegardee_apres_chaque_action() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie(&contenu))));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();

        let renommer = Choix::Gerer(ActionDeGestion::Renommer {
            emplacement: Emplacement::Equipe(1),
            surnom: "Croc".to_string(),
        });
        repondre_sans_tirage(&mut deroulement, renommer, &contenu, &depot).unwrap();
        assert_eq!(depot.sauvegardes.borrow().len(), 1);
        let equipe = deroulement.partie().unwrap().collection().equipe();
        assert_eq!(equipe[1].nom(), "Croc");

        // Une action refusée ne sauvegarde pas.
        let echange = Choix::Gerer(ActionDeGestion::Echanger { equipe: 0, camp: 0 });
        assert!(matches!(
            repondre_sans_tirage(&mut deroulement, echange, &contenu, &depot),
            Err(ErreurChoix::Collection(_))
        ));
        assert_eq!(depot.sauvegardes.borrow().len(), 1);

        let relacher = Choix::Gerer(ActionDeGestion::Relacher(Emplacement::Equipe(1)));
        repondre_sans_tirage(&mut deroulement, relacher.clone(), &contenu, &depot).unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::Confirmation(Confirmation::Relacher(loup)) if loup.nom() == "Croc"
        ));
        repondre_sans_tirage(&mut deroulement, Choix::Non, &contenu, &depot).unwrap();
        assert_eq!(deroulement.partie().unwrap().collection().equipe().len(), 2);

        repondre_sans_tirage(&mut deroulement, relacher, &contenu, &depot).unwrap();
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert!(matches!(&evenements[..], [Evenement::Relache(loup)] if loup.nom() == "Croc"));
        assert_eq!(depot.sauvegardes.borrow().len(), 2);

        // Le dernier monstre ne se relâche pas : pas de confirmation
        // demandée.
        let dernier = Choix::Gerer(ActionDeGestion::Relacher(Emplacement::Equipe(0)));
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, dernier, &contenu, &depot),
            Err(ErreurChoix::Collection(ErreurCollection::DernierMonstre))
        );
        assert_eq!(deroulement.attente(), Attente::Camp);
    }

    #[test]
    fn expedition_avec_coffre_potion_confirmee_et_retour() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire {
            en_panne: true,
            ..DepotMemoire::default()
        };
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie(&contenu))));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::PartirEnExpedition(ZoneId::new("marais")),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::ZoneFermee)
        );

        // Un coffre d'une potion et de 12 pièces d'argent, sans capsule.
        let mut aleatoire = AleatoireScripte::new([81, 1, 12, 21, 100]);
        let mut contexte = Contexte {
            contenu: &contenu,
            aleatoire: &mut aleatoire,
            depot: &depot,
            maintenant: Horodatage::depuis_secondes(0),
        };
        let evenements = deroulement
            .repondre(
                Choix::PartirEnExpedition(ZoneId::new("lisiere")),
                &mut contexte,
            )
            .unwrap();
        assert_eq!(
            evenements,
            vec![
                Evenement::Depart("Lisière du Bois-Sombre".to_string()),
                // Minuit UTC : le jour (section 9.6).
                Evenement::JourOuNuitAuDepart(JourOuNuit::Jour),
                Evenement::Etape {
                    numero: 1,
                    total: Some(5)
                },
                Evenement::Coffre {
                    potions: 1,
                    argent: 12,
                    argent_du_pillard: 0,
                    capsule: None,
                    capsule_d_habilete: false,
                    pierre: None,
                    booster: None,
                    filets: None,
                    tresor: None,
                },
                Evenement::Conseil(Conseil::PremierArretEntreDeuxEtapes),
            ]
        );
        assert_eq!(deroulement.attente(), Attente::EntreDeuxEtapes);

        // La Salamandre a tous ses PV : la potion serait perdue (Q53).
        repondre_sans_tirage(
            &mut deroulement,
            Choix::UtiliserSoin(Soin::Potion, 0),
            &contenu,
            &depot,
        )
        .unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::Confirmation(Confirmation::SoinPerdu {
                soin: Soin::Potion,
                ..
            })
        ));
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::SoinUtilise {
                soin: Soin::Potion,
                effet: EffetDuSoin::PvRendus(0),
                ..
            }]
        ));
        assert_eq!(deroulement.partie().unwrap().sac().potions(), 0);

        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::RentrerAuCamp, &contenu, &depot).unwrap();
        assert_eq!(
            evenements,
            vec![
                Evenement::RetourAuCamp,
                Evenement::Conseil(Conseil::PremierRetourAuCamp),
                Evenement::SauvegardeImpossible(ErreurSauvegarde::Acces("panne".to_string())),
            ]
        );
        assert_eq!(deroulement.attente(), Attente::Camp);
        assert!(deroulement.expedition().is_none());
        // L'argent trouvé reste dans le sac (section 13.3).
        assert_eq!(deroulement.partie().unwrap().sac().argent(), 12);
    }

    #[test]
    fn malvoise_arrive_au_bout_de_la_lisiere_pour_acheter_et_vendre() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie(&contenu))));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::AcheterSoins(Soin::Potion, 1),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::Marchand(ErreurMarchand::AucunMarchand))
        );

        // Le brigand déjà vaincu et l'âme de Bertrand portée, la Lisière ne
        // finit que par des coffres : une potion et 10 pièces d'argent
        // chacun, sans capsule.
        let coffre = vec![81, 1, 10, 100, 100];
        let mut etat = etat_de(deroulement.partie().unwrap());
        etat.humains_vaincus = vec![HumainId::new("brigand")];
        etat.ames_portees = vec![bertrand(&contenu)];
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let lisiere = Choix::PartirEnExpedition(ZoneId::new("lisiere"));
        repondre_a(
            &mut deroulement,
            lisiere,
            &contenu,
            &depot,
            0,
            coffre.clone(),
        )
        .unwrap();
        for _ in 0..4 {
            let continuer = Choix::ContinuerLExpedition;
            repondre_a(
                &mut deroulement,
                continuer,
                &contenu,
                &depot,
                0,
                coffre.clone(),
            )
            .unwrap();
        }
        assert_eq!(deroulement.attente(), Attente::Annonce(Annonce::Indice));
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Poursuivre, &contenu, &depot).unwrap();
        // Malvoise, qui a vu le joueur recueillir l'âme, rentre au camp avec
        // lui ; l'indice se lit au camp (section 14.2).
        let position = |cherche: fn(&Evenement) -> bool| evenements.iter().position(cherche);
        let malvoise =
            position(|evenement| *evenement == Evenement::MalvoiseRencontre { avec_une_ame: true });
        let retour = position(|evenement| *evenement == Evenement::RetourAuCamp);
        let indice = position(|evenement| matches!(evenement, Evenement::Indice { .. }));
        assert!(malvoise.unwrap() < retour.unwrap());
        assert!(retour.unwrap() < indice.unwrap());
        let partie = deroulement.partie().unwrap();
        assert!(partie.marchand_au_camp());
        assert!(
            depot
                .sauvegardes
                .borrow()
                .last()
                .unwrap()
                .marchand_au_camp()
        );
        // 50 pièces d'argent trouvées : deux potions, et pas de quoi en payer
        // une troisième.
        let acheter = || Choix::AcheterSoins(Soin::Potion, 1);
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::AcheterSoins(Soin::Potion, 2),
                &contenu,
                &depot
            ),
            Ok(vec![Evenement::SoinsAchetes {
                soin: Soin::Potion,
                nombre: 2,
                prix: 40
            }])
        );
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, acheter(), &contenu, &depot),
            Err(ErreurChoix::Marchand(ErreurMarchand::ArgentInsuffisant {
                prix: 20,
                argent: 10
            }))
        );

        // Le Loup de niveau 3, d'espèce commune et de base 285, vaut
        // 3 x 285 / 50 = 17 pièces d'argent.
        let vente = Choix::Vendre(Emplacement::Equipe(1));
        repondre_sans_tirage(&mut deroulement, vente, &contenu, &depot).unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::Confirmation(Confirmation::Vendre { prix: 17, .. })
        ));
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::MonstreVendu { prix: 17, monstre }] if monstre.espece().id().as_str() == "loup"
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.coffre_du_camp().argent(), 17);
        assert_eq!(sauvegarde.collection().equipe().len(), 1);

        // Le dernier monstre ne se vend pas : pas de confirmation demandée.
        let dernier = Choix::Vendre(Emplacement::Equipe(0));
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, dernier, &contenu, &depot),
            Err(ErreurChoix::Marchand(ErreurMarchand::Collection(
                ErreurCollection::DernierMonstre
            )))
        );
        assert_eq!(deroulement.attente(), Attente::Camp);
    }

    /// Joue avec le joueur simple jusqu'à ce que le combat en cours soit
    /// fini, et rend ce qui s'est passé.
    fn jouer_le_combat(
        deroulement: &mut Deroulement,
        contenu: &ContenuV1,
        depot: &DepotMemoire,
    ) -> Vec<Evenement> {
        let mut aleatoire = GenerateurAleatoire::new(31);
        let mut vus = Vec::new();
        while matches!(
            deroulement.attente(),
            Attente::ActionDeCombat(_)
                | Attente::Remplacement(_)
                | Attente::CaptureDesKo(_)
                | Attente::OubliAttaque { .. }
                | Attente::Evolution { .. }
                | Attente::PlaceManquante(_)
        ) {
            let choix = jouer_un_coup(deroulement, contenu);
            let mut contexte = Contexte {
                contenu,
                aleatoire: &mut aleatoire,
                depot,
                maintenant: Horodatage::depuis_secondes(0),
            };
            vus.extend(deroulement.repondre(choix, &mut contexte).unwrap());
        }
        vus
    }

    #[test]
    fn bertrand_est_abattu_a_l_etape_4_et_son_ame_portee_sans_malvoise() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_pour_les_expeditions(&contenu), &contenu, &depot);
        // Trois coffres d'une potion et de 10 pièces d'argent, sans capsule.
        let coffre = vec![81, 1, 10, 100, 100];
        let lisiere = Choix::PartirEnExpedition(ZoneId::new("lisiere"));
        repondre_a(
            &mut deroulement,
            lisiere,
            &contenu,
            &depot,
            0,
            coffre.clone(),
        )
        .unwrap();
        for _ in 0..2 {
            let continuer = Choix::ContinuerLExpedition;
            repondre_a(
                &mut deroulement,
                continuer,
                &contenu,
                &depot,
                0,
                coffre.clone(),
            )
            .unwrap();
        }
        // À l'étape 4, sans tirage de sa nature : le Loup du brigand, au
        // niveau 7, non shiny, valeurs internes nulles, première habileté.
        let tirages = vec![2, 0, 0, 0, 0, 0, 0, 0];
        let evenements = repondre_a(
            &mut deroulement,
            Choix::ContinuerLExpedition,
            &contenu,
            &depot,
            0,
            tirages,
        )
        .unwrap();
        assert!(matches!(
            &evenements[1],
            Evenement::AmeRencontree(texte) if texte.contains("brigand")
        ));
        let Attente::ActionDeCombat(combat) = deroulement.attente() else {
            panic!("combat attendu");
        };
        // Un combat ordinaire contre un monstre sauvage (section 14.2).
        assert!(combat.filet_possible() && combat.fuite_possible());
        let loup = &combat.adversaires()[0];
        assert_eq!(loup.espece().id().as_str(), "loup");
        assert_eq!(loup.niveau().valeur(), 7);
        let vus = jouer_le_combat(&mut deroulement, &contenu, &depot);
        assert!(vus.iter().any(|evenement| matches!(
            evenement,
            Evenement::AmeRecueillie(texte) if texte.contains("sans trop savoir")
        )));
        let partie = deroulement.partie().unwrap();
        assert!(partie.a_recueilli_l_ame(&MarchandId::new("bertrand")));
        assert_eq!(partie.ames_portees().len(), 1);
        // Portée, elle n'ajoute rien à l'étal tant qu'elle n'est pas donnée.
        assert_eq!(partie.soins_en_vente(), vec![Soin::Potion]);
    }

    #[test]
    fn l_ame_donnee_a_malvoise_ajoute_le_remede_a_son_etal() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie_pour_les_expeditions(&contenu));
        etat.ames_portees = vec![bertrand(&contenu)];
        let mut deroulement = au_camp(Partie::restaurer(etat.clone()).unwrap(), &contenu, &depot);
        let donner = || Choix::DonnerAme(MarchandId::new("bertrand"));
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, donner(), &contenu, &depot),
            Err(ErreurChoix::Marchand(ErreurMarchand::AucunMarchand))
        );
        etat.marchand_au_camp = true;
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, donner(), &contenu, &depot),
            Ok(vec![Evenement::AmeDonnee {
                marchand: "Bertrand".to_string(),
                soin: Soin::Remede
            }])
        );
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert!(sauvegarde.ames_portees().is_empty());
        assert_eq!(
            sauvegarde.soins_en_vente(),
            vec![Soin::Potion, Soin::Remede]
        );
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, donner(), &contenu, &depot),
            Err(ErreurChoix::Marchand(ErreurMarchand::AmeNonPortee))
        );
    }

    #[test]
    fn un_booster_se_donne_sans_confirmation() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie_pour_les_expeditions(&contenu));
        etat.coffre_du_camp = Inventaire::default().avec_boosters(Booster::MoelleDeBete, 1);
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let avant = deroulement.partie().unwrap().collection().equipe()[0].clone();
        let donner = Choix::DonnerBooster {
            emplacement: Emplacement::Equipe(0),
            booster: Booster::MoelleDeBete,
        };
        let evenements =
            repondre_sans_tirage(&mut deroulement, donner.clone(), &contenu, &depot).unwrap();
        let attendu = (avant.valeurs_internes().valeurs().attaque + 5).min(31);
        assert!(matches!(
            &evenements[..],
            [Evenement::BoosterDonne { booster: Booster::MoelleDeBete, avant: monstre, apres }]
                if *monstre == avant && apres.valeurs_internes().valeurs().attaque == attendu
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.coffre_du_camp().boosters(), &[]);
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, donner, &contenu, &depot),
            Err(ErreurChoix::Booster(ErreurBooster::Aucun(
                Booster::MoelleDeBete
            )))
        );
    }

    #[test]
    fn malvoise_rend_un_monstre_shiny_apres_confirmation() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie_pour_les_expeditions(&contenu));
        etat.marchand_au_camp = true;
        etat.coffre_du_camp = Inventaire::default().avec_philtres_chromatiques(1);
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let avant = deroulement.partie().unwrap().collection().equipe()[0].clone();
        assert!(!avant.est_shiny());
        let rendre = Choix::RendreShiny(Emplacement::Equipe(0));
        repondre_sans_tirage(&mut deroulement, rendre.clone(), &contenu, &depot).unwrap();
        assert_eq!(
            deroulement.attente(),
            Attente::Confirmation(&Confirmation::RendreShiny(avant.clone()))
        );
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::MonstreRenduShiny(monstre)] if monstre.est_shiny()
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.coffre_du_camp().philtres_chromatiques(), 0);
        assert!(sauvegarde.collection().equipe()[0].est_shiny());
        // Il connaît déjà 4 attaques : le joueur choisit celle qu'oublie le
        // Rayon chromatique, ou y renonce (section 5.1).
        assert_eq!(avant.attaques().liste().len(), 4);
        let rayon = rayon_chromatique();
        assert!(matches!(
            deroulement.attente(),
            Attente::OubliAttaque { nouvelle, .. } if *nouvelle == rayon
        ));
        repondre_sans_tirage(&mut deroulement, Choix::OublierAttaque(1), &contenu, &depot).unwrap();
        let shiny = &deroulement.partie().unwrap().collection().equipe()[0];
        assert_eq!(shiny.attaques().liste()[1], rayon);
        // Déjà shiny : le refus vient sans confirmation.
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, rendre, &contenu, &depot),
            Err(ErreurChoix::Marchand(ErreurMarchand::DejaShiny))
        );
    }

    /// Malvoise rend aussi le héros shiny, contre un Philtre chromatique
    /// (sections 4.1 et 13.7).
    #[test]
    fn malvoise_rend_le_heros_shiny_apres_confirmation() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie_pour_les_expeditions(&contenu));
        etat.marchand_au_camp = true;
        etat.sac = Inventaire::default().avec_philtres_chromatiques(1);
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let stats_avant = deroulement.partie().unwrap().heros().combattant().stats();
        repondre_sans_tirage(
            &mut deroulement,
            Choix::RendreLeHerosShiny,
            &contenu,
            &depot,
        )
        .unwrap();
        assert_eq!(
            deroulement.attente(),
            Attente::Confirmation(&Confirmation::RendreLeHerosShiny)
        );
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert_eq!(evenements, [Evenement::HerosRenduShiny]);
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert!(sauvegarde.heros().est_shiny());
        assert_eq!(sauvegarde.sac().philtres_chromatiques(), 0);
        assert_eq!(sauvegarde.heros().combattant().stats(), stats_avant);
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::RendreLeHerosShiny,
                &contenu,
                &depot
            ),
            Err(ErreurChoix::Marchand(ErreurMarchand::DejaShiny))
        );
    }

    #[test]
    fn les_machines_de_gunther_au_camp() {
        use crate::amelioration::Ameliorations;
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie_pour_les_expeditions(&contenu));
        let ameliorations = Ameliorations::new(0, 0, 0)
            .unwrap()
            .avec_machines(1, 1)
            .unwrap();
        etat.forge = Forge::restaurer(true, ameliorations, None).unwrap();
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let mut repondre = |choix: Choix, secondes: u64, tirages: Vec<u32>| {
            repondre_a(&mut deroulement, choix, &contenu, &depot, secondes, tirages)
        };
        assert_eq!(
            repondre(Choix::PoserLesPieges(ZoneId::new("vallee")), 0, vec![]),
            Err(ErreurChoix::Machine(ErreurMachine::ZoneNonExploree))
        );
        assert_eq!(
            repondre(Choix::PoserLesPieges(ZoneId::new("lisiere")), 0, vec![]),
            Ok(vec![Evenement::PiegesPoses(
                "Lisière du Bois-Sombre".to_string()
            )])
        );
        assert_eq!(
            repondre(Choix::ReleverLesPieges, 7_199, vec![]),
            Err(ErreurChoix::Machine(ErreurMachine::PiegesVides))
        );
        // Une ressource toutes les 2 h : la deuxième de la Lisière.
        let evenements = repondre(Choix::ReleverLesPieges, 7_200, vec![1]).unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::PiegesReleves(prises)]
                if prises.len() == 1 && prises[0].0.id().as_str() == "plume-de-corbeau"
        ));
        // Déplacés dans le Marais, ils sont d'abord relevés.
        let evenements = repondre(
            Choix::PoserLesPieges(ZoneId::new("marais")),
            14_400,
            vec![2],
        )
        .unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::PiegesReleves(prises), Evenement::PiegesPoses(zone)]
                if prises[0].0.id().as_str() == "ecaille-de-vipere" && zone == "Marais des Pendus"
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.coffre_du_camp().ressources().len(), 2);
        assert_eq!(
            sauvegarde.machines().pieges().unwrap().zone(),
            &ZoneId::new("marais")
        );
        // Le premier Loup entre dans la cour, d'une place au niveau 1.
        let evenements = repondre(
            Choix::MettreALEntrainement(Emplacement::Equipe(1)),
            14_400,
            vec![],
        )
        .unwrap();
        assert!(
            matches!(&evenements[..], [Evenement::EntreeALEntrainement(loup)] if loup.niveau().valeur() == 3)
        );
        assert_eq!(
            repondre(
                Choix::MettreALEntrainement(Emplacement::Equipe(0)),
                14_400,
                vec![]
            ),
            Err(ErreurChoix::Machine(ErreurMachine::CourPleine))
        );
        // 10 h plus tard, 500 XP : il rejoint l'équipe, en troisième place.
        let evenements =
            repondre(Choix::ReprendreDeLEntrainement(0), 14_400 + 36_000, vec![]).unwrap();
        assert!(matches!(
            &evenements[..],
            [
                Evenement::SortieDeLEntrainement {
                    experience: 500,
                    emplacement: Emplacement::Equipe(2),
                    ..
                },
                Evenement::Experience { .. },
            ]
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert!(sauvegarde.machines().cour().is_empty());
        assert_eq!(sauvegarde.collection().places_reservees(), 0);
        assert_eq!(sauvegarde.collection().equipe()[2].experience(), 27 + 500);
    }

    #[test]
    fn amis_et_duels_se_sauvegardent_au_camp() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_pour_les_expeditions(&contenu), &contenu, &depot);
        assert_eq!(
            deroulement.changer_d_ami(42, true, &depot),
            Some(Vec::new())
        );
        assert_eq!(deroulement.changer_d_ami(42, true, &depot), None);
        deroulement.noter_duel(true, &depot).unwrap();
        deroulement.noter_duel(false, &depot).unwrap();
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.relations().amis(), &[42]);
        assert_eq!(sauvegarde.relations().palmares().victoires, 1);
        assert_eq!(sauvegarde.relations().palmares().defaites, 1);
        assert_eq!(
            deroulement.changer_d_ami(42, false, &depot),
            Some(Vec::new())
        );
        assert_eq!(deroulement.changer_d_ami(42, false, &depot), None);
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert!(sauvegarde.relations().amis().is_empty());
        // Hors du camp, rien ne change.
        let mut ailleurs = Deroulement::demarrer(Ok(None));
        assert_eq!(ailleurs.changer_d_ami(42, true, &depot), None);
        assert_eq!(ailleurs.noter_duel(true, &depot), None);
    }

    #[test]
    fn changer_d_habilete_apres_confirmation() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie_pour_les_expeditions(&contenu));
        etat.coffre_du_camp = Inventaire::default().avec_capsules_d_habilete(1);
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let avant = deroulement.partie().unwrap().collection().equipe()[0].habilete();
        let changer = Choix::ChangerDHabilete(Emplacement::Equipe(0));
        repondre_sans_tirage(&mut deroulement, changer.clone(), &contenu, &depot).unwrap();
        let Attente::Confirmation(Confirmation::ChangerDHabilete { nouvelle, .. }) =
            deroulement.attente()
        else {
            panic!("confirmation attendue");
        };
        let nouvelle = *nouvelle;
        assert_ne!(Some(nouvelle), avant);
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::HabileteChangee { monstre, ancienne }]
                if *ancienne == avant && monstre.habilete() == Some(nouvelle)
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.capsules_d_habilete(), 0);
        // Sans capsule, le refus vient sans confirmation.
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, changer, &contenu, &depot),
            Err(ErreurChoix::Habilete(
                ErreurChangementDHabilete::AucuneCapsule
            ))
        );
    }

    fn bertrand(contenu: &ContenuV1) -> MarchandMort {
        contenu
            .marchand_mort(&MarchandId::new("bertrand"))
            .expect("Bertrand est au contenu")
    }

    #[test]
    fn grete_relevee_attaque_sans_filet_ni_codex() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie_pour_les_expeditions(&contenu));
        etat.marchand_au_camp = true;
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        // Le marchand zombie, fixe, ne tire rien.
        let marais = Choix::PartirEnExpedition(ZoneId::new("marais"));
        repondre_a(&mut deroulement, marais, &contenu, &depot, 0, vec![71]).unwrap();
        let Attente::ActionDeCombat(combat) = deroulement.attente() else {
            panic!("combat attendu");
        };
        assert!(combat.fuite_possible() && !combat.filet_possible());
        let zombie = &combat.adversaires()[0];
        assert_eq!(zombie.espece().id().as_str(), "marchand-zombie");
        assert_eq!(zombie.niveau().valeur(), 13);
        let partie = deroulement.partie().unwrap();
        assert_eq!(
            partie.codex().etat(&EspeceId::new("marchand-zombie")),
            EtatCodex::Inconnue
        );
        // Une fuite laisse l'âme à trouver ; le zombie frappe une dernière
        // fois, au hasard.
        let mut aleatoire = GenerateurAleatoire::new(13);
        let mut contexte = Contexte {
            contenu: &contenu,
            aleatoire: &mut aleatoire,
            depot: &depot,
            maintenant: Horodatage::depuis_secondes(0),
        };
        let evenements = deroulement
            .repondre(Choix::Combat(Action::Fuir), &mut contexte)
            .unwrap();
        assert!(
            !evenements
                .iter()
                .any(|evenement| matches!(evenement, Evenement::AmeRecueillie { .. }))
        );
        let partie = deroulement.partie().unwrap();
        assert!(partie.ames_portees().is_empty());
        assert_eq!(partie.ames_a_retrouver(), &[MarchandId::new("grete")]);
        // Elle revient à coup sûr, à une étape tirée parmi les 4 premières :
        // ici la première, sans tirage de sa nature.
        let mut etat = etat_de(partie);
        etat.marchand_au_camp = true;
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let marais = Choix::PartirEnExpedition(ZoneId::new("marais"));
        repondre_a(&mut deroulement, marais, &contenu, &depot, 0, vec![1]).unwrap();
        let Attente::ActionDeCombat(combat) = deroulement.attente() else {
            panic!("combat attendu");
        };
        assert_eq!(
            combat.adversaires()[0].espece().id().as_str(),
            "marchand-zombie"
        );
        // L'ordre du héros, sans tirage, ne prend pas le tour.
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::DonnerOrdre, &contenu, &depot).unwrap();
        assert!(matches!(
            evenements.as_slice(),
            [Evenement::OrdreDonne { ordre: Ordre::Incantation, monstre, .. }]
                if monstre.espece().id().as_str() == "salamandre"
        ));
        let Attente::ActionDeCombat(combat) = deroulement.attente() else {
            panic!("combat attendu");
        };
        assert!(combat.a_recu_l_ordre(0));
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, Choix::DonnerOrdre, &contenu, &depot),
            Err(ErreurChoix::Combat(ErreurCombat::OrdreImpossible))
        );
        assert!(matches!(deroulement.attente(), Attente::ActionDeCombat(_)));
    }

    #[test]
    fn acheter_des_potions_au_marchand() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut partie = partie(&contenu);
        let mut etat = etat_de(&partie);
        etat.coffre_du_camp = Inventaire::new(50, 0, Vec::new());
        etat.marchand_au_camp = true;
        partie = Partie::restaurer(etat).unwrap();
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie)));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();
        let evenements = repondre_sans_tirage(
            &mut deroulement,
            Choix::AcheterSoins(Soin::Potion, 2),
            &contenu,
            &depot,
        )
        .unwrap();
        assert_eq!(
            evenements,
            vec![Evenement::SoinsAchetes {
                soin: Soin::Potion,
                nombre: 2,
                prix: 40
            }]
        );
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.argent(), 10);
        assert_eq!(sauvegarde.coffre_du_camp().potions(), 2);
    }

    #[test]
    fn ranger_et_sortir_du_coffre_du_camp() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie(&contenu));
        etat.sac = Inventaire::default();
        etat.coffre_du_camp = Inventaire::new(50, 3, Vec::new());
        let mut deroulement = Deroulement::demarrer(Ok(Some(Partie::restaurer(etat).unwrap())));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();
        let evenements = repondre_sans_tirage(
            &mut deroulement,
            Choix::SortirDuCoffre(Objet::Soins(Soin::Potion, 2)),
            &contenu,
            &depot,
        )
        .unwrap();
        assert_eq!(
            evenements,
            vec![Evenement::SortiDuCoffre(Inventaire::new(0, 2, Vec::new()))]
        );
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::RangerAuCoffre(Objet::Argent(1)),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::Objet(ErreurObjet::Insuffisant))
        );
        repondre_sans_tirage(
            &mut deroulement,
            Choix::ToutRangerAuCoffre,
            &contenu,
            &depot,
        )
        .unwrap();
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert!(sauvegarde.sac().est_vide());
        assert_eq!(
            sauvegarde.coffre_du_camp(),
            &Inventaire::new(50, 3, Vec::new())
        );
    }

    #[test]
    fn capsule_trouvee_dans_un_coffre_puis_apprise_au_camp() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie(&contenu))));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();
        // Un coffre : une potion, 12 pièces d'argent et la première des trois
        // capsules de la Lisière, Brasier.
        let mut aleatoire = AleatoireScripte::new([81, 1, 12, 20, 0, 100]);
        let mut contexte = Contexte {
            contenu: &contenu,
            aleatoire: &mut aleatoire,
            depot: &depot,
            maintenant: Horodatage::depuis_secondes(0),
        };
        let evenements = deroulement
            .repondre(
                Choix::PartirEnExpedition(ZoneId::new("lisiere")),
                &mut contexte,
            )
            .unwrap();
        assert!(matches!(
            &evenements[3],
            Evenement::Coffre { capsule: Some(capsule), .. } if capsule.id().as_str() == "brasier"
        ));
        // La capsule est aussitôt dans le sac, et y reste au retour.
        let brasier = AttaqueId::new("brasier");
        assert!(
            deroulement
                .partie()
                .unwrap()
                .sac()
                .contient_capsule(&brasier)
        );
        repondre_sans_tirage(&mut deroulement, Choix::RentrerAuCamp, &contenu, &depot).unwrap();
        assert!(
            deroulement
                .partie()
                .unwrap()
                .sac()
                .contient_capsule(&brasier)
        );

        // La Salamandre, de type Feu, connaît 2 attaques : elle apprend
        // Brasier sans en oublier.
        let apprendre = |attaque: &str, oubliee| Choix::ApprendreAttaque {
            emplacement: Emplacement::Equipe(0),
            attaque: AttaqueId::new(attaque),
            oubliee,
        };
        let evenements = repondre_sans_tirage(
            &mut deroulement,
            apprendre("brasier", None),
            &contenu,
            &depot,
        )
        .unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::AttaqueApprise { attaque, .. }] if attaque.id().as_str() == "brasier"
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.capsules().count(), 1);
        assert!(
            sauvegarde.collection().equipe()[0]
                .attaques()
                .connait(&AttaqueId::new("brasier"))
        );
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                apprendre("brasier", None),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::Apprentissage(
                ErreurApprentissage::AttaqueInaccessible
            ))
        );
    }

    #[test]
    fn acheter_une_capsule_au_marchand() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie(&contenu));
        etat.coffre_du_camp = Inventaire::new(160, 0, Vec::new());
        etat.marchand_au_camp = true;
        let mut deroulement = Deroulement::demarrer(Ok(Some(Partie::restaurer(etat).unwrap())));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::AcheterCapsule(AttaqueId::new("griffure")),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::CapsuleInconnue(AttaqueId::new("griffure")))
        );
        let evenements = repondre_sans_tirage(
            &mut deroulement,
            Choix::AcheterCapsule(AttaqueId::new("deferlante")),
            &contenu,
            &depot,
        )
        .unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::CapsuleAchetee { prix: 150, capsule }] if capsule.id().as_str() == "deferlante"
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.argent(), 10);
        assert!(sauvegarde.possede_capsule(&AttaqueId::new("deferlante")));
    }

    #[test]
    fn monstre_rare_pris_sans_place_apres_confirmation_d_une_relache() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut pleine = partie(&contenu);
        let loup = pleine.collection().equipe()[1].clone();
        for _ in 0..31 {
            pleine.prendre_monstre_rare(loup.clone());
        }
        assert_eq!(pleine.collection().camp().len(), 30);
        let mut deroulement = Deroulement::demarrer(Ok(Some(pleine)));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();

        // Un monstre rare, qui n'est pas le Schnoz : niveau 5, non shiny,
        // valeurs internes, habileté.
        let mut aleatoire = AleatoireScripte::new([96, 2, 5, 2, 0, 0, 0, 0, 0, 0, 0]);
        let mut contexte = Contexte {
            contenu: &contenu,
            aleatoire: &mut aleatoire,
            depot: &depot,
            maintenant: Horodatage::depuis_secondes(0),
        };
        let partir = Choix::PartirEnExpedition(ZoneId::new("lisiere"));
        let evenements = deroulement.repondre(partir, &mut contexte).unwrap();
        assert!(matches!(
            evenements.last(),
            Some(Evenement::MonstreRarePacifique(cerf)) if cerf.nom() == "Cerf blanc"
        ));
        repondre_sans_tirage(
            &mut deroulement,
            Choix::PrendreLeMonstreRare,
            &contenu,
            &depot,
        )
        .unwrap();
        assert!(matches!(deroulement.attente(), Attente::PlaceManquante(_)));

        let relacher = Choix::RelacherPourLaCapture(Emplacement::Camp(0));
        repondre_sans_tirage(&mut deroulement, relacher, &contenu, &depot).unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::Confirmation(Confirmation::Relacher(_))
        ));
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert!(matches!(
            &evenements[..],
            [
                Evenement::RelachePourLaCapture { capture, .. },
                Evenement::Conseil(Conseil::PremierArretEntreDeuxEtapes),
            ] if capture.nom() == "Cerf blanc"
        ));
        let camp = deroulement.partie().unwrap().collection().camp();
        assert_eq!(camp[0].nom(), "Cerf blanc");
        assert_eq!(deroulement.attente(), Attente::EntreDeuxEtapes);
    }

    #[test]
    fn le_schnoz_se_rencontre_et_se_laisse_prendre() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie(&contenu))));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();

        // Le monstre caché, une fois sur 20 : niveau 3, non shiny, valeurs
        // internes, habileté.
        let mut aleatoire = AleatoireScripte::new([96, 1, 3, 2, 0, 0, 0, 0, 0, 0, 1]);
        let mut contexte = Contexte {
            contenu: &contenu,
            aleatoire: &mut aleatoire,
            depot: &depot,
            maintenant: Horodatage::depuis_secondes(0),
        };
        let partir = Choix::PartirEnExpedition(ZoneId::new("lisiere"));
        let evenements = deroulement.repondre(partir, &mut contexte).unwrap();
        assert!(matches!(
            evenements.last(),
            Some(Evenement::MonstreRarePacifique(schnoz)) if schnoz.nom() == "Schnoz"
        ));
        repondre_sans_tirage(
            &mut deroulement,
            Choix::PrendreLeMonstreRare,
            &contenu,
            &depot,
        )
        .unwrap();
        let partie = deroulement.partie().unwrap();
        let collection = partie.collection();
        assert!(
            collection
                .equipe()
                .iter()
                .chain(collection.camp())
                .any(|monstre| monstre.nom() == "Schnoz")
        );
        assert_eq!(
            partie.codex().etat(&EspeceId::new("schnoz")),
            EtatCodex::Capturee
        );
        // La seconde de ses habiletés possibles.
        let schnoz = collection
            .equipe()
            .iter()
            .find(|monstre| monstre.nom() == "Schnoz")
            .unwrap();
        assert_eq!(schnoz.habilete(), Some(Habilete::Fouineur));
    }

    #[test]
    fn un_choix_hors_de_propos_ne_change_rien() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = Deroulement::demarrer(Ok(None));
        for choix in [
            Choix::ContinuerLExpedition,
            Choix::Oui,
            Choix::Combat(Action::Fuir),
        ] {
            assert_eq!(
                repondre_sans_tirage(&mut deroulement, choix, &contenu, &depot),
                Err(ErreurChoix::HorsDePropos)
            );
        }
        assert_eq!(deroulement.attente(), Attente::NomDuHeros);
    }

    /// Classement d'une attaque contre un défenseur : puissance, précision et
    /// efficacité, sans les dégâts exacts.
    fn valeur_d_attaque(attaque: &Attaque, defenseur: &Monstre) -> u32 {
        let facteur = match efficacite(attaque.type_attaque(), defenseur.espece().types()) {
            Efficacite::Nulle => 0,
            Efficacite::TresReduite => 1,
            Efficacite::Reduite => 2,
            Efficacite::Normale => 4,
            Efficacite::Forte => 8,
            Efficacite::TresForte => 16,
        };
        u32::from(attaque.puissance()) * u32::from(attaque.precision()) * facteur
    }

    /// Quand le sac déborde : les ressources d'abord, puis les filets, puis
    /// les potions, un par un.
    fn objet_a_jeter(sac: &Inventaire) -> Objet {
        if let Some((ressource, _)) = sac.ressources().first() {
            return Objet::Ressources(ressource.id().clone(), 1);
        }
        if let Some((filet, _)) = sac.filets().presents().next() {
            return Objet::Filets(filet, 1);
        }
        if let Some((soin, _)) = sac.soins().presents().next() {
            return Objet::Soins(soin, 1);
        }
        if let Some((pierre, _)) = sac.pierres().first() {
            return Objet::Pierres(*pierre, 1);
        }
        if let Some((booster, _)) = sac.boosters().first() {
            return Objet::Boosters(*booster, 1);
        }
        if sac.capsules_d_habilete() > 0 {
            return Objet::CapsulesDHabilete(1);
        }
        if sac.philtres_chromatiques() > 0 {
            return Objet::PhiltresChromatiques(1);
        }
        match sac.capsules().first() {
            Some(capsule) => Objet::Capsule(capsule.id().clone()),
            None => Objet::CharmesDeGarde(1),
        }
    }

    /// Joueur simple : la meilleure attaque, une potion sous la moitié des
    /// PV, retour au camp en mauvaise posture, le Marais à partir du niveau
    /// 12, la Vallée à partir du niveau 24, l'Abbaye à partir du niveau 34,
    /// toutes les captures possibles.
    fn jouer_un_coup(deroulement: &Deroulement, contenu: &ContenuV1) -> Choix {
        let partie = deroulement.partie();
        match deroulement.attente() {
            Attente::Reprise(_) => Choix::ReprendreLaPartie,
            Attente::SauvegardeIllisible(_) => Choix::NouvellePartie,
            Attente::NomDuHeros => Choix::NomDuHeros("Aldric".to_string()),
            Attente::Classe => Choix::Classe(Classe::Mage),
            Attente::Camp => {
                let partie = partie.unwrap();
                let zones = partie.zones_accessibles(contenu.zones(), contenu.piste());
                let niveau = partie.collection().equipe()[0].niveau().valeur();
                let zone = if zones.len() > 4 && niveau >= 44 {
                    zones[4]
                } else if zones.len() > 3 && niveau >= 34 {
                    zones[3]
                } else if zones.len() > 2 && niveau >= 24 {
                    zones[2]
                } else if zones.len() > 1 && niveau >= 12 {
                    zones[1]
                } else {
                    zones[0]
                };
                Choix::PartirEnExpedition(zone.id().clone())
            }
            Attente::EntreDeuxEtapes => {
                let partie = partie.unwrap();
                let equipe = partie.collection().equipe();
                let blesse = equipe.iter().position(|monstre| {
                    !monstre.est_ko() && monstre.pv_actuels() * 2 < monstre.stats().pv
                });
                match blesse {
                    Some(index) if partie.sac().potions() > 0 => {
                        Choix::UtiliserSoin(Soin::Potion, index)
                    }
                    Some(index) if equipe[index].pv_actuels() * 4 < equipe[index].stats().pv => {
                        Choix::RentrerAuCamp
                    }
                    _ => Choix::ContinuerLExpedition,
                }
            }
            Attente::MonstreRare(_) => Choix::PrendreLeMonstreRare,
            Attente::Prise(_) => Choix::GarderLaPrise,
            Attente::AdversaireDeLArene { .. } => Choix::Poursuivre,
            Attente::ActionDeCombat(combat) => {
                let attaquant = combat.monstre(combat.actif(combat::Cote::Joueur));
                let defenseur = combat.monstre(combat.actif(combat::Cote::Adversaire));
                let meilleure = attaquant
                    .attaques()
                    .liste()
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, attaque)| valeur_d_attaque(attaque, defenseur))
                    .map_or(0, |(position, _)| position);
                Choix::Combat(Action::Attaquer(meilleure))
            }
            Attente::Remplacement(combat) => {
                let apte = combat.equipe().iter().position(|monstre| !monstre.est_ko());
                Choix::Remplacer(apte.unwrap())
            }
            Attente::CaptureDesKo(combat) => Choix::Capturer(combat.adversaires_capturables()[0]),
            Attente::OubliAttaque { monstre, nouvelle } => {
                let plus_faible = monstre
                    .attaques()
                    .liste()
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, attaque)| attaque.puissance())
                    .unwrap();
                if plus_faible.1.puissance() < nouvelle.puissance() {
                    Choix::OublierAttaque(plus_faible.0)
                } else {
                    Choix::RenoncerALAttaque
                }
            }
            Attente::Evolution { .. } => Choix::Evoluer,
            Attente::PlaceManquante(_) => Choix::RelacherLaCapture,
            Attente::Annonce(_) => Choix::Poursuivre,
            Attente::Confirmation(_) => Choix::Oui,
            Attente::CombatDeGroupe | Attente::AttenteDuGroupe => {
                panic!("l'expédition en groupe mène son combat et ses étapes")
            }
            Attente::SacPlein => Choix::Jeter(objet_a_jeter(partie.unwrap().sac())),
        }
    }

    fn salamandre(contenu: &ContenuV1, niveau: u8) -> Monstre {
        Monstre::creer(
            contenu.espece(&EspeceId::new("salamandre")).unwrap(),
            Niveau::new(niveau).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    fn partie_de(monstre: Monstre) -> Partie {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
        Partie::nouvelle(heros, monstre)
    }

    #[test]
    fn faire_evoluer_au_camp_apres_confirmation() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_de(salamandre(&contenu, 16)), &contenu, &depot);
        let evoluer = Choix::FaireEvoluer(Emplacement::Equipe(0));
        repondre_sans_tirage(&mut deroulement, evoluer, &contenu, &depot).unwrap();
        let Attente::Confirmation(Confirmation::Evoluer {
            monstre,
            vers,
            pierre: None,
        }) = deroulement.attente()
        else {
            panic!("confirmation attendue");
        };
        assert_eq!(monstre.espece().id().as_str(), "salamandre");
        assert_eq!(vers.id().as_str(), "lezard-de-braise");
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        let [
            Evenement::Evolution {
                avant,
                apres,
                attaque: None,
            },
        ] = &evenements[..]
        else {
            panic!("évolution attendue : {evenements:?}");
        };
        assert_eq!(avant.espece().id().as_str(), "salamandre");
        assert_eq!(apres.espece().id().as_str(), "lezard-de-braise");
        // Elle connaît déjà 4 attaques : l'attaque d'évolution est à choisir,
        // et la partie est sauvegardée avec ce choix en attente.
        let Attente::OubliAttaque { nouvelle, .. } = deroulement.attente() else {
            panic!("choix de l'attaque à oublier attendu");
        };
        assert_eq!(nouvelle.id().as_str(), "jet-de-lave");
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.attaques_a_choisir().len(), 1);
        let lezard = EspeceId::new("lezard-de-braise");
        assert_eq!(sauvegarde.codex().etat(&lezard), EtatCodex::Capturee);
        repondre_sans_tirage(&mut deroulement, Choix::OublierAttaque(0), &contenu, &depot).unwrap();
        assert_eq!(deroulement.attente(), Attente::Camp);
        let monstre = &deroulement.partie().unwrap().collection().equipe()[0];
        assert!(monstre.attaques().connait(&AttaqueId::new("jet-de-lave")));
        // Une forme sans évolution est refusée sans confirmation.
        let encore = Choix::FaireEvoluer(Emplacement::Equipe(0));
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, encore, &contenu, &depot),
            Err(ErreurChoix::Evolution(
                ErreurEvolution::ImpossiblePourLInstant
            ))
        );
    }

    fn a(secondes: u64) -> Horodatage {
        Horodatage::depuis_secondes(secondes)
    }

    fn repondre_a_l_heure(
        deroulement: &mut Deroulement,
        choix: Choix,
        contenu: &ContenuV1,
        depot: &DepotMemoire,
        maintenant: Horodatage,
    ) -> Result<Vec<Evenement>, ErreurChoix> {
        let mut aleatoire = AleatoireScripte::new([]);
        let mut contexte = Contexte {
            contenu,
            aleatoire: &mut aleatoire,
            depot,
            maintenant,
        };
        deroulement.repondre(choix, &mut contexte)
    }

    /// Gunther au camp, et des ressources : 3 crocs dans le sac, 2 dans le
    /// coffre, et 5 de chaque autre ressource du premier enclos au coffre.
    fn partie_avec_gunther(contenu: &ContenuV1) -> Partie {
        let ressource = |id: &str| contenu.ressource(&RessourceId::new(id)).unwrap();
        let mut etat = etat_de(&partie(contenu));
        etat.forge = Forge::restaurer(true, Default::default(), None).unwrap();
        etat.sac = Inventaire::default().avec_ressources(ressource("croc-de-loup"), 3);
        etat.coffre_du_camp = Inventaire::default()
            .avec_ressources(ressource("croc-de-loup"), 2)
            .avec_ressources(ressource("plume-de-corbeau"), 5)
            .avec_ressources(ressource("ecaille-de-vipere"), 5);
        Partie::restaurer(etat).unwrap()
    }

    #[test]
    fn sans_gunther_pas_d_amelioration() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie(&contenu), &contenu, &depot);
        let lancer = Choix::LancerAmelioration(Amelioration::Enclos);
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, lancer, &contenu, &depot),
            Err(ErreurChoix::Amelioration(ErreurAmelioration::AucunArtisan))
        );
    }

    #[test]
    fn gunther_agrandit_l_enclos_une_fois_le_travail_constate() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_avec_gunther(&contenu), &contenu, &depot);
        let lancer = Choix::LancerAmelioration(Amelioration::Enclos);
        repondre_a_l_heure(&mut deroulement, lancer.clone(), &contenu, &depot, a(1_000)).unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::Confirmation(Confirmation::LancerAmelioration(plan))
                if plan.amelioration() == Amelioration::Enclos && plan.niveau() == 1
        ));
        let evenements =
            repondre_a_l_heure(&mut deroulement, Choix::Oui, &contenu, &depot, a(1_000)).unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::AmeliorationLancee { plan, .. }] if plan.niveau() == 1
        ));
        // Les crocs du sac d'abord, puis ceux du coffre.
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert!(sauvegarde.sac().ressources().is_empty());
        assert!(sauvegarde.coffre_du_camp().ressources().is_empty());
        let chantier = sauvegarde.forge().chantier().unwrap().clone();
        assert_eq!(chantier.fin, a(1_000 + 3_600));
        // Un seul chantier à la fois, et rien avant la fin.
        assert_eq!(
            repondre_a_l_heure(&mut deroulement, lancer, &contenu, &depot, a(2_000)),
            Err(ErreurChoix::Amelioration(
                ErreurAmelioration::ChantierEnCours
            ))
        );
        assert_eq!(
            repondre_a_l_heure(
                &mut deroulement,
                Choix::ConstaterTravail,
                &contenu,
                &depot,
                a(4_599)
            ),
            Err(ErreurChoix::Amelioration(
                ErreurAmelioration::PasEncoreFini(a(4_600))
            ))
        );
        let evenements = repondre_a_l_heure(
            &mut deroulement,
            Choix::ConstaterTravail,
            &contenu,
            &depot,
            a(4_600),
        )
        .unwrap();
        assert_eq!(evenements, vec![Evenement::TravailConstate(chantier)]);
        let partie = deroulement.partie().unwrap();
        assert_eq!(
            partie.forge().ameliorations().niveau(Amelioration::Enclos),
            1
        );
        assert_eq!(partie.collection().tailles().camp, 40);
        assert_eq!(partie.forge().chantier(), None);
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.collection().tailles().camp, 40);
    }

    #[test]
    fn acheter_des_filets_puis_en_faire_forger_un_lot() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let ressource = |id: &str| contenu.ressource(&RessourceId::new(id)).unwrap();
        let mut etat = etat_de(&partie(&contenu));
        etat.sac = Inventaire::default();
        etat.marchand_au_camp = true;
        etat.forge = Forge::restaurer(true, Default::default(), None).unwrap();
        etat.coffre_du_camp = Inventaire::new(40, 0, Vec::new())
            .avec_ressources(ressource("croc-de-loup"), 3)
            .avec_ressources(ressource("plume-de-corbeau"), 4);
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let evenements = repondre_a_l_heure(
            &mut deroulement,
            Choix::AcheterFilets(Filet::Leste, 1),
            &contenu,
            &depot,
            a(1_000),
        )
        .unwrap();
        assert_eq!(
            evenements,
            vec![Evenement::FiletsAchetes {
                filet: Filet::Leste,
                nombre: 1,
                prix: 30
            }]
        );
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.argent(), 10);
        assert_eq!(sauvegarde.coffre_du_camp().filets().nombre(Filet::Leste), 1);
        // Gunther forge le chanvre : 3 crocs de loup et 3 plumes de corbeau,
        // 30 minutes, après confirmation.
        let forger = Choix::FabriquerFilets(Filet::Chanvre);
        repondre_a_l_heure(&mut deroulement, forger, &contenu, &depot, a(1_000)).unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::Confirmation(Confirmation::FabriquerFilets(plan))
                if plan.filet() == Filet::Chanvre && plan.nombre() == 5
        ));
        let evenements =
            repondre_a_l_heure(&mut deroulement, Choix::Oui, &contenu, &depot, a(1_000)).unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::FiletsLances { chantier, .. }] if chantier.fin == a(2_800)
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        let restants: Vec<(&str, u32)> = sauvegarde
            .coffre_du_camp()
            .ressources()
            .iter()
            .map(|(ressource, nombre)| (ressource.id().as_str(), *nombre))
            .collect();
        assert_eq!(restants, vec![("plume-de-corbeau", 1)]);
        let evenements = repondre_a_l_heure(
            &mut deroulement,
            Choix::ConstaterTravail,
            &contenu,
            &depot,
            a(2_800),
        )
        .unwrap();
        assert!(matches!(&evenements[..], [Evenement::TravailConstate(_)]));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(
            sauvegarde.coffre_du_camp().filets().nombre(Filet::Chanvre),
            5
        );
        // Plus assez de ressources pour un autre lot : refus sans confirmation.
        assert!(matches!(
            repondre_a_l_heure(
                &mut deroulement,
                Choix::FabriquerFilets(Filet::Chanvre),
                &contenu,
                &depot,
                a(3_000)
            ),
            Err(ErreurChoix::Amelioration(
                ErreurAmelioration::RessourcesManquantes(_)
            ))
        ));
    }

    #[test]
    fn gunther_forge_un_objet_que_le_heros_porte_ensuite() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let ressource = |id: &str| contenu.ressource(&RessourceId::new(id)).unwrap();
        let mut etat = etat_de(&partie_de(salamandre(&contenu, 40)));
        etat.forge = Forge::restaurer(true, Default::default(), None).unwrap();
        etat.coffre_du_camp = Inventaire::default()
            .avec_ressources(ressource("plume-de-corbeau"), 7)
            .avec_ressources(ressource("ecaille-de-vipere"), 4);
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let couteau = ObjetDEquipementId::new("baton-d-if");
        let equiper = Choix::Equiper {
            place: PlaceDEquipement::Arme,
            objet: couteau.clone(),
        };
        let lancer = Choix::LancerFabrication(couteau.clone());
        // Le couteau de chasse est une arme d'Assassin : Gunther ne la forge
        // pas pour un Mage (section 4.3).
        assert_eq!(
            repondre_a_l_heure(
                &mut deroulement,
                Choix::LancerFabrication(ObjetDEquipementId::new("couteau-de-chasse")),
                &contenu,
                &depot,
                a(1_000)
            ),
            Err(ErreurChoix::Amelioration(ErreurAmelioration::AutreClasse))
        );
        repondre_a_l_heure(&mut deroulement, lancer.clone(), &contenu, &depot, a(1_000)).unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::Confirmation(Confirmation::LancerFabrication(plan))
                if plan.objet().id() == &couteau
        ));
        let evenements =
            repondre_a_l_heure(&mut deroulement, Choix::Oui, &contenu, &depot, a(1_000)).unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::FabricationLancee { chantier, .. }] if chantier.fin == a(4_600)
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        let restants: Vec<(&str, u32)> = sauvegarde
            .coffre_du_camp()
            .ressources()
            .iter()
            .map(|(ressource, nombre)| (ressource.id().as_str(), *nombre))
            .collect();
        assert_eq!(restants, vec![("plume-de-corbeau", 1)]);
        // Rien à porter avant le constat, et une amélioration attend la fin
        // de la fabrication.
        assert_eq!(
            repondre_a_l_heure(
                &mut deroulement,
                equiper.clone(),
                &contenu,
                &depot,
                a(2_000)
            ),
            Err(ErreurChoix::Equipement(ErreurEquipement::NonPossede(
                couteau.clone()
            )))
        );
        assert_eq!(
            repondre_a_l_heure(
                &mut deroulement,
                Choix::LancerAmelioration(Amelioration::Enclos),
                &contenu,
                &depot,
                a(2_000)
            ),
            Err(ErreurChoix::Amelioration(
                ErreurAmelioration::ChantierEnCours
            ))
        );
        let evenements = repondre_a_l_heure(
            &mut deroulement,
            Choix::ConstaterTravail,
            &contenu,
            &depot,
            a(4_600),
        )
        .unwrap();
        assert!(matches!(&evenements[..], [Evenement::TravailConstate(_)]));
        assert!(
            deroulement
                .partie()
                .unwrap()
                .heros()
                .equipement()
                .possede(&couteau)
        );
        // Un objet ne se forge qu'une fois.
        assert_eq!(
            repondre_a_l_heure(&mut deroulement, lancer, &contenu, &depot, a(5_000)),
            Err(ErreurChoix::Amelioration(ErreurAmelioration::DejaPossede))
        );
        let evenements = repondre_a_l_heure(
            &mut deroulement,
            equiper.clone(),
            &contenu,
            &depot,
            a(5_000),
        )
        .unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::ObjetEquipe { place: PlaceDEquipement::Arme, objet, remplace: None }]
                if objet.id() == &couteau
        ));
        // Remis à sa place, il ne change rien.
        let evenements =
            repondre_a_l_heure(&mut deroulement, equiper, &contenu, &depot, a(5_000)).unwrap();
        assert!(evenements.is_empty());
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert!(
            sauvegarde
                .heros()
                .equipement()
                .porte(PlaceDEquipement::Arme)
                .is_some()
        );
        // En combat, +3 % d'Attaque et d'Attaque spéciale pour ses
        // monstres, en plus de la classe et du rôle de la Salamandre,
        // offensive (+10 % d'Attaque et d'Attaque spéciale, section 4.2).
        let mut partie = deroulement.partie().unwrap().clone();
        let monstre = partie.collection().equipe()[0].clone();
        let sauvage = salamandre(&contenu, 40);
        let combat = partie.preparer_combat(vec![sauvage]).unwrap();
        let stats = combat.stats_en_combat(combat.actif(crate::combat::Cote::Joueur));
        assert_eq!(stats.attaque, monstre.stats().attaque * 113 / 100);
        assert_eq!(
            stats.attaque_speciale,
            monstre.stats().attaque_speciale * 123 / 100
        );
        // Le monstre sauvage n'a pas le bonus de son rôle.
        let adversaire = combat.actif(crate::combat::Cote::Adversaire);
        assert_eq!(
            combat.stats_en_combat(adversaire).attaque,
            combat.monstre(adversaire).stats().attaque
        );
        let evenements = repondre_a_l_heure(
            &mut deroulement,
            Choix::RetirerEquipement(PlaceDEquipement::Arme),
            &contenu,
            &depot,
            a(5_000),
        )
        .unwrap();
        assert!(matches!(&evenements[..], [Evenement::ObjetRetire { .. }]));
        assert_eq!(
            deroulement.partie().unwrap().heros().bonus_en_combat(),
            Classe::Mage.bonus()
        );
    }

    #[test]
    fn sans_les_ressources_gunther_dit_ce_qui_manque() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie_avec_gunther(&contenu));
        etat.coffre_du_camp = Inventaire::default();
        etat.sac = Inventaire::default();
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        // Refusé sans demander de confirmation.
        let lancer = Choix::LancerAmelioration(Amelioration::Filets);
        let Err(ErreurChoix::Amelioration(ErreurAmelioration::RessourcesManquantes(manquantes))) =
            repondre_sans_tirage(&mut deroulement, lancer, &contenu, &depot)
        else {
            panic!("ressources manquantes attendues");
        };
        let manquantes: Vec<(&str, u32)> = manquantes
            .iter()
            .map(|(ressource, nombre)| (ressource.id().as_str(), *nombre))
            .collect();
        assert_eq!(
            manquantes,
            vec![("croc-de-loup", 4), ("ecaille-de-vipere", 4)]
        );
        assert_eq!(deroulement.attente(), Attente::Camp);
    }

    #[test]
    fn les_filets_ameliores_augmentent_la_chance_de_capture() {
        let contenu = ContenuV1::new();
        let mut etat = etat_de(&partie(&contenu));
        let mut sauvage = Monstre::creer(
            contenu.espece(&EspeceId::new("loup")).unwrap(),
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        let pv_max = sauvage.stats().pv;
        let perdus = pv_max / 2;
        sauvage.subir_degats(perdus);
        let sans_filets = Partie::restaurer(etat.clone())
            .unwrap()
            .preparer_combat(vec![sauvage.clone()])
            .unwrap()
            .chance_de_capture(Filet::Chanvre);
        etat.forge = Forge::restaurer(
            true,
            crate::amelioration::Ameliorations::new(0, 3, 0).unwrap(),
            None,
        )
        .unwrap();
        let avec_filets = Partie::restaurer(etat)
            .unwrap()
            .preparer_combat(vec![sauvage])
            .unwrap()
            .chance_de_capture(Filet::Chanvre);
        // À niveau égal : PV perdus / PV max, puis x1,3 avec les filets du
        // niveau 3.
        assert_eq!(sans_filets, perdus * 100 / pv_max);
        assert_eq!(avec_filets, perdus * 100 * 13 / (pv_max * 10));
        assert!(avec_filets > sans_filets);
    }

    #[test]
    fn une_pierre_fait_evoluer_au_camp_et_se_consomme() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let feu_follet = Monstre::creer(
            contenu.espece(&EspeceId::new("feu-follet")).unwrap(),
            Niveau::new(8).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        let mut etat = etat_de(&partie_de(feu_follet));
        etat.marchand_au_camp = true;
        etat.coffre_du_camp = Inventaire::new(300, 0, Vec::new());
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let evoluer = Choix::FaireEvoluer(Emplacement::Equipe(0));
        // Ni le niveau ni un combat ne suffisent : il faut la pierre.
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, evoluer.clone(), &contenu, &depot),
            Err(ErreurChoix::Evolution(ErreurEvolution::PierreManquante(
                PierreDEvolution::Braise
            )))
        );
        let achat = Choix::AcheterPierres(PierreDEvolution::Braise, 1);
        let evenements = repondre_sans_tirage(&mut deroulement, achat, &contenu, &depot).unwrap();
        assert_eq!(
            evenements,
            vec![Evenement::PierresAchetees {
                pierre: PierreDEvolution::Braise,
                nombre: 1,
                prix: 300
            }]
        );
        repondre_sans_tirage(&mut deroulement, evoluer, &contenu, &depot).unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::Confirmation(Confirmation::Evoluer {
                vers,
                pierre: Some(PierreDEvolution::Braise),
                ..
            }) if vers.id().as_str() == "lanterne-des-morts"
        ));
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        // Au niveau 8, il connaît 2 attaques : il apprend Flamme funèbre.
        assert!(matches!(
            &evenements[..],
            [Evenement::Evolution { apres, attaque: Some(attaque), .. }]
                if apres.espece().id().as_str() == "lanterne-des-morts"
                    && attaque.id().as_str() == "flamme-funebre"
        ));
        let partie = deroulement.partie().unwrap();
        assert_eq!(partie.nombre_de_pierres(PierreDEvolution::Braise), 0);
        assert_eq!(deroulement.attente(), Attente::Camp);
    }

    #[test]
    fn pas_d_evolution_au_camp_avant_le_niveau() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_de(salamandre(&contenu, 15)), &contenu, &depot);
        let evoluer = Choix::FaireEvoluer(Emplacement::Equipe(0));
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, evoluer, &contenu, &depot),
            Err(ErreurChoix::Evolution(
                ErreurEvolution::ImpossiblePourLInstant
            ))
        );
        assert_eq!(deroulement.attente(), Attente::Camp);
    }

    /// Une évolution repoussée après un combat revient au niveau suivant, et
    /// seulement alors (section 5.4).
    #[test]
    fn evolution_proposee_apres_un_combat_puis_repoussee() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut depart = salamandre(&contenu, 15);
        depart.gagner_experience(16 * 16 * 16 - 1 - depart.experience());
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie_de(depart))));
        let mut aleatoire = GenerateurAleatoire::new(7);
        let mut propositions = Vec::new();
        for _ in 0..5_000 {
            let choix = match deroulement.attente() {
                Attente::Evolution { monstre, vers } => {
                    propositions.push(monstre.niveau().valeur());
                    assert_eq!(vers.id().as_str(), "lezard-de-braise");
                    if propositions.len() == 1 {
                        Choix::NePasEvoluer
                    } else {
                        Choix::Evoluer
                    }
                }
                _ => jouer_un_coup(&deroulement, &contenu),
            };
            let repousse = choix == Choix::NePasEvoluer;
            let mut contexte = Contexte {
                contenu: &contenu,
                aleatoire: &mut aleatoire,
                depot: &depot,
                maintenant: Horodatage::depuis_secondes(0),
            };
            let evenements = deroulement.repondre(choix, &mut contexte).unwrap();
            if repousse {
                assert!(matches!(
                    &evenements[..],
                    [Evenement::EvolutionRepoussee(_), ..]
                ));
            }
            if propositions.len() == 2 && deroulement.attente() == Attente::Camp {
                break;
            }
        }
        assert_eq!(propositions, vec![16, 17]);
        let partie = deroulement.partie().unwrap();
        let monstre = &partie.collection().equipe()[0];
        assert_eq!(monstre.espece().id().as_str(), "lezard-de-braise");
        let lezard = EspeceId::new("lezard-de-braise");
        assert_eq!(partie.codex().etat(&lezard), EtatCodex::Capturee);
    }

    #[test]
    fn un_fouineur_trouve_des_potions_apres_des_victoires() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let loup = Monstre::creer(
            contenu.espece(&EspeceId::new("loup")).unwrap(),
            Niveau::new(40).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
        .avec_habilete(Habilete::Fouineur);
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier);
        let mut deroulement = Deroulement::demarrer(Ok(Some(Partie::nouvelle(heros, loup))));
        let mut aleatoire = GenerateurAleatoire::new(28);
        let mut trouvees = 0;
        for _ in 0..2_000 {
            let choix = jouer_un_coup(&deroulement, &contenu);
            let mut contexte = Contexte {
                contenu: &contenu,
                aleatoire: &mut aleatoire,
                depot: &depot,
                maintenant: Horodatage::depuis_secondes(0),
            };
            let evenements = deroulement.repondre(choix, &mut contexte).unwrap();
            for evenement in &evenements {
                if let Evenement::PotionTrouvee(fouineur) = evenement {
                    assert!(fouineur.a_l_habilete(Habilete::Fouineur));
                    trouvees += 1;
                }
            }
        }
        assert!(trouvees > 0);
    }

    /// Les 13 Apôtres vaincus, Malvoise au camp, et une équipe de
    /// niveau 100 avec un Loup de niveau 30 au camp, à sacrifier (section
    /// 9.4).
    fn partie_pour_le_demiurge(contenu: &ContenuV1) -> Partie {
        // Une équipe de fin de jeu, au maximum de ses valeurs internes, avec
        // des types qui frappent l'Obscur (section 5.2).
        let fort = |id: &str| {
            Monstre::creer(
                contenu.espece(&EspeceId::new(id)).unwrap(),
                Niveau::new(100).unwrap(),
                ValeursInternes::maximales(),
                false,
            )
            .unwrap()
        };
        let mut partie = partie_de(fort("seraphin-de-vitrail"));
        partie.prendre_monstre_rare(fort("sphinx-des-sables"));
        partie.prendre_monstre_rare(fort("sylphe-des-neiges"));
        // L'équipe est pleine : le Loup va au camp.
        partie.prendre_monstre_rare(monstre_du_contenu(contenu, "loup", 30));
        let mut etat = etat_de(&partie);
        etat.sac.ajouter_potions(10);
        etat.marchand_au_camp = true;
        etat.apotres_vaincus = crate::partie::tests::ames_des_13_apotres(contenu.piste());
        Partie::restaurer(etat).unwrap()
    }

    #[test]
    fn le_beherit_ouvre_l_eclipse_ou_le_demiurge_se_releve_sous_sa_seconde_forme() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_pour_le_demiurge(&contenu), &contenu, &depot);
        let eclipse = contenu.invocation().eclipse().id().clone();
        // Ni départ ni sacrifice avant le Béhérit.
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::PartirEnExpedition(eclipse.clone()),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::Beherit(ErreurBeherit::EclipseFermee))
        );
        repondre_sans_tirage(&mut deroulement, Choix::ForgerLeBeherit, &contenu, &depot).unwrap();
        assert_eq!(
            deroulement.attente(),
            Attente::Confirmation(&Confirmation::ForgerLeBeherit)
        );
        let forge = repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert!(matches!(&forge[..], [Evenement::BeheritForge(_), ..]));
        let partie = deroulement.partie().unwrap();
        assert_eq!(partie.beherit(), EtatDuBeherit::Forge);
        assert!(partie.ames_d_apotres().is_empty());
        // Le Béhérit attend son sacrifice : un monstre de niveau 30 ou plus.
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::PartirEnExpedition(eclipse.clone()),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::Beherit(ErreurBeherit::EclipseFermee))
        );
        let loup = Emplacement::Camp(0);
        assert_eq!(
            deroulement.partie().unwrap().monstres_sacrifiables(),
            vec![
                Emplacement::Equipe(0),
                Emplacement::Equipe(1),
                Emplacement::Equipe(2),
                loup
            ]
        );
        repondre_sans_tirage(&mut deroulement, Choix::Sacrifier(loup), &contenu, &depot).unwrap();
        let mut aleatoire = GenerateurAleatoire::new(11);
        let mut vus = Vec::new();
        let mut choix = Choix::Oui;
        for _ in 0..500 {
            let mut contexte = Contexte {
                contenu: &contenu,
                aleatoire: &mut aleatoire,
                depot: &depot,
                maintenant: Horodatage::depuis_secondes(0),
            };
            vus.extend(deroulement.repondre(choix, &mut contexte).unwrap());
            if deroulement.attente() == Attente::Camp {
                break;
            }
            choix = jouer_un_coup(&deroulement, &contenu);
        }
        assert!(
            matches!(&vus[..], [Evenement::Sacrifice(recit), Evenement::Depart(_), ..] if recit.contains("Loup"))
        );
        // Combat final au niveau 100 : une défaite laisse l'Éclipse ouverte,
        // et le joueur y retourne sans nouveau sacrifice (section 9.4).
        for _ in 0..60 {
            if vus.contains(&Evenement::RecompenseDuDemiurge) {
                break;
            }
            assert_eq!(
                deroulement.partie().unwrap().beherit(),
                EtatDuBeherit::EclipseOuverte
            );
            vus.extend(jouer_l_expedition(
                &mut deroulement,
                &contenu,
                &depot,
                &eclipse,
                &mut aleatoire,
            ));
        }
        assert!(
            !vus.iter()
                .skip(1)
                .any(|evenement| matches!(evenement, Evenement::Sacrifice(_)))
        );
        let metamorphoses = vus
            .iter()
            .filter(|evenement| {
                matches!(evenement, Evenement::Combat { evenements, .. }
                    if evenements.iter().any(|e| matches!(e, combat::Evenement::Metamorphose(_))))
            })
            .count();
        assert!(metamorphoses >= 1);
        assert!(vus.iter().any(|evenement| matches!(evenement, Evenement::Epilogue(texte) if texte.contains("Démiurge"))));
        assert!(vus.contains(&Evenement::RecompenseDuDemiurge));
        assert!(!vus.iter().any(|evenement| matches!(
            evenement,
            Evenement::ZoneExploree(_) | Evenement::AmeDApotreRecueillie(_)
        )));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.beherit(), EtatDuBeherit::DemiurgeVaincu);
        assert_eq!(sauvegarde.sac().philtres_chromatiques(), 3);
        assert!(!sauvegarde.est_exploree(&eclipse));
        for forme in ["demiurge", "demiurge-eveille"] {
            assert_eq!(
                sauvegarde.codex().etat(&EspeceId::new(forme)),
                EtatCodex::Capturee
            );
        }
        let collection = sauvegarde.collection();
        assert!(
            collection
                .equipe()
                .iter()
                .chain(collection.camp())
                .all(|monstre| monstre.espece().id().as_str() != "loup")
        );
        // L'Éclipse s'est refermée.
        let mut deroulement = au_camp(sauvegarde, &contenu, &depot);
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::PartirEnExpedition(eclipse),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::Beherit(ErreurBeherit::EclipseFermee))
        );
    }

    /// Du camp au retour, une expédition dans cette zone, jouée par
    /// `jouer_un_coup`. Rend tout ce qui s'est passé.
    fn jouer_l_expedition(
        deroulement: &mut Deroulement,
        contenu: &ContenuV1,
        depot: &DepotMemoire,
        zone: &ZoneId,
        aleatoire: &mut GenerateurAleatoire,
    ) -> Vec<Evenement> {
        let mut vus = Vec::new();
        let mut choix = Choix::PartirEnExpedition(zone.clone());
        for _ in 0..500 {
            let mut contexte = Contexte {
                contenu,
                aleatoire: &mut *aleatoire,
                depot,
                maintenant: Horodatage::depuis_secondes(0),
            };
            vus.extend(deroulement.repondre(choix, &mut contexte).unwrap());
            if deroulement.attente() == Attente::Camp {
                return vus;
            }
            choix = jouer_un_coup(deroulement, contenu);
        }
        panic!("expédition sans fin : {:?}", deroulement.attente());
    }

    /// Une annexe de la Lisière, pour les tests : 3 étapes, des Loups, et
    /// un Philtre chromatique dans chaque coffre (section 9.9).
    fn contenu_avec_annexe() -> ContenuV1 {
        let contenu = ContenuV1::new();
        let niveau = Niveau::new(5).unwrap();
        let terrier = Zone::new(
            ZoneId::new("terrier"),
            "Terrier",
            (niveau, niveau),
            3,
            1,
            vec![contenu.espece(&EspeceId::new("loup")).unwrap()],
            Vec::new(),
            crate::donnees_de_test::parametres_expedition_automatique(),
            crate::expedition::ParametresCoffre::new(10..=10, Vec::new())
                .unwrap()
                .avec_tresors(vec![(Tresor::PhiltreChromatique, 1)]),
        )
        .unwrap()
        .avec_repartition(crate::zone::Repartition {
            combat: 60,
            coffre: 30,
        })
        .unwrap();
        contenu.avec_annexes(vec![crate::annexe::Annexe::new(
            terrier,
            ZoneId::new("lisiere"),
            "Une carte griffonnée mène à un terrier.",
        )])
    }

    /// Section 9.9 : une fois le brigand vaincu, un coffre de la Lisière
    /// contient parfois la carte de son annexe, qui l'ouvre aussitôt ; ses
    /// coffres y contiennent chacun un trésor, et elle ne compte pas comme
    /// explorée.
    #[test]
    fn la_carte_d_un_coffre_ouvre_l_annexe_aux_coffres_riches() {
        let contenu = contenu_avec_annexe();
        let depot = DepotMemoire::default();
        let terrier = ZoneId::new("terrier");
        let lisiere = ZoneId::new("lisiere");
        let zone_de_la_lisiere = zone_du_contenu(&contenu, &lisiere).unwrap();
        let partie = partie_de(salamandre(&contenu, 40));
        assert_eq!(
            partie.annexe_a_decouvrir(zone_de_la_lisiere, contenu.annexes()),
            None
        );
        let mut etat = etat_de(&partie);
        etat.humains_vaincus = vec![HumainId::new("brigand")];
        etat.sac.ajouter_potions(10);
        let partie = Partie::restaurer(etat).unwrap();
        assert_eq!(
            partie.annexe_a_decouvrir(zone_de_la_lisiere, contenu.annexes()),
            Some(terrier.clone())
        );
        let mut deroulement = au_camp(partie, &contenu, &depot);
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::PartirEnExpedition(terrier.clone()),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::ZoneFermee)
        );
        let mut aleatoire = GenerateurAleatoire::new(5);
        let mut vus = Vec::new();
        for _ in 0..200 {
            vus.extend(jouer_l_expedition(
                &mut deroulement,
                &contenu,
                &depot,
                &lisiere,
                &mut aleatoire,
            ));
            if deroulement.partie().unwrap().annexe_ouverte(&terrier) {
                break;
            }
        }
        let decouvertes = vus
            .iter()
            .filter(|evenement| {
                matches!(evenement, Evenement::AnnexeDecouverte { nom, .. } if nom == "Terrier")
            })
            .count();
        assert_eq!(decouvertes, 1);
        let partie = deroulement.partie().unwrap();
        assert_eq!(
            partie.annexe_a_decouvrir(zone_de_la_lisiere, contenu.annexes()),
            None
        );
        let philtres = partie.philtres_chromatiques();
        let vus = jouer_l_expedition(&mut deroulement, &contenu, &depot, &terrier, &mut aleatoire);
        let tresors = vus
            .iter()
            .filter(|evenement| {
                matches!(
                    evenement,
                    Evenement::Coffre {
                        tresor: Some(Tresor::PhiltreChromatique),
                        ..
                    }
                )
            })
            .count();
        let coffres = vus
            .iter()
            .filter(|evenement| matches!(evenement, Evenement::Coffre { .. }))
            .count();
        assert_eq!(tresors, coffres);
        let partie = deroulement.partie().unwrap();
        assert_eq!(
            partie.philtres_chromatiques(),
            philtres + u32::try_from(tresors).unwrap()
        );
        assert!(!partie.est_exploree(&terrier));
    }

    /// Section 9.10 : fermée avant la chute du Démiurge, la zone légendaire
    /// s'achève par le légendaire, capturé d'office une fois vaincu, qui ne
    /// revient plus.
    #[test]
    fn le_legendaire_vaincu_est_capture_d_office_et_ne_revient_plus() {
        use crate::legendaire::{Legendaire, ZoneLegendaire};

        let contenu = ContenuV1::new();
        let niveau = Niveau::new(5).unwrap();
        // Une seule étape : le légendaire, sans combat sauvage avant lui.
        let abysses = Zone::new(
            ZoneId::new("abysses"),
            "Abysses",
            (niveau, niveau),
            1,
            1,
            vec![contenu.espece(&EspeceId::new("loup")).unwrap()],
            Vec::new(),
            crate::donnees_de_test::parametres_expedition_automatique(),
            crate::donnees_de_test::parametres_coffre(),
        )
        .unwrap();
        let legendaire = Legendaire::new(
            contenu.espece(&EspeceId::new("cerf-blanc")).unwrap(),
            Niveau::new(10).unwrap(),
            "Les flots se soulèvent.",
            "Le Cerf blanc vous rejoint.",
        )
        .unwrap();
        let contenu =
            contenu.avec_zones_legendaires(vec![ZoneLegendaire::new(abysses, legendaire)]);
        let depot = DepotMemoire::default();
        let zone = ZoneId::new("abysses");
        let partie = partie_pour_le_demiurge(&contenu);
        let mut deroulement = au_camp(partie.clone(), &contenu, &depot);
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::PartirEnExpedition(zone.clone()),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::ZoneFermee)
        );
        let mut etat = etat_de(&partie);
        etat.beherit = EtatDuBeherit::DemiurgeVaincu;
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let mut aleatoire = GenerateurAleatoire::new(3);
        let vus = jouer_l_expedition(&mut deroulement, &contenu, &depot, &zone, &mut aleatoire);
        assert!(vus.contains(&Evenement::LegendaireRencontre(
            "Les flots se soulèvent.".to_string()
        )));
        assert!(vus.contains(&Evenement::LegendaireCapture(
            "Le Cerf blanc vous rejoint.".to_string()
        )));
        let cerf = EspeceId::new("cerf-blanc");
        let partie = deroulement.partie().unwrap();
        assert!(partie.legendaire_capture(&cerf));
        let collection = partie.collection();
        let capture = collection
            .equipe()
            .iter()
            .chain(collection.camp())
            .find(|monstre| monstre.espece().id() == &cerf)
            .expect("le légendaire a rejoint le joueur");
        assert_eq!(capture.niveau().valeur(), 10);
        assert_eq!(capture.valeurs_internes(), &ValeursInternes::maximales());
        // Aucun filet n'a servi.
        assert_eq!(
            partie.sac().filets().nombre(crate::filet::Filet::Chanvre),
            10
        );
        let vus = jouer_l_expedition(&mut deroulement, &contenu, &depot, &zone, &mut aleatoire);
        assert!(
            !vus.iter()
                .any(|evenement| matches!(evenement, Evenement::LegendaireRencontre(_)))
        );
        // La zone ne compte pas comme explorée.
        assert!(!deroulement.partie().unwrap().est_exploree(&zone));
    }

    fn partie_pour_la_tour(contenu: &ContenuV1) -> Partie {
        let mut partie = partie_de(salamandre(contenu, 40));
        partie.prendre_monstre_rare(monstre_du_contenu(contenu, "crapaud", 40));
        partie.prendre_monstre_rare(monstre_du_contenu(contenu, "mandragore", 40));
        let mut etat = etat_de(&partie);
        etat.zones_explorees = vec![ZoneId::new("lisiere")];
        etat.sac.ajouter_potions(10);
        Partie::restaurer(etat).unwrap()
    }

    /// Section 9.5 : la Tour s'ouvre avec la Lisière menée au bout.
    #[test]
    fn la_tour_reste_fermee_avant_la_lisiere_menee_au_bout() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_de(salamandre(&contenu, 40)), &contenu, &depot);
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::PartirEnExpedition(contenu.tour().id().clone()),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::ZoneFermee)
        );
    }

    /// Section 9.5 : des étapes sans fin, un gardien à l'étape 10 suivi d'un
    /// repos, et le record de la dernière étape franchie au retour.
    #[test]
    fn une_descente_dans_la_tour_passe_le_gardien_et_bat_le_record() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let tour = contenu.tour().id().clone();
        let mut deroulement = au_camp(partie_pour_la_tour(&contenu), &contenu, &depot);
        let mut aleatoire = GenerateurAleatoire::new(5);
        let mut vus = Vec::new();
        let mut choix = Choix::PartirEnExpedition(tour.clone());
        for _ in 0..5_000 {
            let mut contexte = Contexte {
                contenu: &contenu,
                aleatoire: &mut aleatoire,
                depot: &depot,
                maintenant: Horodatage::depuis_secondes(0),
            };
            vus.extend(deroulement.repondre(choix, &mut contexte).unwrap());
            if deroulement.attente() == Attente::Camp {
                break;
            }
            if let Some(expedition) = deroulement.expedition() {
                assert!(expedition.est_sans_fin());
                assert_eq!(expedition.zone().id(), &tour);
            }
            choix = jouer_un_coup(&deroulement, &contenu);
        }
        assert_eq!(deroulement.attente(), Attente::Camp);
        assert!(matches!(
            &vus[..],
            [Evenement::Depart(nom), Evenement::EntreeDansLaTour(_), Evenement::Etape { numero: 1, total: None }, ..]
                if nom == "Tour sans fond"
        ));
        let derniere = vus
            .iter()
            .filter_map(|evenement| match evenement {
                Evenement::Etape { numero, .. } => Some(*numero),
                _ => None,
            })
            .max()
            .unwrap();
        assert!(derniere > 10, "descente arrêtée à l'étape {derniere}");
        let gardien = vus
            .iter()
            .position(|evenement| matches!(evenement, Evenement::GardienDeLaTour(_)))
            .expect("gardien à l'étape 10");
        assert!(matches!(
            &vus[gardien - 1],
            Evenement::Etape { numero: 10, .. }
        ));
        assert!(
            vus.iter()
                .any(|evenement| matches!(evenement, Evenement::ReposDansLaTour(_)))
        );
        let defaite = vus
            .iter()
            .any(|evenement| matches!(evenement, Evenement::Defaite { .. }));
        let franchies = derniere - u32::from(defaite);
        assert!(vus.contains(&Evenement::NouveauRecordDeLaTour(franchies)));
        assert!(
            !vus.iter()
                .any(|evenement| matches!(evenement, Evenement::ZoneExploree(_)))
        );
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.records_de_la_tour().seul, franchies);
        assert_eq!(sauvegarde.records_de_la_tour().a_deux, 0);
        assert!(!sauvegarde.est_exploree(&tour));
    }

    /// Au retour d'une expédition manuelle, quelle qu'en soit l'issue, la
    /// location d'un monstre se décompte ; à la dernière, il repart chez
    /// Malvoise (section 14.2).
    #[test]
    fn le_monstre_loue_repart_au_retour_de_sa_derniere_expedition() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut partie = partie_de(salamandre(&contenu, 30));
        let cerf = Monstre::creer(
            contenu.espece(&EspeceId::new("cerf-blanc")).unwrap(),
            Niveau::new(20).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
        .louer(1);
        partie.collection_mut().accueillir_capture(cerf);
        let mut deroulement = au_camp(partie, &contenu, &depot);
        let mut aleatoire = GenerateurAleatoire::new(3);
        let mut finies = Vec::new();
        let mut retours = 0;
        for _ in 0..2_000 {
            let choix = jouer_un_coup(&deroulement, &contenu);
            let mut contexte = Contexte {
                contenu: &contenu,
                aleatoire: &mut aleatoire,
                depot: &depot,
                maintenant: Horodatage::depuis_secondes(0),
            };
            for evenement in deroulement.repondre(choix, &mut contexte).unwrap() {
                match evenement {
                    Evenement::LocationFinie(monstre) => finies.push(monstre),
                    Evenement::RetourAuCamp => retours += 1,
                    _ => {}
                }
            }
            if retours == 2 {
                break;
            }
        }
        assert_eq!(finies.len(), 1);
        assert_eq!(finies[0].espece().id().as_str(), "cerf-blanc");
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        let collection = sauvegarde.collection();
        assert!(collection.monstre_loue().is_none());
        assert!(
            collection
                .equipe()
                .iter()
                .chain(collection.camp())
                .all(|monstre| monstre.espece().id().as_str() != "cerf-blanc")
        );
    }

    #[test]
    fn une_partie_complete_jusqu_a_l_epilogue() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut aleatoire = GenerateurAleatoire::new(2026);
        let mut deroulement = Deroulement::demarrer(depot.charger(&contenu));
        let mut vus = Vec::new();
        let mut annonces = Vec::new();
        let mut attendus: Vec<EspeceId> = Vec::new();
        let mut combats_de_jour = 0;
        for _ in 0..200_000 {
            if let Attente::Annonce(annonce) = deroulement.attente() {
                annonces.push(annonce);
                // Le combat annoncé dit qui attend : l'humain lui-même, ou
                // l'Apôtre (section 5.1).
                match (annonce, deroulement.combat_annonce()) {
                    (Annonce::Combat, Some(combat)) => {
                        let attend = combat
                            .heros(Cote::Adversaire)
                            .first()
                            .unwrap_or_else(|| combat.monstre(combat.actif(Cote::Adversaire)));
                        if !attendus.contains(attend.espece().id()) {
                            attendus.push(attend.espece().id().clone());
                        }
                    }
                    (Annonce::Indice, None) => {}
                    (annonce, combat) => panic!("{annonce:?} : {combat:?}"),
                }
            }
            // Les combats de fin de zone ne suivent pas le cycle ; les autres
            // combats des zones, partis à minuit UTC, sont de jour (section
            // 9.6).
            if let Attente::ActionDeCombat(combat) = deroulement.attente() {
                // Tous les combats d'une expédition suivent sa météo, ceux
                // de fin de zone compris (section 9.7).
                let meteo = deroulement.expedition().and_then(Expedition::meteo);
                assert_eq!(combat.meteo(), meteo);
                if combat.est_contre_un_humain() || combat.est_contre_un_apotre() {
                    assert_eq!(combat.jour_ou_nuit(), None);
                } else if combat.jour_ou_nuit() == Some(JourOuNuit::Jour) {
                    combats_de_jour += 1;
                }
            }
            let choix = jouer_un_coup(&deroulement, &contenu);
            let mut contexte = Contexte {
                contenu: &contenu,
                aleatoire: &mut aleatoire,
                depot: &depot,
                maintenant: Horodatage::depuis_secondes(0),
            };
            let evenements = deroulement
                .repondre(choix.clone(), &mut contexte)
                .unwrap_or_else(|erreur| panic!("{choix:?} refusé : {erreur}"));
            vus.extend(evenements);
            // Jusqu'à la chute du troisième Apôtre, puis le retour au camp.
            let epilogues = vus
                .iter()
                .filter(|evenement| matches!(evenement, Evenement::Epilogue(_)))
                .count();
            if epilogues == 3 && deroulement.attente() == Attente::Camp {
                break;
            }
        }
        assert!(combats_de_jour > 0);
        let indices = vus
            .iter()
            .filter(|evenement| matches!(evenement, Evenement::Indice { .. }))
            .count();
        // Le septième, au bout de l'expédition qui abat le Margrave, ouvre le
        // Désert de sel (section 9.4).
        assert_eq!(indices, 7);
        // Chaque indice et chaque combat de fin de zone sont d'abord annoncés
        // par un bouton (section 9.2).
        let annonces_d_indice = annonces
            .iter()
            .filter(|annonce| **annonce == Annonce::Indice)
            .count();
        assert_eq!(annonces_d_indice, indices);
        let fins_de_zone = vus
            .iter()
            .filter(|evenement| {
                matches!(
                    evenement,
                    Evenement::HumainRencontre(_) | Evenement::RecitDeLApotre(_)
                )
            })
            .count();
        let annonces_de_combat = annonces
            .iter()
            .filter(|annonce| **annonce == Annonce::Combat)
            .count();
        assert!(fins_de_zone >= 7);
        assert_eq!(annonces_de_combat, fins_de_zone);
        for attendu in ["humain-brigand", "humain-bourreau", "baron-pendu"] {
            assert!(attendus.contains(&EspeceId::new(attendu)), "{attendu}");
        }
        assert!(
            vus.iter()
                .any(|evenement| matches!(evenement, Evenement::Epilogue(_)))
        );
        for conseil in [
            Conseil::PremierCombat,
            Conseil::PremierArretEntreDeuxEtapes,
            Conseil::PremierRetourAuCamp,
        ] {
            let montres = vus
                .iter()
                .filter(|evenement| **evenement == Evenement::Conseil(conseil))
                .count();
            assert_eq!(montres, 1, "{conseil:?}");
        }
        // Chaque Apôtre attendait à la dernière étape : l'épilogue est suivi
        // du retour au camp et de la sauvegarde. Le Baron laisse l'indice 3,
        // qui ouvre l'Abbaye, l'Abbesse l'indice 5, qui ouvre le Col, le
        // Margrave l'indice 7, qui ouvre le Désert (section 9.4).
        assert_eq!(deroulement.attente(), Attente::Camp);
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.indices_trouves(), 7);
        assert_eq!(
            sauvegarde.apotres_vaincus(),
            &[
                EspeceId::new("baron-pendu"),
                EspeceId::new("abbesse-ecorchee"),
                EspeceId::new("margrave-de-givre")
            ]
        );
        // Les Apôtres ne se capturent pas : chacun laisse son âme, et sa
        // fiche s'ouvre au codex (section 9.4).
        let ames: Vec<&Evenement> = vus
            .iter()
            .filter(|evenement| matches!(evenement, Evenement::AmeDApotreRecueillie(_)))
            .collect();
        assert_eq!(ames.len(), 3);
        assert!(matches!(
            ames[0],
            Evenement::AmeDApotreRecueillie(texte) if texte.contains("Baron pendu")
        ));
        assert_eq!(sauvegarde.ames_d_apotres(), sauvegarde.apotres_vaincus());
        // Chacun laisse aussi un Philtre chromatique, juste après son âme (section
        // 14.3).
        let apres_les_ames: Vec<&Evenement> = vus
            .windows(2)
            .filter(|paire| matches!(paire[0], Evenement::AmeDApotreRecueillie(_)))
            .map(|paire| &paire[1])
            .collect();
        assert_eq!(
            apres_les_ames,
            vec![
                &Evenement::PhiltreChromatiqueRamasse,
                &Evenement::PhiltreChromatiqueRamasse,
                &Evenement::PhiltreChromatiqueRamasse
            ]
        );
        let collection = sauvegarde.collection();
        assert!(
            collection
                .equipe()
                .iter()
                .chain(collection.camp())
                .all(|monstre| monstre.espece().rarete() != crate::espece::Rarete::Apotre)
        );
        for apotre in sauvegarde.apotres_vaincus() {
            assert_eq!(sauvegarde.codex().etat(apotre), EtatCodex::Capturee);
        }
        // Le brigand, le Bourreau, l'Inquisiteur et le Porte-étendard ont
        // barré leur zone
        // jusqu'à leur défaite (section 10.7).
        assert_eq!(
            sauvegarde.humains_vaincus(),
            &[
                HumainId::new("brigand"),
                HumainId::new("bourreau"),
                HumainId::new("inquisiteur"),
                HumainId::new("porte-etendard")
            ]
        );
        // La défaite du Bourreau a libéré Gunther, une fois (section 13.4).
        let liberations = vus
            .iter()
            .filter(|evenement| matches!(evenement, Evenement::GuntherLibere(_)))
            .count();
        assert_eq!(liberations, 1);
        assert!(sauvegarde.forge().est_au_camp());
        // Sans expédition automatique ni marchand, l'argent du joueur est
        // celui des coffres et du brigand, moins celui des sacs perdus
        // (section 9.3).
        let trouve: u32 = vus
            .iter()
            .map(|evenement| match evenement {
                Evenement::Coffre { argent, .. } | Evenement::HumainVaincu { argent, .. } => {
                    *argent
                }
                _ => 0,
            })
            .sum();
        let perdu: u32 = vus
            .iter()
            .map(|evenement| match evenement {
                Evenement::Defaite { sac_perdu } => sac_perdu.argent(),
                _ => 0,
            })
            .sum();
        assert!(trouve > 0);
        assert_eq!(sauvegarde.sac().argent(), trouve - perdu);
        assert!(sauvegarde.coffre_du_camp().est_vide());
        // Malvoise est rencontré une fois et reste au camp ; chaque âme
        // recueillie l'est une fois, et reste portée tant qu'elle n'est pas
        // donnée (section 14.2).
        let malvoise = vus
            .iter()
            .filter(|evenement| matches!(evenement, Evenement::MalvoiseRencontre { .. }))
            .count();
        assert_eq!(malvoise, 1);
        assert!(sauvegarde.marchand_au_camp());
        let compter = |cherche: fn(&Evenement) -> bool| vus.iter().filter(|e| cherche(e)).count();
        let rencontrees = compter(|evenement| matches!(evenement, Evenement::AmeRencontree(_)));
        let recueillies = compter(|evenement| matches!(evenement, Evenement::AmeRecueillie { .. }));
        assert!(rencontrees >= recueillies);
        assert_eq!(sauvegarde.ames_portees().len(), recueillies);
        assert!(recueillies > 0);
        // Les capsules du joueur sont celles des coffres, moins celles des sacs
        // perdus.
        let trouvees = vus
            .iter()
            .filter(|evenement| {
                matches!(
                    evenement,
                    Evenement::Coffre {
                        capsule: Some(_),
                        ..
                    }
                )
            })
            .count();
        let perdues: usize = vus
            .iter()
            .map(|evenement| match evenement {
                Evenement::Defaite { sac_perdu } => sac_perdu.capsules().len(),
                _ => 0,
            })
            .sum();
        assert_eq!(sauvegarde.capsules().count(), trouvees - perdues);
    }

    /// Réponse donnée à cette heure, avec ces tirages.
    fn repondre_a(
        deroulement: &mut Deroulement,
        choix: Choix,
        contenu: &ContenuV1,
        depot: &DepotMemoire,
        secondes: u64,
        tirages: Vec<u32>,
    ) -> Result<Vec<Evenement>, ErreurChoix> {
        let mut aleatoire = AleatoireScripte::new(tirages);
        let mut contexte = Contexte {
            contenu,
            aleatoire: &mut aleatoire,
            depot,
            maintenant: Horodatage::depuis_secondes(secondes),
        };
        deroulement.repondre(choix, &mut contexte)
    }

    fn monstre_du_contenu(contenu: &ContenuV1, id: &str, niveau: u8) -> Monstre {
        let espece = contenu.espece(&EspeceId::new(id)).unwrap();
        Monstre::creer(
            espece,
            Niveau::new(niveau).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    /// Salamandre de niveau 13 (34 PV, 4 attaques) et deux Loups de niveau 3
    /// (16 PV), avec la Lisière et le Marais déjà explorés.
    fn partie_pour_les_expeditions(contenu: &ContenuV1) -> Partie {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
        let mut partie = Partie::nouvelle(heros, monstre_du_contenu(contenu, "salamandre", 13));
        partie.prendre_monstre_rare(monstre_du_contenu(contenu, "loup", 3));
        partie.prendre_monstre_rare(monstre_du_contenu(contenu, "loup", 3));
        Partie::restaurer(EtatPartie {
            heros: partie.heros().clone(),
            collection: partie.collection().clone(),
            sac: partie.sac().clone(),
            coffre_du_camp: partie.coffre_du_camp().clone(),
            marchand_au_camp: partie.marchand_au_camp(),
            ames_donnees: Vec::new(),
            ames_portees: Vec::new(),
            ames_a_retrouver: Vec::new(),
            codex: partie.codex().clone(),
            zones_explorees: vec![ZoneId::new("lisiere"), ZoneId::new("marais")],
            indices_trouves: 3,
            apotres_vaincus: Vec::new(),
            humains_vaincus: Vec::new(),
            conseils_vus: Vec::new(),
            expeditions_automatiques: Vec::new(),
            attaques_a_choisir: Vec::new(),
            peche: Default::default(),
            forge: Default::default(),
            relations: Default::default(),
            machines: Default::default(),
            beherit: Default::default(),
            records_de_la_tour: Default::default(),
            rite: None,
            paliers_du_codex: Vec::new(),
            arene: Default::default(),
            decouvertes: Default::default(),
        })
        .unwrap()
    }

    fn partie_d_ysolde(contenu: &ContenuV1) -> Partie {
        let heros = Heros::new(Nom::new("Ysolde").unwrap(), Classe::Templier);
        Partie::nouvelle(heros, monstre_du_contenu(contenu, "mandragore", 5))
    }

    #[test]
    fn un_don_entre_deux_joueurs_au_camp_est_sauvegarde_des_deux_cotes() {
        let contenu = ContenuV1::new();
        let (depot_d_aldric, depot_d_ysolde) = (DepotMemoire::default(), DepotMemoire::default());
        let mut aldric = au_camp(partie(&contenu), &contenu, &depot_d_aldric);
        let mut ysolde = au_camp(partie_d_ysolde(&contenu), &contenu, &depot_d_ysolde);
        let offre = Offre::don(
            aldric.partie_au_camp().unwrap(),
            Emplacement::Equipe(1),
            ysolde.partie_au_camp().unwrap(),
        )
        .unwrap();
        let (du_donneur, du_receveur) = Deroulement::conclure_offre(
            &mut aldric,
            &mut ysolde,
            &offre,
            &depot_d_aldric,
            &depot_d_ysolde,
        )
        .unwrap();
        assert!(matches!(
            du_donneur.as_slice(),
            [Evenement::MonstreCede { heros, monstre }]
                if heros == "Ysolde" && monstre.espece().id().as_str() == "loup"
        ));
        assert!(matches!(
            du_receveur.as_slice(),
            [Evenement::MonstreRecu { heros, monstre }]
                if heros == "Aldric" && monstre.recu_de().map(Nom::as_str) == Some("Aldric")
        ));
        let sauvee = depot_d_ysolde.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvee.collection().equipe().len(), 2);
        let sauvee = depot_d_aldric.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvee.collection().equipe().len(), 1);
    }

    #[test]
    fn pas_d_echange_avec_un_joueur_en_expedition() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut aldric = au_camp(partie(&contenu), &contenu, &depot);
        let mut ysolde = au_camp(partie_d_ysolde(&contenu), &contenu, &depot);
        let offre = Offre::echange(
            aldric.partie_au_camp().unwrap(),
            Emplacement::Equipe(1),
            ysolde.partie_au_camp().unwrap(),
            Emplacement::Equipe(0),
        )
        .unwrap();
        let lisiere = Choix::PartirEnExpedition(ZoneId::new("lisiere"));
        let mut aleatoire = GenerateurAleatoire::new(3);
        let mut contexte = Contexte {
            contenu: &contenu,
            aleatoire: &mut aleatoire,
            depot: &depot,
            maintenant: Horodatage::depuis_secondes(0),
        };
        ysolde.repondre(lisiere, &mut contexte).unwrap();
        assert!(ysolde.partie_au_camp().is_none());
        let sauvegardes = depot.sauvegardes.borrow().len();
        assert_eq!(
            Deroulement::conclure_offre(&mut aldric, &mut ysolde, &offre, &depot, &depot),
            Err(ErreurEchange::ReceveurPasAuCamp)
        );
        assert_eq!(depot.sauvegardes.borrow().len(), sauvegardes);
        assert_eq!(aldric.partie().unwrap().collection().equipe().len(), 2);
    }

    #[test]
    fn un_monstre_recu_trop_fort_peut_desobeir_en_expedition() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
        let mut fort = monstre_du_contenu(&contenu, "salamandre", 15);
        fort.marquer_recu(Nom::new("Ysolde").unwrap());
        let mut deroulement = au_camp(Partie::nouvelle(heros, fort), &contenu, &depot);
        let mut aleatoire = GenerateurAleatoire::new(7);
        let mut choix = Choix::PartirEnExpedition(ZoneId::new("lisiere"));
        loop {
            let mut contexte = Contexte {
                contenu: &contenu,
                aleatoire: &mut aleatoire,
                depot: &depot,
                maintenant: Horodatage::depuis_secondes(0),
            };
            deroulement.repondre(choix, &mut contexte).unwrap();
            match deroulement.attente() {
                // Seule la Lisière est ouverte : il obéit jusqu'au niveau
                // 12, et désobéit 30 fois sur 100 au niveau 15.
                Attente::ActionDeCombat(combat) => {
                    assert_eq!(combat.chances_de_desobeir(0), 30);
                    break;
                }
                Attente::EntreDeuxEtapes => choix = Choix::ContinuerLExpedition,
                Attente::MonstreRare(_) => choix = Choix::LaisserLeMonstreRare,
                autre => panic!("combat attendu : {autre:?}"),
            }
        }
    }

    fn au_camp(partie: Partie, contenu: &ContenuV1, depot: &DepotMemoire) -> Deroulement {
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie)));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, contenu, depot).unwrap();
        deroulement
    }

    /// Joue un combat de la Fosse en attaquant toujours avec la première
    /// attaque, jusqu'à sa fin. Rend les événements du dernier tour.
    fn combattre_dans_la_fosse_jusqu_au_bout(
        deroulement: &mut Deroulement,
        contenu: &ContenuV1,
        depot: &DepotMemoire,
    ) -> Vec<Evenement> {
        let mut aleatoire = crate::aleatoire::GenerateurAleatoire::new(7);
        loop {
            let mut contexte = Contexte {
                contenu,
                aleatoire: &mut aleatoire,
                depot,
                maintenant: Horodatage::depuis_secondes(0),
            };
            let evenements = deroulement
                .repondre(Choix::Combat(Action::Attaquer(0)), &mut contexte)
                .unwrap();
            if deroulement.attente() == Attente::Camp {
                return evenements;
            }
        }
    }

    /// Section 9.8 : la Fosse s'ouvre après le brigand ; chaque palier se
    /// présente, se combat héros contre héros, et sa première victoire
    /// donne sa récompense ; l'abandon ne coûte rien.
    #[test]
    fn la_fosse_aux_damnes_se_combat_de_palier_en_palier() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut fermee = au_camp(partie(&contenu), &contenu, &depot);
        assert_eq!(
            repondre_sans_tirage(&mut fermee, Choix::EntrerDansLaFosse, &contenu, &depot),
            Err(ErreurChoix::Arene(ErreurArene::FosseFermee))
        );
        // Un Mage armé du grimoire du champion et de la robe des cendres, au
        // niveau 40, face au Gueux, au niveau 10.
        let mut etat = etat_de(&partie(&contenu));
        etat.humains_vaincus = vec![HumainId::new("brigand")];
        etat.coffre_du_camp = Inventaire::default();
        let arme = contenu.arme_du_champion(Classe::Mage).clone();
        let robe = contenu
            .objet_d_equipement(&ObjetDEquipementId::new("robe-des-cendres"))
            .unwrap();
        let portes = [
            (PlaceDEquipement::Arme, arme.id().clone()),
            (PlaceDEquipement::Armure, robe.id().clone()),
        ];
        etat.heros = etat.heros.clone().avec_equipement(
            crate::equipement::Equipement::restaurer(vec![arme, robe], &portes).unwrap(),
        );
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        repondre_sans_tirage(&mut deroulement, Choix::EntrerDansLaFosse, &contenu, &depot).unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::AdversaireDeLArene { palier: 1, adversaire } if adversaire.nom() == "Le Gueux"
        ));
        // Revenir au camp ne coûte rien.
        repondre_sans_tirage(&mut deroulement, Choix::RentrerAuCamp, &contenu, &depot).unwrap();
        assert_eq!(deroulement.attente(), Attente::Camp);
        repondre_sans_tirage(&mut deroulement, Choix::EntrerDansLaFosse, &contenu, &depot).unwrap();
        repondre_sans_tirage(&mut deroulement, Choix::Poursuivre, &contenu, &depot).unwrap();
        let Attente::ActionDeCombat(combat) = deroulement.attente() else {
            panic!("combat attendu");
        };
        assert!(combat.est_dans_l_arene());
        assert!(!combat.fuite_possible() && !combat.filet_possible());
        assert_eq!(combat.ordre_disponible(), None);
        assert_eq!(
            combat.monstre(combat.actif(Cote::Joueur)).niveau().valeur(),
            40
        );
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::Combat(Action::Fuir),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::HorsDePropos)
        );
        let evenements = combattre_dans_la_fosse_jusqu_au_bout(&mut deroulement, &contenu, &depot);
        assert!(evenements.iter().any(|evenement| matches!(
            evenement,
            Evenement::PalierDeLaFosseVaincu { palier: 1, recompense, conquise: false, .. }
                if recompense.argent == 40 && recompense.monstre.is_none()
        )));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.arene().paliers_vaincus, 1);
        assert_eq!(sauvegarde.coffre_du_camp().argent(), 40);
        // Le palier 2 se présente ; l'abandon est une défaite sans perte.
        repondre_sans_tirage(&mut deroulement, Choix::EntrerDansLaFosse, &contenu, &depot).unwrap();
        repondre_sans_tirage(&mut deroulement, Choix::Poursuivre, &contenu, &depot).unwrap();
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Abandonner, &contenu, &depot).unwrap();
        assert_eq!(
            evenements.last(),
            Some(&Evenement::DefaiteDansLaFosse { abandon: true })
        );
        assert_eq!(deroulement.attente(), Attente::Camp);
        assert_eq!(deroulement.partie().unwrap().arene().paliers_vaincus, 1);
    }

    /// Section 9.8 : le dernier palier donne le Taureau d'airain et l'arme
    /// du palier 4 de la classe du héros ; la Fosse est alors conquise.
    #[test]
    fn le_champion_vaincu_donne_le_taureau_et_l_arme_du_palier_4() {
        let contenu = ContenuV1::new();
        let mut etat = etat_de(&partie(&contenu));
        etat.humains_vaincus = vec![HumainId::new("brigand")];
        etat.arene.paliers_vaincus = 9;
        let arme = contenu
            .objet_d_equipement(&ObjetDEquipementId::new("sceptre-des-cendres"))
            .unwrap();
        let robe = contenu
            .objet_d_equipement(&ObjetDEquipementId::new("robe-des-cendres"))
            .unwrap();
        let portes = [
            (PlaceDEquipement::Arme, arme.id().clone()),
            (PlaceDEquipement::Armure, robe.id().clone()),
        ];
        etat.heros = etat.heros.clone().avec_equipement(
            crate::equipement::Equipement::restaurer(vec![arme, robe], &portes).unwrap(),
        );
        let partie = Partie::restaurer(etat).unwrap();
        let (palier, champion) = partie.palier_a_affronter(contenu.fosse()).unwrap();
        assert_eq!((palier, champion.nom()), (10, "Le Champion de la Fosse"));
        // Adversaire miroir : il prend la classe du héros, Mage, et son arme
        // du palier 3 (section 9.8).
        assert_eq!(champion.classe(), Classe::Mage);
        let attaques: Vec<String> = champion
            .generer_heros()
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str().to_string())
            .collect();
        assert_eq!(
            attaques,
            ["rayon-occulte", "onde-engourdissante", "emprise-mentale"]
        );
        let mut partie = partie;
        let recompense = partie.vaincre_le_palier(
            palier,
            &champion,
            contenu.arme_du_champion(Classe::Mage),
            &mut crate::aleatoire::GenerateurAleatoire::new(1),
        );
        assert_eq!(recompense.argent, 400);
        let (taureau, placement) = recompense.monstre.unwrap();
        assert_eq!(taureau.espece().id().as_str(), "taureau-d-airain");
        assert_eq!(taureau.niveau().valeur(), 35);
        assert_eq!(placement, Placement::Equipe(2));
        assert_eq!(
            recompense.arme.unwrap().id().as_str(),
            "grimoire-du-champion"
        );
        assert_eq!(
            partie
                .palier_a_affronter(contenu.fosse())
                .map(|(palier, _)| palier),
            Err(ErreurArene::FosseConquise)
        );
    }

    #[test]
    fn reclamer_la_recompense_du_codex_la_range_au_coffre() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie(&contenu));
        etat.coffre_du_camp = Inventaire::default();
        for espece in contenu.especes().iter().take(12) {
            etat.codex.noter_capture(espece.id());
        }
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let evenements = repondre_sans_tirage(
            &mut deroulement,
            Choix::ReclamerRecompenseDuCodex,
            &contenu,
            &depot,
        )
        .unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::RecompenseDuCodex(recompense)]
                if recompense.palier == crate::recompenses_du_codex::PalierDuCodex::Especes(10)
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.coffre_du_camp().argent(), 300);
        assert_eq!(sauvegarde.paliers_du_codex().len(), 1);
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::ReclamerRecompenseDuCodex,
                &contenu,
                &depot
            ),
            Err(ErreurChoix::Codex(ErreurCodex::AucuneRecompense))
        );
    }

    #[test]
    fn le_rite_du_sang_mele_se_confirme_puis_donne_son_petit() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let ressource = |id: &str| contenu.ressource(&RessourceId::new(id)).unwrap();
        let mut partie = partie_de(monstre_du_contenu(&contenu, "loup", 10));
        partie
            .collection_mut()
            .accueillir_capture(monstre_du_contenu(&contenu, "warg", 20));
        let mut etat = etat_de(&partie);
        etat.marchand_au_camp = true;
        etat.coffre_du_camp =
            Inventaire::new(60, 0, Vec::new()).avec_ressources(ressource("croc-de-loup"), 3);
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        let lancer = Choix::LancerLeRite {
            premier: Emplacement::Equipe(0),
            second: Emplacement::Equipe(1),
        };
        repondre_a(&mut deroulement, lancer, &contenu, &depot, 1_000, vec![]).unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::Confirmation(Confirmation::LancerLeRite(apercu))
                if apercu.petit.id().as_str() == "loup"
        ));
        let evenements = repondre_a(
            &mut deroulement,
            Choix::Oui,
            &contenu,
            &depot,
            1_000,
            vec![],
        )
        .unwrap();
        assert!(matches!(&evenements[..], [Evenement::RiteLance(_)]));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.argent(), 10);
        assert!(sauvegarde.collection().equipe().is_empty());
        assert_eq!(
            sauvegarde.rite().map(|rite| rite.fin()),
            Some(Horodatage::depuis_secondes(8_200))
        );
        assert_eq!(
            repondre_a(
                &mut deroulement,
                Choix::AccueillirLePetit,
                &contenu,
                &depot,
                8_199,
                vec![]
            ),
            Err(ErreurChoix::Rite(ErreurRite::PasEncoreFini(
                Horodatage::depuis_secondes(8_200)
            )))
        );
        // Pas shiny, sans l'attaque d'œuf, puis la première habileté du Loup.
        let evenements = repondre_a(
            &mut deroulement,
            Choix::AccueillirLePetit,
            &contenu,
            &depot,
            8_200,
            vec![2, 2, 0],
        )
        .unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::PetitNe(naissance)] if naissance.petit.niveau().valeur() == 1
        ));
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert!(sauvegarde.rite().is_none());
        assert_eq!(sauvegarde.collection().equipe().len(), 3);
    }

    #[test]
    fn envoyer_puis_rappeler_une_expedition_automatique() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_pour_les_expeditions(&contenu), &contenu, &depot);
        let lisiere = ZoneId::new("lisiere");
        let envoi = Choix::EnvoyerEnExpeditionAutomatique {
            zone: lisiere.clone(),
            monstres: vec![Emplacement::Equipe(1)],
            protection: Protection::default(),
        };
        let evenements =
            repondre_a(&mut deroulement, envoi, &contenu, &depot, 1_000, vec![]).unwrap();
        let [Evenement::ExpeditionAutomatiquePartie(expedition)] = &evenements[..] else {
            panic!("départ attendu : {evenements:?}");
        };
        assert_eq!(expedition.retour(), Horodatage::depuis_secondes(4_600));
        assert_eq!(depot.sauvegardes.borrow().len(), 1);
        assert_eq!(deroulement.attente(), Attente::Camp);
        assert_eq!(
            repondre_a(
                &mut deroulement,
                Choix::AccueillirExpeditionAutomatique(lisiere.clone()),
                &contenu,
                &depot,
                4_599,
                vec![]
            ),
            Err(ErreurChoix::ExpeditionAutomatique(
                ErreurExpeditionAutomatique::PasEncoreRentree
            ))
        );

        // Le rappel demande confirmation ; refusé, rien ne change.
        let rappel = Choix::RappelerExpeditionAutomatique(lisiere.clone());
        repondre_a(
            &mut deroulement,
            rappel.clone(),
            &contenu,
            &depot,
            2_000,
            vec![],
        )
        .unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::Confirmation(Confirmation::RappelerLExpedition(_))
        ));
        repondre_sans_tirage(&mut deroulement, Choix::Non, &contenu, &depot).unwrap();
        assert!(
            deroulement
                .partie()
                .unwrap()
                .expedition_automatique(&lisiere)
                .is_some()
        );

        repondre_a(&mut deroulement, rappel, &contenu, &depot, 2_000, vec![]).unwrap();
        let evenements = repondre_a(
            &mut deroulement,
            Choix::Oui,
            &contenu,
            &depot,
            2_000,
            vec![],
        )
        .unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::ExpeditionAutomatiqueRappelee { places, .. }]
                if places == &[Emplacement::Equipe(2)]
        ));
        let partie = deroulement.partie().unwrap();
        assert!(partie.expeditions_automatiques().is_empty());
        assert_eq!(partie.collection().equipe().len(), 3);
        assert!(partie.collection().camp().is_empty());
        assert_eq!(depot.sauvegardes.borrow().len(), 2);
    }

    #[test]
    fn une_expedition_rentree_ne_se_rappelle_plus() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_pour_les_expeditions(&contenu), &contenu, &depot);
        let lisiere = ZoneId::new("lisiere");
        let envoi = Choix::EnvoyerEnExpeditionAutomatique {
            zone: lisiere.clone(),
            monstres: vec![Emplacement::Equipe(1)],
            protection: Protection::default(),
        };
        repondre_a(&mut deroulement, envoi, &contenu, &depot, 0, vec![]).unwrap();
        assert_eq!(
            repondre_a(
                &mut deroulement,
                Choix::RappelerExpeditionAutomatique(lisiere),
                &contenu,
                &depot,
                3_600,
                vec![]
            ),
            Err(ErreurChoix::ExpeditionAutomatique(
                ErreurExpeditionAutomatique::DejaRentree
            ))
        );
        assert_eq!(deroulement.attente(), Attente::Camp);
    }

    #[test]
    fn accueillir_une_expedition_puis_choisir_l_attaque_a_oublier() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_pour_les_expeditions(&contenu), &contenu, &depot);
        let marais = ZoneId::new("marais");
        let envoi = Choix::EnvoyerEnExpeditionAutomatique {
            zone: marais.clone(),
            monstres: vec![Emplacement::Equipe(0), Emplacement::Equipe(1)],
            protection: Protection::default(),
        };
        repondre_a(&mut deroulement, envoi, &contenu, &depot, 0, vec![]).unwrap();

        // 2 h de base, 1 h 30 à deux. 40 dégâts en deux parts de 20 : le
        // Loup (16 PV) tombe et la Salamandre (34 PV) encaisse 24. Elle gagne
        // 600 XP et passe au niveau 14, où elle veut apprendre Fournaise ;
        // l'expédition rapporte aussi une potion et 55 pièces d'argent.
        let evenements = repondre_a(
            &mut deroulement,
            Choix::AccueillirExpeditionAutomatique(marais),
            &contenu,
            &depot,
            5_400,
            vec![40, 600, 1, 55],
        )
        .unwrap();
        let [
            Evenement::ExpeditionAutomatiqueRentree { zone, bilan },
            Evenement::Experience { monstre, rapport },
        ] = &evenements[..]
        else {
            panic!("retour attendu : {evenements:?}");
        };
        assert_eq!(zone, "Marais des Pendus");
        assert!(bilan.reussite);
        assert!(bilan.monstres[1].ko);
        assert_eq!(monstre.espece().id().as_str(), "salamandre");
        assert_eq!(rapport.niveau_apres, Niveau::new(14).unwrap());
        assert_eq!(deroulement.partie().unwrap().coffre_du_camp().potions(), 1);
        // 55, et 10 % de plus pour la Salamandre, offensive, debout.
        assert_eq!(bilan.argent, 60);
        assert_eq!(deroulement.partie().unwrap().coffre_du_camp().argent(), 60);
        // Sauvegardée dès l'accueil, avec l'attaque qui reste à choisir.
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.attaques_a_choisir().len(), 1);

        let Attente::OubliAttaque { nouvelle, .. } = deroulement.attente() else {
            panic!("une attaque à oublier était attendue");
        };
        assert_eq!(nouvelle.id().as_str(), "fournaise");
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::Gerer(ActionDeGestion::EnvoyerAuCamp(0)),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::HorsDePropos)
        );

        // Une reprise après un arrêt du jeu repropose le même choix.
        let mut reprise = au_camp(sauvegarde, &contenu, &depot);
        assert!(matches!(reprise.attente(), Attente::OubliAttaque { .. }));
        let evenements =
            repondre_sans_tirage(&mut reprise, Choix::OublierAttaque(0), &contenu, &depot).unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::AttaqueRemplacee { oubliee, .. }] if oubliee.id().as_str() == "souffle-chaud"
        ));
        assert_eq!(reprise.attente(), Attente::Camp);
        // Partie de l'équipe, elle y est revenue, après le Loup resté
        // (section 12.7).
        assert_eq!(bilan.monstres[0].emplacement, Emplacement::Equipe(1));
        let salamandre = &reprise.partie().unwrap().collection().equipe()[1];
        assert!(
            salamandre
                .attaques()
                .liste()
                .iter()
                .any(|attaque| attaque.id().as_str() == "fournaise")
        );
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert!(sauvegarde.attaques_a_choisir().is_empty());
    }

    #[test]
    fn toute_l_equipe_part_et_la_laisse_vide() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut partie = partie_pour_les_expeditions(&contenu);
        partie.prendre_monstre_rare(monstre_du_contenu(&contenu, "vipere", 5));
        let mut deroulement = au_camp(partie, &contenu, &depot);
        let envoi = Choix::EnvoyerEnExpeditionAutomatique {
            zone: ZoneId::new("lisiere"),
            monstres: vec![
                Emplacement::Equipe(0),
                Emplacement::Equipe(1),
                Emplacement::Equipe(2),
            ],
            protection: Protection::default(),
        };
        let evenements = repondre_a(&mut deroulement, envoi, &contenu, &depot, 0, vec![]).unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::ExpeditionAutomatiquePartie(_)]
        ));
        let collection = deroulement.partie().unwrap().collection();
        assert!(collection.equipe().is_empty());
        assert_eq!(collection.camp().len(), 1);
        assert_eq!(collection.places_reservees(), 3);
        // Sans équipe, pas d'expédition manuelle (section 13.2).
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::PartirEnExpedition(ZoneId::new("lisiere")),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::EquipeVide)
        );
    }

    fn repondre_avec_tirages(
        deroulement: &mut Deroulement,
        choix: Choix,
        contenu: &ContenuV1,
        depot: &DepotMemoire,
        maintenant: Horodatage,
        tirages: Vec<u32>,
    ) -> Result<Vec<Evenement>, ErreurChoix> {
        let mut aleatoire = AleatoireScripte::new(tirages);
        let mut contexte = Contexte {
            contenu,
            aleatoire: &mut aleatoire,
            depot,
            maintenant,
        };
        deroulement.repondre(choix, &mut contexte)
    }

    /// Partie à 2 h UTC, l'expédition est de nuit jusqu'au retour : la
    /// Lisière n'y donne que des Loups et des Corbeaux, de 2 niveaux de
    /// plus, et ses combats suivent la nuit (section 9.6).
    #[test]
    fn une_expedition_partie_de_nuit_le_reste() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie(&contenu))));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();
        // Combat (1), un monstre (1), la première espèce de la nuit (0),
        // niveau 5 (5), non shiny (2), valeurs internes, habileté (0).
        let evenements = repondre_avec_tirages(
            &mut deroulement,
            Choix::PartirEnExpedition(ZoneId::new("lisiere")),
            &contenu,
            &depot,
            Horodatage::depuis_secondes(2 * 3_600),
            vec![1, 1, 0, 5, 2, 0, 0, 0, 0, 0, 0, 0],
        )
        .unwrap();
        assert_eq!(
            evenements[1],
            Evenement::JourOuNuitAuDepart(JourOuNuit::Nuit)
        );
        let rencontre = evenements
            .iter()
            .find_map(|evenement| match evenement {
                Evenement::Rencontre(combat) => Some(combat),
                _ => None,
            })
            .unwrap();
        assert_eq!(rencontre.jour_ou_nuit(), Some(JourOuNuit::Nuit));
        let loup = &rencontre.adversaires()[0];
        assert_eq!(loup.espece().id().as_str(), "loup");
        assert_eq!(loup.niveau(), Niveau::new(7).unwrap());
        assert_eq!(
            deroulement.expedition().unwrap().jour_ou_nuit(),
            Some(JourOuNuit::Nuit)
        );
    }

    /// Un coffre fait déborder le sac plein : à la fin de l'étape, le joueur
    /// jette ce qu'il veut, sauf l'argent, qui ne prend pas de place, puis
    /// l'expédition reprend (section 13.3).
    #[test]
    fn un_sac_qui_deborde_se_vide_a_la_fin_de_l_etape() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie(&contenu));
        etat.sac = Inventaire::default().avec_filets(Filet::Chanvre, 30);
        let mut deroulement = Deroulement::demarrer(Ok(Some(Partie::restaurer(etat).unwrap())));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();
        // Un coffre d'une potion et de 12 pièces d'argent, sans capsule.
        repondre_avec_tirages(
            &mut deroulement,
            Choix::PartirEnExpedition(ZoneId::new("lisiere")),
            &contenu,
            &depot,
            Horodatage::depuis_secondes(0),
            vec![81, 1, 12, 21, 100],
        )
        .unwrap();
        assert_eq!(deroulement.attente(), Attente::SacPlein);
        assert_eq!(deroulement.partie().unwrap().exces_du_sac(), 1);
        for refuse in [Choix::Jeter(Objet::Argent(12)), Choix::ContinuerLExpedition] {
            assert_eq!(
                repondre_sans_tirage(&mut deroulement, refuse, &contenu, &depot),
                Err(ErreurChoix::HorsDePropos)
            );
        }
        assert_eq!(
            repondre_sans_tirage(
                &mut deroulement,
                Choix::Jeter(Objet::Soins(Soin::Potion, 2)),
                &contenu,
                &depot
            ),
            Err(ErreurChoix::Objet(ErreurObjet::Insuffisant))
        );
        let evenements = repondre_sans_tirage(
            &mut deroulement,
            Choix::Jeter(Objet::Filets(Filet::Chanvre, 1)),
            &contenu,
            &depot,
        )
        .unwrap();
        assert!(matches!(
            &evenements[0],
            Evenement::ObjetsJetes(jete) if jete.filets().nombre(Filet::Chanvre) == 1
        ));
        assert_eq!(deroulement.attente(), Attente::EntreDeuxEtapes);
        assert_eq!(deroulement.partie().unwrap().sac().places_prises(), 30);
    }

    /// Partie sous une météo, l'expédition la garde jusqu'au retour et le
    /// récit la dit (section 9.7).
    #[test]
    fn une_expedition_garde_la_meteo_de_son_depart() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let lisiere = contenu.zone(&ZoneId::new("lisiere")).unwrap();
        let (maintenant, meteo) = (0..)
            .map(|creneau| Horodatage::depuis_secondes(creneau * 30 * 60))
            .find_map(|maintenant| {
                Meteo::de_la_zone(&lisiere, maintenant).map(|meteo| (maintenant, meteo))
            })
            .unwrap();
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie(&contenu))));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, &contenu, &depot).unwrap();
        // Un coffre d'une potion et de 12 pièces d'argent, sans capsule.
        let evenements = repondre_avec_tirages(
            &mut deroulement,
            Choix::PartirEnExpedition(ZoneId::new("lisiere")),
            &contenu,
            &depot,
            maintenant,
            vec![81, 1, 12, 21, 100],
        )
        .unwrap();
        assert!(evenements.contains(&Evenement::MeteoAuDepart(meteo)));
        assert_eq!(deroulement.expedition().unwrap().meteo(), Some(meteo));
    }

    /// Ansgar au camp, avec la Salamandre et le Loup d'Aldric ; seule la
    /// Lisière, de niveau 3 à 7, est ouverte.
    fn partie_avec_ansgar(contenu: &ContenuV1) -> Partie {
        let mut etat = etat_de(&partie(contenu));
        etat.peche = Peche::restaurer(true, None, None).unwrap();
        Partie::restaurer(etat).unwrap()
    }

    /// Prise d'un Noyé de niveau 6 : nature (1), espèce (0), niveau (6),
    /// shiny (2 : non), six valeurs internes, puis habileté (0).
    const NOYE_DE_NIVEAU_6: [u32; 11] = [1, 0, 6, 2, 0, 0, 0, 0, 0, 0, 0];

    #[test]
    fn sans_ansgar_pas_de_peche() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie(&contenu), &contenu, &depot);
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, Choix::LancerLaLigne, &contenu, &depot),
            Err(ErreurChoix::Peche(ErreurPeche::AucunPecheur))
        );
    }

    #[test]
    fn ansgar_peche_un_monstre_que_le_joueur_garde() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_avec_ansgar(&contenu), &contenu, &depot);
        let evenements = repondre_a_l_heure(
            &mut deroulement,
            Choix::LancerLaLigne,
            &contenu,
            &depot,
            a(100),
        )
        .unwrap();
        assert_eq!(
            evenements,
            vec![Evenement::LigneLancee {
                fin: a(7_300),
                delai: Duration::from_secs(7_200)
            }]
        );
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.peche().ligne(), Some(a(7_300)));
        assert_eq!(
            repondre_a_l_heure(
                &mut deroulement,
                Choix::LancerLaLigne,
                &contenu,
                &depot,
                a(200)
            ),
            Err(ErreurChoix::Peche(ErreurPeche::LigneDejaALEau))
        );
        assert_eq!(
            repondre_a_l_heure(
                &mut deroulement,
                Choix::ReleverLaLigne,
                &contenu,
                &depot,
                a(7_299)
            ),
            Err(ErreurChoix::Peche(ErreurPeche::PasEncorePrete(a(7_300))))
        );
        let evenements = repondre_avec_tirages(
            &mut deroulement,
            Choix::ReleverLaLigne,
            &contenu,
            &depot,
            a(7_300),
            NOYE_DE_NIVEAU_6.to_vec(),
        )
        .unwrap();
        let noye = match &evenements[..] {
            [Evenement::PrisePechee(Prise::Monstre(monstre))] => Monstre::clone(monstre),
            autres => panic!("prise attendue : {autres:?}"),
        };
        assert_eq!(noye.espece().id().as_str(), "noye");
        assert_eq!(noye.niveau().valeur(), 6);
        assert_eq!(deroulement.attente(), Attente::Prise(&noye));
        // La prise est sauvegardée, rencontrée au codex, avant toute réponse.
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.peche().prise(), Some(&noye));
        assert_eq!(sauvegarde.peche().ligne(), None);
        assert_eq!(
            sauvegarde.codex().etat(noye.espece().id()),
            EtatCodex::Rencontree
        );
        // Rien d'autre au camp tant que le joueur n'a pas répondu.
        assert_eq!(
            repondre_a_l_heure(
                &mut deroulement,
                Choix::LancerLaLigne,
                &contenu,
                &depot,
                a(7_300)
            ),
            Err(ErreurChoix::HorsDePropos)
        );
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::GarderLaPrise, &contenu, &depot).unwrap();
        assert_eq!(evenements, vec![Evenement::RejointLEquipe(noye.clone())]);
        assert_eq!(deroulement.attente(), Attente::Camp);
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.collection().equipe()[2], noye);
        assert_eq!(sauvegarde.peche().prise(), None);
        assert_eq!(
            sauvegarde.codex().etat(noye.espece().id()),
            EtatCodex::Capturee
        );
    }

    #[test]
    fn un_monstre_relache_retourne_a_l_eau_et_un_objet_va_au_coffre() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut deroulement = au_camp(partie_avec_ansgar(&contenu), &contenu, &depot);
        let pecher = |deroulement: &mut Deroulement, tirages: Vec<u32>| {
            repondre_a_l_heure(deroulement, Choix::LancerLaLigne, &contenu, &depot, a(0)).unwrap();
            repondre_avec_tirages(
                deroulement,
                Choix::ReleverLaLigne,
                &contenu,
                &depot,
                a(7_200),
                tirages,
            )
            .unwrap()
        };
        let evenements = pecher(&mut deroulement, NOYE_DE_NIVEAU_6.to_vec());
        let Evenement::PrisePechee(Prise::Monstre(noye)) = &evenements[0] else {
            panic!("monstre attendu : {evenements:?}");
        };
        let noye = Monstre::clone(noye);
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::RelacherLaPrise, &contenu, &depot)
                .unwrap();
        assert_eq!(evenements, vec![Evenement::PriseRelachee(noye.clone())]);
        let partie = deroulement.partie().unwrap();
        assert_eq!(partie.collection().nombre_de_monstres(), 2);
        assert_eq!(
            partie.codex().etat(noye.espece().id()),
            EtatCodex::Rencontree
        );
        // Une potion, rangée dans le coffre, sans question.
        let evenements = pecher(&mut deroulement, vec![61]);
        assert_eq!(evenements, vec![Evenement::PrisePechee(Prise::Potion)]);
        assert_eq!(deroulement.attente(), Attente::Camp);
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.coffre_du_camp().potions(), 1);
        // Une bourse noyée de la Lisière, de 10 à 25 pièces d'argent.
        let evenements = pecher(&mut deroulement, vec![84, 25]);
        assert_eq!(evenements, vec![Evenement::PrisePechee(Prise::Argent(25))]);
        assert_eq!(deroulement.partie().unwrap().coffre_du_camp().argent(), 25);
        // Un booster, puis un Philtre chromatique (section 14.3).
        let evenements = pecher(&mut deroulement, vec![95, 2]);
        assert_eq!(
            evenements,
            vec![Evenement::PrisePechee(Prise::Booster(Booster::EcorceDeFer))]
        );
        let evenements = pecher(&mut deroulement, vec![100]);
        assert_eq!(
            evenements,
            vec![Evenement::PrisePechee(Prise::PhiltreChromatique)]
        );
        let coffre = deroulement.partie().unwrap().coffre_du_camp();
        assert_eq!(coffre.nombre_de_boosters(Booster::EcorceDeFer), 1);
        assert_eq!(coffre.philtres_chromatiques(), 1);
    }

    #[test]
    fn sans_place_la_ligne_ne_se_releve_pas() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut partie = partie_avec_ansgar(&contenu);
        let loup = partie.collection().equipe()[1].clone();
        while partie.collection().a_de_la_place() {
            partie.prendre_monstre_rare(loup.clone());
        }
        let mut deroulement = au_camp(partie, &contenu, &depot);
        repondre_a_l_heure(
            &mut deroulement,
            Choix::LancerLaLigne,
            &contenu,
            &depot,
            a(0),
        )
        .unwrap();
        // Refusé avant tout tirage : la ligne reste à relever.
        assert_eq!(
            repondre_a_l_heure(
                &mut deroulement,
                Choix::ReleverLaLigne,
                &contenu,
                &depot,
                a(7_200)
            ),
            Err(ErreurChoix::Peche(ErreurPeche::PlaceManquante))
        );
        assert!(deroulement.partie().unwrap().peche().est_prete(a(7_200)));
    }

    #[test]
    fn ansgar_sauve_a_l_etape_2_du_marais_s_installe_au_camp() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut etat = etat_de(&partie_pour_les_expeditions(&contenu));
        etat.collection = crate::collection::Collection::new(salamandre(&contenu, 40));
        let mut deroulement = au_camp(Partie::restaurer(etat).unwrap(), &contenu, &depot);
        // Un coffre d'une potion et de 25 pièces d'argent, sans capsule.
        let marais = Choix::PartirEnExpedition(ZoneId::new("marais"));
        repondre_a(
            &mut deroulement,
            marais,
            &contenu,
            &depot,
            0,
            vec![81, 1, 25, 100, 100],
        )
        .unwrap();
        // À l'étape 2, sans tirage de sa nature : le Noyé, au niveau 13, non
        // shiny, valeurs internes nulles, première habileté.
        let evenements = repondre_a(
            &mut deroulement,
            Choix::ContinuerLExpedition,
            &contenu,
            &depot,
            0,
            vec![2, 0, 0, 0, 0, 0, 0, 0],
        )
        .unwrap();
        assert!(matches!(
            &evenements[1],
            Evenement::PecheurRencontre(texte) if texte.contains("vieil homme")
        ));
        let Attente::ActionDeCombat(combat) = deroulement.attente() else {
            panic!("combat attendu");
        };
        // Un combat ordinaire contre un monstre sauvage (section 9.2).
        assert!(combat.filet_possible() && combat.fuite_possible());
        let noye = &combat.adversaires()[0];
        assert_eq!(noye.espece().id().as_str(), "noye");
        assert_eq!(noye.niveau().valeur(), 13);
        let vus = jouer_le_combat(&mut deroulement, &contenu, &depot);
        assert!(vus.iter().any(|evenement| matches!(
            evenement,
            Evenement::PecheurSauve(texte) if texte.contains("Ansgar")
        )));
        // Le Noyé ne lâche pas de ressource.
        assert!(
            !vus.iter()
                .any(|evenement| matches!(evenement, Evenement::RessourcesTrouvees(_)))
        );
        let partie = deroulement.partie().unwrap();
        assert!(partie.peche().pecheur_au_camp());
        // Il ne se rencontre plus.
        let marais = &contenu.zones()[1];
        let mut expedition = partie
            .partir_en_expedition(marais, contenu.piste())
            .unwrap();
        expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 25, 100, 100]));
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 25, 100, 100])),
            Some(EvenementExpedition::Coffre { .. })
        ));
    }
}
