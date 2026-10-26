//! Soins du joueur (section 14.4) : la potion de la V1, puis le remède, la
//! grande potion et l'élixir, vendus par Malvoise une fois l'âme de leur
//! marchand recueillie (section 14.2), le grand élixir, ajouté à l'étape
//! 134, avec l'élixir, et la panacée, ajoutée à l'étape 142 avec Hilde, la
//! marchande morte du Désert de sel. Ils s'utilisent en combat, comme
//! action du tour, ou entre deux étapes d'une expédition.

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::monstre::Monstre;
use crate::statut::Statut;

/// Fourchettes du pourcentage de PV max rendu.
const POTION: (u32, u32) = (40, 60);
const GRANDE_POTION: (u32, u32) = (80, 100);
/// Un élixir relève un monstre KO avec la moitié de ses PV max.
const POURCENTAGE_DE_L_ELIXIR: u32 = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Soin {
    Potion,
    /// Guérit le poison ou la paralysie.
    Remede,
    GrandePotion,
    /// Relève un monstre KO.
    Elixir,
    /// Relève un monstre KO avec tous ses PV, comme le Rappel max de Pokémon
    /// (section 14.4).
    GrandElixir,
    /// Rend tous les PV et guérit le poison ou la paralysie, comme la
    /// Guérison de Pokémon, sans relever un monstre KO (section 14.4).
    Panacee,
}

impl Soin {
    pub const TOUS: [Soin; 6] = [
        Soin::Potion,
        Soin::Remede,
        Soin::GrandePotion,
        Soin::Elixir,
        Soin::GrandElixir,
        Soin::Panacee,
    ];

    fn position(self) -> usize {
        match self {
            Soin::Potion => 0,
            Soin::Remede => 1,
            Soin::GrandePotion => 2,
            Soin::Elixir => 3,
            Soin::GrandElixir => 4,
            Soin::Panacee => 5,
        }
    }

    /// L'élixir et le grand élixir relèvent un monstre KO, et rien d'autre.
    pub fn releve(self) -> bool {
        matches!(self, Soin::Elixir | Soin::GrandElixir)
    }

    /// Le soin du marchand dont l'âme le met en vente chez Malvoise : celle
    /// d'Ulrich ajoute le grand élixir à l'élixir (section 14.2).
    pub fn vendu_avec(self) -> Soin {
        match self {
            Soin::GrandElixir => Soin::Elixir,
            autre => autre,
        }
    }
}

/// Le nombre de chaque soin, dans le sac, le coffre du camp ou un combat.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Soins([u32; 6]);

impl Soins {
    pub fn nombre(&self, soin: Soin) -> u32 {
        self.0[soin.position()]
    }

    pub fn ajouter(&mut self, soin: Soin, nombre: u32) {
        let actuel = &mut self.0[soin.position()];
        *actuel = actuel.saturating_add(nombre);
    }

    /// Rend `false`, sans rien changer, s'il n'y en a pas assez.
    pub(crate) fn retirer(&mut self, soin: Soin, nombre: u32) -> bool {
        let actuel = &mut self.0[soin.position()];
        if *actuel < nombre {
            return false;
        }
        *actuel -= nombre;
        true
    }

    pub fn est_vide(&self) -> bool {
        self.0.iter().all(|nombre| *nombre == 0)
    }

    /// Ceux qu'il y a, dans l'ordre de `Soin::TOUS`, avec leur nombre.
    pub fn presents(&self) -> impl Iterator<Item = (Soin, u32)> + '_ {
        Soin::TOUS
            .into_iter()
            .map(|soin| (soin, self.nombre(soin)))
            .filter(|(_, nombre)| *nombre > 0)
    }
}

/// Ce qu'a fait un soin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffetDuSoin {
    /// Potion, grande potion, ou élixir qui relève le monstre.
    PvRendus(u32),
    /// Le remède a guéri ce statut, ou rien.
    StatutGueri(Option<Statut>),
    /// La panacée a rendu ces PV et guéri ce statut.
    ToutGueri { pv: u32, statut: Option<Statut> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurSoin {
    Aucun(Soin),
    MonstreInexistant,
    /// Seuls l'élixir et le grand élixir relèvent un monstre KO.
    MonstreKo,
}

impl fmt::Display for ErreurSoin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurSoin::Aucun(soin) => write!(f, "aucun soin de ce genre : {soin:?}"),
            ErreurSoin::MonstreInexistant => write!(f, "ce monstre n'est pas dans l'équipe"),
            ErreurSoin::MonstreKo => write!(f, "seul un élixir relève un monstre KO"),
        }
    }
}

impl std::error::Error for ErreurSoin {}

/// Seul l'élixir s'utilise sur un monstre KO (section 14.4). Un soin sans
/// effet est permis mais perdu : demander confirmation au joueur revient à
/// l'interface (`soin_sans_effet`).
pub fn verifier_cible(soin: Soin, monstre: &Monstre) -> Result<(), ErreurSoin> {
    if monstre.est_ko() && !soin.releve() {
        Err(ErreurSoin::MonstreKo)
    } else {
        Ok(())
    }
}

/// Le soin serait consommé pour rien : PV déjà pleins, aucun statut à
/// guérir, ou un élixir sur un monstre qui n'est pas KO (Q53).
pub fn soin_sans_effet(soin: Soin, monstre: &Monstre) -> bool {
    match soin {
        Soin::Potion | Soin::GrandePotion => monstre.pv_actuels() >= monstre.stats().pv,
        Soin::Remede => monstre.statut().is_none(),
        Soin::Elixir | Soin::GrandElixir => !monstre.est_ko(),
        Soin::Panacee => monstre.pv_actuels() >= monstre.stats().pv && monstre.statut().is_none(),
    }
}

/// Applique le soin à une cible déjà vérifiée par `verifier_cible`. Un
/// tirage pour une potion ou une grande potion qui a un effet, aucun
/// sinon.
pub(crate) fn appliquer_soin(
    soin: Soin,
    monstre: &mut Monstre,
    aleatoire: &mut dyn Aleatoire,
) -> EffetDuSoin {
    if soin_sans_effet(soin, monstre) {
        return match soin {
            Soin::Remede => EffetDuSoin::StatutGueri(None),
            Soin::Potion | Soin::GrandePotion | Soin::Elixir | Soin::GrandElixir => {
                EffetDuSoin::PvRendus(0)
            }
            Soin::Panacee => EffetDuSoin::ToutGueri {
                pv: 0,
                statut: None,
            },
        };
    }
    match soin {
        Soin::Potion | Soin::GrandePotion => {
            let (minimum, maximum) = if soin == Soin::Potion {
                POTION
            } else {
                GRANDE_POTION
            };
            let pourcentage = aleatoire.tirer(minimum..=maximum);
            let pv = monstre.stats().pv * pourcentage / 100;
            EffetDuSoin::PvRendus(monstre.soigner(pv))
        }
        Soin::Remede => EffetDuSoin::StatutGueri(monstre.guerir_statut()),
        Soin::Elixir => {
            let pv = (monstre.stats().pv * POURCENTAGE_DE_L_ELIXIR / 100).max(1);
            EffetDuSoin::PvRendus(monstre.relever(pv))
        }
        Soin::GrandElixir => EffetDuSoin::PvRendus(monstre.relever(monstre.stats().pv)),
        Soin::Panacee => {
            let pv = monstre.soigner(monstre.stats().pv);
            EffetDuSoin::ToutGueri {
                pv,
                statut: monstre.guerir_statut(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::attaque::Categorie;
    use crate::donnees_de_test::{attaque, monstre, stats_uniformes};
    use crate::types::Type;

    /// 20 PV max au niveau 5.
    fn blesse(pv_restants: u32) -> Monstre {
        let coup = attaque("coup", Type::Normal, Categorie::Physique, 40, 100);
        let mut monstre = monstre(&[Type::Feu], stats_uniformes(50), 5, vec![coup]);
        monstre.subir_degats(20 - pv_restants);
        monstre
    }

    fn appliquer(soin: Soin, monstre: &mut Monstre, tirages: &[u32]) -> EffetDuSoin {
        appliquer_soin(
            soin,
            monstre,
            &mut AleatoireScripte::new(tirages.iter().copied()),
        )
    }

    #[test]
    fn une_grande_potion_rend_de_80_a_100_pour_cent() {
        let mut monstre = blesse(1);
        assert_eq!(
            appliquer(Soin::GrandePotion, &mut monstre, &[80]),
            EffetDuSoin::PvRendus(16)
        );
        assert_eq!(monstre.pv_actuels(), 17);
    }

    /// Section 14.4, décidée le 25/10/2026 pour l'étape 134.
    #[test]
    fn un_grand_elixir_releve_avec_tous_les_pv() {
        let mut monstre = blesse(1);
        assert_eq!(verifier_cible(Soin::GrandElixir, &monstre), Ok(()));
        assert!(soin_sans_effet(Soin::GrandElixir, &monstre));
        monstre.subir_degats(1);
        assert_eq!(
            appliquer(Soin::GrandElixir, &mut monstre, &[]),
            EffetDuSoin::PvRendus(20)
        );
        assert_eq!(Soin::GrandElixir.vendu_avec(), Soin::Elixir);
    }

    /// Section 14.4, décidée le 26/10/2026 pour l'étape 142.
    #[test]
    fn une_panacee_rend_tous_les_pv_et_guerit_le_statut_sans_relever() {
        let mut monstre = blesse(5);
        monstre.infliger_statut(Statut::Paralysie);
        assert!(!soin_sans_effet(Soin::Panacee, &monstre));
        assert_eq!(
            appliquer(Soin::Panacee, &mut monstre, &[]),
            EffetDuSoin::ToutGueri {
                pv: 15,
                statut: Some(Statut::Paralysie)
            }
        );
        assert_eq!(monstre.pv_actuels(), 20);
        assert_eq!(monstre.statut(), None);
        assert!(soin_sans_effet(Soin::Panacee, &monstre));
        // Un statut seul suffit à lui donner un effet.
        monstre.infliger_statut(Statut::Poison);
        assert!(!soin_sans_effet(Soin::Panacee, &monstre));
        let ko = blesse(0);
        assert_eq!(
            verifier_cible(Soin::Panacee, &ko),
            Err(ErreurSoin::MonstreKo)
        );
        assert_eq!(Soin::Panacee.vendu_avec(), Soin::Panacee);
    }

    #[test]
    fn un_remede_guerit_le_statut() {
        let mut monstre = blesse(10);
        monstre.infliger_statut(Statut::Poison);
        assert!(!soin_sans_effet(Soin::Remede, &monstre));
        assert_eq!(
            appliquer(Soin::Remede, &mut monstre, &[]),
            EffetDuSoin::StatutGueri(Some(Statut::Poison))
        );
        assert_eq!(monstre.statut(), None);
        assert!(soin_sans_effet(Soin::Remede, &monstre));
    }

    #[test]
    fn un_elixir_releve_un_monstre_ko_avec_la_moitie_de_ses_pv() {
        let mut monstre = blesse(0);
        assert_eq!(
            verifier_cible(Soin::Potion, &monstre),
            Err(ErreurSoin::MonstreKo)
        );
        assert_eq!(verifier_cible(Soin::Elixir, &monstre), Ok(()));
        assert_eq!(
            appliquer(Soin::Elixir, &mut monstre, &[]),
            EffetDuSoin::PvRendus(10)
        );
        assert!(!monstre.est_ko());
        // Debout, il n'en a pas besoin.
        assert!(soin_sans_effet(Soin::Elixir, &monstre));
    }

    #[test]
    fn soins_comptes_par_genre() {
        let mut soins = Soins::default();
        soins.ajouter(Soin::Remede, 2);
        assert!(!soins.retirer(Soin::Remede, 3));
        assert!(soins.retirer(Soin::Remede, 1));
        assert_eq!(soins.nombre(Soin::Remede), 1);
        assert_eq!(
            soins.presents().collect::<Vec<_>>(),
            vec![(Soin::Remede, 1)]
        );
    }
}
