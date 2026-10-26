//! Sprites des monstres et des humains (section 5.1) : pour chaque espèce,
//! chaque classe de héros et chaque humain de l'IA, une grille de 24 x 24
//! pixels avec sa palette, écrite en texte dans
//! `crates/bersemon-discord/sprites/`. Le bot la rend en PNG agrandi, joint
//! à son message, seule ou à côté d'autres. Le PNG est écrit à la main, sans
//! crate : une image à palette, non compressée, suffit pour quelques
//! dizaines de kilo-octets.
//!
//! Format d'un fichier : des lignes de commentaire qui commencent par `#`,
//! puis `palette`, une couleur par ligne (`b 8a8a92` : un caractère et sa
//! couleur en hexadécimal), `shiny`, les mêmes caractères avec leurs couleurs
//! shiny, et enfin `dessin`, 24 lignes de 24 caractères, où `.` est
//! transparent. Un humain, jamais shiny, n'a pas de section `shiny`.

use std::fmt;

use bersemon_core::espece::EspeceId;

/// Côté de la grille, en pixels.
pub const COTE: usize = 24;
/// Chaque pixel de la grille devient un carré de 6 x 6 : 144 pixels de côté
/// sur Discord.
pub const ECHELLE: usize = 6;
const TRANSPARENT: char = '.';
/// Écart entre deux sprites d'une même image, en pixels de la grille.
const ECART: usize = 4;
/// Une image en montre au plus quatre, les quatre classes au choix de la
/// classe ; avec 63 couleurs chacun, ils tiennent dans les 256 couleurs d'un
/// PNG à palette, l'indice 0 restant transparent.
const CASES_MAX: usize = 4;
const COULEURS_MAX: usize = 63;

/// Identifiant de l'espèce et texte de son sprite, dans l'ordre du codex.
const SPRITES: [(&str, &str); 65] = [
    ("salamandre", include_str!("../sprites/salamandre.txt")),
    ("crapaud", include_str!("../sprites/crapaud.txt")),
    ("mandragore", include_str!("../sprites/mandragore.txt")),
    ("chat-noir", include_str!("../sprites/chat-noir.txt")),
    ("loup", include_str!("../sprites/loup.txt")),
    ("corbeau", include_str!("../sprites/corbeau.txt")),
    ("vipere", include_str!("../sprites/vipere.txt")),
    ("feu-follet", include_str!("../sprites/feu-follet.txt")),
    (
        "armure-hantee",
        include_str!("../sprites/armure-hantee.txt"),
    ),
    ("fee-noire", include_str!("../sprites/fee-noire.txt")),
    ("squelette", include_str!("../sprites/squelette.txt")),
    ("cerf-blanc", include_str!("../sprites/cerf-blanc.txt")),
    ("basilic", include_str!("../sprites/basilic.txt")),
    ("elfe", include_str!("../sprites/elfe.txt")),
    ("vouivre", include_str!("../sprites/vouivre.txt")),
    ("baron-pendu", include_str!("../sprites/baron-pendu.txt")),
    ("schnoz", include_str!("../sprites/schnoz.txt")),
    (
        "lezard-de-braise",
        include_str!("../sprites/lezard-de-braise.txt"),
    ),
    (
        "crapaud-geant",
        include_str!("../sprites/crapaud-geant.txt"),
    ),
    (
        "mandragore-ancienne",
        include_str!("../sprites/mandragore-ancienne.txt"),
    ),
    (
        "chat-des-ombres",
        include_str!("../sprites/chat-des-ombres.txt"),
    ),
    ("warg", include_str!("../sprites/warg.txt")),
    (
        "corbeau-des-gibets",
        include_str!("../sprites/corbeau-des-gibets.txt"),
    ),
    ("aspic", include_str!("../sprites/aspic.txt")),
    ("fee-sorciere", include_str!("../sprites/fee-sorciere.txt")),
    (
        "basilic-royal",
        include_str!("../sprites/basilic-royal.txt"),
    ),
    (
        "lanterne-des-morts",
        include_str!("../sprites/lanterne-des-morts.txt"),
    ),
    ("cerf-de-lune", include_str!("../sprites/cerf-de-lune.txt")),
    (
        "chevalier-noir",
        include_str!("../sprites/chevalier-noir.txt"),
    ),
    (
        "chevalier-squelette",
        include_str!("../sprites/chevalier-squelette.txt"),
    ),
    ("gargouille", include_str!("../sprites/gargouille.txt")),
    ("goule", include_str!("../sprites/goule.txt")),
    (
        "chauve-souris-de-crypte",
        include_str!("../sprites/chauve-souris-de-crypte.txt"),
    ),
    ("chimere", include_str!("../sprites/chimere.txt")),
    ("ange-dechu", include_str!("../sprites/ange-dechu.txt")),
    (
        "abbesse-ecorchee",
        include_str!("../sprites/abbesse-ecorchee.txt"),
    ),
    (
        "reine-des-ronces",
        include_str!("../sprites/reine-des-ronces.txt"),
    ),
    ("noye", include_str!("../sprites/noye.txt")),
    (
        "anguille-des-tourbieres",
        include_str!("../sprites/anguille-des-tourbieres.txt"),
    ),
    (
        "sangsue-geante",
        include_str!("../sprites/sangsue-geante.txt"),
    ),
    ("kelpie", include_str!("../sprites/kelpie.txt")),
    ("wendigo", include_str!("../sprites/wendigo.txt")),
    (
        "troll-des-cavernes",
        include_str!("../sprites/troll-des-cavernes.txt"),
    ),
    (
        "harpie-des-orages",
        include_str!("../sprites/harpie-des-orages.txt"),
    ),
    (
        "wyrm-de-givre",
        include_str!("../sprites/wyrm-de-givre.txt"),
    ),
    (
        "sylphe-des-neiges",
        include_str!("../sprites/sylphe-des-neiges.txt"),
    ),
    (
        "margrave-de-givre",
        include_str!("../sprites/margrave-de-givre.txt"),
    ),
    (
        "wyverne-de-cendre",
        include_str!("../sprites/wyverne-de-cendre.txt"),
    ),
    ("crapaud-roi", include_str!("../sprites/crapaud-roi.txt")),
    ("arbre-pendu", include_str!("../sprites/arbre-pendu.txt")),
    ("cait-sith", include_str!("../sprites/cait-sith.txt")),
    ("loup-garou", include_str!("../sprites/loup-garou.txt")),
    ("demiurge", include_str!("../sprites/demiurge.txt")),
    (
        "demiurge-eveille",
        include_str!("../sprites/demiurge-eveille.txt"),
    ),
    (
        "cloche-hurlante",
        include_str!("../sprites/cloche-hurlante.txt"),
    ),
    (
        "seraphin-de-vitrail",
        include_str!("../sprites/seraphin-de-vitrail.txt"),
    ),
    (
        "molosse-des-fosses",
        include_str!("../sprites/molosse-des-fosses.txt"),
    ),
    (
        "taureau-d-airain",
        include_str!("../sprites/taureau-d-airain.txt"),
    ),
    (
        "crotale-des-salines",
        include_str!("../sprites/crotale-des-salines.txt"),
    ),
    (
        "scarabee-solaire",
        include_str!("../sprites/scarabee-solaire.txt"),
    ),
    ("momie-de-sel", include_str!("../sprites/momie-de-sel.txt")),
    (
        "ver-des-salines",
        include_str!("../sprites/ver-des-salines.txt"),
    ),
    (
        "sphinx-des-sables",
        include_str!("../sprites/sphinx-des-sables.txt"),
    ),
    ("phenix-blanc", include_str!("../sprites/phenix-blanc.txt")),
    (
        "dragon-des-brumes",
        include_str!("../sprites/dragon-des-brumes.txt"),
    ),
];

/// Identifiant de l'espèce du combattant et texte de son sprite : les
/// classes des héros des joueurs, puis les humains de l'IA, ceux des zones
/// puis ceux de la Fosse aux damnés (section 5.1).
const SPRITES_DES_HUMAINS: [(&str, &str); 19] = [
    (
        "heros-guerrier",
        include_str!("../sprites/heros/guerrier.txt"),
    ),
    ("heros-mage", include_str!("../sprites/heros/mage.txt")),
    (
        "heros-templier",
        include_str!("../sprites/heros/templier.txt"),
    ),
    (
        "heros-assassin",
        include_str!("../sprites/heros/assassin.txt"),
    ),
    (
        "humain-brigand",
        include_str!("../sprites/humains/brigand.txt"),
    ),
    (
        "humain-bourreau",
        include_str!("../sprites/humains/bourreau.txt"),
    ),
    (
        "humain-inquisiteur",
        include_str!("../sprites/humains/inquisiteur.txt"),
    ),
    (
        "humain-porte-etendard",
        include_str!("../sprites/humains/porte-etendard.txt"),
    ),
    (
        "humain-pythie-de-sel",
        include_str!("../sprites/humains/pythie-de-sel.txt"),
    ),
    ("humain-gueux", include_str!("../sprites/humains/gueux.txt")),
    (
        "humain-coupe-jarret",
        include_str!("../sprites/humains/coupe-jarret.txt"),
    ),
    (
        "humain-novice-de-l-ordre",
        include_str!("../sprites/humains/novice-de-l-ordre.txt"),
    ),
    (
        "humain-apprenti-necromant",
        include_str!("../sprites/humains/apprenti-necromant.txt"),
    ),
    (
        "humain-mercenaire",
        include_str!("../sprites/humains/mercenaire.txt"),
    ),
    (
        "humain-lame-noire",
        include_str!("../sprites/humains/lame-noire.txt"),
    ),
    (
        "humain-sorciere-des-cendres",
        include_str!("../sprites/humains/sorciere-des-cendres.txt"),
    ),
    (
        "humain-templier-renegat",
        include_str!("../sprites/humains/templier-renegat.txt"),
    ),
    (
        "humain-gladiateur",
        include_str!("../sprites/humains/gladiateur.txt"),
    ),
    (
        "humain-champion-de-la-fosse",
        include_str!("../sprites/humains/champion-de-la-fosse.txt"),
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurSprite {
    /// Il manque `palette` ou `dessin`, ou les sections sont dans le
    /// désordre.
    SectionManquante,
    CouleurIllisible(String),
    /// Une couleur de la palette manque à la palette shiny, ou l'inverse.
    PalettesDifferentes,
    TropDeCouleurs,
    TailleIncorrecte,
    /// Un caractère du dessin n'a pas de couleur.
    CaractereSansCouleur(char),
}

impl fmt::Display for ErreurSprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErreurSprite::SectionManquante => {
                write!(f, "section palette ou dessin manquante")
            }
            ErreurSprite::CouleurIllisible(ligne) => write!(f, "couleur illisible : {ligne}"),
            ErreurSprite::PalettesDifferentes => {
                write!(
                    f,
                    "la palette et la palette shiny n'ont pas les mêmes caractères"
                )
            }
            ErreurSprite::TropDeCouleurs => write!(f, "plus de {COULEURS_MAX} couleurs"),
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
    /// `None` pour un humain, jamais shiny.
    shiny: Option<Palette>,
    /// `COTE` lignes de `COTE` caractères.
    dessin: Vec<Vec<char>>,
}

/// Un sprite placé dans une image : avec sa palette shiny ou non, et
/// retourné ou non, pour regarder vers la droite comme le combattant du
/// joueur face à l'adversaire.
#[derive(Debug, Clone, Copy)]
pub struct Case<'a> {
    pub sprite: &'a Sprite,
    pub shiny: bool,
    pub retourne: bool,
}

impl Sprite {
    pub fn lire(texte: &str) -> Result<Self, ErreurSprite> {
        let mut section = "";
        let mut palette = Vec::new();
        let mut shiny = None;
        let mut dessin = Vec::new();
        for ligne in texte.lines() {
            let ligne = ligne.trim_end();
            if ligne.is_empty() || ligne.starts_with('#') {
                continue;
            }
            match (section, ligne) {
                ("palette", "shiny") => {
                    shiny = Some(Vec::new());
                    section = ligne;
                }
                ("", "palette") | ("palette" | "shiny", "dessin") => section = ligne,
                ("palette", _) => palette.push(lire_couleur(ligne)?),
                ("shiny", _) => shiny
                    .get_or_insert_with(Vec::new)
                    .push(lire_couleur(ligne)?),
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
        if shiny
            .as_ref()
            .is_some_and(|shiny| caracteres(&palette) != caracteres(shiny))
        {
            return Err(ErreurSprite::PalettesDifferentes);
        }
        if palette.len() > COULEURS_MAX {
            return Err(ErreurSprite::TropDeCouleurs);
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

    #[cfg(test)]
    fn a_un_shiny(&self) -> bool {
        self.shiny.is_some()
    }

    /// La palette shiny, si elle est demandée et que le sprite en a une.
    fn palette(&self, shiny: bool) -> &Palette {
        match &self.shiny {
            Some(palette) if shiny => palette,
            _ => &self.palette,
        }
    }

    /// Le PNG agrandi de ce seul sprite.
    #[cfg(test)]
    fn png(&self, shiny: bool) -> Vec<u8> {
        png_cote_a_cote(&[Some(Case {
            sprite: self,
            shiny,
            retourne: false,
        })])
    }
}

/// Les sprites côte à côte, de gauche à droite, dans un PNG agrandi, à
/// palette : l'indice 0 est transparent, les suivants sont les couleurs de
/// chaque sprite dans l'ordre de son fichier. Une case vide reste
/// transparente.
pub fn png_cote_a_cote(cases: &[Option<Case<'_>>]) -> Vec<u8> {
    debug_assert!(cases.len() <= CASES_MAX, "au plus {CASES_MAX} sprites");
    let largeur = cases.len() * COTE + cases.len().saturating_sub(1) * ECART;
    let mut grille = vec![vec![0_u8; largeur]; COTE];
    let mut plte = vec![0, 0, 0];
    let mut trns = vec![0];
    for (position, case) in cases.iter().enumerate() {
        let Some(case) = case else {
            continue;
        };
        let premier = trns.len();
        let palette = case.sprite.palette(case.shiny);
        for (_, couleur) in palette {
            plte.extend_from_slice(couleur);
            trns.push(255);
        }
        let gauche = position * (COTE + ECART);
        for (ligne, caracteres) in grille.iter_mut().zip(&case.sprite.dessin) {
            for (x, c) in caracteres.iter().enumerate() {
                let Some(rang) = palette.iter().position(|(k, _)| k == c) else {
                    continue;
                };
                let x = if case.retourne { COTE - 1 - x } else { x };
                ligne[gauche + x] = (premier + rang) as u8;
            }
        }
    }
    let mut lignes = Vec::with_capacity(COTE * ECHELLE * (largeur * ECHELLE + 1));
    for ligne in &grille {
        let pixels: Vec<u8> = ligne
            .iter()
            .flat_map(|indice| std::iter::repeat_n(*indice, ECHELLE))
            .collect();
        for _ in 0..ECHELLE {
            // Filtre 0 : la ligne telle quelle.
            lignes.push(0);
            lignes.extend_from_slice(&pixels);
        }
    }
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&((largeur * ECHELLE) as u32).to_be_bytes());
    ihdr.extend_from_slice(&((COTE * ECHELLE) as u32).to_be_bytes());
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

/// Le sprite de l'espèce, s'il est dessiné : celle d'un monstre, ou celle
/// d'un héros ou d'un humain en combat.
pub fn sprite(espece: &EspeceId) -> Option<Sprite> {
    let (_, texte) = SPRITES
        .iter()
        .chain(&SPRITES_DES_HUMAINS)
        .find(|(id, _)| *id == espece.as_str())?;
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
        for (id, texte) in SPRITES.iter().chain(&SPRITES_DES_HUMAINS) {
            Sprite::lire(texte).unwrap_or_else(|erreur| panic!("{id} : {erreur}"));
        }
    }

    /// Chaque espèce du contenu a son sprite, et aucun sprite ne reste sans
    /// espèce (section 5.1).
    #[test]
    fn chaque_espece_a_son_sprite() {
        let contenu = bersemon_core::contenu::ContenuV1::new();
        let especes: Vec<&str> = contenu
            .especes()
            .iter()
            .map(|espece| espece.id().as_str())
            .collect();
        let dessinees: Vec<&str> = especes_dessinees().collect();
        assert_eq!(dessinees, especes);
        for espece in contenu.especes() {
            let sprite = sprite(espece.id()).unwrap_or_else(|| panic!("{}", espece.id().as_str()));
            assert!(sprite.a_un_shiny(), "{}", espece.id().as_str());
        }
    }

    /// Chaque classe et chaque humain de l'IA ont leur sprite, sans palette
    /// shiny, et aucun sprite d'humain ne reste sans combattant (section
    /// 5.1).
    #[test]
    fn chaque_humain_a_son_sprite() {
        use bersemon_core::catalogue::Catalogue;
        use bersemon_core::heros::Classe;

        let contenu = bersemon_core::contenu::ContenuV1::new();
        let attendus: Vec<EspeceId> = Classe::TOUTES
            .iter()
            .map(|classe| classe.espece_du_combattant())
            .chain(
                contenu
                    .zones()
                    .iter()
                    .filter_map(|zone| zone.humain())
                    .map(|humain| humain.id().espece_du_combattant()),
            )
            .chain(
                contenu
                    .fosse()
                    .adversaires()
                    .iter()
                    .map(|adversaire| adversaire.id().espece_du_combattant()),
            )
            .collect();
        let dessines: Vec<EspeceId> = SPRITES_DES_HUMAINS
            .iter()
            .map(|(id, _)| EspeceId::new(*id))
            .collect();
        assert_eq!(dessines, attendus);
        for espece in &attendus {
            let sprite = sprite(espece).unwrap_or_else(|| panic!("{}", espece.as_str()));
            assert!(!sprite.a_un_shiny(), "{}", espece.as_str());
            // Aucune espèce de monstre ne porte le même identifiant.
            assert!(contenu.espece(espece).is_none(), "{}", espece.as_str());
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
        for (id, _) in SPRITES.iter().chain(&SPRITES_DES_HUMAINS) {
            let espece = EspeceId::new(*id);
            let sprite = sprite(&espece).unwrap();
            for shiny in [false, true]
                .into_iter()
                .take(1 + usize::from(sprite.a_un_shiny()))
            {
                std::fs::write(
                    dossier.join(nom_de_fichier(&espece, shiny)),
                    sprite.png(shiny),
                )
                .unwrap();
            }
        }
        // Deux exemples d'images composées : un combat, le choix de la
        // classe.
        let guerrier = sprite(&EspeceId::new("heros-guerrier")).unwrap();
        let loup = sprite(&EspeceId::new("loup")).unwrap();
        let case = |sprite, retourne| {
            Some(Case {
                sprite,
                shiny: false,
                retourne,
            })
        };
        std::fs::write(
            dossier.join("exemple-combat.png"),
            png_cote_a_cote(&[case(&guerrier, true), case(&loup, false)]),
        )
        .unwrap();
        let classes: Vec<Sprite> = ["guerrier", "mage", "templier", "assassin"]
            .iter()
            .map(|classe| sprite(&EspeceId::new(format!("heros-{classe}"))).unwrap())
            .collect();
        let cases: Vec<Option<Case<'_>>> =
            classes.iter().map(|sprite| case(sprite, false)).collect();
        std::fs::write(dossier.join("exemple-classes.png"), png_cote_a_cote(&cases)).unwrap();
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

    /// En combat, le combattant du joueur, à gauche, est retourné vers
    /// l'adversaire (section 5.1).
    #[test]
    fn deux_sprites_face_a_face() {
        let dessin = |ligne: &str| vec![ligne; COTE].join("\n");
        let gauche = Sprite::lire(&format!(
            "palette\na 102030\nshiny\na 405060\ndessin\n{}\n",
            dessin(&format!("a{}", ".".repeat(COTE - 1)))
        ))
        .unwrap();
        let droite = Sprite::lire(&format!(
            "palette\nb 708090\ndessin\n{}\n",
            dessin(&format!("b{}", ".".repeat(COTE - 1)))
        ))
        .unwrap();
        assert!(!droite.a_un_shiny());
        let png = png_cote_a_cote(&[
            Some(Case {
                sprite: &gauche,
                shiny: true,
                retourne: true,
            }),
            Some(Case {
                sprite: &droite,
                shiny: true,
                retourne: false,
            }),
        ]);
        let largeur = (2 * COTE + ECART) * ECHELLE;
        assert_eq!(&png[16..20], &(largeur as u32).to_be_bytes());
        assert_eq!(&png[20..24], &144_u32.to_be_bytes());
        // PLTE : le transparent, la couleur shiny du premier, puis celle du
        // second, qui n'a pas de shiny.
        assert_eq!(&png[33..37], &9_u32.to_be_bytes());
        assert_eq!(&png[37..41], b"PLTE");
        assert_eq!(&png[41..50], &[0, 0, 0, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90]);
        // Première ligne, après les blocs IHDR, PLTE, tRNS, l'en-tête zlib et
        // celui du bloc stocké : le filtre, puis les pixels.
        let debut = 8 + 25 + 21 + 15 + 8 + 2 + 5;
        assert_eq!(&png[debut - 4 - 2 - 5..debut - 2 - 5], b"IDAT");
        let ligne = &png[debut + 1..debut + 1 + largeur];
        let colonne = |x: usize| ligne[x * ECHELLE];
        // Le premier, retourné, a son pixel au bord droit de sa case ; le
        // second, au bord gauche de la sienne.
        assert_eq!(colonne(0), 0);
        assert_eq!(colonne(COTE - 1), 1);
        assert_eq!(colonne(COTE), 0);
        assert_eq!(colonne(COTE + ECART), 2);
        // Une case vide reste transparente.
        let seul = png_cote_a_cote(&[
            None,
            Some(Case {
                sprite: &droite,
                shiny: false,
                retourne: false,
            }),
        ]);
        assert_eq!(&seul[16..20], &(largeur as u32).to_be_bytes());
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
        assert_eq!(
            Sprite::lire("palette\nb 102030\n"),
            Err(ErreurSprite::SectionManquante)
        );
        // Sans palette shiny : un humain.
        assert!(Sprite::lire(&correct.replace("shiny\nb 405060\n", "")).is_ok());
        let couleurs: String = (0..=COULEURS_MAX)
            .map(|rang| format!("{} 000000\n", char::from(b'0' + rang as u8)))
            .collect();
        assert_eq!(
            Sprite::lire(&format!("palette\n{couleurs}dessin\n{dessin}\n")),
            Err(ErreurSprite::TropDeCouleurs)
        );
    }
}
