//! Accès au contenu du jeu par identifiant, par exemple pour recharger une
//! sauvegarde, qui ne garde que les identifiants (section 13.9). Le contenu
//! lui-même vit hors des règles : le noyau ne fait que le consulter.

use crate::espece::{Espece, EspeceId};

pub trait Catalogue {
    fn espece(&self, id: &EspeceId) -> Option<Espece>;
}
