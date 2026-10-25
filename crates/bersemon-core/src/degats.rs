//! Résolution d'une attaque : précision, coup critique et dégâts (section
//! 10.4).
//!
//! Aucun tirage n'est fait quand son issue est certaine (cible immunisée,
//! précision de 100 %) : les tests n'ont ainsi à imposer que les tirages qui
//! comptent.

use crate::aleatoire::Aleatoire;
use crate::attaque::{Attaque, Categorie};
use crate::habilete::{BONUS_D_AFFINITE, Habilete};
use crate::monstre::Monstre;
use crate::stats::Stats;
use crate::types::{Efficacite, efficacite};

/// Un coup critique sur 24 (Q21).
const CHANCES_CRITIQUE: u32 = 24;
/// Fourchette du pourcentage de dégâts (Q20).
const POURCENTAGE_MIN: u32 = 85;
const POURCENTAGE_MAX: u32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultatAttaque {
    Ratee,
    SansEffet,
    Touchee {
        degats: u32,
        critique: bool,
        efficacite: Efficacite,
    },
}

/// Les stats sont celles du combat, bonus de classe compris (section 4.2).
/// `renforts` : les pourcentages de dégâts en plus que le moment du jour,
/// puis la météo, donnent au type de l'attaque (sections 9.6 et 9.7).
/// Tirages, dans l'ordre : précision (seulement si elle est inférieure à
/// 100 %), coup critique, pourcentage de dégâts.
pub(crate) fn resoudre_attaque(
    attaquant: &Monstre,
    stats_attaquant: &Stats<u32>,
    attaque: &Attaque,
    defenseur: &Monstre,
    stats_defenseur: &Stats<u32>,
    renforts: &[u64],
    aleatoire: &mut dyn Aleatoire,
) -> ResultatAttaque {
    let efficacite = efficacite(attaque.type_attaque(), defenseur.espece().types());
    // Comme dans Pokémon, une cible immunisée est signalée comme telle plutôt
    // que par une attaque ratée.
    if efficacite == Efficacite::Nulle {
        return ResultatAttaque::SansEffet;
    }
    if attaque.precision() < Attaque::PRECISION_MAX
        && aleatoire.tirer(1..=100) > u32::from(attaque.precision())
    {
        return ResultatAttaque::Ratee;
    }
    let critique = aleatoire.tirer(1..=CHANCES_CRITIQUE) == 1;
    let pourcentage = aleatoire.tirer(POURCENTAGE_MIN..=POURCENTAGE_MAX);
    let degats = calculer_degats(&parametres(
        attaquant,
        stats_attaquant,
        attaque,
        stats_defenseur,
        renforts,
        critique,
        pourcentage,
        efficacite,
    ));
    ResultatAttaque::Touchee {
        degats,
        critique,
        efficacite,
    }
}

/// Dégâts d'une attaque qui touche sans coup critique, du pourcentage
/// minimal au maximal : ce que le joueur voit avant de choisir (section
/// 10.6). (0, 0) contre une cible immunisée.
pub(crate) fn estimer_degats(
    attaquant: &Monstre,
    stats_attaquant: &Stats<u32>,
    attaque: &Attaque,
    defenseur: &Monstre,
    stats_defenseur: &Stats<u32>,
    renforts: &[u64],
) -> (u32, u32) {
    let efficacite = efficacite(attaque.type_attaque(), defenseur.espece().types());
    if efficacite == Efficacite::Nulle {
        return (0, 0);
    }
    let avec = |pourcentage| {
        calculer_degats(&parametres(
            attaquant,
            stats_attaquant,
            attaque,
            stats_defenseur,
            renforts,
            false,
            pourcentage,
            efficacite,
        ))
    };
    (avec(POURCENTAGE_MIN), avec(POURCENTAGE_MAX))
}

/// L'attaque a l'un des types du monstre qui l'utilise : dégâts x1,5
/// (section 10.4).
pub(crate) fn a_le_bonus_de_type(attaquant: &Monstre, attaque: &Attaque) -> bool {
    attaquant
        .espece()
        .types()
        .liste()
        .contains(&attaque.type_attaque())
}

#[allow(clippy::too_many_arguments)]
fn parametres<'a>(
    attaquant: &Monstre,
    stats_attaquant: &Stats<u32>,
    attaque: &Attaque,
    stats_defenseur: &Stats<u32>,
    renforts: &'a [u64],
    critique: bool,
    pourcentage: u32,
    efficacite: Efficacite,
) -> ParametresDegats<'a> {
    let (stat_attaque, stat_defense) = match attaque.categorie() {
        Categorie::Physique => (stats_attaquant.attaque, stats_defenseur.defense),
        Categorie::Speciale => (
            stats_attaquant.attaque_speciale,
            stats_defenseur.defense_speciale,
        ),
    };
    let bonus_de_type = a_le_bonus_de_type(attaquant, attaque);
    ParametresDegats {
        niveau: attaquant.niveau().valeur(),
        puissance: attaque.puissance(),
        stat_attaque,
        stat_defense,
        critique,
        pourcentage,
        bonus_de_type,
        affinite: bonus_de_type && attaquant.a_l_habilete(Habilete::Affinite),
        renforts,
        efficacite,
    }
}

/// Tout ce dont dépend la formule, une fois les tirages faits.
struct ParametresDegats<'a> {
    niveau: u8,
    puissance: u16,
    stat_attaque: u32,
    stat_defense: u32,
    critique: bool,
    pourcentage: u32,
    bonus_de_type: bool,
    /// L'habileté Affinité ne renforce que les attaques qui ont le bonus de
    /// type (section 5.9).
    affinite: bool,
    /// Le moment du jour, puis la météo, renforcent chacun quelques types,
    /// en se multipliant (sections 9.6 et 9.7).
    renforts: &'a [u64],
    efficacite: Efficacite,
}

/// Formule de Pokémon (Q20), en divisions entières de gauche à droite, avec
/// arrondi à l'entier inférieur après chaque modificateur.
fn calculer_degats(parametres: &ParametresDegats<'_>) -> u32 {
    // Calcul en u64 : puissance x Attaque peut dépasser u32 avec des valeurs
    // extrêmes. La Défense vaut au moins 5 par construction des stats, d'où
    // l'absence de division par zéro.
    let niveau = u64::from(parametres.niveau);
    let mut degats =
        (2 * niveau / 5 + 2) * u64::from(parametres.puissance) * u64::from(parametres.stat_attaque)
            / u64::from(parametres.stat_defense)
            / 50
            + 2;
    if parametres.critique {
        degats = degats * 3 / 2;
    }
    degats = degats * u64::from(parametres.pourcentage) / 100;
    if parametres.bonus_de_type {
        degats = degats * 3 / 2;
    }
    if parametres.affinite {
        degats = degats * (100 + BONUS_D_AFFINITE) / 100;
    }
    for renfort in parametres.renforts {
        degats = degats * (100 + renfort) / 100;
    }
    degats = match parametres.efficacite {
        Efficacite::Nulle => return 0,
        Efficacite::TresReduite => degats / 4,
        Efficacite::Reduite => degats / 2,
        Efficacite::Normale => degats,
        Efficacite::Forte => degats * 2,
        Efficacite::TresForte => degats * 4,
    };
    // Une attaque qui n'est pas sans effet inflige toujours au moins 1 dégât.
    u32::try_from(degats.max(1)).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::attaque::Attaque;
    use crate::donnees_de_test::{attaque, monstre, stats_uniformes};
    use crate::stats::Stats;
    use crate::types::Type;

    fn parametres(
        niveau: u8,
        puissance: u16,
        stat_attaque: u32,
        stat_defense: u32,
    ) -> ParametresDegats<'static> {
        ParametresDegats {
            niveau,
            puissance,
            stat_attaque,
            stat_defense,
            critique: false,
            pourcentage: 100,
            bonus_de_type: false,
            affinite: false,
            renforts: &[],
            efficacite: Efficacite::Normale,
        }
    }

    #[test]
    fn affinite_apres_le_bonus_de_type() {
        let mut p = parametres(5, 40, 10, 10);
        p.bonus_de_type = true;
        assert_eq!(calculer_degats(&p), 7);
        p.affinite = true;
        assert_eq!(calculer_degats(&p), 8);
    }

    #[test]
    fn affinite_seulement_pour_les_attaques_de_son_type() {
        let braise = attaque("braise", Type::Feu, Categorie::Speciale, 40, 100);
        let coup = attaque("coup", Type::Normal, Categorie::Physique, 40, 100);
        let attaquant = monstre(
            &[Type::Feu],
            stats_uniformes(50),
            5,
            vec![braise.clone(), coup.clone()],
        )
        .avec_habilete(Habilete::Affinite);
        let defenseur = monstre(&[Type::Normal], stats_uniformes(50), 5, vec![coup.clone()]);
        let (stats, stats_du_defenseur) = (attaquant.stats(), defenseur.stats());
        // Braise : 4 à 5, x1,5 pour le bonus de type, puis x1,2.
        let estimer = |attaque: &Attaque| {
            estimer_degats(
                &attaquant,
                &stats,
                attaque,
                &defenseur,
                &stats_du_defenseur,
                &[],
            )
        };
        assert_eq!(estimer(&braise), (7, 8));
        assert_eq!(estimer(&coup), (4, 5));
    }

    #[test]
    fn les_renforts_suivent_l_affinite_et_precedent_l_efficacite() {
        let mut p = parametres(5, 40, 10, 10);
        p.bonus_de_type = true;
        p.affinite = true;
        assert_eq!(calculer_degats(&p), 8);
        p.renforts = &[20];
        // 8 x 1,2 = 9,6, arrondi à 9, puis doublé par l'efficacité.
        assert_eq!(calculer_degats(&p), 9);
        p.efficacite = Efficacite::Forte;
        assert_eq!(calculer_degats(&p), 18);
    }

    /// Le moment du jour et la météo se multiplient : x1,2 puis x1,2, en
    /// divisions entières (section 9.7).
    #[test]
    fn deux_renforts_se_multiplient() {
        let mut p = parametres(50, 100, 100, 100);
        let sans = calculer_degats(&p);
        p.renforts = &[20, 20];
        assert_eq!(calculer_degats(&p), sans * 6 / 5 * 6 / 5);
        assert!(calculer_degats(&p) > sans * 7 / 5);
    }

    #[test]
    fn exemple_niveau_5_puissance_40() {
        // Exemple consigné avec la décision Q20.
        let mut p = parametres(5, 40, 10, 10);
        assert_eq!(calculer_degats(&p), 5);
        p.pourcentage = 85;
        assert_eq!(calculer_degats(&p), 4);
    }

    #[test]
    fn exemple_de_reference_de_pokemon() {
        // Exemple classique de la documentation de Pokémon : Givrali niveau 75
        // (Attaque 123) utilise Crocs Givre (puissance 65) contre Carchacrok
        // (Défense 163), avec bonus de type et efficacité x4 : 168 à 196.
        let mut p = parametres(75, 65, 123, 163);
        p.bonus_de_type = true;
        p.efficacite = Efficacite::TresForte;
        p.pourcentage = 85;
        assert_eq!(calculer_degats(&p), 168);
        p.pourcentage = 100;
        assert_eq!(calculer_degats(&p), 196);
    }

    #[test]
    fn coup_critique_multiplie_par_1_5() {
        let mut p = parametres(5, 40, 10, 10);
        p.critique = true;
        assert_eq!(calculer_degats(&p), 7);
    }

    #[test]
    fn efficacite_appliquee_en_dernier() {
        let mut p = parametres(5, 40, 10, 10);
        p.efficacite = Efficacite::Forte;
        assert_eq!(calculer_degats(&p), 10);
        p.efficacite = Efficacite::Reduite;
        assert_eq!(calculer_degats(&p), 2);
        p.efficacite = Efficacite::Nulle;
        assert_eq!(calculer_degats(&p), 0);
    }

    #[test]
    fn au_moins_un_degat_si_l_attaque_a_un_effet() {
        let mut p = parametres(1, 1, 5, 1000);
        p.pourcentage = 85;
        p.efficacite = Efficacite::TresReduite;
        assert_eq!(calculer_degats(&p), 1);
    }

    #[test]
    fn valeurs_extremes_sans_debordement() {
        let mut p = parametres(100, u16::MAX, u32::MAX, 5);
        p.critique = true;
        p.bonus_de_type = true;
        p.efficacite = Efficacite::TresForte;
        assert_eq!(calculer_degats(&p), u32::MAX);
    }

    fn coup_de_test(precision: u8) -> Attaque {
        attaque("coup", Type::Normal, Categorie::Physique, 40, precision)
    }

    /// Résolution avec les stats propres des monstres, sans bonus.
    fn resoudre(
        attaquant: &Monstre,
        attaque: &Attaque,
        defenseur: &Monstre,
        aleatoire: &mut dyn Aleatoire,
    ) -> ResultatAttaque {
        resoudre_attaque(
            attaquant,
            &attaquant.stats(),
            attaque,
            defenseur,
            &defenseur.stats(),
            &[],
            aleatoire,
        )
    }

    fn feu_niveau_5() -> Monstre {
        monstre(
            &[Type::Feu],
            stats_uniformes(50),
            5,
            vec![coup_de_test(100)],
        )
    }

    #[test]
    fn cible_immunisee_sans_aucun_tirage() {
        let attaquant = feu_niveau_5();
        let spectre = monstre(
            &[Type::Spectre],
            stats_uniformes(50),
            5,
            vec![coup_de_test(100)],
        );
        let mut aleatoire = AleatoireScripte::new([]);
        assert_eq!(
            resoudre(&attaquant, &coup_de_test(50), &spectre, &mut aleatoire),
            ResultatAttaque::SansEffet
        );
    }

    #[test]
    fn precision_de_100_ne_tire_pas() {
        let (attaquant, defenseur) = (feu_niveau_5(), feu_niveau_5());
        let mut aleatoire = AleatoireScripte::new([2, 100]);
        assert_eq!(
            resoudre(&attaquant, &coup_de_test(100), &defenseur, &mut aleatoire),
            ResultatAttaque::Touchee {
                degats: 5,
                critique: false,
                efficacite: Efficacite::Normale,
            }
        );
    }

    #[test]
    fn attaque_ratee_au_dessus_de_la_precision() {
        let (attaquant, defenseur) = (feu_niveau_5(), feu_niveau_5());
        let mut aleatoire = AleatoireScripte::new([91]);
        assert_eq!(
            resoudre(&attaquant, &coup_de_test(90), &defenseur, &mut aleatoire),
            ResultatAttaque::Ratee
        );
    }

    #[test]
    fn attaque_touche_a_la_limite_de_la_precision() {
        let (attaquant, defenseur) = (feu_niveau_5(), feu_niveau_5());
        let mut aleatoire = AleatoireScripte::new([90, 2, 85]);
        assert_eq!(
            resoudre(&attaquant, &coup_de_test(90), &defenseur, &mut aleatoire),
            ResultatAttaque::Touchee {
                degats: 4,
                critique: false,
                efficacite: Efficacite::Normale,
            }
        );
    }

    #[test]
    fn coup_critique_sur_un_tirage_de_1() {
        let (attaquant, defenseur) = (feu_niveau_5(), feu_niveau_5());
        let mut aleatoire = AleatoireScripte::new([1, 100]);
        assert_eq!(
            resoudre(&attaquant, &coup_de_test(100), &defenseur, &mut aleatoire),
            ResultatAttaque::Touchee {
                degats: 7,
                critique: true,
                efficacite: Efficacite::Normale,
            }
        );
    }

    #[test]
    fn categorie_choisit_les_stats_et_bonus_de_type() {
        // Attaque 105 et Attaque spéciale 25 au niveau 50 ; Défense et
        // Défense spéciale 55 pour la cible.
        let base_attaquant = Stats {
            attaque: 100,
            attaque_speciale: 20,
            ..stats_uniformes(50)
        };
        let defenseur = monstre(
            &[Type::Feu],
            stats_uniformes(50),
            50,
            vec![coup_de_test(100)],
        );
        let feu = monstre(&[Type::Feu], base_attaquant, 50, vec![coup_de_test(100)]);
        let normal = monstre(&[Type::Normal], base_attaquant, 50, vec![coup_de_test(100)]);
        let physique = attaque("physique", Type::Normal, Categorie::Physique, 40, 100);
        let speciale = attaque("speciale", Type::Normal, Categorie::Speciale, 40, 100);

        let degats = |attaquant: &Monstre, attaque: &Attaque| {
            let mut aleatoire = AleatoireScripte::new([2, 100]);
            match resoudre(attaquant, attaque, &defenseur, &mut aleatoire) {
                ResultatAttaque::Touchee { degats, .. } => degats,
                autre => panic!("attaque censée toucher : {autre:?}"),
            }
        };
        assert_eq!(degats(&feu, &physique), 35);
        assert_eq!(degats(&feu, &speciale), 10);
        assert_eq!(degats(&normal, &physique), 52);
    }
}
