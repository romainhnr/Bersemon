//! Questions posées en expédition (section 9.2) : entre deux étapes, devant
//! un monstre rare pacifique, et quand une capture ne trouve pas de place.

use bersemon_core::collection::{Collection, Emplacement};
use bersemon_core::deroulement::{ActionDeGestion, Choix};
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;

use crate::affichage;
use crate::saisie::Entree;

/// Rend `None` si l'entrée est fermée.
pub fn entre_deux_etapes(partie: &Partie, entree: &mut Entree) -> Option<Choix> {
    loop {
        println!();
        println!("{}", affichage::equipe_en_expedition(partie));
        println!("Que faire ?");
        println!("  1. Continuer l'expédition");
        println!(
            "  2. Utiliser une potion ({})",
            partie.inventaire().potions()
        );
        println!("  3. Changer l'ordre de l'équipe");
        println!("  4. Rentrer au camp");
        match entree.choix(1, 4)? {
            1 => return Some(Choix::ContinuerLExpedition),
            2 => {
                println!("Soigner quel monstre ? (0 pour annuler)");
                let choix = entree.choix(0, partie.collection().equipe().len())?;
                if choix > 0 {
                    return Some(Choix::UtiliserPotion(choix - 1));
                }
            }
            3 => {
                if let Some(choix) = changer_ordre(partie, entree)? {
                    return Some(choix);
                }
            }
            _ => return Some(Choix::RentrerAuCamp),
        }
    }
}

/// Le premier monstre en état de combattre commencera le prochain combat ;
/// les échanges avec le camp restent impossibles en expédition (Q55).
/// `Some(None)` si le joueur annule.
fn changer_ordre(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let taille = partie.collection().equipe().len();
    println!("Quel monstre déplacer ? (0 pour annuler)");
    let de = entree.choix(0, taille)?;
    if de == 0 {
        return Some(None);
    }
    println!("À quelle place le mettre ?");
    let vers = entree.choix(1, taille)?;
    Some(Some(Choix::Gerer(ActionDeGestion::ChangerLOrdre {
        de: de - 1,
        vers: vers - 1,
    })))
}

pub fn monstre_rare(entree: &mut Entree) -> Option<Choix> {
    let choix = if entree.confirmation("Le prendre avec vous ?")? {
        Choix::PrendreLeMonstreRare
    } else {
        Choix::LaisserLeMonstreRare
    };
    Some(choix)
}

/// Le joueur relâche un de ses monstres pour garder la capture, ou relâche
/// la capture (Q12).
pub fn place_manquante(
    capture: &Monstre,
    collection: &Collection,
    entree: &mut Entree,
) -> Option<Choix> {
    println!();
    println!(
        "Votre équipe et le camp sont pleins. Quel monstre relâcher pour garder {} ?",
        capture.nom()
    );
    println!("{}", affichage::collection(collection));
    println!("  0. Relâcher {}", capture.nom());
    let taille_equipe = collection.equipe().len();
    let total = taille_equipe + collection.camp().len();
    let choix = match entree.choix(0, total)? {
        0 => Choix::RelacherLaCapture,
        numero if numero <= taille_equipe => {
            Choix::RelacherPourLaCapture(Emplacement::Equipe(numero - 1))
        }
        numero => Choix::RelacherPourLaCapture(Emplacement::Camp(numero - 1 - taille_equipe)),
    };
    Some(choix)
}
