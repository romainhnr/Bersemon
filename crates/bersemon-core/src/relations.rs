//! Relations d'un joueur avec les autres joueurs du bot (section 15.4) :
//! sa liste d'amis, qu'il tient seul, et son palmarès des duels. La
//! console, partie seule, les garde sans les montrer.

/// Identifiant d'un autre joueur, donné par l'interface : sur Discord,
/// celui de son compte.
pub type IdJoueur = u64;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Relations {
    /// Sans doublon, dans l'ordre où ils ont été ajoutés.
    amis: Vec<IdJoueur>,
    palmares: Palmares,
}

/// Victoires et défaites en duel, abandons et forfaits compris (Q94).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Palmares {
    pub victoires: u32,
    pub defaites: u32,
}

impl Palmares {
    pub fn duels(&self) -> u32 {
        self.victoires.saturating_add(self.defaites)
    }
}

impl Relations {
    /// Reprise d'une sauvegarde ; les doublons ne sont gardés qu'une fois.
    pub fn restaurer(amis: Vec<IdJoueur>, palmares: Palmares) -> Self {
        let mut relations = Self {
            amis: Vec::new(),
            palmares,
        };
        for ami in amis {
            relations.ajouter_ami(ami);
        }
        relations
    }

    pub fn amis(&self) -> &[IdJoueur] {
        &self.amis
    }

    pub fn est_ami(&self, joueur: IdJoueur) -> bool {
        self.amis.contains(&joueur)
    }

    /// Rend `false` s'il l'était déjà.
    pub fn ajouter_ami(&mut self, joueur: IdJoueur) -> bool {
        if self.est_ami(joueur) {
            return false;
        }
        self.amis.push(joueur);
        true
    }

    /// Rend `false` s'il ne l'était pas.
    pub fn retirer_ami(&mut self, joueur: IdJoueur) -> bool {
        let avant = self.amis.len();
        self.amis.retain(|ami| *ami != joueur);
        self.amis.len() < avant
    }

    pub fn palmares(&self) -> Palmares {
        self.palmares
    }

    pub fn noter_duel(&mut self, victoire: bool) {
        let compte = if victoire {
            &mut self.palmares.victoires
        } else {
            &mut self.palmares.defaites
        };
        *compte = compte.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_ami_ne_s_ajoute_qu_une_fois() {
        let mut relations = Relations::restaurer(vec![7, 3, 7], Palmares::default());
        assert_eq!(relations.amis(), &[7, 3]);
        assert!(!relations.ajouter_ami(3));
        assert!(relations.ajouter_ami(9));
        assert!(relations.retirer_ami(7));
        assert!(!relations.retirer_ami(7));
        assert_eq!(relations.amis(), &[3, 9]);
    }

    #[test]
    fn le_palmares_compte_victoires_et_defaites() {
        let mut relations = Relations::default();
        relations.noter_duel(true);
        relations.noter_duel(false);
        relations.noter_duel(true);
        assert_eq!(
            relations.palmares(),
            Palmares {
                victoires: 2,
                defaites: 1
            }
        );
        assert_eq!(relations.palmares().duels(), 3);
    }
}
