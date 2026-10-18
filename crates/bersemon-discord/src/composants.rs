//! Identifiants des composants des messages : boutons, menus et formulaires.
//! Chacun porte ce qu'il déclenche et la version de la partie qu'il
//! affichait : un composant d'un message dépassé est ainsi reconnu, même s'il
//! propose un choix encore possible, comme Oui ou une attaque.
//!
//! Un bouton porte tout son choix ; un menu porte le genre du choix, et
//! l'option choisie le complète ; un formulaire porte le genre du texte saisi.

use bersemon_core::collection::Emplacement;
use bersemon_core::combat::Action;
use bersemon_core::deroulement::{ActionDeGestion, Choix};
use bersemon_core::expedition_automatique::MONSTRES_MAX;
use bersemon_core::heros::Classe;
use bersemon_core::zone::ZoneId;

/// Distingue les composants de Bersemon de ceux d'autres usages éventuels.
const PREFIXE: &str = "bersemon";

/// Ce que montre le camp. Changer de vue ne touche pas à la partie.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Vue {
    #[default]
    Camp,
    Monstres,
    /// Avec la position, dans le codex, de l'espèce dont la fiche est
    /// ouverte.
    Codex(Option<usize>),
    /// Avec le monstre choisi pour être géré.
    Gestion(Option<Emplacement>),
    /// Les zones explorées et leurs expéditions automatiques (section 12).
    Expeditions,
    /// Départ d'une expédition automatique en préparation : la position de
    /// la zone dans le contenu et les monstres déjà choisis.
    Preparation {
        zone: usize,
        choisis: Choisis,
    },
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
    /// Menu de l'équipe, entre deux étapes.
    Potion,
    /// Menu de l'équipe : le monstre choisi passe en tête (Q55).
    EnTete,
    Prendre,
    Laisser,
    /// Menu des monstres à relâcher pour garder une capture.
    Liberer(u8),
    RelacherLaCapture,
    Attaquer(usize),
    Filet,
    Fuir,
    /// Menu de l'équipe, en combat.
    Changer,
    /// Menu de l'équipe, en combat.
    PotionEnCombat,
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
            Cle::Potion => jouer(Choix::UtiliserPotion(position()?)),
            Cle::EnTete => gerer(ActionDeGestion::ChangerLOrdre {
                de: position()?,
                vers: 0,
            }),
            Cle::Prendre => jouer(Choix::PrendreLeMonstreRare),
            Cle::Laisser => jouer(Choix::LaisserLeMonstreRare),
            Cle::Liberer(_) => jouer(Choix::RelacherPourLaCapture(lire_emplacement(valeur?)?)),
            Cle::RelacherLaCapture => jouer(Choix::RelacherLaCapture),
            Cle::Attaquer(index) => jouer(Choix::Combat(Action::Attaquer(*index))),
            Cle::Filet => jouer(Choix::Combat(Action::LancerFilet)),
            Cle::Fuir => jouer(Choix::Combat(Action::Fuir)),
            Cle::Changer => jouer(Choix::Combat(Action::Changer(position()?))),
            Cle::PotionEnCombat => jouer(Choix::Combat(Action::UtiliserPotion(position()?))),
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
        }
    }

    fn en_texte(&self) -> String {
        match self {
            Cle::ReprendreLaPartie => "reprendre".to_string(),
            Cle::NouvellePartie => "nouvelle-partie".to_string(),
            Cle::Classe(Classe::Guerrier) => "classe:guerrier".to_string(),
            Cle::Classe(Classe::Mage) => "classe:mage".to_string(),
            Cle::Classe(Classe::Templier) => "classe:templier".to_string(),
            Cle::Nommer => "nommer".to_string(),
            Cle::NomSaisi => "nom".to_string(),
            Cle::Oui => "oui".to_string(),
            Cle::Non => "non".to_string(),
            Cle::Partir => "partir".to_string(),
            Cle::Afficher(Vue::Camp) => "vue:camp".to_string(),
            Cle::Afficher(Vue::Monstres) => "vue:monstres".to_string(),
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
            Cle::Potion => "potion".to_string(),
            Cle::EnTete => "en-tete".to_string(),
            Cle::Prendre => "prendre".to_string(),
            Cle::Laisser => "laisser".to_string(),
            Cle::Liberer(numero) => format!("liberer:{numero}"),
            Cle::RelacherLaCapture => "relacher-capture".to_string(),
            Cle::Attaquer(index) => format!("attaque:{index}"),
            Cle::Filet => "filet".to_string(),
            Cle::Fuir => "fuir".to_string(),
            Cle::Changer => "changer".to_string(),
            Cle::PotionEnCombat => "potion-combat".to_string(),
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
        }
    }

    fn depuis_texte(texte: &str) -> Option<Self> {
        let cle = match texte {
            "reprendre" => Cle::ReprendreLaPartie,
            "nouvelle-partie" => Cle::NouvellePartie,
            "classe:guerrier" => Cle::Classe(Classe::Guerrier),
            "classe:mage" => Cle::Classe(Classe::Mage),
            "classe:templier" => Cle::Classe(Classe::Templier),
            "nommer" => Cle::Nommer,
            "nom" => Cle::NomSaisi,
            "oui" => Cle::Oui,
            "non" => Cle::Non,
            "partir" => Cle::Partir,
            "vue:camp" => Cle::Afficher(Vue::Camp),
            "vue:monstres" => Cle::Afficher(Vue::Monstres),
            "vue:codex" => Cle::Afficher(Vue::Codex(None)),
            "vue:gestion" => Cle::Afficher(Vue::Gestion(None)),
            "vue:expeditions" => Cle::Afficher(Vue::Expeditions),
            "fiche" => Cle::Fiche,
            "continuer" => Cle::Continuer,
            "rentrer" => Cle::Rentrer,
            "potion" => Cle::Potion,
            "en-tete" => Cle::EnTete,
            "prendre" => Cle::Prendre,
            "laisser" => Cle::Laisser,
            "relacher-capture" => Cle::RelacherLaCapture,
            "filet" => Cle::Filet,
            "fuir" => Cle::Fuir,
            "changer" => Cle::Changer,
            "potion-combat" => Cle::PotionEnCombat,
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
            Cle::Nommer,
            Cle::NomSaisi,
            Cle::Oui,
            Cle::Non,
            Cle::Partir,
            Cle::Afficher(Vue::Camp),
            Cle::Afficher(Vue::Monstres),
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
            Cle::Potion,
            Cle::EnTete,
            Cle::Prendre,
            Cle::Laisser,
            Cle::Liberer(1),
            Cle::RelacherLaCapture,
            Cle::Attaquer(3),
            Cle::Filet,
            Cle::Fuir,
            Cle::Changer,
            Cle::PotionEnCombat,
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
        assert_eq!(Cle::Potion.commande(Some("x"), None), None);
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
    fn le_premier_de_l_equipe_ne_monte_pas() {
        assert_eq!(Cle::Monter(0).commande(None, None), None);
    }
}
