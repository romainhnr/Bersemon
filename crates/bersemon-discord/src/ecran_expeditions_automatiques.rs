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
use bersemon_core::partie::Partie;
use bersemon_core::zone::Zone;
use bersemon_presentation::expeditions_automatiques::{
    erreur_expedition_automatique, estimation, noms,
};

use crate::composants::{Choisis, Cle, Vue};
use crate::ecran_camp::{menus_de_monstres, retour};
use crate::message::{DANGER, PRINCIPAL, Question, SECONDAIRE, bouton, rangees};

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
        match partie.estimer_expedition_automatique(zone, &emplacements, Protection::default()) {
            Ok(previsions) => {
                lignes.push(estimation(zone, &previsions));
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
