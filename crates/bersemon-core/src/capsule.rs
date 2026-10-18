//! Capsules d'attaque et attaques apprises au camp (section 13.10).

use std::fmt;

use crate::attaque::{Attaque, AttaqueId, ErreurAttaquesMonstre};
use crate::collection::ErreurCollection;
use crate::espece::Espece;
use crate::monstre::Monstre;

/// Enseigne son attaque à tout monstre qui a le type de cette attaque. Elle
/// n'est pas consommée : une fois obtenue, elle sert autant qu'on veut.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capsule {
    attaque: Attaque,
    /// Chez le marchand, en pièces d'argent (section 14.5).
    prix: u32,
}

impl Capsule {
    pub fn new(attaque: Attaque, prix: u32) -> Self {
        Self { attaque, prix }
    }

    /// Une capsule se désigne par l'attaque qu'elle enseigne.
    pub fn id(&self) -> &AttaqueId {
        self.attaque.id()
    }

    pub fn attaque(&self) -> &Attaque {
        &self.attaque
    }

    pub fn prix(&self) -> u32 {
        self.prix
    }

    /// L'espèce a le type de l'attaque : ses monstres peuvent l'apprendre,
    /// sauf ceux qui la connaissent déjà.
    pub fn convient_a(&self, espece: &Espece) -> bool {
        espece
            .types()
            .liste()
            .contains(&self.attaque.type_attaque())
    }
}

/// Attaques qu'un monstre peut apprendre au camp et qu'il ne connaît pas :
/// celles de son espèce jusqu'à son niveau, y compris celles qu'il a
/// oubliées ou refusées, puis celles des capsules de son type, chacune une
/// seule fois.
pub fn attaques_a_apprendre<'c>(
    monstre: &Monstre,
    capsules: impl IntoIterator<Item = &'c Capsule>,
) -> Vec<Attaque> {
    let mut attaques: Vec<Attaque> = Vec::new();
    let mut ajouter = |attaque: &Attaque| {
        let deja_vue = attaques.iter().any(|vue| vue.id() == attaque.id());
        if !deja_vue && !monstre.attaques().connait(attaque.id()) {
            attaques.push(attaque.clone());
        }
    };
    for apprise in monstre.espece().apprentissage() {
        if apprise.niveau <= monstre.niveau() {
            ajouter(&apprise.attaque);
        }
    }
    for capsule in capsules {
        if capsule.convient_a(monstre.espece()) {
            ajouter(capsule.attaque());
        }
    }
    attaques
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurApprentissage {
    Collection(ErreurCollection),
    /// Ni son espèce jusqu'à son niveau, ni une capsule possédée de son type
    /// ne la lui enseigne, ou il la connaît déjà.
    AttaqueInaccessible,
    /// Il connaît déjà 4 attaques : il doit en oublier une (section 13.10).
    OubliManquant,
    /// Il lui reste de la place : il n'a rien à oublier.
    OubliInutile,
    Attaque(ErreurAttaquesMonstre),
}

impl fmt::Display for ErreurApprentissage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurApprentissage::Collection(erreur) => write!(f, "{erreur}"),
            ErreurApprentissage::AttaqueInaccessible => {
                write!(f, "ce monstre ne peut pas apprendre cette attaque au camp")
            }
            ErreurApprentissage::OubliManquant => {
                write!(
                    f,
                    "le monstre connaît déjà 4 attaques : il doit en oublier une"
                )
            }
            ErreurApprentissage::OubliInutile => {
                write!(f, "le monstre a encore de la place : il n'oublie rien")
            }
            ErreurApprentissage::Attaque(erreur) => write!(f, "{erreur}"),
        }
    }
}

impl std::error::Error for ErreurApprentissage {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attaque::{AttaquesMonstre, Categorie};
    use crate::donnees_de_test::{attaque, stats_uniformes};
    use crate::espece::{AttaqueApprise, Espece, EspeceId, Role};
    use crate::niveau::Niveau;
    use crate::stats::ValeursInternes;
    use crate::types::{Type, TypesMonstre};

    fn feu(id: &str) -> Attaque {
        attaque(id, Type::Feu, Categorie::Speciale, 40, 100)
    }

    /// Espèce Feu qui apprend « braise » au niveau 1, « flamme » au 3 et
    /// « fournaise » au 9, créée au niveau 5 : elle connaît braise et
    /// flamme.
    fn salamandre() -> Monstre {
        let apprise = |niveau: u8, attaque: Attaque| AttaqueApprise {
            niveau: Niveau::new(niveau).unwrap(),
            attaque,
        };
        let espece = Espece::new(
            EspeceId::new("salamandre"),
            "Salamandre",
            TypesMonstre::new(&[Type::Feu]).unwrap(),
            Role::Offensif,
            stats_uniformes(50),
            64,
            vec![
                apprise(1, feu("braise")),
                apprise(3, feu("flamme")),
                apprise(9, feu("fournaise")),
            ],
        );
        Monstre::creer(
            espece,
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    fn ids(attaques: &[Attaque]) -> Vec<&str> {
        attaques
            .iter()
            .map(|attaque| attaque.id().as_str())
            .collect()
    }

    #[test]
    fn une_capsule_convient_aux_monstres_de_son_type() {
        let salamandre = salamandre();
        assert!(Capsule::new(feu("brasier"), 150).convient_a(salamandre.espece()));
        let eau = Capsule::new(
            attaque("vague", Type::Eau, Categorie::Physique, 40, 100),
            150,
        );
        assert!(!eau.convient_a(salamandre.espece()));
    }

    #[test]
    fn attaques_oubliees_de_l_espece_puis_capsules_de_son_type() {
        let mut salamandre = salamandre();
        assert!(attaques_a_apprendre(&salamandre, &[]).is_empty());
        // Elle oublie braise pour une autre attaque : elle peut la
        // réapprendre, mais pas fournaise, apprise au niveau 9.
        salamandre.remplacer_attaque(0, feu("etincelle")).unwrap();
        let capsules = [
            Capsule::new(feu("brasier"), 150),
            Capsule::new(
                attaque("vague", Type::Eau, Categorie::Physique, 40, 100),
                150,
            ),
            Capsule::new(feu("etincelle"), 150),
            Capsule::new(feu("braise"), 150),
        ];
        assert_eq!(
            ids(&attaques_a_apprendre(&salamandre, &capsules)),
            vec!["braise", "brasier"]
        );
        assert_eq!(
            salamandre.attaques(),
            &AttaquesMonstre::new(vec![feu("etincelle"), feu("flamme")]).unwrap()
        );
    }
}
