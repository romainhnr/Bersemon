//! Attaques des monstres (sections 5.1 et 10.4).

use std::fmt;

use crate::types::Type;

/// Identifiant stable d'une attaque, enregistré dans les sauvegardes à la
/// place des données de l'attaque.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttaqueId(String);

impl AttaqueId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Une attaque physique oppose l'Attaque à la Défense, une attaque spéciale
/// l'Attaque spéciale à la Défense spéciale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Categorie {
    Physique,
    Speciale,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attaque {
    id: AttaqueId,
    nom: String,
    type_attaque: Type,
    categorie: Categorie,
    puissance: u16,
    precision: u8,
}

impl Attaque {
    pub const PRECISION_MAX: u8 = 100;

    /// `precision` est un pourcentage, de 1 à 100.
    pub fn new(
        id: AttaqueId,
        nom: impl Into<String>,
        type_attaque: Type,
        categorie: Categorie,
        puissance: u16,
        precision: u8,
    ) -> Result<Self, ErreurAttaque> {
        // Les attaques sans dégâts relèvent des effets de statut (Q26), qui
        // ne sont pas encore définis.
        if puissance == 0 {
            return Err(ErreurAttaque::PuissanceNulle);
        }
        if !(1..=Self::PRECISION_MAX).contains(&precision) {
            return Err(ErreurAttaque::PrecisionInvalide(precision));
        }
        Ok(Self {
            id,
            nom: nom.into(),
            type_attaque,
            categorie,
            puissance,
            precision,
        })
    }

    pub fn id(&self) -> &AttaqueId {
        &self.id
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }

    pub fn type_attaque(&self) -> Type {
        self.type_attaque
    }

    pub fn categorie(&self) -> Categorie {
        self.categorie
    }

    pub fn puissance(&self) -> u16 {
        self.puissance
    }

    pub fn precision(&self) -> u8 {
        self.precision
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurAttaque {
    PuissanceNulle,
    PrecisionInvalide(u8),
}

impl fmt::Display for ErreurAttaque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurAttaque::PuissanceNulle => write!(f, "une attaque doit avoir une puissance"),
            ErreurAttaque::PrecisionInvalide(precision) => write!(
                f,
                "précision {precision} hors de la plage 1 à {}",
                Attaque::PRECISION_MAX
            ),
        }
    }
}

impl std::error::Error for ErreurAttaque {}

/// Attaques connues par un monstre : de 1 à 4 (Q4), sans doublon. Le minimum
/// et l'absence de doublon sont des hypothèses du code (Q48).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttaquesMonstre(Vec<Attaque>);

impl AttaquesMonstre {
    pub const MAX: usize = 4;

    pub fn new(attaques: Vec<Attaque>) -> Result<Self, ErreurAttaquesMonstre> {
        if attaques.is_empty() {
            return Err(ErreurAttaquesMonstre::Aucune);
        }
        if attaques.len() > Self::MAX {
            return Err(ErreurAttaquesMonstre::TropNombreuses(attaques.len()));
        }
        for (position, attaque) in attaques.iter().enumerate() {
            if attaques[..position]
                .iter()
                .any(|precedente| precedente.id() == attaque.id())
            {
                return Err(ErreurAttaquesMonstre::EnDouble(attaque.id().clone()));
            }
        }
        Ok(Self(attaques))
    }

    pub fn liste(&self) -> &[Attaque] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurAttaquesMonstre {
    Aucune,
    TropNombreuses(usize),
    EnDouble(AttaqueId),
}

impl fmt::Display for ErreurAttaquesMonstre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurAttaquesMonstre::Aucune => {
                write!(f, "un monstre doit connaître au moins une attaque")
            }
            ErreurAttaquesMonstre::TropNombreuses(nombre) => write!(
                f,
                "un monstre connaît au plus {} attaques, {nombre} données",
                AttaquesMonstre::MAX
            ),
            ErreurAttaquesMonstre::EnDouble(id) => {
                write!(f, "attaque {} présente plusieurs fois", id.as_str())
            }
        }
    }
}

impl std::error::Error for ErreurAttaquesMonstre {}

#[cfg(test)]
mod tests {
    use super::*;

    fn attaque(id: &str) -> Attaque {
        Attaque::new(
            AttaqueId::new(id),
            id,
            Type::Normal,
            Categorie::Physique,
            40,
            100,
        )
        .unwrap()
    }

    #[test]
    fn refuse_une_puissance_nulle() {
        let resultat = Attaque::new(
            AttaqueId::new("a"),
            "A",
            Type::Normal,
            Categorie::Physique,
            0,
            100,
        );
        assert_eq!(resultat, Err(ErreurAttaque::PuissanceNulle));
    }

    #[test]
    fn refuse_une_precision_hors_plage() {
        for precision in [0, 101] {
            let resultat = Attaque::new(
                AttaqueId::new("a"),
                "A",
                Type::Normal,
                Categorie::Physique,
                40,
                precision,
            );
            assert_eq!(resultat, Err(ErreurAttaque::PrecisionInvalide(precision)));
        }
    }

    #[test]
    fn accepte_de_une_a_quatre_attaques() {
        assert!(AttaquesMonstre::new(vec![attaque("a")]).is_ok());
        let quatre = vec![attaque("a"), attaque("b"), attaque("c"), attaque("d")];
        assert_eq!(AttaquesMonstre::new(quatre).unwrap().liste().len(), 4);
    }

    #[test]
    fn refuse_aucune_attaque() {
        assert_eq!(
            AttaquesMonstre::new(vec![]),
            Err(ErreurAttaquesMonstre::Aucune)
        );
    }

    #[test]
    fn refuse_plus_de_quatre_attaques() {
        let cinq = vec![
            attaque("a"),
            attaque("b"),
            attaque("c"),
            attaque("d"),
            attaque("e"),
        ];
        assert_eq!(
            AttaquesMonstre::new(cinq),
            Err(ErreurAttaquesMonstre::TropNombreuses(5))
        );
    }

    #[test]
    fn refuse_une_attaque_en_double() {
        assert_eq!(
            AttaquesMonstre::new(vec![attaque("a"), attaque("b"), attaque("a")]),
            Err(ErreurAttaquesMonstre::EnDouble(AttaqueId::new("a")))
        );
    }
}
