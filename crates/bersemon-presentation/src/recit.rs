//! Récit des événements du déroulement, dans l'ordre où ils arrivent.

use bersemon_core::combat::Cote;
use bersemon_core::contenu::{
    INTRODUCTION, RENCONTRE_DU_MARCHAND, fin_de_l_introduction, texte_du_conseil,
};
use bersemon_core::deroulement::Evenement;

use crate::combat::{rapport_experience, remplacement, renoncement, soin_de_potion};
use crate::erreurs::erreur_sauvegarde;
use crate::expeditions_automatiques;
use crate::monstres::{nom, nom_sauvage};
use crate::partie::{argent, potions};

/// Une ou plusieurs lignes de récit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recit {
    pub texte: String,
    /// Le récit ouvre un nouveau paragraphe, séparé du précédent par une
    /// ligne vide.
    pub nouveau_paragraphe: bool,
}

pub fn raconter(evenements: &[Evenement]) -> Vec<Recit> {
    let mut recits = Vec::new();
    let mut precedent = None;
    for evenement in evenements {
        recits.push(raconter_un(evenement, precedent));
        precedent = Some(evenement);
    }
    recits
}

fn raconter_un(evenement: &Evenement, precedent: Option<&Evenement>) -> Recit {
    let (texte, nouveau_paragraphe) = match evenement {
        Evenement::Introduction(classe) => (
            format!("{INTRODUCTION}\n\n{}", fin_de_l_introduction(*classe)),
            true,
        ),
        Evenement::PartieCommencee(depart) => (
            format!("{} vous accompagne désormais.", depart.nom()),
            false,
        ),
        Evenement::Depart(zone) => (format!("Vous partez pour {zone}."), true),
        Evenement::Etape(numero) => (format!("Étape {numero}."), true),
        Evenement::Coffre {
            potions: nombre,
            argent: montant,
        } => {
            let contenu = if *montant == 0 {
                potions(*nombre)
            } else {
                format!("{} et {}", potions(*nombre), argent(*montant))
            };
            (format!("Vous trouvez un coffre : {contenu}."), false)
        }
        Evenement::MonstreRarePacifique(monstre) => (
            format!(
                "Un monstre rare apparaît : {}. Il ne semble pas hostile.",
                nom_sauvage(monstre)
            ),
            false,
        ),
        Evenement::MonstreRareHostile(monstre) => (
            format!("Un monstre rare vous attaque : {} !", nom_sauvage(monstre)),
            false,
        ),
        Evenement::MonstreRareLaisse => ("Vous le laissez partir.".to_string(), false),
        Evenement::RecitDeLApotre(recit) => (recit.clone(), false),
        Evenement::Rencontre(combat) => (crate::combat::rencontre(combat), true),
        Evenement::Combat { evenements, combat } => {
            let lignes: Vec<String> = evenements
                .iter()
                .map(|evenement_du_tour| crate::combat::evenement(evenement_du_tour, combat))
                .collect();
            (lignes.join("\n"), false)
        }
        Evenement::Experience { monstre, rapport } => (rapport_experience(monstre, rapport), true),
        Evenement::AttaqueRemplacee {
            monstre,
            oubliee,
            nouvelle,
        } => (remplacement(monstre, oubliee, nouvelle), false),
        Evenement::AttaqueAbandonnee { monstre, nouvelle } => {
            (renoncement(monstre, nouvelle), false)
        }
        Evenement::RejointLEquipe(monstre) => {
            (format!("{} rejoint votre équipe.", monstre.nom()), false)
        }
        Evenement::PartPourLeCamp(monstre) => {
            (format!("{} part pour le camp.", monstre.nom()), false)
        }
        Evenement::CaptureRelachee(monstre) | Evenement::Relache(monstre) => {
            (format!("Vous relâchez {}.", monstre.nom()), false)
        }
        Evenement::RelachePourLaCapture { relache, capture } => (
            format!(
                "Vous relâchez {}. {} prend sa place.",
                relache.nom(),
                capture.nom()
            ),
            false,
        ),
        Evenement::Epilogue(epilogue) => (epilogue.clone(), true),
        Evenement::Defaite { argent_perdu } => {
            let texte = if *argent_perdu == 0 {
                "Toute votre équipe est KO : vous rentrez au camp et perdez vos potions."
                    .to_string()
            } else {
                format!(
                    "Toute votre équipe est KO : vous rentrez au camp et perdez vos potions, ainsi que l'argent trouvé en chemin ({}).",
                    argent(*argent_perdu)
                )
            };
            (texte, true)
        }
        Evenement::PotionUtilisee { monstre, pv_rendus } => (
            soin_de_potion(&nom(monstre, Cote::Joueur), *pv_rendus),
            false,
        ),
        Evenement::ZoneExploree(zone) => (format!("Vous avez exploré {zone} jusqu'au bout."), true),
        Evenement::RetourAuCamp => (
            "Retour au camp : vos monstres sont soignés.".to_string(),
            !matches!(precedent, Some(Evenement::ZoneExploree(_))),
        ),
        Evenement::ArgentRapporte(montant) => {
            (format!("Vous rapportez {}.", argent(*montant)), false)
        }
        Evenement::Indice { numero, texte } => (format!("Indice {numero} : {texte}"), true),
        Evenement::NouvelleZone(zone) => (
            format!("Une nouvelle zone apparaît sur la carte : {zone}."),
            false,
        ),
        Evenement::Conseil(conseil) => (texte_du_conseil(*conseil).to_string(), true),
        Evenement::SauvegardeImpossible(erreur) => (
            format!("Sauvegarde impossible : {}.", erreur_sauvegarde(erreur)),
            false,
        ),
        Evenement::ExpeditionAutomatiquePartie(expedition) => {
            (expeditions_automatiques::depart(expedition), true)
        }
        Evenement::ExpeditionAutomatiqueRappelee(expedition) => {
            (expeditions_automatiques::rappel(expedition), true)
        }
        Evenement::ExpeditionAutomatiqueRentree { zone, bilan } => {
            (expeditions_automatiques::retour(zone, bilan), true)
        }
        Evenement::MarchandRencontre => (RENCONTRE_DU_MARCHAND.to_string(), false),
        Evenement::PotionsAchetees { nombre, prix } => (
            format!("Vous achetez {} pour {}.", potions(*nombre), argent(*prix)),
            false,
        ),
        Evenement::MonstreVendu { monstre, prix } => (
            format!(
                "Vous vendez {} au marchand pour {}.",
                monstre.nom(),
                argent(*prix)
            ),
            false,
        ),
    };
    Recit {
        texte,
        nouveau_paragraphe,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_retour_suit_la_zone_exploree_dans_le_meme_paragraphe() {
        let recits = raconter(&[
            Evenement::ZoneExploree("Lisière du Bois-Sombre".to_string()),
            Evenement::RetourAuCamp,
        ]);
        assert_eq!(
            recits,
            vec![
                Recit {
                    texte: "Vous avez exploré Lisière du Bois-Sombre jusqu'au bout.".to_string(),
                    nouveau_paragraphe: true,
                },
                Recit {
                    texte: "Retour au camp : vos monstres sont soignés.".to_string(),
                    nouveau_paragraphe: false,
                },
            ]
        );
        assert!(raconter(&[Evenement::RetourAuCamp])[0].nouveau_paragraphe);
    }

    #[test]
    fn un_coffre_dit_combien_de_potions_et_d_argent() {
        let recit = &raconter(&[Evenement::Coffre {
            potions: 2,
            argent: 15,
        }])[0];
        assert_eq!(
            recit.texte,
            "Vous trouvez un coffre : 2 potions et 15 pièces d'argent."
        );
        let sans_argent = &raconter(&[Evenement::Coffre {
            potions: 1,
            argent: 0,
        }])[0];
        assert_eq!(sans_argent.texte, "Vous trouvez un coffre : 1 potion.");
    }

    #[test]
    fn une_defaite_dit_l_argent_perdu() {
        let recit = &raconter(&[Evenement::Defaite { argent_perdu: 1 }])[0];
        assert_eq!(
            recit.texte,
            "Toute votre équipe est KO : vous rentrez au camp et perdez vos potions, ainsi que l'argent trouvé en chemin (1 pièce d'argent)."
        );
    }

    #[test]
    fn achat_et_vente_disent_le_prix() {
        let recit = &raconter(&[Evenement::PotionsAchetees {
            nombre: 2,
            prix: 40,
        }])[0];
        assert_eq!(
            recit.texte,
            "Vous achetez 2 potions pour 40 pièces d'argent."
        );
    }

    #[test]
    fn l_argent_rapporte_suit_le_retour_au_camp() {
        let recits = raconter(&[Evenement::RetourAuCamp, Evenement::ArgentRapporte(40)]);
        assert_eq!(recits[1].texte, "Vous rapportez 40 pièces d'argent.");
        assert!(!recits[1].nouveau_paragraphe);
    }
}
