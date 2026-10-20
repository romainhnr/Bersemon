//! Expéditions automatiques sur Discord (section 12) : les zones explorées et
//! leur état, puis la préparation d'un départ. Les monstres s'ajoutent un par
//! un depuis les menus de monstres, car un seul menu ne peut pas proposer les
//! 33 monstres d'une équipe et d'un camp pleins ; la durée et les chances
//! s'affichent dès le premier (section 12.4).

use bersemon_core::collection::Emplacement;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::expedition_automatique::ExpeditionAutomatique;
use bersemon_core::expedition_automatique::Protection;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::zone::Zone;
use bersemon_presentation::expeditions_automatiques::{
    erreur_expedition_automatique, estimation, ligne_de_protection, noms,
};

use crate::composants::{Choisis, Cle, Vue};
use crate::ecran_camp::{menus_de_monstres, retour};
use crate::message::{Bouton, DANGER, PRINCIPAL, Question, SECONDAIRE, SUCCES, bouton, rangees};

const PRESENTATION: &str =
    "Expéditions automatiques : vos monstres partent seuls dans une zone explorée.";

/// Les zones explorées, chacune avec le bouton que permet son état.
pub fn liste(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> Question {
    let mut lignes = vec![PRESENTATION.to_string()];
    let mut boutons = Vec::new();
    for (position, zone) in contenu.zones().iter().enumerate() {
        if !partie.est_exploree(zone.id()) {
            continue;
        }
        match partie.expedition_automatique(zone.id()) {
            None => {
                lignes.push(format!("{} : libre", zone.nom()));
                boutons.push(bouton(
                    Cle::Afficher(Vue::Preparation {
                        zone: position,
                        choisis: Choisis::default(),
                    }),
                    &format!("Envoyer dans : {}", zone.nom()),
                    PRINCIPAL,
                ));
            }
            Some(expedition) if expedition.est_rentree(maintenant) => {
                lignes.push(format!(
                    "{} : rentrée, à accueillir ({})",
                    zone.nom(),
                    noms(expedition.monstres())
                ));
                boutons.push(bouton(
                    Cle::Accueillir(zone.id().clone()),
                    &format!("Accueillir : {}", zone.nom()),
                    PRINCIPAL,
                ));
            }
            Some(expedition) => {
                lignes.push(en_cours(zone, expedition));
                boutons.push(bouton(
                    Cle::Rappeler(zone.id().clone()),
                    &format!("Rappeler : {}", zone.nom()),
                    DANGER,
                ));
            }
        }
    }
    if boutons.is_empty() {
        lignes.push(
            "Aucune zone n'est encore explorée : menez d'abord une expédition jusqu'au bout."
                .to_string(),
        );
    } else {
        boutons.push(bouton(
            Cle::Afficher(Vue::Expeditions),
            "Actualiser",
            SECONDAIRE,
        ));
    }
    boutons.push(retour());
    Question {
        texte: lignes.join("\n"),
        rangees: rangees(boutons),
    }
}

/// Discord affiche l'heure de retour en temps relatif (`<t:...:R>`) et la
/// met à jour tout seul.
fn en_cours(zone: &Zone, expedition: &ExpeditionAutomatique) -> String {
    format!(
        "{} : en cours, retour <t:{}:R> ({})",
        zone.nom(),
        expedition.retour().secondes(),
        noms(expedition.monstres())
    )
}

/// Avec au moins deux monstres : de quoi protéger chacun des autres, ou
/// n'en protéger aucun, puis donner ou reprendre un charme de garde s'il y
/// en a un (section 12.5). Au plus 4 boutons.
fn boutons_de_protection(
    partie: &Partie,
    position: usize,
    choisis: Choisis,
    monstres: &[Monstre],
) -> Vec<Bouton> {
    if monstres.len() < 2 {
        return Vec::new();
    }
    let protection = choisis.protection();
    let vers = |protege: Option<u8>, charme: bool| {
        Cle::Afficher(Vue::Preparation {
            zone: position,
            choisis: choisis.avec_protection(protege, charme),
        })
    };
    let mut boutons: Vec<Bouton> = monstres
        .iter()
        .enumerate()
        .filter(|(rang, _)| protection.protege != Some(*rang))
        .filter_map(|(rang, monstre)| {
            let rang = u8::try_from(rang).ok()?;
            let libelle = format!("Protéger {}", monstre.nom());
            Some(bouton(
                vers(Some(rang), protection.charme),
                &libelle,
                SECONDAIRE,
            ))
        })
        .collect();
    if let Some(protege) = protection.protege.and_then(|rang| u8::try_from(rang).ok()) {
        boutons.push(bouton(vers(None, false), "Sans protection", SECONDAIRE));
        if protection.charme {
            boutons.push(bouton(
                vers(Some(protege), false),
                "Sans charme",
                SECONDAIRE,
            ));
        } else if partie.charmes_de_garde() > 0 {
            boutons.push(bouton(
                vers(Some(protege), true),
                "Avec un charme de garde",
                SUCCES,
            ));
        }
    }
    boutons
}

/// Une zone qui n'est plus libre ramène à la liste, par exemple après le
/// départ de l'expédition.
pub fn preparation(
    partie: &Partie,
    contenu: &ContenuV1,
    position: usize,
    choisis: Choisis,
    maintenant: Horodatage,
) -> Question {
    let zone = contenu
        .zones()
        .get(position)
        .filter(|zone| partie.est_exploree(zone.id()))
        .filter(|zone| partie.expedition_automatique(zone.id()).is_none());
    let Some(zone) = zone else {
        return liste(partie, contenu, maintenant);
    };
    let collection = partie.collection();
    let emplacements = choisis.liste();
    let mut lignes = vec![format!("Expédition automatique : {}", zone.nom())];
    let mut boutons = Vec::new();
    if emplacements.is_empty() {
        lignes.push("Choisissez de 1 à 3 monstres dans les menus.".to_string());
    } else {
        let monstres: Vec<_> = emplacements
            .iter()
            .filter_map(|emplacement| collection.monstre(*emplacement).cloned())
            .collect();
        lignes.push(format!("Monstres choisis : {}.", noms(&monstres)));
        let protection = choisis.protection();
        match partie.estimer_expedition_automatique(zone, &emplacements, protection) {
            Ok(previsions) => {
                lignes.push(estimation(zone, &previsions));
                if monstres.len() > 1 {
                    lignes.push(ligne_de_protection(&monstres, protection));
                }
                boutons.push(bouton(
                    Cle::Expedier {
                        zone: zone.id().clone(),
                        choisis,
                    },
                    "Envoyer",
                    PRINCIPAL,
                ));
            }
            Err(erreur) => lignes.push(erreur_expedition_automatique(&erreur)),
        }
        boutons.push(bouton(
            Cle::Afficher(Vue::Preparation {
                zone: position,
                choisis: Choisis::default(),
            }),
            "Recommencer",
            SECONDAIRE,
        ));
        boutons.extend(boutons_de_protection(partie, position, choisis, &monstres));
    }
    // Proposé seulement si ce départ est possible : il faut une place au
    // camp pour chaque monstre de l'équipe à son retour. L'équipe reste vide
    // (section 12.1).
    let toute_l_equipe = (0..collection.equipe().len())
        .map(Emplacement::Equipe)
        .try_fold(Choisis::default(), Choisis::avec);
    if let Some(toute_l_equipe) = toute_l_equipe {
        let possible = partie
            .estimer_expedition_automatique(zone, &toute_l_equipe.liste(), Protection::default())
            .is_ok();
        if possible && toute_l_equipe != choisis {
            boutons.push(bouton(
                Cle::Afficher(Vue::Preparation {
                    zone: position,
                    choisis: toute_l_equipe,
                }),
                "Toute l'équipe",
                SECONDAIRE,
            ));
        }
    }
    boutons.push(bouton(
        Cle::Afficher(Vue::Expeditions),
        "Retour",
        SECONDAIRE,
    ));
    let mut lignes_de_composants = Vec::new();
    if !choisis.est_complet() {
        lignes_de_composants = menus_de_monstres(
            collection,
            |menu| Cle::Ajouter {
                menu,
                zone: position,
                choisis,
            },
            "Ajouter",
            |emplacement| !choisis.contient(emplacement),
        );
    }
    lignes_de_composants.extend(rangees(boutons));
    Question {
        texte: lignes.join("\n"),
        rangees: lignes_de_composants,
    }
}
