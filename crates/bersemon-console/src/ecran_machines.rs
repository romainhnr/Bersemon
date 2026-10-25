//! Les machines de Gunther (section 13.6), depuis sa forge : poser et
//! relever les pièges, placer un monstre dans la cour d'entraînement ou l'en
//! reprendre. La cour a aussi son écran, depuis le menu du camp.

use bersemon_core::combat::Cote;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Choix;
use bersemon_core::partie::Partie;
use bersemon_presentation::machines::{etat_de_la_cour, machines};
use bersemon_presentation::mise_en_valeur::sans_marques;
use bersemon_presentation::monstres::nom;

use crate::ecran_camp::designer;
use crate::horloge::maintenant;
use crate::saisie::Entree;

/// Ce que le joueur peut faire de ses machines.
enum Action {
    Choix(Choix),
    /// Le monstre se désigne ensuite.
    Entrainer,
}

/// `Some(None)` au retour vers le menu du camp.
pub fn choisir(partie: &Partie, contenu: &ContenuV1, entree: &mut Entree) -> Option<Option<Choix>> {
    let maintenant = maintenant();
    let plafond = partie.plafond_d_entrainement(contenu.zones(), contenu.piste());
    println!();
    println!(
        "{}",
        sans_marques(&machines(partie, contenu.zones(), plafond, maintenant))
    );
    let ameliorations = partie.forge().ameliorations();
    let mut actions: Vec<(Action, String)> = Vec::new();
    if let Some(regles) = ameliorations.pieges() {
        let pieges = partie.machines().pieges();
        if pieges.is_some_and(|pieges| pieges.stock(&regles, maintenant) > 0) {
            actions.push((
                Action::Choix(Choix::ReleverLesPieges),
                "Relever les pièges".to_string(),
            ));
        }
        for zone in contenu.zones() {
            let ici = pieges.is_some_and(|pieges| pieges.zone() == zone.id());
            if partie.est_exploree(zone.id()) && !ici {
                actions.push((
                    Action::Choix(Choix::PoserLesPieges(zone.id().clone())),
                    format!("Poser les pièges dans {}", zone.nom()),
                ));
            }
        }
    }
    actions.extend(actions_de_la_cour(partie));
    executer(actions, partie, entree)
}

/// La cour seule, depuis le menu du camp. `Some(None)` au retour vers le
/// menu du camp.
pub fn choisir_dans_la_cour(
    partie: &Partie,
    contenu: &ContenuV1,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    let plafond = partie.plafond_d_entrainement(contenu.zones(), contenu.piste());
    let Some(etat) = etat_de_la_cour(partie, plafond, maintenant()) else {
        return Some(None);
    };
    println!();
    println!("{}", sans_marques(&etat));
    executer(actions_de_la_cour(partie), partie, entree)
}

/// Mettre un monstre à l'entraînement s'il reste une place, puis reprendre
/// chaque monstre de la cour ; rien si elle n'est pas construite.
fn actions_de_la_cour(partie: &Partie) -> Vec<(Action, String)> {
    let mut actions = Vec::new();
    let Some(regles) = partie.forge().ameliorations().cour() else {
        return actions;
    };
    let cour = partie.machines().cour();
    if cour.len() < regles.places {
        actions.push((
            Action::Entrainer,
            "Mettre un monstre à l'entraînement".to_string(),
        ));
    }
    for (position, entrainement) in cour.iter().enumerate() {
        actions.push((
            Action::Choix(Choix::ReprendreDeLEntrainement(position)),
            format!("Reprendre {}", nom(entrainement.monstre(), Cote::Joueur)),
        ));
    }
    actions
}

/// Numérote les actions, puis rend celle que choisit le joueur ;
/// `Some(None)` au retour vers le menu du camp.
fn executer(
    mut actions: Vec<(Action, String)>,
    partie: &Partie,
    entree: &mut Entree,
) -> Option<Option<Choix>> {
    for (position, (_, libelle)) in actions.iter().enumerate() {
        println!("  {}. {libelle}", position + 1);
    }
    println!("  0. Retour au camp");
    let numero = entree.choix(0, actions.len())?;
    if numero == 0 {
        return Some(None);
    }
    match actions.swap_remove(numero - 1).0 {
        Action::Choix(choix) => Some(Some(choix)),
        Action::Entrainer => {
            let emplacement = designer(
                partie.collection(),
                entree,
                "Quel monstre mettre à l'entraînement ?",
            )?;
            Some(emplacement.map(Choix::MettreALEntrainement))
        }
    }
}
