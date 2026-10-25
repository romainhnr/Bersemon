//! La forge de Gunther (sections 13.4 et 13.5) : le joueur lance une
//! amélioration, la fabrication d'un objet ou d'un lot de filets, que le
//! déroulement fait confirmer, constate le travail fini, ou gère les
//! machines (section 13.6).

use bersemon_core::amelioration::Amelioration;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::partie::Partie;
use bersemon_presentation::equipement::objet_et_bonus;
use bersemon_presentation::expeditions_automatiques::duree;
use bersemon_presentation::filets::liste_de_filets;
use bersemon_presentation::forge::{
    cout, etat_de_la_forge, etat_des_fabrications, plan_des_filets,
};

use crate::ecran_machines;
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
    // Aussi souvent que le joueur le veut (section 13.5).
    let plan = contenu.plan_du_charme();
    choix.push((
        Choix::FabriquerCharme,
        format!(
            "Fabriquer un charme de garde ({}, {}) : vous en avez {}",
            cout(plan.cout()),
            duree(plan.duree()),
            partie.charmes_de_garde()
        ),
    ));
    let fabriquer = choix.len() + 1;
    let filets = fabriquer + 1;
    // Les machines se gèrent ici dès que l'une d'elles est construite.
    let ameliorations = partie.forge().ameliorations();
    let avec_machines = ameliorations.pieges().is_some() || ameliorations.cour().is_some();
    let machines = filets + 1;
    for (position, (_, libelle)) in choix.iter().enumerate() {
        println!("  {}. {libelle}", position + 1);
    }
    println!("  {fabriquer}. Fabriquer un objet");
    println!("  {filets}. Fabriquer des filets");
    if avec_machines {
        println!("  {machines}. Machines");
    }
    println!("  0. Retour au camp");
    let numero = entree.choix(0, if avec_machines { machines } else { filets })?;
    if numero == fabriquer {
        return fabriquer_un_objet(partie, contenu, entree);
    }
    if numero == filets {
        return fabriquer_des_filets(partie, contenu, entree);
    }
    if numero == machines {
        return ecran_machines::choisir(partie, contenu, entree);
    }
    Some((numero > 0).then(|| choix.swap_remove(numero - 1).0))
}

/// Les trois qualités, par lot, aussi souvent que le joueur le veut, même
/// faute de ressources ou si Gunther est occupé : le refus dit pourquoi
/// (section 13.5).
fn fabriquer_des_filets(
    partie: &Partie,
    contenu: &ContenuV1,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    let possedes = liste_de_filets(&partie.filets());
    println!();
    if possedes.is_empty() {
        println!("Vous n'avez aucun filet, entre votre sac et le coffre.");
    } else {
        println!(
            "Vous avez {}, entre votre sac et le coffre.",
            possedes.join(", ")
        );
    }
    let plans = contenu.plans_des_filets();
    for (position, plan) in plans.iter().enumerate() {
        println!("  {}. {}", position + 1, plan_des_filets(plan));
    }
    println!("  0. Retour au camp");
    let numero = entree.choix(0, plans.len())?;
    Some((numero > 0).then(|| Choix::FabriquerFilets(plans[numero - 1].filet())))
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
        .plans_de_fabrication_pour(partie.heros().classe())
        .into_iter()
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
