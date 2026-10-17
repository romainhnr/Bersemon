//! Partie en cours : le héros et tout ce que le joueur possède ou sait. C'est
//! ce qu'une sauvegarde devra conserver.

use crate::aleatoire::Aleatoire;
use crate::codex::Codex;
use crate::collection::{Collection, ErreurCollection, Placement};
use crate::combat::{BilanCombat, Combat, ErreurCombat, Issue};
use crate::expedition::Expedition;
use crate::heros::Heros;
use crate::inventaire::{ErreurPotion, Inventaire};
use crate::monstre::Monstre;
use crate::zone::{Zone, ZoneId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Partie {
    heros: Heros,
    collection: Collection,
    inventaire: Inventaire,
    codex: Codex,
    zones_explorees: Vec<ZoneId>,
}

impl Partie {
    /// Le monstre de départ compte comme capturé dans le codex (Q54).
    pub fn nouvelle(heros: Heros, depart: Monstre) -> Self {
        let mut codex = Codex::default();
        codex.noter_capture(depart.espece().id());
        Self {
            heros,
            collection: Collection::new(depart),
            inventaire: Inventaire::default(),
            codex,
            zones_explorees: Vec::new(),
        }
    }

    pub fn heros(&self) -> &Heros {
        &self.heros
    }

    pub fn collection(&self) -> &Collection {
        &self.collection
    }

    /// Gestion des monstres au camp (section 13.2).
    pub fn collection_mut(&mut self) -> &mut Collection {
        &mut self.collection
    }

    pub fn inventaire(&self) -> &Inventaire {
        &self.inventaire
    }

    pub fn codex(&self) -> &Codex {
        &self.codex
    }

    pub fn est_exploree(&self, zone: &ZoneId) -> bool {
        self.zones_explorees.contains(zone)
    }

    pub fn zones_explorees(&self) -> &[ZoneId] {
        &self.zones_explorees
    }

    /// Reconstitue une partie depuis une sauvegarde : chaque morceau a déjà
    /// été revérifié par sa propre restauration.
    pub fn restaurer(
        heros: Heros,
        collection: Collection,
        inventaire: Inventaire,
        codex: Codex,
        zones_explorees: Vec<ZoneId>,
    ) -> Self {
        Self {
            heros,
            collection,
            inventaire,
            codex,
            zones_explorees,
        }
    }

    /// La première zone est toujours accessible ; chacune des suivantes
    /// apparaît sur la carte une fois la précédente explorée (Q40).
    pub fn zones_accessibles<'a>(&self, zones: &'a [Zone]) -> Vec<&'a Zone> {
        let mut accessibles = Vec::new();
        for (position, zone) in zones.iter().enumerate() {
            if position > 0 && !self.est_exploree(zones[position - 1].id()) {
                break;
            }
            accessibles.push(zone);
        }
        accessibles
    }

    /// Combat contre un groupe sauvage, avec l'équipe, le bonus de classe du
    /// héros et les potions portées. Les espèces du groupe entrent dans le
    /// codex comme rencontrées.
    pub fn preparer_combat(&mut self, groupe: Vec<Monstre>) -> Result<Combat, ErreurCombat> {
        let especes: Vec<_> = groupe
            .iter()
            .map(|monstre| monstre.espece().id().clone())
            .collect();
        let combat = Combat::new(self.collection.equipe().to_vec(), groupe)?
            .avec_bonus_du_joueur(self.heros.classe().bonus())
            .avec_potions(self.inventaire.potions());
        for espece in &especes {
            self.codex.noter_rencontre(espece);
        }
        Ok(combat)
    }

    /// Reprend l'équipe et les potions à la fin du combat, inscrit les
    /// captures au codex et les rend, à placer ensuite avec
    /// `Collection::accueillir_capture`. Une défaite fait perdre toutes les
    /// potions (section 9.3). Les choix d'attaques en attente se font sur
    /// l'équipe du bilan, avant cet appel.
    pub fn conclure_combat(
        &mut self,
        bilan: BilanCombat,
    ) -> Result<Vec<Monstre>, ErreurCollection> {
        self.collection.actualiser_equipe(bilan.equipe)?;
        self.inventaire.definir_potions(bilan.potions);
        if bilan.issue == Some(Issue::Defaite) {
            self.inventaire.perdre_tout();
        }
        for capture in &bilan.captures {
            self.codex.noter_capture(capture.espece().id());
        }
        Ok(bilan.captures)
    }

    /// Un monstre aperçu, comme un monstre rare qui n'attaque pas.
    pub fn noter_rencontre(&mut self, monstre: &Monstre) {
        self.codex.noter_rencontre(monstre.espece().id());
    }

    /// Monstre rare non hostile que le joueur choisit de prendre (section
    /// 9.2) : il compte comme capturé.
    pub fn prendre_monstre_rare(&mut self, monstre: Monstre) -> Placement {
        self.codex.noter_capture(monstre.espece().id());
        self.collection.accueillir_capture(monstre)
    }

    pub fn ouvrir_coffre(&mut self, potions: u32) {
        self.inventaire.ajouter_potions(potions);
    }

    /// Potion utilisée entre deux étapes d'une expédition. Rend les PV
    /// rendus.
    pub fn utiliser_potion(
        &mut self,
        index_equipe: usize,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<u32, ErreurPotion> {
        let monstre = self
            .collection
            .membre_de_l_equipe_mut(index_equipe)
            .ok_or(ErreurPotion::MonstreInexistant)?;
        self.inventaire.utiliser_potion(monstre, aleatoire)
    }

    /// Tous les monstres sont soignés (section 13.1) ; la zone devient
    /// explorée si l'expédition est allée jusqu'au bout sans défaite. Rend
    /// `true` si la zone vient d'être explorée pour la première fois.
    pub fn rentrer_au_camp(&mut self, expedition: &Expedition) -> bool {
        self.collection.soigner_tout();
        let zone = expedition.zone().id();
        if expedition.est_reussie() && !self.est_exploree(zone) {
            self.zones_explorees.push(zone.clone());
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::codex::EtatCodex;
    use crate::combat::Action;
    use crate::donnees_de_test::espece;
    use crate::espece::EspeceId;
    use crate::heros::Classe;
    use crate::niveau::Niveau;
    use crate::nom::Nom;
    use crate::stats::ValeursInternes;

    fn creer(id: &str, niveau: u8) -> Monstre {
        Monstre::creer(
            espece(id),
            Niveau::new(niveau).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    fn partie() -> Partie {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier);
        Partie::nouvelle(heros, creer("depart", 5))
    }

    fn zone(id: &str) -> Zone {
        let niveau = Niveau::new(5).unwrap();
        Zone::new(
            ZoneId::new(id),
            id,
            (niveau, niveau),
            1,
            vec![espece("loup")],
            vec![],
        )
        .unwrap()
    }

    #[test]
    fn le_monstre_de_depart_est_capture_au_codex() {
        let partie = partie();
        assert_eq!(
            partie.codex().etat(&EspeceId::new("depart")),
            EtatCodex::Capturee
        );
        assert_eq!(partie.collection().equipe().len(), 1);
    }

    #[test]
    fn les_zones_se_devoilent_une_a_une() {
        let zones = vec![zone("bois"), zone("marais"), zone("cimes")];
        let mut partie = partie();
        let ids = |partie: &Partie| -> Vec<String> {
            partie
                .zones_accessibles(&zones)
                .iter()
                .map(|zone| zone.id().as_str().to_string())
                .collect()
        };
        assert_eq!(ids(&partie), vec!["bois"]);

        let mut expedition = Expedition::partir(zones[0].clone(), &mut AleatoireScripte::new([4]));
        for _ in 0..4 {
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 1]));
        }
        assert!(partie.rentrer_au_camp(&expedition));
        assert!(!partie.rentrer_au_camp(&expedition));
        assert_eq!(ids(&partie), vec!["bois", "marais"]);
    }

    #[test]
    fn rentrer_avant_la_fin_n_explore_pas_la_zone() {
        let mut partie = partie();
        let mut expedition = Expedition::partir(zone("bois"), &mut AleatoireScripte::new([4]));
        expedition.etape_suivante(&mut AleatoireScripte::new([81, 1]));
        assert!(!partie.rentrer_au_camp(&expedition));
        assert!(!partie.est_exploree(&ZoneId::new("bois")));
    }

    #[test]
    fn combat_note_les_rencontres_et_les_captures_et_rend_les_potions() {
        let mut partie = partie();
        partie.ouvrir_coffre(2);
        let mut loup = creer("loup", 5);
        loup.subir_degats(loup.stats().pv - 2);
        let mut combat = partie.preparer_combat(vec![loup]).unwrap();
        assert_eq!(combat.potions(), 2);
        assert_eq!(
            partie.codex().etat(&EspeceId::new("loup")),
            EtatCodex::Rencontree
        );
        combat
            .jouer_tour(Action::LancerFilet, &mut AleatoireScripte::new([90]))
            .unwrap();
        let bilan = combat.terminer(&mut AleatoireScripte::new([100]));
        let captures = partie.conclure_combat(bilan).unwrap();
        assert_eq!(captures.len(), 1);
        assert_eq!(
            partie.codex().etat(&EspeceId::new("loup")),
            EtatCodex::Capturee
        );
        assert_eq!(partie.inventaire().potions(), 2);
    }

    #[test]
    fn une_defaite_fait_perdre_les_potions() {
        let mut partie = partie();
        partie.ouvrir_coffre(3);
        let combat = partie.preparer_combat(vec![creer("loup", 50)]).unwrap();
        let mut bilan = combat.terminer(&mut AleatoireScripte::new([]));
        bilan.issue = Some(Issue::Defaite);
        partie.conclure_combat(bilan).unwrap();
        assert_eq!(partie.inventaire().potions(), 0);
    }

    #[test]
    fn potion_entre_deux_etapes() {
        let mut partie = partie();
        partie.ouvrir_coffre(1);
        assert_eq!(
            partie.utiliser_potion(0, &mut AleatoireScripte::new([])),
            Err(ErreurPotion::PvDejaPleins)
        );
        assert_eq!(
            partie.utiliser_potion(3, &mut AleatoireScripte::new([])),
            Err(ErreurPotion::MonstreInexistant)
        );
        let mut equipe = partie.collection().equipe().to_vec();
        equipe[0].subir_degats(10);
        partie.collection_mut().actualiser_equipe(equipe).unwrap();
        assert_eq!(
            partie.utiliser_potion(0, &mut AleatoireScripte::new([50])),
            Ok(10)
        );
        assert_eq!(partie.inventaire().potions(), 0);
    }

    #[test]
    fn prendre_un_monstre_rare_le_capture() {
        let mut partie = partie();
        let placement = partie.prendre_monstre_rare(creer("cerf-blanc", 5));
        assert_eq!(placement, Placement::Equipe(1));
        assert_eq!(
            partie.codex().etat(&EspeceId::new("cerf-blanc")),
            EtatCodex::Capturee
        );
    }
}
