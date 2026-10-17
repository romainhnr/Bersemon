//! Gestion des monstres au camp de base (section 13.2). Les monstres sont
//! désignés par leur numéro dans la liste de la collection : l'équipe
//! d'abord, le camp ensuite.

use bersemon_core::collection::{Collection, Emplacement, ErreurCollection};
use bersemon_core::nom::Nom;

use crate::affichage;
use crate::saisie::Entree;

/// Rend `false` si l'entrée est fermée.
pub fn gerer_monstres(collection: &mut Collection, entree: &mut Entree) -> bool {
    loop {
        println!();
        println!("{}", affichage::collection(collection));
        println!("Que faire ?");
        println!("  1. Changer l'ordre de l'équipe");
        println!("  2. Envoyer un monstre de l'équipe au camp");
        println!("  3. Faire venir un monstre du camp");
        println!("  4. Échanger un monstre de l'équipe et un du camp");
        println!("  5. Renommer un monstre");
        println!("  6. Relâcher un monstre");
        println!("  0. Retour");
        let Some(choix) = entree.choix(0, 6) else {
            return false;
        };
        let continuer = match choix {
            0 => return true,
            1 => changer_ordre(collection, entree),
            2 => envoyer_au_camp(collection, entree),
            3 => faire_venir(collection, entree),
            4 => echanger(collection, entree),
            5 => renommer(collection, entree),
            _ => relacher(collection, entree),
        };
        if !continuer {
            return false;
        }
    }
}

/// Ce que le joueur a désigné : un monstre, rien (0), ou la fin de l'entrée.
enum Designation {
    Monstre(Emplacement),
    Annulee,
    EntreeFermee,
}

fn designer(collection: &Collection, entree: &mut Entree, question: &str) -> Designation {
    println!("{question} (0 pour annuler)");
    let taille_equipe = collection.equipe().len();
    let total = taille_equipe + collection.camp().len();
    match entree.choix(0, total) {
        None => Designation::EntreeFermee,
        Some(0) => Designation::Annulee,
        Some(numero) if numero <= taille_equipe => {
            Designation::Monstre(Emplacement::Equipe(numero - 1))
        }
        Some(numero) => Designation::Monstre(Emplacement::Camp(numero - 1 - taille_equipe)),
    }
}

fn changer_ordre(collection: &mut Collection, entree: &mut Entree) -> bool {
    let de = match designer(collection, entree, "Quel monstre de l'équipe déplacer ?") {
        Designation::Monstre(Emplacement::Equipe(index)) => index,
        Designation::Monstre(Emplacement::Camp(_)) => {
            println!("Choisissez un monstre de l'équipe.");
            return true;
        }
        Designation::Annulee => return true,
        Designation::EntreeFermee => return false,
    };
    println!("À quelle place le mettre ?");
    let Some(vers) = entree.choix(1, collection.equipe().len()) else {
        return false;
    };
    rapporter(collection.deplacer_dans_l_equipe(de, vers - 1));
    true
}

fn envoyer_au_camp(collection: &mut Collection, entree: &mut Entree) -> bool {
    match designer(collection, entree, "Quel monstre envoyer au camp ?") {
        Designation::Monstre(Emplacement::Equipe(index)) => {
            rapporter(collection.envoyer_au_camp(index));
        }
        Designation::Monstre(Emplacement::Camp(_)) => {
            println!("Ce monstre est déjà au camp.");
        }
        Designation::Annulee => {}
        Designation::EntreeFermee => return false,
    }
    true
}

fn faire_venir(collection: &mut Collection, entree: &mut Entree) -> bool {
    match designer(collection, entree, "Quel monstre faire venir du camp ?") {
        Designation::Monstre(Emplacement::Camp(index)) => {
            rapporter(collection.faire_venir(index));
        }
        Designation::Monstre(Emplacement::Equipe(_)) => {
            println!("Ce monstre est déjà dans l'équipe.");
        }
        Designation::Annulee => {}
        Designation::EntreeFermee => return false,
    }
    true
}

fn echanger(collection: &mut Collection, entree: &mut Entree) -> bool {
    let index_equipe = match designer(collection, entree, "Quel monstre de l'équipe ?") {
        Designation::Monstre(Emplacement::Equipe(index)) => index,
        Designation::Monstre(Emplacement::Camp(_)) => {
            println!("Choisissez d'abord un monstre de l'équipe.");
            return true;
        }
        Designation::Annulee => return true,
        Designation::EntreeFermee => return false,
    };
    match designer(collection, entree, "Avec quel monstre du camp ?") {
        Designation::Monstre(Emplacement::Camp(index_camp)) => {
            rapporter(collection.echanger(index_equipe, index_camp));
        }
        Designation::Monstre(Emplacement::Equipe(_)) => {
            println!("Choisissez un monstre du camp.");
        }
        Designation::Annulee => {}
        Designation::EntreeFermee => return false,
    }
    true
}

fn renommer(collection: &mut Collection, entree: &mut Entree) -> bool {
    let emplacement = match designer(collection, entree, "Quel monstre renommer ?") {
        Designation::Monstre(emplacement) => emplacement,
        Designation::Annulee => return true,
        Designation::EntreeFermee => return false,
    };
    let Some(texte) = entree.texte("Surnom (laisser vide pour l'effacer) : ") else {
        return false;
    };
    let surnom = if texte.trim().is_empty() {
        None
    } else {
        match Nom::new(&texte) {
            Ok(nom) => Some(nom),
            Err(erreur) => {
                println!("Surnom refusé : {erreur}.");
                return true;
            }
        }
    };
    rapporter(collection.renommer(emplacement, surnom));
    true
}

fn relacher(collection: &mut Collection, entree: &mut Entree) -> bool {
    let emplacement = match designer(collection, entree, "Quel monstre relâcher ?") {
        Designation::Monstre(emplacement) => emplacement,
        Designation::Annulee => return true,
        Designation::EntreeFermee => return false,
    };
    let Some(nom) = collection
        .monstre(emplacement)
        .map(|monstre| monstre.nom().to_string())
    else {
        println!("Ce monstre n'existe pas.");
        return true;
    };
    match entree.confirmation(&format!("Relâcher définitivement {nom} ?")) {
        None => false,
        Some(false) => true,
        Some(true) => {
            match collection.relacher(emplacement) {
                Ok(relache) => println!("Vous relâchez {}.", relache.nom()),
                Err(erreur) => println!("{}", affichage::erreur_collection(&erreur)),
            }
            true
        }
    }
}

fn rapporter(resultat: Result<(), ErreurCollection>) {
    if let Err(erreur) = resultat {
        println!("{}", affichage::erreur_collection(&erreur));
    }
}
