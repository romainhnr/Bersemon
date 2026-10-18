//! Expéditions automatiques (section 12) : les zones explorées et leur état,
//! puis le départ, le rappel ou l'accueil d'une expédition. La durée et les
//! chances de réussite sont montrées avant le départ (section 12.4).

use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::expedition_automatique::MONSTRES_MAX;
use bersemon_core::partie::Partie;
use bersemon_core::zone::Zone;
use bersemon_presentation::expeditions_automatiques::{
    erreur_expedition_automatique, estimation, etat_de_la_zone,
};
use bersemon_presentation::partie::collection as liste_des_monstres;

use crate::ecran_camp::emplacement_du_numero;
use crate::horloge::maintenant;
use crate::saisie::Entree;

/// `Some(None)` si le joueur revient au camp, `None` si l'entrée est fermée.
pub fn choisir(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    let explorees: Vec<&Zone> = contenu
        .zones()
        .iter()
        .filter(|zone| partie.est_exploree(zone.id()))
        .collect();
    println!();
    if explorees.is_empty() {
        println!(
            "Vos monstres ne partent seuls que dans une zone déjà explorée : menez d'abord une expédition jusqu'au bout."
        );
        return Some(None);
    }
    println!("Expéditions automatiques : vos monstres partent seuls dans une zone explorée.");
    let heure = maintenant();
    for (position, zone) in explorees.iter().enumerate() {
        println!(
            "  {}. {}",
            position + 1,
            etat_de_la_zone(partie, zone, heure)
        );
    }
    println!("  0. Retour");
    let choix = entree.choix(0, explorees.len())?;
    if choix == 0 {
        return Some(None);
    }
    let zone = explorees[choix - 1];
    match partie.expedition_automatique(zone.id()) {
        Some(expedition) if expedition.est_rentree(heure) => Some(Some(
            Choix::AccueillirExpeditionAutomatique(zone.id().clone()),
        )),
        Some(_) => rappeler(zone, entree),
        None => envoyer(partie, zone, entree),
    }
}

/// La confirmation est demandée par le déroulement.
fn rappeler(zone: &Zone, entree: &mut Entree) -> Option<Option<Choix>> {
    println!("  1. Rappeler l'expédition");
    println!("  0. Retour");
    let choix = entree.choix(0, 1)?;
    Some((choix == 1).then(|| Choix::RappelerExpeditionAutomatique(zone.id().clone())))
}

/// Les monstres sont choisis un par un, par leur numéro dans la liste ;
/// la durée et les chances s'affichent ensuite, avant la confirmation.
fn envoyer(partie: &Partie, zone: &Zone, entree: &mut Entree) -> Option<Option<Choix>> {
    let collection = partie.collection();
    let total = collection.equipe().len() + collection.camp().len();
    println!();
    println!("{}", liste_des_monstres(collection));
    let mut monstres = Vec::new();
    while monstres.len() < MONSTRES_MAX {
        if monstres.is_empty() {
            println!("Quel monstre envoyer ? (0 pour annuler)");
        } else {
            println!("Quel autre monstre ? (0 pour partir avec ceux-là)");
        }
        let numero = entree.choix(0, total)?;
        if numero == 0 {
            if monstres.is_empty() {
                return Some(None);
            }
            break;
        }
        let emplacement = emplacement_du_numero(collection, numero);
        if monstres.contains(&emplacement) {
            println!("Ce monstre est déjà choisi.");
        } else {
            monstres.push(emplacement);
        }
    }
    match partie.estimer_expedition_automatique(zone, &monstres) {
        Err(erreur) => {
            println!("{}", erreur_expedition_automatique(&erreur));
            Some(None)
        }
        Ok(previsions) => {
            println!("{}", estimation(zone, &previsions));
            let partir = entree.confirmation("Envoyer ces monstres ?")?;
            Some(partir.then(|| Choix::EnvoyerEnExpeditionAutomatique {
                zone: zone.id().clone(),
                monstres,
            }))
        }
    }
}
