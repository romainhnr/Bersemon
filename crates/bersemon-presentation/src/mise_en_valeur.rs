//! Mise en valeur des informations importantes du combat (section 2) : coups
//! critiques, statuts, efficacité, KO et captures. Ces passages sont entourés
//! de `**`, que Discord affiche en gras ; la console retire ces marques.

const MARQUE: &str = "**";

pub fn important(texte: &str) -> String {
    format!("{MARQUE}{texte}{MARQUE}")
}

pub fn sans_marques(texte: &str) -> String {
    texte.replace(MARQUE, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_console_retire_les_marques() {
        let texte = format!("{} Loup perd 12 PV.", important("Coup critique !"));
        assert_eq!(texte, "**Coup critique !** Loup perd 12 PV.");
        assert_eq!(sans_marques(&texte), "Coup critique ! Loup perd 12 PV.");
    }
}
