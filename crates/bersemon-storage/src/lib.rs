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

/// %APPDATA%\Bersemon\sauvegarde.json sous Windows (section 13.9), ou le
/// dossier courant si la variable n'existe pas.
pub fn emplacement_par_defaut() -> PathBuf {
    match std::env::var_os("APPDATA") {
        Some(dossier) => PathBuf::from(dossier)
            .join("Bersemon")
            .join("sauvegarde.json"),
        None => PathBuf::from("sauvegarde.json"),
    }
}
