//! Niveau d'un monstre, de 1 à 100 (section 7).

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Niveau(u8);

impl Niveau {
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 100;

    pub fn new(valeur: u8) -> Result<Self, ErreurNiveau> {
        if (Self::MIN..=Self::MAX).contains(&valeur) {
            Ok(Self(valeur))
        } else {
            Err(ErreurNiveau { valeur })
        }
    }

    pub fn valeur(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurNiveau {
    pub valeur: u8,
}

impl fmt::Display for ErreurNiveau {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "niveau {} hors de la plage {} à {}",
            self.valeur,
            Niveau::MIN,
            Niveau::MAX
        )
    }
}

impl std::error::Error for ErreurNiveau {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepte_les_bornes() {
        assert_eq!(Niveau::new(1).map(Niveau::valeur), Ok(1));
        assert_eq!(Niveau::new(100).map(Niveau::valeur), Ok(100));
    }

    #[test]
    fn refuse_hors_bornes() {
        assert_eq!(Niveau::new(0), Err(ErreurNiveau { valeur: 0 }));
        assert_eq!(Niveau::new(101), Err(ErreurNiveau { valeur: 101 }));
    }
}
