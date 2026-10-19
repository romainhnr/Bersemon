//! Espèces de monstres : ce que tous les monstres d'une même espèce ont en
//! commun (sections 5.1 et 7).

use crate::attaque::{Attaque, AttaqueId};
use crate::habilete::Habilete;
use crate::niveau::Niveau;
use crate::ressource::Ressource;
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

/// Rareté d'une espèce, qui fixe son prix de revente (section 14.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rarete {
    /// Monstres de départ compris.
    Commune,
    Rare,
    Apotre,
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
    rarete: Rarete,
    /// Celles qu'un monstre de l'espèce peut avoir ; la première est donnée
    /// aux monstres des sauvegardes antérieures (section 5.9).
    habiletes: Vec<Habilete>,
    /// Lâchée par un monstre sauvage de l'espèce mis KO (section 14.3). En
    /// boîte, pour que chaque monstre, qui porte son espèce, reste léger.
    ressource: Option<Box<Ressource>>,
}

impl Espece {
    /// `apprentissage` est rangé par niveau ; à niveau égal, l'ordre donné est
    /// conservé. L'espèce est commune, sauf rareté donnée ensuite avec
    /// `avec_rarete`, et sans habileté, sauf celles données avec
    /// `avec_habiletes`.
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
            rarete: Rarete::Commune,
            habiletes: Vec::new(),
            ressource: None,
        }
    }

    pub fn avec_ressource(mut self, ressource: Ressource) -> Self {
        self.ressource = Some(Box::new(ressource));
        self
    }

    pub fn ressource(&self) -> Option<&Ressource> {
        self.ressource.as_deref()
    }

    pub fn avec_rarete(mut self, rarete: Rarete) -> Self {
        self.rarete = rarete;
        self
    }

    pub fn avec_habiletes(mut self, habiletes: Vec<Habilete>) -> Self {
        self.habiletes = habiletes;
        self
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

    /// Avec la rareté, il fixe le prix de revente (section 14.5).
    pub fn total_des_stats_de_base(&self) -> u32 {
        let base = &self.stats_de_base;
        [
            base.pv,
            base.attaque,
            base.defense,
            base.attaque_speciale,
            base.defense_speciale,
            base.vitesse,
        ]
        .iter()
        .map(|&stat| u32::from(stat))
        .sum()
    }

    /// Sert au calcul de l'XP que rapporte un monstre de l'espèce vaincu
    /// (section 7).
    pub fn rendement_experience(&self) -> u16 {
        self.rendement_experience
    }

    pub fn apprentissage(&self) -> &[AttaqueApprise] {
        &self.apprentissage
    }

    pub fn rarete(&self) -> Rarete {
        self.rarete
    }

    pub fn habiletes(&self) -> &[Habilete] {
        &self.habiletes
    }

    pub fn attaques_du_niveau(&self, niveau: Niveau) -> impl Iterator<Item = &Attaque> {
        self.apprentissage
            .iter()
            .filter(move |apprise| apprise.niveau == niveau)
            .map(|apprise| &apprise.attaque)
    }

    /// Dans la V1, un monstre n'apprend que les attaques de la liste de son
    /// espèce : c'est là qu'on retrouve une attaque d'après son identifiant.
    pub fn attaque_apprise(&self, id: &AttaqueId) -> Option<&Attaque> {
        self.apprentissage
            .iter()
            .map(|apprise| &apprise.attaque)
            .find(|attaque| attaque.id() == id)
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
