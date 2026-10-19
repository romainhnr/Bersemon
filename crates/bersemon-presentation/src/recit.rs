//! Récit des événements du déroulement, dans l'ordre où ils arrivent.

use bersemon_core::combat::Cote;
use bersemon_core::contenu::{
    ARRIVEE_DE_MALVOISE, INTRODUCTION, fin_de_l_introduction, texte_du_conseil,
};
use bersemon_core::deroulement::Evenement;

use crate::combat::{rapport_experience, remplacement, renoncement};
use crate::erreurs::erreur_sauvegarde;
use crate::expeditions_automatiques;
use crate::mise_en_valeur::important;
use crate::monstres::{nom_en_valeur, nom_sauvage};
use crate::partie::{
    argent, argent_et_part_du_pillard, capsule as nom_de_capsule, contenu, enumerer, potions,
};
use crate::soins::{le_soin, soin_utilise, soins};

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
            format!(
                "{} vous accompagne désormais.",
                nom_en_valeur(depart, Cote::Joueur)
            ),
            false,
        ),
        Evenement::Depart(zone) => (format!("Vous partez pour {zone}."), true),
        Evenement::Etape { numero, total } => (format!("Étape {numero}/{total}."), true),
        Evenement::Coffre {
            potions: nombre,
            argent: montant,
            argent_du_pillard,
            capsule,
        } => {
            let mut contenu = vec![potions(*nombre)];
            if *montant > 0 {
                contenu.push(argent_et_part_du_pillard(*montant, *argent_du_pillard));
            }
            contenu.extend(capsule.iter().map(nom_de_capsule));
            (
                format!("Vous trouvez un coffre : {}.", enumerer(&contenu)),
                false,
            )
        }
        Evenement::MonstreRarePacifique(monstre) => (
            format!(
                "Un monstre rare apparaît : {}. Il ne semble pas hostile.",
                important(&nom_sauvage(monstre))
            ),
            false,
        ),
        Evenement::MonstreRareHostile(monstre) => (
            format!(
                "Un monstre rare vous attaque : {} !",
                important(&nom_sauvage(monstre))
            ),
            false,
        ),
        Evenement::MonstreRareLaisse => ("Vous le laissez partir.".to_string(), false),
        Evenement::RecitDeLApotre(recit)
        | Evenement::HumainRencontre(recit)
        | Evenement::AmeRencontree(recit) => (recit.clone(), false),
        Evenement::AmeRecueillie { texte, soin } => (
            format!(
                "{texte} Malvoise vendra désormais {}.",
                le_soin(*soin).to_lowercase()
            ),
            true,
        ),
        Evenement::HumainVaincu {
            texte,
            argent: montant,
        } => (
            format!(
                "{texte} Vous ramassez {}, qui vont dans le sac.",
                argent(*montant)
            ),
            true,
        ),
        Evenement::Rencontre(combat) => (crate::combat::rencontre(combat), true),
        Evenement::Combat { evenements, combat } => {
            let lignes: Vec<String> = evenements
                .iter()
                .map(|evenement_du_tour| crate::combat::evenement(evenement_du_tour, combat))
                .collect();
            (lignes.join("\n"), false)
        }
        Evenement::Experience { monstre, rapport } => (rapport_experience(monstre, rapport), true),
        Evenement::PotionTrouvee(fouineur) => (
            format!(
                "Grâce à son habileté Fouineur, {} déniche une potion, rangée dans le sac.",
                nom_en_valeur(fouineur, Cote::Joueur)
            ),
            false,
        ),
        Evenement::AttaqueRemplacee {
            monstre,
            oubliee,
            nouvelle,
        } => (remplacement(monstre, oubliee, nouvelle), false),
        Evenement::AttaqueAbandonnee { monstre, nouvelle } => {
            (renoncement(monstre, nouvelle), false)
        }
        Evenement::AttaqueApprise { monstre, attaque } => (
            format!(
                "{} apprend {}.",
                nom_en_valeur(monstre, Cote::Joueur),
                attaque.nom()
            ),
            false,
        ),
        Evenement::RejointLEquipe(monstre) => (
            format!(
                "{} rejoint votre équipe.",
                nom_en_valeur(monstre, Cote::Joueur)
            ),
            false,
        ),
        Evenement::PartPourLeCamp(monstre) => (
            format!(
                "{} part pour le camp.",
                nom_en_valeur(monstre, Cote::Joueur)
            ),
            false,
        ),
        Evenement::CaptureRelachee(monstre) | Evenement::Relache(monstre) => (
            format!("Vous relâchez {}.", nom_en_valeur(monstre, Cote::Joueur)),
            false,
        ),
        Evenement::RelachePourLaCapture { relache, capture } => (
            format!(
                "Vous relâchez {}. {} prend sa place.",
                nom_en_valeur(relache, Cote::Joueur),
                nom_en_valeur(capture, Cote::Joueur)
            ),
            false,
        ),
        Evenement::Epilogue(epilogue) => (epilogue.clone(), true),
        Evenement::Defaite { sac_perdu } => {
            let texte = if sac_perdu.est_vide() {
                "Toute votre équipe est KO : vous rentrez au camp, le sac vide.".to_string()
            } else {
                format!(
                    "Toute votre équipe est KO : vous rentrez au camp et perdez tout votre sac : {}.",
                    contenu(sac_perdu)
                )
            };
            (important(&texte), true)
        }
        Evenement::SoinUtilise {
            soin,
            monstre,
            effet,
        } => (
            soin_utilise(*soin, &nom_en_valeur(monstre, Cote::Joueur), *effet),
            false,
        ),
        Evenement::ZoneExploree(zone) => (
            important(&format!("Vous avez exploré {zone} jusqu'au bout.")),
            true,
        ),
        Evenement::RetourAuCamp => (
            "Retour au camp : vos monstres sont soignés.".to_string(),
            !matches!(precedent, Some(Evenement::ZoneExploree(_))),
        ),

        Evenement::Indice { numero, texte } => {
            (important(&format!("Indice {numero} : {texte}")), true)
        }
        Evenement::NouvelleZone(zone) => (
            format!("Une nouvelle zone apparaît sur la carte : {zone}."),
            false,
        ),
        Evenement::Conseil(conseil) => (texte_du_conseil(*conseil).to_string(), true),
        Evenement::SauvegardeImpossible(erreur) => (
            format!("Sauvegarde impossible : {}.", erreur_sauvegarde(erreur)),
            false,
        ),
        Evenement::Suicide => (
            "Vous retournez votre lame contre vous. Le feu du camp s'éteint, vos monstres se dispersent dans la nuit, et votre nom s'efface des routes. Ailleurs, une autre âme se relève parmi les cendres.".to_string(),
            true,
        ),
        Evenement::EffacementImpossible(erreur) => (
            format!(
                "La sauvegarde n'a pas pu être effacée : {}. Elle sera remplacée à la création de votre nouveau héros.",
                erreur_sauvegarde(erreur)
            ),
            true,
        ),
        Evenement::ExpeditionAutomatiquePartie(expedition) => {
            (expeditions_automatiques::depart(expedition), true)
        }
        Evenement::ExpeditionAutomatiqueRappelee { expedition, places } => {
            (expeditions_automatiques::rappel(expedition, places), true)
        }
        Evenement::ExpeditionAutomatiqueRentree { zone, bilan } => {
            (expeditions_automatiques::retour(zone, bilan), true)
        }
        Evenement::MalvoiseArrive => (ARRIVEE_DE_MALVOISE.to_string(), true),
        Evenement::SoinsAchetes { soin, nombre, prix } => (
            format!("Vous achetez {} pour {}.", soins(*soin, *nombre), argent(*prix)),
            false,
        ),
        Evenement::MonstreVendu { monstre, prix } => (
            format!(
                "Vous vendez {} à Malvoise pour {}.",
                nom_en_valeur(monstre, Cote::Joueur),
                argent(*prix)
            ),
            false,
        ),
        // Tout ranger reste possible avec un sac vide.
        Evenement::RangeAuCoffre(range) if range.est_vide() => {
            ("Le sac est déjà vide.".to_string(), false)
        }
        Evenement::RangeAuCoffre(range) => (
            format!("Vous rangez dans le coffre : {}.", contenu(range)),
            false,
        ),
        Evenement::SortiDuCoffre(sorti) => (
            format!("Vous prenez dans le coffre : {}.", contenu(sorti)),
            false,
        ),
        Evenement::CapsuleAchetee { capsule, prix } => (
            format!(
                "Vous achetez {} pour {}.",
                nom_de_capsule(capsule),
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
                    texte: "**Vous avez exploré Lisière du Bois-Sombre jusqu'au bout.**"
                        .to_string(),
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
            argent_du_pillard: 0,
            capsule: None,
        }])[0];
        assert_eq!(
            recit.texte,
            "Vous trouvez un coffre : 2 potions et 15 pièces d'argent."
        );
        let sans_argent = &raconter(&[Evenement::Coffre {
            potions: 1,
            argent: 0,
            argent_du_pillard: 0,
            capsule: None,
        }])[0];
        assert_eq!(sans_argent.texte, "Vous trouvez un coffre : 1 potion.");
        let avec_capsule = &raconter(&[Evenement::Coffre {
            potions: 1,
            argent: 12,
            argent_du_pillard: 0,
            capsule: Some(brasier()),
        }])[0];
        assert_eq!(
            avec_capsule.texte,
            "Vous trouvez un coffre : 1 potion, 12 pièces d'argent et la capsule d'attaque Brasier."
        );
        let avec_pillard = &raconter(&[Evenement::Coffre {
            potions: 1,
            argent: 22,
            argent_du_pillard: 7,
            capsule: None,
        }])[0];
        assert_eq!(
            avec_pillard.texte,
            "Vous trouvez un coffre : 1 potion et 22 pièces d'argent, dont 7 grâce à l'habileté Pillard."
        );
    }

    #[test]
    fn le_fouineur_annonce_sa_potion() {
        use bersemon_core::catalogue::Catalogue;
        let contenu = bersemon_core::contenu::ContenuV1::new();
        let loup = bersemon_core::monstre::Monstre::creer(
            contenu
                .espece(&bersemon_core::espece::EspeceId::new("loup"))
                .unwrap(),
            bersemon_core::niveau::Niveau::new(5).unwrap(),
            bersemon_core::stats::ValeursInternes::default(),
            false,
        )
        .unwrap();
        let recit = &raconter(&[Evenement::PotionTrouvee(loup)])[0];
        assert_eq!(
            recit.texte,
            "Grâce à son habileté Fouineur, **Loup** déniche une potion, rangée dans le sac."
        );
    }

    #[test]
    fn le_brigand_vaincu_laisse_sa_bourse() {
        let recit = &raconter(&[Evenement::HumainVaincu {
            texte: "Le brigand détale.".to_string(),
            argent: 60,
        }])[0];
        assert_eq!(
            recit.texte,
            "Le brigand détale. Vous ramassez 60 pièces d'argent, qui vont dans le sac."
        );
        assert!(recit.nouveau_paragraphe);
    }

    fn brasier() -> bersemon_core::capsule::Capsule {
        use bersemon_core::catalogue::Catalogue;
        bersemon_core::contenu::ContenuV1::new()
            .capsule(&bersemon_core::attaque::AttaqueId::new("brasier"))
            .unwrap()
    }

    #[test]
    fn une_defaite_dit_ce_que_contenait_le_sac() {
        let sac = bersemon_core::inventaire::Inventaire::new(1, 2, vec![brasier()]);
        let recit = &raconter(&[Evenement::Defaite { sac_perdu: sac }])[0];
        assert_eq!(
            recit.texte,
            "**Toute votre équipe est KO : vous rentrez au camp et perdez tout votre sac : 2 potions, 1 pièce d'argent et la capsule d'attaque Brasier.**"
        );
        let vide = Evenement::Defaite {
            sac_perdu: bersemon_core::inventaire::Inventaire::default(),
        };
        assert_eq!(
            raconter(&[vide])[0].texte,
            "**Toute votre équipe est KO : vous rentrez au camp, le sac vide.**"
        );
    }

    #[test]
    fn une_ame_recueillie_dit_ce_que_malvoise_vendra() {
        let recit = &raconter(&[Evenement::AmeRecueillie {
            texte: "L'âme d'Ulrich s'échappe.".to_string(),
            soin: bersemon_core::soin::Soin::Elixir,
        }])[0];
        assert_eq!(
            recit.texte,
            "L'âme d'Ulrich s'échappe. Malvoise vendra désormais l'élixir."
        );
    }

    #[test]
    fn achat_et_vente_disent_le_prix() {
        let recit = &raconter(&[Evenement::SoinsAchetes {
            soin: bersemon_core::soin::Soin::GrandePotion,
            nombre: 2,
            prix: 120,
        }])[0];
        assert_eq!(
            recit.texte,
            "Vous achetez 2 grandes potions pour 120 pièces d'argent."
        );
    }

    #[test]
    fn ranger_au_coffre_dit_ce_qui_a_ete_range() {
        let range = bersemon_core::inventaire::Inventaire::new(40, 0, Vec::new());
        let recit = &raconter(&[Evenement::RangeAuCoffre(range)])[0];
        assert_eq!(
            recit.texte,
            "Vous rangez dans le coffre : 40 pièces d'argent."
        );
        let rien = bersemon_core::inventaire::Inventaire::default();
        let recit = &raconter(&[Evenement::RangeAuCoffre(rien)])[0];
        assert_eq!(recit.texte, "Le sac est déjà vide.");
    }
}
