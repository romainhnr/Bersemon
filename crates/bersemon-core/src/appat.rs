//! Appâts (section 11.2) : trois niveaux, qu'Ansgar prépare au camp avec les
//! matières que remonte sa ligne (section 13.8). Lancé en combat, un appât
//! prend le tour et multiplie la chance du prochain filet lancé sur le
//! monstre qui l'a reçu. Ils se rangent dans le sac et le coffre, comme les
//! filets.

/// Du plus faible au plus fort.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Appat {
    Vers,
    Chair,
    Ambre,
}

impl Appat {
    pub const TOUS: [Appat; 3] = [Appat::Vers, Appat::Chair, Appat::Ambre];

    /// Pour la sauvegarde et les interfaces.
    pub fn id(self) -> &'static str {
        match self {
            Appat::Vers => "appat-de-vers",
            Appat::Chair => "appat-de-chair",
            Appat::Ambre => "appat-d-ambre",
        }
    }

    pub fn depuis_id(id: &str) -> Option<Appat> {
        Appat::TOUS.into_iter().find(|appat| appat.id() == id)
    }

    /// Ce qui multiplie la chance du prochain filet, en fraction : x1,25,
    /// x1,5 et x2 (docs/contenu-v1.md, section 53).
    pub fn multiplicateur(self) -> (u64, u64) {
        match self {
            Appat::Vers => (5, 4),
            Appat::Chair => (3, 2),
            Appat::Ambre => (2, 1),
        }
    }

    fn position(self) -> usize {
        match self {
            Appat::Vers => 0,
            Appat::Chair => 1,
            Appat::Ambre => 2,
        }
    }
}

/// Le nombre d'appâts de chaque niveau, dans le sac, le coffre du camp ou
/// un combat.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Appats([u32; 3]);

impl Appats {
    pub fn nombre(&self, appat: Appat) -> u32 {
        self.0[appat.position()]
    }

    pub fn ajouter(&mut self, appat: Appat, nombre: u32) {
        let actuel = &mut self.0[appat.position()];
        *actuel = actuel.saturating_add(nombre);
    }

    /// Rend `false`, sans rien changer, s'il n'y en a pas assez.
    pub(crate) fn retirer(&mut self, appat: Appat, nombre: u32) -> bool {
        let actuel = &mut self.0[appat.position()];
        if *actuel < nombre {
            return false;
        }
        *actuel -= nombre;
        true
    }

    pub fn est_vide(&self) -> bool {
        self.0.iter().all(|nombre| *nombre == 0)
    }

    /// Ceux qu'il y a, dans l'ordre de `Appat::TOUS`, avec leur nombre.
    pub fn presents(&self) -> impl Iterator<Item = (Appat, u32)> + '_ {
        Appat::TOUS
            .into_iter()
            .map(|appat| (appat, self.nombre(appat)))
            .filter(|(_, nombre)| *nombre > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appats_ajoutes_retires_et_presents() {
        let mut appats = Appats::default();
        assert!(appats.est_vide());
        appats.ajouter(Appat::Ambre, 2);
        appats.ajouter(Appat::Vers, 1);
        assert!(!appats.retirer(Appat::Vers, 2));
        assert!(appats.retirer(Appat::Vers, 1));
        assert_eq!(
            appats.presents().collect::<Vec<_>>(),
            vec![(Appat::Ambre, 2)]
        );
        assert_eq!(Appat::depuis_id("appat-de-chair"), Some(Appat::Chair));
        assert_eq!(Appat::depuis_id("appat"), None);
    }

    #[test]
    fn chaque_niveau_multiplie_plus_que_le_precedent() {
        let valeurs: Vec<f64> = Appat::TOUS
            .into_iter()
            .map(|appat| {
                let (numerateur, denominateur) = appat.multiplicateur();
                numerateur as f64 / denominateur as f64
            })
            .collect();
        assert_eq!(valeurs, vec![1.25, 1.5, 2.0]);
    }
}
