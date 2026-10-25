//! Le Rite du sang mêlé de Malvoise (section 13.11) : deux monstres d'une
//! même famille d'évolution lui sont confiés pendant 2 heures réelles, puis
//! donnent un petit de la forme de base de la famille, au niveau 1, qui
//! reçoit, pour chaque stat, la meilleure valeur interne de ses parents.

use std::fmt;
use std::time::Duration;

use crate::aleatoire::Aleatoire;
use crate::collection::ErreurCollection;
use crate::espece::Espece;
use crate::horodatage::Horodatage;
use crate::monstre::{Monstre, tirer_shiny};
use crate::niveau::Niveau;
use crate::ressource::Ressource;

/// Valeurs de docs/contenu-v1.md, section 28.
pub const DUREE_DU_RITE: Duration = Duration::from_secs(2 * 60 * 60);
pub const PRIX_DU_RITE: u32 = 50;
/// De la ressource que lâche l'espèce du petit ; une espèce qui n'en lâche
/// pas ne coûte que l'argent.
pub const RESSOURCES_DU_RITE: u32 = 3;
pub const NIVEAU_DU_PETIT: u8 = 1;
/// Places du camp gardées pendant le rite : celles des deux parents, qui
/// les gardent comme un monstre parti, et celle du petit à naître.
pub const PLACES_DU_RITE: usize = 3;

/// La forme de base de la famille d'évolution de cette espèce : celle
/// qu'aucune autre ne précède (section 5.4).
pub fn forme_de_base<'a>(espece: &'a Espece, especes: &'a [Espece]) -> &'a Espece {
    let mut forme = espece;
    // Borné par le nombre d'espèces, au cas où un contenu formerait une
    // boucle d'évolutions.
    for _ in 0..especes.len() {
        let precedente = especes.iter().find(|autre| {
            autre
                .evolution()
                .is_some_and(|evolution| evolution.vers() == forme.id())
        });
        match precedente {
            Some(precedente) => forme = precedente,
            None => break,
        }
    }
    forme
}

/// Les deux parents confiés à Malvoise, jusqu'à la fin du rite. Ils gardent
/// leur place au camp, et le petit en a une de réservée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rite {
    parents: [Monstre; 2],
    fin: Horodatage,
}

impl Rite {
    pub fn new(parents: [Monstre; 2], fin: Horodatage) -> Self {
        Self { parents, fin }
    }

    pub fn parents(&self) -> &[Monstre; 2] {
        &self.parents
    }

    pub fn fin(&self) -> Horodatage {
        self.fin
    }

    pub fn est_fini(&self, maintenant: Horodatage) -> bool {
        maintenant >= self.fin
    }

    pub(crate) fn rendre_les_parents(self) -> [Monstre; 2] {
        self.parents
    }

    /// Le petit, de l'espèce `forme_de_base`, avec la meilleure valeur
    /// interne de ses parents pour chaque stat. Comme pour un monstre
    /// sauvage, il a 1 chance sur 512 d'être shiny et tire son habileté :
    /// tirages, le shiny, puis l'habileté.
    pub(crate) fn faire_naitre(
        &self,
        forme_de_base: &Espece,
        aleatoire: &mut dyn Aleatoire,
    ) -> Monstre {
        let [premier, second] = &self.parents;
        let valeurs_internes = premier
            .valeurs_internes()
            .meilleures(second.valeurs_internes());
        let shiny = tirer_shiny(aleatoire);
        let niveau = Niveau::new(NIVEAU_DU_PETIT).expect("niveau du petit valide");
        Monstre::creer(forme_de_base.clone(), niveau, valeurs_internes, shiny)
            .expect("toute forme de base connaît une attaque au niveau 1")
            .tirer_son_habilete(aleatoire)
    }
}

/// Ce que le rite demandera et donnera, de quoi demander confirmation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApercuDuRite {
    pub parents: [Monstre; 2],
    pub petit: Espece,
    /// Vide pour une espèce qui ne lâche pas de ressource.
    pub ressources: Vec<(Ressource, u32)>,
    pub prix: u32,
    pub duree: Duration,
}

/// Le petit, à sa place dans l'équipe ou au camp, et ses parents rendus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaissanceDuRite {
    pub petit: Monstre,
    pub parents: [Monstre; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurRite {
    /// Malvoise n'est pas encore au camp (section 14.2).
    AucunMarchand,
    /// Un rite à la fois.
    RiteEnCours,
    FamillesDifferentes,
    /// Aucune place au camp à réserver au petit.
    AucunePlacePourLePetit,
    RessourcesManquantes(Vec<(Ressource, u32)>),
    ArgentInsuffisant {
        prix: u32,
        argent: u32,
    },
    AucunRite,
    PasEncoreFini(Horodatage),
    Collection(ErreurCollection),
}

impl fmt::Display for ErreurRite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurRite::AucunMarchand => write!(f, "aucun marchand au camp"),
            ErreurRite::RiteEnCours => write!(f, "un rite est déjà en cours"),
            ErreurRite::FamillesDifferentes => {
                write!(f, "les deux monstres ne sont pas de la même famille")
            }
            ErreurRite::AucunePlacePourLePetit => write!(f, "aucune place au camp pour le petit"),
            ErreurRite::RessourcesManquantes(manquantes) => {
                let liste: Vec<String> = manquantes
                    .iter()
                    .map(|(ressource, nombre)| format!("{nombre} {}", ressource.nom()))
                    .collect();
                write!(f, "il manque {}", liste.join(", "))
            }
            ErreurRite::ArgentInsuffisant { prix, argent } => {
                write!(f, "il faut {prix} pièces, le joueur en a {argent}")
            }
            ErreurRite::AucunRite => write!(f, "aucun rite en cours"),
            ErreurRite::PasEncoreFini(_) => write!(f, "le rite n'est pas fini"),
            ErreurRite::Collection(erreur) => write!(f, "{erreur}"),
        }
    }
}

impl std::error::Error for ErreurRite {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalogue::Catalogue;
    use crate::contenu::ContenuV1;
    use crate::espece::{EspeceId, Rarete};

    #[test]
    fn la_forme_de_base_est_celle_qu_aucune_ne_precede() {
        let contenu = ContenuV1::new();
        let especes = contenu.especes();
        let base = |id: &str| {
            let espece = contenu.espece(&EspeceId::new(id)).unwrap();
            forme_de_base(&espece, especes).id().as_str().to_string()
        };
        assert_eq!(base("salamandre"), "salamandre");
        assert_eq!(base("lezard-de-braise"), "salamandre");
        assert_eq!(base("loup"), "loup");
    }

    #[test]
    fn chaque_forme_de_base_capturable_nait_au_niveau_1() {
        let contenu = ContenuV1::new();
        let niveau = Niveau::new(NIVEAU_DU_PETIT).unwrap();
        for espece in contenu.especes() {
            if matches!(espece.rarete(), Rarete::Apotre | Rarete::Demiurge) {
                continue;
            }
            let base = forme_de_base(espece, contenu.especes());
            assert!(
                Monstre::creer(base.clone(), niveau, Default::default(), false).is_ok(),
                "{}",
                base.nom()
            );
        }
    }
}
