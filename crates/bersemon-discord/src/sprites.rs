//! Sprites des monstres (section 5.1) : pour chaque espèce, une grille de
//! 24 x 24 pixels avec sa palette et sa palette shiny, écrite en texte dans
//! `crates/bersemon-discord/sprites/`. Le bot la rend en PNG agrandi, joint
//! à son message. Le PNG est écrit à la main, sans crate : une image à
//! palette, non compressée, suffit pour quelques kilo-octets.
//!
//! Format d'un fichier : des lignes de commentaire qui commencent par `#`,
//! puis `palette`, une couleur par ligne (`b 8a8a92` : un caractère et sa
//! couleur en hexadécimal), `shiny`, les mêmes caractères avec leurs couleurs
//! shiny, et enfin `dessin`, 24 lignes de 24 caractères, où `.` est
//! transparent.

use std::fmt;

use bersemon_core::espece::EspeceId;

/// Côté de la grille, en pixels.
pub const COTE: usize = 24;
/// Chaque pixel de la grille devient un carré de 6 x 6 : 144 pixels de côté
/// sur Discord.
pub const ECHELLE: usize = 6;
const TRANSPARENT: char = '.';

/// Identifiant de l'espèce et texte de son sprite.
const SPRITES: [(&str, &str); 5] = [
    ("salamandre", include_str!("../sprites/salamandre.txt")),
    ("crapaud", include_str!("../sprites/crapaud.txt")),
    ("mandragore", include_str!("../sprites/mandragore.txt")),
    ("chat-noir", include_str!("../sprites/chat-noir.txt")),
    ("loup", include_str!("../sprites/loup.txt")),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurSprite {
    /// Il manque `palette`, `shiny` ou `dessin`, ou ils sont dans le désordre.
    SectionManquante,
    CouleurIllisible(String),
    /// Une couleur de la palette manque à la palette shiny, ou l'inverse.
    PalettesDifferentes,
    TailleIncorrecte,
    /// Un caractère du dessin n'a pas de couleur.
    CaractereSansCouleur(char),
}

impl fmt::Display for ErreurSprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurSprite::SectionManquante => {
                write!(f, "section palette, shiny ou dessin manquante")
            }
            ErreurSprite::CouleurIllisible(ligne) => write!(f, "couleur illisible : {ligne}"),
            ErreurSprite::PalettesDifferentes => {
                write!(
                    f,
                    "la palette et la palette shiny n'ont pas les mêmes caractères"
                )
            }
            ErreurSprite::TailleIncorrecte => write!(f, "le dessin doit faire {COTE} x {COTE}"),
            ErreurSprite::CaractereSansCouleur(caractere) => {
                write!(f, "caractère sans couleur : {caractere}")
            }
        }
    }
}

impl std::error::Error for ErreurSprite {}

type Palette = Vec<(char, [u8; 3])>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sprite {
    palette: Palette,
    shiny: Palette,
    /// `COTE` lignes de `COTE` caractères.
    dessin: Vec<Vec<char>>,
}

impl Sprite {
    pub fn lire(texte: &str) -> Result<Self, ErreurSprite> {
        let mut section = "";
        let mut palette = Vec::new();
        let mut shiny = Vec::new();
        let mut dessin = Vec::new();
        for ligne in texte.lines() {
            let ligne = ligne.trim_end();
            if ligne.is_empty() || ligne.starts_with('#') {
                continue;
            }
            match (section, ligne) {
                ("", "palette") | ("palette", "shiny") | ("shiny", "dessin") => section = ligne,
                ("palette", _) => palette.push(lire_couleur(ligne)?),
                ("shiny", _) => shiny.push(lire_couleur(ligne)?),
                ("dessin", _) => dessin.push(ligne.chars().collect::<Vec<char>>()),
                _ => return Err(ErreurSprite::SectionManquante),
            }
        }
        if section != "dessin" {
            return Err(ErreurSprite::SectionManquante);
        }
        let caracteres = |palette: &Palette| {
            let mut caracteres: Vec<char> = palette.iter().map(|(c, _)| *c).collect();
            caracteres.sort_unstable();
            caracteres
        };
        if caracteres(&palette) != caracteres(&shiny) {
            return Err(ErreurSprite::PalettesDifferentes);
        }
        if dessin.len() != COTE || dessin.iter().any(|ligne| ligne.len() != COTE) {
            return Err(ErreurSprite::TailleIncorrecte);
        }
        if let Some(sans_couleur) = dessin
            .iter()
            .flatten()
            .find(|c| **c != TRANSPARENT && !palette.iter().any(|(k, _)| k == *c))
        {
            return Err(ErreurSprite::CaractereSansCouleur(*sans_couleur));
        }
        Ok(Self {
            palette,
            shiny,
            dessin,
        })
    }

    /// Le PNG agrandi, à palette : l'indice 0 est transparent, les suivants
    /// sont les couleurs dans l'ordre du fichier.
    pub fn png(&self, shiny: bool) -> Vec<u8> {
        let palette = if shiny { &self.shiny } else { &self.palette };
        let indice = |c: char| {
            palette
                .iter()
                .position(|(k, _)| *k == c)
                .map_or(0, |position| position as u8 + 1)
        };
        let cote = COTE * ECHELLE;
        let mut lignes = Vec::with_capacity(cote * (cote + 1));
        for ligne in &self.dessin {
            let pixels: Vec<u8> = ligne
                .iter()
                .flat_map(|c| std::iter::repeat_n(indice(*c), ECHELLE))
                .collect();
            for _ in 0..ECHELLE {
                // Filtre 0 : la ligne telle quelle.
                lignes.push(0);
                lignes.extend_from_slice(&pixels);
            }
        }
        let mut plte = vec![0, 0, 0];
        let mut trns = vec![0];
        for (_, couleur) in palette {
            plte.extend_from_slice(couleur);
            trns.push(255);
        }
        let cote = cote as u32;
        let mut ihdr = Vec::with_capacity(13);
        ihdr.extend_from_slice(&cote.to_be_bytes());
        ihdr.extend_from_slice(&cote.to_be_bytes());
        // 8 bits par pixel, couleurs à palette, sans entrelacement.
        ihdr.extend_from_slice(&[8, 3, 0, 0, 0]);
        let mut png = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
        ecrire_bloc(&mut png, b"IHDR", &ihdr);
        ecrire_bloc(&mut png, b"PLTE", &plte);
        ecrire_bloc(&mut png, b"tRNS", &trns);
        ecrire_bloc(&mut png, b"IDAT", &zlib_sans_compression(&lignes));
        ecrire_bloc(&mut png, b"IEND", &[]);
        png
    }
}

/// Le sprite de l'espèce, s'il est dessiné.
pub fn sprite(espece: &EspeceId) -> Option<Sprite> {
    let (_, texte) = SPRITES.iter().find(|(id, _)| *id == espece.as_str())?;
    Sprite::lire(texte).ok()
}

/// "loup.png", ou "loup-shiny.png" : Discord affiche la pièce jointe sous
/// ce nom.
pub fn nom_de_fichier(espece: &EspeceId, shiny: bool) -> String {
    if shiny {
        format!("{}-shiny.png", espece.as_str())
    } else {
        format!("{}.png", espece.as_str())
    }
}

/// Les espèces dessinées, dans l'ordre du fichier.
#[cfg(test)]
fn especes_dessinees() -> impl Iterator<Item = &'static str> {
    SPRITES.iter().map(|(id, _)| *id)
}

fn lire_couleur(ligne: &str) -> Result<(char, [u8; 3]), ErreurSprite> {
    let illisible = || ErreurSprite::CouleurIllisible(ligne.to_string());
    let mut morceaux = ligne.split_whitespace();
    let (Some(caractere), Some(hexa), None) = (morceaux.next(), morceaux.next(), morceaux.next())
    else {
        return Err(illisible());
    };
    let mut caracteres = caractere.chars();
    let (Some(caractere), None) = (caracteres.next(), caracteres.next()) else {
        return Err(illisible());
    };
    if caractere == TRANSPARENT || hexa.len() != 6 {
        return Err(illisible());
    }
    let composante = |debut: usize| {
        hexa.get(debut..debut + 2)
            .and_then(|deux| u8::from_str_radix(deux, 16).ok())
            .ok_or_else(illisible)
    };
    Ok((caractere, [composante(0)?, composante(2)?, composante(4)?]))
}

fn ecrire_bloc(png: &mut Vec<u8>, genre: &[u8; 4], donnees: &[u8]) {
    png.extend_from_slice(&(donnees.len() as u32).to_be_bytes());
    let debut = png.len();
    png.extend_from_slice(genre);
    png.extend_from_slice(donnees);
    let crc = crc32(&png[debut..]);
    png.extend_from_slice(&crc.to_be_bytes());
}

/// Un flux zlib fait de blocs deflate stockés, sans compression : au plus
/// 65 535 octets par bloc.
fn zlib_sans_compression(donnees: &[u8]) -> Vec<u8> {
    const BLOC_MAX: usize = 65_535;
    let mut flux = vec![0x78, 0x01];
    let mut blocs = donnees.chunks(BLOC_MAX).peekable();
    if blocs.peek().is_none() {
        flux.extend_from_slice(&[1, 0, 0, 0xff, 0xff]);
    }
    while let Some(bloc) = blocs.next() {
        let dernier = u8::from(blocs.peek().is_none());
        let longueur = bloc.len() as u16;
        flux.push(dernier);
        flux.extend_from_slice(&longueur.to_le_bytes());
        flux.extend_from_slice(&(!longueur).to_le_bytes());
        flux.extend_from_slice(bloc);
    }
    flux.extend_from_slice(&adler32(donnees).to_be_bytes());
    flux
}

fn crc32(donnees: &[u8]) -> u32 {
    let mut crc = 0xffff_ffff_u32;
    for octet in donnees {
        crc ^= u32::from(*octet);
        for _ in 0..8 {
            let masque = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xedb8_8320 & masque);
        }
    }
    !crc
}

fn adler32(donnees: &[u8]) -> u32 {
    const MODULO: u32 = 65_521;
    let (mut a, mut b) = (1_u32, 0_u32);
    for octet in donnees {
        a = (a + u32::from(*octet)) % MODULO;
        b = (b + a) % MODULO;
    }
    (b << 16) | a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaque_sprite_dessine_se_lit() {
        for (id, texte) in SPRITES {
            Sprite::lire(texte).unwrap_or_else(|erreur| panic!("{id} : {erreur}"));
        }
    }

    /// Pour voir les sprites hors de Discord : `cargo test -p bersemon-discord
    /// ecrire_les_png -- --ignored`, avec la variable BERSEMON_SPRITES qui
    /// donne le dossier où écrire chaque PNG, normal et shiny.
    #[test]
    #[ignore]
    fn ecrire_les_png() {
        let dossier = std::path::PathBuf::from(
            std::env::var("BERSEMON_SPRITES").expect("variable BERSEMON_SPRITES"),
        );
        std::fs::create_dir_all(&dossier).unwrap();
        for id in especes_dessinees() {
            let espece = EspeceId::new(id);
            let sprite = sprite(&espece).unwrap();
            for shiny in [false, true] {
                std::fs::write(
                    dossier.join(nom_de_fichier(&espece, shiny)),
                    sprite.png(shiny),
                )
                .unwrap();
            }
        }
    }

    #[test]
    fn controles_connus() {
        // Valeurs de référence de CRC-32 et d'Adler-32 pour "123456789".
        assert_eq!(crc32(b"123456789"), 0xcbf4_3926);
        assert_eq!(adler32(b"123456789"), 0x091e_01de);
    }

    #[test]
    fn le_png_a_la_bonne_taille_et_ses_blocs() {
        let loup = sprite(&EspeceId::new("loup")).unwrap();
        let png = loup.png(false);
        assert_eq!(&png[..8], &[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]);
        assert_eq!(&png[12..16], b"IHDR");
        assert_eq!(&png[16..20], &144_u32.to_be_bytes());
        assert_eq!(&png[20..24], &144_u32.to_be_bytes());
        assert!(png.ends_with(b"IEND\xae\x42\x60\x82"));
        // Le shiny ne change que la palette.
        assert_eq!(loup.png(true).len(), png.len());
        assert_ne!(loup.png(true), png);
    }

    #[test]
    fn un_sprite_mal_forme_est_refuse() {
        let ligne = ".".repeat(COTE);
        let dessin = vec![ligne.as_str(); COTE].join("\n");
        let correct = format!("palette\nb 102030\nshiny\nb 405060\ndessin\n{dessin}\n");
        assert!(Sprite::lire(&correct).is_ok());
        assert_eq!(
            Sprite::lire(&correct.replace("shiny\nb 405060", "shiny\nc 405060")),
            Err(ErreurSprite::PalettesDifferentes)
        );
        assert_eq!(
            Sprite::lire(&correct.replacen(
                "........................",
                "...x....................",
                1
            )),
            Err(ErreurSprite::CaractereSansCouleur('x'))
        );
        assert_eq!(
            Sprite::lire(&correct.replace("b 102030", "b 10203")),
            Err(ErreurSprite::CouleurIllisible("b 10203".to_string()))
        );
        assert_eq!(
            Sprite::lire("dessin\n"),
            Err(ErreurSprite::SectionManquante)
        );
    }
}
