//! Camp de base : départ en expédition par la carte, expéditions
//! automatiques, gestion des monstres (section 13.2), coffre du camp (section
//! 13.3), codex, marchand et options (section 13.9). Les monstres sont
//! désignés par leur numéro dans la liste de la collection : l'équipe
//! d'abord, le camp ensuite.

use bersemon_core::collection::{Collection, Emplacement, TriDuCamp};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{ActionDeGestion, Choix};
use bersemon_core::partie::Partie;
use bersemon_presentation::expeditions_automatiques::expeditions_rentrees;
use bersemon_presentation::marchand::liste_de_rachat;
use bersemon_presentation::monstres::{attaques_connues, decrire_attaque, fiche_du_monstre};
use bersemon_presentation::partie::{
    capsules_d_habilete, collection as liste_des_monstres, en_tete_du_camp,
};

use crate::affichage;
use crate::ecran_coffre;
use crate::ecran_expeditions_automatiques;
use crate::ecran_marchand;
use crate::horloge::maintenant;
use crate::saisie::Entree;

/// Le menu de gestion, celui du coffre et celui du marchand restent ouverts
/// d'une action à l'autre, jusqu'au retour.
#[derive(Debug, Default)]
pub struct MenuDuCamp {
    ouvert: SousMenu,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum SousMenu {
    #[default]
    Aucun,
    Gestion,
    Coffre,
    Marchand,
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
        for avis in expeditions_rentrees(partie, maintenant()) {
            println!("{avis}");
        }
        println!("  1. Partir en expédition");
        println!("  2. Expéditions automatiques");
        println!("  3. Gérer vos monstres");
        println!("  4. Coffre du camp");
        println!("  5. Consulter le codex");
        // Le marchand n'apparaît qu'une fois réfugié au camp (section 14.2).
        let marchand = partie.marchand_au_camp();
        if marchand {
            println!("  6. Voir Malvoise");
        }
        let options = if marchand { 7 } else { 6 };
        println!("  {options}. Options");
        let quitter = options + 1;
        println!("  {quitter}. Quitter");
        let choix = match entree.choix(1, quitter)? {
            1 => carte(partie, contenu, entree)?,
            2 => ecran_expeditions_automatiques::choisir(partie, contenu, entree)?,
            3 => {
                menu.ouvert = SousMenu::Gestion;
                None
            }
            4 => {
                menu.ouvert = SousMenu::Coffre;
                None
            }
            5 => {
                println!("{}", affichage::codex(partie.codex(), contenu));
                None
            }
            6 if marchand => {
                menu.ouvert = SousMenu::Marchand;
                None
            }
            numero if numero == options => choisir_une_option(entree)?,
            _ => return None,
        };
        if choix.is_some() {
            return choix;
        }
    }
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

/// `Some(None)` si le joueur reste au camp.
fn carte(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    let accessibles = partie.zones_accessibles(contenu.zones(), contenu.piste());
    println!();
    println!("{}", affichage::carte(partie, &accessibles));
    println!("  0. Rester au camp");
    let choix = entree.choix(0, accessibles.len())?;
    Some((choix > 0).then(|| Choix::PartirEnExpedition(accessibles[choix - 1].id().clone())))
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
        println!("  11. Faire évoluer un monstre");
        if marchand {
            println!("  12. Vendre un monstre à Malvoise");
        }
        println!("  0. Retour");
        let dernier = if marchand { 12 } else { 11 };
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
            11 => faire_evoluer(partie, contenu, entree)?,
            _ => vendre(collection, entree)?,
        };
        if let Some(choix) = choix {
            return Some(Some(choix));
        }
    }
}

/// Ce que le joueur a désigné : un monstre, ou rien (0). `None` si l'entrée
/// est fermée.
fn designer(
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
        println!("  {}. {}", position + 1, decrire_attaque(attaque));
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
