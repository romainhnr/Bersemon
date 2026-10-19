//! Marchands morts sur les routes, un par zone : le joueur rapporte leur âme
//! à Malvoise, le nécromancien du camp, dont l'étal s'enrichit alors de
//! l'objet que vendait chacun (section 14.2).

use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::espece::{Espece, EspeceId};
use crate::humain::valeurs_internes_fixes;
use crate::monstre::Monstre;
use crate::niveau::Niveau;
use crate::soin::Soin;
use crate::zone::Zone;

/// Identifiant stable d'un marchand, enregistré dans la sauvegarde une fois
/// son âme recueillie.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarchandId(String);

impl MarchandId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Ce qui attaque le joueur quand il trouve le marchand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gardien {
    /// Une espèce commune de la zone, au niveau maximal de la zone : un
    /// combat ordinaire contre un monstre sauvage.
    MonstreDeLaZone,
    /// Le marchand relevé d'entre les morts : fuite possible, mais ni filet
    /// ni capture, et il n'entre pas au codex. Valeurs internes fixes,
    /// jamais shiny, comme les monstres d'un humain.
    MortVivant { espece: Espece, niveau: Niveau },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurMarchandMort {
    /// L'espèce du mort-vivant n'apprend aucune attaque, ou en apprend une
    /// en double, jusqu'à son niveau.
    EspeceInvalide { espece: EspeceId, niveau: Niveau },
}

impl fmt::Display for ErreurMarchandMort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurMarchandMort::EspeceInvalide { espece, niveau } => write!(
                f,
                "l'espèce {} ne peut pas être créée au niveau {}",
                espece.as_str(),
                niveau.valeur()
            ),
        }
    }
}

impl std::error::Error for ErreurMarchandMort {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarchandMort {
    id: MarchandId,
    nom: String,
    /// Ce que Malvoise vend une fois son âme recueillie.
    soin: Soin,
    gardien: Gardien,
    avant_combat: String,
    apres_victoire: String,
    /// L'étape où son âme se rencontre à coup sûr, tant qu'elle n'est pas
    /// recueillie ; sans elle, 10 % des étapes, puis une étape tirée après
    /// une fuite ou une défaite (section 14.2).
    etape: Option<u32>,
}

impl MarchandMort {
    /// Vérifie que le mort-vivant peut être créé à son niveau : sa
    /// génération ne peut ensuite plus échouer.
    pub fn new(
        id: MarchandId,
        nom: impl Into<String>,
        soin: Soin,
        gardien: Gardien,
        avant_combat: impl Into<String>,
        apres_victoire: impl Into<String>,
    ) -> Result<Self, ErreurMarchandMort> {
        if let Gardien::MortVivant { espece, niveau } = &gardien {
            Monstre::creer(espece.clone(), *niveau, valeurs_internes_fixes(), false).map_err(
                |_| ErreurMarchandMort::EspeceInvalide {
                    espece: espece.id().clone(),
                    niveau: *niveau,
                },
            )?;
        }
        Ok(Self {
            id,
            nom: nom.into(),
            soin,
            gardien,
            avant_combat: avant_combat.into(),
            apres_victoire: apres_victoire.into(),
            etape: None,
        })
    }

    pub fn a_l_etape(mut self, etape: u32) -> Self {
        self.etape = Some(etape);
        self
    }

    pub fn etape(&self) -> Option<u32> {
        self.etape
    }

    pub fn id(&self) -> &MarchandId {
        &self.id
    }

    pub fn nom(&self) -> &str {
        &self.nom
    }

    pub fn soin(&self) -> Soin {
        self.soin
    }

    pub fn gardien(&self) -> &Gardien {
        &self.gardien
    }

    pub fn avant_combat(&self) -> &str {
        &self.avant_combat
    }

    pub fn apres_victoire(&self) -> &str {
        &self.apres_victoire
    }

    /// Le monstre qui attaque le joueur dans `zone`. Tirages : ceux de
    /// `Zone::generer_gardien` pour un monstre de la zone, aucun pour un
    /// mort-vivant.
    pub(crate) fn generer_adversaire(&self, zone: &Zone, aleatoire: &mut dyn Aleatoire) -> Monstre {
        match &self.gardien {
            Gardien::MonstreDeLaZone => zone.generer_gardien(aleatoire),
            Gardien::MortVivant { espece, niveau } => {
                Monstre::creer(espece.clone(), *niveau, valeurs_internes_fixes(), false)
                    .expect("mort-vivant vérifié par MarchandMort::new")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aleatoire::AleatoireScripte;
    use crate::attaque::Categorie;
    use crate::donnees_de_test::{attaque, espece, stats_uniformes, zone};
    use crate::espece::{AttaqueApprise, Role};
    use crate::types::{Type, TypesMonstre};

    fn niveau(valeur: u8) -> Niveau {
        Niveau::new(valeur).unwrap()
    }

    fn mort(gardien: Gardien) -> Result<MarchandMort, ErreurMarchandMort> {
        MarchandMort::new(
            MarchandId::new("grete"),
            "Grete",
            Soin::GrandePotion,
            gardien,
            "",
            "",
        )
    }

    #[test]
    fn un_mort_vivant_est_fixe_sans_tirage() {
        let grete = mort(Gardien::MortVivant {
            espece: espece("zombie"),
            niveau: niveau(13),
        })
        .unwrap();
        let mut aleatoire = AleatoireScripte::new([]);
        let zombie = grete.generer_adversaire(&zone("marais"), &mut aleatoire);
        assert_eq!(zombie.niveau(), niveau(13));
        assert!(!zombie.est_shiny());
        assert_eq!(zombie.valeurs_internes().valeurs().attaque, 15);
    }

    #[test]
    fn un_monstre_de_la_zone_garde_le_corps_au_niveau_maximal() {
        let bertrand = mort(Gardien::MonstreDeLaZone).unwrap();
        // Une seule espèce et un seul niveau, sans tirage : shiny (2 : non),
        // puis six valeurs internes ; l'espèce n'a pas d'habileté à tirer.
        let mut aleatoire = AleatoireScripte::new([2, 0, 0, 0, 0, 0, 0]);
        let loup = bertrand.generer_adversaire(&zone("lisiere"), &mut aleatoire);
        assert_eq!(loup.espece().id().as_str(), "loup");
        assert_eq!(loup.niveau(), niveau(5));
    }

    #[test]
    fn mort_vivant_invalide_refuse() {
        let tardive = Espece::new(
            EspeceId::new("tardive"),
            "Tardive",
            TypesMonstre::new(&[Type::Feu]).unwrap(),
            Role::Offensif,
            stats_uniformes(50),
            64,
            vec![AttaqueApprise {
                niveau: niveau(9),
                attaque: attaque("coup", Type::Normal, Categorie::Physique, 40, 100),
            }],
        );
        assert_eq!(
            mort(Gardien::MortVivant {
                espece: tardive,
                niveau: niveau(8),
            }),
            Err(ErreurMarchandMort::EspeceInvalide {
                espece: EspeceId::new("tardive"),
                niveau: niveau(8),
            })
        );
    }
}
