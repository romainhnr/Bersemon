//! Monstres du joueur : son équipe, qui l'accompagne, et ceux qu'il laisse
//! au camp (sections 11.2 et 13.2).

use std::fmt;

use crate::monstre::Monstre;
use crate::nom::Nom;

/// Position d'un monstre dans l'équipe ou au camp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emplacement {
    Equipe(usize),
    Camp(usize),
}

/// Où est allé un monstre capturé (Q12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Placement {
    Equipe(usize),
    Camp(usize),
    /// L'équipe et le camp sont pleins : le monstre est rendu, et le joueur
    /// choisit lequel relâcher (`relacher_pour_capture`), ou renonce à lui.
    PlaceManquante(Monstre),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurCollection {
    EmplacementInexistant(Emplacement),
    EquipePleine,
    CampPlein,
    DernierMonstreDeLEquipe,
    TailleDEquipeDifferente {
        attendue: usize,
        recue: usize,
    },
    /// Le même monstre est choisi deux fois.
    EmplacementEnDouble(Emplacement),
}

impl fmt::Display for ErreurCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurCollection::EmplacementInexistant(emplacement) => {
                write!(f, "aucun monstre à l'emplacement {emplacement:?}")
            }
            ErreurCollection::EquipePleine => write!(
                f,
                "l'équipe compte déjà {} monstres",
                Collection::TAILLE_EQUIPE
            ),
            ErreurCollection::CampPlein => {
                write!(
                    f,
                    "le camp compte déjà {} monstres",
                    Collection::TAILLE_CAMP
                )
            }
            ErreurCollection::DernierMonstreDeLEquipe => {
                write!(f, "l'équipe doit garder au moins un monstre")
            }
            ErreurCollection::TailleDEquipeDifferente { attendue, recue } => {
                write!(f, "équipe de {recue} monstres reçue, {attendue} attendus")
            }
            ErreurCollection::EmplacementEnDouble(emplacement) => {
                write!(f, "le monstre {emplacement:?} est choisi deux fois")
            }
        }
    }
}

impl std::error::Error for ErreurCollection {}

/// L'équipe compte de 1 à 3 monstres et le camp au plus 30 (section 13.2).
/// L'équipe garde toujours au moins un monstre (Q51) : le joueur ne peut donc
/// jamais perdre son dernier monstre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection {
    equipe: Vec<Monstre>,
    camp: Vec<Monstre>,
    /// Une par monstre parti en expédition automatique : il garde sa place
    /// au camp, où il reviendra (section 12.1).
    places_reservees: usize,
}

impl Collection {
    pub const TAILLE_EQUIPE: usize = 3;
    pub const TAILLE_CAMP: usize = 30;

    /// Le monstre de départ forme seul la première équipe.
    pub fn new(premier: Monstre) -> Self {
        Self {
            equipe: vec![premier],
            camp: Vec::new(),
            places_reservees: 0,
        }
    }

    /// Reconstitue la collection depuis une sauvegarde, en revérifiant les
    /// tailles de l'équipe et du camp.
    pub fn restaurer(equipe: Vec<Monstre>, camp: Vec<Monstre>) -> Result<Self, ErreurCollection> {
        if equipe.is_empty() {
            return Err(ErreurCollection::DernierMonstreDeLEquipe);
        }
        if equipe.len() > Self::TAILLE_EQUIPE {
            return Err(ErreurCollection::EquipePleine);
        }
        if camp.len() > Self::TAILLE_CAMP {
            return Err(ErreurCollection::CampPlein);
        }
        Ok(Self {
            equipe,
            camp,
            places_reservees: 0,
        })
    }

    pub fn equipe(&self) -> &[Monstre] {
        &self.equipe
    }

    pub fn camp(&self) -> &[Monstre] {
        &self.camp
    }

    /// Places du camp gardées pour les monstres en expédition automatique.
    pub fn places_reservees(&self) -> usize {
        self.places_reservees
    }

    pub fn monstre(&self, emplacement: Emplacement) -> Option<&Monstre> {
        match emplacement {
            Emplacement::Equipe(index) => self.equipe.get(index),
            Emplacement::Camp(index) => self.camp.get(index),
        }
    }

    pub fn envoyer_au_camp(&mut self, index_equipe: usize) -> Result<(), ErreurCollection> {
        self.verifier(Emplacement::Equipe(index_equipe))?;
        if self.equipe.len() == 1 {
            return Err(ErreurCollection::DernierMonstreDeLEquipe);
        }
        if self.places_libres_au_camp() == 0 {
            return Err(ErreurCollection::CampPlein);
        }
        let monstre = self.equipe.remove(index_equipe);
        self.camp.push(monstre);
        Ok(())
    }

    /// Le monstre rejoint la fin de l'équipe.
    pub fn faire_venir(&mut self, index_camp: usize) -> Result<(), ErreurCollection> {
        self.verifier(Emplacement::Camp(index_camp))?;
        if self.equipe.len() >= Self::TAILLE_EQUIPE {
            return Err(ErreurCollection::EquipePleine);
        }
        let monstre = self.camp.remove(index_camp);
        self.equipe.push(monstre);
        Ok(())
    }

    /// Chacun prend la place de l'autre, ce qui marche même équipe pleine.
    pub fn echanger(
        &mut self,
        index_equipe: usize,
        index_camp: usize,
    ) -> Result<(), ErreurCollection> {
        self.verifier(Emplacement::Equipe(index_equipe))?;
        self.verifier(Emplacement::Camp(index_camp))?;
        std::mem::swap(&mut self.equipe[index_equipe], &mut self.camp[index_camp]);
        Ok(())
    }

    /// Le premier monstre en état de combattre commence chaque combat
    /// (section 13.2).
    pub fn deplacer_dans_l_equipe(
        &mut self,
        de: usize,
        vers: usize,
    ) -> Result<(), ErreurCollection> {
        self.verifier(Emplacement::Equipe(de))?;
        self.verifier(Emplacement::Equipe(vers))?;
        let monstre = self.equipe.remove(de);
        self.equipe.insert(vers, monstre);
        Ok(())
    }

    /// Définitif. La confirmation revient à l'interface.
    pub fn relacher(&mut self, emplacement: Emplacement) -> Result<Monstre, ErreurCollection> {
        self.verifier(emplacement)?;
        match emplacement {
            Emplacement::Equipe(index) => {
                if self.equipe.len() == 1 {
                    return Err(ErreurCollection::DernierMonstreDeLEquipe);
                }
                Ok(self.equipe.remove(index))
            }
            Emplacement::Camp(index) => Ok(self.camp.remove(index)),
        }
    }

    /// `None` efface le surnom.
    pub fn renommer(
        &mut self,
        emplacement: Emplacement,
        surnom: Option<Nom>,
    ) -> Result<(), ErreurCollection> {
        self.verifier(emplacement)?;
        self.monstre_mut(emplacement).renommer(surnom);
        Ok(())
    }

    /// Soin complet de tous les monstres, au retour au camp (section 13.1).
    pub fn soigner_tout(&mut self) {
        for monstre in self.equipe.iter_mut().chain(self.camp.iter_mut()) {
            monstre.soigner_completement();
        }
    }

    /// Le monstre capturé rejoint l'équipe s'il y a de la place, sinon le
    /// camp (Q12).
    pub fn accueillir_capture(&mut self, capture: Monstre) -> Placement {
        if self.equipe.len() < Self::TAILLE_EQUIPE {
            self.equipe.push(capture);
            Placement::Equipe(self.equipe.len() - 1)
        } else if self.places_libres_au_camp() > 0 {
            self.camp.push(capture);
            Placement::Camp(self.camp.len() - 1)
        } else {
            Placement::PlaceManquante(capture)
        }
    }

    /// Relâche le monstre de cet emplacement, et la capture prend sa place
    /// (Q12). Rend le monstre relâché.
    pub fn relacher_pour_capture(
        &mut self,
        emplacement: Emplacement,
        capture: Monstre,
    ) -> Result<Monstre, ErreurCollection> {
        self.verifier(emplacement)?;
        Ok(std::mem::replace(self.monstre_mut(emplacement), capture))
    }

    /// Remplace l'équipe par son état à la fin d'un combat : PV, statuts, XP
    /// et attaques. L'équipe reçue doit avoir la même taille.
    pub fn actualiser_equipe(&mut self, equipe: Vec<Monstre>) -> Result<(), ErreurCollection> {
        if equipe.len() != self.equipe.len() {
            return Err(ErreurCollection::TailleDEquipeDifferente {
                attendue: self.equipe.len(),
                recue: equipe.len(),
            });
        }
        self.equipe = equipe;
        Ok(())
    }

    /// Ce que `retirer_pour_expedition` refuserait. L'équipe garde un
    /// monstre, et un monstre pris dans l'équipe a besoin d'une place libre
    /// au camp, où il reviendra (section 12.1).
    pub(crate) fn verifier_depart(
        &self,
        emplacements: &[Emplacement],
    ) -> Result<(), ErreurCollection> {
        for (position, emplacement) in emplacements.iter().enumerate() {
            self.verifier(*emplacement)?;
            if emplacements[..position].contains(emplacement) {
                return Err(ErreurCollection::EmplacementEnDouble(*emplacement));
            }
        }
        let de_l_equipe = indices_de_l_equipe(emplacements).len();
        if de_l_equipe >= self.equipe.len() {
            return Err(ErreurCollection::DernierMonstreDeLEquipe);
        }
        if de_l_equipe > self.places_libres_au_camp() {
            return Err(ErreurCollection::CampPlein);
        }
        Ok(())
    }

    /// Rend les monstres dans l'ordre des emplacements ; leurs places au camp
    /// restent réservées jusqu'à leur retour.
    pub(crate) fn retirer_pour_expedition(
        &mut self,
        emplacements: &[Emplacement],
    ) -> Result<Vec<Monstre>, ErreurCollection> {
        self.verifier_depart(emplacements)?;
        let monstres = emplacements
            .iter()
            .filter_map(|emplacement| self.monstre(*emplacement).cloned())
            .collect();
        // Du dernier au premier, pour que les positions restant à retirer ne
        // bougent pas.
        let mut equipe = indices_de_l_equipe(emplacements);
        equipe.sort_unstable_by(|a, b| b.cmp(a));
        for index in equipe {
            self.equipe.remove(index);
        }
        let mut camp: Vec<usize> = emplacements
            .iter()
            .filter_map(|emplacement| match emplacement {
                Emplacement::Camp(index) => Some(*index),
                Emplacement::Equipe(_) => None,
            })
            .collect();
        camp.sort_unstable_by(|a, b| b.cmp(a));
        for index in camp {
            self.camp.remove(index);
        }
        self.places_reservees += emplacements.len();
        Ok(monstres)
    }

    /// Les monstres rentrés d'une expédition automatique reprennent, à la
    /// fin du camp, les places qui leur étaient réservées.
    pub(crate) fn accueillir_retour(&mut self, monstres: Vec<Monstre>) {
        debug_assert!(monstres.len() <= self.places_reservees);
        self.places_reservees = self.places_reservees.saturating_sub(monstres.len());
        self.camp.extend(monstres);
    }

    /// À la reprise d'une sauvegarde : une place par monstre parti en
    /// expédition automatique, qui garde la sienne au camp.
    pub(crate) fn definir_places_reservees(
        &mut self,
        nombre: usize,
    ) -> Result<(), ErreurCollection> {
        if self.camp.len() + nombre > Self::TAILLE_CAMP {
            return Err(ErreurCollection::CampPlein);
        }
        self.places_reservees = nombre;
        Ok(())
    }

    fn places_libres_au_camp(&self) -> usize {
        Self::TAILLE_CAMP.saturating_sub(self.camp.len() + self.places_reservees)
    }

    /// Pour les soins hors combat, comme une potion entre deux étapes.
    pub(crate) fn membre_de_l_equipe_mut(&mut self, index: usize) -> Option<&mut Monstre> {
        self.equipe.get_mut(index)
    }

    /// Pour les attaques à choisir après une expédition automatique.
    pub(crate) fn monstre_du_camp_mut(&mut self, index: usize) -> Option<&mut Monstre> {
        self.camp.get_mut(index)
    }

    fn verifier(&self, emplacement: Emplacement) -> Result<(), ErreurCollection> {
        match self.monstre(emplacement) {
            Some(_) => Ok(()),
            None => Err(ErreurCollection::EmplacementInexistant(emplacement)),
        }
    }

    /// Panique si l'emplacement est vide : à appeler après `verifier`.
    fn monstre_mut(&mut self, emplacement: Emplacement) -> &mut Monstre {
        match emplacement {
            Emplacement::Equipe(index) => &mut self.equipe[index],
            Emplacement::Camp(index) => &mut self.camp[index],
        }
    }
}

fn indices_de_l_equipe(emplacements: &[Emplacement]) -> Vec<usize> {
    emplacements
        .iter()
        .filter_map(|emplacement| match emplacement {
            Emplacement::Equipe(index) => Some(*index),
            Emplacement::Camp(_) => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attaque::Categorie;
    use crate::donnees_de_test::{attaque, monstre, stats_uniformes};
    use crate::types::Type;

    /// Monstres distingués par leur surnom.
    fn monstre_nomme(nom: &str) -> Monstre {
        let coup = attaque("coup", Type::Normal, Categorie::Physique, 40, 100);
        let mut monstre = monstre(&[Type::Feu], stats_uniformes(50), 5, vec![coup]);
        monstre.renommer(Some(Nom::new(nom).unwrap()));
        monstre
    }

    fn noms(monstres: &[Monstre]) -> Vec<&str> {
        monstres.iter().map(Monstre::nom).collect()
    }

    /// Équipe de 3 (e1 à e3) et camp plein (c1 à c30).
    fn collection_pleine() -> Collection {
        let mut collection = Collection::new(monstre_nomme("e1"));
        collection.accueillir_capture(monstre_nomme("e2"));
        collection.accueillir_capture(monstre_nomme("e3"));
        for numero in 1..=30 {
            collection.accueillir_capture(monstre_nomme(&format!("c{numero}")));
        }
        collection
    }

    #[test]
    fn les_captures_remplissent_l_equipe_puis_le_camp() {
        let mut collection = Collection::new(monstre_nomme("a"));
        assert_eq!(
            collection.accueillir_capture(monstre_nomme("b")),
            Placement::Equipe(1)
        );
        assert_eq!(
            collection.accueillir_capture(monstre_nomme("c")),
            Placement::Equipe(2)
        );
        assert_eq!(
            collection.accueillir_capture(monstre_nomme("d")),
            Placement::Camp(0)
        );
        assert_eq!(noms(collection.equipe()), vec!["a", "b", "c"]);
        assert_eq!(noms(collection.camp()), vec!["d"]);
    }

    #[test]
    fn capture_rendue_quand_tout_est_plein() {
        let mut collection = collection_pleine();
        let nouveau = monstre_nomme("nouveau");
        assert_eq!(
            collection.accueillir_capture(nouveau.clone()),
            Placement::PlaceManquante(nouveau)
        );
    }

    #[test]
    fn relacher_un_monstre_pour_garder_la_capture() {
        let mut collection = collection_pleine();
        let relache = collection
            .relacher_pour_capture(Emplacement::Equipe(1), monstre_nomme("nouveau"))
            .unwrap();
        assert_eq!(relache.nom(), "e2");
        assert_eq!(noms(collection.equipe()), vec!["e1", "nouveau", "e3"]);
        let relache = collection
            .relacher_pour_capture(Emplacement::Camp(29), monstre_nomme("autre"))
            .unwrap();
        assert_eq!(relache.nom(), "c30");
        assert_eq!(collection.camp()[29].nom(), "autre");
    }

    #[test]
    fn echanges_entre_equipe_et_camp() {
        let mut collection = Collection::new(monstre_nomme("a"));
        collection.accueillir_capture(monstre_nomme("b"));
        collection.accueillir_capture(monstre_nomme("c"));
        collection.accueillir_capture(monstre_nomme("d"));
        assert_eq!(
            collection.faire_venir(0),
            Err(ErreurCollection::EquipePleine)
        );
        collection.echanger(0, 0).unwrap();
        assert_eq!(noms(collection.equipe()), vec!["d", "b", "c"]);
        assert_eq!(noms(collection.camp()), vec!["a"]);
        collection.envoyer_au_camp(1).unwrap();
        assert_eq!(noms(collection.equipe()), vec!["d", "c"]);
        collection.faire_venir(0).unwrap();
        assert_eq!(noms(collection.equipe()), vec!["d", "c", "a"]);
        assert_eq!(noms(collection.camp()), vec!["b"]);
    }

    #[test]
    fn choisir_l_ordre_de_l_equipe() {
        let mut collection = Collection::new(monstre_nomme("a"));
        collection.accueillir_capture(monstre_nomme("b"));
        collection.accueillir_capture(monstre_nomme("c"));
        collection.deplacer_dans_l_equipe(2, 0).unwrap();
        assert_eq!(noms(collection.equipe()), vec!["c", "a", "b"]);
        assert_eq!(
            collection.deplacer_dans_l_equipe(3, 0),
            Err(ErreurCollection::EmplacementInexistant(
                Emplacement::Equipe(3)
            ))
        );
    }

    #[test]
    fn l_equipe_garde_toujours_un_monstre() {
        let mut collection = Collection::new(monstre_nomme("a"));
        collection.accueillir_capture(monstre_nomme("b"));
        collection.envoyer_au_camp(1).unwrap();
        assert_eq!(
            collection.envoyer_au_camp(0),
            Err(ErreurCollection::DernierMonstreDeLEquipe)
        );
        assert_eq!(
            collection.relacher(Emplacement::Equipe(0)),
            Err(ErreurCollection::DernierMonstreDeLEquipe)
        );
        assert_eq!(
            collection
                .relacher(Emplacement::Camp(0))
                .map(|monstre| monstre.nom().to_string()),
            Ok("b".to_string())
        );
        assert!(collection.camp().is_empty());
    }

    #[test]
    fn envoyer_au_camp_plein_refuse() {
        let mut collection = collection_pleine();
        assert_eq!(
            collection.envoyer_au_camp(0),
            Err(ErreurCollection::CampPlein)
        );
    }

    #[test]
    fn renommer_un_monstre_du_camp() {
        let mut collection = Collection::new(monstre_nomme("a"));
        collection.accueillir_capture(monstre_nomme("b"));
        collection
            .renommer(Emplacement::Equipe(1), Some(Nom::new("Ombre").unwrap()))
            .unwrap();
        assert_eq!(collection.equipe()[1].nom(), "Ombre");
        assert_eq!(
            collection.renommer(Emplacement::Camp(0), None),
            Err(ErreurCollection::EmplacementInexistant(Emplacement::Camp(
                0
            )))
        );
    }

    #[test]
    fn soigner_tout_soigne_l_equipe_et_le_camp() {
        let mut collection = collection_pleine();
        let mut blesse = collection.equipe().to_vec();
        blesse[0].subir_degats(u32::MAX);
        collection.actualiser_equipe(blesse).unwrap();
        assert!(collection.equipe()[0].est_ko());
        collection.soigner_tout();
        assert!(!collection.equipe()[0].est_ko());
    }

    #[test]
    fn actualiser_l_equipe_exige_la_meme_taille() {
        let mut collection = Collection::new(monstre_nomme("a"));
        assert_eq!(
            collection.actualiser_equipe(vec![]),
            Err(ErreurCollection::TailleDEquipeDifferente {
                attendue: 1,
                recue: 0,
            })
        );
    }

    #[test]
    fn restaurer_reverifie_les_tailles() {
        let trois = || vec![monstre_nomme("a"), monstre_nomme("b"), monstre_nomme("c")];
        assert!(Collection::restaurer(trois(), trois()).is_ok());
        assert_eq!(
            Collection::restaurer(vec![], trois()),
            Err(ErreurCollection::DernierMonstreDeLEquipe)
        );
        let mut quatre = trois();
        quatre.push(monstre_nomme("d"));
        assert_eq!(
            Collection::restaurer(quatre, vec![]),
            Err(ErreurCollection::EquipePleine)
        );
        let trente_et_un = (0..31)
            .map(|numero| monstre_nomme(&format!("c{numero}")))
            .collect();
        assert_eq!(
            Collection::restaurer(trois(), trente_et_un),
            Err(ErreurCollection::CampPlein)
        );
    }

    #[test]
    fn depart_en_expedition_automatique_depuis_l_equipe_et_le_camp() {
        let mut collection = Collection::new(monstre_nomme("a"));
        for nom in ["b", "c", "d", "e"] {
            collection.accueillir_capture(monstre_nomme(nom));
        }
        let partis = collection
            .retirer_pour_expedition(&[
                Emplacement::Camp(1),
                Emplacement::Equipe(0),
                Emplacement::Equipe(2),
            ])
            .unwrap();
        assert_eq!(noms(&partis), vec!["e", "a", "c"]);
        assert_eq!(noms(collection.equipe()), vec!["b"]);
        assert_eq!(noms(collection.camp()), vec!["d"]);
        assert_eq!(collection.places_reservees(), 3);
        collection.accueillir_retour(partis);
        assert_eq!(noms(collection.camp()), vec!["d", "e", "a", "c"]);
        assert_eq!(collection.places_reservees(), 0);
    }

    #[test]
    fn depart_refuse_sans_rien_changer() {
        let mut collection = Collection::new(monstre_nomme("a"));
        collection.accueillir_capture(monstre_nomme("b"));
        let avant = collection.clone();
        assert_eq!(
            collection.retirer_pour_expedition(&[Emplacement::Equipe(0), Emplacement::Equipe(1)]),
            Err(ErreurCollection::DernierMonstreDeLEquipe)
        );
        assert_eq!(
            collection.retirer_pour_expedition(&[Emplacement::Equipe(1), Emplacement::Equipe(1)]),
            Err(ErreurCollection::EmplacementEnDouble(Emplacement::Equipe(
                1
            )))
        );
        assert_eq!(
            collection.retirer_pour_expedition(&[Emplacement::Camp(0)]),
            Err(ErreurCollection::EmplacementInexistant(Emplacement::Camp(
                0
            )))
        );
        assert_eq!(collection, avant);
    }

    #[test]
    fn les_places_reservees_comptent_dans_la_limite_du_camp() {
        let mut collection = collection_pleine();
        // Un monstre de l'équipe n'aurait pas de place où revenir.
        assert_eq!(
            collection.retirer_pour_expedition(&[Emplacement::Equipe(0)]),
            Err(ErreurCollection::CampPlein)
        );
        // Un monstre du camp garde la sienne : le camp reste plein.
        collection
            .retirer_pour_expedition(&[Emplacement::Camp(0)])
            .unwrap();
        assert_eq!(collection.camp().len(), 29);
        let nouveau = monstre_nomme("nouveau");
        assert_eq!(
            collection.accueillir_capture(nouveau.clone()),
            Placement::PlaceManquante(nouveau)
        );
        assert_eq!(
            collection.envoyer_au_camp(0),
            Err(ErreurCollection::CampPlein)
        );
    }
}
