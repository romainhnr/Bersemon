//! Icônes des attaques en combat (section 10.6, étape 210) : une par type
//! et par catégorie, carrée pour une attaque physique, ronde pour une
//! spéciale. Sur les boutons, elles remplacent le type et la catégorie
//! écrits, que l'écran d'un téléphone coupait. Le bot les téléverse lui-même
//! comme émojis de son application, au démarrage ; tant qu'il ne les a pas,
//! un émoji Unicode du type et la lettre P ou S en tiennent lieu.

use std::collections::HashMap;
use std::sync::OnceLock;

use bersemon_core::attaque::Categorie;
use bersemon_core::types::Type;
use poise::serenity_prelude as serenity;

use crate::sprites::{Sprite, png_d_icone};

/// Le type et la catégorie d'une attaque, pour le monstre qui la lance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconeDAttaque {
    pub type_attaque: Type,
    pub physique: bool,
}

impl IconeDAttaque {
    /// `categorie_pour` ne rend jamais la catégorie adaptative : elle compte
    /// comme spéciale, comme dans le libellé de la console.
    pub fn new(type_attaque: Type, categorie: Categorie) -> Self {
        Self {
            type_attaque,
            physique: matches!(categorie, Categorie::Physique),
        }
    }

    /// Le nom de l'émoji d'application : Discord n'y admet que des lettres
    /// sans accent, des chiffres et des soulignés, de 2 à 32 caractères.
    pub fn nom(self) -> String {
        format!(
            "bersemon_{}_{}",
            identifiant(self.type_attaque),
            self.lettre().to_lowercase()
        )
    }

    fn lettre(self) -> &'static str {
        if self.physique { "P" } else { "S" }
    }

    /// Le fichier de l'icône, au format des sprites.
    fn fichier(self) -> String {
        format!(
            "{}-{}",
            identifiant(self.type_attaque),
            self.lettre().to_lowercase()
        )
    }

    /// Le PNG à téléverser, de 120 pixels de côté.
    pub fn png(self) -> Vec<u8> {
        let fichier = self.fichier();
        let texte = ICONES
            .iter()
            .find(|(nom, _)| *nom == fichier)
            .map(|(_, texte)| *texte)
            .expect("une icône par type et par catégorie");
        png_d_icone(&Sprite::lire(texte).expect("icône lisible"))
    }

    /// Les 46 icônes : chaque type, physique puis spéciale.
    pub fn toutes() -> impl Iterator<Item = IconeDAttaque> {
        Type::TOUS.into_iter().flat_map(|type_attaque| {
            [true, false].map(|physique| IconeDAttaque {
                type_attaque,
                physique,
            })
        })
    }
}

/// Fichiers de crates/bersemon-discord/icones, dessinés par Claude (Q125).
const ICONES: [(&str, &str); 46] = [
    ("acier-p", include_str!("../icones/acier-p.txt")),
    ("acier-s", include_str!("../icones/acier-s.txt")),
    ("combat-p", include_str!("../icones/combat-p.txt")),
    ("combat-s", include_str!("../icones/combat-s.txt")),
    ("dragon-p", include_str!("../icones/dragon-p.txt")),
    ("dragon-s", include_str!("../icones/dragon-s.txt")),
    ("eau-p", include_str!("../icones/eau-p.txt")),
    ("eau-s", include_str!("../icones/eau-s.txt")),
    ("electrique-p", include_str!("../icones/electrique-p.txt")),
    ("electrique-s", include_str!("../icones/electrique-s.txt")),
    ("feerique-p", include_str!("../icones/feerique-p.txt")),
    ("feerique-s", include_str!("../icones/feerique-s.txt")),
    ("feu-p", include_str!("../icones/feu-p.txt")),
    ("feu-s", include_str!("../icones/feu-s.txt")),
    ("glace-p", include_str!("../icones/glace-p.txt")),
    ("glace-s", include_str!("../icones/glace-s.txt")),
    ("humain-p", include_str!("../icones/humain-p.txt")),
    ("humain-s", include_str!("../icones/humain-s.txt")),
    ("insecte-p", include_str!("../icones/insecte-p.txt")),
    ("insecte-s", include_str!("../icones/insecte-s.txt")),
    ("lumiere-p", include_str!("../icones/lumiere-p.txt")),
    ("lumiere-s", include_str!("../icones/lumiere-s.txt")),
    ("normal-p", include_str!("../icones/normal-p.txt")),
    ("normal-s", include_str!("../icones/normal-s.txt")),
    ("obscur-p", include_str!("../icones/obscur-p.txt")),
    ("obscur-s", include_str!("../icones/obscur-s.txt")),
    ("plante-p", include_str!("../icones/plante-p.txt")),
    ("plante-s", include_str!("../icones/plante-s.txt")),
    ("poison-p", include_str!("../icones/poison-p.txt")),
    ("poison-s", include_str!("../icones/poison-s.txt")),
    ("psychique-p", include_str!("../icones/psychique-p.txt")),
    ("psychique-s", include_str!("../icones/psychique-s.txt")),
    ("roche-p", include_str!("../icones/roche-p.txt")),
    ("roche-s", include_str!("../icones/roche-s.txt")),
    ("sol-p", include_str!("../icones/sol-p.txt")),
    ("sol-s", include_str!("../icones/sol-s.txt")),
    ("son-p", include_str!("../icones/son-p.txt")),
    ("son-s", include_str!("../icones/son-s.txt")),
    ("spectre-p", include_str!("../icones/spectre-p.txt")),
    ("spectre-s", include_str!("../icones/spectre-s.txt")),
    ("stellaire-p", include_str!("../icones/stellaire-p.txt")),
    ("stellaire-s", include_str!("../icones/stellaire-s.txt")),
    ("tenebres-p", include_str!("../icones/tenebres-p.txt")),
    ("tenebres-s", include_str!("../icones/tenebres-s.txt")),
    ("vol-p", include_str!("../icones/vol-p.txt")),
    ("vol-s", include_str!("../icones/vol-s.txt")),
];

fn identifiant(type_attaque: Type) -> &'static str {
    match type_attaque {
        Type::Normal => "normal",
        Type::Combat => "combat",
        Type::Vol => "vol",
        Type::Poison => "poison",
        Type::Sol => "sol",
        Type::Roche => "roche",
        Type::Insecte => "insecte",
        Type::Spectre => "spectre",
        Type::Acier => "acier",
        Type::Feu => "feu",
        Type::Eau => "eau",
        Type::Plante => "plante",
        Type::Electrique => "electrique",
        Type::Psychique => "psychique",
        Type::Glace => "glace",
        Type::Dragon => "dragon",
        Type::Tenebres => "tenebres",
        Type::Feerique => "feerique",
        Type::Stellaire => "stellaire",
        Type::Obscur => "obscur",
        Type::Lumiere => "lumiere",
        Type::Son => "son",
        Type::Humain => "humain",
    }
}

/// L'émoji Unicode qui tient lieu d'icône tant que le bot n'a pas les
/// siennes : un objet ou un animal qui évoque le type.
fn unicode(type_attaque: Type) -> &'static str {
    match type_attaque {
        // Cercle blanc.
        Type::Normal => "\u{26AA}",
        // Poing.
        Type::Combat => "\u{1F44A}",
        // Aigle.
        Type::Vol => "\u{1F985}",
        // Tête de mort.
        Type::Poison => "\u{2620}\u{FE0F}",
        // Montagne.
        Type::Sol => "\u{26F0}\u{FE0F}",
        // Rocher.
        Type::Roche => "\u{1FAA8}",
        // Chenille.
        Type::Insecte => "\u{1F41B}",
        // Fantôme.
        Type::Spectre => "\u{1F47B}",
        // Engrenage.
        Type::Acier => "\u{2699}\u{FE0F}",
        // Feu.
        Type::Feu => "\u{1F525}",
        // Goutte.
        Type::Eau => "\u{1F4A7}",
        // Herbe.
        Type::Plante => "\u{1F33F}",
        // Éclair.
        Type::Electrique => "\u{26A1}",
        // Boule de cristal.
        Type::Psychique => "\u{1F52E}",
        // Flocon.
        Type::Glace => "\u{2744}\u{FE0F}",
        // Dragon.
        Type::Dragon => "\u{1F409}",
        // Nouvelle lune.
        Type::Tenebres => "\u{1F311}",
        // Fée.
        Type::Feerique => "\u{1F9DA}",
        // Étoile filante.
        Type::Stellaire => "\u{1F320}",
        // Trou.
        Type::Obscur => "\u{1F573}\u{FE0F}",
        // Soleil.
        Type::Lumiere => "\u{2600}\u{FE0F}",
        // Cloche.
        Type::Son => "\u{1F514}",
        // Silhouette.
        Type::Humain => "\u{1F464}",
    }
}

/// Les émojis de l'application, par nom, trouvés ou téléversés au démarrage.
static EMOTES: OnceLock<HashMap<String, serenity::EmojiId>> = OnceLock::new();

/// L'émoji du bouton, et, faute d'icône, la lettre de la catégorie à mettre
/// devant le libellé.
pub fn emoji(icone: IconeDAttaque) -> (serenity::ReactionType, Option<&'static str>) {
    emoji_parmi(icone, EMOTES.get())
}

fn emoji_parmi(
    icone: IconeDAttaque,
    emotes: Option<&HashMap<String, serenity::EmojiId>>,
) -> (serenity::ReactionType, Option<&'static str>) {
    let nom = icone.nom();
    match emotes.and_then(|emotes| emotes.get(&nom)) {
        Some(id) => (
            serenity::ReactionType::Custom {
                animated: false,
                id: *id,
                name: Some(nom),
            },
            None,
        ),
        None => (
            serenity::ReactionType::Unicode(unicode(icone.type_attaque).to_string()),
            Some(icone.lettre()),
        ),
    }
}

/// Au démarrage : retrouve les émojis de l'application et téléverse les
/// icônes qui manquent. Un échec laisse les émojis Unicode, sans empêcher le
/// bot de jouer.
pub async fn preparer(http: &serenity::Http) {
    let existants = match http.get_application_emojis().await {
        Ok(existants) => existants,
        Err(erreur) => {
            eprintln!("Icônes des attaques indisponibles, émojis Unicode à la place : {erreur}.");
            return;
        }
    };
    let mut emotes: HashMap<String, serenity::EmojiId> = existants
        .into_iter()
        .map(|emoji| (emoji.name, emoji.id))
        .collect();
    let mut televersees = 0;
    for icone in IconeDAttaque::toutes() {
        let nom = icone.nom();
        if emotes.contains_key(&nom) {
            continue;
        }
        let image = format!("data:image/png;base64,{}", base64(&icone.png()));
        let corps = HashMap::from([("name", nom.clone()), ("image", image)]);
        match http.create_application_emoji(&corps).await {
            Ok(emoji) => {
                emotes.insert(nom, emoji.id);
                televersees += 1;
            }
            Err(erreur) => eprintln!("Icône {nom} non téléversée : {erreur}."),
        }
    }
    let disponibles = IconeDAttaque::toutes()
        .filter(|icone| emotes.contains_key(&icone.nom()))
        .count();
    println!(
        "Icônes des attaques : {disponibles} sur {} ({televersees} téléversées).",
        ICONES.len()
    );
    if EMOTES.set(emotes).is_err() {
        eprintln!("Icônes des attaques déjà préparées.");
    }
}

/// Base64 standard, avec remplissage, pour l'image d'un émoji.
fn base64(octets: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut texte = String::with_capacity(octets.len().div_ceil(3) * 4);
    for bloc in octets.chunks(3) {
        let octet = |rang: usize| u32::from(bloc.get(rang).copied().unwrap_or(0));
        let nombre = (octet(0) << 16) | (octet(1) << 8) | octet(2);
        for rang in 0..4 {
            if rang <= bloc.len() {
                let indice = (nombre >> (18 - 6 * rang)) & 63;
                texte.push(char::from(ALPHABET[indice as usize]));
            } else {
                texte.push('=');
            }
        }
    }
    texte
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaque_type_a_ses_deux_icones_lisibles_et_distinctes() {
        let toutes: Vec<IconeDAttaque> = IconeDAttaque::toutes().collect();
        assert_eq!(toutes.len(), ICONES.len());
        for icone in &toutes {
            let nom = icone.nom();
            assert!(
                (2..=32).contains(&nom.len())
                    && nom.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
                "{nom}"
            );
            // Discord refuse un émoji de plus de 256 Kio.
            assert!(icone.png().len() < 256 * 1024, "{nom}");
        }
        for type_attaque in Type::TOUS {
            let physique = IconeDAttaque::new(type_attaque, Categorie::Physique);
            let speciale = IconeDAttaque::new(type_attaque, Categorie::Speciale);
            assert_ne!(physique.png(), speciale.png(), "{type_attaque:?}");
        }
    }

    #[test]
    fn sans_icone_un_emoji_unicode_et_la_lettre_de_la_categorie() {
        let icone = IconeDAttaque::new(Type::Feu, Categorie::Speciale);
        assert_eq!(icone.nom(), "bersemon_feu_s");
        let (emoji, lettre) = emoji_parmi(icone, None);
        assert_eq!(
            emoji,
            serenity::ReactionType::Unicode("\u{1F525}".to_string())
        );
        assert_eq!(lettre, Some("S"));
        let emotes = HashMap::from([("bersemon_feu_s".to_string(), serenity::EmojiId::new(42))]);
        let (emoji, lettre) = emoji_parmi(icone, Some(&emotes));
        assert_eq!(
            emoji,
            serenity::ReactionType::Custom {
                animated: false,
                id: serenity::EmojiId::new(42),
                name: Some("bersemon_feu_s".to_string()),
            }
        );
        assert_eq!(lettre, None);
        let physique = IconeDAttaque::new(Type::Combat, Categorie::Physique);
        assert_eq!(emoji_parmi(physique, Some(&emotes)).1, Some("P"));
    }

    #[test]
    fn base64_standard() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"M"), "TQ==");
        assert_eq!(base64(b"Ma"), "TWE=");
        assert_eq!(base64(b"Man"), "TWFu");
        assert_eq!(base64(b"Bersemon"), "QmVyc2Vtb24=");
    }

    /// Pour téléverser les icônes à la main ou les voir hors de Discord :
    /// `cargo test -p bersemon-discord ecrire_les_icones -- --ignored`, avec
    /// la variable BERSEMON_ICONES qui donne le dossier où écrire chaque PNG.
    #[test]
    #[ignore]
    fn ecrire_les_icones() {
        let dossier = std::path::PathBuf::from(
            std::env::var("BERSEMON_ICONES").expect("variable BERSEMON_ICONES"),
        );
        std::fs::create_dir_all(&dossier).unwrap();
        for icone in IconeDAttaque::toutes() {
            std::fs::write(dossier.join(format!("{}.png", icone.nom())), icone.png()).unwrap();
        }
    }
}
