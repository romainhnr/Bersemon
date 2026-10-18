//! Heure du monde réel, pour les expéditions automatiques qui s'écoulent en
//! temps réel (section 12.2).
//!
//! Le noyau ne lit jamais l'horloge : l'interface lui donne l'heure. Les
//! durées comptent ainsi même quand le jeu est fermé ou le bot éteint, et
//! les tests fixent l'heure qu'ils veulent.

use std::time::Duration;

/// Instant compté en secondes depuis le 1er janvier 1970 (UTC).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Horodatage(u64);

impl Horodatage {
    pub fn depuis_secondes(secondes: u64) -> Self {
        Self(secondes)
    }

    pub fn secondes(self) -> u64 {
        self.0
    }

    /// Les fractions de seconde de la durée sont ignorées.
    pub fn plus(self, duree: Duration) -> Self {
        Self(self.0.saturating_add(duree.as_secs()))
    }

    /// Temps écoulé jusqu'à `plus_tard`. Nul si `plus_tard` est plus tôt,
    /// par exemple quand l'horloge du PC a été reculée : une durée ne devient
    /// jamais négative.
    pub fn duree_jusqu_a(self, plus_tard: Horodatage) -> Duration {
        Duration::from_secs(plus_tard.0.saturating_sub(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn une_heure_plus_tard() {
        let depart = Horodatage::depuis_secondes(1_000);
        let retour = depart.plus(Duration::from_secs(3_600));
        assert_eq!(retour.secondes(), 4_600);
        assert_eq!(depart.duree_jusqu_a(retour), Duration::from_secs(3_600));
    }

    #[test]
    fn une_horloge_reculee_ne_donne_pas_de_duree_negative() {
        let depart = Horodatage::depuis_secondes(1_000);
        let avant = Horodatage::depuis_secondes(400);
        assert_eq!(depart.duree_jusqu_a(avant), Duration::ZERO);
    }
}
