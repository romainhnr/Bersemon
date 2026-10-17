//! Combat au tour par tour entre le monstre du joueur et un monstre sauvage
//! (section 10). La réserve, le changement de monstre, la fuite et la capture
//! ne sont pas encore gérés.
//!
//! Le combat ne fait aucun affichage : chaque tour rend la liste des
//! événements survenus, que l'interface met en texte. Il ne bloque pas non
//! plus en attendant le joueur, ce qui convient à la console comme à un bot
//! qui traite une commande à la fois.
//!
//! Tirages d'un tour, dans l'ordre : attaque choisie par l'IA (si son monstre
//! en connaît plusieurs), ordre d'action en cas d'égalité de vitesse, puis,
//! pour chaque attaque lancée, ceux de la résolution d'une attaque.

use std::cmp::Ordering;
use std::fmt;

use crate::aleatoire::Aleatoire;
use crate::attaque::Attaque;
use crate::degats::{ResultatAttaque, resoudre_attaque};
use crate::monstre::Monstre;
use crate::types::Efficacite;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Issue {
    Victoire,
    Defaite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Position de l'attaque dans la liste des attaques du monstre.
    Attaquer(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evenement {
    AttaqueLancee {
        cote: Cote,
        attaque: Attaque,
    },
    AttaqueRatee {
        cote: Cote,
    },
    SansEffet {
        cible: Cote,
    },
    Degats {
        cible: Cote,
        degats: u32,
        critique: bool,
        efficacite: Efficacite,
    },
    Ko {
        cote: Cote,
    },
    FinDuCombat {
        issue: Issue,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurCombat {
    MonstreKo(Cote),
    CombatTermine,
    AttaqueInexistante(usize),
}

impl fmt::Display for ErreurCombat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurCombat::MonstreKo(Cote::Joueur) => {
                write!(f, "le monstre du joueur est KO, il ne peut pas combattre")
            }
            ErreurCombat::MonstreKo(Cote::Adversaire) => {
                write!(f, "le monstre adverse est KO, il ne peut pas combattre")
            }
            ErreurCombat::CombatTermine => write!(f, "le combat est déjà terminé"),
            ErreurCombat::AttaqueInexistante(index) => {
                write!(f, "aucune attaque en position {index}")
            }
        }
    }
}

impl std::error::Error for ErreurCombat {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Combat {
    joueur: Monstre,
    adversaire: Monstre,
    issue: Option<Issue>,
}

impl Combat {
    /// Les monstres sont rendus avec leurs PV par `into_monstres`, une fois le
    /// combat terminé.
    pub fn new(joueur: Monstre, adversaire: Monstre) -> Result<Self, ErreurCombat> {
        if joueur.est_ko() {
            return Err(ErreurCombat::MonstreKo(Cote::Joueur));
        }
        if adversaire.est_ko() {
            return Err(ErreurCombat::MonstreKo(Cote::Adversaire));
        }
        Ok(Self {
            joueur,
            adversaire,
            issue: None,
        })
    }

    pub fn joueur(&self) -> &Monstre {
        &self.joueur
    }

    pub fn adversaire(&self) -> &Monstre {
        &self.adversaire
    }

    pub fn issue(&self) -> Option<Issue> {
        self.issue
    }

    /// Joue un tour complet : l'action du joueur et celle de l'IA, dans
    /// l'ordre de vitesse. Une action refusée ne consomme aucun tirage et ne
    /// modifie pas le combat.
    pub fn jouer_tour(
        &mut self,
        action: Action,
        aleatoire: &mut dyn Aleatoire,
    ) -> Result<Vec<Evenement>, ErreurCombat> {
        if self.issue.is_some() {
            return Err(ErreurCombat::CombatTermine);
        }
        let attaque_joueur = match action {
            Action::Attaquer(index) => {
                if index >= self.joueur.attaques().liste().len() {
                    return Err(ErreurCombat::AttaqueInexistante(index));
                }
                index
            }
        };
        let attaque_adversaire = choisir_attaque_au_hasard(&self.adversaire, aleatoire);

        let mut evenements = Vec::new();
        for cote in self.ordre_du_tour(aleatoire) {
            let index = match cote {
                Cote::Joueur => attaque_joueur,
                Cote::Adversaire => attaque_adversaire,
            };
            self.lancer_attaque(cote, index, aleatoire, &mut evenements);
            if self.issue.is_some() {
                break;
            }
        }
        Ok(evenements)
    }

    /// Rend le monstre du joueur puis celui de l'adversaire.
    pub fn into_monstres(self) -> (Monstre, Monstre) {
        (self.joueur, self.adversaire)
    }

    /// Le plus rapide agit en premier ; à vitesse égale, tirage au sort à
    /// chaque tour (Q23).
    fn ordre_du_tour(&self, aleatoire: &mut dyn Aleatoire) -> [Cote; 2] {
        let vitesse_joueur = self.joueur.stats().vitesse;
        let vitesse_adversaire = self.adversaire.stats().vitesse;
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

    fn lancer_attaque(
        &mut self,
        cote: Cote,
        index_attaque: usize,
        aleatoire: &mut dyn Aleatoire,
        evenements: &mut Vec<Evenement>,
    ) {
        let (attaquant, defenseur) = match cote {
            Cote::Joueur => (&self.joueur, &mut self.adversaire),
            Cote::Adversaire => (&self.adversaire, &mut self.joueur),
        };
        let cible = cote.oppose();
        let attaque = attaquant.attaques().liste()[index_attaque].clone();
        let resultat = resoudre_attaque(attaquant, &attaque, defenseur, aleatoire);
        evenements.push(Evenement::AttaqueLancee { cote, attaque });
        match resultat {
            ResultatAttaque::Ratee => evenements.push(Evenement::AttaqueRatee { cote }),
            ResultatAttaque::SansEffet => evenements.push(Evenement::SansEffet { cible }),
            ResultatAttaque::Touchee {
                degats,
                critique,
                efficacite,
            } => {
                defenseur.subir_degats(degats);
                evenements.push(Evenement::Degats {
                    cible,
                    degats,
                    critique,
                    efficacite,
                });
                if defenseur.est_ko() {
                    evenements.push(Evenement::Ko { cote: cible });
                    let issue = match cible {
                        Cote::Adversaire => Issue::Victoire,
                        Cote::Joueur => Issue::Defaite,
                    };
                    self.issue = Some(issue);
                    evenements.push(Evenement::FinDuCombat { issue });
                }
            }
        }
    }
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
    use crate::stats::Stats;
    use crate::types::Type;

    // Base 50 au niveau 5 avec des valeurs internes nulles : 20 PV et 10 dans
    // les autres stats. Une base de vitesse de 60 donne 11. Le coup inflige
    // alors 5 dégâts avec un pourcentage de 100, sans critique.

    fn coup() -> Attaque {
        attaque("coup", Type::Normal, Categorie::Physique, 40, 100)
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

    fn degats(cible: Cote, degats: u32) -> Evenement {
        Evenement::Degats {
            cible,
            degats,
            critique: false,
            efficacite: Efficacite::Normale,
        }
    }

    fn lancee(cote: Cote, attaque: Attaque) -> Evenement {
        Evenement::AttaqueLancee { cote, attaque }
    }

    #[test]
    fn le_plus_rapide_agit_en_premier() {
        let mut combat = Combat::new(rapide(), lent()).unwrap();
        let mut aleatoire = AleatoireScripte::new([2, 100, 2, 100]);
        let evenements = combat
            .jouer_tour(Action::Attaquer(0), &mut aleatoire)
            .unwrap();
        assert_eq!(
            evenements,
            vec![
                lancee(Cote::Joueur, coup()),
                degats(Cote::Adversaire, 5),
                lancee(Cote::Adversaire, coup()),
                degats(Cote::Joueur, 5),
            ]
        );
        assert_eq!(combat.joueur().pv_actuels(), 15);
        assert_eq!(combat.adversaire().pv_actuels(), 15);
        assert_eq!(combat.issue(), None);
    }

    #[test]
    fn egalite_de_vitesse_tiree_au_sort() {
        let mut combat = Combat::new(lent(), lent()).unwrap();
        let mut aleatoire = AleatoireScripte::new([1, 2, 100, 2, 100]);
        let evenements = combat
            .jouer_tour(Action::Attaquer(0), &mut aleatoire)
            .unwrap();
        assert_eq!(evenements[0], lancee(Cote::Adversaire, coup()));
        assert_eq!(evenements[2], lancee(Cote::Joueur, coup()));
    }

    #[test]
    fn ko_de_l_adversaire_donne_la_victoire_sans_riposte() {
        let mut adversaire = lent();
        adversaire.subir_degats(17);
        let mut combat = Combat::new(rapide(), adversaire).unwrap();
        let mut aleatoire = AleatoireScripte::new([2, 100]);
        let evenements = combat
            .jouer_tour(Action::Attaquer(0), &mut aleatoire)
            .unwrap();
        assert_eq!(
            evenements,
            vec![
                lancee(Cote::Joueur, coup()),
                degats(Cote::Adversaire, 5),
                Evenement::Ko {
                    cote: Cote::Adversaire
                },
                Evenement::FinDuCombat {
                    issue: Issue::Victoire
                },
            ]
        );
        assert_eq!(combat.issue(), Some(Issue::Victoire));
    }

    #[test]
    fn ko_du_joueur_donne_la_defaite() {
        let mut joueur = lent();
        joueur.subir_degats(17);
        let mut combat = Combat::new(joueur, rapide()).unwrap();
        let mut aleatoire = AleatoireScripte::new([2, 100]);
        let evenements = combat
            .jouer_tour(Action::Attaquer(0), &mut aleatoire)
            .unwrap();
        assert_eq!(
            evenements.last(),
            Some(&Evenement::FinDuCombat {
                issue: Issue::Defaite
            })
        );
        assert_eq!(combat.issue(), Some(Issue::Defaite));
    }

    #[test]
    fn aucun_tour_apres_la_fin_du_combat() {
        let mut adversaire = lent();
        adversaire.subir_degats(17);
        let mut combat = Combat::new(rapide(), adversaire).unwrap();
        combat
            .jouer_tour(Action::Attaquer(0), &mut AleatoireScripte::new([2, 100]))
            .unwrap();
        assert_eq!(
            combat.jouer_tour(Action::Attaquer(0), &mut AleatoireScripte::new([])),
            Err(ErreurCombat::CombatTermine)
        );
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
        let mut combat = Combat::new(rapide(), adversaire).unwrap();
        let mut aleatoire = AleatoireScripte::new([1, 2, 100, 2, 100]);
        let evenements = combat
            .jouer_tour(Action::Attaquer(0), &mut aleatoire)
            .unwrap();
        assert_eq!(evenements[2], lancee(Cote::Adversaire, morsure));
    }

    #[test]
    fn cible_immunisee() {
        let spectre = monstre(&[Type::Spectre], stats_uniformes(50), 5, vec![coup()]);
        let mut combat = Combat::new(rapide(), spectre).unwrap();
        // Le coup du joueur, de type Normal, n'a aucun effet sur le Spectre et
        // ne tire rien ; seule l'attaque du Spectre contre le joueur demande
        // des tirages.
        let mut aleatoire = AleatoireScripte::new([2, 100]);
        let evenements = combat
            .jouer_tour(Action::Attaquer(0), &mut aleatoire)
            .unwrap();
        assert_eq!(
            evenements[..2],
            [
                lancee(Cote::Joueur, coup()),
                Evenement::SansEffet {
                    cible: Cote::Adversaire
                },
            ]
        );
        assert_eq!(combat.adversaire().pv_actuels(), 20);
    }

    #[test]
    fn attaque_inexistante_refusee_sans_rien_changer() {
        let mut combat = Combat::new(rapide(), lent()).unwrap();
        let avant = combat.clone();
        assert_eq!(
            combat.jouer_tour(Action::Attaquer(1), &mut AleatoireScripte::new([])),
            Err(ErreurCombat::AttaqueInexistante(1))
        );
        assert_eq!(combat, avant);
    }

    #[test]
    fn refuse_un_monstre_deja_ko() {
        let mut joueur = lent();
        joueur.subir_degats(u32::MAX);
        assert_eq!(
            Combat::new(joueur, lent()),
            Err(ErreurCombat::MonstreKo(Cote::Joueur))
        );
    }

    #[test]
    fn les_monstres_gardent_leurs_pv_apres_le_combat() {
        let mut combat = Combat::new(rapide(), lent()).unwrap();
        combat
            .jouer_tour(
                Action::Attaquer(0),
                &mut AleatoireScripte::new([2, 100, 2, 85]),
            )
            .unwrap();
        let (joueur, adversaire) = combat.into_monstres();
        assert_eq!(joueur.pv_actuels(), 16);
        assert_eq!(adversaire.pv_actuels(), 15);
    }
}
