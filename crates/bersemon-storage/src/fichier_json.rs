//! Sauvegarde dans un fichier JSON (section 13.9).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use bersemon_core::catalogue::Catalogue;
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::{ErreurSauvegarde, SauvegardeRepository};

use crate::format::{self, SauvegardeV7, VERSION};

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
        // Le format en cours sait lire toutes les versions précédentes.
        if !(1..=VERSION).contains(&version) {
            return Err(ErreurSauvegarde::VersionInconnue(version));
        }
        let sauvegarde: SauvegardeV7 = serde_json::from_value(valeur)
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
    use bersemon_core::capsule::Capsule;
    use bersemon_core::collection::Emplacement;
    use bersemon_core::espece::{AttaqueApprise, Espece, EspeceId, Role};
    use bersemon_core::expedition::ParametresCoffre;
    use bersemon_core::expedition_automatique::ParametresExpeditionAutomatique;
    use bersemon_core::heros::{Classe, Heros};
    use bersemon_core::horodatage::Horodatage;
    use bersemon_core::inventaire::Inventaire;
    use bersemon_core::monstre::Monstre;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::{AttaqueAChoisir, EtatPartie};
    use bersemon_core::stats::{Stats, ValeursInternes};
    use bersemon_core::statut::Statut;
    use bersemon_core::tutoriel::Conseil;
    use bersemon_core::types::{Type, TypesMonstre};
    use bersemon_core::zone::{Zone, ZoneId};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::Duration;

    struct CatalogueDeTest {
        especes: Vec<Espece>,
        zones: Vec<Zone>,
        capsules: Vec<Capsule>,
    }

    impl Catalogue for CatalogueDeTest {
        fn espece(&self, id: &EspeceId) -> Option<Espece> {
            self.especes
                .iter()
                .find(|espece| espece.id() == id)
                .cloned()
        }

        fn zone(&self, id: &ZoneId) -> Option<Zone> {
            self.zones.iter().find(|zone| zone.id() == id).cloned()
        }

        fn capsule(&self, id: &AttaqueId) -> Option<Capsule> {
            self.capsules
                .iter()
                .find(|capsule| capsule.id() == id)
                .cloned()
        }
    }

    fn capsule(id: &str, type_attaque: Type) -> Capsule {
        let attaque = Attaque::new(
            AttaqueId::new(id),
            id,
            type_attaque,
            Categorie::Physique,
            70,
            100,
        )
        .unwrap();
        Capsule::new(attaque, 150)
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

    fn lisiere() -> Zone {
        let niveau = Niveau::new(3).unwrap();
        let parametres = ParametresExpeditionAutomatique::new(
            Duration::from_secs(3_600),
            15..=60,
            100..=200,
            0..=1,
            15..=30,
        )
        .unwrap();
        Zone::new(
            ZoneId::new("lisiere"),
            "Lisière",
            (niveau, niveau),
            4,
            1,
            vec![espece("loup")],
            vec![],
            parametres,
            ParametresCoffre::new(10..=25, Vec::new()).unwrap(),
        )
        .unwrap()
    }

    /// Les espèces de test sont de type Normal : la capsule « vague » ne
    /// leur convient pas.
    fn catalogue() -> CatalogueDeTest {
        CatalogueDeTest {
            especes: vec![espece("salamandre"), espece("loup")],
            zones: vec![lisiere()],
            capsules: vec![
                capsule("grand-coup", Type::Normal),
                capsule("vague", Type::Eau),
            ],
        }
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
        partie.ouvrir_coffre(2, 0, None);
        partie.noter_conseil_vu(Conseil::PremierArretEntreDeuxEtapes);
        partie.noter_conseil_vu(Conseil::PremierCombat);
        // La traque n'avance qu'en expédition : elle est posée directement.
        let etat = |partie: &Partie, attaques_a_choisir| EtatPartie {
            heros: partie.heros().clone(),
            collection: partie.collection().clone(),
            sac: partie.sac().clone(),
            coffre_du_camp: Inventaire::new(135, 0, vec![capsule("grand-coup", Type::Normal)]),
            marchand_au_camp: true,
            codex: partie.codex().clone(),
            zones_explorees: vec![ZoneId::new("lisiere")],
            indices_trouves: 3,
            apotres_vaincus: vec![EspeceId::new("loup")],
            conseils_vus: partie.conseils_vus().to_vec(),
            expeditions_automatiques: partie.expeditions_automatiques().to_vec(),
            attaques_a_choisir,
        };
        let mut partie = Partie::restaurer(etat(&partie, Vec::new())).unwrap();
        // Croc apprend l'attaque de la capsule, hors de la liste de son
        // espèce.
        partie
            .apprendre_attaque(Emplacement::Equipe(1), &AttaqueId::new("grand-coup"), None)
            .unwrap();
        // Le premier loup du camp part en expédition automatique ; l'autre
        // attend de choisir une attaque, comme au retour d'une expédition.
        partie
            .envoyer_en_expedition_automatique(
                &lisiere(),
                &[Emplacement::Camp(0)],
                Horodatage::depuis_secondes(1_000),
            )
            .unwrap();
        let griffe = espece("loup")
            .attaque_apprise(&AttaqueId::new("griffe"))
            .cloned()
            .unwrap();
        let a_choisir = vec![AttaqueAChoisir {
            position_au_camp: 0,
            attaque: griffe,
        }];
        Partie::restaurer(etat(&partie, a_choisir)).unwrap()
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
    fn la_classe_templier_se_relit() {
        let chemin = chemin_de_test("templier");
        let fichier = FichierJson::new(&chemin);
        let heros = Heros::new(Nom::new("Ysolde").unwrap(), Classe::Templier);
        let partie = Partie::nouvelle(heros, creer("salamandre", 5));
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
        partie.ouvrir_coffre(5, 0, None);
        fichier.enregistrer(&partie).unwrap();
        let rechargee = fichier.charger(&catalogue()).unwrap().unwrap();
        let temporaire_restant = chemin.with_extension("json.tmp").exists();
        nettoyer(&chemin);
        assert_eq!(rechargee.sac().potions(), 7);
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

    /// La partie complète écrite comme avant la version 7 : tout ce que
    /// possède le joueur, sans sac ni coffre.
    fn en_version(version: u64) -> serde_json::Value {
        let partie = partie_complete();
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), version.into());
        objet.remove("sac");
        objet.remove("coffre_du_camp");
        let potions = partie.sac().potions() + partie.coffre_du_camp().potions();
        objet.insert("potions".to_string(), potions.into());
        objet.insert("argent".to_string(), partie.argent().into());
        let capsules: Vec<String> = partie
            .capsules()
            .map(|capsule| capsule.id().as_str().to_string())
            .collect();
        objet.insert("capsules".to_string(), serde_json::json!(capsules));
        valeur
    }

    #[test]
    fn une_sauvegarde_en_version_6_range_tout_au_coffre() {
        let partie = partie_complete();
        let rechargee = charger_texte(&en_version(6).to_string()).unwrap().unwrap();
        assert!(rechargee.sac().est_vide());
        let coffre = rechargee.coffre_du_camp();
        assert_eq!(
            coffre.potions(),
            partie.sac().potions() + partie.coffre_du_camp().potions()
        );
        assert_eq!(coffre.argent(), 135);
        assert!(coffre.contient_capsule(&AttaqueId::new("grand-coup")));
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
        let texte = texte_de_partie().replacen("\"version\":7", "\"version\":8", 1);
        assert_eq!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::VersionInconnue(8))
        );
    }

    #[test]
    fn une_sauvegarde_en_version_5_n_a_pas_de_capsule() {
        let partie = partie_complete();
        assert_eq!(partie.capsules().count(), 1);
        let mut valeur = en_version(5);
        valeur.as_object_mut().unwrap().remove("capsules");
        let rechargee = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(rechargee.capsules().count(), 0);
        // L'attaque déjà apprise reste : la capsule existe toujours dans le
        // jeu.
        assert_eq!(rechargee.collection(), partie.collection());
    }

    #[test]
    fn capsule_disparue_du_jeu() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        valeur["coffre_du_camp"]["capsules"] = serde_json::json!(["eclair"]);
        assert_eq!(
            charger_texte(&valeur.to_string()),
            Err(ErreurSauvegarde::CapsuleInconnue("eclair".to_string()))
        );
    }

    #[test]
    fn attaque_d_une_capsule_d_un_autre_type_refusee() {
        let texte = texte_de_partie().replace("\"grand-coup\"", "\"vague\"");
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::AttaqueInconnue { .. })
        ));
    }

    #[test]
    fn une_sauvegarde_en_version_4_n_a_pas_de_marchand() {
        let partie = partie_complete();
        assert!(partie.marchand_au_camp());
        let mut valeur = en_version(4);
        let objet = valeur.as_object_mut().unwrap();
        objet.remove("marchand_au_camp");
        objet.remove("capsules");
        let rechargee = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert!(!rechargee.marchand_au_camp());
        assert_eq!(rechargee.argent(), partie.argent());
    }

    #[test]
    fn une_sauvegarde_en_version_3_n_a_pas_d_argent() {
        let partie = partie_complete();
        assert_eq!(partie.argent(), 135);
        let mut valeur = en_version(3);
        let objet = valeur.as_object_mut().unwrap();
        objet.remove("argent");
        objet.remove("marchand_au_camp");
        objet.remove("capsules");
        let rechargee = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(rechargee.argent(), 0);
        assert_eq!(rechargee.collection(), partie.collection());
        assert_eq!(
            rechargee.expeditions_automatiques(),
            partie.expeditions_automatiques()
        );
    }

    #[test]
    fn une_sauvegarde_en_version_2_n_a_pas_d_expedition_automatique() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 2.into());
        for champ in ["expeditions_automatiques", "attaques_a_choisir"] {
            objet.remove(champ);
        }
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert!(partie.expeditions_automatiques().is_empty());
        assert!(partie.attaques_a_choisir().is_empty());
        assert_eq!(partie.collection().places_reservees(), 0);
        assert_eq!(partie.indices_trouves(), 3);
    }

    #[test]
    fn expedition_dans_une_zone_disparue_du_jeu() {
        let texte = texte_de_partie().replace("\"zone\":\"lisiere\"", "\"zone\":\"atlantide\"");
        assert_eq!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::ZoneInconnue("atlantide".to_string()))
        );
    }

    #[test]
    fn attaque_a_choisir_sans_monstre() {
        let texte = texte_de_partie().replace("\"position_au_camp\":0", "\"position_au_camp\":7");
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    #[test]
    fn une_sauvegarde_en_version_1_reprend_au_debut_de_la_traque() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 1.into());
        for champ in ["indices_trouves", "apotres_vaincus", "conseils_vus"] {
            objet.remove(champ);
        }
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(partie.indices_trouves(), 0);
        assert!(partie.apotres_vaincus().is_empty());
        assert!(partie.conseils_vus().is_empty());
        assert_eq!(partie.collection(), partie_complete().collection());
    }

    #[test]
    fn conseil_inconnu() {
        let texte = texte_de_partie().replace("\"premier-combat\"", "\"dernier-combat\"");
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
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
