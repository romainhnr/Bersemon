//! Identifiants des composants des messages : boutons, menus et formulaires.
//! Chacun porte ce qu'il déclenche et la version de la partie qu'il
//! affichait : un composant d'un message dépassé est ainsi reconnu, même s'il
//! propose un choix encore possible, comme Oui ou une attaque.
//!
//! Un bouton porte tout son choix ; un menu porte le genre du choix, et
//! l'option choisie le complète ; un formulaire porte le genre du texte saisi.

use bersemon_core::ame::MarchandId;
use bersemon_core::attaque::AttaqueId;
use bersemon_core::collection::{Emplacement, TriDuCamp};
use bersemon_core::combat::Action;
use bersemon_core::deroulement::{ActionDeGestion, Choix};
use bersemon_core::expedition_automatique::MONSTRES_MAX;
use bersemon_core::heros::Classe;
use bersemon_core::inventaire::Objet;
use bersemon_core::soin::Soin;
use bersemon_core::zone::ZoneId;

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
    /// Les attaques que ce monstre connaît et celles qu'il peut apprendre
    /// au camp (section 13.10).
    Attaques(Emplacement),
    /// Le sac et le coffre du camp (section 13.3).
    Coffre,
    /// Le suicide du héros, pour recommencer (section 13.9).
    Options,
}

/// Monstres choisis pour une expédition automatique, dans l'ordre du choix
/// (section 12.1).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Choisis([Option<Emplacement>; MONSTRES_MAX]);

impl Choisis {
    pub fn liste(&self) -> Vec<Emplacement> {
        self.0.iter().flatten().copied().collect()
    }

    pub fn contient(&self, emplacement: Emplacement) -> bool {
        self.0.contains(&Some(emplacement))
    }

    pub fn est_complet(&self) -> bool {
        self.0.iter().all(Option::is_some)
    }

    /// `None` si le monstre est déjà choisi ou qu'il y en a déjà 3.
    pub fn avec(self, emplacement: Emplacement) -> Option<Self> {
        if self.contient(emplacement) {
            return None;
        }
        let mut choisis = self;
        *choisis.0.iter_mut().find(|place| place.is_none())? = Some(emplacement);
        Some(choisis)
    }

    /// `-` pour aucun monstre, sinon les emplacements séparés par des points.
    fn en_texte(&self) -> String {
        let liste: Vec<String> = self.liste().into_iter().map(emplacement_en_texte).collect();
        if liste.is_empty() {
            "-".to_string()
        } else {
            liste.join(".")
        }
    }

    fn lire(texte: &str) -> Option<Self> {
        if texte == "-" {
            return Some(Self::default());
        }
        texte
            .split('.')
            .try_fold(Self::default(), |choisis, emplacement| {
                choisis.avec(lire_emplacement(emplacement)?)
            })
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
    /// Menu des espèces capturées du codex.
    Fiche,
    /// Menu des monstres à gérer.
    Choisir(u8),
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
    Filet,
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
    AcheterSoins(Soin, u32),
    /// Menu des capsules en vente.
    AcheterCapsule,
    /// Menu des âmes portées, à donner à Malvoise (section 14.2).
    DonnerAme,
    /// Menu de monstres numéroté, avec leur prix.
    Vendre(u8),
    /// Le monstre choisi dans la gestion.
    VendreCeMonstre(Emplacement),
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
    ToutRanger,
    SeSuicider,
}

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
            Cle::Partir => jouer(Choix::PartirEnExpedition(ZoneId::new(valeur?))),
            Cle::Afficher(vue) => Some(Commande::Afficher(*vue)),
            Cle::Fiche => Some(Commande::Afficher(Vue::Codex(Some(position()?)))),
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
            Cle::Filet => jouer(Choix::Combat(Action::LancerFilet)),
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
            Cle::Ajouter { zone, choisis, .. } => Some(Commande::Afficher(Vue::Preparation {
                zone: *zone,
                choisis: choisis.avec(lire_emplacement(valeur?)?)?,
            })),
            Cle::Expedier { zone, choisis } => jouer(Choix::EnvoyerEnExpeditionAutomatique {
                zone: zone.clone(),
                monstres: choisis.liste(),
            }),
            Cle::Accueillir(zone) => jouer(Choix::AccueillirExpeditionAutomatique(zone.clone())),
            Cle::Rappeler(zone) => jouer(Choix::RappelerExpeditionAutomatique(zone.clone())),
            Cle::AcheterSoins(soin, nombre) => jouer(Choix::AcheterSoins(*soin, *nombre)),
            Cle::AcheterCapsule => jouer(Choix::AcheterCapsule(AttaqueId::new(valeur?))),
            Cle::DonnerAme => jouer(Choix::DonnerAme(MarchandId::new(valeur?))),
            Cle::Vendre(_) => jouer(Choix::Vendre(lire_emplacement(valeur?)?)),
            Cle::VendreCeMonstre(emplacement) => jouer(Choix::Vendre(*emplacement)),
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
            Cle::ToutRanger => jouer(Choix::ToutRangerAuCoffre),
            Cle::SeSuicider => jouer(Choix::SeSuicider),
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
            Cle::Afficher(Vue::Attaques(emplacement)) => {
                format!("vue:attaques:{}", emplacement_en_texte(*emplacement))
            }
            Cle::Afficher(Vue::Coffre) => "vue:coffre".to_string(),
            Cle::Afficher(Vue::Options) => "vue:options".to_string(),
            Cle::Fiche => "fiche".to_string(),
            Cle::Choisir(numero) => format!("choisir:{numero}"),
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
            Cle::Filet => "filet".to_string(),
            Cle::Fuir => "fuir".to_string(),
            Cle::Changer => "changer".to_string(),
            Cle::SoinEnCombat => "soin-combat".to_string(),
            Cle::Remplacer(index) => format!("remplacer:{index}"),
            Cle::Capturer(index) => format!("capturer:{index}"),
            Cle::NePlusCapturer => "ne-plus-capturer".to_string(),
            Cle::Oublier(index) => format!("oublier:{index}"),
            Cle::Renoncer => "renoncer".to_string(),
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
            Cle::AcheterSoins(soin, nombre) => {
                format!("acheter-soins:{}:{nombre}", soin_en_texte(*soin))
            }
            Cle::AcheterCapsule => "acheter-capsule".to_string(),
            Cle::DonnerAme => "donner-ame".to_string(),
            Cle::Vendre(numero) => format!("vendre:{numero}"),
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
            Cle::ToutRanger => "tout-ranger".to_string(),
            Cle::SeSuicider => "se-suicider".to_string(),
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
            "vue:coffre" => Cle::Afficher(Vue::Coffre),
            "sortir-du-coffre" => Cle::SortirDuCoffre,
            "ranger-au-coffre" => Cle::RangerAuCoffre,
            "tout-ranger" => Cle::ToutRanger,
            "vue:options" => Cle::Afficher(Vue::Options),
            "se-suicider" => Cle::SeSuicider,
            "acheter-capsule" => Cle::AcheterCapsule,
            "donner-ame" => Cle::DonnerAme,
            "fiche" => Cle::Fiche,
            "continuer" => Cle::Continuer,
            "rentrer" => Cle::Rentrer,
            "soin" => Cle::Soin,
            "en-tete" => Cle::EnTete,
            "prendre" => Cle::Prendre,
            "poursuivre" => Cle::Poursuivre,
            "laisser" => Cle::Laisser,
            "relacher-capture" => Cle::RelacherLaCapture,
            "filet" => Cle::Filet,
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
                            "gestion" => {
                                Cle::Afficher(Vue::Gestion(Some(lire_emplacement(parametre)?)))
                            }
                            "attaques" => {
                                Cle::Afficher(Vue::Attaques(lire_emplacement(parametre)?))
                            }
                            "preparation" => {
                                let (zone, choisis) = parametre.split_once(':')?;
                                Cle::Afficher(Vue::Preparation {
                                    zone: zone.parse().ok()?,
                                    choisis: Choisis::lire(choisis)?,
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
                    "expedier" => {
                        let (zone, choisis) = parametre.split_once(':')?;
                        Cle::Expedier {
                            zone: ZoneId::new(zone),
                            choisis: Choisis::lire(choisis)?,
                        }
                    }
                    "accueillir" => Cle::Accueillir(ZoneId::new(parametre)),
                    "rappeler" => Cle::Rappeler(ZoneId::new(parametre)),
                    "acheter-soins" => {
                        let (soin, nombre) = parametre.split_once(':')?;
                        Cle::AcheterSoins(lire_soin(soin)?, nombre.parse().ok()?)
                    }
                    "vendre" => Cle::Vendre(numero()?),
                    "vendre-monstre" => Cle::VendreCeMonstre(emplacement()?),
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
pub fn soin_en_texte(soin: Soin) -> &'static str {
    match soin {
        Soin::Potion => "potion",
        Soin::Remede => "remede",
        Soin::GrandePotion => "grande-potion",
        Soin::Elixir => "elixir",
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

/// Valeur d'option des menus du coffre : `soins:` suivi du soin et de la
/// quantité, `argent:` suivi de la quantité, ou `capsule:` suivi de
/// l'attaque.
pub fn objet_en_texte(objet: &Objet) -> String {
    match objet {
        Objet::Soins(soin, nombre) => format!("soins:{}:{nombre}", soin_en_texte(*soin)),
        Objet::Argent(montant) => format!("argent:{montant}"),
        Objet::Capsule(attaque) => format!("capsule:{}", attaque.as_str()),
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
            Cle::Afficher(Vue::Gestion(None)),
            Cle::Afficher(Vue::Gestion(Some(Emplacement::Camp(29)))),
            Cle::Fiche,
            Cle::Choisir(2),
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
            Cle::Filet,
            Cle::Fuir,
            Cle::Changer,
            Cle::SoinEnCombat,
            Cle::Remplacer(2),
            Cle::Capturer(1),
            Cle::NePlusCapturer,
            Cle::Oublier(0),
            Cle::Renoncer,
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
            Cle::AcheterSoins(Soin::Potion, 5),
            Cle::AcheterSoins(Soin::GrandePotion, 1),
            Cle::AcheterCapsule,
            Cle::DonnerAme,
            Cle::Vendre(2),
            Cle::VendreCeMonstre(Emplacement::Equipe(2)),
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
            Cle::SortirDuCoffre,
            Cle::RangerAuCoffre,
            Cle::ToutRanger,
            Cle::Afficher(Vue::Options),
            Cle::SeSuicider,
        ]
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
    }

    #[test]
    fn un_menu_du_coffre_deplace_l_objet_choisi() {
        for objet in [
            Objet::Soins(Soin::Potion, 5),
            Objet::Soins(Soin::GrandePotion, 2),
            Objet::Argent(120),
            Objet::Capsule(AttaqueId::new("brasier")),
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
