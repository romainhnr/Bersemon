//! La Fosse aux damnés simulée (section 9.8 de gamedesign.md) : le héros de
//! chaque classe, à chaque niveau de combat que donne son équipement, contre
//! l'adversaire de chaque palier. Comme dans la Fosse, il combat seul et
//! sans objet ; il attaque avec l'attaque aux dégâts attendus les plus
//! hauts, comme le joueur automatique en expédition.

use bersemon_core::aleatoire::GenerateurAleatoire;
use bersemon_core::catalogue::Catalogue;
use bersemon_core::combat::{Action, Combat, Cote, Issue, Phase};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::equipement::SorteDObjet;
use bersemon_core::espece::{Espece, Role};
use bersemon_core::heros::{Classe, creer_combattant};
use bersemon_core::monstre::Monstre;
use bersemon_core::types::{Type, TypesMonstre};

/// Le niveau de combat du héros, puis les paliers de l'arme et de l'armure
/// qui le donnent, l'arme d'abord, comme le joueur automatique les forge.
/// L'arme du palier 4 est celle que donne le dernier palier de la Fosse.
pub const EQUIPEMENTS: [(u8, u8, u8); 7] = [
    (10, 1, 0),
    (15, 1, 1),
    (20, 2, 1),
    (25, 2, 2),
    (30, 3, 2),
    (35, 3, 3),
    (40, 4, 3),
];

/// Assez pour que la part de victoires d'une case varie de quelques points
/// seulement d'une graine à l'autre.
pub const COMBATS_PAR_CASE: u64 = 200;

/// Au-delà, le combat compte comme perdu : aucun des deux héros n'abat
/// l'autre.
const TOURS_MAX: u32 = 300;

#[derive(Debug, Clone, PartialEq)]
pub struct MesureDuPalier {
    pub numero: u8,
    pub adversaire: String,
    pub classe: Classe,
    pub niveau: u8,
    pub types: String,
    /// Pour chaque classe du héros, la part de victoires, en pourcentage,
    /// à chaque niveau d'`EQUIPEMENTS`.
    pub victoires: Vec<(Classe, Vec<f64>)>,
    /// La durée moyenne d'un combat, en tours, toutes classes et tous
    /// niveaux confondus.
    pub tours: f64,
}

/// `avec_second_type` faux : tous les humains restent Humain seul, comme
/// avant l'étape 145, pour mesurer l'effet du second type.
pub fn mesurer_la_fosse(contenu: &ContenuV1, avec_second_type: bool) -> Vec<MesureDuPalier> {
    let typer = |heros: Monstre| {
        if avec_second_type {
            heros
        } else {
            sans_second_type(heros)
        }
    };
    let mut mesures = Vec::new();
    for (position, adversaire) in contenu.fosse().adversaires().iter().enumerate() {
        let numero = u8::try_from(position + 1).expect("10 paliers");
        let en_face = typer(adversaire.generer_heros());
        let mut victoires = Vec::new();
        let mut tours = 0u64;
        let mut combats = 0u64;
        for classe in Classe::TOUTES {
            let mut parts = Vec::new();
            for (niveau, arme, armure) in EQUIPEMENTS {
                let heros = typer(heros_equipe(contenu, classe, arme, armure));
                debug_assert_eq!(heros.niveau().valeur(), niveau);
                let mut gagnes = 0u64;
                for graine in 0..COMBATS_PAR_CASE {
                    let graine = u64::from(numero) * 1_000_000
                        + u64::from(niveau) * 10_000
                        + classe_en_nombre(classe) * 1_000
                        + graine;
                    let (gagne, duree) =
                        combattre(heros.clone(), en_face.clone(), adversaire.ia(), graine);
                    gagnes += u64::from(gagne);
                    tours += u64::from(duree);
                    combats += 1;
                }
                parts.push(gagnes as f64 * 100.0 / COMBATS_PAR_CASE as f64);
            }
            victoires.push((classe, parts));
        }
        let types: Vec<&str> = en_face
            .espece()
            .types()
            .liste()
            .iter()
            .map(|type_monstre| type_monstre.nom())
            .collect();
        mesures.push(MesureDuPalier {
            numero,
            adversaire: adversaire.nom().to_string(),
            classe: adversaire.classe(),
            niveau: en_face.niveau().valeur(),
            types: types.join(", "),
            victoires,
            tours: tours as f64 / combats as f64,
        });
    }
    mesures
}

fn classe_en_nombre(classe: Classe) -> u64 {
    match classe {
        Classe::Guerrier => 0,
        Classe::Mage => 1,
        Classe::Templier => 2,
        Classe::Assassin => 3,
    }
}

fn heros_equipe(contenu: &ContenuV1, classe: Classe, arme: u8, armure: u8) -> Monstre {
    let arme = contenu.objet_de_classe(classe, SorteDObjet::Arme, arme);
    let armure = (armure > 0)
        .then(|| contenu.objet_de_classe(classe, SorteDObjet::Armure, armure))
        .flatten();
    debug_assert!(arme.is_some(), "arme de classe du contenu");
    creer_combattant(classe.nom(), classe, arme.as_ref(), armure.as_ref())
}

/// Le même héros, Humain seul.
fn sans_second_type(heros: Monstre) -> Monstre {
    if heros.espece().types().liste().len() == 1 {
        return heros;
    }
    let espece = Espece::new(
        heros.espece().id().clone(),
        heros.espece().nom(),
        TypesMonstre::new(&[Type::Humain]).expect("un type"),
        Role::Offensif,
        *heros.espece().stats_de_base(),
        0,
        Vec::new(),
    );
    Monstre::new(
        espece,
        heros.niveau(),
        *heros.valeurs_internes(),
        heros.attaques().clone(),
        false,
    )
}

/// Vrai si le héros gagne, avec le nombre de tours joués.
fn combattre(
    heros: Monstre,
    adversaire: Monstre,
    ia: bersemon_core::humain::IaHumain,
    graine: u64,
) -> (bool, u32) {
    let mut combat = Combat::dans_l_arene(heros, adversaire, ia).expect("deux héros valides");
    let mut aleatoire = GenerateurAleatoire::new(graine);
    for tour in 0..TOURS_MAX {
        match combat.phase() {
            Phase::Termine(issue) => return (issue == Issue::Victoire, tour),
            // Seul face à un adversaire seul : aucun remplacement possible.
            Phase::Remplacement => return (false, tour),
            Phase::ChoixAction => {
                let action = Action::Attaquer(meilleure_attaque(&combat));
                combat
                    .jouer_tour(action, &mut aleatoire)
                    .expect("attaque du héros valide");
            }
        }
    }
    (false, TOURS_MAX)
}

fn meilleure_attaque(combat: &Combat) -> usize {
    let attaquant = combat.monstre(combat.actif(Cote::Joueur));
    (0..attaquant.attaques().liste().len())
        .max_by_key(|index| {
            let precision = attaquant.attaques().liste()[*index].precision();
            combat.apercu_attaque(*index).map_or(0, |apercu| {
                (apercu.degats_min + apercu.degats_max) * u32::from(precision)
            })
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaque_equipement_donne_son_niveau_de_combat() {
        let contenu = ContenuV1::new();
        for classe in Classe::TOUTES {
            for (niveau, arme, armure) in EQUIPEMENTS {
                let heros = heros_equipe(&contenu, classe, arme, armure);
                assert_eq!(heros.niveau().valeur(), niveau, "{}", classe.nom());
            }
        }
    }

    #[test]
    fn sans_second_type_le_heros_reste_humain_seul() {
        let contenu = ContenuV1::new();
        let heros = heros_equipe(&contenu, Classe::Guerrier, 3, 3);
        assert_eq!(heros.espece().types().liste(), [Type::Humain, Type::Combat]);
        let humain = sans_second_type(heros.clone());
        assert_eq!(humain.espece().types().liste(), [Type::Humain]);
        assert_eq!(humain.stats(), heros.stats());
        assert_eq!(humain.attaques(), heros.attaques());
    }
}
