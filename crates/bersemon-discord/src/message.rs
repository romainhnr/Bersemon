//! Assemblage des messages du bot : ce qui vient de se passer, un éventuel
//! refus, puis la question en cours avec ses composants. Les écrans décrivent
//! leurs composants ; ils ne deviennent des composants Discord qu'à l'envoi,
//! ce qui permet aux tests de jouer à travers eux. Les limites de Discord
//! sont tenues ici : 2000 caractères par message, 5 rangées, 5 boutons par
//! rangée, 25 options par menu.

use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{Attente, Confirmation, Evenement};
use bersemon_core::horodatage::Horodatage;
use bersemon_presentation::erreurs::erreur_de_choix;
use bersemon_presentation::partie::{compteur_d_etapes, question_de_confirmation};
use bersemon_presentation::recit::raconter;
use poise::serenity_prelude as serenity;

use crate::composants::{Cle, Composant, Vue};
use crate::duels::erreur_de_defi;
use crate::echanges::erreur_d_offre;
use crate::groupes::erreur_d_invitation;
use crate::joueurs::{Refus, Session};
use crate::{
    ecran_accueil, ecran_camp, ecran_combat, ecran_duel, ecran_echanges, ecran_expedition,
    ecran_groupe, ecran_peche,
};

pub const LONGUEUR_MAX: usize = 2000;
pub const RANGEES_MAX: usize = 5;
pub const BOUTONS_PAR_RANGEE: usize = 5;
pub const OPTIONS_PAR_MENU: usize = 25;
/// Libellé d'un bouton.
const LIBELLE_MAX: usize = 80;
/// Libellé, valeur ou description d'une option de menu ; indication d'un
/// menu.
const TEXTE_D_OPTION_MAX: usize = 100;

pub const PRINCIPAL: serenity::ButtonStyle = serenity::ButtonStyle::Primary;
pub const SECONDAIRE: serenity::ButtonStyle = serenity::ButtonStyle::Secondary;
pub const SUCCES: serenity::ButtonStyle = serenity::ButtonStyle::Success;
pub const DANGER: serenity::ButtonStyle = serenity::ButtonStyle::Danger;

pub struct Bouton {
    pub cle: Cle,
    pub libelle: String,
    pub style: serenity::ButtonStyle,
}

#[derive(Clone)]
pub struct OptionDeMenu {
    pub libelle: String,
    pub valeur: String,
    pub description: Option<String>,
}

pub struct Menu {
    pub cle: Cle,
    pub indication: String,
    pub options: Vec<OptionDeMenu>,
}

pub enum Rangee {
    Boutons(Vec<Bouton>),
    Menu(Menu),
}

/// La question en cours : son texte et ses rangées de composants.
pub struct Question {
    pub texte: String,
    pub rangees: Vec<Rangee>,
}

pub struct Message {
    pub texte: String,
    pub rangees: Vec<Rangee>,
    /// Version de la partie, portée par chaque composant.
    pub version: u32,
}

impl Message {
    pub fn composants(&self) -> Vec<serenity::CreateActionRow> {
        self.rangees
            .iter()
            .map(|rangee| match rangee {
                Rangee::Boutons(boutons) => serenity::CreateActionRow::Buttons(
                    boutons
                        .iter()
                        .map(|bouton| {
                            serenity::CreateButton::new(self.identifiant(&bouton.cle))
                                .label(couper(&bouton.libelle, LIBELLE_MAX))
                                .style(bouton.style)
                        })
                        .collect(),
                ),
                Rangee::Menu(menu) => {
                    let options = menu
                        .options
                        .iter()
                        .map(|option| {
                            let creee = serenity::CreateSelectMenuOption::new(
                                couper(&option.libelle, TEXTE_D_OPTION_MAX),
                                couper(&option.valeur, TEXTE_D_OPTION_MAX),
                            );
                            match &option.description {
                                Some(description) => {
                                    creee.description(couper(description, TEXTE_D_OPTION_MAX))
                                }
                                None => creee,
                            }
                        })
                        .collect();
                    serenity::CreateActionRow::SelectMenu(
                        serenity::CreateSelectMenu::new(
                            self.identifiant(&menu.cle),
                            serenity::CreateSelectMenuKind::String { options },
                        )
                        .placeholder(couper(&menu.indication, TEXTE_D_OPTION_MAX)),
                    )
                }
            })
            .collect()
    }

    fn identifiant(&self, cle: &Cle) -> String {
        Composant {
            version: self.version,
            cle: cle.clone(),
        }
        .identifiant()
    }
}

/// `maintenant` décide de ce que montrent les expéditions automatiques :
/// rentrées ou encore en cours. Un récit trop long perd ses plus anciens
/// passages, pour que la question reste entière : en groupe, celui des
/// choix du compagnon peut s'allonger (section 15.5).
pub fn message(
    session: &Session,
    evenements: &[Evenement],
    refus: Option<&Refus>,
    contenu: &ContenuV1,
    maintenant: Horodatage,
) -> Message {
    let recits = raconter(evenements);
    let question = question(session, contenu, maintenant);
    let composer = |depuis: usize| {
        let mut texte = String::new();
        if depuis > 0 {
            texte.push_str("...");
        }
        for recit in &recits[depuis..] {
            if !texte.is_empty() {
                texte.push_str(if recit.nouveau_paragraphe {
                    "\n\n"
                } else {
                    "\n"
                });
            }
            texte.push_str(&recit.texte);
        }
        if let Some(refus) = refus {
            ajouter_paragraphe(&mut texte, &expliquer(refus));
        }
        ajouter_paragraphe(&mut texte, &question.texte);
        // En expédition, le compteur d'étapes est toujours en haut
        // (section 9.2).
        if let Some(expedition) = session.deroulement.expedition() {
            texte = format!("{}\n\n{texte}", compteur_d_etapes(expedition));
        }
        echapper_les_debuts_de_ligne(&texte)
    };
    let mut depuis = 0;
    let mut texte = composer(depuis);
    while texte.chars().count() > LONGUEUR_MAX && depuis < recits.len() {
        depuis += 1;
        texte = composer(depuis);
    }
    let mut rangees = question.rangees;
    rangees.truncate(RANGEES_MAX);
    Message {
        texte: couper(&texte, LONGUEUR_MAX),
        rangees,
        version: session.version,
    }
}

/// Discord lit « 1. » en début de ligne comme une liste numérotée, « > »
/// comme une citation et « - » ou « * » comme une puce, et les met en forme
/// à sa façon : les numéros des listes du jeu se décaleraient. Une barre
/// oblique inverse les fait afficher tels quels.
fn echapper_les_debuts_de_ligne(texte: &str) -> String {
    texte
        .split('\n')
        .map(|ligne| {
            let reste = ligne.trim_start();
            let retrait = &ligne[..ligne.len() - reste.len()];
            let chiffres = reste.chars().take_while(char::is_ascii_digit).count();
            if chiffres > 0 && reste[chiffres..].starts_with(". ") {
                format!("{retrait}{}\\{}", &reste[..chiffres], &reste[chiffres..])
            } else if ["> ", "- ", "* "]
                .iter()
                .any(|marque| reste.starts_with(marque))
            {
                format!("{retrait}\\{reste}")
            } else {
                ligne.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn expliquer(refus: &Refus) -> String {
    match refus {
        Refus::MessagePerime => {
            "Ce message n'était plus à jour : voici où en est votre partie.".to_string()
        }
        Refus::Choix { choix, erreur } => erreur_de_choix(choix, erreur),
        Refus::Echange(erreur) => erreur_d_offre(erreur),
        Refus::Duel(erreur) => erreur_de_defi(erreur),
        Refus::Groupe(erreur) => erreur_d_invitation(erreur),
    }
}

/// Au camp et en expédition, une partie est en jeu.
const PARTIE_EN_JEU: &str = "une partie est en jeu";

fn question(session: &Session, contenu: &ContenuV1, maintenant: Horodatage) -> Question {
    // Le duel remplace le camp à l'écran (section 15.3).
    if let (Some(duel), Some(partie)) = (&session.duel, session.deroulement.partie()) {
        return ecran_duel::question(duel, partie, maintenant);
    }
    // En groupe, le combat commun et l'attente du compagnon ont leurs
    // écrans ; les autres questions sont celles d'une expédition seul, sous
    // l'en-tête de la descente (section 15.5).
    if let (Some(groupe), Some(partie)) = (&session.groupe, session.deroulement.partie()) {
        let attente = session.deroulement.attente();
        if let Some(question) = ecran_groupe::question(groupe, partie, attente, maintenant) {
            return question;
        }
        let mut question = question_de_la_partie(session, contenu, maintenant);
        question.texte = format!(
            "{}\n\n{}",
            ecran_groupe::entete(groupe, attente),
            question.texte
        );
        return question;
    }
    question_de_la_partie(session, contenu, maintenant)
}

fn question_de_la_partie(
    session: &Session,
    contenu: &ContenuV1,
    maintenant: Horodatage,
) -> Question {
    let partie = || session.deroulement.partie().expect(PARTIE_EN_JEU);
    match session.deroulement.attente() {
        Attente::Reprise(partie) => ecran_accueil::reprise(partie),
        Attente::SauvegardeIllisible(erreur) => ecran_accueil::sauvegarde_illisible(erreur),
        Attente::NomDuHeros => ecran_accueil::nom_du_heros(),
        Attente::Classe => ecran_accueil::classe(contenu),
        Attente::Camp => match session.vue {
            Vue::Echanges { avec, cede } => {
                ecran_echanges::echanges(partie(), &session.echanges, avec, cede)
            }
            Vue::Classement => ecran_echanges::classement(&session.echanges),
            Vue::RecordsDeLaTour => ecran_echanges::records_de_la_tour(partie(), &session.echanges),
            vue => ecran_camp::question(partie(), vue, contenu, maintenant),
        },
        Attente::EntreDeuxEtapes => ecran_expedition::entre_deux_etapes(partie()),
        Attente::MonstreRare(_) => ecran_expedition::monstre_rare(),
        Attente::Prise(monstre) => ecran_peche::prise(monstre),
        Attente::PlaceManquante(capture) => {
            ecran_expedition::place_manquante(capture, partie().collection())
        }
        Attente::ActionDeCombat(combat) => ecran_combat::action(combat, partie()),
        Attente::Remplacement(combat) => ecran_combat::remplacement(combat, partie().codex()),
        Attente::CaptureDesKo(combat) => ecran_combat::capture(combat, partie().collection()),
        Attente::OubliAttaque { monstre, nouvelle } => ecran_combat::oubli(monstre, nouvelle),
        Attente::Evolution { monstre, vers } => ecran_combat::evolution(monstre, vers),
        Attente::Annonce(annonce) => ecran_expedition::annonce(annonce),
        Attente::Confirmation(confirmation) => confirmer(confirmation),
        Attente::SacPlein => ecran_expedition::sac_plein(partie()),
        Attente::AdversaireDeLArene { palier, adversaire } => {
            ecran_combat::devant_l_adversaire(palier, adversaire)
        }
        // Le combat commun et l'attente du compagnon passent par
        // `ecran_groupe` ; sans expédition en groupe, rien à faire.
        Attente::CombatDeGroupe | Attente::AttenteDuGroupe => Question {
            texte: String::new(),
            rangees: rangees(vec![bouton(Cle::Actualiser, "Actualiser", PRINCIPAL)]),
        },
    }
}

fn confirmer(confirmation: &Confirmation) -> Question {
    Question {
        texte: question_de_confirmation(confirmation),
        rangees: rangees(vec![
            bouton(Cle::Oui, "Oui", DANGER),
            bouton(Cle::Non, "Non", SECONDAIRE),
        ]),
    }
}

pub fn ajouter_paragraphe(texte: &mut String, paragraphe: &str) {
    if paragraphe.is_empty() {
        return;
    }
    if !texte.is_empty() {
        texte.push_str("\n\n");
    }
    texte.push_str(paragraphe);
}

pub fn bouton(cle: Cle, libelle: &str, style: serenity::ButtonStyle) -> Bouton {
    Bouton {
        cle,
        libelle: libelle.to_string(),
        style,
    }
}

/// Les boutons, cinq par rangée.
pub fn rangees(boutons: Vec<Bouton>) -> Vec<Rangee> {
    let mut rangees = Vec::new();
    let mut restants = boutons.into_iter().peekable();
    while restants.peek().is_some() {
        rangees.push(Rangee::Boutons(
            restants.by_ref().take(BOUTONS_PAR_RANGEE).collect(),
        ));
    }
    rangees
}

/// `None` sans option. Les appelants découpent eux-mêmes les listes de plus
/// de 25 options.
pub fn menu(cle: Cle, indication: &str, options: Vec<OptionDeMenu>) -> Option<Rangee> {
    if options.is_empty() {
        return None;
    }
    Some(Rangee::Menu(Menu {
        cle,
        indication: indication.to_string(),
        options,
    }))
}

/// Coupe un texte trop long pour Discord plutôt que de voir le message
/// refusé.
pub fn couper(texte: &str, longueur_max: usize) -> String {
    if texte.chars().count() <= longueur_max {
        return texte.to_string();
    }
    let mut coupe: String = texte.chars().take(longueur_max - 3).collect();
    coupe.push_str("...");
    coupe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_texte_trop_long_est_coupe() {
        let coupe = couper(&"x".repeat(LONGUEUR_MAX + 10), LONGUEUR_MAX);
        assert_eq!(coupe.chars().count(), LONGUEUR_MAX);
        assert!(coupe.ends_with("..."));
        assert_eq!(couper("court", 10), "court");
    }

    #[test]
    fn cinq_boutons_par_rangee() {
        let boutons: Vec<_> = (0..7)
            .map(|index| bouton(Cle::Attaquer(index), "Attaque", PRINCIPAL))
            .collect();
        assert_eq!(rangees(boutons).len(), 2);
    }

    #[test]
    fn les_listes_ne_sont_pas_mises_en_forme_par_discord() {
        let texte = "Équipe (2/3) :\n  1. Salamandre\n  2. Loup\n  > actif\n- puce\n**Coup critique !**\nNiveau 12. Rien";
        assert_eq!(
            echapper_les_debuts_de_ligne(texte),
            "Équipe (2/3) :\n  1\\. Salamandre\n  2\\. Loup\n  \\> actif\n\\- puce\n**Coup critique !**\nNiveau 12. Rien"
        );
    }

    #[test]
    fn un_menu_sans_option_n_est_pas_cree() {
        assert!(menu(Cle::Soin, "Soin", Vec::new()).is_none());
    }
}
