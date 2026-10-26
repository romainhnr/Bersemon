//! Les lieux à part de la carte : les annexes ouvertes, puis les zones
//! légendaires une fois le Démiurge vaincu (sections 9.9 et 9.10). La
//! console les numérote après les autres destinations ; le bot les propose
//! dans un menu à part, celui des expéditions ne tenant pas plus de 25
//! options (Q118).

use bersemon_core::contenu::ContenuV1;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::partie::Partie;
use bersemon_core::zone::Zone;

use crate::meteo::meteo_de_la_zone;

pub struct LieuAPart<'c> {
    pub zone: &'c Zone,
    /// "annexe, niveaux 50 à 53", ou "zone légendaire, niveaux 98 à 98,
    /// Léviathan attend au bout".
    pub precision: String,
}

/// Les annexes dans l'ordre du contenu, puis les zones légendaires.
pub fn lieux_a_part<'c>(partie: &Partie, contenu: &'c ContenuV1) -> Vec<LieuAPart<'c>> {
    let niveaux = |zone: &Zone| {
        let (minimum, maximum) = zone.niveaux();
        format!("niveaux {} à {}", minimum.valeur(), maximum.valeur())
    };
    let annexes = partie
        .annexes_accessibles(contenu.annexes())
        .into_iter()
        .map(|annexe| LieuAPart {
            zone: annexe.zone(),
            precision: format!("annexe, {}", niveaux(annexe.zone())),
        });
    let legendaires = contenu
        .zones_legendaires()
        .iter()
        .filter(|_| partie.zones_legendaires_ouvertes())
        .map(|legendaire| {
            let espece = legendaire.legendaire().espece();
            let etat = if partie.legendaire_capture(espece.id()) {
                "capturé"
            } else {
                "attend au bout"
            };
            LieuAPart {
                zone: legendaire.zone(),
                precision: format!(
                    "zone légendaire, {}, {} {etat}",
                    niveaux(legendaire.zone()),
                    espece.nom()
                ),
            }
        });
    annexes.chain(legendaires).collect()
}

/// Sur la carte de la console : "Épave du Léviathan (annexe, niveaux 50 à
/// 53) : pluie, encore 42 min".
pub fn ligne_du_lieu(lieu: &LieuAPart, maintenant: Horodatage) -> String {
    let meteo = meteo_de_la_zone(lieu.zone, maintenant)
        .map(|meteo| format!(" : {meteo}"))
        .unwrap_or_default();
    format!("{} ({}){meteo}", lieu.zone.nom(), lieu.precision)
}

/// Sous le nom du lieu, dans le menu du bot : "Annexe, niveaux 50 à 53 ;
/// pluie, encore 42 min".
pub fn description_du_lieu(lieu: &LieuAPart, maintenant: Horodatage) -> String {
    let meteo = meteo_de_la_zone(lieu.zone, maintenant)
        .map(|meteo| format!(" ; {meteo}"))
        .unwrap_or_default();
    let mut precision = lieu.precision.chars();
    let precision = precision
        .next()
        .map(|premiere| premiere.to_uppercase().chain(precision).collect::<String>())
        .unwrap_or_default();
    format!("{precision}{meteo}")
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
    use bersemon_core::partie::{Decouvertes, EtatPartie};
    use bersemon_core::stats::ValeursInternes;
    use bersemon_core::zone::ZoneId;

    fn partie(contenu: &ContenuV1, annexes_ouvertes: Vec<ZoneId>) -> Partie {
        let depart = Monstre::creer(
            contenu.espece(&EspeceId::new("crapaud")).unwrap(),
            Niveau::new(5).unwrap(),
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
            marchand_au_camp: false,
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
            peche: Default::default(),
            forge: Default::default(),
            relations: Default::default(),
            machines: Default::default(),
            beherit: Default::default(),
            records_de_la_tour: Default::default(),
            rite: None,
            paliers_du_codex: Vec::new(),
            arene: Default::default(),
            decouvertes: Decouvertes {
                annexes_ouvertes,
                legendaires_captures: Vec::new(),
            },
        })
        .unwrap()
    }

    #[test]
    fn seules_les_annexes_ouvertes_se_proposent() {
        let contenu = ContenuV1::new();
        assert!(lieux_a_part(&partie(&contenu, Vec::new()), &contenu).is_empty());
        let filon = ZoneId::new("filon-ecarlate");
        let partie = partie(&contenu, vec![filon.clone()]);
        let lieux = lieux_a_part(&partie, &contenu);
        assert_eq!(lieux.len(), 1);
        assert_eq!(lieux[0].zone.id(), &filon);
        // Un instant sans météo : chaque zone n'en a qu'une fois sur trois.
        let calme = (0..)
            .map(|creneau| Horodatage::depuis_secondes(creneau * 30 * 60))
            .find(|maintenant| meteo_de_la_zone(lieux[0].zone, *maintenant).is_none())
            .unwrap();
        assert_eq!(
            ligne_du_lieu(&lieux[0], calme),
            "Filon écarlate (annexe, niveaux 56 à 59)"
        );
        assert_eq!(
            description_du_lieu(&lieux[0], calme),
            "Annexe, niveaux 56 à 59"
        );
    }
}
