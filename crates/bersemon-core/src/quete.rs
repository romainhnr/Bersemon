//! Les quêtes du jour et de la semaine (section 13.12) : chaque jour, trois
//! quêtes, et chaque semaine, trois autres, les mêmes pour tous les joueurs.
//! Elles sont tirées parmi six objectifs d'après le numéro du jour ou de la
//! semaine, comme la météo d'après son créneau, pour que tous voient les
//! mêmes sans rien sauvegarder du tirage. Seul l'avancement du joueur se
//! sauvegarde.
//!
//! Une quête accomplie verse aussitôt sa récompense au coffre du camp, même
//! en expédition : le coffre ne se perd pas (section 13.3). Les valeurs sont
//! des hypothèses retenues, à confirmer (Q119).

use std::time::Duration;

use crate::aleatoire::GenerateurAleatoire;
use crate::booster::Booster;
use crate::horodatage::Horodatage;
use crate::inventaire::Inventaire;
use crate::zone::{Zone, tirer_entre};

/// Trois quêtes du jour, et trois de la semaine.
pub const QUETES_PAR_PERIODE: usize = 3;

const SECONDES_PAR_JOUR: u64 = 24 * 3_600;
const JOURS_PAR_SEMAINE: u64 = 7;
/// Le 1er janvier 1970 était un jeudi : le lundi qui ouvre sa semaine
/// tombait 3 jours plus tôt.
const JOURS_DEPUIS_LE_LUNDI_AU_1ER_JANVIER_1970: u64 = 3;

/// Une quête de la semaine demande 5 fois ce que demande la même quête du
/// jour.
const CIBLE_DE_LA_SEMAINE_PAR_CIBLE_DU_JOUR: u32 = 5;

/// La récompense vaut l'argent de ce nombre de coffres de la zone ouverte la
/// plus avancée, chacun tiré dans sa fourchette, comme la bourse d'Ansgar
/// (section 13.8).
const COFFRES_D_UNE_QUETE_DU_JOUR: u32 = 2;
const COFFRES_D_UNE_QUETE_DE_LA_SEMAINE: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Periode {
    /// De minuit à minuit UTC, comme le cycle du jour (Q109).
    Jour,
    /// Du lundi à minuit UTC au lundi suivant.
    Semaine,
}

impl Periode {
    pub const TOUTES: [Periode; 2] = [Periode::Jour, Periode::Semaine];

    /// Le numéro du jour, ou de la semaine, qui contient cet instant.
    pub fn numero(self, maintenant: Horodatage) -> u64 {
        let jour = maintenant.secondes() / SECONDES_PAR_JOUR;
        match self {
            Periode::Jour => jour,
            Periode::Semaine => {
                (jour + JOURS_DEPUIS_LE_LUNDI_AU_1ER_JANVIER_1970) / JOURS_PAR_SEMAINE
            }
        }
    }

    /// Le temps qui reste avant les quêtes suivantes.
    pub fn duree_restante(self, maintenant: Horodatage) -> Duration {
        let suivante = self.numero(maintenant) + 1;
        let fin_en_jours = match self {
            Periode::Jour => suivante,
            Periode::Semaine => {
                suivante * JOURS_PAR_SEMAINE - JOURS_DEPUIS_LE_LUNDI_AU_1ER_JANVIER_1970
            }
        };
        maintenant.duree_jusqu_a(Horodatage::depuis_secondes(
            fin_en_jours * SECONDES_PAR_JOUR,
        ))
    }

    /// Les quêtes de ce jour ou de cette semaine, les mêmes pour tous les
    /// joueurs : trois objectifs différents, tirés sur un générateur propre à
    /// la période.
    pub fn quetes(self, numero: u64) -> [Quete; QUETES_PAR_PERIODE] {
        let mut aleatoire = GenerateurAleatoire::new(graine(self, numero, ""));
        let mut objectifs = Objectif::TOUS;
        // Les premiers objectifs d'un mélange partiel, sans doublon.
        for position in 0..QUETES_PAR_PERIODE {
            let derniere = (objectifs.len() - 1) as u32;
            let tiree = tirer_entre(position as u32, derniere, &mut aleatoire) as usize;
            objectifs.swap(position, tiree);
        }
        std::array::from_fn(|position| Quete {
            periode: self,
            objectif: objectifs[position],
        })
    }

    fn coffres_de_la_recompense(self) -> u32 {
        match self {
            Periode::Jour => COFFRES_D_UNE_QUETE_DU_JOUR,
            Periode::Semaine => COFFRES_D_UNE_QUETE_DE_LA_SEMAINE,
        }
    }
}

/// Ce qu'une quête demande de faire. Chacun est possible dès le début de la
/// partie, dans la console comme sur Discord, et le reste jusqu'au bout :
/// aucun ne dépend d'un personnage du camp ni des autres joueurs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Objectif {
    /// Mis KO ou capturés (section 7), en expédition, quelle que soit
    /// l'issue du combat ; ni les monstres d'un humain, ni un Apôtre.
    VaincreDesMonstresSauvages,
    /// En combat, au filet ou après une victoire (section 11.2).
    CapturerDesMonstres,
    /// En expédition manuelle, zones, annexes et Tour sans fond comprises.
    OuvrirDesCoffres,
    /// Lâchées par les monstres sauvages vaincus (section 14.3).
    RecolterDesRessources,
    /// Jusqu'à la dernière étape, dans une zone, une annexe ou une zone
    /// légendaire ; ni la Tour sans fond, qui n'en a pas, ni l'Éclipse.
    MenerDesExpeditionsAuBout,
    /// Comptées à l'accueil (section 12.7).
    ReussirDesExpeditionsAutomatiques,
}

impl Objectif {
    /// Dans l'ordre des tirages et de docs/contenu-v1.md, section 50.
    pub const TOUS: [Objectif; 6] = [
        Objectif::VaincreDesMonstresSauvages,
        Objectif::CapturerDesMonstres,
        Objectif::OuvrirDesCoffres,
        Objectif::RecolterDesRessources,
        Objectif::MenerDesExpeditionsAuBout,
        Objectif::ReussirDesExpeditionsAutomatiques,
    ];

    /// L'identifiant gardé par la sauvegarde.
    pub fn id(self) -> &'static str {
        match self {
            Objectif::VaincreDesMonstresSauvages => "vaincre-des-monstres-sauvages",
            Objectif::CapturerDesMonstres => "capturer-des-monstres",
            Objectif::OuvrirDesCoffres => "ouvrir-des-coffres",
            Objectif::RecolterDesRessources => "recolter-des-ressources",
            Objectif::MenerDesExpeditionsAuBout => "mener-des-expeditions-au-bout",
            Objectif::ReussirDesExpeditionsAutomatiques => "reussir-des-expeditions-automatiques",
        }
    }

    pub fn depuis_id(id: &str) -> Option<Objectif> {
        Objectif::TOUS
            .into_iter()
            .find(|objectif| objectif.id() == id)
    }

    /// Environ deux ou trois expéditions manuelles d'une soirée.
    fn cible_du_jour(self) -> u32 {
        match self {
            Objectif::VaincreDesMonstresSauvages => 15,
            Objectif::CapturerDesMonstres => 3,
            Objectif::OuvrirDesCoffres => 2,
            Objectif::RecolterDesRessources => 5,
            Objectif::MenerDesExpeditionsAuBout => 2,
            Objectif::ReussirDesExpeditionsAutomatiques => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quete {
    pub periode: Periode,
    pub objectif: Objectif,
}

impl Quete {
    pub fn cible(self) -> u32 {
        let cible = self.objectif.cible_du_jour();
        match self.periode {
            Periode::Jour => cible,
            Periode::Semaine => cible * CIBLE_DE_LA_SEMAINE_PAR_CIBLE_DU_JOUR,
        }
    }

    /// L'argent que la récompense peut rapporter dans cette zone, de la plus
    /// petite somme à la plus grande.
    pub fn argent_possible(self, zone: &Zone) -> (u32, u32) {
        let coffres = self.periode.coffres_de_la_recompense();
        let fourchette = zone.coffre().argent();
        (
            fourchette.start().saturating_mul(coffres),
            fourchette.end().saturating_mul(coffres),
        )
    }

    /// Une quête de la semaine rapporte aussi un booster.
    pub fn donne_un_booster(self) -> bool {
        self.periode == Periode::Semaine
    }

    /// Ce que verse la quête accomplie dans cette période, avec `zone`, la
    /// zone ouverte la plus avancée. Tirages, sur un générateur propre à la
    /// quête et à sa période, pour ne rien changer aux autres tirages de la
    /// partie : l'argent de chaque coffre, puis le booster.
    pub fn recompense(self, numero: u64, zone: &Zone) -> Inventaire {
        let mut aleatoire =
            GenerateurAleatoire::new(graine(self.periode, numero, self.objectif.id()));
        let fourchette = zone.coffre().argent();
        let argent = (0..self.periode.coffres_de_la_recompense())
            .map(|_| tirer_entre(*fourchette.start(), *fourchette.end(), &mut aleatoire))
            .fold(0_u32, u32::saturating_add);
        let mut recompense = Inventaire::default();
        recompense.ajouter_argent(argent);
        if self.donne_un_booster() {
            let dernier = (Booster::TOUS.len() - 1) as u32;
            let booster = Booster::TOUS[tirer_entre(0, dernier, &mut aleatoire) as usize];
            recompense.ajouter_boosters(booster, 1);
        }
        recompense
    }
}

/// Où en est une quête : son progrès, qui ne dépasse pas sa cible, et si sa
/// récompense est versée.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EtatDeQuete {
    pub quete: Quete,
    pub progres: u32,
    pub accomplie: bool,
}

/// Les quêtes d'un jour ou d'une semaine, telles que le joueur les a
/// avancées.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Avancement {
    pub numero: u64,
    pub quetes: Vec<EtatDeQuete>,
}

impl Avancement {
    fn nouveau(periode: Periode, numero: u64) -> Self {
        Self {
            numero,
            quetes: periode
                .quetes(numero)
                .into_iter()
                .map(|quete| EtatDeQuete {
                    quete,
                    progres: 0,
                    accomplie: false,
                })
                .collect(),
        }
    }
}

/// L'avancement des quêtes du jour et de la semaine en cours. Celui d'une
/// période passée ne sert plus : ses quêtes sont remplacées par celles de la
/// période suivante, à zéro, sans rien à réclamer.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JournalDesQuetes {
    jour: Option<Avancement>,
    semaine: Option<Avancement>,
}

/// Une quête que le joueur vient d'accomplir, et ce qu'elle a versé au
/// coffre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueteAccomplie {
    pub quete: Quete,
    pub recompense: Inventaire,
}

impl JournalDesQuetes {
    /// Tel que la sauvegarde l'a gardé : ses quêtes restent celles de la
    /// période, même si le tirage venait à changer.
    pub fn new(jour: Option<Avancement>, semaine: Option<Avancement>) -> Self {
        Self { jour, semaine }
    }

    /// Pour la sauvegarde.
    pub fn avancement(&self, periode: Periode) -> Option<&Avancement> {
        match periode {
            Periode::Jour => self.jour.as_ref(),
            Periode::Semaine => self.semaine.as_ref(),
        }
    }

    /// Les quêtes de la période en cours, à zéro si le joueur ne les a pas
    /// encore avancées.
    pub fn quetes(&self, periode: Periode, maintenant: Horodatage) -> Vec<EtatDeQuete> {
        let numero = periode.numero(maintenant);
        match self.avancement(periode) {
            Some(avancement) if avancement.numero == numero => avancement.quetes.clone(),
            _ => Avancement::nouveau(periode, numero).quetes,
        }
    }

    /// Ajoute `nombre` aux quêtes de cet objectif, du jour et de la semaine
    /// en cours. Rend celles qui viennent d'atteindre leur cible : leur
    /// récompense reste à verser.
    pub(crate) fn progresser(
        &mut self,
        objectif: Objectif,
        nombre: u32,
        maintenant: Horodatage,
    ) -> Vec<Quete> {
        if nombre == 0 {
            return Vec::new();
        }
        let mut accomplies = Vec::new();
        for periode in Periode::TOUTES {
            let numero = periode.numero(maintenant);
            let place = match periode {
                Periode::Jour => &mut self.jour,
                Periode::Semaine => &mut self.semaine,
            };
            let avancement = match place {
                Some(avancement) if avancement.numero == numero => avancement,
                _ => place.insert(Avancement::nouveau(periode, numero)),
            };
            for etat in &mut avancement.quetes {
                if etat.quete.objectif != objectif || etat.accomplie {
                    continue;
                }
                let cible = etat.quete.cible();
                etat.progres = etat.progres.saturating_add(nombre).min(cible);
                if etat.progres >= cible {
                    etat.accomplie = true;
                    accomplies.push(etat.quete);
                }
            }
        }
        accomplies
    }
}

#[cfg(test)]
impl JournalDesQuetes {
    /// Les quêtes du jour et de la semaine de cet instant, toutes accomplies :
    /// un test qui compte l'argent ou les objets du joueur n'a pas à compter
    /// leurs récompenses.
    pub(crate) fn tout_accompli(maintenant: Horodatage) -> Self {
        let accompli = |periode: Periode| {
            let mut avancement = Avancement::nouveau(periode, periode.numero(maintenant));
            for etat in &mut avancement.quetes {
                etat.progres = etat.quete.cible();
                etat.accomplie = true;
            }
            avancement
        };
        Self::new(
            Some(accompli(Periode::Jour)),
            Some(accompli(Periode::Semaine)),
        )
    }
}

/// FNV-1a de la période et du texte, mêlé au numéro, comme la graine de la
/// météo : elle ne dépend ni de la plateforme ni de la version de Rust.
fn graine(periode: Periode, numero: u64, texte: &str) -> u64 {
    let prefixe = match periode {
        Periode::Jour => "quetes-du-jour",
        Periode::Semaine => "quetes-de-la-semaine",
    };
    let empreinte = prefixe
        .bytes()
        .chain(texte.bytes())
        .fold(0xCBF2_9CE4_8422_2325_u64, |empreinte, octet| {
            (empreinte ^ u64::from(octet)).wrapping_mul(0x0100_0000_01B3)
        });
    empreinte ^ numero.wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::donnees_de_test::zone;
    use std::collections::HashSet;

    fn a(jours: u64, heures: u64) -> Horodatage {
        Horodatage::depuis_secondes(jours * SECONDES_PAR_JOUR + heures * 3_600)
    }

    #[test]
    fn le_jour_change_a_minuit_utc_et_la_semaine_le_lundi() {
        assert_eq!(Periode::Jour.numero(a(0, 23)), 0);
        assert_eq!(Periode::Jour.numero(a(1, 0)), 1);
        // Jeudi 1er janvier 1970 au dimanche 4 : semaine 0 ; lundi 5 :
        // semaine 1.
        assert_eq!(Periode::Semaine.numero(a(0, 0)), 0);
        assert_eq!(Periode::Semaine.numero(a(3, 23)), 0);
        assert_eq!(Periode::Semaine.numero(a(4, 0)), 1);
        assert_eq!(Periode::Semaine.numero(a(10, 23)), 1);
        assert_eq!(Periode::Semaine.numero(a(11, 0)), 2);
        // Le vendredi 6 novembre 2026 est dans la même semaine que le lundi
        // 2, pas que le dimanche 1er.
        let lundi = Horodatage::depuis_secondes(1_793_577_600);
        let dimanche = Horodatage::depuis_secondes(1_793_577_600 - 1);
        let vendredi = lundi.plus(Duration::from_secs(4 * SECONDES_PAR_JOUR));
        assert_eq!(
            Periode::Semaine.numero(vendredi),
            Periode::Semaine.numero(lundi)
        );
        assert_eq!(
            Periode::Semaine.numero(dimanche) + 1,
            Periode::Semaine.numero(lundi)
        );
    }

    #[test]
    fn le_temps_restant_mene_aux_quetes_suivantes() {
        assert_eq!(
            Periode::Jour.duree_restante(a(5, 21)),
            Duration::from_secs(3 * 3_600)
        );
        // Du jeudi 1er janvier 1970 à 0 h au lundi 5 à 0 h.
        assert_eq!(
            Periode::Semaine.duree_restante(a(0, 0)),
            Duration::from_secs(4 * SECONDES_PAR_JOUR)
        );
        let veille = Horodatage::depuis_secondes(11 * SECONDES_PAR_JOUR - 1);
        assert_eq!(
            Periode::Semaine.duree_restante(veille),
            Duration::from_secs(1)
        );
    }

    #[test]
    fn trois_objectifs_differents_par_periode_les_memes_pour_tous() {
        let mut tires = HashSet::new();
        for numero in 0..200 {
            for periode in Periode::TOUTES {
                let quetes = periode.quetes(numero);
                assert_eq!(quetes, periode.quetes(numero));
                let objectifs: HashSet<Objectif> =
                    quetes.iter().map(|quete| quete.objectif).collect();
                assert_eq!(objectifs.len(), QUETES_PAR_PERIODE);
                assert!(quetes.iter().all(|quete| quete.periode == periode));
                tires.extend(objectifs);
            }
        }
        // Chaque objectif finit par être tiré.
        assert_eq!(tires.len(), Objectif::TOUS.len());
        // Deux jours qui se suivent n'ont pas toujours les mêmes quêtes.
        assert!(
            (0..30).any(|numero| Periode::Jour.quetes(numero) != Periode::Jour.quetes(numero + 1))
        );
    }

    #[test]
    fn la_cible_de_la_semaine_vaut_cinq_fois_celle_du_jour() {
        for objectif in Objectif::TOUS {
            let du_jour = Quete {
                periode: Periode::Jour,
                objectif,
            };
            let de_la_semaine = Quete {
                periode: Periode::Semaine,
                objectif,
            };
            assert_eq!(de_la_semaine.cible(), 5 * du_jour.cible());
            assert_eq!(Objectif::depuis_id(objectif.id()), Some(objectif));
        }
        assert_eq!(Objectif::depuis_id("inconnu"), None);
    }

    /// Un objectif tiré à la fois pour le jour et pour la semaine 0.
    fn objectif_commun(maintenant: Horodatage) -> Objectif {
        let du_jour = Periode::Jour.quetes(Periode::Jour.numero(maintenant));
        let de_la_semaine = Periode::Semaine.quetes(Periode::Semaine.numero(maintenant));
        du_jour
            .iter()
            .map(|quete| quete.objectif)
            .find(|objectif| {
                de_la_semaine
                    .iter()
                    .any(|quete| quete.objectif == *objectif)
            })
            .expect("deux tirages de 3 objectifs sur 6 en ont un en commun")
    }

    #[test]
    fn un_progres_compte_pour_le_jour_et_la_semaine_puis_accomplit_une_fois() {
        let maintenant = a(0, 10);
        let objectif = objectif_commun(maintenant);
        let du_jour = Quete {
            periode: Periode::Jour,
            objectif,
        };
        let mut journal = JournalDesQuetes::default();
        assert!(journal.progresser(objectif, 0, maintenant).is_empty());
        assert_eq!(journal, JournalDesQuetes::default());
        let presque = du_jour.cible() - 1;
        assert!(journal.progresser(objectif, presque, maintenant).is_empty());
        assert_eq!(
            journal.progresser(objectif, 5, maintenant),
            vec![du_jour],
            "le progrès s'arrête à la cible et accomplit la quête du jour"
        );
        let etat = journal
            .quetes(Periode::Jour, maintenant)
            .into_iter()
            .find(|etat| etat.quete == du_jour)
            .unwrap();
        assert_eq!(etat.progres, du_jour.cible());
        assert!(etat.accomplie);
        let de_la_semaine = journal
            .quetes(Periode::Semaine, maintenant)
            .into_iter()
            .find(|etat| etat.quete.objectif == objectif)
            .unwrap();
        assert_eq!(de_la_semaine.progres, presque + 5);
        assert!(!de_la_semaine.accomplie);
        // Accomplie, la quête du jour ne rapporte plus rien.
        let reste = de_la_semaine.quete.cible() - de_la_semaine.progres;
        assert_eq!(
            journal.progresser(objectif, reste, maintenant),
            vec![de_la_semaine.quete]
        );
        assert!(journal.progresser(objectif, 100, maintenant).is_empty());
    }

    #[test]
    fn le_lendemain_les_quetes_repartent_de_zero() {
        let maintenant = a(0, 10);
        let objectif = objectif_commun(maintenant);
        let mut journal = JournalDesQuetes::default();
        journal.progresser(objectif, 1, maintenant);
        let lendemain = a(1, 0);
        assert!(
            journal
                .quetes(Periode::Jour, lendemain)
                .iter()
                .all(|etat| etat.progres == 0 && !etat.accomplie)
        );
        assert_eq!(
            journal.quetes(Periode::Jour, lendemain),
            JournalDesQuetes::default().quetes(Periode::Jour, lendemain)
        );
        // La semaine, elle, garde son progrès jusqu'au lundi.
        assert!(
            journal
                .quetes(Periode::Semaine, lendemain)
                .iter()
                .any(|etat| etat.progres == 1)
        );
        // Un objectif hors des quêtes du jour n'avance rien ce jour-là.
        let absent = Objectif::TOUS
            .into_iter()
            .find(|objectif| {
                !Periode::Jour
                    .quetes(1)
                    .iter()
                    .any(|quete| quete.objectif == *objectif)
            })
            .unwrap();
        journal.progresser(absent, 1, lendemain);
        assert!(
            journal
                .quetes(Periode::Jour, lendemain)
                .iter()
                .all(|etat| etat.progres == 0)
        );
    }

    #[test]
    fn la_recompense_vaut_l_argent_de_coffres_de_la_zone() {
        // 10 pièces par coffre, sans tirage.
        let zone = zone("lisiere");
        let du_jour = Quete {
            periode: Periode::Jour,
            objectif: Objectif::OuvrirDesCoffres,
        };
        let recompense = du_jour.recompense(7, &zone);
        assert_eq!(recompense.argent(), 20);
        assert!(recompense.boosters().is_empty());
        assert_eq!(du_jour.argent_possible(&zone), (20, 20));
        let de_la_semaine = Quete {
            periode: Periode::Semaine,
            ..du_jour
        };
        let recompense = de_la_semaine.recompense(7, &zone);
        assert_eq!(recompense.argent(), 50);
        assert_eq!(recompense.boosters().len(), 1);
        assert_eq!(recompense.boosters()[0].1, 1);
        assert_eq!(recompense, de_la_semaine.recompense(7, &zone));
    }
}
