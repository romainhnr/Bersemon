//! Le coffre du camp (section 13.3) : le joueur y range ce qu'il veut mettre
//! à l'abri, et prend dans le sac ce qu'il emporte en expédition.

use bersemon_core::deroulement::Choix;
use bersemon_core::inventaire::{Inventaire, Objet};
use bersemon_core::partie::Partie;
use bersemon_presentation::boosters::boosters as texte_des_boosters;
use bersemon_presentation::filets::filets as texte_des_filets;
use bersemon_presentation::monstres::decrire_attaque;
use bersemon_presentation::partie::{
    coffre_du_camp, pierres as texte_des_pierres, ressources as texte_des_ressources,
};

use crate::ecran_combat::choisir_soin;
use crate::saisie::Entree;

/// `Some(None)` au retour vers le menu du camp.
pub fn choisir(partie: &Partie, entree: &mut Entree) -> Option<Option<Choix>> {
    let sac = partie.sac();
    let coffre = partie.coffre_du_camp();
    loop {
        println!();
        println!("{}", coffre_du_camp(partie));
        println!("  1. Prendre des soins");
        println!("  2. Ranger des soins");
        println!("  3. Prendre des filets");
        println!("  4. Ranger des filets");
        println!("  5. Prendre de l'argent");
        println!("  6. Ranger de l'argent");
        println!("  7. Prendre une capsule d'attaque");
        println!("  8. Ranger une capsule d'attaque");
        println!("  9. Prendre des capsules d'habileté");
        println!("  10. Ranger des capsules d'habileté");
        println!("  11. Prendre des ressources");
        println!("  12. Ranger des ressources");
        println!("  13. Prendre des pierres d'évolution");
        println!("  14. Ranger des pierres d'évolution");
        println!("  15. Prendre des boosters");
        println!("  16. Ranger des boosters");
        println!("  17. Prendre des Philtres chromatiques");
        println!("  18. Ranger des Philtres chromatiques");
        println!("  19. Prendre des charmes de garde");
        println!("  20. Ranger des charmes de garde");
        println!("  21. Tout ranger dans le coffre");
        println!("  0. Retour au camp");
        let choix = match entree.choix(0, 21)? {
            0 => return Some(None),
            1 => soins(coffre, entree)?.map(Choix::SortirDuCoffre),
            2 => soins(sac, entree)?.map(Choix::RangerAuCoffre),
            3 => filets(coffre, entree)?.map(Choix::SortirDuCoffre),
            4 => filets(sac, entree)?.map(Choix::RangerAuCoffre),
            5 => quantite(coffre.argent(), "pièces d'argent", entree)?
                .map(|montant| Choix::SortirDuCoffre(Objet::Argent(montant))),
            6 => quantite(sac.argent(), "pièces d'argent", entree)?
                .map(|montant| Choix::RangerAuCoffre(Objet::Argent(montant))),
            7 => capsule(coffre, entree)?.map(Choix::SortirDuCoffre),
            8 => capsule(sac, entree)?.map(Choix::RangerAuCoffre),
            9 => quantite(coffre.capsules_d_habilete(), "capsules d'habileté", entree)?
                .map(|nombre| Choix::SortirDuCoffre(Objet::CapsulesDHabilete(nombre))),
            10 => quantite(sac.capsules_d_habilete(), "capsules d'habileté", entree)?
                .map(|nombre| Choix::RangerAuCoffre(Objet::CapsulesDHabilete(nombre))),
            11 => ressources(coffre, entree)?.map(Choix::SortirDuCoffre),
            12 => ressources(sac, entree)?.map(Choix::RangerAuCoffre),
            13 => pierres(coffre, entree)?.map(Choix::SortirDuCoffre),
            14 => pierres(sac, entree)?.map(Choix::RangerAuCoffre),
            15 => boosters(coffre, entree)?.map(Choix::SortirDuCoffre),
            16 => boosters(sac, entree)?.map(Choix::RangerAuCoffre),
            17 => quantite(
                coffre.philtres_chromatiques(),
                "Philtres chromatiques",
                entree,
            )?
            .map(|nombre| Choix::SortirDuCoffre(Objet::PhiltresChromatiques(nombre))),
            18 => quantite(sac.philtres_chromatiques(), "Philtres chromatiques", entree)?
                .map(|nombre| Choix::RangerAuCoffre(Objet::PhiltresChromatiques(nombre))),
            19 => quantite(coffre.charmes_de_garde(), "charmes de garde", entree)?
                .map(|nombre| Choix::SortirDuCoffre(Objet::CharmesDeGarde(nombre))),
            20 => quantite(sac.charmes_de_garde(), "charmes de garde", entree)?
                .map(|nombre| Choix::RangerAuCoffre(Objet::CharmesDeGarde(nombre))),
            _ => Some(Choix::ToutRangerAuCoffre),
        };
        if choix.is_some() {
            return Some(choix);
        }
    }
}

/// Au plus ce qu'il y a ; `Some(None)` s'il n'y a rien ou si le joueur
/// annule.
fn quantite(disponible: u32, objets: &str, entree: &mut Entree) -> Option<Option<u32>> {
    if disponible == 0 {
        println!("Il n'y en a pas.");
        return Some(None);
    }
    println!("Combien de {objets} ? (0 pour annuler)");
    let nombre = entree.choix(0, disponible as usize)?;
    Some((nombre > 0).then_some(nombre as u32))
}

fn soins(inventaire: &Inventaire, entree: &mut Entree) -> Option<Option<Objet>> {
    let Some(soin) = choisir_soin(inventaire.soins(), entree)? else {
        return Some(None);
    };
    let nombre = quantite(inventaire.soins().nombre(soin), "soins", entree)?;
    Some(nombre.map(|nombre| Objet::Soins(soin, nombre)))
}

fn filets(inventaire: &Inventaire, entree: &mut Entree) -> Option<Option<Objet>> {
    let presents: Vec<_> = inventaire.filets().presents().collect();
    if presents.is_empty() {
        println!("Il n'y a aucun filet.");
        return Some(None);
    }
    for (position, (filet, nombre)) in presents.iter().enumerate() {
        println!("  {}. {}", position + 1, texte_des_filets(*filet, *nombre));
    }
    println!("Lequel ? (0 pour annuler)");
    let choix = entree.choix(0, presents.len())?;
    if choix == 0 {
        return Some(None);
    }
    let (filet, disponible) = presents[choix - 1];
    let nombre = quantite(disponible, "filets", entree)?;
    Some(nombre.map(|nombre| Objet::Filets(filet, nombre)))
}

fn ressources(inventaire: &Inventaire, entree: &mut Entree) -> Option<Option<Objet>> {
    let presentes = inventaire.ressources();
    if presentes.is_empty() {
        println!("Il n'y a aucune ressource.");
        return Some(None);
    }
    for (position, (ressource, nombre)) in presentes.iter().enumerate() {
        println!(
            "  {}. {}",
            position + 1,
            texte_des_ressources(ressource, *nombre)
        );
    }
    println!("Laquelle ? (0 pour annuler)");
    let choix = entree.choix(0, presentes.len())?;
    if choix == 0 {
        return Some(None);
    }
    let (ressource, disponible) = &presentes[choix - 1];
    let nombre = quantite(*disponible, "ressources", entree)?;
    Some(nombre.map(|nombre| Objet::Ressources(ressource.id().clone(), nombre)))
}

fn pierres(inventaire: &Inventaire, entree: &mut Entree) -> Option<Option<Objet>> {
    let presentes = inventaire.pierres();
    if presentes.is_empty() {
        println!("Il n'y a aucune pierre d'évolution.");
        return Some(None);
    }
    for (position, (pierre, nombre)) in presentes.iter().enumerate() {
        println!(
            "  {}. {}",
            position + 1,
            texte_des_pierres(*pierre, *nombre)
        );
    }
    println!("Laquelle ? (0 pour annuler)");
    let choix = entree.choix(0, presentes.len())?;
    if choix == 0 {
        return Some(None);
    }
    let (pierre, disponible) = presentes[choix - 1];
    let nombre = quantite(disponible, "pierres", entree)?;
    Some(nombre.map(|nombre| Objet::Pierres(pierre, nombre)))
}

fn boosters(inventaire: &Inventaire, entree: &mut Entree) -> Option<Option<Objet>> {
    let presents = inventaire.boosters();
    if presents.is_empty() {
        println!("Il n'y a aucun booster.");
        return Some(None);
    }
    for (position, (booster, nombre)) in presents.iter().enumerate() {
        println!(
            "  {}. {}",
            position + 1,
            texte_des_boosters(*booster, *nombre)
        );
    }
    println!("Lequel ? (0 pour annuler)");
    let choix = entree.choix(0, presents.len())?;
    if choix == 0 {
        return Some(None);
    }
    let (booster, disponible) = presents[choix - 1];
    let nombre = quantite(disponible, "boosters", entree)?;
    Some(nombre.map(|nombre| Objet::Boosters(booster, nombre)))
}

fn capsule(inventaire: &Inventaire, entree: &mut Entree) -> Option<Option<Objet>> {
    let capsules = inventaire.capsules();
    if capsules.is_empty() {
        println!("Il n'y a aucune capsule.");
        return Some(None);
    }
    for (position, capsule) in capsules.iter().enumerate() {
        println!("  {}. {}", position + 1, decrire_attaque(capsule.attaque()));
    }
    println!("Laquelle ? (0 pour annuler)");
    let choix = entree.choix(0, capsules.len())?;
    Some((choix > 0).then(|| Objet::Capsule(capsules[choix - 1].id().clone())))
}
