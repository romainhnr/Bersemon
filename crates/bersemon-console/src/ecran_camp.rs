//! Camp de base : départ en expédition par la carte, expéditions
//! automatiques, gestion des monstres (section 13.2), coffre du camp (section
//! 13.3), codex, marchand, forge, équipement du héros (section 4.3), pêche
//! (section 13.8) et options (section 13.9). Les monstres sont
//! désignés par leur numéro dans la liste de la collection : l'équipe
//! d'abord, le camp ensuite.

use bersemon_core::beherit::EtatDuBeherit;
use bersemon_core::booster::Booster;
use bersemon_core::collection::{Collection, Emplacement, TriDuCamp};
use bersemon_core::combat::Cote;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{ActionDeGestion, Choix};
use bersemon_core::partie::Partie;
use bersemon_presentation::arene::lieu_sur_la_carte;
use bersemon_presentation::beherit::{DEMANDE_DE_SACRIFICE, destination_de_l_eclipse};
use bersemon_presentation::boosters::{boosters, effet_du_booster};
use bersemon_presentation::expeditions_automatiques::expeditions_rentrees;
use bersemon_presentation::forge::travail_fini;
use bersemon_presentation::jour_et_nuit::moment_present;
use bersemon_presentation::machines::avis_des_pieges;
use bersemon_presentation::marchand::liste_de_rachat;
use bersemon_presentation::monstres::{
    attaques_connues, decrire_attaque_de, fiche_du_monstre, ligne_monstre,
};
use bersemon_presentation::partie::{
    ames_d_apotres, capsules_d_habilete, collection as liste_des_monstres, en_tete_du_camp,
};
use bersemon_presentation::peche::ligne_prete;
use bersemon_presentation::recompenses_du_codex::{progression_du_codex, reclamer};
use bersemon_presentation::rite::rite_acheve;
use bersemon_presentation::tour::destination_de_la_tour;

use crate::affichage;
use crate::ecran_coffre;
use crate::ecran_equipement;
use crate::ecran_expeditions_automatiques;
use crate::ecran_forge;
use crate::ecran_machines;
use crate::ecran_marchand;
use crate::ecran_peche;
use crate::horloge::maintenant;
use crate::saisie::Entree;

/// Le menu de gestion, ceux du coffre, du marchand et de l'équipement restent
/// ouverts
/// d'une action à l'autre, jusqu'au retour.
#[derive(Debug, Default)]
pub struct MenuDuCamp {
    ouvert: SousMenu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntreeDuCamp {
    Carte,
    Automatiques,
    Gestion,
    Coffre,
    Codex,
    Marchand,
    Forge,
    Equipement,
    Peche,
    Cour,
    Options,
    Quitter,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum SousMenu {
    #[default]
    Aucun,
    Gestion,
    Coffre,
    Marchand,
    Equipement,
}

/// Rend `None` si le joueur quitte ou que l'entrée est fermée.
pub fn choisir(
    partie: &Partie,
    contenu: &ContenuV1,
    menu: &mut MenuDuCamp,
    entree: &mut Entree,
) -> Option<Choix> {
    loop {
        let sous_menu = match menu.ouvert {
            SousMenu::Aucun => None,
            SousMenu::Gestion => Some(gerer(partie, contenu, entree)?),
            SousMenu::Coffre => Some(ecran_coffre::choisir(partie, entree)?),
            SousMenu::Marchand => Some(ecran_marchand::choisir(partie, contenu, entree)?),
            SousMenu::Equipement => Some(ecran_equipement::choisir(partie, entree)?),
        };
        match sous_menu {
            Some(Some(choix)) => return Some(choix),
            Some(None) => {
                menu.ouvert = SousMenu::Aucun;
                continue;
            }
            None => {}
        }
        println!();
        println!("{}", en_tete_du_camp(partie));
        println!("{}", moment_present(maintenant()));
        if let Some(ames) = ames_d_apotres(partie, contenu) {
            println!("{ames}");
        }
        for avis in expeditions_rentrees(partie, maintenant()) {
            println!("{avis}");
        }
        if let Some(avis) = travail_fini(partie, maintenant()) {
            println!("{avis}");
        }
        if let Some(avis) = ligne_prete(partie, maintenant()) {
            println!("{avis}");
        }
        if let Some(avis) = avis_des_pieges(partie, maintenant()) {
            println!("{avis}");
        }
        if let Some(avis) = rite_acheve(partie, maintenant()) {
            println!("{avis}");
        }
        // Malvoise, Gunther et Ansgar n'apparaissent qu'une fois au camp
        // (sections 13.4, 13.8 et 14.2).
        let mut entrees = vec![
            (EntreeDuCamp::Carte, "Partir en expédition"),
            (EntreeDuCamp::Automatiques, "Expéditions automatiques"),
            (EntreeDuCamp::Gestion, "Gérer vos monstres"),
            (EntreeDuCamp::Coffre, "Coffre du camp"),
            (EntreeDuCamp::Codex, "Consulter le codex"),
        ];
        if partie.marchand_au_camp() {
            entrees.push((EntreeDuCamp::Marchand, "Voir Malvoise"));
        }
        if partie.forge().est_au_camp() {
            entrees.push((EntreeDuCamp::Forge, "Forge de Gunther"));
        }
        // L'équipement vient de la forge (section 13.5).
        if partie.forge().est_au_camp() || !partie.heros().equipement().possedes().is_empty() {
            entrees.push((EntreeDuCamp::Equipement, "Équipement du héros"));
        }
        if partie.peche().pecheur_au_camp() {
            entrees.push((EntreeDuCamp::Peche, "Pêche d'Ansgar"));
        }
        // Un accès direct, en plus de celui des machines de la forge, une
        // fois la cour construite (section 13.6).
        if partie.forge().ameliorations().cour().is_some() {
            entrees.push((EntreeDuCamp::Cour, "Cour d'entraînement"));
        }
        entrees.push((EntreeDuCamp::Options, "Options"));
        entrees.push((EntreeDuCamp::Quitter, "Quitter"));
        for (position, (_, libelle)) in entrees.iter().enumerate() {
            println!("  {}. {libelle}", position + 1);
        }
        let (entree_choisie, _) = entrees[entree.choix(1, entrees.len())? - 1];
        let choix = match entree_choisie {
            EntreeDuCamp::Carte => carte(partie, contenu, entree)?,
            EntreeDuCamp::Automatiques => {
                ecran_expeditions_automatiques::choisir(partie, contenu, entree)?
            }
            EntreeDuCamp::Gestion => {
                menu.ouvert = SousMenu::Gestion;
                None
            }
            EntreeDuCamp::Coffre => {
                menu.ouvert = SousMenu::Coffre;
                None
            }
            EntreeDuCamp::Codex => consulter_le_codex(partie, contenu, entree)?,
            EntreeDuCamp::Marchand => {
                menu.ouvert = SousMenu::Marchand;
                None
            }
            EntreeDuCamp::Forge => ecran_forge::choisir(partie, contenu, entree)?,
            EntreeDuCamp::Equipement => {
                menu.ouvert = SousMenu::Equipement;
                None
            }
            EntreeDuCamp::Peche => ecran_peche::choisir(partie, entree)?,
            EntreeDuCamp::Cour => ecran_machines::choisir_dans_la_cour(partie, contenu, entree)?,
            EntreeDuCamp::Options => choisir_une_option(entree)?,
            EntreeDuCamp::Quitter => return None,
        };
        if choix.is_some() {
            return choix;
        }
    }
}

/// `Some(None)` au retour vers le menu du camp. La récompense d'un palier
/// atteint se réclame d'ici (section 5.6).
fn consulter_le_codex(
    partie: &Partie,
    contenu: &ContenuV1,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    println!("{}", affichage::codex(partie, contenu));
    println!("{}", progression_du_codex(partie, contenu));
    let recompenses = contenu.recompenses_du_codex();
    let Some(a_reclamer) = partie.recompense_a_reclamer(recompenses, contenu.especes()) else {
        return Some(None);
    };
    println!("  1. {}", reclamer(a_reclamer));
    println!("  0. Retour au camp");
    let choix = entree.choix(0, 1)?;
    Some((choix == 1).then_some(Choix::ReclamerRecompenseDuCodex))
}

/// `Some(None)` au retour vers le menu du camp. Le déroulement demande
/// ensuite confirmation du suicide (section 13.9).
fn choisir_une_option(entree: &mut Entree) -> Option<Option<Choix>> {
    println!();
    println!("Options :");
    println!("  1. Se suicider (la partie est effacée)");
    println!("  0. Retour au camp");
    let choix = entree.choix(0, 1)?;
    Some((choix == 1).then_some(Choix::SeSuicider))
}

/// `Some(None)` si le joueur reste au camp. La Tour sans fond suit les
/// zones, une fois ouverte (section 9.5), puis la Fosse aux damnés (section
/// 9.8) et l'Éclipse, avec le Béhérit (section 9.4).
fn carte(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    let accessibles = partie.zones_accessibles(contenu.zones(), contenu.piste());
    println!();
    // Le moment du départ est celui de toute l'expédition (section 9.6).
    println!("{}", moment_present(maintenant()));
    println!("{}", affichage::carte(partie, &accessibles, maintenant()));
    let mut dernier = accessibles.len();
    let tour = destination_de_la_tour(partie, contenu).map(|ligne| {
        dernier += 1;
        println!("  {dernier}. {ligne}");
        dernier
    });
    let fosse = partie.fosse_ouverte(contenu.fosse()).then(|| {
        dernier += 1;
        println!(
            "  {dernier}. {}",
            lieu_sur_la_carte(partie.arene().paliers_vaincus)
        );
        dernier
    });
    let eclipse = destination_de_l_eclipse(partie, contenu).map(|ligne| {
        dernier += 1;
        println!("  {dernier}. {ligne}");
        dernier
    });
    println!("  0. Rester au camp");
    let choix = entree.choix(0, dernier)?;
    match choix {
        0 => Some(None),
        numero if numero <= accessibles.len() => Some(Some(Choix::PartirEnExpedition(
            accessibles[numero - 1].id().clone(),
        ))),
        numero if Some(numero) == tour => {
            Some(Some(Choix::PartirEnExpedition(contenu.tour().id().clone())))
        }
        numero if Some(numero) == fosse => Some(Some(Choix::EntrerDansLaFosse)),
        numero if Some(numero) == eclipse && partie.beherit() == EtatDuBeherit::Forge => {
            choisir_le_sacrifice(partie, entree)
        }
        _ => Some(Some(Choix::PartirEnExpedition(
            contenu.invocation().eclipse().id().clone(),
        ))),
    }
}

/// Seuls les monstres que le Béhérit accepte sont proposés.
fn choisir_le_sacrifice(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let sacrifiables = partie.monstres_sacrifiables();
    println!();
    println!("{DEMANDE_DE_SACRIFICE}");
    if sacrifiables.is_empty() {
        println!("Aucun de vos monstres ne convient.");
        return Some(None);
    }
    for (position, emplacement) in sacrifiables.iter().enumerate() {
        if let Some(monstre) = partie.collection().monstre(*emplacement) {
            println!(
                "  {}. {}",
                position + 1,
                ligne_monstre(monstre, Cote::Joueur)
            );
        }
    }
    println!("Lequel sacrifier ? (0 pour renoncer)");
    let choix = entree.choix(0, sacrifiables.len())?;
    Some((choix > 0).then(|| Choix::Sacrifier(sacrifiables[choix - 1])))
}

/// `Some(None)` au retour vers le menu du camp.
fn gerer(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    let collection = partie.collection();
    let marchand = partie.marchand_au_camp();
    loop {
        println!();
        println!("{}", liste_des_monstres(collection));
        println!("Que faire ?");
        println!("  1. Changer l'ordre de l'équipe");
        println!("  2. Envoyer un monstre de l'équipe au camp");
        println!("  3. Faire venir un monstre du camp");
        println!("  4. Échanger un monstre de l'équipe et un du camp");
        println!("  5. Renommer un monstre");
        println!("  6. Relâcher un monstre");
        println!("  7. Voir la fiche d'un monstre");
        println!("  8. Gérer les attaques d'un monstre");
        println!("  9. Trier le camp");
        println!(
            "  10. Changer l'habileté d'un monstre ({})",
            capsules_d_habilete(partie.capsules_d_habilete())
        );
        println!("  11. Donner un booster à un monstre");
        println!("  12. Faire évoluer un monstre");
        if marchand {
            println!("  13. Vendre un monstre à Malvoise");
        }
        println!("  0. Retour");
        let dernier = if marchand { 13 } else { 12 };
        let gestion = |action: Option<ActionDeGestion>| action.map(Choix::Gerer);
        let choix = match entree.choix(0, dernier)? {
            0 => return Some(None),
            1 => gestion(changer_ordre(collection, entree)?),
            2 => gestion(envoyer_au_camp(collection, entree)?),
            3 => gestion(faire_venir(collection, entree)?),
            4 => gestion(echanger(collection, entree)?),
            5 => gestion(renommer(collection, entree)?),
            6 => gestion(relacher(collection, entree)?),
            7 => {
                fiche(collection, entree)?;
                None
            }
            8 => attaques(partie, entree)?,
            9 => gestion(trier(entree)?),
            10 => changer_d_habilete(collection, entree)?,
            11 => donner_booster(partie, entree)?,
            12 => faire_evoluer(partie, contenu, entree)?,
            _ => vendre(collection, entree)?,
        };
        if let Some(choix) = choix {
            return Some(Some(choix));
        }
    }
}

/// Ce que le joueur a désigné : un monstre, ou rien (0). `None` si l'entrée
/// est fermée.
pub fn designer(
    collection: &Collection,
    entree: &mut Entree,
    question: &str,
) -> Option<Option<Emplacement>> {
    println!("{question} (0 pour annuler)");
    let total = collection.equipe().len() + collection.camp().len();
    let emplacement = match entree.choix(0, total)? {
        0 => None,
        numero => Some(emplacement_du_numero(collection, numero)),
    };
    Some(emplacement)
}

/// Le numéro, à partir de 1, d'un monstre de la liste de la collection.
pub fn emplacement_du_numero(collection: &Collection, numero: usize) -> Emplacement {
    let taille_equipe = collection.equipe().len();
    if numero <= taille_equipe {
        Emplacement::Equipe(numero - 1)
    } else {
        Emplacement::Camp(numero - 1 - taille_equipe)
    }
}

fn changer_ordre(collection: &Collection, entree: &mut Entree) -> Option<Option<ActionDeGestion>> {
    let de = match designer(collection, entree, "Quel monstre de l'équipe déplacer ?")? {
        Some(Emplacement::Equipe(index)) => index,
        Some(Emplacement::Camp(_)) => {
            println!("Choisissez un monstre de l'équipe.");
            return Some(None);
        }
        None => return Some(None),
    };
    println!("À quelle place le mettre ?");
    let vers = entree.choix(1, collection.equipe().len())?;
    Some(Some(ActionDeGestion::ChangerLOrdre { de, vers: vers - 1 }))
}

fn envoyer_au_camp(
    collection: &Collection,
    entree: &mut Entree,
) -> Option<Option<ActionDeGestion>> {
    let action = match designer(collection, entree, "Quel monstre envoyer au camp ?")? {
        Some(Emplacement::Equipe(index)) => Some(ActionDeGestion::EnvoyerAuCamp(index)),
        Some(Emplacement::Camp(_)) => {
            println!("Ce monstre est déjà au camp.");
            None
        }
        None => None,
    };
    Some(action)
}

fn faire_venir(collection: &Collection, entree: &mut Entree) -> Option<Option<ActionDeGestion>> {
    let action = match designer(collection, entree, "Quel monstre faire venir du camp ?")? {
        Some(Emplacement::Camp(index)) => Some(ActionDeGestion::FaireVenir(index)),
        Some(Emplacement::Equipe(_)) => {
            println!("Ce monstre est déjà dans l'équipe.");
            None
        }
        None => None,
    };
    Some(action)
}

fn echanger(collection: &Collection, entree: &mut Entree) -> Option<Option<ActionDeGestion>> {
    let equipe = match designer(collection, entree, "Quel monstre de l'équipe ?")? {
        Some(Emplacement::Equipe(index)) => index,
        Some(Emplacement::Camp(_)) => {
            println!("Choisissez d'abord un monstre de l'équipe.");
            return Some(None);
        }
        None => return Some(None),
    };
    let action = match designer(collection, entree, "Avec quel monstre du camp ?")? {
        Some(Emplacement::Camp(camp)) => Some(ActionDeGestion::Echanger { equipe, camp }),
        Some(Emplacement::Equipe(_)) => {
            println!("Choisissez un monstre du camp.");
            None
        }
        None => None,
    };
    Some(action)
}

fn renommer(collection: &Collection, entree: &mut Entree) -> Option<Option<ActionDeGestion>> {
    let Some(emplacement) = designer(collection, entree, "Quel monstre renommer ?")? else {
        return Some(None);
    };
    let surnom = entree.texte("Surnom (laisser vide pour l'effacer) : ")?;
    Some(Some(ActionDeGestion::Renommer {
        emplacement,
        surnom,
    }))
}

/// Types, stats et valeurs internes (section 13.2).
fn fiche(collection: &Collection, entree: &mut Entree) -> Option<()> {
    let emplacement = designer(collection, entree, "Quel monstre ?")?;
    if let Some(monstre) = emplacement.and_then(|emplacement| collection.monstre(emplacement)) {
        println!();
        println!("{}", fiche_du_monstre(monstre));
        println!("{}", attaques_connues(monstre));
    }
    Some(())
}

/// L'ordre du camp change pour de bon (section 13.2).
fn trier(entree: &mut Entree) -> Option<Option<ActionDeGestion>> {
    println!("Trier le camp par :");
    println!("  1. Nom");
    println!("  2. Niveau");
    println!("  3. Type");
    println!("  0. Annuler");
    let tri = match entree.choix(0, 3)? {
        0 => return Some(None),
        1 => TriDuCamp::Nom,
        2 => TriDuCamp::Niveau,
        _ => TriDuCamp::Type,
    };
    Some(Some(ActionDeGestion::TrierLeCamp(tri)))
}

/// La confirmation est demandée par le déroulement.
fn relacher(collection: &Collection, entree: &mut Entree) -> Option<Option<ActionDeGestion>> {
    let emplacement = designer(collection, entree, "Quel monstre relâcher ?")?;
    Some(emplacement.map(ActionDeGestion::Relacher))
}

/// La confirmation, qui annonce la nouvelle habileté, est demandée par le
/// déroulement (section 5.9).
fn changer_d_habilete(collection: &Collection, entree: &mut Entree) -> Option<Option<Choix>> {
    let emplacement = designer(
        collection,
        entree,
        "Sur quel monstre utiliser une capsule d'habileté ?",
    )?;
    Some(emplacement.map(Choix::ChangerDHabilete))
}

/// Parmi les boosters du sac et du coffre, sans confirmation (section
/// 14.3).
fn donner_booster(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let possedes: Vec<(Booster, u32)> = Booster::TOUS
        .into_iter()
        .map(|booster| (booster, partie.nombre_de_boosters(booster)))
        .filter(|(_, nombre)| *nombre > 0)
        .collect();
    if possedes.is_empty() {
        println!("Vous n'avez aucun booster.");
        return Some(None);
    }
    println!("Quel booster donner ? (0 pour annuler)");
    for (numero, (booster, nombre)) in possedes.iter().enumerate() {
        println!(
            "  {}. {} : {}",
            numero + 1,
            boosters(*booster, *nombre),
            effet_du_booster(*booster)
        );
    }
    let choix = entree.choix(0, possedes.len())?;
    if choix == 0 {
        return Some(None);
    }
    let booster = possedes[choix - 1].0;
    let emplacement = designer(
        partie.collection(),
        entree,
        &format!("À quel monstre donner la {} ?", booster.nom()),
    )?;
    Some(emplacement.map(|emplacement| Choix::DonnerBooster {
        emplacement,
        booster,
    }))
}

/// Parmi les monstres qui en remplissent la condition ; la confirmation est
/// demandée par le déroulement (section 5.4).
fn faire_evoluer(
    partie: &Partie,
    contenu: &ContenuV1,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    let possibles = partie.evolutions_possibles(contenu);
    if possibles.is_empty() {
        println!("Aucun monstre ne peut évoluer pour l'instant.");
        return Some(None);
    }
    println!("Quel monstre faire évoluer ? (0 pour annuler)");
    for (numero, (emplacement, vers)) in possibles.iter().enumerate() {
        let monstre = partie
            .collection()
            .monstre(*emplacement)
            .expect("emplacement de la collection");
        println!("  {}. {} en {}", numero + 1, monstre.nom(), vers.nom());
    }
    let choix = entree.choix(0, possibles.len())?;
    Some((choix > 0).then(|| Choix::FaireEvoluer(possibles[choix - 1].0)))
}

/// Avec le prix de chaque monstre ; la confirmation est demandée par le
/// déroulement (section 14.2).
pub fn vendre(collection: &Collection, entree: &mut Entree) -> Option<Option<Choix>> {
    println!();
    println!("{}", liste_de_rachat(collection));
    let emplacement = designer(collection, entree, "Quel monstre vendre à Malvoise ?")?;
    Some(emplacement.map(Choix::Vendre))
}

/// Une attaque de son espèce jusqu'à son niveau, ou d'une capsule de son
/// type ; s'il en connaît déjà 4, il en oublie une (section 13.10).
fn attaques(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let collection = partie.collection();
    let Some(emplacement) = designer(collection, entree, "Quel monstre ?")? else {
        return Some(None);
    };
    let Some(monstre) = collection.monstre(emplacement) else {
        return Some(None);
    };
    println!();
    println!("{}", fiche_du_monstre(monstre));
    println!("{}", attaques_connues(monstre));
    let a_apprendre = partie.attaques_a_apprendre(emplacement).unwrap_or_default();
    if a_apprendre.is_empty() {
        println!("Rien de nouveau à apprendre au camp.");
        return Some(None);
    }
    println!("À apprendre au camp :");
    for (position, attaque) in a_apprendre.iter().enumerate() {
        println!(
            "  {}. {}",
            position + 1,
            decrire_attaque_de(monstre, attaque)
        );
    }
    println!("Quelle attaque apprendre ? (0 pour annuler)");
    let choix = entree.choix(0, a_apprendre.len())?;
    if choix == 0 {
        return Some(None);
    }
    let attaque = a_apprendre[choix - 1].id().clone();
    let oubliee = if monstre.attaques().est_complete() {
        println!(
            "{} connaît déjà 4 attaques. Laquelle oublier ? (0 pour annuler)",
            monstre.nom()
        );
        match entree.choix(0, monstre.attaques().liste().len())? {
            0 => return Some(None),
            position => Some(position - 1),
        }
    } else {
        None
    };
    Some(Some(Choix::ApprendreAttaque {
        emplacement,
        attaque,
        oubliee,
    }))
}
