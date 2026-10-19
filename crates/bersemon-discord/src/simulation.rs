//! Partie complète jouée à travers les écrans du bot, sans Discord : chaque
//! coup est un clic sur un composant que le message propose vraiment, ou
//! l'envoi d'un formulaire qu'un de ses boutons ouvre. Chaque message est
//! vérifié contre les limites de Discord.

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use bersemon_core::attaque::Attaque;
use bersemon_core::codex::EtatCodex;
use bersemon_core::collection::Emplacement;
use bersemon_core::combat::{Combat, Cote};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{Attente, Evenement};
use bersemon_core::espece::EspeceId;
use bersemon_core::heros::Classe;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::inventaire::Inventaire;
use bersemon_core::marchand::prix_du_soin;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::EtatPartie;
use bersemon_core::soin::Soin;
use bersemon_core::types::{Efficacite, efficacite};
use bersemon_core::zone::ZoneId;
use bersemon_presentation::marchand::ressources_a_vendre;

use crate::composants::{Choisis, Cle, Commande, Composant, Vue};
use crate::joueurs::{IdJoueur, Joueurs, Session};
use crate::message::{
    BOUTONS_PAR_RANGEE, LONGUEUR_MAX, Message, OPTIONS_PAR_MENU, RANGEES_MAX, Rangee, message,
};

const JOUEUR: IdJoueur = 7;

/// Un clic sur un bouton, une option choisie dans un menu, ou un texte envoyé
/// par un formulaire.
struct Clic {
    cle: Cle,
    valeur: Option<String>,
    texte: Option<String>,
}

fn bouton(cle: Cle) -> Clic {
    Clic {
        cle,
        valeur: None,
        texte: None,
    }
}

fn verifier_les_limites(message: &Message) {
    assert!(
        message.texte.chars().count() <= LONGUEUR_MAX,
        "{}",
        message.texte
    );
    assert!(message.rangees.len() <= RANGEES_MAX);
    let mut identifiants = HashSet::new();
    for rangee in &message.rangees {
        let cles: Vec<&Cle> = match rangee {
            Rangee::Boutons(boutons) => {
                assert!(!boutons.is_empty() && boutons.len() <= BOUTONS_PAR_RANGEE);
                assert!(boutons.iter().all(|bouton| !bouton.libelle.is_empty()));
                boutons.iter().map(|bouton| &bouton.cle).collect()
            }
            Rangee::Menu(menu) => {
                assert!(!menu.options.is_empty() && menu.options.len() <= OPTIONS_PAR_MENU);
                let valeurs: HashSet<&str> = menu
                    .options
                    .iter()
                    .map(|option| option.valeur.as_str())
                    .collect();
                assert_eq!(valeurs.len(), menu.options.len(), "valeurs en double");
                vec![&menu.cle]
            }
        };
        for cle in cles {
            let identifiant = Composant {
                version: message.version,
                cle: cle.clone(),
            }
            .identifiant();
            assert!(identifiant.len() < 100);
            assert!(identifiants.insert(identifiant), "identifiant en double");
        }
    }
}

fn boutons(message: &Message) -> Vec<&Cle> {
    message
        .rangees
        .iter()
        .flat_map(|rangee| match rangee {
            Rangee::Boutons(boutons) => boutons.iter().map(|bouton| &bouton.cle).collect(),
            Rangee::Menu(_) => Vec::new(),
        })
        .collect()
}

/// Le bouton proposé qui répond à `voulu`.
fn cliquer(message: &Message, voulu: impl Fn(&Cle) -> bool) -> Clic {
    let cle = boutons(message)
        .into_iter()
        .find(|cle| voulu(cle))
        .unwrap_or_else(|| panic!("bouton absent : {}", message.texte));
    bouton(cle.clone())
}

fn propose(message: &Message, voulu: impl Fn(&Cle) -> bool) -> bool {
    boutons(message).into_iter().any(voulu)
}

/// L'option `valeur` d'un menu proposé dont la clé répond à `voulu`.
fn choisir_dans(message: &Message, voulu: impl Fn(&Cle) -> bool, valeur: &str) -> Option<Clic> {
    message.rangees.iter().find_map(|rangee| match rangee {
        Rangee::Menu(menu)
            if voulu(&menu.cle) && menu.options.iter().any(|option| option.valeur == valeur) =>
        {
            Some(Clic {
                cle: menu.cle.clone(),
                valeur: Some(valeur.to_string()),
                texte: None,
            })
        }
        _ => None,
    })
}

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

fn meilleure_attaque(combat: &Combat) -> usize {
    let attaquant = combat.monstre(combat.actif(Cote::Joueur));
    let defenseur = combat.monstre(combat.actif(Cote::Adversaire));
    attaquant
        .attaques()
        .liste()
        .iter()
        .enumerate()
        .max_by_key(|(_, attaque)| valeur_d_attaque(attaque, defenseur))
        .map_or(0, |(position, _)| position)
}

/// Visite, une fois, les vues du camp : codex et fiche, gestion avec un
/// surnom donné par formulaire.
const TOUR_DU_CAMP: usize = 7;

/// Une expédition automatique dans la Lisière, une seule fois, avec le
/// premier monstre du camp : préparation, départ, puis accueil une fois que
/// l'horloge simulée l'a fait rentrer. `None` s'il n'y a rien à faire.
fn expedition_automatique(session: &Session, message: &Message, faite: bool) -> Option<Clic> {
    let partie = session.deroulement.partie()?;
    let lisiere = ZoneId::new("lisiere");
    let en_cours = partie.expedition_automatique(&lisiere).is_some();
    let possible = !partie.collection().camp().is_empty() || en_cours;
    if faite || !partie.est_exploree(&lisiere) || !possible {
        return None;
    }
    match session.vue {
        Vue::Camp => Some(cliquer(message, |cle| {
            *cle == Cle::Afficher(Vue::Expeditions)
        })),
        Vue::Expeditions => {
            assert!(
                !propose(message, |cle| *cle == Cle::Rappeler(lisiere.clone())),
                "l'horloge simulée devait faire rentrer l'expédition"
            );
            if propose(message, |cle| *cle == Cle::Accueillir(lisiere.clone())) {
                return Some(cliquer(message, |cle| {
                    *cle == Cle::Accueillir(lisiere.clone())
                }));
            }
            Some(cliquer(message, |cle| {
                matches!(cle, Cle::Afficher(Vue::Preparation { zone: 0, .. }))
            }))
        }
        Vue::Preparation { .. } => {
            if propose(message, |cle| matches!(cle, Cle::Expedier { .. })) {
                return Some(cliquer(message, |cle| matches!(cle, Cle::Expedier { .. })));
            }
            choisir_dans(message, |cle| matches!(cle, Cle::Ajouter { .. }), "c0")
        }
        _ => None,
    }
}

/// Où en est la visite au marchand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Visite {
    AFaire,
    PotionAchetee,
    MonstreVendu,
    Faite,
}

/// Une visite, une fois le marchand au camp : une potion achetée sur son
/// étal, puis le dernier monstre du camp vendu, avec confirmation, puis,
/// au coffre du camp, le sac rangé puis une potion reprise du coffre.
/// `None` s'il n'y a rien à faire.
fn visite_au_marchand(session: &Session, message: &Message, visite: Visite) -> Option<Clic> {
    let partie = session.deroulement.partie()?;
    // Toute âme portée est donnée à Malvoise dès qu'il est au camp.
    if let Some(ame) = partie
        .ames_portees()
        .first()
        .filter(|_| partie.marchand_au_camp())
    {
        return match session.vue {
            Vue::Camp => Some(cliquer(message, |cle| *cle == Cle::Afficher(Vue::Marchand))),
            Vue::Marchand => choisir_dans(message, |cle| *cle == Cle::DonnerAme, ame.id().as_str()),
            _ => None,
        };
    }
    let camp = partie.collection().camp().len();
    let possible = match visite {
        Visite::AFaire => {
            partie.marchand_au_camp() && camp > 0 && partie.argent() >= prix_du_soin(Soin::Potion)
        }
        Visite::PotionAchetee => camp > 0,
        Visite::MonstreVendu => true,
        Visite::Faite => false,
    };
    if !possible {
        return None;
    }
    match (session.vue, visite) {
        (Vue::Camp, Visite::MonstreVendu) => {
            Some(cliquer(message, |cle| *cle == Cle::Afficher(Vue::Coffre)))
        }
        (Vue::Camp, _) => Some(cliquer(message, |cle| *cle == Cle::Afficher(Vue::Marchand))),
        (Vue::Marchand, Visite::AFaire) => Some(cliquer(message, |cle| {
            *cle == Cle::AcheterSoins(Soin::Potion, 1)
        })),
        (Vue::Marchand, Visite::PotionAchetee) => {
            Some(cliquer(message, |cle| *cle == Cle::Afficher(Vue::Vente)))
        }
        // Les ressources sont vendues avant le monstre (section 14.3).
        (Vue::Vente, Visite::PotionAchetee) if !ressources_a_vendre(partie).is_empty() => {
            let (ressource, _) = &ressources_a_vendre(partie)[0];
            choisir_dans(
                message,
                |cle| *cle == Cle::VendreRessources,
                ressource.id().as_str(),
            )
        }
        (Vue::Vente, Visite::PotionAchetee) => choisir_dans(
            message,
            |cle| matches!(cle, Cle::Vendre(_)),
            &format!("c{}", camp - 1),
        ),
        (Vue::Coffre, Visite::MonstreVendu) if !partie.sac().est_vide() => {
            Some(cliquer(message, |cle| *cle == Cle::ToutRanger))
        }
        (Vue::Coffre, Visite::MonstreVendu) => {
            choisir_dans(message, |cle| *cle == Cle::SortirDuCoffre, "soins:potion:1")
        }
        _ => None,
    }
}

/// Le joueur simple de `deroulement` : la meilleure attaque, une potion sous
/// la moitié des PV, le Marais à partir du niveau 12, toutes les captures,
/// une expédition automatique et une visite au marchand.
fn jouer_un_coup(
    session: &Session,
    message: &Message,
    tour: &mut usize,
    automatique_faite: bool,
    visite: Visite,
) -> Clic {
    match session.deroulement.attente() {
        Attente::NomDuHeros => {
            assert!(propose(message, |cle| *cle == Cle::Nommer));
            Clic {
                cle: Cle::NomSaisi,
                valeur: None,
                texte: Some("Aldric".to_string()),
            }
        }
        Attente::Classe => cliquer(message, |cle| *cle == Cle::Classe(Classe::Mage)),
        Attente::Camp if *tour < TOUR_DU_CAMP => {
            *tour += 1;
            let premier = Emplacement::Equipe(0);
            match *tour {
                1 => cliquer(message, |cle| *cle == Cle::Afficher(Vue::Codex(None))),
                2 => choisir_dans(message, |cle| matches!(cle, Cle::Fiche(_)), "0").expect("fiche"),
                3 => cliquer(message, |cle| *cle == Cle::Afficher(Vue::Camp)),
                4 => cliquer(message, |cle| *cle == Cle::Afficher(Vue::Gestion(None))),
                5 => choisir_dans(message, |cle| matches!(cle, Cle::Choisir(_)), "e0")
                    .expect("choix du monstre"),
                6 => {
                    assert!(propose(message, |cle| *cle == Cle::Renommer(premier)));
                    Clic {
                        cle: Cle::SurnomSaisi(premier),
                        valeur: None,
                        texte: Some("Braise".to_string()),
                    }
                }
                _ => cliquer(message, |cle| *cle == Cle::Afficher(Vue::Camp)),
            }
        }
        Attente::Camp => {
            if let Some(clic) = expedition_automatique(session, message, automatique_faite) {
                return clic;
            }
            if let Some(clic) = visite_au_marchand(session, message, visite) {
                return clic;
            }
            if session.vue != Vue::Camp {
                return cliquer(message, |cle| *cle == Cle::Afficher(Vue::Camp));
            }
            let partie = session.deroulement.partie().unwrap();
            let niveau = partie.collection().equipe()[0].niveau().valeur();
            let vallee = choisir_dans(message, |cle| *cle == Cle::Partir, "vallee");
            let marais = choisir_dans(message, |cle| *cle == Cle::Partir, "marais");
            match (vallee, marais) {
                (Some(clic), _) if niveau >= 24 => clic,
                (_, Some(clic)) if niveau >= 12 => clic,
                _ => choisir_dans(message, |cle| *cle == Cle::Partir, "lisiere").expect("lisière"),
            }
        }
        Attente::EntreDeuxEtapes => {
            let partie = session.deroulement.partie().unwrap();
            let equipe = partie.collection().equipe();
            let blesse = equipe.iter().position(|monstre| {
                !monstre.est_ko() && monstre.pv_actuels() * 2 < monstre.stats().pv
            });
            if let Some(index) = blesse {
                if let Some(clic) =
                    choisir_dans(message, |cle| *cle == Cle::Soin, &format!("potion:{index}"))
                {
                    return clic;
                }
                if equipe[index].pv_actuels() * 4 < equipe[index].stats().pv {
                    return cliquer(message, |cle| *cle == Cle::Rentrer);
                }
            }
            cliquer(message, |cle| *cle == Cle::Continuer)
        }
        Attente::MonstreRare(_) => cliquer(message, |cle| *cle == Cle::Prendre),
        Attente::ActionDeCombat(combat) => {
            let meilleure = meilleure_attaque(combat);
            cliquer(message, |cle| *cle == Cle::Attaquer(meilleure))
        }
        Attente::Remplacement(_) => cliquer(message, |cle| matches!(cle, Cle::Remplacer(_))),
        // Une espèce déjà capturée est laissée KO : elle peut lâcher sa
        // ressource (section 14.3).
        Attente::CaptureDesKo(combat) => {
            let codex = session.deroulement.partie().unwrap().codex();
            let nouvelle = |cle: &Cle| match cle {
                Cle::Capturer(index) => {
                    let espece = combat.adversaires()[*index].espece().id();
                    codex.etat(espece) != EtatCodex::Capturee
                }
                _ => false,
            };
            if boutons(message).into_iter().any(nouvelle) {
                cliquer(message, nouvelle)
            } else {
                cliquer(message, |cle| *cle == Cle::NePlusCapturer)
            }
        }
        Attente::OubliAttaque { monstre, nouvelle } => {
            let (position, plus_faible) = monstre
                .attaques()
                .liste()
                .iter()
                .enumerate()
                .min_by_key(|(_, attaque)| attaque.puissance())
                .unwrap();
            if plus_faible.puissance() < nouvelle.puissance() {
                cliquer(message, |cle| *cle == Cle::Oublier(position))
            } else {
                cliquer(message, |cle| *cle == Cle::Renoncer)
            }
        }
        Attente::Evolution { .. } => {
            assert!(propose(message, |cle| *cle == Cle::NePasEvoluer));
            cliquer(message, |cle| *cle == Cle::Evoluer)
        }
        Attente::PlaceManquante(_) => cliquer(message, |cle| *cle == Cle::RelacherLaCapture),
        Attente::Annonce(_) => cliquer(message, |cle| *cle == Cle::Poursuivre),
        Attente::Confirmation(_) => cliquer(message, |cle| *cle == Cle::Oui),
        Attente::Reprise(_) => cliquer(message, |cle| *cle == Cle::ReprendreLaPartie),
        Attente::SauvegardeIllisible(_) => cliquer(message, |cle| *cle == Cle::NouvellePartie),
    }
}

fn dossier_de_test() -> PathBuf {
    std::env::temp_dir().join(format!(
        "bersemon-discord-simulation-{}",
        std::process::id()
    ))
}

#[test]
fn l_equipe_et_le_camp_pleins_tiennent_dans_les_limites() {
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::heros::Heros;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::Partie;
    use bersemon_core::stats::ValeursInternes;

    let contenu = ContenuV1::new();
    let creer = |id: &str, niveau: u8| {
        Monstre::creer(
            contenu.espece(&EspeceId::new(id)).unwrap(),
            Niveau::new(niveau).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    };
    let loup = creer("loup", 5);
    // Au niveau 14, la Salamandre connaît 4 attaques : elle peut réapprendre
    // Souffle chaud ou apprendre Brasier, en oubliant une des siennes.
    let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
    let mut partie = Partie::nouvelle(heros, creer("salamandre", 14));
    for _ in 0..32 {
        partie.prendre_monstre_rare(loup.clone());
    }
    assert_eq!(partie.collection().camp().len(), 30);
    // Les deux zones explorées, pour préparer une expédition automatique,
    // le marchand au camp, la capsule Brasier dans le sac et les autres
    // dans le coffre.
    let capsules = contenu.capsules();
    let partie = Partie::restaurer(EtatPartie {
        heros: partie.heros().clone(),
        collection: partie.collection().clone(),
        sac: Inventaire::new(45, 3, capsules[..1].to_vec()),
        coffre_du_camp: Inventaire::new(1_000, 12, capsules[1..].to_vec()),
        marchand_au_camp: true,
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
    })
    .unwrap();
    let maintenant = Horodatage::depuis_secondes(0);
    let deux_choisis = Choisis::default()
        .avec(Emplacement::Equipe(1))
        .and_then(|choisis| choisis.avec(Emplacement::Camp(29)))
        .unwrap();
    let vues = [
        Vue::Gestion(None),
        Vue::Gestion(Some(Emplacement::Camp(29))),
        Vue::Expeditions,
        Vue::Preparation {
            zone: 1,
            choisis: Choisis::default(),
        },
        Vue::Preparation {
            zone: 1,
            choisis: deux_choisis,
        },
        Vue::Camp,
        Vue::Marchand,
        Vue::Vente,
        Vue::Attaques(Emplacement::Equipe(0)),
        Vue::Attaques(Emplacement::Camp(29)),
        Vue::Coffre,
        Vue::Options,
    ];
    let mut questions: Vec<_> = vues
        .into_iter()
        .map(|vue| crate::ecran_camp::question(&partie, vue, &contenu, maintenant))
        .collect();
    questions.push(crate::ecran_expedition::place_manquante(
        &loup,
        partie.collection(),
    ));
    for question in questions {
        let affiche = Message {
            texte: question.texte,
            rangees: question.rangees,
            version: 1,
        };
        verifier_les_limites(&affiche);
    }
}

#[test]
fn les_attaques_au_camp_se_choisissent_par_menu() {
    use bersemon_core::attaque::AttaqueId;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::deroulement::Choix;
    use bersemon_core::heros::Heros;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::Partie;
    use bersemon_core::stats::ValeursInternes;

    let contenu = ContenuV1::new();
    let creer = |id: &str, niveau: u8| {
        Monstre::creer(
            contenu.espece(&EspeceId::new(id)).unwrap(),
            Niveau::new(niveau).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    };
    let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
    let mut partie = Partie::nouvelle(heros, creer("salamandre", 14));
    partie.prendre_monstre_rare(creer("salamandre", 5));
    let partie = Partie::restaurer(EtatPartie {
        heros: partie.heros().clone(),
        collection: partie.collection().clone(),
        sac: partie.sac().clone(),
        coffre_du_camp: Inventaire::new(0, 0, contenu.capsules()[..1].to_vec()),
        marchand_au_camp: false,
        ames_donnees: Vec::new(),
        ames_portees: Vec::new(),
        ames_a_retrouver: Vec::new(),
        codex: partie.codex().clone(),
        zones_explorees: Vec::new(),
        indices_trouves: 0,
        apotres_vaincus: Vec::new(),
        humains_vaincus: Vec::new(),
        conseils_vus: Vec::new(),
        expeditions_automatiques: Vec::new(),
        attaques_a_choisir: Vec::new(),
    })
    .unwrap();
    let menus = |emplacement: Emplacement| -> Vec<(Cle, Vec<String>)> {
        let question = crate::ecran_camp::question(
            &partie,
            Vue::Attaques(emplacement),
            &contenu,
            Horodatage::depuis_secondes(0),
        );
        question
            .rangees
            .into_iter()
            .filter_map(|rangee| match rangee {
                Rangee::Menu(menu) => Some((
                    menu.cle,
                    menu.options
                        .into_iter()
                        .map(|option| option.valeur)
                        .collect(),
                )),
                Rangee::Boutons(_) => None,
            })
            .collect()
    };

    // Quatre attaques connues : un menu par attaque à oublier.
    let complete = menus(Emplacement::Equipe(0));
    assert_eq!(complete.len(), 4);
    for (position, (cle, valeurs)) in complete.iter().enumerate() {
        assert_eq!(valeurs, &vec!["souffle-chaud", "brasier"]);
        assert_eq!(
            cle.commande(Some("brasier"), None),
            Some(Commande::Jouer(Choix::ApprendreAttaque {
                emplacement: Emplacement::Equipe(0),
                attaque: AttaqueId::new("brasier"),
                oubliee: Some(position),
            }))
        );
    }
    // Au niveau 5, la Salamandre n'en connaît que 2 : un seul menu, sans
    // rien oublier.
    let libre = menus(Emplacement::Equipe(1));
    assert_eq!(libre.len(), 1);
    assert_eq!(
        libre[0].0,
        Cle::Apprendre {
            emplacement: Emplacement::Equipe(1),
            oubliee: None,
        }
    );
    assert_eq!(libre[0].1, vec!["brasier"]);
}

#[test]
fn une_partie_complete_a_travers_les_ecrans_du_bot() {
    let contenu = ContenuV1::new();
    let dossier = dossier_de_test();
    let mut joueurs = Joueurs::new(dossier.clone(), 2026);
    let mut evenements: Vec<Evenement> = Vec::new();
    let mut vus = Vec::new();
    let mut tour = 0;
    // Horloge simulée : elle n'avance qu'au départ de l'expédition
    // automatique, assez pour la faire rentrer.
    let mut heure = 0;
    let mut automatique_faite = false;
    let mut visite = Visite::AFaire;
    for _ in 0..50_000 {
        let maintenant = Horodatage::depuis_secondes(heure);
        let session = joueurs.session(JOUEUR, &contenu);
        let affiche = message(session, &evenements, None, &contenu, maintenant);
        verifier_les_limites(&affiche);
        if evenements
            .iter()
            .any(|evenement| matches!(evenement, Evenement::Epilogue(_)))
        {
            break;
        }
        let clic = jouer_un_coup(session, &affiche, &mut tour, automatique_faite, visite);
        let version = session.version;
        let commande = clic
            .cle
            .commande(clic.valeur.as_deref(), clic.texte.as_deref())
            .expect("composant lisible");
        evenements = match commande {
            Commande::Jouer(choix) => joueurs
                .jouer(JOUEUR, version, choix.clone(), &contenu, maintenant)
                .unwrap_or_else(|refus| panic!("{choix:?} refusé : {refus:?}")),
            Commande::Afficher(vue) => {
                joueurs.afficher(JOUEUR, version, vue, &contenu).unwrap();
                Vec::new()
            }
            Commande::Ouvrir(formulaire) => panic!("formulaire {formulaire:?} non envoyé"),
        };
        for evenement in &evenements {
            match evenement {
                Evenement::ExpeditionAutomatiquePartie(expedition) => {
                    // Le retour est à annoncer dès son heure, pas avant.
                    assert!(joueurs.retours_echus(maintenant).is_empty());
                    heure = expedition.retour().secondes();
                    assert_eq!(
                        joueurs.retours_echus(expedition.retour()).len(),
                        1,
                        "retour à annoncer"
                    );
                }
                Evenement::ExpeditionAutomatiqueRentree { .. } => {
                    automatique_faite = true;
                    // Accueillie avant l'annonce : il n'y a plus rien à
                    // annoncer.
                    assert!(joueurs.retours_echus(maintenant).is_empty());
                }
                Evenement::SoinsAchetes { nombre: 1, .. } => visite = Visite::PotionAchetee,
                Evenement::MonstreVendu { .. } => visite = Visite::MonstreVendu,
                Evenement::SortiDuCoffre(_) => visite = Visite::Faite,
                _ => {}
            }
        }
        vus.extend(evenements.iter().cloned());
    }
    assert!(
        vus.iter()
            .any(|evenement| matches!(evenement, Evenement::Epilogue(_)))
    );
    // Le brigand et le Bourreau barrent la Lisière et le Marais jusqu'à leur
    // défaite, une seule chacun (section 10.7).
    let humains_vaincus = vus
        .iter()
        .filter(|evenement| matches!(evenement, Evenement::HumainVaincu { .. }))
        .count();
    assert_eq!(humains_vaincus, 2);
    assert!(
        automatique_faite,
        "expédition automatique jamais accueillie"
    );
    assert_eq!(visite, Visite::Faite, "visite à Malvoise jamais faite");
    // Malvoise est rencontré une fois ; les âmes recueillies passent par
    // l'écran du combat et le récit, puis sont données sur son étal (section
    // 14.2).
    let compter = |cherche: fn(&Evenement) -> bool| vus.iter().filter(|e| cherche(e)).count();
    assert_eq!(
        compter(|evenement| matches!(evenement, Evenement::MalvoiseRencontre { .. })),
        1
    );
    assert!(compter(|evenement| matches!(evenement, Evenement::AmeRecueillie(_))) > 0);
    assert!(compter(|evenement| matches!(evenement, Evenement::AmeDonnee { .. })) > 0);
    // Les monstres battus lâchent des ressources, que Malvoise rachète
    // (section 14.3).
    assert!(compter(|evenement| matches!(evenement, Evenement::RessourcesTrouvees(_))) > 0);
    assert!(compter(|evenement| matches!(evenement, Evenement::RessourcesVendues { .. })) > 0);
    assert!(
        vus.iter()
            .any(|evenement| matches!(evenement, Evenement::RangeAuCoffre(_))),
        "sac jamais rangé"
    );
    // La Salamandre de départ évolue après un combat (section 5.4).
    assert!(compter(|evenement| matches!(evenement, Evenement::Evolution { .. })) > 0);
    let session = joueurs.session(JOUEUR, &contenu);
    assert_eq!(session.deroulement.attente(), Attente::Camp);
    let partie = session.deroulement.partie().unwrap();
    assert_eq!(partie.indices_trouves(), 2);
    assert_eq!(partie.apotres_vaincus(), &[EspeceId::new("baron-pendu")]);
    assert_eq!(partie.collection().equipe()[0].nom(), "Braise");
    assert!(dossier.join(format!("{JOUEUR}.json")).exists());
    let _ = fs::remove_dir_all(dossier);
}

#[test]
fn toute_l_equipe_se_prepare_d_un_bouton_s_il_y_a_de_la_place_au_camp() {
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::heros::Heros;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::Partie;
    use bersemon_core::stats::ValeursInternes;

    let contenu = ContenuV1::new();
    let loup = Monstre::creer(
        contenu.espece(&EspeceId::new("loup")).unwrap(),
        Niveau::new(5).unwrap(),
        ValeursInternes::default(),
        false,
    )
    .unwrap();
    let explorer = |partie: &Partie| {
        Partie::restaurer(EtatPartie {
            heros: partie.heros().clone(),
            collection: partie.collection().clone(),
            sac: partie.sac().clone(),
            coffre_du_camp: partie.coffre_du_camp().clone(),
            marchand_au_camp: partie.marchand_au_camp(),
            ames_donnees: partie.ames_donnees().to_vec(),
            ames_portees: partie.ames_portees().to_vec(),
            ames_a_retrouver: partie.ames_a_retrouver().to_vec(),
            codex: partie.codex().clone(),
            zones_explorees: vec![ZoneId::new("lisiere")],
            indices_trouves: 1,
            apotres_vaincus: Vec::new(),
            humains_vaincus: Vec::new(),
            conseils_vus: Vec::new(),
            expeditions_automatiques: Vec::new(),
            attaques_a_choisir: Vec::new(),
        })
        .unwrap()
    };
    let preparation = |partie: &Partie| {
        let question = crate::ecran_camp::question(
            partie,
            Vue::Preparation {
                zone: 0,
                choisis: Choisis::default(),
            },
            &contenu,
            Horodatage::depuis_secondes(0),
        );
        let affiche = Message {
            texte: question.texte,
            rangees: question.rangees,
            version: 1,
        };
        verifier_les_limites(&affiche);
        boutons(&affiche).into_iter().find_map(|cle| match cle {
            Cle::Afficher(Vue::Preparation { choisis, .. }) if !choisis.liste().is_empty() => {
                Some(choisis.liste())
            }
            _ => None,
        })
    };

    // Équipe de 3 Loups, aucun au camp : le bouton choisit les trois
    // monstres de l'équipe, qui restera vide.
    let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
    let mut partie = Partie::nouvelle(heros, loup.clone());
    for _ in 0..2 {
        partie.prendre_monstre_rare(loup.clone());
    }
    let toute_l_equipe = Some(vec![
        Emplacement::Equipe(0),
        Emplacement::Equipe(1),
        Emplacement::Equipe(2),
    ]);
    assert_eq!(preparation(&explorer(&partie)), toute_l_equipe);

    // Trois places libres au camp : juste assez pour leur retour.
    while partie.collection().camp().len() < 27 {
        partie.prendre_monstre_rare(loup.clone());
    }
    assert_eq!(preparation(&explorer(&partie)), toute_l_equipe);

    // Deux places libres : pas de bouton.
    partie.prendre_monstre_rare(loup.clone());
    assert_eq!(preparation(&explorer(&partie)), None);
}

#[test]
fn les_ecrans_du_camp_les_plus_charges_restent_dans_les_limites() {
    use bersemon_core::ame::MarchandId;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::heros::Heros;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::Partie;
    use bersemon_core::pierre::PierreDEvolution;
    use bersemon_core::stats::ValeursInternes;

    let contenu = ContenuV1::new();
    // Au niveau 16, chacun peut évoluer (section 5.4).
    let creer = |id: &str| {
        Monstre::creer(
            contenu.espece(&EspeceId::new(id)).unwrap(),
            Niveau::new(16).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    };
    let marchand = |id: &str| contenu.marchand_mort(&MarchandId::new(id)).unwrap();
    let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
    let mut partie = Partie::nouvelle(heros, creer("salamandre"));
    for id in ["loup", "corbeau", "vipere", "feu-follet"] {
        partie.prendre_monstre_rare(creer(id));
    }
    while partie.collection().camp().len() < 30 {
        partie.prendre_monstre_rare(creer("loup"));
    }
    // Tous les soins, toutes les capsules d'attaque, de l'argent, des
    // capsules d'habileté et toutes les ressources dans le coffre comme dans
    // le sac, et un camp plein.
    // Une capsule d'attaque ne s'obtient qu'une fois : elles sont toutes au
    // coffre.
    let plein = |capsules: bool| {
        let capsules = if capsules {
            contenu.capsules().to_vec()
        } else {
            Vec::new()
        };
        let mut inventaire = Inventaire::new(1_000, 9, capsules)
            .avec_capsules_d_habilete(3)
            .avec_pierres(PierreDEvolution::Braise, 2)
            .avec_pierres(PierreDEvolution::Lune, 2);
        for soin in [Soin::Remede, Soin::GrandePotion, Soin::Elixir] {
            inventaire = inventaire.avec_soins(soin, 7);
        }
        for ressource in contenu
            .especes()
            .iter()
            .filter_map(|espece| espece.ressource())
        {
            inventaire = inventaire.avec_ressources(ressource.clone(), 3);
        }
        inventaire
    };
    // Toutes les espèces capturées : le codex a plus de 25 fiches.
    let mut codex = partie.codex().clone();
    for espece in contenu.especes() {
        codex.noter_capture(espece.id());
    }
    let etat = |donnees: Vec<&str>, portees: Vec<&str>| EtatPartie {
        heros: partie.heros().clone(),
        collection: partie.collection().clone(),
        sac: plein(false),
        coffre_du_camp: plein(true),
        marchand_au_camp: true,
        ames_donnees: donnees.into_iter().map(marchand).collect(),
        ames_portees: portees.into_iter().map(marchand).collect(),
        ames_a_retrouver: Vec::new(),
        codex: codex.clone(),
        zones_explorees: vec![ZoneId::new("lisiere")],
        indices_trouves: 1,
        apotres_vaincus: Vec::new(),
        humains_vaincus: Vec::new(),
        conseils_vus: Vec::new(),
        expeditions_automatiques: Vec::new(),
        attaques_a_choisir: Vec::new(),
    };
    let afficher = |partie: &Partie, vue: Vue| {
        let question =
            crate::ecran_camp::question(partie, vue, &contenu, Horodatage::depuis_secondes(0));
        let affiche = Message {
            texte: question.texte,
            rangees: question.rangees,
            version: 1,
        };
        verifier_les_limites(&affiche);
        boutons(&affiche).into_iter().cloned().collect::<Vec<Cle>>()
    };
    // Tous les soins en vente, ou une âme encore à donner.
    for (donnees, portees) in [
        (vec!["bertrand", "grete", "ulrich"], vec![]),
        (vec!["bertrand", "grete"], vec!["ulrich"]),
    ] {
        let partie = Partie::restaurer(etat(donnees, portees)).unwrap();
        let cles = afficher(&partie, Vue::Marchand);
        assert!(cles.contains(&Cle::AcheterCapsuleDHabilete));
        assert!(cles.contains(&Cle::AcheterPierre(PierreDEvolution::Lune)));
        afficher(&partie, Vue::Coffre);
        afficher(&partie, Vue::Vente);
        afficher(&partie, Vue::Camp);
        // Un monstre de l'équipe, qui peut monter et descendre, et un du
        // camp, qui peut s'échanger.
        for emplacement in [Emplacement::Equipe(1), Emplacement::Camp(0)] {
            let cles = afficher(&partie, Vue::Gestion(Some(emplacement)));
            assert!(cles.contains(&Cle::ChangerDHabilete(emplacement)));
            assert!(cles.contains(&Cle::FaireEvoluer(emplacement)));
        }
        // Le Feu follet, deuxième du camp, évolue avec la Pierre de braise.
        let feu_follet = Emplacement::Camp(1);
        let cles = afficher(&partie, Vue::Gestion(Some(feu_follet)));
        assert!(cles.contains(&Cle::FaireEvoluer(feu_follet)));
        let dernier = contenu.especes().len() - 1;
        for fiche in [None, Some(0), Some(dernier)] {
            afficher(&partie, Vue::Codex(fiche));
        }
    }
}
