//! Pêche au camp (section 13.8) : Ansgar, le pêcheur sauvé dans le Marais,
//! tend une ligne au camp ; le joueur la relève quand elle est prête, et
//! remonte un monstre aquatique, qu'il garde ou relâche, ou un objet.

use std::fmt;
use std::time::Duration;

use crate::aleatoire::Aleatoire;
use crate::espece::{Espece, EspeceId};
use crate::horodatage::Horodatage;
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::pierre::PierreDEvolution;
use crate::stats::ValeursInternes;
use crate::zone::{Zone, creer_monstre_sauvage, tirer_entre};

/// Bornes du tirage de la prise, sur 100 (docs/contenu-v1.md, section 15) :
/// monstre commun 55 %, monstre rare 5 %, potion 20 %, bourse noyée 10 %,
/// pierre d'évolution 5 %, capsule d'habileté 5 %.
const JUSQU_AU_MONSTRE_COMMUN: u32 = 55;
const JUSQU_AU_MONSTRE_RARE: u32 = 60;
const JUSQU_A_LA_POTION: u32 = 80;
const JUSQU_A_LA_BOURSE: u32 = 90;
const JUSQU_A_LA_PIERRE: u32 = 95;

/// Ce que la pêche peut remonter (docs/contenu-v1.md, section 15).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParametresPeche {
    duree: Duration,
    especes: Vec<Espece>,
    rare: Espece,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurParametresPeche {
    /// Aucune espèce commune à pêcher.
    SansEspece,
    /// L'espèce n'apprend aucune attaque, ou en apprend une en double,
    /// jusqu'à ce niveau : elle ne pourrait pas être pêchée dans toutes les
    /// zones.
    EspeceInvalide { espece: EspeceId, niveau: Niveau },
}

impl fmt::Display for ErreurParametresPeche {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurParametresPeche::SansEspece => write!(f, "aucune espèce à pêcher"),
            ErreurParametresPeche::EspeceInvalide { espece, niveau } => write!(
                f,
                "l'espèce {} ne peut pas être pêchée au niveau {}",
                espece.as_str(),
                niveau.valeur()
            ),
        }
    }
}

impl std::error::Error for ErreurParametresPeche {}

impl ParametresPeche {
    /// Vérifie que chaque espèce peut être créée à chaque niveau de ces
    /// zones : la prise d'un monstre ne peut ensuite plus échouer.
    pub fn new(
        duree: Duration,
        especes: Vec<Espece>,
        rare: Espece,
        zones: &[Zone],
    ) -> Result<Self, ErreurParametresPeche> {
        if especes.is_empty() {
            return Err(ErreurParametresPeche::SansEspece);
        }
        for espece in especes.iter().chain([&rare]) {
            for zone in zones {
                let (min, max) = zone.niveaux();
                for valeur in min.valeur()..=max.valeur() {
                    let niveau = Niveau::new(valeur).expect("niveau compris dans la zone");
                    Monstre::creer(espece.clone(), niveau, ValeursInternes::default(), false)
                        .map_err(|_| ErreurParametresPeche::EspeceInvalide {
                            espece: espece.id().clone(),
                            niveau,
                        })?;
                }
            }
        }
        Ok(Self {
            duree,
            especes,
            rare,
        })
    }

    /// Le temps qu'il faut à la ligne pour être prête.
    pub fn duree(&self) -> Duration {
        self.duree
    }

    pub fn especes(&self) -> &[Espece] {
        &self.especes
    }

    pub fn rare(&self) -> &Espece {
        &self.rare
    }

    /// Un monstre pêché a un niveau de `zone`, la plus avancée que le joueur
    /// a ouverte, et une bourse noyée l'argent d'un de ses coffres.
    /// Tirages : la nature de la prise, puis, pour un monstre commun, son
    /// espèce ; pour un monstre, ceux d'un monstre sauvage de la zone
    /// (niveau, shiny, valeurs internes s'il n'est pas shiny, habileté) ;
    /// pour une bourse, son argent ; pour une pierre, laquelle.
    pub(crate) fn tirer_prise(&self, zone: &Zone, aleatoire: &mut dyn Aleatoire) -> Prise {
        match aleatoire.tirer(1..=100) {
            tirage if tirage <= JUSQU_AU_MONSTRE_COMMUN => {
                let position = tirer_entre(0, (self.especes.len() - 1) as u32, aleatoire);
                let espece = &self.especes[position as usize];
                Prise::Monstre(Box::new(zone.generer_monstre(espece, aleatoire)))
            }
            tirage if tirage <= JUSQU_AU_MONSTRE_RARE => {
                Prise::Monstre(Box::new(zone.generer_monstre(&self.rare, aleatoire)))
            }
            tirage if tirage <= JUSQU_A_LA_POTION => Prise::Potion,
            tirage if tirage <= JUSQU_A_LA_BOURSE => {
                let fourchette = zone.coffre().argent();
                Prise::Argent(tirer_entre(
                    *fourchette.start(),
                    *fourchette.end(),
                    aleatoire,
                ))
            }
            tirage if tirage <= JUSQU_A_LA_PIERRE => {
                let derniere = (PierreDEvolution::TOUTES.len() - 1) as u32;
                Prise::Pierre(
                    PierreDEvolution::TOUTES[tirer_entre(0, derniere, aleatoire) as usize],
                )
            }
            _ => Prise::CapsuleDHabilete,
        }
    }
}

/// Ansgar, le pêcheur, tant qu'il n'est pas au camp : il se rencontre à
/// coup sûr à son étape des expéditions dans sa zone, aux prises avec ce
/// qu'il vient de ferrer (section 9.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pecheur {
    etape: u32,
    espece: Espece,
    niveau: Niveau,
    avant_combat: String,
    apres_victoire: String,
}

impl Pecheur {
    /// Vérifie que le monstre qui l'attaque peut être créé à son niveau :
    /// sa génération ne peut ensuite plus échouer.
    pub fn new(
        etape: u32,
        espece: Espece,
        niveau: Niveau,
        avant_combat: impl Into<String>,
        apres_victoire: impl Into<String>,
    ) -> Result<Self, ErreurParametresPeche> {
        Monstre::creer(espece.clone(), niveau, ValeursInternes::default(), false).map_err(
            |_| ErreurParametresPeche::EspeceInvalide {
                espece: espece.id().clone(),
                niveau,
            },
        )?;
        Ok(Self {
            etape,
            espece,
            niveau,
            avant_combat: avant_combat.into(),
            apres_victoire: apres_victoire.into(),
        })
    }

    pub fn etape(&self) -> u32 {
        self.etape
    }

    pub fn avant_combat(&self) -> &str {
        &self.avant_combat
    }

    pub fn apres_victoire(&self) -> &str {
        &self.apres_victoire
    }

    /// Un monstre sauvage, comme le Loup qui suit le brigand. Tirages : son
    /// shiny, ses valeurs internes s'il n'est pas shiny, puis son habileté.
    pub(crate) fn generer_adversaire(&self, aleatoire: &mut dyn Aleatoire) -> Monstre {
        creer_monstre_sauvage(&self.espece, self.niveau, aleatoire)
    }
}

/// Ce que remonte la ligne. Le monstre est en boîte, pour que les objets
/// restent légers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prise {
    /// Le joueur le garde ou le relâche.
    Monstre(Box<Monstre>),
    /// Les objets vont dans le coffre, comme tout ce qui est gagné au camp
    /// (section 13.3).
    Potion,
    /// Une bourse noyée.
    Argent(u32),
    Pierre(PierreDEvolution),
    CapsuleDHabilete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurPeche {
    /// Ansgar n'est pas encore au camp.
    AucunPecheur,
    /// Une seule ligne à la fois.
    LigneDejaALEau,
    AucuneLigne,
    /// La ligne sera prête à cette heure.
    PasEncorePrete(Horodatage),
    /// L'équipe et le camp sont pleins : la ligne ne se relève pas, pour
    /// qu'un monstre pêché ait toujours sa place.
    PlaceManquante,
}

impl fmt::Display for ErreurPeche {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurPeche::AucunPecheur => write!(f, "personne ne pêche au camp"),
            ErreurPeche::LigneDejaALEau => write!(f, "la ligne est déjà à l'eau"),
            ErreurPeche::AucuneLigne => write!(f, "aucune ligne à l'eau"),
            ErreurPeche::PasEncorePrete(_) => write!(f, "la ligne n'est pas encore prête"),
            ErreurPeche::PlaceManquante => write!(
                f,
                "votre équipe et votre camp sont pleins : faites de la place avant de relever la ligne"
            ),
        }
    }
}

impl std::error::Error for ErreurPeche {}

/// Où en est la pêche dans une partie.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Peche {
    pecheur_au_camp: bool,
    /// L'heure où la ligne à l'eau sera prête.
    ligne: Option<Horodatage>,
    /// Le monstre pêché, que le joueur garde ou relâche avant toute autre
    /// action au camp.
    prise: Option<Monstre>,
}

impl Peche {
    /// Reprise d'une sauvegarde : sans Ansgar, ni ligne ni prise.
    pub fn restaurer(
        pecheur_au_camp: bool,
        ligne: Option<Horodatage>,
        prise: Option<Monstre>,
    ) -> Option<Self> {
        let sans_ansgar = ligne.is_none() && prise.is_none();
        (pecheur_au_camp || sans_ansgar).then_some(Self {
            pecheur_au_camp,
            ligne,
            prise,
        })
    }

    pub fn pecheur_au_camp(&self) -> bool {
        self.pecheur_au_camp
    }

    /// L'heure où la ligne à l'eau sera prête.
    pub fn ligne(&self) -> Option<Horodatage> {
        self.ligne
    }

    pub fn est_prete(&self, maintenant: Horodatage) -> bool {
        self.ligne.is_some_and(|fin| maintenant >= fin)
    }

    pub fn prise(&self) -> Option<&Monstre> {
        self.prise.as_ref()
    }

    pub(crate) fn accueillir_ansgar(&mut self) {
        self.pecheur_au_camp = true;
    }

    /// Rend l'heure où la ligne sera prête.
    pub(crate) fn lancer(
        &mut self,
        duree: Duration,
        maintenant: Horodatage,
    ) -> Result<Horodatage, ErreurPeche> {
        if !self.pecheur_au_camp {
            return Err(ErreurPeche::AucunPecheur);
        }
        if self.ligne.is_some() {
            return Err(ErreurPeche::LigneDejaALEau);
        }
        let fin = maintenant.plus(duree);
        self.ligne = Some(fin);
        Ok(fin)
    }

    /// Ce qui empêche de relever la ligne, sans compter la place.
    pub(crate) fn verifier_releve(&self, maintenant: Horodatage) -> Result<(), ErreurPeche> {
        match self.ligne {
            None => Err(ErreurPeche::AucuneLigne),
            Some(fin) if maintenant < fin => Err(ErreurPeche::PasEncorePrete(fin)),
            Some(_) => Ok(()),
        }
    }

    pub(crate) fn relever(&mut self) {
        self.ligne = None;
    }

    pub(crate) fn garder_en_attente(&mut self, monstre: Monstre) {
        self.prise = Some(monstre);
    }

    pub(crate) fn prendre_la_prise(&mut self) -> Option<Monstre> {
        self.prise.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::donnees_de_test::{espece, zone};

    const DEUX_HEURES: Duration = Duration::from_secs(7_200);

    fn parametres() -> ParametresPeche {
        ParametresPeche::new(
            DEUX_HEURES,
            vec![espece("noye"), espece("anguille")],
            espece("kelpie"),
            &[zone("marais")],
        )
        .unwrap()
    }

    fn a(secondes: u64) -> Horodatage {
        Horodatage::depuis_secondes(secondes)
    }

    #[test]
    fn sans_espece_refuse() {
        assert_eq!(
            ParametresPeche::new(DEUX_HEURES, Vec::new(), espece("kelpie"), &[]),
            Err(ErreurParametresPeche::SansEspece)
        );
    }

    #[test]
    fn la_prise_suit_le_tirage() {
        let parametres = parametres();
        let marais = zone("marais");
        // Monstre commun : espèce (1 : la seconde), puis shiny (2 : non) et
        // six valeurs internes ; niveau unique et pas d'habileté à tirer.
        let prise = parametres.tirer_prise(
            &marais,
            &mut AleatoireScripte::new([55, 1, 2, 0, 0, 0, 0, 0, 0]),
        );
        assert!(
            matches!(&prise, Prise::Monstre(monstre) if monstre.espece().id().as_str() == "anguille")
        );
        let prise = parametres.tirer_prise(
            &marais,
            &mut AleatoireScripte::new([56, 2, 0, 0, 0, 0, 0, 0]),
        );
        assert!(
            matches!(&prise, Prise::Monstre(monstre) if monstre.espece().id().as_str() == "kelpie")
        );
        for tirage in [61, 80] {
            assert_eq!(
                parametres.tirer_prise(&marais, &mut AleatoireScripte::new([tirage])),
                Prise::Potion
            );
        }
        // L'argent d'un coffre de la zone, toujours 10, sans tirage.
        assert_eq!(
            parametres.tirer_prise(&marais, &mut AleatoireScripte::new([90])),
            Prise::Argent(10)
        );
        assert_eq!(
            parametres.tirer_prise(&marais, &mut AleatoireScripte::new([91, 1])),
            Prise::Pierre(PierreDEvolution::Lune)
        );
        assert_eq!(
            parametres.tirer_prise(&marais, &mut AleatoireScripte::new([96])),
            Prise::CapsuleDHabilete
        );
    }

    #[test]
    fn une_ligne_a_la_fois_relevee_une_fois_prete() {
        let mut peche = Peche::default();
        assert_eq!(
            peche.lancer(DEUX_HEURES, a(0)),
            Err(ErreurPeche::AucunPecheur)
        );
        peche.accueillir_ansgar();
        assert_eq!(peche.verifier_releve(a(0)), Err(ErreurPeche::AucuneLigne));
        assert_eq!(peche.lancer(DEUX_HEURES, a(100)), Ok(a(7_300)));
        assert_eq!(
            peche.lancer(DEUX_HEURES, a(200)),
            Err(ErreurPeche::LigneDejaALEau)
        );
        assert_eq!(
            peche.verifier_releve(a(7_299)),
            Err(ErreurPeche::PasEncorePrete(a(7_300)))
        );
        assert!(!peche.est_prete(a(7_299)));
        assert!(peche.est_prete(a(7_300)));
        assert_eq!(peche.verifier_releve(a(7_300)), Ok(()));
        peche.relever();
        assert_eq!(peche.ligne(), None);
    }

    #[test]
    fn le_monstre_d_ansgar_est_sauvage_a_son_niveau() {
        let ansgar = Pecheur::new(2, espece("noye"), Niveau::new(13).unwrap(), "", "").unwrap();
        // Shiny (2 : non), puis six valeurs internes ; pas d'habileté à tirer.
        let noye = ansgar.generer_adversaire(&mut AleatoireScripte::new([2, 7, 0, 0, 0, 0, 0]));
        assert_eq!(noye.espece().id().as_str(), "noye");
        assert_eq!(noye.niveau().valeur(), 13);
        assert_eq!(noye.valeurs_internes().valeurs().pv, 7);
    }

    #[test]
    fn sans_ansgar_ni_ligne_ni_prise() {
        assert!(Peche::restaurer(false, Some(a(1)), None).is_none());
        assert!(Peche::restaurer(false, None, None).is_some());
        assert!(Peche::restaurer(true, Some(a(1)), None).is_some());
    }
}
