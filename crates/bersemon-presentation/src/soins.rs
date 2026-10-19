//! Les soins (section 14.4) : leur nom, leur effet et leur usage.

use bersemon_core::marchand::prix_du_soin;
use bersemon_core::soin::{EffetDuSoin, Soin, Soins};
use bersemon_core::statut::Statut;

use crate::partie::argent;

/// Nom d'un bouton ou d'un menu.
pub fn nom_du_soin(soin: Soin) -> &'static str {
    match soin {
        Soin::Potion => "Potion",
        Soin::Remede => "Remède",
        Soin::GrandePotion => "Grande potion",
        Soin::Elixir => "Élixir",
    }
}

/// "1 remède", "3 grandes potions".
pub fn soins(soin: Soin, nombre: u32) -> String {
    let (singulier, pluriel) = match soin {
        Soin::Potion => ("potion", "potions"),
        Soin::Remede => ("remède", "remèdes"),
        Soin::GrandePotion => ("grande potion", "grandes potions"),
        Soin::Elixir => ("élixir", "élixirs"),
    };
    if nombre > 1 {
        format!("{nombre} {pluriel}")
    } else {
        format!("{nombre} {singulier}")
    }
}

/// En début de phrase.
fn le_soin(soin: Soin) -> &'static str {
    match soin {
        Soin::Potion => "La potion",
        Soin::Remede => "Le remède",
        Soin::GrandePotion => "La grande potion",
        Soin::Elixir => "L'élixir",
    }
}

pub fn effet_du_soin(soin: Soin) -> &'static str {
    match soin {
        Soin::Potion => "rend de 40 à 60 % des PV max",
        Soin::Remede => "guérit le poison ou la paralysie",
        Soin::GrandePotion => "rend de 80 à 100 % des PV max",
        Soin::Elixir => "relève un monstre KO avec la moitié de ses PV max",
    }
}

/// Chaque soin présent, avec son nombre, dans l'ordre de `Soin::TOUS`.
pub fn liste_de_soins(soins: &Soins) -> Vec<String> {
    soins
        .presents()
        .map(|(soin, nombre)| self::soins(soin, nombre))
        .collect()
}

/// Le soin, son effet et son prix, à l'étal du marchand.
pub fn offre_de_soin(soin: Soin) -> String {
    format!(
        "{} ({}) : {}",
        nom_du_soin(soin),
        effet_du_soin(soin),
        argent(prix_du_soin(soin))
    )
}

pub fn plus_aucun(soin: Soin) -> &'static str {
    match soin {
        Soin::Potion => "Vous n'avez plus de potion.",
        Soin::Remede => "Vous n'avez plus de remède.",
        Soin::GrandePotion => "Vous n'avez plus de grande potion.",
        Soin::Elixir => "Vous n'avez plus d'élixir.",
    }
}

/// `nom` est celui du monstre soigné, déjà mis en forme.
pub fn soin_utilise(soin: Soin, nom: &str, effet: EffetDuSoin) -> String {
    match effet {
        EffetDuSoin::PvRendus(0) | EffetDuSoin::StatutGueri(None) => {
            format!("{} est sans effet sur {nom}.", le_soin(soin))
        }
        EffetDuSoin::PvRendus(pv) if soin == Soin::Elixir => {
            format!("L'élixir relève {nom} avec {pv} PV.")
        }
        EffetDuSoin::PvRendus(pv) => format!("{nom} récupère {pv} PV."),
        EffetDuSoin::StatutGueri(Some(Statut::Poison)) => {
            format!("Le remède guérit {nom} du poison.")
        }
        EffetDuSoin::StatutGueri(Some(Statut::Paralysie)) => {
            format!("Le remède guérit {nom} de la paralysie.")
        }
    }
}

/// `nom` est celui du monstre visé, déjà mis en forme.
pub fn question_du_soin_perdu(soin: Soin, nom: &str) -> String {
    let raison = match soin {
        Soin::Potion | Soin::GrandePotion => format!("{nom} a déjà tous ses PV"),
        Soin::Remede => format!("{nom} n'a ni poison ni paralysie"),
        Soin::Elixir => format!("{nom} n'est pas KO"),
    };
    format!(
        "{raison} : {} serait perdu{}. L'utiliser quand même ?",
        le_soin(soin).to_lowercase(),
        if matches!(soin, Soin::Potion | Soin::GrandePotion) {
            "e"
        } else {
            ""
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noms_au_singulier_et_au_pluriel() {
        assert_eq!(soins(Soin::GrandePotion, 3), "3 grandes potions");
        assert_eq!(soins(Soin::Elixir, 1), "1 élixir");
        let mut sac = Soins::default();
        sac.ajouter(Soin::Elixir, 2);
        sac.ajouter(Soin::Potion, 1);
        assert_eq!(liste_de_soins(&sac), vec!["1 potion", "2 élixirs"]);
    }

    #[test]
    fn chaque_effet_se_raconte() {
        assert_eq!(
            soin_utilise(Soin::Remede, "Loup", EffetDuSoin::StatutGueri(None)),
            "Le remède est sans effet sur Loup."
        );
        assert_eq!(
            soin_utilise(Soin::Elixir, "Loup", EffetDuSoin::PvRendus(12)),
            "L'élixir relève Loup avec 12 PV."
        );
        assert_eq!(
            soin_utilise(Soin::GrandePotion, "Loup", EffetDuSoin::PvRendus(12)),
            "Loup récupère 12 PV."
        );
        assert_eq!(
            soin_utilise(
                Soin::Remede,
                "Loup",
                EffetDuSoin::StatutGueri(Some(Statut::Paralysie))
            ),
            "Le remède guérit Loup de la paralysie."
        );
    }

    #[test]
    fn un_soin_sans_effet_demande_confirmation() {
        assert_eq!(
            question_du_soin_perdu(Soin::Potion, "Loup"),
            "Loup a déjà tous ses PV : la potion serait perdue. L'utiliser quand même ?"
        );
        assert_eq!(
            question_du_soin_perdu(Soin::Elixir, "Loup"),
            "Loup n'est pas KO : l'élixir serait perdu. L'utiliser quand même ?"
        );
        assert_eq!(
            offre_de_soin(Soin::Remede),
            "Remède (guérit le poison ou la paralysie) : 25 pièces d'argent"
        );
    }
}
