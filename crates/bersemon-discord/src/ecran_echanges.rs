//! Écran des échanges (section 15.1) : l'offre reçue, à accepter ou à
//! refuser, l'offre envoyée, à retirer, puis, sans offre envoyée, une offre à
//! faire en trois temps : le joueur, le monstre à lui céder, et celui qu'on
//! lui demande, ou aucun pour un don.

use bersemon_core::collection::{Collection, Emplacement};
use bersemon_core::espece::Rarete;
use bersemon_core::partie::Partie;
use bersemon_presentation::echanges::{
    decrire_offre_envoyee, decrire_offre_recue, joueur, nom_et_niveau,
};

use crate::composants::{Cle, Vue};
use crate::echanges::{AutreJoueur, TableauDesEchanges};
use crate::ecran_camp::{menus_de_monstres, retour};
use crate::joueurs::IdJoueur;
use crate::message::{
    DANGER, OptionDeMenu, PRINCIPAL, Question, Rangee, SECONDAIRE, SUCCES, bouton, menu, rangees,
};

fn qui(autre: &AutreJoueur) -> String {
    joueur(&autre.heros, autre.pseudo.as_deref())
}

/// Un Apôtre ne s'offre ni ne se demande : il n'est pas proposé.
fn n_est_pas_un_apotre(collection: &Collection) -> impl Fn(Emplacement) -> bool + '_ {
    move |emplacement| {
        collection
            .monstre(emplacement)
            .is_some_and(|monstre| monstre.espece().rarete() != Rarete::Apotre)
    }
}

pub fn echanges(
    partie: &Partie,
    tableau: &TableauDesEchanges,
    avec: Option<IdJoueur>,
    cede: Option<Emplacement>,
) -> Question {
    let mut paragraphes = Vec::new();
    let mut lignes: Vec<Rangee> = Vec::new();
    let mut boutons = Vec::new();
    if let Some((autre, offre)) = &tableau.recue {
        paragraphes.push(decrire_offre_recue(&qui(autre), offre));
        boutons.push(bouton(Cle::AccepterOffre, "Accepter l'offre", SUCCES));
        boutons.push(bouton(Cle::RefuserOffre, "Refuser l'offre", DANGER));
    }
    let choisi = tableau
        .choisi
        .as_ref()
        .filter(|(autre, _)| Some(autre.id) == avec);
    match (&tableau.envoyee, choisi) {
        (Some((autre, offre)), _) => {
            paragraphes.push(decrire_offre_envoyee(&qui(autre), offre));
            boutons.push(bouton(Cle::RetirerOffre, "Retirer l'offre", SECONDAIRE));
        }
        (None, None) => {
            let options: Vec<OptionDeMenu> = tableau
                .joueurs
                .iter()
                .map(|autre| OptionDeMenu {
                    libelle: autre.heros.clone(),
                    valeur: autre.id.to_string(),
                    description: autre.pseudo.clone(),
                })
                .collect();
            if options.is_empty() {
                paragraphes.push("Aucun autre joueur n'a de partie sur le bot.".to_string());
            } else {
                paragraphes.push(
                    "Choisissez un joueur pour lui donner un monstre ou lui en proposer un échange."
                        .to_string(),
                );
            }
            lignes.extend(menu(Cle::ChoisirJoueur, "Choisir un joueur", options));
        }
        (None, Some((autre, collection))) => {
            let id = autre.id;
            let a_moi = partie.collection();
            match cede.and_then(|cede| a_moi.monstre(cede).map(|monstre| (cede, monstre))) {
                None => {
                    paragraphes.push(format!(
                        "Offre à {} : choisissez le monstre à céder.",
                        qui(autre)
                    ));
                    lignes.extend(menus_de_monstres(
                        a_moi,
                        |menu| Cle::Ceder { menu, avec: id },
                        "Céder",
                        n_est_pas_un_apotre(a_moi),
                    ));
                }
                Some((cede, monstre)) => {
                    paragraphes.push(format!(
                        "Offre à {} : vous cédez {}. Choisissez le monstre demandé en échange, ou donnez-le sans contrepartie.",
                        qui(autre),
                        nom_et_niveau(monstre)
                    ));
                    lignes.extend(menus_de_monstres(
                        collection,
                        |menu| Cle::Demander {
                            menu,
                            avec: id,
                            cede,
                        },
                        "Demander",
                        n_est_pas_un_apotre(collection),
                    ));
                    boutons.push(bouton(
                        Cle::Donner { avec: id, cede },
                        "Le donner sans contrepartie",
                        PRINCIPAL,
                    ));
                }
            }
            boutons.push(bouton(
                Cle::Afficher(Vue::Echanges {
                    avec: None,
                    cede: None,
                }),
                "Annuler l'offre",
                SECONDAIRE,
            ));
        }
    }
    boutons.push(retour());
    lignes.extend(rangees(boutons));
    Question {
        texte: format!(
            "Échanges avec les autres joueurs.\n\n{}",
            paragraphes.join("\n\n")
        ),
        rangees: lignes,
    }
}
