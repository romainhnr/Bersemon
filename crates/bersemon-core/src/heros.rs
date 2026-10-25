//! Héros du joueur (section 4) : un nom, une classe et son équipement. Il
//! donne un ordre par combat et, depuis l'étape 130, combat lui-même quand
//! tous ses monstres sont KO (section 4.4) : il n'a ni XP ni niveau propre,
//! mais des stats tirées de sa classe et un niveau de combat tiré de son
//! équipement.

use crate::attaque::{Attaque, AttaqueId, AttaquesMonstre, Categorie};
use crate::equipement::{Equipement, ObjetDEquipement, PlaceDEquipement};
use crate::espece::{Espece, EspeceId, Role};
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::nom::Nom;
use crate::stats::{BonusDeStats, Stats, StatsDeBase, ValeursInternes, cumuler_bonus};
use crate::statut::Statut;
use crate::types::{Type, TypesMonstre};

/// Guerrier et Mage dans la V1 (Q2), puis le Templier et l'Assassin
/// (section 4.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Classe {
    Guerrier,
    Mage,
    Templier,
    Assassin,
}

impl Classe {
    pub const TOUTES: [Classe; 4] = [
        Classe::Guerrier,
        Classe::Mage,
        Classe::Templier,
        Classe::Assassin,
    ];

    pub fn nom(self) -> &'static str {
        match self {
            Classe::Guerrier => "Guerrier",
            Classe::Mage => "Mage",
            Classe::Templier => "Templier",
            Classe::Assassin => "Assassin",
        }
    }

    /// +10 % dans une stat des monstres du joueur en combat (Q34), dans les
    /// deux défenses pour le Templier (section 4.2).
    pub fn bonus(self) -> BonusDeStats {
        match self {
            Classe::Guerrier => Stats {
                attaque: 10,
                ..Stats::default()
            },
            Classe::Mage => Stats {
                attaque_speciale: 10,
                ..Stats::default()
            },
            Classe::Templier => Stats {
                defense: 10,
                defense_speciale: 10,
                ..Stats::default()
            },
            Classe::Assassin => Stats {
                vitesse: 10,
                ..Stats::default()
            },
        }
    }

    pub fn ordre(self) -> Ordre {
        match self {
            Classe::Guerrier => Ordre::CriDeGuerre,
            Classe::Mage => Ordre::Incantation,
            Classe::Templier => Ordre::BouclierDeFoi,
            Classe::Assassin => Ordre::PasDeLOmbre,
        }
    }

    /// Celles du héros qui combat, comme celles d'une espèce
    /// (docs/contenu-v1.md, section 34).
    pub fn stats_de_base(self) -> StatsDeBase {
        let [
            pv,
            attaque,
            defense,
            attaque_speciale,
            defense_speciale,
            vitesse,
        ] = match self {
            Classe::Guerrier => [90, 100, 85, 40, 65, 70],
            Classe::Mage => [70, 40, 55, 110, 90, 85],
            Classe::Templier => [95, 45, 100, 75, 95, 40],
            Classe::Assassin => [70, 105, 55, 40, 60, 120],
        };
        Stats {
            pv,
            attaque,
            defense,
            attaque_speciale,
            defense_speciale,
            vitesse,
        }
    }
}

/// Niveau de combat d'un héros sans arme ni armure ; chaque palier de son
/// arme et de son armure l'élève d'autant (section 4.4).
const NIVEAU_DE_COMBAT_DE_BASE: u8 = 5;
const NIVEAUX_PAR_PALIER: u8 = 5;

/// Celles de tout héros qui combat, faute de les tirer au sort (section
/// 4.4).
const VALEURS_INTERNES_DU_HEROS: u8 = 15;

/// L'attaque du héros sans arme (docs/contenu-v1.md, section 34).
pub fn coup_de_poing() -> Attaque {
    Attaque::new(
        AttaqueId::new("coup-de-poing"),
        "Coup de poing",
        Type::Humain,
        Categorie::Physique,
        40,
        100,
    )
    .expect("attaque du héros valide")
}

/// 5 sans rien, plus 5 par palier de l'arme et de l'armure.
pub fn niveau_de_combat(
    arme: Option<&ObjetDEquipement>,
    armure: Option<&ObjetDEquipement>,
) -> Niveau {
    let paliers: u8 = [arme, armure]
        .into_iter()
        .flatten()
        .map(ObjetDEquipement::palier)
        .sum();
    Niveau::new(NIVEAU_DE_COMBAT_DE_BASE + NIVEAUX_PAR_PALIER * paliers)
        .expect("niveau de combat dans les bornes")
}

/// Un héros qui combat, celui du joueur comme celui d'un humain de l'IA :
/// un monstre de type Humain, au nom du héros, aux stats de sa classe et
/// au niveau de son équipement, avec les attaques de son arme et tous ses
/// PV (section 4.4). Il n'a ni habileté, ni XP à gagner.
pub fn creer_combattant(
    nom: &str,
    classe: Classe,
    arme: Option<&ObjetDEquipement>,
    armure: Option<&ObjetDEquipement>,
) -> Monstre {
    let espece = Espece::new(
        EspeceId::new("heros"),
        nom,
        TypesMonstre::new(&[Type::Humain]).expect("un type"),
        Role::Offensif,
        classe.stats_de_base(),
        0,
        Vec::new(),
    );
    let attaques = match arme.map(ObjetDEquipement::attaques) {
        Some(attaques) if !attaques.is_empty() => attaques.to_vec(),
        _ => vec![coup_de_poing()],
    };
    let valeur = VALEURS_INTERNES_DU_HEROS;
    let valeurs_internes = ValeursInternes::new(Stats {
        pv: valeur,
        attaque: valeur,
        defense: valeur,
        attaque_speciale: valeur,
        defense_speciale: valeur,
        vitesse: valeur,
    })
    .expect("valeurs internes valides");
    Monstre::new(
        espece,
        niveau_de_combat(arme, armure),
        valeurs_internes,
        AttaquesMonstre::new(attaques).expect("attaques du héros valides"),
        false,
    )
}

/// Action de soutien que le héros donne en combat, une fois par combat, au
/// monstre actif (section 4.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordre {
    CriDeGuerre,
    Incantation,
    BouclierDeFoi,
    PasDeLOmbre,
}

/// Les ordres renforcent la stat du bonus de classe.
const POURCENTAGE_D_ORDRE: u8 = 20;

impl Ordre {
    pub fn nom(self) -> &'static str {
        match self {
            Ordre::CriDeGuerre => "Cri de guerre",
            Ordre::Incantation => "Incantation",
            Ordre::BouclierDeFoi => "Bouclier de foi",
            Ordre::PasDeLOmbre => "Pas de l'ombre",
        }
    }

    /// Bonus du monstre qui reçoit l'ordre, jusqu'à la fin du combat.
    pub fn bonus(self) -> BonusDeStats {
        let pourcentage = POURCENTAGE_D_ORDRE;
        match self {
            Ordre::CriDeGuerre => Stats {
                attaque: pourcentage,
                ..Stats::default()
            },
            Ordre::Incantation => Stats {
                attaque_speciale: pourcentage,
                ..Stats::default()
            },
            Ordre::BouclierDeFoi => Stats {
                defense: pourcentage,
                defense_speciale: pourcentage,
                ..Stats::default()
            },
            Ordre::PasDeLOmbre => Stats {
                vitesse: pourcentage,
                ..Stats::default()
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heros {
    nom: Nom,
    classe: Classe,
    equipement: Equipement,
    /// Les PV qu'il a perdus en combat : ils durent toute l'expédition,
    /// comme ceux d'un monstre, et se soignent au camp (section 4.4).
    pv_perdus: u32,
    statut: Option<Statut>,
}

impl Heros {
    pub fn new(nom: Nom, classe: Classe) -> Self {
        Self {
            nom,
            classe,
            equipement: Equipement::default(),
            pv_perdus: 0,
            statut: None,
        }
    }

    /// Reprise d'une sauvegarde : ses PV perdus et son statut. Il n'a jamais
    /// moins de 1 PV : KO, il perd le combat et rentre au camp.
    pub fn avec_blessures(mut self, pv_perdus: u32, statut: Option<Statut>) -> Self {
        self.pv_perdus = pv_perdus;
        self.statut = statut;
        self
    }

    pub fn pv_perdus(&self) -> u32 {
        self.pv_perdus
    }

    pub fn statut(&self) -> Option<Statut> {
        self.statut
    }

    pub fn niveau_de_combat(&self) -> Niveau {
        niveau_de_combat(
            self.equipement.porte(PlaceDEquipement::Arme),
            self.equipement.porte(PlaceDEquipement::Armure),
        )
    }

    /// Le héros tel qu'il entre en combat : ses PV et son statut du moment.
    pub fn combattant(&self) -> Monstre {
        let mut combattant = creer_combattant(
            self.nom.as_str(),
            self.classe,
            self.equipement.porte(PlaceDEquipement::Arme),
            self.equipement.porte(PlaceDEquipement::Armure),
        );
        let pv_max = combattant.pv_actuels();
        combattant.subir_degats(self.pv_perdus.min(pv_max.saturating_sub(1)));
        if let Some(statut) = self.statut {
            combattant.infliger_statut(statut);
        }
        combattant
    }

    /// Après un combat, ou un soin entre deux étapes, il garde les PV et le
    /// statut de son combattant.
    pub(crate) fn garder_l_etat_de(&mut self, combattant: &Monstre) {
        self.pv_perdus = combattant.stats().pv - combattant.pv_actuels();
        self.statut = combattant.statut();
    }

    /// Au camp, comme ses monstres (section 13.1).
    pub(crate) fn soigner(&mut self) {
        self.pv_perdus = 0;
        self.statut = None;
    }

    /// Reprise d'une sauvegarde (section 13.9).
    pub fn avec_equipement(mut self, equipement: Equipement) -> Self {
        self.equipement = equipement;
        self
    }

    pub fn nom(&self) -> &Nom {
        &self.nom
    }

    pub fn classe(&self) -> Classe {
        self.classe
    }

    pub fn equipement(&self) -> &Equipement {
        &self.equipement
    }

    pub(crate) fn equipement_mut(&mut self) -> &mut Equipement {
        &mut self.equipement
    }

    /// Bonus des monstres du joueur en combat : celui de la classe, plus
    /// celui des objets portés (sections 4.2 et 4.3).
    pub fn bonus_en_combat(&self) -> BonusDeStats {
        cumuler_bonus(&self.classe.bonus(), &self.equipement.bonus())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equipement::{ObjetDEquipementId, SorteDObjet};

    fn objet(sorte: SorteDObjet, palier: u8, attaques: Vec<Attaque>) -> ObjetDEquipement {
        ObjetDEquipement::new(
            ObjetDEquipementId::new(format!("{sorte:?}-{palier}")),
            "objet",
            sorte,
            BonusDeStats::default(),
        )
        .de_classe(Classe::Guerrier, palier)
        .avec_attaques(attaques)
    }

    #[test]
    fn le_niveau_de_combat_vient_de_l_arme_et_de_l_armure() {
        let arme = objet(SorteDObjet::Arme, 2, Vec::new());
        let armure = objet(SorteDObjet::Armure, 3, Vec::new());
        assert_eq!(niveau_de_combat(None, None).valeur(), 5);
        assert_eq!(niveau_de_combat(Some(&arme), None).valeur(), 15);
        assert_eq!(niveau_de_combat(Some(&arme), Some(&armure)).valeur(), 30);
    }

    #[test]
    fn sans_arme_le_heros_n_a_que_coup_de_poing() {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier);
        let combattant = heros.combattant();
        assert_eq!(combattant.nom(), "Aldric");
        assert_eq!(combattant.espece().types().liste(), [Type::Humain]);
        assert_eq!(combattant.niveau().valeur(), 5);
        assert_eq!(combattant.attaques().liste(), [coup_de_poing()]);
        assert_eq!(combattant.habilete(), None);
        // Base 90 au niveau 5, valeurs internes à 15 : (180 + 15) x 5 / 100
        // + 5 + 10.
        assert_eq!(combattant.stats().pv, 24);
    }

    #[test]
    fn les_blessures_du_heros_durent_jusqu_au_camp() {
        let mut heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
        let mut combattant = heros.combattant();
        combattant.subir_degats(10);
        combattant.infliger_statut(Statut::Poison);
        heros.garder_l_etat_de(&combattant);
        assert_eq!(heros.pv_perdus(), 10);
        let repris = heros.combattant();
        assert_eq!(repris.pv_actuels(), combattant.pv_actuels());
        assert_eq!(repris.statut(), Some(Statut::Poison));
        heros.soigner();
        assert_eq!(
            heros.combattant().pv_actuels(),
            heros.combattant().stats().pv
        );
        assert_eq!(heros.statut(), None);
    }

    #[test]
    fn chaque_classe_renforce_sa_stat() {
        assert_eq!(Classe::Guerrier.bonus().attaque, 10);
        assert_eq!(Classe::Guerrier.bonus().attaque_speciale, 0);
        assert_eq!(Classe::Mage.bonus().attaque_speciale, 10);
        assert_eq!(Classe::Mage.bonus().attaque, 0);
        let templier = Classe::Templier.bonus();
        assert_eq!((templier.defense, templier.defense_speciale), (10, 10));
        assert_eq!((templier.attaque, templier.attaque_speciale), (0, 0));
        let assassin = Classe::Assassin.bonus();
        assert_eq!(assassin.vitesse, 10);
        assert_eq!((assassin.attaque, assassin.defense), (0, 0));
    }

    #[test]
    fn l_ordre_renforce_la_stat_de_classe_de_20_pour_cent() {
        for classe in Classe::TOUTES {
            let classe_bonus = classe.bonus();
            let ordre = classe.ordre().bonus();
            let doubler = |valeur: u8| valeur * 2;
            assert_eq!(
                ordre,
                Stats {
                    pv: doubler(classe_bonus.pv),
                    attaque: doubler(classe_bonus.attaque),
                    defense: doubler(classe_bonus.defense),
                    attaque_speciale: doubler(classe_bonus.attaque_speciale),
                    defense_speciale: doubler(classe_bonus.defense_speciale),
                    vitesse: doubler(classe_bonus.vitesse),
                },
                "{}",
                classe.nom()
            );
        }
    }
}
