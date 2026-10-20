//! Messages privés envoyés au retour des expéditions automatiques (section
//! 12.7), à la fin des constructions de Gunther (section 13.4) et quand la
//! ligne d'Ansgar est prête (section 13.8). Chaque
//! retour n'est annoncé qu'une fois, même si le bot redémarre avant que le
//! joueur l'accueille (Q65) : les retours annoncés sont notés, un par ligne,
//! dans un fichier texte du dossier des parties.

use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bersemon_core::amelioration::Chantier;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::deroulement::Evenement;
use bersemon_core::expedition_automatique::ExpeditionAutomatique;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::sauvegarde::SauvegardeRepository;
use bersemon_core::zone::ZoneId;
use bersemon_presentation::forge::ouvrage;
use bersemon_storage::FichierJson;
use poise::serenity_prelude as serenity;

use crate::horloge::maintenant;
use crate::joueurs::{IdJoueur, Joueurs, verrouiller};

/// À côté des parties, qui sont les seuls fichiers `.json` du dossier.
const FICHIER: &str = "retours-annonces.txt";

/// Un retour est annoncé au plus une minute après son heure.
const PERIODE: Duration = Duration::from_secs(60);

/// Dans le fichier, à la place d'une zone : la forge de Gunther, qu'aucune
/// zone ne porte comme identifiant.
const CLE_DE_LA_FORGE: &str = "forge";

/// De même pour la ligne d'Ansgar.
const CLE_DE_LA_PECHE: &str = "peche";

/// Une expédition se reconnaît à son joueur, sa zone et son heure de retour :
/// une nouvelle expédition dans la même zone a une autre heure. Une
/// construction, à son joueur, à la forge et à son heure de fin ; une ligne,
/// à son joueur, à la pêche et à l'heure où elle est prête.
type Reference = (IdJoueur, String, Horodatage);

/// Ce qui est à annoncer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sujet {
    Expedition {
        zone: ZoneId,
        nom_de_zone: String,
    },
    /// Le niveau construit, déjà mis en texte.
    Forge {
        travail: String,
    },
    Peche,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Retour {
    pub joueur: IdJoueur,
    pub sujet: Sujet,
    pub heure: Horodatage,
}

impl Retour {
    fn de(joueur: IdJoueur, expedition: &ExpeditionAutomatique) -> Self {
        Self {
            joueur,
            sujet: Sujet::Expedition {
                zone: expedition.zone().id().clone(),
                nom_de_zone: expedition.zone().nom().to_string(),
            },
            heure: expedition.retour(),
        }
    }

    fn de_la_forge(joueur: IdJoueur, chantier: &Chantier) -> Self {
        Self {
            joueur,
            sujet: Sujet::Forge {
                travail: ouvrage(&chantier.ouvrage),
            },
            heure: chantier.fin,
        }
    }

    fn de_la_peche(joueur: IdJoueur, prete: Horodatage) -> Self {
        Self {
            joueur,
            sujet: Sujet::Peche,
            heure: prete,
        }
    }

    fn cle(&self) -> &str {
        match &self.sujet {
            Sujet::Expedition { zone, .. } => zone.as_str(),
            Sujet::Forge { .. } => CLE_DE_LA_FORGE,
            Sujet::Peche => CLE_DE_LA_PECHE,
        }
    }

    fn reference(&self) -> Reference {
        (self.joueur, self.cle().to_string(), self.heure)
    }

    /// Le résultat d'une expédition n'est tiré qu'à l'accueil : le message
    /// n'en dit rien.
    pub fn texte(&self) -> String {
        match &self.sujet {
            Sujet::Expedition { nom_de_zone, .. } => format!(
                "Votre expédition automatique dans {nom_de_zone} est rentrée. Ouvrez /bersemon pour l'accueillir."
            ),
            Sujet::Forge { travail } => {
                format!("Gunther a fini : {travail}. Ouvrez /bersemon pour constater son travail.")
            }
            Sujet::Peche => {
                "La ligne d'Ansgar est prête. Ouvrez /bersemon pour la relever.".to_string()
            }
        }
    }
}

/// Les retours qui restent à annoncer, tenus sous le même verrou que les
/// parties.
#[derive(Debug)]
pub struct Annonces {
    fichier: PathBuf,
    a_annoncer: Vec<Retour>,
}

impl Annonces {
    pub fn new(dossier: &Path) -> Self {
        Self {
            fichier: dossier.join(FICHIER),
            a_annoncer: Vec::new(),
        }
    }

    /// Au démarrage du bot : les expéditions de toutes les parties du
    /// dossier, sauf celles déjà annoncées. Le fichier des retours annoncés
    /// est récrit sans les expéditions qui n'existent plus.
    pub fn charger(&mut self, dossier: &Path, contenu: &ContenuV1) {
        let deja_annonces = lire_le_fichier(&self.fichier);
        let mut toujours_en_attente = Vec::new();
        self.a_annoncer.clear();
        for (joueur, chemin) in parties(dossier) {
            let partie = match FichierJson::new(&chemin).charger(contenu) {
                Ok(Some(partie)) => partie,
                Ok(None) => continue,
                Err(erreur) => {
                    eprintln!(
                        "Partie {} illisible, ses retours ne seront pas annoncés : {erreur}.",
                        chemin.display()
                    );
                    continue;
                }
            };
            let expeditions = partie
                .expeditions_automatiques()
                .iter()
                .map(|expedition| Retour::de(joueur, expedition));
            let chantier = partie
                .forge()
                .chantier()
                .map(|chantier| Retour::de_la_forge(joueur, chantier));
            let ligne = partie
                .peche()
                .ligne()
                .map(|prete| Retour::de_la_peche(joueur, prete));
            for retour in expeditions.chain(chantier).chain(ligne) {
                if deja_annonces.contains(&retour.reference()) {
                    toujours_en_attente.push(retour.reference());
                } else {
                    self.a_annoncer.push(retour);
                }
            }
        }
        if let Err(erreur) = fs::write(&self.fichier, en_lignes(&toujours_en_attente)) {
            eprintln!("Retours annoncés impossibles à noter : {erreur}.");
        }
    }

    /// Ce que vient de faire le joueur : un départ, un rappel, un accueil,
    /// une construction ou une fabrication lancée ou constatée, une ligne
    /// lancée ou relevée, ou une nouvelle partie, qui remplace l'ancienne
    /// avec ses expéditions, sa forge et sa ligne.
    pub fn suivre(&mut self, joueur: IdJoueur, evenements: &[Evenement]) {
        for evenement in evenements {
            match evenement {
                Evenement::ExpeditionAutomatiquePartie(expedition) => {
                    self.a_annoncer.push(Retour::de(joueur, expedition));
                }
                Evenement::ExpeditionAutomatiqueRappelee { expedition, .. } => {
                    self.oublier(joueur, Some(expedition.zone().id().as_str()));
                }
                Evenement::ExpeditionAutomatiqueRentree { bilan, .. } => {
                    self.oublier(joueur, Some(bilan.zone.as_str()));
                }
                Evenement::AmeliorationLancee { chantier, .. }
                | Evenement::FabricationLancee { chantier, .. } => {
                    self.a_annoncer.push(Retour::de_la_forge(joueur, chantier));
                }
                Evenement::TravailConstate(_) => {
                    self.oublier(joueur, Some(CLE_DE_LA_FORGE));
                }
                Evenement::LigneLancee { fin, .. } => {
                    self.a_annoncer.push(Retour::de_la_peche(joueur, *fin));
                }
                Evenement::PrisePechee(_) => self.oublier(joueur, Some(CLE_DE_LA_PECHE)),
                Evenement::PartieCommencee(_) | Evenement::Suicide => self.oublier(joueur, None),
                _ => {}
            }
        }
    }

    /// Tout ce qui est à annoncer au joueur quand `cle` vaut `None`.
    fn oublier(&mut self, joueur: IdJoueur, cle: Option<&str>) {
        self.a_annoncer
            .retain(|retour| retour.joueur != joueur || cle.is_some_and(|cle| retour.cle() != cle));
    }

    pub fn echus(&self, maintenant: Horodatage) -> Vec<Retour> {
        self.a_annoncer
            .iter()
            .filter(|retour| retour.heure <= maintenant)
            .cloned()
            .collect()
    }

    /// Le retour ne sera plus annoncé, même après un redémarrage.
    pub fn noter_annonce(&mut self, retour: &Retour) {
        self.a_annoncer.retain(|autre| autre != retour);
        let ajout = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.fichier)
            .and_then(|mut fichier| fichier.write_all(en_lignes(&[retour.reference()]).as_bytes()));
        if let Err(erreur) = ajout {
            eprintln!("Retour annoncé impossible à noter : {erreur}.");
        }
    }
}

/// Tâche de fond du bot. Le verrou des parties n'est jamais tenu pendant un
/// échange avec Discord. Un message refusé, par exemple parce que le joueur
/// n'accepte pas les messages privés du bot, est signalé et n'est pas
/// réessayé.
pub async fn annoncer_les_retours(http: Arc<serenity::Http>, joueurs: Arc<Mutex<Joueurs>>) {
    let mut minuteur = tokio::time::interval(PERIODE);
    loop {
        minuteur.tick().await;
        let echus = verrouiller(&joueurs).retours_echus(maintenant());
        for retour in echus {
            let message = serenity::CreateMessage::new().content(retour.texte());
            let envoi = serenity::UserId::new(retour.joueur)
                .direct_message(&http, message)
                .await;
            if let Err(erreur) = envoi {
                eprintln!(
                    "Message privé impossible pour le joueur {} : {erreur}.",
                    retour.joueur
                );
            }
            verrouiller(&joueurs).noter_annonce(&retour);
        }
    }
}

/// Les parties du dossier, une par compte : `<identifiant>.json`. Un
/// identifiant Discord n'est jamais nul.
pub(crate) fn parties(dossier: &Path) -> Vec<(IdJoueur, PathBuf)> {
    let Ok(entrees) = fs::read_dir(dossier) else {
        return Vec::new();
    };
    entrees
        .filter_map(Result::ok)
        .map(|entree| entree.path())
        .filter(|chemin| {
            chemin
                .extension()
                .is_some_and(|extension| extension == "json")
        })
        .filter_map(|chemin| {
            let joueur = chemin.file_stem()?.to_str()?.parse::<IdJoueur>().ok()?;
            (joueur > 0).then_some((joueur, chemin))
        })
        .collect()
}

/// Une ligne par retour : le joueur, la zone, la forge ou la pêche, et l'heure en
/// secondes.
fn en_lignes(references: &[Reference]) -> String {
    references
        .iter()
        .map(|(joueur, cle, heure)| format!("{joueur} {cle} {}\n", heure.secondes()))
        .collect()
}

/// Un fichier absent ou une ligne illisible ne bloque rien : au pire, un
/// retour est annoncé une seconde fois.
fn lire_le_fichier(fichier: &Path) -> HashSet<Reference> {
    let Ok(texte) = fs::read_to_string(fichier) else {
        return HashSet::new();
    };
    texte
        .lines()
        .filter_map(|ligne| {
            let mut morceaux = ligne.split_whitespace();
            let joueur = morceaux.next()?.parse().ok()?;
            let cle = morceaux.next()?.to_string();
            let heure = Horodatage::depuis_secondes(morceaux.next()?.parse().ok()?);
            Some((joueur, cle, heure))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bersemon_core::catalogue::Catalogue;
    use bersemon_core::collection::Emplacement;
    use bersemon_core::espece::EspeceId;
    use bersemon_core::heros::{Classe, Heros};
    use bersemon_core::monstre::Monstre;
    use bersemon_core::niveau::Niveau;
    use bersemon_core::nom::Nom;
    use bersemon_core::partie::{EtatPartie, Partie};
    use bersemon_core::stats::ValeursInternes;
    use std::sync::atomic::{AtomicU32, Ordering};

    const JOUEUR: IdJoueur = 42;

    fn dossier_de_test() -> PathBuf {
        static COMPTEUR: AtomicU32 = AtomicU32::new(0);
        let numero = COMPTEUR.fetch_add(1, Ordering::Relaxed);
        let dossier = std::env::temp_dir().join(format!(
            "bersemon-annonces-test-{}-{numero}",
            std::process::id()
        ));
        fs::create_dir_all(&dossier).unwrap();
        dossier
    }

    fn loup(contenu: &ContenuV1) -> Monstre {
        Monstre::creer(
            contenu.espece(&EspeceId::new("loup")).unwrap(),
            Niveau::new(5).unwrap(),
            ValeursInternes::default(),
            false,
        )
        .unwrap()
    }

    /// Une partie sauvegardée dont un Loup est parti dans la Lisière à
    /// l'heure 0, pour une heure.
    fn sauvegarder_une_expedition(dossier: &Path, contenu: &ContenuV1) -> Partie {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
        let mut partie = Partie::nouvelle(heros, loup(contenu));
        partie.prendre_monstre_rare(loup(contenu));
        let mut partie = Partie::restaurer(EtatPartie {
            heros: partie.heros().clone(),
            collection: partie.collection().clone(),
            sac: partie.sac().clone(),
            coffre_du_camp: partie.coffre_du_camp().clone(),
            marchand_au_camp: partie.marchand_au_camp(),
            ames_donnees: partie.ames_donnees().to_vec(),
            ames_portees: partie.ames_portees().to_vec(),
            ames_a_retrouver: partie.ames_a_retrouver().to_vec(),
            codex: partie.codex().clone(),
            zones_explorees: vec![ZoneId::new("lisiere")],
            indices_trouves: 1,
            apotres_vaincus: Vec::new(),
            humains_vaincus: Vec::new(),
            conseils_vus: Vec::new(),
            expeditions_automatiques: Vec::new(),
            attaques_a_choisir: Vec::new(),
            peche: Default::default(),
            forge: Default::default(),
            relations: Default::default(),
        })
        .unwrap();
        partie
            .envoyer_en_expedition_automatique(
                &contenu.zones()[0],
                &[Emplacement::Equipe(1)],
                Horodatage::depuis_secondes(0),
            )
            .unwrap();
        FichierJson::new(dossier.join(format!("{JOUEUR}.json")))
            .enregistrer(&partie)
            .unwrap();
        partie
    }

    #[test]
    fn un_retour_n_est_annonce_qu_une_fois_meme_apres_un_redemarrage() {
        let contenu = ContenuV1::new();
        let dossier = dossier_de_test();
        sauvegarder_une_expedition(&dossier, &contenu);
        let mut annonces = Annonces::new(&dossier);
        annonces.charger(&dossier, &contenu);
        assert!(
            annonces
                .echus(Horodatage::depuis_secondes(3_599))
                .is_empty()
        );
        let echus = annonces.echus(Horodatage::depuis_secondes(3_600));
        assert_eq!(echus.len(), 1);
        assert_eq!(echus[0].joueur, JOUEUR);
        assert_eq!(
            echus[0].texte(),
            "Votre expédition automatique dans Lisière du Bois-Sombre est rentrée. Ouvrez /bersemon pour l'accueillir."
        );
        annonces.noter_annonce(&echus[0]);
        assert!(
            annonces
                .echus(Horodatage::depuis_secondes(3_600))
                .is_empty()
        );

        // Au redémarrage, le retour déjà annoncé ne l'est pas de nouveau.
        let mut apres_redemarrage = Annonces::new(&dossier);
        apres_redemarrage.charger(&dossier, &contenu);
        let echus = apres_redemarrage.echus(Horodatage::depuis_secondes(10_000));
        let _ = fs::remove_dir_all(&dossier);
        assert!(echus.is_empty());
    }

    #[test]
    fn un_retour_accueilli_ou_rappele_n_est_plus_a_annoncer() {
        let contenu = ContenuV1::new();
        let dossier = dossier_de_test();
        let partie = sauvegarder_une_expedition(&dossier, &contenu);
        let expedition = partie.expeditions_automatiques()[0].clone();
        let mut annonces = Annonces::new(&dossier);
        annonces.suivre(
            JOUEUR,
            &[Evenement::ExpeditionAutomatiquePartie(expedition.clone())],
        );
        annonces.suivre(
            7,
            &[Evenement::ExpeditionAutomatiquePartie(expedition.clone())],
        );
        let heure = expedition.retour();
        assert_eq!(annonces.echus(heure).len(), 2);
        annonces.suivre(
            JOUEUR,
            &[Evenement::ExpeditionAutomatiqueRappelee {
                expedition: expedition.clone(),
                places: Vec::new(),
            }],
        );
        let echus = annonces.echus(heure);
        assert_eq!(echus.len(), 1);
        assert_eq!(echus[0].joueur, 7);
        // Une nouvelle partie remplace l'ancienne et ses expéditions.
        annonces.suivre(7, &[Evenement::PartieCommencee(loup(&contenu))]);
        let _ = fs::remove_dir_all(&dossier);
        assert!(annonces.echus(heure).is_empty());
    }

    #[test]
    fn la_fin_d_une_construction_est_annoncee_une_fois() {
        use bersemon_core::amelioration::{Amelioration, Chantier};
        use bersemon_core::contenu::ContenuV1;

        let contenu = ContenuV1::new();
        let dossier = dossier_de_test();
        let plan = contenu
            .plan_d_amelioration(Amelioration::Filets, 1)
            .unwrap()
            .clone();
        let chantier = Chantier {
            ouvrage: bersemon_core::amelioration::Ouvrage::Amelioration {
                amelioration: Amelioration::Filets,
                niveau: 1,
            },
            fin: Horodatage::depuis_secondes(3_600),
        };
        let mut annonces = Annonces::new(&dossier);
        annonces.suivre(
            JOUEUR,
            &[Evenement::AmeliorationLancee {
                plan: Box::new(plan),
                chantier: chantier.clone(),
            }],
        );
        assert!(
            annonces
                .echus(Horodatage::depuis_secondes(3_599))
                .is_empty()
        );
        let echus = annonces.echus(Horodatage::depuis_secondes(3_600));
        assert_eq!(echus.len(), 1);
        assert_eq!(
            echus[0].texte(),
            "Gunther a fini : Filets, niveau 1 (chance de capture +10 %). Ouvrez /bersemon pour constater son travail."
        );
        // Constatée avant l'annonce : il n'y a plus rien à annoncer.
        annonces.suivre(JOUEUR, &[Evenement::TravailConstate(chantier)]);
        let _ = fs::remove_dir_all(&dossier);
        assert!(
            annonces
                .echus(Horodatage::depuis_secondes(3_600))
                .is_empty()
        );
    }
}
