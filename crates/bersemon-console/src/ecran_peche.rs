//! La pêche d'Ansgar (section 13.8) : le joueur lance la ligne, la relève
//! une fois prête, puis garde ou relâche le monstre pêché.

use bersemon_core::deroulement::Choix;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_presentation::mise_en_valeur::sans_marques;
use bersemon_presentation::monstres::fiche_du_monstre;
use bersemon_presentation::peche::{etat_de_la_ligne, question_de_la_prise};

use crate::horloge::maintenant;
use crate::saisie::Entree;

/// `Some(None)` au retour vers le menu du camp. Une ligne à l'eau qui n'est
/// pas encore prête ne propose que le retour.
pub fn choisir(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let maintenant = maintenant();
    let peche = partie.peche();
    println!();
    println!("Pêche d'Ansgar :");
    println!("{}", etat_de_la_ligne(partie, maintenant));
    let action = match peche.ligne() {
        None => Some((Choix::LancerLaLigne, "Lancer la ligne")),
        Some(_) if peche.est_prete(maintenant) => Some((Choix::ReleverLaLigne, "Relever la ligne")),
        Some(_) => None,
    };
    if let Some((_, libelle)) = &action {
        println!("  1. {libelle}");
    }
    println!("  0. Retour au camp");
    let numero = entree.choix(0, usize::from(action.is_some()))?;
    Some(action.filter(|_| numero == 1).map(|(choix, _)| choix))
}

/// Le monstre pêché, avec sa fiche, que le joueur garde ou relâche.
pub fn prise(monstre: &Monstre, entree: &mut Entree) -> Option<Choix> {
    println!();
    println!("{}", fiche_du_monstre(monstre));
    println!("{}", sans_marques(&question_de_la_prise(monstre)));
    println!("  1. Garder");
    println!("  2. Relâcher");
    let choix = match entree.choix(1, 2)? {
        1 => Choix::GarderLaPrise,
        _ => Choix::RelacherLaPrise,
    };
    Some(choix)
}
