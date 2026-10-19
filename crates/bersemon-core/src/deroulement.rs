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

use crate::aleatoire::Aleatoire;
use crate::ame::{Gardien, MarchandId, MarchandMort};
use crate::amelioration::{Amelioration, Chantier, ErreurAmelioration, PlanDAmelioration};
use crate::attaque::{Attaque, AttaqueId, ErreurAttaquesMonstre};
use crate::capsule::{Capsule, ErreurApprentissage};
use crate::collection::{Emplacement, ErreurCollection, Placement, TriDuCamp};
use crate::combat::{self, Action, BilanCombat, Combat, ErreurCombat, Issue, Phase};
use crate::contenu::ContenuV1;
use crate::espece::Espece;
use crate::evolution::{Circonstances, ErreurEvolution, evolution_possible, faire_evoluer};
use crate::expedition::{ErreurExpedition, EvenementExpedition, Expedition};
use crate::expedition_automatique::{
    BilanExpeditionAutomatique, ErreurExpeditionAutomatique, ExpeditionAutomatique,
};
use crate::habilete::{ErreurChangementDHabilete, Habilete};
use crate::heros::{Classe, Heros};
use crate::horodatage::Horodatage;
use crate::inventaire::{ErreurObjet, Inventaire, Objet};
use crate::marchand::ErreurMarchand;
use crate::monstre::{Monstre, RapportExperience};
use crate::nom::{ErreurNom, Nom};
use crate::partie::{Partie, creer_monstre_de_depart};
use crate::pierre::PierreDEvolution;
use crate::ressource::{Ressource, RessourceId};
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
    },
    /// Une expédition rentrée, accueillie au camp (section 12.7).
    AccueillirExpeditionAutomatique(ZoneId),
    /// Avant son terme, après confirmation (section 12.7).
    RappelerExpeditionAutomatique(ZoneId),
    /// Au camp, chez le marchand, sans confirmation : un soin de
    /// `Partie::soins_en_vente` (section 14.2).
    AcheterSoins(Soin, u32),
    /// Au camp, chez le marchand, après confirmation, comme un relâcher
    /// (section 14.2).
    Vendre(Emplacement),
    /// Au camp, chez le marchand, sans confirmation : une capsule de
    /// `ContenuV1::capsules` (section 14.2).
    AcheterCapsule(AttaqueId),
    /// Au camp, chez Malvoise, sans confirmation : une âme que le joueur
    /// porte (section 14.2).
    DonnerAme(MarchandId),
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
    /// Au camp, après confirmation : Gunther construit le niveau suivant de
    /// cette amélioration (section 13.4).
    LancerAmelioration(Amelioration),
    /// Au camp : le joueur constate le travail fini de Gunther.
    ConstaterAmelioration,
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
    PrendreLeMonstreRare,
    LaisserLeMonstreRare,
    Combat(Action),
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
    /// Le joueur poursuit par `Choix::Poursuivre`.
    Annonce(Annonce),
    Confirmation(&'a Confirmation),
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
    /// L'évolution est définitive, et consomme la pierre éventuelle (section
    /// 5.4).
    Evoluer {
        monstre: Monstre,
        vers: Espece,
        pierre: Option<PierreDEvolution>,
    },
}

/// Ce qui s'est passé, dans l'ordre. Les monstres et les combats sont des
/// copies prises sur le moment, pour que l'interface puisse les nommer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evenement {
    /// Après le choix de la classe, dont dépend son dernier paragraphe
    /// (section 6).
    Introduction(Classe),
    PartieCommencee(Monstre),
    /// Nom de la zone.
    Depart(String),
    /// Numéro de l'étape et nombre d'étapes de l'expédition (section 9.2).
    Etape {
        numero: u32,
        total: u32,
    },
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
    },
    MonstreRarePacifique(Monstre),
    MonstreRareHostile(Monstre),
    MonstreRareLaisse,
    RecitDeLApotre(String),
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
    /// Toute l'équipe est KO : retour au camp, tout le sac perdu (section
    /// 9.3).
    Defaite {
        sac_perdu: Inventaire,
    },
    /// Entre deux étapes ; `monstre` est le monstre soigné.
    SoinUtilise {
        soin: Soin,
        monstre: Monstre,
        effet: EffetDuSoin,
    },
    /// Nom de la zone menée jusqu'au bout.
    ZoneExploree(String),
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
    /// Le niveau construit compte désormais.
    AmeliorationConstatee(Chantier),
    /// Le monstre après le changement.
    HabileteChangee {
        monstre: Monstre,
        ancienne: Option<Habilete>,
    },
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
    MonstreVendu {
        monstre: Monstre,
        prix: u32,
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
    Evolution(ErreurEvolution),
    Amelioration(ErreurAmelioration),
    /// Le contenu n'a pas de plan pour ce niveau.
    PlanInconnu,
}

impl fmt::Display for ErreurChoix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurChoix::HorsDePropos => write!(f, "ce choix ne répond pas à la question"),
            ErreurChoix::Nom(erreur) => write!(f, "{erreur}"),
            ErreurChoix::ZoneInconnue(zone) => write!(f, "zone inconnue : {}", zone.as_str()),
            ErreurChoix::ZoneFermee => write!(f, "cette zone n'est pas encore sur la carte"),
            ErreurChoix::EquipeVide => write!(f, "l'équipe est vide"),
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
            ErreurChoix::Evolution(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Amelioration(erreur) => write!(f, "{erreur}"),
            ErreurChoix::PlanInconnu => write!(f, "aucun plan pour ce niveau"),
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

    /// `None` hors expédition.
    pub fn expedition(&self) -> Option<&Expedition> {
        match &self.etat {
            Etat::EnJeu(jeu) => jeu.expedition.as_ref().map(|en_cours| &en_cours.expedition),
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
                EnJeu::au_camp(Partie::clone(partie))
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
                let partie = Partie::nouvelle(Heros::new(nom.clone(), *classe), depart);
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
    if rapport.niveau_apres <= rapport.niveau_avant {
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
}

impl EnJeu {
    fn au_camp(partie: Partie) -> Etat {
        Etat::EnJeu(Box::new(Self {
            partie,
            expedition: None,
        }))
    }

    fn attente(&self) -> Attente<'_> {
        match &self.expedition {
            None => match self.partie.attaque_a_choisir() {
                Some((monstre, nouvelle)) => Attente::OubliAttaque { monstre, nouvelle },
                None => Attente::Camp,
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
        match choix {
            Choix::PartirEnExpedition(id) => {
                let zone = zone_du_contenu(contexte.contenu, id)?;
                let expedition = self
                    .partie
                    .partir_en_expedition(zone, contexte.contenu.piste())
                    .map_err(|erreur| match erreur {
                        ErreurExpedition::ZoneFermee => ErreurChoix::ZoneFermee,
                        ErreurExpedition::EquipeVide => ErreurChoix::EquipeVide,
                    })?;
                evenements.push(Evenement::Depart(zone.nom().to_string()));
                let mut en_cours = EnExpedition {
                    expedition,
                    moment: Moment::EntreDeuxEtapes,
                    ame: None,
                };
                let reponse = en_cours.etape_suivante(&mut self.partie, contexte, evenements);
                let reponse =
                    en_cours.annoncer_l_indice(&self.partie, contexte.contenu.piste(), reponse);
                self.expedition = Some(en_cours);
                if reponse == Reponse::RetourAuCamp {
                    self.rentrer(contexte, evenements);
                }
                Ok(Reponse::Faite)
            }
            Choix::Gerer(action) => {
                let reponse = gerer(&mut self.partie, action, confirme, evenements)?;
                if reponse == Reponse::Faite {
                    sauvegarder(&self.partie, contexte.depot, evenements);
                }
                Ok(reponse)
            }
            Choix::EnvoyerEnExpeditionAutomatique { zone, monstres } => {
                let zone = zone_du_contenu(contexte.contenu, zone)?;
                self.partie
                    .envoyer_en_expedition_automatique(zone, monstres, contexte.maintenant)
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
            Choix::ConstaterAmelioration => {
                let chantier = self
                    .partie
                    .constater_amelioration(contexte.maintenant)
                    .map_err(ErreurChoix::Amelioration)?;
                evenements.push(Evenement::AmeliorationConstatee(chantier));
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

    /// Soin de tous les monstres, indice éventuel, nouvelle zone, puis
    /// sauvegarde (sections 9.4, 13.1 et 13.9).
    fn rentrer(&mut self, contexte: &mut Contexte<'_>, evenements: &mut Vec<Evenement>) {
        let Some(en_cours) = self.expedition.take() else {
            return;
        };
        let piste = contexte.contenu.piste();
        let zones = contexte.contenu.zones();
        let avant = self.partie.zones_accessibles(zones, piste).len();
        let indice = self.partie.rentrer_au_camp(&en_cours.expedition, piste);
        if en_cours.expedition.est_reussie() {
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
    /// Le marchand mort dont le combat en cours peut recueillir l'âme.
    ame: Option<MarchandMort>,
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
}

/// Ce qui reste à régler une fois les suites d'un combat terminées.
#[derive(Debug, Clone)]
struct FinDeCombat {
    issue: Option<Issue>,
    contre_un_apotre: bool,
    contre_un_humain: bool,
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
                self.faire_place(partie, attente, choix, confirme, contexte, evenements)
            }
            Moment::AnnonceDIndice => {
                if *choix != Choix::Poursuivre {
                    self.moment = Moment::AnnonceDIndice;
                    return Err(ErreurChoix::HorsDePropos);
                }
                return Ok(Reponse::RetourAuCamp);
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
                .indice_suivant(partie.indices_trouves())
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
            Choix::ContinuerLExpedition => Ok(self.etape_suivante(partie, contexte, evenements)),
            Choix::UtiliserSoin(soin, index) => {
                let equipe = partie.collection().equipe();
                if let Some(confirmation) =
                    soin_a_confirmer(*soin, partie.sac().soins(), equipe, *index, confirme)
                {
                    return Ok(Reponse::a_confirmer(confirmation));
                }
                let effet = partie
                    .utiliser_soin(*soin, *index, &mut *contexte.aleatoire)
                    .map_err(ErreurChoix::Soin)?;
                evenements.push(Evenement::SoinUtilise {
                    soin: *soin,
                    monstre: partie.collection().equipe()[*index].clone(),
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
        let resultat = match (combat.phase(), choix) {
            (Phase::ChoixAction, Choix::Combat(action)) => {
                if let Action::UtiliserSoin(soin, index) = *action {
                    let confirmation =
                        soin_a_confirmer(soin, combat.soins(), combat.equipe(), index, confirme);
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
        if !tour.is_empty() {
            evenements.push(Evenement::Combat {
                evenements: tour,
                combat: combat.clone(),
            });
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
        self.placer(partie, captures.into(), Some(fin), contexte, evenements)
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
        contexte: &mut Contexte<'_>,
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
        self.conclure(partie, fin, contexte, evenements)
    }

    fn faire_place(
        &mut self,
        partie: &mut Partie,
        attente: CaptureSansPlace,
        choix: &Choix,
        confirme: bool,
        contexte: &mut Contexte<'_>,
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
        Ok(self.placer(partie, attente.restantes, attente.fin, contexte, evenements))
    }

    /// L'Apôtre vaincu ne réapparaît plus et l'épilogue suit ; un humain
    /// vaincu non plus, et son argent va dans le sac ; une victoire recueille
    /// l'âme du marchand mort, que la fuite ou la défaite laisse à retrouver ;
    /// une défaite met fin à l'expédition (sections
    /// 9.3, 9.4, 10.7 et 14.2).
    fn conclure(
        &mut self,
        partie: &mut Partie,
        fin: Option<FinDeCombat>,
        contexte: &mut Contexte<'_>,
        evenements: &mut Vec<Evenement>,
    ) -> Reponse {
        if let Some(fin) = fin {
            if fin.contre_un_apotre && fin.issue == Some(Issue::Victoire) {
                let apotre = contexte.contenu.piste().apotre();
                partie.noter_apotre_vaincu(apotre);
                evenements.push(Evenement::Epilogue(apotre.epilogue().to_string()));
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

    fn apres_l_etape(&mut self, partie: &mut Partie, evenements: &mut Vec<Evenement>) -> Reponse {
        if self.expedition.est_terminee() {
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
        evenements.push(Evenement::Etape {
            numero: self.expedition.etapes_faites(),
            total: self.expedition.nombre_etapes(),
        });
        match evenement {
            EvenementExpedition::Combat(groupe) => {
                let combat = partie.preparer_combat(groupe).expect(EQUIPE_APTE);
                evenements.push(Evenement::Rencontre(combat.clone()));
                self.engager(partie, combat, evenements)
            }
            EvenementExpedition::Coffre {
                potions,
                argent,
                capsule,
                capsule_d_habilete,
                pierre,
            } => {
                let argent_du_pillard = partie.ouvrir_coffre(potions, argent, capsule.clone());
                if capsule_d_habilete {
                    partie.trouver_capsule_d_habilete();
                }
                if let Some(pierre) = pierre {
                    partie.trouver_pierre(pierre);
                }
                evenements.push(Evenement::Coffre {
                    potions,
                    argent: argent + argent_du_pillard,
                    argent_du_pillard,
                    capsule,
                    capsule_d_habilete,
                    pierre,
                });
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
                .expect(EQUIPE_APTE);
                evenements.push(Evenement::Rencontre(combat.clone()));
                self.ame = Some(*marchand);
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
                    .expect(EQUIPE_APTE);
                evenements.push(Evenement::Rencontre(combat.clone()));
                self.engager(partie, combat, evenements)
            }
            EvenementExpedition::Apotre(monstre) => {
                // Le récit de l'Apôtre tient lieu d'annonce de la rencontre.
                let recit = contexte.contenu.piste().apotre().avant_combat();
                evenements.push(Evenement::RecitDeLApotre(recit.to_string()));
                let combat = partie
                    .preparer_combat_contre_apotre(monstre)
                    .expect(EQUIPE_APTE);
                self.moment = Moment::AnnonceDeCombat(combat);
                Reponse::Faite
            }
            EvenementExpedition::Humain { humain, equipe } => {
                // Comme pour l'Apôtre, son texte annonce la rencontre.
                evenements.push(Evenement::HumainRencontre(
                    humain.avant_combat().to_string(),
                ));
                let combat = partie
                    .preparer_combat_contre_humain(equipe, humain.ia())
                    .expect(EQUIPE_APTE);
                self.moment = Moment::AnnonceDeCombat(combat);
                Reponse::Faite
            }
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
    equipe: &[Monstre],
    index: usize,
    confirme: bool,
) -> Option<Confirmation> {
    let monstre = equipe.get(index)?;
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
    use crate::catalogue::Catalogue;
    use crate::codex::EtatCodex;
    use crate::espece::EspeceId;
    use crate::habilete::Habilete;
    use crate::humain::HumainId;
    use crate::niveau::Niveau;
    use crate::partie::EtatPartie;
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
            forge: *partie.forge(),
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
        // la première des deux de la Mandragore.
        let mut aleatoire = AleatoireScripte::new([1, 0]);
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
        // La partie est sauvegardée dès sa création (section 13.9).
        assert_eq!(depot.sauvegardes.borrow().len(), 1);
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
                Evenement::Etape {
                    numero: 1,
                    total: 5
                },
                Evenement::Coffre {
                    potions: 1,
                    argent: 12,
                    argent_du_pillard: 0,
                    capsule: None,
                    capsule_d_habilete: false,
                    pierre: None
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
            &evenements[2],
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

    /// Joueur simple : la meilleure attaque, une potion sous la moitié des
    /// PV, retour au camp en mauvaise posture, le Marais à partir du niveau
    /// 12, la Vallée à partir du niveau 24, toutes les captures possibles.
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
                let zone = if zones.len() > 2 && niveau >= 24 {
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
        let chantier = *sauvegarde.forge().chantier().unwrap();
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
                Choix::ConstaterAmelioration,
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
            Choix::ConstaterAmelioration,
            &contenu,
            &depot,
            a(4_600),
        )
        .unwrap();
        assert_eq!(evenements, vec![Evenement::AmeliorationConstatee(chantier)]);
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
            .chance_de_capture();
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
            .chance_de_capture();
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

    #[test]
    fn une_partie_complete_jusqu_a_l_epilogue() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut aleatoire = GenerateurAleatoire::new(2026);
        let mut deroulement = Deroulement::demarrer(depot.charger(&contenu));
        let mut vus = Vec::new();
        let mut annonces = Vec::new();
        for _ in 0..50_000 {
            if let Attente::Annonce(annonce) = deroulement.attente() {
                annonces.push(annonce);
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
            let fin = evenements
                .iter()
                .any(|evenement| matches!(evenement, Evenement::Epilogue(_)));
            vus.extend(evenements);
            if fin {
                break;
            }
        }
        let indices = vus
            .iter()
            .filter(|evenement| matches!(evenement, Evenement::Indice { .. }))
            .count();
        assert_eq!(indices, 2);
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
        assert!(fins_de_zone >= 3);
        assert_eq!(annonces_de_combat, fins_de_zone);
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
        // L'Apôtre attendait à la dernière étape : l'épilogue est suivi du
        // retour au camp et de la sauvegarde.
        assert_eq!(deroulement.attente(), Attente::Camp);
        let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
        assert_eq!(sauvegarde.indices_trouves(), 2);
        assert_eq!(
            sauvegarde.apotres_vaincus(),
            &[EspeceId::new("baron-pendu")]
        );
        // Le brigand et le Bourreau ont barré la Lisière et le Marais jusqu'à
        // leur défaite (section 10.7).
        assert_eq!(
            sauvegarde.humains_vaincus(),
            &[HumainId::new("brigand"), HumainId::new("bourreau")]
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
            forge: Default::default(),
        })
        .unwrap()
    }

    fn au_camp(partie: Partie, contenu: &ContenuV1, depot: &DepotMemoire) -> Deroulement {
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie)));
        repondre_sans_tirage(&mut deroulement, Choix::ReprendreLaPartie, contenu, depot).unwrap();
        deroulement
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
        assert_eq!(bilan.argent, 55);
        assert_eq!(deroulement.partie().unwrap().coffre_du_camp().argent(), 55);
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
}
