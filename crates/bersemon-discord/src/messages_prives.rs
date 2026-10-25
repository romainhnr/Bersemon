//! Messages privés envoyés dès que quelque chose se termine (section 2) : le
//! retour d'une expédition automatique (section 12.7), un travail de Gunther
//! (section 13.4), la ligne d'Ansgar prête (section 13.8), le Rite du sang
//! mêlé achevé (section 13.11), les pièges pleins et un monstre de la cour
//! d'entraînement à son plafond (section 13.6). Ce qui est à annoncer se
//! déduit de la partie, à chaque action du joueur et au démarrage du bot.
//! Chaque retour n'est annoncé qu'une fois, même si le bot redémarre avant
//! que le joueur l'accueille (Q65) : les retours annoncés sont notés, un par
//! ligne, dans un fichier texte du dossier des parties.

use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bersemon_core::amelioration::Chantier;
use bersemon_core::combat::Cote;
use bersemon_core::contenu::ContenuV1;
use bersemon_core::expedition_automatique::ExpeditionAutomatique;
use bersemon_core::horodatage::Horodatage;
use bersemon_core::partie::Partie;
use bersemon_core::sauvegarde::SauvegardeRepository;
use bersemon_core::zone::ZoneId;
use bersemon_presentation::forge::ouvrage;
use bersemon_presentation::monstres::nom;
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

/// De même pour la ligne d'Ansgar, le rite, les pièges et la cour.
const CLE_DE_LA_PECHE: &str = "peche";
const CLE_DU_RITE: &str = "rite";
const CLE_DES_PIEGES: &str = "pieges";
const CLE_DE_LA_COUR: &str = "cour";

/// Une expédition se reconnaît à son joueur, sa zone et son heure de retour :
/// une nouvelle expédition dans la même zone a une autre heure. Les autres
/// retours, à leur joueur, à leur clé et à leur heure : la fin d'un travail
/// ou d'un rite, l'heure où la ligne est prête, où les pièges sont pleins ou
/// où un monstre de la cour atteint son plafond.
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
    Rite,
    Pieges,
    /// Le nom du monstre à son plafond.
    Entrainement {
        monstre: String,
    },
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

    fn cle(&self) -> &str {
        match &self.sujet {
            Sujet::Expedition { zone, .. } => zone.as_str(),
            Sujet::Forge { .. } => CLE_DE_LA_FORGE,
            Sujet::Peche => CLE_DE_LA_PECHE,
            Sujet::Rite => CLE_DU_RITE,
            Sujet::Pieges => CLE_DES_PIEGES,
            Sujet::Entrainement { .. } => CLE_DE_LA_COUR,
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
            Sujet::Rite => "Le Rite du sang mêlé est achevé : le petit attend auprès de Malvoise. Ouvrez /bersemon pour l'accueillir.".to_string(),
            Sujet::Pieges => {
                "Les pièges de Gunther sont pleins. Ouvrez /bersemon pour les relever.".to_string()
            }
            Sujet::Entrainement { monstre } => format!(
                "Entraînement terminé : {monstre} a atteint le plafond de la cour d'entraînement. Ouvrez /bersemon pour reprendre ce monstre."
            ),
        }
    }
}

/// Tout ce qui finira dans cette partie, annoncé ou non : ses expéditions
/// automatiques, le travail de Gunther, la ligne d'Ansgar, le rite, les
/// pièges et chaque monstre de la cour qui n'est pas à son plafond.
fn retours_de(joueur: IdJoueur, partie: &Partie, contenu: &ContenuV1) -> Vec<Retour> {
    let retour = |sujet: Sujet, heure: Horodatage| Retour {
        joueur,
        sujet,
        heure,
    };
    let mut retours: Vec<Retour> = partie
        .expeditions_automatiques()
        .iter()
        .map(|expedition| Retour::de(joueur, expedition))
        .collect();
    retours.extend(
        partie
            .forge()
            .chantier()
            .map(|chantier| Retour::de_la_forge(joueur, chantier)),
    );
    retours.extend(
        partie
            .peche()
            .ligne()
            .map(|prete| retour(Sujet::Peche, prete)),
    );
    retours.extend(partie.rite().map(|rite| retour(Sujet::Rite, rite.fin())));
    let ameliorations = partie.forge().ameliorations();
    if let (Some(regles), Some(pieges)) = (ameliorations.pieges(), partie.machines().pieges()) {
        retours.push(retour(Sujet::Pieges, pieges.pleins_a(&regles)));
    }
    if let Some(regles) = ameliorations.cour() {
        let plafond = partie.plafond_d_entrainement(contenu.zones(), contenu.piste());
        for entrainement in partie.machines().cour() {
            let monstre = nom(entrainement.monstre(), Cote::Joueur);
            let heure = entrainement.plafond_atteint_a(&regles, plafond);
            retours.push(retour(Sujet::Entrainement { monstre }, heure));
        }
    }
    retours
}

/// Les retours qui restent à annoncer, tenus sous le même verrou que les
/// parties.
#[derive(Debug)]
pub struct Annonces {
    fichier: PathBuf,
    a_annoncer: Vec<Retour>,
    /// Ceux du fichier : à ne plus annoncer, même s'ils se déduisent encore
    /// de la partie.
    deja_annonces: HashSet<Reference>,
}

impl Annonces {
    pub fn new(dossier: &Path) -> Self {
        Self {
            fichier: dossier.join(FICHIER),
            a_annoncer: Vec::new(),
            deja_annonces: HashSet::new(),
        }
    }

    /// Au démarrage du bot : les retours de toutes les parties du dossier,
    /// sauf ceux déjà annoncés. Le fichier des retours annoncés est récrit
    /// sans ceux qui ne se déduisent plus d'aucune partie.
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
            for retour in retours_de(joueur, &partie, contenu) {
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
        self.deja_annonces = toujours_en_attente.into_iter().collect();
    }

    /// Après chaque action du joueur : ce qui est à lui annoncer se déduit
    /// de nouveau de sa partie, qu'il ait lancé, constaté, relevé, accueilli
    /// ou rappelé quelque chose. Sans partie, par exemple après un suicide,
    /// plus rien.
    pub fn actualiser(&mut self, joueur: IdJoueur, partie: Option<&Partie>, contenu: &ContenuV1) {
        self.a_annoncer.retain(|retour| retour.joueur != joueur);
        let Some(partie) = partie else {
            return;
        };
        for retour in retours_de(joueur, partie, contenu) {
            if !self.deja_annonces.contains(&retour.reference()) {
                self.a_annoncer.push(retour);
            }
        }
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
        self.deja_annonces.insert(retour.reference());
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

    /// Deux Loups, la Lisière explorée : rien n'y finit encore.
    fn etat_de_depart(contenu: &ContenuV1) -> EtatPartie {
        let heros = Heros::new(Nom::new("Aldric").unwrap(), Classe::Mage);
        let mut partie = Partie::nouvelle(heros, loup(contenu));
        partie.prendre_monstre_rare(loup(contenu));
        EtatPartie {
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
            machines: Default::default(),
            beherit: Default::default(),
            records_de_la_tour: Default::default(),
            rite: None,
            paliers_du_codex: Vec::new(),
        }
    }

    /// Une partie sauvegardée dont un Loup est parti dans la Lisière à
    /// l'heure 0, pour une heure.
    fn sauvegarder_une_expedition(dossier: &Path, contenu: &ContenuV1) -> Partie {
        let mut partie = Partie::restaurer(etat_de_depart(contenu)).unwrap();
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
        let mut partie = sauvegarder_une_expedition(&dossier, &contenu);
        let heure = partie.expeditions_automatiques()[0].retour();
        let mut annonces = Annonces::new(&dossier);
        annonces.actualiser(JOUEUR, Some(&partie), &contenu);
        annonces.actualiser(7, Some(&partie), &contenu);
        assert_eq!(annonces.echus(heure).len(), 2);
        partie
            .rappeler_expedition_automatique(
                &ZoneId::new("lisiere"),
                Horodatage::depuis_secondes(60),
            )
            .unwrap();
        annonces.actualiser(JOUEUR, Some(&partie), &contenu);
        let echus = annonces.echus(heure);
        assert_eq!(echus.len(), 1);
        assert_eq!(echus[0].joueur, 7);
        // Sans partie, par exemple après un suicide, plus rien.
        annonces.actualiser(7, None, &contenu);
        let _ = fs::remove_dir_all(&dossier);
        assert!(annonces.echus(heure).is_empty());
    }

    #[test]
    fn tout_ce_qui_se_termine_est_annonce_une_fois() {
        use bersemon_core::amelioration::{Ameliorations, Chantier, Forge, Ouvrage};
        use bersemon_core::filet::Filet;
        use bersemon_core::machines::{Entrainement, Machines, Pieges};
        use bersemon_core::rite::Rite;

        let contenu = ContenuV1::new();
        let dossier = dossier_de_test();
        let a = Horodatage::depuis_secondes;
        let mut etat = etat_de_depart(&contenu);
        // Un lot de filets, que le suivi d'avant ne voyait pas.
        let chantier = Chantier {
            ouvrage: Ouvrage::Filets {
                filet: Filet::Leste,
                nombre: 5,
            },
            fin: a(3_600),
        };
        let ameliorations = Ameliorations::new(0, 0, 0)
            .unwrap()
            .avec_machines(1, 1)
            .unwrap();
        etat.forge = Forge::restaurer(true, ameliorations, Some(chantier)).unwrap();
        etat.rite = Some(Rite::new([loup(&contenu), loup(&contenu)], a(7_200)));
        let entrainement = Entrainement::new(loup(&contenu), a(0));
        etat.machines = Machines::restaurer(
            Some(Pieges::new(ZoneId::new("lisiere"), a(0))),
            vec![entrainement.clone()],
        );
        let partie = Partie::restaurer(etat).unwrap();
        let plafond = partie.plafond_d_entrainement(contenu.zones(), contenu.piste());
        let regles_de_la_cour = partie.forge().ameliorations().cour().unwrap();
        let au_plafond = entrainement.plafond_atteint_a(&regles_de_la_cour, plafond);
        let mut annonces = Annonces::new(&dossier);
        annonces.actualiser(JOUEUR, Some(&partie), &contenu);
        let textes = |heure: Horodatage| -> Vec<String> {
            annonces.echus(heure).iter().map(Retour::texte).collect()
        };
        assert!(textes(a(3_599)).is_empty());
        assert_eq!(
            textes(a(3_600)),
            vec!["Gunther a fini : 5 filets lestés. Ouvrez /bersemon pour constater son travail."]
        );
        assert!(textes(a(7_200)).contains(&"Le Rite du sang mêlé est achevé : le petit attend auprès de Malvoise. Ouvrez /bersemon pour l'accueillir.".to_string()));
        // Les pièges du niveau 1 sont pleins au bout de 6 fois 2 heures.
        assert!(
            !textes(a(43_199))
                .iter()
                .any(|texte| texte.contains("pièges"))
        );
        assert!(textes(a(43_200)).contains(
            &"Les pièges de Gunther sont pleins. Ouvrez /bersemon pour les relever.".to_string()
        ));
        assert!(textes(au_plafond).contains(&"Entraînement terminé : Loup a atteint le plafond de la cour d'entraînement. Ouvrez /bersemon pour reprendre ce monstre.".to_string()));
        // Annoncés, ils ne reviennent pas avec l'action suivante.
        let tous = annonces.echus(a(1_000_000));
        assert_eq!(tous.len(), 4);
        for retour in &tous {
            annonces.noter_annonce(retour);
        }
        annonces.actualiser(JOUEUR, Some(&partie), &contenu);
        let _ = fs::remove_dir_all(&dossier);
        assert!(annonces.echus(a(1_000_000)).is_empty());
    }
}
