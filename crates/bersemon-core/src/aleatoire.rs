//! Source de hasard du jeu.
//!
//! Le noyau ne tire jamais de hasard par lui-même : il reçoit un `Aleatoire`
//! de l'appelant. L'interface fournit un générateur initialisé par une graine,
//! les tests imposent leurs tirages, ce qui rend chaque règle vérifiable de
//! façon exacte.

use std::ops::RangeInclusive;

#[cfg(test)]
pub(crate) use scripte::AleatoireScripte;

/// Fournisseur de tirages aléatoires.
pub trait Aleatoire {
    /// Tire un entier uniformément dans `plage`, bornes comprises.
    ///
    /// Une plage vide est une erreur de programmation : les implémentations
    /// paniquent dans ce cas.
    fn tirer(&mut self, plage: RangeInclusive<u32>) -> u32;
}

/// Générateur SplitMix64 : simple, rapide et de qualité suffisante pour un
/// jeu, sans dépendance externe. Il ne convient pas à un usage cryptographique.
#[derive(Debug, Clone)]
pub struct GenerateurAleatoire {
    etat: u64,
}

impl GenerateurAleatoire {
    /// Deux générateurs créés avec la même graine produisent la même suite de
    /// tirages.
    pub fn new(graine: u64) -> Self {
        Self { etat: graine }
    }

    fn suivant(&mut self) -> u64 {
        self.etat = self.etat.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.etat;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

impl Aleatoire for GenerateurAleatoire {
    fn tirer(&mut self, plage: RangeInclusive<u32>) -> u32 {
        let (min, max) = plage.into_inner();
        assert!(min <= max, "plage de tirage vide : {min} à {max}");
        let taille = u64::from(max - min) + 1;
        // Un simple modulo favoriserait légèrement les petites valeurs. On
        // rejette les tirages situés sous ce seuil, pour que le nombre de
        // tirages acceptés soit un multiple exact de la taille de la plage.
        let seuil = taille.wrapping_neg() % taille;
        loop {
            let tirage = self.suivant();
            if tirage >= seuil {
                // Le reste est inférieur à `taille`, donc tient dans un u32.
                return min + (tirage % taille) as u32;
            }
        }
    }
}

#[cfg(test)]
mod scripte {
    use super::Aleatoire;
    use std::collections::VecDeque;
    use std::ops::RangeInclusive;

    /// Aléatoire de test : rend les tirages imposés, dans l'ordre.
    pub(crate) struct AleatoireScripte {
        tirages: VecDeque<u32>,
    }

    impl AleatoireScripte {
        pub(crate) fn new(tirages: impl IntoIterator<Item = u32>) -> Self {
            Self {
                tirages: tirages.into_iter().collect(),
            }
        }
    }

    impl Aleatoire for AleatoireScripte {
        fn tirer(&mut self, plage: RangeInclusive<u32>) -> u32 {
            let tirage = self
                .tirages
                .pop_front()
                .expect("aucun tirage scripté restant");
            // Un tirage hors plage révèle un test mal écrit, ou une règle qui
            // ne demande pas la plage attendue.
            assert!(
                plage.contains(&tirage),
                "tirage scripté {tirage} hors de la plage {plage:?}"
            );
            tirage
        }
    }

    impl Drop for AleatoireScripte {
        fn drop(&mut self) {
            // Des tirages restants signalent qu'une règle tire moins souvent
            // que le test ne le suppose. Rien n'est vérifié pendant une
            // panique déjà en cours, pour ne pas masquer la première erreur.
            if !std::thread::panicking() {
                assert!(
                    self.tirages.is_empty(),
                    "tirages scriptés non consommés : {:?}",
                    self.tirages
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn premier_tirage_conforme_a_splitmix64() {
        // Valeur de référence publiée de SplitMix64 pour la graine 0.
        let mut generateur = GenerateurAleatoire::new(0);
        assert_eq!(generateur.suivant(), 0xE220_A839_7B1D_CDAF);
    }

    #[test]
    fn meme_graine_meme_suite() {
        let mut premier = GenerateurAleatoire::new(42);
        let mut second = GenerateurAleatoire::new(42);
        for _ in 0..100 {
            assert_eq!(premier.tirer(0..=1000), second.tirer(0..=1000));
        }
    }

    #[test]
    fn tirage_reste_dans_la_plage_et_couvre_toutes_les_valeurs() {
        let mut generateur = GenerateurAleatoire::new(7);
        let mut vus = [false; 5];
        for _ in 0..1000 {
            let tirage = generateur.tirer(3..=7);
            assert!((3..=7).contains(&tirage));
            vus[(tirage - 3) as usize] = true;
        }
        assert!(vus.iter().all(|&vu| vu));
    }

    #[test]
    fn plage_d_une_seule_valeur() {
        let mut generateur = GenerateurAleatoire::new(1);
        assert_eq!(generateur.tirer(9..=9), 9);
    }

    #[test]
    fn plage_complete_sans_debordement() {
        let mut generateur = GenerateurAleatoire::new(1);
        generateur.tirer(0..=u32::MAX);
    }

    #[test]
    fn repartition_uniforme() {
        let mut generateur = GenerateurAleatoire::new(2026);
        let mut effectifs = [0u32; 4];
        for _ in 0..40_000 {
            effectifs[generateur.tirer(0..=3) as usize] += 1;
        }
        for effectif in effectifs {
            assert!((9_500..=10_500).contains(&effectif), "{effectifs:?}");
        }
    }

    #[test]
    #[should_panic(expected = "plage de tirage vide")]
    fn plage_vide_panique() {
        let mut generateur = GenerateurAleatoire::new(1);
        generateur.tirer(RangeInclusive::new(5, 4));
    }

    #[test]
    fn aleatoire_scripte_rend_les_tirages_dans_l_ordre() {
        let mut aleatoire = AleatoireScripte::new([4, 0, 31]);
        assert_eq!(aleatoire.tirer(0..=31), 4);
        assert_eq!(aleatoire.tirer(0..=31), 0);
        assert_eq!(aleatoire.tirer(0..=31), 31);
    }

    #[test]
    #[should_panic(expected = "hors de la plage")]
    fn aleatoire_scripte_refuse_un_tirage_hors_plage() {
        let mut aleatoire = AleatoireScripte::new([32]);
        aleatoire.tirer(0..=31);
    }

    #[test]
    #[should_panic(expected = "non consommés")]
    fn aleatoire_scripte_signale_les_tirages_restants() {
        let _aleatoire = AleatoireScripte::new([1]);
    }
}
