//! Camp de base sur Discord : départ en expédition par la carte, expéditions
//! automatiques, codex, gestion des monstres et de leurs attaques (sections
//! 13.2 et 13.10), coffre du camp (section 13.3), marchand, options (section
//! 13.9), forge de Gunther, équipement du héros (sections 4.3, 13.4 et
//! 13.5) et pêche d'Ansgar (section 13.8). Les vues ne changent rien à la partie ; seules les actions la
//! modifient, et la sauvegardent.

use bersemon_core::amelioration::Amelioration;
use bersemon_core::booster::Booster;
use bersemon_core::codex::EtatCodex;
use bersemon_core::collection::{Collection, Emplacement, TriDuCamp};
use bersemon_core::combat::Cote;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::equipement::PlaceDEquipement;
use bersemon_core::habilete::autre_habilete;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::marchand::prix_de_revente;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::stats::ValeursInternes;
use bersemon_presentation::boosters::{boosters, effet_du_booster};
use bersemon_presentation::equipement::{
    decrire_bonus, etat_de_l_equipement, nom_de_place, objet_et_bonus,
};
use bersemon_presentation::expeditions_automatiques::duree;
use bersemon_presentation::expeditions_automatiques::expeditions_rentrees;
use bersemon_presentation::forge::{cout, etat_de_la_forge, travail_fini};
use bersemon_presentation::monstres::{
    attaques_connues, decrire_attaque, fiche_du_monstre, ligne_monstre, nom, provenance,
};
use bersemon_presentation::partie::{
    ames_d_apotres, argent, collection as liste_complete, en_tete_du_camp, fiche_d_espece,
    ligne_du_codex,
};
use bersemon_presentation::peche::ligne_prete;

use crate::composants::{Cle, RETIRER, Vue, booster_en_texte, emplacement_en_texte};
use crate::echanges::TableauDesEchanges;
use crate::message::{
    Bouton, DANGER, OPTIONS_PAR_MENU, OptionDeMenu, PRINCIPAL, Question, Rangee, SECONDAIRE,
    SUCCES, bouton, menu, rangees,
};
use crate::{
    ecran_coffre, ecran_echanges, ecran_expeditions_automatiques, ecran_marchand, ecran_peche,
};

pub fn question(
    partie: &Partie,
    vue: Vue,
    contenu: &ContenuV1,
    maintenant: Horodatage,
) -> Question {
    match vue {
        Vue::Camp => principale(partie, contenu, maintenant),
        Vue::Codex(fiche) => codex(partie, fiche, contenu),
        Vue::Gestion(choisi) => gestion(partie, choisi, contenu),
        Vue::Expeditions => ecran_expeditions_automatiques::liste(partie, contenu, maintenant),
        Vue::Preparation { zone, choisis } => {
            ecran_expeditions_automatiques::preparation(partie, contenu, zone, choisis, maintenant)
        }
        Vue::Marchand => ecran_marchand::etal(partie, contenu),
        Vue::Vente => ecran_marchand::vente(partie),
        Vue::Attaques(emplacement) => attaques(partie, emplacement, contenu),
        Vue::Coffre => ecran_coffre::coffre(partie),
        Vue::Options => options(),
        Vue::Forge => forge(partie, contenu, maintenant),
        Vue::Equipement => equipement(partie),
        Vue::Peche => ecran_peche::peche(partie, maintenant),
        // Le tableau des échanges vient des autres parties : le message le
        // prépare (`message::question`). Sans lui, l'écran n'en montre rien.
        Vue::Echanges { avec, cede } => {
            ecran_echanges::echanges(partie, &TableauDesEchanges::default(), avec, cede)
        }
        Vue::Classement => ecran_echanges::classement(&TableauDesEchanges::default()),
    }
}

pub fn retour() -> Bouton {
    bouton(Cle::Afficher(Vue::Camp), "Retour au camp", SECONDAIRE)
}

fn principale(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> Question {
    let mut equipe: Vec<String> = partie
        .collection()
        .equipe()
        .iter()
        .map(|monstre| ligne_monstre(monstre, Cote::Joueur))
        .collect();
    // Au camp, l'équipe peut être vide (section 13.2).
    if equipe.is_empty() {
        equipe.push("aucun monstre".to_string());
    }
    let zones = partie
        .zones_accessibles(contenu.zones(), contenu.piste())
        .into_iter()
        .map(|zone| {
            let (minimum, maximum) = zone.niveaux();
            let exploree = if partie.est_exploree(zone.id()) {
                ", explorée"
            } else {
                ""
            };
            OptionDeMenu {
                libelle: zone.nom().to_string(),
                valeur: zone.id().as_str().to_string(),
                description: Some(format!(
                    "Niveaux {} à {}{exploree}",
                    minimum.valeur(),
                    maximum.valeur()
                )),
            }
        })
        .collect();
    let mut lignes: Vec<Rangee> = menu(Cle::Partir, "Partir en expédition", zones)
        .into_iter()
        .collect();
    let mut boutons = vec![
        bouton(
            Cle::Afficher(Vue::Expeditions),
            "Expéditions automatiques",
            SECONDAIRE,
        ),
        bouton(
            Cle::Afficher(Vue::Gestion(None)),
            "Gérer vos monstres",
            SECONDAIRE,
        ),
        bouton(Cle::Afficher(Vue::Coffre), "Coffre du camp", SECONDAIRE),
        bouton(Cle::Afficher(Vue::Codex(None)), "Codex", SECONDAIRE),
    ];
    // Malvoise, Gunther et Ansgar n'apparaissent qu'une fois au camp
    // (sections 13.4, 13.8 et 14.2), et l'équipement avec la forge (section
    // 13.5) ; les autres joueurs sont toujours là (sections 15.1 et 15.3) :
    // au plus 10 boutons, sur deux rangées pleines.
    if partie.marchand_au_camp() {
        boutons.push(bouton(Cle::Afficher(Vue::Marchand), "Malvoise", SECONDAIRE));
    }
    if partie.forge().est_au_camp() {
        boutons.push(bouton(
            Cle::Afficher(Vue::Forge),
            "Forge de Gunther",
            SECONDAIRE,
        ));
    }
    if partie.forge().est_au_camp() || !partie.heros().equipement().possedes().is_empty() {
        boutons.push(bouton(
            Cle::Afficher(Vue::Equipement),
            "Équipement",
            SECONDAIRE,
        ));
    }
    if partie.peche().pecheur_au_camp() {
        boutons.push(bouton(Cle::Afficher(Vue::Peche), "Pêche", SECONDAIRE));
    }
    boutons.push(bouton(
        Cle::Afficher(Vue::Echanges {
            avec: None,
            cede: None,
        }),
        "Autres joueurs",
        SECONDAIRE,
    ));
    boutons.push(bouton(Cle::Afficher(Vue::Options), "Options", SECONDAIRE));
    lignes.extend(rangees(boutons));
    let mut en_tete = vec![en_tete_du_camp(partie)];
    en_tete.extend(ames_d_apotres(partie, contenu));
    en_tete.extend(expeditions_rentrees(partie, maintenant));
    en_tete.extend(travail_fini(partie, maintenant));
    en_tete.extend(ligne_prete(partie, maintenant));
    Question {
        texte: format!(
            "{}\nVotre équipe :\n{}",
            en_tete.join("\n"),
            equipe.join("\n")
        ),
        rangees: lignes,
    }
}

/// Un bouton par amélioration qui n'est pas au niveau maximal, et un menu
/// des objets que le héros ne possède pas, même faute de ressources : le
/// refus dit ce qui manque. Le déroulement fait confirmer le lancement
/// (sections 13.4 et 13.5).
fn forge(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> Question {
    if !partie.forge().est_au_camp() {
        return Question {
            texte: "Aucun artisan ne s'est encore installé au camp.".to_string(),
            rangees: rangees(vec![retour()]),
        };
    }
    let mut boutons = Vec::new();
    if partie
        .forge()
        .chantier()
        .is_some_and(|chantier| chantier.est_fini(maintenant))
    {
        boutons.push(bouton(Cle::Constater, "Constater le travail", SUCCES));
    }
    for amelioration in Amelioration::TOUTES {
        if partie.forge().ameliorations().niveau(amelioration) < amelioration.niveau_max() {
            let libelle = format!("Améliorer : {}", amelioration.nom());
            boutons.push(bouton(Cle::Ameliorer(amelioration), &libelle, PRINCIPAL));
        }
    }
    boutons.push(retour());
    let objets: Vec<OptionDeMenu> = contenu
        .plans_de_fabrication()
        .iter()
        .filter(|plan| !partie.heros().equipement().possede(plan.objet().id()))
        .map(|plan| OptionDeMenu {
            libelle: objet_et_bonus(plan.objet()),
            valeur: plan.objet().id().as_str().to_string(),
            description: Some(format!("{}, {}", cout(plan.cout()), duree(plan.duree()))),
        })
        .collect();
    let mut lignes: Vec<Rangee> = menu(Cle::Fabriquer, "Fabriquer un objet", objets)
        .into_iter()
        .collect();
    lignes.extend(rangees(boutons));
    Question {
        texte: etat_de_la_forge(partie, contenu, maintenant),
        rangees: lignes,
    }
}

/// Un menu par place : les objets de sa sorte portés nulle part, et de quoi
/// retirer celui qui y est. Sans confirmation (section 4.3).
fn equipement(partie: &Partie) -> Question {
    let equipement = partie.heros().equipement();
    let mut lignes: Vec<Rangee> = PlaceDEquipement::TOUTES
        .iter()
        .filter_map(|place| {
            let mut options: Vec<OptionDeMenu> = equipement
                .disponibles(*place)
                .into_iter()
                .map(|objet| OptionDeMenu {
                    libelle: format!("Porter : {}", objet.nom()),
                    valeur: objet.id().as_str().to_string(),
                    description: Some(decrire_bonus(&objet.bonus())),
                })
                .collect();
            if let Some(porte) = equipement.porte(*place) {
                options.push(OptionDeMenu {
                    libelle: format!("Retirer : {}", porte.nom()),
                    valeur: RETIRER.to_string(),
                    description: Some(objet_et_bonus(porte)),
                });
            }
            menu(Cle::Equiper(*place), nom_de_place(*place), options)
        })
        .collect();
    lignes.extend(rangees(vec![retour()]));
    Question {
        texte: etat_de_l_equipement(equipement),
        rangees: lignes,
    }
}

/// Le déroulement demande ensuite confirmation du suicide (section 13.9).
fn options() -> Question {
    Question {
        texte: "Options de la partie.".to_string(),
        rangees: rangees(vec![
            bouton(Cle::SeSuicider, "Se suicider", DANGER),
            retour(),
        ]),
    }
}

/// Une ligne par espèce ; la fiche d'une espèce capturée s'ouvre par le menu
/// (section 5.6).
/// Avec plus de 40 espèces, la liste et une fiche ne tiennent plus ensemble
/// dans un message : une fiche ouverte remplace la liste, qu'un bouton
/// rouvre.
fn codex(partie: &Partie, fiche: Option<usize>, contenu: &ContenuV1) -> Question {
    let codex = partie.codex();
    let especes = contenu.especes();
    let ouverte = fiche
        .and_then(|position| especes.get(position))
        .filter(|espece| codex.etat(espece.id()) == EtatCodex::Capturee);
    let mut lignes = Vec::new();
    match ouverte {
        Some(espece) => {
            lignes.push(espece.nom().to_string());
            lignes.extend(fiche_d_espece(espece, contenu));
        }
        None => {
            lignes.push("Codex :".to_string());
            for (position, espece) in especes.iter().enumerate() {
                lignes.push(format!(
                    "  {}. {}",
                    position + 1,
                    ligne_du_codex(codex, espece)
                ));
            }
        }
    }
    let capturees: Vec<OptionDeMenu> = especes
        .iter()
        .enumerate()
        .filter(|(_, espece)| codex.etat(espece.id()) == EtatCodex::Capturee)
        .map(|(position, espece)| OptionDeMenu {
            libelle: espece.nom().to_string(),
            valeur: position.to_string(),
            description: None,
        })
        .collect();
    // 25 options au plus par menu : avec les formes évoluées, le codex
    // dépasse 25 espèces (section 5.4).
    let mut rangees_du_codex: Vec<Rangee> = capturees
        .chunks(OPTIONS_PAR_MENU)
        .enumerate()
        .filter_map(|(numero, options)| {
            let indication = if numero == 0 {
                "Voir la fiche d'une espèce capturée".to_string()
            } else {
                format!("Voir la fiche d'une espèce capturée (suite {numero})")
            };
            let numero = u8::try_from(numero).ok()?;
            menu(Cle::Fiche(numero), &indication, options.to_vec())
        })
        .collect();
    let mut boutons = Vec::new();
    if ouverte.is_some() {
        boutons.push(bouton(
            Cle::Afficher(Vue::Codex(None)),
            "Liste du codex",
            SECONDAIRE,
        ));
    }
    boutons.push(retour());
    rangees_du_codex.extend(rangees(boutons));
    Question {
        texte: lignes.join("\n"),
        rangees: rangees_du_codex,
    }
}

/// La liste complète tient au-dessous de cette longueur ; au-delà, celle du
/// camp laisse la place au reste du message, ses monstres étant décrits dans
/// les menus.
const LONGUEUR_MAX_DE_LA_LISTE: usize = 1_400;

/// Équipe puis camp, numérotés à la suite. Avec l'enclos agrandi (section
/// 13.4), un camp nombreux ne se résume qu'à son compte.
pub fn liste_des_monstres(collection: &Collection) -> String {
    let complete = liste_complete(collection);
    if complete.chars().count() <= LONGUEUR_MAX_DE_LA_LISTE {
        return complete;
    }
    let equipe = collection.equipe();
    let mut lignes = vec![format!(
        "Équipe ({}/{}) :",
        equipe.len(),
        collection.tailles().equipe
    )];
    for (index, monstre) in equipe.iter().enumerate() {
        lignes.push(format!(
            "  {}. {}",
            index + 1,
            ligne_monstre(monstre, Cote::Joueur)
        ));
    }
    lignes.push(format!(
        "Camp ({}/{}) : les monstres {} à {}, décrits dans les menus.",
        collection.camp().len(),
        collection.tailles().camp,
        equipe.len() + 1,
        equipe.len() + collection.camp().len()
    ));
    lignes.join("\n")
}

/// Pour un monstre reçu d'un autre joueur, sa fiche dit aussi de qui il
/// vient et jusqu'à quel niveau il obéit (section 15.2).
fn fiche(partie: &Partie, monstre: &Monstre, contenu: &ContenuV1) -> String {
    let limite = partie.limite_d_obeissance(contenu.zones(), contenu.piste());
    let mut fiche = fiche_du_monstre(monstre);
    if let Some(provenance) = provenance(monstre, limite) {
        fiche.push('\n');
        fiche.push_str(&provenance);
    }
    fiche
}

/// Des menus pour choisir un monstre, puis les actions possibles pour le
/// monstre choisi.
fn gestion(partie: &Partie, choisi: Option<Emplacement>, contenu: &ContenuV1) -> Question {
    let collection = partie.collection();
    let liste = liste_des_monstres(collection);
    let choisi = choisi.and_then(|emplacement| {
        collection
            .monstre(emplacement)
            .map(|monstre| (emplacement, monstre))
    });
    let Some((emplacement, monstre)) = choisi else {
        let mut lignes = menus_de_monstres(collection, Cle::Choisir, "Choisir", |_| true);
        // Le tri change l'ordre du camp pour de bon (section 13.2).
        let mut boutons = Vec::new();
        if collection.camp().len() > 1 {
            boutons.extend([
                bouton(
                    Cle::Trier(TriDuCamp::Nom),
                    "Trier le camp par nom",
                    SECONDAIRE,
                ),
                bouton(
                    Cle::Trier(TriDuCamp::Niveau),
                    "Trier par niveau",
                    SECONDAIRE,
                ),
                bouton(Cle::Trier(TriDuCamp::Type), "Trier par type", SECONDAIRE),
            ]);
        }
        boutons.push(retour());
        lignes.extend(rangees(boutons));
        return Question {
            texte: format!("{liste}\n\nChoisissez un monstre à gérer."),
            rangees: lignes,
        };
    };
    let mut boutons = Vec::new();
    let mut echange = None;
    match emplacement {
        Emplacement::Equipe(index) => {
            if index > 0 {
                boutons.push(bouton(Cle::Monter(index), "Monter", SECONDAIRE));
            }
            if index + 1 < collection.equipe().len() {
                boutons.push(bouton(Cle::Descendre(index), "Descendre", SECONDAIRE));
            }
            boutons.push(bouton(
                Cle::EnvoyerAuCamp(index),
                "Envoyer au camp",
                PRINCIPAL,
            ));
        }
        Emplacement::Camp(index) => {
            boutons.push(bouton(
                Cle::FaireVenir(index),
                "Faire venir dans l'équipe",
                PRINCIPAL,
            ));
            echange = menu(
                Cle::EchangerAvec(index),
                "Échanger avec un monstre de l'équipe",
                options_d_equipe(collection.equipe()),
            );
        }
    }
    boutons.push(bouton(Cle::Renommer(emplacement), "Renommer", SECONDAIRE));
    boutons.push(bouton(Cle::Relacher(emplacement), "Relâcher", DANGER));
    boutons.push(bouton(
        Cle::Afficher(Vue::Attaques(emplacement)),
        "Attaques",
        SECONDAIRE,
    ));
    if partie.marchand_au_camp() {
        let libelle = format!("Vendre ({})", argent(prix_de_revente(monstre)));
        boutons.push(bouton(Cle::VendreCeMonstre(emplacement), &libelle, DANGER));
    }
    // Au plus 10 boutons : ils tiennent en deux rangées (sections 5.4, 5.9
    // et 13.7).
    if partie.marchand_au_camp() && partie.eclats_d_astre() > 0 && !monstre.est_shiny() {
        boutons.push(bouton(
            Cle::RendreShiny(emplacement),
            "Rendre shiny (Malvoise)",
            PRINCIPAL,
        ));
    }
    if partie.capsules_d_habilete() > 0 {
        if let Some(nouvelle) = autre_habilete(monstre) {
            let libelle = format!("Passer à {}", nouvelle.nom());
            boutons.push(bouton(
                Cle::ChangerDHabilete(emplacement),
                &libelle,
                SECONDAIRE,
            ));
        }
    }
    // Par niveau, ou par une pierre que le joueur a (section 5.4).
    if let Ok((_, vers, _)) = partie.estimer_evolution(emplacement, contenu) {
        let libelle = format!("Évoluer en {}", vers.nom());
        boutons.push(bouton(Cle::FaireEvoluer(emplacement), &libelle, PRINCIPAL));
    }
    let mut lignes = rangees(boutons);
    lignes.extend(echange);
    lignes.extend(menu(
        Cle::DonnerBooster(emplacement),
        "Donner un booster",
        options_de_boosters(partie, monstre),
    ));
    lignes.extend(rangees(vec![
        bouton(
            Cle::Afficher(Vue::Gestion(None)),
            "Choisir un autre monstre",
            SECONDAIRE,
        ),
        retour(),
    ]));
    Question {
        texte: format!("Monstre choisi :\n{}", fiche(partie, monstre, contenu)),
        rangees: lignes,
    }
}

/// Les boosters du sac et du coffre qui servent encore à ce monstre : 6 au
/// plus (section 14.3).
fn options_de_boosters(partie: &Partie, monstre: &Monstre) -> Vec<OptionDeMenu> {
    Booster::TOUS
        .into_iter()
        .filter(|booster| {
            *monstre.valeurs_internes().valeurs().de(booster.stat()) < ValeursInternes::MAX
        })
        .filter_map(|booster| {
            let nombre = partie.nombre_de_boosters(booster);
            (nombre > 0).then(|| OptionDeMenu {
                libelle: boosters(booster, nombre),
                valeur: booster_en_texte(booster).to_string(),
                description: Some(effet_du_booster(booster)),
            })
        })
        .collect()
}

/// Les monstres de l'équipe, puis ceux du camp, numérotés à la suite et 25
/// au plus par menu : avec l'enclos agrandi, l'équipe et 60 monstres au camp
/// tiennent en trois menus (section 13.4). Les options portent le numéro de
/// la liste des monstres ; seuls les monstres que `garder` accepte y
/// figurent.
pub fn menus_de_monstres(
    collection: &Collection,
    cle: impl Fn(u8) -> Cle,
    verbe: &str,
    garder: impl Fn(Emplacement) -> bool,
) -> Vec<Rangee> {
    menus_de_monstres_decrits(collection, cle, verbe, garder, |monstre| {
        ligne_monstre(monstre, Cote::Joueur)
    })
}

/// Comme `menus_de_monstres`, avec la description de chaque option, comme
/// le prix de rachat.
pub fn menus_de_monstres_decrits(
    collection: &Collection,
    cle: impl Fn(u8) -> Cle,
    verbe: &str,
    garder: impl Fn(Emplacement) -> bool,
    decrire: impl Fn(&Monstre) -> String,
) -> Vec<Rangee> {
    let taille_equipe = collection.equipe().len();
    let equipe = collection
        .equipe()
        .iter()
        .enumerate()
        .map(|(index, monstre)| (Emplacement::Equipe(index), index + 1, monstre));
    let camp = collection
        .camp()
        .iter()
        .enumerate()
        .map(|(index, monstre)| (Emplacement::Camp(index), taille_equipe + index + 1, monstre));
    let gardes: Vec<(Emplacement, usize, &Monstre)> = equipe
        .chain(camp)
        .filter(|(emplacement, _, _)| garder(*emplacement))
        .collect();
    let plusieurs = gardes.len() > OPTIONS_PAR_MENU;
    gardes
        .chunks(OPTIONS_PAR_MENU)
        .enumerate()
        .filter_map(|(numero, morceau)| {
            let dans_l_equipe = morceau
                .iter()
                .filter(|(emplacement, _, _)| matches!(emplacement, Emplacement::Equipe(_)))
                .count();
            let provenance = if dans_l_equipe == morceau.len() {
                "de l'équipe"
            } else if dans_l_equipe > 0 {
                "de l'équipe ou du camp"
            } else {
                "du camp"
            };
            let mut indication = format!("{verbe} un monstre {provenance}");
            if plusieurs {
                let premier = morceau.first().map_or(0, |(_, numero, _)| *numero);
                let dernier = morceau.last().map_or(0, |(_, numero, _)| *numero);
                indication.push_str(&format!(" ({premier} à {dernier})"));
            }
            let options = morceau
                .iter()
                .map(|(emplacement, numero, monstre)| OptionDeMenu {
                    libelle: format!("{numero}. {}", nom(monstre, Cote::Joueur)),
                    valeur: emplacement_en_texte(*emplacement),
                    description: Some(decrire(monstre)),
                })
                .collect();
            menu(cle(u8::try_from(numero).ok()?), &indication, options)
        })
        .collect()
}

/// Les attaques connues, puis un menu des attaques à apprendre ; s'il en
/// connaît déjà 4, un menu par attaque à oublier (section 13.10).
fn attaques(partie: &Partie, emplacement: Emplacement, contenu: &ContenuV1) -> Question {
    let Some(monstre) = partie.collection().monstre(emplacement) else {
        return gestion(partie, None, contenu);
    };
    let a_apprendre = partie.attaques_a_apprendre(emplacement).unwrap_or_default();
    let options = || -> Vec<OptionDeMenu> {
        a_apprendre
            .iter()
            .map(|attaque| OptionDeMenu {
                libelle: attaque.nom().to_string(),
                valeur: attaque.id().as_str().to_string(),
                description: Some(decrire_attaque(attaque)),
            })
            .collect()
    };
    let mut texte = format!(
        "{}\n{}",
        fiche(partie, monstre, contenu),
        attaques_connues(monstre)
    );
    let mut lignes: Vec<Rangee> = Vec::new();
    if a_apprendre.is_empty() {
        texte.push_str("\nRien de nouveau à apprendre au camp.");
    } else if monstre.attaques().est_complete() {
        texte.push_str(
            "\nIl connaît déjà 4 attaques : choisissez la nouvelle dans le menu de celle à oublier.",
        );
        for (position, connue) in monstre.attaques().liste().iter().enumerate() {
            let cle = Cle::Apprendre {
                emplacement,
                oubliee: Some(position),
            };
            let indication = format!("Oublier {} pour apprendre", connue.nom());
            lignes.extend(menu(cle, &indication, options()));
        }
    } else {
        let cle = Cle::Apprendre {
            emplacement,
            oubliee: None,
        };
        lignes.extend(menu(cle, "Apprendre une attaque", options()));
    }
    lignes.extend(rangees(vec![
        bouton(
            Cle::Afficher(Vue::Gestion(Some(emplacement))),
            "Retour au monstre",
            SECONDAIRE,
        ),
        retour(),
    ]));
    Question {
        texte,
        rangees: lignes,
    }
}

/// Les monstres de l'équipe, désignés par leur position.
pub fn options_d_equipe(equipe: &[Monstre]) -> Vec<OptionDeMenu> {
    equipe
        .iter()
        .enumerate()
        .map(|(index, monstre)| OptionDeMenu {
            libelle: nom(monstre, Cote::Joueur),
            valeur: index.to_string(),
            description: Some(ligne_monstre(monstre, Cote::Joueur)),
        })
        .collect()
}
