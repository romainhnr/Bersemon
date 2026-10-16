//! Types des monstres et des attaques, et table d'efficacité (section 5.2).

use std::fmt;

/// Types de la V1. Stellaire, Obscur, Lumière et Son viendront après la V1
/// (Q5).
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
}

impl Type {
    pub const TOUS: [Type; 18] = [
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
    ];
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
/// Glace résiste au type Eau (section 5.2).
fn rapport(attaque: Type, defense: Type) -> Rapport {
    use Type::*;
    let (super_efficace_contre, peu_efficace_contre, sans_effet_sur): (&[Type], &[Type], &[Type]) =
        match attaque {
            Normal => (&[], &[Roche, Acier], &[Spectre]),
            Combat => (
                &[Normal, Roche, Acier, Glace, Tenebres],
                &[Vol, Poison, Insecte, Psychique, Feerique],
                &[Spectre],
            ),
            Vol => (&[Combat, Insecte, Plante], &[Roche, Acier, Electrique], &[]),
            Poison => (
                &[Plante, Feerique],
                &[Poison, Sol, Roche, Spectre],
                &[Acier],
            ),
            Sol => (
                &[Poison, Roche, Acier, Feu, Electrique],
                &[Insecte, Plante],
                &[Vol],
            ),
            Roche => (&[Vol, Insecte, Feu, Glace], &[Combat, Sol, Acier], &[]),
            Insecte => (
                &[Plante, Psychique, Tenebres],
                &[Combat, Vol, Poison, Spectre, Acier, Feu, Feerique],
                &[],
            ),
            Spectre => (&[Spectre, Psychique], &[Tenebres], &[Normal]),
            Acier => (
                &[Roche, Glace, Feerique],
                &[Acier, Feu, Eau, Electrique],
                &[],
            ),
            Feu => (
                &[Insecte, Acier, Plante, Glace],
                &[Roche, Feu, Eau, Dragon],
                &[],
            ),
            Eau => (&[Sol, Roche, Feu], &[Eau, Plante, Dragon, Glace], &[]),
            Plante => (
                &[Sol, Roche, Eau],
                &[Vol, Poison, Insecte, Acier, Feu, Plante, Dragon],
                &[],
            ),
            Electrique => (&[Vol, Eau], &[Plante, Electrique, Dragon], &[Sol]),
            Psychique => (&[Combat, Poison], &[Acier, Psychique], &[Tenebres]),
            Glace => (&[Vol, Sol, Plante, Dragon], &[Acier, Feu, Eau, Glace], &[]),
            Dragon => (&[Dragon], &[Acier], &[Feerique]),
            Tenebres => (&[Spectre, Psychique], &[Combat, Tenebres, Feerique], &[]),
            Feerique => (&[Combat, Dragon, Tenebres], &[Poison, Acier, Feu], &[]),
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
        // immunités ; Bersemon ajoute la résistance de Glace à Eau.
        let mut faiblesses = 0;
        let mut resistances = 0;
        let mut immunites = 0;
        for attaque in Type::TOUS {
            for defense in Type::TOUS {
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
