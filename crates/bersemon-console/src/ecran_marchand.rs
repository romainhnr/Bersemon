//! Malvoise, le nécromancien du camp (section 14.2) : soins, filets, capsules,
//! pierres d'évolution, rachat des monstres et des ressources, don des
//! âmes, monstre rendu shiny contre un Philtre chromatique, achat et location de
//! monstres, Rite du sang mêlé. Les achats se font sans confirmation ; la
//! vente, le service shiny, le rite et le retour d'un monstre loué sont
//! confirmés par le déroulement.

use bersemon_core::combat::Cote;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::marchand::{
    PRIX_CAPSULE_D_HABILETE, PRIX_PIERRE_D_EVOLUTION, prix_du_filet, prix_du_soin,
};
use bersemon_core::partie::Partie;
use bersemon_core::pierre::PierreDEvolution;
use bersemon_presentation::beherit::{FORGER_LE_BEHERIT, peut_forger_le_beherit};
use bersemon_presentation::boosters::SERVICE_SHINY;
use bersemon_presentation::filets::{liste_de_filets, offre_de_filet};
use bersemon_presentation::marchand::{
    capsules_en_vente, etal, offre_d_achat, offre_de_capsule, offre_de_location,
    offre_de_ressources, ressources_a_vendre,
};
use bersemon_presentation::monstres::ligne_monstre;
use bersemon_presentation::partie::{argent, collection as liste_des_monstres};
use bersemon_presentation::rite::{etat_du_rite, service_du_rite};
use bersemon_presentation::soins::{le_soin, offre_de_soin};

use crate::ecran_camp::{designer, vendre};
use crate::horloge::maintenant;
use crate::saisie::Entree;

/// `Some(None)` au retour vers le menu du camp.
pub fn choisir(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    loop {
        println!();
        println!("{}", etal(partie));
        println!("  1. Acheter des soins");
        println!("  2. Acheter des filets");
        println!("  3. Acheter une capsule d'attaque");
        println!("  4. Vendre un monstre");
        println!("  5. Donner une âme");
        println!(
            "  6. Acheter des capsules d'habileté ({} l'une)",
            argent(PRIX_CAPSULE_D_HABILETE)
        );
        println!("  7. Vendre des ressources");
        println!(
            "  8. Acheter des pierres d'évolution ({} l'une)",
            argent(PRIX_PIERRE_D_EVOLUTION)
        );
        println!(
            "  9. {SERVICE_SHINY} : vous en avez {}",
            partie.philtres_chromatiques()
        );
        println!("  10. Acheter un monstre");
        println!("  11. Louer un monstre");
        // Le rite en cours se suit et s'accueille par la même entrée.
        let entree_du_rite = etat_du_rite(partie, maintenant()).unwrap_or_else(service_du_rite);
        println!("  12. {entree_du_rite}");
        // Le retour d'un monstre loué n'est proposé que s'il y en a un.
        let loue = partie.collection().monstre_loue();
        if let Some((_, monstre)) = loue {
            println!("  13. Rendre {} à Malvoise", monstre.nom());
        }
        println!("  0. Retour au camp");
        let dernier = if loue.is_some() { 13 } else { 12 };
        let choix = match entree.choix(0, dernier)? {
            0 => return Some(None),
            1 => acheter_soins(partie, entree)?,
            2 => acheter_filets(partie, entree)?,
            3 => acheter_capsule(partie, contenu, entree)?,
            4 => vendre(partie.collection(), entree)?,
            5 => donner_ame(partie, contenu, entree)?,
            6 => acheter_capsules_d_habilete(partie, entree)?,
            7 => vendre_ressources(partie, entree)?,
            8 => acheter_pierres(partie, entree)?,
            9 => designer(partie.collection(), entree, "Quel monstre rendre shiny ?")?
                .map(Choix::RendreShiny),
            10 => acheter_monstre(partie, contenu, entree)?,
            11 => louer_monstre(partie, contenu, entree)?,
            12 => rite(partie, contenu, entree)?,
            _ => Some(Choix::RendreLeMonstreLoue),
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

/// Le filet de chanvre et le filet lesté ; le joueur ne choisit que ce
/// qu'il peut payer (section 14.2).
fn acheter_filets(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let en_vente = partie.filets_en_vente();
    println!(
        "Filets en vente (vous en avez {}) :",
        liste_des_filets(partie)
    );
    for (position, filet) in en_vente.iter().enumerate() {
        let offre = offre_de_filet(*filet).unwrap_or_default();
        println!("  {}. {offre}", position + 1);
    }
    println!("Lequel acheter ? (0 pour annuler)");
    let choix = entree.choix(0, en_vente.len())?;
    if choix == 0 {
        return Some(None);
    }
    let filet = en_vente[choix - 1];
    let au_plus = prix_du_filet(filet).map_or(0, |prix| partie.argent() / prix);
    if au_plus == 0 {
        println!("Vous n'avez pas assez d'argent pour ce filet.");
        return Some(None);
    }
    println!("Combien en acheter ? (0 pour annuler)");
    let nombre = entree.choix(0, au_plus as usize)?;
    Some((nombre > 0).then_some(Choix::AcheterFilets(filet, nombre as u32)))
}

/// "10 filets de chanvre, 2 filets lestés", du sac et du coffre, ou
/// "aucun".
fn liste_des_filets(partie: &Partie) -> String {
    let liste = liste_de_filets(&partie.filets());
    if liste.is_empty() {
        "aucun".to_string()
    } else {
        liste.join(", ")
    }
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
/// Les âmes des Apôtres, une fois toutes réunies, se donnent ensemble pour
/// forger le Béhérit, après confirmation (section 9.4).
fn donner_ame(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    let ames = partie.ames_portees();
    let beherit = peut_forger_le_beherit(partie, contenu);
    if ames.is_empty() && !beherit {
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
    if beherit {
        println!("  {}. {FORGER_LE_BEHERIT}", ames.len() + 1);
    }
    println!("Laquelle donner à Malvoise ? (0 pour annuler)");
    let choix = entree.choix(0, ames.len() + usize::from(beherit))?;
    Some(match choix {
        0 => None,
        numero if numero <= ames.len() => Some(Choix::DonnerAme(ames[numero - 1].id().clone())),
        _ => Some(Choix::ForgerLeBeherit),
    })
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

/// Le joueur ne choisit que ce qu'il peut payer (section 5.4).
fn acheter_pierres(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let au_plus = partie.argent() / PRIX_PIERRE_D_EVOLUTION;
    if au_plus == 0 {
        println!("Vous n'avez pas assez d'argent pour une pierre d'évolution.");
        return Some(None);
    }
    println!("Pierres d'évolution en vente :");
    for (position, pierre) in PierreDEvolution::TOUTES.iter().enumerate() {
        println!("  {}. {}", position + 1, pierre.nom());
    }
    println!("Laquelle acheter ? (0 pour annuler)");
    let choix = entree.choix(0, PierreDEvolution::TOUTES.len())?;
    if choix == 0 {
        return Some(None);
    }
    println!("Combien en acheter ? (0 pour annuler)");
    let nombre = entree.choix(0, au_plus as usize)?;
    let pierre = PierreDEvolution::TOUTES[choix - 1];
    Some((nombre > 0).then_some(Choix::AcheterPierres(pierre, nombre as u32)))
}

/// Le rite en cours s'accueille une fois fini, et sinon le refus dit le
/// temps qu'il reste ; sans rite, le joueur choisit deux parents d'une même
/// famille d'évolution (section 13.11).
fn rite(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    if partie.rite().is_some() {
        return Some(Some(Choix::AccueillirLePetit));
    }
    let collection = partie.collection();
    println!();
    println!("{}", liste_des_monstres(collection));
    let question = "Quel premier parent confier à Malvoise ?";
    let Some(premier) = designer(collection, entree, question)? else {
        return Some(None);
    };
    let compatibles = partie.seconds_parents_possibles(premier, contenu.especes());
    if compatibles.is_empty() {
        println!("Aucun autre de vos monstres n'est de sa famille d'évolution.");
        return Some(None);
    }
    println!("Second parent, de la même famille :");
    for (position, emplacement) in compatibles.iter().enumerate() {
        if let Some(autre) = collection.monstre(*emplacement) {
            println!("  {}. {}", position + 1, ligne_monstre(autre, Cote::Joueur));
        }
    }
    println!("Lequel ? (0 pour annuler)");
    let choix = entree.choix(0, compatibles.len())?;
    Some((choix > 0).then(|| Choix::LancerLeRite {
        premier,
        second: compatibles[choix - 1],
    }))
}

/// Les espèces communes des zones menées au bout (section 14.2).
fn acheter_monstre(
    partie: &Partie,
    contenu: &ContenuV1,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    let offres = partie.offres_d_achat(contenu.zones(), contenu.especes());
    if offres.is_empty() {
        println!("Malvoise n'a encore aucun monstre à vendre : menez une expédition au bout.");
        return Some(None);
    }
    println!("Monstres en vente :");
    for (position, offre) in offres.iter().enumerate() {
        println!("  {}. {}", position + 1, offre_d_achat(offre));
    }
    println!("Lequel acheter ? (0 pour annuler)");
    let choix = entree.choix(0, offres.len())?;
    Some((choix > 0).then(|| Choix::AcheterMonstre(offres[choix - 1].espece.id().clone())))
}

/// Les espèces rares des zones menées au bout (section 14.2).
fn louer_monstre(
    partie: &Partie,
    contenu: &ContenuV1,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    let offres = partie.offres_de_location(contenu.zones(), contenu.piste());
    if offres.is_empty() {
        println!("Malvoise n'a encore aucun monstre à louer.");
        return Some(None);
    }
    println!("Monstres à louer :");
    for (position, offre) in offres.iter().enumerate() {
        println!("  {}. {}", position + 1, offre_de_location(offre));
    }
    println!("Lequel louer ? (0 pour annuler)");
    let choix = entree.choix(0, offres.len())?;
    Some((choix > 0).then(|| Choix::LouerMonstre(offres[choix - 1].espece.id().clone())))
}

/// Toutes celles d'une sorte, du sac et du coffre, sans confirmation
/// (section 14.3).
fn vendre_ressources(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let a_vendre = ressources_a_vendre(partie);
    if a_vendre.is_empty() {
        println!("Vous n'avez aucune ressource.");
        return Some(None);
    }
    println!("Malvoise rachète toutes les ressources d'une sorte, du sac et du coffre :");
    for (position, (ressource, nombre)) in a_vendre.iter().enumerate() {
        println!(
            "  {}. {}",
            position + 1,
            offre_de_ressources(ressource, *nombre)
        );
    }
    println!("Lesquelles vendre ? (0 pour annuler)");
    let choix = entree.choix(0, a_vendre.len())?;
    Some((choix > 0).then(|| Choix::VendreRessources(a_vendre[choix - 1].0.id().clone())))
}
