//! Offres de don et d'échange entre joueurs du bot (section 15.1), gardées en
//! mémoire jusqu'à leur réponse : un redémarrage du bot les perd (Q89). Les
//! règles du transfert sont dans le noyau (`bersemon_core::echange`).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use bersemon_core::collection::{Collection, Emplacement};
use bersemon_core::echange::{ErreurEchange, Offre};
use bersemon_presentation::echanges::erreur_echange;

use crate::joueurs::IdJoueur;

/// Le pseudo Discord de chaque joueur, noté à chacune de ses interactions,
/// pour que les autres le reconnaissent dans la liste des joueurs.
const FICHIER_DES_PSEUDOS: &str = "pseudos.txt";

/// Ce que le joueur fait depuis l'écran des échanges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionDEchange {
    /// Sans `demande`, c'est un don.
    Offrir {
        a: IdJoueur,
        cede: Emplacement,
        demande: Option<Emplacement>,
    },
    /// L'offre envoyée, tant qu'elle n'est pas acceptée.
    Retirer,
    /// L'offre reçue.
    Accepter,
    Refuser,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurDOffre {
    AvecSoiMeme,
    JoueurSansPartie,
    /// Une offre envoyée à la fois.
    OffreDejaEnvoyee,
    /// L'autre joueur a déjà une offre reçue en attente.
    AutreDejaSollicite,
    AucuneOffre,
    Echange(ErreurEchange),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffreEnAttente {
    pub de: IdJoueur,
    pub a: IdJoueur,
    pub offre: Offre,
}

/// Au plus une offre envoyée et une offre reçue par joueur.
#[derive(Debug, Default)]
pub struct Bourse {
    offres: Vec<OffreEnAttente>,
}

impl Bourse {
    pub fn envoyee(&self, joueur: IdJoueur) -> Option<&OffreEnAttente> {
        self.offres.iter().find(|offre| offre.de == joueur)
    }

    pub fn recue(&self, joueur: IdJoueur) -> Option<&OffreEnAttente> {
        self.offres.iter().find(|offre| offre.a == joueur)
    }

    pub fn deposer(&mut self, offre: OffreEnAttente) -> Result<(), ErreurDOffre> {
        if self.envoyee(offre.de).is_some() {
            return Err(ErreurDOffre::OffreDejaEnvoyee);
        }
        if self.recue(offre.a).is_some() {
            return Err(ErreurDOffre::AutreDejaSollicite);
        }
        self.offres.push(offre);
        Ok(())
    }

    pub fn retirer_l_envoyee(&mut self, joueur: IdJoueur) -> Option<OffreEnAttente> {
        let position = self.offres.iter().position(|offre| offre.de == joueur)?;
        Some(self.offres.remove(position))
    }

    pub fn retirer_la_recue(&mut self, joueur: IdJoueur) -> Option<OffreEnAttente> {
        let position = self.offres.iter().position(|offre| offre.a == joueur)?;
        Some(self.offres.remove(position))
    }
}

pub struct Pseudos {
    fichier: PathBuf,
    pseudos: HashMap<IdJoueur, String>,
}

impl Pseudos {
    /// Un fichier absent ou une ligne illisible ne bloque rien : le pseudo
    /// manquera seulement dans la liste des joueurs.
    pub fn charger(dossier: &Path) -> Self {
        let fichier = dossier.join(FICHIER_DES_PSEUDOS);
        let pseudos = fs::read_to_string(&fichier)
            .unwrap_or_default()
            .lines()
            .filter_map(|ligne| {
                let (joueur, pseudo) = ligne.split_once(' ')?;
                Some((joueur.parse().ok()?, pseudo.to_string()))
            })
            .collect();
        Self { fichier, pseudos }
    }

    pub fn pseudo(&self, joueur: IdJoueur) -> Option<&str> {
        self.pseudos.get(&joueur).map(String::as_str)
    }

    /// N'écrit le fichier que si le pseudo change. Un échec d'écriture est
    /// signalé dans la console du bot, sans bloquer le joueur.
    pub fn noter(&mut self, joueur: IdJoueur, pseudo: &str) {
        let pseudo = pseudo.replace(['\n', '\r'], " ");
        if self.pseudo(joueur) == Some(pseudo.as_str()) {
            return;
        }
        self.pseudos.insert(joueur, pseudo);
        let mut lignes: Vec<String> = self
            .pseudos
            .iter()
            .map(|(joueur, pseudo)| format!("{joueur} {pseudo}\n"))
            .collect();
        lignes.sort();
        let ecriture = self
            .fichier
            .parent()
            .map_or(Ok(()), fs::create_dir_all)
            .and_then(|()| fs::write(&self.fichier, lignes.concat()));
        if let Err(erreur) = ecriture {
            eprintln!("Pseudos impossibles à enregistrer : {erreur}.");
        }
    }
}

/// Un autre joueur du bot, tel que le montre la liste des joueurs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutreJoueur {
    pub id: IdJoueur,
    pub heros: String,
    pub pseudo: Option<String>,
}

/// Ce que montre l'écran des échanges, préparé par `Joueurs` : l'écran n'a
/// accès qu'à la partie de son joueur.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TableauDesEchanges {
    /// Les autres joueurs qui ont une partie, triés par nom du héros, 25 au
    /// plus (Q89).
    pub joueurs: Vec<AutreJoueur>,
    pub recue: Option<(AutreJoueur, Offre)>,
    pub envoyee: Option<(AutreJoueur, Offre)>,
    /// Le joueur choisi pour une offre, avec ses monstres.
    pub choisi: Option<(AutreJoueur, Collection)>,
}

/// Un message privé à envoyer dès que possible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessagePrive {
    pub joueur: IdJoueur,
    pub texte: String,
}

/// Le refus d'une action de l'écran des échanges, vu par celui qui l'a
/// tentée.
pub fn erreur_d_offre(erreur: &ErreurDOffre) -> String {
    match erreur {
        ErreurDOffre::AvecSoiMeme => "Vous ne pouvez pas vous faire une offre.".to_string(),
        ErreurDOffre::JoueurSansPartie => "Ce joueur n'a pas de partie sur le bot.".to_string(),
        ErreurDOffre::OffreDejaEnvoyee => {
            "Vous avez déjà une offre en attente : retirez-la avant d'en faire une autre."
                .to_string()
        }
        ErreurDOffre::AutreDejaSollicite => {
            "Ce joueur a déjà une offre en attente : réessayez plus tard.".to_string()
        }
        ErreurDOffre::AucuneOffre => "Cette offre n'est plus en attente.".to_string(),
        ErreurDOffre::Echange(erreur) => erreur_echange(erreur),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_pseudos_se_relisent() {
        let dossier =
            std::env::temp_dir().join(format!("bersemon-discord-pseudos-{}", std::process::id()));
        let mut pseudos = Pseudos::charger(&dossier);
        assert_eq!(pseudos.pseudo(7), None);
        pseudos.noter(7, "Ysolde la Grise");
        pseudos.noter(3, "aldric");
        let relus = Pseudos::charger(&dossier);
        assert_eq!(relus.pseudo(7), Some("Ysolde la Grise"));
        assert_eq!(relus.pseudo(3), Some("aldric"));
        let _ = fs::remove_dir_all(dossier);
    }
}
