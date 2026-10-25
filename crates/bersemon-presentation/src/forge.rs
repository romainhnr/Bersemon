//! La forge de Gunther (sections 13.4 et 13.5) : améliorations du camp,
//! objets d'équipement, leurs coûts, le travail en cours et son constat.

use bersemon_core::amelioration::{
    Amelioration, Chantier, EffetDAmelioration, ErreurAmelioration, Ouvrage, PlanDAmelioration,
    PlanDeFabrication, PlanDesFilets, PlanDuCharme,
};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::expedition_automatique::PROTECTION_DU_CHARME;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::partie::Partie;
use bersemon_core::ressource::Ressource;

use crate::equipement::objet_et_bonus;
use crate::expeditions_automatiques::duree;
use crate::filets::{effet_du_filet, filets};
use crate::machines::{effet_de_la_cour, effet_des_pieges};
use crate::partie::{enumerer, ressources};

pub fn effet(amelioration: Amelioration, niveau: u8) -> String {
    match amelioration.effet(niveau) {
        EffetDAmelioration::PlacesAuCamp(places) => format!("camp de {places} places"),
        EffetDAmelioration::PlacesDansLEquipe(places) => format!("équipe de {places} monstres"),
        EffetDAmelioration::PlacesDansLeSac(places) => format!("sac de {places} places"),
        EffetDAmelioration::ChanceDeCaptureEnPlus(pour_cent) => {
            format!("chance de capture +{pour_cent} %")
        }
        EffetDAmelioration::Pieges(regles) => effet_des_pieges(&regles),
        EffetDAmelioration::Cour(regles) => effet_de_la_cour(&regles),
    }
}

/// "Enclos, niveau 1 (camp de 40 places)".
pub fn niveau_d_amelioration(amelioration: Amelioration, niveau: u8) -> String {
    format!(
        "{}, niveau {niveau} ({})",
        amelioration.nom(),
        effet(amelioration, niveau)
    )
}

/// Le niveau d'amélioration, ou l'objet et son bonus.
pub fn ouvrage(ouvrage: &Ouvrage) -> String {
    match ouvrage {
        Ouvrage::Amelioration {
            amelioration,
            niveau,
        } => niveau_d_amelioration(*amelioration, *niveau),
        Ouvrage::Objet(objet) => objet_et_bonus(objet),
        Ouvrage::CharmeDeGarde => "Charme de garde".to_string(),
        Ouvrage::Filets { filet, nombre } => filets(*filet, *nombre),
    }
}

pub fn cout(cout: &[(Ressource, u32)]) -> String {
    let liste: Vec<String> = cout
        .iter()
        .map(|(ressource, nombre)| ressources(ressource, *nombre))
        .collect();
    enumerer(&liste)
}

pub fn question_de_lancement(plan: &PlanDAmelioration) -> String {
    format!(
        "Faire construire {} ? Gunther prendra {}, et aura fini dans {}. Une construction lancée ne s'annule pas.",
        niveau_d_amelioration(plan.amelioration(), plan.niveau()),
        cout(plan.cout()),
        duree(plan.duree())
    )
}

pub fn lancement(plan: &PlanDAmelioration) -> String {
    format!(
        "Vous remettez à Gunther {}. Il se met au travail : {}. Il aura fini dans {}.",
        cout(plan.cout()),
        niveau_d_amelioration(plan.amelioration(), plan.niveau()),
        duree(plan.duree())
    )
}

pub fn question_de_fabrication(plan: &PlanDeFabrication) -> String {
    format!(
        "Faire forger {} ? Gunther prendra {}, et aura fini dans {}. Un travail lancé ne s'annule pas.",
        objet_et_bonus(plan.objet()),
        cout(plan.cout()),
        duree(plan.duree())
    )
}

pub fn lancement_de_fabrication(plan: &PlanDeFabrication) -> String {
    format!(
        "Vous remettez à Gunther {}. Il se met à forger : {}. Il aura fini dans {}.",
        cout(plan.cout()),
        objet_et_bonus(plan.objet()),
        duree(plan.duree())
    )
}

pub fn constat(chantier: &Chantier) -> String {
    match &chantier.ouvrage {
        Ouvrage::Amelioration { .. } => {
            format!("Gunther a fini : {}.", ouvrage(&chantier.ouvrage))
        }
        Ouvrage::Objet(objet) => format!(
            "Gunther a fini : {}. Vous pouvez le porter depuis l'équipement du héros.",
            objet_et_bonus(objet)
        ),
        Ouvrage::CharmeDeGarde => {
            "Gunther a fini : un charme de garde. Il va dans le coffre.".to_string()
        }
        Ouvrage::Filets { filet, nombre } => format!(
            "Gunther a fini : {}. Ils vont dans le coffre.",
            filets(*filet, *nombre)
        ),
    }
}

/// "5 filets lestés (chance de capture x1,5)".
pub fn lot_de_filets(plan: &PlanDesFilets) -> String {
    format!(
        "{} ({})",
        filets(plan.filet(), plan.nombre()),
        effet_du_filet(plan.filet())
    )
}

/// "5 filets lestés (chance de capture x1,5) : Fragment d'armure (3) et Os
/// de squelette (3), 1 h".
pub fn plan_des_filets(plan: &PlanDesFilets) -> String {
    format!(
        "{} : {}, {}",
        lot_de_filets(plan),
        cout(plan.cout()),
        duree(plan.duree())
    )
}

pub fn question_des_filets(plan: &PlanDesFilets) -> String {
    format!(
        "Faire forger {} ? Gunther prendra {}, et aura fini dans {}. Un travail lancé ne s'annule pas.",
        lot_de_filets(plan),
        cout(plan.cout()),
        duree(plan.duree())
    )
}

pub fn lancement_des_filets(plan: &PlanDesFilets) -> String {
    format!(
        "Vous remettez à Gunther {}. Il se met à forger {}. Il aura fini dans {}.",
        cout(plan.cout()),
        filets(plan.filet(), plan.nombre()),
        duree(plan.duree())
    )
}

/// "Charme de garde : 50 % de protection au monstre protégé d'une
/// expédition automatique" (section 12.5).
pub fn effet_du_charme() -> String {
    format!(
        "{PROTECTION_DU_CHARME} % de protection au monstre protégé d'une expédition automatique, un par expédition"
    )
}

pub fn question_du_charme(plan: &PlanDuCharme) -> String {
    format!(
        "Faire forger un charme de garde ({}) ? Gunther prendra {}, et aura fini dans {}. Un travail lancé ne s'annule pas.",
        effet_du_charme(),
        cout(plan.cout()),
        duree(plan.duree())
    )
}

pub fn lancement_du_charme(plan: &PlanDuCharme) -> String {
    format!(
        "Vous remettez à Gunther {}. Il se met à forger un charme de garde. Il aura fini dans {}.",
        cout(plan.cout()),
        duree(plan.duree())
    )
}

/// Pour la construction en cours : son temps restant, ou qu'elle est finie.
pub fn etat_du_chantier(chantier: &Chantier, maintenant: Horodatage) -> String {
    let niveau = ouvrage(&chantier.ouvrage);
    if chantier.est_fini(maintenant) {
        format!("{niveau} : fini, à constater.")
    } else {
        format!(
            "{niveau} : fini dans {}.",
            duree(maintenant.duree_jusqu_a(chantier.fin))
        )
    }
}

/// Au camp, quand Gunther a fini et attend que le joueur constate son
/// travail (section 13.4).
pub fn travail_fini(partie: &Partie, maintenant: Horodatage) -> Option<String> {
    let chantier = partie.forge().chantier()?;
    chantier.est_fini(maintenant).then(|| {
        format!(
            "Gunther a fini : {}. Passez à sa forge pour constater son travail.",
            ouvrage(&chantier.ouvrage)
        )
    })
}

/// Le niveau atteint de chaque amélioration, ce que coûte le suivant, et la
/// construction en cours.
pub fn etat_de_la_forge(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> String {
    let forge = partie.forge();
    let mut lignes = vec!["Forge de Gunther :".to_string()];
    for amelioration in Amelioration::TOUTES {
        let niveau = forge.ameliorations().niveau(amelioration);
        let actuel = if niveau == 0 {
            "aucun niveau".to_string()
        } else {
            format!("niveau {niveau} ({})", effet(amelioration, niveau))
        };
        let suivant = match contenu.plan_d_amelioration(amelioration, niveau + 1) {
            Some(plan) => format!(
                "Niveau suivant, {} : {}, {}.",
                effet(amelioration, plan.niveau()),
                cout(plan.cout()),
                duree(plan.duree())
            ),
            None => "Niveau maximal atteint.".to_string(),
        };
        lignes.push(format!(
            "  {} : {actuel}, sur {}. {suivant}",
            amelioration.nom(),
            amelioration.niveau_max()
        ));
    }
    let chantier = match forge.chantier() {
        Some(chantier) => format!("En cours : {}", etat_du_chantier(chantier, maintenant)),
        None => "Aucun travail en cours.".to_string(),
    };
    lignes.push(chantier);
    lignes.join("\n")
}

/// Chaque objet que Gunther sait forger, avec son coût et sa durée, ou
/// qu'il est déjà possédé (section 13.5).
pub fn etat_des_fabrications(partie: &Partie, contenu: &ContenuV1) -> String {
    let mut lignes = vec!["Objets que Gunther sait forger :".to_string()];
    for plan in contenu.plans_de_fabrication() {
        let detail = if partie.heros().equipement().possede(plan.objet().id()) {
            "possédé".to_string()
        } else {
            format!("{}, {}", cout(plan.cout()), duree(plan.duree()))
        };
        lignes.push(format!(
            "  {} : {}, {detail}.",
            plan.objet().sorte().nom(),
            objet_et_bonus(plan.objet())
        ));
    }
    lignes.join("\n")
}

pub fn erreur_amelioration(erreur: &ErreurAmelioration) -> String {
    match erreur {
        ErreurAmelioration::AucunArtisan => {
            "Aucun artisan au camp : quelqu'un, quelque part, sait forger.".to_string()
        }
        ErreurAmelioration::ChantierEnCours => {
            "Gunther est déjà au travail : il ne construit qu'une chose à la fois.".to_string()
        }
        ErreurAmelioration::NiveauMaximal => {
            "Cette amélioration est au niveau maximal.".to_string()
        }
        ErreurAmelioration::DejaPossede => "Vous possédez déjà cet objet.".to_string(),
        ErreurAmelioration::RessourcesManquantes(manquantes) => {
            format!("Il vous manque {}.", cout(manquantes))
        }
        ErreurAmelioration::AucunChantier => "Gunther n'a rien en construction.".to_string(),
        ErreurAmelioration::PasEncoreFini(_) => "Gunther n'a pas encore fini.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::filet::Filet;

    #[test]
    fn la_question_dit_le_cout_la_duree_et_l_effet() {
        let contenu = ContenuV1::new();
        let plan = contenu
            .plan_d_amelioration(Amelioration::Enclos, 1)
            .unwrap();
        assert_eq!(
            question_de_lancement(plan),
            "Faire construire Enclos, niveau 1 (camp de 40 places) ? Gunther prendra Croc de loup (5), Plume de corbeau (5) et Écaille de vipère (5), et aura fini dans 1 h. Une construction lancée ne s'annule pas."
        );
        let filets = contenu
            .plan_d_amelioration(Amelioration::Filets, 3)
            .unwrap();
        assert_eq!(
            niveau_d_amelioration(filets.amelioration(), filets.niveau()),
            "Filets, niveau 3 (chance de capture +30 %)"
        );
    }

    #[test]
    fn un_lot_de_filets_se_demande_se_lance_et_se_constate() {
        let contenu = ContenuV1::new();
        let plan = contenu.plan_des_filets(Filet::Leste).unwrap();
        assert_eq!(
            question_des_filets(plan),
            "Faire forger 5 filets lestés (chance de capture x1,5) ? Gunther prendra Fragment d'armure (3) et Os de squelette (3), et aura fini dans 1 h. Un travail lancé ne s'annule pas."
        );
        assert_eq!(
            lancement_des_filets(plan),
            "Vous remettez à Gunther Fragment d'armure (3) et Os de squelette (3). Il se met à forger 5 filets lestés. Il aura fini dans 1 h."
        );
        let chantier = Chantier {
            ouvrage: Ouvrage::Filets {
                filet: Filet::Leste,
                nombre: 5,
            },
            fin: Horodatage::depuis_secondes(3_600),
        };
        assert_eq!(
            constat(&chantier),
            "Gunther a fini : 5 filets lestés. Ils vont dans le coffre."
        );
    }

    #[test]
    fn le_chantier_dit_son_temps_restant_puis_qu_il_est_fini() {
        let chantier = Chantier {
            ouvrage: Ouvrage::Amelioration {
                amelioration: Amelioration::Enclos,
                niveau: 2,
            },
            fin: Horodatage::depuis_secondes(7_200),
        };
        assert_eq!(
            etat_du_chantier(&chantier, Horodatage::depuis_secondes(2_400)),
            "Enclos, niveau 2 (camp de 50 places) : fini dans 1 h 20."
        );
        assert_eq!(
            etat_du_chantier(&chantier, Horodatage::depuis_secondes(7_200)),
            "Enclos, niveau 2 (camp de 50 places) : fini, à constater."
        );
    }
}
