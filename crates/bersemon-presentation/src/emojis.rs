//! Émojis des rendus de Discord (étape 215) : un par sorte d'événement du
//! récit, par ligne de combat, par moment et par météo, pour rendre le jeu
//! plus vivant. La console ne s'en sert pas : la console de Windows 10 les
//! affiche mal.

use bersemon_core::combat::{Evenement as EvenementDeCombat, Issue};
use bersemon_core::deroulement::Evenement;
use bersemon_core::jour_et_nuit::JourOuNuit;
use bersemon_core::meteo::Meteo;
use bersemon_core::statut::Statut;

pub const AME: &str = "\u{1F47B}";
pub const ARGENT: &str = "\u{1F4B0}";
pub const AVERTISSEMENT: &str = "\u{26A0}\u{FE0F}";
pub const BOOSTER: &str = "\u{1F4AA}";
pub const CAMP: &str = "\u{1F3D5}\u{FE0F}";
pub const CAPTURE: &str = "\u{1F389}";
pub const COFFRE_DU_CAMP: &str = "\u{1F9F0}";
pub const COFFRE_TROUVE: &str = "\u{1F4E6}";
pub const COMBAT: &str = "\u{2694}\u{FE0F}";
pub const CONSEIL: &str = "\u{1F4A1}";
pub const DEFAITE: &str = "\u{1F480}";
pub const DEGATS: &str = "\u{1F4A5}";
pub const DEPART: &str = "\u{1F9ED}";
pub const ECHEC: &str = "\u{1F4A8}";
pub const ENTRAINEMENT: &str = "\u{1F3CB}\u{FE0F}";
pub const ENTREE: &str = "\u{1F4E5}";
pub const EQUIPEMENT: &str = "\u{1F5E1}\u{FE0F}";
pub const EVOLUTION: &str = "\u{1F98B}";
pub const EXPERIENCE: &str = "\u{1F4C8}";
pub const FILET: &str = "\u{1F578}\u{FE0F}";
pub const FORGE: &str = "\u{2692}\u{FE0F}";
pub const FUITE: &str = "\u{1F3C3}";
pub const HEROS: &str = "\u{1F6E1}\u{FE0F}";
pub const INDICE: &str = "\u{1F4DC}";
pub const LEGENDAIRE: &str = "\u{1F31F}";
pub const LIBERATION: &str = "\u{1F54A}\u{FE0F}";
pub const MALVOISE: &str = "\u{1F6D2}";
pub const MONSTRE: &str = "\u{1F43E}";
pub const OBJET_D_EXPERIENCE: &str = "\u{1F4A0}";
pub const PARFUM: &str = "\u{1F338}";
pub const PECHE: &str = "\u{1F3A3}";
pub const RARE: &str = "\u{2728}";
pub const RESSOURCES: &str = "\u{1F392}";
pub const RETOUR: &str = "\u{21A9}\u{FE0F}";
pub const RITE: &str = "\u{1FA78}";
pub const SAC: &str = "\u{1F392}";
pub const SANS_EFFET: &str = "\u{1F6AB}";
pub const SOIN: &str = "\u{1F9EA}";
pub const SORTIE: &str = "\u{1F4E4}";
pub const TOUR: &str = "\u{1F3F0}";
pub const VALIDE: &str = "\u{2705}";
pub const VICTOIRE: &str = "\u{1F3C6}";
pub const ZONE: &str = "\u{1F5FA}\u{FE0F}";

pub fn emoji_du_moment(moment: JourOuNuit) -> &'static str {
    match moment {
        JourOuNuit::Jour => "\u{2600}\u{FE0F}",
        JourOuNuit::Nuit => "\u{1F319}",
    }
}

pub fn emoji_de_la_meteo(meteo: Meteo) -> &'static str {
    match meteo {
        Meteo::Pluie => "\u{1F327}\u{FE0F}",
        Meteo::Tempete => "\u{26C8}\u{FE0F}",
        Meteo::Grele => "\u{1F328}\u{FE0F}",
        Meteo::TempeteDeSable => "\u{1F3DC}\u{FE0F}",
        Meteo::Cyclone => "\u{1F300}",
        Meteo::Canicule => "\u{1F321}\u{FE0F}",
    }
}

/// L'émoji en tête du récit d'un événement ; aucun pour un combat, dont
/// chaque ligne a le sien (`emoji_de_combat`), ni pour l'entrée en chemin.
pub fn emoji_d_evenement(evenement: &Evenement) -> Option<&'static str> {
    use Evenement as E;
    Some(match evenement {
        E::Combat { .. } | E::EnChemin => return None,
        E::JourOuNuitAuDepart(moment) => emoji_du_moment(*moment),
        E::MeteoAuDepart(meteo) => emoji_de_la_meteo(*meteo),
        E::Introduction(_) | E::Indice { .. } | E::QueteEcoutee(_) | E::QueteRemise(_) => INDICE,
        E::PalierDeLaFosseVaincu { .. }
        | E::NouveauRecordDeLaTour(_)
        | E::HumainVaincu { .. }
        | E::SuccesObtenus { .. } => VICTOIRE,
        E::DefaiteDansLaFosse { .. } | E::Defaite { .. } | E::Suicide => DEFAITE,
        E::PartPourLeCamp(_) | E::RetourAuCamp | E::ExpeditionAutomatiqueRentree { .. } => CAMP,
        E::HerosShiny
        | E::MonstreRarePacifique(_)
        | E::MonstreRenduShiny(_)
        | E::HerosRenduShiny => RARE,
        E::Depart(_) | E::ExpeditionAutomatiquePartie(_) => DEPART,
        E::Etape { .. } => "\u{1F463}",
        E::EntreeDansLaTour(_) => TOUR,
        E::GardienDeLaTour(_) => HEROS,
        E::ReposDansLaTour(_) => "\u{1F4A4}",
        E::Coffre { .. } | E::ObjetDeQueteTrouve(_) => COFFRE_TROUVE,
        E::MonstreRareHostile(_)
        | E::GardienDeQuete(_)
        | E::SauvegardeImpossible(_)
        | E::EffacementImpossible(_) => AVERTISSEMENT,
        E::MonstreRareLaisse => "\u{1F6B6}",
        E::RecitDeLApotre(_) => "\u{1F441}\u{FE0F}",
        E::AnnexeDecouverte { .. } | E::NouvelleZone(_) => ZONE,
        E::LegendaireRencontre(_) | E::LegendaireCapture(_) => LEGENDAIRE,
        E::HumainRencontre(_) | E::Rencontre(_) => COMBAT,
        E::GuntherLibere(_) => "\u{1F513}",
        E::AmeRencontree(_)
        | E::AmeRecueillie(_)
        | E::AmeDApotreRecueillie(_)
        | E::AmeDonnee { .. } => AME,
        E::PecheurRencontre(_)
        | E::PecheurSauve(_)
        | E::CanneFabriquee(_)
        | E::LigneLancee { .. }
        | E::PrisePechee(_)
        | E::MatieresPechees(_) => PECHE,
        E::TitrePorte(_) => "\u{1F3F7}\u{FE0F}",
        E::CouleurPortee(_) => "\u{1F3A8}",
        E::Experience { .. } => EXPERIENCE,
        E::PotionTrouvee(_)
        | E::SoinUtilise { .. }
        | E::SoinsAchetes { .. }
        | E::RecompenseDuDemiurge => SOIN,
        E::RessourcesTrouvees(_) => RESSOURCES,
        E::BoostersLaches(_) | E::BoosterDonne { .. } => BOOSTER,
        E::ButinFlaire(_) => "\u{1F443}",
        E::RessourcesVendues { .. } | E::MonstreVendu { .. } => ARGENT,
        E::AttaqueRemplacee { .. } | E::AttaqueApprise { .. } => "\u{1F4D8}",
        E::AttaqueAbandonnee { .. } => "\u{274C}",
        E::Evolution { .. } => EVOLUTION,
        E::EvolutionRepoussee(_) => "\u{270B}",
        E::PartieCommencee(_) | E::RejointLEquipe(_) | E::MonstreLoue { .. } => "\u{1F91D}",
        E::CaptureRelachee(_)
        | E::RelachePourLaCapture { .. }
        | E::Relache(_)
        | E::PriseRelachee(_) => LIBERATION,
        E::Epilogue(_) => "\u{1F305}",
        E::PiegesPoses(_) | E::PiegesReleves(_) => "\u{1FAA4}",
        E::EntreeALEntrainement(_) | E::SortieDeLEntrainement { .. } => ENTRAINEMENT,
        E::RiteLance(_) | E::Sacrifice(_) | E::BeheritForge(_) => RITE,
        E::PetitNe(_) => "\u{1F95A}",
        E::RecompenseDuCodex(_) => "\u{1F381}",
        E::QueteAccomplie(_) | E::TravailConstate(_) => VALIDE,
        E::ObjetDExperienceRamasse(_)
        | E::ObjetsDExperienceAchetes { .. }
        | E::ObjetDExperienceDonne { .. } => OBJET_D_EXPERIENCE,
        E::OrdreDonne { .. } => "\u{1F4E3}",
        E::ZoneExploree(_) => "\u{1F6A9}",
        E::RaccourciActive(_) => "\u{1F4CC}",
        E::ObjetsJetes(_) => "\u{1F5D1}\u{FE0F}",
        E::FuiteAvantLeCombat => FUITE,
        E::Conseil(_) => CONSEIL,
        E::ExpeditionAutomatiqueRappelee { .. } => "\u{1F4EF}",
        E::MalvoiseRencontre { .. } | E::MonstreAchete { .. } | E::CapsuleAchetee { .. } => {
            MALVOISE
        }
        E::CapsulesDHabileteAchetees { .. } | E::HabileteChangee { .. } => "\u{1F52E}",
        E::PierresAchetees { .. } => "\u{1F48E}",
        E::AmeliorationLancee { .. } => "\u{1F528}",
        E::FabricationLancee { .. } | E::CharmeLance { .. } => FORGE,
        E::FiletsLances { .. } | E::FiletsAchetes { .. } => FILET,
        E::AppatPrepare(_) => "\u{1F356}",
        E::ParfumPrepare(_) | E::ParfumRepandu(_) => PARFUM,
        E::ObjetEquipe { .. } | E::ObjetRetire { .. } => EQUIPEMENT,
        E::MonstreTransforme { .. } => "\u{267B}\u{FE0F}",
        E::MonstreLoueRendu(_) | E::LocationFinie(_) => RETOUR,
        E::MonstreCede { .. } | E::RessourcesCedees { .. } | E::SortiDuCoffre(_) => SORTIE,
        E::MonstreRecu { .. } | E::RessourcesRecues { .. } | E::RangeAuCoffre(_) => ENTREE,
    })
}

/// L'émoji en tête d'une ligne du récit d'un combat.
pub fn emoji_de_combat(evenement: &EvenementDeCombat) -> &'static str {
    use EvenementDeCombat as E;
    match evenement {
        E::MonstreEnvoye(_) => MONSTRE,
        E::HerosEntre(_) | E::HerosRecule { .. } => HEROS,
        E::Metamorphose(_) => "\u{1F311}",
        E::AttaqueLancee { .. } => COMBAT,
        E::AttaqueRatee(_) | E::CaptureRatee(_) => ECHEC,
        E::SansEffet(_) => SANS_EFFET,
        E::Degats { .. } => DEGATS,
        E::StatutInflige {
            statut: Statut::Poison,
            ..
        }
        | E::DegatsDePoison { .. } => "\u{2623}\u{FE0F}",
        E::StatutInflige {
            statut: Statut::Paralysie,
            ..
        }
        | E::BloqueParLaParalysie(_) => "\u{26A1}",
        E::Desobeissance(_) => "\u{1F624}",
        E::Ko(_) | E::FinDuCombat(Issue::Defaite) => DEFAITE,
        E::FiletLance { .. } => FILET,
        E::AppatLance { .. } => "\u{1F356}",
        E::CaptureReussie(_) => CAPTURE,
        E::SoinUtilise { .. } => SOIN,
        E::Fuite | E::FinDuCombat(Issue::Fuite) => FUITE,
        E::FinDuCombat(Issue::Victoire) => VICTOIRE,
        E::Abandon(_) | E::Forfait(_) => "\u{1F3F3}\u{FE0F}",
    }
}

/// Le texte précédé de son émoji, devant sa première ligne ; rien pour un
/// texte vide.
pub fn devant(emoji: &str, texte: &str) -> String {
    if texte.is_empty() {
        return String::new();
    }
    format!("{emoji} {texte}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaque_moment_et_chaque_meteo_a_son_emoji() {
        assert_eq!(emoji_du_moment(JourOuNuit::Nuit), "\u{1F319}");
        let meteos = Meteo::TOUTES.map(emoji_de_la_meteo);
        for (rang, emoji) in meteos.iter().enumerate() {
            assert!(!meteos[rang + 1..].contains(emoji), "{emoji}");
        }
    }

    #[test]
    fn un_texte_vide_reste_vide() {
        assert_eq!(devant(DEFAITE, ""), "");
        assert_eq!(devant(DEFAITE, "Loup est KO."), "\u{1F480} Loup est KO.");
    }
}
