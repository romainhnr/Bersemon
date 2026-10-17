//! Sauvegarde dans un fichier JSON (section 13.9).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use bersemon_core::catalogue::Catalogue;
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::{ErreurSauvegarde, SauvegardeRepository};

use crate::format::{self, SauvegardeV1, VERSION};

pub struct FichierJson {
    chemin: PathBuf,
}

impl FichierJson {
    pub fn new(chemin: impl Into<PathBuf>) -> Self {
        Self {
            chemin: chemin.into(),
        }
    }

    pub fn chemin(&self) -> &Path {
        &self.chemin
    }

    fn lire(&self, texte: &str, catalogue: &dyn Catalogue) -> Result<Partie, ErreurSauvegarde> {
        // La version est lue avant le reste : une sauvegarde d'un autre
        // format est ainsi signalée comme telle, et non comme illisible.
        let valeur: serde_json::Value = serde_json::from_str(texte)
            .map_err(|erreur| ErreurSauvegarde::Illisible(erreur.to_string()))?;
        let version = valeur
            .get("version")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| ErreurSauvegarde::Illisible("version absente".to_string()))?;
        if version != VERSION {
            return Err(ErreurSauvegarde::VersionInconnue(version));
        }
        let sauvegarde: SauvegardeV1 = serde_json::from_value(valeur)
            .map_err(|erreur| ErreurSauvegarde::Illisible(erreur.to_string()))?;
        format::vers_partie(sauvegarde, catalogue)
    }
}

impl SauvegardeRepository for FichierJson {
    fn charger(&self, catalogue: &dyn Catalogue) -> Result<Option<Partie>, ErreurSauvegarde> {
        match fs::read_to_string(&self.chemin) {
            Ok(texte) => self.lire(&texte, catalogue).map(Some),
            Err(erreur) if erreur.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(erreur) => Err(acces(erreur)),
        }
    }

    /// Écrit d'abord un fichier temporaire, puis le renomme : une
    /// interruption pendant l'écriture laisse l'ancienne sauvegarde intacte.
    fn enregistrer(&self, partie: &Partie) -> Result<(), ErreurSauvegarde> {
        if let Some(dossier) = self.chemin.parent() {
            if !dossier.as_os_str().is_empty() {
                fs::create_dir_all(dossier).map_err(acces)?;
            }
        }
        let texte = serde_json::to_string_pretty(&format::depuis_partie(partie))
            .map_err(|erreur| ErreurSauvegarde::Illisible(erreur.to_string()))?;
        let temporaire = self.chemin.with_extension("json.tmp");
        fs::write(&temporaire, texte).map_err(acces)?;
        fs::rename(&temporaire, &self.chemin).map_err(acces)
    }
}

fn acces(erreur: io::Error) -> ErreurSauvegarde {
    ErreurSauvegarde::Acces(erreur.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::attaque::{Attaque, AttaqueId, Categorie, EffetSecondaire};
    use bersemon_core::espece::{AttaqueApprise, Espece, EspeceId, Role};
    use bersemon_core::heros::{Classe, Heros};
    use bersemon_core::monstre::Monstre;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::stats::{Stats, ValeursInternes};
    use bersemon_core::statut::Statut;
    use bersemon_core::types::{Type, TypesMonstre};
    use std::sync::atomic::{AtomicU32, Ordering};

    struct CatalogueDeTest(Vec<Espece>);

    impl Catalogue for CatalogueDeTest {
        fn espece(&self, id: &EspeceId) -> Option<Espece> {
            self.0.iter().find(|espece| espece.id() == id).cloned()
        }
    }

    fn espece(id: &str) -> Espece {
        let attaque = |id: &str, niveau: u8| AttaqueApprise {
            niveau: Niveau::new(niveau).unwrap(),
            attaque: Attaque::new(
                AttaqueId::new(id),
                id,
                Type::Normal,
                Categorie::Physique,
                40,
                100,
            )
            .unwrap(),
        };
        let mut apprentissage = vec![attaque("coup", 1), attaque("griffe", 3)];
        apprentissage[1].attaque = apprentissage[1]
            .attaque
            .clone()
            .avec_effet(EffetSecondaire::new(Statut::Poison, 30).unwrap());
        Espece::new(
            EspeceId::new(id),
            id,
            TypesMonstre::new(&[Type::Normal]).unwrap(),
            Role::Offensif,
            Stats {
                pv: 50,
                attaque: 50,
                defense: 50,
                attaque_speciale: 50,
                defense_speciale: 50,
                vitesse: 50,
            },
            64,
            apprentissage,
        )
    }

    fn catalogue() -> CatalogueDeTest {
        CatalogueDeTest(vec![espece("salamandre"), espece("loup")])
    }

    fn creer(id: &str, niveau: u8) -> Monstre {
        let vi = ValeursInternes::new(Stats {
            pv: 1,
            attaque: 2,
            defense: 3,
            attaque_speciale: 4,
            defense_speciale: 5,
            vitesse: 31,
        })
        .unwrap();
        Monstre::creer(espece(id), Niveau::new(niveau).unwrap(), vi, false).unwrap()
    }

    /// Une partie qui touche à tout ce que la sauvegarde doit garder.
    fn partie_complete() -> Partie {
        let heros = Heros::new(Nom::new("Éléonore").unwrap(), Classe::Mage);
        let mut partie = Partie::nouvelle(heros, creer("salamandre", 5));
        let mut loup = creer("loup", 4);
        loup.renommer(Some(Nom::new("Croc").unwrap()));
        loup.gagner_experience(30);
        loup.subir_degats(5);
        loup.infliger_statut(Statut::Paralysie);
        partie.prendre_monstre_rare(loup);
        for _ in 0..3 {
            partie.prendre_monstre_rare(creer("loup", 3));
        }
        partie.ouvrir_coffre(2);
        partie
    }

    /// Un fichier par test, dans le dossier temporaire du système.
    fn chemin_de_test(nom: &str) -> PathBuf {
        static COMPTEUR: AtomicU32 = AtomicU32::new(0);
        let numero = COMPTEUR.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir()
            .join(format!("bersemon-test-{}-{numero}", std::process::id()))
            .join(format!("{nom}.json"))
    }

    fn nettoyer(chemin: &Path) {
        if let Some(dossier) = chemin.parent() {
            let _ = fs::remove_dir_all(dossier);
        }
    }

    #[test]
    fn aller_retour_sans_perte() {
        let chemin = chemin_de_test("aller-retour");
        let fichier = FichierJson::new(&chemin);
        let partie = partie_complete();
        fichier.enregistrer(&partie).unwrap();
        let rechargee = fichier.charger(&catalogue());
        nettoyer(&chemin);
        assert_eq!(rechargee, Ok(Some(partie)));
    }

    #[test]
    fn aucune_sauvegarde() {
        let chemin = chemin_de_test("absente");
        assert_eq!(FichierJson::new(&chemin).charger(&catalogue()), Ok(None));
    }

    #[test]
    fn une_nouvelle_sauvegarde_remplace_l_ancienne() {
        let chemin = chemin_de_test("remplacement");
        let fichier = FichierJson::new(&chemin);
        let mut partie = partie_complete();
        fichier.enregistrer(&partie).unwrap();
        partie.ouvrir_coffre(5);
        fichier.enregistrer(&partie).unwrap();
        let rechargee = fichier.charger(&catalogue()).unwrap().unwrap();
        let temporaire_restant = chemin.with_extension("json.tmp").exists();
        nettoyer(&chemin);
        assert_eq!(rechargee.inventaire().potions(), 7);
        assert!(!temporaire_restant);
    }

    fn charger_texte(texte: &str) -> Result<Option<Partie>, ErreurSauvegarde> {
        let chemin = chemin_de_test("texte");
        fs::create_dir_all(chemin.parent().unwrap()).unwrap();
        fs::write(&chemin, texte).unwrap();
        let resultat = FichierJson::new(&chemin).charger(&catalogue());
        nettoyer(&chemin);
        resultat
    }

    fn texte_de_partie() -> String {
        serde_json::to_string(&format::depuis_partie(&partie_complete())).unwrap()
    }

    #[test]
    fn fichier_illisible() {
        assert!(matches!(
            charger_texte("pas du json"),
            Err(ErreurSauvegarde::Illisible(_))
        ));
        assert!(matches!(
            charger_texte("{\"version\": 1}"),
            Err(ErreurSauvegarde::Illisible(_))
        ));
    }

    #[test]
    fn version_inconnue() {
        let texte = texte_de_partie().replacen("\"version\":1", "\"version\":2", 1);
        assert_eq!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::VersionInconnue(2))
        );
    }

    #[test]
    fn espece_disparue_du_jeu() {
        let texte = texte_de_partie().replace("\"loup\"", "\"dahut\"");
        assert_eq!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::EspeceInconnue("dahut".to_string()))
        );
    }

    #[test]
    fn attaque_disparue_de_l_espece() {
        let texte = texte_de_partie().replace("\"griffe\"", "\"tranche\"");
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::AttaqueInconnue { .. })
        ));
    }

    #[test]
    fn donnees_contraires_aux_regles() {
        let texte = texte_de_partie().replacen("\"niveau\":5", "\"niveau\":0", 1);
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }
}
