//! Questions posées en expédition (section 9.2) : entre deux étapes, devant
//! un monstre rare pacifique, quand une capture ne trouve pas de place, quand
//! le sac déborde, et pour une annonce.

use bersemon_core::collection::{Collection, Emplacement};
use bersemon_core::deroulement::{ActionDeGestion, Annonce, Choix};
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_presentation::partie::{
    FUIR_AVANT_LE_COMBAT, annonce as texte_d_annonce, collection as liste_des_monstres,
    place_manquante as place_manquante_texte,
};
use bersemon_presentation::sac::{question_du_sac_plein, sortes_a_jeter};
use bersemon_presentation::tour::activer_le_raccourci;

use crate::affichage;
use crate::ecran_combat::choisir_soin;
use crate::saisie::Entree;

/// À la fin d'une étape où le sac déborde : une sorte d'objet, puis combien
/// en jeter (section 13.3). Rend `None` si l'entrée est fermée.
pub fn sac_plein(partie: &Partie, entree: &mut Entree) -> Option<Choix> {
    let sortes = sortes_a_jeter(partie.sac());
    println!();
    println!("{}", question_du_sac_plein(partie));
    for (position, sorte) in sortes.iter().enumerate() {
        println!("  {}. {}", position + 1, sorte.libelle);
    }
    let sorte = &sortes[entree.choix(1, sortes.len())? - 1];
    let nombre = if sorte.nombre > 1 {
        println!("Combien en jeter ?");
        entree.choix(1, sorte.nombre as usize)? as u32
    } else {
        1
    };
    Some(Choix::Jeter(sorte.objet(nombre)))
}

/// Rend `None` si l'entrée est fermée. Le contenu du sac est rappelé : une
/// défaite le ferait perdre (section 9.3).
pub fn entre_deux_etapes(
    partie: &Partie,
    raccourci: Option<u32>,
    entree: &mut Entree,
) -> Option<Choix> {
    loop {
        println!();
        println!("{}", affichage::equipe_en_expedition(partie));
        println!("Que faire ?");
        println!("  1. Continuer l'expédition");
        println!(
            "  2. Utiliser un soin ({})",
            affichage::sac(partie.sac().soins())
        );
        println!("  3. Changer l'ordre de l'équipe");
        println!("  4. Rentrer au camp");
        // Dans la Tour, seul : la dernière étape franchie peut devenir le
        // raccourci (section 9.5).
        if let Some(etape) = raccourci {
            println!("  5. {}", activer_le_raccourci(etape));
        }
        let dernier = if raccourci.is_some() { 5 } else { 4 };
        match entree.choix(1, dernier)? {
            5 => return Some(Choix::ActiverLeRaccourci),
            1 => return Some(Choix::ContinuerLExpedition),
            2 => {
                if let Some(soin) = choisir_soin(partie.sac().soins(), entree)? {
                    println!("Soigner qui ? (0 pour annuler)");
                    let choix = entree.choix(0, partie.collection().equipe().len() + 1)?;
                    if choix > 0 {
                        return Some(Choix::UtiliserSoin(soin, choix - 1));
                    }
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

/// Le bouton de l'annonce est l'unique choix d'un menu (section 9.2) ; un
/// combat propose aussi de fuir avant de l'engager (section 10.3).
pub fn annonce(annonce: Annonce, entree: &mut Entree) -> Option<Choix> {
    let (texte, bouton) = texte_d_annonce(annonce);
    println!("{texte}");
    println!("  1. {bouton}");
    if annonce != Annonce::Combat {
        return entree.choix(1, 1).map(|_| Choix::Poursuivre);
    }
    println!("  2. {FUIR_AVANT_LE_COMBAT}");
    match entree.choix(1, 2)? {
        1 => Some(Choix::Poursuivre),
        _ => Some(Choix::RentrerAuCamp),
    }
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
    println!("{}", place_manquante_texte(capture));
    println!("{}", liste_des_monstres(collection));
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
