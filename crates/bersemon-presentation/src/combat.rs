//! Combat : rencontre, état des deux camps, événements du tour et suites
//! (XP, attaques apprises ou oubliées).

use bersemon_core::attaque::{Attaque, Categorie};
use bersemon_core::codex::Codex;
use bersemon_core::combat::{Combat, Combattant, Cote, Evenement, Issue};
use bersemon_core::contenu::METAMORPHOSE_DU_DEMIURGE;
use bersemon_core::heros::Ordre;
use bersemon_core::monstre::{Monstre, RapportExperience};
use bersemon_core::statut::Statut;
use bersemon_core::types::Efficacite;

use crate::equipement::decrire_bonus;
use crate::mise_en_valeur::important;
use crate::monstres::{
    decrire_attaque, ligne_monstre, nom, nom_en_combat, nom_en_valeur, progression_d_experience,
};
use crate::partie::types_connus;
use crate::soins::soin_utilise;

/// La seconde forme du Démiurge reste cachée jusqu'à son entrée (section
/// 9.4).
pub fn rencontre(combat: &Combat) -> String {
    let visibles = if combat.est_en_deux_phases() {
        1
    } else {
        combat.adversaires().len()
    };
    let noms: Vec<String> = (0..visibles)
        .map(|index| {
            important(&nom_en_combat(
                combat,
                Combattant {
                    cote: Cote::Adversaire,
                    index,
                },
            ))
        })
        .collect();
    format!("Des monstres surgissent : {}.", noms.join(", "))
}

/// L'équipe, avec un repère devant le monstre actif, puis le monstre adverse
/// actif, avec ses types si son espèce est capturée (section 10.6), et ceux
/// qui attendent.
pub fn etat(combat: &Combat, codex: &Codex) -> String {
    let actif_joueur = combat.actif(Cote::Joueur);
    let mut lignes = vec!["Votre équipe :".to_string()];
    for (index, monstre) in combat.equipe().iter().enumerate() {
        let repere = if index == actif_joueur.index {
            ">"
        } else {
            " "
        };
        lignes.push(format!(
            "  {repere} {}",
            ligne_monstre(monstre, Cote::Joueur)
        ));
    }
    let adversaire = combat.actif(Cote::Adversaire);
    let monstre = combat.monstre(adversaire);
    lignes.push(format!(
        "En face : {} ({}, niv. {}) : {}/{} PV{}",
        nom_en_combat(combat, adversaire),
        types_connus(codex, monstre.espece()),
        monstre.niveau().valeur(),
        monstre.pv_actuels(),
        monstre.stats().pv,
        etat_du_monstre(monstre)
    ));
    let en_attente = combat
        .adversaires()
        .iter()
        .enumerate()
        .filter(|&(index, monstre)| index > adversaire.index && !monstre.est_ko())
        .count();
    if en_attente > 0 && !combat.est_en_deux_phases() {
        lignes.push(format!("  et {en_attente} autre(s) monstre(s) en attente"));
    }
    lignes.join("\n")
}

/// Ce qu'une attaque ferait au monstre adverse actif : dégâts estimés sans
/// coup critique, précision, catégorie et effet (section 10.6).
pub fn attaque_en_combat(combat: &Combat, index_attaque: usize) -> Option<String> {
    let attaque = combat
        .monstre(combat.actif(Cote::Joueur))
        .attaques()
        .liste()
        .get(index_attaque)?;
    let apercu = combat.apercu_attaque(index_attaque)?;
    let degats = if apercu.efficacite == Efficacite::Nulle {
        "sans effet".to_string()
    } else if apercu.degats_min == apercu.degats_max {
        format!("{} dégâts", apercu.degats_max)
    } else {
        format!("{} à {} dégâts", apercu.degats_min, apercu.degats_max)
    };
    let categorie = match attaque.categorie() {
        Categorie::Physique => "Phy",
        Categorie::Speciale => "Spé",
    };
    let effet = match attaque.effet() {
        Some(effet) => {
            let statut = match effet.statut() {
                Statut::Poison => "poison",
                Statut::Paralysie => "paralysie",
            };
            format!(", {statut} {} %", effet.chance())
        }
        None => String::new(),
    };
    Some(format!(
        "{} : {degats}, {} %, {categorie}{effet}",
        attaque.nom(),
        attaque.precision()
    ))
}

/// KO ou statut, à la suite des PV.
fn etat_du_monstre(monstre: &Monstre) -> &'static str {
    if monstre.est_ko() {
        return ", KO";
    }
    match monstre.statut() {
        Some(Statut::Poison) => ", poison",
        Some(Statut::Paralysie) => ", paralysie",
        None => "",
    }
}

/// Les noms des monstres sont mis en valeur, sauf dans une phrase déjà mise
/// en valeur tout entière.
pub fn evenement(evenement: &Evenement, combat: &Combat) -> String {
    let nom_de = |combattant: Combattant| nom_en_combat(combat, combattant);
    let fort = |combattant: Combattant| important(&nom_de(combattant));
    match evenement {
        Evenement::MonstreEnvoye(combattant) => match combattant.cote {
            Cote::Joueur => format!("Vous envoyez {}.", fort(*combattant)),
            Cote::Adversaire => format!("{} entre en combat.", fort(*combattant)),
        },
        Evenement::Metamorphose(_) => important(METAMORPHOSE_DU_DEMIURGE),
        Evenement::AttaqueLancee { lanceur, attaque } => {
            format!("{} utilise {}.", fort(*lanceur), attaque.nom())
        }
        Evenement::AttaqueRatee(lanceur) => format!("L'attaque de {} rate.", fort(*lanceur)),
        Evenement::SansEffet(cible) => {
            important(&format!("Cela n'affecte pas {}.", nom_de(*cible)))
        }
        Evenement::Degats {
            cible,
            degats,
            critique,
            efficacite,
        } => {
            let critique = if *critique {
                format!("{} ", important("Coup critique !"))
            } else {
                String::new()
            };
            let efficacite = phrase_efficacite(*efficacite)
                .map(|phrase| format!("{} ", important(phrase)))
                .unwrap_or_default();
            format!("{critique}{efficacite}{} perd {degats} PV.", fort(*cible))
        }
        Evenement::StatutInflige { cible, statut } => important(&match statut {
            Statut::Poison => format!("Le poison atteint {}.", nom_de(*cible)),
            Statut::Paralysie => format!("La paralysie frappe {}.", nom_de(*cible)),
        }),
        Evenement::BloqueParLaParalysie(combattant) => important(&format!(
            "La paralysie empêche {} d'attaquer.",
            nom_de(*combattant)
        )),
        Evenement::Desobeissance(combattant) => important(&format!(
            "{} n'en fait qu'à sa tête et ignore votre ordre.",
            nom_de(*combattant)
        )),
        Evenement::DegatsDePoison { cible, degats } => important(&format!(
            "Le poison fait perdre {degats} PV à {}.",
            nom_de(*cible)
        )),
        Evenement::Ko(combattant) => important(&format!("{} est KO.", nom_de(*combattant))),
        Evenement::FiletLance { cible, chance } => format!(
            "Vous lancez un filet sur {} ({chance} % de chances).",
            fort(*cible)
        ),
        Evenement::CaptureReussie(cible) => important(&format!(
            "Capture réussie : {}.",
            combat.monstre(*cible).nom()
        )),
        Evenement::CaptureRatee(cible) => format!("{} se dégage du filet.", fort(*cible)),
        Evenement::SoinUtilise { soin, cible, effet } => soin_utilise(*soin, &fort(*cible), *effet),
        Evenement::Fuite => "Vous prenez la fuite.".to_string(),
        Evenement::FinDuCombat(Issue::Victoire) => "Victoire !".to_string(),
        Evenement::FinDuCombat(Issue::Defaite) => important("Défaite : toute votre équipe est KO."),
        Evenement::FinDuCombat(Issue::Fuite) => "Le combat est terminé.".to_string(),
        // Le récit d'un duel nomme le héros qui abandonne ou perd par
        // forfait (`duels::raconter`) : ces phrases ne servent qu'à défaut.
        Evenement::Abandon(cote) => match cote {
            Cote::Joueur => "Vous abandonnez le duel.".to_string(),
            Cote::Adversaire => "Votre adversaire abandonne le duel.".to_string(),
        },
        Evenement::Forfait(cote) => match cote {
            Cote::Joueur => "Vous perdez le duel par forfait.".to_string(),
            Cote::Adversaire => "Votre adversaire perd le duel par forfait.".to_string(),
        },
    }
}

/// "Cri de guerre (+20 % d'Attaque)".
pub fn ordre_et_bonus(ordre: Ordre) -> String {
    format!("{} ({})", ordre.nom(), decrire_bonus(&ordre.bonus()))
}

/// `monstre` est le nom, déjà mis en valeur, du monstre qui reçoit l'ordre
/// (docs/contenu-v1.md, section 12).
pub fn ordre_donne(heros: &str, ordre: Ordre, monstre: &str) -> String {
    let geste = match ordre {
        Ordre::CriDeGuerre => "pousse un cri de guerre",
        Ordre::Incantation => "murmure une incantation",
        Ordre::BouclierDeFoi => "invoque un bouclier de foi",
        Ordre::PasDeLOmbre => "souffle le pas de l'ombre",
    };
    let bonus = decrire_bonus(&ordre.bonus());
    format!(
        "{heros} {geste} : {monstre} gagne {} jusqu'à la fin du combat.",
        bonus.trim_start_matches('+')
    )
}

fn phrase_efficacite(efficacite: Efficacite) -> Option<&'static str> {
    match efficacite {
        Efficacite::TresForte => Some("C'est extrêmement efficace !"),
        Efficacite::Forte => Some("C'est très efficace !"),
        Efficacite::Normale | Efficacite::Nulle => None,
        Efficacite::Reduite => Some("Ce n'est pas très efficace."),
        Efficacite::TresReduite => Some("Ce n'est presque pas efficace."),
    }
}

/// `monstre` est pris après le gain : sa progression vise le niveau qui suit
/// celui qu'il vient d'atteindre.
pub fn rapport_experience(monstre: &Monstre, rapport: &RapportExperience) -> String {
    let nom = nom_en_valeur(monstre, Cote::Joueur);
    let mut lignes = vec![format!(
        "{nom} gagne {} XP ({}).",
        rapport.experience_gagnee,
        progression_d_experience(monstre)
    )];
    if rapport.niveau_apres > rapport.niveau_avant {
        lignes.push(format!(
            "{nom} passe au niveau {}.",
            rapport.niveau_apres.valeur()
        ));
    }
    for attaque in &rapport.attaques_apprises {
        lignes.push(format!("{nom} apprend {}.", attaque.nom()));
    }
    lignes.join("\n")
}

/// Le monstre connaît déjà 4 attaques (Q33).
pub fn attaque_de_trop(monstre: &Monstre, nouvelle: &Attaque) -> String {
    format!(
        "{} veut apprendre {}, mais connaît déjà 4 attaques.",
        nom(monstre, Cote::Joueur),
        decrire_attaque(nouvelle)
    )
}

pub fn renoncement(monstre: &Monstre, nouvelle: &Attaque) -> String {
    format!(
        "{} n'apprend pas {}.",
        nom_en_valeur(monstre, Cote::Joueur),
        nouvelle.nom()
    )
}

pub fn remplacement(monstre: &Monstre, oubliee: &Attaque, nouvelle: &Attaque) -> String {
    format!(
        "{} oublie {} et apprend {}.",
        nom_en_valeur(monstre, Cote::Joueur),
        oubliee.nom(),
        nouvelle.nom()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::contenu::ContenuV1;
    use bersemon_core::espece::EspeceId;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::stats::ValeursInternes;

    fn creer(contenu: &ContenuV1, id: &str, niveau: u8) -> Monstre {
        Monstre::creer(
            contenu.espece(&EspeceId::new(id)).unwrap(),
            Niveau::new(niveau).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    /// Le loup, comme reçu d'Ysolde.
    fn loup_recu(contenu: &ContenuV1) -> Monstre {
        let loup = creer(contenu, "loup", 15);
        Monstre::restaurer(bersemon_core::monstre::EtatMonstre {
            espece: loup.espece().clone(),
            surnom: None,
            niveau: loup.niveau(),
            experience: loup.experience(),
            valeurs_internes: *loup.valeurs_internes(),
            attaques: loup.attaques().clone(),
            shiny: false,
            pv_actuels: loup.pv_actuels(),
            statut: None,
            habilete: loup.habilete(),
            recu_de: Some(bersemon_core::nom::Nom::new("Ysolde").unwrap()),
            location: None,
        })
        .unwrap()
    }

    #[test]
    fn un_monstre_recu_dit_de_qui_il_vient_et_peut_desobeir() {
        let contenu = ContenuV1::new();
        let loup = loup_recu(&contenu);
        assert_eq!(
            crate::monstres::provenance(&loup, 12).as_deref(),
            Some("Reçu de Ysolde. Obéit jusqu'au niveau 12.")
        );
        assert_eq!(
            crate::monstres::provenance(&creer(&contenu, "loup", 5), 12),
            None
        );
        let combat = Combat::new(vec![loup], vec![creer(&contenu, "corbeau", 5)]).unwrap();
        let texte = evenement(
            &Evenement::Desobeissance(combat.actif(Cote::Joueur)),
            &combat,
        );
        assert!(
            texte.contains("n'en fait qu'à sa tête et ignore votre ordre."),
            "{texte}"
        );
    }

    #[test]
    fn les_ordres_disent_leur_bonus() {
        assert_eq!(
            ordre_et_bonus(Ordre::CriDeGuerre),
            "Cri de guerre (+20 % d'Attaque)"
        );
        assert_eq!(
            ordre_donne("Guts", Ordre::CriDeGuerre, "Loup"),
            "Guts pousse un cri de guerre : Loup gagne 20 % d'Attaque jusqu'à la fin du combat."
        );
        assert_eq!(
            ordre_donne("Guts", Ordre::BouclierDeFoi, "Loup"),
            "Guts invoque un bouclier de foi : Loup gagne 20 % de Défense et de Défense spéciale jusqu'à la fin du combat."
        );
        assert_eq!(
            ordre_donne("Guts", Ordre::Incantation, "Loup"),
            "Guts murmure une incantation : Loup gagne 20 % d'Attaque spéciale jusqu'à la fin du combat."
        );
        assert_eq!(
            ordre_donne("Guts", Ordre::PasDeLOmbre, "Loup"),
            "Guts souffle le pas de l'ombre : Loup gagne 20 % de Vitesse jusqu'à la fin du combat."
        );
    }

    #[test]
    fn un_combat_sans_capture_ne_dit_pas_l_adversaire_sauvage() {
        let contenu = ContenuV1::new();
        let combat = || {
            Combat::new(
                vec![creer(&contenu, "salamandre", 5)],
                vec![creer(&contenu, "loup", 5)],
            )
            .unwrap()
        };
        assert_eq!(
            rencontre(&combat()),
            "Des monstres surgissent : **Loup sauvage**."
        );
        assert_eq!(
            rencontre(&combat().sans_capture()),
            "Des monstres surgissent : **Loup**."
        );
    }

    #[test]
    fn une_attaque_dit_ses_degats_sa_precision_et_sa_categorie() {
        let contenu = ContenuV1::new();
        let combat = Combat::new(
            vec![creer(&contenu, "salamandre", 5)],
            vec![creer(&contenu, "armure-hantee", 8)],
        )
        .unwrap();
        // Souffle chaud, de type Normal, n'affecte pas un Spectre.
        assert_eq!(
            attaque_en_combat(&combat, 0).unwrap(),
            "Souffle chaud : sans effet, 100 %, Spé"
        );
        let braises = attaque_en_combat(&combat, 1).unwrap();
        assert!(braises.starts_with("Crachat de braises : "), "{braises}");
        assert!(braises.ends_with(" dégâts, 100 %, Spé"), "{braises}");
        assert_eq!(attaque_en_combat(&combat, 2), None);
    }

    #[test]
    fn les_types_de_l_adversaire_ne_se_voient_qu_une_fois_son_espece_capturee() {
        let contenu = ContenuV1::new();
        let combat = Combat::new(
            vec![creer(&contenu, "salamandre", 5)],
            vec![creer(&contenu, "loup", 5)],
        )
        .unwrap();
        let mut codex = Codex::default();
        codex.noter_rencontre(&EspeceId::new("loup"));
        assert!(etat(&combat, &codex).contains("En face : Loup sauvage (?, niv. 5)"));
        codex.noter_capture(&EspeceId::new("loup"));
        assert!(etat(&combat, &codex).contains("En face : Loup sauvage (Normal, niv. 5)"));
    }

    #[test]
    fn un_gain_d_xp_montre_l_xp_du_niveau_suivant() {
        let contenu = ContenuV1::new();
        let mut loup = creer(&contenu, "loup", 5);
        let rapport = loup.gagner_experience(5);
        assert!(rapport_experience(&loup, &rapport).contains("gagne 5 XP (130/216)."));
        // Au niveau 6, la progression vise le niveau 7 : 7 x 7 x 7 = 343.
        let rapport = loup.gagner_experience(100);
        let texte = rapport_experience(&loup, &rapport);
        assert!(texte.contains("gagne 100 XP (230/343)."), "{texte}");
        assert!(texte.contains("passe au niveau 6."), "{texte}");
        assert!(crate::monstres::fiche_du_monstre(&loup).contains("\nXP : 230/343\n"));
    }

    #[test]
    fn la_fiche_montre_l_habilete_et_son_effet() {
        let contenu = ContenuV1::new();
        let loup = creer(&contenu, "loup", 5);
        let fiche = crate::monstres::fiche_du_monstre(&loup);
        assert!(
            fiche.contains("\nHabileté : Traqueur (chance de capture x1,25 quand il est actif)\n"),
            "{fiche}"
        );
        let espece = crate::partie::fiche_d_espece(loup.espece(), &contenu);
        assert_eq!(espece[1], "Habiletés possibles : Traqueur et Fouineur");
        assert_eq!(espece[4], "Évolution : Warg au niveau 16");
        let fiche = |id: &str| {
            let espece = contenu.espece(&EspeceId::new(id)).unwrap();
            crate::partie::fiche_d_espece(&espece, &contenu)
                .pop()
                .unwrap()
        };
        assert_eq!(
            fiche("armure-hantee"),
            "Évolution : Chevalier noir dès le niveau 18, en gagnant un niveau dans la zone Vallée des Brumes"
        );
        assert_eq!(
            fiche("squelette"),
            "Évolution : Chevalier squelette dès le niveau 20, une fois l'Apôtre Baron pendu vaincu"
        );
        assert_eq!(
            fiche("cerf-blanc"),
            "Évolution : Cerf de lune avec une pierre de lune"
        );
    }

    #[test]
    fn au_niveau_100_l_xp_n_a_plus_de_suite() {
        let contenu = ContenuV1::new();
        let loup = creer(&contenu, "loup", 100);
        assert_eq!(progression_d_experience(&loup), "1000000, niveau maximum");
    }
}
