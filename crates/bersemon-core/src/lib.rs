//! Noyau métier de Bersemon : les règles du jeu.
//!
//! Ce crate ne fait ni affichage, ni accès aux fichiers, ni lecture de
//! l'horloge, ni tirage aléatoire global : tout ce qui vient de l'extérieur
//! lui est fourni par l'appelant, pour qu'il serve tel quel à la console
//! comme au futur bot Discord, et reste testable de façon déterministe.

pub mod aleatoire;
pub mod attaque;
pub mod capture;
pub mod catalogue;
pub mod codex;
pub mod collection;
pub mod combat;
mod degats;
pub mod espece;
pub mod expedition;
pub mod experience;
pub mod heros;
pub mod inventaire;
pub mod monstre;
pub mod niveau;
pub mod nom;
pub mod partie;
pub mod sauvegarde;
pub mod stats;
pub mod statut;
pub mod types;
pub mod zone;

#[cfg(test)]
mod donnees_de_test;
