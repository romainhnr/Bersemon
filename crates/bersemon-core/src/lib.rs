//! Noyau métier de Bersemon : les règles du jeu.
//!
//! Ce crate ne fait ni affichage, ni accès aux fichiers, ni lecture de
//! l'horloge, ni tirage aléatoire global : tout ce qui vient de l'extérieur
//! lui est fourni par l'appelant, pour qu'il serve tel quel à la console
//! comme au futur bot Discord, et reste testable de façon déterministe.

pub mod aleatoire;
pub mod ame;
pub mod attaque;
pub mod capsule;
pub mod capture;
pub mod catalogue;
pub mod codex;
pub mod collection;
pub mod combat;
pub mod contenu;
mod degats;
pub mod deroulement;
pub mod espece;
pub mod evolution;
pub mod expedition;
pub mod expedition_automatique;
pub mod experience;
pub mod habilete;
pub mod heros;
pub mod horodatage;
pub mod humain;
pub mod inventaire;
pub mod marchand;
pub mod monstre;
pub mod niveau;
pub mod nom;
pub mod partie;
pub mod ressource;
pub mod sauvegarde;
pub mod soin;
pub mod stats;
pub mod statut;
pub mod traque;
pub mod tutoriel;
pub mod types;
pub mod zone;

#[cfg(test)]
mod donnees_de_test;
