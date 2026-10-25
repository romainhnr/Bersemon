//! Le joueur automatique : une stratégie simple et constante, pour que les
//! écarts d'une zone à l'autre viennent des valeurs du jeu, et non du
//! joueur. Il part dans la zone la plus avancée à sa portée, avec ses
//! monstres de plus haut niveau, ou les mieux armés contre l'Apôtre qui
//! l'attend. En combat, il envoie le monstre le mieux armé selon la table
//! des types, attaque avec la meilleure attaque, lance un filet sur une
//! espèce qu'il n'a pas quand la chance est bonne et fuit quand son dernier
//! monstre faiblit. Entre deux étapes, il soigne sous la moitié des PV,
//! rentre quand l'équipe faiblit et n'affronte le combat de fin de zone
//! qu'en forme. Au camp, il fait agrandir l'équipe, forger et porter
//! l'équipement du héros, puis améliorer les filets.
//!
//! Il n'emploie ni les expéditions automatiques, ni la cour d'entraînement,
//! ni les pièges, ni le rite, ni les capsules, habiletés, pierres et
//! boosters, ni la Tour sans fond, et ne revient pas dans une zone pour y
//! chercher une ressource : il mesure une progression sans ces aides.

use std::collections::BTreeMap;

use bersemon_core::aleatoire::GenerateurAleatoire;
use bersemon_core::amelioration::Amelioration;
use bersemon_core::attaque::Categorie;
use bersemon_core::catalogue::Catalogue;
use bersemon_core::codex::EtatCodex;
use bersemon_core::collection::Tailles;
use bersemon_core::combat::{Action, Combat, Cote};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::{
    ActionDeGestion, Attente, Choix, Contexte, Deroulement, Evenement,
};
use bersemon_core::equipement::{ObjetDEquipement, PlaceDEquipement};
use bersemon_core::filet::Filet;
use bersemon_core::heros::Classe;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::inventaire::Objet;
use bersemon_core::marchand::{prix_du_filet, prix_du_soin};
use bersemon_core::monstre::Monstre;
use bersemon_core::partie::Partie;
use bersemon_core::recompenses_du_codex::especes_capturees;
use bersemon_core::sauvegarde::{ErreurSauvegarde, SauvegardeRepository};
use bersemon_core::soin::Soin;
use bersemon_core::stats::BonusDeStats;
use bersemon_core::types::{Efficacite, efficacite};
use bersemon_core::zone::{Zone, ZoneId};

/// Au-delà, la partie est dite inachevée : le joueur tourne en rond.
const LIMITE_D_EXPEDITIONS: usize = 1_500;
/// Garde-fou contre une boucle de choix refusés.
const LIMITE_DE_REFUS: u32 = 20;
/// Ce que le joueur emporte dans son sac à chaque départ.
const POTIONS_EMPORTEES: u32 = 6;
const FILETS_EMPORTES: u32 = 10;
/// Il ne lance un filet qu'avec au moins cette chance, en pourcentage.
const CHANCE_MINIMALE_DU_FILET: u32 = 50;
/// La partie s'arrête à la chute du troisième Apôtre, le Margrave de givre.
const APOTRES_A_VAINCRE: usize = 3;
/// Hypothèse : le temps réel qui passe à chaque choix du joueur, pauses
/// comprises. Il règle ce que les travaux de Gunther, de 30 minutes à 8
/// heures, laissent faire d'expéditions pendant qu'ils durent.
const SECONDES_PAR_CHOIX: u64 = 60;
/// Il n'entre dans une zone qu'une fois passé le premier tiers de ses
/// niveaux.
const PART_DES_NIVEAUX_AVANT_D_ENTRER: u8 = 3;
/// Avant le combat de fin de zone, il rentre si l'équipe a moins de ces PV,
/// en pourcentage, ou un monstre KO.
const PV_AVANT_LE_BOSS: u32 = 80;
/// En combat de fin de zone, il soigne le monstre actif sous ces PV.
const PV_DU_SOIN_EN_COMBAT: u32 = 35;
/// Il ne change de monstre que pour un monstre qui vaut au moins deux fois
/// plus face à l'adversaire : changer coûte une attaque subie. Une marge de
/// 1,5 multipliait les défaites dans les premières zones, une de 3 ne
/// faisait pas mieux que 2.
const MARGE_DU_CHANGEMENT: f64 = 2.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Issue {
    MeneeAuBout,
    Rentree,
    Defaite,
}

/// Le combat de fin de zone d'une expédition, contre un humain ou un
/// Apôtre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombatDeBoss {
    pub nom: String,
    pub vaincu: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpeditionJouee {
    pub zone: ZoneId,
    pub issue: Issue,
    /// De l'équipe, au départ et au retour.
    pub niveau_au_depart: f64,
    pub niveau_au_retour: f64,
    /// Variations du sac et du coffre pendant l'expédition : une défaite
    /// fait perdre le sac.
    pub argent: i64,
    pub filets_depenses: i64,
    pub potions_depensees: i64,
    pub nouvelles_especes: i64,
    pub boss: Option<CombatDeBoss>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bilan {
    pub classe: Classe,
    pub expeditions: Vec<ExpeditionJouee>,
    /// Le troisième Apôtre est tombé avant la limite d'expéditions.
    pub terminee: bool,
    /// Ce que le joueur a dépensé chez Malvoise, en soins et en filets.
    pub achats: u64,
    /// Les objets d'équipement forgés à la fin de la partie.
    pub objets_forges: usize,
    /// L'argent du sac et du coffre à la fin de la partie.
    pub argent_final: u32,
    /// Le numéro de la première expédition partie avec une équipe agrandie.
    pub equipe_agrandie: Option<usize>,
    /// Les ressources trouvées, par nom, qu'elles aient été gardées ou non.
    pub ressources: BTreeMap<String, u32>,
    /// Les choix que le noyau a refusés : un joueur automatique bien réglé
    /// n'en fait presque pas.
    pub choix_refuses: u32,
}

/// Rien n'est écrit : la partie ne vit que le temps de la simulation.
struct DepotMuet;

impl SauvegardeRepository for DepotMuet {
    fn charger(&self, _: &dyn Catalogue) -> Result<Option<Partie>, ErreurSauvegarde> {
        Ok(None)
    }

    fn enregistrer(&self, _: &Partie) -> Result<(), ErreurSauvegarde> {
        Ok(())
    }

    fn supprimer(&self) -> Result<(), ErreurSauvegarde> {
        Ok(())
    }
}

/// Ce qui se mesure au départ et au retour d'une expédition.
#[derive(Debug, Clone, Copy)]
struct Releve {
    argent: i64,
    filets: i64,
    potions: i64,
    especes: i64,
    niveau: f64,
}

fn relever(partie: &Partie, contenu: &ContenuV1) -> Releve {
    let filets = partie.filets();
    let potions = partie.sac().soins().nombre(Soin::Potion)
        + partie.coffre_du_camp().soins().nombre(Soin::Potion);
    Releve {
        argent: i64::from(partie.argent()),
        filets: Filet::TOUS
            .iter()
            .map(|filet| i64::from(filets.nombre(*filet)))
            .sum(),
        potions: i64::from(potions),
        especes: i64::from(especes_capturees(partie.codex(), contenu.especes())),
        niveau: niveau_de_reference(partie),
    }
}

/// Le niveau moyen des monstres de plus haut niveau, équipe et camp réunis,
/// autant qu'en compte une équipe pleine : préparer l'équipe contre un
/// Apôtre ne fait ni changer de zone ni baisser le niveau mesuré.
fn niveau_de_reference(partie: &Partie) -> f64 {
    let collection = partie.collection();
    let mut niveaux: Vec<u8> = collection
        .equipe()
        .iter()
        .chain(
            collection
                .camp()
                .iter()
                .filter(|monstre| !monstre.est_loue()),
        )
        .map(|monstre| monstre.niveau().valeur())
        .collect();
    niveaux.sort_unstable_by(|un, autre| autre.cmp(un));
    niveaux.truncate(collection.tailles().equipe);
    if niveaux.is_empty() {
        return 0.0;
    }
    let total: u32 = niveaux.iter().map(|niveau| u32::from(*niveau)).sum();
    f64::from(total) / niveaux.len() as f64
}

/// L'expédition en cours, le temps de la mesurer.
struct EnCours {
    zone: ZoneId,
    nom_de_zone: String,
    depart: Releve,
    etape: u32,
    total: Option<u32>,
    boss: Option<CombatDeBoss>,
    defaite: bool,
}

pub fn jouer_une_partie(contenu: &ContenuV1, classe: Classe, graine: u64) -> Bilan {
    let mut aleatoire = GenerateurAleatoire::new(graine);
    let depot = DepotMuet;
    let mut deroulement = Deroulement::demarrer(Ok(None));
    let mut bilan = Bilan {
        classe,
        expeditions: Vec::new(),
        terminee: false,
        achats: 0,
        objets_forges: 0,
        argent_final: 0,
        equipe_agrandie: None,
        ressources: BTreeMap::new(),
        choix_refuses: 0,
    };
    let mut en_cours: Option<EnCours> = None;
    let mut refus = 0;
    let mut secondes = 0;
    while bilan.expeditions.len() < LIMITE_D_EXPEDITIONS {
        if en_cours.is_none()
            && matches!(deroulement.attente(), Attente::Camp)
            && deroulement
                .partie()
                .is_some_and(|partie| partie.apotres_vaincus().len() >= APOTRES_A_VAINCRE)
        {
            bilan.terminee = true;
            break;
        }
        let maintenant = Horodatage::depuis_secondes(secondes);
        let choix = if refus == 0 {
            choisir(&deroulement, contenu, classe, maintenant)
        } else {
            repli(&deroulement, contenu)
        };
        if let (Choix::PartirEnExpedition(zone), Some(partie)) = (&choix, deroulement.partie()) {
            if bilan.equipe_agrandie.is_none()
                && partie.collection().tailles().equipe > Tailles::DE_DEPART.equipe
            {
                bilan.equipe_agrandie = Some(bilan.expeditions.len() + 1);
            }
            let nom_de_zone = contenu
                .zones()
                .iter()
                .find(|autre| autre.id() == zone)
                .map_or_else(
                    || zone.as_str().to_string(),
                    |autre| autre.nom().to_string(),
                );
            en_cours = Some(EnCours {
                zone: zone.clone(),
                nom_de_zone,
                depart: relever(partie, contenu),
                etape: 0,
                total: None,
                boss: None,
                defaite: false,
            });
        }
        let depense = cout_du_choix(&choix);
        secondes += SECONDES_PAR_CHOIX;
        let mut contexte = Contexte {
            contenu,
            aleatoire: &mut aleatoire,
            depot: &depot,
            maintenant: Horodatage::depuis_secondes(secondes),
        };
        match deroulement.repondre(choix.clone(), &mut contexte) {
            Ok(evenements) => {
                refus = 0;
                bilan.achats += depense;
                for evenement in &evenements {
                    if let Evenement::RessourcesTrouvees(ressources) = evenement {
                        for ressource in ressources {
                            *bilan
                                .ressources
                                .entry(ressource.nom().to_string())
                                .or_default() += 1;
                        }
                    }
                }
                if let Some(expedition) = en_cours.as_mut() {
                    let finie = suivre(expedition, &evenements);
                    if finie {
                        let expedition = en_cours.take().expect("expédition suivie");
                        let partie = deroulement.partie().expect("partie en cours");
                        bilan
                            .expeditions
                            .push(conclure(expedition, partie, contenu));
                    }
                }
            }
            Err(erreur) => {
                refus += 1;
                bilan.choix_refuses += 1;
                assert!(
                    refus < LIMITE_DE_REFUS,
                    "choix refusés en boucle : {choix:?} ({erreur})"
                );
            }
        }
    }
    if let Some(partie) = deroulement.partie() {
        bilan.objets_forges = partie.heros().equipement().possedes().len();
        bilan.argent_final = partie.argent();
    }
    bilan
}

fn cout_du_choix(choix: &Choix) -> u64 {
    match choix {
        Choix::AcheterSoins(soin, nombre) => u64::from(prix_du_soin(*soin) * nombre),
        Choix::AcheterFilets(filet, nombre) => {
            u64::from(prix_du_filet(*filet).unwrap_or(0) * nombre)
        }
        _ => 0,
    }
}

/// Vrai quand l'expédition est finie.
fn suivre(expedition: &mut EnCours, evenements: &[Evenement]) -> bool {
    let mut finie = false;
    for evenement in evenements {
        match evenement {
            Evenement::Etape { numero, total } => {
                expedition.etape = *numero;
                expedition.total = *total;
            }
            Evenement::HumainRencontre(_) => {
                expedition.boss = Some(CombatDeBoss {
                    nom: format!("{} : humain", expedition.nom_de_zone),
                    vaincu: false,
                });
            }
            Evenement::RecitDeLApotre(_) => {
                expedition.boss = Some(CombatDeBoss {
                    nom: format!("{} : Apôtre", expedition.nom_de_zone),
                    vaincu: false,
                });
            }
            Evenement::HumainVaincu { .. } | Evenement::AmeDApotreRecueillie(_) => {
                if let Some(boss) = expedition.boss.as_mut() {
                    boss.vaincu = true;
                }
            }
            Evenement::Defaite { .. } => {
                expedition.defaite = true;
                finie = true;
            }
            Evenement::RetourAuCamp => finie = true,
            _ => {}
        }
    }
    finie
}

fn conclure(expedition: EnCours, partie: &Partie, contenu: &ContenuV1) -> ExpeditionJouee {
    let retour = relever(partie, contenu);
    let depart = expedition.depart;
    let issue = if expedition.defaite {
        Issue::Defaite
    } else if expedition
        .total
        .is_some_and(|total| expedition.etape >= total)
    {
        Issue::MeneeAuBout
    } else {
        Issue::Rentree
    };
    ExpeditionJouee {
        zone: expedition.zone,
        issue,
        niveau_au_depart: depart.niveau,
        niveau_au_retour: retour.niveau,
        argent: retour.argent - depart.argent,
        filets_depenses: depart.filets - retour.filets,
        potions_depensees: depart.potions - retour.potions,
        nouvelles_especes: retour.especes - depart.especes,
        boss: expedition.boss,
    }
}

/// Après un refus : une réponse qui convient toujours à la question.
fn repli(deroulement: &Deroulement, contenu: &ContenuV1) -> Choix {
    match deroulement.attente() {
        Attente::ActionDeCombat(_) => Choix::Combat(Action::Attaquer(0)),
        Attente::EntreDeuxEtapes => Choix::ContinuerLExpedition,
        Attente::Camp => match deroulement.partie() {
            Some(partie) => partir(partie, contenu),
            None => Choix::Poursuivre,
        },
        Attente::Confirmation(_) => Choix::Non,
        Attente::CaptureDesKo(_) => Choix::NePlusCapturer,
        _ => Choix::Poursuivre,
    }
}

fn choisir(
    deroulement: &Deroulement,
    contenu: &ContenuV1,
    classe: Classe,
    maintenant: Horodatage,
) -> Choix {
    match deroulement.attente() {
        Attente::Reprise(_) => Choix::ReprendreLaPartie,
        Attente::SauvegardeIllisible(_) => Choix::NouvellePartie,
        Attente::NomDuHeros => Choix::NomDuHeros("Aldric".to_string()),
        Attente::Classe => Choix::Classe(classe),
        Attente::Camp => {
            let partie = deroulement.partie().expect("partie au camp");
            au_camp(partie, contenu, maintenant)
        }
        Attente::EntreDeuxEtapes => {
            let partie = deroulement.partie().expect("partie en expédition");
            // L'étape suivante est la dernière, celle du combat de fin de
            // zone tant que son humain ou son Apôtre attend.
            let avant_le_boss = deroulement.expedition().is_some_and(|expedition| {
                expedition.etapes_faites() + 1 == expedition.nombre_etapes()
                    && (expedition.humain().is_some() || expedition.apotre().is_some())
            });
            entre_deux_etapes(partie, avant_le_boss)
        }
        Attente::MonstreRare(_) => Choix::PrendreLeMonstreRare,
        Attente::Prise(_) => Choix::GarderLaPrise,
        // Le joueur simulé n'entre pas dans la Fosse aux damnés.
        Attente::AdversaireDeLArene { .. } => Choix::RentrerAuCamp,
        Attente::ActionDeCombat(combat) => {
            let partie = deroulement.partie().expect("partie en combat");
            en_combat(combat, partie)
        }
        Attente::Remplacement(combat) => remplacer(combat),
        Attente::CaptureDesKo(combat) => {
            let partie = deroulement.partie().expect("partie en combat");
            capturer_un_ko(combat, partie)
        }
        Attente::OubliAttaque { monstre, nouvelle } => {
            let plus_faible = monstre
                .attaques()
                .liste()
                .iter()
                .enumerate()
                .min_by_key(|(_, attaque)| attaque.puissance());
            match plus_faible {
                Some((position, attaque)) if attaque.puissance() < nouvelle.puissance() => {
                    Choix::OublierAttaque(position)
                }
                _ => Choix::RenoncerALAttaque,
            }
        }
        Attente::Evolution { .. } => Choix::Evoluer,
        Attente::PlaceManquante(_) => Choix::RelacherLaCapture,
        Attente::Annonce(_) => Choix::Poursuivre,
        Attente::Confirmation(_) => Choix::Oui,
        Attente::CombatDeGroupe | Attente::AttenteDuGroupe => {
            unreachable!("le joueur automatique joue seul")
        }
        Attente::SacPlein => {
            let partie = deroulement.partie().expect("partie en expédition");
            Choix::Jeter(objet_a_jeter(partie))
        }
    }
}

/// Dans l'ordre : les âmes à Malvoise, les récompenses du codex, le travail
/// de Gunther, l'équipement, la meilleure équipe, l'argent et les
/// ressources à l'abri, les potions et les filets du sac, puis le départ.
fn au_camp(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> Choix {
    if partie.marchand_au_camp() {
        if let Some(ame) = partie.ames_portees().first() {
            return Choix::DonnerAme(ame.id().clone());
        }
    }
    if partie
        .recompense_a_reclamer(contenu.recompenses_du_codex(), contenu.especes())
        .is_some()
    {
        return Choix::ReclamerRecompenseDuCodex;
    }
    if let Some(choix) = forge(partie, contenu, maintenant) {
        return choix;
    }
    if let Some(choix) = s_equiper(partie) {
        return choix;
    }
    if let Some(gestion) = meilleure_equipe(partie, contenu) {
        return Choix::Gerer(gestion);
    }
    // Le butin rejoint le coffre : le sac, limité, ne repart qu'avec les
    // potions et les filets (section 13.3).
    let sac = partie.sac();
    let potions = sac.soins().nombre(Soin::Potion);
    let filets: u32 = sac.filets().presents().map(|(_, nombre)| nombre).sum();
    if sac.argent() > 0
        || sac.places_prises() > potions + filets
        || potions > POTIONS_EMPORTEES
        || filets > FILETS_EMPORTES
    {
        return Choix::ToutRangerAuCoffre;
    }
    if let Some(choix) = remplir_le_sac(partie) {
        return choix;
    }
    partir(partie, contenu)
}

/// Le travail fini constaté ; sinon, Gunther libre, le premier travail
/// abordable : l'équipe agrandie, l'objet d'équipement aux plus gros bonus,
/// puis les filets améliorés.
fn forge(partie: &Partie, contenu: &ContenuV1, maintenant: Horodatage) -> Option<Choix> {
    let forge = partie.forge();
    if !forge.est_au_camp() {
        return None;
    }
    if let Some(chantier) = forge.chantier() {
        return chantier
            .est_fini(maintenant)
            .then_some(Choix::ConstaterTravail);
    }
    let amelioration = |amelioration: Amelioration| {
        let niveau = forge.ameliorations().niveau(amelioration) + 1;
        contenu
            .plan_d_amelioration(amelioration, niveau)
            .filter(|plan| partie.verifier_amelioration(plan).is_ok())
            .map(|_| Choix::LancerAmelioration(amelioration))
    };
    if let Some(choix) = amelioration(Amelioration::Equipe) {
        return Some(choix);
    }
    let objet = contenu
        .plans_de_fabrication()
        .iter()
        .filter(|plan| partie.verifier_fabrication(plan).is_ok())
        .max_by_key(|plan| total(plan.objet().bonus()))
        .map(|plan| Choix::LancerFabrication(plan.objet().id().clone()));
    objet.or_else(|| amelioration(Amelioration::Filets))
}

fn total(bonus: BonusDeStats) -> u32 {
    [
        bonus.pv,
        bonus.attaque,
        bonus.defense,
        bonus.attaque_speciale,
        bonus.defense_speciale,
        bonus.vitesse,
    ]
    .into_iter()
    .map(u32::from)
    .sum()
}

/// À chaque place, l'objet aux plus gros bonus, s'il fait mieux que celui
/// qui y est porté.
fn s_equiper(partie: &Partie) -> Option<Choix> {
    let equipement = partie.heros().equipement();
    PlaceDEquipement::TOUTES.into_iter().find_map(|place| {
        let porte = equipement
            .porte(place)
            .map_or(0, |objet| total(objet.bonus()));
        equipement
            .disponibles(place)
            .into_iter()
            .max_by_key(|objet| total(objet.bonus()))
            .filter(|objet: &&ObjetDEquipement| total(objet.bonus()) > porte)
            .map(|objet| Choix::Equiper {
                place,
                objet: objet.id().clone(),
            })
    })
}

/// Les monstres se classent par niveau, ou, si un Apôtre attend au bout de
/// la zone visée, par ce qu'ils valent face à lui. Une place libre dans
/// l'équipe accueille le meilleur du camp ; sinon, le moins bon de l'équipe
/// lui cède sa place, s'il vaut moins. Contre l'Apôtre, le meilleur de
/// l'équipe passe en tête.
fn meilleure_equipe(partie: &Partie, contenu: &ContenuV1) -> Option<ActionDeGestion> {
    let zone = zone_visee(partie, contenu);
    // Un tirage à part, qui ne touche pas à celui de la partie : seules
    // comptent l'espèce, le niveau et les valeurs internes, fixes.
    let apotre = contenu
        .piste()
        .apotre_en_attente(
            zone.id(),
            partie.indices_trouves(),
            partie.apotres_vaincus(),
        )
        .map(|apotre| apotre.generer(&mut GenerateurAleatoire::new(0)));
    let valeur = |monstre: &Monstre| match &apotre {
        Some(apotre) => valeur_face_a(monstre, apotre),
        None => f64::from(monstre.niveau().valeur()),
    };
    let collection = partie.collection();
    let meilleur_du_camp = collection
        .camp()
        .iter()
        .enumerate()
        .filter(|(_, monstre)| !monstre.est_loue())
        .max_by(|(_, un), (_, autre)| valeur(un).total_cmp(&valeur(autre)));
    if collection.equipe().len() < collection.tailles().equipe {
        return meilleur_du_camp.map(|(camp, _)| ActionDeGestion::FaireVenir(camp));
    }
    let (equipe, moins_bon) = collection
        .equipe()
        .iter()
        .enumerate()
        .min_by(|(_, un), (_, autre)| valeur(un).total_cmp(&valeur(autre)))?;
    if let Some((camp, meilleur)) = meilleur_du_camp {
        if valeur(meilleur) > valeur(moins_bon) {
            return Some(ActionDeGestion::Echanger { equipe, camp });
        }
    }
    apotre.as_ref()?;
    let (tete, _) = collection
        .equipe()
        .iter()
        .enumerate()
        .max_by(|(_, un), (_, autre)| valeur(un).total_cmp(&valeur(autre)))?;
    (tete != 0).then_some(ActionDeGestion::ChangerLOrdre { de: tete, vers: 0 })
}

/// Quand le sac déborde : ce que ce joueur n'emploie pas d'abord (pierres,
/// boosters, capsules, Philtres, charmes), puis les ressources, les filets
/// et les potions, un par un.
fn objet_a_jeter(partie: &Partie) -> Objet {
    let sac = partie.sac();
    if let Some((pierre, _)) = sac.pierres().first() {
        return Objet::Pierres(*pierre, 1);
    }
    if let Some((booster, _)) = sac.boosters().first() {
        return Objet::Boosters(*booster, 1);
    }
    if let Some(capsule) = sac.capsules().first() {
        return Objet::Capsule(capsule.id().clone());
    }
    if sac.capsules_d_habilete() > 0 {
        return Objet::CapsulesDHabilete(1);
    }
    if sac.philtres_chromatiques() > 0 {
        return Objet::PhiltresChromatiques(1);
    }
    if sac.charmes_de_garde() > 0 {
        return Objet::CharmesDeGarde(1);
    }
    if let Some((ressource, _)) = sac.ressources().first() {
        return Objet::Ressources(ressource.id().clone(), 1);
    }
    if let Some((filet, _)) = sac.filets().presents().next() {
        return Objet::Filets(filet, 1);
    }
    let (soin, _) = sac
        .soins()
        .presents()
        .next()
        .expect("un sac qui déborde a un objet");
    Objet::Soins(soin, 1)
}

fn remplir_le_sac(partie: &Partie) -> Option<Choix> {
    let sac = partie.sac();
    let coffre = partie.coffre_du_camp();
    let potions = sac.soins().nombre(Soin::Potion);
    if potions < POTIONS_EMPORTEES {
        let manque = POTIONS_EMPORTEES - potions;
        let au_coffre = coffre.soins().nombre(Soin::Potion);
        if au_coffre > 0 {
            return Some(Choix::SortirDuCoffre(Objet::Soins(
                Soin::Potion,
                au_coffre.min(manque),
            )));
        }
        let abordables = partie.argent() / prix_du_soin(Soin::Potion);
        if partie.marchand_au_camp() && abordables > 0 {
            return Some(Choix::AcheterSoins(Soin::Potion, abordables.min(manque)));
        }
    }
    let filets: u32 = sac.filets().presents().map(|(_, nombre)| nombre).sum();
    if filets < FILETS_EMPORTES {
        let manque = FILETS_EMPORTES - filets;
        if let Some((filet, au_coffre)) = coffre.filets().presents().last() {
            return Some(Choix::SortirDuCoffre(Objet::Filets(
                filet,
                au_coffre.min(manque),
            )));
        }
        let prix = prix_du_filet(Filet::Chanvre).unwrap_or(u32::MAX);
        let abordables = partie.argent() / prix;
        // Les filets passent après les potions : il en reste de quoi en
        // racheter.
        let reserve = prix_du_soin(Soin::Potion) * POTIONS_EMPORTEES / prix;
        if partie.marchand_au_camp() && abordables > reserve {
            return Some(Choix::AcheterFilets(
                Filet::Chanvre,
                (abordables - reserve).min(manque),
            ));
        }
    }
    None
}

fn partir(partie: &Partie, contenu: &ContenuV1) -> Choix {
    Choix::PartirEnExpedition(zone_visee(partie, contenu).id().clone())
}

/// La zone la plus avancée où le niveau de référence a passé le premier
/// tiers des niveaux, sinon la première.
fn zone_visee<'a>(partie: &Partie, contenu: &'a ContenuV1) -> &'a Zone {
    let niveau = niveau_de_reference(partie);
    let accessibles = partie.zones_accessibles(contenu.zones(), contenu.piste());
    accessibles
        .iter()
        .rev()
        .find(|zone| {
            let (minimum, maximum) = zone.niveaux();
            let entree = minimum.valeur()
                + (maximum.valeur() - minimum.valeur()) / PART_DES_NIVEAUX_AVANT_D_ENTRER;
            f64::from(entree) <= niveau
        })
        .or_else(|| accessibles.first())
        .copied()
        .expect("au moins une zone accessible")
}

/// Une potion sous la moitié des PV ; le retour si la moitié de l'équipe
/// est KO, s'il ne lui reste que la moitié de ses PV, ou, avant le combat de
/// fin de zone, si elle n'est pas en forme.
fn entre_deux_etapes(partie: &Partie, avant_le_boss: bool) -> Choix {
    let equipe = partie.collection().equipe();
    let potions = partie.sac().soins().nombre(Soin::Potion);
    let blesse = equipe
        .iter()
        .position(|monstre| !monstre.est_ko() && monstre.pv_actuels() * 2 < monstre.stats().pv);
    if let (Some(index), true) = (blesse, potions > 0) {
        return Choix::UtiliserSoin(Soin::Potion, index);
    }
    let pv: u32 = equipe.iter().map(|monstre| monstre.pv_actuels()).sum();
    let pv_max: u32 = equipe.iter().map(|monstre| monstre.stats().pv).sum();
    let ko = equipe.iter().filter(|monstre| monstre.est_ko()).count();
    let en_forme = ko == 0 && pv * 100 >= pv_max * PV_AVANT_LE_BOSS;
    if ko * 2 >= equipe.len() || pv * 2 < pv_max || (avant_le_boss && !en_forme) {
        Choix::RentrerAuCamp
    } else {
        Choix::ContinuerLExpedition
    }
}

fn en_combat(combat: &Combat, partie: &Partie) -> Choix {
    let boss = combat.est_contre_un_apotre() || combat.est_contre_un_humain();
    if boss && combat.ordre_disponible().is_some() {
        return Choix::DonnerOrdre;
    }
    let actif = combat.actif(Cote::Joueur);
    let monstre = combat.monstre(actif);
    if boss
        && combat.soins().nombre(Soin::Potion) > 0
        && monstre.pv_actuels() * 100 < monstre.stats().pv * PV_DU_SOIN_EN_COMBAT
    {
        return Choix::Combat(Action::UtiliserSoin(Soin::Potion, actif.index));
    }
    if doit_fuir(combat) {
        return Choix::Combat(Action::Fuir);
    }
    if let Some(index) = changement(combat) {
        return Choix::Combat(Action::Changer(index));
    }
    if let Some(filet) = filet_a_lancer(combat, partie) {
        return Choix::Combat(Action::LancerFilet(filet));
    }
    Choix::Combat(Action::Attaquer(meilleure_attaque(combat)))
}

/// Quand la fuite est permise, il fuit dès que son dernier monstre debout
/// a perdu la moitié de ses PV : la fuite réussit toujours et ne met jamais
/// KO (section 10.3).
fn doit_fuir(combat: &Combat) -> bool {
    if !combat.fuite_possible() {
        return false;
    }
    let mut debout = combat.equipe().iter().filter(|monstre| !monstre.est_ko());
    match (debout.next(), debout.next()) {
        (Some(dernier), None) => dernier.pv_actuels() * 2 < dernier.stats().pv,
        _ => false,
    }
}

fn multiplicateur(efficacite: Efficacite) -> f64 {
    match efficacite {
        Efficacite::Nulle => 0.0,
        Efficacite::TresReduite => 0.25,
        Efficacite::Reduite => 0.5,
        Efficacite::Normale => 1.0,
        Efficacite::Forte => 2.0,
        Efficacite::TresForte => 4.0,
    }
}

/// Ce que vaut un monstre face à l'adversaire, pour un joueur qui connaît la
/// table des types mais pas les attaques adverses : les dégâts de sa
/// meilleure attaque, multipliés par ce qu'il encaisse, PV restants et
/// défenses, contre des attaques supposées des types de l'adversaire.
fn valeur_face_a(monstre: &Monstre, adversaire: &Monstre) -> f64 {
    let stats = monstre.stats();
    let en_face = adversaire.stats();
    let types = monstre.espece().types();
    let types_en_face = adversaire.espece().types();
    let degats = monstre
        .attaques()
        .liste()
        .iter()
        .filter(|attaque| attaque.puissance() > 0)
        .map(|attaque| {
            let (force, resistance) = match attaque.categorie() {
                Categorie::Physique => (stats.attaque, en_face.defense),
                Categorie::Speciale => (stats.attaque_speciale, en_face.defense_speciale),
            };
            let bonus_de_type = if types.liste().contains(&attaque.type_attaque()) {
                1.5
            } else {
                1.0
            };
            f64::from(attaque.puissance())
                * bonus_de_type
                * multiplicateur(efficacite(attaque.type_attaque(), types_en_face))
                * f64::from(force)
                / f64::from(resistance.max(1))
                * f64::from(attaque.precision())
        })
        .fold(0.0, f64::max)
        * (2.0 * f64::from(monstre.niveau().valeur()) / 5.0 + 2.0);
    // Une immunité ne rend pas le monstre invincible : l'adversaire a
    // souvent une attaque d'un autre type.
    let menace = types_en_face
        .liste()
        .iter()
        .map(|type_| multiplicateur(efficacite(*type_, types)))
        .fold(0.0, f64::max)
        .max(0.25);
    let encaisse =
        f64::from(monstre.pv_actuels()) * f64::from(stats.defense + stats.defense_speciale);
    degats * encaisse / menace
}

/// Le monstre de l'équipe nettement mieux armé que le monstre actif contre
/// l'adversaire, comme un joueur qui retire un monstre d'un mauvais
/// affrontement de types.
fn changement(combat: &Combat) -> Option<usize> {
    let actif = combat.actif(Cote::Joueur);
    let adversaire = combat.monstre(combat.actif(Cote::Adversaire));
    let valeur_actuelle = valeur_face_a(combat.monstre(actif), adversaire);
    combat
        .equipe()
        .iter()
        .enumerate()
        .filter(|(index, monstre)| *index != actif.index && !monstre.est_ko())
        .map(|(index, monstre)| (index, valeur_face_a(monstre, adversaire)))
        .filter(|(_, valeur)| *valeur > valeur_actuelle * MARGE_DU_CHANGEMENT)
        .max_by(|(_, une), (_, autre)| une.total_cmp(autre))
        .map(|(index, _)| index)
}

/// Sur une espèce sauvage pas encore capturée, le filet qui donne la
/// meilleure chance, s'il atteint la chance minimale.
fn filet_a_lancer(combat: &Combat, partie: &Partie) -> Option<Filet> {
    if combat.est_contre_un_apotre() || combat.est_contre_un_humain() || combat.est_sans_capture() {
        return None;
    }
    let adversaire = combat.monstre(combat.actif(Cote::Adversaire));
    if partie.codex().etat(adversaire.espece().id()) == EtatCodex::Capturee {
        return None;
    }
    combat
        .filets()
        .presents()
        .map(|(filet, _)| (filet, combat.chance_de_capture(filet)))
        .filter(|(_, chance)| *chance >= CHANCE_MINIMALE_DU_FILET)
        .min_by_key(|(filet, _)| *filet as u8)
        .map(|(filet, _)| filet)
}

/// Les dégâts moyens attendus, précision comprise.
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

/// Le monstre valide qui vaut le plus face à l'adversaire actif.
fn remplacer(combat: &Combat) -> Choix {
    let adversaire = combat.monstre(combat.actif(Cote::Adversaire));
    let (index, _) = combat
        .equipe()
        .iter()
        .enumerate()
        .filter(|(_, monstre)| !monstre.est_ko())
        .map(|(index, monstre)| (index, valeur_face_a(monstre, adversaire)))
        .max_by(|(_, une), (_, autre)| une.total_cmp(autre))
        .expect("un monstre valide à envoyer");
    Choix::Remplacer(index)
}

/// Une espèce pas encore capturée, s'il en reste une parmi les vaincus.
fn capturer_un_ko(combat: &Combat, partie: &Partie) -> Choix {
    let codex = partie.codex();
    combat
        .adversaires_capturables()
        .into_iter()
        .find(|index| codex.etat(combat.adversaires()[*index].espece().id()) != EtatCodex::Capturee)
        .map_or(Choix::NePlusCapturer, Choix::Capturer)
}
