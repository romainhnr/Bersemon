//! Textes des boosters et du service de Malvoise qui rend un monstre ou le
//! héros shiny (sections 4.1, 13.7 et 14.3, docs/contenu-v1.md, sections 17
//! et 43).

use bersemon_core::booster::{Booster, GAIN_D_UN_BOOSTER};
use bersemon_core::combat::Cote;
use bersemon_core::monstre::{Monstre, rayon_chromatique};
use bersemon_core::stats::{Stat, ValeursInternes};

use crate::mise_en_valeur::important;
use crate::monstres::nom_en_valeur;

/// Après l'âme d'un Apôtre vaincu (section 9.4).
pub const PHILTRE_RAMASSE: &str = "Dans la poussière de sa dépouille roule une fiole aux reflets changeants : un Philtre chromatique. Vous le ramassez.";

/// Le libellé du service, à l'étal de Malvoise, qui rend aussi le héros
/// shiny (section 13.7).
pub const SERVICE_SHINY: &str = "Rendre un monstre ou votre héros shiny (1 Philtre chromatique)";

/// À la création, 1 fois sur 512 (section 4.1).
pub const HEROS_NE_SHINY: &str =
    "Une lueur étrange émane de vous : vous êtes shiny, une chance sur 512.";

/// Seule la couleur du héros change (section 4.1).
pub const QUESTION_SHINY_DU_HEROS: &str = "Faire rendre votre héros shiny par Malvoise contre un Philtre chromatique ? Seule sa couleur changera.";

pub const HEROS_RENDU_SHINY: &str = "Malvoise vous fait boire le Philtre chromatique ; votre sang s'altère, et une lueur nouvelle vous enveloppe. Vous êtes désormais shiny.";

pub fn nom_de_stat(stat: Stat) -> &'static str {
    match stat {
        Stat::Pv => "PV",
        Stat::Attaque => "Attaque",
        Stat::Defense => "Défense",
        Stat::AttaqueSpeciale => "Attaque spéciale",
        Stat::DefenseSpeciale => "Défense spéciale",
        Stat::Vitesse => "Vitesse",
    }
}

/// "de PV", "d'Attaque".
fn de_la_stat(stat: Stat) -> String {
    let nom = nom_de_stat(stat);
    if nom.starts_with('A') {
        format!("d'{nom}")
    } else {
        format!("de {nom}")
    }
}

fn au_pluriel(booster: Booster) -> &'static str {
    match booster {
        Booster::FioleDeSang => "Fioles de sang",
        Booster::MoelleDeBete => "Moelles de bête",
        Booster::EcorceDeFer => "Écorces de fer",
        Booster::EncreDArcane => "Encres d'arcane",
        Booster::CendreBenite => "Cendres bénites",
        Booster::PoudreDeVifArgent => "Poudres de vif-argent",
    }
}

/// "1 Fiole de sang", "2 Fioles de sang".
pub fn boosters(booster: Booster, nombre: u32) -> String {
    if nombre > 1 {
        format!("{nombre} {}", au_pluriel(booster))
    } else {
        format!("{nombre} {}", booster.nom())
    }
}

/// "1 Philtre chromatique", "2 Philtres chromatiques".
pub fn philtres_chromatiques(nombre: u32) -> String {
    if nombre > 1 {
        format!("{nombre} Philtres chromatiques")
    } else {
        format!("{nombre} Philtre chromatique")
    }
}

/// "+5 à la valeur interne de PV, 31 au plus."
pub fn effet_du_booster(booster: Booster) -> String {
    format!(
        "+{GAIN_D_UN_BOOSTER} à la valeur interne {}, {} au plus.",
        de_la_stat(booster.stat()),
        ValeursInternes::MAX
    )
}

pub fn booster_donne(booster: Booster, avant: &Monstre, apres: &Monstre) -> String {
    let stat = booster.stat();
    format!(
        "{} : valeur interne {} de {} à {}.",
        nom_en_valeur(apres, Cote::Joueur),
        de_la_stat(stat),
        avant.valeurs_internes().valeurs().de(stat),
        apres.valeurs_internes().valeurs().de(stat)
    )
}

pub fn refus_valeur_maximale(booster: Booster) -> String {
    format!(
        "La valeur interne {} de ce monstre est déjà à {}.",
        de_la_stat(booster.stat()),
        ValeursInternes::MAX
    )
}

pub fn refus_sans_booster(booster: Booster) -> String {
    format!("Vous n'avez pas de {}.", booster.nom())
}

pub fn question_shiny(monstre: &Monstre) -> String {
    format!(
        "Faire rendre {} shiny par Malvoise contre un Philtre chromatique ? Ses valeurs internes passeront toutes à {}.",
        monstre.nom(),
        ValeursInternes::MAX
    )
}

/// Le nom seul : celui d'un shiny porte déjà la mention "(shiny)". Le
/// philtre altère la lignée du monstre, d'où sa couleur nouvelle (section
/// 13.7).
pub fn monstre_rendu_shiny(monstre: &Monstre) -> String {
    let nom = important(monstre.nom());
    // Sans place libre, la question de l'attaque à oublier suit (section
    // 5.1).
    let rayon = rayon_chromatique();
    let apprend = if monstre.attaques().connait(rayon.id()) {
        format!(" Il apprend {}.", rayon.nom())
    } else {
        String::new()
    };
    format!(
        "Malvoise fait boire le Philtre chromatique à {nom} ; sa lignée s'altère, et sa robe prend une couleur nouvelle. {nom} est désormais shiny.{apprend}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noms_et_effets() {
        assert_eq!(boosters(Booster::CendreBenite, 2), "2 Cendres bénites");
        assert_eq!(boosters(Booster::FioleDeSang, 1), "1 Fiole de sang");
        assert_eq!(philtres_chromatiques(3), "3 Philtres chromatiques");
        assert_eq!(
            effet_du_booster(Booster::FioleDeSang),
            "+5 à la valeur interne de PV, 31 au plus."
        );
        assert_eq!(
            effet_du_booster(Booster::EncreDArcane),
            "+5 à la valeur interne d'Attaque spéciale, 31 au plus."
        );
    }

    #[test]
    fn le_monstre_rendu_shiny_n_est_dit_shiny_qu_une_fois() {
        use bersemon_core::catalogue::Catalogue;
        use bersemon_core::contenu::ContenuV1;
        use bersemon_core::espece::EspeceId;
        use bersemon_core::niveau::Niveau;
        let contenu = ContenuV1::new();
        let loup = Monstre::creer(
            contenu.espece(&EspeceId::new("loup")).unwrap().clone(),
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            true,
        )
        .unwrap();
        let texte = monstre_rendu_shiny(&loup);
        assert_eq!(texte.matches("shiny").count(), 1, "{texte}");
        assert!(
            texte.ends_with("**Loup** est désormais shiny. Il apprend Rayon chromatique."),
            "{texte}"
        );
        // Sans place libre, le texte n'en dit rien : la question de l'attaque
        // à oublier suit (section 5.1).
        let sans_rayon = Monstre::creer(
            contenu.espece(&EspeceId::new("loup")).unwrap().clone(),
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        assert!(monstre_rendu_shiny(&sans_rayon).ends_with("est désormais shiny."));
    }

    /// Section 5.1 : la catégorie du Rayon chromatique suit les stats du
    /// monstre ; sans monstre, les deux sont dites.
    #[test]
    fn la_categorie_du_rayon_se_dit_pour_le_monstre() {
        use crate::monstres::{decrire_attaque, decrire_attaque_de};
        use bersemon_core::catalogue::Catalogue;
        use bersemon_core::contenu::ContenuV1;
        use bersemon_core::espece::EspeceId;
        use bersemon_core::niveau::Niveau;

        let contenu = ContenuV1::new();
        let creer = |id: &str| {
            Monstre::creer(
                contenu.espece(&EspeceId::new(id)).unwrap(),
                Niveau::new(30).unwrap(),
                ValeursInternes::default(),
                true,
            )
            .unwrap()
        };
        let rayon = rayon_chromatique();
        assert_eq!(
            decrire_attaque(&rayon),
            "Rayon chromatique (Stellaire, physique ou spéciale, puissance 80, précision 100 %)"
        );
        // Le Loup frappe en physique, la Salamandre en spécial.
        assert!(decrire_attaque_de(&creer("loup"), &rayon).contains("Stellaire, physique,"));
        assert!(decrire_attaque_de(&creer("salamandre"), &rayon).contains("Stellaire, spéciale,"));
    }
}
