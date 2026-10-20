//! Expéditions automatiques (section 12) : état des zones, estimation avant
//! le départ, départ, rappel, retour et durées.

use std::time::Duration;

use bersemon_core::collection::{Emplacement, ErreurCollection};
use bersemon_core::expedition_automatique::{
    BilanExpeditionAutomatique, Chances, ErreurExpeditionAutomatique, Estimation,
    ExpeditionAutomatique, MONSTRES_MAX, MONSTRES_MIN, PROTECTION_DU_CHARME, Protection,
};
use bersemon_core::horodatage::Horodatage;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::zone::Zone;

use crate::erreurs::erreur_collection;
use crate::partie::{argent_et_part_du_pillard, potions};

/// Une zone explorée : libre, en cours ou rentrée (section 12.1).
pub fn etat_de_la_zone(partie: &Partie, zone: &Zone, maintenant: Horodatage) -> String {
    match partie.expedition_automatique(zone.id()) {
        None => format!("{} : libre", zone.nom()),
        Some(expedition) if expedition.est_rentree(maintenant) => format!(
            "{} : rentrée, à accueillir ({})",
            zone.nom(),
            noms(expedition.monstres())
        ),
        Some(expedition) => format!(
            "{} : en cours, retour dans {} ({})",
            zone.nom(),
            duree(expedition.temps_restant(maintenant)),
            noms(expedition.monstres())
        ),
    }
}

/// Une ligne par expédition qui attend d'être accueillie (section 12.7).
pub fn expeditions_rentrees(partie: &Partie, maintenant: Horodatage) -> Vec<String> {
    partie
        .expeditions_automatiques()
        .iter()
        .filter(|expedition| expedition.est_rentree(maintenant))
        .map(|expedition| {
            format!(
                "L'expédition partie pour {} est rentrée : accueillez-la.",
                expedition.zone().nom()
            )
        })
        .collect()
}

/// Ce que le joueur voit avant le départ (section 12.4).
pub fn estimation(zone: &Zone, estimation: &Estimation) -> String {
    let (minimum, maximum) = zone.niveaux();
    let degats = zone.expedition_automatique().degats();
    format!(
        "Difficulté : niveaux {} à {}. Dégâts à encaisser : {} à {}.\nDurée : {}. Chances de réussite : {}.",
        minimum.valeur(),
        maximum.valeur(),
        degats.start(),
        degats.end(),
        duree(estimation.duree),
        chances(estimation.chances)
    )
}

/// Le monstre protégé et sa protection, avant le départ (section 12.5) ;
/// `monstres` dans l'ordre de l'expédition.
pub fn ligne_de_protection(monstres: &[Monstre], protection: Protection) -> String {
    match protection.protege.and_then(|protege| monstres.get(protege)) {
        Some(monstre) => {
            let charme = if protection.charme {
                ", avec un charme de garde"
            } else {
                ""
            };
            format!(
                "Monstre protégé : {}, à {} %{charme} (25 % par autre monstre de soutien, {PROTECTION_DU_CHARME} % avec un charme).",
                monstre.nom(),
                protection.pourcentage(monstres)
            )
        }
        None => "Aucun monstre protégé.".to_string(),
    }
}

/// Arrondies à l'entier inférieur ; une réussite possible n'est jamais
/// annoncée à 0 %.
pub fn chances(chances: Chances) -> String {
    match chances.pourcentage() {
        0 if chances.favorables > 0 => "moins de 1 %".to_string(),
        pourcentage => format!("{pourcentage} %"),
    }
}

pub fn erreur_expedition_automatique(erreur: &ErreurExpeditionAutomatique) -> String {
    match erreur {
        ErreurExpeditionAutomatique::ZoneNonExploree => {
            "Menez d'abord une expédition jusqu'au bout dans cette zone.".to_string()
        }
        ErreurExpeditionAutomatique::DejaEnCours => {
            "Une expédition automatique est déjà en cours dans cette zone.".to_string()
        }
        ErreurExpeditionAutomatique::NombreDeMonstres(_) => {
            format!("Envoyez de {MONSTRES_MIN} à {MONSTRES_MAX} monstres.")
        }
        // Les monstres pris dans l'équipe gardent une place au camp, où ils
        // reviendront (section 12.1) : c'est elle qui manque.
        ErreurExpeditionAutomatique::Collection(ErreurCollection::CampPlein) => {
            "Le camp est plein : un monstre de l'équipe a besoin d'une place au camp, où il reviendra. Envoyez des monstres du camp, ou faites de la place."
                .to_string()
        }
        ErreurExpeditionAutomatique::Collection(erreur) => erreur_collection(erreur),
        ErreurExpeditionAutomatique::Inexistante => {
            "Aucune expédition automatique dans cette zone.".to_string()
        }
        ErreurExpeditionAutomatique::ProtectionImpossible => {
            "Un monstre ne se protège qu'avec au moins un autre, qui reprend sa part des dégâts."
                .to_string()
        }
        ErreurExpeditionAutomatique::CharmeSansProtege => {
            "Un charme de garde protège un monstre : désignez-le d'abord.".to_string()
        }
        ErreurExpeditionAutomatique::AucunCharme => {
            "Vous n'avez aucun charme de garde : Gunther en forge à sa forge.".to_string()
        }
        ErreurExpeditionAutomatique::PasEncoreRentree => {
            "L'expédition n'est pas encore rentrée.".to_string()
        }
        ErreurExpeditionAutomatique::DejaRentree => {
            "L'expédition est déjà rentrée : accueillez-la.".to_string()
        }
    }
}

/// Arrondie à la minute supérieure, pour qu'un temps restant ne s'affiche
/// jamais à 0 avant le retour.
pub fn duree(duree: Duration) -> String {
    let minutes = duree.as_secs().div_ceil(60);
    match (minutes / 60, minutes % 60) {
        (0, 0) => "moins d'une minute".to_string(),
        (0, minutes) => format!("{minutes} min"),
        (heures, 0) => format!("{heures} h"),
        (heures, minutes) => format!("{heures} h {minutes:02}"),
    }
}

pub fn depart(expedition: &ExpeditionAutomatique) -> String {
    let monstres = expedition.monstres();
    format!(
        "{} {} pour {}, sans vous. Retour prévu dans {}.",
        noms(monstres),
        if monstres.len() > 1 {
            "partent"
        } else {
            "part"
        },
        expedition.zone().nom(),
        duree(expedition.depart().duree_jusqu_a(expedition.retour()))
    )
}

pub fn rappel(expedition: &ExpeditionAutomatique, places: &[Emplacement]) -> String {
    format!(
        "Vous rappelez l'expédition partie pour {} : elle ne rapporte rien. {}.",
        expedition.zone().nom(),
        destinations(expedition.monstres().iter().zip(places.iter().copied()))
    )
}

/// "a rejoint l'équipe ; b et c retournent au camp" (section 12.7).
fn destinations<'a>(retours: impl Iterator<Item = (&'a Monstre, Emplacement)>) -> String {
    let (equipe, camp): (Vec<_>, Vec<_>) =
        retours.partition(|(_, place)| matches!(place, Emplacement::Equipe(_)));
    let groupe = |monstres: Vec<(&Monstre, Emplacement)>, un: &str, plusieurs: &str| {
        let monstres: Vec<Monstre> = monstres.into_iter().map(|(m, _)| m.clone()).collect();
        let verbe = if monstres.len() > 1 { plusieurs } else { un };
        format!("{} {verbe}", noms(&monstres))
    };
    let mut parties = Vec::new();
    if !equipe.is_empty() {
        parties.push(groupe(equipe, "rejoint l'équipe", "rejoignent l'équipe"));
    }
    if !camp.is_empty() {
        parties.push(groupe(camp, "retourne au camp", "retournent au camp"));
    }
    parties.join(" ; ")
}

pub fn question_de_rappel(expedition: &ExpeditionAutomatique) -> String {
    format!(
        "Rappeler l'expédition partie pour {} ? Elle ne rapportera rien.",
        expedition.zone().nom()
    )
}

/// Le bilan, une ligne par monstre ; l'XP gagnée suit dans son propre
/// récit.
pub fn retour(zone: &str, bilan: &BilanExpeditionAutomatique) -> String {
    let mut lignes = vec![format!(
        "L'expédition partie pour {zone} est de retour : elle a dû encaisser {} dégâts.",
        bilan.degats
    )];
    for (position, rentre) in bilan.monstres.iter().enumerate() {
        let ko = if rentre.ko { " et tombe KO" } else { "" };
        let protege = match bilan.protection {
            Some((protege, pourcentage)) if protege == position => {
                format!(", protégé à {pourcentage} %,")
            }
            _ => String::new(),
        };
        lignes.push(format!(
            "{}{protege} perd {} PV{ko}.",
            rentre.monstre.nom(),
            rentre.degats_subis
        ));
    }
    let mut gains = Vec::new();
    if bilan.potions > 0 {
        gains.push(potions(bilan.potions));
    }
    if bilan.argent > 0 {
        let mut argent = argent_et_part_du_pillard(bilan.argent, bilan.argent_du_pillard);
        if bilan.argent_des_offensifs > 0 {
            let lien = if bilan.argent_du_pillard > 0 {
                " et"
            } else {
                ", dont"
            };
            argent.push_str(&format!(
                "{lien} {} grâce aux monstres offensifs",
                bilan.argent_des_offensifs
            ));
        }
        gains.push(argent);
    }
    let conclusion = if !bilan.reussite {
        "Échec : tous vos monstres sont tombés KO, l'expédition ne rapporte rien.".to_string()
    } else if gains.is_empty() {
        "Réussite, mais vos monstres ne rapportent rien.".to_string()
    } else {
        format!("Réussite : vos monstres rapportent {}.", gains.join(" et "))
    };
    lignes.push(conclusion);
    let retours = bilan
        .monstres
        .iter()
        .map(|rentre| (&rentre.monstre, rentre.emplacement));
    lignes.push(format!("Soins faits : {}.", destinations(retours)));
    lignes.join("\n")
}

/// "a", "a et b", "a, b et c".
pub fn noms(monstres: &[Monstre]) -> String {
    let noms: Vec<&str> = monstres.iter().map(Monstre::nom).collect();
    match noms.split_last() {
        None => String::new(),
        Some((dernier, [])) => dernier.to_string(),
        Some((dernier, autres)) => format!("{} et {dernier}", autres.join(", ")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::collection::Emplacement;
    use bersemon_core::contenu::ContenuV1;
    use bersemon_core::espece::EspeceId;
    use bersemon_core::heros::{Classe, Heros};
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::EtatPartie;
    use bersemon_core::stats::ValeursInternes;
    use bersemon_core::zone::ZoneId;

    fn creer(contenu: &ContenuV1, id: &str) -> Monstre {
        let espece = contenu.espece(&EspeceId::new(id)).unwrap();
        Monstre::creer(
            espece,
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    /// Une Salamandre et un Loup, avec la Lisière explorée.
    fn partie(contenu: &ContenuV1) -> Partie {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
        let mut partie = Partie::nouvelle(heros, creer(contenu, "salamandre"));
        partie.prendre_monstre_rare(creer(contenu, "loup"));
        Partie::restaurer(EtatPartie {
            heros: partie.heros().clone(),
            collection: partie.collection().clone(),
            sac: partie.sac().clone(),
            coffre_du_camp: partie.coffre_du_camp().clone(),
            marchand_au_camp: partie.marchand_au_camp(),
            ames_donnees: partie.ames_donnees().to_vec(),
            ames_portees: partie.ames_portees().to_vec(),
            ames_a_retrouver: partie.ames_a_retrouver().to_vec(),
            codex: partie.codex().clone(),
            zones_explorees: vec![ZoneId::new("lisiere")],
            indices_trouves: 1,
            apotres_vaincus: Vec::new(),
            humains_vaincus: Vec::new(),
            conseils_vus: Vec::new(),
            expeditions_automatiques: Vec::new(),
            attaques_a_choisir: Vec::new(),
            peche: Default::default(),
            forge: Default::default(),
            relations: Default::default(),
            machines: Default::default(),
        })
        .unwrap()
    }

    #[test]
    fn etat_d_une_zone_libre_en_cours_puis_rentree() {
        let contenu = ContenuV1::new();
        let lisiere = &contenu.zones()[0];
        let mut partie = partie(&contenu);
        let a = Horodatage::depuis_secondes;
        assert_eq!(
            etat_de_la_zone(&partie, lisiere, a(0)),
            "Lisière du Bois-Sombre : libre"
        );
        partie
            .envoyer_en_expedition_automatique(lisiere, &[Emplacement::Equipe(1)], a(0))
            .unwrap();
        assert_eq!(
            etat_de_la_zone(&partie, lisiere, a(1_200)),
            "Lisière du Bois-Sombre : en cours, retour dans 40 min (Loup)"
        );
        assert!(expeditions_rentrees(&partie, a(1_200)).is_empty());
        assert_eq!(
            etat_de_la_zone(&partie, lisiere, a(3_600)),
            "Lisière du Bois-Sombre : rentrée, à accueillir (Loup)"
        );
        assert_eq!(
            expeditions_rentrees(&partie, a(3_600)),
            vec!["L'expédition partie pour Lisière du Bois-Sombre est rentrée : accueillez-la."]
        );
    }

    #[test]
    fn le_retour_dit_les_potions_et_l_argent_rapportes() {
        let contenu = ContenuV1::new();
        let lisiere = &contenu.zones()[0];
        let mut partie = partie(&contenu);
        let a = Horodatage::depuis_secondes;
        partie
            .envoyer_en_expedition_automatique(lisiere, &[Emplacement::Equipe(1)], a(0))
            .unwrap();
        let mut aleatoire = bersemon_core::aleatoire::GenerateurAleatoire::new(1);
        let mut bilan = partie
            .accueillir_expedition_automatique(lisiere.id(), a(3_600), &mut aleatoire)
            .unwrap();
        bilan.reussite = true;
        bilan.potions = 1;
        bilan.argent = 25;
        bilan.argent_des_offensifs = 0;
        let texte = retour("Lisière du Bois-Sombre", &bilan);
        assert!(
            texte.contains("Réussite : vos monstres rapportent 1 potion et 25 pièces d'argent."),
            "{texte}"
        );
        assert!(
            texte.ends_with("Soins faits : Loup rejoint l'équipe."),
            "{texte}"
        );
        bilan.monstres.push(bilan.monstres[0].clone());
        bilan.monstres[1].emplacement = Emplacement::Camp(0);
        bilan.monstres.push(bilan.monstres[1].clone());
        let texte = retour("Lisière du Bois-Sombre", &bilan);
        assert!(
            texte.ends_with(
                "Soins faits : Loup rejoint l'équipe ; Loup et Loup retournent au camp."
            ),
            "{texte}"
        );
        bilan.monstres.truncate(1);
        bilan.argent_du_pillard = 8;
        let texte = retour("Lisière du Bois-Sombre", &bilan);
        assert!(
            texte.contains("25 pièces d'argent, dont 8 grâce à l'habileté Pillard."),
            "{texte}"
        );
        // Les monstres offensifs et la protection (sections 12.5 et 12.6).
        bilan.argent_des_offensifs = 2;
        bilan.protection = Some((0, 50));
        let texte = retour("Lisière du Bois-Sombre", &bilan);
        assert!(
            texte.contains(
                "25 pièces d'argent, dont 8 grâce à l'habileté Pillard et 2 grâce aux monstres offensifs."
            ),
            "{texte}"
        );
        assert!(texte.contains("Loup, protégé à 50 %, perd"), "{texte}");
        bilan.argent_du_pillard = 0;
        let texte = retour("Lisière du Bois-Sombre", &bilan);
        assert!(
            texte.contains("25 pièces d'argent, dont 2 grâce aux monstres offensifs."),
            "{texte}"
        );
        bilan.argent_des_offensifs = 0;
        bilan.potions = 0;
        bilan.argent = 0;
        bilan.argent_du_pillard = 0;
        let texte = retour("Lisière du Bois-Sombre", &bilan);
        assert!(
            texte.contains("Réussite, mais vos monstres ne rapportent rien."),
            "{texte}"
        );
    }

    #[test]
    fn un_camp_plein_s_explique_et_dit_quoi_faire() {
        let erreur = ErreurExpeditionAutomatique::Collection(ErreurCollection::CampPlein);
        let texte = erreur_expedition_automatique(&erreur);
        assert!(texte.contains("où il reviendra"), "{texte}");
        assert!(texte.contains("Envoyez des monstres du camp"), "{texte}");
    }

    #[test]
    fn chances_arrondies_sans_annoncer_0_pour_une_reussite_possible() {
        let chances_sur = |favorables, possibles| {
            chances(Chances {
                favorables,
                possibles,
            })
        };
        assert_eq!(chances_sur(33, 46), "71 %");
        assert_eq!(chances_sur(46, 46), "100 %");
        assert_eq!(chances_sur(1, 1_000), "moins de 1 %");
        assert_eq!(chances_sur(0, 81), "0 %");
    }

    #[test]
    fn noms_enumeres() {
        let contenu = ContenuV1::new();
        let monstres = ["loup", "vipere", "corbeau"].map(|id| creer(&contenu, id));
        assert_eq!(noms(&monstres[..1]), "Loup");
        assert_eq!(noms(&monstres[..2]), "Loup et Vipère");
        assert_eq!(noms(&monstres), "Loup, Vipère et Corbeau");
    }

    #[test]
    fn durees_arrondies_a_la_minute_superieure() {
        assert_eq!(duree(Duration::from_secs(3_600)), "1 h");
        assert_eq!(duree(Duration::from_secs(5_400)), "1 h 30");
        assert_eq!(duree(Duration::from_secs(3_601)), "1 h 01");
        assert_eq!(duree(Duration::from_secs(2_700)), "45 min");
        assert_eq!(duree(Duration::from_secs(1)), "1 min");
        assert_eq!(duree(Duration::ZERO), "moins d'une minute");
    }
}
