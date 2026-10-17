//! Codex : ce que le joueur sait de chaque espèce (section 5.6).

use crate::espece::EspeceId;

/// Une espèce rencontrée montre son nom et ses types ; une espèce capturée
/// montre aussi ses stats de base et ses attaques.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtatCodex {
    Inconnue,
    Rencontree,
    Capturee,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Codex {
    rencontrees: Vec<EspeceId>,
    capturees: Vec<EspeceId>,
}

impl Codex {
    pub fn noter_rencontre(&mut self, id: &EspeceId) {
        if !self.rencontrees.contains(id) {
            self.rencontrees.push(id.clone());
        }
    }

    /// Une capture vaut aussi rencontre.
    pub fn noter_capture(&mut self, id: &EspeceId) {
        self.noter_rencontre(id);
        if !self.capturees.contains(id) {
            self.capturees.push(id.clone());
        }
    }

    /// Dans l'ordre des rencontres, captures comprises.
    pub fn rencontrees(&self) -> &[EspeceId] {
        &self.rencontrees
    }

    pub fn capturees(&self) -> &[EspeceId] {
        &self.capturees
    }

    pub fn etat(&self, id: &EspeceId) -> EtatCodex {
        if self.capturees.contains(id) {
            EtatCodex::Capturee
        } else if self.rencontrees.contains(id) {
            EtatCodex::Rencontree
        } else {
            EtatCodex::Inconnue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rencontre_puis_capture() {
        let loup = EspeceId::new("loup");
        let mut codex = Codex::default();
        assert_eq!(codex.etat(&loup), EtatCodex::Inconnue);
        codex.noter_rencontre(&loup);
        assert_eq!(codex.etat(&loup), EtatCodex::Rencontree);
        codex.noter_capture(&loup);
        assert_eq!(codex.etat(&loup), EtatCodex::Capturee);
        codex.noter_rencontre(&loup);
        assert_eq!(codex.etat(&loup), EtatCodex::Capturee);
    }

    #[test]
    fn une_capture_vaut_rencontre() {
        let vipere = EspeceId::new("vipere");
        let mut codex = Codex::default();
        codex.noter_capture(&vipere);
        assert_eq!(codex.etat(&vipere), EtatCodex::Capturee);
    }
}
