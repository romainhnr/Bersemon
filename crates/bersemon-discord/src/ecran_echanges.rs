//! Écran des autres joueurs (sections 15.1 et 15.3) : l'offre et le défi
//! reçus, à accepter ou à refuser, l'offre et le défi envoyés, à retirer,
//! puis, sans offre envoyée, une offre à faire en trois temps : le joueur,
//! le monstre à lui céder, et celui qu'on lui demande, ou aucun pour un
//! don. Une fois le joueur choisi, il peut aussi être défié en duel, et
//! ajouté aux amis ou retiré (section 15.4). Le palmarès du joueur est en
//! tête, et le classement des duels s'ouvre d'un bouton, comme celui des
//! records de la Tour sans fond (section 9.5).

use bersemon_core::collection::{Collection, Emplacement};
use bersemon_core::espece::Rarete;
use bersemon_core::partie::Partie;
use bersemon_core::relations::Palmares;
use bersemon_core::tour::RecordsDeLaTour;
use bersemon_presentation::duels::{classement as texte_du_classement, duels_de, vos_duels};
use bersemon_presentation::echanges::{
    decrire_offre_envoyee, decrire_offre_recue, joueur, nom_et_niveau,
};
use bersemon_presentation::groupe::{
    ACCEPTER_L_INVITATION, INVITER_DANS_LA_TOUR, REFUSER_L_INVITATION, RETIRER_L_INVITATION,
    invitation_envoyee, vous_invite,
};
use bersemon_presentation::tour::records_de_la_tour as texte_des_records;

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
    if let Some(autre) = &tableau.defi_recu {
        paragraphes.push(format!("{} vous défie en duel.", qui(autre)));
        boutons.push(bouton(Cle::AccepterDefi, "Accepter le défi", SUCCES));
        boutons.push(bouton(Cle::RefuserDefi, "Refuser le défi", DANGER));
    }
    if let Some(autre) = &tableau.defi_envoye {
        paragraphes.push(format!(
            "Vous avez défié {} en duel. En attente de sa réponse.",
            qui(autre)
        ));
        boutons.push(bouton(Cle::RetirerDefi, "Retirer le défi", SECONDAIRE));
    }
    if let Some(autre) = &tableau.invitation_recue {
        paragraphes.push(vous_invite(&qui(autre)));
        boutons.push(bouton(
            Cle::AccepterInvitation,
            ACCEPTER_L_INVITATION,
            SUCCES,
        ));
        boutons.push(bouton(Cle::RefuserInvitation, REFUSER_L_INVITATION, DANGER));
    }
    if let Some(autre) = &tableau.invitation_envoyee {
        paragraphes.push(invitation_envoyee(&qui(autre)));
        boutons.push(bouton(
            Cle::RetirerInvitation,
            RETIRER_L_INVITATION,
            SECONDAIRE,
        ));
    }
    // Un défi ou une invitation à la fois par joueur, envoyé ou reçu
    // (sections 15.3 et 15.5).
    let peut_defier = tableau.defi_recu.is_none()
        && tableau.defi_envoye.is_none()
        && tableau.invitation_recue.is_none()
        && tableau.invitation_envoyee.is_none();
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
            let amis = partie.relations();
            let options: Vec<OptionDeMenu> = tableau
                .joueurs
                .iter()
                .map(|autre| OptionDeMenu {
                    libelle: autre.heros.clone(),
                    valeur: autre.id.to_string(),
                    description: if amis.est_ami(autre.id) {
                        Some(match &autre.pseudo {
                            Some(pseudo) => format!("Ami - {pseudo}"),
                            None => "Ami".to_string(),
                        })
                    } else {
                        autre.pseudo.clone()
                    },
                })
                .collect();
            if options.is_empty() {
                paragraphes.push("Aucun autre joueur n'a de partie sur le bot.".to_string());
            } else {
                paragraphes.push(
                    "Choisissez un joueur pour lui donner un monstre, lui en proposer un échange ou le défier en duel."
                        .to_string(),
                );
            }
            lignes.extend(menu(Cle::ChoisirJoueur, "Choisir un joueur", options));
        }
        (None, Some((autre, collection))) => {
            let id = autre.id;
            let a_moi = partie.collection();
            paragraphes.push(duels_de(&autre.heros, autre.palmares));
            if peut_defier {
                boutons.push(bouton(Cle::Defier(id), "Défier en duel", PRINCIPAL));
                boutons.push(bouton(Cle::Inviter(id), INVITER_DANS_LA_TOUR, PRINCIPAL));
            }
            if partie.relations().est_ami(id) {
                boutons.push(bouton(Cle::RetirerAmi(id), "Retirer des amis", SECONDAIRE));
            } else {
                boutons.push(bouton(Cle::AjouterAmi(id), "Ajouter aux amis", SUCCES));
            }
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
    // Au plus 9 boutons : ils tiennent en deux rangées. Le défi ou
    // l'invitation en attente en compte 2 au plus, l'offre reçue 2, le
    // joueur choisi 5 sans défi ni invitation en attente, ou 3 avec, et le
    // classement et le retour 2.
    boutons.push(bouton(
        Cle::Afficher(Vue::Classement),
        "Classement",
        SECONDAIRE,
    ));
    boutons.push(retour());
    lignes.extend(rangees(boutons));
    Question {
        texte: format!(
            "Autres joueurs : offres de monstres et duels.\n{}\n\n{}",
            vos_duels(partie.relations().palmares()),
            paragraphes.join("\n\n")
        ),
        rangees: lignes,
    }
}

/// Les 10 premiers joueurs du bot au classement des duels (section 15.4).
pub fn classement(tableau: &TableauDesEchanges) -> Question {
    let classes: Vec<(String, Option<String>, Palmares)> = tableau
        .classement
        .iter()
        .map(|joueur| (joueur.heros.clone(), joueur.pseudo.clone(), joueur.palmares))
        .collect();
    Question {
        texte: texte_du_classement(&classes),
        rangees: rangees(vec![
            bouton(
                Cle::Afficher(Vue::RecordsDeLaTour),
                "Records de la Tour",
                SECONDAIRE,
            ),
            bouton(
                Cle::Afficher(Vue::Echanges {
                    avec: None,
                    cede: None,
                }),
                "Retour aux autres joueurs",
                SECONDAIRE,
            ),
            retour(),
        ]),
    }
}

/// Les 10 meilleurs records de la Tour sans fond, seul et à deux, sous ceux
/// du joueur (section 9.5).
pub fn records_de_la_tour(partie: &Partie, tableau: &TableauDesEchanges) -> Question {
    let classes = |joueurs: &[AutreJoueur],
                   record: fn(RecordsDeLaTour) -> u32|
     -> Vec<(String, Option<String>, u32)> {
        joueurs
            .iter()
            .map(|joueur| {
                let heros = joueur.heros.clone();
                (heros, joueur.pseudo.clone(), record(joueur.records))
            })
            .collect()
    };
    let seul = classes(&tableau.records_seul, |records| records.seul);
    let a_deux = classes(&tableau.records_a_deux, |records| records.a_deux);
    Question {
        texte: texte_des_records(partie.records_de_la_tour(), &seul, &a_deux),
        rangees: rangees(vec![
            bouton(
                Cle::Afficher(Vue::Classement),
                "Classement des duels",
                SECONDAIRE,
            ),
            bouton(
                Cle::Afficher(Vue::Echanges {
                    avec: None,
                    cede: None,
                }),
                "Retour aux autres joueurs",
                SECONDAIRE,
            ),
            retour(),
        ]),
    }
}
