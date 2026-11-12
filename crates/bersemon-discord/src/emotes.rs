//! Icônes des attaques en combat (section 10.6) : depuis l'étape 214, une
//! par type, en émoji du bouton, et un émoji Unicode pour la catégorie en
//! tête du libellé. Un bouton n'a qu'un émoji, et son libellé est du texte
//! brut, où une émote personnalisée ne s'affiche pas : type et catégorie ne
//! peuvent pas être deux émotes du bot. Le bot téléverse ses icônes comme
//! émojis de son application, au démarrage ; tant qu'il n'a pas celle d'un
//! type, un émoji Unicode en tient lieu, dans le libellé (étape 215).

use std::collections::HashMap;
use std::sync::RwLock;

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

    /// L'émoji Unicode de la catégorie, en tête du libellé : des épées
    /// croisées pour une attaque physique, des étincelles pour une spéciale.
    pub fn categorie(self) -> &'static str {
        if self.physique {
            "\u{2694}\u{FE0F}"
        } else {
            "\u{2728}"
        }
    }
}

/// Le nom de l'émoji d'application : Discord n'y admet que des lettres
/// sans accent, des chiffres et des soulignés, de 2 à 32 caractères.
fn nom_de_l_icone(type_attaque: Type) -> String {
    format!("bersemon_{}", identifiant(type_attaque))
}

/// Le PNG à téléverser, de 120 pixels de côté.
fn png_du_type(type_attaque: Type) -> Vec<u8> {
    let fichier = identifiant(type_attaque);
    let texte = ICONES
        .iter()
        .find(|(nom, _)| *nom == fichier)
        .map(|(_, texte)| *texte)
        .expect("une icône par type");
    png_d_icone(&Sprite::lire(texte).expect("icône lisible"))
}

/// Fichiers de crates/bersemon-discord/icones, dessinés par Claude (Q125).
const ICONES: [(&str, &str); 23] = [
    ("acier", include_str!("../icones/acier.txt")),
    ("combat", include_str!("../icones/combat.txt")),
    ("dragon", include_str!("../icones/dragon.txt")),
    ("eau", include_str!("../icones/eau.txt")),
    ("electrique", include_str!("../icones/electrique.txt")),
    ("feerique", include_str!("../icones/feerique.txt")),
    ("feu", include_str!("../icones/feu.txt")),
    ("glace", include_str!("../icones/glace.txt")),
    ("humain", include_str!("../icones/humain.txt")),
    ("insecte", include_str!("../icones/insecte.txt")),
    ("lumiere", include_str!("../icones/lumiere.txt")),
    ("normal", include_str!("../icones/normal.txt")),
    ("obscur", include_str!("../icones/obscur.txt")),
    ("plante", include_str!("../icones/plante.txt")),
    ("poison", include_str!("../icones/poison.txt")),
    ("psychique", include_str!("../icones/psychique.txt")),
    ("roche", include_str!("../icones/roche.txt")),
    ("sol", include_str!("../icones/sol.txt")),
    ("son", include_str!("../icones/son.txt")),
    ("spectre", include_str!("../icones/spectre.txt")),
    ("stellaire", include_str!("../icones/stellaire.txt")),
    ("tenebres", include_str!("../icones/tenebres.txt")),
    ("vol", include_str!("../icones/vol.txt")),
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

/// Les icônes de type disponibles, par nom, à mesure que le démarrage les
/// trouve ou les téléverse : chacune sert dès qu'elle est là (étape 214).
static EMOTES: RwLock<Vec<(String, serenity::EmojiId)>> = RwLock::new(Vec::new());

/// L'émoji du bouton : l'icône du type, si le bot l'a ; sinon, l'émoji
/// Unicode du type va dans le libellé (`unicode_du_type`).
pub fn emoji(icone: IconeDAttaque) -> Option<serenity::ReactionType> {
    EMOTES
        .read()
        .ok()
        .and_then(|emotes| emoji_parmi(icone.type_attaque, &emotes))
}

pub fn unicode_du_type(icone: IconeDAttaque) -> &'static str {
    unicode(icone.type_attaque)
}

fn emoji_parmi(
    type_attaque: Type,
    emotes: &[(String, serenity::EmojiId)],
) -> Option<serenity::ReactionType> {
    let nom = nom_de_l_icone(type_attaque);
    emotes
        .iter()
        .find(|(present, _)| *present == nom)
        .map(|(_, id)| serenity::ReactionType::Custom {
            animated: false,
            id: *id,
            name: Some(nom),
        })
}

fn retenir(nom: String, id: serenity::EmojiId) {
    if let Ok(mut emotes) = EMOTES.write() {
        emotes.push((nom, id));
    }
}

/// Au démarrage : retrouve les icônes déjà téléversées, qui servent aussitôt,
/// puis téléverse celles qui manquent, une à une. Un échec laisse les émojis
/// Unicode, sans empêcher le bot de jouer.
pub async fn preparer(http: &serenity::Http) {
    let existants = match http.get_application_emojis().await {
        Ok(existants) => existants,
        Err(erreur) => {
            eprintln!("Icônes des types indisponibles, émojis Unicode à la place : {erreur}.");
            return;
        }
    };
    let noms: Vec<String> = Type::TOUS.into_iter().map(nom_de_l_icone).collect();
    let mut disponibles = 0;
    let mut anciennes = 0;
    for emoji in existants {
        if noms.contains(&emoji.name) {
            retenir(emoji.name, emoji.id);
            disponibles += 1;
        } else if emoji.name.starts_with("bersemon_")
            && (emoji.name.ends_with("_p") || emoji.name.ends_with("_s"))
        {
            anciennes += 1;
        }
    }
    println!(
        "Icônes des types : {disponibles} sur {} déjà téléversées.",
        noms.len()
    );
    if anciennes > 0 {
        println!(
            "{anciennes} icônes d'avant l'étape 214 (bersemon_..._p et _s) ne servent plus : elles peuvent être supprimées dans le portail développeur, onglet Emojis."
        );
    }
    let mut televersees = 0;
    for type_attaque in Type::TOUS {
        let nom = nom_de_l_icone(type_attaque);
        if EMOTES
            .read()
            .is_ok_and(|emotes| emotes.iter().any(|(present, _)| *present == nom))
        {
            continue;
        }
        let image = format!(
            "data:image/png;base64,{}",
            base64(&png_du_type(type_attaque))
        );
        let corps = HashMap::from([("name", nom.clone()), ("image", image)]);
        match http.create_application_emoji(&corps).await {
            Ok(emoji) => {
                retenir(nom, emoji.id);
                televersees += 1;
            }
            Err(erreur) => eprintln!("Icône {nom} non téléversée : {erreur}."),
        }
    }
    if televersees > 0 {
        println!("Icônes des types : {televersees} téléversées.");
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
    fn chaque_type_a_son_icone_lisible_et_distincte() {
        assert_eq!(ICONES.len(), Type::TOUS.len());
        let mut images = Vec::new();
        for type_attaque in Type::TOUS {
            let nom = nom_de_l_icone(type_attaque);
            assert!(
                (2..=32).contains(&nom.len())
                    && nom.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
                "{nom}"
            );
            let png = png_du_type(type_attaque);
            // Discord refuse un émoji de plus de 256 Kio.
            assert!(png.len() < 256 * 1024, "{nom}");
            assert!(!images.contains(&png), "{nom}");
            images.push(png);
        }
    }

    #[test]
    fn l_icone_du_type_ou_son_emoji_unicode_et_la_categorie_en_emoji() {
        let icone = IconeDAttaque::new(Type::Feu, Categorie::Speciale);
        assert_eq!(nom_de_l_icone(Type::Feu), "bersemon_feu");
        assert_eq!(emoji_parmi(Type::Feu, &[]), None);
        assert_eq!(unicode_du_type(icone), "\u{1F525}");
        let emotes = [("bersemon_feu".to_string(), serenity::EmojiId::new(42))];
        assert_eq!(
            emoji_parmi(Type::Feu, &emotes),
            Some(serenity::ReactionType::Custom {
                animated: false,
                id: serenity::EmojiId::new(42),
                name: Some("bersemon_feu".to_string()),
            })
        );
        assert_eq!(emoji_parmi(Type::Combat, &emotes), None);
        assert_eq!(icone.categorie(), "\u{2728}");
        assert_eq!(
            IconeDAttaque::new(Type::Feu, Categorie::Physique).categorie(),
            "\u{2694}\u{FE0F}"
        );
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
        for type_attaque in Type::TOUS {
            std::fs::write(
                dossier.join(format!("{}.png", nom_de_l_icone(type_attaque))),
                png_du_type(type_attaque),
            )
            .unwrap();
        }
    }
}
