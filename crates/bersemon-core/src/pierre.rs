//! Pierres d'évolution (section 5.4) : chacune fait évoluer, au camp, les
//! espèces dont l'évolution la demande, et se consomme.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PierreDEvolution {
    Braise,
    Lune,
}

impl PierreDEvolution {
    /// Dans l'ordre de docs/contenu-v1.md, section 6, qui est aussi celui
    /// des affichages.
    pub const TOUTES: [PierreDEvolution; 2] = [PierreDEvolution::Braise, PierreDEvolution::Lune];

    pub fn nom(self) -> &'static str {
        match self {
            PierreDEvolution::Braise => "Pierre de braise",
            PierreDEvolution::Lune => "Pierre de lune",
        }
    }
}
