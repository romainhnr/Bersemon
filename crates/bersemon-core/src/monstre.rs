//! Monstre : un individu d'une espèce, avec son éventuel surnom, son niveau,
//! son expérience, ses valeurs internes, ses attaques, ses PV et son statut
//! du moment (sections 5 et 7).

use crate::aleatoire::Aleatoire;
use crate::attaque::{Attaque, AttaquesMonstre, ErreurAttaquesMonstre};
use crate::espece::Espece;
use crate::experience::experience_requise;
use crate::niveau::Niveau;
use crate::nom::Nom;
use crate::stats::{Stats, ValeursInternes, calculer_stats};
use crate::statut::Statut;

/// Un monstre sauvage sur 512 est shiny (section 5.1).
pub const CHANCES_SHINY: u32 = 512;

/// Tirage du shiny d'un monstre sauvage, fait à la rencontre.
pub fn tirer_shiny(aleatoire: &mut dyn Aleatoire) -> bool {
    aleatoire.tirer(1..=CHANCES_SHINY) == 1
}

/// Ce qu'a produit un gain d'XP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RapportExperience {
    pub experience_gagnee: u32,
    pub niveau_avant: Niveau,
    pub niveau_apres: Niveau,
    pub attaques_apprises: Vec<Attaque>,
    /// Attaques que le monstre n'a pas pu apprendre faute de place : pour
    /// chacune, dans l'ordre, le joueur oublie une attaque connue
    /// (`Monstre::remplacer_attaque`) ou renonce à la nouvelle (Q33).
    pub attaques_en_attente: Vec<Attaque>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monstre {
    espece: Espece,
    surnom: Option<Nom>,
    niveau: Niveau,
    experience: u32,
    valeurs_internes: ValeursInternes,
    attaques: AttaquesMonstre,
    shiny: bool,
    pv_actuels: u32,
    statut: Option<Statut>,
}

impl Monstre {
    /// Crée un monstre avec tous ses PV, sans statut, et l'XP minimale de son
    /// niveau. Le shiny est reçu déjà décidé : il ne se tire que pour un
    /// monstre sauvage (`tirer_shiny`).
    pub fn new(
        espece: Espece,
        niveau: Niveau,
        valeurs_internes: ValeursInternes,
        attaques: AttaquesMonstre,
        shiny: bool,
    ) -> Self {
        let mut monstre = Self {
            espece,
            surnom: None,
            niveau,
            experience: experience_requise(niveau),
            valeurs_internes,
            attaques,
            shiny,
            pv_actuels: 0,
            statut: None,
        };
        monstre.soigner_completement();
        monstre
    }

    /// Comme `new`, avec les 4 dernières attaques de la liste d'apprentissage
    /// de l'espèce jusqu'à ce niveau (Q33). Échoue si l'espèce n'apprend
    /// aucune attaque jusqu'à ce niveau.
    pub fn creer(
        espece: Espece,
        niveau: Niveau,
        valeurs_internes: ValeursInternes,
        shiny: bool,
    ) -> Result<Self, ErreurAttaquesMonstre> {
        let connues: Vec<Attaque> = espece
            .apprentissage()
            .iter()
            .filter(|apprise| apprise.niveau <= niveau)
            .map(|apprise| apprise.attaque.clone())
            .collect();
        let debut = connues.len().saturating_sub(AttaquesMonstre::MAX);
        let attaques = AttaquesMonstre::new(connues[debut..].to_vec())?;
        Ok(Self::new(espece, niveau, valeurs_internes, attaques, shiny))
    }

    pub fn espece(&self) -> &Espece {
        &self.espece
    }

    pub fn surnom(&self) -> Option<&Nom> {
        self.surnom.as_ref()
    }

    /// Le surnom s'il y en a un, sinon le nom de l'espèce (section 5.5).
    pub fn nom(&self) -> &str {
        self.surnom.as_ref().map_or(self.espece.nom(), Nom::as_str)
    }

    /// `None` efface le surnom.
    pub fn renommer(&mut self, surnom: Option<Nom>) {
        self.surnom = surnom;
    }

    pub fn niveau(&self) -> Niveau {
        self.niveau
    }

    /// XP totale accumulée depuis le niveau 1.
    pub fn experience(&self) -> u32 {
        self.experience
    }

    pub fn valeurs_internes(&self) -> &ValeursInternes {
        &self.valeurs_internes
    }

    pub fn attaques(&self) -> &AttaquesMonstre {
        &self.attaques
    }

    /// Recalculées à chaque appel plutôt que mémorisées, pour qu'elles
    /// suivent sans risque d'oubli un changement de niveau ou d'espèce.
    pub fn stats(&self) -> Stats<u32> {
        calculer_stats(
            self.espece.stats_de_base(),
            &self.valeurs_internes,
            self.niveau,
        )
    }

    pub fn pv_actuels(&self) -> u32 {
        self.pv_actuels
    }

    /// Un monstre à 0 PV est KO ; il n'y a pas de mort définitive (section
    /// 5.7).
    pub fn est_ko(&self) -> bool {
        self.pv_actuels == 0
    }

    pub fn est_shiny(&self) -> bool {
        self.shiny
    }

    pub fn statut(&self) -> Option<Statut> {
        self.statut
    }

    /// Un monstre KO, déjà atteint d'un statut ou immunisé ne peut pas en
    /// recevoir un nouveau (section 10.5).
    pub fn peut_subir(&self, statut: Statut) -> bool {
        !self.est_ko() && self.statut.is_none() && !statut.epargne(self.espece.types())
    }

    /// Rend `false`, sans rien changer, si le monstre ne peut pas subir ce
    /// statut.
    pub fn infliger_statut(&mut self, statut: Statut) -> bool {
        let inflige = self.peut_subir(statut);
        if inflige {
            self.statut = Some(statut);
        }
        inflige
    }

    /// Les PV ne descendent pas sous 0 : le surplus de dégâts est perdu.
    pub fn subir_degats(&mut self, degats: u32) {
        self.pv_actuels = self.pv_actuels.saturating_sub(degats);
    }

    /// Rend tous les PV et guérit le statut, même d'un monstre KO, comme au
    /// retour au camp (section 13.1).
    pub fn soigner_completement(&mut self) {
        self.pv_actuels = self.stats().pv;
        self.statut = None;
    }

    /// Soin partiel, comme celui d'une potion : ne relève pas un monstre KO,
    /// ne dépasse pas les PV max et ne touche pas au statut. Rend les PV
    /// réellement rendus.
    pub fn soigner(&mut self, pv: u32) -> u32 {
        if self.est_ko() {
            return 0;
        }
        let rendus = pv.min(self.stats().pv - self.pv_actuels);
        self.pv_actuels += rendus;
        rendus
    }

    /// Ajoute l'XP et fait monter le monstre d'autant de niveaux qu'elle le
    /// permet, jusqu'au niveau 100 (section 7). À chaque niveau atteint, il
    /// apprend les attaques prévues tant qu'il lui reste de la place ; les
    /// autres sont rendues en attente du choix du joueur. Les PV actuels
    /// augmentent d'autant que les PV max, sauf pour un monstre KO, qui le
    /// reste.
    pub fn gagner_experience(&mut self, experience: u32) -> RapportExperience {
        let niveau_avant = self.niveau;
        let pv_max_avant = self.stats().pv;
        let plafond = Niveau::new(Niveau::MAX).map_or(u32::MAX, experience_requise);
        self.experience = self.experience.saturating_add(experience).min(plafond);

        let mut attaques_apprises = Vec::new();
        let mut attaques_en_attente = Vec::new();
        while let Ok(suivant) = Niveau::new(self.niveau.valeur() + 1) {
            if self.experience < experience_requise(suivant) {
                break;
            }
            self.niveau = suivant;
            for attaque in self.espece.attaques_du_niveau(suivant) {
                if self.attaques.connait(attaque.id()) {
                    continue;
                }
                if self.attaques.est_complete() {
                    attaques_en_attente.push(attaque.clone());
                } else if self.attaques.ajouter(attaque.clone()).is_ok() {
                    attaques_apprises.push(attaque.clone());
                }
            }
        }

        if !self.est_ko() {
            self.pv_actuels += self.stats().pv - pv_max_avant;
        }
        RapportExperience {
            experience_gagnee: experience,
            niveau_avant,
            niveau_apres: self.niveau,
            attaques_apprises,
            attaques_en_attente,
        }
    }

    /// Oublie l'attaque en `position` pour apprendre `attaque` (Q33).
    pub fn remplacer_attaque(
        &mut self,
        position: usize,
        attaque: Attaque,
    ) -> Result<(), ErreurAttaquesMonstre> {
        self.attaques.remplacer(position, attaque)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attaque::{AttaqueId, Categorie};
    use crate::donnees_de_test::attaque;
    use crate::espece::{AttaqueApprise, EspeceId, Role};
    use crate::stats::StatsDeBase;
    use crate::types::{Type, TypesMonstre};

    fn attaque_de_test(id: &str) -> Attaque {
        attaque(id, Type::Normal, Categorie::Physique, 40, 100)
    }

    fn attaques_de_test() -> AttaquesMonstre {
        AttaquesMonstre::new(vec![attaque_de_test("attaque-de-test")]).unwrap()
    }

    fn apprise(niveau: u8, id: &str) -> AttaqueApprise {
        AttaqueApprise {
            niveau: Niveau::new(niveau).unwrap(),
            attaque: attaque_de_test(id),
        }
    }

    /// Apprend a et b au niveau 1, c au 3, d au 5, e au 6, f et g au 7.
    fn espece_de_test() -> Espece {
        let base: StatsDeBase = Stats {
            pv: 50,
            attaque: 50,
            defense: 50,
            attaque_speciale: 50,
            defense_speciale: 50,
            vitesse: 50,
        };
        Espece::new(
            EspeceId::new("espece-de-test"),
            "Espèce de test",
            TypesMonstre::new(&[Type::Feu]).unwrap(),
            Role::Offensif,
            base,
            64,
            vec![
                apprise(1, "a"),
                apprise(1, "b"),
                apprise(3, "c"),
                apprise(5, "d"),
                apprise(6, "e"),
                apprise(7, "f"),
                apprise(7, "g"),
            ],
        )
    }

    fn vi_maximales() -> ValeursInternes {
        ValeursInternes::new(Stats {
            pv: 31,
            attaque: 31,
            defense: 31,
            attaque_speciale: 31,
            defense_speciale: 31,
            vitesse: 31,
        })
        .unwrap()
    }

    fn monstre_de_test(niveau: u8) -> Monstre {
        Monstre::new(
            espece_de_test(),
            Niveau::new(niveau).unwrap(),
            vi_maximales(),
            attaques_de_test(),
            false,
        )
    }

    fn cree(niveau: u8) -> Monstre {
        Monstre::creer(
            espece_de_test(),
            Niveau::new(niveau).unwrap(),
            vi_maximales(),
            false,
        )
        .unwrap()
    }

    fn ids(attaques: &[Attaque]) -> Vec<&str> {
        attaques
            .iter()
            .map(|attaque| attaque.id().as_str())
            .collect()
    }

    #[test]
    fn nouveau_monstre_a_tous_ses_pv_et_l_xp_de_son_niveau() {
        let monstre = monstre_de_test(5);
        assert_eq!(monstre.stats().pv, 21);
        assert_eq!(monstre.pv_actuels(), 21);
        assert_eq!(monstre.experience(), 125);
        assert!(!monstre.est_ko());
    }

    #[test]
    fn stats_calculees_depuis_espece_niveau_et_valeurs_internes() {
        let monstre = monstre_de_test(100);
        assert_eq!(monstre.stats().pv, 241);
        assert_eq!(monstre.stats().attaque, 136);
    }

    #[test]
    fn degats_partiels_puis_ko_sans_pv_negatifs() {
        let mut monstre = monstre_de_test(5);
        monstre.subir_degats(20);
        assert_eq!(monstre.pv_actuels(), 1);
        assert!(!monstre.est_ko());
        monstre.subir_degats(50);
        assert_eq!(monstre.pv_actuels(), 0);
        assert!(monstre.est_ko());
    }

    #[test]
    fn soin_complet_releve_un_monstre_ko() {
        let mut monstre = monstre_de_test(5);
        monstre.subir_degats(u32::MAX);
        monstre.soigner_completement();
        assert_eq!(monstre.pv_actuels(), 21);
        assert!(!monstre.est_ko());
    }

    #[test]
    fn shiny_conserve() {
        let monstre = Monstre::new(
            espece_de_test(),
            Niveau::new(5).unwrap(),
            vi_maximales(),
            attaques_de_test(),
            true,
        );
        assert!(monstre.est_shiny());
    }

    #[test]
    fn un_seul_statut_a_la_fois() {
        let mut monstre = monstre_de_test(5);
        assert!(monstre.infliger_statut(Statut::Paralysie));
        assert!(!monstre.infliger_statut(Statut::Poison));
        assert_eq!(monstre.statut(), Some(Statut::Paralysie));
    }

    #[test]
    fn un_monstre_ko_ne_recoit_pas_de_statut() {
        let mut monstre = monstre_de_test(5);
        monstre.subir_degats(u32::MAX);
        assert!(!monstre.infliger_statut(Statut::Poison));
        assert_eq!(monstre.statut(), None);
    }

    #[test]
    fn soin_complet_guerit_le_statut() {
        let mut monstre = monstre_de_test(5);
        monstre.infliger_statut(Statut::Poison);
        monstre.soigner_completement();
        assert_eq!(monstre.statut(), None);
    }

    #[test]
    fn shiny_sur_un_tirage_sur_512() {
        use crate::aleatoire::AleatoireScripte;
        assert!(tirer_shiny(&mut AleatoireScripte::new([1])));
        assert!(!tirer_shiny(&mut AleatoireScripte::new([512])));
    }

    #[test]
    fn creer_donne_les_quatre_dernieres_attaques_jusqu_au_niveau() {
        assert_eq!(ids(cree(1).attaques().liste()), vec!["a", "b"]);
        assert_eq!(ids(cree(5).attaques().liste()), vec!["a", "b", "c", "d"]);
        assert_eq!(ids(cree(7).attaques().liste()), vec!["d", "e", "f", "g"]);
    }

    #[test]
    fn creer_echoue_sans_attaque_jusqu_au_niveau() {
        let espece = Espece::new(
            EspeceId::new("tardive"),
            "Tardive",
            TypesMonstre::new(&[Type::Feu]).unwrap(),
            Role::Offensif,
            *espece_de_test().stats_de_base(),
            64,
            vec![apprise(3, "c")],
        );
        assert_eq!(
            Monstre::creer(espece, Niveau::new(1).unwrap(), vi_maximales(), false),
            Err(ErreurAttaquesMonstre::Aucune)
        );
    }

    #[test]
    fn un_niveau_gagne_augmente_les_pv_actuels_d_autant_que_les_pv_max() {
        let mut monstre = cree(5);
        monstre.subir_degats(11);
        let rapport = monstre.gagner_experience(91);
        assert_eq!(rapport.niveau_apres, Niveau::new(6).unwrap());
        assert_eq!(monstre.experience(), 216);
        // 21 PV max au niveau 5, 23 au niveau 6.
        assert_eq!(monstre.pv_actuels(), 12);
    }

    #[test]
    fn pas_de_niveau_sans_assez_d_xp() {
        let mut monstre = cree(5);
        let rapport = monstre.gagner_experience(90);
        assert_eq!(rapport.niveau_apres, Niveau::new(5).unwrap());
        assert_eq!(monstre.experience(), 215);
    }

    #[test]
    fn plusieurs_niveaux_et_attaques_apprises_puis_en_attente() {
        // Niveau 3 : a, b, c. Le niveau 5 apporte d, le niveau 6 apporte e,
        // qui attend faute de place.
        let mut monstre = cree(3);
        let rapport = monstre.gagner_experience(216 - 27);
        assert_eq!(rapport.niveau_avant, Niveau::new(3).unwrap());
        assert_eq!(rapport.niveau_apres, Niveau::new(6).unwrap());
        assert_eq!(ids(&rapport.attaques_apprises), vec!["d"]);
        assert_eq!(ids(&rapport.attaques_en_attente), vec!["e"]);
        assert_eq!(ids(monstre.attaques().liste()), vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn remplacer_une_attaque_en_attente() {
        let mut monstre = cree(5);
        let rapport = monstre.gagner_experience(91);
        let nouvelle = rapport.attaques_en_attente[0].clone();
        assert_eq!(monstre.remplacer_attaque(0, nouvelle), Ok(()));
        assert_eq!(ids(monstre.attaques().liste()), vec!["e", "b", "c", "d"]);
        assert_eq!(
            monstre.remplacer_attaque(4, attaque_de_test("z")),
            Err(ErreurAttaquesMonstre::PositionInexistante(4))
        );
        assert!(!monstre.attaques().connait(&AttaqueId::new("a")));
    }

    #[test]
    fn xp_plafonnee_au_niveau_100() {
        let mut monstre = monstre_de_test(100);
        let rapport = monstre.gagner_experience(5_000);
        assert_eq!(rapport.niveau_apres, Niveau::new(100).unwrap());
        assert_eq!(monstre.experience(), 1_000_000);
        let mut presque = monstre_de_test(99);
        presque.gagner_experience(u32::MAX);
        assert_eq!(presque.niveau(), Niveau::new(100).unwrap());
        assert_eq!(presque.experience(), 1_000_000);
    }

    #[test]
    fn un_monstre_ko_reste_ko_en_montant_de_niveau() {
        let mut monstre = cree(5);
        monstre.subir_degats(u32::MAX);
        monstre.gagner_experience(91);
        assert!(monstre.est_ko());
    }

    #[test]
    fn le_surnom_remplace_le_nom_de_l_espece() {
        let mut monstre = cree(5);
        assert_eq!(monstre.nom(), "Espèce de test");
        monstre.renommer(Some(Nom::new("Braise").unwrap()));
        assert_eq!(monstre.nom(), "Braise");
        monstre.renommer(None);
        assert_eq!(monstre.nom(), "Espèce de test");
    }

    #[test]
    fn soin_partiel_plafonne_aux_pv_max_et_sans_relever_un_ko() {
        let mut monstre = monstre_de_test(5);
        monstre.subir_degats(10);
        assert_eq!(monstre.soigner(4), 4);
        assert_eq!(monstre.pv_actuels(), 15);
        assert_eq!(monstre.soigner(50), 6);
        assert_eq!(monstre.pv_actuels(), 21);
        monstre.subir_degats(u32::MAX);
        assert_eq!(monstre.soigner(50), 0);
        assert!(monstre.est_ko());
    }
}
