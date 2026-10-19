//! La forge de Gunther (section 13.4) : le joueur lance une amélioration,
//! que le déroulement fait confirmer, ou constate le travail fini.

use bersemon_core::amelioration::Amelioration;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::partie::Partie;
use bersemon_presentation::forge::etat_de_la_forge;

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
            Choix::ConstaterAmelioration,
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
    for (position, (_, libelle)) in choix.iter().enumerate() {
        println!("  {}. {libelle}", position + 1);
    }
    println!("  0. Retour au camp");
    let numero = entree.choix(0, choix.len())?;
    Some((numero > 0).then(|| choix.swap_remove(numero - 1).0))
}
