//! Filets de capture (sections 11.2 et 14.3) : trois qualités, rangées dans
//! le sac et le coffre, que chaque capture en combat consomme. Malvoise en
//! vend, Gunther en fabrique, et les coffres d'expédition en contiennent
//! parfois.

/// Du moins cher au plus efficace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Filet {
    Chanvre,
    Leste,
    Chaines,
}

impl Filet {
    pub const TOUS: [Filet; 3] = [Filet::Chanvre, Filet::Leste, Filet::Chaines];

    /// Pour la sauvegarde et les interfaces.
    pub fn id(self) -> &'static str {
        match self {
            Filet::Chanvre => "filet-de-chanvre",
            Filet::Leste => "filet-leste",
            Filet::Chaines => "filet-de-chaines",
        }
    }

    pub fn depuis_id(id: &str) -> Option<Filet> {
        Filet::TOUS.into_iter().find(|filet| filet.id() == id)
    }

    /// Ce qui multiplie la chance de capture, en fraction : x1, x1,5 et x2
    /// (docs/contenu-v1.md, section 26).
    pub fn multiplicateur(self) -> (u64, u64) {
        match self {
            Filet::Chanvre => (1, 1),
            Filet::Leste => (3, 2),
            Filet::Chaines => (2, 1),
        }
    }

    fn position(self) -> usize {
        match self {
            Filet::Chanvre => 0,
            Filet::Leste => 1,
            Filet::Chaines => 2,
        }
    }
}

/// Le nombre de filets de chaque qualité, dans le sac, le coffre du camp ou
/// un combat.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Filets([u32; 3]);

impl Filets {
    pub fn nombre(&self, filet: Filet) -> u32 {
        self.0[filet.position()]
    }

    pub fn ajouter(&mut self, filet: Filet, nombre: u32) {
        let actuel = &mut self.0[filet.position()];
        *actuel = actuel.saturating_add(nombre);
    }

    /// Rend `false`, sans rien changer, s'il n'y en a pas assez.
    pub(crate) fn retirer(&mut self, filet: Filet, nombre: u32) -> bool {
        let actuel = &mut self.0[filet.position()];
        if *actuel < nombre {
            return false;
        }
        *actuel -= nombre;
        true
    }

    pub fn est_vide(&self) -> bool {
        self.0.iter().all(|nombre| *nombre == 0)
    }

    /// Ceux qu'il y a, dans l'ordre de `Filet::TOUS`, avec leur nombre.
    pub fn presents(&self) -> impl Iterator<Item = (Filet, u32)> + '_ {
        Filet::TOUS
            .into_iter()
            .map(|filet| (filet, self.nombre(filet)))
            .filter(|(_, nombre)| *nombre > 0)
    }

    /// Le moins efficace qui reste : celui que prend la capture d'un monstre
    /// KO, garantie quel que soit le filet (section 11.2).
    pub fn le_plus_simple(&self) -> Option<Filet> {
        self.presents().map(|(filet, _)| filet).next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filets_ajoutes_retires_et_le_plus_simple() {
        let mut filets = Filets::default();
        assert!(filets.est_vide());
        assert_eq!(filets.le_plus_simple(), None);
        filets.ajouter(Filet::Chaines, 2);
        filets.ajouter(Filet::Leste, 1);
        assert_eq!(filets.le_plus_simple(), Some(Filet::Leste));
        assert!(!filets.retirer(Filet::Leste, 2));
        assert!(filets.retirer(Filet::Leste, 1));
        assert_eq!(
            filets.presents().collect::<Vec<_>>(),
            vec![(Filet::Chaines, 2)]
        );
        assert_eq!(Filet::depuis_id("filet-leste"), Some(Filet::Leste));
        assert_eq!(Filet::depuis_id("filet"), None);
    }
}
