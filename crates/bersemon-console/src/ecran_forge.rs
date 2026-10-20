//! La forge de Gunther (sections 13.4 et 13.5) : le joueur lance une
//! amélioration ou la fabrication d'un objet, que le déroulement fait
//! confirmer, ou constate le travail fini.

use bersemon_core::amelioration::Amelioration;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::partie::Partie;
use bersemon_presentation::equipement::objet_et_bonus;
use bersemon_presentation::forge::{etat_de_la_forge, etat_des_fabrications};

use crate::horloge::maintenant;
use crate::saisie::Entree;

/// `Some(None)` au retour vers le menu du camp. Une amélioration impossible,
/// faute de ressources par exemple, est proposée quand même : le refus dit
/// ce qui manque.
pub fn choisir(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    let maintenant = maintenant();
    println!();
    println!("{}", etat_de_la_forge(partie, contenu, maintenant));
    let mut choix = Vec::new();
    if partie
        .forge()
        .chantier()
        .is_some_and(|chantier| chantier.est_fini(maintenant))
    {
        choix.push((
            Choix::ConstaterTravail,
            "Constater le travail de Gunther".to_string(),
        ));
    }
    for amelioration in Amelioration::TOUTES {
        if partie.forge().ameliorations().niveau(amelioration) < amelioration.niveau_max() {
            choix.push((
                Choix::LancerAmelioration(amelioration),
                format!("Améliorer : {}", amelioration.nom()),
            ));
        }
    }
    let fabriquer = choix.len() + 1;
    for (position, (_, libelle)) in choix.iter().enumerate() {
        println!("  {}. {libelle}", position + 1);
    }
    println!("  {fabriquer}. Fabriquer un objet");
    println!("  0. Retour au camp");
    let numero = entree.choix(0, fabriquer)?;
    if numero == fabriquer {
        return fabriquer_un_objet(partie, contenu, entree);
    }
    Some((numero > 0).then(|| choix.swap_remove(numero - 1).0))
}

/// Les objets que le héros ne possède pas encore, même faute de ressources
/// ou si Gunther est occupé : le refus dit pourquoi (section 13.5).
fn fabriquer_un_objet(
    partie: &Partie,
    contenu: &ContenuV1,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    println!();
    println!("{}", etat_des_fabrications(partie, contenu));
    let a_fabriquer: Vec<_> = contenu
        .plans_de_fabrication()
        .iter()
        .filter(|plan| !partie.heros().equipement().possede(plan.objet().id()))
        .collect();
    for (position, plan) in a_fabriquer.iter().enumerate() {
        println!("  {}. {}", position + 1, objet_et_bonus(plan.objet()));
    }
    println!("  0. Retour au camp");
    let numero = entree.choix(0, a_fabriquer.len())?;
    Some(
        (numero > 0)
            .then(|| Choix::LancerFabrication(a_fabriquer[numero - 1].objet().id().clone())),
    )
}
