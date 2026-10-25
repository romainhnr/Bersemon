//! Les textes de la capacité du sac (section 13.3, docs/contenu-v1.md,
//! section 33).

use bersemon_core::inventaire::{Inventaire, Objet};
use bersemon_core::partie::Partie;

use crate::boosters::{boosters, philtres_chromatiques};
use crate::filets::filets;
use crate::partie::{capsules_d_habilete, charmes_de_garde, contenu, pierres, ressources};
use crate::soins::soins;

/// "18/30" : les places prises et celles du sac.
pub fn places(partie: &Partie) -> String {
    format!(
        "{}/{}",
        partie.sac().places_prises(),
        partie.places_du_sac()
    )
}

/// À la fin d'une étape où le sac déborde.
pub fn question_du_sac_plein(partie: &Partie) -> String {
    let exces = partie.exces_du_sac();
    format!(
        "Votre sac déborde : {} objets pour {} places. Jetez-en {exces} pour poursuivre ; ce que vous jetez est perdu.",
        partie.sac().places_prises(),
        partie.places_du_sac()
    )
}

pub fn objets_jetes(jete: &Inventaire) -> String {
    format!("Vous jetez {}.", contenu(jete))
}

/// Une sorte d'objet du sac, à jeter : une unité, et combien il y en a.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SorteAJeter {
    /// L'objet, en un exemplaire.
    pub unite: Objet,
    pub nombre: u32,
    /// "3 potions", "Capsule Brasier".
    pub libelle: String,
}

impl SorteAJeter {
    /// L'objet en `nombre` exemplaires ; une capsule reste unique.
    pub fn objet(&self, nombre: u32) -> Objet {
        match &self.unite {
            Objet::Argent(_) => Objet::Argent(nombre),
            Objet::Soins(soin, _) => Objet::Soins(*soin, nombre),
            Objet::Capsule(id) => Objet::Capsule(id.clone()),
            Objet::CapsulesDHabilete(_) => Objet::CapsulesDHabilete(nombre),
            Objet::Ressources(id, _) => Objet::Ressources(id.clone(), nombre),
            Objet::Pierres(pierre, _) => Objet::Pierres(*pierre, nombre),
            Objet::Boosters(booster, _) => Objet::Boosters(*booster, nombre),
            Objet::PhiltresChromatiques(_) => Objet::PhiltresChromatiques(nombre),
            Objet::CharmesDeGarde(_) => Objet::CharmesDeGarde(nombre),
            Objet::Filets(filet, _) => Objet::Filets(*filet, nombre),
        }
    }
}

/// Tout ce qui prend une place dans le sac, sorte par sorte : l'argent n'y
/// est pas. Les ressources viennent en dernier.
pub fn sortes_a_jeter(sac: &Inventaire) -> Vec<SorteAJeter> {
    let mut sortes = Vec::new();
    let mut ajouter = |unite: Objet, nombre: u32, libelle: String| {
        if nombre > 0 {
            sortes.push(SorteAJeter {
                unite,
                nombre,
                libelle,
            });
        }
    };
    for (soin, nombre) in sac.soins().presents() {
        ajouter(Objet::Soins(soin, 1), nombre, soins(soin, nombre));
    }
    for (filet, nombre) in sac.filets().presents() {
        ajouter(Objet::Filets(filet, 1), nombre, filets(filet, nombre));
    }
    for capsule in sac.capsules() {
        ajouter(
            Objet::Capsule(capsule.id().clone()),
            1,
            format!("Capsule {}", capsule.attaque().nom()),
        );
    }
    let nombre = sac.capsules_d_habilete();
    ajouter(
        Objet::CapsulesDHabilete(1),
        nombre,
        capsules_d_habilete(nombre),
    );
    for &(pierre, nombre) in sac.pierres() {
        ajouter(Objet::Pierres(pierre, 1), nombre, pierres(pierre, nombre));
    }
    for &(booster, nombre) in sac.boosters() {
        ajouter(
            Objet::Boosters(booster, 1),
            nombre,
            boosters(booster, nombre),
        );
    }
    let nombre = sac.philtres_chromatiques();
    ajouter(
        Objet::PhiltresChromatiques(1),
        nombre,
        philtres_chromatiques(nombre),
    );
    let nombre = sac.charmes_de_garde();
    ajouter(Objet::CharmesDeGarde(1), nombre, charmes_de_garde(nombre));
    for (ressource, nombre) in sac.ressources() {
        ajouter(
            Objet::Ressources(ressource.id().clone(), 1),
            *nombre,
            ressources(ressource, *nombre),
        );
    }
    sortes
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::filet::Filet;
    use bersemon_core::soin::Soin;

    #[test]
    fn chaque_sorte_se_jette_sauf_l_argent() {
        let sac = Inventaire::new(50, 3, Vec::new()).avec_filets(Filet::Chanvre, 10);
        let sortes = sortes_a_jeter(&sac);
        assert_eq!(sortes.len(), 2);
        assert_eq!(sortes[0].unite, Objet::Soins(Soin::Potion, 1));
        assert_eq!(sortes[0].nombre, 3);
        assert_eq!(sortes[0].libelle, "3 potions");
        assert_eq!(sortes[1].objet(4), Objet::Filets(Filet::Chanvre, 4));
        assert_eq!(
            objets_jetes(&Inventaire::new(0, 2, Vec::new())),
            "Vous jetez 2 potions."
        );
    }
}
