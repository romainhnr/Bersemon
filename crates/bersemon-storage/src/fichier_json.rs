//! Sauvegarde dans un fichier JSON (section 13.9).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use bersemon_core::catalogue::Catalogue;
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::{ErreurSauvegarde, SauvegardeRepository};

use crate::format::{self, SauvegardeV14, VERSION};

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
        let sauvegarde: SauvegardeV14 = serde_json::from_value(valeur)
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

    fn supprimer(&self) -> Result<(), ErreurSauvegarde> {
        match fs::remove_file(&self.chemin) {
            Ok(()) => Ok(()),
            Err(erreur) if erreur.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(erreur) => Err(acces(erreur)),
        }
    }
}

fn acces(erreur: io::Error) -> ErreurSauvegarde {
    ErreurSauvegarde::Acces(erreur.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::ame::{Gardien, MarchandId, MarchandMort};
    use bersemon_core::amelioration::{Amelioration, Ameliorations, Chantier, Forge, Ouvrage};
    use bersemon_core::attaque::{Attaque, AttaqueId, Categorie, EffetSecondaire};
    use bersemon_core::beherit::EtatDuBeherit;
    use bersemon_core::booster::Booster;
    use bersemon_core::capsule::Capsule;
    use bersemon_core::codex::EtatCodex;
    use bersemon_core::collection::Emplacement;
    use bersemon_core::equipement::{
        Equipement, ObjetDEquipement, ObjetDEquipementId, PlaceDEquipement, SorteDObjet,
    };
    use bersemon_core::espece::{AttaqueApprise, Espece, EspeceId, Rarete, Role};
    use bersemon_core::expedition::ParametresCoffre;
    use bersemon_core::expedition_automatique::ParametresExpeditionAutomatique;
    use bersemon_core::filet::Filet;
    use bersemon_core::habilete::Habilete;
    use bersemon_core::heros::{Classe, Heros};
    use bersemon_core::horodatage::Horodatage;
    use bersemon_core::humain::HumainId;
    use bersemon_core::inventaire::Inventaire;
    use bersemon_core::machines::{Entrainement, Machines, Pieges};
    use bersemon_core::monstre::Monstre;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::{AttaqueAChoisir, EtatPartie};
    use bersemon_core::peche::Peche;
    use bersemon_core::pierre::PierreDEvolution;
    use bersemon_core::recompenses_du_codex::PalierDuCodex;
    use bersemon_core::relations::{Palmares, Relations};
    use bersemon_core::ressource::{Ressource, RessourceId};
    use bersemon_core::rite::{PLACES_DU_RITE, Rite};
    use bersemon_core::soin::Soin;
    use bersemon_core::stats::{Stats, ValeursInternes};
    use bersemon_core::statut::Statut;
    use bersemon_core::tour::RecordsDeLaTour;
    use bersemon_core::tutoriel::Conseil;
    use bersemon_core::types::{Type, TypesMonstre};
    use bersemon_core::zone::{Zone, ZoneId};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::Duration;

    struct CatalogueDeTest {
        especes: Vec<Espece>,
        zones: Vec<Zone>,
        capsules: Vec<Capsule>,
        marchands: Vec<MarchandMort>,
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

        fn marchand_mort(&self, id: &MarchandId) -> Option<MarchandMort> {
            self.marchands
                .iter()
                .find(|marchand| marchand.id() == id)
                .cloned()
        }

        fn ressource(&self, id: &RessourceId) -> Option<Ressource> {
            Some(croc()).filter(|croc| croc.id() == id)
        }

        fn objet_d_equipement(&self, id: &ObjetDEquipementId) -> Option<ObjetDEquipement> {
            [couteau(), talisman()]
                .into_iter()
                .find(|objet| objet.id() == id)
        }
    }

    fn couteau() -> ObjetDEquipement {
        ObjetDEquipement::new(
            ObjetDEquipementId::new("couteau"),
            "Couteau",
            SorteDObjet::Arme,
            Stats {
                attaque: 5,
                ..Stats::default()
            },
        )
    }

    fn talisman() -> ObjetDEquipement {
        ObjetDEquipement::new(
            ObjetDEquipementId::new("talisman"),
            "Talisman",
            SorteDObjet::Talisman,
            Stats {
                vitesse: 5,
                ..Stats::default()
            },
        )
    }

    fn bertrand() -> MarchandMort {
        MarchandMort::new(
            MarchandId::new("bertrand"),
            "Bertrand",
            Soin::Remede,
            Gardien::MonstreDeLaZone,
            "",
            "",
        )
        .unwrap()
    }

    fn croc() -> Ressource {
        Ressource::new(RessourceId::new("croc"), "Croc", 5)
    }

    #[test]
    fn une_ressource_disparue_du_jeu_est_oubliee() {
        let partie = charger_texte(&texte_de_partie()).unwrap().unwrap();
        assert_eq!(partie.coffre_du_camp().nombre_de_ressources(croc().id()), 4);
        let texte = texte_de_partie().replace("\"croc\"", "\"inconnue\"");
        let partie = charger_texte(&texte).unwrap().unwrap();
        assert!(partie.coffre_du_camp().ressources().is_empty());
    }

    fn grete() -> MarchandMort {
        marchand("grete", "Grete", Soin::GrandePotion)
    }

    fn ulrich() -> MarchandMort {
        marchand("ulrich", "Ulrich", Soin::Elixir)
    }

    fn marchand(id: &str, nom: &str, soin: Soin) -> MarchandMort {
        MarchandMort::new(
            MarchandId::new(id),
            nom,
            soin,
            Gardien::MonstreDeLaZone,
            "",
            "",
        )
        .unwrap()
    }

    #[test]
    fn les_ames_portees_et_a_retrouver_se_relisent() {
        let partie = charger_texte(&texte_de_partie()).unwrap().unwrap();
        assert_eq!(partie.ames_portees()[0].id().as_str(), "grete");
        assert_eq!(partie.ames_a_retrouver(), &[MarchandId::new("ulrich")]);
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
        .avec_habiletes(vec![Habilete::Pillard, Habilete::Fouineur])
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
            especes: vec![
                espece("salamandre"),
                espece("loup"),
                espece("baron").avec_rarete(Rarete::Apotre),
            ],
            zones: vec![lisiere()],
            capsules: vec![
                capsule("grand-coup", Type::Normal),
                capsule("vague", Type::Eau),
            ],
            marchands: vec![bertrand(), grete(), ulrich()],
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
        let equipement = Equipement::restaurer(
            vec![couteau(), talisman()],
            &[(PlaceDEquipement::SecondTalisman, talisman().id().clone())],
        )
        .unwrap();
        let heros =
            Heros::new(Nom::new("Éléonore").unwrap(), Classe::Mage).avec_equipement(equipement);
        let mut partie = Partie::nouvelle(heros, creer("salamandre", 5));
        let mut loup = creer("loup", 4).avec_habilete(Habilete::Fouineur);
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
            coffre_du_camp: Inventaire::new(135, 0, vec![capsule("grand-coup", Type::Normal)])
                .avec_soins(Soin::Remede, 2)
                .avec_soins(Soin::GrandePotion, 1)
                .avec_soins(Soin::Elixir, 3)
                .avec_capsules_d_habilete(2)
                .avec_ressources(croc(), 4)
                .avec_pierres(PierreDEvolution::Lune, 2)
                .avec_pierres(PierreDEvolution::Braise, 1)
                .avec_boosters(Booster::CendreBenite, 2)
                .avec_boosters(Booster::FioleDeSang, 1)
                .avec_philtres_chromatiques(1)
                .avec_charmes_de_garde(2)
                .avec_filets(Filet::Chaines, 3),
            marchand_au_camp: true,
            ames_donnees: vec![bertrand()],
            ames_portees: vec![grete()],
            ames_a_retrouver: vec![MarchandId::new("ulrich")],
            codex: partie.codex().clone(),
            zones_explorees: vec![ZoneId::new("lisiere")],
            indices_trouves: 3,
            apotres_vaincus: vec![EspeceId::new("loup")],
            humains_vaincus: vec![HumainId::new("brigand")],
            conseils_vus: partie.conseils_vus().to_vec(),
            expeditions_automatiques: partie.expeditions_automatiques().to_vec(),
            attaques_a_choisir,
            peche: Peche::restaurer(
                true,
                Some(Horodatage::depuis_secondes(8_000)),
                Some(creer("loup", 6)),
            )
            .unwrap(),
            forge: Forge::restaurer(
                true,
                Ameliorations::new(1, 2, 1)
                    .unwrap()
                    .avec_machines(2, 1)
                    .unwrap()
                    .avec_sac(2)
                    .unwrap(),
                Some(Chantier {
                    ouvrage: Ouvrage::Amelioration {
                        amelioration: Amelioration::Enclos,
                        niveau: 2,
                    },
                    fin: Horodatage::depuis_secondes(9_000),
                }),
            )
            .unwrap(),
            relations: Relations::restaurer(
                vec![1_234_567_890_123, 42],
                Palmares {
                    victoires: 3,
                    defaites: 1,
                },
            ),
            machines: Machines::restaurer(
                Some(Pieges::new(
                    ZoneId::new("lisiere"),
                    Horodatage::depuis_secondes(500),
                )),
                vec![Entrainement::new(
                    creer("loup", 4),
                    Horodatage::depuis_secondes(600),
                )],
            ),
            beherit: EtatDuBeherit::EclipseOuverte,
            records_de_la_tour: RecordsDeLaTour {
                seul: 23,
                a_deux: 31,
            },
            rite: Some(Rite::new(
                [creer("loup", 5), creer("loup", 9)],
                Horodatage::depuis_secondes(7_200),
            )),
            paliers_du_codex: vec![PalierDuCodex::Especes(10), PalierDuCodex::Complet],
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
            emplacement: Emplacement::Camp(0),
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
    fn chaque_classe_se_relit() {
        for classe in Classe::TOUTES {
            let chemin = chemin_de_test(classe.nom());
            let fichier = FichierJson::new(&chemin);
            let heros = Heros::new(Nom::new("Ysolde").unwrap(), classe);
            let partie = Partie::nouvelle(heros, creer("salamandre", 5));
            fichier.enregistrer(&partie).unwrap();
            let rechargee = fichier.charger(&catalogue());
            nettoyer(&chemin);
            assert_eq!(rechargee, Ok(Some(partie)));
        }
    }

    #[test]
    fn aucune_sauvegarde() {
        let chemin = chemin_de_test("absente");
        assert_eq!(FichierJson::new(&chemin).charger(&catalogue()), Ok(None));
    }

    #[test]
    fn une_sauvegarde_supprimee_n_existe_plus() {
        let chemin = chemin_de_test("suppression");
        let fichier = FichierJson::new(&chemin);
        fichier.enregistrer(&partie_complete()).unwrap();
        let supprimee = fichier.supprimer();
        let rechargee = fichier.charger(&catalogue());
        // Supprimer une sauvegarde absente ne fait rien.
        let encore = fichier.supprimer();
        nettoyer(&chemin);
        assert_eq!(supprimee, Ok(()));
        assert_eq!(rechargee, Ok(None));
        assert_eq!(encore, Ok(()));
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

    /// Avant la version 14, l'enclos n'est pas agrandi : seuls les monstres
    /// se comparent.
    fn assert_memes_monstres(
        relue: &bersemon_core::collection::Collection,
        attendue: &bersemon_core::collection::Collection,
    ) {
        assert_eq!(relue.equipe(), attendue.equipe());
        assert_eq!(relue.camp(), attendue.camp());
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
    fn une_sauvegarde_en_version_7_donne_la_premiere_habilete_et_31_aux_shiny() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 7.into());
        let equipe = objet["equipe"].as_array_mut().unwrap();
        for monstre in equipe.iter_mut() {
            monstre.as_object_mut().unwrap().remove("habilete");
        }
        equipe[0]["shiny"] = true.into();
        let rechargee = charger_texte(&valeur.to_string()).unwrap().unwrap();
        let equipe = rechargee.collection().equipe();
        // Croc était Fouineur : sans habileté écrite, il reçoit la première
        // de son espèce.
        assert_eq!(equipe[1].nom(), "Croc");
        assert_eq!(equipe[1].habilete(), Some(Habilete::Pillard));
        assert!(equipe[0].est_shiny());
        assert_eq!(equipe[0].valeurs_internes(), &ValeursInternes::maximales());
    }

    #[test]
    fn une_sauvegarde_de_l_ancienne_piste_reprend_au_premier_indice() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        assert_eq!(objet["indices_trouves"], 3);
        objet.insert("version".to_string(), 9.into());
        let rechargee = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(rechargee.indices_trouves(), 1);
        // L'Apôtre vaincu le reste.
        assert_eq!(rechargee.apotres_vaincus(), &[EspeceId::new("loup")]);
        // Depuis la version 10, les indices se relisent tels quels.
        let rechargee = charger_texte(&texte_de_partie()).unwrap().unwrap();
        assert_eq!(rechargee.indices_trouves(), 3);
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
    fn les_philtres_chromatiques_gardent_dans_le_fichier_le_nom_des_eclats_d_astre() {
        let valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        assert_eq!(valeur["coffre_du_camp"]["eclats_d_astre"], 1);
        assert!(
            valeur["coffre_du_camp"]
                .get("philtres_chromatiques")
                .is_none()
        );
        let rechargee = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(rechargee.coffre_du_camp().philtres_chromatiques(), 1);
    }

    #[test]
    fn version_inconnue() {
        let texte = texte_de_partie().replacen("\"version\":30", "\"version\":31", 1);
        assert_eq!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::VersionInconnue(31))
        );
    }

    #[test]
    fn un_lot_de_filets_en_fabrication_se_relit() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        valeur["chantier"] = serde_json::json!({
            "filets": {"filet": "filet-de-chaines", "nombre": 5},
            "fin": 9_000
        });
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(
            partie.forge().chantier().map(|chantier| &chantier.ouvrage),
            Some(&Ouvrage::Filets {
                filet: Filet::Chaines,
                nombre: 5
            })
        );
        let relue = charger_texte(&serde_json::to_string(&format::depuis_partie(&partie)).unwrap())
            .unwrap()
            .unwrap();
        assert_eq!(relue, partie);
        // Un lot vide, un filet inconnu ou un lot avec un charme : refusés.
        for chantier in [
            serde_json::json!({"filets": {"filet": "filet-leste", "nombre": 0}, "fin": 1}),
            serde_json::json!({"filets": {"filet": "filet-d-or", "nombre": 5}, "fin": 1}),
            serde_json::json!({
                "filets": {"filet": "filet-leste", "nombre": 5},
                "charme": true,
                "fin": 1
            }),
        ] {
            valeur["chantier"] = chantier;
            assert!(matches!(
                charger_texte(&valeur.to_string()),
                Err(ErreurSauvegarde::Incoherente(_))
            ));
        }
    }

    #[test]
    fn un_charme_en_fabrication_se_relit() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        valeur["chantier"] = serde_json::json!({"charme": true, "fin": 9_000});
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(
            partie.forge().chantier().map(|chantier| &chantier.ouvrage),
            Some(&Ouvrage::CharmeDeGarde)
        );
        let relue = charger_texte(&serde_json::to_string(&format::depuis_partie(&partie)).unwrap())
            .unwrap()
            .unwrap();
        assert_eq!(relue, partie);
        // Un charme et un objet à la fois : refusé.
        valeur["chantier"]["objet"] = "couteau-de-chasse".into();
        assert!(matches!(
            charger_texte(&valeur.to_string()),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    #[test]
    fn la_protection_d_une_expedition_se_relit_ou_se_refuse() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        // L'expédition de la partie n'emmène qu'un monstre : il ne se
        // protège pas.
        valeur["expeditions_automatiques"][0]["protege"] = 0.into();
        assert!(matches!(
            charger_texte(&valeur.to_string()),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        valeur["expeditions_automatiques"][0]["charme"] = true.into();
        assert!(matches!(
            charger_texte(&valeur.to_string()),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    #[test]
    fn une_sauvegarde_en_version_21_n_a_pas_de_machine() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 21.into());
        for champ in ["pieges", "entrainement"] {
            assert!(objet.remove(champ).is_some(), "{champ}");
        }
        let ameliorations = objet["ameliorations"].as_object_mut().unwrap();
        for champ in ["pieges", "cour"] {
            assert!(ameliorations.remove(champ).is_some(), "{champ}");
        }
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(partie.machines(), &Machines::default());
        assert_eq!(
            partie.forge().ameliorations().niveau(Amelioration::Pieges),
            0
        );
        // Le monstre à l'entraînement n'y est plus : sa place se libère, comme
        // celles du rite, plus récent encore.
        assert_eq!(
            partie.collection().places_reservees(),
            partie_complete().collection().places_reservees() - 1 - PLACES_DU_RITE
        );
    }

    #[test]
    fn des_pieges_dans_une_zone_inexploree_sont_refuses() {
        let texte = texte_de_partie().replace(
            "\"pieges\":{\"zone\":\"lisiere\"",
            "\"pieges\":{\"zone\":\"marais\"",
        );
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    #[test]
    fn une_sauvegarde_en_version_20_n_a_ni_ami_ni_duel() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 20.into());
        for champ in ["amis", "victoires_en_duel", "defaites_en_duel"] {
            assert!(objet.remove(champ).is_some(), "{champ}");
        }
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(partie.relations(), &Relations::default());
    }

    #[test]
    fn avant_la_version_20_chaque_apotre_vaincu_laisse_un_philtre_au_coffre() {
        let partie = partie_complete();
        assert_eq!(partie.apotres_vaincus().len(), 1);
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        valeur["version"] = 19.into();
        valeur["coffre_du_camp"]
            .as_object_mut()
            .unwrap()
            .remove("boosters");
        valeur["coffre_du_camp"]["eclats_d_astre"] = 0.into();
        let rechargee = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(rechargee.coffre_du_camp().philtres_chromatiques(), 1);
        assert_eq!(rechargee.coffre_du_camp().boosters(), &[]);
        // Avant la version 27, les filets du sac sont oubliés (section
        // 11.2).
        assert_eq!(rechargee.sac().potions(), partie.sac().potions());
        assert!(rechargee.sac().filets().est_vide());
        // En version 20, rien n'est ajouté.
        let relue = charger_texte(&texte_de_partie()).unwrap().unwrap();
        assert_eq!(relue.coffre_du_camp().philtres_chromatiques(), 1);
    }

    #[test]
    fn booster_inconnu_refuse() {
        let texte = texte_de_partie().replace("\"cendre-benite\"", "\"sel-de-lune\"");
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    /// La partie complète en version 18, avec un Baron capturé en tête du
    /// camp, à la fin de l'équipe et dans l'expédition automatique.
    fn avec_un_baron_capture() -> serde_json::Value {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 18.into());
        objet.insert("apotres_vaincus".to_string(), serde_json::json!(["baron"]));
        let mut baron = objet["camp"][0].clone();
        baron["espece"] = "baron".into();
        objet["camp"]
            .as_array_mut()
            .unwrap()
            .insert(0, baron.clone());
        objet["equipe"].as_array_mut().unwrap().push(baron.clone());
        objet["expeditions_automatiques"][0]["monstres"]
            .as_array_mut()
            .unwrap()
            .push(baron);
        // L'attaque à choisir suit son Loup, décalé par le Baron.
        objet["attaques_a_choisir"][0]["position"] = 1.into();
        valeur
    }

    #[test]
    fn en_version_18_un_apotre_capture_devient_son_ame() {
        let partie = partie_complete();
        let rechargee = charger_texte(&avec_un_baron_capture().to_string())
            .unwrap()
            .unwrap();
        let baron = EspeceId::new("baron");
        let collection = rechargee.collection();
        let partis = rechargee
            .expeditions_automatiques()
            .iter()
            .flat_map(|expedition| expedition.monstres());
        assert!(
            collection
                .equipe()
                .iter()
                .chain(collection.camp())
                .chain(partis)
                .all(|monstre| monstre.espece().id() != &baron)
        );
        assert_memes_monstres(collection, partie.collection());
        assert_eq!(
            rechargee.expeditions_automatiques(),
            partie.expeditions_automatiques()
        );
        assert_eq!(rechargee.attaques_a_choisir(), partie.attaques_a_choisir());
        assert_eq!(rechargee.ames_d_apotres(), std::slice::from_ref(&baron));
        assert_eq!(rechargee.codex().etat(&baron), EtatCodex::Capturee);
        // En version 19, rien n'est converti.
        let mut valeur = avec_un_baron_capture();
        valeur["version"] = 19.into();
        valeur["attaques_a_choisir"][0]["position"] = 1.into();
        let gardee = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(gardee.collection().camp()[0].espece().id(), &baron);
    }

    #[test]
    fn un_apotre_seul_monstre_du_joueur_reste() {
        let mut valeur = avec_un_baron_capture();
        let objet = valeur.as_object_mut().unwrap();
        let baron = objet["equipe"].as_array().unwrap().last().unwrap().clone();
        objet.insert("equipe".to_string(), serde_json::json!([baron]));
        objet.insert("camp".to_string(), serde_json::json!([]));
        objet.insert(
            "expeditions_automatiques".to_string(),
            serde_json::json!([]),
        );
        objet.insert("attaques_a_choisir".to_string(), serde_json::json!([]));
        objet.remove("prise");
        let rechargee = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(rechargee.collection().equipe().len(), 1);
    }

    #[test]
    fn une_sauvegarde_en_version_17_n_a_pas_de_peche() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 17.into());
        for champ in ["ansgar_au_camp", "ligne", "prise"] {
            assert!(objet.remove(champ).is_some(), "{champ}");
        }
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(partie.peche(), &Peche::default());
    }

    #[test]
    fn une_peche_sans_ansgar_est_refusee() {
        let texte =
            texte_de_partie().replace("\"ansgar_au_camp\":true", "\"ansgar_au_camp\":false");
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
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
        assert_memes_monstres(rechargee.collection(), partie.collection());
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
    fn l_attaque_d_oeuf_de_sa_famille_se_relit() {
        let assaut = Attaque::new(
            AttaqueId::new("assaut-de-meute"),
            "Assaut de meute",
            Type::Normal,
            Categorie::Physique,
            80,
            100,
        )
        .unwrap();
        let mut avec_oeuf = catalogue();
        for espece in &mut avec_oeuf.especes {
            if espece.id().as_str() == "loup" {
                *espece = espece.clone().avec_attaque_d_oeuf(assaut.clone());
            }
        }
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let equipe = valeur["equipe"].as_array_mut().unwrap();
        let position = equipe
            .iter()
            .position(|monstre| monstre["espece"] == "loup")
            .unwrap();
        let attaques = equipe[position]["attaques"].as_array_mut().unwrap();
        attaques.truncate(3);
        attaques.push("assaut-de-meute".into());
        let texte = valeur.to_string();
        let chemin = chemin_de_test("attaque-d-oeuf");
        fs::create_dir_all(chemin.parent().unwrap()).unwrap();
        fs::write(&chemin, &texte).unwrap();
        let relue = FichierJson::new(&chemin).charger(&avec_oeuf);
        nettoyer(&chemin);
        let relue = relue.unwrap().unwrap();
        assert!(
            relue.collection().equipe()[position]
                .attaques()
                .liste()
                .contains(&assaut)
        );
        // Sans être l'attaque d'œuf de sa famille, elle est refusée.
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::AttaqueInconnue { .. })
        ));
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
        // Sans zone explorée, ni Malvoise, ni expédition automatique.
        objet.insert("zones_explorees".to_string(), serde_json::json!([]));
        objet.insert(
            "expeditions_automatiques".to_string(),
            serde_json::json!([]),
        );
        let rechargee = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert!(!rechargee.marchand_au_camp());
        assert_eq!(rechargee.argent(), partie.argent());
    }

    #[test]
    fn avant_la_version_11_une_zone_exploree_amene_malvoise() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        assert_eq!(objet["ames_recueillies"], serde_json::json!(["bertrand"]));
        objet.insert("marchand_au_camp".to_string(), false.into());
        objet.remove("ames_recueillies");
        let en_version_11 = valeur.to_string();
        valeur
            .as_object_mut()
            .unwrap()
            .insert("version".to_string(), 10.into());
        let rechargee = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert!(rechargee.marchand_au_camp());
        assert!(rechargee.ames_donnees().is_empty());
        // Depuis la version 11, Malvoise n'est au camp que s'il y est arrivé.
        let rechargee = charger_texte(&en_version_11).unwrap().unwrap();
        assert!(!rechargee.marchand_au_camp());
    }

    #[test]
    fn une_ame_dont_le_marchand_a_disparu_est_oubliee() {
        let texte = texte_de_partie().replacen("\"bertrand\"", "\"inconnu\"", 1);
        let rechargee = charger_texte(&texte).unwrap().unwrap();
        assert!(rechargee.ames_donnees().is_empty());
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
        assert_memes_monstres(rechargee.collection(), partie.collection());
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
        // L'ancienne piste reprend au premier indice (section 9.4).
        assert_eq!(partie.indices_trouves(), 1);
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
        let texte = texte_de_partie().replace("\"position\":0", "\"position\":7");
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    #[test]
    fn une_attaque_a_choisir_peut_designer_un_monstre_de_l_equipe() {
        let texte = texte_de_partie().replace(
            "\"position\":0,\"dans_l_equipe\":false",
            "\"position\":1,\"dans_l_equipe\":true",
        );
        let partie = charger_texte(&texte).unwrap().unwrap();
        assert_eq!(
            partie.attaques_a_choisir()[0].emplacement,
            Emplacement::Equipe(1)
        );
    }

    #[test]
    fn avant_la_version_12_une_attaque_a_choisir_designe_le_camp() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 11.into());
        let a_choisir = objet["attaques_a_choisir"][0].as_object_mut().unwrap();
        a_choisir.remove("dans_l_equipe");
        let position = a_choisir.remove("position").unwrap();
        a_choisir.insert("position_au_camp".to_string(), position);
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(
            partie.attaques_a_choisir()[0].emplacement,
            Emplacement::Camp(0)
        );
    }

    #[test]
    fn une_sauvegarde_en_version_1_reprend_au_debut_de_la_traque() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 1.into());
        for champ in [
            "indices_trouves",
            "apotres_vaincus",
            "humains_vaincus",
            "conseils_vus",
        ] {
            objet.remove(champ);
        }
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(partie.indices_trouves(), 0);
        assert!(partie.apotres_vaincus().is_empty());
        assert!(partie.humains_vaincus().is_empty());
        assert!(partie.conseils_vus().is_empty());
        assert_memes_monstres(partie.collection(), partie_complete().collection());
    }

    #[test]
    fn avant_la_version_13_ni_sac_ni_coffre_n_ont_de_pierre() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 12.into());
        for inventaire in ["sac", "coffre_du_camp"] {
            objet[inventaire].as_object_mut().unwrap().remove("pierres");
        }
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert!(partie.coffre_du_camp().pierres().is_empty());
        assert!(partie.sac().pierres().is_empty());
        assert_eq!(partie.coffre_du_camp().capsules_d_habilete(), 2);
    }

    #[test]
    fn avant_la_version_14_gunther_est_au_camp_si_le_bourreau_est_vaincu() {
        for (humains, attendu) in [
            (vec!["brigand"], false),
            (vec!["brigand", "bourreau"], true),
        ] {
            let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
            let objet = valeur.as_object_mut().unwrap();
            objet.insert("version".to_string(), 13.into());
            for champ in ["gunther_au_camp", "ameliorations", "chantier"] {
                objet.remove(champ);
            }
            objet.insert("humains_vaincus".to_string(), humains.into());
            let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
            assert_eq!(partie.forge().est_au_camp(), attendu);
            assert_eq!(partie.forge().ameliorations(), &Ameliorations::default());
            assert_eq!(partie.forge().chantier(), None);
        }
    }

    #[test]
    fn avant_la_version_16_le_heros_n_a_aucun_objet() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 15.into());
        let heros = objet["heros"].as_object_mut().unwrap();
        heros.remove("objets");
        heros.remove("equipement_porte");
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(partie.heros().equipement(), &Equipement::default());
        assert_eq!(partie.heros().nom().as_str(), "Éléonore");
        assert!(partie.forge().chantier().is_some());
    }

    #[test]
    fn la_marque_d_un_monstre_recu_se_relit() {
        let texte = texte_de_partie();
        // Avant la version 17, aucun monstre n'est reçu.
        assert!(!texte.contains("recu_de"), "{texte}");
        let mut valeur: serde_json::Value = serde_json::from_str(&texte).unwrap();
        valeur["equipe"][0]
            .as_object_mut()
            .unwrap()
            .insert("recu_de".to_string(), "Ysolde".into());
        let relue = charger_texte(&valeur.to_string()).unwrap().unwrap();
        let monstre = &relue.collection().equipe()[0];
        assert_eq!(
            monstre.recu_de().map(|heros| heros.as_str()),
            Some("Ysolde")
        );
        let ecrite = serde_json::to_string(&format::depuis_partie(&relue)).unwrap();
        assert!(ecrite.contains("\"recu_de\":\"Ysolde\""), "{ecrite}");
    }

    #[test]
    fn les_paliers_du_codex_se_relisent_depuis_la_version_29() {
        let texte = texte_de_partie();
        assert!(
            texte.contains("\"paliers_du_codex\":[\"10\",\"complet\"]"),
            "{texte}"
        );
        let relue = charger_texte(&texte).unwrap().unwrap();
        assert_eq!(
            relue.paliers_du_codex(),
            &[PalierDuCodex::Especes(10), PalierDuCodex::Complet]
        );
        // Avant la version 29, rien de réclamé (section 5.6).
        let ancienne = texte.replacen("\"version\":30", "\"version\":28", 1);
        assert!(
            charger_texte(&ancienne)
                .unwrap()
                .unwrap()
                .paliers_du_codex()
                .is_empty()
        );
        let inconnu = texte.replacen("\"complet\"", "\"presque\"", 1);
        assert!(matches!(
            charger_texte(&inconnu),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    #[test]
    fn le_rite_se_relit_depuis_la_version_28() {
        let texte = texte_de_partie();
        let relue = charger_texte(&texte).unwrap().unwrap();
        let rite = relue.rite().unwrap();
        assert_eq!(rite.fin(), Horodatage::depuis_secondes(7_200));
        assert_eq!(rite.parents()[1].niveau().valeur(), 9);
        assert_eq!(relue, partie_complete());
        // Avant la version 28, pas de rite (section 13.11).
        let ancienne = texte.replacen("\"version\":30", "\"version\":27", 1);
        assert!(charger_texte(&ancienne).unwrap().unwrap().rite().is_none());
        let mut valeur: serde_json::Value = serde_json::from_str(&texte).unwrap();
        let parents = valeur["rite"]["parents"].as_array_mut().unwrap();
        parents.pop();
        assert!(matches!(
            charger_texte(&valeur.to_string()),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    #[test]
    fn les_filets_se_relisent_depuis_la_version_27() {
        let texte = texte_de_partie();
        let relue = charger_texte(&texte).unwrap().unwrap();
        assert_eq!(relue.sac().filets().nombre(Filet::Chanvre), 10);
        assert_eq!(relue.coffre_du_camp().filets().nombre(Filet::Chaines), 3);
        // Avant la version 27, les filets étaient illimités : la partie en
        // trouve 10 de chanvre au coffre, et rien d'autre (Q102).
        let ancienne = texte.replacen("\"version\":30", "\"version\":26", 1);
        let ancienne = charger_texte(&ancienne).unwrap().unwrap();
        assert!(ancienne.sac().filets().est_vide());
        assert_eq!(
            ancienne
                .coffre_du_camp()
                .filets()
                .presents()
                .collect::<Vec<_>>(),
            vec![(Filet::Chanvre, 10)]
        );
        let inconnu = texte.replacen("filet-de-chaines", "filet-d-or", 1);
        assert!(matches!(
            charger_texte(&inconnu),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    #[test]
    fn les_records_de_la_tour_se_relisent_depuis_la_version_26() {
        let texte = texte_de_partie();
        let relue = charger_texte(&texte).unwrap().unwrap();
        assert_eq!(
            relue.records_de_la_tour(),
            RecordsDeLaTour {
                seul: 23,
                a_deux: 31
            }
        );
        // Avant la version 26, pas de Tour sans fond (section 9.5).
        let ancienne = texte.replacen("\"version\":30", "\"version\":25", 1);
        let ancienne = charger_texte(&ancienne).unwrap().unwrap();
        assert_eq!(ancienne.records_de_la_tour(), RecordsDeLaTour::default());
    }

    #[test]
    fn le_beherit_se_relit_depuis_la_version_25() {
        let texte = texte_de_partie();
        assert!(texte.contains("\"beherit\":\"eclipse-ouverte\""), "{texte}");
        let relue = charger_texte(&texte).unwrap().unwrap();
        assert_eq!(relue.beherit(), EtatDuBeherit::EclipseOuverte);
        // Avant la version 25, pas de Béhérit (section 9.4).
        let ancienne = texte.replacen("\"version\":30", "\"version\":24", 1);
        let ancienne = charger_texte(&ancienne).unwrap().unwrap();
        assert_eq!(ancienne.beherit(), EtatDuBeherit::Aucun);
        let inconnu = texte.replacen("eclipse-ouverte", "brise", 1);
        assert!(matches!(
            charger_texte(&inconnu),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    #[test]
    fn la_location_d_un_monstre_se_relit_depuis_la_version_24() {
        let texte = texte_de_partie();
        assert!(!texte.contains("location"), "{texte}");
        let mut valeur: serde_json::Value = serde_json::from_str(&texte).unwrap();
        valeur["camp"][0]
            .as_object_mut()
            .unwrap()
            .insert("location".to_string(), 2.into());
        let relue = charger_texte(&valeur.to_string()).unwrap().unwrap();
        let monstre = &relue.collection().camp()[0];
        assert_eq!(monstre.expeditions_de_location(), Some(2));
        let ecrite = serde_json::to_string(&format::depuis_partie(&relue)).unwrap();
        assert!(ecrite.contains("\"location\":2"), "{ecrite}");
        // Avant la version 24, aucun monstre n'est loué (section 14.2).
        valeur
            .as_object_mut()
            .unwrap()
            .insert("version".to_string(), 23.into());
        let ancienne = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert!(ancienne.collection().monstre_loue().is_none());
    }

    #[test]
    fn l_equipement_et_l_objet_en_fabrication_se_relisent() {
        let partie = partie_complete();
        let equipement = partie.heros().equipement();
        assert_eq!(equipement.possedes(), [couteau(), talisman()]);
        assert_eq!(equipement.porte(PlaceDEquipement::Arme), None);
        assert_eq!(
            equipement.porte(PlaceDEquipement::SecondTalisman),
            Some(&talisman())
        );
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let chantier = valeur["chantier"].as_object_mut().unwrap();
        chantier.remove("amelioration");
        chantier.remove("niveau");
        chantier.insert("objet".to_string(), "couteau".into());
        let relue = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(
            relue.forge().chantier().map(|chantier| &chantier.ouvrage),
            Some(&Ouvrage::Objet(couteau()))
        );
        let ecrite = format::depuis_partie(&relue);
        let texte = serde_json::to_string(&ecrite).unwrap();
        assert!(texte.contains("\"objet\":\"couteau\""), "{texte}");
        assert!(!texte.contains("\"amelioration\""), "{texte}");
    }

    #[test]
    fn un_objet_disparu_du_jeu_est_refuse() {
        let texte = texte_de_partie().replacen("\"couteau\"", "\"inconnu\"", 1);
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        valeur["chantier"]
            .as_object_mut()
            .unwrap()
            .insert("objet".to_string(), "couteau".into());
        assert!(matches!(
            charger_texte(&valeur.to_string()),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    #[test]
    fn en_version_14_l_equipe_n_est_pas_amelioree() {
        let mut valeur: serde_json::Value = serde_json::from_str(&texte_de_partie()).unwrap();
        let objet = valeur.as_object_mut().unwrap();
        objet.insert("version".to_string(), 14.into());
        objet["ameliorations"]
            .as_object_mut()
            .unwrap()
            .remove("equipe");
        let partie = charger_texte(&valeur.to_string()).unwrap().unwrap();
        assert_eq!(
            partie.forge().ameliorations(),
            &Ameliorations::new(1, 2, 0).unwrap()
        );
        assert_eq!(partie.collection().tailles().equipe, 3);
    }

    #[test]
    fn des_ameliorations_sans_gunther_sont_refusees() {
        let texte =
            texte_de_partie().replace("\"gunther_au_camp\":true", "\"gunther_au_camp\":false");
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
    }

    #[test]
    fn pierre_inconnue() {
        let texte = texte_de_partie().replace("\"lune\"", "\"soleil\"");
        assert!(matches!(
            charger_texte(&texte),
            Err(ErreurSauvegarde::Incoherente(_))
        ));
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
