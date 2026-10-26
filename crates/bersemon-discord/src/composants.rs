//! Identifiants des composants des messages : boutons, menus et formulaires.
//! Chacun porte ce qu'il déclenche et la version de la partie qu'il
//! affichait : un composant d'un message dépassé est ainsi reconnu, même s'il
//! propose un choix encore possible, comme Oui ou une attaque.
//!
//! Un bouton porte tout son choix ; un menu porte le genre du choix, et
//! l'option choisie le complète ; un formulaire porte le genre du texte saisi.

use bersemon_core::ame::MarchandId;
use bersemon_core::amelioration::Amelioration;
use bersemon_core::attaque::AttaqueId;
use bersemon_core::booster::Booster;
use bersemon_core::collection::{Emplacement, TriDuCamp};
use bersemon_core::combat::Action;
use bersemon_core::deroulement::{ActionDeGestion, Choix};
use bersemon_core::equipement::{ObjetDEquipementId, PlaceDEquipement};
use bersemon_core::espece::EspeceId;
use bersemon_core::expedition_automatique::MONSTRES_MAX;
use bersemon_core::expedition_automatique::Protection;
use bersemon_core::filet::Filet;
use bersemon_core::heros::Classe;
use bersemon_core::inventaire::Objet;
use bersemon_core::pierre::PierreDEvolution;
use bersemon_core::ressource::RessourceId;
use bersemon_core::soin::Soin;
use bersemon_core::zone::ZoneId;

use crate::duels::ActionDeDuel;
use crate::echanges::ActionDEchange;
use crate::groupes::ActionDeGroupe;
use crate::joueurs::IdJoueur;

/// Distingue les composants de Bersemon de ceux d'autres usages éventuels.
const PREFIXE: &str = "bersemon";

/// Ce que montre le camp. Changer de vue ne touche pas à la partie.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Vue {
    #[default]
    Camp,
    /// Avec la position, dans le codex, de l'espèce dont la fiche est
    /// ouverte.
    Codex(Option<usize>),
    /// Une page de la liste du codex après la première, qui est celle de
    /// `Codex(None)` : avec le Col du Givre-Noir, la liste ne tient plus dans
    /// un message (étape 82).
    PageDuCodex(u8),
    /// Avec le monstre choisi pour être géré.
    Gestion(Option<Emplacement>),
    /// Les zones explorées et leurs expéditions automatiques (section 12).
    Expeditions,
    /// Départ d'une expédition automatique en préparation : la position de
    /// la zone dans le contenu et les monstres déjà choisis.
    Preparation { zone: usize, choisis: Choisis },
    /// L'étal du marchand, une fois réfugié au camp (section 14.2).
    Marchand,
    /// Les monstres que le marchand rachète, avec leur prix.
    Vente,
    /// Les monstres que Malvoise vend et loue (section 14.2).
    Achats,
    /// L'Éclipse : le sacrifice à faire, ou le départ (section 9.4).
    Eclipse,
    /// Les attaques que ce monstre connaît et celles qu'il peut apprendre
    /// au camp (section 13.10).
    Attaques(Emplacement),
    /// Le sac et le coffre du camp (section 13.3).
    Coffre,
    /// Les ressources du sac et du coffre, depuis le coffre (section 14.3).
    Ressources,
    /// Le suicide du héros, pour recommencer (section 13.9).
    Options,
    /// La forge de Gunther, une fois libéré (section 13.4).
    Forge,
    /// Ce que porte le héros (section 4.3).
    Equipement,
    /// La ligne d'Ansgar, une fois au camp (section 13.8).
    Peche,
    /// Les offres de don et d'échange (section 15.1), avec le joueur choisi
    /// pour une offre, puis le monstre à lui céder.
    Echanges {
        avec: Option<IdJoueur>,
        cede: Option<Emplacement>,
    },
    /// Le classement des duels de tous les joueurs du bot (section 15.4).
    Classement,
    /// Le classement des records de la Tour sans fond (section 9.5).
    RecordsDeLaTour,
    /// Le classement de l'arène, par cote (section 15.6).
    ClassementDeLArene,
    /// Les machines de Gunther, depuis sa forge (section 13.6).
    Machines,
    /// Le choix du monstre à mettre à l'entraînement, depuis les machines.
    Entrainement,
    /// La cour d'entraînement seule, depuis la gestion des monstres.
    Cour,
    /// Le choix du second parent du Rite du sang mêlé, le premier choisi
    /// depuis la gestion des monstres (section 13.11).
    Rite(Emplacement),
    /// Le choix du monstre à mettre à l'entraînement, depuis la cour.
    EntrainementDepuisLaCour,
}

/// Monstres choisis pour une expédition automatique, dans l'ordre du choix
/// (section 12.1), avec le monstre protégé, par sa position parmi eux, et
/// le charme de garde (section 12.5).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Choisis {
    monstres: [Option<Emplacement>; MONSTRES_MAX],
    protege: Option<u8>,
    charme: bool,
}

impl Choisis {
    pub fn liste(&self) -> Vec<Emplacement> {
        self.monstres.iter().flatten().copied().collect()
    }

    pub fn contient(&self, emplacement: Emplacement) -> bool {
        self.monstres.contains(&Some(emplacement))
    }

    pub fn est_complet(&self) -> bool {
        self.monstres.iter().all(Option::is_some)
    }

    /// `None` si le monstre est déjà choisi ou qu'il y en a déjà 3. Le
    /// monstre protégé garde sa position.
    pub fn avec(self, emplacement: Emplacement) -> Option<Self> {
        if self.contient(emplacement) {
            return None;
        }
        let mut choisis = self;
        *choisis.monstres.iter_mut().find(|place| place.is_none())? = Some(emplacement);
        Some(choisis)
    }

    pub fn protection(&self) -> Protection {
        Protection {
            protege: self.protege.map(usize::from),
            charme: self.charme,
        }
    }

    /// Sans monstre protégé, pas de charme.
    pub fn avec_protection(mut self, protege: Option<u8>, charme: bool) -> Self {
        self.protege = protege;
        self.charme = charme && protege.is_some();
        self
    }

    /// `-` pour aucun monstre, sinon les emplacements séparés par des points,
    /// suivis de `~` et de la position du monstre protégé, puis de `c` avec
    /// un charme.
    fn en_texte(&self) -> String {
        let liste: Vec<String> = self.liste().into_iter().map(emplacement_en_texte).collect();
        let mut texte = if liste.is_empty() {
            "-".to_string()
        } else {
            liste.join(".")
        };
        if let Some(protege) = self.protege {
            texte.push_str(&format!("~{protege}"));
            if self.charme {
                texte.push('c');
            }
        }
        texte
    }

    fn lire(texte: &str) -> Option<Self> {
        let (monstres, protection) = match texte.split_once('~') {
            Some((monstres, protection)) => (monstres, Some(protection)),
            None => (texte, None),
        };
        let choisis = if monstres == "-" {
            Self::default()
        } else {
            monstres
                .split('.')
                .try_fold(Self::default(), |choisis, emplacement| {
                    choisis.avec(lire_emplacement(emplacement)?)
                })?
        };
        let Some(protection) = protection else {
            return Some(choisis);
        };
        let (protege, charme) = match protection.strip_suffix('c') {
            Some(protege) => (protege, true),
            None => (protection, false),
        };
        Some(choisis.avec_protection(Some(protege.parse().ok()?), charme))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Formulaire {
    NomDuHeros,
    Surnom(Emplacement),
}

/// Ce que déclenche un composant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Commande {
    Jouer(Choix),
    Afficher(Vue),
    Ouvrir(Formulaire),
    Echanger(ActionDEchange),
    Duel(ActionDeDuel),
    Groupe(ActionDeGroupe),
    /// Remet le message à jour, sans rien changer : en duel, pour voir le
    /// tour que l'autre joueur a fait jouer (section 15.3), et en groupe,
    /// les choix du compagnon (section 15.5).
    Actualiser,
}

/// Un composant tel qu'il figure dans un message. Les menus présents
/// plusieurs fois dans un même message, comme ceux du camp, sont numérotés :
/// Discord veut des identifiants distincts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cle {
    ReprendreLaPartie,
    NouvellePartie,
    Classe(Classe),
    Nommer,
    NomSaisi,
    Oui,
    Non,
    /// Menu des zones.
    Partir,
    Afficher(Vue),
    /// Menu numéroté des espèces capturées du codex : il en faut plusieurs
    /// au-delà de 25 espèces.
    Fiche(u8),
    /// Menu des monstres à gérer.
    Choisir(u8),
    /// Menu numéroté des monstres que le Béhérit accepte (section 9.4).
    Sacrifier(u8),
    PartirVersLEclipse,
    Monter(usize),
    Descendre(usize),
    EnvoyerAuCamp(usize),
    FaireVenir(usize),
    /// Menu de l'équipe, pour ce monstre du camp.
    EchangerAvec(usize),
    Renommer(Emplacement),
    SurnomSaisi(Emplacement),
    Relacher(Emplacement),
    Continuer,
    Rentrer,
    /// Menu des soins et des monstres à soigner, entre deux étapes.
    Soin,
    /// Menu de l'équipe : le monstre choisi passe en tête (Q55).
    EnTete,
    Prendre,
    /// Le bouton d'une annonce (section 9.2).
    Poursuivre,
    Laisser,
    /// Menu des monstres à relâcher pour garder une capture.
    Liberer(u8),
    RelacherLaCapture,
    Attaquer(usize),
    /// L'ordre du héros, en combat (section 4.2).
    Ordre,
    /// Un filet de cette qualité (section 11.2).
    Filet(Filet),
    Fuir,
    /// Menu de l'équipe, en combat.
    Changer,
    /// Menu des soins et des monstres à soigner, en combat.
    SoinEnCombat,
    Remplacer(usize),
    Capturer(usize),
    NePlusCapturer,
    Oublier(usize),
    Renoncer,
    /// Répond à l'évolution proposée après un combat (section 5.4).
    Evoluer,
    NePasEvoluer,
    /// Au camp, le monstre choisi dans la gestion.
    FaireEvoluer(Emplacement),
    /// Menu de monstres numéroté : le monstre choisi s'ajoute à ceux de
    /// l'expédition automatique en préparation.
    Ajouter {
        menu: u8,
        zone: usize,
        choisis: Choisis,
    },
    Expedier {
        zone: ZoneId,
        choisis: Choisis,
    },
    Accueillir(ZoneId),
    Rappeler(ZoneId),
    /// Menu des lots de soins en vente (section 14.4).
    AcheterSoins,
    /// Un lot de filets, chez Malvoise (section 14.2).
    AcheterFilets(Filet, u32),
    /// Menu des capsules en vente.
    AcheterCapsule,
    /// Menu des âmes portées, à donner à Malvoise (section 14.2).
    DonnerAme,
    /// Menu de monstres numéroté, avec leur prix.
    Vendre(u8),
    /// Le monstre choisi dans la gestion.
    VendreCeMonstre(Emplacement),
    /// Une capsule d'habileté sur le monstre choisi (section 5.9).
    ChangerDHabilete(Emplacement),
    /// Menu des boosters à donner au monstre choisi (section 14.3).
    DonnerBooster(Emplacement),
    /// Malvoise rend shiny le monstre choisi (section 13.7).
    RendreShiny(Emplacement),
    /// Malvoise rend le héros shiny, depuis l'écran de son équipement
    /// (sections 4.1 et 13.7).
    RendreLeHerosShiny,
    /// Une capsule d'habileté, chez Malvoise.
    AcheterCapsuleDHabilete,
    /// Menu des espèces que Malvoise vend (section 14.2).
    AcheterMonstre,
    /// Menu des espèces que Malvoise loue.
    LouerMonstre,
    RendreLeMonstreLoue,
    /// Une pierre d'évolution, chez Malvoise (section 5.4).
    AcheterPierre(PierreDEvolution),
    /// Menu des attaques à apprendre par ce monstre, en oubliant celle en
    /// position `oubliee` s'il en connaît déjà 4.
    Apprendre {
        emplacement: Emplacement,
        oubliee: Option<usize>,
    },
    Trier(TriDuCamp),
    /// Menu de ce que contient le coffre du camp.
    SortirDuCoffre,
    /// Menu de ce que contient le sac.
    RangerAuCoffre,
    /// Menus des ressources du coffre et du sac, à part pour que les autres
    /// menus restent sous la limite de 25 options (section 14.3) ; au-delà
    /// de 25 sortes, numérotés comme les suites du codex.
    SortirRessourcesDuCoffre(u8),
    RangerRessourcesAuCoffre(u8),
    /// Menus des ressources que Malvoise rachète, toutes celles d'une sorte.
    VendreRessources(u8),
    ToutRanger,
    /// Quand le sac déborde, menus de ce qu'il contient, un exemplaire à la
    /// fois : les ressources à part, pour rester sous les 25 options
    /// (section 13.3).
    Jeter,
    JeterUneRessource,
    SeSuicider,
    /// Gunther construit le niveau suivant (section 13.4).
    Ameliorer(Amelioration),
    /// Le joueur constate le travail fini de Gunther.
    Constater,
    /// Gunther forge un charme de garde (section 13.5).
    FabriquerCharme,
    /// Menu des lots de filets que Gunther peut forger (section 13.5).
    FabriquerFilets,
    /// Menu des zones explorées où poser les pièges (section 13.6).
    PoserLesPieges,
    ReleverLesPieges,
    /// Menu de monstres numéroté, pour la cour d'entraînement.
    Entrainer(u8),
    /// Le monstre à cette position de la cour.
    Reprendre(usize),
    /// Ansgar tend une ligne, que le joueur relève une fois prête (section
    /// 13.8).
    LancerLaLigne,
    ReleverLaLigne,
    /// Répond au monstre pêché.
    GarderLaPrise,
    RelacherLaPrise,
    /// Menu des objets que Gunther peut forger (section 13.5).
    Fabriquer,
    /// Menu des objets à porter à cette place, avec de quoi retirer celui
    /// qui y est (section 4.3).
    Equiper(PlaceDEquipement),
    /// Menu des autres joueurs, pour leur faire une offre (section 15.1).
    ChoisirJoueur,
    /// Menu de monstres numéroté : le second parent du Rite du sang mêlé,
    /// avec le premier (section 13.11).
    SecondParent {
        menu: u8,
        premier: Emplacement,
    },
    /// Le petit du rite achevé, depuis le camp.
    AccueillirLePetit,
    /// La récompense du premier palier du codex atteint, depuis le codex
    /// (section 5.6).
    ReclamerRecompenseDuCodex,
    /// Menu de monstres numéroté : celui à céder à ce joueur.
    Ceder {
        menu: u8,
        avec: IdJoueur,
    },
    /// Menu numéroté des monstres de ce joueur : celui demandé en échange.
    Demander {
        menu: u8,
        avec: IdJoueur,
        cede: Emplacement,
    },
    /// Le monstre choisi, offert sans contrepartie.
    Donner {
        avec: IdJoueur,
        cede: Emplacement,
    },
    AccepterOffre,
    RefuserOffre,
    RetirerOffre,
    /// Défie ce joueur en duel (section 15.3).
    Defier(IdJoueur),
    /// Défie ce joueur à l'arène (section 15.6).
    DefierALArene(IdJoueur),
    /// La liste d'amis du joueur (section 15.4).
    AjouterAmi(IdJoueur),
    RetirerAmi(IdJoueur),
    AccepterDefi,
    RefuserDefi,
    RetirerDefi,
    Abandonner,
    /// Dans la Fosse aux damnés, le héros jette son arme (section 9.8).
    AbandonnerLaFosse,
    ReclamerLaVictoire,
    /// Après la fin du duel.
    RevenirDuDuel,
    Actualiser,
    /// Invite ce joueur à descendre dans la Tour sans fond (section 15.5).
    Inviter(IdJoueur),
    AccepterInvitation,
    RefuserInvitation,
    RetirerInvitation,
    /// Après 5 minutes sans réponse du compagnon.
    RamenerLeGroupe,
}

/// Valeur de l'option qui retire l'objet porté, dans le menu d'une place :
/// un identifiant d'objet n'a jamais de tiret seul.
pub const RETIRER: &str = "-";

/// La valeur de l'Éclipse dans le menu des expéditions : l'identifiant de
/// ce lieu dans le contenu (section 9.4).
pub const ECLIPSE: &str = "eclipse";

/// La valeur de la Fosse aux damnés dans le menu des expéditions (section
/// 9.8).
pub const FOSSE: &str = "fosse-aux-damnes";

/// La valeur qui forge le Béhérit dans le menu des âmes de l'étal.
pub const BEHERIT: &str = "beherit";

impl Cle {
    /// `valeur` est l'option choisie dans un menu, `texte` le texte saisi
    /// dans un formulaire. `None` si ce qu'il faut manque ou ne se lit pas.
    pub fn commande(&self, valeur: Option<&str>, texte: Option<&str>) -> Option<Commande> {
        let jouer = |choix: Choix| Some(Commande::Jouer(choix));
        let gerer = |action: ActionDeGestion| Some(Commande::Jouer(Choix::Gerer(action)));
        let position = || valeur?.parse::<usize>().ok();
        match self {
            Cle::ReprendreLaPartie => jouer(Choix::ReprendreLaPartie),
            Cle::NouvellePartie => jouer(Choix::NouvellePartie),
            Cle::Classe(classe) => jouer(Choix::Classe(*classe)),
            Cle::Nommer => Some(Commande::Ouvrir(Formulaire::NomDuHeros)),
            Cle::NomSaisi => jouer(Choix::NomDuHeros(texte?.to_string())),
            Cle::Oui => jouer(Choix::Oui),
            Cle::Non => jouer(Choix::Non),
            // L'Éclipse s'ouvre d'abord sur son écran : le sacrifice ou le
            // départ (section 9.4).
            Cle::Partir if valeur? == ECLIPSE => Some(Commande::Afficher(Vue::Eclipse)),
            Cle::Partir if valeur? == FOSSE => jouer(Choix::EntrerDansLaFosse),
            Cle::Partir => jouer(Choix::PartirEnExpedition(ZoneId::new(valeur?))),
            Cle::PartirVersLEclipse => jouer(Choix::PartirEnExpedition(ZoneId::new(ECLIPSE))),
            Cle::Sacrifier(_) => jouer(Choix::Sacrifier(lire_emplacement(valeur?)?)),
            Cle::Afficher(vue) => Some(Commande::Afficher(*vue)),
            Cle::Fiche(_) => Some(Commande::Afficher(Vue::Codex(Some(position()?)))),
            Cle::Choisir(_) => Some(Commande::Afficher(Vue::Gestion(Some(lire_emplacement(
                valeur?,
            )?)))),
            Cle::Monter(index) => gerer(ActionDeGestion::ChangerLOrdre {
                de: *index,
                vers: index.checked_sub(1)?,
            }),
            Cle::Descendre(index) => gerer(ActionDeGestion::ChangerLOrdre {
                de: *index,
                vers: index + 1,
            }),
            Cle::EnvoyerAuCamp(index) => gerer(ActionDeGestion::EnvoyerAuCamp(*index)),
            Cle::FaireVenir(index) => gerer(ActionDeGestion::FaireVenir(*index)),
            Cle::EchangerAvec(camp) => gerer(ActionDeGestion::Echanger {
                equipe: position()?,
                camp: *camp,
            }),
            Cle::Renommer(emplacement) => Some(Commande::Ouvrir(Formulaire::Surnom(*emplacement))),
            Cle::SurnomSaisi(emplacement) => gerer(ActionDeGestion::Renommer {
                emplacement: *emplacement,
                surnom: texte?.to_string(),
            }),
            Cle::Relacher(emplacement) => gerer(ActionDeGestion::Relacher(*emplacement)),
            Cle::Continuer => jouer(Choix::ContinuerLExpedition),
            Cle::Rentrer => jouer(Choix::RentrerAuCamp),
            Cle::Soin => {
                let (soin, index) = lire_soin_et_position(valeur?)?;
                jouer(Choix::UtiliserSoin(soin, index))
            }
            Cle::EnTete => gerer(ActionDeGestion::ChangerLOrdre {
                de: position()?,
                vers: 0,
            }),
            Cle::Prendre => jouer(Choix::PrendreLeMonstreRare),
            Cle::Poursuivre => jouer(Choix::Poursuivre),
            Cle::Laisser => jouer(Choix::LaisserLeMonstreRare),
            Cle::Liberer(_) => jouer(Choix::RelacherPourLaCapture(lire_emplacement(valeur?)?)),
            Cle::RelacherLaCapture => jouer(Choix::RelacherLaCapture),
            Cle::Attaquer(index) => jouer(Choix::Combat(Action::Attaquer(*index))),
            Cle::Ordre => jouer(Choix::DonnerOrdre),
            Cle::Filet(filet) => jouer(Choix::Combat(Action::LancerFilet(*filet))),
            Cle::Fuir => jouer(Choix::Combat(Action::Fuir)),
            Cle::Changer => jouer(Choix::Combat(Action::Changer(position()?))),
            Cle::SoinEnCombat => {
                let (soin, index) = lire_soin_et_position(valeur?)?;
                jouer(Choix::Combat(Action::UtiliserSoin(soin, index)))
            }
            Cle::Remplacer(index) => jouer(Choix::Remplacer(*index)),
            Cle::Capturer(index) => jouer(Choix::Capturer(*index)),
            Cle::NePlusCapturer => jouer(Choix::NePlusCapturer),
            Cle::Oublier(index) => jouer(Choix::OublierAttaque(*index)),
            Cle::Renoncer => jouer(Choix::RenoncerALAttaque),
            Cle::Evoluer => jouer(Choix::Evoluer),
            Cle::NePasEvoluer => jouer(Choix::NePasEvoluer),
            Cle::FaireEvoluer(emplacement) => jouer(Choix::FaireEvoluer(*emplacement)),
            Cle::Ajouter { zone, choisis, .. } => Some(Commande::Afficher(Vue::Preparation {
                zone: *zone,
                choisis: choisis.avec(lire_emplacement(valeur?)?)?,
            })),
            Cle::Expedier { zone, choisis } => jouer(Choix::EnvoyerEnExpeditionAutomatique {
                zone: zone.clone(),
                monstres: choisis.liste(),
                protection: choisis.protection(),
            }),
            Cle::Accueillir(zone) => jouer(Choix::AccueillirExpeditionAutomatique(zone.clone())),
            Cle::Rappeler(zone) => jouer(Choix::RappelerExpeditionAutomatique(zone.clone())),
            Cle::AcheterSoins => {
                let (soin, nombre) = lire_lot_de_soins(valeur?)?;
                jouer(Choix::AcheterSoins(soin, nombre))
            }
            Cle::AcheterFilets(filet, nombre) => jouer(Choix::AcheterFilets(*filet, *nombre)),
            Cle::AcheterCapsule => jouer(Choix::AcheterCapsule(AttaqueId::new(valeur?))),
            // Les âmes des Apôtres se donnent ensemble, depuis le même menu
            // que celles des marchands (section 9.4).
            Cle::DonnerAme if valeur? == BEHERIT => jouer(Choix::ForgerLeBeherit),
            Cle::DonnerAme => jouer(Choix::DonnerAme(MarchandId::new(valeur?))),
            Cle::Vendre(_) => jouer(Choix::Vendre(lire_emplacement(valeur?)?)),
            Cle::VendreCeMonstre(emplacement) => jouer(Choix::Vendre(*emplacement)),
            Cle::ChangerDHabilete(emplacement) => jouer(Choix::ChangerDHabilete(*emplacement)),
            Cle::DonnerBooster(emplacement) => jouer(Choix::DonnerBooster {
                emplacement: *emplacement,
                booster: lire_booster(valeur?)?,
            }),
            Cle::RendreShiny(emplacement) => jouer(Choix::RendreShiny(*emplacement)),
            Cle::RendreLeHerosShiny => jouer(Choix::RendreLeHerosShiny),
            Cle::AcheterCapsuleDHabilete => jouer(Choix::AcheterCapsulesDHabilete(1)),
            Cle::AcheterMonstre => jouer(Choix::AcheterMonstre(EspeceId::new(valeur?))),
            Cle::LouerMonstre => jouer(Choix::LouerMonstre(EspeceId::new(valeur?))),
            Cle::RendreLeMonstreLoue => jouer(Choix::RendreLeMonstreLoue),
            Cle::AcheterPierre(pierre) => jouer(Choix::AcheterPierres(*pierre, 1)),
            Cle::Apprendre {
                emplacement,
                oubliee,
            } => jouer(Choix::ApprendreAttaque {
                emplacement: *emplacement,
                attaque: AttaqueId::new(valeur?),
                oubliee: *oubliee,
            }),
            Cle::Trier(tri) => gerer(ActionDeGestion::TrierLeCamp(*tri)),
            Cle::SortirDuCoffre => jouer(Choix::SortirDuCoffre(lire_objet(valeur?)?)),
            Cle::RangerAuCoffre => jouer(Choix::RangerAuCoffre(lire_objet(valeur?)?)),
            Cle::SortirRessourcesDuCoffre(_) => jouer(Choix::SortirDuCoffre(lire_objet(valeur?)?)),
            Cle::RangerRessourcesAuCoffre(_) => jouer(Choix::RangerAuCoffre(lire_objet(valeur?)?)),
            Cle::VendreRessources(_) => jouer(Choix::VendreRessources(RessourceId::new(valeur?))),
            Cle::ToutRanger => jouer(Choix::ToutRangerAuCoffre),
            Cle::Jeter | Cle::JeterUneRessource => jouer(Choix::Jeter(lire_objet(valeur?)?)),
            Cle::SeSuicider => jouer(Choix::SeSuicider),
            Cle::Ameliorer(amelioration) => jouer(Choix::LancerAmelioration(*amelioration)),
            Cle::Constater => jouer(Choix::ConstaterTravail),
            Cle::FabriquerCharme => jouer(Choix::FabriquerCharme),
            Cle::FabriquerFilets => jouer(Choix::FabriquerFilets(Filet::depuis_id(valeur?)?)),
            Cle::PoserLesPieges => jouer(Choix::PoserLesPieges(ZoneId::new(valeur?))),
            Cle::ReleverLesPieges => jouer(Choix::ReleverLesPieges),
            Cle::Entrainer(_) => jouer(Choix::MettreALEntrainement(lire_emplacement(valeur?)?)),
            Cle::Reprendre(position) => jouer(Choix::ReprendreDeLEntrainement(*position)),
            Cle::LancerLaLigne => jouer(Choix::LancerLaLigne),
            Cle::ReleverLaLigne => jouer(Choix::ReleverLaLigne),
            Cle::GarderLaPrise => jouer(Choix::GarderLaPrise),
            Cle::RelacherLaPrise => jouer(Choix::RelacherLaPrise),
            Cle::Fabriquer => jouer(Choix::LancerFabrication(ObjetDEquipementId::new(valeur?))),
            Cle::Equiper(place) => match valeur? {
                RETIRER => jouer(Choix::RetirerEquipement(*place)),
                objet => jouer(Choix::Equiper {
                    place: *place,
                    objet: ObjetDEquipementId::new(objet),
                }),
            },
            Cle::ChoisirJoueur => Some(Commande::Afficher(Vue::Echanges {
                avec: Some(valeur?.parse().ok()?),
                cede: None,
            })),
            Cle::SecondParent { premier, .. } => jouer(Choix::LancerLeRite {
                premier: *premier,
                second: lire_emplacement(valeur?)?,
            }),
            Cle::AccueillirLePetit => jouer(Choix::AccueillirLePetit),
            Cle::ReclamerRecompenseDuCodex => jouer(Choix::ReclamerRecompenseDuCodex),
            Cle::Ceder { avec, .. } => Some(Commande::Afficher(Vue::Echanges {
                avec: Some(*avec),
                cede: Some(lire_emplacement(valeur?)?),
            })),
            Cle::Demander { avec, cede, .. } => Some(Commande::Echanger(ActionDEchange::Offrir {
                a: *avec,
                cede: *cede,
                demande: Some(lire_emplacement(valeur?)?),
            })),
            Cle::Donner { avec, cede } => Some(Commande::Echanger(ActionDEchange::Offrir {
                a: *avec,
                cede: *cede,
                demande: None,
            })),
            Cle::AccepterOffre => Some(Commande::Echanger(ActionDEchange::Accepter)),
            Cle::RefuserOffre => Some(Commande::Echanger(ActionDEchange::Refuser)),
            Cle::RetirerOffre => Some(Commande::Echanger(ActionDEchange::Retirer)),
            Cle::AjouterAmi(autre) => Some(Commande::Echanger(ActionDEchange::AjouterAmi(*autre))),
            Cle::RetirerAmi(autre) => Some(Commande::Echanger(ActionDEchange::RetirerAmi(*autre))),
            Cle::Defier(a) => Some(Commande::Duel(ActionDeDuel::Defier(*a))),
            Cle::DefierALArene(a) => Some(Commande::Duel(ActionDeDuel::DefierALArene(*a))),
            Cle::AccepterDefi => Some(Commande::Duel(ActionDeDuel::AccepterDefi)),
            Cle::RefuserDefi => Some(Commande::Duel(ActionDeDuel::RefuserDefi)),
            Cle::RetirerDefi => Some(Commande::Duel(ActionDeDuel::RetirerDefi)),
            Cle::Abandonner => Some(Commande::Duel(ActionDeDuel::Abandonner)),
            Cle::AbandonnerLaFosse => jouer(Choix::Abandonner),
            Cle::ReclamerLaVictoire => Some(Commande::Duel(ActionDeDuel::ReclamerLaVictoire)),
            Cle::RevenirDuDuel => Some(Commande::Duel(ActionDeDuel::RevenirAuCamp)),
            Cle::Actualiser => Some(Commande::Actualiser),
            Cle::Inviter(a) => Some(Commande::Groupe(ActionDeGroupe::Inviter(*a))),
            Cle::AccepterInvitation => Some(Commande::Groupe(ActionDeGroupe::AccepterInvitation)),
            Cle::RefuserInvitation => Some(Commande::Groupe(ActionDeGroupe::RefuserInvitation)),
            Cle::RetirerInvitation => Some(Commande::Groupe(ActionDeGroupe::RetirerInvitation)),
            Cle::RamenerLeGroupe => Some(Commande::Groupe(ActionDeGroupe::RamenerLeGroupe)),
        }
    }

    fn en_texte(&self) -> String {
        match self {
            Cle::ReprendreLaPartie => "reprendre".to_string(),
            Cle::NouvellePartie => "nouvelle-partie".to_string(),
            Cle::Classe(Classe::Guerrier) => "classe:guerrier".to_string(),
            Cle::Classe(Classe::Mage) => "classe:mage".to_string(),
            Cle::Classe(Classe::Templier) => "classe:templier".to_string(),
            Cle::Classe(Classe::Assassin) => "classe:assassin".to_string(),
            Cle::Nommer => "nommer".to_string(),
            Cle::NomSaisi => "nom".to_string(),
            Cle::Oui => "oui".to_string(),
            Cle::Non => "non".to_string(),
            Cle::Partir => "partir".to_string(),
            Cle::Afficher(Vue::Camp) => "vue:camp".to_string(),
            Cle::Afficher(Vue::Codex(None)) => "vue:codex".to_string(),
            Cle::Afficher(Vue::Codex(Some(index))) => format!("vue:codex:{index}"),
            Cle::Afficher(Vue::PageDuCodex(page)) => format!("vue:page-du-codex:{page}"),
            Cle::Afficher(Vue::Gestion(None)) => "vue:gestion".to_string(),
            Cle::Afficher(Vue::Gestion(Some(emplacement))) => {
                format!("vue:gestion:{}", emplacement_en_texte(*emplacement))
            }
            Cle::Afficher(Vue::Expeditions) => "vue:expeditions".to_string(),
            Cle::Afficher(Vue::Preparation { zone, choisis }) => {
                format!("vue:preparation:{zone}:{}", choisis.en_texte())
            }
            Cle::Afficher(Vue::Marchand) => "vue:marchand".to_string(),
            Cle::Afficher(Vue::Vente) => "vue:vente".to_string(),
            Cle::Afficher(Vue::Achats) => "vue:achats".to_string(),
            Cle::Afficher(Vue::Eclipse) => "vue:eclipse".to_string(),
            Cle::Afficher(Vue::Attaques(emplacement)) => {
                format!("vue:attaques:{}", emplacement_en_texte(*emplacement))
            }
            Cle::Afficher(Vue::Rite(premier)) => {
                format!("vue:rite:{}", emplacement_en_texte(*premier))
            }
            Cle::Afficher(Vue::Coffre) => "vue:coffre".to_string(),
            Cle::Afficher(Vue::Ressources) => "vue:ressources".to_string(),
            Cle::Afficher(Vue::Options) => "vue:options".to_string(),
            Cle::Afficher(Vue::Forge) => "vue:forge".to_string(),
            Cle::Afficher(Vue::Equipement) => "vue:equipement".to_string(),
            Cle::Afficher(Vue::Peche) => "vue:peche".to_string(),
            Cle::Afficher(Vue::Echanges { avec: None, .. }) => "vue:echanges".to_string(),
            Cle::Afficher(Vue::Classement) => "vue:classement".to_string(),
            Cle::Afficher(Vue::RecordsDeLaTour) => "vue:records-de-la-tour".to_string(),
            Cle::Afficher(Vue::ClassementDeLArene) => "vue:classement-arene".to_string(),
            Cle::AjouterAmi(autre) => format!("ajouter-ami:{autre}"),
            Cle::RetirerAmi(autre) => format!("retirer-ami:{autre}"),
            Cle::Afficher(Vue::Echanges {
                avec: Some(avec),
                cede: None,
            }) => format!("vue:echanges:{avec}"),
            Cle::Afficher(Vue::Echanges {
                avec: Some(avec),
                cede: Some(cede),
            }) => format!("vue:echanges:{avec}:{}", emplacement_en_texte(*cede)),
            Cle::Fiche(numero) => format!("fiche:{numero}"),
            Cle::Choisir(numero) => format!("choisir:{numero}"),
            Cle::Sacrifier(numero) => format!("sacrifier:{numero}"),
            Cle::PartirVersLEclipse => "partir-vers-l-eclipse".to_string(),
            Cle::Monter(index) => format!("monter:{index}"),
            Cle::Descendre(index) => format!("descendre:{index}"),
            Cle::EnvoyerAuCamp(index) => format!("envoyer:{index}"),
            Cle::FaireVenir(index) => format!("faire-venir:{index}"),
            Cle::EchangerAvec(index) => format!("echanger:{index}"),
            Cle::Renommer(emplacement) => {
                format!("renommer:{}", emplacement_en_texte(*emplacement))
            }
            Cle::SurnomSaisi(emplacement) => {
                format!("surnom:{}", emplacement_en_texte(*emplacement))
            }
            Cle::Relacher(emplacement) => {
                format!("relacher:{}", emplacement_en_texte(*emplacement))
            }
            Cle::Continuer => "continuer".to_string(),
            Cle::Rentrer => "rentrer".to_string(),
            Cle::Soin => "soin".to_string(),
            Cle::EnTete => "en-tete".to_string(),
            Cle::Prendre => "prendre".to_string(),
            Cle::Poursuivre => "poursuivre".to_string(),
            Cle::Laisser => "laisser".to_string(),
            Cle::Liberer(numero) => format!("liberer:{numero}"),
            Cle::RelacherLaCapture => "relacher-capture".to_string(),
            Cle::Attaquer(index) => format!("attaque:{index}"),
            Cle::Ordre => "ordre".to_string(),
            Cle::Filet(filet) => format!("filet:{}", filet.id()),
            Cle::Fuir => "fuir".to_string(),
            Cle::Changer => "changer".to_string(),
            Cle::SoinEnCombat => "soin-combat".to_string(),
            Cle::Remplacer(index) => format!("remplacer:{index}"),
            Cle::Capturer(index) => format!("capturer:{index}"),
            Cle::NePlusCapturer => "ne-plus-capturer".to_string(),
            Cle::Oublier(index) => format!("oublier:{index}"),
            Cle::Renoncer => "renoncer".to_string(),
            Cle::Evoluer => "evoluer".to_string(),
            Cle::NePasEvoluer => "ne-pas-evoluer".to_string(),
            Cle::FaireEvoluer(emplacement) => {
                format!("faire-evoluer:{}", emplacement_en_texte(*emplacement))
            }
            Cle::Ajouter {
                menu,
                zone,
                choisis,
            } => format!("ajouter:{menu}:{zone}:{}", choisis.en_texte()),
            Cle::Expedier { zone, choisis } => {
                format!("expedier:{}:{}", zone.as_str(), choisis.en_texte())
            }
            Cle::Accueillir(zone) => format!("accueillir:{}", zone.as_str()),
            Cle::Rappeler(zone) => format!("rappeler:{}", zone.as_str()),
            Cle::AcheterSoins => "acheter-soins".to_string(),
            Cle::AcheterFilets(filet, nombre) => {
                format!("acheter-filets:{}:{nombre}", filet.id())
            }
            Cle::AcheterCapsule => "acheter-capsule".to_string(),
            Cle::DonnerAme => "donner-ame".to_string(),
            Cle::Vendre(numero) => format!("vendre:{numero}"),
            Cle::ChangerDHabilete(emplacement) => {
                format!("changer-habilete:{}", emplacement_en_texte(*emplacement))
            }
            Cle::DonnerBooster(emplacement) => {
                format!("donner-booster:{}", emplacement_en_texte(*emplacement))
            }
            Cle::RendreShiny(emplacement) => {
                format!("rendre-shiny:{}", emplacement_en_texte(*emplacement))
            }
            Cle::AcheterCapsuleDHabilete => "acheter-capsule-d-habilete".to_string(),
            Cle::AcheterMonstre => "acheter-monstre".to_string(),
            Cle::LouerMonstre => "louer-monstre".to_string(),
            Cle::RendreLeMonstreLoue => "rendre-le-monstre-loue".to_string(),
            Cle::RendreLeHerosShiny => "rendre-le-heros-shiny".to_string(),
            Cle::AcheterPierre(pierre) => format!("acheter-pierre:{}", pierre_en_texte(*pierre)),
            Cle::VendreCeMonstre(emplacement) => {
                format!("vendre-monstre:{}", emplacement_en_texte(*emplacement))
            }
            // `-` quand il n'y a rien à oublier.
            Cle::Apprendre {
                emplacement,
                oubliee,
            } => {
                let oubliee = oubliee.map_or("-".to_string(), |position| position.to_string());
                format!("apprendre:{}:{oubliee}", emplacement_en_texte(*emplacement))
            }
            Cle::Trier(TriDuCamp::Nom) => "trier:nom".to_string(),
            Cle::Trier(TriDuCamp::Niveau) => "trier:niveau".to_string(),
            Cle::Trier(TriDuCamp::Type) => "trier:type".to_string(),
            Cle::SortirDuCoffre => "sortir-du-coffre".to_string(),
            Cle::RangerAuCoffre => "ranger-au-coffre".to_string(),
            Cle::SortirRessourcesDuCoffre(numero) => format!("sortir-ressources:{numero}"),
            Cle::RangerRessourcesAuCoffre(numero) => format!("ranger-ressources:{numero}"),
            Cle::VendreRessources(numero) => format!("vendre-ressources:{numero}"),
            Cle::ToutRanger => "tout-ranger".to_string(),
            Cle::Jeter => "jeter".to_string(),
            Cle::JeterUneRessource => "jeter-ressource".to_string(),
            Cle::SeSuicider => "se-suicider".to_string(),
            Cle::Ameliorer(amelioration) => {
                format!("ameliorer:{}", amelioration_en_texte(*amelioration))
            }
            Cle::Constater => "constater".to_string(),
            Cle::Afficher(Vue::Machines) => "vue:machines".to_string(),
            Cle::Afficher(Vue::Entrainement) => "vue:entrainement".to_string(),
            Cle::Afficher(Vue::Cour) => "vue:cour".to_string(),
            Cle::Afficher(Vue::EntrainementDepuisLaCour) => "vue:entrainement-cour".to_string(),
            Cle::FabriquerCharme => "fabriquer-charme".to_string(),
            Cle::FabriquerFilets => "fabriquer-filets".to_string(),
            Cle::PoserLesPieges => "poser-pieges".to_string(),
            Cle::ReleverLesPieges => "relever-pieges".to_string(),
            Cle::Entrainer(numero) => format!("entrainer:{numero}"),
            Cle::Reprendre(position) => format!("reprendre:{position}"),
            Cle::LancerLaLigne => "lancer-ligne".to_string(),
            Cle::ReleverLaLigne => "relever-ligne".to_string(),
            Cle::GarderLaPrise => "garder-prise".to_string(),
            Cle::RelacherLaPrise => "relacher-prise".to_string(),
            Cle::Fabriquer => "fabriquer".to_string(),
            Cle::Equiper(place) => format!("equiper:{}", place_en_texte(*place)),
            Cle::ChoisirJoueur => "choisir-joueur".to_string(),
            Cle::SecondParent { menu, premier } => {
                format!("second-parent:{menu}:{}", emplacement_en_texte(*premier))
            }
            Cle::AccueillirLePetit => "accueillir-le-petit".to_string(),
            Cle::ReclamerRecompenseDuCodex => "reclamer-codex".to_string(),
            Cle::Ceder { menu, avec } => format!("ceder:{menu}:{avec}"),
            Cle::Demander { menu, avec, cede } => {
                format!("demander:{menu}:{avec}:{}", emplacement_en_texte(*cede))
            }
            Cle::Donner { avec, cede } => format!("donner:{avec}:{}", emplacement_en_texte(*cede)),
            Cle::AccepterOffre => "accepter-offre".to_string(),
            Cle::RefuserOffre => "refuser-offre".to_string(),
            Cle::RetirerOffre => "retirer-offre".to_string(),
            Cle::Defier(a) => format!("defier:{a}"),
            Cle::DefierALArene(a) => format!("defier-arene:{a}"),
            Cle::AccepterDefi => "accepter-defi".to_string(),
            Cle::RefuserDefi => "refuser-defi".to_string(),
            Cle::RetirerDefi => "retirer-defi".to_string(),
            Cle::Abandonner => "abandonner".to_string(),
            Cle::AbandonnerLaFosse => "abandonner-la-fosse".to_string(),
            Cle::ReclamerLaVictoire => "reclamer-victoire".to_string(),
            Cle::RevenirDuDuel => "revenir-du-duel".to_string(),
            Cle::Actualiser => "actualiser".to_string(),
            Cle::Inviter(a) => format!("inviter:{a}"),
            Cle::AccepterInvitation => "accepter-invitation".to_string(),
            Cle::RefuserInvitation => "refuser-invitation".to_string(),
            Cle::RetirerInvitation => "retirer-invitation".to_string(),
            Cle::RamenerLeGroupe => "ramener-le-groupe".to_string(),
        }
    }

    fn depuis_texte(texte: &str) -> Option<Self> {
        let cle = match texte {
            "reprendre" => Cle::ReprendreLaPartie,
            "nouvelle-partie" => Cle::NouvellePartie,
            "classe:guerrier" => Cle::Classe(Classe::Guerrier),
            "classe:mage" => Cle::Classe(Classe::Mage),
            "classe:templier" => Cle::Classe(Classe::Templier),
            "classe:assassin" => Cle::Classe(Classe::Assassin),
            "nommer" => Cle::Nommer,
            "nom" => Cle::NomSaisi,
            "oui" => Cle::Oui,
            "non" => Cle::Non,
            "partir" => Cle::Partir,
            "vue:camp" => Cle::Afficher(Vue::Camp),
            // La liste des monstres a disparu du camp : un bouton d'un ancien
            // message ouvre la gestion, qui la montre.
            "vue:monstres" => Cle::Afficher(Vue::Gestion(None)),
            "trier:nom" => Cle::Trier(TriDuCamp::Nom),
            "trier:niveau" => Cle::Trier(TriDuCamp::Niveau),
            "trier:type" => Cle::Trier(TriDuCamp::Type),
            "vue:codex" => Cle::Afficher(Vue::Codex(None)),
            "vue:gestion" => Cle::Afficher(Vue::Gestion(None)),
            "vue:expeditions" => Cle::Afficher(Vue::Expeditions),
            "vue:marchand" => Cle::Afficher(Vue::Marchand),
            "vue:vente" => Cle::Afficher(Vue::Vente),
            "vue:achats" => Cle::Afficher(Vue::Achats),
            "vue:eclipse" => Cle::Afficher(Vue::Eclipse),
            "partir-vers-l-eclipse" => Cle::PartirVersLEclipse,
            "vue:coffre" => Cle::Afficher(Vue::Coffre),
            "vue:ressources" => Cle::Afficher(Vue::Ressources),
            "sortir-du-coffre" => Cle::SortirDuCoffre,
            "ranger-au-coffre" => Cle::RangerAuCoffre,
            // Ceux des messages affichés avant l'étape 163, sans numéro.
            "sortir-ressources" => Cle::SortirRessourcesDuCoffre(0),
            "ranger-ressources" => Cle::RangerRessourcesAuCoffre(0),
            "vendre-ressources" => Cle::VendreRessources(0),
            "tout-ranger" => Cle::ToutRanger,
            "jeter" => Cle::Jeter,
            "jeter-ressource" => Cle::JeterUneRessource,
            "vue:options" => Cle::Afficher(Vue::Options),
            "vue:forge" => Cle::Afficher(Vue::Forge),
            "constater" => Cle::Constater,
            "vue:machines" => Cle::Afficher(Vue::Machines),
            "vue:entrainement" => Cle::Afficher(Vue::Entrainement),
            "vue:cour" => Cle::Afficher(Vue::Cour),
            "vue:entrainement-cour" => Cle::Afficher(Vue::EntrainementDepuisLaCour),
            "fabriquer-charme" => Cle::FabriquerCharme,
            "fabriquer-filets" => Cle::FabriquerFilets,
            "poser-pieges" => Cle::PoserLesPieges,
            "relever-pieges" => Cle::ReleverLesPieges,
            "vue:equipement" => Cle::Afficher(Vue::Equipement),
            "vue:peche" => Cle::Afficher(Vue::Peche),
            "lancer-ligne" => Cle::LancerLaLigne,
            "relever-ligne" => Cle::ReleverLaLigne,
            "garder-prise" => Cle::GarderLaPrise,
            "relacher-prise" => Cle::RelacherLaPrise,
            "vue:classement" => Cle::Afficher(Vue::Classement),
            "vue:records-de-la-tour" => Cle::Afficher(Vue::RecordsDeLaTour),
            "vue:classement-arene" => Cle::Afficher(Vue::ClassementDeLArene),
            "vue:echanges" => Cle::Afficher(Vue::Echanges {
                avec: None,
                cede: None,
            }),
            "choisir-joueur" => Cle::ChoisirJoueur,
            "accepter-offre" => Cle::AccepterOffre,
            "refuser-offre" => Cle::RefuserOffre,
            "retirer-offre" => Cle::RetirerOffre,
            "accepter-defi" => Cle::AccepterDefi,
            "refuser-defi" => Cle::RefuserDefi,
            "retirer-defi" => Cle::RetirerDefi,
            "abandonner" => Cle::Abandonner,
            "abandonner-la-fosse" => Cle::AbandonnerLaFosse,
            "reclamer-victoire" => Cle::ReclamerLaVictoire,
            "revenir-du-duel" => Cle::RevenirDuDuel,
            "actualiser" => Cle::Actualiser,
            "accepter-invitation" => Cle::AccepterInvitation,
            "refuser-invitation" => Cle::RefuserInvitation,
            "retirer-invitation" => Cle::RetirerInvitation,
            "ramener-le-groupe" => Cle::RamenerLeGroupe,
            "accueillir-le-petit" => Cle::AccueillirLePetit,
            "reclamer-codex" => Cle::ReclamerRecompenseDuCodex,
            "fabriquer" => Cle::Fabriquer,
            "se-suicider" => Cle::SeSuicider,
            "acheter-soins" => Cle::AcheterSoins,
            "acheter-capsule" => Cle::AcheterCapsule,
            "acheter-capsule-d-habilete" => Cle::AcheterCapsuleDHabilete,
            "acheter-monstre" => Cle::AcheterMonstre,
            "louer-monstre" => Cle::LouerMonstre,
            "rendre-le-monstre-loue" => Cle::RendreLeMonstreLoue,
            "rendre-le-heros-shiny" => Cle::RendreLeHerosShiny,
            "donner-ame" => Cle::DonnerAme,
            // Le menu unique d'avant l'étape 38 (section 5.4).
            "fiche" => Cle::Fiche(0),
            "evoluer" => Cle::Evoluer,
            "ne-pas-evoluer" => Cle::NePasEvoluer,
            "continuer" => Cle::Continuer,
            "rentrer" => Cle::Rentrer,
            "soin" => Cle::Soin,
            "en-tete" => Cle::EnTete,
            "prendre" => Cle::Prendre,
            "poursuivre" => Cle::Poursuivre,
            "laisser" => Cle::Laisser,
            "relacher-capture" => Cle::RelacherLaCapture,
            "ordre" => Cle::Ordre,
            "fuir" => Cle::Fuir,
            "changer" => Cle::Changer,
            "soin-combat" => Cle::SoinEnCombat,
            "ne-plus-capturer" => Cle::NePlusCapturer,
            "renoncer" => Cle::Renoncer,
            autre => {
                let (genre, parametre) = autre.split_once(':')?;
                let position = || parametre.parse::<usize>().ok();
                let numero = || parametre.parse::<u8>().ok();
                let emplacement = || lire_emplacement(parametre);
                match genre {
                    "vue" => {
                        let (vue, parametre) = parametre.split_once(':')?;
                        match vue {
                            "codex" => Cle::Afficher(Vue::Codex(Some(parametre.parse().ok()?))),
                            "page-du-codex" => {
                                Cle::Afficher(Vue::PageDuCodex(parametre.parse().ok()?))
                            }
                            "gestion" => {
                                Cle::Afficher(Vue::Gestion(Some(lire_emplacement(parametre)?)))
                            }
                            "attaques" => {
                                Cle::Afficher(Vue::Attaques(lire_emplacement(parametre)?))
                            }
                            "rite" => Cle::Afficher(Vue::Rite(lire_emplacement(parametre)?)),
                            "preparation" => {
                                let (zone, choisis) = parametre.split_once(':')?;
                                Cle::Afficher(Vue::Preparation {
                                    zone: zone.parse().ok()?,
                                    choisis: Choisis::lire(choisis)?,
                                })
                            }
                            "echanges" => {
                                let (avec, cede) = match parametre.split_once(':') {
                                    Some((avec, cede)) => (avec, Some(lire_emplacement(cede)?)),
                                    None => (parametre, None),
                                };
                                Cle::Afficher(Vue::Echanges {
                                    avec: Some(avec.parse().ok()?),
                                    cede,
                                })
                            }
                            _ => return None,
                        }
                    }
                    "ajouter" => {
                        let mut morceaux = parametre.splitn(3, ':');
                        Cle::Ajouter {
                            menu: morceaux.next()?.parse().ok()?,
                            zone: morceaux.next()?.parse().ok()?,
                            choisis: Choisis::lire(morceaux.next()?)?,
                        }
                    }
                    "ceder" => {
                        let (menu, avec) = parametre.split_once(':')?;
                        Cle::Ceder {
                            menu: menu.parse().ok()?,
                            avec: avec.parse().ok()?,
                        }
                    }
                    "second-parent" => {
                        let (menu, premier) = parametre.split_once(':')?;
                        Cle::SecondParent {
                            menu: menu.parse().ok()?,
                            premier: lire_emplacement(premier)?,
                        }
                    }
                    "demander" => {
                        let mut morceaux = parametre.splitn(3, ':');
                        Cle::Demander {
                            menu: morceaux.next()?.parse().ok()?,
                            avec: morceaux.next()?.parse().ok()?,
                            cede: lire_emplacement(morceaux.next()?)?,
                        }
                    }
                    "defier" => Cle::Defier(parametre.parse().ok()?),
                    "defier-arene" => Cle::DefierALArene(parametre.parse().ok()?),
                    "inviter" => Cle::Inviter(parametre.parse().ok()?),
                    "filet" => Cle::Filet(Filet::depuis_id(parametre)?),
                    "ajouter-ami" => Cle::AjouterAmi(parametre.parse().ok()?),
                    "retirer-ami" => Cle::RetirerAmi(parametre.parse().ok()?),
                    "donner" => {
                        let (avec, cede) = parametre.split_once(':')?;
                        Cle::Donner {
                            avec: avec.parse().ok()?,
                            cede: lire_emplacement(cede)?,
                        }
                    }
                    "expedier" => {
                        let (zone, choisis) = parametre.split_once(':')?;
                        Cle::Expedier {
                            zone: ZoneId::new(zone),
                            choisis: Choisis::lire(choisis)?,
                        }
                    }
                    "accueillir" => Cle::Accueillir(ZoneId::new(parametre)),
                    "rappeler" => Cle::Rappeler(ZoneId::new(parametre)),
                    "acheter-filets" => {
                        let (filet, nombre) = parametre.split_once(':')?;
                        Cle::AcheterFilets(Filet::depuis_id(filet)?, nombre.parse().ok()?)
                    }
                    "vendre" => Cle::Vendre(numero()?),
                    "entrainer" => Cle::Entrainer(numero()?),
                    "reprendre" => Cle::Reprendre(position()?),
                    "vendre-monstre" => Cle::VendreCeMonstre(emplacement()?),
                    "changer-habilete" => Cle::ChangerDHabilete(emplacement()?),
                    "donner-booster" => Cle::DonnerBooster(emplacement()?),
                    "rendre-shiny" => Cle::RendreShiny(emplacement()?),
                    "faire-evoluer" => Cle::FaireEvoluer(emplacement()?),
                    "acheter-pierre" => Cle::AcheterPierre(lire_pierre(parametre)?),
                    "ameliorer" => Cle::Ameliorer(lire_amelioration(parametre)?),
                    "equiper" => Cle::Equiper(lire_place(parametre)?),
                    "fiche" => Cle::Fiche(numero()?),
                    "sortir-ressources" => Cle::SortirRessourcesDuCoffre(numero()?),
                    "ranger-ressources" => Cle::RangerRessourcesAuCoffre(numero()?),
                    "vendre-ressources" => Cle::VendreRessources(numero()?),
                    "apprendre" => {
                        let (emplacement, oubliee) = parametre.split_once(':')?;
                        let oubliee = match oubliee {
                            "-" => None,
                            position => Some(position.parse().ok()?),
                        };
                        Cle::Apprendre {
                            emplacement: lire_emplacement(emplacement)?,
                            oubliee,
                        }
                    }
                    "choisir" => Cle::Choisir(numero()?),
                    "sacrifier" => Cle::Sacrifier(numero()?),
                    "monter" => Cle::Monter(position()?),
                    "descendre" => Cle::Descendre(position()?),
                    "envoyer" => Cle::EnvoyerAuCamp(position()?),
                    "faire-venir" => Cle::FaireVenir(position()?),
                    "echanger" => Cle::EchangerAvec(position()?),
                    "renommer" => Cle::Renommer(emplacement()?),
                    "surnom" => Cle::SurnomSaisi(emplacement()?),
                    "relacher" => Cle::Relacher(emplacement()?),
                    "liberer" => Cle::Liberer(numero()?),
                    "attaque" => Cle::Attaquer(position()?),
                    "remplacer" => Cle::Remplacer(position()?),
                    "capturer" => Cle::Capturer(position()?),
                    "oublier" => Cle::Oublier(position()?),
                    _ => return None,
                }
            }
        };
        Some(cle)
    }
}

/// Valeur d'option d'un menu de monstres : `e` pour l'équipe, `c` pour le
/// camp, suivi de la position.
pub fn emplacement_en_texte(emplacement: Emplacement) -> String {
    match emplacement {
        Emplacement::Equipe(index) => format!("e{index}"),
        Emplacement::Camp(index) => format!("c{index}"),
    }
}

fn lire_emplacement(texte: &str) -> Option<Emplacement> {
    if let Some(index) = texte.strip_prefix('e') {
        return index.parse().ok().map(Emplacement::Equipe);
    }
    texte.strip_prefix('c')?.parse().ok().map(Emplacement::Camp)
}

/// Nom fixé ici, et non le nom Rust, comme dans la sauvegarde.
fn amelioration_en_texte(amelioration: Amelioration) -> &'static str {
    match amelioration {
        Amelioration::Enclos => "enclos",
        Amelioration::Filets => "filets",
        Amelioration::Equipe => "equipe",
        Amelioration::Sac => "sac",
        Amelioration::Pieges => "pieges",
        Amelioration::Cour => "cour",
    }
}

fn lire_amelioration(texte: &str) -> Option<Amelioration> {
    Amelioration::TOUTES
        .into_iter()
        .find(|amelioration| amelioration_en_texte(*amelioration) == texte)
}

fn place_en_texte(place: PlaceDEquipement) -> &'static str {
    match place {
        PlaceDEquipement::Arme => "arme",
        PlaceDEquipement::Armure => "armure",
        PlaceDEquipement::PremierTalisman => "talisman-1",
        PlaceDEquipement::SecondTalisman => "talisman-2",
    }
}

fn lire_place(texte: &str) -> Option<PlaceDEquipement> {
    PlaceDEquipement::TOUTES
        .into_iter()
        .find(|place| place_en_texte(*place) == texte)
}

/// Nom fixé ici, et non le nom Rust, comme dans la sauvegarde.
fn pierre_en_texte(pierre: PierreDEvolution) -> &'static str {
    match pierre {
        PierreDEvolution::Braise => "braise",
        PierreDEvolution::Lune => "lune",
    }
}

fn lire_pierre(texte: &str) -> Option<PierreDEvolution> {
    PierreDEvolution::TOUTES
        .into_iter()
        .find(|pierre| pierre_en_texte(*pierre) == texte)
}

/// Nom fixé ici, et non le nom Rust, comme dans la sauvegarde.
pub fn booster_en_texte(booster: Booster) -> &'static str {
    match booster {
        Booster::FioleDeSang => "fiole-de-sang",
        Booster::MoelleDeBete => "moelle-de-bete",
        Booster::EcorceDeFer => "ecorce-de-fer",
        Booster::EncreDArcane => "encre-d-arcane",
        Booster::CendreBenite => "cendre-benite",
        Booster::PoudreDeVifArgent => "poudre-de-vif-argent",
    }
}

fn lire_booster(texte: &str) -> Option<Booster> {
    Booster::TOUS
        .into_iter()
        .find(|booster| booster_en_texte(*booster) == texte)
}

/// Nom fixé ici, et non le nom Rust, comme dans la sauvegarde.
pub fn soin_en_texte(soin: Soin) -> &'static str {
    match soin {
        Soin::Potion => "potion",
        Soin::Remede => "remede",
        Soin::GrandePotion => "grande-potion",
        Soin::Elixir => "elixir",
        Soin::GrandElixir => "grand-elixir",
        Soin::Panacee => "panacee",
    }
}

fn lire_soin(texte: &str) -> Option<Soin> {
    Soin::TOUS
        .into_iter()
        .find(|soin| soin_en_texte(*soin) == texte)
}

/// Valeur d'option des menus de soins : le soin, puis la position, dans
/// l'équipe, du monstre à soigner.
pub fn soin_et_position_en_texte(soin: Soin, index: usize) -> String {
    format!("{}:{index}", soin_en_texte(soin))
}

fn lire_soin_et_position(texte: &str) -> Option<(Soin, usize)> {
    let (soin, index) = texte.split_once(':')?;
    Some((lire_soin(soin)?, index.parse().ok()?))
}

/// Valeur d'option du menu des soins de l'étal : le soin, puis le nombre.
pub fn lot_de_soins_en_texte(soin: Soin, nombre: u32) -> String {
    format!("{}:{nombre}", soin_en_texte(soin))
}

fn lire_lot_de_soins(texte: &str) -> Option<(Soin, u32)> {
    let (soin, nombre) = texte.split_once(':')?;
    Some((lire_soin(soin)?, nombre.parse().ok()?))
}

/// Valeur d'option des menus du coffre : `soins:` suivi du soin et de la
/// quantité, `argent:` suivi de la quantité, `capsule:` suivi de
/// l'attaque, `capsules-d-habilete:` suivi de la quantité, `pierres:`
/// suivi de la pierre et de la quantité, `boosters:` suivi du booster et de
/// la quantité, `eclats-d-astre:` (les Philtres chromatiques, sous leur
/// ancien nom, que gardent les messages déjà envoyés) ou `charmes-de-garde:`
/// suivi de la quantité, `filets:` suivi du filet et de la quantité.
pub fn objet_en_texte(objet: &Objet) -> String {
    match objet {
        Objet::Soins(soin, nombre) => format!("soins:{}:{nombre}", soin_en_texte(*soin)),
        Objet::Argent(montant) => format!("argent:{montant}"),
        Objet::Capsule(attaque) => format!("capsule:{}", attaque.as_str()),
        Objet::CapsulesDHabilete(nombre) => format!("capsules-d-habilete:{nombre}"),
        Objet::Pierres(pierre, nombre) => format!("pierres:{}:{nombre}", pierre_en_texte(*pierre)),
        Objet::Ressources(id, nombre) => format!("ressources:{}:{nombre}", id.as_str()),
        Objet::Boosters(booster, nombre) => {
            format!("boosters:{}:{nombre}", booster_en_texte(*booster))
        }
        Objet::PhiltresChromatiques(nombre) => format!("eclats-d-astre:{nombre}"),
        Objet::CharmesDeGarde(nombre) => format!("charmes-de-garde:{nombre}"),
        Objet::Filets(filet, nombre) => format!("filets:{}:{nombre}", filet.id()),
    }
}

fn lire_objet(texte: &str) -> Option<Objet> {
    let (genre, parametre) = texte.split_once(':')?;
    match genre {
        "soins" => {
            let (soin, nombre) = parametre.split_once(':')?;
            Some(Objet::Soins(lire_soin(soin)?, nombre.parse().ok()?))
        }
        "argent" => Some(Objet::Argent(parametre.parse().ok()?)),
        "capsule" => Some(Objet::Capsule(AttaqueId::new(parametre))),
        "capsules-d-habilete" => Some(Objet::CapsulesDHabilete(parametre.parse().ok()?)),
        "pierres" => {
            let (pierre, nombre) = parametre.split_once(':')?;
            Some(Objet::Pierres(lire_pierre(pierre)?, nombre.parse().ok()?))
        }
        "boosters" => {
            let (booster, nombre) = parametre.split_once(':')?;
            Some(Objet::Boosters(
                lire_booster(booster)?,
                nombre.parse().ok()?,
            ))
        }
        "eclats-d-astre" => Some(Objet::PhiltresChromatiques(parametre.parse().ok()?)),
        "charmes-de-garde" => Some(Objet::CharmesDeGarde(parametre.parse().ok()?)),
        "filets" => {
            let (filet, nombre) = parametre.split_once(':')?;
            Some(Objet::Filets(
                Filet::depuis_id(filet)?,
                nombre.parse().ok()?,
            ))
        }
        "ressources" => {
            let (id, nombre) = parametre.rsplit_once(':')?;
            Some(Objet::Ressources(
                RessourceId::new(id),
                nombre.parse().ok()?,
            ))
        }
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composant {
    pub version: u32,
    pub cle: Cle,
}

impl Composant {
    /// Moins de 100 caractères, la limite de Discord.
    pub fn identifiant(&self) -> String {
        format!("{PREFIXE}:{}:{}", self.version, self.cle.en_texte())
    }

    /// `None` pour un identifiant qui ne vient pas de Bersemon.
    pub fn lire(identifiant: &str) -> Option<Self> {
        let mut morceaux = identifiant.splitn(3, ':');
        if morceaux.next()? != PREFIXE {
            return None;
        }
        let version = morceaux.next()?.parse().ok()?;
        let cle = Cle::depuis_texte(morceaux.next()?)?;
        Some(Self { version, cle })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toutes_les_cles() -> Vec<Cle> {
        vec![
            Cle::ReprendreLaPartie,
            Cle::NouvellePartie,
            Cle::Classe(Classe::Guerrier),
            Cle::Classe(Classe::Mage),
            Cle::Classe(Classe::Templier),
            Cle::Classe(Classe::Assassin),
            Cle::Nommer,
            Cle::NomSaisi,
            Cle::Oui,
            Cle::Non,
            Cle::Partir,
            Cle::Afficher(Vue::Camp),
            Cle::Afficher(Vue::Codex(None)),
            Cle::Afficher(Vue::Codex(Some(10))),
            Cle::Afficher(Vue::PageDuCodex(1)),
            Cle::Afficher(Vue::Gestion(None)),
            Cle::Afficher(Vue::Gestion(Some(Emplacement::Camp(29)))),
            Cle::Fiche(0),
            Cle::Fiche(1),
            Cle::Choisir(2),
            Cle::Sacrifier(1),
            Cle::PartirVersLEclipse,
            Cle::Afficher(Vue::Eclipse),
            Cle::Monter(1),
            Cle::Descendre(0),
            Cle::EnvoyerAuCamp(2),
            Cle::FaireVenir(29),
            Cle::EchangerAvec(3),
            Cle::Renommer(Emplacement::Equipe(0)),
            Cle::SurnomSaisi(Emplacement::Camp(4)),
            Cle::Relacher(Emplacement::Camp(0)),
            Cle::Continuer,
            Cle::Rentrer,
            Cle::Soin,
            Cle::EnTete,
            Cle::Prendre,
            Cle::Poursuivre,
            Cle::Laisser,
            Cle::Liberer(1),
            Cle::RelacherLaCapture,
            Cle::Attaquer(3),
            Cle::Ordre,
            Cle::Afficher(Vue::Echanges {
                avec: None,
                cede: None,
            }),
            Cle::Afficher(Vue::Echanges {
                avec: Some(123_456_789_012_345_678),
                cede: None,
            }),
            Cle::Afficher(Vue::Echanges {
                avec: Some(42),
                cede: Some(Emplacement::Camp(59)),
            }),
            Cle::ChoisirJoueur,
            Cle::Ceder {
                menu: 2,
                avec: 123_456_789_012_345_678,
            },
            Cle::Demander {
                menu: 1,
                avec: 123_456_789_012_345_678,
                cede: Emplacement::Camp(59),
            },
            Cle::Donner {
                avec: 7,
                cede: Emplacement::Equipe(3),
            },
            Cle::AccepterOffre,
            Cle::RefuserOffre,
            Cle::RetirerOffre,
            Cle::Defier(123_456_789_012_345_678),
            Cle::AjouterAmi(123_456_789_012_345_678),
            Cle::RetirerAmi(7),
            Cle::Afficher(Vue::Classement),
            Cle::Afficher(Vue::RecordsDeLaTour),
            Cle::Afficher(Vue::ClassementDeLArene),
            Cle::DefierALArene(123_456_789_012_345_678),
            Cle::AccepterDefi,
            Cle::RefuserDefi,
            Cle::RetirerDefi,
            Cle::Abandonner,
            Cle::AbandonnerLaFosse,
            Cle::ReclamerLaVictoire,
            Cle::RevenirDuDuel,
            Cle::Actualiser,
            Cle::Inviter(123_456_789_012_345_678),
            Cle::AccepterInvitation,
            Cle::RefuserInvitation,
            Cle::RetirerInvitation,
            Cle::RamenerLeGroupe,
            Cle::Filet(Filet::Leste),
            Cle::Fuir,
            Cle::Changer,
            Cle::SoinEnCombat,
            Cle::Remplacer(2),
            Cle::Capturer(1),
            Cle::NePlusCapturer,
            Cle::Oublier(0),
            Cle::Renoncer,
            Cle::Evoluer,
            Cle::NePasEvoluer,
            Cle::FaireEvoluer(Emplacement::Camp(12)),
            Cle::Afficher(Vue::Expeditions),
            Cle::Afficher(Vue::Preparation {
                zone: 1,
                choisis: Choisis::default(),
            }),
            Cle::Afficher(Vue::Preparation {
                zone: 0,
                choisis: trois_choisis(),
            }),
            Cle::Ajouter {
                menu: 2,
                zone: 1,
                choisis: trois_choisis(),
            },
            Cle::Expedier {
                zone: ZoneId::new("marais"),
                choisis: trois_choisis(),
            },
            Cle::Accueillir(ZoneId::new("lisiere")),
            Cle::Rappeler(ZoneId::new("marais")),
            Cle::Afficher(Vue::Marchand),
            Cle::Afficher(Vue::Vente),
            Cle::Afficher(Vue::Attaques(Emplacement::Camp(29))),
            Cle::AcheterSoins,
            Cle::AcheterFilets(Filet::Chanvre, 5),
            Cle::AcheterFilets(Filet::Leste, 5),
            Cle::AcheterCapsule,
            Cle::DonnerAme,
            Cle::Vendre(2),
            Cle::VendreCeMonstre(Emplacement::Equipe(2)),
            Cle::ChangerDHabilete(Emplacement::Camp(7)),
            Cle::DonnerBooster(Emplacement::Camp(59)),
            Cle::RendreShiny(Emplacement::Equipe(3)),
            Cle::RendreLeHerosShiny,
            Cle::AcheterCapsuleDHabilete,
            Cle::AcheterMonstre,
            Cle::LouerMonstre,
            Cle::RendreLeMonstreLoue,
            Cle::Afficher(Vue::Achats),
            Cle::AcheterPierre(PierreDEvolution::Braise),
            Cle::AcheterPierre(PierreDEvolution::Lune),
            Cle::Apprendre {
                emplacement: Emplacement::Camp(29),
                oubliee: None,
            },
            Cle::Apprendre {
                emplacement: Emplacement::Equipe(0),
                oubliee: Some(3),
            },
            Cle::Trier(TriDuCamp::Nom),
            Cle::Trier(TriDuCamp::Niveau),
            Cle::Trier(TriDuCamp::Type),
            Cle::Afficher(Vue::Coffre),
            Cle::Afficher(Vue::Ressources),
            Cle::SortirDuCoffre,
            Cle::RangerAuCoffre,
            Cle::SortirRessourcesDuCoffre(0),
            Cle::RangerRessourcesAuCoffre(1),
            Cle::VendreRessources(2),
            Cle::ToutRanger,
            Cle::Jeter,
            Cle::JeterUneRessource,
            Cle::Afficher(Vue::Options),
            Cle::SeSuicider,
            Cle::Afficher(Vue::Forge),
            Cle::Ameliorer(Amelioration::Enclos),
            Cle::Ameliorer(Amelioration::Filets),
            Cle::Ameliorer(Amelioration::Equipe),
            Cle::Ameliorer(Amelioration::Sac),
            Cle::Constater,
            Cle::Afficher(Vue::Machines),
            Cle::Afficher(Vue::Entrainement),
            Cle::Afficher(Vue::Cour),
            Cle::Afficher(Vue::EntrainementDepuisLaCour),
            Cle::Afficher(Vue::Rite(Emplacement::Camp(12))),
            Cle::SecondParent {
                menu: 1,
                premier: Emplacement::Equipe(2),
            },
            Cle::AccueillirLePetit,
            Cle::ReclamerRecompenseDuCodex,
            Cle::FabriquerCharme,
            Cle::FabriquerFilets,
            Cle::PoserLesPieges,
            Cle::ReleverLesPieges,
            Cle::Entrainer(2),
            Cle::Reprendre(1),
            Cle::Fabriquer,
            Cle::Afficher(Vue::Equipement),
            Cle::Equiper(PlaceDEquipement::Arme),
            Cle::Equiper(PlaceDEquipement::Armure),
            Cle::Equiper(PlaceDEquipement::PremierTalisman),
            Cle::Equiper(PlaceDEquipement::SecondTalisman),
            Cle::Afficher(Vue::Peche),
            Cle::LancerLaLigne,
            Cle::ReleverLaLigne,
            Cle::GarderLaPrise,
            Cle::RelacherLaPrise,
        ]
    }

    #[test]
    fn le_second_parent_lance_le_rite() {
        let cle = Cle::SecondParent {
            menu: 0,
            premier: Emplacement::Equipe(1),
        };
        let valeur = emplacement_en_texte(Emplacement::Camp(4));
        assert_eq!(
            cle.commande(Some(&valeur), None),
            Some(Commande::Jouer(Choix::LancerLeRite {
                premier: Emplacement::Equipe(1),
                second: Emplacement::Camp(4),
            }))
        );
        assert_eq!(
            Cle::AccueillirLePetit.commande(None, None),
            Some(Commande::Jouer(Choix::AccueillirLePetit))
        );
    }

    #[test]
    fn les_filets_s_achetent_et_se_forgent() {
        assert_eq!(
            Cle::AcheterFilets(Filet::Leste, 5).commande(None, None),
            Some(Commande::Jouer(Choix::AcheterFilets(Filet::Leste, 5)))
        );
        assert_eq!(
            Cle::FabriquerFilets.commande(Some("filet-de-chaines"), None),
            Some(Commande::Jouer(Choix::FabriquerFilets(Filet::Chaines)))
        );
        assert_eq!(
            Cle::FabriquerFilets.commande(Some("filet-d-or"), None),
            None
        );
    }

    #[test]
    fn la_protection_suit_les_monstres_choisis() {
        let avec = trois_choisis().avec_protection(Some(1), true);
        let cle = Cle::Expedier {
            zone: ZoneId::new("lisiere"),
            choisis: avec,
        };
        let relue = Cle::depuis_texte(&cle.en_texte()).unwrap();
        assert_eq!(relue, cle);
        assert_eq!(
            relue.commande(None, None),
            Some(Commande::Jouer(Choix::EnvoyerEnExpeditionAutomatique {
                zone: ZoneId::new("lisiere"),
                monstres: trois_choisis().liste(),
                protection: Protection {
                    protege: Some(1),
                    charme: true
                },
            }))
        );
        // Sans monstre protégé, pas de charme.
        assert_eq!(
            trois_choisis().avec_protection(None, true).protection(),
            Protection::default()
        );
    }

    fn trois_choisis() -> Choisis {
        [
            Emplacement::Equipe(2),
            Emplacement::Camp(29),
            Emplacement::Camp(0),
        ]
        .into_iter()
        .try_fold(Choisis::default(), Choisis::avec)
        .unwrap()
    }

    #[test]
    fn trois_monstres_choisis_au_plus_et_chacun_une_fois() {
        let un = Choisis::default().avec(Emplacement::Camp(3)).unwrap();
        assert_eq!(un.avec(Emplacement::Camp(3)), None);
        assert_eq!(trois_choisis().avec(Emplacement::Equipe(0)), None);
        assert!(trois_choisis().est_complet());
        assert_eq!(
            trois_choisis().liste(),
            vec![
                Emplacement::Equipe(2),
                Emplacement::Camp(29),
                Emplacement::Camp(0)
            ]
        );
        assert_eq!(Choisis::lire("c1.c1"), None);
        assert_eq!(Choisis::lire("e0.e1.e2.c0"), None);
    }

    #[test]
    fn un_monstre_ajoute_par_menu_rouvre_la_preparation() {
        let cle = Cle::Ajouter {
            menu: 0,
            zone: 1,
            choisis: Choisis::default(),
        };
        assert_eq!(
            cle.commande(Some("e1"), None),
            Some(Commande::Afficher(Vue::Preparation {
                zone: 1,
                choisis: Choisis::default().avec(Emplacement::Equipe(1)).unwrap(),
            }))
        );
        let expedier = Cle::Expedier {
            zone: ZoneId::new("lisiere"),
            choisis: trois_choisis(),
        };
        assert_eq!(
            expedier.commande(None, None),
            Some(Commande::Jouer(Choix::EnvoyerEnExpeditionAutomatique {
                zone: ZoneId::new("lisiere"),
                monstres: trois_choisis().liste(),
                protection: Protection::default(),
            }))
        );
    }

    #[test]
    fn chaque_composant_se_relit_a_l_identique() {
        for cle in toutes_les_cles() {
            let composant = Composant {
                version: u32::MAX,
                cle,
            };
            let identifiant = composant.identifiant();
            assert!(identifiant.len() < 100, "{identifiant}");
            assert_eq!(Composant::lire(&identifiant), Some(composant));
        }
    }

    #[test]
    fn identifiant_etranger_ignore() {
        for identifiant in [
            "autre:1:oui",
            "bersemon:abc:oui",
            "bersemon:1:inconnu",
            "bersemon:1:attaque:x",
            "bersemon:1:relacher:z3",
            "bersemon:1",
            "",
        ] {
            assert_eq!(Composant::lire(identifiant), None, "{identifiant}");
        }
    }

    #[test]
    fn un_menu_complete_son_choix_par_l_option_choisie() {
        assert_eq!(
            Cle::Changer.commande(Some("2"), None),
            Some(Commande::Jouer(Choix::Combat(Action::Changer(2))))
        );
        assert_eq!(
            Cle::Liberer(1).commande(Some("c27"), None),
            Some(Commande::Jouer(Choix::RelacherPourLaCapture(
                Emplacement::Camp(27)
            )))
        );
        assert_eq!(
            Cle::EnTete.commande(Some("2"), None),
            Some(Commande::Jouer(Choix::Gerer(
                ActionDeGestion::ChangerLOrdre { de: 2, vers: 0 }
            )))
        );
        assert_eq!(Cle::Changer.commande(None, None), None);
        assert_eq!(Cle::Soin.commande(Some("x"), None), None);
        assert_eq!(
            Cle::SoinEnCombat.commande(Some(&soin_et_position_en_texte(Soin::Elixir, 2)), None),
            Some(Commande::Jouer(Choix::Combat(Action::UtiliserSoin(
                Soin::Elixir,
                2
            ))))
        );
        assert_eq!(
            Cle::Soin.commande(Some("remede:0"), None),
            Some(Commande::Jouer(Choix::UtiliserSoin(Soin::Remede, 0)))
        );
        assert_eq!(
            Cle::DonnerBooster(Emplacement::Camp(3))
                .commande(Some(booster_en_texte(Booster::CendreBenite)), None),
            Some(Commande::Jouer(Choix::DonnerBooster {
                emplacement: Emplacement::Camp(3),
                booster: Booster::CendreBenite
            }))
        );
        assert_eq!(
            Cle::SortirRessourcesDuCoffre(1).commande(
                Some(&objet_en_texte(&Objet::Boosters(Booster::FioleDeSang, 2))),
                None
            ),
            Some(Commande::Jouer(Choix::SortirDuCoffre(Objet::Boosters(
                Booster::FioleDeSang,
                2
            ))))
        );
        assert_eq!(
            Cle::RangerRessourcesAuCoffre(0)
                .commande(Some(&objet_en_texte(&Objet::PhiltresChromatiques(1))), None),
            Some(Commande::Jouer(Choix::RangerAuCoffre(
                Objet::PhiltresChromatiques(1)
            )))
        );
    }

    #[test]
    fn un_formulaire_complete_son_choix_par_le_texte_saisi() {
        assert_eq!(
            Cle::NomSaisi.commande(None, Some("Aldric")),
            Some(Commande::Jouer(Choix::NomDuHeros("Aldric".to_string())))
        );
        assert_eq!(
            Cle::SurnomSaisi(Emplacement::Equipe(1)).commande(None, Some("")),
            Some(Commande::Jouer(Choix::Gerer(ActionDeGestion::Renommer {
                emplacement: Emplacement::Equipe(1),
                surnom: String::new(),
            })))
        );
        assert_eq!(
            Cle::Nommer.commande(None, None),
            Some(Commande::Ouvrir(Formulaire::NomDuHeros))
        );
    }

    #[test]
    fn un_menu_d_attaques_apprend_l_attaque_choisie() {
        let cle = Cle::Apprendre {
            emplacement: Emplacement::Camp(4),
            oubliee: Some(1),
        };
        assert_eq!(
            cle.commande(Some("lianes-etrangleuses"), None),
            Some(Commande::Jouer(Choix::ApprendreAttaque {
                emplacement: Emplacement::Camp(4),
                attaque: AttaqueId::new("lianes-etrangleuses"),
                oubliee: Some(1),
            }))
        );
        assert_eq!(
            Cle::Vendre(1).commande(Some("c12"), None),
            Some(Commande::Jouer(Choix::Vendre(Emplacement::Camp(12))))
        );
        assert_eq!(Cle::AcheterCapsule.commande(None, None), None);
        assert_eq!(
            Cle::AcheterSoins.commande(Some(&lot_de_soins_en_texte(Soin::Panacee, 5)), None),
            Some(Commande::Jouer(Choix::AcheterSoins(Soin::Panacee, 5)))
        );
    }

    #[test]
    fn un_menu_du_coffre_deplace_l_objet_choisi() {
        for objet in [
            Objet::Soins(Soin::Potion, 5),
            Objet::Soins(Soin::GrandePotion, 2),
            Objet::Argent(120),
            Objet::Capsule(AttaqueId::new("brasier")),
            Objet::CapsulesDHabilete(3),
            Objet::Ressources(RessourceId::new("croc-de-loup"), 2),
            Objet::Pierres(PierreDEvolution::Lune, 4),
        ] {
            let valeur = objet_en_texte(&objet);
            assert_eq!(
                Cle::SortirDuCoffre.commande(Some(&valeur), None),
                Some(Commande::Jouer(Choix::SortirDuCoffre(objet.clone())))
            );
            assert_eq!(
                Cle::RangerAuCoffre.commande(Some(&valeur), None),
                Some(Commande::Jouer(Choix::RangerAuCoffre(objet)))
            );
        }
        assert_eq!(
            Cle::SortirDuCoffre.commande(Some("soins:potion:x"), None),
            None
        );
        assert_eq!(
            Cle::SortirDuCoffre.commande(Some("soins:baume:1"), None),
            None
        );
        assert_eq!(Cle::RangerAuCoffre.commande(Some("epee:1"), None), None);
    }

    #[test]
    fn le_premier_de_l_equipe_ne_monte_pas() {
        assert_eq!(Cle::Monter(0).commande(None, None), None);
    }
}
