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
use crate::attaque::{Attaque, ErreurAttaquesMonstre};
use crate::collection::{Emplacement, ErreurCollection, Placement};
use crate::combat::{self, Action, BilanCombat, Combat, ErreurCombat, Issue, Phase};
use crate::contenu::ContenuV1;
use crate::expedition::{ErreurExpedition, EvenementExpedition, Expedition};
use crate::expedition_automatique::{
    BilanExpeditionAutomatique, ErreurExpeditionAutomatique, ExpeditionAutomatique,
};
use crate::heros::{Classe, Heros};
use crate::horodatage::Horodatage;
use crate::inventaire::{ErreurPotion, potion_sans_effet};
use crate::monstre::{Monstre, RapportExperience};
use crate::nom::{ErreurNom, Nom};
use crate::partie::{Partie, creer_monstre_de_depart};
use crate::sauvegarde::{ErreurSauvegarde, SauvegardeRepository};
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
    /// Au camp, toutes les actions ; entre deux étapes, seulement l'ordre de
    /// l'équipe (Q55).
    Gerer(ActionDeGestion),
    ContinuerLExpedition,
    /// Position, dans l'équipe, du monstre à soigner entre deux étapes.
    UtiliserPotion(usize),
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
    RelacherPourLaCapture(Emplacement),
    RelacherLaCapture,
    Oui,
    Non,
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
    /// L'équipe et le camp sont pleins : le joueur relâche un de ses monstres
    /// pour garder la capture, ou relâche la capture (Q12).
    PlaceManquante(&'a Monstre),
    Confirmation(&'a Confirmation),
}

/// Ce que le joueur doit confirmer avant que son choix s'applique.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Confirmation {
    /// Une nouvelle partie remplace celle de ce héros (Q56).
    RemplacerLaPartie(Heros),
    /// Le monstre a déjà tous ses PV : la potion serait perdue (Q53).
    PotionPerdue(Monstre),
    Relacher(Monstre),
    /// Les monstres rentreraient sans rien rapporter (section 12.7).
    RappelerLExpedition(ExpeditionAutomatique),
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
    /// Numéro de l'étape.
    Etape(u32),
    Coffre {
        potions: u32,
    },
    MonstreRarePacifique(Monstre),
    MonstreRareHostile(Monstre),
    MonstreRareLaisse,
    RecitDeLApotre(String),
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
    AttaqueRemplacee {
        monstre: Monstre,
        oubliee: Attaque,
        nouvelle: Attaque,
    },
    AttaqueAbandonnee {
        monstre: Monstre,
        nouvelle: Attaque,
    },
    RejointLEquipe(Monstre),
    PartPourLeCamp(Monstre),
    CaptureRelachee(Monstre),
    RelachePourLaCapture {
        relache: Monstre,
        capture: Monstre,
    },
    Epilogue(String),
    /// Toute l'équipe est KO : retour au camp et potions perdues (section
    /// 9.3).
    Defaite,
    PotionUtilisee {
        monstre: Monstre,
        pv_rendus: u32,
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
    /// Copie de l'expédition rappelée, avant le retour de ses monstres.
    ExpeditionAutomatiqueRappelee(ExpeditionAutomatique),
    /// Suivi d'un événement `Experience` par monstre qui en a gagné.
    ExpeditionAutomatiqueRentree {
        /// Nom de la zone.
        zone: String,
        bilan: BilanExpeditionAutomatique,
    },
    /// La partie continue : la sauvegarde sera retentée plus tard.
    SauvegardeImpossible(ErreurSauvegarde),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurChoix {
    /// Le choix ne répond pas à la question posée.
    HorsDePropos,
    Nom(ErreurNom),
    ZoneInconnue(ZoneId),
    ZoneFermee,
    Collection(ErreurCollection),
    Potion(ErreurPotion),
    Combat(ErreurCombat),
    Attaque(ErreurAttaquesMonstre),
    ExpeditionAutomatique(ErreurExpeditionAutomatique),
}

impl fmt::Display for ErreurChoix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurChoix::HorsDePropos => write!(f, "ce choix ne répond pas à la question"),
            ErreurChoix::Nom(erreur) => write!(f, "{erreur}"),
            ErreurChoix::ZoneInconnue(zone) => write!(f, "zone inconnue : {}", zone.as_str()),
            ErreurChoix::ZoneFermee => write!(f, "cette zone n'est pas encore sur la carte"),
            ErreurChoix::Collection(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Potion(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Combat(erreur) => write!(f, "{erreur}"),
            ErreurChoix::Attaque(erreur) => write!(f, "{erreur}"),
            ErreurChoix::ExpeditionAutomatique(erreur) => write!(f, "{erreur}"),
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
    AConfirmer(Confirmation),
    /// L'expédition est finie : il reste à rentrer au camp.
    RetourAuCamp,
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
        if let Reponse::AConfirmer(confirmation) = reponse {
            self.a_confirmer = Some((choix, confirmation));
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
                    return Ok(Reponse::AConfirmer(Confirmation::RemplacerLaPartie(heros)));
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
                    .partir_en_expedition(zone, contexte.contenu.piste(), &mut *contexte.aleatoire)
                    .map_err(|erreur| match erreur {
                        ErreurExpedition::ZoneFermee => ErreurChoix::ZoneFermee,
                    })?;
                evenements.push(Evenement::Depart(zone.nom().to_string()));
                let mut en_cours = EnExpedition {
                    expedition,
                    moment: Moment::EntreDeuxEtapes,
                };
                let reponse = en_cours.etape_suivante(&mut self.partie, contexte, evenements);
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
                    return Ok(Reponse::AConfirmer(Confirmation::RappelerLExpedition(
                        expedition,
                    )));
                }
                self.partie
                    .rappeler_expedition_automatique(zone, contexte.maintenant)
                    .map_err(ErreurChoix::ExpeditionAutomatique)?;
                evenements.push(Evenement::ExpeditionAutomatiqueRappelee(expedition));
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

    /// Comme après un combat (Q33), sur un monstre rentré au camp. Chaque
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
        let position_au_camp = a_choisir.position_au_camp;
        let monstre = self
            .partie
            .collection_mut()
            .monstre_du_camp_mut(position_au_camp)
            .ok_or(ErreurChoix::Collection(
                ErreurCollection::EmplacementInexistant(Emplacement::Camp(position_au_camp)),
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
                return Ok(Reponse::AConfirmer(Confirmation::Relacher(monstre)));
            }
            collection
                .relacher(*emplacement)
                .map(|relache| evenements.push(Evenement::Relache(relache)))
        }
    }
    .map_err(ErreurChoix::Collection)?;
    Ok(Reponse::Faite)
}

struct EnExpedition {
    expedition: Expedition,
    moment: Moment,
}

enum Moment {
    EntreDeuxEtapes,
    MonstreRare(Monstre),
    Combat(Combat),
    OubliAttaque(OubliEnCours),
    PlaceManquante(CaptureSansPlace),
}

/// Ce qui reste à régler une fois les suites d'un combat terminées.
#[derive(Debug, Clone, Copy)]
struct FinDeCombat {
    issue: Option<Issue>,
    contre_un_apotre: bool,
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
            Moment::Combat(combat) => match combat.phase() {
                Phase::ChoixAction => Attente::ActionDeCombat(combat),
                Phase::Remplacement => Attente::Remplacement(combat),
                Phase::Termine(_) => Attente::CaptureDesKo(combat),
            },
            Moment::OubliAttaque(oubli) => Attente::OubliAttaque {
                monstre: &oubli.bilan.equipe[oubli.monstre],
                nouvelle: &oubli.nouvelle,
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
        match std::mem::replace(&mut self.moment, Moment::EntreDeuxEtapes) {
            Moment::EntreDeuxEtapes => {
                self.entre_deux_etapes(partie, choix, confirme, contexte, evenements)
            }
            Moment::MonstreRare(monstre) => self.monstre_rare(partie, monstre, choix, evenements),
            Moment::Combat(combat) => {
                self.combattre(partie, combat, choix, confirme, contexte, evenements)
            }
            Moment::OubliAttaque(oubli) => self.oublier(partie, oubli, choix, contexte, evenements),
            Moment::PlaceManquante(attente) => {
                self.faire_place(partie, attente, choix, confirme, contexte, evenements)
            }
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
            Choix::UtiliserPotion(index) => {
                let equipe = partie.collection().equipe();
                if let Some(confirmation) =
                    potion_a_confirmer(partie.inventaire().potions(), equipe, *index, confirme)
                {
                    return Ok(Reponse::AConfirmer(confirmation));
                }
                let pv_rendus = partie
                    .utiliser_potion(*index, &mut *contexte.aleatoire)
                    .map_err(ErreurChoix::Potion)?;
                evenements.push(Evenement::PotionUtilisee {
                    monstre: partie.collection().equipe()[*index].clone(),
                    pv_rendus,
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
                if let Action::UtiliserPotion(index) = *action {
                    let confirmation =
                        potion_a_confirmer(combat.potions(), combat.equipe(), index, confirme);
                    if let Some(confirmation) = confirmation {
                        self.moment = Moment::Combat(combat);
                        return Ok(Reponse::AConfirmer(confirmation));
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
                });
                return Reponse::Faite;
            }
        }
        let captures = partie
            .conclure_combat(bilan)
            .expect("le combat rend l'équipe qu'il a reçue");
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
                    return Ok(Reponse::AConfirmer(Confirmation::Relacher(monstre)));
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

    /// L'Apôtre vaincu ne réapparaît plus et l'épilogue suit ; une défaite met
    /// fin à l'expédition (sections 9.3 et 9.4).
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
            if fin.issue == Some(Issue::Defaite) {
                self.expedition.noter_defaite();
                evenements.push(Evenement::Defaite);
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
        evenements.push(Evenement::Etape(self.expedition.etapes_faites()));
        match evenement {
            EvenementExpedition::Combat(groupe) => {
                let combat = partie.preparer_combat(groupe).expect(EQUIPE_APTE);
                evenements.push(Evenement::Rencontre(combat.clone()));
                self.engager(partie, combat, evenements)
            }
            EvenementExpedition::Coffre { potions } => {
                partie.ouvrir_coffre(potions);
                evenements.push(Evenement::Coffre { potions });
                self.apres_l_etape(partie, evenements)
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
                self.engager(partie, combat, evenements)
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

/// Sur un monstre qui a tous ses PV, la potion serait perdue : le joueur
/// confirme d'abord (Q53). Les autres cas sont laissés aux règles de la
/// potion, qui les refusent ou les appliquent.
fn potion_a_confirmer(
    potions: u32,
    equipe: &[Monstre],
    index: usize,
    confirme: bool,
) -> Option<Confirmation> {
    let monstre = equipe.get(index)?;
    let perdue = !confirme && potions > 0 && potion_sans_effet(monstre);
    perdue.then(|| Confirmation::PotionPerdue(monstre.clone()))
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
        Placement::PlaceManquante(capture) => Some(capture),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::{AleatoireScripte, GenerateurAleatoire};
    use crate::catalogue::Catalogue;
    use crate::espece::EspeceId;
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

        // Valeurs internes, puis shiny.
        let mut aleatoire = AleatoireScripte::new([31, 31, 31, 31, 31, 31, 1]);
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

        // Le dernier monstre de l'équipe ne se relâche pas : pas de
        // confirmation demandée.
        let dernier = Choix::Gerer(ActionDeGestion::Relacher(Emplacement::Equipe(0)));
        assert_eq!(
            repondre_sans_tirage(&mut deroulement, dernier, &contenu, &depot),
            Err(ErreurChoix::Collection(
                ErreurCollection::DernierMonstreDeLEquipe
            ))
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

        // 4 étapes, puis un coffre d'une potion.
        let mut aleatoire = AleatoireScripte::new([4, 81, 1]);
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
                Evenement::Etape(1),
                Evenement::Coffre { potions: 1 },
                Evenement::Conseil(Conseil::PremierArretEntreDeuxEtapes),
            ]
        );
        assert_eq!(deroulement.attente(), Attente::EntreDeuxEtapes);

        // La Salamandre a tous ses PV : la potion serait perdue (Q53).
        repondre_sans_tirage(&mut deroulement, Choix::UtiliserPotion(0), &contenu, &depot).unwrap();
        assert!(matches!(
            deroulement.attente(),
            Attente::Confirmation(Confirmation::PotionPerdue(_))
        ));
        let evenements =
            repondre_sans_tirage(&mut deroulement, Choix::Oui, &contenu, &depot).unwrap();
        assert!(matches!(
            &evenements[..],
            [Evenement::PotionUtilisee { pv_rendus: 0, .. }]
        ));
        assert_eq!(deroulement.partie().unwrap().inventaire().potions(), 0);

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

        // 4 étapes, un monstre rare : niveau 5, non shiny, valeurs internes.
        let mut aleatoire = AleatoireScripte::new([4, 96, 5, 2, 0, 0, 0, 0, 0, 0]);
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
    /// 12, toutes les captures possibles.
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
                let zone = if zones.len() > 1 && niveau >= 12 {
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
                    Some(index) if partie.inventaire().potions() > 0 => {
                        Choix::UtiliserPotion(index)
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
            Attente::PlaceManquante(_) => Choix::RelacherLaCapture,
            Attente::Confirmation(_) => Choix::Oui,
        }
    }

    #[test]
    fn une_partie_complete_jusqu_a_l_epilogue() {
        let contenu = ContenuV1::new();
        let depot = DepotMemoire::default();
        let mut aleatoire = GenerateurAleatoire::new(2026);
        let mut deroulement = Deroulement::demarrer(depot.charger(&contenu));
        let mut vus = Vec::new();
        for _ in 0..50_000 {
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
        assert_eq!(indices, 3);
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
        assert_eq!(sauvegarde.indices_trouves(), 3);
        assert_eq!(
            sauvegarde.apotres_vaincus(),
            &[EspeceId::new("baron-pendu")]
        );
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
            inventaire: partie.inventaire().clone(),
            codex: partie.codex().clone(),
            zones_explorees: vec![ZoneId::new("lisiere"), ZoneId::new("marais")],
            indices_trouves: 3,
            apotres_vaincus: Vec::new(),
            conseils_vus: Vec::new(),
            expeditions_automatiques: Vec::new(),
            attaques_a_choisir: Vec::new(),
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
            [Evenement::ExpeditionAutomatiqueRappelee(_)]
        ));
        let partie = deroulement.partie().unwrap();
        assert!(partie.expeditions_automatiques().is_empty());
        assert_eq!(partie.collection().camp().len(), 1);
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
        // 600 XP et passe au niveau 14, où elle veut apprendre Fournaise.
        let evenements = repondre_a(
            &mut deroulement,
            Choix::AccueillirExpeditionAutomatique(marais),
            &contenu,
            &depot,
            5_400,
            vec![40, 600, 1],
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
        assert_eq!(deroulement.partie().unwrap().inventaire().potions(), 1);
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
        let salamandre = &reprise.partie().unwrap().collection().camp()[0];
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
}
