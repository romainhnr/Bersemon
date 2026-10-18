//! Réglages du bot, lus dans les variables d'environnement : le jeton n'est
//! jamais écrit dans le dépôt.

use std::fmt;
use std::num::NonZeroU64;
use std::path::PathBuf;

/// Jeton du bot, donné par le portail développeur de Discord.
const VARIABLE_JETON: &str = "DISCORD_TOKEN";
/// Serveur où les commandes sont enregistrées tout de suite, pour développer.
const VARIABLE_SERVEUR_DE_TEST: &str = "BERSEMON_SERVEUR_DE_TEST";
/// Dossier des sauvegardes des joueurs Discord, utile pour l'hébergement.
const VARIABLE_DOSSIER: &str = "BERSEMON_DONNEES";
/// Renseignée, /bersemon répond par un message visible de tout le salon,
/// pour les tests à plusieurs (section 2).
const VARIABLE_MESSAGES_PUBLICS: &str = "BERSEMON_MESSAGES_PUBLICS";

/// Pas de `Debug` : le jeton ne doit apparaître dans aucun message.
pub struct Configuration {
    pub jeton: String,
    /// `None` : commandes enregistrées pour tous les serveurs et les messages
    /// privés.
    pub serveur_de_test: Option<NonZeroU64>,
    /// %APPDATA%\Bersemon\discord par défaut, à côté de la sauvegarde de la
    /// console, dont les parties Discord sont séparées (Q59).
    pub dossier_des_parties: PathBuf,
    /// `false` par défaut : les réponses ne sont visibles que du joueur.
    pub messages_publics: bool,
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
            std::env::var(VARIABLE_DOSSIER).ok(),
            std::env::var(VARIABLE_MESSAGES_PUBLICS).ok(),
        )
    }

    /// Une variable vide compte comme absente.
    fn lire(
        jeton: Option<String>,
        serveur_de_test: Option<String>,
        dossier: Option<String>,
        messages_publics: Option<String>,
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
        let dossier_des_parties = renseignee(dossier)
            .map(PathBuf::from)
            .unwrap_or_else(|| bersemon_storage::dossier_par_defaut().join("discord"));
        Ok(Self {
            jeton,
            serveur_de_test,
            dossier_des_parties,
            messages_publics: renseignee(messages_publics).is_some(),
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
        Configuration::lire(
            jeton.map(String::from),
            serveur.map(String::from),
            None,
            None,
        )
    }

    #[test]
    fn messages_publics_seulement_si_la_variable_est_renseignee() {
        assert!(!lire(Some("jeton"), None).unwrap().messages_publics);
        let lire_publics = |valeur: &str| {
            Configuration::lire(
                Some("jeton".to_string()),
                None,
                None,
                Some(valeur.to_string()),
            )
            .unwrap()
            .messages_publics
        };
        assert!(!lire_publics(" "));
        assert!(lire_publics("1"));
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

    #[test]
    fn dossier_des_parties_par_defaut_ou_donne() {
        let par_defaut = lire(Some("jeton"), None).unwrap();
        assert!(par_defaut.dossier_des_parties.ends_with("discord"));
        let donne = Configuration::lire(
            Some("jeton".to_string()),
            None,
            Some("D:\\Bersemon\\parties".to_string()),
            None,
        )
        .unwrap();
        assert_eq!(
            donne.dossier_des_parties,
            PathBuf::from("D:\\Bersemon\\parties")
        );
    }
}
