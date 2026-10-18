//! Lecture des saisies du joueur. Chaque lecture rend `None` quand l'entrée
//! est fermée, ce qui arrête proprement la partie.

use std::io::{self, BufRead, Write};

pub struct Entree {
    lignes: io::Lines<io::StdinLock<'static>>,
}

impl Entree {
    pub fn depuis_le_clavier() -> Self {
        Self {
            lignes: io::stdin().lock().lines(),
        }
    }

    /// Redemande tant que la saisie n'est pas un nombre de `min` à `max`.
    pub fn choix(&mut self, min: usize, max: usize) -> Option<usize> {
        loop {
            let ligne = self.lire(&format!("Votre choix ({min} à {max}) : "))?;
            match ligne.trim().parse::<usize>() {
                Ok(choix) if (min..=max).contains(&choix) => return Some(choix),
                _ => println!("Choix invalide : entrez un numéro de {min} à {max}."),
            }
        }
    }

    pub fn texte(&mut self, invite: &str) -> Option<String> {
        self.lire(invite)
    }

    /// Redemande tant que la réponse n'est ni oui ni non.
    pub fn confirmation(&mut self, question: &str) -> Option<bool> {
        loop {
            let ligne = self.lire(&format!("{question} (o/n) : "))?;
            match ligne.trim().to_lowercase().as_str() {
                "o" | "oui" => return Some(true),
                "n" | "non" => return Some(false),
                _ => println!("Répondez par o ou n."),
            }
        }
    }

    fn lire(&mut self, invite: &str) -> Option<String> {
        print!("{invite}");
        // Sans vidage explicite, l'invite resterait en mémoire tampon jusqu'au
        // prochain saut de ligne. Un échec ici n'empêche pas de jouer.
        let _ = io::stdout().flush();
        self.lignes.next()?.ok()
    }
}
