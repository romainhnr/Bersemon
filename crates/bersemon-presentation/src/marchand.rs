//! Malvoise, le nécromancien du camp (section 14.2) : son étal, les soins et
//! les capsules d'attaque qu'il vend et le prix auquel il rachète les
//! monstres.

use bersemon_core::capsule::Capsule;
use bersemon_core::collection::Collection;
use bersemon_core::combat::Cote;
use bersemon_core::marchand::prix_de_revente;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;

use crate::monstres::{decrire_attaque, nom};
use crate::partie::argent;
use crate::soins::offre_de_soin;

/// Ce que le joueur peut dépenser : l'argent du sac et du coffre paient
/// ensemble, et les achats vont dans le coffre (section 13.3).
pub fn etal(partie: &Partie) -> String {
    let soins: Vec<String> = partie
        .soins_en_vente()
        .into_iter()
        .map(|soin| format!("  {}", offre_de_soin(soin)))
        .collect();
    format!(
        "Malvoise vous attend près du feu. Vous avez {}, entre votre sac et le coffre ; vos achats iront dans le coffre.\nSoins en vente :\n{}\n{}",
        argent(partie.argent()),
        soins.join("\n"),
        capsules_obtenues(partie)
    )
}

pub fn capsules_obtenues(partie: &Partie) -> String {
    let noms: Vec<&str> = partie
        .capsules()
        .map(|capsule| capsule.attaque().nom())
        .collect();
    if noms.is_empty() {
        "Aucune capsule d'attaque obtenue.".to_string()
    } else {
        format!("Capsules d'attaque obtenues : {}.", noms.join(", "))
    }
}

/// Celles que le joueur n'a pas encore : une capsule sert sans s'user.
pub fn capsules_en_vente<'c>(partie: &Partie, capsules: &'c [Capsule]) -> Vec<&'c Capsule> {
    capsules
        .iter()
        .filter(|capsule| !partie.possede_capsule(capsule.id()))
        .collect()
}

/// L'attaque enseignée et son prix.
pub fn offre_de_capsule(capsule: &Capsule) -> String {
    format!(
        "{} : {}",
        decrire_attaque(capsule.attaque()),
        argent(capsule.prix())
    )
}

/// Avec un surnom, le nom de l'espèce reste visible : c'est d'elle que
/// dépend le prix (section 14.5).
pub fn offre_de_rachat(monstre: &Monstre) -> String {
    let espece = if monstre.surnom().is_some() {
        format!("{}, ", monstre.espece().nom())
    } else {
        String::new()
    };
    format!(
        "{} ({espece}niv. {}) : {}",
        nom(monstre, Cote::Joueur),
        monstre.niveau().valeur(),
        argent(prix_de_revente(monstre))
    )
}

/// Équipe puis camp, numérotés à la suite comme la liste des monstres.
pub fn liste_de_rachat(collection: &Collection) -> String {
    let equipe = collection.equipe();
    let mut lignes = vec!["Équipe :".to_string()];
    for (index, monstre) in equipe.iter().enumerate() {
        lignes.push(format!("  {}. {}", index + 1, offre_de_rachat(monstre)));
    }
    lignes.push("Camp :".to_string());
    if collection.camp().is_empty() {
        lignes.push("  aucun monstre".to_string());
    }
    for (index, monstre) in collection.camp().iter().enumerate() {
        lignes.push(format!(
            "  {}. {}",
            equipe.len() + index + 1,
            offre_de_rachat(monstre)
        ));
    }
    lignes.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::contenu::ContenuV1;
    use bersemon_core::espece::EspeceId;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::stats::ValeursInternes;

    #[test]
    fn le_rachat_dit_l_espece_le_niveau_et_le_prix() {
        let contenu = ContenuV1::new();
        let mut cerf = Monstre::creer(
            contenu.espece(&EspeceId::new("cerf-blanc")).unwrap(),
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        assert_eq!(
            offre_de_rachat(&cerf),
            "Cerf blanc (niv. 5) : 80 pièces d'argent"
        );
        cerf.renommer(Some(Nom::new("Lune").unwrap()));
        assert_eq!(
            offre_de_rachat(&cerf),
            "Lune (Cerf blanc, niv. 5) : 80 pièces d'argent"
        );
    }

    #[test]
    fn une_capsule_dit_son_attaque_et_son_prix() {
        let contenu = ContenuV1::new();
        assert_eq!(
            offre_de_capsule(&contenu.capsules()[0]),
            "Brasier (Feu, spéciale, puissance 70, précision 100 %) : 150 pièces d'argent"
        );
    }
}
