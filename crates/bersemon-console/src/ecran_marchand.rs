//! Le marchand réfugié au camp (section 14.2) : potions, capsules d'attaque
//! et rachat des monstres. Les achats se font sans confirmation ; la vente
//! est confirmée par le déroulement.

use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::marchand::PRIX_DE_LA_POTION;
use bersemon_core::partie::Partie;
use bersemon_presentation::marchand::{capsules_en_vente, etal, offre_de_capsule};

use crate::ecran_camp::vendre;
use crate::saisie::Entree;

/// `Some(None)` au retour vers le menu du camp.
pub fn choisir(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    loop {
        println!();
        println!("{}", etal(partie));
        println!("  1. Acheter des potions");
        println!("  2. Acheter une capsule d'attaque");
        println!("  3. Vendre un monstre");
        println!("  0. Retour au camp");
        let choix = match entree.choix(0, 3)? {
            0 => return Some(None),
            1 => acheter_potions(partie, entree)?,
            2 => acheter_capsule(partie, contenu, entree)?,
            _ => vendre(partie.collection(), entree)?,
        };
        if choix.is_some() {
            return Some(choix);
        }
    }
}

/// Le joueur ne choisit que ce qu'il peut payer.
fn acheter_potions(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let au_plus = partie.argent() / PRIX_DE_LA_POTION;
    if au_plus == 0 {
        println!("Vous n'avez pas de quoi payer une potion.");
        return Some(None);
    }
    println!("Combien de potions acheter ? (0 pour annuler)");
    let nombre = entree.choix(0, au_plus as usize)?;
    Some((nombre > 0).then_some(Choix::AcheterPotions(nombre as u32)))
}

fn acheter_capsule(
    partie: &Partie,
    contenu: &ContenuV1,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    let en_vente = capsules_en_vente(partie, contenu.capsules());
    if en_vente.is_empty() {
        println!("Vous avez déjà toutes les capsules du marchand.");
        return Some(None);
    }
    println!("Capsules d'attaque en vente :");
    for (position, capsule) in en_vente.iter().enumerate() {
        println!("  {}. {}", position + 1, offre_de_capsule(capsule));
    }
    println!("Laquelle acheter ? (0 pour annuler)");
    let choix = entree.choix(0, en_vente.len())?;
    Some((choix > 0).then(|| Choix::AcheterCapsule(en_vente[choix - 1].id().clone())))
}
