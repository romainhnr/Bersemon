//! Améliorations du camp (section 13.4) : Gunther, le forgeron, agrandit
//! l'enclos, améliore les filets et fait une place de plus dans l'équipe,
//! contre des ressources, un niveau à la fois, chacun avec sa durée de
//! construction.
//!
//! Les effets de chaque niveau sont des règles, fixées ici ; les coûts et
//! les durées sont du contenu (`PlanDAmelioration`).

use std::fmt;
use std::time::Duration;

use crate::collection::Tailles;
use crate::horodatage::Horodatage;
use crate::ressource::Ressource;

/// Places que chaque niveau d'enclos ajoute au camp (section 13.4).
const PLACES_PAR_NIVEAU_D_ENCLOS: usize = 10;
/// Chaque niveau de filets ajoute 10 % à la chance de capture, gardée en
/// fraction pour rester en calcul entier.
const DIXIEMES_PAR_NIVEAU_DE_FILETS: u64 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Amelioration {
    Enclos,
    Filets,
    Equipe,
}

impl Amelioration {
    /// Dans l'ordre de docs/contenu-v1.md, section 9, qui est aussi celui
    /// des affichages.
    pub const TOUTES: [Amelioration; 3] = [
        Amelioration::Enclos,
        Amelioration::Filets,
        Amelioration::Equipe,
    ];

    pub fn nom(self) -> &'static str {
        match self {
            Amelioration::Enclos => "Enclos",
            Amelioration::Filets => "Filets",
            Amelioration::Equipe => "Équipe",
        }
    }

    pub fn niveau_max(self) -> u8 {
        match self {
            Amelioration::Enclos | Amelioration::Filets => 3,
            Amelioration::Equipe => 1,
        }
    }

    /// Ce que donne ce niveau, pour l'annoncer au joueur.
    pub fn effet(self, niveau: u8) -> EffetDAmelioration {
        match self {
            Amelioration::Enclos => EffetDAmelioration::PlacesAuCamp(places_au_camp(niveau)),
            Amelioration::Filets => EffetDAmelioration::ChanceDeCaptureEnPlus(
                u32::from(niveau) * DIXIEMES_PAR_NIVEAU_DE_FILETS as u32 * 10,
            ),
            Amelioration::Equipe => {
                EffetDAmelioration::PlacesDansLEquipe(places_dans_l_equipe(niveau))
            }
        }
    }
}

fn places_dans_l_equipe(niveau_d_equipe: u8) -> usize {
    Tailles::DE_DEPART.equipe + usize::from(niveau_d_equipe)
}

fn places_au_camp(niveau_d_enclos: u8) -> usize {
    Tailles::DE_DEPART.camp + usize::from(niveau_d_enclos) * PLACES_PAR_NIVEAU_D_ENCLOS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffetDAmelioration {
    PlacesAuCamp(usize),
    PlacesDansLEquipe(usize),
    /// En pourcentage de la chance de capture.
    ChanceDeCaptureEnPlus(u32),
}

/// Les niveaux atteints, 0 au départ.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Ameliorations {
    enclos: u8,
    filets: u8,
    equipe: u8,
}

impl Ameliorations {
    /// Reprise d'une sauvegarde : `None` pour un niveau au-delà du maximum.
    pub fn new(enclos: u8, filets: u8, equipe: u8) -> Option<Self> {
        let ameliorations = Self {
            enclos,
            filets,
            equipe,
        };
        Amelioration::TOUTES
            .iter()
            .all(|amelioration| ameliorations.niveau(*amelioration) <= amelioration.niveau_max())
            .then_some(ameliorations)
    }

    pub fn niveau(&self, amelioration: Amelioration) -> u8 {
        match amelioration {
            Amelioration::Enclos => self.enclos,
            Amelioration::Filets => self.filets,
            Amelioration::Equipe => self.equipe,
        }
    }

    fn monter(&mut self, amelioration: Amelioration) {
        let niveau = match amelioration {
            Amelioration::Enclos => &mut self.enclos,
            Amelioration::Filets => &mut self.filets,
            Amelioration::Equipe => &mut self.equipe,
        };
        *niveau = (*niveau + 1).min(amelioration.niveau_max());
    }

    /// 30 places au camp, et 10 de plus par niveau d'enclos ; 3 dans
    /// l'équipe, et une de plus avec l'amélioration de l'équipe (section
    /// 13.2).
    pub fn tailles(&self) -> Tailles {
        Tailles {
            equipe: places_dans_l_equipe(self.equipe),
            camp: places_au_camp(self.enclos),
        }
    }

    /// Multiplie la chance de capture : x1, puis x1,1, x1,2 et x1,3 (section
    /// 11.2).
    pub fn facteur_des_filets(&self) -> (u64, u64) {
        (
            10 + u64::from(self.filets) * DIXIEMES_PAR_NIVEAU_DE_FILETS,
            10,
        )
    }
}

/// La forge de Gunther : présente une fois Gunther libéré, avec les niveaux
/// atteints et le niveau en construction (section 13.4).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Forge {
    au_camp: bool,
    ameliorations: Ameliorations,
    chantier: Option<Chantier>,
}

impl Forge {
    /// Reprise d'une sauvegarde : sans Gunther, ni amélioration ni chantier.
    pub fn restaurer(
        au_camp: bool,
        ameliorations: Ameliorations,
        chantier: Option<Chantier>,
    ) -> Option<Self> {
        let sans_gunther = ameliorations == Ameliorations::default() && chantier.is_none();
        (au_camp || sans_gunther).then_some(Self {
            au_camp,
            ameliorations,
            chantier,
        })
    }

    pub fn est_au_camp(&self) -> bool {
        self.au_camp
    }

    pub fn ameliorations(&self) -> &Ameliorations {
        &self.ameliorations
    }

    pub fn chantier(&self) -> Option<&Chantier> {
        self.chantier.as_ref()
    }

    pub(crate) fn accueillir_gunther(&mut self) {
        self.au_camp = true;
    }

    /// Ce qui empêche de lancer ce plan, sans compter les ressources.
    pub(crate) fn verifier_lancement(
        &self,
        plan: &PlanDAmelioration,
    ) -> Result<(), ErreurAmelioration> {
        if !self.au_camp {
            return Err(ErreurAmelioration::AucunArtisan);
        }
        if self.chantier.is_some() {
            return Err(ErreurAmelioration::ChantierEnCours);
        }
        let actuel = self.ameliorations.niveau(plan.amelioration());
        if actuel >= plan.amelioration().niveau_max() || plan.niveau() != actuel + 1 {
            return Err(ErreurAmelioration::NiveauMaximal);
        }
        Ok(())
    }

    pub(crate) fn lancer(&mut self, plan: &PlanDAmelioration, maintenant: Horodatage) -> Chantier {
        let chantier = Chantier {
            amelioration: plan.amelioration(),
            niveau: plan.niveau(),
            fin: maintenant.plus(plan.duree()),
        };
        self.chantier = Some(chantier);
        chantier
    }

    /// Le niveau construit compte dès que le joueur constate le travail
    /// fini. Rend le chantier achevé.
    pub(crate) fn achever(
        &mut self,
        maintenant: Horodatage,
    ) -> Result<Chantier, ErreurAmelioration> {
        let chantier = self.chantier.ok_or(ErreurAmelioration::AucunChantier)?;
        if !chantier.est_fini(maintenant) {
            return Err(ErreurAmelioration::PasEncoreFini(chantier.fin));
        }
        chantier.appliquer(&mut self.ameliorations);
        self.chantier = None;
        Ok(chantier)
    }
}

/// Ce que coûte un niveau d'amélioration et le temps qu'il demande à
/// Gunther (docs/contenu-v1.md, section 9).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanDAmelioration {
    amelioration: Amelioration,
    niveau: u8,
    cout: Vec<(Ressource, u32)>,
    duree: Duration,
}

impl PlanDAmelioration {
    pub fn new(
        amelioration: Amelioration,
        niveau: u8,
        cout: Vec<(Ressource, u32)>,
        duree: Duration,
    ) -> Result<Self, ErreurPlan> {
        if niveau == 0 || niveau > amelioration.niveau_max() {
            return Err(ErreurPlan::NiveauHorsLimites);
        }
        if cout.is_empty() || cout.iter().any(|(_, nombre)| *nombre == 0) {
            return Err(ErreurPlan::CoutVide);
        }
        if duree.as_secs() == 0 {
            return Err(ErreurPlan::DureeNulle);
        }
        Ok(Self {
            amelioration,
            niveau,
            cout,
            duree,
        })
    }

    pub fn amelioration(&self) -> Amelioration {
        self.amelioration
    }

    /// Le niveau qu'il fait atteindre.
    pub fn niveau(&self) -> u8 {
        self.niveau
    }

    pub fn cout(&self) -> &[(Ressource, u32)] {
        &self.cout
    }

    pub fn duree(&self) -> Duration {
        self.duree
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurPlan {
    NiveauHorsLimites,
    CoutVide,
    DureeNulle,
}

impl fmt::Display for ErreurPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurPlan::NiveauHorsLimites => write!(f, "niveau d'amélioration hors limites"),
            ErreurPlan::CoutVide => write!(f, "une amélioration coûte des ressources"),
            ErreurPlan::DureeNulle => write!(f, "une amélioration demande du temps"),
        }
    }
}

impl std::error::Error for ErreurPlan {}

/// Le niveau que Gunther construit, et l'heure où il aura fini.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chantier {
    pub amelioration: Amelioration,
    pub niveau: u8,
    pub fin: Horodatage,
}

impl Chantier {
    pub fn est_fini(&self, maintenant: Horodatage) -> bool {
        maintenant >= self.fin
    }

    /// Le niveau construit compte une fois le travail constaté (section
    /// 13.4).
    pub(crate) fn appliquer(&self, ameliorations: &mut Ameliorations) {
        if ameliorations.niveau(self.amelioration) < self.niveau {
            ameliorations.monter(self.amelioration);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurAmelioration {
    /// Gunther n'est pas encore libéré (section 13.4).
    AucunArtisan,
    /// Il ne construit qu'un niveau à la fois.
    ChantierEnCours,
    NiveauMaximal,
    /// Il manque ces ressources, avec le nombre qui manque.
    RessourcesManquantes(Vec<(Ressource, u32)>),
    AucunChantier,
    PasEncoreFini(Horodatage),
}

impl fmt::Display for ErreurAmelioration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurAmelioration::AucunArtisan => write!(f, "aucun artisan au camp"),
            ErreurAmelioration::ChantierEnCours => write!(f, "une construction est en cours"),
            ErreurAmelioration::NiveauMaximal => write!(f, "niveau maximal déjà atteint"),
            ErreurAmelioration::RessourcesManquantes(manquantes) => {
                let liste: Vec<String> = manquantes
                    .iter()
                    .map(|(ressource, nombre)| format!("{nombre} {}", ressource.nom()))
                    .collect();
                write!(f, "il manque {}", liste.join(", "))
            }
            ErreurAmelioration::AucunChantier => write!(f, "aucune construction en cours"),
            ErreurAmelioration::PasEncoreFini(_) => write!(f, "la construction n'est pas finie"),
        }
    }
}

impl std::error::Error for ErreurAmelioration {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ressource::RessourceId;

    fn croc() -> Ressource {
        Ressource::new(RessourceId::new("croc"), "Croc", 5)
    }

    #[test]
    fn chaque_niveau_d_enclos_ajoute_10_places_au_camp() {
        let mut ameliorations = Ameliorations::default();
        assert_eq!(ameliorations.tailles(), Tailles::DE_DEPART);
        for (niveau, places) in [(1, 40), (2, 50), (3, 60)] {
            Chantier {
                amelioration: Amelioration::Enclos,
                niveau,
                fin: Horodatage::depuis_secondes(0),
            }
            .appliquer(&mut ameliorations);
            assert_eq!(ameliorations.tailles().camp, places);
            assert_eq!(ameliorations.tailles().equipe, Tailles::DE_DEPART.equipe);
        }
    }

    #[test]
    fn l_equipe_passe_a_4_monstres() {
        let mut ameliorations = Ameliorations::default();
        Chantier {
            amelioration: Amelioration::Equipe,
            niveau: 1,
            fin: Horodatage::depuis_secondes(0),
        }
        .appliquer(&mut ameliorations);
        assert_eq!(ameliorations.tailles().equipe, 4);
        assert_eq!(ameliorations.tailles().camp, Tailles::DE_DEPART.camp);
        assert_eq!(Ameliorations::new(0, 0, 2), None);
    }

    #[test]
    fn les_filets_multiplient_la_chance_de_capture() {
        assert_eq!(Ameliorations::default().facteur_des_filets(), (10, 10));
        assert_eq!(
            Ameliorations::new(0, 3, 0).unwrap().facteur_des_filets(),
            (13, 10)
        );
    }

    #[test]
    fn un_niveau_au_dela_du_maximum_est_refuse() {
        assert_eq!(Ameliorations::new(4, 0, 0), None);
        assert!(Ameliorations::new(3, 3, 1).is_some());
        let plan = |niveau| {
            PlanDAmelioration::new(
                Amelioration::Filets,
                niveau,
                vec![(croc(), 1)],
                Duration::from_secs(60),
            )
        };
        assert_eq!(plan(0), Err(ErreurPlan::NiveauHorsLimites));
        assert_eq!(plan(4), Err(ErreurPlan::NiveauHorsLimites));
        assert!(plan(3).is_ok());
    }

    #[test]
    fn un_plan_coute_des_ressources_et_du_temps() {
        let sans_cout =
            PlanDAmelioration::new(Amelioration::Enclos, 1, vec![], Duration::from_secs(60));
        assert_eq!(sans_cout, Err(ErreurPlan::CoutVide));
        let sans_duree =
            PlanDAmelioration::new(Amelioration::Enclos, 1, vec![(croc(), 1)], Duration::ZERO);
        assert_eq!(sans_duree, Err(ErreurPlan::DureeNulle));
    }

    #[test]
    fn un_chantier_ne_fait_monter_qu_une_fois() {
        let mut ameliorations = Ameliorations::default();
        let chantier = Chantier {
            amelioration: Amelioration::Filets,
            niveau: 1,
            fin: Horodatage::depuis_secondes(100),
        };
        assert!(!chantier.est_fini(Horodatage::depuis_secondes(99)));
        assert!(chantier.est_fini(Horodatage::depuis_secondes(100)));
        chantier.appliquer(&mut ameliorations);
        chantier.appliquer(&mut ameliorations);
        assert_eq!(ameliorations.niveau(Amelioration::Filets), 1);
    }
}
