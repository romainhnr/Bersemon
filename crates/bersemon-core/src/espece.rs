//! Espèces de monstres : ce que tous les monstres d'une même espèce ont en
//! commun (sections 5.1 et 7).

use crate::attaque::Attaque;
use crate::niveau::Niveau;
use crate::stats::StatsDeBase;
use crate::types::TypesMonstre;

/// Identifiant stable d'une espèce. C'est lui, et non les données de
/// l'espèce, qui est enregistré dans une sauvegarde : rééquilibrer une espèce
/// s'applique ainsi aux monstres déjà capturés.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EspeceId(String);

impl EspeceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Rôle d'une espèce, en plus de ses types. Il n'a pas d'effet dans la V1
/// (Q19). Hypothèse du code : le rôle est porté par l'espèce, pas par chaque
/// monstre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Soutien,
    Offensif,
}

/// Attaque qu'un monstre de l'espèce apprend en atteignant ce niveau (Q33).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttaqueApprise {
    pub niveau: Niveau,
    pub attaque: Attaque,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Espece {
    id: EspeceId,
    nom: String,
    types: TypesMonstre,
    role: Role,
    stats_de_base: StatsDeBase,
    rendement_experience: u16,
    apprentissage: Vec<AttaqueApprise>,
}

impl Espece {
    /// `apprentissage` est rangé par niveau ; à niveau égal, l'ordre donné est
    /// conservé.
    pub fn new(
        id: EspeceId,
        nom: impl Into<String>,
        types: TypesMonstre,
        role: Role,
        stats_de_base: StatsDeBase,
        rendement_experience: u16,
        mut apprentissage: Vec<AttaqueApprise>,
    ) -> Self {
        apprentissage.sort_by_key(|apprise| apprise.niveau);
        Self {
            id,
            nom: nom.into(),
            types,
            role,
            stats_de_base,
            rendement_experience,
            apprentissage,
        }
    }

    pub fn id(&self) -> &EspeceId {
        &self.id
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }

    pub fn types(&self) -> &TypesMonstre {
        &self.types
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn stats_de_base(&self) -> &StatsDeBase {
        &self.stats_de_base
    }

    /// Sert au calcul de l'XP que rapporte un monstre de l'espèce vaincu
    /// (section 7).
    pub fn rendement_experience(&self) -> u16 {
        self.rendement_experience
    }

    pub fn apprentissage(&self) -> &[AttaqueApprise] {
        &self.apprentissage
    }

    pub fn attaques_du_niveau(&self, niveau: Niveau) -> impl Iterator<Item = &Attaque> {
        self.apprentissage
            .iter()
            .filter(move |apprise| apprise.niveau == niveau)
            .map(|apprise| &apprise.attaque)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attaque::Categorie;
    use crate::donnees_de_test::{attaque, stats_uniformes};
    use crate::types::Type;

    fn apprise(niveau: u8, id: &str) -> AttaqueApprise {
        AttaqueApprise {
            niveau: Niveau::new(niveau).unwrap(),
            attaque: attaque(id, Type::Normal, Categorie::Physique, 40, 100),
        }
    }

    #[test]
    fn apprentissage_range_par_niveau_en_gardant_l_ordre_a_niveau_egal() {
        let espece = Espece::new(
            EspeceId::new("e"),
            "E",
            TypesMonstre::new(&[Type::Normal]).unwrap(),
            Role::Offensif,
            stats_uniformes(50),
            64,
            vec![
                apprise(6, "c"),
                apprise(1, "a"),
                apprise(6, "d"),
                apprise(3, "b"),
            ],
        );
        let ids: Vec<&str> = espece
            .apprentissage()
            .iter()
            .map(|apprise| apprise.attaque.id().as_str())
            .collect();
        assert_eq!(ids, vec!["a", "b", "c", "d"]);
        let du_niveau_6: Vec<&str> = espece
            .attaques_du_niveau(Niveau::new(6).unwrap())
            .map(|attaque| attaque.id().as_str())
            .collect();
        assert_eq!(du_niveau_6, vec!["c", "d"]);
    }
}
