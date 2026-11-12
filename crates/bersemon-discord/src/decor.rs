//! Un émoji par sorte de bouton et de menu (étape 215), pour rendre les
//! écrans plus vivants. Il se met en tête du libellé ou de l'indication,
//! en texte : un émoji de composant que Discord ne reconnaîtrait pas ferait
//! refuser tout le message, pas un émoji dans un texte.

use bersemon_presentation::emojis;

use crate::composants::{Cle, Vue};

pub fn emoji_de_la_cle(cle: &Cle) -> Option<&'static str> {
    use Cle as C;
    Some(match cle {
        // Le bouton d'une attaque a son icône de type (étape 214), et le
        // choix de la classe ses sprites.
        C::Attaquer(_) | C::Classe(_) | C::NomSaisi | C::SurnomSaisi(_) => return None,
        C::Afficher(vue) => return emoji_de_la_vue(vue),
        C::ReprendreLaPartie => "\u{25B6}\u{FE0F}",
        C::NouvellePartie => "\u{1F195}",
        C::Nommer | C::Renommer(_) => "\u{270F}\u{FE0F}",
        C::Oui
        | C::GarderLaPrise
        | C::AccepterOffre
        | C::AccepterDefi
        | C::AccepterInvitation
        | C::Constater => emojis::VALIDE,
        C::Non | C::Renoncer | C::RefuserOffre | C::RefuserDefi | C::RefuserInvitation => {
            "\u{274C}"
        }
        C::Partir | C::Expedier { .. } => emojis::DEPART,
        C::PartirAilleurs => emojis::ZONE,
        C::Fiche(_) => "\u{1F4D6}",
        C::LieuDuJournal(_) => "\u{1F4CD}",
        C::Choisir(_) | C::ChoisirJoueur | C::FaireVenir(_) => emojis::MONSTRE,
        C::Sacrifier(_) | C::SecondParent { .. } => emojis::RITE,
        C::PartirVersLEclipse => "\u{1F311}",
        C::Monter(_) | C::EnTete => "\u{2B06}\u{FE0F}",
        C::Descendre(_) => "\u{2B07}\u{FE0F}",
        C::EnvoyerAuCamp(_)
        | C::Rentrer
        | C::Accueillir(_)
        | C::RevenirDuDuel
        | C::RamenerLeGroupe => emojis::CAMP,
        C::EchangerAvec(_) | C::Trier(_) | C::Changer | C::Remplacer(_) | C::Actualiser => {
            "\u{1F504}"
        }
        C::Relacher(_) | C::RelacherLaCapture | C::RelacherLaPrise | C::Liberer(_) => {
            emojis::LIBERATION
        }
        C::Continuer => "\u{27A1}\u{FE0F}",
        C::ActiverLeRaccourci => "\u{1F4CC}",
        C::Soin | C::SoinEnCombat | C::AcheterSoins => emojis::SOIN,
        C::Prendre | C::NePlusCapturer | C::NePasEvoluer => "\u{270B}",
        C::Poursuivre | C::Defier(_) => emojis::COMBAT,
        C::Laisser => "\u{1F6B6}",
        C::Ordre => "\u{1F4E3}",
        C::Filet(_) | C::Capturer(_) | C::AcheterFilets(..) | C::FabriquerFilets => emojis::FILET,
        C::LancerAppat | C::PreparerAppat => "\u{1F356}",
        C::Fuir => emojis::FUITE,
        C::Oublier(_) | C::Jeter | C::JeterUneRessource => "\u{1F5D1}\u{FE0F}",
        C::Evoluer | C::FaireEvoluer(_) => emojis::EVOLUTION,
        C::Ajouter { .. } | C::AjouterAmi(_) => "\u{2795}",
        C::RetirerAmi(_) => "\u{2796}",
        C::Rappeler(_) => "\u{1F4EF}",
        C::AcheterCapsule(_) => "\u{1F48A}",
        C::DonnerAme => emojis::AME,
        C::Vendre(_) | C::VendreCeMonstre(_) | C::VendreRessources(_) => emojis::ARGENT,
        C::Transformer(_) => "\u{267B}\u{FE0F}",
        C::ChangerDHabilete(_) | C::AcheterCapsuleDHabilete => "\u{1F52E}",
        C::DonnerBooster(_) => emojis::BOOSTER,
        C::RendreShiny(_) | C::RendreLeHerosShiny => emojis::RARE,
        C::AcheterMonstre(_) => emojis::MALVOISE,
        C::LouerMonstre(_) => "\u{1F91D}",
        C::RendreLeMonstreLoue
        | C::Reprendre(_)
        | C::RetirerOffre
        | C::RetirerDefi
        | C::RetirerInvitation => emojis::RETOUR,
        C::AcheterPierre(_) | C::AcheterObjetDEvolution => "\u{1F48E}",
        C::Apprendre { .. } => "\u{1F4D8}",
        C::SortirDuCoffre
        | C::SortirRessourcesDuCoffre(_)
        | C::Ceder { .. }
        | C::CederRessource { .. }
        | C::NombreACeder { .. } => emojis::SORTIE,
        C::RangerAuCoffre
        | C::RangerRessourcesAuCoffre(_)
        | C::ToutRanger
        | C::Demander { .. }
        | C::DemanderRessource { .. }
        | C::NombreADemander { .. } => emojis::ENTREE,
        C::SeSuicider => emojis::DEFAITE,
        C::Ameliorer(_) => "\u{1F528}",
        C::FabriquerCharme | C::Fabriquer => emojis::FORGE,
        C::PoserLesPieges | C::ReleverLesPieges => "\u{1FAA4}",
        C::Entrainer(_) => emojis::ENTRAINEMENT,
        C::LancerLaLigne | C::LancerAvec(_) | C::FabriquerCanne(_) | C::ReleverLaLigne => {
            emojis::PECHE
        }
        C::PreparerParfum(_) => emojis::PARFUM,
        C::Equiper(_) => emojis::EQUIPEMENT,
        C::PorterTitre(_) => "\u{1F3F7}\u{FE0F}",
        C::PorterCouleur => "\u{1F3A8}",
        C::EcouterQuete(_) => "\u{1F442}",
        C::RemettreQuete(_) => emojis::COFFRE_TROUVE,
        C::AccueillirLePetit => "\u{1F95A}",
        C::ReclamerRecompenseDuCodex | C::Donner { .. } => "\u{1F381}",
        C::DefierALArene(_) => "\u{1F3DF}\u{FE0F}",
        C::Abandonner | C::AbandonnerLaFosse => "\u{1F3F3}\u{FE0F}",
        C::ReclamerLaVictoire => emojis::VICTOIRE,
        C::Inviter(_) => "\u{2709}\u{FE0F}",
    })
}

fn emoji_de_la_vue(vue: &Vue) -> Option<&'static str> {
    use Vue as V;
    Some(match vue {
        V::Camp => emojis::CAMP,
        V::Codex(_) | V::PageDuCodex(_) => "\u{1F4D6}",
        V::Journal(_) => "\u{1F4D4}",
        V::Gestion(_) => emojis::MONSTRE,
        V::Expeditions | V::Preparation { .. } => emojis::DEPART,
        V::Marchand | V::Achats => emojis::MALVOISE,
        V::Vente => emojis::ARGENT,
        V::Transformation => "\u{267B}\u{FE0F}",
        V::Locations => "\u{1F91D}",
        V::Capsules => "\u{1F48A}",
        V::Eclipse => "\u{1F311}",
        V::Attaques(_) => emojis::COMBAT,
        V::Coffre => emojis::COFFRE_DU_CAMP,
        V::Ressources => emojis::RESSOURCES,
        V::Heros => emojis::HEROS,
        V::Succes(_) | V::Trophees | V::Classement => emojis::VICTOIRE,
        V::QuetesDuCamp | V::Quetes => emojis::INDICE,
        V::Forge => emojis::FORGE,
        V::Equipement => emojis::EQUIPEMENT,
        V::Peche => emojis::PECHE,
        V::Echanges { .. } => "\u{1F501}",
        V::RecordsDeLaTour => emojis::TOUR,
        V::ClassementDeLArene => "\u{1F3DF}\u{FE0F}",
        V::Machines => "\u{2699}\u{FE0F}",
        V::Entrainement | V::EntrainementDepuisLaCour => emojis::ENTRAINEMENT,
        V::Cour => "\u{1F333}",
        V::Rite(_) => emojis::RITE,
    })
}

/// "<émoji> <texte>", ou le texte seul.
pub fn decorer(cle: &Cle, texte: &str) -> String {
    match emoji_de_la_cle(cle) {
        Some(emoji) => format!("{emoji} {texte}"),
        None => texte.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaque_bouton_courant_a_son_emoji() {
        assert_eq!(decorer(&Cle::Fuir, "Fuir"), "\u{1F3C3} Fuir");
        assert_eq!(
            decorer(&Cle::Afficher(Vue::Camp), "Retour au camp"),
            "\u{1F3D5}\u{FE0F} Retour au camp"
        );
        assert_eq!(
            decorer(&Cle::Attaquer(0), "Morsure 60, 100 %"),
            "Morsure 60, 100 %"
        );
    }
}
