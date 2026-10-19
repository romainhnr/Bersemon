//! Humains qui combattent avec leurs monstres (section 10.7) : le premier
//! est le brigand qui attend au bout de la Lisière.

use std::fmt;

use crate::espece::{Espece, EspeceId};
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::stats::{Stats, ValeursInternes};

/// Les monstres d'un humain ont des valeurs internes fixes : le combat est le
/// même pour tous les joueurs (section 10.7).
const VALEUR_INTERNE: u8 = 15;

/// IA d'un humain (section 10.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IaHumain {
    /// L'attaque aux dégâts estimés les plus hauts.
    Simple,
    /// La même, et un changement de monstre quand le sien ne peut que peu ou
    /// pas blesser.
    Relevee,
}

/// Identifiant stable d'un humain, enregistré dans la sauvegarde une fois
/// qu'il est vaincu.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HumainId(String);

impl HumainId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurHumain {
    EquipeVide,
    /// L'espèce n'apprend aucune attaque, ou en apprend une en double,
    /// jusqu'à ce niveau.
    EspeceInvalide {
        espece: EspeceId,
        niveau: Niveau,
    },
}

impl fmt::Display for ErreurHumain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurHumain::EquipeVide => write!(f, "un humain combat avec au moins un monstre"),
            ErreurHumain::EspeceInvalide { espece, niveau } => write!(
                f,
                "l'espèce {} ne peut pas être créée au niveau {}",
                espece.as_str(),
                niveau.valeur()
            ),
        }
    }
}

impl std::error::Error for ErreurHumain {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Humain {
    id: HumainId,
    nom: String,
    /// Ses monstres, dans l'ordre où ils entrent en combat.
    equipe: Vec<(Espece, Niveau)>,
    /// Gagné par le joueur qui le vainc.
    argent: u32,
    avant_combat: String,
    apres_victoire: String,
    ia: IaHumain,
}

impl Humain {
    /// Vérifie que chaque monstre de son équipe peut être créé à son niveau :
    /// sa génération ne peut ensuite plus échouer. Son IA est simple, sauf
    /// autre donnée avec `avec_ia`.
    pub fn new(
        id: HumainId,
        nom: impl Into<String>,
        equipe: Vec<(Espece, Niveau)>,
        argent: u32,
        avant_combat: impl Into<String>,
        apres_victoire: impl Into<String>,
    ) -> Result<Self, ErreurHumain> {
        if equipe.is_empty() {
            return Err(ErreurHumain::EquipeVide);
        }
        for (espece, niveau) in &equipe {
            Monstre::creer(espece.clone(), *niveau, valeurs_internes(), false).map_err(|_| {
                ErreurHumain::EspeceInvalide {
                    espece: espece.id().clone(),
                    niveau: *niveau,
                }
            })?;
        }
        Ok(Self {
            id,
            nom: nom.into(),
            equipe,
            argent,
            avant_combat: avant_combat.into(),
            apres_victoire: apres_victoire.into(),
            ia: IaHumain::Simple,
        })
    }

    pub fn avec_ia(mut self, ia: IaHumain) -> Self {
        self.ia = ia;
        self
    }

    pub fn ia(&self) -> IaHumain {
        self.ia
    }

    pub fn id(&self) -> &HumainId {
        &self.id
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }

    pub fn argent(&self) -> u32 {
        self.argent
    }

    pub fn avant_combat(&self) -> &str {
        &self.avant_combat
    }

    pub fn apres_victoire(&self) -> &str {
        &self.apres_victoire
    }

    /// Valeurs internes fixes, jamais shiny, la première habileté de leur
    /// espèce : aucun tirage (section 10.7).
    pub fn generer_equipe(&self) -> Vec<Monstre> {
        self.equipe
            .iter()
            .map(|(espece, niveau)| {
                Monstre::creer(espece.clone(), *niveau, valeurs_internes(), false)
                    .expect("équipe vérifiée par Humain::new")
            })
            .collect()
    }
}

fn valeurs_internes() -> ValeursInternes {
    let valeur = VALEUR_INTERNE;
    ValeursInternes::new(Stats {
        pv: valeur,
        attaque: valeur,
        defense: valeur,
        attaque_speciale: valeur,
        defense_speciale: valeur,
        vitesse: valeur,
    })
    .expect("valeur interne des humains valide")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attaque::Categorie;
    use crate::donnees_de_test::{attaque, espece, stats_uniformes};
    use crate::espece::{AttaqueApprise, Role};
    use crate::habilete::Habilete;
    use crate::types::{Type, TypesMonstre};

    fn niveau(valeur: u8) -> Niveau {
        Niveau::new(valeur).unwrap()
    }

    #[test]
    fn son_equipe_est_fixe() {
        let loup = espece("loup").avec_habiletes(vec![Habilete::Traqueur, Habilete::Fouineur]);
        let brigand = Humain::new(
            HumainId::new("brigand"),
            "Brigand",
            vec![(loup, niveau(7)), (espece("corbeau"), niveau(8))],
            60,
            "",
            "",
        )
        .unwrap();
        let equipe = brigand.generer_equipe();
        assert_eq!(equipe.len(), 2);
        assert_eq!(equipe[0].niveau(), niveau(7));
        assert_eq!(equipe[1].espece().id().as_str(), "corbeau");
        assert_eq!(equipe[0].valeurs_internes().valeurs().vitesse, 15);
        assert!(!equipe[0].est_shiny());
        assert_eq!(equipe[0].habilete(), Some(Habilete::Traqueur));
        assert_eq!(brigand.generer_equipe(), equipe);
    }

    #[test]
    fn equipe_invalide_refusee() {
        let creer = |equipe| Humain::new(HumainId::new("h"), "H", equipe, 0, "", "");
        assert_eq!(creer(Vec::new()), Err(ErreurHumain::EquipeVide));
        let tardive = Espece::new(
            EspeceId::new("tardive"),
            "Tardive",
            TypesMonstre::new(&[Type::Feu]).unwrap(),
            Role::Offensif,
            stats_uniformes(50),
            64,
            vec![AttaqueApprise {
                niveau: niveau(9),
                attaque: attaque("coup", Type::Normal, Categorie::Physique, 40, 100),
            }],
        );
        assert_eq!(
            creer(vec![(tardive, niveau(8))]),
            Err(ErreurHumain::EspeceInvalide {
                espece: EspeceId::new("tardive"),
                niveau: niveau(8),
            })
        );
    }
}
