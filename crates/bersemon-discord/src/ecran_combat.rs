//! Combat sur Discord : toutes les actions du tour dans un seul message, puis
//! le remplacement d'un monstre KO, la capture des KO et l'attaque à oublier.

use bersemon_core::attaque::Attaque;
use bersemon_core::codex::Codex;
use bersemon_core::collection::Collection;
use bersemon_core::combat::{ApercuAttaque, Combat, Combattant, Cote};
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::types::Efficacite;
use bersemon_presentation::combat::{attaque_de_trop, attaque_en_combat, etat};
use bersemon_presentation::monstres::{decrire_attaque, ligne_monstre, nom, nom_en_combat};
use bersemon_presentation::partie::places_pour_une_capture;
use poise::serenity_prelude as serenity;

use crate::composants::Cle;
use crate::message::{
    DANGER, OptionDeMenu, PRINCIPAL, Question, SECONDAIRE, SUCCES, bouton, menu, rangees,
};

/// Une rangée d'attaques, le filet et la fuite (absents face à un Apôtre,
/// section 9.4), puis un menu pour changer de monstre et un pour la potion.
/// Chaque attaque dit ses dégâts estimés, et sa couleur son efficacité
/// (section 10.6).
pub fn action(combat: &Combat, partie: &Partie) -> Question {
    let actif = combat.actif(Cote::Joueur);
    let attaques = combat
        .monstre(actif)
        .attaques()
        .liste()
        .iter()
        .enumerate()
        .map(|(index, attaque)| {
            let libelle =
                attaque_en_combat(combat, index).unwrap_or_else(|| attaque.nom().to_string());
            let style = combat
                .apercu_attaque(index)
                .map_or(SECONDAIRE, style_d_attaque);
            bouton(Cle::Attaquer(index), &libelle, style)
        })
        .collect();
    let mut lignes = rangees(attaques);
    let mut autres = Vec::new();
    if combat.filet_possible() {
        let libelle = format!(
            "Lancer un filet ({} % de chances)",
            combat.chance_de_capture()
        );
        autres.push(bouton(Cle::Filet, &libelle, SECONDAIRE));
    }
    if combat.fuite_possible() {
        autres.push(bouton(Cle::Fuir, "Fuir", SECONDAIRE));
    }
    lignes.extend(rangees(autres));
    let remplacants = options_aptes(combat, |index| index != actif.index);
    lignes.extend(menu(Cle::Changer, "Changer de monstre", remplacants));
    if combat.potions() > 0 {
        let soignables = options_aptes(combat, |_| true);
        lignes.extend(menu(
            Cle::PotionEnCombat,
            &format!("Utiliser une potion ({}) sur", combat.potions()),
            soignables,
        ));
    }
    let mut texte = etat(combat, partie.codex());
    if combat.filet_possible() {
        texte.push('\n');
        texte.push_str(&places_pour_une_capture(partie.collection()));
    }
    Question {
        texte: format!("{texte}\n\nQue faire ?"),
        rangees: lignes,
    }
}

/// Vert : très efficace ; bleu : bonus de type ; rouge : peu efficace ou
/// sans effet ; gris sinon. Très efficace l'emporte sur le bonus de type,
/// et une attaque peu efficace reste rouge même avec le bonus.
fn style_d_attaque(apercu: ApercuAttaque) -> serenity::ButtonStyle {
    match apercu.efficacite {
        Efficacite::Forte | Efficacite::TresForte => SUCCES,
        Efficacite::Reduite | Efficacite::TresReduite | Efficacite::Nulle => DANGER,
        Efficacite::Normale if apercu.bonus_de_type => PRINCIPAL,
        Efficacite::Normale => SECONDAIRE,
    }
}

/// Les monstres de l'équipe en état de combattre que `garder` retient.
fn options_aptes(combat: &Combat, garder: impl Fn(usize) -> bool) -> Vec<OptionDeMenu> {
    combat
        .equipe()
        .iter()
        .enumerate()
        .filter(|(index, monstre)| !monstre.est_ko() && garder(*index))
        .map(|(index, monstre)| OptionDeMenu {
            libelle: nom(monstre, Cote::Joueur),
            valeur: index.to_string(),
            description: Some(ligne_monstre(monstre, Cote::Joueur)),
        })
        .collect()
}

pub fn remplacement(combat: &Combat, codex: &Codex) -> Question {
    let boutons = combat
        .equipe()
        .iter()
        .enumerate()
        .filter(|(_, monstre)| !monstre.est_ko())
        .map(|(index, monstre)| {
            bouton(
                Cle::Remplacer(index),
                &nom(monstre, Cote::Joueur),
                PRINCIPAL,
            )
        })
        .collect();
    Question {
        texte: format!("{}\n\nQuel monstre envoyer ?", etat(combat, codex)),
        rangees: rangees(boutons),
    }
}

/// Plusieurs monstres KO peuvent être capturés après une victoire (Q29).
pub fn capture(combat: &Combat, collection: &Collection) -> Question {
    let mut boutons: Vec<_> = combat
        .adversaires_capturables()
        .into_iter()
        .map(|index| {
            let combattant = Combattant {
                cote: Cote::Adversaire,
                index,
            };
            let libelle = format!("Capturer {}", nom_en_combat(combat, combattant));
            bouton(Cle::Capturer(index), &libelle, PRINCIPAL)
        })
        .collect();
    boutons.push(bouton(Cle::NePlusCapturer, "Ne pas capturer", SECONDAIRE));
    Question {
        texte: format!(
            "Capturer un monstre KO ?\n{}",
            places_pour_une_capture(collection)
        ),
        rangees: rangees(boutons),
    }
}

/// Le monstre connaît déjà 4 attaques : le joueur en oublie une pour
/// apprendre la nouvelle, ou y renonce (Q33).
pub fn oubli(monstre: &Monstre, nouvelle: &Attaque) -> Question {
    let connues = monstre.attaques().liste();
    let lignes: Vec<String> = connues
        .iter()
        .enumerate()
        .map(|(position, attaque)| format!("  {}. {}", position + 1, decrire_attaque(attaque)))
        .collect();
    let mut boutons: Vec<_> = connues
        .iter()
        .enumerate()
        .map(|(position, attaque)| {
            let libelle = format!("Oublier {}", attaque.nom());
            bouton(Cle::Oublier(position), &libelle, SECONDAIRE)
        })
        .collect();
    boutons.push(bouton(
        Cle::Renoncer,
        &format!("Renoncer à {}", nouvelle.nom()),
        PRINCIPAL,
    ));
    Question {
        texte: format!(
            "{}\nQuelle attaque oublier ?\n{}",
            attaque_de_trop(monstre, nouvelle),
            lignes.join("\n")
        ),
        rangees: rangees(boutons),
    }
}
