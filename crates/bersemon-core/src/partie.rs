//! Partie en cours : le héros et tout ce que le joueur possède ou sait. C'est
//! ce qu'une sauvegarde devra conserver.

use crate::aleatoire::Aleatoire;
use crate::attaque::ErreurAttaquesMonstre;
use crate::codex::Codex;
use crate::collection::{Collection, ErreurCollection, Placement};
use crate::combat::{BilanCombat, Combat, ErreurCombat, Issue};
use crate::espece::{Espece, EspeceId};
use crate::expedition::{ErreurExpedition, Expedition};
use crate::heros::Heros;
use crate::inventaire::{ErreurPotion, Inventaire};
use crate::monstre::{Monstre, tirer_shiny};
use crate::niveau::Niveau;
use crate::stats::ValeursInternes;
use crate::traque::{Apotre, Indice, Piste};
use crate::tutoriel::Conseil;
use crate::zone::{Zone, ZoneId};

/// Le monstre de départ est au niveau 5 (section 6).
const NIVEAU_DE_DEPART: u8 = 5;

/// Valeurs internes tirées au hasard et 1 chance sur 512 d'être shiny, comme
/// un monstre sauvage (section 6). Tirages : valeurs internes, puis shiny.
/// Échoue si l'espèce n'apprend aucune attaque jusqu'au niveau 5.
pub fn creer_monstre_de_depart(
    espece: Espece,
    aleatoire: &mut dyn Aleatoire,
) -> Result<Monstre, ErreurAttaquesMonstre> {
    let valeurs_internes = ValeursInternes::tirer(aleatoire);
    let shiny = tirer_shiny(aleatoire);
    let niveau = Niveau::new(NIVEAU_DE_DEPART).expect("niveau de départ valide");
    Monstre::creer(espece, niveau, valeurs_internes, shiny)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Partie {
    heros: Heros,
    collection: Collection,
    inventaire: Inventaire,
    codex: Codex,
    zones_explorees: Vec<ZoneId>,
    /// Indices trouvés sur la piste de l'Apôtre (section 9.4).
    indices_trouves: usize,
    apotres_vaincus: Vec<EspeceId>,
    conseils_vus: Vec<Conseil>,
}

/// Tout ce qu'une sauvegarde conserve d'une partie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtatPartie {
    pub heros: Heros,
    pub collection: Collection,
    pub inventaire: Inventaire,
    pub codex: Codex,
    pub zones_explorees: Vec<ZoneId>,
    pub indices_trouves: usize,
    pub apotres_vaincus: Vec<EspeceId>,
    pub conseils_vus: Vec<Conseil>,
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
            indices_trouves: 0,
            apotres_vaincus: Vec::new(),
            conseils_vus: Vec::new(),
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

    pub fn indices_trouves(&self) -> usize {
        self.indices_trouves
    }

    pub fn apotres_vaincus(&self) -> &[EspeceId] {
        &self.apotres_vaincus
    }

    pub fn est_vaincu(&self, apotre: &Apotre) -> bool {
        self.apotres_vaincus.contains(apotre.espece().id())
    }

    pub fn conseils_vus(&self) -> &[Conseil] {
        &self.conseils_vus
    }

    /// Reconstitue une partie depuis une sauvegarde : chaque morceau a déjà
    /// été revérifié par sa propre restauration.
    pub fn restaurer(etat: EtatPartie) -> Self {
        Self {
            heros: etat.heros,
            collection: etat.collection,
            inventaire: etat.inventaire,
            codex: etat.codex,
            zones_explorees: etat.zones_explorees,
            indices_trouves: etat.indices_trouves,
            apotres_vaincus: etat.apotres_vaincus,
            conseils_vus: etat.conseils_vus,
        }
    }

    /// Les zones ouvertes par la piste de l'Apôtre (section 9.1), dans
    /// l'ordre de la carte.
    pub fn zones_accessibles<'a>(&self, zones: &'a [Zone], piste: &Piste) -> Vec<&'a Zone> {
        zones
            .iter()
            .filter(|zone| piste.zone_ouverte(zone.id(), self.indices_trouves))
            .collect()
    }

    /// Une fois tous les indices trouvés, une expédition dans la zone de
    /// l'Apôtre s'achève par le combat contre lui, tant qu'il n'est pas
    /// vaincu (section 9.4).
    pub fn partir_en_expedition(
        &self,
        zone: &Zone,
        piste: &Piste,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<Expedition, ErreurExpedition> {
        if !piste.zone_ouverte(zone.id(), self.indices_trouves) {
            return Err(ErreurExpedition::ZoneFermee);
        }
        let expedition = Expedition::partir(zone.clone(), aleatoire);
        let apotre = piste.apotre();
        if piste.antre_localise(self.indices_trouves)
            && !self.est_vaincu(apotre)
            && apotre.zone() == zone.id()
        {
            return Ok(expedition.vers_l_antre(apotre.clone()));
        }
        Ok(expedition)
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

    /// Comme `preparer_combat`, sans fuite ni filet (section 9.4).
    pub fn preparer_combat_contre_apotre(
        &mut self,
        apotre: Monstre,
    ) -> Result<Combat, ErreurCombat> {
        self.preparer_combat(vec![apotre])
            .map(Combat::contre_un_apotre)
    }

    /// Après la victoire, capturé ou non, l'Apôtre ne réapparaît plus
    /// (section 9.4).
    pub fn noter_apotre_vaincu(&mut self, apotre: &Apotre) {
        if !self.est_vaincu(apotre) {
            self.apotres_vaincus.push(apotre.espece().id().clone());
        }
    }

    /// Rend `true` si le conseil n'avait encore jamais été montré : c'est le
    /// moment de l'afficher (section 6).
    pub fn noter_conseil_vu(&mut self, conseil: Conseil) -> bool {
        if self.conseils_vus.contains(&conseil) {
            return false;
        }
        self.conseils_vus.push(conseil);
        true
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

    /// Tous les monstres sont soignés (section 13.1). Une expédition menée
    /// jusqu'au bout sans défaite rend la zone explorée et, si l'indice
    /// suivant de la piste se trouve dans cette zone, le révèle et le rend
    /// (section 9.4).
    pub fn rentrer_au_camp<'p>(
        &mut self,
        expedition: &Expedition,
        piste: &'p Piste,
    ) -> Option<&'p Indice> {
        self.collection.soigner_tout();
        if !expedition.est_reussie() {
            return None;
        }
        let zone = expedition.zone().id();
        if !self.est_exploree(zone) {
            self.zones_explorees.push(zone.clone());
        }
        let indice = piste
            .indice_suivant(self.indices_trouves)
            .filter(|indice| &indice.zone == zone)?;
        self.indices_trouves += 1;
        Some(indice)
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
    use crate::expedition::EvenementExpedition;
    use crate::heros::Classe;
    use crate::nom::Nom;

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

    /// Indices dans le bois, puis deux fois dans le marais, où attend
    /// l'Apôtre ; la zone des cimes n'est sur aucune piste.
    fn carte() -> (Vec<Zone>, Piste) {
        let zones = vec![zone("bois"), zone("marais"), zone("cimes")];
        let indice = |zone: &str| Indice {
            zone: ZoneId::new(zone),
            texte: String::new(),
        };
        let apotre = Apotre::new(espece("baron"), &zones[1], "", "").unwrap();
        let piste = Piste::new(
            vec![indice("bois"), indice("marais"), indice("marais")],
            apotre,
        )
        .unwrap();
        (zones, piste)
    }

    fn ids(zones: &[&Zone]) -> Vec<String> {
        zones
            .iter()
            .map(|zone| zone.id().as_str().to_string())
            .collect()
    }

    /// Expédition de 4 étapes, toutes des coffres.
    fn expedition_complete(partie: &Partie, zone: &Zone, piste: &Piste) -> Expedition {
        let mut expedition = partie
            .partir_en_expedition(zone, piste, &mut AleatoireScripte::new([4]))
            .unwrap();
        for _ in 0..4 {
            assert!(matches!(
                expedition.etape_suivante(&mut AleatoireScripte::new([81, 1])),
                Some(EvenementExpedition::Coffre { .. })
            ));
        }
        expedition
    }

    #[test]
    fn les_indices_ouvrent_la_piste_jusqu_a_l_antre() {
        let (zones, piste) = carte();
        let mut partie = partie();
        assert_eq!(ids(&partie.zones_accessibles(&zones, &piste)), vec!["bois"]);

        let expedition = expedition_complete(&partie, &zones[0], &piste);
        assert_eq!(
            partie.rentrer_au_camp(&expedition, &piste),
            Some(&piste.indices()[0])
        );
        assert!(partie.est_exploree(&ZoneId::new("bois")));
        assert_eq!(
            ids(&partie.zones_accessibles(&zones, &piste)),
            vec!["bois", "marais"]
        );
        // L'indice suivant est dans le marais : le bois n'en donne plus.
        let expedition = expedition_complete(&partie, &zones[0], &piste);
        assert_eq!(partie.rentrer_au_camp(&expedition, &piste), None);

        for numero in 2..=3 {
            let expedition = expedition_complete(&partie, &zones[1], &piste);
            assert!(partie.rentrer_au_camp(&expedition, &piste).is_some());
            assert_eq!(partie.indices_trouves(), numero);
        }
        assert_eq!(
            partie.partir_en_expedition(&zones[2], &piste, &mut AleatoireScripte::new([])),
            Err(ErreurExpedition::ZoneFermee)
        );
    }

    #[test]
    fn rentrer_avant_la_fin_ne_donne_pas_d_indice() {
        let (zones, piste) = carte();
        let mut partie = partie();
        let mut expedition = partie
            .partir_en_expedition(&zones[0], &piste, &mut AleatoireScripte::new([4]))
            .unwrap();
        expedition.etape_suivante(&mut AleatoireScripte::new([81, 1]));
        assert_eq!(partie.rentrer_au_camp(&expedition, &piste), None);
        assert!(!partie.est_exploree(&ZoneId::new("bois")));
        assert_eq!(partie.indices_trouves(), 0);
    }

    #[test]
    fn l_apotre_attend_au_bout_de_l_expedition_tant_qu_il_n_est_pas_vaincu() {
        let (zones, piste) = carte();
        let mut partie = partie();
        partie.indices_trouves = 3;
        let mut expedition = partie
            .partir_en_expedition(&zones[1], &piste, &mut AleatoireScripte::new([4]))
            .unwrap();
        for _ in 0..3 {
            expedition.etape_suivante(&mut AleatoireScripte::new([81, 1]));
        }
        let Some(EvenementExpedition::Apotre(apotre)) =
            expedition.etape_suivante(&mut AleatoireScripte::new([2]))
        else {
            panic!("l'Apôtre devait attendre à la dernière étape");
        };
        let combat = partie.preparer_combat_contre_apotre(apotre).unwrap();
        assert!(combat.est_contre_un_apotre());

        partie.noter_apotre_vaincu(piste.apotre());
        partie.noter_apotre_vaincu(piste.apotre());
        assert_eq!(partie.apotres_vaincus(), &[EspeceId::new("baron")]);
        let expedition = expedition_complete(&partie, &zones[1], &piste);
        assert_eq!(partie.rentrer_au_camp(&expedition, &piste), None);
    }

    #[test]
    fn chaque_conseil_n_est_montre_qu_une_fois() {
        let mut partie = partie();
        assert!(partie.noter_conseil_vu(Conseil::PremierCombat));
        assert!(!partie.noter_conseil_vu(Conseil::PremierCombat));
        assert!(partie.noter_conseil_vu(Conseil::PremierRetourAuCamp));
        assert_eq!(
            partie.conseils_vus(),
            &[Conseil::PremierCombat, Conseil::PremierRetourAuCamp]
        );
    }

    #[test]
    fn monstre_de_depart_au_niveau_5() {
        let mut tirages = vec![31; 6];
        tirages.push(1);
        let depart =
            creer_monstre_de_depart(espece("depart"), &mut AleatoireScripte::new(tirages)).unwrap();
        assert_eq!(depart.niveau(), Niveau::new(5).unwrap());
        assert!(depart.est_shiny());
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
        partie.ouvrir_coffre(2);
        assert_eq!(
            partie.utiliser_potion(3, &mut AleatoireScripte::new([])),
            Err(ErreurPotion::MonstreInexistant)
        );
        // Sur un monstre qui a tous ses PV, la potion est perdue (Q53).
        assert_eq!(
            partie.utiliser_potion(0, &mut AleatoireScripte::new([])),
            Ok(0)
        );
        assert_eq!(partie.inventaire().potions(), 1);
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
