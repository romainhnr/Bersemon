//! Duel en temps réel entre deux joueurs (section 15.3) : défis, récit du
//! duel vu par chaque joueur, attente et fin. Textes dans
//! docs/contenu-v1.md, section 14.

use bersemon_core::combat::{Combat, Cote, Evenement, Issue, Phase};
use bersemon_core::duel::EvenementDeDuel;

use crate::combat::{evenement, ordre_donne};
use crate::echanges::joueur;
use crate::mise_en_valeur::important;
use crate::monstres::nom_en_valeur;

/// Le message privé qui prévient le joueur d'un défi reçu.
pub fn defi_recu(heros: &str, pseudo: Option<&str>) -> String {
    format!(
        "{} vous défie en duel. Ouvrez /bersemon, au camp, pour répondre.",
        joueur(heros, pseudo)
    )
}

pub fn defi_accepte(heros: &str) -> String {
    format!("{heros} a accepté votre défi : le duel commence. Ouvrez /bersemon.")
}

pub fn defi_refuse(heros: &str) -> String {
    format!("{heros} a refusé votre défi.")
}

pub fn en_attente(adversaire: &str) -> String {
    format!("En attente de l'action de {adversaire}.")
}

/// `None` tant que le duel n'est pas fini.
pub fn issue_du_duel(combat: &Combat, adversaire: &str) -> Option<String> {
    match combat.phase() {
        Phase::Termine(Issue::Victoire) => Some(important(&format!(
            "Victoire : vous remportez le duel contre {adversaire}."
        ))),
        Phase::Termine(_) => Some(important(&format!(
            "Défaite : {adversaire} remporte le duel."
        ))),
        _ => None,
    }
}

/// Le récit vu par un joueur, `combat` étant le duel vu par lui. La fin du
/// duel n'y figure pas : l'écran la dit (`issue_du_duel`). `heros` est le
/// nom de son héros, `adversaire` celui du héros de l'autre joueur.
pub fn raconter(
    evenements: &[EvenementDeDuel],
    combat: &Combat,
    heros: &str,
    adversaire: &str,
) -> String {
    evenements
        .iter()
        .filter_map(|evenement_de_duel| match evenement_de_duel {
            EvenementDeDuel::OrdreDonne { ordre, cible } => {
                let qui = match cible.cote {
                    Cote::Joueur => heros,
                    Cote::Adversaire => adversaire,
                };
                let monstre = nom_en_valeur(combat.monstre(*cible), Cote::Joueur);
                Some(ordre_donne(qui, *ordre, &monstre))
            }
            EvenementDeDuel::Combat(Evenement::FinDuCombat(_))
            | EvenementDeDuel::Combat(Evenement::Abandon(Cote::Joueur))
            | EvenementDeDuel::Combat(Evenement::Forfait(Cote::Joueur)) => None,
            EvenementDeDuel::Combat(Evenement::Abandon(Cote::Adversaire)) => {
                Some(format!("{adversaire} abandonne le duel."))
            }
            EvenementDeDuel::Combat(Evenement::Forfait(Cote::Adversaire)) => Some(format!(
                "{adversaire} n'a pas joué depuis 5 minutes : vous remportez le duel par forfait."
            )),
            EvenementDeDuel::Combat(autre) => Some(evenement(autre, combat)),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::combat::Combattant;
    use bersemon_core::contenu::ContenuV1;
    use bersemon_core::espece::EspeceId;
    use bersemon_core::heros::Ordre;
    use bersemon_core::monstre::Monstre;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::stats::ValeursInternes;

    fn creer(contenu: &ContenuV1, id: &str) -> Monstre {
        Monstre::creer(
            contenu.espece(&EspeceId::new(id)).unwrap(),
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    #[test]
    fn le_recit_d_un_duel_nomme_les_heros() {
        let contenu = ContenuV1::new();
        let combat = Combat::duel(
            vec![creer(&contenu, "loup")],
            vec![creer(&contenu, "corbeau")],
        )
        .unwrap();
        let corbeau = Combattant {
            cote: Cote::Adversaire,
            index: 0,
        };
        let recit = raconter(
            &[
                EvenementDeDuel::OrdreDonne {
                    ordre: Ordre::Incantation,
                    cible: corbeau,
                },
                EvenementDeDuel::Combat(Evenement::Ko(corbeau)),
                EvenementDeDuel::Combat(Evenement::Abandon(Cote::Adversaire)),
                EvenementDeDuel::Combat(Evenement::FinDuCombat(Issue::Victoire)),
            ],
            &combat,
            "Aldric",
            "Ysolde",
        );
        let lignes: Vec<&str> = recit.lines().collect();
        assert_eq!(lignes.len(), 3, "{recit}");
        assert!(
            lignes[0].starts_with("Ysolde murmure une incantation : **Corbeau** gagne"),
            "{recit}"
        );
        // Les monstres de l'autre joueur ne sont pas sauvages.
        assert!(!recit.contains("sauvage"), "{recit}");
        assert_eq!(lignes[2], "Ysolde abandonne le duel.");
        assert_eq!(issue_du_duel(&combat, "Ysolde"), None);
        assert_eq!(
            defi_recu("Ysolde", Some("ysolde_la_grise")),
            "Ysolde (ysolde_la_grise) vous défie en duel. Ouvrez /bersemon, au camp, pour répondre."
        );
    }
}
