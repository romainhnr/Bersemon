//! Mise en texte des événements du déroulement, dans l'ordre où ils
//! arrivent.

use bersemon_core::combat::Cote;
use bersemon_core::contenu::{INTRODUCTION, texte_du_conseil};
use bersemon_core::deroulement::Evenement;

use crate::affichage;

pub fn raconter(evenements: &[Evenement]) {
    let mut precedent = None;
    for evenement in evenements {
        raconter_un(evenement, precedent);
        precedent = Some(evenement);
    }
}

fn raconter_un(evenement: &Evenement, precedent: Option<&Evenement>) {
    match evenement {
        Evenement::Introduction => {
            println!();
            println!("{INTRODUCTION}");
            println!();
        }
        Evenement::PartieCommencee(depart) => {
            println!("{} vous accompagne désormais.", depart.nom());
        }
        Evenement::Depart(zone) => {
            println!();
            println!("Vous partez pour {zone}.");
        }
        Evenement::Etape(numero) => {
            println!();
            println!("Étape {numero}.");
        }
        Evenement::Coffre { potions } => {
            println!("Vous trouvez un coffre : {}.", affichage::potions(*potions));
        }
        Evenement::MonstreRarePacifique(monstre) => println!(
            "Un monstre rare apparaît : {}. Il ne semble pas hostile.",
            affichage::nom_sauvage(monstre)
        ),
        Evenement::MonstreRareHostile(monstre) => println!(
            "Un monstre rare vous attaque : {} !",
            affichage::nom_sauvage(monstre)
        ),
        Evenement::MonstreRareLaisse => println!("Vous le laissez partir."),
        Evenement::RecitDeLApotre(recit) => println!("{recit}"),
        Evenement::Rencontre(combat) => {
            println!();
            println!("{}", affichage::rencontre(combat));
        }
        Evenement::Combat { evenements, combat } => {
            for evenement in evenements {
                println!("{}", affichage::evenement(evenement, combat));
            }
        }
        Evenement::Experience { monstre, rapport } => {
            println!();
            println!("{}", affichage::rapport_experience(monstre, rapport));
        }
        Evenement::AttaqueRemplacee {
            monstre,
            oubliee,
            nouvelle,
        } => println!("{}", affichage::remplacement(monstre, oubliee, nouvelle)),
        Evenement::AttaqueAbandonnee { monstre, nouvelle } => {
            println!("{}", affichage::renoncement(monstre, nouvelle));
        }
        Evenement::RejointLEquipe(monstre) => println!("{} rejoint votre équipe.", monstre.nom()),
        Evenement::PartPourLeCamp(monstre) => println!("{} part pour le camp.", monstre.nom()),
        Evenement::CaptureRelachee(monstre) | Evenement::Relache(monstre) => {
            println!("Vous relâchez {}.", monstre.nom());
        }
        Evenement::RelachePourLaCapture { relache, capture } => println!(
            "Vous relâchez {}. {} prend sa place.",
            relache.nom(),
            capture.nom()
        ),
        Evenement::Epilogue(epilogue) => {
            println!();
            println!("{epilogue}");
        }
        Evenement::Defaite => {
            println!();
            println!("Toute votre équipe est KO : vous rentrez au camp et perdez vos potions.");
        }
        Evenement::PotionUtilisee { monstre, pv_rendus } => {
            let nom = affichage::nom(monstre, Cote::Joueur);
            println!("{}", affichage::soin_de_potion(&nom, *pv_rendus));
        }
        Evenement::ZoneExploree(zone) => {
            println!();
            println!("Vous avez exploré {zone} jusqu'au bout.");
        }
        Evenement::RetourAuCamp => {
            if !matches!(precedent, Some(Evenement::ZoneExploree(_))) {
                println!();
            }
            println!("Retour au camp : vos monstres sont soignés.");
        }
        Evenement::Indice { numero, texte } => {
            println!();
            println!("Indice {numero} : {texte}");
        }
        Evenement::NouvelleZone(zone) => {
            println!("Une nouvelle zone apparaît sur la carte : {zone}.");
        }
        Evenement::Conseil(conseil) => {
            println!();
            println!("{}", texte_du_conseil(*conseil));
        }
        Evenement::SauvegardeImpossible(erreur) => println!(
            "Sauvegarde impossible : {}.",
            affichage::erreur_sauvegarde(erreur)
        ),
    }
}
