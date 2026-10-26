//! Combat sur Discord : toutes les actions du tour dans un seul message, puis
//! le remplacement d'un monstre KO, la capture des KO, l'attaque à oublier
//! et l'évolution.

use bersemon_core::arene::AdversaireDeLArene;
use bersemon_core::attaque::Attaque;
use bersemon_core::codex::Codex;
use bersemon_core::collection::Collection;
use bersemon_core::combat::{ApercuAttaque, Combat, Combattant, Cote};
use bersemon_core::espece::Espece;
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::soin::Soins;
use bersemon_core::types::Efficacite;
use bersemon_presentation::arene::annonce_du_palier;
use bersemon_presentation::combat::{attaque_de_trop, attaque_en_combat, etat, ordre_et_bonus};
use bersemon_presentation::filets::nom_du_filet;
use bersemon_presentation::monstres::{
    decrire_attaque, ligne_du_heros, ligne_monstre, nom, nom_en_combat,
};
use bersemon_presentation::partie::{places_pour_une_capture, proposition_d_evolution};
use bersemon_presentation::soins::nom_du_soin;
use poise::serenity_prelude as serenity;

use crate::composants::{Cle, soin_et_position_en_texte};
use crate::message::{
    DANGER, OPTIONS_PAR_MENU, OptionDeMenu, PRINCIPAL, Question, SECONDAIRE, SUCCES, bouton, menu,
    rangees,
};

/// Une rangée d'attaques, l'ordre du héros tant qu'il n'est pas donné
/// (section 4.2), le filet et la fuite (absents face à un Apôtre, section
/// 9.4), puis un menu pour changer de monstre et un pour les soins.
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
    if let Some(ordre) = combat.ordre_disponible() {
        let libelle = format!("Ordre : {}", ordre_et_bonus(ordre));
        autres.push(bouton(Cle::Ordre, &libelle, SECONDAIRE));
    }
    // Un bouton par qualité de filet qui reste : avec l'ordre et la fuite,
    // 5 au plus (section 11.2).
    if combat.filet_possible() {
        for (filet, nombre) in combat.filets().presents() {
            let libelle = format!(
                "{} x{nombre} ({} %)",
                nom_du_filet(filet),
                combat.chance_de_capture(filet)
            );
            autres.push(bouton(Cle::Filet(filet), &libelle, SECONDAIRE));
        }
    }
    if combat.fuite_possible() {
        autres.push(bouton(Cle::Fuir, "Fuir", SECONDAIRE));
    }
    // Dans la Fosse aux damnés, attaquer ou jeter son arme (section 9.8) ;
    // en duel d'arène, l'écran du duel a son propre abandon.
    if combat.est_dans_l_arene() && !combat.est_un_duel() {
        autres.push(bouton(Cle::AbandonnerLaFosse, "Abandonner", DANGER));
    }
    lignes.extend(rangees(autres));
    // En groupe, le joueur ne fait entrer que ses monstres (section 15.5).
    let remplacants = options_aptes(combat, |index| {
        index != actif.index && combat.proprietaire(index) == combat.membre_actif()
    });
    lignes.extend(menu(Cle::Changer, "Changer de monstre", remplacants));
    // Le héros du joueur qui agit se soigne aussi (section 4.4).
    let heros = combat
        .heros(Cote::Joueur)
        .iter()
        .enumerate()
        .map(|(position, heros)| (combat.equipe().len() + position, heros))
        .find(|(index, _)| combat.proprietaire(*index) == combat.membre_actif());
    lignes.extend(menu(
        Cle::SoinEnCombat,
        "Utiliser un soin",
        options_de_soins(combat.soins(), combat.equipe(), heros),
    ));
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

/// Chaque soin du sac avec chaque monstre à qui il peut servir : l'élixir
/// aux monstres KO, les autres soins aux monstres debout et au héros, à sa
/// position, qui suit l'équipe (section 4.4). Au plus 4 soins pour 4
/// monstres et le héros : 20 options, sous la limite de Discord ; en
/// groupe, avec jusqu'à 8 monstres, le menu s'arrête à la limite (section
/// 15.5).
pub fn options_de_soins(
    soins: &Soins,
    equipe: &[Monstre],
    heros: Option<(usize, &Monstre)>,
) -> Vec<OptionDeMenu> {
    let mut options = Vec::new();
    for (soin, nombre) in soins.presents() {
        for (index, monstre) in equipe.iter().enumerate() {
            if monstre.est_ko() != soin.releve() {
                continue;
            }
            options.push(OptionDeMenu {
                libelle: format!(
                    "{} ({nombre}) : {}",
                    nom_du_soin(soin),
                    nom(monstre, Cote::Joueur)
                ),
                valeur: soin_et_position_en_texte(soin, index),
                description: Some(ligne_monstre(monstre, Cote::Joueur)),
            });
        }
        if let Some((index, heros)) = heros.filter(|_| !soin.releve()) {
            options.push(OptionDeMenu {
                libelle: format!("{} ({nombre}) : {}", nom_du_soin(soin), heros.nom()),
                valeur: soin_et_position_en_texte(soin, index),
                description: Some(ligne_du_heros(heros)),
            });
        }
    }
    options.truncate(OPTIONS_PAR_MENU);
    options
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

/// En groupe, seulement les monstres de celui qui remplace (section 15.5).
pub fn remplacement(combat: &Combat, codex: &Codex) -> Question {
    let qui_remplace = combat.membre_qui_remplace();
    let boutons = combat
        .equipe()
        .iter()
        .enumerate()
        .filter(|(index, monstre)| {
            !monstre.est_ko()
                && qui_remplace.is_none_or(|membre| combat.proprietaire(*index) == membre)
        })
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
/// Repoussée, l'évolution sera proposée de nouveau au prochain niveau gagné
/// (section 5.4).
pub fn evolution(monstre: &Monstre, vers: &Espece) -> Question {
    Question {
        texte: proposition_d_evolution(monstre, vers),
        rangees: rangees(vec![
            bouton(Cle::Evoluer, "Évoluer", PRINCIPAL),
            bouton(Cle::NePasEvoluer, "Pas maintenant", SECONDAIRE),
        ]),
    }
}

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

/// Le palier de la Fosse aux damnés et son adversaire : combattre, ou
/// revenir au camp (section 9.8).
pub fn devant_l_adversaire(palier: u8, adversaire: &AdversaireDeLArene) -> Question {
    Question {
        texte: annonce_du_palier(palier, adversaire),
        rangees: rangees(vec![
            bouton(Cle::Poursuivre, "Combattre", PRINCIPAL),
            bouton(Cle::Rentrer, "Revenir au camp", SECONDAIRE),
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::heros::{Classe, Heros};
    use bersemon_core::nom::Nom;
    use bersemon_core::soin::Soin;

    /// Section 9.8 : dans la Fosse, attaquer ou abandonner, rien d'autre.
    #[test]
    fn dans_la_fosse_le_heros_attaque_ou_abandonne() {
        use crate::message::Rangee;
        use bersemon_core::contenu::ContenuV1;

        let contenu = ContenuV1::new();
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier);
        let gueux = contenu.fosse().adversaire(1).unwrap();
        let combat =
            Combat::dans_l_arene(heros.combattant(), gueux.generer_heros(), gueux.ia()).unwrap();
        let depart = heros.combattant();
        let partie = Partie::nouvelle(heros, depart);
        let question = action(&combat, &partie);
        let mut cles = Vec::new();
        for rangee in &question.rangees {
            match rangee {
                Rangee::Boutons(boutons) => cles.extend(boutons.iter().map(|b| b.cle.clone())),
                Rangee::Menu(menu) => cles.push(menu.cle.clone()),
            }
        }
        assert_eq!(cles, vec![Cle::Attaquer(0), Cle::AbandonnerLaFosse]);
        let annonce = devant_l_adversaire(1, gueux);
        assert!(annonce.texte.contains("Le Gueux"));
    }

    /// Section 4.4 : les soins servent au héros, sauf l'élixir.
    #[test]
    fn les_soins_se_proposent_aussi_pour_le_heros() {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier).combattant();
        let mut soins = Soins::default();
        soins.ajouter(Soin::Potion, 2);
        soins.ajouter(Soin::Elixir, 1);
        let options = options_de_soins(&soins, &[], Some((0, &heros)));
        let libelles: Vec<&str> = options
            .iter()
            .map(|option| option.libelle.as_str())
            .collect();
        assert_eq!(libelles, ["Potion (2) : Aldric"]);
        assert_eq!(
            options[0].description.as_deref(),
            Some("Aldric (héros, niv. 5) : 33/33 PV")
        );
    }
}
