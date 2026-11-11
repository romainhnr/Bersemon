//! Les textes des quêtes des personnages du camp (section 13.13,
//! docs/contenu-v1.md, section 52).

use bersemon_core::contenu::ContenuV1;
use bersemon_core::partie::Partie;
use bersemon_core::quete_du_camp::{
    DemandeDeQuete, ErreurQuete, EtatDeLaChaine, Personnage, Quete, RemiseDeQuete,
};

use crate::partie::{contenu as contenu_de, ressources};

/// L'état de la chaîne de chaque personnage, une fois au camp.
pub fn chaines<'c>(
    partie: &Partie,
    contenu: &'c ContenuV1,
) -> Vec<(Personnage, EtatDeLaChaine<'c>)> {
    Personnage::TOUS
        .into_iter()
        .map(|personnage| {
            let etat = partie.chaine(
                personnage,
                contenu.quetes_du_camp(),
                contenu.zones(),
                contenu.piste(),
            );
            (personnage, etat)
        })
        .filter(|(_, etat)| *etat != EtatDeLaChaine::Absent)
        .collect()
}

/// Ce qu'il reste à faire : "Rapporter 5 Cendre de feu follet (vous en avez
/// 3)." ou "Objet à trouver : Psautier de Frère Malvoise (Abbaye des
/// Cendres, étape 3)."
pub fn objectif(partie: &Partie, quete: &Quete) -> String {
    match quete.demande() {
        DemandeDeQuete::Rapporter { ressource, nombre } => {
            let possedees = partie
                .sac()
                .nombre_de_ressources(ressource.id())
                .saturating_add(partie.coffre_du_camp().nombre_de_ressources(ressource.id()));
            format!(
                "Rapporter {nombre} {} (vous en avez {possedees}).",
                ressource.nom()
            )
        }
        DemandeDeQuete::Explorer(exploration) => {
            let objet = exploration.objet();
            if partie.possede_l_objet_de_quete(objet.quete()) {
                format!("Objet à remettre : {}.", objet.nom())
            } else {
                format!(
                    "Objet à trouver : {} ({}, étape {}).",
                    objet.nom(),
                    exploration.nom_de_la_zone(),
                    exploration.etape()
                )
            }
        }
    }
}

/// L'écran des quêtes : chaque personnage au camp, ses quêtes accomplies et
/// où en est sa chaîne. Le texte d'une quête en cours se relit ici.
pub fn ecran_des_quetes_du_camp(partie: &Partie, contenu: &ContenuV1) -> String {
    let chaines = chaines(partie, contenu);
    if chaines.is_empty() {
        return "Personne au camp n'a de demande pour l'instant.".to_string();
    }
    let quetes = contenu.quetes_du_camp();
    chaines
        .iter()
        .map(|(personnage, etat)| {
            let total = quetes
                .iter()
                .filter(|quete| quete.personnage() == *personnage)
                .count();
            let accomplies = partie.quetes_du_camp().accomplies_de(*personnage, quetes);
            let suite = match etat {
                EtatDeLaChaine::EnCours(quete) => format!(
                    "En cours : {}. {}\n{}",
                    quete.titre(),
                    objectif(partie, quete),
                    quete.texte_de_la_demande()
                ),
                EtatDeLaChaine::Proposee(_) => "Une demande à écouter.".to_string(),
                EtatDeLaChaine::RienPourLInstant => {
                    "Rien à vous demander pour l'instant.".to_string()
                }
                EtatDeLaChaine::Achevee => "Toutes ses quêtes sont accomplies.".to_string(),
                EtatDeLaChaine::Absent => String::new(),
            };
            format!(
                "{} ({accomplies}/{total} quêtes accomplies)\n{suite}",
                personnage.nom()
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}

/// Au camp : qui a une demande, et à qui remettre une quête.
pub fn avis_des_quetes_du_camp(partie: &Partie, contenu: &ContenuV1) -> Vec<String> {
    chaines(partie, contenu)
        .into_iter()
        .filter_map(|(personnage, etat)| match etat {
            EtatDeLaChaine::Proposee(_) => {
                Some(format!("{} a une demande pour vous.", personnage.nom()))
            }
            EtatDeLaChaine::EnCours(_) if peut_remettre(partie, contenu, personnage) => Some(
                format!("Vous pouvez remettre sa quête à {}.", personnage.nom()),
            ),
            _ => None,
        })
        .collect()
}

pub fn peut_remettre(partie: &Partie, contenu: &ContenuV1, personnage: Personnage) -> bool {
    partie
        .verifier_remise(
            personnage,
            contenu.quetes_du_camp(),
            contenu.zones(),
            contenu.piste(),
        )
        .is_ok()
}

pub fn quete_ecoutee(quete: &Quete) -> String {
    format!(
        "{}\nNouvelle quête : {}.",
        quete.texte_de_la_demande(),
        quete.titre()
    )
}

pub fn quete_remise(remise: &RemiseDeQuete) -> String {
    let quete = &remise.quete;
    let mut texte = format!(
        "{}\nQuête accomplie : {}. Vous recevez {}, rangés dans le coffre.",
        quete.texte_de_la_remise(),
        quete.titre(),
        contenu_de(quete.recompense())
    );
    if remise.service_ameliore {
        texte.push('\n');
        texte.push_str(service_ameliore(quete.personnage()));
    }
    texte
}

fn service_ameliore(personnage: Personnage) -> &'static str {
    match personnage {
        Personnage::Malvoise => "Le Rite du sang mêlé dure désormais 1 h 30.",
        Personnage::Gunther => "Les travaux de Gunther durent désormais 25 % de moins.",
        Personnage::Ansgar => "La ligne d'Ansgar est désormais prête en 1 h 30.",
    }
}

pub fn erreur_quete(erreur: &ErreurQuete) -> String {
    match erreur {
        ErreurQuete::PersonnageAbsent(personnage) => {
            format!("{} n'est pas au camp.", personnage.nom())
        }
        ErreurQuete::RienADemander(personnage) => {
            format!("{} n'a rien à vous demander.", personnage.nom())
        }
        ErreurQuete::RessourcesManquantes(ressource, manque) => {
            format!("Il vous manque {}.", ressources(ressource, *manque))
        }
        ErreurQuete::ObjetManquant(objet) => {
            format!("Vous n'avez pas l'objet de la quête : {}.", objet.nom())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::heros::{Classe, Heros};
    use bersemon_core::monstre::Monstre;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::EtatPartie;
    use bersemon_core::stats::ValeursInternes;
    use bersemon_core::zone::ZoneId;

    /// Malvoise au camp, le Marais ouvert.
    fn chez_malvoise(contenu: &ContenuV1) -> Partie {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier);
        let depart = Monstre::creer(
            contenu.especes()[0].clone(),
            Niveau::new(10).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        let partie = Partie::nouvelle(heros, depart);
        Partie::restaurer(EtatPartie {
            marchand_au_camp: true,
            indices_trouves: 1,
            zones_explorees: vec![ZoneId::new("lisiere")],
            ..etat(&partie)
        })
        .unwrap()
    }

    fn etat(partie: &Partie) -> EtatPartie {
        EtatPartie {
            heros: partie.heros().clone(),
            collection: partie.collection().clone(),
            sac: partie.sac().clone(),
            coffre_du_camp: partie.coffre_du_camp().clone(),
            marchand_au_camp: partie.marchand_au_camp(),
            ames_donnees: Vec::new(),
            ames_portees: Vec::new(),
            ames_a_retrouver: Vec::new(),
            codex: partie.codex().clone(),
            zones_explorees: Vec::new(),
            indices_trouves: 0,
            apotres_vaincus: Vec::new(),
            humains_vaincus: Vec::new(),
            conseils_vus: Vec::new(),
            expeditions_automatiques: Vec::new(),
            attaques_a_choisir: Vec::new(),
            forge: Default::default(),
            peche: Default::default(),
            relations: Default::default(),
            machines: Default::default(),
            beherit: Default::default(),
            records_de_la_tour: Default::default(),
            rite: None,
            paliers_du_codex: Vec::new(),
            arene: Default::default(),
            succes_obtenus: Vec::new(),
            quetes: Default::default(),
            quetes_du_camp: Default::default(),
            decouvertes: Default::default(),
        }
    }

    #[test]
    fn l_ecran_des_quetes_suit_la_chaine() {
        let contenu = ContenuV1::new();
        let mut partie = chez_malvoise(&contenu);
        assert_eq!(
            ecran_des_quetes_du_camp(&partie, &contenu),
            "Malvoise (0/4 quêtes accomplies)\nUne demande à écouter."
        );
        assert_eq!(
            avis_des_quetes_du_camp(&partie, &contenu),
            ["Malvoise a une demande pour vous."]
        );
        let quete = partie
            .ecouter(
                Personnage::Malvoise,
                contenu.quetes_du_camp(),
                contenu.zones(),
                contenu.piste(),
            )
            .unwrap();
        assert!(quete_ecoutee(&quete).ends_with("Nouvelle quête : La lueur du crâne."));
        let ecran = ecran_des_quetes_du_camp(&partie, &contenu);
        assert!(
            ecran.contains(
                "En cours : La lueur du crâne. Rapporter 15 Cendre de feu follet (vous en avez 0)."
            ),
            "{ecran}"
        );
        assert!(avis_des_quetes_du_camp(&partie, &contenu).is_empty());
    }
}
