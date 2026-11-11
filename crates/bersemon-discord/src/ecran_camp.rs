//! Camp de base sur Discord : départ en expédition par la carte, expéditions
//! automatiques, codex, gestion des monstres et de leurs attaques (sections
//! 13.2 et 13.10), coffre du camp (section 13.3), marchand, options (section
//! 13.9), forge de Gunther, équipement du héros (sections 4.3, 13.4 et
//! 13.5), pêche d'Ansgar (section 13.8) et quêtes (section 13.12). Les vues
//! ne changent rien à la partie ; seules les actions la modifient, et la
//! sauvegardent.

use crate::composants::objet_d_experience_en_valeur;
use bersemon_core::amelioration::Amelioration;
use bersemon_core::booster::Booster;
use bersemon_core::codex::EtatCodex;
use bersemon_core::collection::{Collection, Emplacement, TriDuCamp};
use bersemon_core::combat::Cote;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::equipement::PlaceDEquipement;
use bersemon_core::espece::Espece;
use bersemon_core::habilete::autre_habilete;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::marchand::prix_de_revente;
use bersemon_core::monstre::Monstre;
use bersemon_core::objet_d_experience::ObjetDExperience;
use bersemon_core::partie::Partie;
use bersemon_core::quete_du_camp::{EtatDeLaChaine, Personnage};
use bersemon_core::recompenses_du_codex::compte_au_codex;
use bersemon_core::stats::ValeursInternes;
use bersemon_presentation::arene::description_de_la_fosse;
use bersemon_presentation::beherit::description_de_l_eclipse;
use bersemon_presentation::boosters::{boosters, effet_du_booster};
use bersemon_presentation::equipement::{
    decrire_bonus, etat_de_l_equipement, fiche_du_heros, nom_de_place, objet_et_bonus,
};
use bersemon_presentation::expeditions_automatiques::duree;
use bersemon_presentation::expeditions_automatiques::expeditions_rentrees;
use bersemon_presentation::forge::{cout, etat_de_la_forge, lot_de_filets, travail_fini};
use bersemon_presentation::jour_et_nuit::moment_present;
use bersemon_presentation::journal::{
    LieuDuJournal, lieux_du_journal, lignes_du_lieu, resume_du_lieu, zones_d_apparition,
};
use bersemon_presentation::lieux_a_part::{description_du_lieu, lieux_a_part};
use bersemon_presentation::machines::{
    avis_des_pieges, etat_de_la_cour, machines as texte_des_machines,
};
use bersemon_presentation::meteo::meteo_de_la_zone;
use bersemon_presentation::monstres::{
    attaques_connues, decrire_attaque_de, fiche_du_monstre, ligne_monstre, nom, provenance,
};
use bersemon_presentation::partie::{
    ames_d_apotres, argent, carnet_de_traque, collection as liste_complete, en_tete_abrege_du_camp,
    especes_du_codex, fiche_d_espece, ligne_du_codex,
};
use bersemon_presentation::peche::ligne_prete;
use bersemon_presentation::quetes::ecran_des_quetes;
use bersemon_presentation::quetes_du_camp::{
    avis_des_quetes_du_camp, chaines, ecran_des_quetes_du_camp, peut_remettre,
};
use bersemon_presentation::recit::{effet_d_un_objet_d_experience, objets_d_experience};
use bersemon_presentation::recompenses_du_codex::{progression_du_codex, reclamer};
use bersemon_presentation::rite::{rite_acheve, service_du_rite};
use bersemon_presentation::succes::{
    bilan_des_succes, couleur_du_heros, lignes_des_succes, titre_du_heros, trophees_du_camp,
};
use bersemon_presentation::tour::{depart_par_le_raccourci, description_de_la_tour};

use crate::composants::{
    AUCUN, Cle, ECLIPSE, FOSSE, RACCOURCI, RETIRER, Vue, booster_en_texte, emplacement_en_texte,
};
use crate::echanges::TableauDesEchanges;
use crate::message::{
    Bouton, DANGER, LONGUEUR_MAX, OPTIONS_PAR_MENU, OptionDeMenu, PRINCIPAL, Question, Rangee,
    SECONDAIRE, SUCCES, bouton, menu, rangees,
};
use crate::{
    ecran_coffre, ecran_echanges, ecran_eclipse, ecran_expeditions_automatiques, ecran_marchand,
    ecran_peche,
};

pub fn question(
    partie: &Partie,
    vue: Vue,
    contenu: &ContenuV1,
    maintenant: Horodatage,
) -> Question {
    match vue {
        Vue::Camp => principale(partie, contenu, maintenant),
        Vue::Codex(fiche) => codex(partie, fiche, 0, contenu),
        Vue::PageDuCodex(page) => codex(partie, None, usize::from(page), contenu),
        Vue::Journal(lieu) => journal(partie, usize::from(lieu), contenu),
        Vue::Gestion(choisi) => gestion(partie, choisi, contenu),
        Vue::Expeditions => ecran_expeditions_automatiques::liste(partie, contenu, maintenant),
        Vue::Preparation { zone, choisis } => {
            ecran_expeditions_automatiques::preparation(partie, contenu, zone, choisis, maintenant)
        }
        Vue::Marchand => ecran_marchand::etal(partie, contenu, maintenant),
        Vue::Vente => ecran_marchand::vente(partie),
        Vue::Achats => ecran_marchand::achats(partie, contenu),
        Vue::Capsules => ecran_marchand::capsules(partie, contenu),
        Vue::Eclipse => ecran_eclipse::eclipse(partie, contenu),
        Vue::Attaques(emplacement) => attaques(partie, emplacement, contenu),
        Vue::Coffre => ecran_coffre::coffre(partie),
        Vue::Ressources => ecran_coffre::ressources(partie),
        Vue::Heros => heros(partie, contenu),
        Vue::Succes(page) => succes(partie, page, contenu),
        Vue::Trophees => trophees(partie, contenu),
        Vue::QuetesDuCamp => quetes_du_camp(partie, contenu),
        Vue::Forge => forge(partie, contenu, maintenant),
        Vue::Equipement => equipement(partie),
        Vue::Peche => ecran_peche::peche(partie, contenu, maintenant),
        Vue::Quetes => quetes(partie, contenu, maintenant),
        // Le tableau des échanges vient des autres parties : le message le
        // prépare (`message::question`). Sans lui, l'écran n'en montre rien.
        Vue::Echanges {
            avec,
            cede,
            demande,
        } => ecran_echanges::echanges(
            partie,
            &TableauDesEchanges::default(),
            avec,
            (cede, demande),
            contenu,
        ),
        Vue::Classement => ecran_echanges::classement(&TableauDesEchanges::default()),
        Vue::RecordsDeLaTour => {
            ecran_echanges::records_de_la_tour(partie, &TableauDesEchanges::default())
        }
        Vue::ClassementDeLArene => {
            ecran_echanges::classement_de_l_arene(partie, &TableauDesEchanges::default())
        }
        Vue::Machines => machines(partie, contenu, maintenant),
        Vue::Entrainement => entrainement(partie, Vue::Machines),
        Vue::Cour => cour(partie, contenu, maintenant),
        Vue::Rite(premier) => rite(partie, premier, contenu),
        Vue::EntrainementDepuisLaCour => entrainement(partie, Vue::Cour),
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
    let mut zones: Vec<OptionDeMenu> = partie
        .zones_accessibles(contenu.zones(), contenu.piste())
        .into_iter()
        .map(|zone| {
            let (minimum, maximum) = zone.niveaux();
            let exploree = if partie.est_exploree(zone.id()) {
                ", explorée"
            } else {
                ""
            };
            // La météo du départ est celle de toute l'expédition (section
            // 9.7).
            let meteo = meteo_de_la_zone(zone, maintenant)
                .map(|meteo| format!(" ; {meteo}"))
                .unwrap_or_default();
            OptionDeMenu {
                libelle: zone.nom().to_string(),
                valeur: zone.id().as_str().to_string(),
                description: Some(format!(
                    "Niveaux {} à {}{exploree}{meteo}",
                    minimum.valeur(),
                    maximum.valeur()
                )),
            }
        })
        .collect();
    // La Tour sans fond suit les zones, une fois ouverte (section 9.5),
    // puis l'Éclipse, avec le Béhérit (section 9.4).
    if let Some(description) = description_de_la_tour(partie, contenu) {
        zones.push(OptionDeMenu {
            libelle: contenu.tour().nom().to_string(),
            valeur: contenu.tour().id().as_str().to_string(),
            description: Some(description),
        });
    }
    if let Some((libelle, reprise)) = depart_par_le_raccourci(partie, contenu) {
        zones.push(OptionDeMenu {
            libelle,
            valeur: RACCOURCI.to_string(),
            description: Some(reprise),
        });
    }
    // La Fosse aux damnés suit la Tour, une fois le brigand vaincu (section
    // 9.8).
    if partie.fosse_ouverte(contenu.fosse()) {
        zones.push(OptionDeMenu {
            libelle: "Fosse aux damnés".to_string(),
            valeur: FOSSE.to_string(),
            description: Some(description_de_la_fosse(partie.arene().paliers_vaincus)),
        });
    }
    if let Some(description) = description_de_l_eclipse(partie, contenu) {
        zones.push(OptionDeMenu {
            libelle: contenu.invocation().eclipse().nom().to_string(),
            valeur: ECLIPSE.to_string(),
            description: Some(description),
        });
    }
    let mut lignes: Vec<Rangee> = menu(Cle::Partir, "Partir en expédition", zones)
        .into_iter()
        .collect();
    // Les annexes et les zones légendaires ont leur menu : avec les 16 zones
    // de la traque, la Tour, la Fosse et l'Éclipse, celui des expéditions
    // n'aurait plus la place (Q118).
    let lieux = lieux_a_part(partie, contenu)
        .iter()
        .map(|lieu| OptionDeMenu {
            libelle: lieu.zone.nom().to_string(),
            valeur: lieu.zone.id().as_str().to_string(),
            description: Some(description_du_lieu(lieu, maintenant)),
        })
        .collect();
    lignes.extend(menu(
        Cle::PartirAilleurs,
        "Partir vers un lieu à part",
        lieux,
    ));
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
    // (sections 13.4, 13.8 et 14.2) ; le héros, les autres joueurs et les
    // quêtes sont toujours là (sections 4.5, 13.12, 15.1 et 15.3), les options
    // ayant rejoint l'écran du héros : au plus 11 boutons avec celui du rite,
    // sur trois rangées. Avec les deux menus de départ, le camp a ses 5
    // rangées.
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
    // Le héros, toujours là, mène à son équipement, à ses succès, au
    // service shiny et au suicide (section 4.5).
    boutons.push(bouton(Cle::Afficher(Vue::Heros), "Héros", SECONDAIRE));
    if partie.peche().pecheur_au_camp() {
        boutons.push(bouton(Cle::Afficher(Vue::Peche), "Pêche", SECONDAIRE));
    }
    boutons.push(bouton(Cle::Afficher(Vue::Quetes), "Quêtes", SECONDAIRE));
    boutons.push(bouton(
        Cle::Afficher(Vue::Echanges {
            avec: None,
            cede: None,
            demande: None,
        }),
        "Multijoueur",
        SECONDAIRE,
    ));
    // Le temps que le petit du rite attende (section 13.11), à la suite des
    // autres boutons.
    let rite_acheve = rite_acheve(partie, maintenant);
    if rite_acheve.is_some() {
        boutons.push(bouton(
            Cle::AccueillirLePetit,
            "Accueillir le petit (Malvoise)",
            SUCCES,
        ));
    }
    lignes.extend(rangees(boutons));
    // Le menu des expéditions est sur cet écran : le moment du départ est
    // celui de toute l'expédition (section 9.6).
    let mut en_tete = vec![
        en_tete_abrege_du_camp(partie, contenu.succes()),
        moment_present(maintenant),
    ];
    en_tete.extend(ames_d_apotres(partie, contenu));
    en_tete.extend(expeditions_rentrees(partie, maintenant));
    en_tete.extend(travail_fini(partie, maintenant));
    en_tete.extend(ligne_prete(partie, maintenant));
    en_tete.extend(avis_des_pieges(partie, maintenant).map(str::to_string));
    en_tete.extend(rite_acheve);
    en_tete.extend(avis_des_quetes_du_camp(partie, contenu));
    Question {
        texte: format!(
            "{}\nVotre équipe :\n{}",
            en_tete.join("\n"),
            equipe.join("\n")
        ),
        rangees: lignes,
    }
}

/// Un bouton par amélioration qui n'est pas au niveau maximal, un menu des
/// objets que le héros ne possède pas et un menu des lots de filets, même
/// faute de ressources : le refus dit ce qui manque. Le déroulement fait
/// confirmer le lancement (sections 13.4 et 13.5).
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
    // Aussi souvent que le joueur le veut (section 13.5). Au plus 9
    // boutons : ils tiennent en deux rangées.
    let charme = contenu.plan_du_charme();
    let libelle = format!(
        "Charme de garde ({}, {})",
        cout(charme.cout()),
        duree(charme.duree())
    );
    boutons.push(bouton(Cle::FabriquerCharme, &libelle, PRINCIPAL));
    let ameliorations = partie.forge().ameliorations();
    if ameliorations.pieges().is_some() || ameliorations.cour().is_some() {
        boutons.push(bouton(Cle::Afficher(Vue::Machines), "Machines", PRINCIPAL));
    }
    boutons.push(retour());
    let objets: Vec<OptionDeMenu> = contenu
        .plans_de_fabrication_pour(partie.heros().classe())
        .into_iter()
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
    // Les boutons de la forge occupent déjà deux rangées : les filets ont
    // leur menu, avec ce que le joueur en a déjà.
    let reserve = partie.filets();
    let filets: Vec<OptionDeMenu> = contenu
        .plans_des_filets()
        .iter()
        .map(|plan| OptionDeMenu {
            libelle: lot_de_filets(plan),
            valeur: plan.filet().id().to_string(),
            description: Some(format!(
                "{}, {} ; vous en avez {}",
                cout(plan.cout()),
                duree(plan.duree()),
                reserve.nombre(plan.filet())
            )),
        })
        .collect();
    lignes.extend(menu(Cle::FabriquerFilets, "Fabriquer des filets", filets));
    lignes.extend(rangees(boutons));
    Question {
        texte: etat_de_la_forge(partie, contenu, maintenant),
        rangees: lignes,
    }
}

/// Les machines de Gunther (section 13.6) : un bouton pour relever les
/// pièges, un pour mettre un monstre à l'entraînement, un pour reprendre
/// chaque monstre de la cour (au plus 5 boutons), et un menu des autres
/// zones explorées où poser les pièges.
fn machines(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> Question {
    let plafond = partie.plafond_d_entrainement(contenu.zones(), contenu.piste());
    let ameliorations = partie.forge().ameliorations();
    let mut lignes: Vec<Rangee> = Vec::new();
    let mut boutons = Vec::new();
    if let Some(regles) = ameliorations.pieges() {
        let pieges = partie.machines().pieges();
        if pieges.is_some_and(|pieges| pieges.stock(&regles, maintenant) > 0) {
            boutons.push(bouton(Cle::ReleverLesPieges, "Relever les pièges", SUCCES));
        }
        let zones: Vec<OptionDeMenu> = contenu
            .zones()
            .iter()
            .filter(|zone| partie.est_exploree(zone.id()))
            .filter(|zone| pieges.is_none_or(|pieges| pieges.zone() != zone.id()))
            .map(|zone| OptionDeMenu {
                libelle: zone.nom().to_string(),
                valeur: zone.id().as_str().to_string(),
                description: None,
            })
            .collect();
        lignes.extend(menu(
            Cle::PoserLesPieges,
            "Poser les pièges dans une zone explorée",
            zones,
        ));
    }
    boutons.extend(boutons_de_la_cour(partie, Vue::Entrainement));
    lignes.extend(rangees(boutons));
    lignes.extend(rangees(vec![
        bouton(Cle::Afficher(Vue::Forge), "Retour à la forge", SECONDAIRE),
        retour(),
    ]));
    Question {
        texte: texte_des_machines(partie, contenu.zones(), plafond, maintenant),
        rangees: lignes,
    }
}

/// Le second parent du Rite du sang mêlé, parmi les monstres de la même
/// famille que le premier, en menus numérotés ; le déroulement fait
/// confirmer (section 13.11).
fn rite(partie: &Partie, premier: Emplacement, contenu: &ContenuV1) -> Question {
    let collection = partie.collection();
    let retours = vec![
        bouton(
            Cle::Afficher(Vue::Gestion(Some(premier))),
            "Retour au monstre",
            SECONDAIRE,
        ),
        retour(),
    ];
    // Un bouton d'un ancien message peut désigner une place vide.
    let Some(monstre) = collection.monstre(premier) else {
        return Question {
            texte: "Choisissez d'abord un monstre dans la gestion.".to_string(),
            rangees: rangees(retours),
        };
    };
    let possibles = partie.seconds_parents_possibles(premier, contenu.especes());
    let mut lignes = menus_de_monstres(
        collection,
        |menu| Cle::SecondParent { menu, premier },
        "Lui unir",
        |emplacement| possibles.contains(&emplacement),
    );
    lignes.extend(rangees(retours));
    Question {
        texte: format!(
            "{}\nChoisissez le second parent, de la même famille d'évolution que {}.",
            service_du_rite(),
            nom(monstre, Cote::Joueur)
        ),
        rangees: lignes,
    }
}

/// "Mettre un monstre à l'entraînement" s'il reste une place, qui ouvre la
/// vue `choix_du_monstre`, puis "Reprendre" pour chaque monstre de la cour :
/// 4 boutons au plus. Rien si la cour n'est pas construite.
fn boutons_de_la_cour(partie: &Partie, choix_du_monstre: Vue) -> Vec<Bouton> {
    let mut boutons = Vec::new();
    let Some(regles) = partie.forge().ameliorations().cour() else {
        return boutons;
    };
    let cour = partie.machines().cour();
    if cour.len() < regles.places {
        boutons.push(bouton(
            Cle::Afficher(choix_du_monstre),
            "Mettre un monstre à l'entraînement",
            PRINCIPAL,
        ));
    }
    for (position, entrainement) in cour.iter().enumerate() {
        let libelle = format!("Reprendre {}", nom(entrainement.monstre(), Cote::Joueur));
        boutons.push(bouton(Cle::Reprendre(position), &libelle, SECONDAIRE));
    }
    boutons
}

/// La cour d'entraînement seule, depuis la gestion des monstres : un accès
/// direct, en plus de celui des machines de la forge (section 13.6).
fn cour(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> Question {
    let plafond = partie.plafond_d_entrainement(contenu.zones(), contenu.piste());
    let retours = vec![
        bouton(
            Cle::Afficher(Vue::Gestion(None)),
            "Retour à la gestion",
            SECONDAIRE,
        ),
        retour(),
    ];
    // Un bouton d'un ancien message peut mener ici avant la construction.
    let Some(texte) = etat_de_la_cour(partie, plafond, maintenant) else {
        return Question {
            texte: "Gunther n'a pas encore construit de cour d'entraînement.".to_string(),
            rangees: rangees(retours),
        };
    };
    let mut lignes = rangees(boutons_de_la_cour(partie, Vue::EntrainementDepuisLaCour));
    lignes.extend(rangees(retours));
    Question {
        texte,
        rangees: lignes,
    }
}

/// Les monstres de l'équipe et du camp, à mettre à l'entraînement ; le
/// bouton de retour mène à la vue d'où le joueur vient, les machines ou la
/// cour.
fn entrainement(partie: &Partie, depuis: Vue) -> Question {
    let mut lignes = menus_de_monstres(partie.collection(), Cle::Entrainer, "Entraîner", |_| true);
    let libelle = if depuis == Vue::Cour {
        "Retour à la cour"
    } else {
        "Retour aux machines"
    };
    lignes.extend(rangees(vec![
        bouton(Cle::Afficher(depuis), libelle, SECONDAIRE),
        retour(),
    ]));
    Question {
        texte: format!(
            "{}\n\nChoisissez le monstre à mettre à l'entraînement.",
            liste_des_monstres(partie.collection())
        ),
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
    lignes.extend(rangees(vec![retour_au_heros()]));
    Question {
        texte: etat_de_l_equipement(equipement),
        rangees: lignes,
    }
}

fn retour_au_heros() -> Bouton {
    bouton(Cle::Afficher(Vue::Heros), "Retour au héros", SECONDAIRE)
}

/// Sa fiche et son sprite ; l'équipement, une fois Gunther au camp ou un
/// objet possédé ; ses succès ; le service shiny de Malvoise, comme celui
/// d'un monstre depuis sa gestion (section 13.7) ; et le suicide, que le
/// déroulement fait confirmer (section 13.9).
fn heros(partie: &Partie, contenu: &ContenuV1) -> Question {
    let mut boutons = Vec::new();
    if partie.forge().est_au_camp() || !partie.heros().equipement().possedes().is_empty() {
        boutons.push(bouton(
            Cle::Afficher(Vue::Equipement),
            "Équipement",
            PRINCIPAL,
        ));
    }
    boutons.push(bouton(Cle::Afficher(Vue::Succes(0)), "Succès", PRINCIPAL));
    if partie.verifier_shiny_du_heros().is_ok() {
        boutons.push(bouton(
            Cle::RendreLeHerosShiny,
            "Rendre shiny (Malvoise)",
            PRINCIPAL,
        ));
    }
    boutons.push(bouton(Cle::SeSuicider, "Se suicider", DANGER));
    boutons.push(retour());
    Question {
        texte: format!(
            "{}\n{} {}\n{}",
            fiche_du_heros(partie.heros()),
            titre_du_heros(partie.heros(), contenu.succes()),
            couleur_du_heros(partie.heros()),
            bilan_des_succes(partie, contenu.succes())
        ),
        rangees: rangees(boutons),
    }
}

/// Assez peu pour qu'une page tienne dans un message, avec les menus des
/// titres et des couleurs.
const SUCCES_PAR_PAGE: usize = 18;

/// Une page de la liste des succès, avec un menu des titres des succès
/// obtenus, par 25, et un menu des couleurs débloquées (section 4.5).
fn succes(partie: &Partie, page: u8, contenu: &ContenuV1) -> Question {
    let tous = contenu.succes();
    let lignes_des_succes = lignes_des_succes(partie, tous);
    let pages = lignes_des_succes.len().div_ceil(SUCCES_PAR_PAGE).max(1);
    let page = usize::from(page).min(pages - 1);
    let debut = page * SUCCES_PAR_PAGE;
    let fin = (debut + SUCCES_PAR_PAGE).min(lignes_des_succes.len());
    let mut lignes: Vec<Rangee> = Vec::new();
    let mut titres: Vec<OptionDeMenu> = tous
        .iter()
        .filter(|un| partie.succes_obtenus().contains(un.id()))
        .map(|un| OptionDeMenu {
            libelle: un.titre().to_string(),
            valeur: un.id().as_str().to_string(),
            description: Some(un.description().to_string()),
        })
        .collect();
    if !titres.is_empty() {
        titres.insert(
            0,
            OptionDeMenu {
                libelle: "Ne porter aucun titre".to_string(),
                valeur: AUCUN.to_string(),
                description: None,
            },
        );
    }
    for (numero, morceau) in titres.chunks(OPTIONS_PAR_MENU).enumerate() {
        let numero = u8::try_from(numero).unwrap_or(u8::MAX);
        lignes.extend(menu(
            Cle::PorterTitre(numero),
            "Porter un titre",
            morceau.to_vec(),
        ));
    }
    let couleurs = partie.couleurs_debloquees(tous);
    if !couleurs.is_empty() {
        let mut options = vec![OptionDeMenu {
            libelle: "Couleur d'origine".to_string(),
            valeur: AUCUN.to_string(),
            description: None,
        }];
        options.extend(couleurs.into_iter().map(|couleur| OptionDeMenu {
            libelle: couleur.nom().to_string(),
            valeur: couleur.identifiant().to_string(),
            description: None,
        }));
        lignes.extend(menu(Cle::PorterCouleur, "Couleur du héros", options));
    }
    let mut boutons = Vec::new();
    if page > 0 {
        let precedente = u8::try_from(page - 1).unwrap_or(0);
        boutons.push(bouton(
            Cle::Afficher(Vue::Succes(precedente)),
            "Page précédente",
            SECONDAIRE,
        ));
    }
    if page + 1 < pages {
        let suivante = u8::try_from(page + 1).unwrap_or(u8::MAX);
        boutons.push(bouton(
            Cle::Afficher(Vue::Succes(suivante)),
            "Page suivante",
            SECONDAIRE,
        ));
    }
    boutons.push(bouton(
        Cle::Afficher(Vue::Trophees),
        "Trophées du camp",
        SECONDAIRE,
    ));
    boutons.push(retour_au_heros());
    lignes.extend(rangees(boutons));
    Question {
        texte: format!(
            "{} (page {}/{pages})\n{}",
            bilan_des_succes(partie, tous),
            page + 1,
            lignes_des_succes[debut..fin].join("\n")
        ),
        rangees: lignes,
    }
}

/// Rien à réclamer : une quête accomplie paie d'elle-même (section 13.12). Les quêtes des
/// personnages du camp ont leur écran, une fois l'un d'eux au camp (section
/// 13.13).
fn quetes(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> Question {
    let mut boutons = Vec::new();
    if Personnage::TOUS
        .into_iter()
        .any(|personnage| partie.personnage_au_camp(personnage))
    {
        boutons.push(bouton(
            Cle::Afficher(Vue::QuetesDuCamp),
            "Personnages du camp",
            PRINCIPAL,
        ));
    }
    boutons.push(retour());
    Question {
        texte: ecran_des_quetes(partie, contenu, maintenant),
        rangees: rangees(boutons),
    }
}

fn trophees(partie: &Partie, contenu: &ContenuV1) -> Question {
    Question {
        texte: trophees_du_camp(partie, contenu.succes()),
        rangees: rangees(vec![
            bouton(Cle::Afficher(Vue::Succes(0)), "Succès", SECONDAIRE),
            retour_au_heros(),
        ]),
    }
}

/// Un bouton par personnage qui a une demande à écouter ou une quête à
/// remettre (section 13.13).
fn quetes_du_camp(partie: &Partie, contenu: &ContenuV1) -> Question {
    let mut boutons: Vec<Bouton> = chaines(partie, contenu)
        .into_iter()
        .filter_map(|(personnage, etat)| match etat {
            EtatDeLaChaine::Proposee(_) => Some(bouton(
                Cle::EcouterQuete(personnage),
                &format!("Écouter {}", personnage.nom()),
                PRINCIPAL,
            )),
            EtatDeLaChaine::EnCours(_) if peut_remettre(partie, contenu, personnage) => {
                Some(bouton(
                    Cle::RemettreQuete(personnage),
                    &format!("Remettre à {}", personnage.nom()),
                    SUCCES,
                ))
            }
            _ => None,
        })
        .collect();
    boutons.push(retour());
    Question {
        texte: ecran_des_quetes_du_camp(partie, contenu),
        rangees: rangees(boutons),
    }
}

/// Une ligne par espèce ; la fiche d'une espèce capturée, ou d'un Apôtre
/// abattu, s'ouvre par le menu. Le carnet de traque suit la liste, à sa
/// dernière page (section 5.6).
/// Avec plus de 40 espèces, la liste et une fiche ne tiennent plus ensemble
/// dans un message : une fiche ouverte remplace la liste, qu'un bouton
/// rouvre. Avec plus de 100, les menus des fiches de tout le codex
/// dépassent les rangées d'un message : ils ne proposent que celles de la
/// page, et une fiche ouverte garde la page de son espèce (étape 166).
/// Assez peu pour qu'une page de la liste tienne dans un message, avec les
/// noms d'espèces les plus longs.
pub(crate) const ESPECES_PAR_PAGE: usize = 30;

fn codex(partie: &Partie, fiche: Option<usize>, page: usize, contenu: &ContenuV1) -> Question {
    let codex = partie.codex();
    let especes = contenu.especes();
    let ouverte = fiche
        .and_then(|position| especes.get(position))
        .filter(|espece| codex.etat(espece.id()) == EtatCodex::Capturee);
    let liste: Vec<(usize, &Espece)> = especes_du_codex(especes).collect();
    let pages_d_especes = liste.len().div_ceil(ESPECES_PAR_PAGE);
    let page_de_la_liste = |page: usize| -> Vec<String> {
        let mut lignes = vec!["Codex :".to_string(), progression_du_codex(partie, contenu)];
        for (numero, (_, espece)) in liste
            .iter()
            .enumerate()
            .skip(page * ESPECES_PAR_PAGE)
            .take(ESPECES_PAR_PAGE)
        {
            lignes.push(format!(
                "  {}. {}",
                numero + 1,
                ligne_du_codex(codex, espece)
            ));
        }
        lignes
    };
    // Le carnet de traque suit la dernière page de la liste, ou prend une
    // page à lui quand il n'y tient plus (étape 166).
    let carnet = carnet_de_traque(partie, contenu);
    let derniere = pages_d_especes.saturating_sub(1);
    let carnet_a_part = !carnet.is_empty() && {
        let mut lignes = page_de_la_liste(derniere);
        lignes.extend(carnet.iter().cloned());
        lignes.join("\n").chars().count() > LONGUEUR_MAX
    };
    let pages = pages_d_especes + usize::from(carnet_a_part);
    // Une fiche ouverte garde la page de son espèce, celle d'un Apôtre
    // abattu la page du carnet.
    let page = match fiche.filter(|_| ouverte.is_some()) {
        Some(position) => liste
            .iter()
            .position(|(autre, _)| *autre == position)
            .map_or(pages.saturating_sub(1), |rang| rang / ESPECES_PAR_PAGE),
        None => page,
    };
    let mut lignes = Vec::new();
    match ouverte {
        Some(espece) => {
            lignes.push(espece.nom().to_string());
            lignes.extend(fiche_d_espece(
                espece,
                codex,
                partie.apotres_vaincus(),
                contenu,
            ));
            lignes.extend(zones_d_apparition(espece, partie, contenu));
        }
        None => {
            lignes.extend(page_de_la_liste(page));
            if page + 1 >= pages {
                lignes.extend(carnet);
            }
        }
    }
    // Les Apôtres abattus, hors de la liste, s'ouvrent depuis la page du
    // carnet (section 5.6).
    let du_carnet = especes
        .iter()
        .enumerate()
        .filter(|(_, espece)| !compte_au_codex(espece))
        .filter(|_| page + 1 >= pages);
    let capturees: Vec<OptionDeMenu> = liste
        .iter()
        .skip(page * ESPECES_PAR_PAGE)
        .take(ESPECES_PAR_PAGE)
        .copied()
        .chain(du_carnet)
        .filter(|(_, espece)| codex.etat(espece.id()) == EtatCodex::Capturee)
        .map(|(position, espece)| OptionDeMenu {
            libelle: espece.nom().to_string(),
            valeur: position.to_string(),
            description: None,
        })
        .collect();
    // 25 options au plus par menu : une page compte plus de 25 espèces, la
    // dernière aussi les Apôtres abattus.
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
    // Au plus 4 boutons : les deux pages, la récompense et le retour.
    if let Some(a_reclamer) =
        partie.recompense_a_reclamer(contenu.recompenses_du_codex(), contenu.especes())
    {
        boutons.push(bouton(
            Cle::ReclamerRecompenseDuCodex,
            &reclamer(a_reclamer),
            SUCCES,
        ));
    }
    let vers_la_page = |page: usize| match page {
        0 => Some(Cle::Afficher(Vue::Codex(None))),
        _ => u8::try_from(page)
            .ok()
            .map(|page| Cle::Afficher(Vue::PageDuCodex(page))),
    };
    if ouverte.is_some() {
        if let Some(cle) = vers_la_page(page) {
            boutons.push(bouton(cle, "Liste du codex", SECONDAIRE));
        }
    } else {
        boutons.push(bouton(
            Cle::Afficher(Vue::Journal(0)),
            "Journal des zones",
            SECONDAIRE,
        ));
        if let Some(cle) = page.checked_sub(1).and_then(vers_la_page) {
            boutons.push(bouton(cle, "Page précédente", SECONDAIRE));
        }
        if page + 1 < pages {
            if let Some(cle) = vers_la_page(page + 1) {
                boutons.push(bouton(cle, "Page suivante", SECONDAIRE));
            }
        }
    }
    boutons.push(retour());
    rangees_du_codex.extend(rangees(boutons));
    Question {
        texte: lignes.join("\n"),
        rangees: rangees_du_codex,
    }
}

/// Le journal des monstres par zone, ouvert sur un lieu découvert, avec un
/// menu des lieux découverts et le retour au codex (section 5.6).
fn journal(partie: &Partie, lieu: usize, contenu: &ContenuV1) -> Question {
    let codex = partie.codex();
    let lieux: Vec<LieuDuJournal<'_>> = lieux_du_journal(partie, contenu)
        .into_iter()
        .filter(|lieu| lieu.decouvert)
        .collect();
    let mut lignes = vec!["Journal des monstres par zone.".to_string()];
    if let Some(ouvert) = lieux.get(lieu).or(lieux.first()) {
        lignes.push(resume_du_lieu(ouvert, codex));
        lignes.extend(lignes_du_lieu(ouvert, codex));
    }
    let options: Vec<OptionDeMenu> = lieux
        .iter()
        .enumerate()
        .map(|(position, lieu)| OptionDeMenu {
            libelle: lieu.nom.clone(),
            valeur: position.to_string(),
            description: Some(resume_du_lieu(lieu, codex)),
        })
        .collect();
    let mut rangees_du_journal =
        crate::message::menus(Cle::LieuDuJournal, "Voir un autre lieu", options, 2);
    rangees_du_journal.extend(rangees(vec![
        bouton(Cle::Afficher(Vue::Codex(None)), "Codex", SECONDAIRE),
        retour(),
    ]));
    Question {
        texte: lignes.join("\n"),
        rangees: rangees_du_journal,
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
        // Le camp a déjà ses deux rangées de boutons : la cour, une fois
        // construite, s'ouvre d'ici, en plus des machines de la forge
        // (section 13.6). 5 boutons au plus.
        if partie.forge().ameliorations().cour().is_some() {
            boutons.push(bouton(
                Cle::Afficher(Vue::Cour),
                "Cour d'entraînement",
                PRINCIPAL,
            ));
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
    // Un monstre loué ne fait que combattre : ni surnom, ni relâcher, ni
    // vente, ni shiny, ni capsule, ni booster ; il se rend à Malvoise
    // (section 14.2).
    let a_soi = !monstre.est_loue();
    if a_soi {
        boutons.push(bouton(Cle::Renommer(emplacement), "Renommer", SECONDAIRE));
        boutons.push(bouton(Cle::Relacher(emplacement), "Relâcher", DANGER));
    } else {
        boutons.push(bouton(
            Cle::RendreLeMonstreLoue,
            "Rendre à Malvoise",
            DANGER,
        ));
    }
    boutons.push(bouton(
        Cle::Afficher(Vue::Attaques(emplacement)),
        "Attaques",
        SECONDAIRE,
    ));
    if partie.marchand_au_camp() && a_soi {
        let libelle = format!("Vendre ({})", argent(prix_de_revente(monstre)));
        boutons.push(bouton(Cle::VendreCeMonstre(emplacement), &libelle, DANGER));
    }
    // Au plus 10 boutons : ils tiennent en deux rangées (sections 5.4, 5.9
    // et 13.7).
    if partie.marchand_au_camp()
        && partie.philtres_chromatiques() > 0
        && !monstre.est_shiny()
        && a_soi
    {
        boutons.push(bouton(
            Cle::RendreShiny(emplacement),
            "Rendre shiny (Malvoise)",
            PRINCIPAL,
        ));
    }
    if partie.capsules_d_habilete() > 0 && a_soi {
        if let Some(nouvelle) = autre_habilete(monstre) {
            let libelle = format!("Passer à {}", nouvelle.nom());
            boutons.push(bouton(
                Cle::ChangerDHabilete(emplacement),
                &libelle,
                SECONDAIRE,
            ));
        }
    }
    // Au plus 11 boutons pour un monstre de l'équipe, qui n'a pas de menu
    // d'échange : 5 rangées en tout (section 13.11).
    if partie.marchand_au_camp()
        && partie.rite().is_none()
        && a_soi
        && !partie
            .seconds_parents_possibles(emplacement, contenu.especes())
            .is_empty()
    {
        boutons.push(bouton(
            Cle::Afficher(Vue::Rite(emplacement)),
            "Rite du sang mêlé (Malvoise)",
            PRINCIPAL,
        ));
    }
    // Par niveau, ou par une pierre que le joueur a (section 5.4).
    if let Ok((_, vers, _)) = partie.estimer_evolution(emplacement, contenu) {
        let libelle = format!("Évoluer en {}", vers.nom());
        boutons.push(bouton(Cle::FaireEvoluer(emplacement), &libelle, PRINCIPAL));
    }
    let mut lignes = rangees(boutons);
    lignes.extend(echange);
    if a_soi {
        lignes.extend(menu(
            Cle::DonnerBooster(emplacement),
            "Donner un booster ou un objet d'expérience",
            options_de_boosters(partie, monstre),
        ));
    }
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
    let boosters_utiles = Booster::TOUS
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
        });
    // Les objets d'expérience suivent les boosters, dans le même menu, tant
    // que le monstre peut encore monter (section 14.3).
    let au_maximum = monstre.niveau().valeur() >= bersemon_core::niveau::Niveau::MAX;
    let objets = ObjetDExperience::TOUS
        .into_iter()
        .filter(|_| !au_maximum && !monstre.est_loue())
        .filter_map(|objet| {
            let nombre = partie.nombre_d_objets_d_experience(objet);
            (nombre > 0).then(|| OptionDeMenu {
                libelle: objets_d_experience(objet, nombre),
                valeur: objet_d_experience_en_valeur(objet),
                description: Some(effet_d_un_objet_d_experience(objet)),
            })
        });
    boosters_utiles.chain(objets).collect()
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
                description: Some(decrire_attaque_de(monstre, attaque)),
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
