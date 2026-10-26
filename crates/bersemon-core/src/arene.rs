//! Fosse aux damnés (section 9.8) : une arène hors des zones, où le héros
//! affronte seul, à mort, un humain de l'IA par palier. Chaque palier vaincu
//! ouvre le suivant et donne sa récompense, la première fois.

use crate::aleatoire::Aleatoire;
use crate::equipement::ObjetDEquipement;
use crate::espece::Espece;
use crate::heros::{Classe, creer_combattant_au_niveau};
use crate::humain::{HumainId, IaHumain};
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::stats::ValeursInternes;

pub const PALIERS_DE_LA_FOSSE: u8 = 10;

/// La cote de chaque joueur au classement de l'arène, avant son premier
/// combat contre un autre joueur (section 15.6).
pub const COTE_DE_DEPART: u32 = 1000;

/// Ce que le joueur a fait dans l'arène : le dernier palier vaincu de la
/// Fosse aux damnés (section 9.8), puis sa cote et le nombre de ses combats
/// contre d'autres joueurs (section 15.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EtatDeLArene {
    pub paliers_vaincus: u8,
    pub cote: u32,
    pub combats_entre_joueurs: u32,
}

impl Default for EtatDeLArene {
    fn default() -> Self {
        Self {
            paliers_vaincus: 0,
            cote: COTE_DE_DEPART,
            combats_entre_joueurs: 0,
        }
    }
}

/// Le poids d'un combat dans la cote, comme aux échecs (section 15.6).
const POIDS_D_UN_COMBAT: f64 = 32.0;

/// Ce que le vainqueur gagne et que le perdant perd, selon leurs cotes
/// avant le combat : 32 x (1 - E), arrondi à l'entier le plus proche, où E
/// est la chance attendue du vainqueur (cote Elo, section 15.6).
pub fn gain_de_cote(cote_du_vainqueur: u32, cote_du_perdant: u32) -> u32 {
    let ecart = (f64::from(cote_du_perdant) - f64::from(cote_du_vainqueur)) / 400.0;
    let attendue = 1.0 / (1.0 + 10f64.powf(ecart));
    (POIDS_D_UN_COMBAT * (1.0 - attendue)).round() as u32
}

impl EtatDeLArene {
    /// Après un combat contre un autre joueur : la cote monte ou baisse du
    /// gain, sans descendre sous 0 (hypothèse retenue, Q112).
    pub(crate) fn noter_combat(&mut self, victoire: bool, gain: u32) {
        self.cote = if victoire {
            self.cote.saturating_add(gain)
        } else {
            self.cote.saturating_sub(gain)
        };
        self.combats_entre_joueurs = self.combats_entre_joueurs.saturating_add(1);
    }
}

/// Pourquoi la Fosse refuse le joueur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErreurArene {
    /// Le brigand de la Lisière n'est pas encore vaincu.
    FosseFermee,
    /// Les 10 paliers sont vaincus.
    FosseConquise,
    /// Le palier donne un monstre, et ni l'équipe ni le camp n'ont de place
    /// (hypothèse retenue, Q112).
    PlusDePlace,
}

impl std::fmt::Display for ErreurArene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErreurArene::FosseFermee => write!(f, "la Fosse aux damnés n'est pas ouverte"),
            ErreurArene::FosseConquise => write!(f, "tous les paliers sont vaincus"),
            ErreurArene::PlusDePlace => write!(f, "aucune place pour le monstre offert"),
        }
    }
}

impl std::error::Error for ErreurArene {}

/// Ce que donne un palier à sa première victoire : de l'argent, parfois un
/// monstre au niveau du palier, et, au dernier, l'arme du palier 4 de la
/// classe du héros (section 9.8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecompenseDeLArene {
    pub argent: u32,
    pub monstre: Option<(Espece, Niveau)>,
    pub arme_du_champion: bool,
}

/// L'humain d'un palier : il combat seul, avec sa classe, son arme et des
/// valeurs internes fixes, comme le héros (section 4.4). Son niveau est fixé
/// à part de son équipement, pour que les paliers se suivent de 2 ou 3
/// niveaux, et non de 5 (section 9.8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdversaireDeLArene {
    id: HumainId,
    nom: String,
    classe: Classe,
    arme: Option<ObjetDEquipement>,
    niveau: Niveau,
    ia: IaHumain,
    presentation: String,
    recompense: RecompenseDeLArene,
    /// Pour un adversaire miroir, l'arme de chaque classe : il prend la
    /// classe du héros qui l'affronte, pour un combat à chances égales que
    /// les affinités de types ne décident pas d'avance (section 9.8).
    armes_du_miroir: Vec<ObjetDEquipement>,
}

impl AdversaireDeLArene {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: HumainId,
        nom: impl Into<String>,
        classe: Classe,
        arme: Option<ObjetDEquipement>,
        niveau: Niveau,
        ia: IaHumain,
        presentation: impl Into<String>,
        recompense: RecompenseDeLArene,
    ) -> Self {
        Self {
            id,
            nom: nom.into(),
            classe,
            arme,
            niveau,
            ia,
            presentation: presentation.into(),
            recompense,
            armes_du_miroir: Vec::new(),
        }
    }

    pub fn en_miroir(mut self, armes: Vec<ObjetDEquipement>) -> Self {
        self.armes_du_miroir = armes;
        self
    }

    pub fn est_un_miroir(&self) -> bool {
        !self.armes_du_miroir.is_empty()
    }

    /// Tel que l'affronte un héros de cette classe : un adversaire miroir
    /// prend sa classe et l'arme de sa classe ; les autres ne changent pas.
    pub fn face_a(&self, classe: Classe) -> AdversaireDeLArene {
        let mut adversaire = self.clone();
        if let Some(arme) = self
            .armes_du_miroir
            .iter()
            .find(|arme| arme.classe() == Some(classe))
        {
            adversaire.classe = classe;
            adversaire.arme = Some(arme.clone());
        }
        adversaire
    }

    pub fn id(&self) -> &HumainId {
        &self.id
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }

    pub fn classe(&self) -> Classe {
        self.classe
    }

    pub fn niveau(&self) -> Niveau {
        self.niveau
    }

    pub fn ia(&self) -> IaHumain {
        self.ia
    }

    pub fn presentation(&self) -> &str {
        &self.presentation
    }

    pub fn recompense(&self) -> &RecompenseDeLArene {
        &self.recompense
    }

    /// Lui-même, prêt au combat, sans tirage.
    pub fn generer_heros(&self) -> Monstre {
        creer_combattant_au_niveau(&self.nom, self.classe, self.arme.as_ref(), self.niveau)
    }
}

/// Le monstre que donne un palier : valeurs internes et habileté tirées,
/// jamais shiny (hypothèse retenue, Q112). Tirages : les six valeurs
/// internes, puis l'habileté.
pub fn generer_le_monstre_offert(
    espece: &Espece,
    niveau: Niveau,
    aleatoire: &mut dyn Aleatoire,
) -> Monstre {
    let valeurs_internes = ValeursInternes::tirer(aleatoire);
    Monstre::creer(espece.clone(), niveau, valeurs_internes, false)
        .expect("monstre offert vérifié par le contenu")
        .tirer_son_habilete(aleatoire)
}

/// Les adversaires des paliers 1 à 10, dans l'ordre, et l'humain dont la
/// défaite ouvre la Fosse (section 9.8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FosseAuxDamnes {
    adversaires: Vec<AdversaireDeLArene>,
    ouverte_par: HumainId,
}

impl FosseAuxDamnes {
    /// Panique s'il n'y a pas un adversaire par palier : c'est une erreur du
    /// contenu, que ses tests détectent.
    pub fn new(adversaires: Vec<AdversaireDeLArene>, ouverte_par: HumainId) -> Self {
        assert_eq!(
            adversaires.len(),
            usize::from(PALIERS_DE_LA_FOSSE),
            "un adversaire par palier"
        );
        Self {
            adversaires,
            ouverte_par,
        }
    }

    pub fn ouverte_par(&self) -> &HumainId {
        &self.ouverte_par
    }

    pub fn adversaires(&self) -> &[AdversaireDeLArene] {
        &self.adversaires
    }

    /// Celui du palier `palier`, de 1 à 10.
    pub fn adversaire(&self, palier: u8) -> Option<&AdversaireDeLArene> {
        let position = usize::from(palier).checked_sub(1)?;
        self.adversaires.get(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::donnees_de_test::espece;

    fn adversaire(palier: u8) -> AdversaireDeLArene {
        AdversaireDeLArene::new(
            HumainId::new(format!("adversaire-{palier}")),
            format!("Adversaire {palier}"),
            Classe::Guerrier,
            None,
            Niveau::new(5).unwrap(),
            IaHumain::Simple,
            "",
            RecompenseDeLArene {
                argent: 10,
                monstre: None,
                arme_du_champion: false,
            },
        )
    }

    #[test]
    fn un_adversaire_par_palier_de_1_a_10() {
        let fosse = FosseAuxDamnes::new(
            (1..=PALIERS_DE_LA_FOSSE).map(adversaire).collect(),
            HumainId::new("brigand"),
        );
        assert_eq!(fosse.adversaire(0), None);
        assert_eq!(fosse.adversaire(1).unwrap().nom(), "Adversaire 1");
        assert_eq!(fosse.adversaire(10).unwrap().nom(), "Adversaire 10");
        assert_eq!(fosse.adversaire(11), None);
        let heros = fosse.adversaire(3).unwrap().generer_heros();
        assert_eq!(heros.nom(), "Adversaire 3");
        assert_eq!(heros.niveau().valeur(), 5);
    }

    /// Section 15.6 : battre un joueur mieux classé rapporte plus.
    #[test]
    fn la_cote_suit_la_formule_d_elo() {
        assert_eq!(gain_de_cote(1000, 1000), 16);
        assert_eq!(gain_de_cote(1200, 1000), 8);
        assert_eq!(gain_de_cote(1000, 1200), 24);
        assert_eq!(gain_de_cote(1800, 1000), 0);
        let mut etat = EtatDeLArene::default();
        etat.noter_combat(true, 16);
        assert_eq!((etat.cote, etat.combats_entre_joueurs), (1016, 1));
        etat.cote = 10;
        etat.noter_combat(false, 24);
        assert_eq!((etat.cote, etat.combats_entre_joueurs), (0, 2));
    }

    #[test]
    fn le_monstre_offert_n_est_jamais_shiny() {
        let tirages = [31, 31, 31, 31, 31, 31];
        let monstre = generer_le_monstre_offert(
            &espece("loup"),
            Niveau::new(20).unwrap(),
            &mut AleatoireScripte::new(tirages),
        );
        assert!(!monstre.est_shiny());
        assert_eq!(monstre.niveau().valeur(), 20);
    }
}
