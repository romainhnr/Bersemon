//! Duel en temps réel entre deux joueurs (section 15.3) : le combat attend
//! l'action de chacun avant de jouer le tour, et mesure le temps pour le
//! forfait. Les équipes sont des copies soignées : le duel ne change rien
//! aux parties.

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::combat::{Action, Combat, Combattant, Cote, ErreurCombat, Evenement, Issue, Phase};
use crate::heros::Ordre;
use crate::horodatage::Horodatage;
use crate::monstre::Monstre;
use crate::partie::Partie;
use crate::traque::Piste;
use crate::zone::Zone;

/// Un joueur qui attend l'autre depuis 5 minutes peut réclamer la victoire.
pub const DELAI_DE_FORFAIT: u64 = 5 * 60;

/// Ce que chaque joueur voit se passer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvenementDeDuel {
    Combat(Evenement),
    /// Le héros de ce côté donne son ordre à son monstre actif (section
    /// 4.2).
    OrdreDonne {
        ordre: Ordre,
        cible: Combattant,
    },
}

impl EvenementDeDuel {
    /// Le même événement, vu par l'autre joueur.
    pub fn miroir(&self) -> EvenementDeDuel {
        match self {
            EvenementDeDuel::Combat(evenement) => EvenementDeDuel::Combat(evenement.miroir()),
            EvenementDeDuel::OrdreDonne { ordre, cible } => EvenementDeDuel::OrdreDonne {
                ordre: *ordre,
                cible: cible.miroir(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurDuel {
    Combat(ErreurCombat),
    /// Ce côté a déjà choisi son action pour ce tour.
    ActionDejaChoisie,
    /// L'autre a joué, ou ce côté ne l'attend pas depuis assez longtemps.
    ForfaitImpossible,
}

impl fmt::Display for ErreurDuel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurDuel::Combat(erreur) => write!(f, "{erreur}"),
            ErreurDuel::ActionDejaChoisie => write!(f, "l'action de ce tour est déjà choisie"),
            ErreurDuel::ForfaitImpossible => write!(f, "pas de forfait possible"),
        }
    }
}

impl std::error::Error for ErreurDuel {}

impl From<ErreurCombat> for ErreurDuel {
    fn from(erreur: ErreurCombat) -> Self {
        ErreurDuel::Combat(erreur)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Duel {
    /// Vu par le premier joueur, côté `Joueur` ; le second est le côté
    /// `Adversaire`.
    combat: Combat,
    /// L'action choisie par chaque côté pour ce tour.
    actions: [Option<Action>; 2],
    /// Depuis quand chaque côté attend l'autre : il a choisi son action, ou
    /// n'a pas de monstre à remplacer quand l'autre en a un.
    pret_depuis: [Option<Horodatage>; 2],
    /// Ce qui s'est passé depuis la dernière action de chaque côté, vu par
    /// lui.
    recits: [Vec<EvenementDeDuel>; 2],
}

fn position(cote: Cote) -> usize {
    match cote {
        Cote::Joueur => 0,
        Cote::Adversaire => 1,
    }
}

impl Duel {
    /// `combat` doit être un duel (`Combat::duel`).
    pub fn new(combat: Combat) -> Result<Duel, ErreurCombat> {
        if !combat.est_un_duel() {
            return Err(ErreurCombat::ReserveAuDuel);
        }
        Ok(Duel {
            combat,
            actions: [None; 2],
            pret_depuis: [None; 2],
            recits: Default::default(),
        })
    }

    /// Avec une copie soignée de l'équipe de chacun, le bonus et l'ordre de
    /// son héros et la limite d'obéissance de ses monstres reçus (sections
    /// 4.2, 4.3 et 15.2). Échoue si une des équipes est vide.
    pub fn entre(
        premier: &Partie,
        second: &Partie,
        zones: &[Zone],
        piste: &Piste,
    ) -> Result<Duel, ErreurCombat> {
        let equipe = |partie: &Partie| -> Vec<Monstre> {
            partie
                .collection()
                .equipe()
                .iter()
                .map(|monstre| {
                    let mut copie = monstre.clone();
                    copie.soigner_completement();
                    copie
                })
                .collect()
        };
        let mut combat = Combat::duel(equipe(premier), equipe(second))?;
        for (cote, partie) in [(Cote::Joueur, premier), (Cote::Adversaire, second)] {
            combat = combat
                .avec_bonus_du_cote(cote, partie.heros().bonus_en_combat())
                .avec_bonus_de_role_du_cote(cote)
                .avec_ordre_du_cote(cote, partie.heros().classe().ordre())
                .avec_limite_du_cote(cote, partie.limite_d_obeissance(zones, piste));
        }
        Duel::new(combat)
    }

    /// Le combat vu par ce côté : le second joueur y est le côté `Joueur`.
    pub fn vu_par(&self, cote: Cote) -> Combat {
        match cote {
            Cote::Joueur => self.combat.clone(),
            Cote::Adversaire => self.combat.miroir(),
        }
    }

    pub fn recit(&self, cote: Cote) -> &[EvenementDeDuel] {
        &self.recits[position(cote)]
    }

    pub fn a_choisi(&self, cote: Cote) -> bool {
        self.actions[position(cote)].is_some()
    }

    /// Ce côté a fait sa part et attend l'autre.
    pub fn attend_l_autre(&self, cote: Cote) -> bool {
        !self.est_fini() && self.pret_depuis[position(cote)].is_some()
    }

    pub fn est_fini(&self) -> bool {
        matches!(self.combat.phase(), Phase::Termine(_))
    }

    pub fn vainqueur(&self) -> Option<Cote> {
        match self.combat.phase() {
            Phase::Termine(Issue::Victoire) => Some(Cote::Joueur),
            Phase::Termine(Issue::Defaite) => Some(Cote::Adversaire),
            _ => None,
        }
    }

    /// Le tour se joue dès que les deux côtés ont choisi.
    pub fn choisir(
        &mut self,
        cote: Cote,
        action: Action,
        maintenant: Horodatage,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<(), ErreurDuel> {
        if self.combat.phase() != Phase::ChoixAction {
            return Err(match self.combat.phase() {
                Phase::Remplacement => ErreurCombat::RemplacementAttendu,
                _ => ErreurCombat::CombatTermine,
            }
            .into());
        }
        if self.a_choisi(cote) {
            return Err(ErreurDuel::ActionDejaChoisie);
        }
        self.combat.verifier_action(cote, action)?;
        self.recits[position(cote)].clear();
        self.actions[position(cote)] = Some(action);
        self.pret_depuis[position(cote)] = Some(maintenant);
        if let [Some(premiere), Some(seconde)] = self.actions {
            self.actions = [None; 2];
            let evenements = self
                .combat
                .jouer_tour_en_duel(premiere, seconde, aleatoire)?;
            self.publier(evenements.into_iter().map(EvenementDeDuel::Combat));
            self.noter_qui_attend(maintenant);
        }
        Ok(())
    }

    /// Chacun remplace son monstre KO ; le tour suivant attend les deux.
    pub fn remplacer(
        &mut self,
        cote: Cote,
        index: usize,
        maintenant: Horodatage,
    ) -> Result<(), ErreurDuel> {
        let evenements = self.combat.remplacer_pour(cote, index)?;
        self.recits[position(cote)].clear();
        self.publier(evenements.into_iter().map(EvenementDeDuel::Combat));
        self.noter_qui_attend(maintenant);
        Ok(())
    }

    /// Avant de choisir son action ; l'autre le voit aussitôt.
    pub fn donner_ordre(&mut self, cote: Cote) -> Result<(), ErreurDuel> {
        if self.a_choisi(cote) {
            return Err(ErreurDuel::ActionDejaChoisie);
        }
        let (ordre, cible) = self.combat.donner_ordre_pour(cote)?;
        self.recits[position(cote)].clear();
        self.publier([EvenementDeDuel::OrdreDonne { ordre, cible }]);
        Ok(())
    }

    pub fn abandonner(&mut self, cote: Cote) -> Result<(), ErreurDuel> {
        let evenements = self.combat.abandonner(cote)?;
        self.recits[position(cote)].clear();
        self.publier(evenements.into_iter().map(EvenementDeDuel::Combat));
        Ok(())
    }

    /// Ce côté attend l'autre depuis au moins 5 minutes.
    pub fn forfait_possible(&self, cote: Cote, maintenant: Horodatage) -> bool {
        let autre = cote.oppose();
        match self.pret_depuis[position(cote)] {
            Some(depuis) => {
                !self.est_fini()
                    && self.pret_depuis[position(autre)].is_none()
                    && maintenant.secondes() >= depuis.secondes() + DELAI_DE_FORFAIT
            }
            None => false,
        }
    }

    pub fn reclamer_la_victoire(
        &mut self,
        cote: Cote,
        maintenant: Horodatage,
    ) -> Result<(), ErreurDuel> {
        if !self.forfait_possible(cote, maintenant) {
            return Err(ErreurDuel::ForfaitImpossible);
        }
        let evenements = self.combat.declarer_forfait(cote.oppose())?;
        self.recits[position(cote)].clear();
        self.publier(evenements.into_iter().map(EvenementDeDuel::Combat));
        Ok(())
    }

    /// `evenements` sont vus par le premier joueur ; le second les voit en
    /// miroir.
    fn publier(&mut self, evenements: impl IntoIterator<Item = EvenementDeDuel>) {
        for evenement in evenements {
            self.recits[1].push(evenement.miroir());
            self.recits[0].push(evenement);
        }
    }

    /// Après un tour ou un remplacement : en attendant des remplacements,
    /// le côté qui n'en a pas à faire attend l'autre dès maintenant.
    fn noter_qui_attend(&mut self, maintenant: Horodatage) {
        for cote in [Cote::Joueur, Cote::Adversaire] {
            let pret = self.combat.phase() == Phase::Remplacement
                && !self.combat.remplacement_attendu(cote);
            self.pret_depuis[position(cote)] = pret.then_some(maintenant);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::attaque::{Attaque, Categorie};
    use crate::donnees_de_test::{attaque, monstre, stats_uniformes};
    use crate::stats::Stats;
    use crate::statut::Statut;
    use crate::types::Type;

    // Comme dans les tests du combat : au niveau 5, 20 PV et 10 dans chaque
    // stat ; le coup inflige 5 dégâts avec un pourcentage de 100.
    fn coup() -> Attaque {
        attaque("coup", Type::Normal, Categorie::Physique, 40, 100)
    }

    fn lent() -> Monstre {
        monstre(&[Type::Feu], stats_uniformes(50), 5, vec![coup()])
    }

    /// Vitesse 11, plus rapide que `lent`.
    fn rapide() -> Monstre {
        let base = Stats {
            vitesse: 60,
            ..stats_uniformes(50)
        };
        monstre(&[Type::Feu], base, 5, vec![coup()])
    }

    fn affaibli(mut monstre: Monstre, pv: u32) -> Monstre {
        monstre.subir_degats(monstre.pv_actuels() - pv);
        monstre
    }

    fn duel(premier: Vec<Monstre>, second: Vec<Monstre>) -> Duel {
        Duel::new(Combat::duel(premier, second).unwrap()).unwrap()
    }

    fn a(secondes: u64) -> Horodatage {
        Horodatage::depuis_secondes(secondes)
    }

    fn joueur(index: usize) -> Combattant {
        Combattant {
            cote: Cote::Joueur,
            index,
        }
    }

    fn adversaire(index: usize) -> Combattant {
        Combattant {
            cote: Cote::Adversaire,
            index,
        }
    }

    #[test]
    fn le_tour_attend_les_deux_actions_et_chacun_le_voit_de_son_cote() {
        let mut duel = duel(vec![rapide()], vec![lent()]);
        let mut sans_tirage = AleatoireScripte::new([]);
        duel.choisir(
            Cote::Adversaire,
            Action::Attaquer(0),
            a(10),
            &mut sans_tirage,
        )
        .unwrap();
        assert!(duel.attend_l_autre(Cote::Adversaire));
        assert_eq!(
            duel.choisir(
                Cote::Adversaire,
                Action::Attaquer(0),
                a(11),
                &mut sans_tirage
            ),
            Err(ErreurDuel::ActionDejaChoisie)
        );
        assert_eq!(
            duel.choisir(Cote::Joueur, Action::Fuir, a(12), &mut sans_tirage),
            Err(ErreurDuel::Combat(ErreurCombat::InterditEnDuel))
        );
        // Le premier, plus rapide, frappe le premier.
        let mut tirages = AleatoireScripte::new([2, 100, 2, 100]);
        duel.choisir(Cote::Joueur, Action::Attaquer(0), a(20), &mut tirages)
            .unwrap();
        assert!(!duel.attend_l_autre(Cote::Joueur));
        let premier = duel.recit(Cote::Joueur);
        assert_eq!(
            premier[0],
            EvenementDeDuel::Combat(Evenement::AttaqueLancee {
                lanceur: joueur(0),
                attaque: coup(),
            })
        );
        // Le second voit la même chose de son côté : c'est l'adversaire qui
        // frappe.
        let second = duel.recit(Cote::Adversaire);
        assert_eq!(second.len(), premier.len());
        assert_eq!(
            second[0],
            EvenementDeDuel::Combat(Evenement::AttaqueLancee {
                lanceur: adversaire(0),
                attaque: coup(),
            })
        );
        assert_eq!(duel.vu_par(Cote::Adversaire).equipe()[0].pv_actuels(), 15);
        assert_eq!(duel.vu_par(Cote::Joueur).equipe()[0].pv_actuels(), 15);
    }

    #[test]
    fn un_joueur_qui_attend_un_remplacement_peut_reclamer_la_victoire_apres_5_minutes() {
        // Le second, plus rapide, met KO le premier monstre du premier.
        let mut duel = duel(vec![affaibli(lent(), 3), lent()], vec![rapide()]);
        let mut sans_tirage = AleatoireScripte::new([]);
        duel.choisir(Cote::Joueur, Action::Attaquer(0), a(0), &mut sans_tirage)
            .unwrap();
        let mut tirages = AleatoireScripte::new([2, 100]);
        duel.choisir(Cote::Adversaire, Action::Attaquer(0), a(100), &mut tirages)
            .unwrap();
        let combat = duel.vu_par(Cote::Joueur);
        assert_eq!(combat.phase(), Phase::Remplacement);
        assert!(combat.remplacement_attendu(Cote::Joueur));
        assert!(!combat.remplacement_attendu(Cote::Adversaire));
        assert!(duel.attend_l_autre(Cote::Adversaire));
        assert!(!duel.forfait_possible(Cote::Adversaire, a(399)));
        assert!(!duel.forfait_possible(Cote::Joueur, a(1_000)));
        assert_eq!(
            duel.reclamer_la_victoire(Cote::Adversaire, a(399)),
            Err(ErreurDuel::ForfaitImpossible)
        );
        duel.reclamer_la_victoire(Cote::Adversaire, a(400)).unwrap();
        assert_eq!(duel.vainqueur(), Some(Cote::Adversaire));
        assert_eq!(
            duel.recit(Cote::Joueur).last(),
            Some(&EvenementDeDuel::Combat(Evenement::Forfait(Cote::Joueur)))
        );
        assert_eq!(
            duel.recit(Cote::Adversaire),
            [EvenementDeDuel::Combat(Evenement::Forfait(
                Cote::Adversaire
            ))]
        );
        assert_eq!(
            duel.vu_par(Cote::Adversaire).phase(),
            Phase::Termine(Issue::Victoire)
        );
    }

    #[test]
    fn chacun_remplace_son_monstre_ko_avant_le_tour_suivant() {
        let mut duel = duel(vec![affaibli(lent(), 3), lent()], vec![rapide()]);
        let mut sans_tirage = AleatoireScripte::new([]);
        duel.choisir(Cote::Joueur, Action::Attaquer(0), a(0), &mut sans_tirage)
            .unwrap();
        let mut tirages = AleatoireScripte::new([2, 100]);
        duel.choisir(Cote::Adversaire, Action::Attaquer(0), a(0), &mut tirages)
            .unwrap();
        assert_eq!(
            duel.choisir(
                Cote::Adversaire,
                Action::Attaquer(0),
                a(1),
                &mut sans_tirage
            ),
            Err(ErreurDuel::Combat(ErreurCombat::RemplacementAttendu))
        );
        assert_eq!(
            duel.remplacer(Cote::Adversaire, 0, a(1)),
            Err(ErreurDuel::Combat(ErreurCombat::AucunRemplacementAttendu))
        );
        duel.remplacer(Cote::Joueur, 1, a(2)).unwrap();
        assert_eq!(duel.vu_par(Cote::Joueur).phase(), Phase::ChoixAction);
        assert!(!duel.attend_l_autre(Cote::Adversaire));
        assert_eq!(
            duel.recit(Cote::Adversaire).last(),
            Some(&EvenementDeDuel::Combat(Evenement::MonstreEnvoye(
                adversaire(1)
            )))
        );
    }

    #[test]
    fn si_les_deux_equipes_tombent_au_meme_tour_la_premiere_tombee_perd() {
        // Le premier, empoisonné à 1 PV, met KO le dernier monstre du second,
        // puis tombe sous le poison.
        let mut empoisonne = affaibli(rapide(), 1);
        empoisonne.infliger_statut(Statut::Poison);
        let mut duel = duel(vec![empoisonne], vec![affaibli(lent(), 3)]);
        let mut sans_tirage = AleatoireScripte::new([]);
        duel.choisir(
            Cote::Adversaire,
            Action::Attaquer(0),
            a(0),
            &mut sans_tirage,
        )
        .unwrap();
        let mut tirages = AleatoireScripte::new([2, 100]);
        duel.choisir(Cote::Joueur, Action::Attaquer(0), a(0), &mut tirages)
            .unwrap();
        let combat = duel.vu_par(Cote::Joueur);
        assert!(combat.equipe()[0].est_ko() && combat.adversaires()[0].est_ko());
        assert_eq!(duel.vainqueur(), Some(Cote::Joueur));
        assert_eq!(
            duel.recit(Cote::Adversaire).last(),
            Some(&EvenementDeDuel::Combat(Evenement::FinDuCombat(
                Issue::Defaite
            )))
        );
    }

    #[test]
    fn l_ordre_se_donne_avant_de_choisir_et_l_abandon_a_tout_moment() {
        let combat = Combat::duel(vec![lent()], vec![lent()])
            .unwrap()
            .avec_ordre_du_cote(Cote::Adversaire, Ordre::BouclierDeFoi);
        let mut duel = Duel::new(combat).unwrap();
        assert_eq!(
            duel.donner_ordre(Cote::Joueur),
            Err(ErreurDuel::Combat(ErreurCombat::OrdreImpossible))
        );
        duel.donner_ordre(Cote::Adversaire).unwrap();
        assert_eq!(
            duel.recit(Cote::Joueur),
            [EvenementDeDuel::OrdreDonne {
                ordre: Ordre::BouclierDeFoi,
                cible: adversaire(0),
            }]
        );
        assert_eq!(
            duel.recit(Cote::Adversaire),
            [EvenementDeDuel::OrdreDonne {
                ordre: Ordre::BouclierDeFoi,
                cible: joueur(0),
            }]
        );
        assert_eq!(
            duel.vu_par(Cote::Joueur)
                .stats_en_combat(adversaire(0))
                .defense,
            12
        );
        let mut sans_tirage = AleatoireScripte::new([]);
        duel.choisir(Cote::Joueur, Action::Attaquer(0), a(0), &mut sans_tirage)
            .unwrap();
        duel.abandonner(Cote::Adversaire).unwrap();
        assert_eq!(duel.vainqueur(), Some(Cote::Joueur));
        assert_eq!(
            duel.abandonner(Cote::Joueur),
            Err(ErreurDuel::Combat(ErreurCombat::CombatTermine))
        );
        assert!(!duel.attend_l_autre(Cote::Joueur));
    }

    #[test]
    fn chaque_cote_a_une_copie_soignee_et_les_regles_de_son_heros() {
        use crate::catalogue::Catalogue;
        use crate::contenu::ContenuV1;
        use crate::espece::EspeceId;
        use crate::heros::{Classe, Heros};
        use crate::niveau::Niveau;
        use crate::nom::Nom;
        use crate::stats::ValeursInternes;

        let contenu = ContenuV1::new();
        let creer = |id: &str| {
            Monstre::creer(
                contenu.espece(&EspeceId::new(id)).unwrap(),
                Niveau::new(5).unwrap(),
                ValeursInternes::default(),
                false,
            )
            .unwrap()
        };
        let mut blesse = creer("salamandre");
        blesse.subir_degats(5);
        let aldric = Partie::nouvelle(
            Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier),
            blesse,
        );
        let ysolde = Partie::nouvelle(
            Heros::new(Nom::new("Ysolde").unwrap(), Classe::Mage),
            creer("mandragore"),
        );
        let duel = Duel::entre(&aldric, &ysolde, contenu.zones(), contenu.piste()).unwrap();
        let combat = duel.vu_par(Cote::Joueur);
        let salamandre = &combat.equipe()[0];
        assert_eq!(salamandre.pv_actuels(), salamandre.stats().pv);
        assert!(aldric.collection().equipe()[0].pv_actuels() < salamandre.stats().pv);
        assert_eq!(
            combat.ordre_disponible_pour(Cote::Joueur),
            Some(Ordre::CriDeGuerre)
        );
        assert_eq!(
            combat.ordre_disponible_pour(Cote::Adversaire),
            Some(Ordre::Incantation)
        );
        // +10 % d'Attaque pour le Guerrier, d'Attaque spéciale pour le Mage,
        // et le bonus du rôle de chaque côté : la Salamandre est offensive,
        // la Mandragore de soutien, avec Carapace (section 4.2).
        let attaque = salamandre.stats().attaque;
        assert_eq!(
            combat.stats_en_combat(joueur(0)).attaque,
            attaque * 120 / 100
        );
        let defense = combat.adversaires()[0].stats().defense;
        assert_eq!(
            combat.stats_en_combat(adversaire(0)).defense,
            defense * 120 / 100
        );
        let mandragore = &combat.adversaires()[0];
        assert_eq!(
            combat.stats_en_combat(adversaire(0)).attaque_speciale,
            mandragore.stats().attaque_speciale * 110 / 100
        );
        assert_eq!(
            duel.vu_par(Cote::Adversaire).ordre_disponible(),
            Some(Ordre::Incantation)
        );
    }

    #[test]
    fn les_monstres_recus_du_second_peuvent_lui_desobeir() {
        let mut recu = lent();
        recu.marquer_recu(crate::nom::Nom::new("Aldric").unwrap());
        let combat = Combat::duel(vec![rapide()], vec![recu])
            .unwrap()
            .avec_limite_du_cote(Cote::Adversaire, 3);
        let mut duel = Duel::new(combat).unwrap();
        let mut sans_tirage = AleatoireScripte::new([]);
        duel.choisir(
            Cote::Adversaire,
            Action::Attaquer(0),
            a(0),
            &mut sans_tirage,
        )
        .unwrap();
        // Le premier frappe, puis le monstre du second, 2 niveaux au-dessus
        // de sa limite, désobéit 20 fois sur 100.
        let mut tirages = AleatoireScripte::new([2, 100, 20]);
        duel.choisir(Cote::Joueur, Action::Attaquer(0), a(0), &mut tirages)
            .unwrap();
        assert_eq!(
            duel.recit(Cote::Adversaire).last(),
            Some(&EvenementDeDuel::Combat(Evenement::Desobeissance(joueur(
                0
            ))))
        );
    }

    #[test]
    fn hors_duel_ni_tour_a_deux_ni_abandon() {
        let mut combat = Combat::new(vec![lent()], vec![lent()]).unwrap();
        let mut sans_tirage = AleatoireScripte::new([]);
        assert_eq!(
            combat.jouer_tour_en_duel(Action::Attaquer(0), Action::Attaquer(0), &mut sans_tirage),
            Err(ErreurCombat::ReserveAuDuel)
        );
        assert_eq!(
            combat.abandonner(Cote::Joueur),
            Err(ErreurCombat::ReserveAuDuel)
        );
        assert_eq!(Duel::new(combat).err(), Some(ErreurCombat::ReserveAuDuel));
        let mut duel = Combat::duel(vec![lent()], vec![lent()]).unwrap();
        assert!(!duel.fuite_possible() && !duel.filet_possible());
        assert_eq!(
            duel.jouer_tour(Action::Attaquer(0), &mut sans_tirage),
            Err(ErreurCombat::InterditEnDuel)
        );
    }
}
