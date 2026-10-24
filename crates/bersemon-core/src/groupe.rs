//! Expédition en groupe dans la Tour sans fond (section 15.5) : deux
//! joueurs, chacun avec son déroulement, sa partie et son sac. L'expédition
//! de l'hôte tire les étapes, celle de l'invité les suit ; chacun tire son
//! coffre et règle les suites de ses combats dans son déroulement, mais le
//! combat commun se joue ici.

use crate::aleatoire::Aleatoire;
use crate::combat::{Combat, Issue, Membre, Phase};
use crate::contenu::ContenuV1;
use crate::deroulement::{Attente, Choix, Contexte, Deroulement, ErreurChoix, Evenement};
use crate::expedition::{ErreurExpedition, EvenementExpedition};
use crate::horodatage::Horodatage;
use crate::monstre::Monstre;
use crate::partie::Partie;
use crate::sauvegarde::SauvegardeRepository;

/// Celui qui attend l'autre depuis 5 minutes peut ramener le groupe au camp.
pub const DELAI_D_ABSENCE: u64 = 5 * 60;

/// Comme `Contexte`, avec la sauvegarde de chacun : celle de l'hôte, puis
/// celle de l'invité.
pub struct ContexteDeGroupe<'a> {
    pub contenu: &'a ContenuV1,
    pub aleatoire: &'a mut dyn Aleatoire,
    pub depots: [&'a dyn SauvegardeRepository; 2],
    pub maintenant: Horodatage,
}

impl ContexteDeGroupe<'_> {
    fn pour(&mut self, membre: Membre) -> Contexte<'_> {
        Contexte {
            contenu: self.contenu,
            aleatoire: &mut *self.aleatoire,
            depot: self.depots[position(membre)],
            maintenant: self.maintenant,
        }
    }
}

/// Ce que chaque joueur voit se passer : l'hôte, puis l'invité.
pub type Recits = [Vec<Evenement>; 2];

fn position(membre: Membre) -> usize {
    match membre {
        Membre::Hote => 0,
        Membre::Invite => 1,
    }
}

const MEMBRES: [Membre; 2] = [Membre::Hote, Membre::Invite];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpeditionEnGroupe {
    combat: Option<Combat>,
    /// Après une victoire, ceux qui ne capturent plus.
    captures_finies: [bool; 2],
    /// L'un a choisi de rentrer, ou l'autre ne jouait plus : chacun rentre
    /// dès qu'il est entre deux étapes.
    retour: bool,
    /// L'heure de la dernière action acceptée, d'où court le délai
    /// d'absence.
    derniere_action: Horodatage,
}

impl ExpeditionEnGroupe {
    /// Les deux joueurs, au camp et sans question en attente, descendent
    /// ensemble ; la première étape suit aussitôt. Chacun doit avoir ouvert
    /// la Tour et avoir au moins un monstre dans l'équipe.
    pub fn partir(
        deroulements: [&mut Deroulement; 2],
        contexte: &mut ContexteDeGroupe<'_>,
    ) -> Result<(ExpeditionEnGroupe, Recits), ErreurChoix> {
        let [hote, invite] = deroulements;
        let tour = contexte.contenu.tour();
        let zones = contexte.contenu.zones();
        let mut expeditions = Vec::new();
        for deroulement in [&*hote, &*invite] {
            if deroulement.attente() != Attente::Camp {
                return Err(ErreurChoix::HorsDePropos);
            }
            let partie = deroulement.partie().ok_or(ErreurChoix::HorsDePropos)?;
            let expedition = partie
                .preparer_la_descente(tour, zones, true)
                .map_err(|erreur| match erreur {
                    ErreurExpedition::ZoneFermee => ErreurChoix::ZoneFermee,
                    ErreurExpedition::EquipeVide => ErreurChoix::EquipeVide,
                })?;
            expeditions.push(expedition);
        }
        let mut recits: Recits = Default::default();
        let mut expeditions = expeditions.into_iter();
        for (deroulement, recit) in [&mut *hote, &mut *invite].into_iter().zip(&mut recits) {
            let expedition = expeditions.next().expect("une expédition par joueur");
            deroulement.partir_en_groupe(expedition, recit)?;
        }
        let mut groupe = ExpeditionEnGroupe {
            combat: None,
            captures_finies: [false; 2],
            retour: false,
            derniere_action: contexte.maintenant,
        };
        groupe.etape_suivante([hote, invite], contexte, &mut recits);
        Ok((groupe, recits))
    }

    /// Le combat commun en cours.
    pub fn combat(&self) -> Option<&Combat> {
        self.combat.as_ref()
    }

    /// Chacun est rentré au camp.
    pub fn est_terminee(&self, deroulements: [&Deroulement; 2]) -> bool {
        deroulements
            .iter()
            .all(|deroulement| deroulement.expedition().is_none())
    }

    /// Ceux dont le groupe attend une réponse : en combat, le propriétaire
    /// du monstre actif, celui qui remplace un monstre KO ou ceux qui
    /// capturent encore ; sinon, ceux qui n'ont pas encore choisi de
    /// continuer.
    pub fn attendus(&self, deroulements: [&Deroulement; 2]) -> Vec<Membre> {
        if let Some(combat) = &self.combat {
            return match combat.phase() {
                Phase::ChoixAction => vec![combat.membre_actif()],
                Phase::Remplacement => combat.membre_qui_remplace().into_iter().collect(),
                Phase::Termine(_) => MEMBRES
                    .into_iter()
                    .filter(|membre| !self.captures_finies[position(*membre)])
                    .collect(),
            };
        }
        MEMBRES
            .into_iter()
            .zip(deroulements)
            .filter(|(_, deroulement)| {
                deroulement.expedition().is_some() && !deroulement.veut_continuer()
            })
            .map(|(membre, _)| membre)
            .collect()
    }

    /// Depuis quand le groupe attend : la dernière action acceptée.
    pub fn derniere_action(&self) -> Horodatage {
        self.derniere_action
    }

    /// Le choix de ce joueur : une action du combat commun, ou une réponse
    /// à son propre déroulement. Continuer attend l'accord des deux ; si
    /// l'un rentre, le groupe rentre (section 15.5).
    pub fn repondre(
        &mut self,
        membre: Membre,
        choix: Choix,
        deroulements: [&mut Deroulement; 2],
        contexte: &mut ContexteDeGroupe<'_>,
    ) -> Result<Recits, ErreurChoix> {
        let mut recits: Recits = Default::default();
        let [hote, invite] = deroulements;
        if self.combat.is_some() {
            self.combattre(membre, &choix, [&*hote, &*invite], contexte, &mut recits)?;
            if self.combat_fini() {
                self.conclure_le_combat([&mut *hote, &mut *invite], contexte, &mut recits);
            }
        } else {
            let deroulement = match membre {
                Membre::Hote => &mut *hote,
                Membre::Invite => &mut *invite,
            };
            let evenements = deroulement.repondre(choix, &mut contexte.pour(membre))?;
            if deroulement.expedition().is_none() {
                self.retour = true;
            }
            recits[position(membre)].extend(evenements);
        }
        self.derniere_action = contexte.maintenant;
        self.avancer([hote, invite], contexte, &mut recits);
        Ok(recits)
    }

    /// Celui qui attend l'autre depuis 5 minutes ramène le groupe au camp,
    /// chacun avec ses gains ; en combat, comme une fuite réussie (section
    /// 15.5).
    pub fn ramener_au_camp(
        &mut self,
        membre: Membre,
        deroulements: [&mut Deroulement; 2],
        contexte: &mut ContexteDeGroupe<'_>,
    ) -> Result<Recits, ErreurChoix> {
        let [hote, invite] = deroulements;
        let attendus = self.attendus([&*hote, &*invite]);
        if attendus.contains(&membre) || !attendus.contains(&membre.autre()) {
            return Err(ErreurChoix::HorsDePropos);
        }
        if contexte.maintenant.secondes() < self.derniere_action.secondes() + DELAI_D_ABSENCE {
            return Err(ErreurChoix::AttenteTropCourte);
        }
        let mut recits: Recits = Default::default();
        if let Some(combat) = &mut self.combat {
            if !matches!(combat.phase(), Phase::Termine(_)) {
                let tour = combat
                    .interrompre_par_une_fuite()
                    .map_err(ErreurChoix::Combat)?;
                raconter_le_tour(combat, tour, &mut recits);
            }
            self.conclure_le_combat([&mut *hote, &mut *invite], contexte, &mut recits);
        }
        self.retour = true;
        self.derniere_action = contexte.maintenant;
        self.avancer([hote, invite], contexte, &mut recits);
        Ok(recits)
    }

    /// Seul le propriétaire du monstre actif agit, ou celui qui remplace un
    /// monstre KO ; après une victoire, chacun capture pour soi.
    fn combattre(
        &mut self,
        membre: Membre,
        choix: &Choix,
        deroulements: [&Deroulement; 2],
        contexte: &mut ContexteDeGroupe<'_>,
        recits: &mut Recits,
    ) -> Result<(), ErreurChoix> {
        let combat = self.combat.as_mut().expect("combat en cours");
        let au_tour_de = |voulu: Option<Membre>| {
            if voulu == Some(membre) {
                Ok(())
            } else {
                Err(ErreurChoix::AuTourDeLAutre)
            }
        };
        let tour = match (combat.phase(), choix) {
            (Phase::ChoixAction, Choix::DonnerOrdre) => {
                au_tour_de(Some(combat.membre_actif()))?;
                let (ordre, cible) = combat.donner_ordre().map_err(ErreurChoix::Combat)?;
                let heros = deroulements[position(membre)]
                    .partie()
                    .map(|partie| partie.heros().nom().as_str().to_string())
                    .unwrap_or_default();
                let evenement = Evenement::OrdreDonne {
                    heros,
                    ordre,
                    monstre: combat.monstre(cible).clone(),
                };
                for recit in recits.iter_mut() {
                    recit.push(evenement.clone());
                }
                return Ok(());
            }
            (Phase::ChoixAction, Choix::Combat(action)) => {
                au_tour_de(Some(combat.membre_actif()))?;
                combat.jouer_tour(*action, &mut *contexte.aleatoire)
            }
            (Phase::Remplacement, Choix::Remplacer(index)) => {
                au_tour_de(combat.membre_qui_remplace())?;
                combat.remplacer(*index)
            }
            (Phase::Termine(Issue::Victoire), Choix::Capturer(index))
                if !self.captures_finies[position(membre)] =>
            {
                combat.capturer_ko_pour(membre, *index)
            }
            (Phase::Termine(_), Choix::NePlusCapturer) => {
                self.captures_finies[position(membre)] = true;
                Ok(Vec::new())
            }
            (Phase::ChoixAction | Phase::Remplacement, _) => {
                return Err(ErreurChoix::AuTourDeLAutre);
            }
            _ => return Err(ErreurChoix::HorsDePropos),
        }
        .map_err(ErreurChoix::Combat)?;
        raconter_le_tour(combat, tour, recits);
        Ok(())
    }

    /// Fini quand il ne reste rien à capturer, ou que chacun a fini de
    /// capturer.
    fn combat_fini(&self) -> bool {
        self.combat.as_ref().is_some_and(|combat| {
            matches!(combat.phase(), Phase::Termine(_))
                && (combat.adversaires_capturables().is_empty()
                    || self.captures_finies.iter().all(|finies| *finies))
        })
    }

    /// Chacun lit sa part du bilan dans son déroulement : son XP, ses
    /// captures, ses ressources, ses soins, ou la perte de son sac.
    fn conclure_le_combat(
        &mut self,
        deroulements: [&mut Deroulement; 2],
        contexte: &mut ContexteDeGroupe<'_>,
        recits: &mut Recits,
    ) {
        let Some(combat) = self.combat.take() else {
            return;
        };
        self.captures_finies = [false; 2];
        let bilan = combat.terminer(&mut *contexte.aleatoire);
        let (hote, invite) = bilan.separer();
        let [deroulement_hote, deroulement_invite] = deroulements;
        recits[0]
            .extend(deroulement_hote.recevoir_le_bilan(hote, &mut contexte.pour(Membre::Hote)));
        if let Some(invite) = invite {
            recits[1].extend(
                deroulement_invite.recevoir_le_bilan(invite, &mut contexte.pour(Membre::Invite)),
            );
        }
    }

    /// Le groupe rentre si l'un est rentré, chacun dès qu'il est entre deux
    /// étapes ; sinon, l'étape suivante vient quand les deux veulent
    /// continuer.
    fn avancer(
        &mut self,
        deroulements: [&mut Deroulement; 2],
        contexte: &mut ContexteDeGroupe<'_>,
        recits: &mut Recits,
    ) {
        if self.combat.is_some() {
            return;
        }
        let [hote, invite] = deroulements;
        if hote.expedition().is_none() || invite.expedition().is_none() {
            self.retour = true;
        }
        if self.retour {
            for (membre, deroulement) in MEMBRES.into_iter().zip([&mut *hote, &mut *invite]) {
                if deroulement.expedition().is_some() {
                    if let Some(evenements) =
                        deroulement.rentrer_avec_le_groupe(&mut contexte.pour(membre))
                    {
                        recits[position(membre)].extend(evenements);
                    }
                }
            }
            return;
        }
        if hote.veut_continuer() && invite.veut_continuer() {
            self.etape_suivante([hote, invite], contexte, recits);
        }
    }

    /// L'expédition de l'hôte tire l'événement, celle de l'invité suit : un
    /// coffre pour chacun, un monstre rare pacifique pour chacun, ou un
    /// combat commun, contre des groupes plus grands (section 9.5).
    fn etape_suivante(
        &mut self,
        deroulements: [&mut Deroulement; 2],
        contexte: &mut ContexteDeGroupe<'_>,
        recits: &mut Recits,
    ) {
        let [hote, invite] = deroulements;
        let Some(evenement) = hote
            .expedition_mut()
            .and_then(|expedition| expedition.etape_suivante(&mut *contexte.aleatoire))
        else {
            return;
        };
        let Some(expedition_de_l_invite) = invite.expedition_mut() else {
            return;
        };
        expedition_de_l_invite.suivre_l_etape();
        let pour_l_invite = match &evenement {
            EvenementExpedition::Coffre { .. } => {
                expedition_de_l_invite.tirer_un_coffre(&mut *contexte.aleatoire)
            }
            autre => autre.clone(),
        };
        let adversaires = adversaires_de(&evenement);
        recits[0].extend(hote.recevoir_l_etape(evenement, &mut contexte.pour(Membre::Hote)));
        recits[1]
            .extend(invite.recevoir_l_etape(pour_l_invite, &mut contexte.pour(Membre::Invite)));
        let Some((adversaires, contre_un_gardien)) = adversaires else {
            return;
        };
        let (Some(partie_hote), Some(partie_invite)) = (hote.partie(), invite.partie()) else {
            return;
        };
        let combat = Partie::preparer_combat_en_groupe(
            partie_hote,
            partie_invite,
            adversaires,
            contre_un_gardien,
            contexte.contenu.zones(),
            contexte.contenu.piste(),
        )
        .expect("le groupe a au moins un monstre apte entre deux étapes");
        recits[0].extend(hote.engager_en_groupe(&combat));
        recits[1].extend(invite.engager_en_groupe(&combat));
        self.combat = Some(combat);
    }
}

/// Le combat commun et s'il oppose le gardien de la Tour ; `None` pour un
/// coffre ou un monstre rare pacifique.
fn adversaires_de(evenement: &EvenementExpedition) -> Option<(Vec<Monstre>, bool)> {
    match evenement {
        EvenementExpedition::Combat(groupe) => Some((groupe.clone(), false)),
        EvenementExpedition::MonstreRare(rare) if !rare.pacifique => {
            Some((vec![rare.monstre.clone()], false))
        }
        EvenementExpedition::Gardien(gardien) => Some((vec![gardien.clone()], true)),
        _ => None,
    }
}

/// Les deux joueurs voient le même tour du combat commun.
fn raconter_le_tour(combat: &Combat, tour: Vec<crate::combat::Evenement>, recits: &mut Recits) {
    if tour.is_empty() {
        return;
    }
    for recit in recits.iter_mut() {
        recit.push(Evenement::Combat {
            evenements: tour.clone(),
            combat: combat.clone(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::GenerateurAleatoire;
    use crate::catalogue::Catalogue;
    use crate::combat::Action;
    use crate::espece::EspeceId;
    use crate::heros::{Classe, Heros};
    use crate::niveau::Niveau;
    use crate::nom::Nom;
    use crate::partie::EtatPartie;
    use crate::sauvegarde::ErreurSauvegarde;
    use crate::stats::ValeursInternes;
    use crate::zone::ZoneId;
    use std::cell::RefCell;

    #[derive(Default)]
    struct DepotMemoire {
        sauvegardes: RefCell<Vec<Partie>>,
    }

    impl SauvegardeRepository for DepotMemoire {
        fn charger(&self, _: &dyn Catalogue) -> Result<Option<Partie>, ErreurSauvegarde> {
            Ok(self.sauvegardes.borrow().last().cloned())
        }

        fn enregistrer(&self, partie: &Partie) -> Result<(), ErreurSauvegarde> {
            self.sauvegardes.borrow_mut().push(partie.clone());
            Ok(())
        }

        fn supprimer(&self) -> Result<(), ErreurSauvegarde> {
            self.sauvegardes.borrow_mut().clear();
            Ok(())
        }
    }

    fn monstre(contenu: &ContenuV1, id: &str, niveau: u8) -> Monstre {
        Monstre::creer(
            contenu.espece(&EspeceId::new(id)).unwrap(),
            Niveau::new(niveau).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    /// Au camp, la Lisière menée au bout, avec 5 potions.
    fn au_camp(contenu: &ContenuV1, nom: &str, monstres: &[(&str, u8)]) -> Deroulement {
        let heros = Heros::new(Nom::new(nom).unwrap(), Classe::Mage);
        let (premier, niveau) = monstres[0];
        let mut partie = Partie::nouvelle(heros, monstre(contenu, premier, niveau));
        for (id, niveau) in &monstres[1..] {
            partie.prendre_monstre_rare(monstre(contenu, id, *niveau));
        }
        let partie = Partie::restaurer(EtatPartie {
            zones_explorees: vec![ZoneId::new("lisiere")],
            ..etat_de(&partie)
        })
        .unwrap();
        let mut deroulement = Deroulement::demarrer(Ok(Some(partie)));
        let depot = DepotMemoire::default();
        let mut aleatoire = GenerateurAleatoire::new(1);
        deroulement
            .repondre(
                Choix::ReprendreLaPartie,
                &mut Contexte {
                    contenu,
                    aleatoire: &mut aleatoire,
                    depot: &depot,
                    maintenant: Horodatage::depuis_secondes(0),
                },
            )
            .unwrap();
        deroulement
    }

    fn etat_de(partie: &Partie) -> EtatPartie {
        let mut sac = partie.sac().clone();
        sac.ajouter_potions(5);
        EtatPartie {
            heros: partie.heros().clone(),
            collection: partie.collection().clone(),
            sac,
            coffre_du_camp: partie.coffre_du_camp().clone(),
            marchand_au_camp: true,
            ames_donnees: Vec::new(),
            ames_portees: Vec::new(),
            ames_a_retrouver: Vec::new(),
            codex: partie.codex().clone(),
            zones_explorees: Vec::new(),
            indices_trouves: 1,
            apotres_vaincus: Vec::new(),
            humains_vaincus: Vec::new(),
            conseils_vus: Vec::new(),
            expeditions_automatiques: Vec::new(),
            attaques_a_choisir: Vec::new(),
            forge: Default::default(),
            peche: Default::default(),
            relations: Default::default(),
            machines: Default::default(),
            beherit: Default::default(),
            records_de_la_tour: Default::default(),
        }
    }

    struct Table {
        contenu: ContenuV1,
        depots: [DepotMemoire; 2],
        aleatoire: GenerateurAleatoire,
        hote: Deroulement,
        invite: Deroulement,
        maintenant: u64,
    }

    impl Table {
        fn new(hote: &[(&str, u8)], invite: &[(&str, u8)], graine: u64) -> Table {
            let contenu = ContenuV1::new();
            let hote = au_camp(&contenu, "Aldric", hote);
            let invite = au_camp(&contenu, "Ysolde", invite);
            Table {
                contenu,
                depots: Default::default(),
                aleatoire: GenerateurAleatoire::new(graine),
                hote,
                invite,
                maintenant: 0,
            }
        }

        fn partir(&mut self) -> (ExpeditionEnGroupe, Recits) {
            let mut contexte = ContexteDeGroupe {
                contenu: &self.contenu,
                aleatoire: &mut self.aleatoire,
                depots: [&self.depots[0], &self.depots[1]],
                maintenant: Horodatage::depuis_secondes(self.maintenant),
            };
            ExpeditionEnGroupe::partir([&mut self.hote, &mut self.invite], &mut contexte).unwrap()
        }

        fn repondre(
            &mut self,
            groupe: &mut ExpeditionEnGroupe,
            membre: Membre,
            choix: Choix,
        ) -> Result<Recits, ErreurChoix> {
            let mut contexte = ContexteDeGroupe {
                contenu: &self.contenu,
                aleatoire: &mut self.aleatoire,
                depots: [&self.depots[0], &self.depots[1]],
                maintenant: Horodatage::depuis_secondes(self.maintenant),
            };
            groupe.repondre(
                membre,
                choix,
                [&mut self.hote, &mut self.invite],
                &mut contexte,
            )
        }

        fn ramener(
            &mut self,
            groupe: &mut ExpeditionEnGroupe,
            membre: Membre,
        ) -> Result<Recits, ErreurChoix> {
            let mut contexte = ContexteDeGroupe {
                contenu: &self.contenu,
                aleatoire: &mut self.aleatoire,
                depots: [&self.depots[0], &self.depots[1]],
                maintenant: Horodatage::depuis_secondes(self.maintenant),
            };
            groupe.ramener_au_camp(membre, [&mut self.hote, &mut self.invite], &mut contexte)
        }

        fn deroulement(&self, membre: Membre) -> &Deroulement {
            match membre {
                Membre::Hote => &self.hote,
                Membre::Invite => &self.invite,
            }
        }

        fn attendus(&self, groupe: &ExpeditionEnGroupe) -> Vec<Membre> {
            groupe.attendus([&self.hote, &self.invite])
        }
    }

    /// Un joueur simple : la meilleure attaque, le premier monstre apte à
    /// lui, aucune capture, ni attaque nouvelle ni évolution ; il continue
    /// tant que l'étape est sous `jusqu_a`.
    fn coup(
        groupe: &ExpeditionEnGroupe,
        deroulement: &Deroulement,
        membre: Membre,
        jusqu_a: u32,
    ) -> Choix {
        if let Some(combat) = groupe.combat() {
            return match combat.phase() {
                Phase::ChoixAction => {
                    let attaques = combat
                        .monstre(combat.actif(crate::combat::Cote::Joueur))
                        .attaques()
                        .liste()
                        .len();
                    let meilleure = (0..attaques)
                        .max_by_key(|index| {
                            combat
                                .apercu_attaque(*index)
                                .map_or(0, |apercu| apercu.degats_max)
                        })
                        .unwrap_or(0);
                    Choix::Combat(Action::Attaquer(meilleure))
                }
                Phase::Remplacement => {
                    let index = (0..combat.equipe().len())
                        .find(|index| {
                            combat.proprietaire(*index) == membre
                                && !combat.equipe()[*index].est_ko()
                        })
                        .unwrap();
                    Choix::Remplacer(index)
                }
                Phase::Termine(_) => Choix::NePlusCapturer,
            };
        }
        let etape = deroulement
            .expedition()
            .map_or(0, |expedition| expedition.etapes_faites());
        match deroulement.attente() {
            Attente::EntreDeuxEtapes if etape >= jusqu_a => Choix::RentrerAuCamp,
            Attente::EntreDeuxEtapes => Choix::ContinuerLExpedition,
            Attente::MonstreRare(_) => Choix::LaisserLeMonstreRare,
            Attente::OubliAttaque { .. } => Choix::RenoncerALAttaque,
            Attente::Evolution { .. } => Choix::NePasEvoluer,
            Attente::PlaceManquante(_) => Choix::RelacherLaCapture,
            Attente::Confirmation(_) => Choix::Oui,
            autre => panic!("{autre:?}"),
        }
    }

    const FORTS: [(&str, u8); 3] = [("salamandre", 45), ("crapaud", 45), ("mandragore", 45)];

    /// Section 15.5 : une descente à deux, avec des groupes de 2 à 4
    /// monstres, le gardien de l'étape 10 suivi d'un repos, l'accord des
    /// deux pour continuer, puis le retour du groupe quand l'hôte rentre.
    #[test]
    fn une_descente_a_deux_jusqu_au_retour_de_l_hote() {
        let mut table = Table::new(&FORTS, &FORTS, 7);
        let (mut groupe, recits) = table.partir();
        for recit in &recits {
            assert!(matches!(
                &recit[..],
                [
                    Evenement::Depart(_),
                    Evenement::EntreeDansLaTour(_),
                    Evenement::Etape {
                        numero: 1,
                        total: None
                    },
                    ..
                ]
            ));
        }
        let mut vus: Recits = recits;
        let mut tailles = Vec::new();
        let mut gardien_vu = false;
        for _ in 0..3_000 {
            if groupe.est_terminee([&table.hote, &table.invite]) {
                break;
            }
            if let Some(combat) = groupe.combat() {
                tailles.push(combat.adversaires().len());
                gardien_vu |= combat.est_contre_un_gardien();
            }
            let attendus = table.attendus(&groupe);
            let membre = attendus[0];
            let choix = coup(&groupe, table.deroulement(membre), membre, 12);
            // Continuer attend l'accord de l'autre : pas d'étape avant.
            let seul_a_continuer = choix == Choix::ContinuerLExpedition
                && !table.deroulement(membre.autre()).veut_continuer();
            let recits = table.repondre(&mut groupe, membre, choix).unwrap();
            if seul_a_continuer {
                assert!(table.deroulement(membre).veut_continuer());
                assert!(
                    !recits
                        .iter()
                        .flatten()
                        .any(|e| matches!(e, Evenement::Etape { .. }))
                );
            }
            for (vu, recit) in vus.iter_mut().zip(recits) {
                vu.extend(recit);
            }
        }
        assert!(groupe.est_terminee([&table.hote, &table.invite]));
        assert!(gardien_vu, "pas de gardien");
        // Un monstre seul est un rare hostile ou le gardien ; les groupes
        // sauvages comptent de 2 à 4 monstres.
        assert!(
            tailles.iter().all(|taille| (1..=4).contains(taille)),
            "{tailles:?}"
        );
        assert!(tailles.contains(&4), "{tailles:?}");
        for (vu, depot) in vus.iter().zip(&table.depots) {
            assert!(
                vu.iter()
                    .any(|e| matches!(e, Evenement::ReposDansLaTour(_)))
            );
            assert!(vu.contains(&Evenement::RetourAuCamp));
            assert!(vu.contains(&Evenement::NouveauRecordDeLaTour(12)));
            let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
            assert_eq!(sauvegarde.records_de_la_tour().a_deux, 12);
            assert_eq!(sauvegarde.records_de_la_tour().seul, 0);
        }
        assert_eq!(table.hote.attente(), Attente::Camp);
        assert_eq!(table.invite.attente(), Attente::Camp);
    }

    /// Section 15.5 : celui qui attend l'autre depuis 5 minutes ramène le
    /// groupe au camp ; pas avant.
    #[test]
    fn celui_qui_attend_ramene_le_groupe_apres_5_minutes() {
        let mut table = Table::new(&FORTS, &FORTS, 3);
        let (mut groupe, _) = table.partir();
        // Le groupe attend un seul des deux.
        while table.attendus(&groupe).len() != 1 {
            let membre = table.attendus(&groupe)[0];
            let choix = coup(&groupe, table.deroulement(membre), membre, 99);
            table.repondre(&mut groupe, membre, choix).unwrap();
        }
        let absent = table.attendus(&groupe)[0];
        let present = absent.autre();
        table.maintenant = groupe.derniere_action().secondes() + DELAI_D_ABSENCE - 1;
        assert_eq!(
            table.ramener(&mut groupe, present),
            Err(ErreurChoix::AttenteTropCourte)
        );
        assert_eq!(
            table.ramener(&mut groupe, absent),
            Err(ErreurChoix::HorsDePropos)
        );
        table.maintenant += 1;
        let recits = table.ramener(&mut groupe, present).unwrap();
        assert!(recits[position(present)].contains(&Evenement::RetourAuCamp));
        // L'absent rentre aussi, ou dès qu'il aura réglé les suites de son
        // dernier combat.
        while !groupe.est_terminee([&table.hote, &table.invite]) {
            let choix = coup(&groupe, table.deroulement(absent), absent, 0);
            table.repondre(&mut groupe, absent, choix).unwrap();
        }
        assert_eq!(table.deroulement(absent).attente(), Attente::Camp);
    }

    /// Section 15.5 : quand tous les monstres des deux sont KO, chacun perd
    /// son sac, et garde la dernière étape franchie.
    #[test]
    fn la_defaite_commune_fait_perdre_le_sac_de_chacun() {
        let mut table = Table::new(&[("salamandre", 8)], &[("crapaud", 8)], 5);
        let (mut groupe, mut vus) = table.partir();
        for _ in 0..3_000 {
            if groupe.est_terminee([&table.hote, &table.invite]) {
                break;
            }
            let membre = table.attendus(&groupe)[0];
            let choix = coup(&groupe, table.deroulement(membre), membre, 999);
            let recits = table.repondre(&mut groupe, membre, choix).unwrap();
            for (vu, recit) in vus.iter_mut().zip(recits) {
                vu.extend(recit);
            }
        }
        assert!(groupe.est_terminee([&table.hote, &table.invite]));
        let derniere = vus[0]
            .iter()
            .filter_map(|e| match e {
                Evenement::Etape { numero, .. } => Some(*numero),
                _ => None,
            })
            .max()
            .unwrap();
        for (vu, depot) in vus.iter().zip(&table.depots) {
            assert!(
                vu.iter().any(|e| matches!(e, Evenement::Defaite { .. })),
                "{vu:?}"
            );
            let sauvegarde = depot.sauvegardes.borrow().last().cloned().unwrap();
            assert_eq!(sauvegarde.sac().potions(), 0);
            assert_eq!(sauvegarde.records_de_la_tour().a_deux, derniere - 1);
        }
    }

    /// Section 15.5 : en combat, seul le propriétaire du monstre actif
    /// agit.
    #[test]
    fn en_combat_seul_le_proprietaire_du_monstre_actif_agit() {
        let mut table = Table::new(&FORTS, &FORTS, 11);
        let (mut groupe, _) = table.partir();
        while groupe
            .combat()
            .is_none_or(|combat| combat.phase() != Phase::ChoixAction)
        {
            let membre = table.attendus(&groupe)[0];
            let choix = coup(&groupe, table.deroulement(membre), membre, 99);
            table.repondre(&mut groupe, membre, choix).unwrap();
        }
        let actif = groupe.combat().unwrap().membre_actif();
        assert_eq!(
            table.repondre(
                &mut groupe,
                actif.autre(),
                Choix::Combat(Action::Attaquer(0))
            ),
            Err(ErreurChoix::AuTourDeLAutre)
        );
        assert_eq!(
            table.repondre(&mut groupe, actif.autre(), Choix::DonnerOrdre),
            Err(ErreurChoix::AuTourDeLAutre)
        );
        let recits = table
            .repondre(&mut groupe, actif, Choix::DonnerOrdre)
            .unwrap();
        assert!(
            recits
                .iter()
                .all(|recit| matches!(recit[..], [Evenement::OrdreDonne { .. }]))
        );
    }
}
