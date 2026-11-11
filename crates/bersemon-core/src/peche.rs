//! Pêche au camp (section 13.8) : Ansgar, le pêcheur sauvé dans le Marais,
//! tend une ligne au camp ; le joueur la relève quand elle est prête, et
//! remonte un monstre aquatique, qu'il garde ou relâche, ou un objet.

use std::fmt;
use std::time::Duration;

use crate::aleatoire::Aleatoire;
use crate::booster::Booster;
use crate::espece::{Espece, EspeceId};
use crate::horodatage::Horodatage;
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::parfum::Parfum;
use crate::pierre::PierreDEvolution;
use crate::ressource::Ressource;
use crate::stats::ValeursInternes;
use crate::zone::{Zone, creer_monstre_sauvage, tirer_entre};

/// Les cannes qu'Ansgar aide à fabriquer (section 13.8) : plus la canne est
/// bonne, plus la ligne reste longtemps à l'eau, et plus ce qu'elle remonte
/// est rare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CanneAPeche {
    Bois,
    Fer,
    Argent,
}

/// Une canne, l'argent et les ressources qu'elle coûte.
pub type PrixDeLaCanne = (CanneAPeche, u32, Vec<(Ressource, u32)>);

/// Bornes du tirage de la prise, sur 100, dans l'ordre : monstre commun,
/// monstre rare, potion, bourse noyée, pierre d'évolution, capsule
/// d'habileté, booster ; le reste, un Philtre chromatique.
type BornesDeLaPrise = [u32; 7];

impl CanneAPeche {
    pub const TOUTES: [CanneAPeche; 3] = [CanneAPeche::Bois, CanneAPeche::Fer, CanneAPeche::Argent];

    pub fn nom(self) -> &'static str {
        match self {
            CanneAPeche::Bois => "Canne en bois",
            CanneAPeche::Fer => "Canne en fer",
            CanneAPeche::Argent => "Canne en argent",
        }
    }

    /// Le temps de la ligne à l'eau, choix de l'utilisateur (section 13.8).
    pub fn duree(self) -> Duration {
        let minutes = match self {
            CanneAPeche::Bois => 30,
            CanneAPeche::Fer => 60,
            CanneAPeche::Argent => 120,
        };
        Duration::from_secs(minutes * 60)
    }

    /// La canne d'avant, qu'il faut posséder pour fabriquer celle-ci.
    pub fn precedente(self) -> Option<CanneAPeche> {
        match self {
            CanneAPeche::Bois => None,
            CanneAPeche::Fer => Some(CanneAPeche::Bois),
            CanneAPeche::Argent => Some(CanneAPeche::Fer),
        }
    }

    /// Prises, sur 100 (docs/contenu-v1.md, section 15). La canne en argent
    /// garde la table d'avant les cannes : monstre commun 55 %, rare 5 %,
    /// potion 14 %, bourse 10 %, pierre 5 %, capsule d'habileté 5 %, booster
    /// 5 %, Philtre chromatique 1 %. Les autres, proposées par Claude (Q123),
    /// remontent moins de raretés.
    fn bornes(self) -> BornesDeLaPrise {
        match self {
            // Commun 60, rare 1, potion 25, bourse 10, pierre 1, capsule 1,
            // booster 2, Philtre 0.
            CanneAPeche::Bois => [60, 61, 86, 96, 97, 98, 100],
            // Commun 58, rare 3, potion 19, bourse 10, pierre 3, capsule 3,
            // booster 4, Philtre 0.
            CanneAPeche::Fer => [58, 61, 80, 90, 93, 96, 100],
            CanneAPeche::Argent => [55, 60, 74, 84, 89, 94, 99],
        }
    }

    /// Le nombre de matières que remonte la ligne, au moins et au plus.
    fn matieres(self) -> (u32, u32) {
        match self {
            CanneAPeche::Bois => (1, 2),
            CanneAPeche::Fer => (2, 2),
            CanneAPeche::Argent => (2, 3),
        }
    }

    /// Les chances des matières, dans leur ordre (Vers de vase, Chair de
    /// noyé, Ambre noir) : la canne en argent garde celles d'avant.
    fn chances_des_matieres(self) -> [u32; 3] {
        match self {
            CanneAPeche::Bois => [80, 20, 0],
            CanneAPeche::Fer => [70, 25, 5],
            CanneAPeche::Argent => [60, 30, 10],
        }
    }
}

/// Ce que la pêche peut remonter (docs/contenu-v1.md, section 15).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParametresPeche {
    duree: Duration,
    /// Le prix de chaque canne, en argent et en ressources (section 13.8).
    cannes: Vec<PrixDeLaCanne>,
    especes: Vec<Espece>,
    rare: Espece,
    /// Les matières que remonte chaque ligne, avec leur chance sur 100, qui
    /// font 100 ensemble (section 13.8).
    matieres: Vec<(Ressource, u32)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurParametresPeche {
    /// Aucune espèce commune à pêcher.
    SansEspece,
    /// L'espèce n'apprend aucune attaque, ou en apprend une en double,
    /// jusqu'à ce niveau : elle ne pourrait pas être pêchée dans toutes les
    /// zones.
    EspeceInvalide { espece: EspeceId, niveau: Niveau },
    /// Les chances des matières ne font pas 100.
    MatieresInvalides,
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
            ErreurParametresPeche::MatieresInvalides => {
                write!(f, "les chances des matières pêchées ne font pas 100")
            }
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
            cannes: Vec::new(),
            especes,
            rare,
            matieres: Vec::new(),
        })
    }

    /// Sans matières, la ligne ne remonte que sa prise.
    pub fn avec_matieres(
        mut self,
        matieres: Vec<(Ressource, u32)>,
    ) -> Result<Self, ErreurParametresPeche> {
        let total: u32 = matieres.iter().map(|(_, chance)| *chance).sum();
        if total != 100 || matieres.iter().any(|(_, chance)| *chance == 0) {
            return Err(ErreurParametresPeche::MatieresInvalides);
        }
        self.matieres = matieres;
        Ok(self)
    }

    pub fn matieres(&self) -> &[(Ressource, u32)] {
        &self.matieres
    }

    pub fn avec_cannes(mut self, cannes: Vec<PrixDeLaCanne>) -> Self {
        self.cannes = cannes;
        self
    }

    /// L'argent et les ressources que demande la canne ; rien si le contenu
    /// ne la chiffre pas.
    pub fn prix_de_la_canne(&self, canne: CanneAPeche) -> (u32, &[(Ressource, u32)]) {
        self.cannes
            .iter()
            .find(|(une, _, _)| *une == canne)
            .map_or((0, &[][..]), |(_, argent, ressources)| {
                (*argent, ressources.as_slice())
            })
    }

    /// Tirages, s'il y a des matières : leur nombre, puis la sorte de
    /// chacune, avec les chances de la canne. Rend chaque sorte avec son
    /// nombre, dans l'ordre des matières.
    pub(crate) fn tirer_matieres(
        &self,
        canne: CanneAPeche,
        aleatoire: &mut dyn Aleatoire,
    ) -> Vec<(Ressource, u32)> {
        if self.matieres.is_empty() {
            return Vec::new();
        }
        // Les chances de la canne, si le contenu a ses trois matières.
        let chances: Vec<u32> = if self.matieres.len() == 3 {
            canne.chances_des_matieres().to_vec()
        } else {
            self.matieres.iter().map(|(_, chance)| *chance).collect()
        };
        let (minimum, maximum) = canne.matieres();
        let mut nombres = vec![0; self.matieres.len()];
        for _ in 0..tirer_entre(minimum, maximum, aleatoire) {
            let mut tirage = aleatoire.tirer(1..=100);
            let position = chances
                .iter()
                .position(|chance| {
                    if tirage <= *chance {
                        true
                    } else {
                        tirage -= chance;
                        false
                    }
                })
                .expect("les chances font 100");
            nombres[position] += 1;
        }
        self.matieres
            .iter()
            .zip(nombres)
            .filter(|(_, nombre)| *nombre > 0)
            .map(|((ressource, _), nombre)| (ressource.clone(), nombre))
            .collect()
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
    /// pour une bourse, son argent ; pour une pierre ou un booster, lequel.
    pub(crate) fn tirer_prise(
        &self,
        zone: &Zone,
        canne: CanneAPeche,
        aleatoire: &mut dyn Aleatoire,
    ) -> Prise {
        let [
            jusqu_au_monstre_commun,
            jusqu_au_monstre_rare,
            jusqu_a_la_potion,
            jusqu_a_la_bourse,
            jusqu_a_la_pierre,
            jusqu_a_la_capsule_d_habilete,
            jusqu_au_booster,
        ] = canne.bornes();
        match aleatoire.tirer(1..=100) {
            tirage if tirage <= jusqu_au_monstre_commun => {
                let position = tirer_entre(0, (self.especes.len() - 1) as u32, aleatoire);
                let espece = &self.especes[position as usize];
                Prise::Monstre(Box::new(zone.generer_monstre(espece, aleatoire)))
            }
            tirage if tirage <= jusqu_au_monstre_rare => {
                Prise::Monstre(Box::new(zone.generer_monstre(&self.rare, aleatoire)))
            }
            tirage if tirage <= jusqu_a_la_potion => Prise::Potion,
            tirage if tirage <= jusqu_a_la_bourse => {
                let fourchette = zone.coffre().argent();
                Prise::Argent(tirer_entre(
                    *fourchette.start(),
                    *fourchette.end(),
                    aleatoire,
                ))
            }
            tirage if tirage <= jusqu_a_la_pierre => {
                let derniere = (PierreDEvolution::TROUVEES.len() - 1) as u32;
                Prise::Pierre(
                    PierreDEvolution::TROUVEES[tirer_entre(0, derniere, aleatoire) as usize],
                )
            }
            tirage if tirage <= jusqu_a_la_capsule_d_habilete => Prise::CapsuleDHabilete,
            tirage if tirage <= jusqu_au_booster => {
                let dernier = (Booster::TOUS.len() - 1) as u32;
                Prise::Booster(Booster::TOUS[tirer_entre(0, dernier, aleatoire) as usize])
            }
            _ => Prise::PhiltreChromatique,
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
    Booster(Booster),
    PhiltreChromatique,
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
    /// Il faut d'abord fabriquer cette canne (section 13.8).
    CanneManquante(CanneAPeche),
    /// Pour la fabriquer, il faut la canne d'avant.
    CannePrecedenteManquante(CanneAPeche),
    CanneDejaFabriquee,
    /// Ce qu'il manque pour la fabriquer : de l'argent, des ressources.
    PrixDeLaCanne {
        argent: u32,
        ressources: Vec<(Ressource, u32)>,
    },
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
            ErreurPeche::CanneManquante(canne) => write!(f, "aucune {}", canne.nom()),
            ErreurPeche::CannePrecedenteManquante(canne) => {
                write!(f, "il faut d'abord la {}", canne.nom())
            }
            ErreurPeche::CanneDejaFabriquee => write!(f, "canne déjà fabriquée"),
            ErreurPeche::PrixDeLaCanne { .. } => write!(f, "de quoi payer la canne manque"),
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
    /// Le parfum qu'Ansgar a préparé et que le héros porte jusqu'à sa
    /// prochaine expédition dans une zone (section 9.2).
    parfum: Option<Parfum>,
    /// Les cannes fabriquées, dans l'ordre de `CanneAPeche::TOUTES`.
    cannes: Vec<CanneAPeche>,
    /// La canne de la ligne à l'eau ; une ligne d'avant les cannes compte
    /// comme lancée avec la canne en argent, qui dure 2 h comme elle.
    canne_de_la_ligne: Option<CanneAPeche>,
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
            parfum: None,
            cannes: Vec::new(),
            canne_de_la_ligne: None,
        })
    }

    /// Reprise d'une sauvegarde : les cannes fabriquées et celle de la ligne
    /// à l'eau ; sans Ansgar, aucune.
    pub fn avec_cannes(
        mut self,
        cannes: Vec<CanneAPeche>,
        canne_de_la_ligne: Option<CanneAPeche>,
    ) -> Option<Self> {
        if !self.pecheur_au_camp && !cannes.is_empty() {
            return None;
        }
        self.cannes = CanneAPeche::TOUTES
            .into_iter()
            .filter(|canne| cannes.contains(canne))
            .collect();
        self.canne_de_la_ligne = canne_de_la_ligne.filter(|_| self.ligne.is_some());
        Some(self)
    }

    pub fn cannes(&self) -> &[CanneAPeche] {
        &self.cannes
    }

    /// La meilleure canne fabriquée, celle que prend la ligne par défaut.
    pub fn meilleure_canne(&self) -> Option<CanneAPeche> {
        self.cannes.last().copied()
    }

    /// La canne de la ligne à l'eau ; `None` sans ligne, ou pour une ligne
    /// lancée avant les cannes, relevée comme avec la canne en argent.
    pub fn canne_de_la_ligne(&self) -> Option<CanneAPeche> {
        self.canne_de_la_ligne
    }

    pub(crate) fn ajouter_canne(&mut self, canne: CanneAPeche) {
        if !self.cannes.contains(&canne) {
            self.cannes.push(canne);
            self.cannes.sort();
        }
    }

    /// Reprise d'une sauvegarde : le parfum porté, qu'Ansgar a forcément
    /// préparé.
    pub fn avec_parfum(mut self, parfum: Option<Parfum>) -> Option<Self> {
        if parfum.is_some() && !self.pecheur_au_camp {
            return None;
        }
        self.parfum = parfum;
        Some(self)
    }

    pub fn parfum(&self) -> Option<&Parfum> {
        self.parfum.as_ref()
    }

    pub(crate) fn porter(&mut self, parfum: Parfum) {
        self.parfum = Some(parfum);
    }

    /// Au départ d'une expédition dans une zone, le parfum s'y répand.
    pub(crate) fn repandre_le_parfum(&mut self) -> Option<Parfum> {
        self.parfum.take()
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
        canne: CanneAPeche,
        duree: Duration,
        maintenant: Horodatage,
    ) -> Result<Horodatage, ErreurPeche> {
        if !self.pecheur_au_camp {
            return Err(ErreurPeche::AucunPecheur);
        }
        if self.ligne.is_some() {
            return Err(ErreurPeche::LigneDejaALEau);
        }
        if !self.cannes.contains(&canne) {
            return Err(ErreurPeche::CanneManquante(canne));
        }
        let fin = maintenant.plus(duree);
        self.ligne = Some(fin);
        self.canne_de_la_ligne = Some(canne);
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
        self.canne_de_la_ligne = None;
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
            CanneAPeche::Argent,
            &mut AleatoireScripte::new([55, 1, 2, 0, 0, 0, 0, 0, 0]),
        );
        assert!(
            matches!(&prise, Prise::Monstre(monstre) if monstre.espece().id().as_str() == "anguille")
        );
        let prise = parametres.tirer_prise(
            &marais,
            CanneAPeche::Argent,
            &mut AleatoireScripte::new([56, 2, 0, 0, 0, 0, 0, 0]),
        );
        assert!(
            matches!(&prise, Prise::Monstre(monstre) if monstre.espece().id().as_str() == "kelpie")
        );
        for tirage in [61, 74] {
            assert_eq!(
                parametres.tirer_prise(
                    &marais,
                    CanneAPeche::Argent,
                    &mut AleatoireScripte::new([tirage])
                ),
                Prise::Potion
            );
        }
        // L'argent d'un coffre de la zone, toujours 10, sans tirage.
        for tirage in [75, 84] {
            assert_eq!(
                parametres.tirer_prise(
                    &marais,
                    CanneAPeche::Argent,
                    &mut AleatoireScripte::new([tirage])
                ),
                Prise::Argent(10)
            );
        }
        assert_eq!(
            parametres.tirer_prise(
                &marais,
                CanneAPeche::Argent,
                &mut AleatoireScripte::new([85, 1])
            ),
            Prise::Pierre(PierreDEvolution::Lune)
        );
        assert_eq!(
            parametres.tirer_prise(
                &marais,
                CanneAPeche::Argent,
                &mut AleatoireScripte::new([89, 0])
            ),
            Prise::Pierre(PierreDEvolution::Braise)
        );
        for tirage in [90, 94] {
            assert_eq!(
                parametres.tirer_prise(
                    &marais,
                    CanneAPeche::Argent,
                    &mut AleatoireScripte::new([tirage])
                ),
                Prise::CapsuleDHabilete
            );
        }
        assert_eq!(
            parametres.tirer_prise(
                &marais,
                CanneAPeche::Argent,
                &mut AleatoireScripte::new([95, 0])
            ),
            Prise::Booster(Booster::FioleDeSang)
        );
        assert_eq!(
            parametres.tirer_prise(
                &marais,
                CanneAPeche::Argent,
                &mut AleatoireScripte::new([99, 5])
            ),
            Prise::Booster(Booster::PoudreDeVifArgent)
        );
        assert_eq!(
            parametres.tirer_prise(
                &marais,
                CanneAPeche::Argent,
                &mut AleatoireScripte::new([100])
            ),
            Prise::PhiltreChromatique
        );
    }

    #[test]
    fn une_ligne_a_la_fois_relevee_une_fois_prete() {
        let mut peche = Peche::default();
        let argent = CanneAPeche::Argent;
        assert_eq!(
            peche.lancer(argent, DEUX_HEURES, a(0)),
            Err(ErreurPeche::AucunPecheur)
        );
        peche.accueillir_ansgar();
        assert_eq!(peche.verifier_releve(a(0)), Err(ErreurPeche::AucuneLigne));
        // Sans canne, pas de ligne (section 13.8).
        assert_eq!(
            peche.lancer(argent, DEUX_HEURES, a(100)),
            Err(ErreurPeche::CanneManquante(argent))
        );
        peche.ajouter_canne(argent);
        assert_eq!(peche.lancer(argent, DEUX_HEURES, a(100)), Ok(a(7_300)));
        assert_eq!(peche.canne_de_la_ligne(), Some(argent));
        assert_eq!(
            peche.lancer(argent, DEUX_HEURES, a(200)),
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
