//! Persistance des parties de Bersemon : fichier JSON pour la V1, SQLite
//! envisagé ensuite.
//!
//! Les structures écrites sur disque sont propres à ce crate et converties
//! vers les types du noyau, pour que le format de sauvegarde évolue
//! indépendamment du modèle métier et que le noyau ne dépende pas du stockage.

mod fichier_json;
mod format;

use std::path::PathBuf;

pub use fichier_json::FichierJson;

/// %APPDATA%\Bersemon sous Windows, ou le dossier courant si la variable
/// n'existe pas.
pub fn dossier_par_defaut() -> PathBuf {
    match std::env::var_os("APPDATA") {
        Some(dossier) => PathBuf::from(dossier).join("Bersemon"),
        None => PathBuf::new(),
    }
}

/// Sauvegarde de la console (section 13.9).
pub fn emplacement_par_defaut() -> PathBuf {
    dossier_par_defaut().join("sauvegarde.json")
}
