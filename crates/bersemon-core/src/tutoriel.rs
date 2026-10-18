//! Tutoriel de la V1 (section 6) : quelques conseils, chacun montré une seule
//! fois par partie, au moment où il devient utile.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Conseil {
    PremierCombat,
    PremierArretEntreDeuxEtapes,
    PremierRetourAuCamp,
}
