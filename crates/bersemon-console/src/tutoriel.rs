//! Conseils du tutoriel (section 6), montrés une seule fois par partie.

use bersemon_core::contenu::texte_du_conseil;
use bersemon_core::partie::Partie;
use bersemon_core::tutoriel::Conseil;

pub fn montrer_conseil(partie: &mut Partie, conseil: Conseil) {
    if partie.noter_conseil_vu(conseil) {
        println!();
        println!("{}", texte_du_conseil(conseil));
    }
}
