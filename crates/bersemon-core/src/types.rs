//! Types des monstres et des attaques, et table d'efficacité (section 5.2).

use std::fmt;

/// Types de la V1, puis le Stellaire, celui du seul Schnoz, l'Obscur, celui
/// des seuls Apôtres, la Lumière et le Son, ajoutés à l'étape 93, et
/// l'Humain, celui des seuls héros, ajouté à l'étape 128.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Normal,
    Combat,
    Vol,
    Poison,
    Sol,
    Roche,
    Insecte,
    Spectre,
    Acier,
    Feu,
    Eau,
    Plante,
    Electrique,
    Psychique,
    Glace,
    Dragon,
    Tenebres,
    Feerique,
    Stellaire,
    Obscur,
    Lumiere,
    Son,
    Humain,
}

impl Type {
    pub const TOUS: [Type; 23] = [
        Type::Normal,
        Type::Combat,
        Type::Vol,
        Type::Poison,
        Type::Sol,
        Type::Roche,
        Type::Insecte,
        Type::Spectre,
        Type::Acier,
        Type::Feu,
        Type::Eau,
        Type::Plante,
        Type::Electrique,
        Type::Psychique,
        Type::Glace,
        Type::Dragon,
        Type::Tenebres,
        Type::Feerique,
        Type::Stellaire,
        Type::Obscur,
        Type::Lumiere,
        Type::Son,
        Type::Humain,
    ];

    /// Nom du type tel qu'il s'écrit dans le jeu, commun à toutes les
    /// interfaces.
    pub fn nom(self) -> &'static str {
        match self {
            Type::Normal => "Normal",
            Type::Combat => "Combat",
            Type::Vol => "Vol",
            Type::Poison => "Poison",
            Type::Sol => "Sol",
            Type::Roche => "Roche",
            Type::Insecte => "Insecte",
            Type::Spectre => "Spectre",
            Type::Acier => "Acier",
            Type::Feu => "Feu",
            Type::Eau => "Eau",
            Type::Plante => "Plante",
            Type::Electrique => "Électrique",
            Type::Psychique => "Psychique",
            Type::Glace => "Glace",
            Type::Dragon => "Dragon",
            Type::Tenebres => "Ténèbres",
            Type::Feerique => "Féérique",
            Type::Stellaire => "Stellaire",
            Type::Obscur => "Obscur",
            Type::Lumiere => "Lumière",
            Type::Son => "Son",
            Type::Humain => "Humain",
        }
    }
}

/// Types d'un monstre : un à trois types distincts (section 5.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypesMonstre(Vec<Type>);

impl TypesMonstre {
    pub const MAX: usize = 3;

    pub fn new(types: &[Type]) -> Result<Self, ErreurTypes> {
        if types.is_empty() {
            return Err(ErreurTypes::Aucun);
        }
        if types.len() > Self::MAX {
            return Err(ErreurTypes::TropNombreux(types.len()));
        }
        for (position, type_monstre) in types.iter().enumerate() {
            if types[..position].contains(type_monstre) {
                return Err(ErreurTypes::EnDouble(*type_monstre));
            }
        }
        Ok(Self(types.to_vec()))
    }

    pub fn liste(&self) -> &[Type] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurTypes {
    Aucun,
    TropNombreux(usize),
    EnDouble(Type),
}

impl fmt::Display for ErreurTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurTypes::Aucun => write!(f, "un monstre doit avoir au moins un type"),
            ErreurTypes::TropNombreux(nombre) => write!(
                f,
                "un monstre a au plus {} types, {nombre} donnés",
                TypesMonstre::MAX
            ),
            ErreurTypes::EnDouble(type_monstre) => {
                write!(f, "type {type_monstre:?} présent plusieurs fois")
            }
        }
    }
}

impl std::error::Error for ErreurTypes {}

/// Multiplicateur de dégâts d'une attaque selon les types du défenseur.
///
/// Les coefficients de chaque type du défenseur se multiplient, puis le
/// résultat est borné entre x0,25 et x4 (Q18) : un monstre à 3 types garde la
/// plage connue de Pokémon au lieu d'aller de x0,125 à x8. Une immunité
/// l'emporte sur tout le reste.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Efficacite {
    /// x0
    Nulle,
    /// x0,25
    TresReduite,
    /// x0,5
    Reduite,
    /// x1
    Normale,
    /// x2
    Forte,
    /// x4
    TresForte,
}

/// Efficacité d'une attaque de type `attaque` contre un défenseur.
pub fn efficacite(attaque: Type, defenseur: &TypesMonstre) -> Efficacite {
    // Chaque faiblesse double les dégâts et chaque résistance les divise par
    // deux : on additionne donc des puissances de deux.
    let mut exposant: i32 = 0;
    for &defense in defenseur.liste() {
        match rapport(attaque, defense) {
            Rapport::Immunite => return Efficacite::Nulle,
            Rapport::Resistance => exposant -= 1,
            Rapport::Neutre => {}
            Rapport::Faiblesse => exposant += 1,
        }
    }
    match exposant {
        ..=-2 => Efficacite::TresReduite,
        -1 => Efficacite::Reduite,
        0 => Efficacite::Normale,
        1 => Efficacite::Forte,
        2.. => Efficacite::TresForte,
    }
}

/// Effet d'un type d'attaque sur un seul type de défenseur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rapport {
    Immunite,
    Resistance,
    Neutre,
    Faiblesse,
}

/// Table de Pokémon, avec une seule modification propre à Bersemon : le type
/// Glace résiste au type Eau (section 5.2). S'y ajoutent l'Obscur, qui
/// écrase presque tout et résiste à presque tout, le Stellaire, son
/// opposé, neutre envers les autres types, puis la Lumière, autre opposé de
/// l'Obscur, le Son et l'Humain (section 5.2).
fn rapport(attaque: Type, defense: Type) -> Rapport {
    use Type::*;
    // Les humains sont des proies : presque tout les frappe fort, et leurs
    // coups ne portent vraiment que contre un autre humain. Seuls les trois
    // opposés de l'Obscur les épargnent.
    match (attaque, defense) {
        (Humain, Humain) => return Rapport::Neutre,
        (Humain, Spectre) => return Rapport::Immunite,
        (Humain, _) => return Rapport::Resistance,
        (Lumiere | Feerique | Stellaire, Humain) => return Rapport::Resistance,
        (_, Humain) => return Rapport::Faiblesse,
        _ => {}
    }
    let (super_efficace_contre, peu_efficace_contre, sans_effet_sur): (&[Type], &[Type], &[Type]) =
        match attaque {
            // Les monstres Obscur résistent à tout sauf aux types Féérique,
            // Psychique, Stellaire et Obscur, qui leur sont très efficaces,
            // et aux types Ténèbres, Poison, Insecte, Dragon et Spectre,
            // neutres (section 5.2).
            Normal => (&[], &[Roche, Acier, Obscur, Son], &[Spectre]),
            Combat => (
                &[Normal, Roche, Acier, Glace, Tenebres],
                &[Vol, Poison, Insecte, Psychique, Feerique, Obscur],
                &[Spectre],
            ),
            Vol => (
                &[Combat, Insecte, Plante],
                &[Roche, Acier, Electrique, Obscur, Son],
                &[],
            ),
            Poison => (
                &[Plante, Feerique],
                &[Poison, Sol, Roche, Spectre],
                &[Acier],
            ),
            Sol => (
                &[Poison, Roche, Acier, Feu, Electrique, Son],
                &[Insecte, Plante, Obscur],
                &[Vol],
            ),
            Roche => (
                &[Vol, Insecte, Feu, Glace],
                &[Combat, Sol, Acier, Obscur],
                &[],
            ),
            Insecte => (
                &[Plante, Psychique, Tenebres],
                &[Combat, Vol, Poison, Spectre, Acier, Feu, Feerique],
                &[],
            ),
            Spectre => (&[Spectre, Psychique, Lumiere], &[Tenebres], &[Normal]),
            Acier => (
                &[Roche, Glace, Feerique],
                &[Acier, Feu, Eau, Electrique, Obscur],
                &[],
            ),
            Feu => (
                &[Insecte, Acier, Plante, Glace],
                &[Roche, Feu, Eau, Dragon, Obscur, Lumiere],
                &[],
            ),
            Eau => (
                &[Sol, Roche, Feu],
                &[Eau, Plante, Dragon, Glace, Obscur],
                &[],
            ),
            Plante => (
                &[Sol, Roche, Eau],
                &[Vol, Poison, Insecte, Acier, Feu, Plante, Dragon, Obscur],
                &[],
            ),
            Electrique => (
                &[Vol, Eau, Son],
                &[Plante, Electrique, Dragon, Obscur],
                &[Sol],
            ),
            Psychique => (&[Combat, Poison, Obscur], &[Acier, Psychique], &[Tenebres]),
            Glace => (
                &[Vol, Sol, Plante, Dragon],
                &[Acier, Feu, Eau, Glace, Obscur],
                &[],
            ),
            Dragon => (&[Dragon], &[Acier], &[Feerique]),
            Tenebres => (
                &[Spectre, Psychique, Lumiere],
                &[Combat, Tenebres, Feerique],
                &[],
            ),
            Feerique => (
                &[Combat, Dragon, Tenebres, Obscur],
                &[Poison, Acier, Feu, Lumiere],
                &[],
            ),
            Stellaire => (&[Obscur], &[], &[]),
            // Très efficace contre tout, sauf les types qui y résistent
            // (Féérique, Stellaire, Lumière) ou y sont neutres (section 5.2).
            Obscur => (
                &[
                    Normal, Combat, Vol, Sol, Roche, Acier, Feu, Eau, Plante, Electrique, Glace,
                    Obscur, Son,
                ],
                &[Feerique, Stellaire, Lumiere],
                &[],
            ),
            // La lumière chasse les ténèbres, les spectres et les Apôtres.
            Lumiere => (&[Tenebres, Spectre, Obscur], &[Feu, Acier, Lumiere], &[]),
            // Le vacarme trouble l'esprit, brise la glace et désoriente ce
            // qui vole ; un spectre n'a pas d'oreilles pour l'entendre.
            Son => (&[Psychique, Glace, Vol], &[Sol, Son, Obscur], &[Spectre]),
            // Ses rapports sont réglés avant ce tableau.
            Humain => (&[], &[], &[]),
        };
    if sans_effet_sur.contains(&defense) {
        Rapport::Immunite
    } else if peu_efficace_contre.contains(&defense) {
        Rapport::Resistance
    } else if super_efficace_contre.contains(&defense) {
        Rapport::Faiblesse
    } else {
        Rapport::Neutre
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Type::*;

    fn types(liste: &[Type]) -> TypesMonstre {
        TypesMonstre::new(liste).unwrap()
    }

    #[test]
    fn table_conforme_a_pokemon_plus_glace_contre_eau() {
        // La table de Pokémon compte 51 faiblesses, 61 résistances et 8
        // immunités ; Bersemon ajoute la résistance de Glace à Eau. Les
        // types de Pokémon sont les 18 premiers.
        let mut faiblesses = 0;
        let mut resistances = 0;
        let mut immunites = 0;
        for attaque in &Type::TOUS[..18] {
            for defense in &Type::TOUS[..18] {
                let (attaque, defense) = (*attaque, *defense);
                match rapport(attaque, defense) {
                    Rapport::Faiblesse => faiblesses += 1,
                    Rapport::Resistance => resistances += 1,
                    Rapport::Immunite => immunites += 1,
                    Rapport::Neutre => {}
                }
            }
        }
        assert_eq!((faiblesses, resistances, immunites), (51, 62, 8));
    }

    #[test]
    fn le_stellaire_n_est_neutre_qu_envers_les_types_autres_que_l_obscur_et_l_humain() {
        for autre in Type::TOUS {
            let attendu = match autre {
                Obscur => Rapport::Faiblesse,
                Humain => Rapport::Resistance,
                _ => Rapport::Neutre,
            };
            assert_eq!(rapport(Stellaire, autre), attendu, "{autre:?}");
            let attendu = if [Obscur, Humain].contains(&autre) {
                Rapport::Resistance
            } else {
                Rapport::Neutre
            };
            assert_eq!(rapport(autre, Stellaire), attendu, "{autre:?}");
        }
    }

    /// Section 5.2, décidée le 18/10/2026 et précisée le 19/10/2026.
    #[test]
    fn l_obscur_en_attaque_et_en_defense() {
        let neutres = [Psychique, Tenebres, Poison, Insecte, Dragon, Spectre];
        let opposes = [Feerique, Stellaire, Lumiere];
        for autre in Type::TOUS {
            let en_attaque = if opposes.contains(&autre) {
                Rapport::Resistance
            } else if neutres.contains(&autre) {
                Rapport::Neutre
            } else {
                Rapport::Faiblesse
            };
            assert_eq!(
                rapport(Obscur, autre),
                en_attaque,
                "Obscur contre {autre:?}"
            );
            let en_defense = if [Feerique, Psychique, Stellaire, Obscur, Lumiere].contains(&autre) {
                Rapport::Faiblesse
            } else if [Tenebres, Poison, Insecte, Dragon, Spectre].contains(&autre) {
                Rapport::Neutre
            } else {
                Rapport::Resistance
            };
            assert_eq!(
                rapport(autre, Obscur),
                en_defense,
                "{autre:?} contre Obscur"
            );
        }
    }

    /// Section 5.2, décidée le 24/10/2026 pour l'étape 93.
    #[test]
    fn la_lumiere_en_attaque_et_en_defense() {
        for autre in Type::TOUS {
            let en_attaque = if [Tenebres, Spectre, Obscur].contains(&autre) {
                Rapport::Faiblesse
            } else if [Feu, Acier, Lumiere, Humain].contains(&autre) {
                Rapport::Resistance
            } else {
                Rapport::Neutre
            };
            assert_eq!(
                rapport(Lumiere, autre),
                en_attaque,
                "Lumière contre {autre:?}"
            );
            let en_defense = if [Tenebres, Spectre].contains(&autre) {
                Rapport::Faiblesse
            } else if [Feu, Feerique, Lumiere, Obscur, Humain].contains(&autre) {
                Rapport::Resistance
            } else {
                Rapport::Neutre
            };
            assert_eq!(
                rapport(autre, Lumiere),
                en_defense,
                "{autre:?} contre Lumière"
            );
        }
    }

    /// Section 5.2, décidée le 24/10/2026 pour l'étape 93.
    #[test]
    fn le_son_en_attaque_et_en_defense() {
        for autre in Type::TOUS {
            let en_attaque = if autre == Spectre {
                Rapport::Immunite
            } else if [Psychique, Glace, Vol, Humain].contains(&autre) {
                Rapport::Faiblesse
            } else if [Sol, Son, Obscur].contains(&autre) {
                Rapport::Resistance
            } else {
                Rapport::Neutre
            };
            assert_eq!(rapport(Son, autre), en_attaque, "Son contre {autre:?}");
            let en_defense = if [Sol, Electrique, Obscur].contains(&autre) {
                Rapport::Faiblesse
            } else if [Normal, Vol, Son, Humain].contains(&autre) {
                Rapport::Resistance
            } else {
                Rapport::Neutre
            };
            assert_eq!(rapport(autre, Son), en_defense, "{autre:?} contre Son");
        }
    }

    /// Section 5.2, décidée le 25/10/2026 pour l'étape 128.
    #[test]
    fn l_humain_en_attaque_et_en_defense() {
        for autre in Type::TOUS {
            let en_attaque = match autre {
                Humain => Rapport::Neutre,
                Spectre => Rapport::Immunite,
                _ => Rapport::Resistance,
            };
            assert_eq!(
                rapport(Humain, autre),
                en_attaque,
                "Humain contre {autre:?}"
            );
            let en_defense = match autre {
                Humain => Rapport::Neutre,
                Lumiere | Feerique | Stellaire => Rapport::Resistance,
                _ => Rapport::Faiblesse,
            };
            assert_eq!(
                rapport(autre, Humain),
                en_defense,
                "{autre:?} contre Humain"
            );
        }
        assert_eq!(efficacite(Obscur, &types(&[Humain])), Efficacite::Forte);
        assert_eq!(efficacite(Humain, &types(&[Spectre])), Efficacite::Nulle);
    }

    #[test]
    fn le_son_n_a_pas_d_effet_sur_un_spectre() {
        assert_eq!(efficacite(Son, &types(&[Spectre])), Efficacite::Nulle);
        assert_eq!(
            efficacite(Son, &types(&[Glace, Spectre])),
            Efficacite::Nulle
        );
    }

    #[test]
    fn glace_resiste_a_eau() {
        assert_eq!(efficacite(Eau, &types(&[Glace])), Efficacite::Reduite);
        assert_eq!(efficacite(Glace, &types(&[Eau])), Efficacite::Reduite);
    }

    #[test]
    fn efficacite_contre_un_seul_type() {
        assert_eq!(efficacite(Eau, &types(&[Feu])), Efficacite::Forte);
        assert_eq!(efficacite(Feu, &types(&[Eau])), Efficacite::Reduite);
        assert_eq!(efficacite(Normal, &types(&[Feu])), Efficacite::Normale);
        assert_eq!(efficacite(Normal, &types(&[Spectre])), Efficacite::Nulle);
    }

    #[test]
    fn faiblesses_et_resistances_se_cumulent() {
        assert_eq!(
            efficacite(Sol, &types(&[Feu, Roche])),
            Efficacite::TresForte
        );
        assert_eq!(
            efficacite(Feu, &types(&[Eau, Roche])),
            Efficacite::TresReduite
        );
        assert_eq!(efficacite(Feu, &types(&[Plante, Eau])), Efficacite::Normale);
    }

    #[test]
    fn trois_faiblesses_plafonnees_a_x4() {
        assert_eq!(
            efficacite(Roche, &types(&[Feu, Vol, Glace])),
            Efficacite::TresForte
        );
    }

    #[test]
    fn trois_resistances_plafonnees_a_x0_25() {
        assert_eq!(
            efficacite(Plante, &types(&[Feu, Vol, Dragon])),
            Efficacite::TresReduite
        );
    }

    #[test]
    fn immunite_l_emporte_sur_les_faiblesses() {
        assert_eq!(
            efficacite(Sol, &types(&[Feu, Roche, Vol])),
            Efficacite::Nulle
        );
    }

    #[test]
    fn types_monstre_refuse_une_liste_vide() {
        assert_eq!(TypesMonstre::new(&[]), Err(ErreurTypes::Aucun));
    }

    #[test]
    fn types_monstre_refuse_plus_de_trois_types() {
        assert_eq!(
            TypesMonstre::new(&[Feu, Eau, Plante, Sol]),
            Err(ErreurTypes::TropNombreux(4))
        );
    }

    #[test]
    fn types_monstre_refuse_un_doublon() {
        assert_eq!(
            TypesMonstre::new(&[Feu, Eau, Feu]),
            Err(ErreurTypes::EnDouble(Feu))
        );
    }

    #[test]
    fn types_monstre_accepte_un_a_trois_types() {
        assert_eq!(types(&[Feu]).liste(), &[Feu]);
        assert_eq!(types(&[Feu, Vol, Dragon]).liste(), &[Feu, Vol, Dragon]);
    }
}
