//! Malvoise, le nécromancien du camp (section 14.2) : soins, capsules d'attaque,
//! rachat des monstres et don des âmes. Les achats se font sans confirmation ; la vente
//! est confirmée par le déroulement.

use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::marchand::{PRIX_CAPSULE_D_HABILETE, prix_du_soin};
use bersemon_core::partie::Partie;
use bersemon_presentation::marchand::{capsules_en_vente, etal, offre_de_capsule};
use bersemon_presentation::partie::argent;
use bersemon_presentation::soins::{le_soin, offre_de_soin};

use crate::ecran_camp::vendre;
use crate::saisie::Entree;

/// `Some(None)` au retour vers le menu du camp.
pub fn choisir(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    loop {
        println!();
        println!("{}", etal(partie));
        println!("  1. Acheter des soins");
        println!("  2. Acheter une capsule d'attaque");
        println!("  3. Vendre un monstre");
        println!("  4. Donner une âme");
        println!(
            "  5. Acheter des capsules d'habileté ({} l'une)",
            argent(PRIX_CAPSULE_D_HABILETE)
        );
        println!("  0. Retour au camp");
        let choix = match entree.choix(0, 5)? {
            0 => return Some(None),
            1 => acheter_soins(partie, entree)?,
            2 => acheter_capsule(partie, contenu, entree)?,
            3 => vendre(partie.collection(), entree)?,
            4 => donner_ame(partie, entree)?,
            _ => acheter_capsules_d_habilete(partie, entree)?,
        };
        if choix.is_some() {
            return Some(choix);
        }
    }
}

/// Le joueur ne choisit que ce qu'il peut payer.
fn acheter_soins(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let en_vente = partie.soins_en_vente();
    println!("Soins en vente :");
    for (position, soin) in en_vente.iter().enumerate() {
        println!("  {}. {}", position + 1, offre_de_soin(*soin));
    }
    println!("Lequel acheter ? (0 pour annuler)");
    let choix = entree.choix(0, en_vente.len())?;
    if choix == 0 {
        return Some(None);
    }
    let soin = en_vente[choix - 1];
    let au_plus = partie.argent() / prix_du_soin(soin);
    if au_plus == 0 {
        println!("Vous n'avez pas assez d'argent pour ce soin.");
        return Some(None);
    }
    println!("Combien en acheter ? (0 pour annuler)");
    let nombre = entree.choix(0, au_plus as usize)?;
    Some((nombre > 0).then_some(Choix::AcheterSoins(soin, nombre as u32)))
}

fn acheter_capsule(
    partie: &Partie,
    contenu: &ContenuV1,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    let en_vente = capsules_en_vente(partie, contenu.capsules());
    if en_vente.is_empty() {
        println!("Vous avez déjà toutes les capsules de Malvoise.");
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

/// Sans confirmation : donner une âme ne fait rien perdre (section 14.2).
fn donner_ame(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let ames = partie.ames_portees();
    if ames.is_empty() {
        println!("Vous ne portez aucune âme.");
        return Some(None);
    }
    println!("Âmes portées :");
    for (position, marchand) in ames.iter().enumerate() {
        println!(
            "  {}. Âme de {} : Malvoise vendra {}",
            position + 1,
            marchand.nom(),
            le_soin(marchand.soin()).to_lowercase()
        );
    }
    println!("Laquelle donner à Malvoise ? (0 pour annuler)");
    let choix = entree.choix(0, ames.len())?;
    Some((choix > 0).then(|| Choix::DonnerAme(ames[choix - 1].id().clone())))
}

/// Le joueur ne choisit que ce qu'il peut payer (section 5.9).
fn acheter_capsules_d_habilete(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let au_plus = partie.argent() / PRIX_CAPSULE_D_HABILETE;
    if au_plus == 0 {
        println!("Vous n'avez pas assez d'argent pour une capsule d'habileté.");
        return Some(None);
    }
    println!("Combien en acheter ? (0 pour annuler)");
    let nombre = entree.choix(0, au_plus as usize)?;
    Some((nombre > 0).then_some(Choix::AcheterCapsulesDHabilete(nombre as u32)))
}
