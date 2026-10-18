//! Traque d'un Apôtre (section 9.4) : une piste d'indices, trouvés de zone en
//! zone au bout des expéditions, mène à l'antre de l'Apôtre.

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::espece::{Espece, EspeceId};
use crate::monstre::{Monstre, tirer_shiny};
use crate::niveau::Niveau;
use crate::stats::ValeursInternes;
use crate::zone::{Zone, ZoneId};

/// Un Apôtre combat 2 niveaux au-dessus du niveau maximal de sa zone (section
/// 9.4).
const NIVEAUX_AU_DESSUS_DE_LA_ZONE: u8 = 2;

/// Indice trouvé au bout d'une expédition menée jusqu'à sa dernière étape
/// dans sa zone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Indice {
    pub zone: ZoneId,
    pub texte: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurTraque {
    AucunIndice,
    /// Le niveau de l'Apôtre dépasserait le niveau maximal.
    NiveauTropHaut,
    /// L'espèce n'apprend aucune attaque, ou en apprend une en double,
    /// jusqu'au niveau de l'Apôtre.
    EspeceInvalide(EspeceId),
}

impl fmt::Display for ErreurTraque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurTraque::AucunIndice => write!(f, "une piste compte au moins un indice"),
            ErreurTraque::NiveauTropHaut => {
                write!(f, "le niveau de l'Apôtre dépasserait le niveau maximal")
            }
            ErreurTraque::EspeceInvalide(espece) => write!(
                f,
                "l'espèce {} ne peut pas être créée au niveau de l'Apôtre",
                espece.as_str()
            ),
        }
    }
}

impl std::error::Error for ErreurTraque {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Apotre {
    espece: Espece,
    zone: ZoneId,
    niveau: Niveau,
    avant_combat: String,
    epilogue: String,
}

impl Apotre {
    /// Vérifie que l'espèce peut être créée au niveau de l'Apôtre : la
    /// génération du combat ne peut ensuite plus échouer.
    pub fn new(
        espece: Espece,
        zone: &Zone,
        avant_combat: impl Into<String>,
        epilogue: impl Into<String>,
    ) -> Result<Self, ErreurTraque> {
        let (_, niveau_max) = zone.niveaux();
        let niveau = niveau_max
            .valeur()
            .checked_add(NIVEAUX_AU_DESSUS_DE_LA_ZONE)
            .and_then(|valeur| Niveau::new(valeur).ok())
            .ok_or(ErreurTraque::NiveauTropHaut)?;
        Monstre::creer(espece.clone(), niveau, ValeursInternes::default(), false)
            .map_err(|_| ErreurTraque::EspeceInvalide(espece.id().clone()))?;
        Ok(Self {
            espece,
            zone: zone.id().clone(),
            niveau,
            avant_combat: avant_combat.into(),
            epilogue: epilogue.into(),
        })
    }

    pub fn espece(&self) -> &Espece {
        &self.espece
    }

    pub fn zone(&self) -> &ZoneId {
        &self.zone
    }

    pub fn niveau(&self) -> Niveau {
        self.niveau
    }

    pub fn avant_combat(&self) -> &str {
        &self.avant_combat
    }

    pub fn epilogue(&self) -> &str {
        &self.epilogue
    }

    /// Comme un monstre sauvage, l'Apôtre a des valeurs internes tirées au
    /// hasard et 1 chance sur 512 d'être shiny (Q58). Tirages : shiny, puis
    /// valeurs internes.
    pub fn generer(&self, aleatoire: &mut dyn Aleatoire) -> Monstre {
        let shiny = tirer_shiny(aleatoire);
        let valeurs_internes = ValeursInternes::tirer(aleatoire);
        Monstre::creer(self.espece.clone(), self.niveau, valeurs_internes, shiny)
            .expect("espèce vérifiée au niveau de l'Apôtre par Apotre::new")
    }
}

/// Piste d'un Apôtre : ses indices, dans l'ordre où on les trouve, puis son
/// antre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piste {
    indices: Vec<Indice>,
    apotre: Apotre,
}

impl Piste {
    pub fn new(indices: Vec<Indice>, apotre: Apotre) -> Result<Self, ErreurTraque> {
        if indices.is_empty() {
            return Err(ErreurTraque::AucunIndice);
        }
        Ok(Self { indices, apotre })
    }

    pub fn indices(&self) -> &[Indice] {
        &self.indices
    }

    pub fn apotre(&self) -> &Apotre {
        &self.apotre
    }

    /// La zone de chaque indice déjà trouvé et celle de l'indice suivant sont
    /// ouvertes ; celle de l'Apôtre l'est une fois tous les indices trouvés
    /// (sections 9.1 et 9.4).
    pub fn zone_ouverte(&self, zone: &ZoneId, indices_trouves: usize) -> bool {
        let par_indice = self
            .indices
            .iter()
            .take(indices_trouves.saturating_add(1))
            .any(|indice| &indice.zone == zone);
        par_indice || (self.antre_localise(indices_trouves) && &self.apotre.zone == zone)
    }

    /// `None` une fois tous les indices trouvés.
    pub fn indice_suivant(&self, indices_trouves: usize) -> Option<&Indice> {
        self.indices.get(indices_trouves)
    }

    pub fn antre_localise(&self, indices_trouves: usize) -> bool {
        indices_trouves >= self.indices.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::donnees_de_test::espece;

    fn zone(id: &str, niveau_max: u8) -> Zone {
        Zone::new(
            ZoneId::new(id),
            id,
            (Niveau::new(1).unwrap(), Niveau::new(niveau_max).unwrap()),
            1,
            vec![espece("loup")],
            vec![],
        )
        .unwrap()
    }

    fn indice(zone: &str) -> Indice {
        Indice {
            zone: ZoneId::new(zone),
            texte: format!("indice de {zone}"),
        }
    }

    fn piste() -> Piste {
        let apotre = Apotre::new(espece("baron"), &zone("marais", 13), "avant", "après").unwrap();
        Piste::new(
            vec![indice("lisiere"), indice("marais"), indice("marais")],
            apotre,
        )
        .unwrap()
    }

    #[test]
    fn l_apotre_combat_deux_niveaux_au_dessus_de_sa_zone() {
        let apotre = piste().apotre().clone();
        assert_eq!(apotre.niveau(), Niveau::new(15).unwrap());
        let monstre = apotre.generer(&mut AleatoireScripte::new([1, 31, 31, 31, 31, 31, 31]));
        assert_eq!(monstre.niveau(), Niveau::new(15).unwrap());
        assert!(monstre.est_shiny());
        assert_eq!(monstre.espece().id().as_str(), "baron");
    }

    #[test]
    fn apotre_au_dela_du_niveau_maximal_refuse() {
        assert_eq!(
            Apotre::new(espece("baron"), &zone("cimes", 99), "", ""),
            Err(ErreurTraque::NiveauTropHaut)
        );
    }

    #[test]
    fn piste_sans_indice_refusee() {
        let apotre = piste().apotre().clone();
        assert_eq!(Piste::new(vec![], apotre), Err(ErreurTraque::AucunIndice));
    }

    #[test]
    fn les_zones_s_ouvrent_avec_les_indices() {
        let piste = piste();
        let lisiere = ZoneId::new("lisiere");
        let marais = ZoneId::new("marais");
        assert!(piste.zone_ouverte(&lisiere, 0));
        assert!(!piste.zone_ouverte(&marais, 0));
        assert!(piste.zone_ouverte(&marais, 1));
        assert!(piste.zone_ouverte(&lisiere, 3));
        assert!(!piste.zone_ouverte(&ZoneId::new("cimes"), 3));
    }

    #[test]
    fn l_antre_est_localise_au_dernier_indice() {
        let piste = piste();
        assert_eq!(piste.indice_suivant(0), Some(&indice("lisiere")));
        assert_eq!(piste.indice_suivant(2), Some(&indice("marais")));
        assert!(!piste.antre_localise(2));
        assert_eq!(piste.indice_suivant(3), None);
        assert!(piste.antre_localise(3));
    }
}
