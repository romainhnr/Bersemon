//! Le journal des monstres par zone et les zones d'apparition du codex
//! (section 5.6) : chaque lieu où vivent des monstres, avec une ligne par
//! espèce qu'on y trouve, "???" tant qu'elle n'est pas vue. Une espèce vue ou
//! capturée ailleurs l'est aussi ici : le journal suit le codex.

use bersemon_core::codex::{Codex, EtatCodex};
use bersemon_core::contenu::ContenuV1;
use bersemon_core::espece::Espece;
use bersemon_core::partie::Partie;

/// Ce que le journal sait du lieu : son nom, s'il est découvert, et ses
/// espèces, communes puis rares.
pub struct LieuDuJournal<'c> {
    pub nom: String,
    pub decouvert: bool,
    pub especes: Vec<&'c Espece>,
}

/// Le lieu de la pêche, au camp, avec Ansgar (section 13.8).
const PECHE: &str = "Pêche d'Ansgar";

/// Les zones de la carte, les annexes, les zones légendaires, puis la
/// pêche, dans l'ordre du contenu. Le Schnoz, caché partout, n'y est pas
/// (section 9.2) ; ni les Apôtres, au carnet de traque.
pub fn lieux_du_journal<'c>(partie: &Partie, contenu: &'c ContenuV1) -> Vec<LieuDuJournal<'c>> {
    let accessibles: Vec<&str> = partie
        .zones_accessibles(contenu.zones(), contenu.piste())
        .iter()
        .map(|zone| zone.id().as_str())
        .collect();
    let annexes: Vec<&str> = partie
        .annexes_accessibles(contenu.annexes())
        .iter()
        .map(|annexe| annexe.zone().id().as_str())
        .collect();
    let especes_de = |zone: &'c bersemon_core::zone::Zone| -> Vec<&'c Espece> {
        let mut especes: Vec<&'c Espece> = Vec::new();
        for espece in zone
            .especes()
            .iter()
            .chain(zone.rares().iter().map(|rare| &rare.espece))
        {
            if !especes.iter().any(|deja| deja.id() == espece.id()) {
                especes.push(espece);
            }
        }
        especes
    };
    let mut lieux: Vec<LieuDuJournal<'c>> = contenu
        .zones()
        .iter()
        .map(|zone| LieuDuJournal {
            nom: zone.nom().to_string(),
            decouvert: accessibles.contains(&zone.id().as_str()),
            especes: especes_de(zone),
        })
        .collect();
    lieux.extend(contenu.annexes().iter().map(|annexe| LieuDuJournal {
        nom: annexe.zone().nom().to_string(),
        decouvert: annexes.contains(&annexe.zone().id().as_str()),
        especes: especes_de(annexe.zone()),
    }));
    lieux.extend(contenu.zones_legendaires().iter().map(|legendaire| {
        let mut especes = especes_de(legendaire.zone());
        especes.push(legendaire.legendaire().espece());
        LieuDuJournal {
            nom: legendaire.zone().nom().to_string(),
            decouvert: partie.zones_legendaires_ouvertes(),
            especes,
        }
    }));
    let peche = contenu.peche();
    lieux.push(LieuDuJournal {
        nom: PECHE.to_string(),
        decouvert: partie.peche().pecheur_au_camp(),
        especes: peche
            .especes()
            .iter()
            .chain(std::iter::once(peche.rare()))
            .collect(),
    });
    lieux
}

/// "Lisière du Bois-Sombre : 2 capturées, 1 vue, sur 5."
pub fn resume_du_lieu(lieu: &LieuDuJournal<'_>, codex: &Codex) -> String {
    let compter = |etat: EtatCodex| {
        lieu.especes
            .iter()
            .filter(|espece| codex.etat(espece.id()) == etat)
            .count()
    };
    format!(
        "{} : {} capturées, {} vues, sur {}.",
        lieu.nom,
        compter(EtatCodex::Capturee),
        compter(EtatCodex::Rencontree),
        lieu.especes.len()
    )
}

/// Une ligne par espèce du lieu : "???" tant qu'elle n'est pas vue, puis
/// "Loup : vu" ou "Loup : capturé".
pub fn lignes_du_lieu(lieu: &LieuDuJournal<'_>, codex: &Codex) -> Vec<String> {
    lieu.especes
        .iter()
        .map(|espece| match codex.etat(espece.id()) {
            EtatCodex::Inconnue => "???".to_string(),
            EtatCodex::Rencontree => format!("{} : vu", espece.nom()),
            EtatCodex::Capturee => format!("{} : capturé", espece.nom()),
        })
        .collect()
}

/// Le journal des lieux découverts, pour la console : le résumé de chaque
/// lieu, puis ses espèces.
pub fn journal(partie: &Partie, contenu: &ContenuV1) -> String {
    let codex = partie.codex();
    let mut lignes = vec!["Journal des monstres par zone :".to_string()];
    for lieu in lieux_du_journal(partie, contenu)
        .iter()
        .filter(|lieu| lieu.decouvert)
    {
        lignes.push(resume_du_lieu(lieu, codex));
        lignes.extend(
            lignes_du_lieu(lieu, codex)
                .into_iter()
                .map(|ligne| format!("  {ligne}")),
        );
    }
    lignes.join("\n")
}

/// Sous la fiche du codex : "Zones : Lisière du Bois-Sombre, ???", les
/// lieux pas encore découverts restant "???" ; `None` pour une espèce qui
/// ne vit dans aucun lieu (formes évoluées, espèces de la Fosse).
pub fn zones_d_apparition(espece: &Espece, partie: &Partie, contenu: &ContenuV1) -> Option<String> {
    let zones: Vec<String> = lieux_du_journal(partie, contenu)
        .iter()
        .filter(|lieu| lieu.especes.iter().any(|une| une.id() == espece.id()))
        .map(|lieu| {
            if lieu.decouvert {
                lieu.nom.clone()
            } else {
                "???".to_string()
            }
        })
        .collect();
    (!zones.is_empty()).then(|| format!("Zones : {}", zones.join(", ")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::espece::EspeceId;
    use bersemon_core::heros::{Classe, Heros};
    use bersemon_core::monstre::Monstre;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::stats::ValeursInternes;

    fn partie(contenu: &ContenuV1) -> Partie {
        let depart = Monstre::creer(
            contenu.espece(&EspeceId::new("crapaud")).unwrap(),
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap();
        Partie::nouvelle(
            Heros::new(Nom::new("Aldric").unwrap(), Classe::Guerrier),
            depart,
        )
    }

    /// Section 5.6 : au départ, la Lisière seule est découverte, toutes ses
    /// espèces à "???" ; une espèce vue ailleurs y est vue.
    #[test]
    fn le_journal_suit_le_codex() {
        let contenu = ContenuV1::new();
        let mut partie = partie(&contenu);
        let lieux = lieux_du_journal(&partie, &contenu);
        let lisiere = &lieux[0];
        assert!(lisiere.decouvert && !lieux[1].decouvert);
        assert!(
            lignes_du_lieu(lisiere, partie.codex())
                .iter()
                .all(|ligne| ligne == "???")
        );
        let loup = contenu.espece(&EspeceId::new("loup")).unwrap();
        partie.noter_rencontre(
            &Monstre::creer(
                loup.clone(),
                Niveau::new(3).unwrap(),
                ValeursInternes::default(),
                false,
            )
            .unwrap(),
        );
        let lieux = lieux_du_journal(&partie, &contenu);
        assert!(lignes_du_lieu(&lieux[0], partie.codex()).contains(&"Loup : vu".to_string()));
        let texte = journal(&partie, &contenu);
        assert!(
            texte.contains("Lisière du Bois-Sombre : 0 capturées, 1 vues, sur"),
            "{texte}"
        );
        assert!(!texte.contains("Marais"), "{texte}");
        // Le Loup vit dans la Lisière, découverte ; la Harpie des orages dans
        // le Col et le Ciel déchiré, pas encore découverts.
        assert_eq!(
            zones_d_apparition(&loup, &partie, &contenu).as_deref(),
            Some("Zones : Lisière du Bois-Sombre")
        );
        let harpie = contenu.espece(&EspeceId::new("harpie-des-orages")).unwrap();
        assert_eq!(
            zones_d_apparition(&harpie, &partie, &contenu).as_deref(),
            Some("Zones : ???, ???")
        );
    }
}
