//! Les humains de fin de zone et les espèces sauvages, mesurés hors des
//! parties (étape 163). Ces mesures ne dépendent ni des équipes ni des choix
//! du joueur automatique : chaque humain affronte des équipes tirées au
//! hasard parmi les espèces qu'un joueur a pu garder avant sa zone ; chaque
//! espèce affronte en un contre un, au même niveau, une même réserve
//! d'adversaires. Les combattants attaquent avec l'attaque aux dégâts
//! attendus les plus hauts, comme dans la Fosse simulée.

use bersemon_core::aleatoire::{Aleatoire, GenerateurAleatoire};
use bersemon_core::catalogue::Catalogue;
use bersemon_core::combat::{Action, Combat, Cote, Issue, Phase};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::equipement::SorteDObjet;
use bersemon_core::espece::{Espece, EspeceId, Rarete};
use bersemon_core::heros::{Classe, creer_combattant};
use bersemon_core::humain::IaHumain;
use bersemon_core::monstre::Monstre;
use bersemon_core::niveau::Niveau;
use bersemon_core::stats::ValeursInternes;
use bersemon_core::zone::Zone;

/// Assez pour que la part de victoires d'un humain varie de quelques points
/// seulement d'un réglage à l'autre.
const EQUIPES_AU_HASARD: u32 = 100;
const COMBATS_PAR_EQUIPE: u64 = 30;
/// Celles qu'un joueur garde encore à ce stade de la partie : les espèces
/// plus faibles ont été remplacées.
const TOTAL_MINIMAL_GARDE: u32 = 380;
/// Le niveau commun des combats en un contre un, et la réserve
/// d'adversaires : les espèces sauvages de 380 à 500 au total.
const NIVEAU_DES_ESPECES: u8 = 50;
const TOTAL_MAXIMAL_DE_LA_RESERVE: u32 = 500;
const COMBATS_PAR_ADVERSAIRE: u64 = 4;
/// Au-delà, le combat compte comme perdu.
const TOURS_MAX: u32 = 300;

#[derive(Debug, Clone, PartialEq)]
pub struct MesureDuGardien {
    pub zone: String,
    pub humain: String,
    pub niveau: u8,
    /// Le niveau de combat du héros du joueur, Mage, équipé au mieux sans
    /// dépasser le niveau maximal de la zone.
    pub heros_du_joueur: u8,
    /// Part de victoires du joueur, en pourcentage.
    pub victoires: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForceDEspece {
    pub zone: String,
    pub espece: String,
    pub total: u32,
    pub victoires: f64,
}

/// Chaque humain de fin de zone, à partir du Marais, contre des équipes de
/// trois monstres au niveau maximal de sa zone, tirées parmi les espèces
/// des zones d'avant, formes évoluées comprises.
pub fn mesurer_les_gardiens(contenu: &ContenuV1) -> Vec<MesureDuGardien> {
    let mut mesures = Vec::new();
    for (position, zone) in contenu.zones().iter().enumerate().skip(1) {
        let Some(humain) = zone.humain() else {
            continue;
        };
        let niveau = zone.niveaux().1;
        let reserve = especes_gardees(contenu, &contenu.zones()[..position]);
        let heros = heros_du_joueur(contenu, niveau.valeur());
        let mut tirage = GenerateurAleatoire::new(position as u64);
        let mut gagnes = 0u64;
        for numero in 0..EQUIPES_AU_HASARD {
            let equipe: Vec<Monstre> = (0..3)
                .map(|_| {
                    let index = tirage.tirer(0..=(reserve.len() as u32 - 1)) as usize;
                    creer(&reserve[index], niveau)
                })
                .collect();
            for graine in 0..COMBATS_PAR_EQUIPE {
                let mut combat = Combat::new(equipe.clone(), humain.generer_equipe())
                    .expect("équipes valides")
                    .avec_heros(Cote::Joueur, heros.clone())
                    .contre_un_humain(humain.ia());
                if let Some(heros_adverse) = humain.generer_heros() {
                    combat = combat.avec_heros(Cote::Adversaire, heros_adverse);
                }
                let graine = u64::from(numero) * 1_000 + graine;
                gagnes += u64::from(combattre(combat, graine));
            }
        }
        mesures.push(MesureDuGardien {
            zone: zone.nom().to_string(),
            humain: humain.nom().to_string(),
            niveau: niveau.valeur(),
            heros_du_joueur: heros.niveau().valeur(),
            victoires: gagnes as f64 * 100.0
                / (f64::from(EQUIPES_AU_HASARD) * COMBATS_PAR_EQUIPE as f64),
        });
    }
    mesures
}

/// Les espèces sauvages des zones de la carte puis des annexes, dans leur
/// ordre, chacune contre toute la réserve.
pub fn mesurer_les_especes(contenu: &ContenuV1) -> Vec<ForceDEspece> {
    let niveau = Niveau::new(NIVEAU_DES_ESPECES).expect("niveau valide");
    let reserve: Vec<&Espece> = contenu
        .especes()
        .iter()
        .filter(|espece| !est_hors_du_codex_sauvage(espece))
        .filter(|espece| {
            (TOTAL_MINIMAL_GARDE..=TOTAL_MAXIMAL_DE_LA_RESERVE)
                .contains(&espece.total_des_stats_de_base())
        })
        .collect();
    let zones = contenu
        .zones()
        .iter()
        .chain(contenu.annexes().iter().map(|annexe| annexe.zone()));
    let mut vues: Vec<EspeceId> = Vec::new();
    let mut forces = Vec::new();
    for zone in zones {
        let especes = zone
            .especes()
            .iter()
            .chain(zone.rares().iter().map(|rare| &rare.espece));
        for espece in especes {
            if vues.contains(espece.id()) {
                continue;
            }
            vues.push(espece.id().clone());
            let mut gagnes = 0u64;
            let mut combats = 0u64;
            for (rang, adversaire) in reserve.iter().enumerate() {
                for graine in 0..COMBATS_PAR_ADVERSAIRE {
                    let combat =
                        Combat::new(vec![creer(espece, niveau)], vec![creer(adversaire, niveau)])
                            .expect("monstres valides")
                            .contre_un_humain(IaHumain::Relevee);
                    gagnes += u64::from(combattre(combat, rang as u64 * 100 + graine));
                    combats += 1;
                }
            }
            forces.push(ForceDEspece {
                zone: zone.nom().to_string(),
                espece: espece.nom().to_string(),
                total: espece.total_des_stats_de_base(),
                victoires: gagnes as f64 * 100.0 / combats as f64,
            });
        }
    }
    forces
}

fn est_hors_du_codex_sauvage(espece: &Espece) -> bool {
    matches!(espece.rarete(), Rarete::Apotre | Rarete::Demiurge)
}

/// Les espèces de ces zones, rares comprises, et toutes leurs formes
/// évoluées, avec celles des monstres de départ : celles qui ont au moins
/// `TOTAL_MINIMAL_GARDE` au total.
fn especes_gardees(contenu: &ContenuV1, zones: &[Zone]) -> Vec<Espece> {
    let mut ids: Vec<EspeceId> = Vec::new();
    let bases = zones
        .iter()
        .flat_map(|zone| {
            zone.especes()
                .iter()
                .chain(zone.rares().iter().map(|rare| &rare.espece))
        })
        .chain(contenu.especes_de_depart());
    for base in bases {
        let mut espece = Some(base.clone());
        while let Some(courante) = espece {
            if !ids.contains(courante.id()) {
                ids.push(courante.id().clone());
            }
            espece = courante
                .evolution()
                .and_then(|evolution| contenu.espece(evolution.vers()));
        }
    }
    ids.iter()
        .filter_map(|id| contenu.espece(id))
        .filter(|espece| !est_hors_du_codex_sauvage(espece))
        .filter(|espece| espece.total_des_stats_de_base() >= TOTAL_MINIMAL_GARDE)
        .collect()
}

/// Un Mage, avec l'arme et l'armure qui donnent le plus haut niveau de
/// combat sans dépasser `niveau_max`, l'arme d'abord.
fn heros_du_joueur(contenu: &ContenuV1, niveau_max: u8) -> Monstre {
    let classe = Classe::Mage;
    let objet = |sorte, palier| contenu.objet_de_classe(classe, sorte, palier);
    let mut meilleur = (0, None, None);
    for arme in 1..=8u8 {
        for armure in 0..=arme {
            let niveau = 5 + 5 * u16::from(arme) + 5 * u16::from(armure);
            if niveau > u16::from(niveau_max) || niveau <= meilleur.0 {
                continue;
            }
            let Some(objet_arme) = objet(SorteDObjet::Arme, arme) else {
                continue;
            };
            let objet_armure = (armure > 0).then(|| objet(SorteDObjet::Armure, armure));
            if objet_armure.as_ref().is_some_and(Option::is_none) {
                continue;
            }
            meilleur = (niveau, Some(objet_arme), objet_armure.flatten());
        }
    }
    creer_combattant(
        classe.nom(),
        classe,
        meilleur.1.as_ref(),
        meilleur.2.as_ref(),
    )
}

fn creer(espece: &Espece, niveau: Niveau) -> Monstre {
    Monstre::creer(espece.clone(), niveau, ValeursInternes::default(), false)
        .expect("espèce sauvage créée au niveau de sa zone")
}

/// Vrai si le joueur gagne. Il remplace un monstre KO par le premier
/// debout.
fn combattre(mut combat: Combat, graine: u64) -> bool {
    let mut aleatoire = GenerateurAleatoire::new(graine);
    for _ in 0..TOURS_MAX {
        match combat.phase() {
            Phase::Termine(issue) => return issue == Issue::Victoire,
            Phase::Remplacement => {
                let Some(debout) = combat.equipe().iter().position(|monstre| !monstre.est_ko())
                else {
                    return false;
                };
                if combat.remplacer(debout).is_err() {
                    return false;
                }
            }
            Phase::ChoixAction => {
                let action = Action::Attaquer(meilleure_attaque(&combat));
                if combat.jouer_tour(action, &mut aleatoire).is_err() {
                    return false;
                }
            }
        }
    }
    false
}

fn meilleure_attaque(combat: &Combat) -> usize {
    let attaquant = combat.monstre(combat.actif(Cote::Joueur));
    (0..attaquant.attaques().liste().len())
        .max_by_key(|index| {
            let precision = attaquant.attaques().liste()[*index].precision();
            combat.apercu_attaque(*index).map_or(0, |apercu| {
                (apercu.degats_min + apercu.degats_max) * u32::from(precision)
            })
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_heros_du_joueur_ne_depasse_pas_le_niveau_de_la_zone() {
        let contenu = ContenuV1::new();
        let niveaux: Vec<u8> = [13, 27, 45, 51, 57]
            .into_iter()
            .map(|niveau| heros_du_joueur(&contenu, niveau).niveau().valeur())
            .collect();
        assert_eq!(niveaux, [10, 25, 45, 50, 55]);
    }

    #[test]
    fn les_especes_gardees_comprennent_les_formes_evoluees() {
        let contenu = ContenuV1::new();
        let gardees = especes_gardees(&contenu, &contenu.zones()[..1]);
        let ids: Vec<&str> = gardees.iter().map(|espece| espece.id().as_str()).collect();
        assert!(ids.contains(&"wyverne-de-cendre"), "{ids:?}");
        assert!(!ids.contains(&"loup"), "{ids:?}");
    }
}
