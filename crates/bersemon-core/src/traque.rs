//! Traque d'un Apôtre (section 9.4) : une piste d'indices, trouvés de zone en
//! zone au bout des expéditions, mène à l'antre de l'Apôtre.

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::espece::{Espece, EspeceId};
use crate::monstre::{Monstre, tirer_shiny};
use crate::niveau::Niveau;
use crate::stats::ValeursInternes;
use crate::zone::{Zone, ZoneId};

/// Un Apôtre combat 2 niveaux au-dessus du niveau maximal de sa zone, sauf
/// le premier (section 9.4).
const NIVEAUX_AU_DESSUS_DE_LA_ZONE: u8 = 2;

/// Indice trouvé au bout d'une expédition menée jusqu'à sa dernière étape
/// dans sa zone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Indice {
    pub zone: ZoneId,
    pub texte: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurTraque {
    AucunIndice,
    /// Le niveau de l'Apôtre dépasserait le niveau maximal.
    NiveauTropHaut,
    /// L'espèce n'apprend aucune attaque, ou en apprend une en double,
    /// jusqu'au niveau de l'Apôtre.
    EspeceInvalide(EspeceId),
}

impl fmt::Display for ErreurTraque {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurTraque::AucunIndice => write!(f, "une piste compte au moins un indice"),
            ErreurTraque::NiveauTropHaut => {
                write!(f, "le niveau de l'Apôtre dépasserait le niveau maximal")
            }
            ErreurTraque::EspeceInvalide(espece) => write!(
                f,
                "l'espèce {} ne peut pas être créée au niveau de l'Apôtre",
                espece.as_str()
            ),
        }
    }
}

impl std::error::Error for ErreurTraque {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Apotre {
    espece: Espece,
    zone: ZoneId,
    niveau: Niveau,
    avant_combat: String,
    epilogue: String,
    /// Le récit de son âme, que le joueur recueille après la victoire
    /// (section 9.4).
    recit_de_l_ame: String,
    /// Ce que le héros écrit dans son carnet de traque, au codex, une fois
    /// l'Apôtre abattu (section 5.6).
    ligne_du_carnet: String,
    /// Le Démiurge se relève sous cette forme quand la première tombe KO
    /// (section 9.4). En boîte, pour que l'Apôtre reste léger.
    seconde_forme: Option<Box<Espece>>,
}

impl Apotre {
    /// Vérifie que l'espèce peut être créée au niveau de l'Apôtre : la
    /// génération du combat ne peut ensuite plus échouer.
    pub fn new(
        espece: Espece,
        zone: &Zone,
        avant_combat: impl Into<String>,
        epilogue: impl Into<String>,
    ) -> Result<Self, ErreurTraque> {
        let (_, niveau_max) = zone.niveaux();
        let niveau = niveau_max
            .valeur()
            .checked_add(NIVEAUX_AU_DESSUS_DE_LA_ZONE)
            .and_then(|valeur| Niveau::new(valeur).ok())
            .ok_or(ErreurTraque::NiveauTropHaut)?;
        Monstre::creer(espece.clone(), niveau, ValeursInternes::default(), false)
            .map_err(|_| ErreurTraque::EspeceInvalide(espece.id().clone()))?;
        Ok(Self {
            espece,
            zone: zone.id().clone(),
            niveau,
            avant_combat: avant_combat.into(),
            epilogue: epilogue.into(),
            recit_de_l_ame: String::new(),
            ligne_du_carnet: String::new(),
            seconde_forme: None,
        })
    }

    /// Vérifie, comme `new`, que la seconde forme peut être créée au niveau
    /// de l'Apôtre.
    pub fn avec_seconde_forme(mut self, espece: Espece) -> Result<Self, ErreurTraque> {
        Monstre::creer(
            espece.clone(),
            self.niveau,
            ValeursInternes::default(),
            false,
        )
        .map_err(|_| ErreurTraque::EspeceInvalide(espece.id().clone()))?;
        self.seconde_forme = Some(Box::new(espece));
        Ok(self)
    }

    /// Le premier Apôtre combat au niveau maximal de sa zone, sans les 2
    /// niveaux de plus des suivants, pour que la difficulté monte d'un
    /// Apôtre à l'autre (section 9.4). Vérifie, comme `new`, que ses formes
    /// peuvent être créées à ce niveau.
    pub fn au_niveau_maximal_de_sa_zone(mut self, zone: &Zone) -> Result<Self, ErreurTraque> {
        debug_assert_eq!(zone.id(), &self.zone, "la zone de l'Apôtre");
        let (_, niveau) = zone.niveaux();
        for espece in std::iter::once(&self.espece).chain(self.seconde_forme.as_deref()) {
            Monstre::creer(espece.clone(), niveau, ValeursInternes::default(), false)
                .map_err(|_| ErreurTraque::EspeceInvalide(espece.id().clone()))?;
        }
        self.niveau = niveau;
        Ok(self)
    }

    pub fn seconde_forme(&self) -> Option<&Espece> {
        self.seconde_forme.as_deref()
    }

    pub fn avec_recit_de_l_ame(mut self, recit: impl Into<String>) -> Self {
        self.recit_de_l_ame = recit.into();
        self
    }

    pub fn recit_de_l_ame(&self) -> &str {
        &self.recit_de_l_ame
    }

    pub fn avec_ligne_du_carnet(mut self, ligne: impl Into<String>) -> Self {
        self.ligne_du_carnet = ligne.into();
        self
    }

    pub fn ligne_du_carnet(&self) -> &str {
        &self.ligne_du_carnet
    }

    pub fn espece(&self) -> &Espece {
        &self.espece
    }

    pub fn zone(&self) -> &ZoneId {
        &self.zone
    }

    pub fn niveau(&self) -> Niveau {
        self.niveau
    }

    pub fn avant_combat(&self) -> &str {
        &self.avant_combat
    }

    pub fn epilogue(&self) -> &str {
        &self.epilogue
    }

    /// Les valeurs internes de l'Apôtre sont fixes, au maximum ; comme un
    /// monstre sauvage, il a 1 chance sur 512 d'être shiny (Q58) et tire son
    /// habileté (section 5.9). Tirages : le shiny, puis l'habileté.
    pub fn generer(&self, aleatoire: &mut dyn Aleatoire) -> Monstre {
        self.generer_forme(&self.espece, aleatoire)
    }

    /// Ses formes, dans l'ordre du combat : la seconde, s'il en a une, suit
    /// la première, et se tire de même après elle (section 9.4).
    pub fn generer_ses_formes(&self, aleatoire: &mut dyn Aleatoire) -> Vec<Monstre> {
        let mut formes = vec![self.generer(aleatoire)];
        if let Some(seconde) = &self.seconde_forme {
            formes.push(self.generer_forme(seconde, aleatoire));
        }
        formes
    }

    fn generer_forme(&self, espece: &Espece, aleatoire: &mut dyn Aleatoire) -> Monstre {
        let shiny = tirer_shiny(aleatoire);
        Monstre::creer(
            espece.clone(),
            self.niveau,
            ValeursInternes::maximales(),
            shiny,
        )
        .expect("espèce vérifiée au niveau de l'Apôtre")
        .tirer_son_habilete(aleatoire)
    }
}

/// Un chapitre de la traque : les indices qui mènent à un Apôtre, dans
/// l'ordre où on les trouve, puis son antre (section 9.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chapitre {
    indices: Vec<Indice>,
    apotre: Apotre,
}

impl Chapitre {
    pub fn new(indices: Vec<Indice>, apotre: Apotre) -> Result<Self, ErreurTraque> {
        if indices.is_empty() {
            return Err(ErreurTraque::AucunIndice);
        }
        Ok(Self { indices, apotre })
    }

    pub fn indices(&self) -> &[Indice] {
        &self.indices
    }

    pub fn apotre(&self) -> &Apotre {
        &self.apotre
    }
}

/// La traque des Apôtres, chapitre après chapitre. Les indices se comptent
/// à la suite d'un chapitre à l'autre ; ceux d'un chapitre ne se trouvent
/// qu'une fois vaincu l'Apôtre du chapitre précédent (section 9.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piste {
    chapitres: Vec<Chapitre>,
}

impl Piste {
    pub fn new(chapitres: Vec<Chapitre>) -> Result<Self, ErreurTraque> {
        if chapitres.is_empty() {
            return Err(ErreurTraque::AucunIndice);
        }
        Ok(Self { chapitres })
    }

    pub fn chapitres(&self) -> &[Chapitre] {
        &self.chapitres
    }

    /// Tous les indices, numérotés à la suite.
    pub fn indices(&self) -> impl Iterator<Item = &Indice> {
        self.chapitres
            .iter()
            .flat_map(|chapitre| chapitre.indices.iter())
    }

    /// Les Apôtres, dans l'ordre de la traque.
    pub fn apotres(&self) -> impl Iterator<Item = &Apotre> {
        self.chapitres.iter().map(|chapitre| &chapitre.apotre)
    }

    /// Chaque chapitre, avec le nombre d'indices à trouver pour localiser
    /// son antre.
    fn chapitres_et_fins(&self) -> impl Iterator<Item = (&Chapitre, usize)> {
        self.chapitres.iter().scan(0, |trouves, chapitre| {
            *trouves += chapitre.indices.len();
            Some((chapitre, *trouves))
        })
    }

    /// La zone de chaque indice déjà trouvé et celle de l'indice suivant sont
    /// ouvertes ; celle d'un Apôtre l'est une fois tous les indices de son
    /// chapitre trouvés (sections 9.1 et 9.4).
    pub fn zone_ouverte(&self, zone: &ZoneId, indices_trouves: usize) -> bool {
        let par_indice = self
            .indices()
            .take(indices_trouves.saturating_add(1))
            .any(|indice| &indice.zone == zone);
        par_indice
            || self
                .chapitres_et_fins()
                .any(|(chapitre, fin)| indices_trouves >= fin && &chapitre.apotre.zone == zone)
    }

    /// `None` une fois tous les indices trouvés, ou tant que l'Apôtre du
    /// chapitre précédent n'est pas vaincu.
    pub fn indice_suivant(
        &self,
        indices_trouves: usize,
        apotres_vaincus: &[EspeceId],
    ) -> Option<&Indice> {
        let mut debut = 0;
        let mut precedent: Option<&Apotre> = None;
        for (chapitre, fin) in self.chapitres_et_fins() {
            if indices_trouves < fin {
                if precedent.is_some_and(|apotre| !apotres_vaincus.contains(apotre.espece().id())) {
                    return None;
                }
                return chapitre.indices.get(indices_trouves - debut);
            }
            debut = fin;
            precedent = Some(&chapitre.apotre);
        }
        None
    }

    /// L'Apôtre dont l'antre, localisé, est dans cette zone, tant qu'il n'est
    /// pas vaincu : il attend au bout de l'expédition (section 9.4).
    pub fn apotre_en_attente(
        &self,
        zone: &ZoneId,
        indices_trouves: usize,
        apotres_vaincus: &[EspeceId],
    ) -> Option<&Apotre> {
        self.chapitres_et_fins()
            .filter(|(_, fin)| indices_trouves >= *fin)
            .map(|(chapitre, _)| &chapitre.apotre)
            .find(|apotre| &apotre.zone == zone && !apotres_vaincus.contains(apotre.espece().id()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::donnees_de_test::{espece, parametres_coffre, parametres_expedition_automatique};

    fn zone(id: &str, niveau_max: u8) -> Zone {
        Zone::new(
            ZoneId::new(id),
            id,
            (Niveau::new(1).unwrap(), Niveau::new(niveau_max).unwrap()),
            4,
            1,
            vec![espece("loup")],
            vec![],
            parametres_expedition_automatique(),
            parametres_coffre(),
        )
        .unwrap()
    }

    fn indice(zone: &str) -> Indice {
        Indice {
            zone: ZoneId::new(zone),
            texte: format!("indice de {zone}"),
        }
    }

    fn piste() -> Piste {
        let apotre = Apotre::new(espece("baron"), &zone("marais", 13), "avant", "après").unwrap();
        Piste::new(vec![
            Chapitre::new(
                vec![indice("lisiere"), indice("marais"), indice("marais")],
                apotre,
            )
            .unwrap(),
        ])
        .unwrap()
    }

    /// Deux Apôtres : le second, dans les cimes, ne se traque qu'une fois le
    /// premier vaincu.
    fn deux_chapitres() -> Piste {
        let baron = Apotre::new(espece("baron"), &zone("marais", 13), "", "").unwrap();
        let abbesse = Apotre::new(espece("abbesse"), &zone("cimes", 27), "", "").unwrap();
        Piste::new(vec![
            Chapitre::new(vec![indice("lisiere"), indice("marais")], baron).unwrap(),
            Chapitre::new(vec![indice("marais"), indice("cimes")], abbesse).unwrap(),
        ])
        .unwrap()
    }

    #[test]
    fn l_apotre_combat_deux_niveaux_au_dessus_de_sa_zone_aux_valeurs_internes_maximales() {
        let apotre = piste().apotres().next().unwrap().clone();
        assert_eq!(apotre.niveau(), Niveau::new(15).unwrap());
        // Seul le shiny est tiré.
        let monstre = apotre.generer(&mut AleatoireScripte::new([1]));
        assert_eq!(monstre.niveau(), Niveau::new(15).unwrap());
        assert_eq!(monstre.valeurs_internes(), &ValeursInternes::maximales());
        assert!(monstre.est_shiny());
        assert_eq!(monstre.espece().id().as_str(), "baron");
    }

    #[test]
    fn le_premier_apotre_peut_combattre_au_niveau_maximal_de_sa_zone() {
        let marais = zone("marais", 13);
        let apotre = Apotre::new(espece("baron"), &marais, "", "")
            .and_then(|apotre| apotre.au_niveau_maximal_de_sa_zone(&marais))
            .unwrap();
        assert_eq!(apotre.niveau(), Niveau::new(13).unwrap());
        let monstre = apotre.generer(&mut AleatoireScripte::new([2]));
        assert_eq!(monstre.niveau(), Niveau::new(13).unwrap());
        assert_eq!(monstre.valeurs_internes(), &ValeursInternes::maximales());
    }

    #[test]
    fn le_chapitre_suivant_attend_la_defaite_de_l_apotre() {
        let piste = deux_chapitres();
        let baron = [EspeceId::new("baron")];
        let cimes = ZoneId::new("cimes");
        let marais = ZoneId::new("marais");
        // Le Baron attend dans le Marais, ses deux indices trouvés.
        assert_eq!(
            piste
                .apotre_en_attente(&marais, 2, &[])
                .map(|apotre| apotre.espece().id().as_str()),
            Some("baron")
        );
        assert_eq!(piste.indice_suivant(2, &[]), None);
        // Vaincu, il laisse le troisième indice, qui ouvre les cimes.
        assert_eq!(piste.apotre_en_attente(&marais, 2, &baron), None);
        assert_eq!(piste.indice_suivant(2, &baron), Some(&indice("marais")));
        assert!(!piste.zone_ouverte(&cimes, 2));
        assert!(piste.zone_ouverte(&cimes, 3));
        assert_eq!(piste.apotre_en_attente(&cimes, 3, &baron), None);
        assert_eq!(
            piste
                .apotre_en_attente(&cimes, 4, &baron)
                .map(|apotre| apotre.espece().id().as_str()),
            Some("abbesse")
        );
        assert_eq!(piste.indice_suivant(4, &baron), None);
        assert_eq!(piste.indices().count(), 4);
        assert_eq!(piste.apotres().count(), 2);
    }

    #[test]
    fn apotre_au_dela_du_niveau_maximal_refuse() {
        assert_eq!(
            Apotre::new(espece("baron"), &zone("cimes", 99), "", ""),
            Err(ErreurTraque::NiveauTropHaut)
        );
    }

    #[test]
    fn piste_sans_indice_refusee() {
        let apotre = piste().apotres().next().unwrap().clone();
        assert_eq!(
            Chapitre::new(vec![], apotre),
            Err(ErreurTraque::AucunIndice)
        );
        assert_eq!(Piste::new(vec![]), Err(ErreurTraque::AucunIndice));
    }

    #[test]
    fn les_zones_s_ouvrent_avec_les_indices() {
        let piste = piste();
        let lisiere = ZoneId::new("lisiere");
        let marais = ZoneId::new("marais");
        assert!(piste.zone_ouverte(&lisiere, 0));
        assert!(!piste.zone_ouverte(&marais, 0));
        assert!(piste.zone_ouverte(&marais, 1));
        assert!(piste.zone_ouverte(&lisiere, 3));
        assert!(!piste.zone_ouverte(&ZoneId::new("cimes"), 3));
    }

    #[test]
    fn l_antre_est_localise_au_dernier_indice() {
        let piste = piste();
        let marais = ZoneId::new("marais");
        assert_eq!(piste.indice_suivant(0, &[]), Some(&indice("lisiere")));
        assert_eq!(piste.indice_suivant(2, &[]), Some(&indice("marais")));
        assert_eq!(piste.apotre_en_attente(&marais, 2, &[]), None);
        assert_eq!(piste.indice_suivant(3, &[]), None);
        assert!(piste.apotre_en_attente(&marais, 3, &[]).is_some());
    }
}
