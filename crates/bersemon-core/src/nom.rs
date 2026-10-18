//! Noms choisis par le joueur : celui de son héros et les surnoms de ses
//! monstres (sections 4.1 et 5.5).

use std::fmt;

/// De 1 à 20 caractères, sans espace au début ni à la fin.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Nom(String);

impl Nom {
    pub const LONGUEUR_MAX: usize = 20;

    /// Les espaces de début et de fin sont retirés. Les caractères de
    /// contrôle sont refusés, car un nom s'affiche tel quel dans la console
    /// comme sur Discord.
    pub fn new(texte: &str) -> Result<Self, ErreurNom> {
        let texte = texte.trim();
        if texte.is_empty() {
            return Err(ErreurNom::Vide);
        }
        let longueur = texte.chars().count();
        if longueur > Self::LONGUEUR_MAX {
            return Err(ErreurNom::TropLong(longueur));
        }
        if let Some(caractere) = texte.chars().find(|caractere| caractere.is_control()) {
            return Err(ErreurNom::CaractereInterdit(caractere));
        }
        Ok(Self(texte.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurNom {
    Vide,
    TropLong(usize),
    CaractereInterdit(char),
}

impl fmt::Display for ErreurNom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurNom::Vide => write!(f, "un nom ne peut pas être vide"),
            ErreurNom::TropLong(longueur) => write!(
                f,
                "un nom compte au plus {} caractères, {longueur} donnés",
                Nom::LONGUEUR_MAX
            ),
            ErreurNom::CaractereInterdit(caractere) => {
                write!(f, "caractère interdit dans un nom : {caractere:?}")
            }
        }
    }
}

impl std::error::Error for ErreurNom {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepte_un_nom_accentue_et_retire_les_espaces_autour() {
        assert_eq!(Nom::new("  Éléonore ").unwrap().as_str(), "Éléonore");
    }

    #[test]
    fn longueur_comptee_en_caracteres() {
        assert!(Nom::new(&"é".repeat(20)).is_ok());
        assert_eq!(Nom::new(&"é".repeat(21)), Err(ErreurNom::TropLong(21)));
    }

    #[test]
    fn refuse_un_nom_vide_ou_blanc() {
        assert_eq!(Nom::new(""), Err(ErreurNom::Vide));
        assert_eq!(Nom::new("   "), Err(ErreurNom::Vide));
    }

    #[test]
    fn refuse_un_caractere_de_controle() {
        assert_eq!(Nom::new("Gu\tts"), Err(ErreurNom::CaractereInterdit('\t')));
    }
}
