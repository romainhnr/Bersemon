//! Outil d'équilibrage de Bersemon : un joueur automatique mène des parties
//! complètes avec le noyau, sans interface ni sauvegarde, de la création du
//! héros à la chute du troisième Apôtre, et le programme écrit leurs
//! statistiques en Markdown. Il sert à régler les valeurs du jeu et ne fait
//! pas partie de ce que joue le joueur.
//!
//! Usage : `cargo run --release -p bersemon-equilibrage -- [parties par classe] [classe]`.
//! Sans classe, les quatre sont jouées. Les parties d'une même graine se
//! rejouent à l'identique.

mod joueur;
mod rapport;

use bersemon_core::contenu::ContenuV1;
use bersemon_core::heros::Classe;

const PARTIES_PAR_CLASSE: u64 = 25;

fn main() {
    let parties = std::env::args()
        .nth(1)
        .and_then(|nombre| nombre.parse().ok())
        .unwrap_or(PARTIES_PAR_CLASSE);
    let seule = std::env::args().nth(2).map(|nom| {
        Classe::TOUTES
            .into_iter()
            .find(|classe| classe.nom().eq_ignore_ascii_case(&nom))
            .unwrap_or_else(|| panic!("classe inconnue : {nom}"))
    });
    let contenu = ContenuV1::new();
    let mut bilans = Vec::new();
    for classe in Classe::TOUTES {
        if seule.is_some_and(|seule| seule != classe) {
            continue;
        }
        for graine in 0..parties {
            bilans.push(joueur::jouer_une_partie(&contenu, classe, graine));
        }
    }
    print!("{}", rapport::rapport(&contenu, &bilans, parties));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Le joueur automatique doit pouvoir finir une partie : sinon, le
    /// rapport mesurerait sa stratégie plutôt que le jeu.
    #[test]
    fn une_partie_se_joue_jusqu_a_la_chute_du_margrave() {
        let contenu = ContenuV1::new();
        let bilan = joueur::jouer_une_partie(&contenu, Classe::Guerrier, 1);
        assert!(bilan.terminee, "{} expéditions", bilan.expeditions.len());
        let rapport = rapport::rapport(&contenu, &[bilan], 1);
        assert!(
            rapport.contains("| Lisière du Bois-Sombre (3 à 7) |"),
            "{rapport}"
        );
    }
}
