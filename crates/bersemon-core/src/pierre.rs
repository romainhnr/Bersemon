//! Objets d'évolution (section 5.4) : chacun fait évoluer, au camp, les
//! espèces dont l'évolution le demande, et se consomme. Les pierres se
//! trouvent dans les coffres et à la pêche ; la Chaîne de lien, qui
//! remplace l'échange entre joueurs, ne s'achète que chez Malvoise.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PierreDEvolution {
    Braise,
    Lune,
    /// Les pierres qui suivent simulent une condition d'évolution : le
    /// moment, la météo ou le lieu ; elles font évoluer au camp, une fois le
    /// niveau atteint (section 5.4).
    Nuit,
    Soleil,
    Orage,
    Pluie,
    Chaude,
    Brumes,
    /// Elle lie le monstre à son maître comme s'il l'avait reçu d'un autre
    /// joueur : elle fait évoluer les espèces qui évoluent par échange.
    ChaineDeLien,
}

impl PierreDEvolution {
    /// Dans l'ordre de docs/contenu-v1.md, sections 6 et 53, qui est aussi
    /// celui des affichages.
    pub const TOUTES: [PierreDEvolution; 9] = [
        PierreDEvolution::Braise,
        PierreDEvolution::Lune,
        PierreDEvolution::Nuit,
        PierreDEvolution::Soleil,
        PierreDEvolution::Orage,
        PierreDEvolution::Pluie,
        PierreDEvolution::Chaude,
        PierreDEvolution::Brumes,
        PierreDEvolution::ChaineDeLien,
    ];

    /// Celles que contiennent les coffres et que remonte la ligne d'Ansgar.
    pub const TROUVEES: [PierreDEvolution; 2] = [PierreDEvolution::Braise, PierreDEvolution::Lune];

    pub fn nom(self) -> &'static str {
        match self {
            PierreDEvolution::Braise => "Pierre de braise",
            PierreDEvolution::Lune => "Pierre de lune",
            PierreDEvolution::Nuit => "Pierre de nuit",
            PierreDEvolution::Soleil => "Pierre de soleil",
            PierreDEvolution::Orage => "Pierre d'orage",
            PierreDEvolution::Pluie => "Pierre de pluie",
            PierreDEvolution::Chaude => "Pierre chaude",
            PierreDEvolution::Brumes => "Pierre des brumes",
            PierreDEvolution::ChaineDeLien => "Chaîne de lien",
        }
    }
}
