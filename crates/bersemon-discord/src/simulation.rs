//! Partie complète jouée à travers les écrans du bot, sans Discord : chaque
//! coup est un clic sur un composant que le message propose vraiment, ou
//! l'envoi d'un formulaire qu'un de ses boutons ouvre. Chaque message est
//! vérifié contre les limites de Discord.

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use bersemon_core::attaque::Attaque;
use bersemon_core::collection::Emplacement;
use bersemon_core::combat::{Combat, Cote};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{Attente, Evenement};
use bersemon_core::espece::EspeceId;
use bersemon_core::heros::Classe;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::EtatPartie;
use bersemon_core::types::{Efficacite, efficacite};
use bersemon_core::zone::ZoneId;

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

/// Visite, une fois, les vues du camp : liste des monstres, codex et fiche,
/// gestion avec un surnom donné par formulaire.
const TOUR_DU_CAMP: usize = 9;

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

/// Le joueur simple de `deroulement` : la meilleure attaque, une potion sous
/// la moitié des PV, le Marais à partir du niveau 12, toutes les captures, et
/// une expédition automatique.
fn jouer_un_coup(
    session: &Session,
    message: &Message,
    tour: &mut usize,
    automatique_faite: bool,
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
                1 => cliquer(message, |cle| *cle == Cle::Afficher(Vue::Monstres)),
                2 => cliquer(message, |cle| *cle == Cle::Afficher(Vue::Camp)),
                3 => cliquer(message, |cle| *cle == Cle::Afficher(Vue::Codex(None))),
                4 => choisir_dans(message, |cle| *cle == Cle::Fiche, "0").expect("fiche"),
                5 => cliquer(message, |cle| *cle == Cle::Afficher(Vue::Camp)),
                6 => cliquer(message, |cle| *cle == Cle::Afficher(Vue::Gestion(None))),
                7 => choisir_dans(message, |cle| matches!(cle, Cle::Choisir(_)), "e0")
                    .expect("choix du monstre"),
                8 => {
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
            if session.vue != Vue::Camp {
                return cliquer(message, |cle| *cle == Cle::Afficher(Vue::Camp));
            }
            let partie = session.deroulement.partie().unwrap();
            let niveau = partie.collection().equipe()[0].niveau().valeur();
            let marais = choisir_dans(message, |cle| *cle == Cle::Partir, "marais");
            match marais {
                Some(clic) if niveau >= 12 => clic,
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
                    choisir_dans(message, |cle| *cle == Cle::Potion, &index.to_string())
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
        Attente::CaptureDesKo(_) => cliquer(message, |cle| matches!(cle, Cle::Capturer(_))),
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
        Attente::PlaceManquante(_) => cliquer(message, |cle| *cle == Cle::RelacherLaCapture),
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
    let loup = Monstre::creer(
        contenu.espece(&EspeceId::new("loup")).unwrap(),
        Niveau::new(5).unwrap(),
        ValeursInternes::default(),
        false,
    )
    .unwrap();
    let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
    let mut partie = Partie::nouvelle(heros, loup.clone());
    for _ in 0..32 {
        partie.prendre_monstre_rare(loup.clone());
    }
    assert_eq!(partie.collection().camp().len(), 30);
    // Les deux zones explorées, pour préparer une expédition automatique.
    let partie = Partie::restaurer(EtatPartie {
        heros: partie.heros().clone(),
        collection: partie.collection().clone(),
        inventaire: partie.inventaire().clone(),
        argent: partie.argent(),
        codex: partie.codex().clone(),
        zones_explorees: vec![ZoneId::new("lisiere"), ZoneId::new("marais")],
        indices_trouves: 3,
        apotres_vaincus: Vec::new(),
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
        let clic = jouer_un_coup(session, &affiche, &mut tour, automatique_faite);
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
                _ => {}
            }
        }
        vus.extend(evenements.iter().cloned());
    }
    assert!(
        vus.iter()
            .any(|evenement| matches!(evenement, Evenement::Epilogue(_)))
    );
    assert!(
        automatique_faite,
        "expédition automatique jamais accueillie"
    );
    let session = joueurs.session(JOUEUR, &contenu);
    assert_eq!(session.deroulement.attente(), Attente::Camp);
    let partie = session.deroulement.partie().unwrap();
    assert_eq!(partie.indices_trouves(), 3);
    assert_eq!(partie.apotres_vaincus(), &[EspeceId::new("baron-pendu")]);
    assert_eq!(partie.collection().equipe()[0].nom(), "Braise");
    assert!(dossier.join(format!("{JOUEUR}.json")).exists());
    let _ = fs::remove_dir_all(dossier);
}

#[test]
fn toute_l_equipe_se_prepare_d_un_bouton_s_il_reste_un_monstre_au_camp() {
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
            inventaire: partie.inventaire().clone(),
            argent: partie.argent(),
            codex: partie.codex().clone(),
            zones_explorees: vec![ZoneId::new("lisiere")],
            indices_trouves: 1,
            apotres_vaincus: Vec::new(),
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

    // Équipe de 3 Loups, aucun au camp : toute l'équipe ne peut pas partir.
    let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
    let mut partie = Partie::nouvelle(heros, loup.clone());
    for _ in 0..2 {
        partie.prendre_monstre_rare(loup.clone());
    }
    assert_eq!(preparation(&explorer(&partie)), None);

    // Un Loup au camp : le bouton choisit les trois monstres de l'équipe.
    partie.prendre_monstre_rare(loup.clone());
    assert_eq!(
        preparation(&explorer(&partie)),
        Some(vec![
            Emplacement::Equipe(0),
            Emplacement::Equipe(1),
            Emplacement::Equipe(2)
        ])
    );

    // Camp plein : les trois monstres de l'équipe n'auraient qu'une place
    // pour revenir, celle du Loup qui les remplace. Pas de bouton.
    while partie.collection().camp().len() < 30 {
        partie.prendre_monstre_rare(loup.clone());
    }
    assert_eq!(preparation(&explorer(&partie)), None);
}
