//! Béhérit pourpre, Éclipse et Démiurge (section 9.4) : textes communs à la
//! console et au bot Discord.

use bersemon_core::beherit::EtatDuBeherit;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::partie::Partie;

/// L'option qui forge le Béhérit, chez Malvoise, parmi les âmes à donner.
pub const FORGER_LE_BEHERIT: &str = "Âmes des Apôtres : forger le Béhérit pourpre";

pub const PARTIR_VERS_L_ECLIPSE: &str = "Partir vers l'Éclipse";

/// Avant le premier départ vers l'Éclipse.
pub const DEMANDE_DE_SACRIFICE: &str = "Le Béhérit pourpre réclame un sacrifice pour ouvrir l'Éclipse : choisissez un de vos monstres de niveau 30 ou plus. Il sera perdu pour toujours.";

/// Malvoise ne forge le Béhérit qu'une fois tous les Apôtres vaincus.
pub fn peut_forger_le_beherit(partie: &Partie, contenu: &ContenuV1) -> bool {
    partie.verifier_la_forge_du_beherit(contenu.piste()).is_ok()
}

/// Le lieu se propose avec le Béhérit, tant que le Démiurge n'est pas
/// vaincu : "Éclipse (niveau 42) : le Démiurge", avec le sacrifice à faire
/// avant le premier départ.
pub fn destination_de_l_eclipse(partie: &Partie, contenu: &ContenuV1) -> Option<String> {
    let niveau = contenu.invocation().demiurge().niveau().valeur();
    let nom = contenu.invocation().eclipse().nom();
    match partie.beherit() {
        EtatDuBeherit::Forge => Some(format!(
            "{nom} (niveau {niveau}) : le Démiurge ; le Béhérit réclame d'abord un sacrifice"
        )),
        EtatDuBeherit::EclipseOuverte => Some(format!("{nom} (niveau {niveau}) : le Démiurge")),
        EtatDuBeherit::Aucun | EtatDuBeherit::DemiurgeVaincu => None,
    }
}

/// Sous le nom du lieu, dans le menu des expéditions du bot : "Niveau 42 :
/// le Démiurge", avec le sacrifice à faire avant le premier départ.
pub fn description_de_l_eclipse(partie: &Partie, contenu: &ContenuV1) -> Option<String> {
    let niveau = contenu.invocation().demiurge().niveau().valeur();
    match partie.beherit() {
        EtatDuBeherit::Forge => Some(format!("Niveau {niveau} : le Démiurge, après un sacrifice")),
        EtatDuBeherit::EclipseOuverte => Some(format!("Niveau {niveau} : le Démiurge")),
        EtatDuBeherit::Aucun | EtatDuBeherit::DemiurgeVaincu => None,
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

    fn partie(contenu: &ContenuV1, beherit: EtatDuBeherit) -> Partie {
        let depart = Monstre::creer(
            contenu.espece(&EspeceId::new("crapaud")).unwrap(),
            Niveau::new(40).unwrap(),
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
            marchand_au_camp: true,
            ames_donnees: Vec::new(),
            ames_portees: Vec::new(),
            ames_a_retrouver: Vec::new(),
            codex: partie.codex().clone(),
            zones_explorees: Vec::new(),
            indices_trouves: 6,
            apotres_vaincus: contenu
                .piste()
                .apotres()
                .map(|apotre| apotre.espece().id().clone())
                .collect(),
            humains_vaincus: Vec::new(),
            conseils_vus: Vec::new(),
            expeditions_automatiques: Vec::new(),
            attaques_a_choisir: Vec::new(),
            peche: Default::default(),
            forge: Default::default(),
            relations: Default::default(),
            machines: Default::default(),
            beherit,
            records_de_la_tour: Default::default(),
        })
        .unwrap()
    }

    #[test]
    fn l_eclipse_se_propose_avec_le_beherit_jusqu_a_la_victoire() {
        let contenu = ContenuV1::new();
        let aucun = partie(&contenu, EtatDuBeherit::Aucun);
        assert!(peut_forger_le_beherit(&aucun, &contenu));
        assert_eq!(destination_de_l_eclipse(&aucun, &contenu), None);
        let forge = partie(&contenu, EtatDuBeherit::Forge);
        assert!(!peut_forger_le_beherit(&forge, &contenu));
        assert_eq!(
            destination_de_l_eclipse(&forge, &contenu).as_deref(),
            Some("Éclipse (niveau 42) : le Démiurge ; le Béhérit réclame d'abord un sacrifice")
        );
        let ouverte = partie(&contenu, EtatDuBeherit::EclipseOuverte);
        assert_eq!(
            destination_de_l_eclipse(&ouverte, &contenu).as_deref(),
            Some("Éclipse (niveau 42) : le Démiurge")
        );
        let vaincu = partie(&contenu, EtatDuBeherit::DemiurgeVaincu);
        assert_eq!(destination_de_l_eclipse(&vaincu, &contenu), None);
    }
}
