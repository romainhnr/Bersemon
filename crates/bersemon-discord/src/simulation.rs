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
use bersemon_core::monstre::Monstre;
use bersemon_core::types::{Efficacite, efficacite};

use crate::composants::{Cle, Commande, Composant, Vue};
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

/// Le joueur simple de `deroulement` : la meilleure attaque, une potion sous
/// la moitié des PV, le Marais à partir du niveau 12, toutes les captures.
fn jouer_un_coup(session: &Session, message: &Message, tour: &mut usize) -> Clic {
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
        Attente::MonstreDeDepart => cliquer(message, |cle| {
            *cle == Cle::MonstreDeDepart(EspeceId::new("salamandre"))
        }),
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
    let questions = [
        crate::ecran_camp::question(&partie, Vue::Gestion(None), &contenu),
        crate::ecran_camp::question(&partie, Vue::Gestion(Some(Emplacement::Camp(29))), &contenu),
        crate::ecran_expedition::place_manquante(&loup, partie.collection()),
    ];
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
    for _ in 0..50_000 {
        let session = joueurs.session(JOUEUR, &contenu);
        let affiche = message(session, &evenements, None, &contenu);
        verifier_les_limites(&affiche);
        if evenements
            .iter()
            .any(|evenement| matches!(evenement, Evenement::Epilogue(_)))
        {
            break;
        }
        let clic = jouer_un_coup(session, &affiche, &mut tour);
        let version = session.version;
        let commande = clic
            .cle
            .commande(clic.valeur.as_deref(), clic.texte.as_deref())
            .expect("composant lisible");
        evenements = match commande {
            Commande::Jouer(choix) => joueurs
                .jouer(JOUEUR, version, choix.clone(), &contenu)
                .unwrap_or_else(|refus| panic!("{choix:?} refusé : {refus:?}")),
            Commande::Afficher(vue) => {
                joueurs.afficher(JOUEUR, version, vue, &contenu).unwrap();
                Vec::new()
            }
            Commande::Ouvrir(formulaire) => panic!("formulaire {formulaire:?} non envoyé"),
        };
        vus.extend(evenements.iter().cloned());
    }
    assert!(
        vus.iter()
            .any(|evenement| matches!(evenement, Evenement::Epilogue(_)))
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
