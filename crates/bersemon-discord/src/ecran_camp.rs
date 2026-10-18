//! Camp de base sur Discord : départ en expédition par la carte, expéditions
//! automatiques, liste des monstres, codex et gestion des monstres (section
//! 13.2). Les vues ne changent rien à la partie ; seules les actions la
//! modifient, et la sauvegardent.

use bersemon_core::codex::EtatCodex;
use bersemon_core::collection::{Collection, Emplacement};
use bersemon_core::combat::Cote;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_presentation::expeditions_automatiques::expeditions_rentrees;
use bersemon_presentation::monstres::{ligne_monstre, nom};
use bersemon_presentation::partie::{
    collection as liste_des_monstres, en_tete_du_camp, fiche_d_espece, ligne_du_codex,
};

use crate::composants::{Cle, Vue, emplacement_en_texte};
use crate::ecran_expeditions_automatiques;
use crate::message::{
    Bouton, DANGER, OPTIONS_PAR_MENU, OptionDeMenu, PRINCIPAL, Question, Rangee, SECONDAIRE,
    bouton, menu, rangees,
};

pub fn question(
    partie: &Partie,
    vue: Vue,
    contenu: &ContenuV1,
    maintenant: Horodatage,
) -> Question {
    match vue {
        Vue::Camp => principale(partie, contenu, maintenant),
        Vue::Monstres => Question {
            texte: liste_des_monstres(partie.collection()),
            rangees: rangees(vec![retour()]),
        },
        Vue::Codex(fiche) => codex(partie, fiche, contenu),
        Vue::Gestion(choisi) => gestion(partie.collection(), choisi),
        Vue::Expeditions => ecran_expeditions_automatiques::liste(partie, contenu, maintenant),
        Vue::Preparation { zone, choisis } => {
            ecran_expeditions_automatiques::preparation(partie, contenu, zone, choisis, maintenant)
        }
    }
}

pub fn retour() -> Bouton {
    bouton(Cle::Afficher(Vue::Camp), "Retour au camp", SECONDAIRE)
}

fn principale(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> Question {
    let equipe: Vec<String> = partie
        .collection()
        .equipe()
        .iter()
        .map(|monstre| ligne_monstre(monstre, Cote::Joueur))
        .collect();
    let zones = partie
        .zones_accessibles(contenu.zones(), contenu.piste())
        .into_iter()
        .map(|zone| {
            let (minimum, maximum) = zone.niveaux();
            let exploree = if partie.est_exploree(zone.id()) {
                ", explorée"
            } else {
                ""
            };
            OptionDeMenu {
                libelle: zone.nom().to_string(),
                valeur: zone.id().as_str().to_string(),
                description: Some(format!(
                    "Niveaux {} à {}{exploree}",
                    minimum.valeur(),
                    maximum.valeur()
                )),
            }
        })
        .collect();
    let mut lignes: Vec<Rangee> = menu(Cle::Partir, "Partir en expédition", zones)
        .into_iter()
        .collect();
    lignes.extend(rangees(vec![
        bouton(
            Cle::Afficher(Vue::Expeditions),
            "Expéditions automatiques",
            SECONDAIRE,
        ),
        bouton(Cle::Afficher(Vue::Monstres), "Vos monstres", SECONDAIRE),
        bouton(
            Cle::Afficher(Vue::Gestion(None)),
            "Gérer vos monstres",
            SECONDAIRE,
        ),
        bouton(Cle::Afficher(Vue::Codex(None)), "Codex", SECONDAIRE),
    ]));
    let mut en_tete = vec![en_tete_du_camp(partie)];
    en_tete.extend(expeditions_rentrees(partie, maintenant));
    Question {
        texte: format!(
            "{}\nVotre équipe :\n{}",
            en_tete.join("\n"),
            equipe.join("\n")
        ),
        rangees: lignes,
    }
}

/// Une ligne par espèce ; la fiche d'une espèce capturée s'ouvre par le menu
/// (section 5.6).
fn codex(partie: &Partie, fiche: Option<usize>, contenu: &ContenuV1) -> Question {
    let codex = partie.codex();
    let especes = contenu.especes();
    let mut lignes = vec!["Codex :".to_string()];
    for (position, espece) in especes.iter().enumerate() {
        lignes.push(format!(
            "  {}. {}",
            position + 1,
            ligne_du_codex(codex, espece)
        ));
    }
    let ouverte = fiche
        .and_then(|position| especes.get(position))
        .filter(|espece| codex.etat(espece.id()) == EtatCodex::Capturee);
    if let Some(espece) = ouverte {
        lignes.push(String::new());
        lignes.push(espece.nom().to_string());
        lignes.extend(fiche_d_espece(espece));
    }
    let capturees = especes
        .iter()
        .enumerate()
        .filter(|(_, espece)| codex.etat(espece.id()) == EtatCodex::Capturee)
        .map(|(position, espece)| OptionDeMenu {
            libelle: espece.nom().to_string(),
            valeur: position.to_string(),
            description: None,
        })
        .collect();
    let mut rangees_du_codex: Vec<Rangee> =
        menu(Cle::Fiche, "Voir la fiche d'une espèce capturée", capturees)
            .into_iter()
            .collect();
    rangees_du_codex.extend(rangees(vec![retour()]));
    Question {
        texte: lignes.join("\n"),
        rangees: rangees_du_codex,
    }
}

/// Des menus pour choisir un monstre, puis les actions possibles pour le
/// monstre choisi.
fn gestion(collection: &Collection, choisi: Option<Emplacement>) -> Question {
    let liste = liste_des_monstres(collection);
    let choisi = choisi.and_then(|emplacement| {
        collection
            .monstre(emplacement)
            .map(|monstre| (emplacement, monstre))
    });
    let Some((emplacement, monstre)) = choisi else {
        let mut lignes = menus_de_monstres(collection, Cle::Choisir, "Choisir", |_| true);
        lignes.extend(rangees(vec![retour()]));
        return Question {
            texte: format!("{liste}\n\nChoisissez un monstre à gérer."),
            rangees: lignes,
        };
    };
    let mut boutons = Vec::new();
    let mut echange = None;
    match emplacement {
        Emplacement::Equipe(index) => {
            if index > 0 {
                boutons.push(bouton(Cle::Monter(index), "Monter", SECONDAIRE));
            }
            if index + 1 < collection.equipe().len() {
                boutons.push(bouton(Cle::Descendre(index), "Descendre", SECONDAIRE));
            }
            boutons.push(bouton(
                Cle::EnvoyerAuCamp(index),
                "Envoyer au camp",
                PRINCIPAL,
            ));
        }
        Emplacement::Camp(index) => {
            boutons.push(bouton(
                Cle::FaireVenir(index),
                "Faire venir dans l'équipe",
                PRINCIPAL,
            ));
            echange = menu(
                Cle::EchangerAvec(index),
                "Échanger avec un monstre de l'équipe",
                options_d_equipe(collection.equipe()),
            );
        }
    }
    boutons.push(bouton(Cle::Renommer(emplacement), "Renommer", SECONDAIRE));
    boutons.push(bouton(Cle::Relacher(emplacement), "Relâcher", DANGER));
    let mut lignes = rangees(boutons);
    lignes.extend(echange);
    lignes.extend(rangees(vec![
        bouton(
            Cle::Afficher(Vue::Gestion(None)),
            "Choisir un autre monstre",
            SECONDAIRE,
        ),
        retour(),
    ]));
    Question {
        texte: format!(
            "{liste}\n\nMonstre choisi : {}",
            ligne_monstre(monstre, Cote::Joueur)
        ),
        rangees: lignes,
    }
}

/// Les monstres de l'équipe, puis ceux du camp, 25 au plus par menu : avec
/// l'équipe et le camp pleins, trois menus. Les options portent le numéro de
/// la liste des monstres ; seuls les monstres que `garder` accepte y
/// figurent.
pub fn menus_de_monstres(
    collection: &Collection,
    cle: impl Fn(u8) -> Cle,
    verbe: &str,
    garder: impl Fn(Emplacement) -> bool,
) -> Vec<Rangee> {
    let taille_equipe = collection.equipe().len();
    let option = |emplacement: Emplacement, numero: usize, monstre: &Monstre| OptionDeMenu {
        libelle: format!("{numero}. {}", nom(monstre, Cote::Joueur)),
        valeur: emplacement_en_texte(emplacement),
        description: Some(ligne_monstre(monstre, Cote::Joueur)),
    };
    let equipe = collection
        .equipe()
        .iter()
        .enumerate()
        .filter(|(index, _)| garder(Emplacement::Equipe(*index)))
        .map(|(index, monstre)| option(Emplacement::Equipe(index), index + 1, monstre))
        .collect();
    let mut menus: Vec<Rangee> = menu(cle(0), &format!("{verbe} un monstre de l'équipe"), equipe)
        .into_iter()
        .collect();
    let camp: Vec<OptionDeMenu> = collection
        .camp()
        .iter()
        .enumerate()
        .filter(|(index, _)| garder(Emplacement::Camp(*index)))
        .map(|(index, monstre)| {
            option(Emplacement::Camp(index), taille_equipe + index + 1, monstre)
        })
        .collect();
    let mut restantes = camp.into_iter().peekable();
    let mut numero = 1;
    while restantes.peek().is_some() {
        let partie_du_camp = restantes.by_ref().take(OPTIONS_PAR_MENU).collect();
        menus.extend(menu(
            cle(numero),
            &format!("{verbe} un monstre du camp"),
            partie_du_camp,
        ));
        numero += 1;
    }
    menus
}

/// Les monstres de l'équipe, désignés par leur position.
pub fn options_d_equipe(equipe: &[Monstre]) -> Vec<OptionDeMenu> {
    equipe
        .iter()
        .enumerate()
        .map(|(index, monstre)| OptionDeMenu {
            libelle: nom(monstre, Cote::Joueur),
            valeur: index.to_string(),
            description: Some(ligne_monstre(monstre, Cote::Joueur)),
        })
        .collect()
}
