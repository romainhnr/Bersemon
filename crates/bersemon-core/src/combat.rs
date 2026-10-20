//! Combat au tour par tour entre l'équipe du joueur et un groupe de monstres
//! sauvages (section 10) : un monstre actif de chaque côté, les autres en
//! réserve.
//!
//! Le combat ne fait aucun affichage : chaque étape rend la liste des
//! événements survenus, que l'interface met en texte. Il ne bloque pas non
//! plus en attendant le joueur, ce qui convient à la console comme à un bot
//! qui traite une commande à la fois.
//!
//! Tirages d'un tour, dans l'ordre :
//! - l'attaque choisie par l'IA, si le monstre adverse en connaît plusieurs ;
//! - pour une attaque du joueur, l'ordre d'action en cas d'égalité de vitesse ;
//!   pour un filet, la capture, si sa chance n'est ni nulle ni certaine ;
//! - pour chaque monstre qui attaque : la paralysie s'il est paralysé, ceux de
//!   la résolution de l'attaque, puis l'effet secondaire si la cible peut le
//!   subir.

use std::cmp::Ordering;
use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::attaque::{Attaque, EffetSecondaire};
use crate::capture::calculer_chance_capture;
use crate::degats::{ResultatAttaque, a_le_bonus_de_type, estimer_degats, resoudre_attaque};
use crate::experience::tirer_gain_experience;
use crate::habilete::{BONUS_D_INSTINCT, Habilete, majorer};
use crate::heros::Ordre;
use crate::humain::IaHumain;
use crate::monstre::{Monstre, RapportExperience};
use crate::soin::{EffetDuSoin, ErreurSoin, Soin, Soins, appliquer_soin, verifier_cible};
use crate::stats::{BonusDeStats, Stats, appliquer_bonus, cumuler_bonus};
use crate::statut::{CHANCES_BLOCAGE_PARALYSIE, Statut, degats_de_poison};
use crate::types::{Efficacite, efficacite};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cote {
    Joueur,
    Adversaire,
}

impl Cote {
    pub fn oppose(self) -> Cote {
        match self {
            Cote::Joueur => Cote::Adversaire,
            Cote::Adversaire => Cote::Joueur,
        }
    }
}

/// Désigne un monstre du combat par son côté et sa position dans l'équipe du
/// joueur ou dans le groupe adverse. Les positions ne changent pas pendant le
/// combat, même après une capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Combattant {
    pub cote: Cote,
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Issue {
    Victoire,
    Defaite,
    Fuite,
}

/// Ce que le combat attend du joueur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// L'action du tour, par `jouer_tour`.
    ChoixAction,
    /// Le monstre à envoyer à la place du monstre actif KO, par `remplacer`.
    Remplacement,
    Termine(Issue),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Position de l'attaque dans la liste des attaques du monstre actif.
    Attaquer(usize),
    /// Position, dans l'équipe, du monstre à envoyer.
    Changer(usize),
    LancerFilet,
    /// Le soin et la position, dans l'équipe, du monstre à soigner (section
    /// 14.4).
    UtiliserSoin(Soin, usize),
    Fuir,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evenement {
    MonstreEnvoye(Combattant),
    AttaqueLancee {
        lanceur: Combattant,
        attaque: Attaque,
    },
    AttaqueRatee(Combattant),
    SansEffet(Combattant),
    /// `degats` est la perte réelle de PV, qui ne dépasse pas les PV restants.
    Degats {
        cible: Combattant,
        degats: u32,
        critique: bool,
        efficacite: Efficacite,
    },
    StatutInflige {
        cible: Combattant,
        statut: Statut,
    },
    BloqueParLaParalysie(Combattant),
    DegatsDePoison {
        cible: Combattant,
        degats: u32,
    },
    Ko(Combattant),
    FiletLance {
        cible: Combattant,
        chance: u32,
    },
    CaptureReussie(Combattant),
    CaptureRatee(Combattant),
    SoinUtilise {
        soin: Soin,
        cible: Combattant,
        effet: EffetDuSoin,
    },
    Fuite,
    FinDuCombat(Issue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurCombat {
    EquipeSansMonstreApte,
    AucunAdversaire,
    AdversaireKo(usize),
    CombatTermine,
    RemplacementAttendu,
    AucunRemplacementAttendu,
    AttaqueInexistante(usize),
    MonstreInexistant(usize),
    MonstreKo(usize),
    MonstreDejaActif(usize),
    CaptureImpossible(usize),
    AucunSoin(Soin),
    FuiteImpossible,
    FiletImpossible,
    /// Aucun ordre dans ce combat, ou il a déjà été donné.
    OrdreImpossible,
}

impl fmt::Display for ErreurCombat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurCombat::EquipeSansMonstreApte => {
                write!(f, "l'équipe n'a aucun monstre en état de combattre")
            }
            ErreurCombat::AucunAdversaire => write!(f, "le groupe adverse est vide"),
            ErreurCombat::AdversaireKo(index) => {
                write!(f, "le monstre adverse en position {index} est déjà KO")
            }
            ErreurCombat::CombatTermine => write!(f, "le combat est terminé"),
            ErreurCombat::RemplacementAttendu => {
                write!(f, "il faut d'abord remplacer le monstre KO")
            }
            ErreurCombat::AucunRemplacementAttendu => {
                write!(f, "aucun monstre n'est à remplacer")
            }
            ErreurCombat::AttaqueInexistante(index) => {
                write!(f, "aucune attaque en position {index}")
            }
            ErreurCombat::MonstreInexistant(index) => {
                write!(f, "aucun monstre en position {index} dans l'équipe")
            }
            ErreurCombat::MonstreKo(index) => {
                write!(f, "le monstre en position {index} est KO")
            }
            ErreurCombat::MonstreDejaActif(index) => {
                write!(f, "le monstre en position {index} combat déjà")
            }
            ErreurCombat::CaptureImpossible(index) => {
                write!(
                    f,
                    "le monstre adverse en position {index} ne peut pas être capturé"
                )
            }
            ErreurCombat::AucunSoin(soin) => write!(f, "aucun soin de ce genre : {soin:?}"),
            ErreurCombat::FuiteImpossible => write!(f, "impossible de fuir ce combat"),
            ErreurCombat::FiletImpossible => {
                write!(f, "impossible de lancer un filet dans ce combat")
            }
            ErreurCombat::OrdreImpossible => {
                write!(f, "le héros n'a plus d'ordre à donner dans ce combat")
            }
        }
    }
}

impl std::error::Error for ErreurCombat {}

/// Ce que le joueur voit d'une attaque avant de la choisir (section 10.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApercuAttaque {
    /// Si elle touche, sans coup critique ; 0 contre une cible immunisée.
    pub degats_min: u32,
    pub degats_max: u32,
    pub efficacite: Efficacite,
    pub bonus_de_type: bool,
}

/// Ce que le combat laisse derrière lui. `issue` vaut `None` si le combat a
/// été interrompu avant la fin ; `captures` suit l'ordre des captures ;
/// `experience` suit l'ordre de l'équipe, avec `None` pour un monstre qui n'a
/// rien gagné ; `soins` sont les soins restants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BilanCombat {
    pub issue: Option<Issue>,
    pub equipe: Vec<Monstre>,
    pub captures: Vec<Monstre>,
    /// Les monstres sauvages mis KO et non capturés, qui peuvent lâcher leur
    /// ressource après une victoire (section 14.3) : ni ceux d'un humain, ni
    /// un Apôtre, ni un mort-vivant.
    pub vaincus_sauvages: Vec<Monstre>,
    pub experience: Vec<Option<RapportExperience>>,
    pub soins: Soins,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Combat {
    equipe: Vec<Monstre>,
    actif_joueur: usize,
    adversaires: Vec<Monstre>,
    actif_adversaire: usize,
    /// Positions des monstres adverses capturés, dans l'ordre des captures :
    /// ils restent dans la liste pour que les positions ne changent pas, mais
    /// ne combattent plus.
    captures: Vec<usize>,
    /// Pour chaque monstre adverse, les monstres de l'équipe qui ont été
    /// actifs face à lui : ce sont eux qui reçoivent son XP (section 7).
    participants: Vec<Vec<usize>>,
    /// Bonus de la classe du héros, appliqué aux stats des monstres du joueur
    /// (section 4.2).
    bonus_joueur: BonusDeStats,
    /// L'ordre que le héros peut donner (section 4.2).
    ordre: Option<Ordre>,
    /// Position, dans l'équipe, du monstre qui a reçu l'ordre : un seul
    /// ordre par combat, dont le bonus dure jusqu'à la fin.
    ordre_donne: Option<usize>,
    /// Multiplie la chance de capture, en fraction : les filets améliorés
    /// par Gunther (section 13.4).
    facteur_des_filets: (u64, u64),
    /// Soins que le joueur porte (section 14.4).
    soins: Soins,
    /// Face à un Apôtre, ni fuite ni filet (section 9.4).
    contre_un_apotre: bool,
    /// Face à un humain, ni fuite ni filet, et une seule capture après la
    /// victoire (section 10.7).
    contre_un_humain: bool,
    /// Face à un marchand relevé d'entre les morts, ni filet ni capture
    /// (section 14.2).
    sans_capture: bool,
    ia: Ia,
    phase: Phase,
}

impl Combat {
    /// Le premier monstre apte de l'équipe et le premier monstre du groupe
    /// commencent le combat. `terminer` rend ensuite l'équipe avec ses PV et
    /// ses statuts.
    pub fn new(equipe: Vec<Monstre>, adversaires: Vec<Monstre>) -> Result<Self, ErreurCombat> {
        let actif_joueur = equipe
            .iter()
            .position(|monstre| !monstre.est_ko())
            .ok_or(ErreurCombat::EquipeSansMonstreApte)?;
        if adversaires.is_empty() {
            return Err(ErreurCombat::AucunAdversaire);
        }
        if let Some(index) = adversaires.iter().position(Monstre::est_ko) {
            return Err(ErreurCombat::AdversaireKo(index));
        }
        let mut combat = Self {
            equipe,
            actif_joueur,
            participants: vec![Vec::new(); adversaires.len()],
            adversaires,
            actif_adversaire: 0,
            captures: Vec::new(),
            bonus_joueur: BonusDeStats::default(),
            ordre: None,
            ordre_donne: None,
            facteur_des_filets: (1, 1),
            soins: Soins::default(),
            contre_un_apotre: false,
            contre_un_humain: false,
            sans_capture: false,
            ia: Ia::Hasard,
            phase: Phase::ChoixAction,
        };
        combat.noter_participation();
        Ok(combat)
    }

    /// Soins que le joueur emporte dans le combat ; le bilan rend ceux qui
    /// restent.
    pub fn avec_soins(mut self, soins: Soins) -> Self {
        self.soins = soins;
        self
    }

    pub fn soins(&self) -> &Soins {
        &self.soins
    }

    /// Combat contre un Apôtre : ni fuite ni filet ; il ne se capture que KO,
    /// après la victoire (section 9.4). Son IA est avancée (section 10.7).
    pub fn contre_un_apotre(mut self) -> Self {
        self.contre_un_apotre = true;
        self.ia = Ia::Avancee;
        self
    }

    pub fn est_contre_un_apotre(&self) -> bool {
        self.contre_un_apotre
    }

    /// Combat contre un humain et son équipe : ni fuite ni filet, son IA
    /// ne joue pas au hasard, et le joueur ne prend qu'un de ses monstres
    /// après la victoire (section 10.7).
    pub fn contre_un_humain(mut self, ia: IaHumain) -> Self {
        self.contre_un_humain = true;
        self.ia = match ia {
            IaHumain::Simple => Ia::PlusForte,
            IaHumain::Relevee => Ia::PlusForteEtChangement,
        };
        self
    }

    pub fn est_contre_un_humain(&self) -> bool {
        self.contre_un_humain
    }

    /// Combat contre un marchand relevé d'entre les morts : fuite possible,
    /// mais ni filet ni capture (section 14.2).
    pub fn sans_capture(mut self) -> Self {
        self.sans_capture = true;
        self
    }

    pub fn est_sans_capture(&self) -> bool {
        self.sans_capture
    }

    pub fn fuite_possible(&self) -> bool {
        !self.contre_un_apotre && !self.contre_un_humain
    }

    pub fn filet_possible(&self) -> bool {
        !self.contre_un_apotre && !self.contre_un_humain && !self.sans_capture
    }

    /// Bonus appliqué aux stats des monstres du joueur pendant tout le combat,
    /// comme celui de la classe du héros (section 4.2).
    pub fn avec_bonus_du_joueur(mut self, bonus: BonusDeStats) -> Self {
        self.bonus_joueur = bonus;
        self
    }

    pub fn avec_ordre(mut self, ordre: Ordre) -> Self {
        self.ordre = Some(ordre);
        self
    }

    /// L'ordre que le héros peut donner maintenant : un seul par combat,
    /// avant de choisir l'action du tour.
    pub fn ordre_disponible(&self) -> Option<Ordre> {
        self.ordre
            .filter(|_| self.ordre_donne.is_none() && self.phase == Phase::ChoixAction)
    }

    /// Le monstre actif du joueur reçoit le bonus de l'ordre jusqu'à la fin
    /// du combat. L'ordre ne prend pas le tour : le joueur choisit ensuite
    /// l'action de son monstre, et l'adversaire n'agit pas entre-temps. Sans
    /// tirage.
    pub fn donner_ordre(&mut self) -> Result<(Ordre, Combattant), ErreurCombat> {
        match self.phase {
            Phase::ChoixAction => {}
            Phase::Remplacement => return Err(ErreurCombat::RemplacementAttendu),
            Phase::Termine(_) => return Err(ErreurCombat::CombatTermine),
        }
        let ordre = self
            .ordre_disponible()
            .ok_or(ErreurCombat::OrdreImpossible)?;
        self.ordre_donne = Some(self.actif_joueur);
        Ok((ordre, self.actif(Cote::Joueur)))
    }

    /// Le monstre de l'équipe qui a reçu l'ordre du héros.
    pub fn a_recu_l_ordre(&self, index: usize) -> bool {
        self.ordre_donne == Some(index)
    }

    pub fn avec_filets(mut self, facteur: (u64, u64)) -> Self {
        self.facteur_des_filets = facteur;
        self
    }

    /// Stats utilisées en combat : celles du monstre, plus le bonus de son
    /// habileté, de chaque côté, et le bonus du joueur pour ses monstres
    /// (section 5.9), avec celui de l'ordre pour le monstre qui l'a reçu
    /// (section 4.2).
    pub fn stats_en_combat(&self, combattant: Combattant) -> Stats<u32> {
        let monstre = self.monstre(combattant);
        let habilete = monstre
            .habilete()
            .map(Habilete::bonus_de_stats)
            .unwrap_or_default();
        let bonus = match combattant.cote {
            Cote::Joueur => {
                let bonus = cumuler_bonus(&habilete, &self.bonus_joueur);
                match self.ordre.filter(|_| self.a_recu_l_ordre(combattant.index)) {
                    Some(ordre) => cumuler_bonus(&bonus, &ordre.bonus()),
                    None => bonus,
                }
            }
            Cote::Adversaire => habilete,
        };
        appliquer_bonus(&monstre.stats(), &bonus)
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn equipe(&self) -> &[Monstre] {
        &self.equipe
    }

    pub fn adversaires(&self) -> &[Monstre] {
        &self.adversaires
    }

    pub fn actif(&self, cote: Cote) -> Combattant {
        let index = match cote {
            Cote::Joueur => self.actif_joueur,
            Cote::Adversaire => self.actif_adversaire,
        };
        Combattant { cote, index }
    }

    /// Panique si `combattant` ne désigne pas un monstre de ce combat.
    pub fn monstre(&self, combattant: Combattant) -> &Monstre {
        match combattant.cote {
            Cote::Joueur => &self.equipe[combattant.index],
            Cote::Adversaire => &self.adversaires[combattant.index],
        }
    }

    /// Chance, en pourcentage, de capturer le monstre adverse actif.
    pub fn chance_de_capture(&self) -> u32 {
        calculer_chance_capture(
            self.monstre(self.actif(Cote::Joueur)),
            self.monstre(self.actif(Cote::Adversaire)),
            self.facteur_des_filets,
        )
    }

    /// Une attaque du monstre actif du joueur face au monstre adverse actif.
    /// `None` si le monstre n'a pas d'attaque à cette position.
    pub fn apercu_attaque(&self, index_attaque: usize) -> Option<ApercuAttaque> {
        let joueur = self.actif(Cote::Joueur);
        let adversaire = self.actif(Cote::Adversaire);
        let attaquant = self.monstre(joueur);
        let defenseur = self.monstre(adversaire);
        let attaque = attaquant.attaques().liste().get(index_attaque)?;
        let (degats_min, degats_max) = estimer_degats(
            attaquant,
            &self.stats_en_combat(joueur),
            attaque,
            defenseur,
            &self.stats_en_combat(adversaire),
        );
        Some(ApercuAttaque {
            degats_min,
            degats_max,
            efficacite: efficacite(attaque.type_attaque(), defenseur.espece().types()),
            bonus_de_type: a_le_bonus_de_type(attaquant, attaque),
        })
    }

    /// Joue un tour complet : l'action du joueur, celle de l'IA, puis la fin
    /// du tour. Une action refusée ne consomme aucun tirage et ne modifie pas
    /// le combat.
    pub fn jouer_tour(
        &mut self,
        action: Action,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<Vec<Evenement>, ErreurCombat> {
        match self.phase {
            Phase::ChoixAction => {}
            Phase::Remplacement => return Err(ErreurCombat::RemplacementAttendu),
            Phase::Termine(_) => return Err(ErreurCombat::CombatTermine),
        }
        self.valider(action)?;
        // L'IA décide avant de connaître l'action du joueur.
        let action_adversaire = self.choisir_action_adversaire(aleatoire);

        let mut evenements = Vec::new();
        // Un changement de monstre passe avant toute attaque ; celui du
        // joueur, s'il change aussi, passe le premier.
        if !matches!(action, Action::Changer(_)) {
            self.changer_d_adversaire(action_adversaire, &mut evenements);
        }
        let attaque_adversaire = match action_adversaire {
            ActionAdversaire::Attaquer(attaque) => Some(attaque),
            ActionAdversaire::Changer(_) => None,
        };
        match action {
            Action::Attaquer(attaque_joueur) => match attaque_adversaire {
                Some(attaque_adversaire) => {
                    for cote in self.ordre_du_tour(aleatoire) {
                        let attaque = match cote {
                            Cote::Joueur => attaque_joueur,
                            Cote::Adversaire => attaque_adversaire,
                        };
                        self.attaquer(cote, attaque, false, aleatoire, &mut evenements);
                    }
                }
                None => {
                    self.attaquer(
                        Cote::Joueur,
                        attaque_joueur,
                        false,
                        aleatoire,
                        &mut evenements,
                    );
                }
            },
            // Changer de monstre, lancer un filet ou utiliser un soin passe
            // avant toute attaque, comme dans Pokémon, et laisse l'adversaire
            // agir.
            Action::Changer(index) => {
                self.actif_joueur = index;
                self.noter_participation();
                evenements.push(Evenement::MonstreEnvoye(self.actif(Cote::Joueur)));
                self.changer_d_adversaire(action_adversaire, &mut evenements);
                self.riposter(attaque_adversaire, false, aleatoire, &mut evenements);
            }
            Action::LancerFilet => {
                if !self.lancer_filet(aleatoire, &mut evenements) {
                    self.riposter(attaque_adversaire, false, aleatoire, &mut evenements);
                }
            }
            Action::UtiliserSoin(soin, index) => {
                self.soins.retirer(soin, 1);
                let effet = appliquer_soin(soin, &mut self.equipe[index], aleatoire);
                evenements.push(Evenement::SoinUtilise {
                    soin,
                    cible: Combattant {
                        cote: Cote::Joueur,
                        index,
                    },
                    effet,
                });
                self.riposter(attaque_adversaire, false, aleatoire, &mut evenements);
            }
            Action::Fuir => {
                self.riposter(attaque_adversaire, true, aleatoire, &mut evenements);
                evenements.push(Evenement::Fuite);
                self.terminer_avec(Issue::Fuite, &mut evenements);
                return Ok(evenements);
            }
        }
        self.finir_le_tour(&mut evenements);
        Ok(evenements)
    }

    /// Envoie un monstre à la place du monstre actif KO. Ce remplacement est
    /// gratuit : l'adversaire n'agit pas (Q25).
    pub fn remplacer(&mut self, index: usize) -> Result<Vec<Evenement>, ErreurCombat> {
        if self.phase != Phase::Remplacement {
            return Err(ErreurCombat::AucunRemplacementAttendu);
        }
        self.verifier_remplacant(index)?;
        self.actif_joueur = index;
        self.noter_participation();
        self.phase = Phase::ChoixAction;
        Ok(vec![Evenement::MonstreEnvoye(self.actif(Cote::Joueur))])
    }

    /// Monstres adverses KO, pas encore capturés, qu'il est possible de
    /// capturer après une victoire (Q29) ; un seul pour un humain (section
    /// 10.7), aucun dans un combat sans capture (section 14.2).
    pub fn adversaires_capturables(&self) -> Vec<usize> {
        if self.phase != Phase::Termine(Issue::Victoire)
            || self.sans_capture
            || (self.contre_un_humain && !self.captures.is_empty())
        {
            return Vec::new();
        }
        (0..self.adversaires.len())
            .filter(|index| self.adversaires[*index].est_ko() && !self.captures.contains(index))
            .collect()
    }

    /// Capture garantie d'un monstre adverse KO, après la victoire (section
    /// 11.2).
    pub fn capturer_ko(&mut self, index: usize) -> Result<Vec<Evenement>, ErreurCombat> {
        if !self.adversaires_capturables().contains(&index) {
            return Err(ErreurCombat::CaptureImpossible(index));
        }
        self.captures.push(index);
        Ok(vec![Evenement::CaptureReussie(Combattant {
            cote: Cote::Adversaire,
            index,
        })])
    }

    /// Verse l'XP des monstres vaincus si le combat est allé à son terme, quelle
    /// que soit son issue (Q32), puis rend l'équipe et les captures. Un combat
    /// interrompu ne rapporte rien.
    pub fn terminer(mut self, aleatoire: &mut dyn Aleatoire) -> BilanCombat {
        let issue = match self.phase {
            Phase::Termine(issue) => Some(issue),
            Phase::ChoixAction | Phase::Remplacement => None,
        };
        let experience = match issue {
            Some(_) => self.verser_experience(aleatoire),
            None => vec![None; self.equipe.len()],
        };
        let mut adversaires: Vec<Option<Monstre>> =
            self.adversaires.into_iter().map(Some).collect();
        let captures = self
            .captures
            .iter()
            .filter_map(|&index| adversaires[index].take())
            .collect();
        let sauvages = !self.contre_un_apotre && !self.contre_un_humain && !self.sans_capture;
        let vaincus_sauvages = adversaires
            .into_iter()
            .flatten()
            .filter(|adversaire| sauvages && adversaire.est_ko())
            .collect();
        BilanCombat {
            issue,
            equipe: self.equipe,
            captures,
            vaincus_sauvages,
            experience,
            soins: self.soins,
        }
    }

    /// Un monstre adverse KO ou capturé rapporte son XP, en entier, à chaque
    /// monstre de l'équipe qui l'a affronté et n'est pas KO (section 7) ;
    /// Instinct augmente le total du combat (section 5.9). Un tirage par
    /// monstre vaincu qui a au moins un bénéficiaire, dans l'ordre du
    /// groupe.
    fn verser_experience(
        &mut self,
        aleatoire: &mut dyn Aleatoire,
    ) -> Vec<Option<RapportExperience>> {
        let mut gains = vec![0u32; self.equipe.len()];
        for (index, vaincu) in self.adversaires.iter().enumerate() {
            if !vaincu.est_ko() && !self.captures.contains(&index) {
                continue;
            }
            let beneficiaires: Vec<usize> = self.participants[index]
                .iter()
                .copied()
                .filter(|&membre| !self.equipe[membre].est_ko())
                .collect();
            if beneficiaires.is_empty() {
                continue;
            }
            let gain = tirer_gain_experience(
                vaincu.espece().rendement_experience(),
                vaincu.niveau(),
                aleatoire,
            );
            for membre in beneficiaires {
                gains[membre] = gains[membre].saturating_add(gain);
            }
        }
        self.equipe
            .iter_mut()
            .zip(gains)
            .map(|(monstre, gain)| {
                let gain = if monstre.a_l_habilete(Habilete::Instinct) {
                    majorer(gain, BONUS_D_INSTINCT)
                } else {
                    gain
                };
                (gain > 0).then(|| monstre.gagner_experience(gain))
            })
            .collect()
    }

    /// Seule l'IA du hasard tire au sort ; les autres ne tirent rien
    /// (sections 10.1 et 10.7).
    fn choisir_action_adversaire(&self, aleatoire: &mut dyn Aleatoire) -> ActionAdversaire {
        let actif = self.actif(Cote::Adversaire);
        match self.ia {
            Ia::Hasard => ActionAdversaire::Attaquer(choisir_attaque_au_hasard(
                self.monstre(actif),
                aleatoire,
            )),
            Ia::PlusForte => ActionAdversaire::Attaquer(self.attaque_la_plus_forte(actif).0),
            Ia::PlusForteEtChangement => match self.changement_utile() {
                Some(index) => ActionAdversaire::Changer(index),
                None => ActionAdversaire::Attaquer(self.attaque_la_plus_forte(actif).0),
            },
            Ia::Avancee => ActionAdversaire::Attaquer(self.attaque_de_l_ia_avancee()),
        }
    }

    fn changer_d_adversaire(&mut self, action: ActionAdversaire, evenements: &mut Vec<Evenement>) {
        if let ActionAdversaire::Changer(index) = action {
            self.actif_adversaire = index;
            self.noter_participation();
            evenements.push(Evenement::MonstreEnvoye(self.actif(Cote::Adversaire)));
        }
    }

    /// L'adversaire attaque, s'il n'a pas changé de monstre.
    fn riposter(
        &mut self,
        attaque: Option<usize>,
        fuite: bool,
        aleatoire: &mut dyn Aleatoire,
        evenements: &mut Vec<Evenement>,
    ) {
        if let Some(attaque) = attaque {
            self.attaquer(Cote::Adversaire, attaque, fuite, aleatoire, evenements);
        }
    }

    /// Pour chaque attaque du monstre adverse, ses dégâts estimés contre le
    /// monstre actif du joueur.
    fn estimations_contre_le_joueur(&self, attaquant: Combattant) -> Vec<(u32, u32)> {
        let cible = self.actif(Cote::Joueur);
        let stats_attaquant = self.stats_en_combat(attaquant);
        let stats_cible = self.stats_en_combat(cible);
        self.monstre(attaquant)
            .attaques()
            .liste()
            .iter()
            .map(|attaque| {
                estimer_degats(
                    self.monstre(attaquant),
                    &stats_attaquant,
                    attaque,
                    self.monstre(cible),
                    &stats_cible,
                )
            })
            .collect()
    }

    /// IA simple d'un humain : l'attaque aux dégâts estimés les plus hauts
    /// contre le monstre actif du joueur, la première à égalité (section
    /// 10.7). Rend sa position et ce haut de fourchette.
    fn attaque_la_plus_forte(&self, attaquant: Combattant) -> (usize, u32) {
        let mut meilleure = (0, 0);
        for (position, (_, degats_max)) in self
            .estimations_contre_le_joueur(attaquant)
            .into_iter()
            .enumerate()
        {
            if degats_max > meilleure.1 {
                meilleure = (position, degats_max);
            }
        }
        meilleure
    }

    /// IA relevée : si le monstre actif n'a que des attaques peu efficaces ou
    /// sans effet contre le monstre actif du joueur, le monstre de l'équipe
    /// adverse dont la meilleure attaque fait plus de dégâts estimés que la
    /// sienne ; celui qui fait le plus, le premier à égalité (section 10.7).
    fn changement_utile(&self) -> Option<usize> {
        let actif = self.actif(Cote::Adversaire);
        let defenseur = self.monstre(self.actif(Cote::Joueur)).espece().types();
        let peu_efficace = self
            .monstre(actif)
            .attaques()
            .liste()
            .iter()
            .all(|attaque| {
                matches!(
                    efficacite(attaque.type_attaque(), defenseur),
                    Efficacite::Nulle | Efficacite::TresReduite | Efficacite::Reduite
                )
            });
        if !peu_efficace {
            return None;
        }
        let mut meilleur = (None, self.attaque_la_plus_forte(actif).1);
        for index in 0..self.adversaires.len() {
            let candidat = Combattant {
                cote: Cote::Adversaire,
                index,
            };
            if index == actif.index
                || self.adversaires[index].est_ko()
                || self.captures.contains(&index)
            {
                continue;
            }
            let degats = self.attaque_la_plus_forte(candidat).1;
            if degats > meilleur.1 {
                meilleur = (Some(index), degats);
            }
        }
        meilleur.0
    }

    /// IA avancée de l'Apôtre (section 10.7) : la plus précise des attaques
    /// qui mettent KO à coup sûr si elles touchent, la plus forte à précision
    /// égale ; sinon l'attaque aux dégâts moyens attendus les plus hauts,
    /// sauf qu'une attaque à statut que la cible peut subir et qui en vaut
    /// au moins 80 % lui est préférée.
    fn attaque_de_l_ia_avancee(&self) -> usize {
        let attaquant = self.actif(Cote::Adversaire);
        let cible = self.monstre(self.actif(Cote::Joueur));
        let attaques = self.monstre(attaquant).attaques().liste();
        let estimations = self.estimations_contre_le_joueur(attaquant);

        let mut achevement: Option<(usize, u8, u32)> = None;
        for (position, attaque) in attaques.iter().enumerate() {
            let (minimum, maximum) = estimations[position];
            if minimum == 0 || minimum < cible.pv_actuels() {
                continue;
            }
            let meilleure = match achevement {
                None => true,
                Some((_, precision, plus_forte)) => {
                    attaque.precision() > precision
                        || (attaque.precision() == precision && maximum > plus_forte)
                }
            };
            if meilleure {
                achevement = Some((position, attaque.precision(), maximum));
            }
        }
        if let Some((position, _, _)) = achevement {
            return position;
        }

        // Dégâts moyens attendus, multipliés par 200 pour rester entiers :
        // (minimum + maximum) x précision.
        let attendus: Vec<u64> = attaques
            .iter()
            .zip(&estimations)
            .map(|(attaque, (minimum, maximum))| {
                u64::from(minimum + maximum) * u64::from(attaque.precision())
            })
            .collect();
        let mut meilleure = 0;
        for (position, attendu) in attendus.iter().enumerate() {
            if *attendu > attendus[meilleure] {
                meilleure = position;
            }
        }
        let mut preferee: Option<usize> = None;
        for (position, attaque) in attaques.iter().enumerate() {
            let statut_possible = attaque
                .effet()
                .is_some_and(|effet| cible.peut_subir(effet.statut()));
            let presque_aussi_bonne = attendus[position] * 100 >= attendus[meilleure] * 80;
            let mieux_que_preferee =
                preferee.is_none_or(|autre| attendus[position] > attendus[autre]);
            if statut_possible
                && presque_aussi_bonne
                && attendus[position] > 0
                && mieux_que_preferee
            {
                preferee = Some(position);
            }
        }
        preferee.unwrap_or(meilleure)
    }

    /// Retient que le monstre actif du joueur affronte le monstre adverse
    /// actif. Un monstre KO, qui attend d'être remplacé, n'affronte personne.
    fn noter_participation(&mut self) {
        if self.equipe[self.actif_joueur].est_ko() {
            return;
        }
        let participants = &mut self.participants[self.actif_adversaire];
        if !participants.contains(&self.actif_joueur) {
            participants.push(self.actif_joueur);
        }
    }

    fn valider(&self, action: Action) -> Result<(), ErreurCombat> {
        match action {
            Action::Attaquer(index) => {
                let nombre = self
                    .monstre(self.actif(Cote::Joueur))
                    .attaques()
                    .liste()
                    .len();
                if index < nombre {
                    Ok(())
                } else {
                    Err(ErreurCombat::AttaqueInexistante(index))
                }
            }
            Action::Changer(index) => {
                if index == self.actif_joueur {
                    return Err(ErreurCombat::MonstreDejaActif(index));
                }
                self.verifier_remplacant(index)
            }
            Action::UtiliserSoin(soin, index) => {
                if self.soins.nombre(soin) == 0 {
                    return Err(ErreurCombat::AucunSoin(soin));
                }
                let Some(monstre) = self.equipe.get(index) else {
                    return Err(ErreurCombat::MonstreInexistant(index));
                };
                verifier_cible(soin, monstre).map_err(|erreur| match erreur {
                    ErreurSoin::MonstreKo => ErreurCombat::MonstreKo(index),
                    ErreurSoin::MonstreInexistant => ErreurCombat::MonstreInexistant(index),
                    ErreurSoin::Aucun(soin) => ErreurCombat::AucunSoin(soin),
                })
            }
            Action::Fuir if !self.fuite_possible() => Err(ErreurCombat::FuiteImpossible),
            Action::LancerFilet if !self.filet_possible() => Err(ErreurCombat::FiletImpossible),
            Action::LancerFilet | Action::Fuir => Ok(()),
        }
    }

    fn verifier_remplacant(&self, index: usize) -> Result<(), ErreurCombat> {
        match self.equipe.get(index) {
            None => Err(ErreurCombat::MonstreInexistant(index)),
            Some(monstre) if monstre.est_ko() => Err(ErreurCombat::MonstreKo(index)),
            Some(_) => Ok(()),
        }
    }

    /// Le monstre actif de ce côté n'est ni KO ni capturé.
    fn est_en_jeu(&self, cote: Cote) -> bool {
        let combattant = self.actif(cote);
        let capture = cote == Cote::Adversaire && self.captures.contains(&combattant.index);
        !capture && !self.monstre(combattant).est_ko()
    }

    fn monstre_mut(&mut self, combattant: Combattant) -> &mut Monstre {
        match combattant.cote {
            Cote::Joueur => &mut self.equipe[combattant.index],
            Cote::Adversaire => &mut self.adversaires[combattant.index],
        }
    }

    fn attaquant_et_defenseur(&mut self, cote: Cote) -> (&Monstre, &mut Monstre) {
        match cote {
            Cote::Joueur => (
                &self.equipe[self.actif_joueur],
                &mut self.adversaires[self.actif_adversaire],
            ),
            Cote::Adversaire => (
                &self.adversaires[self.actif_adversaire],
                &mut self.equipe[self.actif_joueur],
            ),
        }
    }

    /// Le plus rapide agit en premier ; à vitesse égale, tirage au sort à
    /// chaque tour (Q23).
    fn ordre_du_tour(&self, aleatoire: &mut dyn Aleatoire) -> [Cote; 2] {
        let vitesse_joueur = self.vitesse_en_combat(Cote::Joueur);
        let vitesse_adversaire = self.vitesse_en_combat(Cote::Adversaire);
        let joueur_en_premier = match vitesse_joueur.cmp(&vitesse_adversaire) {
            Ordering::Greater => true,
            Ordering::Less => false,
            Ordering::Equal => aleatoire.tirer(0..=1) == 0,
        };
        if joueur_en_premier {
            [Cote::Joueur, Cote::Adversaire]
        } else {
            [Cote::Adversaire, Cote::Joueur]
        }
    }

    /// `fuite` : l'attaque laisse au moins 1 PV à sa cible (Q13).
    fn attaquer(
        &mut self,
        cote: Cote,
        index_attaque: usize,
        fuite: bool,
        aleatoire: &mut dyn Aleatoire,
        evenements: &mut Vec<Evenement>,
    ) {
        // Un monstre mis KO ou capturé plus tôt dans le tour n'agit pas.
        if !self.est_en_jeu(cote) || !self.est_en_jeu(cote.oppose()) {
            return;
        }
        let lanceur = self.actif(cote);
        let cible = self.actif(cote.oppose());
        if self.monstre(lanceur).statut() == Some(Statut::Paralysie)
            && aleatoire.tirer(1..=CHANCES_BLOCAGE_PARALYSIE) == 1
        {
            evenements.push(Evenement::BloqueParLaParalysie(lanceur));
            return;
        }

        let stats_attaquant = self.stats_en_combat(lanceur);
        let stats_defenseur = self.stats_en_combat(cible);
        let (attaquant, defenseur) = self.attaquant_et_defenseur(cote);
        let attaque = attaquant.attaques().liste()[index_attaque].clone();
        let resultat = resoudre_attaque(
            attaquant,
            &stats_attaquant,
            &attaque,
            defenseur,
            &stats_defenseur,
            aleatoire,
        );
        let effet = attaque.effet();
        evenements.push(Evenement::AttaqueLancee { lanceur, attaque });
        match resultat {
            ResultatAttaque::Ratee => evenements.push(Evenement::AttaqueRatee(lanceur)),
            ResultatAttaque::SansEffet => evenements.push(Evenement::SansEffet(cible)),
            ResultatAttaque::Touchee {
                degats,
                critique,
                efficacite,
            } => {
                let pv_cedables = if fuite {
                    defenseur.pv_actuels().saturating_sub(1)
                } else {
                    defenseur.pv_actuels()
                };
                let degats = degats.min(pv_cedables);
                defenseur.subir_degats(degats);
                evenements.push(Evenement::Degats {
                    cible,
                    degats,
                    critique,
                    efficacite,
                });
                if defenseur.est_ko() {
                    evenements.push(Evenement::Ko(cible));
                } else if let Some(effet) = effet {
                    if tenter_effet(defenseur, effet, aleatoire) {
                        evenements.push(Evenement::StatutInflige {
                            cible,
                            statut: effet.statut(),
                        });
                    }
                }
            }
        }
    }

    /// Rend `true` si le monstre adverse actif est capturé.
    fn lancer_filet(
        &mut self,
        aleatoire: &mut dyn Aleatoire,
        evenements: &mut Vec<Evenement>,
    ) -> bool {
        let cible = self.actif(Cote::Adversaire);
        let chance = self.chance_de_capture();
        evenements.push(Evenement::FiletLance { cible, chance });
        let reussie = match chance {
            0 => false,
            100.. => true,
            _ => aleatoire.tirer(1..=100) <= chance,
        };
        if reussie {
            self.captures.push(cible.index);
            evenements.push(Evenement::CaptureReussie(cible));
        } else {
            evenements.push(Evenement::CaptureRatee(cible));
        }
        reussie
    }

    /// Poison des monstres actifs, puis remplacement des monstres KO ou
    /// capturés, ou fin du combat.
    fn finir_le_tour(&mut self, evenements: &mut Vec<Evenement>) {
        for cote in [Cote::Joueur, Cote::Adversaire] {
            if !self.est_en_jeu(cote) {
                continue;
            }
            let combattant = self.actif(cote);
            let monstre = self.monstre_mut(combattant);
            if monstre.statut() != Some(Statut::Poison) {
                continue;
            }
            let degats = degats_de_poison(monstre.stats().pv).min(monstre.pv_actuels());
            monstre.subir_degats(degats);
            let ko = monstre.est_ko();
            evenements.push(Evenement::DegatsDePoison {
                cible: combattant,
                degats,
            });
            if ko {
                evenements.push(Evenement::Ko(combattant));
            }
        }

        // Une équipe entièrement KO perd, même si le dernier adversaire tombe
        // au même tour (Q50, section 9.3).
        if self.equipe.iter().all(Monstre::est_ko) {
            self.terminer_avec(Issue::Defaite, evenements);
            return;
        }
        if !self.est_en_jeu(Cote::Adversaire) {
            match self.prochain_adversaire() {
                Some(index) => {
                    self.actif_adversaire = index;
                    self.noter_participation();
                    evenements.push(Evenement::MonstreEnvoye(self.actif(Cote::Adversaire)));
                }
                None => {
                    self.terminer_avec(Issue::Victoire, evenements);
                    return;
                }
            }
        }
        if !self.est_en_jeu(Cote::Joueur) {
            self.phase = Phase::Remplacement;
        }
    }

    /// Les monstres du groupe entrent dans l'ordre de la rencontre (Q49).
    fn prochain_adversaire(&self) -> Option<usize> {
        (0..self.adversaires.len())
            .find(|index| !self.captures.contains(index) && !self.adversaires[*index].est_ko())
    }

    fn terminer_avec(&mut self, issue: Issue, evenements: &mut Vec<Evenement>) {
        self.phase = Phase::Termine(issue);
        evenements.push(Evenement::FinDuCombat(issue));
    }

    /// La paralysie divise la vitesse par 2 (section 10.5), après le bonus.
    fn vitesse_en_combat(&self, cote: Cote) -> u32 {
        let combattant = self.actif(cote);
        let vitesse = self.stats_en_combat(combattant).vitesse;
        if self.monstre(combattant).statut() == Some(Statut::Paralysie) {
            vitesse / 2
        } else {
            vitesse
        }
    }
}

/// Aucun tirage si la cible ne peut pas subir le statut ou si la chance est
/// de 100 %. Rend `true` si le statut est infligé.
fn tenter_effet(
    cible: &mut Monstre,
    effet: EffetSecondaire,
    aleatoire: &mut dyn Aleatoire,
) -> bool {
    if !cible.peut_subir(effet.statut()) {
        return false;
    }
    let touche = effet.chance() >= EffetSecondaire::CHANCE_MAX
        || aleatoire.tirer(1..=100) <= u32::from(effet.chance());
    touche && cible.infliger_statut(effet.statut())
}

/// Façon dont l'adversaire choisit son action (sections 10.1 et 10.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ia {
    /// Monstres sauvages : une attaque au hasard (Q24).
    Hasard,
    /// Humain ordinaire.
    PlusForte,
    /// Humain qui termine la deuxième zone.
    PlusForteEtChangement,
    /// Apôtre.
    Avancee,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActionAdversaire {
    Attaquer(usize),
    /// Envoie ce monstre de son équipe, à la place de l'actif.
    Changer(usize),
}

/// IA des monstres sauvages de la V1 : une attaque au hasard (Q24).
fn choisir_attaque_au_hasard(monstre: &Monstre, aleatoire: &mut dyn Aleatoire) -> usize {
    let nombre = monstre.attaques().liste().len();
    if nombre == 1 {
        0
    } else {
        aleatoire.tirer(0..=(nombre - 1) as u32) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::attaque::Categorie;
    use crate::donnees_de_test::{attaque, monstre, stats_uniformes};
    use crate::humain::IaHumain;
    use crate::niveau::Niveau;
    use crate::stats::Stats;
    use crate::types::Type;

    // Base 50 au niveau 5 avec des valeurs internes nulles : 20 PV et 10 dans
    // les autres stats. Une base de vitesse de 60 donne 11. Le coup inflige
    // alors 5 dégâts à un monstre Feu avec un pourcentage de 100, sans
    // critique : chaque attaque qui touche consomme les tirages [2, 100].

    fn coup() -> Attaque {
        attaque("coup", Type::Normal, Categorie::Physique, 40, 100)
    }

    fn dard_empoisonne() -> Attaque {
        attaque("dard", Type::Poison, Categorie::Physique, 40, 100)
            .avec_effet(EffetSecondaire::new(Statut::Poison, 30).unwrap())
    }

    fn rapide() -> Monstre {
        let base = Stats {
            vitesse: 60,
            ..stats_uniformes(50)
        };
        monstre(&[Type::Feu], base, 5, vec![coup()])
    }

    fn lent() -> Monstre {
        monstre(&[Type::Feu], stats_uniformes(50), 5, vec![coup()])
    }

    /// Base de vitesse 70 : 12 au niveau 5, plus rapide que `rapide`.
    fn tres_rapide() -> Monstre {
        let base = Stats {
            vitesse: 70,
            ..stats_uniformes(50)
        };
        monstre(&[Type::Feu], base, 5, vec![coup()])
    }

    fn terminer(combat: Combat, tirages: &[u32]) -> BilanCombat {
        combat.terminer(&mut AleatoireScripte::new(tirages.iter().copied()))
    }

    fn xp_gagnee(bilan: &BilanCombat, membre: usize) -> Option<u32> {
        bilan.experience[membre]
            .as_ref()
            .map(|rapport| rapport.experience_gagnee)
    }

    fn affaibli(mut monstre: Monstre, pv_restants: u32) -> Monstre {
        monstre.subir_degats(monstre.stats().pv - pv_restants);
        monstre
    }

    fn paralyse(mut monstre: Monstre) -> Monstre {
        monstre.infliger_statut(Statut::Paralysie);
        monstre
    }

    fn combat(equipe: Vec<Monstre>, adversaires: Vec<Monstre>) -> Combat {
        Combat::new(equipe, adversaires).unwrap()
    }

    fn tour(combat: &mut Combat, action: Action, tirages: &[u32]) -> Vec<Evenement> {
        let mut aleatoire = AleatoireScripte::new(tirages.iter().copied());
        combat.jouer_tour(action, &mut aleatoire).unwrap()
    }

    fn j(index: usize) -> Combattant {
        Combattant {
            cote: Cote::Joueur,
            index,
        }
    }

    fn a(index: usize) -> Combattant {
        Combattant {
            cote: Cote::Adversaire,
            index,
        }
    }

    fn lancee(lanceur: Combattant, attaque: Attaque) -> Evenement {
        Evenement::AttaqueLancee { lanceur, attaque }
    }

    fn degats(cible: Combattant, degats: u32) -> Evenement {
        Evenement::Degats {
            cible,
            degats,
            critique: false,
            efficacite: Efficacite::Normale,
        }
    }

    #[test]
    fn apercu_des_degats_sans_coup_critique() {
        let braise = attaque("braise", Type::Feu, Categorie::Speciale, 40, 100);
        let attaquant = monstre(&[Type::Feu], stats_uniformes(50), 5, vec![coup(), braise]);
        let contre_le_feu = combat(vec![attaquant], vec![lent()]);
        // 5 dégâts à 100 %, 4 à 85 %.
        assert_eq!(
            contre_le_feu.apercu_attaque(0),
            Some(ApercuAttaque {
                degats_min: 4,
                degats_max: 5,
                efficacite: Efficacite::Normale,
                bonus_de_type: false,
            })
        );
        // Bonus de type, puis la résistance du Feu au Feu.
        assert_eq!(
            contre_le_feu.apercu_attaque(1),
            Some(ApercuAttaque {
                degats_min: 3,
                degats_max: 3,
                efficacite: Efficacite::Reduite,
                bonus_de_type: true,
            })
        );
        assert_eq!(contre_le_feu.apercu_attaque(2), None);
        let spectre = monstre(&[Type::Spectre], stats_uniformes(50), 5, vec![coup()]);
        let contre_un_spectre = combat(vec![lent()], vec![spectre]);
        let apercu = contre_un_spectre.apercu_attaque(0).unwrap();
        assert_eq!(
            (apercu.degats_min, apercu.degats_max, apercu.efficacite),
            (0, 0, Efficacite::Nulle)
        );
    }

    #[test]
    fn le_plus_rapide_agit_en_premier() {
        let mut combat = combat(vec![rapide()], vec![lent()]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100, 2, 100]);
        assert_eq!(
            evenements,
            vec![
                lancee(j(0), coup()),
                degats(a(0), 5),
                lancee(a(0), coup()),
                degats(j(0), 5),
            ]
        );
        assert_eq!(combat.phase(), Phase::ChoixAction);
    }

    #[test]
    fn egalite_de_vitesse_tiree_au_sort() {
        let mut combat = combat(vec![lent()], vec![lent()]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[1, 2, 100, 2, 100]);
        assert_eq!(evenements[0], lancee(a(0), coup()));
    }

    #[test]
    fn ia_choisit_son_attaque_au_hasard() {
        let morsure = attaque("morsure", Type::Tenebres, Categorie::Physique, 60, 100);
        let adversaire = monstre(
            &[Type::Feu],
            stats_uniformes(50),
            5,
            vec![coup(), morsure.clone()],
        );
        let mut combat = combat(vec![rapide()], vec![adversaire]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[1, 2, 100, 2, 100]);
        assert_eq!(evenements[2], lancee(a(0), morsure));
    }

    #[test]
    fn cible_immunisee_sans_tirage() {
        let spectre = monstre(&[Type::Spectre], stats_uniformes(50), 5, vec![coup()]);
        let mut combat = combat(vec![rapide()], vec![spectre]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(
            evenements[..2],
            [lancee(j(0), coup()), Evenement::SansEffet(a(0))]
        );
    }

    #[test]
    fn la_paralysie_divise_la_vitesse_par_deux() {
        // 11 de vitesse divisés par 2 : le monstre lent agit en premier.
        let mut combat = combat(vec![paralyse(rapide())], vec![lent()]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100, 2, 2, 100]);
        assert_eq!(evenements[0], lancee(a(0), coup()));
        assert_eq!(evenements[2], lancee(j(0), coup()));
    }

    #[test]
    fn la_paralysie_peut_empecher_d_attaquer() {
        let mut combat = combat(vec![rapide()], vec![paralyse(lent())]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100, 1]);
        assert_eq!(
            evenements,
            vec![
                lancee(j(0), coup()),
                degats(a(0), 5),
                Evenement::BloqueParLaParalysie(a(0)),
            ]
        );
    }

    #[test]
    fn effet_secondaire_inflige_un_statut_et_le_poison_agit_en_fin_de_tour() {
        let joueur = monstre(
            &[Type::Feu],
            Stats {
                vitesse: 60,
                ..stats_uniformes(50)
            },
            5,
            vec![dard_empoisonne()],
        );
        let mut combat = combat(vec![joueur], vec![lent()]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100, 30, 2, 100]);
        assert_eq!(
            evenements,
            vec![
                lancee(j(0), dard_empoisonne()),
                degats(a(0), 5),
                Evenement::StatutInflige {
                    cible: a(0),
                    statut: Statut::Poison,
                },
                lancee(a(0), coup()),
                degats(j(0), 5),
                Evenement::DegatsDePoison {
                    cible: a(0),
                    degats: 2,
                },
            ]
        );
        assert_eq!(combat.adversaires()[0].pv_actuels(), 13);
    }

    #[test]
    fn effet_secondaire_peut_echouer() {
        let joueur = monstre(
            &[Type::Feu],
            Stats {
                vitesse: 60,
                ..stats_uniformes(50)
            },
            5,
            vec![dard_empoisonne()],
        );
        let mut combat = combat(vec![joueur], vec![lent()]);
        tour(&mut combat, Action::Attaquer(0), &[2, 100, 31, 2, 100]);
        assert_eq!(combat.adversaires()[0].statut(), None);
    }

    #[test]
    fn aucun_tirage_d_effet_sur_une_cible_immunisee_au_statut() {
        let joueur = monstre(
            &[Type::Feu],
            Stats {
                vitesse: 60,
                ..stats_uniformes(50)
            },
            5,
            vec![dard_empoisonne()],
        );
        let vipere = monstre(&[Type::Poison], stats_uniformes(50), 5, vec![coup()]);
        let mut combat = combat(vec![joueur], vec![vipere]);
        // Pas de tirage d'effet entre les deux attaques.
        tour(&mut combat, Action::Attaquer(0), &[2, 100, 2, 100]);
        assert_eq!(combat.adversaires()[0].statut(), None);
    }

    #[test]
    fn le_poison_peut_donner_la_victoire() {
        let mut spectre = affaibli(
            monstre(&[Type::Spectre], stats_uniformes(50), 5, vec![coup()]),
            2,
        );
        spectre.infliger_statut(Statut::Poison);
        let mut combat = combat(vec![rapide()], vec![spectre]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(
            evenements[evenements.len() - 3..],
            [
                Evenement::DegatsDePoison {
                    cible: a(0),
                    degats: 2,
                },
                Evenement::Ko(a(0)),
                Evenement::FinDuCombat(Issue::Victoire),
            ]
        );
    }

    #[test]
    fn changer_de_monstre_coute_le_tour() {
        let mut combat = combat(vec![rapide(), lent()], vec![lent()]);
        let evenements = tour(&mut combat, Action::Changer(1), &[2, 100]);
        assert_eq!(
            evenements,
            vec![
                Evenement::MonstreEnvoye(j(1)),
                lancee(a(0), coup()),
                degats(j(1), 5),
            ]
        );
        assert_eq!(combat.equipe()[0].pv_actuels(), 20);
        assert_eq!(combat.actif(Cote::Joueur), j(1));
    }

    #[test]
    fn changements_refuses_sans_rien_changer() {
        let mut combat = combat(vec![rapide(), affaibli(lent(), 0)], vec![lent()]);
        let avant = combat.clone();
        let mut aleatoire = AleatoireScripte::new([]);
        for (action, erreur) in [
            (Action::Changer(0), ErreurCombat::MonstreDejaActif(0)),
            (Action::Changer(1), ErreurCombat::MonstreKo(1)),
            (Action::Changer(2), ErreurCombat::MonstreInexistant(2)),
            (Action::Attaquer(1), ErreurCombat::AttaqueInexistante(1)),
        ] {
            assert_eq!(combat.jouer_tour(action, &mut aleatoire), Err(erreur));
        }
        assert_eq!(combat, avant);
    }

    #[test]
    fn remplacement_gratuit_apres_un_ko() {
        let mut combat = combat(vec![affaibli(lent(), 3), lent()], vec![rapide()]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(
            evenements,
            vec![lancee(a(0), coup()), degats(j(0), 3), Evenement::Ko(j(0))]
        );
        assert_eq!(combat.phase(), Phase::Remplacement);
        assert_eq!(
            combat.jouer_tour(Action::Attaquer(0), &mut AleatoireScripte::new([])),
            Err(ErreurCombat::RemplacementAttendu)
        );
        assert_eq!(combat.remplacer(0), Err(ErreurCombat::MonstreKo(0)));
        assert_eq!(
            combat.remplacer(1),
            Ok(vec![Evenement::MonstreEnvoye(j(1))])
        );
        assert_eq!(combat.phase(), Phase::ChoixAction);
        assert_eq!(
            combat.remplacer(1),
            Err(ErreurCombat::AucunRemplacementAttendu)
        );
    }

    #[test]
    fn defaite_quand_toute_l_equipe_est_ko() {
        let mut combat = combat(vec![affaibli(lent(), 3)], vec![rapide()]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(
            evenements.last(),
            Some(&Evenement::FinDuCombat(Issue::Defaite))
        );
        assert_eq!(combat.phase(), Phase::Termine(Issue::Defaite));
        assert_eq!(
            combat.jouer_tour(Action::Attaquer(0), &mut AleatoireScripte::new([])),
            Err(ErreurCombat::CombatTermine)
        );
    }

    #[test]
    fn le_groupe_sauvage_entre_dans_l_ordre() {
        let mut combat = combat(vec![rapide()], vec![affaibli(lent(), 3), lent()]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(
            evenements,
            vec![
                lancee(j(0), coup()),
                degats(a(0), 3),
                Evenement::Ko(a(0)),
                Evenement::MonstreEnvoye(a(1)),
            ]
        );
        assert_eq!(combat.actif(Cote::Adversaire), a(1));
        assert_eq!(combat.phase(), Phase::ChoixAction);
    }

    #[test]
    fn victoire_quand_tout_le_groupe_est_ko() {
        let mut combat = combat(vec![rapide()], vec![affaibli(lent(), 3)]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(
            evenements.last(),
            Some(&Evenement::FinDuCombat(Issue::Victoire))
        );
    }

    #[test]
    fn la_fuite_laisse_au_moins_un_pv() {
        let mut combat = combat(vec![affaibli(lent(), 3)], vec![rapide()]);
        let evenements = tour(&mut combat, Action::Fuir, &[2, 100]);
        assert_eq!(
            evenements,
            vec![
                lancee(a(0), coup()),
                degats(j(0), 2),
                Evenement::Fuite,
                Evenement::FinDuCombat(Issue::Fuite),
            ]
        );
        assert_eq!(combat.equipe()[0].pv_actuels(), 1);
        assert!(combat.adversaires_capturables().is_empty());
    }

    #[test]
    fn ni_fuite_ni_filet_face_a_un_apotre() {
        let mut combat = combat(vec![rapide()], vec![lent()]).contre_un_apotre();
        assert!(!combat.fuite_possible());
        assert!(!combat.filet_possible());
        assert_eq!(
            combat.jouer_tour(Action::Fuir, &mut AleatoireScripte::new([])),
            Err(ErreurCombat::FuiteImpossible)
        );
        assert_eq!(
            combat.jouer_tour(Action::LancerFilet, &mut AleatoireScripte::new([])),
            Err(ErreurCombat::FiletImpossible)
        );
        assert_eq!(combat.phase(), Phase::ChoixAction);
    }

    #[test]
    fn ni_fuite_ni_filet_face_a_un_humain() {
        let mut combat = combat(vec![rapide()], vec![lent()]).contre_un_humain(IaHumain::Simple);
        assert!(!combat.fuite_possible() && !combat.filet_possible());
        assert_eq!(
            combat.jouer_tour(Action::Fuir, &mut AleatoireScripte::new([])),
            Err(ErreurCombat::FuiteImpossible)
        );
        assert_eq!(
            combat.jouer_tour(Action::LancerFilet, &mut AleatoireScripte::new([])),
            Err(ErreurCombat::FiletImpossible)
        );
    }

    #[test]
    fn l_ia_d_un_humain_choisit_l_attaque_la_plus_forte_sans_tirage() {
        let braise = attaque("braise", Type::Feu, Categorie::Speciale, 40, 100);
        let base_rapide = Stats {
            vitesse: 60,
            ..stats_uniformes(50)
        };
        let plante = monstre(&[Type::Plante], base_rapide, 5, vec![coup()]);
        let adversaire = monstre(
            &[Type::Feu],
            stats_uniformes(50),
            5,
            vec![coup(), braise.clone()],
        );
        let mut combat = combat(vec![plante], vec![adversaire]).contre_un_humain(IaHumain::Simple);
        // Braise, très efficace contre la Plante, l'emporte sur le coup.
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100, 2, 100]);
        assert_eq!(evenements[2], lancee(a(0), braise));
    }

    /// Un joueur Eau, rapide, face à un monstre Feu qui n'a que Braise, peu
    /// efficace, et un monstre Plante dont le Fouet est très efficace.
    fn combat_contre_un_humain(ia: IaHumain) -> Combat {
        let base_rapide = Stats {
            vitesse: 60,
            ..stats_uniformes(50)
        };
        let eau = monstre(&[Type::Eau], base_rapide, 5, vec![coup()]);
        let braise = attaque("braise", Type::Feu, Categorie::Speciale, 40, 100);
        let fouet = attaque("fouet", Type::Plante, Categorie::Physique, 40, 100);
        let feu = monstre(&[Type::Feu], stats_uniformes(50), 5, vec![braise]);
        let plante = monstre(&[Type::Plante], stats_uniformes(50), 5, vec![fouet]);
        combat(vec![eau], vec![feu, plante]).contre_un_humain(ia)
    }

    #[test]
    fn l_ia_relevee_change_de_monstre_quand_le_sien_blesse_peu() {
        let mut combat = combat_contre_un_humain(IaHumain::Relevee);
        // Le changement passe avant l'attaque du joueur, qui touche le
        // nouveau venu ; l'adversaire n'attaque pas, et aucun tirage ne
        // départage l'ordre du tour.
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(evenements[0], Evenement::MonstreEnvoye(a(1)));
        assert_eq!(evenements[1], lancee(j(0), coup()));
        assert!(
            !evenements
                .iter()
                .any(|evenement| matches!(evenement, Evenement::AttaqueLancee { lanceur, .. } if lanceur.cote == Cote::Adversaire))
        );
        assert_eq!(combat.actif(Cote::Adversaire), a(1));
    }

    #[test]
    fn l_ia_simple_ne_change_pas_de_monstre() {
        let mut combat = combat_contre_un_humain(IaHumain::Simple);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100, 2, 100]);
        assert!(matches!(
            evenements[2],
            Evenement::AttaqueLancee { lanceur, .. } if lanceur == a(0)
        ));
    }

    /// Normal, puissance 120, précision 50 : 9 à 11 dégâts au niveau 5.
    fn gros_coup() -> Attaque {
        attaque("gros-coup", Type::Normal, Categorie::Physique, 120, 50)
    }

    /// Comme le coup, avec 20 % de paralysie : 4 à 5 dégâts.
    fn cri() -> Attaque {
        attaque("cri", Type::Normal, Categorie::Speciale, 40, 100)
            .avec_effet(EffetSecondaire::new(Statut::Paralysie, 20).unwrap())
    }

    fn contre_un_apotre(joueur: Monstre, attaques: Vec<Attaque>) -> Combat {
        let apotre = monstre(&[Type::Feu], stats_uniformes(50), 5, attaques);
        combat(vec![joueur], vec![apotre]).contre_un_apotre()
    }

    #[test]
    fn l_ia_avancee_acheve_avec_l_attaque_la_plus_precise() {
        let mut combat = contre_un_apotre(affaibli(rapide(), 3), vec![gros_coup(), coup()]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100, 2, 100]);
        assert_eq!(evenements[2], lancee(a(0), coup()));
    }

    #[test]
    fn l_ia_avancee_vise_les_degats_moyens_attendus() {
        // Gros coup : (9 + 11) x 50 = 1000 ; coup : (4 + 5) x 100 = 900.
        let mut combat = contre_un_apotre(rapide(), vec![coup(), gros_coup()]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100, 50, 2, 100]);
        assert_eq!(evenements[2], lancee(a(0), gros_coup()));
    }

    #[test]
    fn l_ia_avancee_prefere_un_statut_presque_aussi_bon() {
        // Le cri vaut 90 % du gros coup et la cible peut être paralysée.
        let mut combat = contre_un_apotre(rapide(), vec![coup(), gros_coup(), cri()]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100, 2, 100, 100]);
        assert_eq!(evenements[2], lancee(a(0), cri()));
        // Déjà paralysée, elle ne peut plus l'être : le gros coup revient.
        // Le joueur, ralenti, agit en second.
        let mut combat = contre_un_apotre(paralyse(rapide()), vec![coup(), gros_coup(), cri()]);
        let evenements = tour(&mut combat, Action::Attaquer(0), &[50, 2, 100, 2, 2, 100]);
        assert_eq!(evenements[0], lancee(a(0), gros_coup()));
    }

    #[test]
    fn un_seul_monstre_d_un_humain_se_prend_apres_la_victoire() {
        let mut combat = combat(
            vec![rapide()],
            vec![affaibli(lent(), 1), affaibli(lent(), 1)],
        )
        .contre_un_humain(IaHumain::Simple);
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(combat.adversaires_capturables(), vec![0, 1]);
        combat.capturer_ko(1).unwrap();
        assert!(combat.adversaires_capturables().is_empty());
        assert_eq!(
            combat.capturer_ko(0),
            Err(ErreurCombat::CaptureImpossible(0))
        );
    }

    #[test]
    fn sans_capture_la_fuite_reste_possible_mais_ni_filet_ni_capture() {
        let mut combat = combat(vec![rapide()], vec![affaibli(lent(), 1)]).sans_capture();
        assert!(combat.fuite_possible());
        assert_eq!(
            combat.jouer_tour(Action::LancerFilet, &mut AleatoireScripte::new([])),
            Err(ErreurCombat::FiletImpossible)
        );
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(combat.phase(), Phase::Termine(Issue::Victoire));
        assert!(combat.adversaires_capturables().is_empty());
        // Un mort-vivant ne lâche pas de ressource (section 14.3).
        assert!(terminer(combat, &[100]).vaincus_sauvages.is_empty());
    }

    #[test]
    fn un_sauvage_ko_et_non_capture_peut_lacher_sa_ressource() {
        let mut combat = combat(vec![rapide()], vec![affaibli(lent(), 1)]);
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        let bilan = terminer(combat, &[100]);
        assert_eq!(bilan.vaincus_sauvages.len(), 1);
        assert!(bilan.captures.is_empty());
    }

    #[test]
    fn l_apotre_ko_se_capture_apres_la_victoire() {
        let mut combat = combat(vec![rapide()], vec![affaibli(lent(), 3)]).contre_un_apotre();
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(combat.adversaires_capturables(), vec![0]);
        combat.capturer_ko(0).unwrap();
        let bilan = terminer(combat, &[100]);
        assert_eq!(bilan.captures.len(), 1);
    }

    #[test]
    fn filet_sans_chance_puis_riposte() {
        let mut combat = combat(vec![rapide()], vec![lent()]);
        let evenements = tour(&mut combat, Action::LancerFilet, &[2, 100]);
        assert_eq!(
            evenements,
            vec![
                Evenement::FiletLance {
                    cible: a(0),
                    chance: 0,
                },
                Evenement::CaptureRatee(a(0)),
                lancee(a(0), coup()),
                degats(j(0), 5),
            ]
        );
    }

    #[test]
    fn capture_ratee_puis_riposte() {
        let mut combat = combat(vec![rapide()], vec![affaibli(lent(), 2)]);
        let evenements = tour(&mut combat, Action::LancerFilet, &[91, 2, 100]);
        assert_eq!(evenements[1], Evenement::CaptureRatee(a(0)));
        assert_eq!(evenements[2], lancee(a(0), coup()));
    }

    #[test]
    fn capture_du_dernier_sauvage_donne_la_victoire() {
        let mut combat = combat(vec![rapide()], vec![affaibli(lent(), 2)]);
        let evenements = tour(&mut combat, Action::LancerFilet, &[90]);
        assert_eq!(
            evenements,
            vec![
                Evenement::FiletLance {
                    cible: a(0),
                    chance: 90,
                },
                Evenement::CaptureReussie(a(0)),
                Evenement::FinDuCombat(Issue::Victoire),
            ]
        );
        assert!(combat.adversaires_capturables().is_empty());
        let bilan = terminer(combat, &[100]);
        assert_eq!(xp_gagnee(&bilan, 0), Some(50));
        assert_eq!(bilan.issue, Some(Issue::Victoire));
        let pv_des_captures: Vec<u32> = bilan.captures.iter().map(Monstre::pv_actuels).collect();
        assert_eq!(pv_des_captures, vec![2]);
    }

    #[test]
    fn plusieurs_monstres_captures_au_filet_dans_un_combat() {
        let mut combat = combat(
            vec![rapide()],
            vec![affaibli(lent(), 2), affaibli(lent(), 2)],
        );
        let evenements = tour(&mut combat, Action::LancerFilet, &[90]);
        assert_eq!(evenements.last(), Some(&Evenement::MonstreEnvoye(a(1))));
        let evenements = tour(&mut combat, Action::LancerFilet, &[90]);
        assert_eq!(
            evenements.last(),
            Some(&Evenement::FinDuCombat(Issue::Victoire))
        );
        assert_eq!(terminer(combat, &[100, 100]).captures.len(), 2);
    }

    #[test]
    fn capture_de_plusieurs_monstres_ko_apres_la_victoire() {
        let mut combat = combat(
            vec![rapide()],
            vec![affaibli(lent(), 3), affaibli(lent(), 3)],
        );
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(combat.phase(), Phase::Termine(Issue::Victoire));
        assert_eq!(combat.adversaires_capturables(), vec![0, 1]);
        assert_eq!(
            combat.capturer_ko(1),
            Ok(vec![Evenement::CaptureReussie(a(1))])
        );
        assert_eq!(combat.adversaires_capturables(), vec![0]);
        assert_eq!(
            combat.capturer_ko(0),
            Ok(vec![Evenement::CaptureReussie(a(0))])
        );
        assert_eq!(
            combat.capturer_ko(0),
            Err(ErreurCombat::CaptureImpossible(0))
        );
        let captures = terminer(combat, &[100, 100]).captures;
        assert_eq!(captures.len(), 2);
        assert!(captures.iter().all(Monstre::est_ko));
    }

    #[test]
    fn capture_au_filet_puis_capture_d_un_ko() {
        let mut combat = combat(
            vec![rapide()],
            vec![affaibli(lent(), 2), affaibli(lent(), 3)],
        );
        tour(&mut combat, Action::LancerFilet, &[90]);
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(combat.phase(), Phase::Termine(Issue::Victoire));
        assert_eq!(combat.adversaires_capturables(), vec![1]);
        combat.capturer_ko(1).unwrap();
        let pv_des_captures: Vec<u32> = terminer(combat, &[100, 100])
            .captures
            .iter()
            .map(Monstre::pv_actuels)
            .collect();
        assert_eq!(pv_des_captures, vec![2, 0]);
    }

    #[test]
    fn creation_refusee() {
        assert_eq!(
            Combat::new(vec![], vec![lent()]),
            Err(ErreurCombat::EquipeSansMonstreApte)
        );
        assert_eq!(
            Combat::new(vec![affaibli(lent(), 0)], vec![lent()]),
            Err(ErreurCombat::EquipeSansMonstreApte)
        );
        assert_eq!(
            Combat::new(vec![lent()], vec![]),
            Err(ErreurCombat::AucunAdversaire)
        );
        assert_eq!(
            Combat::new(vec![lent()], vec![lent(), affaibli(lent(), 0)]),
            Err(ErreurCombat::AdversaireKo(1))
        );
    }

    #[test]
    fn le_premier_monstre_apte_commence() {
        let combat = combat(vec![affaibli(lent(), 0), rapide()], vec![lent()]);
        assert_eq!(combat.actif(Cote::Joueur), j(1));
    }

    #[test]
    fn terminer_un_combat_interrompu_rend_l_equipe() {
        let mut combat = combat(vec![rapide()], vec![lent()]);
        tour(&mut combat, Action::Attaquer(0), &[2, 100, 2, 85]);
        // Aucun tirage d'XP : un combat interrompu ne rapporte rien.
        let bilan = terminer(combat, &[]);
        assert_eq!(bilan.issue, None);
        assert_eq!(bilan.equipe[0].pv_actuels(), 16);
        assert!(bilan.captures.is_empty());
        assert_eq!(bilan.experience, vec![None]);
    }

    #[test]
    fn xp_versee_en_entier_a_chaque_participant() {
        let mut combat = combat(vec![lent(), rapide()], vec![affaibli(lent(), 3)]);
        tour(&mut combat, Action::Changer(1), &[2, 100]);
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        // Rendement de test 70 au niveau 5 : 50 XP avec un pourcentage de 100.
        let bilan = terminer(combat, &[100]);
        assert_eq!(xp_gagnee(&bilan, 0), Some(50));
        assert_eq!(xp_gagnee(&bilan, 1), Some(50));
        assert_eq!(bilan.equipe[0].experience(), 175);
    }

    #[test]
    fn un_participant_ko_ne_recoit_pas_d_xp() {
        let mut combat = combat(
            vec![affaibli(lent(), 3), rapide()],
            vec![affaibli(tres_rapide(), 3)],
        );
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        combat.remplacer(1).unwrap();
        tour(&mut combat, Action::Attaquer(0), &[2, 100, 2, 100]);
        assert_eq!(combat.phase(), Phase::Termine(Issue::Victoire));
        let bilan = terminer(combat, &[100]);
        assert_eq!(xp_gagnee(&bilan, 0), None);
        assert_eq!(xp_gagnee(&bilan, 1), Some(50));
    }

    #[test]
    fn xp_gagnee_meme_apres_une_fuite() {
        let mut combat = combat(vec![rapide()], vec![affaibli(lent(), 3), lent()]);
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        tour(&mut combat, Action::Fuir, &[2, 100]);
        // Seul le premier monstre a été vaincu : un seul tirage.
        let bilan = terminer(combat, &[110]);
        assert_eq!(bilan.issue, Some(Issue::Fuite));
        assert_eq!(xp_gagnee(&bilan, 0), Some(55));
    }

    #[test]
    fn assez_d_xp_pour_monter_de_niveau() {
        let mut combat = combat(
            vec![rapide()],
            vec![affaibli(lent(), 2), affaibli(lent(), 3)],
        );
        tour(&mut combat, Action::LancerFilet, &[90]);
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        // 125 + 2 x 50 = 225 : le niveau 6 demande 216.
        let bilan = terminer(combat, &[100, 100]);
        let rapport = bilan.experience[0].as_ref().unwrap();
        assert_eq!(rapport.experience_gagnee, 100);
        assert_eq!(rapport.niveau_apres, Niveau::new(6).unwrap());
    }

    fn bonus_d_attaque() -> BonusDeStats {
        Stats {
            attaque: 10,
            ..Stats::default()
        }
    }

    #[test]
    fn le_bonus_du_joueur_ne_touche_que_ses_monstres() {
        let combat = combat(vec![rapide()], vec![lent()]).avec_bonus_du_joueur(bonus_d_attaque());
        assert_eq!(combat.stats_en_combat(j(0)).attaque, 11);
        assert_eq!(combat.stats_en_combat(a(0)).attaque, 10);
    }

    #[test]
    fn l_ordre_renforce_le_monstre_actif_jusqu_a_la_fin_du_combat() {
        let mut combat = combat(vec![rapide(), lent()], vec![lent()])
            .avec_bonus_du_joueur(bonus_d_attaque())
            .avec_ordre(Ordre::CriDeGuerre);
        assert_eq!(combat.ordre_disponible(), Some(Ordre::CriDeGuerre));
        // L'ordre ne fait aucun tirage et ne prend pas le tour.
        assert_eq!(combat.donner_ordre(), Ok((Ordre::CriDeGuerre, j(0))));
        assert_eq!(combat.phase(), Phase::ChoixAction);
        assert!(combat.a_recu_l_ordre(0));
        // 10 d'Attaque, +10 % de bonus du joueur et +20 % d'ordre.
        assert_eq!(combat.stats_en_combat(j(0)).attaque, 13);
        assert_eq!(combat.stats_en_combat(j(1)).attaque, 11);
        assert_eq!(combat.stats_en_combat(a(0)).attaque, 10);
        assert_eq!(combat.ordre_disponible(), None);
        assert_eq!(combat.donner_ordre(), Err(ErreurCombat::OrdreImpossible));
        tour(&mut combat, Action::Changer(1), &[2, 100]);
        tour(&mut combat, Action::Changer(0), &[2, 100]);
        assert_eq!(combat.stats_en_combat(j(0)).attaque, 13);
    }

    #[test]
    fn sans_ordre_le_heros_ne_peut_rien_donner() {
        let mut combat = combat(vec![rapide()], vec![lent()]);
        assert_eq!(combat.ordre_disponible(), None);
        assert_eq!(combat.donner_ordre(), Err(ErreurCombat::OrdreImpossible));
    }

    #[test]
    fn pas_d_ordre_pendant_un_remplacement() {
        let mut combat = combat(vec![affaibli(lent(), 3), lent()], vec![tres_rapide()])
            .avec_ordre(Ordre::Incantation);
        tour(&mut combat, Action::Attaquer(0), &[2, 100]);
        assert_eq!(combat.phase(), Phase::Remplacement);
        assert_eq!(combat.ordre_disponible(), None);
        assert_eq!(
            combat.donner_ordre(),
            Err(ErreurCombat::RemplacementAttendu)
        );
        combat.remplacer(1).unwrap();
        assert_eq!(combat.donner_ordre(), Ok((Ordre::Incantation, j(1))));
    }

    #[test]
    fn le_bonus_d_attaque_augmente_les_degats() {
        // Au niveau 50 : 55 dans chaque stat. Le coup inflige 19 dégâts, et 21
        // avec les 60 d'Attaque que donne le bonus.
        let base_rapide = Stats {
            vitesse: 60,
            ..stats_uniformes(50)
        };
        let joueur = monstre(&[Type::Feu], base_rapide, 50, vec![coup()]);
        let adversaire = monstre(&[Type::Feu], stats_uniformes(50), 50, vec![coup()]);
        let mut combat =
            combat(vec![joueur], vec![adversaire]).avec_bonus_du_joueur(bonus_d_attaque());
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100, 2, 100]);
        assert_eq!(evenements[1], degats(a(0), 21));
        assert_eq!(evenements[3], degats(j(0), 19));
    }

    #[test]
    fn l_habilete_de_stats_joue_des_deux_cotes_et_s_ajoute_a_la_classe() {
        let joueur = lent().avec_habilete(Habilete::Ferocite);
        let adversaire = lent().avec_habilete(Habilete::Carapace);
        let combat = combat(vec![joueur], vec![adversaire]).avec_bonus_du_joueur(bonus_d_attaque());
        // 10, puis +10 % de la classe et +10 % de l'habileté : 12.
        assert_eq!(combat.stats_en_combat(j(0)).attaque, 12);
        assert_eq!(combat.stats_en_combat(a(0)).defense, 11);
        assert_eq!(combat.stats_en_combat(a(0)).attaque, 10);
    }

    #[test]
    fn instinct_ajoute_20_pour_cent_a_l_xp_du_combat() {
        let instinct = rapide().avec_habilete(Habilete::Instinct);
        let mut combat = combat(vec![instinct], vec![affaibli(lent(), 2)]);
        tour(&mut combat, Action::LancerFilet, &[90]);
        // 50 XP de base, 60 avec Instinct.
        let bilan = terminer(combat, &[100]);
        assert_eq!(xp_gagnee(&bilan, 0), Some(60));
    }

    #[test]
    fn le_bonus_de_l_assassin_departage_sans_tirage() {
        // 10 de Vitesse de chaque côté : le bonus en donne 11 au joueur, qui
        // agit en premier sans tirage au sort.
        let mut combat = combat(vec![lent()], vec![lent()])
            .avec_bonus_du_joueur(crate::heros::Classe::Assassin.bonus());
        let evenements = tour(&mut combat, Action::Attaquer(0), &[2, 100, 2, 100]);
        assert_eq!(evenements[0], lancee(j(0), coup()));
    }

    fn potions(nombre: u32) -> Soins {
        let mut soins = Soins::default();
        soins.ajouter(Soin::Potion, nombre);
        soins
    }

    #[test]
    fn une_potion_soigne_puis_l_adversaire_agit() {
        let mut combat = combat(vec![affaibli(rapide(), 5)], vec![lent()]).avec_soins(potions(2));
        // 50 % de 20 PV max : 10 PV rendus.
        let evenements = tour(
            &mut combat,
            Action::UtiliserSoin(Soin::Potion, 0),
            &[50, 2, 100],
        );
        assert_eq!(
            evenements,
            vec![
                Evenement::SoinUtilise {
                    soin: Soin::Potion,
                    cible: j(0),
                    effet: EffetDuSoin::PvRendus(10),
                },
                lancee(a(0), coup()),
                degats(j(0), 5),
            ]
        );
        assert_eq!(combat.equipe()[0].pv_actuels(), 10);
        assert_eq!(combat.soins().nombre(Soin::Potion), 1);
        assert_eq!(terminer(combat, &[]).soins, potions(1));
    }

    #[test]
    fn soin_refuse_sans_rien_changer() {
        let sans_potion = combat(vec![affaibli(rapide(), 5)], vec![lent()]);
        let mut aleatoire = AleatoireScripte::new([]);
        assert_eq!(
            sans_potion
                .clone()
                .jouer_tour(Action::UtiliserSoin(Soin::Potion, 0), &mut aleatoire),
            Err(ErreurCombat::AucunSoin(Soin::Potion))
        );
        let mut combat =
            combat(vec![rapide(), affaibli(lent(), 0)], vec![lent()]).avec_soins(potions(1));
        let avant = combat.clone();
        for (action, erreur) in [
            (
                Action::UtiliserSoin(Soin::Potion, 1),
                ErreurCombat::MonstreKo(1),
            ),
            (
                Action::UtiliserSoin(Soin::Potion, 5),
                ErreurCombat::MonstreInexistant(5),
            ),
            (
                Action::UtiliserSoin(Soin::Elixir, 1),
                ErreurCombat::AucunSoin(Soin::Elixir),
            ),
        ] {
            assert_eq!(combat.jouer_tour(action, &mut aleatoire), Err(erreur));
        }
        assert_eq!(combat, avant);
    }

    #[test]
    fn potion_perdue_sur_un_monstre_qui_a_tous_ses_pv() {
        // Aucun tirage pour la potion : son effet nul est certain.
        let mut combat = combat(vec![rapide()], vec![lent()]).avec_soins(potions(1));
        let evenements = tour(
            &mut combat,
            Action::UtiliserSoin(Soin::Potion, 0),
            &[2, 100],
        );
        assert_eq!(
            evenements[0],
            Evenement::SoinUtilise {
                soin: Soin::Potion,
                cible: j(0),
                effet: EffetDuSoin::PvRendus(0),
            }
        );
        assert!(combat.soins().est_vide());
    }

    #[test]
    fn un_elixir_releve_un_monstre_ko_de_l_equipe() {
        let mut soins = Soins::default();
        soins.ajouter(Soin::Elixir, 1);
        let mut combat =
            combat(vec![rapide(), affaibli(lent(), 0)], vec![lent()]).avec_soins(soins);
        let evenements = tour(
            &mut combat,
            Action::UtiliserSoin(Soin::Elixir, 1),
            &[2, 100],
        );
        assert_eq!(
            evenements[0],
            Evenement::SoinUtilise {
                soin: Soin::Elixir,
                cible: j(1),
                effet: EffetDuSoin::PvRendus(10),
            }
        );
        assert_eq!(combat.equipe()[1].pv_actuels(), 10);
        assert!(combat.soins().est_vide());
    }
}
