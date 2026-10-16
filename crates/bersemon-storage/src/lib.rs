//! Persistance des parties de Bersemon : fichier JSON pour la V1, SQLite
//! envisagé ensuite.
//!
//! Les structures écrites sur disque sont propres à ce crate et converties
//! vers les types du noyau, pour que le format de sauvegarde évolue
//! indépendamment du modèle métier et que le noyau ne dépende pas du stockage.
