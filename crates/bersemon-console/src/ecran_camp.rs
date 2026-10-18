//! Camp de base : départ en expédition par la carte, liste des monstres,
//! gestion des monstres (section 13.2) et codex. Les monstres sont désignés
//! par leur numéro dans la liste de la collection : l'équipe d'abord, le camp
//! ensuite.

use bersemon_core::collection::{Collection, Emplacement};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{ActionDeGestion, Choix};
use bersemon_core::partie::Partie;
use bersemon_presentation::partie::{collection as liste_des_monstres, en_tete_du_camp};

use crate::affichage;
use crate::saisie::Entree;

/// Le menu de gestion reste ouvert d'une action à l'autre, jusqu'au retour.
#[derive(Debug, Default)]
pub struct MenuDuCamp {
    gestion_ouverte: bool,
}

/// Rend `None` si le joueur quitte ou que l'entrée est fermée.
pub fn choisir(
    partie: &Partie,
    contenu: &ContenuV1,
    menu: &mut MenuDuCamp,
    entree: &mut Entree,
) -> Option<Choix> {
    loop {
        if menu.gestion_ouverte {
            match gerer(partie.collection(), entree)? {
                Some(action) => return Some(Choix::Gerer(action)),
                None => menu.gestion_ouverte = false,
            }
            continue;
        }
        println!();
        println!("{}", en_tete_du_camp(partie));
        println!("  1. Partir en expédition");
        println!("  2. Voir vos monstres");
        println!("  3. Gérer vos monstres");
        println!("  4. Consulter le codex");
        println!("  5. Quitter");
        match entree.choix(1, 5)? {
            1 => {
                if let Some(choix) = carte(partie, contenu, entree)? {
                    return Some(choix);
                }
            }
            2 => println!("{}", liste_des_monstres(partie.collection())),
            3 => menu.gestion_ouverte = true,
            4 => println!("{}", affichage::codex(partie.codex(), contenu.especes())),
            _ => return None,
        }
    }
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
fn gerer(collection: &Collection, entree: &mut Entree) -> Option<Option<ActionDeGestion>> {
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
        println!("  0. Retour");
        let action = match entree.choix(0, 6)? {
            0 => return Some(None),
            1 => changer_ordre(collection, entree)?,
            2 => envoyer_au_camp(collection, entree)?,
            3 => faire_venir(collection, entree)?,
            4 => echanger(collection, entree)?,
            5 => renommer(collection, entree)?,
            _ => relacher(collection, entree)?,
        };
        if let Some(action) = action {
            return Some(Some(action));
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
    let taille_equipe = collection.equipe().len();
    let total = taille_equipe + collection.camp().len();
    let emplacement = match entree.choix(0, total)? {
        0 => None,
        numero if numero <= taille_equipe => Some(Emplacement::Equipe(numero - 1)),
        numero => Some(Emplacement::Camp(numero - 1 - taille_equipe)),
    };
    Some(emplacement)
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

/// La confirmation est demandée par le déroulement.
fn relacher(collection: &Collection, entree: &mut Entree) -> Option<Option<ActionDeGestion>> {
    let emplacement = designer(collection, entree, "Quel monstre relâcher ?")?;
    Some(emplacement.map(ActionDeGestion::Relacher))
}
