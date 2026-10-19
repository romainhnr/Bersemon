//! Formulaires de texte : le nom du héros et le surnom d'un monstre.

use bersemon_core::nom::Nom;
use bersemon_core::partie::Partie;
use poise::serenity_prelude as serenity;

use crate::composants::{Cle, Composant, Formulaire};

/// Champ unique de chaque formulaire.
pub const CHAMP_TEXTE: &str = "texte";

pub fn formulaire(
    version: u32,
    formulaire: Formulaire,
    partie: Option<&Partie>,
) -> serenity::CreateModal {
    let longueur_max = u16::try_from(Nom::LONGUEUR_MAX).unwrap_or(u16::MAX);
    let (cle, titre, champ) = match formulaire {
        Formulaire::NomDuHeros => (
            Cle::NomSaisi,
            "Votre héros",
            serenity::CreateInputText::new(
                serenity::InputTextStyle::Short,
                "Nom du héros",
                CHAMP_TEXTE,
            )
            .min_length(1)
            .max_length(longueur_max),
        ),
        Formulaire::Surnom(emplacement) => {
            // Un surnom vide efface le surnom actuel, montré pour être modifié.
            let actuel = partie
                .and_then(|partie| partie.collection().monstre(emplacement))
                .and_then(|monstre| monstre.surnom())
                .map(|surnom| surnom.as_str().to_string());
            let champ = serenity::CreateInputText::new(
                serenity::InputTextStyle::Short,
                "Surnom (laisser vide pour l'effacer)",
                CHAMP_TEXTE,
            )
            .max_length(longueur_max)
            .required(false);
            let champ = match actuel {
                Some(surnom) => champ.value(surnom),
                None => champ,
            };
            (Cle::SurnomSaisi(emplacement), "Surnom", champ)
        }
    };
    serenity::CreateModal::new(Composant { version, cle }.identifiant(), titre)
        .components(vec![serenity::CreateActionRow::InputText(champ)])
}
