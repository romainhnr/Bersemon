//! Le rapport des parties simulées, en Markdown : par classe, par zone et
//! par combat de fin de zone.

use std::fmt::Write;

use bersemon_core::contenu::ContenuV1;
use bersemon_core::heros::Classe;

use crate::arene::{COMBATS_PAR_CASE, EQUIPEMENTS, MesureDuPalier};
use crate::joueur::{Bilan, ExpeditionJouee, Issue};

fn moyenne(valeurs: impl IntoIterator<Item = f64>) -> f64 {
    let (somme, nombre) = valeurs
        .into_iter()
        .fold((0.0, 0usize), |(somme, nombre), valeur| {
            (somme + valeur, nombre + 1)
        });
    if nombre == 0 {
        0.0
    } else {
        somme / nombre as f64
    }
}

fn mediane(mut valeurs: Vec<f64>) -> f64 {
    if valeurs.is_empty() {
        return 0.0;
    }
    valeurs.sort_by(f64::total_cmp);
    let milieu = valeurs.len() / 2;
    if valeurs.len() % 2 == 0 {
        (valeurs[milieu - 1] + valeurs[milieu]) / 2.0
    } else {
        valeurs[milieu]
    }
}

fn pourcentage(part: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        part as f64 * 100.0 / total as f64
    }
}

pub fn rapport(contenu: &ContenuV1, bilans: &[Bilan], parties_par_classe: u64) -> String {
    let mut texte = String::new();
    let _ = writeln!(texte, "# Équilibrage de Bersemon\n");
    let _ = writeln!(
        texte,
        "{parties_par_classe} parties par classe, du héros à la chute de la Pythie de sel, au bout du Désert de sel, après celle du Margrave de givre, jouées par le joueur automatique (crates/bersemon-equilibrage). Le niveau de l'équipe est la moyenne des monstres de plus haut niveau, équipe et camp réunis, autant qu'en compte l'équipe.\n"
    );
    parties(&mut texte, bilans);
    zones(&mut texte, contenu, bilans);
    zones_par_classe(&mut texte, contenu, bilans);
    combats_de_fin_de_zone(&mut texte, contenu, bilans);
    victoires_selon_le_niveau(&mut texte, contenu, bilans);
    ressources(&mut texte, bilans);
    texte
}

fn parties(texte: &mut String, bilans: &[Bilan]) {
    let _ = writeln!(texte, "## Parties\n");
    let _ = writeln!(
        texte,
        "| Classe | Terminées | Expéditions (médiane) | Défaites (moyenne) | Niveau final de l'équipe | Achats chez Malvoise | Argent à la fin | Équipe de 4 dès l'expédition | Objets forgés | Choix refusés |"
    );
    let _ = writeln!(texte, "|---|---|---|---|---|---|---|---|---|---|");
    for classe in Classe::TOUTES {
        let de_la_classe: Vec<&Bilan> = bilans
            .iter()
            .filter(|bilan| bilan.classe == classe)
            .collect();
        if de_la_classe.is_empty() {
            continue;
        }
        let terminees = de_la_classe.iter().filter(|bilan| bilan.terminee).count();
        let expeditions = mediane(
            de_la_classe
                .iter()
                .map(|bilan| bilan.expeditions.len() as f64)
                .collect(),
        );
        let defaites = moyenne(de_la_classe.iter().map(|bilan| {
            bilan
                .expeditions
                .iter()
                .filter(|expedition| expedition.issue == Issue::Defaite)
                .count() as f64
        }));
        let niveau = moyenne(de_la_classe.iter().filter_map(|bilan| {
            bilan
                .expeditions
                .last()
                .map(|expedition| expedition.niveau_au_retour)
        }));
        let achats = moyenne(de_la_classe.iter().map(|bilan| bilan.achats as f64));
        let agrandies: Vec<f64> = de_la_classe
            .iter()
            .filter_map(|bilan| bilan.equipe_agrandie.map(|numero| numero as f64))
            .collect();
        let equipe_agrandie = if agrandies.is_empty() {
            "jamais".to_string()
        } else {
            format!(
                "{:.0} ({} parties)",
                mediane(agrandies.clone()),
                agrandies.len()
            )
        };
        let objets = moyenne(de_la_classe.iter().map(|bilan| bilan.objets_forges as f64));
        let refus = moyenne(
            de_la_classe
                .iter()
                .map(|bilan| f64::from(bilan.choix_refuses)),
        );
        let argent = moyenne(
            de_la_classe
                .iter()
                .map(|bilan| f64::from(bilan.argent_final)),
        );
        let _ = writeln!(
            texte,
            "| {} | {terminees}/{} | {expeditions:.0} | {defaites:.1} | {niveau:.1} | {achats:.0} | {argent:.0} | {equipe_agrandie} | {objets:.1} | {refus:.1} |",
            classe.nom(),
            de_la_classe.len()
        );
    }
    let _ = writeln!(texte);
}

fn zones(texte: &mut String, contenu: &ContenuV1, bilans: &[Bilan]) {
    let _ = writeln!(texte, "## Zones\n");
    let _ = writeln!(
        texte,
        "Moyennes par expédition, sauf mention. L'argent, les filets et les potions sont la variation du sac et du coffre : une défaite fait perdre le sac.\n"
    );
    let _ = writeln!(
        texte,
        "| Zone (niveaux) | Expéditions par partie | Menées au bout | Défaites | Rentrées | Niveau à la première | Niveaux gagnés | Argent | Filets | Potions | Nouvelles espèces |"
    );
    let _ = writeln!(texte, "|---|---|---|---|---|---|---|---|---|---|---|");
    for zone in contenu.zones() {
        let de_la_zone: Vec<&ExpeditionJouee> = bilans
            .iter()
            .flat_map(|bilan| &bilan.expeditions)
            .filter(|expedition| expedition.zone == *zone.id())
            .collect();
        if de_la_zone.is_empty() {
            continue;
        }
        let total = de_la_zone.len();
        let issue = |voulue: Issue| {
            pourcentage(
                de_la_zone
                    .iter()
                    .filter(|expedition| expedition.issue == voulue)
                    .count(),
                total,
            )
        };
        let premiere = moyenne(bilans.iter().filter_map(|bilan| {
            bilan
                .expeditions
                .iter()
                .find(|expedition| expedition.zone == *zone.id())
                .map(|expedition| expedition.niveau_au_depart)
        }));
        let par = |valeur: fn(&ExpeditionJouee) -> f64| {
            moyenne(de_la_zone.iter().map(|expedition| valeur(expedition)))
        };
        let (minimum, maximum) = zone.niveaux();
        let _ = writeln!(
            texte,
            "| {} ({} à {}) | {:.1} | {:.0} % | {:.0} % | {:.0} % | {premiere:.1} | {:.2} | {:.0} | {:.2} | {:.2} | {:.2} |",
            zone.nom(),
            minimum.valeur(),
            maximum.valeur(),
            total as f64 / bilans.len() as f64,
            issue(Issue::MeneeAuBout),
            issue(Issue::Defaite),
            issue(Issue::Rentree),
            par(|expedition| expedition.niveau_au_retour - expedition.niveau_au_depart),
            par(|expedition| expedition.argent as f64),
            par(|expedition| expedition.filets_depenses as f64),
            par(|expedition| expedition.potions_depensees as f64),
            par(|expedition| expedition.nouvelles_especes as f64),
        );
    }
    let _ = writeln!(texte);
}

/// Où chaque classe passe ses expéditions, et où elle tombe : de quoi
/// situer l'écart d'une classe.
fn zones_par_classe(texte: &mut String, contenu: &ContenuV1, bilans: &[Bilan]) {
    let _ = writeln!(texte, "## Zones par classe\n");
    let _ = writeln!(
        texte,
        "Expéditions par partie, et entre parenthèses les défaites par partie.\n"
    );
    let mut entete = "| Classe |".to_string();
    let mut separation = "|---|".to_string();
    for zone in contenu.zones() {
        entete.push_str(&format!(" {} |", zone.nom()));
        separation.push_str("---|");
    }
    let _ = writeln!(texte, "{entete}");
    let _ = writeln!(texte, "{separation}");
    for classe in Classe::TOUTES {
        let de_la_classe: Vec<&Bilan> = bilans
            .iter()
            .filter(|bilan| bilan.classe == classe)
            .collect();
        if de_la_classe.is_empty() {
            continue;
        }
        let mut ligne = format!("| {} |", classe.nom());
        for zone in contenu.zones() {
            let compte = |defaites: bool| {
                moyenne(de_la_classe.iter().map(|bilan| {
                    bilan
                        .expeditions
                        .iter()
                        .filter(|expedition| {
                            expedition.zone == *zone.id()
                                && (!defaites || expedition.issue == Issue::Defaite)
                        })
                        .count() as f64
                }))
            };
            ligne.push_str(&format!(" {:.1} ({:.1}) |", compte(false), compte(true)));
        }
        let _ = writeln!(texte, "{ligne}");
    }
    let _ = writeln!(texte);
}

fn combats_de_fin_de_zone(texte: &mut String, contenu: &ContenuV1, bilans: &[Bilan]) {
    let _ = writeln!(texte, "## Combats de fin de zone\n");
    let _ = writeln!(
        texte,
        "| Combat | Niveau maximal de la zone | Tentatives par partie | Victoires au premier essai | Niveau de l'équipe à la victoire |"
    );
    let _ = writeln!(texte, "|---|---|---|---|---|");
    for (maximum, nom) in combats(contenu, bilans) {
        let mut tentatives = Vec::new();
        let mut premiers_essais = 0;
        let mut niveaux = Vec::new();
        for bilan in bilans {
            let combats: Vec<&ExpeditionJouee> = bilan
                .expeditions
                .iter()
                .filter(|expedition| expedition.boss.as_ref().is_some_and(|boss| boss.nom == nom))
                .collect();
            if combats.is_empty() {
                continue;
            }
            let victoire = combats
                .iter()
                .position(|expedition| expedition.boss.as_ref().is_some_and(|boss| boss.vaincu));
            tentatives.push(victoire.map_or(combats.len(), |position| position + 1) as f64);
            if victoire == Some(0) {
                premiers_essais += 1;
            }
            if let Some(position) = victoire {
                niveaux.push(combats[position].niveau_au_depart);
            }
        }
        let _ = writeln!(
            texte,
            "| {nom} | {maximum} | {:.1} | {:.0} % | {:.1} |",
            moyenne(tentatives.iter().copied()),
            pourcentage(premiers_essais, tentatives.len()),
            moyenne(niveaux)
        );
    }
    let _ = writeln!(texte);
}

/// Les combats de fin de zone rencontrés, dans l'ordre des zones, avec le
/// niveau maximal de leur zone.
fn combats(contenu: &ContenuV1, bilans: &[Bilan]) -> Vec<(u8, String)> {
    let mut noms: Vec<(usize, String)> = Vec::new();
    for bilan in bilans {
        for expedition in &bilan.expeditions {
            if let Some(boss) = &expedition.boss {
                if !noms.iter().any(|(_, nom)| *nom == boss.nom) {
                    let rang = contenu
                        .zones()
                        .iter()
                        .position(|zone| *zone.id() == expedition.zone)
                        .unwrap_or(usize::MAX);
                    noms.push((rang, boss.nom.clone()));
                }
            }
        }
    }
    noms.sort();
    noms.into_iter()
        .map(|(rang, nom)| {
            let maximum = contenu
                .zones()
                .get(rang)
                .map_or(0, |zone| zone.niveaux().1.valeur());
            (maximum, nom)
        })
        .collect()
}

/// Bornes des tranches d'écart entre le niveau moyen de l'équipe au départ
/// et le niveau maximal de la zone : jusqu'à -4, de -3 à -2, de -1 à 0, de 1
/// à 2, puis 3 et plus.
const TRANCHES: [(&str, f64, f64); 5] = [
    ("-4 ou moins", f64::NEG_INFINITY, -3.0),
    ("-3 à -2", -3.0, -1.0),
    ("-1 à 0", -1.0, 1.0),
    ("+1 à +2", 1.0, 3.0),
    ("+3 ou plus", 3.0, f64::INFINITY),
];

/// La chance de gagner une tentative selon le niveau de l'équipe : ne
/// dépend pas du moment où le joueur automatique se risque au combat,
/// contrairement au nombre de tentatives.
fn victoires_selon_le_niveau(texte: &mut String, contenu: &ContenuV1, bilans: &[Bilan]) {
    let _ = writeln!(texte, "## Victoires selon le niveau de l'équipe\n");
    let _ = writeln!(
        texte,
        "Part des tentatives gagnées, selon l'écart entre le niveau moyen de l'équipe au départ de l'expédition et le niveau maximal de la zone ; entre parenthèses, le nombre de tentatives. Un Apôtre a 2 niveaux de plus que le maximum de sa zone.\n"
    );
    let mut entete = "| Combat |".to_string();
    let mut separation = "|---|".to_string();
    for (libelle, _, _) in TRANCHES {
        entete.push_str(&format!(" {libelle} |"));
        separation.push_str("---|");
    }
    let _ = writeln!(texte, "{entete}");
    let _ = writeln!(texte, "{separation}");
    for (maximum, nom) in combats(contenu, bilans) {
        let mut ligne = format!("| {nom} |");
        for (_, debut, fin) in TRANCHES {
            let tentatives: Vec<bool> = bilans
                .iter()
                .flat_map(|bilan| &bilan.expeditions)
                .filter_map(|expedition| {
                    let boss = expedition.boss.as_ref()?;
                    let ecart = expedition.niveau_au_depart - f64::from(maximum);
                    (boss.nom == nom && ecart >= debut && ecart < fin).then_some(boss.vaincu)
                })
                .collect();
            if tentatives.is_empty() {
                ligne.push_str(" - |");
            } else {
                let gagnees = tentatives.iter().filter(|vaincu| **vaincu).count();
                ligne.push_str(&format!(
                    " {:.0} % ({}) |",
                    pourcentage(gagnees, tentatives.len()),
                    tentatives.len()
                ));
            }
        }
        let _ = writeln!(texte, "{ligne}");
    }
    let _ = writeln!(texte);
}

/// Ce que les ressources trouvées permettent de forger chez Gunther.
fn ressources(texte: &mut String, bilans: &[Bilan]) {
    let _ = writeln!(texte, "## Ressources trouvées\n");
    let _ = writeln!(
        texte,
        "Moyenne par partie, perdues ou non dans une défaite, et part des parties qui en trouvent au moins une.\n"
    );
    let _ = writeln!(texte, "| Ressource | Par partie | Parties |");
    let _ = writeln!(texte, "|---|---|---|");
    let mut noms: Vec<&String> = bilans
        .iter()
        .flat_map(|bilan| bilan.ressources.keys())
        .collect();
    noms.sort();
    noms.dedup();
    for nom in noms {
        let trouvees = |bilan: &Bilan| bilan.ressources.get(nom).copied().unwrap_or(0);
        let par_partie = moyenne(bilans.iter().map(|bilan| f64::from(trouvees(bilan))));
        let parties = bilans.iter().filter(|bilan| trouvees(bilan) > 0).count();
        let _ = writeln!(
            texte,
            "| {nom} | {par_partie:.1} | {:.0} % |",
            pourcentage(parties, bilans.len())
        );
    }
    let _ = writeln!(texte);
}

/// La Fosse aux damnés, palier par palier : la part de victoires du héros
/// selon son niveau de combat, le niveau où chaque classe gagne une fois sur
/// deux, puis l'effet du second type des classes (section 4.4).
pub fn rapport_de_la_fosse(avec: &[MesureDuPalier], sans: &[MesureDuPalier]) -> String {
    let mut texte = String::new();
    let equipements: Vec<String> = EQUIPEMENTS
        .iter()
        .map(|(niveau, arme, armure)| {
            let armure = if *armure == 0 {
                "aucune".to_string()
            } else {
                armure.to_string()
            };
            format!("{niveau} = {arme} et {armure}")
        })
        .collect();
    let _ = writeln!(texte, "## Fosse aux damnés\n");
    let _ = writeln!(
        texte,
        "Le héros seul contre l'adversaire de chaque palier, sans objet, {COMBATS_PAR_CASE} combats par case, en attaquant avec l'attaque aux dégâts attendus les plus hauts. Part de victoires, moyenne des quatre classes, selon le niveau de combat du héros (paliers de l'arme et de l'armure : {}). Dès le niveau 25, les humains ont le second type de leur classe.\n",
        equipements.join(", ")
    );
    let mut entete = "| Palier | Adversaire |".to_string();
    let mut separation = "|---|---|".to_string();
    for (niveau, _, _) in EQUIPEMENTS {
        entete.push_str(&format!(" {niveau} |"));
        separation.push_str("---|");
    }
    entete.push_str(" Tours |");
    separation.push_str("---|");
    let _ = writeln!(texte, "{entete}");
    let _ = writeln!(texte, "{separation}");
    for palier in avec {
        let mut ligne = format!(
            "| {} | {} ({}, {}, {}) |",
            palier.numero, palier.adversaire, palier.classe, palier.niveau, palier.types
        );
        for part in parts_moyennes(palier) {
            ligne.push_str(&format!(" {part:.0} % |"));
        }
        ligne.push_str(&format!(" {:.1} |", palier.tours));
        let _ = writeln!(texte, "{ligne}");
    }
    let _ = writeln!(texte);
    let _ = writeln!(
        texte,
        "Niveau de combat à partir duquel chaque classe gagne au moins une fois sur deux (au-delà de 40 : jamais).\n"
    );
    let mut entete = "| Palier |".to_string();
    let mut separation = "|---|".to_string();
    for classe in Classe::TOUTES {
        entete.push_str(&format!(" {} |", classe.nom()));
        separation.push_str("---|");
    }
    let _ = writeln!(texte, "{entete}");
    let _ = writeln!(texte, "{separation}");
    for palier in avec {
        let mut ligne = format!("| {} |", palier.numero);
        for (_, parts) in &palier.victoires {
            let niveau = EQUIPEMENTS
                .iter()
                .zip(parts)
                .find(|(_, part)| **part >= 50.0)
                .map_or_else(
                    || "jamais".to_string(),
                    |((niveau, _, _), _)| niveau.to_string(),
                );
            ligne.push_str(&format!(" {niveau} |"));
        }
        let _ = writeln!(texte, "{ligne}");
    }
    let _ = writeln!(texte);
    let _ = writeln!(texte, "### Effet du second type\n");
    let _ = writeln!(
        texte,
        "La même mesure, tous les humains restant Humain seul, comme avant l'étape 145 : écart de la part de victoires, en points (avec le second type moins sans), et durée des combats sans le second type.\n"
    );
    let mut entete = "| Palier |".to_string();
    let mut separation = "|---|".to_string();
    for (niveau, _, _) in EQUIPEMENTS {
        entete.push_str(&format!(" {niveau} |"));
        separation.push_str("---|");
    }
    entete.push_str(" Tours sans |");
    separation.push_str("---|");
    let _ = writeln!(texte, "{entete}");
    let _ = writeln!(texte, "{separation}");
    for (palier, sans) in avec.iter().zip(sans) {
        let mut ligne = format!("| {} |", palier.numero);
        for (part, part_sans) in parts_moyennes(palier).iter().zip(parts_moyennes(sans)) {
            ligne.push_str(&format!(" {:+.0} |", part - part_sans));
        }
        ligne.push_str(&format!(" {:.1} |", sans.tours));
        let _ = writeln!(texte, "{ligne}");
    }
    let _ = writeln!(texte);
    texte
}

fn parts_moyennes(palier: &MesureDuPalier) -> Vec<f64> {
    (0..EQUIPEMENTS.len())
        .map(|rang| moyenne(palier.victoires.iter().map(|(_, parts)| parts[rang])))
        .collect()
}
