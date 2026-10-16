//! Noyau métier de Bersemon : les règles du jeu.
//!
//! Ce crate ne fait ni affichage, ni accès aux fichiers, ni lecture de
//! l'horloge, ni tirage aléatoire global : tout ce qui vient de l'extérieur
//! lui est fourni par l'appelant, pour qu'il serve tel quel à la console
//! comme au futur bot Discord, et reste testable de façon déterministe.

pub mod aleatoire;
pub mod espece;
pub mod monstre;
pub mod niveau;
pub mod stats;
pub mod types;
