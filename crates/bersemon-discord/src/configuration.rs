//! Réglages du bot, lus dans les variables d'environnement : le jeton n'est
//! jamais écrit dans le dépôt.

use std::fmt;
use std::num::NonZeroU64;

/// Jeton du bot, donné par le portail développeur de Discord.
const VARIABLE_JETON: &str = "DISCORD_TOKEN";
/// Serveur où les commandes sont enregistrées tout de suite, pour développer.
const VARIABLE_SERVEUR_DE_TEST: &str = "BERSEMON_SERVEUR_DE_TEST";

/// Pas de `Debug` : le jeton ne doit apparaître dans aucun message.
pub struct Configuration {
    pub jeton: String,
    /// `None` : commandes enregistrées pour tous les serveurs et les messages
    /// privés.
    pub serveur_de_test: Option<NonZeroU64>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErreurDeConfiguration {
    JetonAbsent,
    ServeurDeTestInvalide(String),
}

impl fmt::Display for ErreurDeConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurDeConfiguration::JetonAbsent => write!(
                f,
                "la variable d'environnement {VARIABLE_JETON} doit contenir le jeton du bot (voir docs/discord.md)"
            ),
            ErreurDeConfiguration::ServeurDeTestInvalide(texte) => write!(
                f,
                "{VARIABLE_SERVEUR_DE_TEST} doit contenir l'identifiant d'un serveur Discord, et non \"{texte}\""
            ),
        }
    }
}

impl std::error::Error for ErreurDeConfiguration {}

impl Configuration {
    pub fn depuis_l_environnement() -> Result<Self, ErreurDeConfiguration> {
        Self::lire(
            std::env::var(VARIABLE_JETON).ok(),
            std::env::var(VARIABLE_SERVEUR_DE_TEST).ok(),
        )
    }

    /// Une variable vide compte comme absente.
    fn lire(
        jeton: Option<String>,
        serveur_de_test: Option<String>,
    ) -> Result<Self, ErreurDeConfiguration> {
        let jeton = renseignee(jeton).ok_or(ErreurDeConfiguration::JetonAbsent)?;
        let serveur_de_test = match renseignee(serveur_de_test) {
            None => None,
            Some(texte) => Some(
                texte
                    .parse()
                    .map_err(|_| ErreurDeConfiguration::ServeurDeTestInvalide(texte))?,
            ),
        };
        Ok(Self {
            jeton,
            serveur_de_test,
        })
    }
}

fn renseignee(valeur: Option<String>) -> Option<String> {
    valeur
        .map(|valeur| valeur.trim().to_string())
        .filter(|valeur| !valeur.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lire(
        jeton: Option<&str>,
        serveur: Option<&str>,
    ) -> Result<Configuration, ErreurDeConfiguration> {
        Configuration::lire(jeton.map(String::from), serveur.map(String::from))
    }

    #[test]
    fn jeton_absent_ou_vide_refuse() {
        assert!(matches!(
            lire(None, None),
            Err(ErreurDeConfiguration::JetonAbsent)
        ));
        assert!(matches!(
            lire(Some("  "), None),
            Err(ErreurDeConfiguration::JetonAbsent)
        ));
    }

    #[test]
    fn sans_serveur_de_test_les_commandes_sont_globales() {
        let configuration = lire(Some(" jeton "), Some("")).unwrap();
        assert_eq!(configuration.jeton, "jeton");
        assert_eq!(configuration.serveur_de_test, None);
    }

    #[test]
    fn serveur_de_test_lu_comme_identifiant() {
        let configuration = lire(Some("jeton"), Some("175928847299117063")).unwrap();
        assert_eq!(
            configuration.serveur_de_test,
            NonZeroU64::new(175928847299117063)
        );
        for invalide in ["0", "abc", "-12"] {
            assert!(matches!(
                lire(Some("jeton"), Some(invalide)),
                Err(ErreurDeConfiguration::ServeurDeTestInvalide(texte)) if texte == invalide
            ));
        }
    }
}
