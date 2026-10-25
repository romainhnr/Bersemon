//! Noyau métier de Bersemon : les règles du jeu.
//!
//! Ce crate ne fait ni affichage, ni accès aux fichiers, ni lecture de
//! l'horloge, ni tirage aléatoire global : tout ce qui vient de l'extérieur
//! lui est fourni par l'appelant, pour qu'il serve tel quel à la console
//! comme au futur bot Discord, et reste testable de façon déterministe.

pub mod aleatoire;
pub mod ame;
pub mod amelioration;
pub mod attaque;
pub mod beherit;
pub mod booster;
pub mod capsule;
pub mod capture;
pub mod catalogue;
pub mod codex;
pub mod collection;
pub mod combat;
pub mod contenu;
mod degats;
pub mod deroulement;
pub mod duel;
pub mod echange;
pub mod equipement;
pub mod espece;
pub mod evolution;
pub mod expedition;
pub mod expedition_automatique;
pub mod experience;
pub mod filet;
pub mod groupe;
pub mod habilete;
pub mod heros;
pub mod horodatage;
pub mod humain;
pub mod inventaire;
pub mod jour_et_nuit;
pub mod machines;
pub mod marchand;
pub mod meteo;
pub mod monstre;
pub mod niveau;
pub mod nom;
pub mod obeissance;
pub mod partie;
pub mod peche;
pub mod pierre;
pub mod recompenses_du_codex;
pub mod relations;
pub mod ressource;
pub mod rite;
pub mod sauvegarde;
pub mod soin;
pub mod stats;
pub mod statut;
pub mod tour;
pub mod traque;
pub mod tutoriel;
pub mod types;
pub mod zone;

#[cfg(test)]
mod donnees_de_test;
