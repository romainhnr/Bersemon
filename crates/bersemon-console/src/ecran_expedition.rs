//! Carte et expédition manuelle (section 9) : choix de la zone, puis les
//! étapes jusqu'au retour au camp.

use bersemon_core::aleatoire::Aleatoire;
use bersemon_core::combat::Issue;
use bersemon_core::expedition::{EvenementExpedition, Expedition};
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::zone::Zone;

use crate::affichage;
use crate::ecran_combat;
use crate::saisie::Entree;

/// Rend `false` si l'entrée est fermée.
pub fn partir(
    partie: &mut Partie,
    zones: &[Zone],
    aleatoire: &mut dyn Aleatoire,
    entree: &mut Entree,
) -> bool {
    let accessibles = partie.zones_accessibles(zones);
    println!();
    println!("{}", affichage::carte(partie, &accessibles));
    println!("  0. Rester au camp");
    let Some(choix) = entree.choix(0, accessibles.len()) else {
        return false;
    };
    if choix == 0 {
        return true;
    }
    let mut expedition = Expedition::partir(accessibles[choix - 1].clone(), aleatoire);
    let nombre_avant = accessibles.len();
    println!();
    println!("Vous partez pour {}.", expedition.zone().nom());

    let entree_ouverte = explorer(partie, &mut expedition, aleatoire, entree);
    partie.rentrer_au_camp(&expedition);
    println!();
    if expedition.est_reussie() {
        println!(
            "Vous avez exploré {} jusqu'au bout.",
            expedition.zone().nom()
        );
    }
    println!("Retour au camp : vos monstres sont soignés.");
    let accessibles = partie.zones_accessibles(zones);
    if let Some(nouvelle) = accessibles.get(nombre_avant) {
        println!(
            "Une nouvelle zone apparaît sur la carte : {}.",
            nouvelle.nom()
        );
    }
    entree_ouverte
}

/// Ce que décide le joueur entre deux étapes.
enum Decision {
    Continuer,
    Rentrer,
    EntreeFermee,
}

/// Rend `false` si l'entrée est fermée.
fn explorer(
    partie: &mut Partie,
    expedition: &mut Expedition,
    aleatoire: &mut dyn Aleatoire,
    entree: &mut Entree,
) -> bool {
    while let Some(evenement) = expedition.etape_suivante(aleatoire) {
        println!();
        println!("Étape {}.", expedition.etapes_faites());
        let entree_ouverte = match evenement {
            EvenementExpedition::Combat(groupe) => {
                combattre(partie, expedition, groupe, aleatoire, entree)
            }
            EvenementExpedition::Coffre { potions } => {
                partie.ouvrir_coffre(potions);
                println!("Vous trouvez un coffre : {}.", affichage::potions(potions));
                true
            }
            EvenementExpedition::MonstreRare(rare) => {
                partie.noter_rencontre(&rare.monstre);
                let nom = affichage::nom_sauvage(&rare.monstre);
                if rare.pacifique {
                    println!("Un monstre rare apparaît : {nom}. Il ne semble pas hostile.");
                    match entree.confirmation("Le prendre avec vous ?") {
                        None => false,
                        Some(true) => {
                            let placement = partie.prendre_monstre_rare(rare.monstre);
                            ecran_combat::traiter_placement(
                                partie.collection_mut(),
                                placement,
                                true,
                                entree,
                            )
                        }
                        Some(false) => {
                            println!("Vous le laissez partir.");
                            true
                        }
                    }
                } else {
                    println!("Un monstre rare vous attaque : {nom} !");
                    combattre(partie, expedition, vec![rare.monstre], aleatoire, entree)
                }
            }
        };
        if !entree_ouverte {
            return false;
        }
        if expedition.est_terminee() {
            return true;
        }
        match entre_deux_etapes(partie, aleatoire, entree) {
            Decision::Continuer => {}
            Decision::Rentrer => return true,
            Decision::EntreeFermee => return false,
        }
    }
    true
}

/// Une défaite met fin à l'expédition et fait perdre les potions (section
/// 9.3). Rend `false` si l'entrée est fermée.
fn combattre(
    partie: &mut Partie,
    expedition: &mut Expedition,
    groupe: Vec<Monstre>,
    aleatoire: &mut dyn Aleatoire,
    entree: &mut Entree,
) -> bool {
    let fin = ecran_combat::combattre(partie, groupe, aleatoire, entree);
    if fin.issue == Some(Issue::Defaite) {
        expedition.noter_defaite();
        println!();
        println!("Toute votre équipe est KO : vous rentrez au camp et perdez vos potions.");
    }
    fin.entree_ouverte
}

fn entre_deux_etapes(
    partie: &mut Partie,
    aleatoire: &mut dyn Aleatoire,
    entree: &mut Entree,
) -> Decision {
    loop {
        println!();
        println!("{}", affichage::equipe_en_expedition(partie));
        println!("Que faire ?");
        println!("  1. Continuer l'expédition");
        println!(
            "  2. Utiliser une potion ({})",
            partie.inventaire().potions()
        );
        println!("  3. Rentrer au camp");
        match entree.choix(1, 3) {
            None => return Decision::EntreeFermee,
            Some(1) => return Decision::Continuer,
            Some(2) => {
                println!("Soigner quel monstre ? (0 pour annuler)");
                let Some(choix) = entree.choix(0, partie.collection().equipe().len()) else {
                    return Decision::EntreeFermee;
                };
                if choix == 0 {
                    continue;
                }
                match partie.utiliser_potion(choix - 1, aleatoire) {
                    Ok(pv) => println!(
                        "{} récupère {pv} PV.",
                        partie.collection().equipe()[choix - 1].nom()
                    ),
                    Err(erreur) => println!("{}", affichage::erreur_potion(&erreur)),
                }
            }
            _ => return Decision::Rentrer,
        }
    }
}
