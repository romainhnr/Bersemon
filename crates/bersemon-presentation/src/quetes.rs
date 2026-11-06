//! Les quêtes du jour et de la semaine (section 13.12) : l'écran des
//! quêtes et le récit d'une quête accomplie, dans la console comme sur
//! Discord. Textes dans docs/contenu-v1.md, section 50.

use std::time::Duration;

use bersemon_core::contenu::ContenuV1;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::partie::Partie;
use bersemon_core::quete::{EtatDeQuete, Objectif, Periode, Quete, QueteAccomplie};
use bersemon_core::zone::Zone;

use crate::expeditions_automatiques::duree;
use crate::partie::{argent, contenu as objets};

/// Rappelé sous les quêtes : rien n'est à réclamer.
pub const VERSEMENT_AU_COFFRE: &str =
    "Une quête accomplie verse aussitôt sa récompense au coffre du camp, même en expédition.";

/// "Vaincre 15 monstres sauvages", "Réussir 1 expédition automatique".
pub fn intitule(quete: Quete) -> String {
    let cible = quete.cible();
    let (verbe, singulier, pluriel) = match quete.objectif {
        Objectif::VaincreDesMonstresSauvages => ("Vaincre", "monstre sauvage", "monstres sauvages"),
        Objectif::CapturerDesMonstres => ("Capturer", "monstre", "monstres"),
        Objectif::OuvrirDesCoffres => ("Ouvrir", "coffre en expédition", "coffres en expédition"),
        Objectif::RecolterDesRessources => (
            "Récolter",
            "ressource sur les monstres vaincus",
            "ressources sur les monstres vaincus",
        ),
        Objectif::MenerDesExpeditionsAuBout => (
            "Mener",
            "expédition jusqu'au bout",
            "expéditions jusqu'au bout",
        ),
        Objectif::ReussirDesExpeditionsAutomatiques => (
            "Réussir",
            "expédition automatique",
            "expéditions automatiques",
        ),
    };
    let nom = if cible > 1 { pluriel } else { singulier };
    format!("{verbe} {cible} {nom}")
}

/// "Quêtes du jour", "Quêtes de la semaine".
fn titre(periode: Periode) -> &'static str {
    match periode {
        Periode::Jour => "Quêtes du jour",
        Periode::Semaine => "Quêtes de la semaine",
    }
}

/// "5 h 12", "3 j 4 h" : au-delà d'un jour, les minutes ne comptent plus.
fn delai(restant: Duration) -> String {
    const JOUR: u64 = 24 * 3_600;
    let secondes = restant.as_secs();
    if secondes < JOUR {
        return duree(restant);
    }
    match (secondes / JOUR, (secondes % JOUR) / 3_600) {
        (jours, 0) => format!("{jours} j"),
        (jours, heures) => format!("{jours} j {heures} h"),
    }
}

/// "de 20 à 50 pièces d'argent et un booster", dans la zone de la
/// récompense.
fn recompense_possible(quete: Quete, zone: &Zone) -> String {
    let (minimum, maximum) = quete.argent_possible(zone);
    let argent = if minimum == maximum {
        argent(minimum)
    } else {
        format!("de {minimum} à {}", argent(maximum))
    };
    if quete.donne_un_booster() {
        format!("{argent} et un booster")
    } else {
        argent
    }
}

/// "  Vaincre 15 monstres sauvages : 7/15. Récompense : de 20 à 50 pièces
/// d'argent." ou "  Ouvrir 2 coffres en expédition : accomplie."
fn ligne(etat: &EtatDeQuete, zone: Option<&Zone>) -> String {
    let intitule = intitule(etat.quete);
    if etat.accomplie {
        return format!("  {intitule} : accomplie.");
    }
    let recompense = zone
        .map(|zone| format!(" Récompense : {}.", recompense_possible(etat.quete, zone)))
        .unwrap_or_default();
    format!(
        "  {intitule} : {}/{}.{recompense}",
        etat.progres,
        etat.quete.cible()
    )
}

/// L'écran des quêtes : celles du jour, puis celles de la semaine, avec le
/// temps avant les suivantes, et la récompense de chacune à la mesure de la
/// zone ouverte la plus avancée.
pub fn ecran_des_quetes(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> String {
    let zone = partie.zone_la_plus_avancee(contenu.zones(), contenu.piste());
    let mut lignes = Vec::new();
    for periode in Periode::TOUTES {
        lignes.push(format!(
            "{} (nouvelles dans {}) :",
            titre(periode),
            delai(periode.duree_restante(maintenant))
        ));
        lignes.extend(
            partie
                .quetes()
                .quetes(periode, maintenant)
                .iter()
                .map(|etat| ligne(etat, zone)),
        );
    }
    lignes.push(VERSEMENT_AU_COFFRE.to_string());
    lignes.join("\n")
}

/// "Quête du jour accomplie : vaincre 15 monstres sauvages. Récompense, au
/// coffre : 34 pièces d'argent."
pub fn quete_accomplie(accomplie: &QueteAccomplie) -> String {
    let periode = match accomplie.quete.periode {
        Periode::Jour => "Quête du jour",
        Periode::Semaine => "Quête de la semaine",
    };
    let intitule = intitule(accomplie.quete);
    let mut lettres = intitule.chars();
    let intitule = match lettres.next() {
        Some(initiale) => initiale.to_lowercase().chain(lettres).collect(),
        None => intitule,
    };
    format!(
        "{periode} accomplie : {intitule}. Récompense, au coffre : {}.",
        objets(&accomplie.recompense)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::booster::Booster;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::espece::EspeceId;
    use bersemon_core::heros::{Classe, Heros};
    use bersemon_core::inventaire::Inventaire;
    use bersemon_core::monstre::Monstre;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::stats::ValeursInternes;

    /// Une partie qui commence : seule la Lisière est ouverte.
    fn partie_de_depart(contenu: &ContenuV1) -> Partie {
        let depart = Monstre::creer(
            contenu.espece(&EspeceId::new("salamandre")).unwrap(),
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        Partie::nouvelle(
            Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage),
            depart,
        )
    }

    fn quete(periode: Periode, objectif: Objectif) -> Quete {
        Quete { periode, objectif }
    }

    #[test]
    fn chaque_quete_dit_ce_qu_elle_demande() {
        assert_eq!(
            intitule(quete(Periode::Jour, Objectif::VaincreDesMonstresSauvages)),
            "Vaincre 15 monstres sauvages"
        );
        assert_eq!(
            intitule(quete(
                Periode::Jour,
                Objectif::ReussirDesExpeditionsAutomatiques
            )),
            "Réussir 1 expédition automatique"
        );
        assert_eq!(
            intitule(quete(
                Periode::Semaine,
                Objectif::ReussirDesExpeditionsAutomatiques
            )),
            "Réussir 5 expéditions automatiques"
        );
        assert_eq!(
            intitule(quete(Periode::Semaine, Objectif::OuvrirDesCoffres)),
            "Ouvrir 10 coffres en expédition"
        );
    }

    #[test]
    fn le_delai_compte_les_jours_au_dela_d_un_jour() {
        assert_eq!(delai(Duration::from_secs(5 * 3_600 + 12 * 60)), "5 h 12");
        assert_eq!(
            delai(Duration::from_secs(3 * 86_400 + 4 * 3_600 + 59)),
            "3 j 4 h"
        );
        assert_eq!(delai(Duration::from_secs(2 * 86_400)), "2 j");
    }

    #[test]
    fn l_ecran_montre_le_jour_puis_la_semaine() {
        let contenu = ContenuV1::new();
        let partie = partie_de_depart(&contenu);
        // Jeudi 1er janvier 1970 à 21 h : 3 h avant les quêtes du jour
        // suivant, 3 j 3 h avant celles de la semaine suivante.
        let maintenant = Horodatage::depuis_secondes(21 * 3_600);
        let ecran = ecran_des_quetes(&partie, &contenu, maintenant);
        let lignes: Vec<&str> = ecran.lines().collect();
        assert_eq!(lignes.len(), 9, "{ecran}");
        assert_eq!(lignes[0], "Quêtes du jour (nouvelles dans 3 h) :");
        assert_eq!(lignes[4], "Quêtes de la semaine (nouvelles dans 3 j 3 h) :");
        assert_eq!(lignes[8], VERSEMENT_AU_COFFRE);
        // Le jour 0 demande d'ouvrir 2 coffres ; ceux de la Lisière ont de
        // 10 à 25 pièces d'argent.
        assert!(
            lignes[1..4].contains(
                &"  Ouvrir 2 coffres en expédition : 0/2. Récompense : de 20 à 50 pièces d'argent."
            ),
            "{ecran}"
        );
        assert!(
            lignes[5..8].contains(
                &"  Ouvrir 10 coffres en expédition : 0/10. Récompense : de 50 à 125 pièces d'argent et un booster."
            ),
            "{ecran}"
        );
    }

    #[test]
    fn une_quete_accomplie_dit_sa_recompense() {
        let mut recompense = Inventaire::default();
        recompense.ajouter_argent(34);
        let accomplie = QueteAccomplie {
            quete: quete(Periode::Jour, Objectif::VaincreDesMonstresSauvages),
            recompense: recompense.clone(),
        };
        assert_eq!(
            quete_accomplie(&accomplie),
            "Quête du jour accomplie : vaincre 15 monstres sauvages. Récompense, au coffre : 34 pièces d'argent."
        );
        recompense.ajouter_boosters(Booster::FioleDeSang, 1);
        let accomplie = QueteAccomplie {
            quete: quete(Periode::Semaine, Objectif::CapturerDesMonstres),
            recompense,
        };
        assert_eq!(
            quete_accomplie(&accomplie),
            "Quête de la semaine accomplie : capturer 15 monstres. Récompense, au coffre : 34 pièces d'argent et 1 Fiole de sang."
        );
    }
}
