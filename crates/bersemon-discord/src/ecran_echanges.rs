//! Écran des autres joueurs (sections 15.1 et 15.3) : l'offre et le défi
//! reçus, à accepter ou à refuser, l'offre et le défi envoyés, à retirer,
//! puis, sans offre envoyée, une offre à faire en trois temps : le joueur,
//! le monstre ou les ressources à lui céder, et le monstre ou les
//! ressources qu'on lui demande, ou rien pour un don. Des ressources se
//! choisissent par leur sorte, puis par leur nombre. Une fois le joueur
//! choisi, il peut aussi être défié en duel, et ajouté aux amis ou retiré
//! (section 15.4). Le palmarès du joueur est en
//! tête, et le classement des duels s'ouvre d'un bouton, comme celui des
//! records de la Tour sans fond (section 9.5).

use bersemon_core::collection::{Collection, Emplacement};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::espece::Rarete;
use bersemon_core::partie::Partie;
use bersemon_core::relations::Palmares;
use bersemon_core::ressource::Ressource;
use bersemon_core::tour::RecordsDeLaTour;
use bersemon_presentation::duels::{
    classement as texte_du_classement, classement_de_l_arene as texte_de_l_arene, duels_de,
    vos_duels,
};
use bersemon_presentation::echanges::{
    decrire_offre_envoyee, decrire_offre_recue, joueur, nom_et_niveau,
};
use bersemon_presentation::groupe::{
    ACCEPTER_L_INVITATION, INVITER_DANS_LA_TOUR, REFUSER_L_INVITATION, RETIRER_L_INVITATION,
    invitation_envoyee, vous_invite,
};
use bersemon_presentation::partie::ressources as ressources_en_texte;
use bersemon_presentation::tour::records_de_la_tour as texte_des_records;

use crate::composants::{Cle, PartEnCours, Vue};
use crate::echanges::{AutreJoueur, Designation, TableauDesEchanges};
use crate::ecran_camp::{menus_de_monstres, retour};
use crate::joueurs::IdJoueur;
use crate::message::{
    DANGER, OptionDeMenu, PRINCIPAL, Question, RANGEES_MAX, Rangee, SECONDAIRE, SUCCES, bouton,
    menu, menus, rangees,
};

/// Les nombres proposés pour des ressources, en plus de "Tout".
const NOMBRES_DE_RESSOURCES: [u32; 10] = [1, 2, 3, 5, 10, 20, 50, 100, 200, 500];

/// Deux rangées pour les boutons, le reste pour les menus de ressources.
const MENUS_DE_RESSOURCES_MAX: usize = RANGEES_MAX - 2;

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

/// Une option par sorte, triées par nom, avec leur rang au contenu pour
/// valeur.
fn options_de_sortes(possedees: &[(Ressource, u32)], rangs: &[Ressource]) -> Vec<OptionDeMenu> {
    let mut options: Vec<(&Ressource, u32, usize)> = possedees
        .iter()
        .filter_map(|(ressource, nombre)| {
            let rang = rangs
                .iter()
                .position(|presente| presente.id() == ressource.id())?;
            Some((ressource, *nombre, rang))
        })
        .collect();
    options.sort_by(|a, b| a.0.nom().cmp(b.0.nom()));
    options
        .into_iter()
        .map(|(ressource, nombre, rang)| OptionDeMenu {
            libelle: ressources_en_texte(ressource, nombre),
            valeur: rang.to_string(),
            description: None,
        })
        .collect()
}

fn options_de_nombres(possedees: u32) -> Vec<OptionDeMenu> {
    NOMBRES_DE_RESSOURCES
        .into_iter()
        .filter(|nombre| *nombre < possedees)
        .map(|nombre| OptionDeMenu {
            libelle: nombre.to_string(),
            valeur: nombre.to_string(),
            description: None,
        })
        .chain((possedees > 0).then(|| OptionDeMenu {
            libelle: format!("Tout ({possedees})"),
            valeur: possedees.to_string(),
            description: None,
        }))
        .collect()
}

/// La sorte de ce rang, et combien en ont le sac et le coffre réunis.
fn sorte(
    rangs: &[Ressource],
    rang: u16,
    possedees: &[(Ressource, u32)],
) -> Option<(Ressource, u32)> {
    let ressource = rangs.get(usize::from(rang))?.clone();
    let nombre = possedees
        .iter()
        .find(|(presente, _)| presente.id() == ressource.id())
        .map_or(0, |(_, nombre)| *nombre);
    Some((ressource, nombre))
}

/// "Croc (Loup, niv. 7)", ou "Fragment d'armure (12)" : `None` si le
/// monstre n'est plus là, ou si le rang ne désigne aucune ressource.
fn designation_en_texte(
    collection: &Collection,
    rangs: &[Ressource],
    designation: Designation,
) -> Option<String> {
    match designation {
        Designation::Monstre(emplacement) => collection.monstre(emplacement).map(nom_et_niveau),
        Designation::Ressources { rang, nombre } => rangs
            .get(usize::from(rang))
            .map(|ressource| ressources_en_texte(ressource, nombre)),
    }
}

pub fn echanges(
    partie: &Partie,
    tableau: &TableauDesEchanges,
    avec: Option<IdJoueur>,
    (cede, demande): (Option<PartEnCours>, Option<PartEnCours>),
    contenu: &ContenuV1,
) -> Question {
    let mut paragraphes = Vec::new();
    let mut lignes: Vec<Rangee> = Vec::new();
    let mut boutons = Vec::new();
    let ou = if tableau.defi_a_l_arene {
        "à l'arène"
    } else {
        "en duel"
    };
    if let Some(autre) = &tableau.defi_recu {
        paragraphes.push(format!("{} vous défie {ou}.", qui(autre)));
        boutons.push(bouton(Cle::AccepterDefi, "Accepter le défi", SUCCES));
        boutons.push(bouton(Cle::RefuserDefi, "Refuser le défi", DANGER));
    }
    if let Some(autre) = &tableau.defi_envoye {
        paragraphes.push(format!(
            "Vous avez défié {} {ou}. En attente de sa réponse.",
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
                    "Choisissez un joueur pour lui donner un monstre ou des ressources, lui proposer un échange ou le défier en duel."
                        .to_string(),
                );
            }
            lignes.extend(menu(Cle::ChoisirJoueur, "Choisir un joueur", options));
        }
        (None, Some((autre, collection))) => {
            let id = autre.id;
            let a_moi = partie.collection();
            let rangs = contenu.ressources();
            let mes_ressources = partie.ressources_possedees();
            // Ce qui n'existe plus, monstre parti ou rang inconnu, ramène au
            // début de l'offre.
            let cede = cede.filter(|part| match part {
                PartEnCours::Choisie(designation) => {
                    designation_en_texte(a_moi, &rangs, *designation).is_some()
                }
                PartEnCours::SorteAChoisir => true,
                PartEnCours::NombreAChoisir(rang) => usize::from(*rang) < rangs.len(),
            });
            paragraphes.push(duels_de(&autre.heros, autre.palmares));
            // Les défis se lancent avant de composer une offre : pendant,
            // leurs boutons laissent la place à ceux de l'offre.
            if peut_defier && cede.is_none() {
                boutons.push(bouton(Cle::Defier(id), "Défier en duel", PRINCIPAL));
                boutons.push(bouton(
                    Cle::DefierALArene(id),
                    "Défier à l'arène",
                    PRINCIPAL,
                ));
                boutons.push(bouton(Cle::Inviter(id), INVITER_DANS_LA_TOUR, PRINCIPAL));
            }
            if partie.relations().est_ami(id) {
                boutons.push(bouton(Cle::RetirerAmi(id), "Retirer des amis", SECONDAIRE));
            } else {
                boutons.push(bouton(Cle::AjouterAmi(id), "Ajouter aux amis", SUCCES));
            }
            let vue = |cede, demande| {
                Cle::Afficher(Vue::Echanges {
                    avec: Some(id),
                    cede,
                    demande,
                })
            };
            match cede {
                None => {
                    paragraphes.push(format!(
                        "Offre à {} : choisissez le monstre à lui céder, ou cédez-lui des ressources.",
                        qui(autre)
                    ));
                    lignes.extend(menus_de_monstres(
                        a_moi,
                        |menu| Cle::Ceder { menu, avec: id },
                        "Céder",
                        n_est_pas_un_apotre(a_moi),
                    ));
                    boutons.push(bouton(
                        vue(Some(PartEnCours::SorteAChoisir), None),
                        "Céder des ressources",
                        PRINCIPAL,
                    ));
                }
                Some(PartEnCours::SorteAChoisir) => {
                    let options = options_de_sortes(&mes_ressources, &rangs);
                    paragraphes.push(if options.is_empty() {
                        "Vous n'avez aucune ressource, ni dans le sac, ni dans le coffre."
                            .to_string()
                    } else {
                        format!(
                            "Offre à {} : choisissez les ressources à lui céder.",
                            qui(autre)
                        )
                    });
                    lignes.extend(menus(
                        |menu| Cle::CederRessource { menu, avec: id },
                        "Céder des ressources",
                        options,
                        MENUS_DE_RESSOURCES_MAX,
                    ));
                }
                Some(PartEnCours::NombreAChoisir(rang)) => {
                    if let Some((ressource, nombre)) = sorte(&rangs, rang, &mes_ressources) {
                        paragraphes.push(format!(
                            "Offre à {} : combien de {} lui céder ? Le sac et le coffre en ont {nombre}.",
                            qui(autre),
                            ressource.nom()
                        ));
                        lignes.extend(menu(
                            Cle::NombreACeder { avec: id, rang },
                            "Nombre à céder",
                            options_de_nombres(nombre),
                        ));
                    }
                }
                Some(PartEnCours::Choisie(designation)) => {
                    let texte = designation_en_texte(a_moi, &rangs, designation)
                        .expect("cession vérifiée plus haut");
                    let ses_ressources = &tableau.ressources_du_choisi;
                    match demande {
                        Some(PartEnCours::SorteAChoisir) => {
                            let options = options_de_sortes(ses_ressources, &rangs);
                            paragraphes.push(if options.is_empty() {
                                format!("Vous cédez {texte}, mais {} n'a aucune ressource.", qui(autre))
                            } else {
                                format!(
                                    "Offre à {} : vous cédez {texte}. Choisissez les ressources demandées en échange.",
                                    qui(autre)
                                )
                            });
                            lignes.extend(menus(
                                |menu| Cle::DemanderRessource {
                                    menu,
                                    avec: id,
                                    cede: designation,
                                },
                                "Demander des ressources",
                                options,
                                MENUS_DE_RESSOURCES_MAX,
                            ));
                        }
                        Some(PartEnCours::NombreAChoisir(rang)) => {
                            if let Some((ressource, nombre)) = sorte(&rangs, rang, ses_ressources) {
                                paragraphes.push(format!(
                                    "Offre à {} : vous cédez {texte}. Combien de {} demander ? {} en a {nombre}.",
                                    qui(autre),
                                    ressource.nom(),
                                    autre.heros
                                ));
                                lignes.extend(menu(
                                    Cle::NombreADemander {
                                        avec: id,
                                        cede: designation,
                                        rang,
                                    },
                                    "Nombre demandé",
                                    options_de_nombres(nombre),
                                ));
                            }
                        }
                        None | Some(PartEnCours::Choisie(_)) => {
                            paragraphes.push(format!(
                                "Offre à {} : vous cédez {texte}. Choisissez le monstre ou les ressources demandés en échange, ou donnez sans contrepartie.",
                                qui(autre)
                            ));
                            lignes.extend(menus_de_monstres(
                                collection,
                                |menu| Cle::Demander {
                                    menu,
                                    avec: id,
                                    cede: designation,
                                },
                                "Demander",
                                n_est_pas_un_apotre(collection),
                            ));
                            boutons.push(bouton(
                                vue(cede, Some(PartEnCours::SorteAChoisir)),
                                "Demander des ressources",
                                PRINCIPAL,
                            ));
                            boutons.push(bouton(
                                Cle::Donner {
                                    avec: id,
                                    cede: designation,
                                },
                                "Donner sans contrepartie",
                                PRINCIPAL,
                            ));
                        }
                    }
                }
            }
            boutons.push(bouton(
                Cle::Afficher(Vue::Echanges {
                    avec: None,
                    cede: None,
                    demande: None,
                }),
                "Annuler l'offre",
                SECONDAIRE,
            ));
        }
    }
    // Au plus 10 boutons : ils tiennent en deux rangées. Le défi ou
    // l'invitation en attente en compte 2 au plus, l'offre reçue 2, le
    // joueur choisi 6 avant l'offre sans défi ni invitation en attente, 3
    // avec, et 4 pendant l'offre, et le classement et le retour 2.
    boutons.push(bouton(
        Cle::Afficher(Vue::Classement),
        "Classement",
        SECONDAIRE,
    ));
    boutons.push(retour());
    lignes.extend(rangees(boutons));
    Question {
        texte: format!(
            "Multijoueur : offres de monstres et de ressources, et duels.\n{}\n\n{}",
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
                Cle::Afficher(Vue::ClassementDeLArene),
                "Classement de l'arène",
                SECONDAIRE,
            ),
            bouton(
                Cle::Afficher(Vue::Echanges {
                    avec: None,
                    cede: None,
                    demande: None,
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
                Cle::Afficher(Vue::ClassementDeLArene),
                "Classement de l'arène",
                SECONDAIRE,
            ),
            bouton(
                Cle::Afficher(Vue::Echanges {
                    avec: None,
                    cede: None,
                    demande: None,
                }),
                "Retour aux autres joueurs",
                SECONDAIRE,
            ),
            retour(),
        ]),
    }
}

/// La cote du joueur, puis les 10 meilleures cotes de l'arène (section
/// 15.6).
pub fn classement_de_l_arene(partie: &Partie, tableau: &TableauDesEchanges) -> Question {
    let classes: Vec<(String, Option<String>, u32)> = tableau
        .classement_de_l_arene
        .iter()
        .map(|joueur| {
            (
                joueur.heros.clone(),
                joueur.pseudo.clone(),
                joueur.arene.cote,
            )
        })
        .collect();
    Question {
        texte: texte_de_l_arene(partie.arene().cote, &classes),
        rangees: rangees(vec![
            bouton(
                Cle::Afficher(Vue::Classement),
                "Classement des duels",
                SECONDAIRE,
            ),
            bouton(
                Cle::Afficher(Vue::RecordsDeLaTour),
                "Records de la Tour",
                SECONDAIRE,
            ),
            bouton(
                Cle::Afficher(Vue::Echanges {
                    avec: None,
                    cede: None,
                    demande: None,
                }),
                "Retour aux autres joueurs",
                SECONDAIRE,
            ),
            retour(),
        ]),
    }
}
