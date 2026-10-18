//! Identifiants des boutons et des formulaires. Chacun porte l'action qu'il
//! déclenche et la version de la partie qu'il affichait : le bouton d'un
//! message dépassé est ainsi reconnu, même s'il propose un choix encore
//! possible, comme Oui.

use bersemon_core::deroulement::Choix;
use bersemon_core::espece::EspeceId;
use bersemon_core::heros::Classe;

/// Distingue les boutons de Bersemon de ceux d'autres usages éventuels.
const PREFIXE: &str = "bersemon";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    ReprendreLaPartie,
    NouvellePartie,
    Classe(Classe),
    MonstreDeDepart(EspeceId),
    Oui,
    Non,
    /// Ouvre le formulaire où le joueur écrit le nom de son héros.
    OuvrirLeFormulaireDuNom,
    /// Le formulaire du nom, une fois envoyé.
    NomSaisi,
}

impl Action {
    /// `None` pour les actions de formulaire, qui ne sont pas encore un choix.
    pub fn choix(&self) -> Option<Choix> {
        let choix = match self {
            Action::ReprendreLaPartie => Choix::ReprendreLaPartie,
            Action::NouvellePartie => Choix::NouvellePartie,
            Action::Classe(classe) => Choix::Classe(*classe),
            Action::MonstreDeDepart(espece) => Choix::MonstreDeDepart(espece.clone()),
            Action::Oui => Choix::Oui,
            Action::Non => Choix::Non,
            Action::OuvrirLeFormulaireDuNom | Action::NomSaisi => return None,
        };
        Some(choix)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bouton {
    pub version: u32,
    pub action: Action,
}

impl Bouton {
    /// Moins de 100 caractères, la limite de Discord.
    pub fn identifiant(&self) -> String {
        let action = match &self.action {
            Action::ReprendreLaPartie => "reprendre".to_string(),
            Action::NouvellePartie => "nouvelle-partie".to_string(),
            Action::Classe(Classe::Guerrier) => "classe:guerrier".to_string(),
            Action::Classe(Classe::Mage) => "classe:mage".to_string(),
            Action::MonstreDeDepart(espece) => format!("depart:{}", espece.as_str()),
            Action::Oui => "oui".to_string(),
            Action::Non => "non".to_string(),
            Action::OuvrirLeFormulaireDuNom => "nommer".to_string(),
            Action::NomSaisi => "nom".to_string(),
        };
        format!("{PREFIXE}:{}:{action}", self.version)
    }

    /// `None` pour un identifiant qui ne vient pas de Bersemon.
    pub fn lire(identifiant: &str) -> Option<Self> {
        let mut morceaux = identifiant.splitn(3, ':');
        if morceaux.next()? != PREFIXE {
            return None;
        }
        let version = morceaux.next()?.parse().ok()?;
        let action = match morceaux.next()? {
            "reprendre" => Action::ReprendreLaPartie,
            "nouvelle-partie" => Action::NouvellePartie,
            "classe:guerrier" => Action::Classe(Classe::Guerrier),
            "classe:mage" => Action::Classe(Classe::Mage),
            "oui" => Action::Oui,
            "non" => Action::Non,
            "nommer" => Action::OuvrirLeFormulaireDuNom,
            "nom" => Action::NomSaisi,
            autre => {
                let espece = autre.strip_prefix("depart:")?;
                Action::MonstreDeDepart(EspeceId::new(espece))
            }
        };
        Some(Self { version, action })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaque_bouton_se_relit_a_l_identique() {
        let actions = [
            Action::ReprendreLaPartie,
            Action::NouvellePartie,
            Action::Classe(Classe::Guerrier),
            Action::Classe(Classe::Mage),
            Action::MonstreDeDepart(EspeceId::new("salamandre")),
            Action::Oui,
            Action::Non,
            Action::OuvrirLeFormulaireDuNom,
            Action::NomSaisi,
        ];
        for action in actions {
            let bouton = Bouton {
                version: u32::MAX,
                action,
            };
            let identifiant = bouton.identifiant();
            assert!(identifiant.len() < 100, "{identifiant}");
            assert_eq!(Bouton::lire(&identifiant), Some(bouton));
        }
    }

    #[test]
    fn identifiant_etranger_ignore() {
        for identifiant in [
            "autre:1:oui",
            "bersemon:abc:oui",
            "bersemon:1:inconnu",
            "bersemon:1",
            "",
        ] {
            assert_eq!(Bouton::lire(identifiant), None, "{identifiant}");
        }
    }

    #[test]
    fn les_actions_de_formulaire_ne_sont_pas_des_choix() {
        assert_eq!(Action::OuvrirLeFormulaireDuNom.choix(), None);
        assert_eq!(Action::NomSaisi.choix(), None);
        assert_eq!(Action::Oui.choix(), Some(Choix::Oui));
    }
}
