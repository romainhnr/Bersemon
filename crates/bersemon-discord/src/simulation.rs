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
use bersemon_core::equipement::PlaceDEquipement;
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

use crate::composants::{Choisis, Cle, Commande, Composant, RETIRER, Vue};
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
            let abbaye = choisir_dans(message, |cle| *cle == Cle::Partir, "abbaye");
            let vallee = choisir_dans(message, |cle| *cle == Cle::Partir, "vallee");
            let marais = choisir_dans(message, |cle| *cle == Cle::Partir, "marais");
            match (abbaye, vallee, marais) {
                (Some(clic), _, _) if niveau >= 34 => clic,
                (_, Some(clic), _) if niveau >= 24 => clic,
                (_, _, Some(clic)) if niveau >= 12 => clic,
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
        Attente::Prise(_) => cliquer(message, |cle| *cle == Cle::GarderLaPrise),
        // L'ordre du héros, gardé pour les Apôtres et les humains (section
        // 4.2).
        Attente::ActionDeCombat(combat)
            if (combat.est_contre_un_apotre() || combat.est_contre_un_humain())
                && combat.ordre_disponible().is_some() =>
        {
            cliquer(message, |cle| *cle == Cle::Ordre)
        }
        Attente::ActionDeCombat(combat) => {
            // Le bouton disparaît une fois l'ordre donné.
            assert_eq!(
                propose(message, |cle| *cle == Cle::Ordre),
                combat.ordre_disponible().is_some()
            );
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
    use bersemon_core::amelioration::{Ameliorations, Forge};
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::collection::Collection;
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
    // L'enclos au niveau 3 accueille 60 monstres au camp, et l'équipe
    // améliorée en compte 4 (section 13.4).
    let mut grand_camp = partie.collection().camp().to_vec();
    while grand_camp.len() < 60 {
        grand_camp.push(loup.clone());
    }
    let mut grande_equipe = partie.collection().equipe().to_vec();
    grande_equipe.push(loup.clone());
    let agrandi = Ameliorations::new(3, 0, 1).unwrap();
    let configurations = [
        (partie.collection().clone(), Forge::default()),
        (
            Collection::restaurer(grande_equipe, grand_camp, agrandi.tailles()).unwrap(),
            Forge::restaurer(true, agrandi, None).unwrap(),
        ),
    ];
    for (collection, forge) in configurations {
        let dernier = collection.camp().len() - 1;
        let partie = Partie::restaurer(EtatPartie {
            heros: partie.heros().clone(),
            collection,
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
            peche: Default::default(),
            forge,
        })
        .unwrap();
        let maintenant = Horodatage::depuis_secondes(0);
        let deux_choisis = Choisis::default()
            .avec(Emplacement::Equipe(1))
            .and_then(|choisis| choisis.avec(Emplacement::Camp(dernier)))
            .unwrap();
        let vues = [
            Vue::Gestion(None),
            Vue::Gestion(Some(Emplacement::Camp(dernier))),
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
            Vue::Attaques(Emplacement::Camp(dernier)),
            Vue::Coffre,
            Vue::Options,
            Vue::Forge,
        ];
        let mut questions: Vec<_> = vues
            .into_iter()
            .map(|vue| crate::ecran_camp::question(&partie, vue, &contenu, maintenant))
            .collect();
        questions.push(crate::ecran_expedition::place_manquante(
            &loup,
            partie.collection(),
        ));
        // Le combat, avec une équipe de 3 ou de 4.
        let mut en_combat = partie.clone();
        let combat = en_combat.preparer_combat(vec![loup.clone()]).unwrap();
        questions.push(crate::ecran_combat::action(&combat, &en_combat));
        questions.push(crate::ecran_combat::remplacement(
            &combat,
            en_combat.codex(),
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
        peche: Default::default(),
        forge: Default::default(),
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
        // Jusqu'à la chute du deuxième Apôtre, puis le retour au camp.
        let epilogues = vus
            .iter()
            .filter(|evenement| matches!(evenement, Evenement::Epilogue(_)))
            .count();
        if epilogues == 2 && session.deroulement.attente() == Attente::Camp {
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
            Commande::Echanger(action) => panic!("échange {action:?} sans autre joueur"),
            Commande::Duel(action) => panic!("duel {action:?} sans autre joueur"),
            Commande::Actualiser => Vec::new(),
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
    // Le brigand, le Bourreau et l'Inquisiteur barrent leur zone jusqu'à
    // leur défaite, une seule chacun (section 10.7).
    let humains_vaincus = vus
        .iter()
        .filter(|evenement| matches!(evenement, Evenement::HumainVaincu { .. }))
        .count();
    assert_eq!(humains_vaincus, 3);
    assert!(
        vus.iter()
            .any(|evenement| matches!(evenement, Evenement::OrdreDonne { .. })),
        "ordre du héros jamais donné"
    );
    assert!(
        automatique_faite,
        "expédition automatique jamais accueillie"
    );
    assert_eq!(visite, Visite::Faite, "visite à Malvoise jamais faite");
    // Ansgar, sauvé une fois dans le Marais, s'est installé au camp (section
    // 9.2).
    assert_eq!(
        vus.iter()
            .filter(|evenement| matches!(evenement, Evenement::PecheurSauve(_)))
            .count(),
        1
    );
    // La défaite du Bourreau a libéré Gunther (section 13.4).
    assert_eq!(
        vus.iter()
            .filter(|evenement| matches!(evenement, Evenement::GuntherLibere(_)))
            .count(),
        1
    );
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
    assert_eq!(partie.indices_trouves(), 4);
    assert_eq!(
        partie.apotres_vaincus(),
        &[
            EspeceId::new("baron-pendu"),
            EspeceId::new("abbesse-ecorchee")
        ]
    );
    assert_eq!(partie.collection().equipe()[0].nom(), "Braise");
    // Les Apôtres ne se capturent pas : le camp rappelle leurs âmes (section
    // 9.4).
    assert_eq!(
        compter(|evenement| matches!(evenement, Evenement::AmeDApotreRecueillie(_))),
        2
    );
    let camp = message(
        session,
        &[],
        None,
        &contenu,
        Horodatage::depuis_secondes(heure),
    );
    assert!(
        camp.texte
            .contains("Âmes d'Apôtres : Baron pendu et Abbesse écorchée."),
        "{}",
        camp.texte
    );
    assert!(dossier.join(format!("{JOUEUR}.json")).exists());
    let _ = fs::remove_dir_all(dossier);
}

/// Une construction lancée depuis la forge, confirmée, annoncée par message
/// privé à son heure, puis constatée (section 13.4).
#[test]
fn gunther_agrandit_l_enclos_a_travers_les_ecrans_du_bot() {
    use bersemon_core::amelioration::{Amelioration, Forge};
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::heros::Heros;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::Partie;
    use bersemon_core::ressource::RessourceId;
    use bersemon_core::sauvegarde::SauvegardeRepository;
    use bersemon_core::stats::ValeursInternes;
    use bersemon_storage::FichierJson;

    let contenu = ContenuV1::new();
    let loup = Monstre::creer(
        contenu.espece(&EspeceId::new("loup")).unwrap(),
        Niveau::new(5).unwrap(),
        ValeursInternes::default(),
        false,
    )
    .unwrap();
    let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
    let partie = Partie::nouvelle(heros, loup);
    let ressource = |id: &str| contenu.ressource(&RessourceId::new(id)).unwrap();
    let coffre = Inventaire::default()
        .avec_ressources(ressource("croc-de-loup"), 5)
        .avec_ressources(ressource("plume-de-corbeau"), 13)
        .avec_ressources(ressource("ecaille-de-vipere"), 5);
    let partie = Partie::restaurer(EtatPartie {
        heros: partie.heros().clone(),
        collection: partie.collection().clone(),
        sac: Inventaire::default(),
        coffre_du_camp: coffre,
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
        peche: Default::default(),
        forge: Forge::restaurer(true, Default::default(), None).unwrap(),
    })
    .unwrap();
    let dossier = dossier_de_test().join("forge");
    fs::create_dir_all(&dossier).unwrap();
    FichierJson::new(dossier.join(format!("{JOUEUR}.json")))
        .enregistrer(&partie)
        .unwrap();
    let mut joueurs = Joueurs::new(dossier.clone(), 2026);
    let afficher_a = |joueurs: &mut Joueurs, secondes: u64| {
        let maintenant = Horodatage::depuis_secondes(secondes);
        let session = joueurs.session(JOUEUR, &contenu);
        let affiche = message(session, &[], None, &contenu, maintenant);
        verifier_les_limites(&affiche);
        affiche
    };
    let cliquer_a = |joueurs: &mut Joueurs, cle: Cle, secondes: u64| {
        let affiche = afficher_a(joueurs, secondes);
        assert!(
            propose(&affiche, |propose| *propose == cle),
            "{cle:?} absent : {}",
            affiche.texte
        );
        let version = joueurs.session(JOUEUR, &contenu).version;
        let maintenant = Horodatage::depuis_secondes(secondes);
        match cle.commande(None, None).expect("composant lisible") {
            Commande::Jouer(choix) => joueurs
                .jouer(JOUEUR, version, choix, &contenu, maintenant)
                .unwrap(),
            Commande::Afficher(vue) => {
                joueurs.afficher(JOUEUR, version, vue, &contenu).unwrap();
                Vec::new()
            }
            Commande::Ouvrir(formulaire) => panic!("formulaire {formulaire:?}"),
            Commande::Echanger(action) => panic!("échange {action:?}"),
            Commande::Duel(action) => panic!("duel {action:?}"),
            Commande::Actualiser => Vec::new(),
        }
    };
    cliquer_a(&mut joueurs, Cle::ReprendreLaPartie, 1_000);
    cliquer_a(&mut joueurs, Cle::Afficher(Vue::Forge), 1_000);
    cliquer_a(&mut joueurs, Cle::Ameliorer(Amelioration::Enclos), 1_000);
    let lancee = cliquer_a(&mut joueurs, Cle::Oui, 1_000);
    assert!(matches!(
        &lancee[..],
        [Evenement::AmeliorationLancee { .. }]
    ));
    // Rien à constater avant l'heure ; à l'heure, le bot l'annonce.
    let avant = afficher_a(&mut joueurs, 4_599);
    assert!(!propose(&avant, |cle| *cle == Cle::Constater));
    let fin = Horodatage::depuis_secondes(4_600);
    assert!(
        joueurs
            .retours_echus(Horodatage::depuis_secondes(4_599))
            .is_empty()
    );
    assert_eq!(joueurs.retours_echus(fin).len(), 1);
    let constatee = cliquer_a(&mut joueurs, Cle::Constater, 4_600);
    assert!(matches!(&constatee[..], [Evenement::TravailConstate(_)]));
    let session = joueurs.session(JOUEUR, &contenu);
    let partie = session.deroulement.partie().unwrap();
    assert_eq!(partie.collection().tailles().camp, 40);
    assert!(joueurs.retours_echus(fin).is_empty());
    // Gunther forge ensuite un talisman, que le héros porte (section 13.5).
    let choisir_a = |joueurs: &mut Joueurs, voulu: Cle, valeur: &str, secondes: u64| {
        let affiche = afficher_a(joueurs, secondes);
        let clic = choisir_dans(&affiche, |cle| *cle == voulu, valeur)
            .unwrap_or_else(|| panic!("{valeur} absent : {}", affiche.texte));
        let version = joueurs.session(JOUEUR, &contenu).version;
        let Some(Commande::Jouer(choix)) = clic.cle.commande(clic.valeur.as_deref(), None) else {
            panic!("choix attendu");
        };
        joueurs
            .jouer(
                JOUEUR,
                version,
                choix,
                &contenu,
                Horodatage::depuis_secondes(secondes),
            )
            .unwrap()
    };
    let talisman = "plume-de-corbeau-sertie";
    choisir_a(&mut joueurs, Cle::Fabriquer, talisman, 4_600);
    let lancee = cliquer_a(&mut joueurs, Cle::Oui, 4_600);
    assert!(matches!(&lancee[..], [Evenement::FabricationLancee { .. }]));
    let fin = Horodatage::depuis_secondes(8_200);
    assert_eq!(joueurs.retours_echus(fin).len(), 1);
    cliquer_a(&mut joueurs, Cle::Constater, 8_200);
    let affiche = afficher_a(&mut joueurs, 8_200);
    assert!(
        choisir_dans(&affiche, |cle| *cle == Cle::Fabriquer, talisman).is_none(),
        "un objet possédé ne se forge plus"
    );
    cliquer_a(&mut joueurs, Cle::Afficher(Vue::Camp), 8_200);
    cliquer_a(&mut joueurs, Cle::Afficher(Vue::Equipement), 8_200);
    let place = PlaceDEquipement::PremierTalisman;
    let portee = choisir_a(&mut joueurs, Cle::Equiper(place), talisman, 8_200);
    assert!(matches!(&portee[..], [Evenement::ObjetEquipe { .. }]));
    // Porté à la première place, il n'est plus proposé à la seconde ; il
    // se retire.
    let affiche = afficher_a(&mut joueurs, 8_200);
    assert!(
        choisir_dans(
            &affiche,
            |cle| *cle == Cle::Equiper(PlaceDEquipement::SecondTalisman),
            talisman
        )
        .is_none()
    );
    let retire = choisir_a(&mut joueurs, Cle::Equiper(place), RETIRER, 8_200);
    assert!(matches!(&retire[..], [Evenement::ObjetRetire { .. }]));
    let _ = fs::remove_dir_all(dossier);
}

/// Une ligne lancée depuis la pêche, annoncée par message privé à son heure,
/// puis relevée ; un monstre pêché est gardé (section 13.8).
#[test]
fn ansgar_peche_a_travers_les_ecrans_du_bot() {
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::heros::Heros;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::Partie;
    use bersemon_core::peche::{Peche, Prise};
    use bersemon_core::sauvegarde::SauvegardeRepository;
    use bersemon_core::stats::ValeursInternes;
    use bersemon_storage::FichierJson;

    let contenu = ContenuV1::new();
    let loup = Monstre::creer(
        contenu.espece(&EspeceId::new("loup")).unwrap(),
        Niveau::new(5).unwrap(),
        ValeursInternes::default(),
        false,
    )
    .unwrap();
    let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
    let partie = Partie::nouvelle(heros, loup);
    let partie = Partie::restaurer(EtatPartie {
        heros: partie.heros().clone(),
        collection: partie.collection().clone(),
        sac: Inventaire::default(),
        coffre_du_camp: Inventaire::default(),
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
        peche: Peche::restaurer(true, None, None).unwrap(),
        forge: Default::default(),
    })
    .unwrap();
    let dossier = dossier_de_test().join("peche");
    fs::create_dir_all(&dossier).unwrap();
    FichierJson::new(dossier.join(format!("{JOUEUR}.json")))
        .enregistrer(&partie)
        .unwrap();
    let mut joueurs = Joueurs::new(dossier.clone(), 2026);
    let afficher_a = |joueurs: &mut Joueurs, secondes: u64| {
        let maintenant = Horodatage::depuis_secondes(secondes);
        let session = joueurs.session(JOUEUR, &contenu);
        let affiche = message(session, &[], None, &contenu, maintenant);
        verifier_les_limites(&affiche);
        affiche
    };
    let cliquer_a = |joueurs: &mut Joueurs, cle: Cle, secondes: u64| {
        let affiche = afficher_a(joueurs, secondes);
        assert!(
            propose(&affiche, |propose| *propose == cle),
            "{cle:?} absent : {}",
            affiche.texte
        );
        let version = joueurs.session(JOUEUR, &contenu).version;
        let maintenant = Horodatage::depuis_secondes(secondes);
        match cle.commande(None, None).expect("composant lisible") {
            Commande::Jouer(choix) => joueurs
                .jouer(JOUEUR, version, choix, &contenu, maintenant)
                .unwrap(),
            Commande::Afficher(vue) => {
                joueurs.afficher(JOUEUR, version, vue, &contenu).unwrap();
                Vec::new()
            }
            autre => panic!("{autre:?}"),
        }
    };
    cliquer_a(&mut joueurs, Cle::ReprendreLaPartie, 1_000);
    cliquer_a(&mut joueurs, Cle::Afficher(Vue::Peche), 1_000);
    let lancee = cliquer_a(&mut joueurs, Cle::LancerLaLigne, 1_000);
    assert!(matches!(&lancee[..], [Evenement::LigneLancee { .. }]));
    // Rien à relever avant l'heure ; à l'heure, le bot l'annonce.
    let avant = afficher_a(&mut joueurs, 8_199);
    assert!(!propose(&avant, |cle| *cle == Cle::ReleverLaLigne));
    assert!(
        joueurs
            .retours_echus(Horodatage::depuis_secondes(8_199))
            .is_empty()
    );
    let prete = Horodatage::depuis_secondes(8_200);
    let echus = joueurs.retours_echus(prete);
    assert_eq!(echus.len(), 1);
    assert_eq!(
        echus[0].texte(),
        "La ligne d'Ansgar est prête. Ouvrez /bersemon pour la relever."
    );
    let relevee = cliquer_a(&mut joueurs, Cle::ReleverLaLigne, 8_200);
    assert!(joueurs.retours_echus(prete).is_empty());
    let [Evenement::PrisePechee(prise)] = &relevee[..] else {
        panic!("prise attendue : {relevee:?}");
    };
    if let Prise::Monstre(monstre) = prise {
        let affiche = afficher_a(&mut joueurs, 8_200);
        assert!(propose(&affiche, |cle| *cle == Cle::RelacherLaPrise));
        let gardee = cliquer_a(&mut joueurs, Cle::GarderLaPrise, 8_200);
        assert_eq!(
            gardee,
            vec![Evenement::RejointLEquipe(Monstre::clone(monstre))]
        );
    }
    let session = joueurs.session(JOUEUR, &contenu);
    assert_eq!(session.deroulement.attente(), Attente::Camp);
    let affiche = afficher_a(&mut joueurs, 8_200);
    assert!(propose(&affiche, |cle| *cle == Cle::LancerLaLigne));
    let _ = fs::remove_dir_all(dossier);
}

/// Les options de la forge et de l'équipement tiennent dans les limites de
/// Discord, même avec tous les objets possédés et portés.
#[test]
fn la_forge_et_l_equipement_tiennent_dans_les_limites() {
    use crate::ecran_camp::question;
    use crate::message::Question;
    use bersemon_core::amelioration::Forge;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::equipement::Equipement;
    use bersemon_core::heros::Heros;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::Partie;

    let contenu = ContenuV1::new();
    let verifier = |question: &Question| {
        for rangee in &question.rangees {
            if let Rangee::Menu(menu) = rangee {
                for option in &menu.options {
                    assert!(option.libelle.chars().count() <= 100, "{}", option.libelle);
                    let description = option.description.clone().unwrap_or_default();
                    assert!(description.chars().count() <= 100, "{description}");
                }
            }
        }
        assert!(question.texte.chars().count() <= LONGUEUR_MAX);
        assert!(question.rangees.len() <= RANGEES_MAX);
    };
    let loup = Monstre::creer(
        contenu.espece(&EspeceId::new("loup")).unwrap(),
        bersemon_core::niveau::Niveau::new(5).unwrap(),
        bersemon_core::stats::ValeursInternes::default(),
        false,
    )
    .unwrap();
    let possedes: Vec<_> = contenu
        .plans_de_fabrication()
        .iter()
        .map(|plan| plan.objet().clone())
        .collect();
    let id = |nom: &str| bersemon_core::equipement::ObjetDEquipementId::new(nom);
    let portes = [
        (PlaceDEquipement::Arme, id("espadon-des-cendres")),
        (PlaceDEquipement::Armure, id("plates-de-gargouille")),
        (PlaceDEquipement::PremierTalisman, id("croc-tresse")),
        (
            PlaceDEquipement::SecondTalisman,
            id("plume-de-corbeau-sertie"),
        ),
    ];
    for (possedes, portes) in [(Vec::new(), &portes[..0]), (possedes, &portes[..])] {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier)
            .avec_equipement(Equipement::restaurer(possedes, portes).unwrap());
        let partie = Partie::nouvelle(heros, loup.clone());
        let partie = Partie::restaurer(EtatPartie {
            heros: partie.heros().clone(),
            collection: partie.collection().clone(),
            sac: Inventaire::default(),
            coffre_du_camp: Inventaire::default(),
            marchand_au_camp: true,
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
            peche: Default::default(),
            forge: Forge::restaurer(true, Default::default(), None).unwrap(),
        })
        .unwrap();
        let maintenant = Horodatage::depuis_secondes(0);
        for vue in [Vue::Camp, Vue::Forge, Vue::Equipement] {
            verifier(&question(&partie, vue, &contenu, maintenant));
        }
    }
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
            peche: Default::default(),
            forge: Default::default(),
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
        // Ansgar a une ligne prête à relever.
        peche: bersemon_core::peche::Peche::restaurer(
            true,
            Some(Horodatage::depuis_secondes(0)),
            None,
        )
        .unwrap(),
        // Gunther a fini un chantier, qui attend d'être constaté.
        forge: bersemon_core::amelioration::Forge::restaurer(
            true,
            bersemon_core::amelioration::Ameliorations::new(2, 1, 0).unwrap(),
            Some(bersemon_core::amelioration::Chantier {
                ouvrage: bersemon_core::amelioration::Ouvrage::Amelioration {
                    amelioration: bersemon_core::amelioration::Amelioration::Filets,
                    niveau: 2,
                },
                fin: Horodatage::depuis_secondes(0),
            }),
        )
        .unwrap(),
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
        // Aucun menu du coffre n'est coupé : chaque ressource et chaque pierre
        // se déplace (section 14.3).
        let coffre = crate::ecran_coffre::coffre(&partie);
        let sortes = contenu
            .especes()
            .iter()
            .filter(|espece| espece.ressource().is_some())
            .count();
        for rangee in &coffre.rangees {
            if let Rangee::Menu(menu) = rangee {
                assert!(menu.options.len() < OPTIONS_PAR_MENU, "{}", menu.indication);
                if menu.cle == Cle::SortirRessourcesDuCoffre {
                    assert_eq!(menu.options.len(), sortes + 4);
                }
            }
        }
        afficher(&partie, Vue::Vente);
        // Malvoise, Gunther et Ansgar au camp : deux rangées de boutons
        // pleines (section 13.8).
        let cles = afficher(&partie, Vue::Camp);
        assert!(cles.contains(&Cle::Afficher(Vue::Forge)));
        assert!(cles.contains(&Cle::Afficher(Vue::Peche)));
        assert_eq!(cles.len(), 2 * BOUTONS_PAR_RANGEE);
        let cles = afficher(&partie, Vue::Peche);
        assert!(cles.contains(&Cle::ReleverLaLigne));
        let cles = afficher(&partie, Vue::Forge);
        assert!(cles.contains(&Cle::Constater));
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

/// Un clic d'un des joueurs, appliqué, puis le message qu'il reçoit en
/// retour, vérifié contre les limites de Discord.
fn executer_le_clic(
    joueurs: &mut Joueurs,
    joueur: IdJoueur,
    clic: Clic,
    contenu: &ContenuV1,
) -> Message {
    executer_le_clic_a(joueurs, joueur, clic, contenu, 0)
}

fn executer_le_clic_a(
    joueurs: &mut Joueurs,
    joueur: IdJoueur,
    clic: Clic,
    contenu: &ContenuV1,
    secondes: u64,
) -> Message {
    let version = joueurs.session(joueur, contenu).version;
    let maintenant = Horodatage::depuis_secondes(secondes);
    let commande = clic
        .cle
        .commande(clic.valeur.as_deref(), clic.texte.as_deref())
        .expect("composant lisible");
    let evenements = match commande {
        Commande::Jouer(choix) => joueurs
            .jouer(joueur, version, choix, contenu, maintenant)
            .unwrap(),
        Commande::Afficher(vue) => {
            joueurs.afficher(joueur, version, vue, contenu).unwrap();
            Vec::new()
        }
        Commande::Echanger(action) => joueurs
            .echanger(joueur, version, action, contenu)
            .unwrap_or_else(|refus| panic!("{action:?} refusé : {refus:?}")),
        Commande::Duel(action) => joueurs
            .agir_en_duel(joueur, version, action, contenu, maintenant)
            .unwrap_or_else(|refus| panic!("{action:?} refusé : {refus:?}")),
        Commande::Actualiser => Vec::new(),
        Commande::Ouvrir(formulaire) => panic!("formulaire {formulaire:?}"),
    };
    let affiche = message(
        joueurs.session(joueur, contenu),
        &evenements,
        None,
        contenu,
        maintenant,
    );
    verifier_les_limites(&affiche);
    affiche
}

fn valeurs_des_menus(message: &Message, voulu: impl Fn(&Cle) -> bool) -> Vec<String> {
    message
        .rangees
        .iter()
        .flat_map(|rangee| match rangee {
            Rangee::Menu(menu) if voulu(&menu.cle) => menu
                .options
                .iter()
                .map(|option| option.valeur.clone())
                .collect(),
            _ => Vec::new(),
        })
        .collect()
}

/// Aldric propose sa Salamandre contre un Loup du camp d'Ysolde, qui accepte,
/// à travers les écrans du bot ; les deux ont une équipe complète et un camp
/// plein (section 15.1).
#[test]
fn deux_joueurs_echangent_a_travers_les_ecrans_du_bot() {
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::heros::Heros;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::Partie;
    use bersemon_core::sauvegarde::SauvegardeRepository;
    use bersemon_core::stats::ValeursInternes;
    use bersemon_storage::FichierJson;

    const YSOLDE: IdJoueur = 8;
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
    let dossier = dossier_de_test().join("echanges");
    fs::create_dir_all(&dossier).unwrap();
    // Ysolde a vaincu le Baron pendu : un Apôtre ne se demande pas.
    for (joueur, nom, depart, deuxieme) in [
        (JOUEUR, "Aldric", "salamandre", "corbeau"),
        (YSOLDE, "Ysolde", "mandragore", "baron-pendu"),
    ] {
        let heros = Heros::new(Nom::new(nom).unwrap(), Classe::Mage);
        let mut partie = Partie::nouvelle(heros, creer(depart, 5));
        partie.prendre_monstre_rare(creer(deuxieme, 20));
        while partie.collection().a_de_la_place() {
            partie.prendre_monstre_rare(creer("loup", 9));
        }
        FichierJson::new(dossier.join(format!("{joueur}.json")))
            .enregistrer(&partie)
            .unwrap();
    }
    let mut joueurs = Joueurs::new(dossier.clone(), 2026);
    joueurs.noter_pseudo(YSOLDE, "ysolde_la_grise");
    for joueur in [JOUEUR, YSOLDE] {
        executer_le_clic(
            &mut joueurs,
            joueur,
            bouton(Cle::ReprendreLaPartie),
            &contenu,
        );
    }

    // Aldric ouvre les échanges depuis le camp et choisit Ysolde.
    let affiche = message(
        joueurs.session(JOUEUR, &contenu),
        &[],
        None,
        &contenu,
        Horodatage::depuis_secondes(0),
    );
    let echanges = Cle::Afficher(Vue::Echanges {
        avec: None,
        cede: None,
    });
    let clic = cliquer(&affiche, |cle| *cle == echanges);
    let affiche = executer_le_clic(&mut joueurs, JOUEUR, clic, &contenu);
    assert!(
        affiche.texte.contains("Choisissez un joueur"),
        "{}",
        affiche.texte
    );
    let clic = choisir_dans(&affiche, |cle| *cle == Cle::ChoisirJoueur, "8").expect("Ysolde");
    let affiche = executer_le_clic(&mut joueurs, JOUEUR, clic, &contenu);
    // 3 monstres dans l'équipe et 30 au camp : deux menus.
    let a_ceder = valeurs_des_menus(&affiche, |cle| matches!(cle, Cle::Ceder { .. }));
    assert_eq!(a_ceder.len(), 33);
    let clic = choisir_dans(&affiche, |cle| matches!(cle, Cle::Ceder { .. }), "e0").unwrap();
    let affiche = executer_le_clic(&mut joueurs, JOUEUR, clic, &contenu);
    assert!(
        affiche.texte.contains("vous cédez Salamandre (niv. 5)"),
        "{}",
        affiche.texte
    );
    let demandables = valeurs_des_menus(&affiche, |cle| matches!(cle, Cle::Demander { .. }));
    assert_eq!(demandables.len(), 32);
    assert!(
        !demandables.contains(&"e1".to_string()),
        "l'Apôtre est proposé"
    );
    assert!(propose(&affiche, |cle| matches!(cle, Cle::Donner { .. })));
    let clic = choisir_dans(&affiche, |cle| matches!(cle, Cle::Demander { .. }), "c0").unwrap();
    let affiche = executer_le_clic(&mut joueurs, JOUEUR, clic, &contenu);
    assert!(
        affiche.texte.contains(
            "Vous proposez à Ysolde (ysolde_la_grise) Salamandre (niv. 5) contre Loup (niv. 9)."
        ),
        "{}",
        affiche.texte
    );
    assert!(propose(&affiche, |cle| *cle == Cle::RetirerOffre));
    let messages = joueurs.prendre_les_messages_prives();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].joueur, YSOLDE);

    // Ysolde voit l'offre et l'accepte.
    let affiche = executer_le_clic(&mut joueurs, YSOLDE, bouton(echanges.clone()), &contenu);
    assert!(
        affiche
            .texte
            .contains("Aldric vous propose Salamandre (niv. 5), contre votre Loup (niv. 9)."),
        "{}",
        affiche.texte
    );
    assert!(propose(&affiche, |cle| *cle == Cle::RefuserOffre));
    let clic = cliquer(&affiche, |cle| *cle == Cle::AccepterOffre);
    let affiche = executer_le_clic(&mut joueurs, YSOLDE, clic, &contenu);
    assert!(affiche.texte.contains("Vous cédez"), "{}", affiche.texte);
    assert!(affiche.texte.contains("de Aldric."), "{}", affiche.texte);
    assert!(!propose(&affiche, |cle| *cle == Cle::AccepterOffre));
    let messages = joueurs.prendre_les_messages_prives();
    assert_eq!(messages[0].texte, "Ysolde a accepté votre offre.");

    // La Salamandre est au camp d'Ysolde, à la place du Loup.
    let gestion = Cle::Afficher(Vue::Gestion(Some(Emplacement::Camp(0))));
    executer_le_clic(
        &mut joueurs,
        YSOLDE,
        bouton(Cle::Afficher(Vue::Camp)),
        &contenu,
    );
    let affiche = executer_le_clic(&mut joueurs, YSOLDE, bouton(gestion), &contenu);
    assert!(affiche.texte.contains("Salamandre"), "{}", affiche.texte);
    assert!(
        affiche
            .texte
            .contains("Reçu de Aldric. Obéit jusqu'au niveau 12."),
        "{}",
        affiche.texte
    );
    let _ = fs::remove_dir_all(dossier);
}

/// Le message que voit ce joueur à cette heure, vérifié contre les limites
/// de Discord.
fn afficher_a(
    joueurs: &mut Joueurs,
    joueur: IdJoueur,
    contenu: &ContenuV1,
    secondes: u64,
) -> Message {
    let affiche = message(
        joueurs.session(joueur, contenu),
        &[],
        None,
        contenu,
        Horodatage::depuis_secondes(secondes),
    );
    verifier_les_limites(&affiche);
    affiche
}

/// Aldric, Guerrier, et Ysolde, Mage, chacun avec trois monstres, au camp.
fn deux_joueurs_au_camp(contenu: &ContenuV1, sous_dossier: &str) -> (Joueurs, PathBuf) {
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::heros::Heros;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::Partie;
    use bersemon_core::sauvegarde::SauvegardeRepository;
    use bersemon_core::stats::ValeursInternes;
    use bersemon_storage::FichierJson;

    let creer = |id: &str| {
        Monstre::creer(
            contenu.espece(&EspeceId::new(id)).unwrap(),
            Niveau::new(8).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    };
    let dossier = dossier_de_test().join(sous_dossier);
    fs::create_dir_all(&dossier).unwrap();
    for (joueur, nom, classe, monstres) in [
        (
            JOUEUR,
            "Aldric",
            Classe::Guerrier,
            ["salamandre", "loup", "corbeau"],
        ),
        (
            YSOLDE,
            "Ysolde",
            Classe::Mage,
            ["mandragore", "vipere", "loup"],
        ),
    ] {
        let heros = Heros::new(Nom::new(nom).unwrap(), classe);
        let mut partie = Partie::nouvelle(heros, creer(monstres[0]));
        for id in &monstres[1..] {
            partie.prendre_monstre_rare(creer(id));
        }
        FichierJson::new(dossier.join(format!("{joueur}.json")))
            .enregistrer(&partie)
            .unwrap();
    }
    let mut joueurs = Joueurs::new(dossier.clone(), 2026);
    joueurs.noter_pseudo(YSOLDE, "ysolde_la_grise");
    for joueur in [JOUEUR, YSOLDE] {
        executer_le_clic(
            &mut joueurs,
            joueur,
            bouton(Cle::ReprendreLaPartie),
            contenu,
        );
    }
    (joueurs, dossier)
}

const YSOLDE: IdJoueur = 8;

/// Aldric défie Ysolde depuis l'écran des autres joueurs ; elle accepte.
fn lancer_le_duel(joueurs: &mut Joueurs, contenu: &ContenuV1) {
    let affiche = afficher_a(joueurs, JOUEUR, contenu, 0);
    let autres = Cle::Afficher(Vue::Echanges {
        avec: None,
        cede: None,
    });
    let clic = cliquer(&affiche, |cle| *cle == autres);
    let affiche = executer_le_clic(joueurs, JOUEUR, clic, contenu);
    let clic = choisir_dans(&affiche, |cle| *cle == Cle::ChoisirJoueur, "8").unwrap();
    let affiche = executer_le_clic(joueurs, JOUEUR, clic, contenu);
    let clic = cliquer(&affiche, |cle| *cle == Cle::Defier(YSOLDE));
    let affiche = executer_le_clic(joueurs, JOUEUR, clic, contenu);
    assert!(
        affiche
            .texte
            .contains("Vous avez défié Ysolde (ysolde_la_grise) en duel."),
        "{}",
        affiche.texte
    );
    let affiche = executer_le_clic(joueurs, YSOLDE, bouton(autres), contenu);
    assert!(
        affiche.texte.contains("Aldric vous défie en duel."),
        "{}",
        affiche.texte
    );
    let clic = cliquer(&affiche, |cle| *cle == Cle::AccepterDefi);
    let affiche = executer_le_clic(joueurs, YSOLDE, clic, contenu);
    assert!(
        affiche.texte.starts_with("Duel contre Aldric."),
        "{}",
        affiche.texte
    );
    assert_eq!(joueurs.prendre_les_messages_prives().len(), 2);
}

/// Un duel joué jusqu'au bout à travers les écrans du bot : chacun donne
/// l'ordre de son héros, attaque avec sa première attaque, remplace ses
/// monstres KO, et actualise quand il attend l'autre (section 15.3).
#[test]
fn un_duel_complet_a_travers_les_ecrans_du_bot() {
    let contenu = ContenuV1::new();
    let (mut joueurs, dossier) = deux_joueurs_au_camp(&contenu, "duel");
    lancer_le_duel(&mut joueurs, &contenu);
    let mut revenus = Vec::new();
    let mut attentes = 0;
    for _ in 0..500 {
        // Aldric regarde deux fois : la seconde, il attend Ysolde.
        for joueur in [JOUEUR, JOUEUR, YSOLDE] {
            if revenus.contains(&joueur) {
                continue;
            }
            let affiche = afficher_a(&mut joueurs, joueur, &contenu, 0);
            assert!(!affiche.texte.contains("sauvage"), "{}", affiche.texte);
            let clic = if propose(&affiche, |cle| *cle == Cle::RevenirDuDuel) {
                assert!(
                    affiche
                        .texte
                        .contains("Victoire : vous remportez le duel contre")
                        || affiche.texte.contains("remporte le duel."),
                    "{}",
                    affiche.texte
                );
                revenus.push(joueur);
                bouton(Cle::RevenirDuDuel)
            } else if propose(&affiche, |cle| *cle == Cle::Ordre) {
                bouton(Cle::Ordre)
            } else if propose(&affiche, |cle| matches!(cle, Cle::Remplacer(_))) {
                cliquer(&affiche, |cle| matches!(cle, Cle::Remplacer(_)))
            } else if propose(&affiche, |cle| *cle == Cle::Attaquer(0)) {
                bouton(Cle::Attaquer(0))
            } else {
                assert!(
                    affiche.texte.contains("En attente de l'action de"),
                    "{}",
                    affiche.texte
                );
                assert!(!propose(&affiche, |cle| *cle == Cle::ReclamerLaVictoire));
                attentes += 1;
                bouton(Cle::Actualiser)
            };
            executer_le_clic(&mut joueurs, joueur, clic, &contenu);
        }
        if revenus.len() == 2 {
            break;
        }
    }
    assert_eq!(revenus.len(), 2, "duel inachevé");
    assert!(attentes > 0);
    // Chacun retrouve son camp, et ses monstres tels qu'avant le duel.
    for joueur in [JOUEUR, YSOLDE] {
        let affiche = afficher_a(&mut joueurs, joueur, &contenu, 0);
        assert!(propose(&affiche, |cle| *cle == Cle::Afficher(Vue::Options)));
        let partie = joueurs
            .session(joueur, &contenu)
            .deroulement
            .partie()
            .unwrap();
        assert!(
            partie
                .collection()
                .equipe()
                .iter()
                .all(|monstre| monstre.pv_actuels() == monstre.stats().pv)
        );
    }
    let _ = fs::remove_dir_all(dossier);
}

/// Aldric attaque, Ysolde ne joue pas : 5 minutes après, Aldric réclame la
/// victoire par forfait.
#[test]
fn un_joueur_qui_ne_joue_pas_perd_par_forfait() {
    let contenu = ContenuV1::new();
    let (mut joueurs, dossier) = deux_joueurs_au_camp(&contenu, "forfait");
    lancer_le_duel(&mut joueurs, &contenu);
    executer_le_clic_a(
        &mut joueurs,
        JOUEUR,
        bouton(Cle::Attaquer(0)),
        &contenu,
        1_000,
    );
    let affiche = afficher_a(&mut joueurs, JOUEUR, &contenu, 1_299);
    assert!(
        affiche.texte.contains("En attente de l'action de Ysolde."),
        "{}",
        affiche.texte
    );
    assert!(!propose(&affiche, |cle| *cle == Cle::ReclamerLaVictoire));
    let affiche = afficher_a(&mut joueurs, JOUEUR, &contenu, 1_300);
    let clic = cliquer(&affiche, |cle| *cle == Cle::ReclamerLaVictoire);
    let affiche = executer_le_clic_a(&mut joueurs, JOUEUR, clic, &contenu, 1_300);
    assert!(
        affiche
            .texte
            .contains("Victoire : vous remportez le duel contre Ysolde."),
        "{}",
        affiche.texte
    );
    let affiche = afficher_a(&mut joueurs, YSOLDE, &contenu, 1_301);
    assert!(
        affiche.texte.contains("Défaite : Aldric remporte le duel."),
        "{}",
        affiche.texte
    );
    let _ = fs::remove_dir_all(dossier);
}
