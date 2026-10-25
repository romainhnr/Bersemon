//! La Fosse aux damnés (section 9.8) : l'adversaire de chaque palier, les
//! victoires, les défaites et les récompenses.

use bersemon_core::arene::{AdversaireDeLArene, ErreurArene, PALIERS_DE_LA_FOSSE};
use bersemon_core::collection::Placement;
use bersemon_core::combat::Cote;
use bersemon_core::partie::RecompenseObtenue;

use crate::equipement::objet_et_bonus;
use crate::mise_en_valeur::important;
use crate::monstres::nom_en_valeur;
use crate::partie::{argent, enumerer};

/// Le lieu, la première fois qu'on y entre et devant chaque palier.
pub const LA_FOSSE: &str = "Une fosse creusée dans la roche, cernée de gradins où se pressent des ombres avides. Ici, les humains s'entretuent pour le plaisir des spectateurs, et le sable ne sèche jamais.";

/// "Fosse aux damnés (arène) : palier 3/10", ou "conquise".
pub fn lieu_sur_la_carte(paliers_vaincus: u8) -> String {
    if paliers_vaincus >= PALIERS_DE_LA_FOSSE {
        "Fosse aux damnés (arène) : conquise".to_string()
    } else {
        format!(
            "Fosse aux damnés (arène) : palier {}/{PALIERS_DE_LA_FOSSE}",
            paliers_vaincus + 1
        )
    }
}

/// "Arène ; prochain palier 3/10", ou "Arène ; conquise", dans le menu des
/// expéditions du bot.
pub fn description_de_la_fosse(paliers_vaincus: u8) -> String {
    if paliers_vaincus >= PALIERS_DE_LA_FOSSE {
        "Arène ; conquise".to_string()
    } else {
        format!(
            "Arène ; prochain palier {}/{PALIERS_DE_LA_FOSSE}",
            paliers_vaincus + 1
        )
    }
}

/// Le palier, l'adversaire et son texte, avant le combat.
pub fn annonce_du_palier(palier: u8, adversaire: &AdversaireDeLArene) -> String {
    let niveau = adversaire.generer_heros().niveau().valeur();
    format!(
        "{LA_FOSSE}\n\nPalier {palier} sur {PALIERS_DE_LA_FOSSE}. Face à vous : {}, {}, niveau {niveau}.\n{}",
        important(adversaire.nom()),
        adversaire.classe().nom(),
        adversaire.presentation()
    )
}

/// La victoire, la récompense, et la Fosse conquise après le dernier
/// palier.
pub fn palier_vaincu(
    palier: u8,
    adversaire: &str,
    recompense: &RecompenseObtenue,
    conquise: bool,
) -> String {
    let mut parties = vec![argent(recompense.argent)];
    if let Some((monstre, placement)) = &recompense.monstre {
        let ou = match placement {
            Placement::Equipe(_) => "qui rejoint l'équipe",
            Placement::Camp(_) => "qui rejoint le camp",
            Placement::PlaceManquante(_) => "qui ne trouve pas de place",
        };
        parties.push(format!("{} {ou}", nom_en_valeur(monstre, Cote::Joueur)));
    }
    if let Some(arme) = &recompense.arme {
        parties.push(format!("{}, dans votre équipement", objet_et_bonus(arme)));
    }
    let mut texte = format!(
        "{}\nRécompense du palier {palier} : {}. L'argent va au coffre du camp.",
        important(&format!(
            "{adversaire} tombe sur le sable. La foule hurle votre nom : palier {palier} vaincu."
        )),
        enumerer(&parties)
    );
    if conquise {
        texte.push('\n');
        texte.push_str(&important(
            "Vous avez vaincu le Champion : la Fosse aux damnés n'a plus d'adversaire à vous opposer.",
        ));
    }
    texte
}

/// Après la chute du héros ou son abandon, que le combat a racontés : rien
/// n'est perdu.
pub const DEFAITE_DANS_LA_FOSSE: &str =
    "Les huées vous raccompagnent hors de la Fosse. Vous pourrez revenir quand vous voudrez.";

/// Dans l'arène, l'abandon de ce côté.
pub fn abandon_dans_l_arene(cote: Cote, adversaire: &str) -> String {
    match cote {
        Cote::Joueur => {
            "Vous jetez votre arme sur le sable : la foule vous couvre d'insultes.".to_string()
        }
        Cote::Adversaire => format!("{adversaire} jette son arme sur le sable."),
    }
}

pub fn erreur_arene(erreur: ErreurArene) -> String {
    match erreur {
        ErreurArene::FosseFermee => {
            "La Fosse aux damnés n'ouvre ses portes qu'à qui a vaincu le brigand de la Lisière."
                .to_string()
        }
        ErreurArene::FosseConquise => {
            "Vous avez vaincu tous les adversaires de la Fosse aux damnés.".to_string()
        }
        ErreurArene::PlusDePlace => {
            "Ce palier offre un monstre : faites-lui de la place dans l'équipe ou au camp avant de combattre."
                .to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::contenu::ContenuV1;

    #[test]
    fn l_adversaire_est_presente_avec_son_niveau() {
        let contenu = ContenuV1::new();
        let gueux = contenu.fosse().adversaire(1).unwrap();
        let texte = annonce_du_palier(1, gueux);
        assert!(
            texte.contains("Palier 1 sur 10. Face à vous : **Le Gueux**, Guerrier, niveau 10."),
            "{texte}"
        );
        assert_eq!(
            lieu_sur_la_carte(2),
            "Fosse aux damnés (arène) : palier 3/10"
        );
        assert_eq!(lieu_sur_la_carte(10), "Fosse aux damnés (arène) : conquise");
    }

    #[test]
    fn la_victoire_dit_la_recompense() {
        let recompense = RecompenseObtenue {
            argent: 40,
            monstre: None,
            arme: None,
        };
        assert_eq!(
            palier_vaincu(1, "Le Gueux", &recompense, false),
            "**Le Gueux tombe sur le sable. La foule hurle votre nom : palier 1 vaincu.**\nRécompense du palier 1 : 40 pièces d'argent. L'argent va au coffre du camp."
        );
    }
}
