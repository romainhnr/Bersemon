//! Textes du jeu, partagés par la console et le bot Discord : noms des
//! monstres, récit des événements, état d'un combat, erreurs. Chaque
//! interface ne garde que sa mise en page : menus numérotés pour la console,
//! boutons pour Discord.
//!
//! Les phrases évitent les accords en genre, que le nom d'un monstre ne
//! permet pas de connaître.

pub mod combat;
pub mod echanges;
pub mod equipement;
pub mod erreurs;
pub mod expeditions_automatiques;
pub mod forge;
pub mod marchand;
pub mod mise_en_valeur;
pub mod monstres;
pub mod partie;
pub mod recit;
pub mod soins;
