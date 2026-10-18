//! Carte et expédition manuelle (section 9) : choix de la zone, puis les
//! étapes jusqu'au retour au camp, avec la traque de l'Apôtre (section 9.4).

use bersemon_core::aleatoire::Aleatoire;
use bersemon_core::combat::{Cote, Issue};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::expedition::{EvenementExpedition, Expedition};
use bersemon_core::inventaire::potion_sans_effet;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::traque::Apotre;
use bersemon_core::tutoriel::Conseil;

use crate::affichage;
use crate::ecran_combat;
use crate::saisie::Entree;
use crate::tutoriel::montrer_conseil;

/// Rend `false` si l'entrée est fermée.
pub fn partir(
    partie: &mut Partie,
    contenu: &ContenuV1,
    aleatoire: &mut dyn Aleatoire,
    entree: &mut Entree,
) -> bool {
    let piste = contenu.piste();
    let accessibles = partie.zones_accessibles(contenu.zones(), piste);
    println!();
    println!("{}", affichage::carte(partie, &accessibles));
    println!("  0. Rester au camp");
    let Some(choix) = entree.choix(0, accessibles.len()) else {
        return false;
    };
    if choix == 0 {
        return true;
    }
    let mut expedition = partie
        .partir_en_expedition(accessibles[choix - 1], piste, aleatoire)
        .expect("zone choisie sur la carte");
    let nombre_avant = accessibles.len();
    println!();
    println!("Vous partez pour {}.", expedition.zone().nom());

    let entree_ouverte = explorer(partie, &mut expedition, piste.apotre(), aleatoire, entree);
    let indice = partie.rentrer_au_camp(&expedition, piste);
    println!();
    if expedition.est_reussie() {
        println!(
            "Vous avez exploré {} jusqu'au bout.",
            expedition.zone().nom()
        );
    }
    println!("Retour au camp : vos monstres sont soignés.");
    if let Some(indice) = indice {
        println!();
        println!("Indice {} : {}", partie.indices_trouves(), indice.texte);
    }
    let accessibles = partie.zones_accessibles(contenu.zones(), piste);
    if let Some(nouvelle) = accessibles.get(nombre_avant) {
        println!(
            "Une nouvelle zone apparaît sur la carte : {}.",
            nouvelle.nom()
        );
    }
    montrer_conseil(partie, Conseil::PremierRetourAuCamp);
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
    apotre: &Apotre,
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
            EvenementExpedition::Apotre(monstre) => {
                affronter_l_apotre(partie, expedition, apotre, monstre, aleatoire, entree)
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

/// Rend `false` si l'entrée est fermée.
fn combattre(
    partie: &mut Partie,
    expedition: &mut Expedition,
    groupe: Vec<Monstre>,
    aleatoire: &mut dyn Aleatoire,
    entree: &mut Entree,
) -> bool {
    let fin = ecran_combat::combattre(partie, groupe, aleatoire, entree);
    noter_defaite(expedition, fin.issue);
    fin.entree_ouverte
}

/// Après la victoire, l'épilogue annonce la suite de la traque (section 6).
/// Rend `false` si l'entrée est fermée.
fn affronter_l_apotre(
    partie: &mut Partie,
    expedition: &mut Expedition,
    apotre: &Apotre,
    monstre: Monstre,
    aleatoire: &mut dyn Aleatoire,
    entree: &mut Entree,
) -> bool {
    println!("{}", apotre.avant_combat());
    let fin = ecran_combat::combattre_l_apotre(partie, monstre, aleatoire, entree);
    if fin.issue == Some(Issue::Victoire) {
        partie.noter_apotre_vaincu(apotre);
        println!();
        println!("{}", apotre.epilogue());
    }
    noter_defaite(expedition, fin.issue);
    fin.entree_ouverte
}

/// Une défaite met fin à l'expédition et fait perdre les potions (section
/// 9.3).
fn noter_defaite(expedition: &mut Expedition, issue: Option<Issue>) {
    if issue == Some(Issue::Defaite) {
        expedition.noter_defaite();
        println!();
        println!("Toute votre équipe est KO : vous rentrez au camp et perdez vos potions.");
    }
}

fn entre_deux_etapes(
    partie: &mut Partie,
    aleatoire: &mut dyn Aleatoire,
    entree: &mut Entree,
) -> Decision {
    montrer_conseil(partie, Conseil::PremierArretEntreDeuxEtapes);
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
        let poursuivre = match entree.choix(1, 4) {
            None => return Decision::EntreeFermee,
            Some(1) => return Decision::Continuer,
            Some(2) => utiliser_potion(partie, aleatoire, entree),
            Some(3) => changer_ordre(partie, entree),
            _ => return Decision::Rentrer,
        };
        if !poursuivre {
            return Decision::EntreeFermee;
        }
    }
}

/// Rend `false` si l'entrée est fermée.
fn utiliser_potion(
    partie: &mut Partie,
    aleatoire: &mut dyn Aleatoire,
    entree: &mut Entree,
) -> bool {
    println!("Soigner quel monstre ? (0 pour annuler)");
    let Some(choix) = entree.choix(0, partie.collection().equipe().len()) else {
        return false;
    };
    if choix == 0 {
        return true;
    }
    let monstre = &partie.collection().equipe()[choix - 1];
    // Sur un monstre qui a tous ses PV, la potion serait perdue : le joueur
    // confirme d'abord (Q53).
    if partie.inventaire().potions() > 0 && potion_sans_effet(monstre) {
        match entree.confirmation(&affichage::potion_perdue(monstre)) {
            None => return false,
            Some(false) => return true,
            Some(true) => {}
        }
    }
    match partie.utiliser_potion(choix - 1, aleatoire) {
        Ok(pv) => {
            let nom = affichage::nom(&partie.collection().equipe()[choix - 1], Cote::Joueur);
            println!("{}", affichage::soin_de_potion(&nom, pv));
        }
        Err(erreur) => println!("{}", affichage::erreur_potion(&erreur)),
    }
    true
}

/// Le premier monstre en état de combattre commencera le prochain combat ;
/// les échanges avec le camp restent impossibles en expédition (Q55). Rend
/// `false` si l'entrée est fermée.
fn changer_ordre(partie: &mut Partie, entree: &mut Entree) -> bool {
    let taille = partie.collection().equipe().len();
    println!("Quel monstre déplacer ? (0 pour annuler)");
    let Some(de) = entree.choix(0, taille) else {
        return false;
    };
    if de == 0 {
        return true;
    }
    println!("À quelle place le mettre ?");
    let Some(vers) = entree.choix(1, taille) else {
        return false;
    };
    if let Err(erreur) = partie
        .collection_mut()
        .deplacer_dans_l_equipe(de - 1, vers - 1)
    {
        println!("{}", affichage::erreur_collection(&erreur));
    }
    true
}
