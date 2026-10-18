//! Partie en cours : le héros et tout ce que le joueur possède ou sait. C'est
//! ce qu'une sauvegarde devra conserver.

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::attaque::{Attaque, AttaqueId, ErreurAttaquesMonstre};
use crate::capsule::{Capsule, ErreurApprentissage, attaques_a_apprendre};
use crate::codex::Codex;
use crate::collection::{Collection, Emplacement, ErreurCollection, Placement};
use crate::combat::{BilanCombat, Combat, ErreurCombat, Issue};
use crate::espece::{Espece, EspeceId};
use crate::expedition::{ErreurExpedition, Expedition};
use crate::expedition_automatique::{
    BilanExpeditionAutomatique, ErreurExpeditionAutomatique, Estimation, ExpeditionAutomatique,
    MONSTRES_MAX, MONSTRES_MIN,
};
use crate::heros::Heros;
use crate::horodatage::Horodatage;
use crate::inventaire::{ErreurPotion, Inventaire};
use crate::marchand::{ErreurMarchand, PRIX_DE_LA_POTION, prix_de_revente};
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
    /// Gardé au camp : une défaite ne fait perdre que celui trouvé pendant
    /// l'expédition (section 9.3).
    argent: u32,
    /// Une fois rencontré, le marchand reste au camp (section 14.2).
    marchand_au_camp: bool,
    /// Dans l'ordre où le joueur les a obtenues, sans doublon (section
    /// 13.10).
    capsules: Vec<Capsule>,
    codex: Codex,
    zones_explorees: Vec<ZoneId>,
    /// Indices trouvés sur la piste de l'Apôtre (section 9.4).
    indices_trouves: usize,
    apotres_vaincus: Vec<EspeceId>,
    conseils_vus: Vec<Conseil>,
    /// Une au plus par zone, dans l'ordre des départs (section 12.1).
    expeditions_automatiques: Vec<ExpeditionAutomatique>,
    /// Dans l'ordre où le joueur les choisira. Tant qu'il en reste, le
    /// déroulement n'accepte aucune autre action au camp : les positions
    /// qu'elles désignent ne bougent donc pas.
    attaques_a_choisir: Vec<AttaqueAChoisir>,
}

/// Attaque qu'un monstre rentré d'expédition automatique veut apprendre en
/// plus des 4 qu'il connaît : le joueur lui en fait oublier une ou y renonce
/// (sections 7 et 12.7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttaqueAChoisir {
    pub position_au_camp: usize,
    pub attaque: Attaque,
}

/// Tout ce qu'une sauvegarde conserve d'une partie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtatPartie {
    pub heros: Heros,
    pub collection: Collection,
    pub inventaire: Inventaire,
    pub argent: u32,
    pub marchand_au_camp: bool,
    pub capsules: Vec<Capsule>,
    pub codex: Codex,
    pub zones_explorees: Vec<ZoneId>,
    pub indices_trouves: usize,
    pub apotres_vaincus: Vec<EspeceId>,
    pub conseils_vus: Vec<Conseil>,
    pub expeditions_automatiques: Vec<ExpeditionAutomatique>,
    pub attaques_a_choisir: Vec<AttaqueAChoisir>,
}

/// Sauvegarde lisible mais contraire aux règles des expéditions
/// automatiques (section 12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurRestaurationPartie {
    /// Le camp n'a pas la place de garder celle des monstres partis.
    Collection(ErreurCollection),
    ZoneNonExploree(ZoneId),
    ExpeditionEnDouble(ZoneId),
    /// Aucun monstre du camp à cette position.
    AttaqueAChoisirSansMonstre(usize),
    CapsuleEnDouble(AttaqueId),
}

impl fmt::Display for ErreurRestaurationPartie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurRestaurationPartie::Collection(erreur) => write!(f, "{erreur}"),
            ErreurRestaurationPartie::ZoneNonExploree(zone) => write!(
                f,
                "expédition automatique dans une zone non explorée : {}",
                zone.as_str()
            ),
            ErreurRestaurationPartie::ExpeditionEnDouble(zone) => write!(
                f,
                "deux expéditions automatiques dans la zone {}",
                zone.as_str()
            ),
            ErreurRestaurationPartie::AttaqueAChoisirSansMonstre(position) => write!(
                f,
                "attaque à choisir pour un monstre absent du camp (position {position})"
            ),
            ErreurRestaurationPartie::CapsuleEnDouble(capsule) => {
                write!(f, "capsule obtenue deux fois : {}", capsule.as_str())
            }
        }
    }
}

impl std::error::Error for ErreurRestaurationPartie {}

impl Partie {
    /// Le monstre de départ compte comme capturé dans le codex (Q54).
    pub fn nouvelle(heros: Heros, depart: Monstre) -> Self {
        let mut codex = Codex::default();
        codex.noter_capture(depart.espece().id());
        Self {
            heros,
            collection: Collection::new(depart),
            inventaire: Inventaire::default(),
            argent: 0,
            marchand_au_camp: false,
            capsules: Vec::new(),
            codex,
            zones_explorees: Vec::new(),
            indices_trouves: 0,
            apotres_vaincus: Vec::new(),
            conseils_vus: Vec::new(),
            expeditions_automatiques: Vec::new(),
            attaques_a_choisir: Vec::new(),
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

    /// En pièces d'argent (section 14.1).
    pub fn argent(&self) -> u32 {
        self.argent
    }

    pub fn marchand_au_camp(&self) -> bool {
        self.marchand_au_camp
    }

    /// Le marchand rencontré se réfugie au camp pour de bon, même si
    /// l'expédition finit en défaite (section 14.2).
    pub(crate) fn accueillir_marchand(&mut self) {
        self.marchand_au_camp = true;
    }

    /// Au camp, sans confirmation (section 14.2). Rend le prix payé.
    pub fn acheter_potions(&mut self, nombre: u32) -> Result<u32, ErreurMarchand> {
        self.verifier_marchand()?;
        if nombre == 0 {
            return Err(ErreurMarchand::QuantiteNulle);
        }
        let prix = nombre.saturating_mul(PRIX_DE_LA_POTION);
        self.payer(prix)?;
        self.inventaire.ajouter_potions(nombre);
        Ok(prix)
    }

    /// Le monstre que la vente retirerait et son prix, sans rien vendre :
    /// de quoi demander confirmation au joueur. Refuse ce que refuserait
    /// `vendre`.
    pub fn estimer_vente(
        &self,
        emplacement: Emplacement,
    ) -> Result<(Monstre, u32), ErreurMarchand> {
        self.verifier_marchand()?;
        let monstre = self
            .collection
            .clone()
            .relacher(emplacement)
            .map_err(ErreurMarchand::Collection)?;
        let prix = prix_de_revente(&monstre);
        Ok((monstre, prix))
    }

    /// Définitive et soumise aux règles du relâcher : jamais le dernier
    /// monstre de l'équipe (section 14.2). La confirmation revient au
    /// déroulement. Rend le monstre vendu et son prix.
    pub fn vendre(&mut self, emplacement: Emplacement) -> Result<(Monstre, u32), ErreurMarchand> {
        self.verifier_marchand()?;
        let monstre = self
            .collection
            .relacher(emplacement)
            .map_err(ErreurMarchand::Collection)?;
        let prix = prix_de_revente(&monstre);
        self.argent = self.argent.saturating_add(prix);
        Ok((monstre, prix))
    }

    pub fn capsules(&self) -> &[Capsule] {
        &self.capsules
    }

    pub fn possede_capsule(&self, id: &AttaqueId) -> bool {
        self.capsules.iter().any(|capsule| capsule.id() == id)
    }

    /// Une capsule déjà obtenue n'est pas ajoutée deux fois : elle sert sans
    /// s'user (section 13.10).
    fn obtenir_capsule(&mut self, capsule: Capsule) {
        if !self.possede_capsule(capsule.id()) {
            self.capsules.push(capsule);
        }
    }

    /// Au camp, sans confirmation (section 14.2). Rend le prix payé.
    pub fn acheter_capsule(&mut self, capsule: &Capsule) -> Result<u32, ErreurMarchand> {
        self.verifier_marchand()?;
        if self.possede_capsule(capsule.id()) {
            return Err(ErreurMarchand::CapsuleDejaPossedee);
        }
        self.payer(capsule.prix())?;
        self.capsules.push(capsule.clone());
        Ok(capsule.prix())
    }

    /// Ce que ce monstre peut apprendre au camp, gratuitement (section
    /// 13.10).
    pub fn attaques_a_apprendre(
        &self,
        emplacement: Emplacement,
    ) -> Result<Vec<Attaque>, ErreurCollection> {
        let monstre = self
            .collection
            .monstre(emplacement)
            .ok_or(ErreurCollection::EmplacementInexistant(emplacement))?;
        Ok(attaques_a_apprendre(monstre, &self.capsules))
    }

    /// Le monstre apprend l'attaque ; s'il en connaît déjà 4, il oublie
    /// celle en position `oubliee`, que l'interface a fait choisir au joueur
    /// (section 13.10). Rend l'attaque oubliée.
    pub fn apprendre_attaque(
        &mut self,
        emplacement: Emplacement,
        attaque: &AttaqueId,
        oubliee: Option<usize>,
    ) -> Result<Option<Attaque>, ErreurApprentissage> {
        let nouvelle = self
            .attaques_a_apprendre(emplacement)
            .map_err(ErreurApprentissage::Collection)?
            .into_iter()
            .find(|possible| possible.id() == attaque)
            .ok_or(ErreurApprentissage::AttaqueInaccessible)?;
        let monstre = self.collection.monstre_a_modifier(emplacement).ok_or(
            ErreurApprentissage::Collection(ErreurCollection::EmplacementInexistant(emplacement)),
        )?;
        match (monstre.attaques().est_complete(), oubliee) {
            (true, None) => Err(ErreurApprentissage::OubliManquant),
            (false, Some(_)) => Err(ErreurApprentissage::OubliInutile),
            (false, None) => monstre
                .ajouter_attaque(nouvelle)
                .map(|()| None)
                .map_err(ErreurApprentissage::Attaque),
            (true, Some(position)) => {
                let ancienne = monstre.attaques().liste().get(position).cloned().ok_or(
                    ErreurApprentissage::Attaque(ErreurAttaquesMonstre::PositionInexistante(
                        position,
                    )),
                )?;
                monstre
                    .remplacer_attaque(position, nouvelle)
                    .map(|()| Some(ancienne))
                    .map_err(ErreurApprentissage::Attaque)
            }
        }
    }

    fn verifier_marchand(&self) -> Result<(), ErreurMarchand> {
        if self.marchand_au_camp {
            Ok(())
        } else {
            Err(ErreurMarchand::AucunMarchand)
        }
    }

    fn payer(&mut self, prix: u32) -> Result<(), ErreurMarchand> {
        if prix > self.argent {
            return Err(ErreurMarchand::ArgentInsuffisant {
                prix,
                argent: self.argent,
            });
        }
        self.argent -= prix;
        Ok(())
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
    /// été revérifié par sa propre restauration. Il reste à vérifier ce qui
    /// les lie : les monstres partis gardent leur place au camp, une seule
    /// expédition automatique par zone explorée, et chaque attaque à choisir
    /// désigne un monstre du camp. Une capsule n'est obtenue qu'une fois.
    pub fn restaurer(etat: EtatPartie) -> Result<Self, ErreurRestaurationPartie> {
        for (position, capsule) in etat.capsules.iter().enumerate() {
            let precedentes = &etat.capsules[..position];
            if precedentes.iter().any(|autre| autre.id() == capsule.id()) {
                return Err(ErreurRestaurationPartie::CapsuleEnDouble(
                    capsule.id().clone(),
                ));
            }
        }
        let mut collection = etat.collection;
        let partis = etat
            .expeditions_automatiques
            .iter()
            .map(|expedition| expedition.monstres().len())
            .sum();
        collection
            .definir_places_reservees(partis)
            .map_err(ErreurRestaurationPartie::Collection)?;
        for (position, expedition) in etat.expeditions_automatiques.iter().enumerate() {
            let zone = expedition.zone().id();
            if !etat.zones_explorees.contains(zone) {
                return Err(ErreurRestaurationPartie::ZoneNonExploree(zone.clone()));
            }
            let precedentes = &etat.expeditions_automatiques[..position];
            if precedentes.iter().any(|autre| autre.zone().id() == zone) {
                return Err(ErreurRestaurationPartie::ExpeditionEnDouble(zone.clone()));
            }
        }
        for a_choisir in &etat.attaques_a_choisir {
            if a_choisir.position_au_camp >= collection.camp().len() {
                return Err(ErreurRestaurationPartie::AttaqueAChoisirSansMonstre(
                    a_choisir.position_au_camp,
                ));
            }
        }
        Ok(Self {
            heros: etat.heros,
            collection,
            inventaire: etat.inventaire,
            argent: etat.argent,
            marchand_au_camp: etat.marchand_au_camp,
            capsules: etat.capsules,
            codex: etat.codex,
            zones_explorees: etat.zones_explorees,
            indices_trouves: etat.indices_trouves,
            apotres_vaincus: etat.apotres_vaincus,
            conseils_vus: etat.conseils_vus,
            expeditions_automatiques: etat.expeditions_automatiques,
            attaques_a_choisir: etat.attaques_a_choisir,
        })
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
    /// vaincu (section 9.4). Tant que le marchand n'est pas au camp, elle
    /// peut le rencontrer (section 9.2). Ses coffres ne contiennent que des
    /// capsules que le joueur n'a pas encore (section 13.10).
    pub fn partir_en_expedition(
        &self,
        zone: &Zone,
        piste: &Piste,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<Expedition, ErreurExpedition> {
        if !piste.zone_ouverte(zone.id(), self.indices_trouves) {
            return Err(ErreurExpedition::ZoneFermee);
        }
        let possedees = self
            .capsules
            .iter()
            .map(|capsule| capsule.id().clone())
            .collect();
        let mut expedition =
            Expedition::partir(zone.clone(), aleatoire).avec_capsules_possedees(possedees);
        if !self.marchand_au_camp {
            expedition = expedition.avec_marchand_possible();
        }
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

    pub fn expeditions_automatiques(&self) -> &[ExpeditionAutomatique] {
        &self.expeditions_automatiques
    }

    pub fn attaques_a_choisir(&self) -> &[AttaqueAChoisir] {
        &self.attaques_a_choisir
    }

    /// La prochaine attaque à choisir, avec le monstre qui veut l'apprendre.
    pub fn attaque_a_choisir(&self) -> Option<(&Monstre, &Attaque)> {
        let a_choisir = self.attaques_a_choisir.first()?;
        let monstre = self.collection.camp().get(a_choisir.position_au_camp)?;
        Some((monstre, &a_choisir.attaque))
    }

    /// Le joueur a fait oublier une attaque au monstre, ou renoncé à la
    /// nouvelle : on passe à la suivante.
    pub(crate) fn attaque_choisie(&mut self) {
        if !self.attaques_a_choisir.is_empty() {
            self.attaques_a_choisir.remove(0);
        }
    }

    pub fn expedition_automatique(&self, zone: &ZoneId) -> Option<&ExpeditionAutomatique> {
        self.expeditions_automatiques
            .iter()
            .find(|expedition| expedition.zone().id() == zone)
    }

    /// Durée et chances de réussite, montrées avant le départ (section
    /// 12.4). Refuse ce que refuserait `envoyer_en_expedition_automatique`.
    pub fn estimer_expedition_automatique(
        &self,
        zone: &Zone,
        emplacements: &[Emplacement],
    ) -> Result<Estimation, ErreurExpeditionAutomatique> {
        self.verifier_depart(zone, emplacements)?;
        let monstres: Vec<Monstre> = emplacements
            .iter()
            .filter_map(|emplacement| self.collection.monstre(*emplacement).cloned())
            .collect();
        let parametres = zone.expedition_automatique();
        Ok(Estimation {
            duree: parametres.duree(monstres.len()),
            chances: parametres.chances_de_reussite(&monstres),
        })
    }

    /// Les monstres quittent l'équipe ou le camp et gardent leur place au
    /// camp, où ils reviendront (section 12.1). Rend le monstre du camp qui
    /// rejoint l'équipe quand elle part tout entière.
    pub fn envoyer_en_expedition_automatique(
        &mut self,
        zone: &Zone,
        emplacements: &[Emplacement],
        maintenant: Horodatage,
    ) -> Result<Option<Monstre>, ErreurExpeditionAutomatique> {
        self.verifier_depart(zone, emplacements)?;
        let depart = self
            .collection
            .retirer_pour_expedition(emplacements)
            .map_err(ErreurExpeditionAutomatique::Collection)?;
        self.expeditions_automatiques
            .push(ExpeditionAutomatique::partir(
                zone.clone(),
                depart.partis,
                maintenant,
            ));
        Ok(depart.rejoint_l_equipe)
    }

    /// Avant le terme, les monstres reviennent au camp sans dégâts ni
    /// récompense ; une expédition rentrée s'accueille (section 12.7).
    pub fn rappeler_expedition_automatique(
        &mut self,
        zone: &ZoneId,
        maintenant: Horodatage,
    ) -> Result<(), ErreurExpeditionAutomatique> {
        let position = self.position_de_l_expedition(zone)?;
        if self.expeditions_automatiques[position].est_rentree(maintenant) {
            return Err(ErreurExpeditionAutomatique::DejaRentree);
        }
        let monstres = self.expeditions_automatiques.remove(position).rappeler();
        self.collection.accueillir_retour(monstres);
        Ok(())
    }

    /// Tire le résultat, ajoute les potions et l'argent, et ramène les
    /// monstres au camp (sections 12.3, 12.6 et 12.7). Les attaques en
    /// attente dans les rapports d'XP s'ajoutent aux attaques à choisir.
    pub fn accueillir_expedition_automatique(
        &mut self,
        zone: &ZoneId,
        maintenant: Horodatage,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<BilanExpeditionAutomatique, ErreurExpeditionAutomatique> {
        let position = self.position_de_l_expedition(zone)?;
        if !self.expeditions_automatiques[position].est_rentree(maintenant) {
            return Err(ErreurExpeditionAutomatique::PasEncoreRentree);
        }
        let expedition = self.expeditions_automatiques.remove(position);
        // Les monstres reviennent à la suite, à la fin du camp.
        let (bilan, monstres) = expedition.resoudre(self.collection.camp().len(), aleatoire);
        self.inventaire.ajouter_potions(bilan.potions);
        self.argent = self.argent.saturating_add(bilan.argent);
        self.collection.accueillir_retour(monstres);
        for rentre in &bilan.monstres {
            let attaques = rentre
                .rapport
                .iter()
                .flat_map(|rapport| &rapport.attaques_en_attente);
            for attaque in attaques {
                self.attaques_a_choisir.push(AttaqueAChoisir {
                    position_au_camp: rentre.position_au_camp,
                    attaque: attaque.clone(),
                });
            }
        }
        Ok(bilan)
    }

    fn verifier_depart(
        &self,
        zone: &Zone,
        emplacements: &[Emplacement],
    ) -> Result<(), ErreurExpeditionAutomatique> {
        if !self.est_exploree(zone.id()) {
            return Err(ErreurExpeditionAutomatique::ZoneNonExploree);
        }
        if self.expedition_automatique(zone.id()).is_some() {
            return Err(ErreurExpeditionAutomatique::DejaEnCours);
        }
        if !(MONSTRES_MIN..=MONSTRES_MAX).contains(&emplacements.len()) {
            return Err(ErreurExpeditionAutomatique::NombreDeMonstres(
                emplacements.len(),
            ));
        }
        self.collection
            .verifier_depart(emplacements)
            .map_err(ErreurExpeditionAutomatique::Collection)
    }

    fn position_de_l_expedition(
        &self,
        zone: &ZoneId,
    ) -> Result<usize, ErreurExpeditionAutomatique> {
        self.expeditions_automatiques
            .iter()
            .position(|expedition| expedition.zone().id() == zone)
            .ok_or(ErreurExpeditionAutomatique::Inexistante)
    }

    /// Tous les monstres sont soignés (section 13.1) ; l'argent et les
    /// capsules trouvés sont acquis, sauf après une défaite (section 9.3).
    /// Une expédition menée
    /// jusqu'au bout sans défaite rend la zone explorée et, si l'indice
    /// suivant de la piste se trouve dans cette zone, le révèle et le rend
    /// (section 9.4).
    pub fn rentrer_au_camp<'p>(
        &mut self,
        expedition: &Expedition,
        piste: &'p Piste,
    ) -> Option<&'p Indice> {
        self.collection.soigner_tout();
        self.argent = self.argent.saturating_add(expedition.argent_rapporte());
        for capsule in expedition.capsules_rapportees() {
            self.obtenir_capsule(capsule.clone());
        }
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
    use crate::attaque::Categorie;
    use crate::codex::EtatCodex;
    use crate::combat::Action;
    use crate::donnees_de_test::{attaque, espece, zone};
    use crate::espece::EspeceId;
    use crate::expedition::EvenementExpedition;
    use crate::heros::Classe;
    use crate::nom::Nom;
    use crate::types::Type;
    use std::time::Duration;

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
    fn l_argent_des_coffres_n_est_acquis_qu_au_retour() {
        let (zones, piste) = carte();
        let mut partie = partie();
        // Quatre coffres de 10 pièces d'argent.
        let expedition = expedition_complete(&partie, &zones[0], &piste);
        assert_eq!(partie.argent(), 0);
        partie.rentrer_au_camp(&expedition, &piste);
        assert_eq!(partie.argent(), 40);

        let mut perdue = expedition_complete(&partie, &zones[0], &piste);
        perdue.noter_defaite();
        partie.rentrer_au_camp(&perdue, &piste);
        assert_eq!(partie.argent(), 40);
    }

    #[test]
    fn le_marchand_ne_se_rencontre_que_s_il_n_est_pas_au_camp() {
        let (zones, piste) = carte();
        let mut partie = partie();
        let mut expedition = partie
            .partir_en_expedition(&zones[0], &piste, &mut AleatoireScripte::new([4]))
            .unwrap();
        assert_eq!(
            expedition.etape_suivante(&mut AleatoireScripte::new([71])),
            Some(EvenementExpedition::Marchand)
        );
        partie.accueillir_marchand();
        let mut expedition = partie
            .partir_en_expedition(&zones[0], &piste, &mut AleatoireScripte::new([4]))
            .unwrap();
        // Le monstre du groupe tire son shiny et ses valeurs internes.
        assert!(matches!(
            expedition.etape_suivante(&mut AleatoireScripte::new([71, 2, 0, 0, 0, 0, 0, 0])),
            Some(EvenementExpedition::Combat(_))
        ));
    }

    /// Le marchand au camp et 50 pièces d'argent.
    fn partie_chez_le_marchand() -> Partie {
        let mut partie = partie();
        partie.collection_mut().accueillir_capture(creer("loup", 3));
        partie.accueillir_marchand();
        partie.argent = 50;
        partie
    }

    #[test]
    fn rien_ne_s_echange_sans_marchand() {
        let mut partie = partie();
        partie.argent = 50;
        partie.collection_mut().accueillir_capture(creer("loup", 3));
        assert_eq!(
            partie.acheter_potions(1),
            Err(ErreurMarchand::AucunMarchand)
        );
        assert_eq!(
            partie.vendre(Emplacement::Equipe(1)),
            Err(ErreurMarchand::AucunMarchand)
        );
        assert_eq!(partie.collection().equipe().len(), 2);
    }

    #[test]
    fn acheter_des_potions_dans_la_limite_de_son_argent() {
        let mut partie = partie_chez_le_marchand();
        assert_eq!(
            partie.acheter_potions(0),
            Err(ErreurMarchand::QuantiteNulle)
        );
        assert_eq!(
            partie.acheter_potions(3),
            Err(ErreurMarchand::ArgentInsuffisant {
                prix: 60,
                argent: 50
            })
        );
        assert_eq!(partie.inventaire().potions(), 0);
        assert_eq!(partie.acheter_potions(2), Ok(40));
        assert_eq!(partie.inventaire().potions(), 2);
        assert_eq!(partie.argent(), 10);
    }

    #[test]
    fn vendre_un_monstre_comme_on_le_relache() {
        let mut partie = partie_chez_le_marchand();
        // Un Loup de niveau 3, d'espèce commune : 15 pièces d'argent.
        let (estime, prix) = partie.estimer_vente(Emplacement::Equipe(1)).unwrap();
        assert_eq!(prix, 15);
        assert_eq!(partie.collection().equipe().len(), 2);
        assert_eq!(partie.vendre(Emplacement::Equipe(1)), Ok((estime, 15)));
        assert_eq!(partie.argent(), 65);
        assert_eq!(partie.collection().equipe().len(), 1);
        let dernier = Err(ErreurMarchand::Collection(
            ErreurCollection::DernierMonstreDeLEquipe,
        ));
        assert_eq!(partie.estimer_vente(Emplacement::Equipe(0)), dernier);
        assert_eq!(partie.vendre(Emplacement::Equipe(0)), dernier);
        assert_eq!(partie.argent(), 65);
    }

    fn capsule(id: &str, type_attaque: Type) -> Capsule {
        Capsule::new(attaque(id, type_attaque, Categorie::Physique, 40, 100), 150)
    }

    #[test]
    fn acheter_une_capsule_une_seule_fois() {
        let mut partie = partie_chez_le_marchand();
        let coup = capsule("grand-coup", Type::Normal);
        assert_eq!(
            partie.acheter_capsule(&coup),
            Err(ErreurMarchand::ArgentInsuffisant {
                prix: 150,
                argent: 50
            })
        );
        partie.argent = 200;
        assert_eq!(partie.acheter_capsule(&coup), Ok(150));
        assert_eq!(partie.argent(), 50);
        assert_eq!(
            partie.acheter_capsule(&coup),
            Err(ErreurMarchand::CapsuleDejaPossedee)
        );
        assert_eq!(partie.capsules(), &[coup]);
    }

    #[test]
    fn apprendre_au_camp_une_attaque_de_capsule() {
        let mut partie = partie_chez_le_marchand();
        partie.argent = 1_000;
        let loup = Emplacement::Equipe(1);
        // Les espèces de test sont de type Normal.
        partie.acheter_capsule(&capsule("feu", Type::Feu)).unwrap();
        assert!(partie.attaques_a_apprendre(loup).unwrap().is_empty());
        assert_eq!(
            partie.apprendre_attaque(loup, &AttaqueId::new("feu"), None),
            Err(ErreurApprentissage::AttaqueInaccessible)
        );
        for id in ["a", "b", "c", "d"] {
            partie.acheter_capsule(&capsule(id, Type::Normal)).unwrap();
        }
        assert_eq!(partie.attaques_a_apprendre(loup).unwrap().len(), 4);
        assert_eq!(
            partie.apprendre_attaque(loup, &AttaqueId::new("a"), Some(0)),
            Err(ErreurApprentissage::OubliInutile)
        );
        for id in ["a", "b", "c"] {
            assert_eq!(
                partie.apprendre_attaque(loup, &AttaqueId::new(id), None),
                Ok(None)
            );
        }
        // Il connaît coup, a, b et c : il doit en oublier une.
        assert_eq!(
            partie.apprendre_attaque(loup, &AttaqueId::new("d"), None),
            Err(ErreurApprentissage::OubliManquant)
        );
        assert_eq!(
            partie.apprendre_attaque(loup, &AttaqueId::new("d"), Some(4)),
            Err(ErreurApprentissage::Attaque(
                ErreurAttaquesMonstre::PositionInexistante(4)
            ))
        );
        let oubliee = partie
            .apprendre_attaque(loup, &AttaqueId::new("d"), Some(0))
            .unwrap()
            .unwrap();
        assert_eq!(oubliee.id().as_str(), "coup");
        // Coup, appris par son espèce au niveau 1, se réapprend.
        let a_apprendre = partie.attaques_a_apprendre(loup).unwrap();
        assert_eq!(a_apprendre.len(), 1);
        assert_eq!(a_apprendre[0].id().as_str(), "coup");
        assert_eq!(
            partie.attaques_a_apprendre(Emplacement::Camp(0)),
            Err(ErreurCollection::EmplacementInexistant(Emplacement::Camp(
                0
            )))
        );
    }

    #[test]
    fn les_capsules_des_coffres_ne_sont_acquises_qu_au_retour() {
        let (zones, piste) = carte();
        let mut zones = zones;
        let niveau = Niveau::new(5).unwrap();
        zones[0] = Zone::new(
            ZoneId::new("bois"),
            "bois",
            (niveau, niveau),
            1,
            vec![espece("loup")],
            vec![],
            crate::donnees_de_test::parametres_expedition_automatique(),
            crate::expedition::ParametresCoffre::new(
                10..=10,
                vec![capsule("a", Type::Normal), capsule("b", Type::Normal)],
            )
            .unwrap(),
        )
        .unwrap();
        let mut partie = partie();
        partie.obtenir_capsule(capsule("a", Type::Normal));
        let mut expedition = partie
            .partir_en_expedition(&zones[0], &piste, &mut AleatoireScripte::new([4]))
            .unwrap();
        // Un coffre, où seule la capsule b est nouvelle.
        expedition.etape_suivante(&mut AleatoireScripte::new([81, 1, 20]));
        assert_eq!(partie.capsules().len(), 1);
        partie.rentrer_au_camp(&expedition, &piste);
        let ids: Vec<&str> = partie
            .capsules()
            .iter()
            .map(|capsule| capsule.id().as_str())
            .collect();
        assert_eq!(ids, vec!["a", "b"]);
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
        // Rentrer avant la fin garde les gains (section 9.2).
        assert_eq!(partie.argent(), 10);
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

    /// Le monstre de départ et un loup de niveau 5, de 20 PV chacun, avec
    /// le bois déjà exploré.
    fn partie_prete_pour_le_bois() -> Partie {
        let mut partie = partie();
        partie.collection_mut().accueillir_capture(creer("loup", 5));
        partie.zones_explorees.push(ZoneId::new("bois"));
        partie
    }

    fn a(secondes: u64) -> Horodatage {
        Horodatage::depuis_secondes(secondes)
    }

    #[test]
    fn une_expedition_automatique_part_dans_une_zone_exploree() {
        let bois = zone("bois");
        let mut partie = partie();
        partie.collection_mut().accueillir_capture(creer("loup", 5));
        let loup = [Emplacement::Equipe(1)];
        assert_eq!(
            partie.envoyer_en_expedition_automatique(&bois, &loup, a(0)),
            Err(ErreurExpeditionAutomatique::ZoneNonExploree)
        );
        partie.zones_explorees.push(ZoneId::new("bois"));
        assert_eq!(
            partie.envoyer_en_expedition_automatique(&bois, &[], a(0)),
            Err(ErreurExpeditionAutomatique::NombreDeMonstres(0))
        );
        assert_eq!(
            partie.envoyer_en_expedition_automatique(
                &bois,
                &[Emplacement::Equipe(0), Emplacement::Equipe(1)],
                a(0)
            ),
            Err(ErreurExpeditionAutomatique::Collection(
                ErreurCollection::DernierMonstreDeLEquipe
            ))
        );

        // 20 PV contre 10 à 40 dégâts : 10 totaux sur 31 laissent le loup
        // debout.
        let estimation = partie.estimer_expedition_automatique(&bois, &loup).unwrap();
        assert_eq!(estimation.duree, Duration::from_secs(3_600));
        assert_eq!(estimation.chances.pourcentage(), 32);

        partie
            .envoyer_en_expedition_automatique(&bois, &loup, a(0))
            .unwrap();
        assert_eq!(partie.collection().equipe().len(), 1);
        assert_eq!(partie.collection().places_reservees(), 1);
        let expedition = partie.expedition_automatique(&ZoneId::new("bois")).unwrap();
        assert_eq!(expedition.retour(), a(3_600));
        assert_eq!(
            partie.estimer_expedition_automatique(&bois, &[Emplacement::Equipe(0)]),
            Err(ErreurExpeditionAutomatique::DejaEnCours)
        );
    }

    #[test]
    fn accueillir_une_expedition_rentree() {
        let mut partie = partie_prete_pour_le_bois();
        partie
            .envoyer_en_expedition_automatique(&zone("bois"), &[Emplacement::Equipe(1)], a(0))
            .unwrap();
        let bois = ZoneId::new("bois");
        assert_eq!(
            partie.accueillir_expedition_automatique(
                &bois,
                a(3_599),
                &mut AleatoireScripte::new([])
            ),
            Err(ErreurExpeditionAutomatique::PasEncoreRentree)
        );
        // 15 dégâts sur 20 PV : réussite, 100 XP, une potion et 20 pièces
        // d'argent.
        let bilan = partie
            .accueillir_expedition_automatique(
                &bois,
                a(3_600),
                &mut AleatoireScripte::new([15, 100, 1]),
            )
            .unwrap();
        assert!(bilan.reussite);
        assert_eq!(bilan.monstres[0].position_au_camp, 0);
        assert_eq!(partie.collection().camp()[0].experience(), 125 + 100);
        assert_eq!(partie.collection().places_reservees(), 0);
        assert_eq!(partie.inventaire().potions(), 1);
        assert_eq!(partie.argent(), 20);
        assert!(partie.expeditions_automatiques().is_empty());
        assert_eq!(
            partie.accueillir_expedition_automatique(
                &bois,
                a(3_600),
                &mut AleatoireScripte::new([])
            ),
            Err(ErreurExpeditionAutomatique::Inexistante)
        );
    }

    #[test]
    fn rappeler_une_expedition_avant_son_terme() {
        let mut partie = partie_prete_pour_le_bois();
        partie
            .envoyer_en_expedition_automatique(&zone("bois"), &[Emplacement::Equipe(1)], a(0))
            .unwrap();
        let bois = ZoneId::new("bois");
        assert_eq!(
            partie.rappeler_expedition_automatique(&bois, a(3_600)),
            Err(ErreurExpeditionAutomatique::DejaRentree)
        );
        partie
            .rappeler_expedition_automatique(&bois, a(10))
            .unwrap();
        assert!(partie.expeditions_automatiques().is_empty());
        assert_eq!(partie.collection().camp().len(), 1);
        assert_eq!(partie.collection().camp()[0].experience(), 125);
        assert_eq!(partie.collection().places_reservees(), 0);
        assert_eq!(partie.inventaire().potions(), 0);
        assert_eq!(partie.argent(), 0);
    }

    fn etat(partie: &Partie) -> EtatPartie {
        EtatPartie {
            heros: partie.heros().clone(),
            collection: partie.collection().clone(),
            inventaire: partie.inventaire().clone(),
            argent: partie.argent(),
            marchand_au_camp: partie.marchand_au_camp(),
            capsules: partie.capsules().to_vec(),
            codex: partie.codex().clone(),
            zones_explorees: partie.zones_explorees().to_vec(),
            indices_trouves: partie.indices_trouves(),
            apotres_vaincus: partie.apotres_vaincus().to_vec(),
            conseils_vus: partie.conseils_vus().to_vec(),
            expeditions_automatiques: partie.expeditions_automatiques().to_vec(),
            attaques_a_choisir: partie.attaques_a_choisir().to_vec(),
        }
    }

    #[test]
    fn restaurer_reverifie_les_expeditions_automatiques() {
        let mut partie = partie_prete_pour_le_bois();
        partie
            .envoyer_en_expedition_automatique(&zone("bois"), &[Emplacement::Equipe(1)], a(0))
            .unwrap();
        // Rechargée telle quelle, la partie garde l'expédition et la place
        // réservée au camp.
        assert_eq!(Partie::restaurer(etat(&partie)), Ok(partie.clone()));

        let mut sans_exploration = etat(&partie);
        sans_exploration.zones_explorees.clear();
        assert_eq!(
            Partie::restaurer(sans_exploration),
            Err(ErreurRestaurationPartie::ZoneNonExploree(ZoneId::new(
                "bois"
            )))
        );

        let mut en_double = etat(&partie);
        let copie = en_double.expeditions_automatiques[0].clone();
        en_double.expeditions_automatiques.push(copie);
        assert_eq!(
            Partie::restaurer(en_double),
            Err(ErreurRestaurationPartie::ExpeditionEnDouble(ZoneId::new(
                "bois"
            )))
        );

        let mut sans_monstre = etat(&partie);
        sans_monstre.attaques_a_choisir.push(AttaqueAChoisir {
            position_au_camp: 0,
            attaque: creer("loup", 5).attaques().liste()[0].clone(),
        });
        assert_eq!(
            Partie::restaurer(sans_monstre),
            Err(ErreurRestaurationPartie::AttaqueAChoisirSansMonstre(0))
        );

        // Le monstre parti a besoin de sa place dans un camp déjà plein.
        let mut camp_plein = etat(&partie);
        let equipe = camp_plein.collection.equipe().to_vec();
        let camp = (0..Collection::TAILLE_CAMP)
            .map(|_| creer("loup", 5))
            .collect();
        camp_plein.collection = Collection::restaurer(equipe, camp).unwrap();
        assert_eq!(
            Partie::restaurer(camp_plein),
            Err(ErreurRestaurationPartie::Collection(
                ErreurCollection::CampPlein
            ))
        );
    }
}
