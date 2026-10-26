//! Récit des événements du déroulement, dans l'ordre où ils arrivent.

use bersemon_core::combat::Cote;
use bersemon_core::contenu::{
    INTRODUCTION, MALVOISE_ET_L_AME, MALVOISE_SANS_AME, RECOMPENSE_DU_DEMIURGE,
    fin_de_l_introduction, texte_du_conseil,
};
use bersemon_core::deroulement::Evenement;
use bersemon_core::marchand::EXPEDITIONS_DE_LOCATION;
use bersemon_core::ressource::Ressource;

use crate::arene;
use crate::boosters::{
    HEROS_NE_SHINY, HEROS_RENDU_SHINY, PHILTRE_RAMASSE, booster_donne, boosters,
    monstre_rendu_shiny,
};
use crate::combat::{ordre_donne, rapport_experience, remplacement, renoncement};
use crate::equipement::{nom_de_place, objet_et_bonus};
use crate::erreurs::erreur_sauvegarde;
use crate::expeditions_automatiques;
use crate::filets::filets;
use crate::forge::{
    constat, lancement, lancement_de_fabrication, lancement_des_filets, lancement_du_charme,
};
use crate::jour_et_nuit;
use crate::machines;
use crate::meteo;
use crate::mise_en_valeur::important;
use crate::monstres::{effet_d_habilete, nom_en_valeur, nom_sauvage};
use crate::partie::{
    argent, argent_et_part_du_pillard, capsule as nom_de_capsule, capsules_d_habilete, contenu,
    enumerer, pierres, potions, ressources,
};
use crate::peche;
use crate::recompenses_du_codex::recompense_reclamee;
use crate::rite::{naissance as texte_de_la_naissance, rite_lance};
use crate::sac;
use crate::soins::{le_soin, soin_utilise, soins};

/// La forme évoluée apprend d'autres attaques que son attaque d'évolution
/// (section 5.4).
const AUTRES_ATTAQUES_AU_CAMP: &str = "D'autres attaques peuvent s'apprendre au camp.";

/// Une ou plusieurs lignes de récit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recit {
    pub texte: String,
    /// Le récit ouvre un nouveau paragraphe, séparé du précédent par une
    /// ligne vide.
    pub nouveau_paragraphe: bool,
}

/// "de Bertrand", mais "d'Ulrich".
fn de(nom: &str) -> String {
    let voyelle = nom
        .chars()
        .next()
        .is_some_and(|initiale| "AEIOUYÉÈaeiouyéè".contains(initiale));
    if voyelle {
        format!("d'{nom}")
    } else {
        format!("de {nom}")
    }
}

pub fn raconter(evenements: &[Evenement]) -> Vec<Recit> {
    let mut recits = Vec::new();
    let mut precedent = None;
    for evenement in evenements {
        recits.push(raconter_un(evenement, precedent));
        precedent = Some(evenement);
    }
    recits
}

fn raconter_un(evenement: &Evenement, precedent: Option<&Evenement>) -> Recit {
    let (texte, nouveau_paragraphe) = match evenement {
        Evenement::PalierDeLaFosseVaincu {
            palier,
            adversaire,
            recompense,
            conquise,
        } => (
            arene::palier_vaincu(*palier, adversaire, recompense, *conquise),
            true,
        ),
        Evenement::DefaiteDansLaFosse { .. } => {
            (arene::DEFAITE_DANS_LA_FOSSE.to_string(), true)
        }
        Evenement::Introduction(classe) => (
            format!("{INTRODUCTION}\n\n{}", fin_de_l_introduction(*classe)),
            true,
        ),
        Evenement::PartieCommencee(depart) => (
            format!(
                "{} vous accompagne désormais.",
                nom_en_valeur(depart, Cote::Joueur)
            ),
            false,
        ),
        Evenement::HerosShiny => (HEROS_NE_SHINY.to_string(), false),
        Evenement::Depart(zone) => (format!("Vous partez pour {zone}."), true),
        Evenement::JourOuNuitAuDepart(jour_ou_nuit) => {
            (jour_et_nuit::au_depart(*jour_ou_nuit).to_string(), false)
        }
        Evenement::MeteoAuDepart(meteo) => (meteo::au_depart(*meteo).to_string(), false),
        Evenement::ObjetsJetes(jete) => (sac::objets_jetes(jete), false),
        Evenement::Etape {
            numero,
            total: Some(total),
        } => (format!("Étape {numero}/{total}."), true),
        Evenement::Etape {
            numero,
            total: None,
        } => (format!("Étape {numero}."), true),
        Evenement::EntreeDansLaTour(recit) => (recit.clone(), false),
        Evenement::GardienDeLaTour(recit) => (recit.clone(), false),
        Evenement::ReposDansLaTour(recit) => (recit.clone(), true),
        Evenement::NouveauRecordDeLaTour(etape) => (
            important(&format!(
                "Nouveau record dans la Tour sans fond : étape {etape}."
            )),
            false,
        ),
        Evenement::Coffre {
            potions: nombre,
            argent: montant,
            argent_du_pillard,
            capsule,
            capsule_d_habilete,
            pierre,
            booster,
            filets: filets_trouves,
        } => {
            let mut contenu = vec![potions(*nombre)];
            if *montant > 0 {
                contenu.push(argent_et_part_du_pillard(*montant, *argent_du_pillard));
            }
            contenu.extend(capsule.iter().map(nom_de_capsule));
            if *capsule_d_habilete {
                contenu.push(capsules_d_habilete(1));
            }
            if let Some(pierre) = pierre {
                contenu.push(pierres(*pierre, 1));
            }
            if let Some(booster) = booster {
                contenu.push(boosters(*booster, 1));
            }
            if let Some((filet, nombre)) = filets_trouves {
                contenu.push(filets(*filet, *nombre));
            }
            (
                format!("Vous trouvez un coffre : {}.", enumerer(&contenu)),
                false,
            )
        }
        Evenement::MonstreRarePacifique(monstre) => (
            format!(
                "Un monstre rare apparaît : {}. Il ne semble pas hostile.",
                important(&nom_sauvage(monstre))
            ),
            false,
        ),
        Evenement::MonstreRareHostile(monstre) => (
            format!(
                "Un monstre rare vous attaque : {} !",
                important(&nom_sauvage(monstre))
            ),
            false,
        ),
        Evenement::MonstreRareLaisse => ("Vous le laissez partir.".to_string(), false),
        Evenement::RecitDeLApotre(recit)
        | Evenement::HumainRencontre(recit)
        | Evenement::AmeRencontree(recit) => (recit.clone(), false),
        Evenement::AmeRecueillie(texte) => (texte.clone(), true),
        Evenement::PecheurRencontre(texte) => (texte.clone(), false),
        Evenement::PecheurSauve(texte) => (texte.clone(), true),
        Evenement::AmeDonnee { marchand, soin } => (
            format!(
                "Vous remettez l'âme {} à Malvoise, qui l'enferme dans le crâne. Malvoise vendra désormais {}.",
                de(marchand),
                le_soin(*soin).to_lowercase()
            ),
            false,
        ),
        Evenement::HumainVaincu {
            texte,
            argent: montant,
        } => (
            format!(
                "{texte} Vous ramassez {}, qui vont dans le sac.",
                argent(*montant)
            ),
            true,
        ),
        Evenement::Rencontre(combat) => (crate::combat::rencontre(combat), true),
        Evenement::Combat { evenements, combat } => {
            let lignes: Vec<String> = evenements
                .iter()
                .map(|evenement_du_tour| crate::combat::evenement(evenement_du_tour, combat))
                .collect();
            (lignes.join("\n"), false)
        }
        Evenement::Experience { monstre, rapport } => (rapport_experience(monstre, rapport), true),
        Evenement::PotionTrouvee(fouineur) => (
            format!(
                "Grâce à son habileté Fouineur, {} déniche une potion, rangée dans le sac.",
                nom_en_valeur(fouineur, Cote::Joueur)
            ),
            false,
        ),
        Evenement::AttaqueRemplacee {
            monstre,
            oubliee,
            nouvelle,
        } => (remplacement(monstre, oubliee, nouvelle), false),
        Evenement::AttaqueAbandonnee { monstre, nouvelle } => {
            (renoncement(monstre, nouvelle), false)
        }
        Evenement::AttaqueApprise { monstre, attaque } => (
            format!(
                "{} apprend {}.",
                nom_en_valeur(monstre, Cote::Joueur),
                attaque.nom()
            ),
            false,
        ),
        Evenement::Evolution {
            avant,
            apres,
            attaque,
        } => {
            let mut lignes = vec![format!(
                "{} évolue en {} !",
                nom_en_valeur(avant, Cote::Joueur),
                important(apres.espece().nom())
            )];
            if let Some(attaque) = attaque {
                lignes.push(format!(
                    "{} apprend {}.",
                    nom_en_valeur(apres, Cote::Joueur),
                    attaque.nom()
                ));
            }
            lignes.push(AUTRES_ATTAQUES_AU_CAMP.to_string());
            (lignes.join("\n"), true)
        }
        Evenement::EvolutionRepoussee(monstre) => (
            format!(
                "{} n'évolue pas pour l'instant.",
                nom_en_valeur(monstre, Cote::Joueur)
            ),
            false,
        ),
        Evenement::RejointLEquipe(monstre) => (
            format!(
                "{} rejoint votre équipe.",
                nom_en_valeur(monstre, Cote::Joueur)
            ),
            false,
        ),
        Evenement::PartPourLeCamp(monstre) => (
            format!(
                "{} part pour le camp.",
                nom_en_valeur(monstre, Cote::Joueur)
            ),
            false,
        ),
        Evenement::CaptureRelachee(monstre) | Evenement::Relache(monstre) => (
            format!("Vous relâchez {}.", nom_en_valeur(monstre, Cote::Joueur)),
            false,
        ),
        Evenement::RelachePourLaCapture { relache, capture } => (
            format!(
                "Vous relâchez {}. {} prend sa place.",
                nom_en_valeur(relache, Cote::Joueur),
                nom_en_valeur(capture, Cote::Joueur)
            ),
            false,
        ),
        Evenement::Epilogue(epilogue) => (epilogue.clone(), true),
        Evenement::AmeDApotreRecueillie(recit) => (recit.clone(), false),
        Evenement::PhiltreChromatiqueRamasse => (PHILTRE_RAMASSE.to_string(), false),
        Evenement::BeheritForge(recit) => (recit.clone(), true),
        Evenement::Sacrifice(recit) => (important(recit), true),
        Evenement::RecompenseDuDemiurge => (RECOMPENSE_DU_DEMIURGE.to_string(), false),
        Evenement::PiegesPoses(zone) => (machines::pieges_poses(zone), false),
        Evenement::PiegesReleves(prises) => (machines::pieges_releves(prises), false),
        Evenement::EntreeALEntrainement(monstre) => {
            (machines::entree_a_l_entrainement(monstre), false)
        }
        Evenement::SortieDeLEntrainement {
            monstre,
            experience,
            ..
        } => (machines::sortie_de_l_entrainement(monstre, *experience), false),
        Evenement::Defaite { sac_perdu } => {
            let texte = if sac_perdu.est_vide() {
                "Votre héros et toute votre équipe sont KO : vous rentrez au camp, le sac vide.".to_string()
            } else {
                format!(
                    "Votre héros et toute votre équipe sont KO : vous rentrez au camp et perdez tout votre sac : {}.",
                    contenu(sac_perdu)
                )
            };
            (important(&texte), true)
        }
        Evenement::OrdreDonne {
            heros,
            ordre,
            monstre,
        } => (
            ordre_donne(heros, *ordre, &nom_en_valeur(monstre, Cote::Joueur)),
            false,
        ),
        Evenement::SoinUtilise {
            soin,
            monstre,
            effet,
        } => (
            soin_utilise(*soin, &nom_en_valeur(monstre, Cote::Joueur), *effet),
            false,
        ),
        Evenement::ZoneExploree(zone) => (
            important(&format!("Vous avez exploré {zone} jusqu'au bout.")),
            true,
        ),
        Evenement::RetourAuCamp => (
            "Retour au camp : votre héros et vos monstres sont soignés.".to_string(),
            !matches!(precedent, Some(Evenement::ZoneExploree(_))),
        ),

        Evenement::Indice { numero, texte } => {
            (important(&format!("Indice {numero} : {texte}")), true)
        }
        Evenement::NouvelleZone(zone) => (
            format!("Une nouvelle zone apparaît sur la carte : {zone}."),
            false,
        ),
        Evenement::Conseil(conseil) => (texte_du_conseil(*conseil).to_string(), true),
        Evenement::SauvegardeImpossible(erreur) => (
            format!("Sauvegarde impossible : {}.", erreur_sauvegarde(erreur)),
            false,
        ),
        Evenement::Suicide => (
            "Vous retournez votre lame contre vous. Le feu du camp s'éteint, vos monstres se dispersent dans la nuit, et votre nom s'efface des routes. Ailleurs, une autre âme se relève parmi les cendres.".to_string(),
            true,
        ),
        Evenement::EffacementImpossible(erreur) => (
            format!(
                "La sauvegarde n'a pas pu être effacée : {}. Elle sera remplacée à la création de votre nouveau héros.",
                erreur_sauvegarde(erreur)
            ),
            true,
        ),
        Evenement::ExpeditionAutomatiquePartie(expedition) => {
            (expeditions_automatiques::depart(expedition), true)
        }
        Evenement::ExpeditionAutomatiqueRappelee { expedition, places } => {
            (expeditions_automatiques::rappel(expedition, places), true)
        }
        Evenement::ExpeditionAutomatiqueRentree { zone, bilan } => {
            (expeditions_automatiques::retour(zone, bilan), true)
        }
        Evenement::MalvoiseRencontre { avec_une_ame } => {
            let texte = if *avec_une_ame {
                MALVOISE_ET_L_AME
            } else {
                MALVOISE_SANS_AME
            };
            (texte.to_string(), true)
        }
        Evenement::SoinsAchetes { soin, nombre, prix } => (
            format!("Vous achetez {} pour {}.", soins(*soin, *nombre), argent(*prix)),
            false,
        ),
        Evenement::FiletsAchetes {
            filet,
            nombre,
            prix,
        } => (
            format!(
                "Vous achetez {} pour {}. Ils vont dans le coffre.",
                filets(*filet, *nombre),
                argent(*prix)
            ),
            false,
        ),
        Evenement::MonstreVendu { monstre, prix } => (
            format!(
                "Vous vendez {} à Malvoise pour {}.",
                nom_en_valeur(monstre, Cote::Joueur),
                argent(*prix)
            ),
            false,
        ),
        Evenement::MonstreAchete {
            monstre,
            prix,
            dans_l_equipe,
        } => (
            format!(
                "Malvoise ouvre une cage contre {} : {} {}.",
                argent(*prix),
                nom_en_valeur(monstre, Cote::Joueur),
                rejoint(*dans_l_equipe)
            ),
            false,
        ),
        Evenement::MonstreLoue {
            monstre,
            prix,
            dans_l_equipe,
        } => (
            format!(
                "Malvoise vous confie {} pour {EXPEDITIONS_DE_LOCATION} expéditions, contre {}. {} {}.",
                nom_en_valeur(monstre, Cote::Joueur),
                argent(*prix),
                monstre.nom(),
                rejoint(*dans_l_equipe)
            ),
            false,
        ),
        Evenement::MonstreLoueRendu(monstre) => (
            format!(
                "{} retourne chez Malvoise.",
                nom_en_valeur(monstre, Cote::Joueur)
            ),
            false,
        ),
        Evenement::LocationFinie(monstre) => (
            format!(
                "{} retourne chez Malvoise : sa location est finie.",
                nom_en_valeur(monstre, Cote::Joueur)
            ),
            false,
        ),
        Evenement::MonstreCede { heros, monstre } => (
            format!(
                "Vous cédez {} à {heros}.",
                nom_en_valeur(monstre, Cote::Joueur)
            ),
            false,
        ),
        Evenement::MonstreRecu { heros, monstre } => (
            format!(
                "Vous recevez {} de {heros}.",
                nom_en_valeur(monstre, Cote::Joueur)
            ),
            false,
        ),
        // Tout ranger reste possible avec un sac vide.
        Evenement::RangeAuCoffre(range) if range.est_vide() => {
            ("Le sac est déjà vide.".to_string(), false)
        }
        Evenement::RangeAuCoffre(range) => (
            format!("Vous rangez dans le coffre : {}.", contenu(range)),
            false,
        ),
        Evenement::SortiDuCoffre(sorti) => (
            format!("Vous prenez dans le coffre : {}.", contenu(sorti)),
            false,
        ),
        Evenement::RessourcesTrouvees(trouvees) => {
            // Regroupées par sorte, dans l'ordre où elles sont trouvées.
            let mut sortes: Vec<(&Ressource, u32)> = Vec::new();
            for ressource in trouvees {
                match sortes.iter_mut().find(|(sorte, _)| sorte.id() == ressource.id()) {
                    Some((_, nombre)) => *nombre += 1,
                    None => sortes.push((ressource, 1)),
                }
            }
            let liste: Vec<String> = sortes
                .into_iter()
                .map(|(ressource, nombre)| ressources(ressource, nombre))
                .collect();
            (
                format!(
                    "Sur les monstres vaincus, vous ramassez : {}.",
                    enumerer(&liste)
                ),
                false,
            )
        }
        Evenement::RessourcesVendues {
            ressource,
            nombre,
            prix,
        } => (
            format!(
                "Vous vendez {} à Malvoise pour {}.",
                ressources(ressource, *nombre),
                argent(*prix)
            ),
            false,
        ),
        Evenement::GuntherLibere(texte) => (texte.clone(), true),
        Evenement::AmeliorationLancee { plan, .. } => (lancement(plan), false),
        Evenement::TravailConstate(chantier) => (constat(chantier), true),
        Evenement::LigneLancee { delai, .. } => (peche::lancement(*delai), false),
        Evenement::PrisePechee(prise) => (peche::prise(prise), true),
        Evenement::PriseRelachee(monstre) => (peche::prise_relachee(monstre), false),
        Evenement::FabricationLancee { plan, .. } => (lancement_de_fabrication(plan), false),
        Evenement::CharmeLance { plan, .. } => (lancement_du_charme(plan), false),
        Evenement::FiletsLances { plan, .. } => (lancement_des_filets(plan), false),
        Evenement::RiteLance(apercu) => (rite_lance(apercu), false),
        Evenement::PetitNe(naissance) => (texte_de_la_naissance(naissance), false),
        Evenement::RecompenseDuCodex(recompense) => (recompense_reclamee(recompense), false),
        Evenement::ObjetEquipe {
            place,
            objet,
            remplace,
        } => {
            let remplace = remplace
                .as_ref()
                .map(|ancien| format!(", à la place de {}", ancien.nom()))
                .unwrap_or_default();
            (
                format!(
                    "{} : vous portez {}{remplace}.",
                    nom_de_place(*place),
                    objet_et_bonus(objet)
                ),
                false,
            )
        }
        Evenement::ObjetRetire { place, objet } => (
            format!(
                "{} : vous retirez {}.",
                nom_de_place(*place),
                objet.nom()
            ),
            false,
        ),
        Evenement::PierresAchetees {
            pierre,
            nombre,
            prix,
        } => (
            format!(
                "Vous achetez {} pour {}.",
                pierres(*pierre, *nombre),
                argent(*prix)
            ),
            false,
        ),
        Evenement::CapsulesDHabileteAchetees { nombre, prix } => (
            format!(
                "Vous achetez {} pour {}.",
                capsules_d_habilete(*nombre),
                argent(*prix)
            ),
            false,
        ),
        Evenement::BoosterDonne {
            booster,
            avant,
            apres,
        } => (booster_donne(*booster, avant, apres), false),
        Evenement::MonstreRenduShiny(monstre) => (monstre_rendu_shiny(monstre), false),
        Evenement::HerosRenduShiny => (HEROS_RENDU_SHINY.to_string(), false),
        Evenement::HabileteChangee { monstre, ancienne } => {
            let avant = ancienne
                .map(|habilete| format!(", à la place de {}", habilete.nom()))
                .unwrap_or_default();
            let texte = match monstre.habilete() {
                Some(habilete) => format!(
                    "{} a désormais l'habileté {}{avant} : {}.",
                    nom_en_valeur(monstre, Cote::Joueur),
                    habilete.nom(),
                    effet_d_habilete(habilete)
                ),
                None => format!("{} n'a plus d'habileté.", nom_en_valeur(monstre, Cote::Joueur)),
            };
            (texte, false)
        }
        Evenement::CapsuleAchetee { capsule, prix } => (
            format!(
                "Vous achetez {} pour {}.",
                nom_de_capsule(capsule),
                argent(*prix)
            ),
            false,
        ),
    };
    Recit {
        texte,
        nouveau_paragraphe,
    }
}

/// Un monstre acheté ou loué prend une place libre (section 14.2).
fn rejoint(dans_l_equipe: bool) -> &'static str {
    if dans_l_equipe {
        "rejoint l'équipe"
    } else {
        "rejoint le camp"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_retour_suit_la_zone_exploree_dans_le_meme_paragraphe() {
        let recits = raconter(&[
            Evenement::ZoneExploree("Lisière du Bois-Sombre".to_string()),
            Evenement::RetourAuCamp,
        ]);
        assert_eq!(
            recits,
            vec![
                Recit {
                    texte: "**Vous avez exploré Lisière du Bois-Sombre jusqu'au bout.**"
                        .to_string(),
                    nouveau_paragraphe: true,
                },
                Recit {
                    texte: "Retour au camp : votre héros et vos monstres sont soignés.".to_string(),
                    nouveau_paragraphe: false,
                },
            ]
        );
        assert!(raconter(&[Evenement::RetourAuCamp])[0].nouveau_paragraphe);
    }

    #[test]
    fn l_evolution_dit_l_attaque_apprise_et_rappelle_le_camp() {
        use bersemon_core::catalogue::Catalogue;
        use bersemon_core::contenu::ContenuV1;
        use bersemon_core::espece::EspeceId;
        use bersemon_core::monstre::Monstre;
        use bersemon_core::niveau::Niveau;
        use bersemon_core::stats::ValeursInternes;

        let contenu = ContenuV1::new();
        let creer = |id: &str| {
            Monstre::creer(
                contenu.espece(&EspeceId::new(id)).unwrap(),
                Niveau::new(16).unwrap(),
                ValeursInternes::default(),
                false,
            )
            .unwrap()
        };
        let apres = creer("lezard-de-braise");
        let attaque = apres.attaques().liste()[0].clone();
        let recits = raconter(&[
            Evenement::Evolution {
                avant: creer("salamandre"),
                apres: apres.clone(),
                attaque: Some(attaque.clone()),
            },
            Evenement::EvolutionRepoussee(creer("loup")),
        ]);
        assert_eq!(
            recits[0].texte,
            format!(
                "**Salamandre** évolue en **Lézard de braise** !\n**Lézard de braise** apprend {}.\nD'autres attaques peuvent s'apprendre au camp.",
                attaque.nom()
            )
        );
        assert!(recits[0].nouveau_paragraphe);
        assert_eq!(recits[1].texte, "**Loup** n'évolue pas pour l'instant.");
    }

    #[test]
    fn un_coffre_dit_combien_de_potions_et_d_argent() {
        let recit = &raconter(&[Evenement::Coffre {
            potions: 2,
            argent: 15,
            argent_du_pillard: 0,
            capsule: None,
            capsule_d_habilete: false,
            pierre: None,
            booster: None,
            filets: None,
        }])[0];
        assert_eq!(
            recit.texte,
            "Vous trouvez un coffre : 2 potions et 15 pièces d'argent."
        );
        let sans_argent = &raconter(&[Evenement::Coffre {
            potions: 1,
            argent: 0,
            argent_du_pillard: 0,
            capsule: None,
            capsule_d_habilete: false,
            pierre: None,
            booster: None,
            filets: None,
        }])[0];
        assert_eq!(sans_argent.texte, "Vous trouvez un coffre : 1 potion.");
        let avec_capsule = &raconter(&[Evenement::Coffre {
            potions: 1,
            argent: 12,
            argent_du_pillard: 0,
            capsule: Some(brasier()),
            capsule_d_habilete: false,
            pierre: None,
            booster: None,
            filets: None,
        }])[0];
        assert_eq!(
            avec_capsule.texte,
            "Vous trouvez un coffre : 1 potion, 12 pièces d'argent et la capsule d'attaque Brasier."
        );
        let avec_pillard = &raconter(&[Evenement::Coffre {
            potions: 1,
            argent: 22,
            argent_du_pillard: 7,
            capsule: None,
            capsule_d_habilete: false,
            pierre: None,
            booster: None,
            filets: None,
        }])[0];
        assert_eq!(
            avec_pillard.texte,
            "Vous trouvez un coffre : 1 potion et 22 pièces d'argent, dont 7 grâce à l'habileté Pillard."
        );
        let avec_pierre = &raconter(&[Evenement::Coffre {
            potions: 2,
            argent: 0,
            argent_du_pillard: 0,
            capsule: None,
            capsule_d_habilete: false,
            pierre: Some(bersemon_core::pierre::PierreDEvolution::Lune),
            booster: None,
            filets: None,
        }])[0];
        assert_eq!(
            avec_pierre.texte,
            "Vous trouvez un coffre : 2 potions et 1 Pierre de lune."
        );
        let avec_booster = &raconter(&[Evenement::Coffre {
            potions: 1,
            argent: 0,
            argent_du_pillard: 0,
            capsule: None,
            capsule_d_habilete: false,
            pierre: None,
            booster: Some(bersemon_core::booster::Booster::EcorceDeFer),
            filets: None,
        }])[0];
        assert_eq!(
            avec_booster.texte,
            "Vous trouvez un coffre : 1 potion et 1 Écorce de fer."
        );
    }

    #[test]
    fn le_fouineur_annonce_sa_potion() {
        use bersemon_core::catalogue::Catalogue;
        let contenu = bersemon_core::contenu::ContenuV1::new();
        let loup = bersemon_core::monstre::Monstre::creer(
            contenu
                .espece(&bersemon_core::espece::EspeceId::new("loup"))
                .unwrap(),
            bersemon_core::niveau::Niveau::new(5).unwrap(),
            bersemon_core::stats::ValeursInternes::default(),
            false,
        )
        .unwrap();
        let recit = &raconter(&[Evenement::PotionTrouvee(loup)])[0];
        assert_eq!(
            recit.texte,
            "Grâce à son habileté Fouineur, **Loup** déniche une potion, rangée dans le sac."
        );
    }

    #[test]
    fn le_brigand_vaincu_laisse_sa_bourse() {
        let recit = &raconter(&[Evenement::HumainVaincu {
            texte: "Le brigand détale.".to_string(),
            argent: 60,
        }])[0];
        assert_eq!(
            recit.texte,
            "Le brigand détale. Vous ramassez 60 pièces d'argent, qui vont dans le sac."
        );
        assert!(recit.nouveau_paragraphe);
    }

    fn brasier() -> bersemon_core::capsule::Capsule {
        use bersemon_core::catalogue::Catalogue;
        bersemon_core::contenu::ContenuV1::new()
            .capsule(&bersemon_core::attaque::AttaqueId::new("brasier"))
            .unwrap()
    }

    #[test]
    fn une_defaite_dit_ce_que_contenait_le_sac() {
        let sac = bersemon_core::inventaire::Inventaire::new(1, 2, vec![brasier()]);
        let recit = &raconter(&[Evenement::Defaite { sac_perdu: sac }])[0];
        assert_eq!(
            recit.texte,
            "**Votre héros et toute votre équipe sont KO : vous rentrez au camp et perdez tout votre sac : 2 potions, 1 pièce d'argent et la capsule d'attaque Brasier.**"
        );
        let vide = Evenement::Defaite {
            sac_perdu: bersemon_core::inventaire::Inventaire::default(),
        };
        assert_eq!(
            raconter(&[vide])[0].texte,
            "**Votre héros et toute votre équipe sont KO : vous rentrez au camp, le sac vide.**"
        );
    }

    #[test]
    fn une_ame_recueillie_dit_ce_que_malvoise_vendra() {
        let recit = &raconter(&[Evenement::AmeDonnee {
            marchand: "Ulrich".to_string(),
            soin: bersemon_core::soin::Soin::Elixir,
        }])[0];
        assert_eq!(
            recit.texte,
            "Vous remettez l'âme d'Ulrich à Malvoise, qui l'enferme dans le crâne. Malvoise vendra désormais l'élixir."
        );
    }

    #[test]
    fn achat_et_vente_disent_le_prix() {
        let recit = &raconter(&[Evenement::SoinsAchetes {
            soin: bersemon_core::soin::Soin::GrandePotion,
            nombre: 2,
            prix: 120,
        }])[0];
        assert_eq!(
            recit.texte,
            "Vous achetez 2 grandes potions pour 120 pièces d'argent."
        );
    }

    #[test]
    fn ranger_au_coffre_dit_ce_qui_a_ete_range() {
        let range = bersemon_core::inventaire::Inventaire::new(40, 0, Vec::new());
        let recit = &raconter(&[Evenement::RangeAuCoffre(range)])[0];
        assert_eq!(
            recit.texte,
            "Vous rangez dans le coffre : 40 pièces d'argent."
        );
        let rien = bersemon_core::inventaire::Inventaire::default();
        let recit = &raconter(&[Evenement::RangeAuCoffre(rien)])[0];
        assert_eq!(recit.texte, "Le sac est déjà vide.");
    }
}
