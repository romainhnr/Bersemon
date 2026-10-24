//! Tour sans fond (section 9.5) : textes communs à la console et au bot
//! Discord.

use bersemon_core::contenu::ContenuV1;
use bersemon_core::partie::Partie;
use bersemon_core::tour::RecordsDeLaTour;

use crate::echanges::joueur;

/// Dans la carte de la console, après les zones, une fois la Tour ouverte :
/// "Tour sans fond (sans fin) : record, étape 13".
pub fn destination_de_la_tour(partie: &Partie, contenu: &ContenuV1) -> Option<String> {
    let tour = contenu.tour();
    partie
        .tour_ouverte(tour)
        .then(|| format!("{} (sans fin) : {}", tour.nom(), record(partie)))
}

/// Sous le nom de la Tour, dans le menu des expéditions du bot :
/// "Sans fin ; record, étape 13".
pub fn description_de_la_tour(partie: &Partie, contenu: &ContenuV1) -> Option<String> {
    partie
        .tour_ouverte(contenu.tour())
        .then(|| format!("Sans fin ; {}", record(partie)))
}

/// Le classement des records, seul et à deux, sous ceux du joueur : les
/// joueurs classés, avec leur héros, leur pseudo s'il est connu, et leur
/// record.
pub fn records_de_la_tour(
    vos_records: RecordsDeLaTour,
    seul: &[(String, Option<String>, u32)],
    a_deux: &[(String, Option<String>, u32)],
) -> String {
    let etape = |record: u32| match record {
        0 => "aucun".to_string(),
        etape => format!("étape {etape}"),
    };
    let liste = |classes: &[(String, Option<String>, u32)]| -> String {
        if classes.is_empty() {
            return "Aucun record pour l'instant.".to_string();
        }
        classes
            .iter()
            .enumerate()
            .map(|(rang, (heros, pseudo, record))| {
                format!(
                    "{}. {} : étape {record}",
                    rang + 1,
                    joueur(heros, pseudo.as_deref())
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    format!(
        "Records de la Tour sans fond.\nVos records : seul, {} ; à deux, {}.\n\nSeul :\n{}\n\nÀ deux :\n{}",
        etape(vos_records.seul),
        etape(vos_records.a_deux),
        liste(seul),
        liste(a_deux)
    )
}

fn record(partie: &Partie) -> String {
    match partie.records_de_la_tour().seul {
        0 => "descendre le plus loin possible".to_string(),
        etape => format!("record, étape {etape}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::espece::EspeceId;
    use bersemon_core::heros::{Classe, Heros};
    use bersemon_core::monstre::Monstre;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::EtatPartie;
    use bersemon_core::stats::ValeursInternes;
    use bersemon_core::tour::RecordsDeLaTour;
    use bersemon_core::zone::ZoneId;

    fn partie(contenu: &ContenuV1, lisiere: bool, record: u32) -> Partie {
        let depart = Monstre::creer(
            contenu.espece(&EspeceId::new("crapaud")).unwrap(),
            Niveau::new(10).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        let partie = Partie::nouvelle(
            Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier),
            depart,
        );
        Partie::restaurer(EtatPartie {
            heros: partie.heros().clone(),
            collection: partie.collection().clone(),
            sac: partie.sac().clone(),
            coffre_du_camp: partie.coffre_du_camp().clone(),
            marchand_au_camp: lisiere,
            ames_donnees: Vec::new(),
            ames_portees: Vec::new(),
            ames_a_retrouver: Vec::new(),
            codex: partie.codex().clone(),
            zones_explorees: if lisiere {
                vec![ZoneId::new("lisiere")]
            } else {
                Vec::new()
            },
            indices_trouves: usize::from(lisiere),
            apotres_vaincus: Vec::new(),
            humains_vaincus: Vec::new(),
            conseils_vus: Vec::new(),
            expeditions_automatiques: Vec::new(),
            attaques_a_choisir: Vec::new(),
            peche: Default::default(),
            forge: Default::default(),
            relations: Default::default(),
            machines: Default::default(),
            beherit: Default::default(),
            records_de_la_tour: RecordsDeLaTour {
                seul: record,
                a_deux: 0,
            },
        })
        .unwrap()
    }

    #[test]
    fn records_classes_seul_et_a_deux() {
        let seul = [
            ("Aldric".to_string(), None, 23),
            ("Ysolde".to_string(), Some("ysolde".to_string()), 12),
        ];
        assert_eq!(
            records_de_la_tour(
                RecordsDeLaTour {
                    seul: 23,
                    a_deux: 0
                },
                &seul,
                &[]
            ),
            "Records de la Tour sans fond.\nVos records : seul, étape 23 ; à deux, aucun.\n\nSeul :\n1. Aldric : étape 23\n2. Ysolde (ysolde) : étape 12\n\nÀ deux :\nAucun record pour l'instant."
        );
    }

    #[test]
    fn la_tour_se_propose_une_fois_la_lisiere_menee_au_bout() {
        let contenu = ContenuV1::new();
        assert_eq!(
            destination_de_la_tour(&partie(&contenu, false, 0), &contenu),
            None
        );
        assert_eq!(
            destination_de_la_tour(&partie(&contenu, true, 0), &contenu).as_deref(),
            Some("Tour sans fond (sans fin) : descendre le plus loin possible")
        );
        assert_eq!(
            description_de_la_tour(&partie(&contenu, true, 13), &contenu).as_deref(),
            Some("Sans fin ; record, étape 13")
        );
    }
}
