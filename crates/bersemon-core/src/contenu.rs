//! Contenu de la V1, tel que validé dans docs/contenu-v1.md : attaques,
//! espèces, zones, piste de l'Apôtre et textes de l'histoire, puis, après la
//! V1, capsules d'attaque. Toute modification se reporte d'abord dans ce
//! document.
//!
//! Ce contenu vit dans le noyau pour être partagé tel quel par la console et
//! le futur bot Discord. Les règles, elles, n'en dépendent pas : elles le
//! reçoivent de l'appelant, comme le hasard.

use crate::annexe::Annexe;
use crate::butin::ButinRare;
use crate::legendaire::{Legendaire, ZoneLegendaire};
use crate::peche::CanneAPeche;
use std::ops::RangeInclusive;
use std::time::Duration;

use crate::ame::{Gardien, MarchandId, MarchandMort};
use crate::amelioration::{
    Amelioration, PlanDAmelioration, PlanDeFabrication, PlanDesFilets, PlanDuCharme,
};
use crate::appat::Appat;
use crate::arene::{AdversaireDeLArene, FosseAuxDamnes, PALIERS_DE_LA_FOSSE, RecompenseDeLArene};
use crate::atelier::AtelierDAnsgar;
use crate::attaque::{Attaque, AttaqueId, Categorie, EffetSecondaire};
use crate::beherit::Invocation;
use crate::booster::Booster;
use crate::capsule::Capsule;
use crate::catalogue::Catalogue;
use crate::equipement::{ObjetDEquipement, ObjetDEquipementId, SorteDObjet};
use crate::espece::{AttaqueApprise, Espece, EspeceId, Rarete, Role};
use crate::evolution::{ConditionEvolution, Evolution};
use crate::expedition::{ParametresCoffre, Tresor};
use crate::expedition_automatique::ParametresExpeditionAutomatique;
use crate::filet::Filet;
use crate::habilete::Habilete;
use crate::heros::Classe;
use crate::humain::{Humain, HumainId, IaHumain};
use crate::inventaire::Inventaire;
use crate::jour_et_nuit::{Activite, JourOuNuit};
use crate::meteo::Meteo;
use crate::niveau::Niveau;
use crate::objet_d_experience::ObjetDExperience;
use crate::peche::{ParametresPeche, Pecheur};
use crate::pierre::PierreDEvolution;
use crate::quete_du_camp::{
    DemandeDeQuete, Exploration, ObjetDeQuete, OuvertureDeQuete, Personnage, Quete, QueteId,
};
use crate::recompenses_du_codex::{PalierDuCodex, RecompenseDuCodex};
use crate::ressource::{Ressource, RessourceId};
use crate::soin::Soin;
use crate::stats::Stats;
use crate::statut::Statut;
use crate::succes::{ConditionDeSucces, CouleurDuHeros, Succes};
use crate::tour::{GardienAttitre, TourSansFond};
use crate::traque::{Apotre, Chapitre, Indice, Piste};
use crate::tutoriel::Conseil;
use crate::types::{Type, TypesMonstre};
use crate::zone::{EspeceRare, Repartition, Zone, ZoneId};

use Categorie::{Physique, Speciale};
use Statut::{Paralysie, Poison};

/// Affichée après le choix de la classe (section 6), suivie du paragraphe
/// propre à la classe, `fin_de_l_introduction`.
pub const INTRODUCTION: &str = "Le royaume s'est éteint sous un ciel de cendre. Sur les routes que les hommes ont désertées, les bêtes ont repris leurs droits, et des choses plus anciennes rôdent au bord des bois.

Ces choses ont un nom : les Apôtres. Autrefois des hommes, ils ont sacrifié ce qu'ils aimaient pour une chair de monstre, et ils servent un maître que nul n'a vu en face, le Démiurge. Une nuit, ses Apôtres ont fondu sur les vôtres. Vous êtes le seul à vous être relevé.

Vous atteignez le camp de base, dernier feu allumé avant les terres sauvages. Ceux qui survivent ici ne le doivent pas à leur seule lame : ils se lient aux créatures, les capturent au filet et les mènent au combat. Pour atteindre le Démiurge, il faudra abattre ses Apôtres, un par un.";

/// Le gardien confie au héros le monstre attitré de sa classe (Q67).
pub fn fin_de_l_introduction(classe: Classe) -> &'static str {
    match classe {
        Classe::Guerrier => {
            "Le gardien du camp jauge votre lame, puis ouvre une cage : un crapaud massif s'en extrait, la peau couverte de cicatrices. Il frappe fort et ne recule pas. Il partagera votre route."
        }
        Classe::Mage => {
            "Le gardien du camp remarque la cendre sur vos doigts, puis ouvre une cage : une salamandre s'en glisse, et l'air tremble autour d'elle. Ses flammes répondront aux vôtres. Elle partagera votre route."
        }
        Classe::Templier => {
            "Le gardien du camp observe votre bouclier, puis ouvre une cage : une mandragore en sort à pas lents, ses racines tressées comme une armure. Elle encaisse ce qui en abattrait d'autres. Elle partagera votre route."
        }
        Classe::Assassin => {
            "Le gardien du camp remarque la lame cachée dans votre manche, puis ouvre une cage : un chat noir s'en glisse sans un bruit, les yeux luisants. Il frappe avant qu'on le voie, mais ne supporte pas les coups. Il partagera votre route."
        }
    }
}

// Chaque indice s'affiche après son numéro, d'où la minuscule initiale.
const INDICE_1: &str = "au bout de la Lisière, des cordes neuves pendent encore à un chêne mort. Les traces d'une procession s'enfoncent vers le sud, là où la forêt se noie : le Marais des Pendus. Quelque chose y règne.";

const INDICE_2: &str = "le bourreau vaincu a parlé : son maître, le Baron pendu, s'est retiré au fond de la Vallée des Brumes, au-delà du Marais, là où les morts ne dorment pas.";

const INDICE_3: &str = "parmi les reliques du Baron, une lettre scellée du même sceau, signée de l'Abbesse : à l'est, au-delà du Bourg des Lamentations, l'Abbaye des Cendres brûle sans jamais s'éteindre. Une autre Apôtre y attend.";

const INDICE_4: &str = "l'inquisiteur vaincu a parlé : l'Abbesse écorchée se terre sous le chœur, dans la crypte où elle a livré ses sœurs au bûcher.";

const INDICE_5: &str = "sous l'autel de l'Abbesse, une plaque de fer gravée du même sceau : au nord, au-delà du Bois des Farfadets, le Col du Givre-Noir, où un margrave a mené son armée mourir dans la neige. Un troisième Apôtre y règne.";

const INDICE_6: &str = "le porte-étendard vaincu a parlé : le Margrave attend au sommet, dans la forteresse où ses soldats ont gelé à sa place.";

const INDICE_7: &str = "dans la forteresse du Margrave, une carte de cuir marquée du même sceau : au sud, au-delà des cols, le Désert de sel, où une ville entière a été changée en sel. Une prophétesse y attendait des nouvelles du Margrave.";

const INDICE_8: &str = "la Pythie vaincue a parlé avant de se taire : un autre Apôtre a reçu la marque du Démiurge. Il attend au-delà du désert, sur la côte où la mer rend ses morts : le Port des Noyés.";

// Port des Noyés et Mines de Fer-Noir, section 44 de docs/contenu-v1.md
// (étape 163).
const INDICE_9: &str = "le contrebandier vaincu a parlé : l'Amiral noyé attend au bout de la jetée, sur le pont d'un vaisseau coulé que la marée découvre chaque nuit.";

const INDICE_10: &str = "dans la cabine de l'Amiral, un livre de bord scellé du même sceau : à l'est, par-delà la Mer des Naufrages, les Mines de Fer-Noir, où les mineurs creusent encore après leur mort. Un cinquième Apôtre y règne. Pour traverser la mer, il faudra un bateau : Gunther saura le construire.";

const INDICE_11: &str = "le contremaître vaincu a parlé : le Prévôt des fosses juge les damnés au fond du dernier puits, là où le fer noir saigne.";

// Sylve écarlate et Catacombes de Sainte-Ombre, section 45 de
// docs/contenu-v1.md (étape 165).
const INDICE_12: &str = "dans la geôle du Prévôt, un registre de condamnés scellé du même sceau : au nord, au-delà de la Grève des Ossements, la Sylve écarlate, où les arbres boivent le sang des voyageurs. Une sixième Apôtre y tient sa cour.";

const INDICE_13: &str = "la chasseresse vaincue a parlé : la Duchesse sanguine attend au cœur de la Sylve, dans un manoir que les ronces ont dévoré.";

const INDICE_14: &str = "parmi les parures de la Duchesse, une bague gravée du même sceau : au-delà des Champs de Pavots noirs, sous la ville morte de Sainte-Ombre, des catacombes où un évêque bénit encore les os. Un septième Apôtre y prêche.";

const INDICE_15: &str = "le fossoyeur vaincu a parlé : l'Évêque des os officie au fond des catacombes, dans une chapelle bâtie de crânes.";

// Plaine du Massacre et Haute-Cité déchue, section 46 de
// docs/contenu-v1.md (étape 166).
const INDICE_16: &str = "sous l'autel de l'Évêque, une liste de recrues scellée du même sceau : à l'ouest, au-delà des Remparts de Grisval, la Plaine du Massacre, où deux armées se battent encore après leur mort. Un huitième Apôtre y commande.";

const INDICE_17: &str = "le capitaine mercenaire vaincu a parlé : le Connétable boucher dresse son camp au centre de la plaine, sur une colline de cadavres.";

const INDICE_18: &str = "dans la tente du Connétable, un ordre de marche scellé du même sceau : au-delà de la plaine et de la Forêt pétrifiée, la Haute-Cité déchue, dont les tours regardent encore le ciel. Un neuvième Apôtre y gouverne.";

const INDICE_19: &str = "l'archiviste vaincu a parlé : le Chancelier aux mille yeux siège au sommet de la plus haute tour, dans une salle tapissée de regards.";

// Gorge de Braise et Pics Foudroyés, section 47 de docs/contenu-v1.md
// (étape 167).
const INDICE_20: &str = "dans les archives du Chancelier, une carte scellée du même sceau : au sud, par le Sanctuaire d'Asphodèle caché sous la terre, la Gorge de Braise, où la terre saigne du feu. Un dixième Apôtre y règne.";

const INDICE_21: &str = "le pyromancien vaincu a parlé : le Prince de braise trône au fond de la gorge, sur un siège de lave refroidie.";

const INDICE_22: &str = "dans les cendres du Prince, une lettre scellée du même sceau : au-delà de la gorge et du Glacier des Âmes, les Pics Foudroyés, où l'orage ne s'arrête jamais. Un onzième Apôtre y monte la garde.";

const INDICE_23: &str = "le héraut de l'orage vaincu a parlé : le Sénéchal des orages attend au sommet du plus haut pic, là où tombe la foudre.";

// Palais de Chair et Puits du Monde, section 48 de docs/contenu-v1.md
// (étape 168).
const INDICE_24: &str = "sous la lance du Sénéchal, une invitation scellée du même sceau : au cœur du royaume, au-delà du Cimetière des Géants, le Palais de Chair, dont les murs respirent. Un douzième Apôtre y tient sa cour.";

const INDICE_25: &str = "la favorite vaincue a parlé : la Reine-mère de chair repose dans la salle du trône, au milieu de sa couvée.";

const INDICE_26: &str = "dans la couche de la Reine-mère, une carte scellée du même sceau : sous le palais, au-delà de la Faille du Néant, le Puits du Monde descend jusqu'au fond des choses. Le dernier Apôtre y attend.";

const INDICE_27: &str = "le chevalier-crâne vaincu a parlé : le Roi sans visage règne au fond du Puits, sur un trône taillé dans la dernière pierre du monde.";

/// Indice de Faille du Néant, trouvé au bout de la première expédition menée
/// au bout (section 9.4, étape 209, texte proposé par Claude, Q124).
const INDICE_DE_LA_FAILLE: &str = "au bord de la faille, un anneau de chevalier porte le même sceau. Plus bas, le néant rejoint le Puits du Monde.";

/// Indice de Cimetière des Géants, trouvé au bout de la première expédition menée
/// au bout (section 9.4, étape 209, texte proposé par Claude, Q124).
const INDICE_DU_CIMETIERE: &str = "entre les côtes d'un géant, une couronne rouillée porte le même sceau. Une odeur de chair monte de l'est : le Palais de Chair n'est plus loin.";

/// Indice de Glacier des Âmes, trouvé au bout de la première expédition menée
/// au bout (section 9.4, étape 209, texte proposé par Claude, Q124).
const INDICE_DU_GLACIER: &str = "dans la glace, un mort aux yeux ouverts serre un médaillon au même sceau. Au-dessus du glacier grondent les Pics Foudroyés.";

/// Indice de Sanctuaire d'Asphodèle, trouvé au bout de la première expédition menée
/// au bout (section 9.4, étape 208, texte proposé par Claude, Q124).
const INDICE_DU_SANCTUAIRE: &str = "au fond du sanctuaire, sous le soleil qui brille sous la terre, un autel porte le même sceau. Une galerie brûlante en part vers le sud, jusqu'à la Gorge de Braise.";

/// Indice de Forêt pétrifiée, trouvé au bout de la première expédition menée
/// au bout (section 9.4, étape 208, texte proposé par Claude, Q124).
const INDICE_DE_LA_FORET: &str = "entre les arbres de pierre, un pèlerin pétrifié tient une tablette au même sceau. Son regard figé fixe les tours de la Haute-Cité déchue.";

/// Indice de Remparts de Grisval, trouvé au bout de la première expédition menée
/// au bout (section 9.4, étape 208, texte proposé par Claude, Q124).
const INDICE_DES_REMPARTS: &str = "sur le plus haut rempart, un étendard déchiré porte le même sceau. De là, on voit à l'ouest la Plaine du Massacre, où deux armées se battent encore.";

/// Indice de Champs de Pavots noirs, trouvé au bout de la première expédition menée
/// au bout (section 9.4, étape 208, texte proposé par Claude, Q124).
const INDICE_DES_PAVOTS: &str = "au milieu des pavots, une stèle porte le même sceau. Au-delà, la route descend vers les clochers muets de Sainte-Ombre et ses catacombes.";

/// Indice de Grève des Ossements, trouvé au bout de la première expédition menée
/// au bout (section 9.4, étape 207, texte proposé par Claude, Q124).
const INDICE_DE_LA_GREVE: &str = "sur la grève, les os dessinent une flèche marquée du même sceau. Elle pointe vers le nord, vers les arbres rouges de la Sylve écarlate.";

/// Indice de Mer des Naufrages, trouvé au bout de la première expédition menée
/// au bout (section 9.4, étape 207, texte proposé par Claude, Q124).
const INDICE_DE_LA_MER: &str = "au large, une épave porte le même sceau sur sa figure de proue. Ses cartes mènent à une côte de fer, à l'est : les Mines de Fer-Noir.";

/// Indice de Bois des Farfadets, trouvé au bout de la première expédition menée
/// au bout (section 9.4, étape 206, texte proposé par Claude, Q124).
const INDICE_DU_BOIS: &str = "au cœur du bois, de petits rires vous guident jusqu'à un sentier de pierres blanches, marqué du même sceau. Il monte vers le nord, vers les neiges du Col du Givre-Noir.";

/// Indice de Bourg des Lamentations, trouvé au bout de la première expédition menée
/// au bout (section 9.4, étape 206, texte proposé par Claude, Q124).
const INDICE_DU_BOURG: &str = "au bout du bourg, sur la porte de l'église, quelqu'un a cloué le même sceau. Le chemin de l'est part de là, vers la fumée de l'Abbaye des Cendres.";

/// Au bout de la première expédition menée au bout, quand le joueur porte
/// l'âme de Bertrand (section 14.2).
pub const MALVOISE_ET_L_AME: &str = "Sur le chemin du camp, un homme en robe noire vous attend, un crâne posé au creux du bras. \"Je t'ai vu recueillir cette âme. Elle m'intéresse, si tu ne sais pas quoi en faire. Je suis Malvoise, et les marchands morts sur ces routes ont encore des choses à vendre.\" Il vous suit jusqu'au camp et s'y installe.";

/// Au bout de la première expédition menée au bout, sans âme.
pub const MALVOISE_SANS_AME: &str = "Sur le chemin du camp, un homme en robe noire vous attend, un crâne posé au creux du bras. \"Je suis Malvoise. Les marchands meurent vite sur ces routes : rapporte-moi leur âme, et je te vendrai ce qu'ils vendaient de leur vivant.\" Il vous suit jusqu'au camp et s'y installe.";

/// Bertrand est abattu à l'étape 4 de la Lisière par le brigand qui attend
/// à l'étape 5 (section 14.2).
const ETAPE_DE_BERTRAND: u32 = 4;

// La bête qui attaque à la place de Bertrand ou garde Ulrich n'est pas
// nommée : le combat la présente aussitôt, et son nom ne dit pas son genre.
const AVANT_BERTRAND: &str = "Un cri déchire la Lisière : un brigand abat un colporteur sous vos yeux, lui arrache sa bourse et s'enfuit vers la sortie du bois. Une bête qui le suivait se tourne vers vous.";

// Le joueur ne connaît pas encore Malvoise (section 14.2).
const APRES_BERTRAND: &str = "L'âme de Bertrand, colporteur, flotte au-dessus de son corps. Vous la recueillez, sans trop savoir ce que c'est.";

const AVANT_GRETE: &str = "Une marchande titube vers vous entre les roseaux, la peau verdie, sa carriole renversée derrière elle. Elle n'est plus tout à fait morte.";

const APRES_GRETE: &str = "Le corps de Grete s'effondre enfin. Son âme flotte au-dessus de la vase : vous la recueillez pour Malvoise.";

const AVANT_ULRICH: &str = "Au pied d'un gibet, un alchimiste pend au bout d'une corde, ses fioles brisées dans l'herbe. Un monstre garde le corps et se jette sur vous.";

const APRES_ULRICH: &str =
    "L'âme d'Ulrich, alchimiste, s'échappe enfin de la corde. Vous la recueillez pour Malvoise.";

const AVANT_HILDE: &str = "Sous une dune, une caravane ensevelie : la marchande Hilde est morte assise contre sa charrette, une sacoche de baumes serrée contre elle. Un monstre rôde autour du corps et se jette sur vous.";

const APRES_HILDE: &str = "L'âme de Hilde, marchande de baumes, s'échappe enfin de la caravane. Vous la recueillez pour Malvoise.";

const AVANT_LE_BRIGAND: &str = "Au sortir de la Lisière, un brigand vous barre la route, un loup et un corbeau à ses côtés. Ses bêtes ont faim, et lui aussi.";

const APRES_LE_BRIGAND: &str =
    "Le brigand détale dans les fourrés, abandonnant ses bêtes et sa bourse.";

const AVANT_LE_BOURREAU: &str = "Au cœur du Marais, sous un gibet chargé de cordes, un bourreau encagoulé aiguise sa hache. Trois bêtes attendent ses ordres. Il sert le Baron pendu, et il ne vous laissera pas passer.";

const APRES_LE_BOURREAU: &str = "Le bourreau s'effondre dans la vase, sa hache brisée.";

/// Ansgar attend à l'étape 2 du Marais (docs/contenu-v1.md, section 15).
const ETAPE_D_ANSGAR: u32 = 2;

const AVANT_ANSGAR: &str = "Au bord d'une mare noire, un vieil homme s'arc-boute sur sa canne. Ce qu'il vient de ferrer surgit de l'eau et se jette sur vous.";

const APRES_ANSGAR: &str = "Le vieil homme reprend son souffle : Ansgar, pêcheur, chassé de sa cabane par ce qui remonte des eaux du Marais. Il vous suit jusqu'au camp et y tend ses lignes.";

// Après la victoire sur le Bourreau (section 13.4).
const LIBERATION_DE_GUNTHER: &str = "Derrière le gibet, un homme enchaîné relève la tête : Gunther, forgeron, que le bourreau gardait pour forger ses chaînes. Libéré, il vous suit jusqu'au camp et y installe sa forge.";

const AVANT_L_INQUISITEUR: &str = "Sur le parvis de l'abbaye, un inquisiteur en robe rouge vous attend, entouré de trois bêtes. Il brûle au nom de l'Abbesse, et il compte bien vous brûler aussi.";

const APRES_L_INQUISITEUR: &str =
    "L'inquisiteur s'effondre sur les dalles, son livre de prières en cendres.";

const AVANT_LE_PORTE_ETENDARD: &str = "Au milieu du col, un soldat couvert de givre tient encore l'étendard d'une armée morte. Trois bêtes sortent de la tempête à son signal.";

const APRES_LE_PORTE_ETENDARD: &str =
    "Le porte-étendard tombe à genoux ; l'étendard gelé se brise dans la neige.";

const AVANT_LA_PYTHIE: &str = "Au cœur du désert, parmi les ruines d'une ville changée en sel, une femme aux yeux blancs vous attend. \"Je t'ai vu venir depuis le col.\" Trois bêtes se dressent dans le sable.";

const APRES_LA_PYTHIE: &str = "La Pythie s'effondre dans le sel, un dernier mot aux lèvres.";

const AVANT_LE_CONTREBANDIER: &str = "Sur le quai, entre des caisses éventrées, un contrebandier compte ses pièces à la lueur d'une lanterne. Il siffle : deux bêtes surgissent de l'eau noire. Personne n'approche de la jetée sans payer.";

const APRES_LE_CONTREBANDIER: &str =
    "Le contrebandier s'effondre sur les pavés mouillés, sa bourse répandue dans une flaque.";

const AVANT_LE_CONTREMAITRE: &str = "Au fond de la galerie, un contremaître au fouet de chaîne surveille des mineurs qui ne respirent plus. Trois bêtes sortent de l'ombre des boisages à son signal.";

const APRES_LE_CONTREMAITRE: &str =
    "Le contremaître tombe à genoux, et les mineurs morts lâchent enfin leurs pioches.";

const AVANT_L_AMIRAL: &str = "Au bout de la jetée, la marée se retire sur l'épave d'un vaisseau de guerre. Sur le pont, un officier aux poumons pleins d'eau se dresse : l'Amiral noyé, Apôtre du Démiurge, qui a mené son équipage par le fond. Il n'y aura pas de fuite.";

const EPILOGUE_DE_L_AMIRAL: &str = "L'Amiral noyé s'effondre sur le pont, et la mer reprend son navire. Le même sceau, une main ouverte, est gravé sur la barre. Quatre Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

const AVANT_LE_PREVOT: &str = "Au fond du dernier puits, sur un trône taillé dans le fer noir, un juge enchaîné à ses condamnés lève la tête : le Prévôt des fosses, Apôtre du Démiurge. Il n'y aura pas de fuite.";

const EPILOGUE_DU_PREVOT: &str = "Le Prévôt des fosses s'effondre, et ses chaînes se brisent avec lui. Le même sceau, une main ouverte, est gravé dans le fer de son trône. Cinq Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

// La carte d'une annexe, trouvée dans un coffre de sa zone (section 9.9).
const DECOUVERTE_DE_L_EPAVE: &str = "Au fond du coffre, une carte marine détrempée marque d'une croix une épave au large du port, là où une bête immense aurait sombré avec un navire.";

const DECOUVERTE_DU_FILON: &str = "Au fond du coffre, un plan de mine griffonné montre une galerie murée, où le filon brille d'un rouge de sang.";

const AVANT_LA_CHASSERESSE: &str = "Une flèche se plante dans l'écorce, à un doigt de votre visage. Entre les troncs rouges, une chasseresse encoche déjà la suivante. Deux bêtes surgissent des fourrés à son sifflement.";

const APRES_LA_CHASSERESSE: &str =
    "La chasseresse tombe à genoux parmi les feuilles rouges, son arc brisé.";

const AVANT_LE_FOSSOYEUR: &str = "Entre deux rangées de niches funéraires, un fossoyeur appuyé sur sa pelle murmure des noms. Les morts qu'il a enterrés se relèvent, et trois bêtes sortent des tombes.";

const APRES_LE_FOSSOYEUR: &str = "Le fossoyeur s'effondre dans la fosse qu'il venait de creuser.";

const AVANT_LA_DUCHESSE: &str = "Au cœur du manoir dévoré par les ronces, une femme en robe de bal boit dans une coupe pleine de sang. La Duchesse sanguine, Apôtre du Démiurge, repose la coupe et sourit. Il n'y aura pas de fuite.";

const EPILOGUE_DE_LA_DUCHESSE: &str = "La Duchesse sanguine s'effondre, et les ronces se flétrissent autour d'elle. Le même sceau, une main ouverte, est gravé sur sa coupe. Six Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

const AVANT_L_EVEQUE: &str = "Dans la chapelle de crânes, un prélat aux os nus sous sa chasuble lève sa crosse : l'Évêque des os, Apôtre du Démiurge, bénit son troupeau de morts. Il n'y aura pas de fuite.";

const EPILOGUE_DE_L_EVEQUE: &str = "L'Évêque des os se disloque au pied de son autel, et le silence retombe sur les catacombes. Le même sceau, une main ouverte, est gravé sur sa crosse. Sept Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

const DECOUVERTE_DE_L_AUTEL: &str = "Au fond du coffre, un parchemin taché de sang montre le chemin d'une clairière cachée, où un autel de ronces attend ses offrandes.";

const DECOUVERTE_DE_LA_CRYPTE: &str = "Au fond du coffre, une clé de bronze et un plan jauni mènent à une crypte scellée, où reposeraient les saints de Sainte-Ombre.";

const AVANT_LE_CAPITAINE: &str = "Au milieu des lances plantées dans la boue, un capitaine mercenaire compte ses gages sur le cadavre d'un chevalier. Il range sa bourse, lève sa hache, et trois bêtes de guerre se dressent derrière lui.";

const APRES_LE_CAPITAINE: &str =
    "Le capitaine mercenaire s'écroule sur sa bourse, que personne ne viendra réclamer.";

const AVANT_LE_CONNETABLE: &str = "Au sommet d'une colline de cadavres, un colosse en armure rouillée essuie son couperet sur une bannière déchirée. Le Connétable boucher, Apôtre du Démiurge, a fait de la guerre un abattoir. Il n'y aura pas de fuite.";

const EPILOGUE_DU_CONNETABLE: &str = "Le Connétable boucher s'effondre sur sa colline de cadavres, et les armées mortes se couchent enfin. Le même sceau, une main ouverte, est gravé sur son couperet. Huit Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

const DECOUVERTE_DU_TERTRE: &str = "Au fond du coffre, une carte d'état-major indique un tertre où les héros de l'ancienne guerre ont été enterrés avec leurs montures.";

const AVANT_L_ARCHIVISTE: &str = "Entre deux rayonnages effondrés, un archiviste aux yeux cousus feuillette un registre. Il referme le livre d'un coup sec : votre nom vient d'y être écrit. Trois créatures surgissent des étagères.";

const APRES_L_ARCHIVISTE: &str =
    "L'archiviste s'effondre, et son registre se referme sur une page blanche.";

const AVANT_LE_CHANCELIER: &str = "Dans la salle tapissée de regards, une silhouette drapée de pourpre lève la tête : sous sa capuche, des centaines d'yeux s'ouvrent à la fois. Le Chancelier aux mille yeux, Apôtre du Démiurge, sait déjà tout de vous. Il n'y aura pas de fuite.";

const EPILOGUE_DU_CHANCELIER: &str = "Le Chancelier aux mille yeux s'effondre, et tous les regards des murs se ferment d'un coup. Le même sceau, une main ouverte, est gravé sur sa chevalière. Neuf Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

const DECOUVERTE_DE_LA_BIBLIOTHEQUE: &str = "Au fond du coffre, une clé gravée d'un œil ouvre, d'après son étiquette, la bibliothèque interdite de la Haute-Cité.";

const AVANT_LE_PYROMANCIEN: &str = "Au bord d'une coulée de lave, un pyromancien aux mains brûlées jusqu'à l'os fait danser les flammes. Il vous montre du doigt, et trois bêtes surgissent de la fournaise.";

const APRES_LE_PYROMANCIEN: &str =
    "Le pyromancien s'effondre, et les flammes qu'il tenait s'éteignent avec lui.";

const AVANT_LE_PRINCE: &str = "Sur un trône de lave refroidie, un jeune prince à la peau craquelée de braises vous regarde approcher. Le Prince de braise, Apôtre du Démiurge, sourit : tout ce qu'il touche finit en cendres. Il n'y aura pas de fuite.";

const EPILOGUE_DU_PRINCE: &str = "Le Prince de braise s'effondre, et la lave se fige autour de son trône. Le même sceau, une main ouverte, est gravé sur sa couronne. Dix Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

const DECOUVERTE_DU_COEUR: &str = "Au fond du coffre, une carte roussie par le feu indique un passage vers le cœur du volcan, là où la lave est la plus ancienne.";

const AVANT_LE_HERAUT: &str = "Sur un ressaut battu par l'orage, un héraut en armure lève une bannière foudroyée et sonne du cor. Trois bêtes répondent à son appel, portées par le vent.";

const APRES_LE_HERAUT: &str = "Le héraut s'écroule, et sa bannière s'envole dans la tempête.";

const AVANT_LE_SENECHAL: &str = "Au sommet du plus haut pic, un chevalier vêtu d'orage tient une lance où la foudre vient mourir. Le Sénéchal des orages, Apôtre du Démiurge, abaisse sa visière. Il n'y aura pas de fuite.";

const EPILOGUE_DU_SENECHAL: &str = "Le Sénéchal des orages s'effondre, et pour la première fois le ciel des Pics se tait. Le même sceau, une main ouverte, est gravé sur sa lance. Onze Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

const DECOUVERTE_DE_L_AIRE: &str = "Au fond du coffre, une plume d'argent et un croquis mènent à une aire au-dessus des nuages, où se poseraient les pégases d'orage.";

const AVANT_LA_FAVORITE: &str = "Sur un lit de soie tachée, une favorite aux yeux fardés vous sourit en remplissant deux coupes. Elle laisse tomber la sienne : deux créatures sortent des tentures.";

const APRES_LA_FAVORITE: &str =
    "La favorite s'effondre sur les coussins, et son sourire ne la quitte pas.";

const AVANT_LA_REINE: &str = "Dans la salle du trône, une masse de chair coiffée d'une couronne se soulève, et cent bouches s'ouvrent à la fois. La Reine-mère de chair, Apôtre du Démiurge, couve le royaume entier. Il n'y aura pas de fuite.";

const EPILOGUE_DE_LA_REINE: &str = "La Reine-mère de chair s'affaisse, et les murs du palais cessent de respirer. Le même sceau, une main ouverte, est gravé sur son sceptre. Douze Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

const DECOUVERTE_DU_JARDIN: &str = "Au fond du coffre, un ruban de soie rouge noue une clé de jardin : derrière le palais, la Reine cultiverait des fleurs qui boivent le sang.";

const AVANT_LE_CHEVALIER_CRANE: &str = "Sur la dernière corniche avant le gouffre, un chevalier dont le heaume est un crâne immense barre le passage, l'épée plantée devant lui. Trois bêtes montent du Puits à son côté.";

const APRES_LE_CHEVALIER_CRANE: &str =
    "Le chevalier-crâne bascule dans le vide, et le Puits avale son épée.";

const AVANT_LE_ROI: &str = "Au fond du Puits, sur un trône taillé dans la dernière pierre du monde, un roi drapé d'ombre lève la tête : à la place du visage, il n'y a rien. Le Roi sans visage, dernier Apôtre du Démiurge, se lève. Il n'y aura pas de fuite.";

const EPILOGUE_DU_ROI: &str = "Le Roi sans visage s'effondre, et le Puits se tait. Le même sceau, une main ouverte, est gravé sur son trône. Les treize Apôtres sont tombés ; il ne reste que le Démiurge. Rapportez leurs âmes à Malvoise.";

const DECOUVERTE_DES_RACINES: &str = "Au fond du coffre, une lanterne de racines tressées éclaire un passage qui descend le long des racines du monde.";

const AVANT_LE_MARGRAVE: &str = "Au sommet, sur un trône de glace entouré de soldats figés, le Margrave de givre se lève. Apôtre du Démiurge, il a offert son armée au froid. Il n'y aura pas de fuite.";

const EPILOGUE_DU_MARGRAVE: &str = "Le Margrave de givre se brise sur son trône. Le même sceau, une main ouverte, est gravé dans la glace. Trois Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

const AVANT_L_ABBESSE: &str = "Au fond de la crypte, une silhouette sans peau prie devant un autel de chair. L'Abbesse écorchée, Apôtre du Démiurge, se retourne lentement. Il n'y aura pas de fuite.";

const EPILOGUE_DE_L_ABBESSE: &str = "L'Abbesse écorchée s'effondre sur l'autel. Le même sceau, une main ouverte, est gravé dans sa chair. Deux Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.";

/// Après l'épilogue, le joueur recueille l'âme de l'Apôtre vaincu (section
/// 9.4).
/// Le carnet de traque du codex : une ligne par Apôtre abattu, puis le
/// Démiurge (section 5.6).
const CARNET_DU_BARON: &str = "Baron pendu, abattu dans la Vallée des Brumes.";
const CARNET_DE_L_ABBESSE: &str = "Abbesse écorchée, abattue dans l'Abbaye des Cendres.";
const CARNET_DU_MARGRAVE: &str = "Margrave de givre, abattu dans le Col du Givre-Noir.";
const CARNET_DE_L_AMIRAL: &str = "Amiral noyé, abattu au Port des Noyés.";
const CARNET_DU_PREVOT: &str = "Prévôt des fosses, abattu dans les Mines de Fer-Noir.";
const CARNET_DE_LA_DUCHESSE: &str = "Duchesse sanguine, abattue dans la Sylve écarlate.";
const CARNET_DE_L_EVEQUE: &str = "Évêque des os, abattu dans les Catacombes de Sainte-Ombre.";
const CARNET_DU_CONNETABLE: &str = "Connétable boucher, abattu dans la Plaine du Massacre.";
const CARNET_DU_CHANCELIER: &str = "Chancelier aux mille yeux, abattu dans la Haute-Cité déchue.";
const CARNET_DU_PRINCE: &str = "Prince de braise, abattu dans la Gorge de Braise.";
const CARNET_DU_SENECHAL: &str = "Sénéchal des orages, abattu dans les Pics Foudroyés.";
const CARNET_DE_LA_REINE: &str = "Reine-mère de chair, abattue dans le Palais de Chair.";
const CARNET_DU_ROI: &str = "Roi sans visage, abattu dans le Puits du Monde.";
const CARNET_DU_DEMIURGE: &str = "Démiurge, abattu dans l'Éclipse.";

const AME_DU_BARON: &str =
    "L'âme du Baron pendu s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DE_L_ABBESSE: &str =
    "L'âme de l'Abbesse écorchée s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DU_MARGRAVE: &str =
    "L'âme du Margrave de givre s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DE_L_AMIRAL: &str =
    "L'âme de l'Amiral noyé s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DU_PREVOT: &str =
    "L'âme du Prévôt des fosses s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DE_LA_DUCHESSE: &str = "L'âme de la Duchesse sanguine s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DE_L_EVEQUE: &str =
    "L'âme de l'Évêque des os s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DU_CONNETABLE: &str =
    "L'âme du Connétable boucher s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DU_CHANCELIER: &str = "L'âme du Chancelier aux mille yeux s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DU_PRINCE: &str =
    "L'âme du Prince de braise s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DU_SENECHAL: &str = "L'âme du Sénéchal des orages s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DE_LA_REINE: &str = "L'âme de la Reine-mère de chair s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

const AME_DU_ROI: &str =
    "L'âme du Roi sans visage s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.";

// Les zones légendaires et leurs gardiens, section 49 de docs/contenu-v1.md
// (étape 169).
const AVANT_LEVIATHAN: &str = "Au plus profond de la Mer sans rivage, l'eau elle-même se soulève : un serpent immense, couvert d'écailles noires, ouvre une gueule où s'engouffrent les courants. Léviathan, maître des eaux, vous barre la route.";

const CAPTURE_DE_LEVIATHAN: &str = "Léviathan s'effondre dans un remous et se laisse approcher. Le maître des eaux vous suit désormais.";

const AVANT_BEHEMOTH: &str = "Au cœur des Terres arides, le sol tremble à chaque pas : une bête colossale aux cornes de pierre se dresse entre les dunes. Béhémoth, premier-né de la terre, charge.";

const CAPTURE_DE_BEHEMOTH: &str = "Béhémoth plie les genoux dans la poussière et baisse ses cornes. Le premier-né de la terre vous suit désormais.";

const AVANT_ZIZ: &str = "Au-dessus des nuages déchirés, une ombre couvre le soleil : un oiseau si grand que ses ailes cachent le ciel plonge vers vous. Ziz, roi des airs, pousse un cri.";

const CAPTURE_DE_ZIZ: &str = "Ziz se pose devant vous, les ailes repliées, et incline la tête. Le roi des airs vous suit désormais.";

/// L'Éclipse et le Démiurge, section 23 de docs/contenu-v1.md (étapes 88 à
/// 90).
/// Le Béhérit réclame les âmes des 13 Apôtres de la traque complète
/// (section 9.4).
const APOTRES_DE_LA_TRAQUE: usize = 13;

const FORGE_DU_BEHERIT: &str = "Malvoise dépose les âmes des Apôtres au creux de ses mains. Elles se tordent, se mêlent et durcissent en un œuf de pierre pourpre, où des yeux et une bouche s'ouvrent et se referment. \"Le Béhérit pourpre. Il t'ouvrira la porte du Démiurge, mais il réclamera son dû.\"";

// Tour sans fond, section 25 de docs/contenu-v1.md (étape 96).
const ENTREE_DANS_LA_TOUR: &str = "Au milieu des ruines se dresse le sommet d'une tour engloutie. Ses marches s'enfoncent dans la terre, et nul n'en a trouvé le fond ; plus on descend, plus les bêtes sont terribles.";
const AVANT_LE_GARDIEN: &str = "Les marches débouchent sur un palier plus large que les autres. Au centre, {monstre} garde l'escalier qui s'enfonce plus bas. Il n'y aura pas de fuite.";
/// Section 25 de docs/contenu-v1.md : le fond de la Tour (étape 204, textes
/// proposés par Claude, Q124).
const ENTREE_DANS_LES_ABYSSES: &str = "Les marches cessent au bord d'un gouffre sans lumière : les Abysses. Seules les bêtes les plus terribles rôdent ici, et plus bas vous attend la Main du Démiurge, cinq Apôtres élevés au rang de dieux.";
const AVANT_LE_COLOSSE: &str = "Un rempart barre l'escalier. Il bouge : {monstre}, un géant de pierre et d'acier, lève un bouclier haut comme une tour. Il n'y aura pas de fuite.";
const AVANT_LE_HELLEQUIN: &str = "Sur le seuil, un cor sonne dans le noir. Un cavalier d'acier couronné de bois de cerf mène la chasse des morts, et le gouffre le ronge depuis des siècles : {monstre} se met en garde. Il n'y aura pas de fuite.";
const ELUS_DE_LA_MAIN: [(&str, &str); 5] = [
    (
        "cardinal-du-vide",
        "Une voix sans bouche résonne sous votre crâne. {monstre}, premier de la Main du Démiurge, pose sur vous des yeux qui ne se ferment jamais. Il n'y aura pas de fuite.",
    ),
    (
        "dame-des-supplices",
        "Une femme drapée de chaînes sourit dans l'ombre, et chaque blessure de vos monstres se met à brûler. {monstre}, deuxième de la Main du Démiurge, s'avance. Il n'y aura pas de fuite.",
    ),
    (
        "bouffon-ecorche",
        "Un rire aigre roule dans le gouffre. {monstre}, troisième de la Main du Démiurge, danse sur des jambes sans peau. Il n'y aura pas de fuite.",
    ),
    (
        "seigneur-pestilence",
        "L'air s'épaissit, grouillant d'insectes. {monstre}, quatrième de la Main du Démiurge, déploie son corps de pourriture. Il n'y aura pas de fuite.",
    ),
    (
        "aile-de-l-eclipse",
        "Le gouffre s'assombrit encore : une éclipse se lève sous la terre. {monstre}, cinquième et dernier de la Main du Démiurge, déploie ses ailes noires. Il n'y aura pas de fuite.",
    ),
];
const REPOS_DANS_LA_TOUR: &str = "Derrière le gardien, une source claire coule entre les pierres. Vos monstres y boivent et reprennent toutes leurs forces.";

const SACRIFICE: &str = "Le Béhérit pourpre s'ouvre en un cri. {monstre} vous est arraché et disparaît dans une lumière rouge. Le ciel s'assombrit : l'Éclipse commence.";

const AVANT_LE_DEMIURGE: &str = "Sous un soleil noir, la terre s'ouvre sur un abîme de chair. Une silhouette ailée descend lentement : le Démiurge, maître des Apôtres. Il n'y aura pas de fuite.";

/// Quand la première forme du Démiurge tombe, à la place de l'arrivée d'un
/// nouvel adversaire.
pub const METAMORPHOSE_DU_DEMIURGE: &str =
    "Le Démiurge s'effondre, puis sa chair se déchire : sa forme véritable s'en arrache, immense.";

const EPILOGUE_DU_DEMIURGE: &str = "Le Démiurge se disloque dans un hurlement qui fait trembler le ciel. Le soleil noir pâlit, et le jour revient sur le royaume. Votre vengeance est accomplie ; mais d'autres choses rôdent encore au bord des bois.";

/// Les Philtres chromatiques ramassés après l'épilogue du Démiurge.
pub const RECOMPENSE_DU_DEMIURGE: &str = "Dans les cendres du Démiurge repose un Philtre chromatique, aux reflets changeants. Vous le ramassez.";

const AVANT_LE_BARON: &str = "Au fond de la Vallée, la brume se déchire sur un gibet dressé contre le ciel. Le Baron pendu, Apôtre du Démiurge, s'y balance encore, puis ouvre les yeux. Il n'y aura pas de fuite.";

const EPILOGUE: &str = "Le Baron pendu s'effondre au pied de son gibet. Sur sa poitrine, un sceau gravé d'une main ouverte : le signe du Démiurge. D'autres Apôtres portent ce sceau, au-delà des terres connues. Votre traque ne fait que commencer.";

pub fn texte_du_conseil(conseil: Conseil) -> &'static str {
    match conseil {
        Conseil::PremierCombat => {
            "Conseil : affaiblissez un monstre avant de lancer un filet, car plus il a perdu de PV, plus la capture a de chances de réussir. Fuir réussit toujours, mais le monstre adverse frappe une dernière fois."
        }
        Conseil::PremierArretEntreDeuxEtapes => {
            "Conseil : vos monstres ne récupèrent leurs PV qu'au camp. Entre deux étapes, soignez-les avec une potion, changez l'ordre de votre équipe, ou rentrez avant qu'il ne soit trop tard : une défaite vous fait perdre tout votre sac, mais pas ce qui est rangé dans le coffre du camp."
        }
        Conseil::PremierRetourAuCamp => {
            "Conseil : au camp, vos monstres sont soignés et la partie est sauvegardée. Trois monstres vous accompagnent ; les autres attendent ici, et vous pouvez les échanger à tout moment."
        }
    }
}

/// Les monstres de départ, un par classe, sont les premières espèces du
/// codex.
const NOMBRE_DE_DEPARTS: usize = 4;

/// Section 9.9 : une fois le Démiurge vaincu, chaque monstre de départ se
/// montre, pacifique et rare, dans deux annexes choisies selon son type,
/// pour que le codex se complète sans dépendre des échanges (proposé par
/// Claude, Q124).
const DEPARTS_D_APRES_LA_FIN: [(&str, &str); 8] = [
    ("coeur-du-volcan", "salamandre"),
    ("filon-ecarlate", "salamandre"),
    ("epave-du-leviathan", "crapaud"),
    ("racines-du-monde", "crapaud"),
    ("autel-des-ronces", "mandragore"),
    ("jardin-de-la-reine", "mandragore"),
    ("crypte-des-saints", "chat-noir"),
    ("bibliotheque-interdite", "chat-noir"),
];

pub struct ContenuV1 {
    especes: Vec<Espece>,
    zones: Vec<Zone>,
    piste: Piste,
    capsules: Vec<Capsule>,
    plans_d_amelioration: Vec<PlanDAmelioration>,
    plans_de_fabrication: Vec<PlanDeFabrication>,
    armes_du_champion: Vec<ObjetDEquipement>,
    plan_du_charme: PlanDuCharme,
    plans_des_filets: Vec<PlanDesFilets>,
    peche: ParametresPeche,
    /// Les recettes d'Ansgar, avec les matières de sa ligne (section 13.8).
    atelier: AtelierDAnsgar,
    invocation: Invocation,
    tour: TourSansFond,
    fosse: FosseAuxDamnes,
    recompenses_du_codex: Vec<RecompenseDuCodex>,
    /// Une par zone d'Apôtre, à partir du quatrième (section 9.9).
    annexes: Vec<Annexe>,
    /// Ouvertes à la chute du Démiurge (section 9.10).
    zones_legendaires: Vec<ZoneLegendaire>,
    /// Dans l'ordre de l'écran des succès (section 4.5).
    succes: Vec<Succes>,
    /// Les chaînes des personnages du camp, chacune dans son ordre (section
    /// 13.13).
    quetes_du_camp: Vec<Quete>,
}

impl ContenuV1 {
    /// Panique si le contenu enfreint une règle du noyau : c'est une erreur
    /// de programmation, que les tests de ce module détectent.
    pub fn new() -> Self {
        let especes: Vec<Espece> = ESPECES.iter().map(creer_espece).collect();
        let capsules: Vec<Capsule> = CAPSULES
            .iter()
            .map(|fiche| Capsule::new(creer_attaque(fiche.attaque), fiche.prix))
            .collect();
        let capsules_de = |zone: &str| {
            CAPSULES
                .iter()
                .zip(&capsules)
                .filter(|(fiche, _)| fiche.zone == zone)
                .map(|(_, capsule)| capsule.clone())
                .collect()
        };
        let trouver = |id: &str| {
            especes
                .iter()
                .find(|espece| espece.id().as_str() == id)
                .cloned()
                .unwrap_or_else(|| panic!("espèce {id} absente du contenu"))
        };
        // Pacifique, il se rencontre dans toutes les zones (section 9.2).
        let schnoz = || EspeceRare {
            espece: trouver("schnoz"),
            pacifique: true,
        };
        // Il attend au bout de la Lisière tant qu'il n'est pas vaincu
        // (section 10.7).
        let brigand = Humain::new(
            HumainId::new("brigand"),
            "Brigand",
            vec![
                (trouver("loup"), niveau(7)),
                (trouver("corbeau"), niveau(8)),
            ],
            60,
            AVANT_LE_BRIGAND,
            APRES_LE_BRIGAND,
        )
        .expect("brigand conforme aux règles")
        .avec_heros(Classe::Assassin, Some(objet("couteau-de-chasse")), None);
        // Au service du Baron pendu, il garde le Marais (section 10.7).
        let bourreau = Humain::new(
            HumainId::new("bourreau"),
            "Bourreau",
            vec![
                (trouver("armure-hantee"), niveau(10)),
                (trouver("feu-follet"), niveau(10)),
                (trouver("basilic"), niveau(11)),
            ],
            150,
            AVANT_LE_BOURREAU,
            APRES_LE_BOURREAU,
        )
        .expect("Bourreau conforme aux règles")
        .avec_heros(
            Classe::Guerrier,
            Some(objet("lame-de-milicien")),
            Some(objet("gambison")),
        )
        // Au niveau de sa zone redécoupée (section 9.1, étape 205).
        .avec_niveau_du_heros(niveau(12))
        .avec_ia(IaHumain::Relevee)
        .liberant_gunther(LIBERATION_DE_GUNTHER);
        // Un marchand mort par zone, dont le joueur donne l'âme à Malvoise
        // (section 14.2).
        let marchand = |id: &str, nom: &str, soin, gardien, avant: &str, apres: &str| {
            MarchandMort::new(MarchandId::new(id), nom, soin, gardien, avant, apres)
                .unwrap_or_else(|_| panic!("{nom} conforme aux règles"))
        };
        let bertrand = marchand(
            "bertrand",
            "Bertrand",
            Soin::Remede,
            Gardien::Bete {
                espece: trouver("loup"),
                niveau: niveau(7),
            },
            AVANT_BERTRAND,
            APRES_BERTRAND,
        )
        .a_l_etape(ETAPE_DE_BERTRAND);
        let grete = marchand(
            "grete",
            "Grete",
            Soin::GrandePotion,
            Gardien::MortVivant {
                espece: creer_espece(&MARCHAND_ZOMBIE),
                niveau: niveau(10),
            },
            AVANT_GRETE,
            APRES_GRETE,
        );
        let ulrich = marchand(
            "ulrich",
            "Ulrich",
            Soin::Elixir,
            Gardien::MonstreDeLaZone,
            AVANT_ULRICH,
            APRES_ULRICH,
        );
        let hilde = marchand(
            "hilde",
            "Hilde",
            Soin::Panacee,
            Gardien::MonstreDeLaZone,
            AVANT_HILDE,
            APRES_HILDE,
        );
        // Aux prises avec un Noyé, au niveau maximal du Marais (section
        // 9.2).
        let ansgar = Pecheur::new(
            ETAPE_D_ANSGAR,
            trouver("noye"),
            niveau(10),
            AVANT_ANSGAR,
            APRES_ANSGAR,
        )
        .expect("Ansgar conforme aux règles");
        let lisiere = Zone::new(
            ZoneId::new("lisiere"),
            "Lisière du Bois-Sombre",
            (niveau(3), niveau(7)),
            5,
            2,
            vec![trouver("loup"), trouver("corbeau"), trouver("vipere")],
            vec![EspeceRare {
                espece: trouver("cerf-blanc"),
                pacifique: true,
            }],
            expedition_automatique(1, 15..=60, 100..=200, 0..=1, 15..=30),
            coffre(10..=25, capsules_de("lisiere")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(brigand)
                .avec_marchand_mort(bertrand)
                .avec_meteos(METEOS_COMMUNES.to_vec())
        })
        .expect("Lisière conforme aux règles");
        let marais = Zone::new(
            ZoneId::new("marais"),
            "Marais des Pendus",
            (niveau(6), niveau(10)),
            5,
            3,
            vec![
                trouver("feu-follet"),
                trouver("armure-hantee"),
                trouver("vipere"),
                trouver("corbeau"),
            ],
            vec![EspeceRare {
                espece: trouver("basilic"),
                pacifique: false,
            }],
            expedition_automatique(2, 30..=91, 170..=350, 1..=2, 40..=80),
            coffre(25..=50, capsules_de("marais")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(bourreau)
                .avec_marchand_mort(grete)
                .avec_pecheur(ansgar)
                .avec_meteos(METEOS_COMMUNES.to_vec())
        })
        .expect("Marais conforme aux règles");
        let vallee = Zone::new(
            ZoneId::new("vallee"),
            "Vallée des Brumes",
            (niveau(9), niveau(13)),
            5,
            3,
            vec![
                trouver("fee-noire"),
                trouver("squelette"),
                trouver("armure-hantee"),
            ],
            vec![
                EspeceRare {
                    espece: trouver("elfe"),
                    pacifique: true,
                },
                EspeceRare {
                    espece: trouver("vouivre"),
                    pacifique: false,
                },
            ],
            expedition_automatique(3, 58..=142, 290..=540, 1..=3, 80..=150),
            coffre(50..=90, capsules_de("vallee")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_marchand_mort(ulrich).avec_meteos(vec![
                Meteo::Pluie,
                Meteo::Tempete,
                Meteo::Grele,
            ])
        })
        .expect("Vallée conforme aux règles");
        // Entre le Baron pendu et l'Abbesse, un bourg hanté, ouvert par l'indice de la Vallée (section 9.1, étape 206).
        let bourg = Zone::new(
            ZoneId::new("bourg-des-lamentations"),
            "Bourg des Lamentations",
            (niveau(12), niveau(16)),
            5,
            3,
            vec![
                trouver("pleureuse"),
                trouver("mendiant-possede"),
                trouver("epouvantail"),
            ],
            vec![EspeceRare {
                espece: trouver("dame-blanche"),
                pacifique: true,
            }],
            expedition_automatique(3, 80..=180, 450..=800, 1..=3, 100..=180),
            coffre(70..=120, Vec::new()),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| zone.avec_meteos(vec![Meteo::Pluie, Meteo::Tempete]))
        .expect("Bourg des Lamentations conforme aux règles");
        // Au service de l'Abbesse, il garde l'Abbaye (section 9.4).
        let inquisiteur = Humain::new(
            HumainId::new("inquisiteur"),
            "Inquisiteur",
            vec![
                (trouver("gargouille"), niveau(19)),
                (trouver("goule"), niveau(19)),
                (trouver("chimere"), niveau(20)),
            ],
            300,
            AVANT_L_INQUISITEUR,
            APRES_L_INQUISITEUR,
        )
        .expect("Inquisiteur conforme aux règles")
        .avec_heros(
            Classe::Mage,
            Some(objet("grimoire-des-brumes")),
            Some(objet("robe-des-brumes")),
        )
        // Au niveau de sa zone redécoupée (section 9.1, étape 205).
        .avec_niveau_du_heros(niveau(17))
        .avec_ia(IaHumain::Relevee);
        // Ouverte par l'indice que laisse le Baron pendu (section 9.4).
        let abbaye = Zone::new(
            ZoneId::new("abbaye"),
            "Abbaye des Cendres",
            (niveau(15), niveau(19)),
            5,
            3,
            vec![
                trouver("squelette"),
                trouver("gargouille"),
                trouver("goule"),
                trouver("chauve-souris-de-crypte"),
                trouver("cloche-hurlante"),
            ],
            vec![
                EspeceRare {
                    espece: trouver("chimere"),
                    pacifique: false,
                },
                EspeceRare {
                    espece: trouver("ange-dechu"),
                    pacifique: true,
                },
                EspeceRare {
                    espece: trouver("seraphin-de-vitrail"),
                    pacifique: true,
                },
            ],
            expedition_automatique(4, 106..=227, 650..=1100, 1..=3, 120..=220),
            coffre(90..=150, capsules_de("abbaye")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(inquisiteur)
                .avec_meteos(METEOS_COMMUNES.to_vec())
        })
        .expect("Abbaye conforme aux règles");
        // Entre l'Abbesse et le Margrave, un bois féerique, ouvert par l'indice de l'Abbaye ; le Farfadet y est un clin d'oeil au Puck de Berserk (section 9.1, étape 206).
        let bois = Zone::new(
            ZoneId::new("bois-des-farfadets"),
            "Bois des Farfadets",
            (niveau(18), niveau(22)),
            5,
            3,
            vec![
                trouver("hibou-blafard"),
                trouver("amanite-rampante"),
                trouver("ours-moussu"),
            ],
            vec![EspeceRare {
                espece: trouver("farfadet"),
                pacifique: true,
            }],
            expedition_automatique(4, 130..=270, 880..=1400, 1..=3, 150..=270),
            coffre(110..=180, Vec::new()),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| zone.avec_meteos(vec![Meteo::Pluie, Meteo::Tempete]))
        .expect("Bois des Farfadets conforme aux règles");
        // Au service du Margrave, il garde le Col (section 9.4).
        let porte_etendard = Humain::new(
            HumainId::new("porte-etendard"),
            "Porte-étendard",
            vec![
                (trouver("wendigo"), niveau(25)),
                (trouver("troll-des-cavernes"), niveau(25)),
                (trouver("wyrm-de-givre"), niveau(26)),
            ],
            400,
            AVANT_LE_PORTE_ETENDARD,
            APRES_LE_PORTE_ETENDARD,
        )
        .expect("Porte-étendard conforme aux règles")
        .avec_heros(
            Classe::Templier,
            Some(objet("encensoir-maudit")),
            Some(objet("plates-consacrees")),
        )
        // Au niveau de sa zone redécoupée (section 9.1, étape 205).
        .avec_niveau_du_heros(niveau(24))
        .avec_ia(IaHumain::Relevee);
        // Ouvert par l'indice que laisse l'Abbesse ; ses neuf niveaux mènent
        // aux stades 3 du niveau 36 (sections 5.4 et 9.4).
        let col = Zone::new(
            ZoneId::new("col-du-givre-noir"),
            "Col du Givre-Noir",
            (niveau(21), niveau(25)),
            5,
            3,
            vec![
                trouver("wendigo"),
                trouver("troll-des-cavernes"),
                trouver("harpie-des-orages"),
                trouver("gargouille"),
                trouver("faux-agneau"),
                trouver("gisant"),
            ],
            vec![
                EspeceRare {
                    espece: trouver("wyrm-de-givre"),
                    pacifique: false,
                },
                EspeceRare {
                    espece: trouver("sylphe-des-neiges"),
                    pacifique: true,
                },
            ],
            expedition_automatique(5, 158..=323, 1140..=1760, 1..=3, 180..=320),
            coffre(140..=220, capsules_de("col-du-givre-noir")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(porte_etendard)
                .avec_meteos(vec![Meteo::Grele, Meteo::Tempete])
        })
        .expect("Col conforme aux règles");
        // Elle annonce un autre Apôtre, sans en garder aucun (section 9.4).
        let pythie = Humain::new(
            HumainId::new("pythie-de-sel"),
            "Pythie de sel",
            vec![
                (trouver("momie-de-sel"), niveau(28)),
                (trouver("ver-des-salines"), niveau(28)),
                (trouver("sphinx-des-sables"), niveau(29)),
            ],
            500,
            AVANT_LA_PYTHIE,
            APRES_LA_PYTHIE,
        )
        .expect("Pythie de sel conforme aux règles")
        .avec_heros(
            Classe::Mage,
            Some(objet("grimoire-du-champion")),
            Some(objet("robe-des-cendres")),
        )
        // Au niveau de sa zone redécoupée (section 9.1, étape 205).
        .avec_niveau_du_heros(niveau(23))
        .avec_ia(IaHumain::Relevee);
        // Ouvert par l'indice que laisse le Margrave ; ses niveaux mènent la
        // Vouivre à son évolution du niveau 45 (sections 5.4 et 9.1).
        let desert = Zone::new(
            ZoneId::new("desert-de-sel"),
            "Désert de sel",
            (niveau(24), niveau(28)),
            5,
            3,
            vec![
                trouver("crotale-des-salines"),
                trouver("scarabee-solaire"),
                trouver("momie-de-sel"),
                trouver("ver-des-salines"),
            ],
            vec![
                EspeceRare {
                    espece: trouver("sphinx-des-sables"),
                    pacifique: false,
                },
                EspeceRare {
                    espece: trouver("phenix-blanc"),
                    pacifique: true,
                },
            ],
            expedition_automatique(6, 178..=355, 1210..=1850, 1..=3, 230..=400),
            coffre(180..=280, capsules_de("desert-de-sel")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(pythie)
                .avec_marchand_mort(hilde)
                .avec_meteos(vec![Meteo::TempeteDeSable, Meteo::Canicule])
        })
        .expect("Désert de sel conforme aux règles");
        // Au service de l'Amiral noyé, il garde le Port. Dès le Port, le
        // héros d'un humain gardien combat au niveau maximal de sa zone, à
        // part de son équipement (section 10.7, étape 163).
        let contrebandier = Humain::new(
            HumainId::new("contrebandier"),
            "Contrebandier",
            vec![
                (trouver("sirene-putride"), niveau(31)),
                (trouver("hydre-des-marees"), niveau(32)),
            ],
            600,
            AVANT_LE_CONTREBANDIER,
            APRES_LE_CONTREBANDIER,
        )
        .expect("Contrebandier conforme aux règles")
        .avec_heros(
            Classe::Assassin,
            Some(objet("crocs-du-champion")),
            Some(objet("manteau-de-cendre")),
        )
        .avec_niveau_du_heros(niveau(31))
        .avec_ia(IaHumain::Relevee);
        // La seule zone de mer, où souffle le cyclone (section 9.7).
        let meteos_du_port = vec![Meteo::Pluie, Meteo::Tempete, Meteo::Cyclone];
        // Ouvert par l'indice que laisse la Pythie (section 9.4).
        let port = Zone::new(
            ZoneId::new("port-des-noyes"),
            "Port des Noyés",
            (niveau(27), niveau(31)),
            5,
            3,
            vec![
                trouver("crabe-necrophage"),
                trouver("sirene-putride"),
                trouver("goeland-charognard"),
            ],
            vec![EspeceRare {
                espece: trouver("hydre-des-marees"),
                pacifique: false,
            }],
            expedition_automatique(7, 203..=389, 1360..=2000, 1..=3, 280..=460),
            coffre(220..=340, capsules_de("port-des-noyes")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(contrebandier)
                .avec_meteos(meteos_du_port.clone())
        })
        .expect("Port des Noyés conforme aux règles");
        // Entre l'Amiral et le Prévôt, la haute mer, où l'on ne part qu'avec le bateau de Gunther (section 9.1, étape 207).
        let mer = Zone::new(
            ZoneId::new("mer-des-naufrages"),
            "Mer des Naufrages",
            (niveau(31), niveau(35)),
            5,
            3,
            vec![
                trouver("meduse-des-naufrages"),
                trouver("requin-noir"),
                trouver("albatros-maudit"),
            ],
            vec![EspeceRare {
                espece: trouver("kraken"),
                pacifique: false,
            }],
            expedition_automatique(7, 230..=440, 1700..=2500, 1..=3, 300..=490),
            coffre(240..=370, Vec::new()),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_meteos(vec![Meteo::Pluie, Meteo::Tempete, Meteo::Cyclone])
                .exigeant_un_bateau()
        })
        .expect("Mer des Naufrages conforme aux règles");
        // Au service du Prévôt des fosses, il garde les Mines (section
        // 10.7).
        let contremaitre = Humain::new(
            HumainId::new("contremaitre"),
            "Contremaître",
            vec![
                (trouver("golem-de-fer-noir"), niveau(39)),
                (trouver("mineur-possede"), niveau(39)),
                (trouver("dragon-de-rouille"), niveau(40)),
            ],
            700,
            AVANT_LE_CONTREMAITRE,
            APRES_LE_CONTREMAITRE,
        )
        .expect("Contremaître conforme aux règles")
        .avec_heros(
            Classe::Guerrier,
            Some(objet("hache-de-fer-noir")),
            Some(objet("cuirasse-des-abysses")),
        )
        .avec_niveau_du_heros(niveau(39))
        .avec_ia(IaHumain::Relevee);
        let meteos_des_mines = vec![Meteo::Grele, Meteo::Tempete];
        // Ouvertes par l'indice que laisse l'Amiral noyé (section 9.4).
        let mines = Zone::new(
            ZoneId::new("mines-de-fer-noir"),
            "Mines de Fer-Noir",
            (niveau(35), niveau(39)),
            5,
            3,
            vec![
                trouver("golem-de-fer-noir"),
                trouver("mineur-possede"),
                trouver("grisou-ardent"),
            ],
            vec![EspeceRare {
                espece: trouver("dragon-de-rouille"),
                pacifique: false,
            }],
            expedition_automatique(8, 272..=516, 2120..=3130, 1..=3, 330..=540),
            coffre(260..=400, capsules_de("mines-de-fer-noir")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(contremaitre)
                .avec_meteos(meteos_des_mines.clone())
        })
        .expect("Mines de Fer-Noir conforme aux règles");
        // Entre le Prévôt et la Duchesse, une plage couverte d'os, que la marée rejette (section 9.1, étape 207).
        let greve = Zone::new(
            ZoneId::new("greve-des-ossements"),
            "Grève des Ossements",
            (niveau(39), niveau(43)),
            5,
            3,
            vec![
                trouver("pagure-a-crane"),
                trouver("mouette-hurleuse"),
                trouver("naufrageur"),
            ],
            vec![EspeceRare {
                espece: trouver("sirene-d-ecume"),
                pacifique: true,
            }],
            expedition_automatique(8, 300..=570, 2600..=3800, 1..=3, 350..=580),
            coffre(280..=430, Vec::new()),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| zone.avec_meteos(vec![Meteo::Pluie, Meteo::Tempete, Meteo::Canicule]))
        .expect("Grève des Ossements conforme aux règles");
        // Les annexes, ouvertes par une carte trouvée dans un coffre de leur
        // zone : un peu plus hautes, avec les espèces de leur zone, une
        // espèce rare qui ne vit que là et des coffres plus riches, qui
        // gardent les capsules de leur zone (section 9.9). Leurs paramètres
        // d'expédition automatique ne servent jamais.
        let annexe = |zone: Result<Zone, _>, de: &Zone, meteos: Vec<Meteo>, decouverte: &str| {
            zone.and_then(|zone: Zone| zone.avec_repartition(Repartition::DES_ANNEXES))
                .and_then(|zone| zone.avec_monstre_cache(schnoz()))
                .and_then(|zone| {
                    let departs = DEPARTS_D_APRES_LA_FIN
                        .iter()
                        .filter(|(annexe, _)| *annexe == zone.id().as_str())
                        .map(|(_, espece)| EspeceRare {
                            espece: trouver(espece),
                            pacifique: true,
                        })
                        .collect();
                    zone.avec_rares_d_apres_la_fin(departs)
                })
                .map(|zone| Annexe::new(zone.avec_meteos(meteos), de.id().clone(), decouverte))
        };
        let epave = annexe(
            Zone::new(
                ZoneId::new("epave-du-leviathan"),
                "Épave du Léviathan",
                (niveau(30), niveau(33)),
                3,
                3,
                port.especes().to_vec(),
                vec![EspeceRare {
                    espece: trouver("nautile-des-abysses"),
                    pacifique: true,
                }],
                port.expedition_automatique().clone(),
                coffre(300..=450, capsules_de("port-des-noyes")).avec_tresors(tresors_d_annexe()),
            ),
            &port,
            meteos_du_port,
            DECOUVERTE_DE_L_EPAVE,
        )
        .expect("Épave du Léviathan conforme aux règles");
        let filon = annexe(
            Zone::new(
                ZoneId::new("filon-ecarlate"),
                "Filon écarlate",
                (niveau(38), niveau(41)),
                3,
                3,
                mines.especes().to_vec(),
                vec![EspeceRare {
                    espece: trouver("gemmame"),
                    pacifique: true,
                }],
                mines.expedition_automatique().clone(),
                coffre(340..=500, capsules_de("mines-de-fer-noir"))
                    .avec_tresors(tresors_d_annexe()),
            ),
            &mines,
            meteos_des_mines,
            DECOUVERTE_DU_FILON,
        )
        .expect("Filon écarlate conforme aux règles");
        // Au service de la Duchesse sanguine, elle garde la Sylve, avec
        // deux bêtes seulement, comme le Contrebandier (section 10.7).
        let chasseresse = Humain::new(
            HumainId::new("chasseresse"),
            "Chasseresse",
            vec![
                (trouver("mante-ecarlate"), niveau(47)),
                (trouver("dryade-sanglante"), niveau(48)),
            ],
            800,
            AVANT_LA_CHASSERESSE,
            APRES_LA_CHASSERESSE,
        )
        .expect("Chasseresse conforme aux règles")
        .avec_heros(
            Classe::Assassin,
            Some(objet("dagues-de-fer-noir")),
            Some(objet("maille-de-fer-noir")),
        )
        .avec_niveau_du_heros(niveau(47))
        .avec_ia(IaHumain::Relevee);
        // Ouverte par l'indice que laisse le Prévôt des fosses (section
        // 9.4).
        let sylve = Zone::new(
            ZoneId::new("sylve-ecarlate"),
            "Sylve écarlate",
            (niveau(43), niveau(47)),
            5,
            3,
            vec![
                trouver("ronce-vampire"),
                trouver("mante-ecarlate"),
                trouver("sanglier-enrage"),
            ],
            vec![EspeceRare {
                espece: trouver("dryade-sanglante"),
                pacifique: false,
            }],
            expedition_automatique(9, 345..=652, 3150..=4610, 1..=3, 380..=620),
            coffre(300..=460, capsules_de("sylve-ecarlate")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(chasseresse)
                .avec_meteos(METEOS_COMMUNES.to_vec())
        })
        .expect("Sylve écarlate conforme aux règles");
        // Entre la Duchesse et l'Évêque, des champs de pavots noirs dont le parfum endort les voyageurs (section 9.1, étape 208).
        let pavots = Zone::new(
            ZoneId::new("champs-de-pavots-noirs"),
            "Champs de Pavots noirs",
            (niveau(47), niveau(50)),
            5,
            3,
            vec![
                trouver("sphinx-tete-de-mort"),
                trouver("lievre-somnambule"),
                trouver("pavot-carnivore"),
            ],
            vec![EspeceRare {
                espece: trouver("marchand-de-sable"),
                pacifique: false,
            }],
            expedition_automatique(9, 378..=714, 3640..=5320, 1..=3, 405..=660),
            coffre(320..=490, Vec::new()),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| zone.avec_meteos(vec![Meteo::Pluie, Meteo::Canicule]))
        .expect("Champs de Pavots noirs conforme aux règles");
        // Au service de l'Évêque des os, il garde les Catacombes (section
        // 10.7).
        let fossoyeur = Humain::new(
            HumainId::new("fossoyeur"),
            "Fossoyeur",
            vec![
                (trouver("rat-des-ossuaires"), niveau(53)),
                (trouver("moine-psalmodiant"), niveau(53)),
                (trouver("liche"), niveau(54)),
            ],
            900,
            AVANT_LE_FOSSOYEUR,
            APRES_LE_FOSSOYEUR,
        )
        .expect("Fossoyeur conforme aux règles")
        .avec_heros(
            Classe::Mage,
            Some(objet("crosse-des-catacombes")),
            Some(objet("robe-de-grisou")),
        )
        .avec_niveau_du_heros(niveau(53))
        .avec_ia(IaHumain::Relevee);
        let meteos_des_catacombes = vec![Meteo::Pluie, Meteo::Tempete];
        // Ouvertes par l'indice que laisse la Duchesse sanguine (section
        // 9.4).
        let catacombes = Zone::new(
            ZoneId::new("catacombes-de-sainte-ombre"),
            "Catacombes de Sainte-Ombre",
            (niveau(50), niveau(53)),
            5,
            3,
            vec![
                trouver("rat-des-ossuaires"),
                trouver("moine-psalmodiant"),
                trouver("araignee-des-cryptes"),
            ],
            vec![EspeceRare {
                espece: trouver("liche"),
                pacifique: false,
            }],
            expedition_automatique(10, 412..=776, 4140..=6030, 1..=3, 430..=700),
            coffre(340..=520, capsules_de("catacombes-de-sainte-ombre")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(fossoyeur)
                .avec_meteos(meteos_des_catacombes.clone())
        })
        .expect("Catacombes de Sainte-Ombre conformes aux règles");
        let autel = annexe(
            Zone::new(
                ZoneId::new("autel-des-ronces"),
                "Autel des ronces",
                (niveau(46), niveau(49)),
                3,
                3,
                sylve.especes().to_vec(),
                vec![EspeceRare {
                    espece: trouver("phalene-d-aube"),
                    pacifique: true,
                }],
                sylve.expedition_automatique().clone(),
                coffre(400..=600, capsules_de("sylve-ecarlate")).avec_tresors(tresors_d_annexe()),
            ),
            &sylve,
            METEOS_COMMUNES.to_vec(),
            DECOUVERTE_DE_L_AUTEL,
        )
        .expect("Autel des ronces conforme aux règles");
        // Entre l'Évêque et le Connétable, les remparts d'une ville assiégée depuis des siècles, que ses défenseurs morts gardent encore (section 9.1, étape 208).
        let remparts = Zone::new(
            ZoneId::new("remparts-de-grisval"),
            "Remparts de Grisval",
            (niveau(53), niveau(56)),
            5,
            3,
            vec![
                trouver("belier-de-siege"),
                trouver("sentinelle-de-pierre"),
                trouver("baliste-hantee"),
            ],
            vec![EspeceRare {
                espece: trouver("chevalier-errant"),
                pacifique: false,
            }],
            expedition_automatique(10, 448..=840, 4700..=6800, 1..=3, 460..=740),
            coffre(360..=550, Vec::new()),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| zone.avec_meteos(vec![Meteo::Pluie, Meteo::Tempete, Meteo::Grele]))
        .expect("Remparts de Grisval conforme aux règles");
        let crypte = annexe(
            Zone::new(
                ZoneId::new("crypte-des-saints"),
                "Crypte des saints",
                (niveau(52), niveau(55)),
                3,
                3,
                catacombes.especes().to_vec(),
                vec![EspeceRare {
                    espece: trouver("ange-de-pierre"),
                    pacifique: true,
                }],
                catacombes.expedition_automatique().clone(),
                coffre(450..=680, capsules_de("catacombes-de-sainte-ombre"))
                    .avec_tresors(tresors_d_annexe()),
            ),
            &catacombes,
            meteos_des_catacombes,
            DECOUVERTE_DE_LA_CRYPTE,
        )
        .expect("Crypte des saints conforme aux règles");
        // Au service du Connétable boucher, il garde la Plaine (section 10.7).
        let capitaine = Humain::new(
            HumainId::new("capitaine-mercenaire"),
            "Capitaine mercenaire",
            vec![
                (trouver("vautour-des-charniers"), niveau(59)),
                (trouver("matin-de-guerre"), niveau(59)),
                (trouver("griffon-cuirasse"), niveau(60)),
            ],
            1000,
            AVANT_LE_CAPITAINE,
            APRES_LE_CAPITAINE,
        )
        .expect("Capitaine mercenaire conforme aux règles")
        .avec_heros(
            Classe::Guerrier,
            Some(objet("hache-du-bourreau-d-os")),
            Some(objet("plates-du-sanglier")),
        )
        .avec_niveau_du_heros(niveau(59))
        .avec_ia(IaHumain::Relevee);
        let meteos_de_la_plaine = vec![Meteo::Tempete, Meteo::Canicule];
        // Ouverte par l'indice que laisse l'Évêque des os (section 9.4).
        let plaine = Zone::new(
            ZoneId::new("plaine-du-massacre"),
            "Plaine du Massacre",
            (niveau(56), niveau(59)),
            5,
            3,
            vec![
                trouver("destrier-spectral"),
                trouver("vautour-des-charniers"),
                trouver("matin-de-guerre"),
            ],
            vec![EspeceRare {
                espece: trouver("griffon-cuirasse"),
                pacifique: false,
            }],
            expedition_automatique(11, 485..=904, 5260..=7560, 1..=3, 490..=780),
            coffre(380..=580, capsules_de("plaine-du-massacre")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(capitaine)
                .avec_meteos(meteos_de_la_plaine.clone())
        })
        .expect("Plaine du Massacre conforme aux règles");
        // Entre le Connétable et le Chancelier, une forêt changée en pierre avec tout ce qui y vivait (section 9.1, étape 208).
        let foret = Zone::new(
            ZoneId::new("foret-petrifiee"),
            "Forêt pétrifiée",
            (niveau(59), niveau(62)),
            5,
            3,
            vec![
                trouver("cerf-de-pierre"),
                trouver("lezard-petrifie"),
                trouver("dryade-de-pierre"),
            ],
            vec![EspeceRare {
                espece: trouver("arbre-roi-petrifie"),
                pacifique: true,
            }],
            expedition_automatique(11, 522..=981, 5960..=8520, 1..=3, 520..=820),
            coffre(400..=610, Vec::new()),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| zone.avec_meteos(vec![Meteo::Tempete, Meteo::Grele]))
        .expect("Forêt pétrifiée conforme aux règles");
        let tertre = annexe(
            Zone::new(
                ZoneId::new("tertre-des-heros"),
                "Tertre des héros",
                (niveau(58), niveau(61)),
                3,
                3,
                plaine.especes().to_vec(),
                vec![EspeceRare {
                    espece: trouver("licorne-des-cendres"),
                    pacifique: true,
                }],
                plaine.expedition_automatique().clone(),
                coffre(500..=760, capsules_de("plaine-du-massacre"))
                    .avec_tresors(tresors_d_annexe()),
            ),
            &plaine,
            meteos_de_la_plaine,
            DECOUVERTE_DU_TERTRE,
        )
        .expect("Tertre des héros conforme aux règles");
        // Au service du Chancelier aux mille yeux, il garde la Haute-Cité
        // (section 10.7).
        let archiviste = Humain::new(
            HumainId::new("archiviste"),
            "Archiviste",
            vec![
                (trouver("automate-brise"), niveau(66)),
                (trouver("bibliophage"), niveau(66)),
                (trouver("archonte-foudroye"), niveau(67)),
            ],
            1100,
            AVANT_L_ARCHIVISTE,
            APRES_L_ARCHIVISTE,
        )
        .expect("Archiviste conforme aux règles")
        .avec_heros(
            Classe::Mage,
            Some(objet("crosse-des-catacombes")),
            Some(objet("robe-de-ronces")),
        )
        .avec_niveau_du_heros(niveau(66))
        .avec_ia(IaHumain::Relevee);
        let meteos_de_la_cite = vec![Meteo::Tempete, Meteo::Grele];
        // Ouverte par l'indice que laisse le Connétable boucher (section 9.4).
        let cite = Zone::new(
            ZoneId::new("haute-cite-dechue"),
            "Haute-Cité déchue",
            (niveau(63), niveau(66)),
            5,
            3,
            vec![
                trouver("automate-brise"),
                trouver("bibliophage"),
                trouver("oeil-errant"),
            ],
            vec![EspeceRare {
                espece: trouver("archonte-foudroye"),
                pacifique: false,
            }],
            expedition_automatique(12, 559..=1058, 6660..=9470, 1..=3, 550..=860),
            coffre(420..=640, capsules_de("haute-cite-dechue")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(archiviste)
                .avec_meteos(meteos_de_la_cite.clone())
        })
        .expect("Haute-Cité déchue conforme aux règles");
        // Entre le Chancelier et le Prince, une vallée cachée sous la terre, éclairée par un soleil souterrain, où des moines gardent une paix ancienne (section 9.1, étape 208).
        let sanctuaire = Zone::new(
            ZoneId::new("sanctuaire-d-asphodele"),
            "Sanctuaire d'Asphodèle",
            (niveau(67), niveau(70)),
            5,
            3,
            vec![
                trouver("moine-de-lumiere"),
                trouver("chimere-de-cristal"),
                trouver("ver-luisant-geant"),
            ],
            vec![EspeceRare {
                espece: trouver("gardien-d-asphodele"),
                pacifique: true,
            }],
            expedition_automatique(12, 601..=1145, 7560..=10690, 1..=3, 580..=900),
            coffre(440..=670, Vec::new()),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| zone.avec_meteos(vec![Meteo::Pluie, Meteo::Canicule]))
        .expect("Sanctuaire d'Asphodèle conforme aux règles");
        let bibliotheque = annexe(
            Zone::new(
                ZoneId::new("bibliotheque-interdite"),
                "Bibliothèque interdite",
                (niveau(65), niveau(68)),
                3,
                3,
                cite.especes().to_vec(),
                vec![EspeceRare {
                    espece: trouver("muse-de-cristal"),
                    pacifique: true,
                }],
                cite.expedition_automatique().clone(),
                coffre(560..=840, capsules_de("haute-cite-dechue"))
                    .avec_tresors(tresors_d_annexe()),
            ),
            &cite,
            meteos_de_la_cite,
            DECOUVERTE_DE_LA_BIBLIOTHEQUE,
        )
        .expect("Bibliothèque interdite conforme aux règles");
        // Au service du Prince de braise, il garde la Gorge (section 10.7).
        let pyromancien = Humain::new(
            HumainId::new("pyromancien"),
            "Pyromancien",
            vec![
                (trouver("forgeron-calcine"), niveau(74)),
                (trouver("chien-des-scories"), niveau(74)),
                (trouver("wyrm-de-magma"), niveau(75)),
            ],
            1200,
            AVANT_LE_PYROMANCIEN,
            APRES_LE_PYROMANCIEN,
        )
        .expect("Pyromancien conforme aux règles")
        .avec_heros(
            Classe::Mage,
            Some(objet("sceptre-d-ether")),
            Some(objet("robe-des-archives")),
        )
        .avec_niveau_du_heros(niveau(74))
        .avec_ia(IaHumain::Relevee);
        let meteos_de_la_gorge = vec![Meteo::Canicule, Meteo::Tempete];
        // Ouverte par l'indice que laisse le Chancelier aux mille yeux (section
        // 9.4).
        let gorge = Zone::new(
            ZoneId::new("gorge-de-braise"),
            "Gorge de Braise",
            (niveau(71), niveau(74)),
            5,
            3,
            vec![
                trouver("golem-de-magma"),
                trouver("chien-des-scories"),
                trouver("forgeron-calcine"),
            ],
            vec![EspeceRare {
                espece: trouver("wyrm-de-magma"),
                pacifique: false,
            }],
            expedition_automatique(13, 643..=1232, 8460..=11910, 1..=3, 610..=940),
            coffre(460..=700, capsules_de("gorge-de-braise")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(pyromancien)
                .avec_meteos(meteos_de_la_gorge.clone())
        })
        .expect("Gorge de Braise conforme aux règles");
        // Entre le Prince et le Sénéchal, un glacier où les âmes des morts restent prises dans la glace (section 9.1, étape 209).
        let glacier = Zone::new(
            ZoneId::new("glacier-des-ames"),
            "Glacier des Âmes",
            (niveau(75), niveau(78)),
            5,
            3,
            vec![
                trouver("ours-des-glaces"),
                trouver("yeti"),
                trouver("esprit-du-blizzard"),
            ],
            vec![EspeceRare {
                espece: trouver("mammouth-fantome"),
                pacifique: false,
            }],
            expedition_automatique(13, 686..=1322, 9480..=13270, 1..=3, 640..=980),
            coffre(480..=730, Vec::new()),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| zone.avec_meteos(vec![Meteo::Grele, Meteo::Tempete]))
        .expect("Glacier des Âmes conforme aux règles");
        let coeur = annexe(
            Zone::new(
                ZoneId::new("coeur-du-volcan"),
                "Cœur du volcan",
                (niveau(73), niveau(76)),
                3,
                3,
                gorge.especes().to_vec(),
                vec![EspeceRare {
                    espece: trouver("ifrit"),
                    pacifique: true,
                }],
                gorge.expedition_automatique().clone(),
                coffre(620..=920, capsules_de("gorge-de-braise")).avec_tresors(tresors_d_annexe()),
            ),
            &gorge,
            meteos_de_la_gorge,
            DECOUVERTE_DU_COEUR,
        )
        .expect("Cœur du volcan conforme aux règles");
        // Au service du Sénéchal des orages, il garde les Pics (section 10.7).
        let heraut = Humain::new(
            HumainId::new("heraut-de-l-orage"),
            "Héraut de l'orage",
            vec![
                (trouver("belier-de-granit"), niveau(82)),
                (trouver("aigle-tonnerre"), niveau(82)),
                (trouver("geant-des-orages"), niveau(83)),
            ],
            1300,
            AVANT_LE_HERAUT,
            APRES_LE_HERAUT,
        )
        .expect("Héraut de l'orage conforme aux règles")
        .avec_heros(
            Classe::Templier,
            Some(objet("marteau-du-griffon")),
            Some(objet("plates-de-la-cite")),
        )
        .avec_niveau_du_heros(niveau(82))
        .avec_ia(IaHumain::Relevee);
        let meteos_des_pics = vec![Meteo::Tempete, Meteo::Grele];
        // Ouverts par l'indice que laisse le Prince de braise (section 9.4).
        let pics = Zone::new(
            ZoneId::new("pics-foudroyes"),
            "Pics Foudroyés",
            (niveau(79), niveau(82)),
            5,
            3,
            vec![
                trouver("aigle-tonnerre"),
                trouver("belier-de-granit"),
                trouver("feu-de-saint-elme"),
            ],
            vec![EspeceRare {
                espece: trouver("geant-des-orages"),
                pacifique: false,
            }],
            expedition_automatique(14, 729..=1411, 10490..=14620, 1..=3, 670..=1020),
            coffre(500..=760, capsules_de("pics-foudroyes")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(heraut)
                .avec_meteos(meteos_des_pics.clone())
        })
        .expect("Pics Foudroyés conforme aux règles");
        // Entre le Sénéchal et la Reine-mère, une vallée obscure où les géants d'autrefois sont venus mourir (section 9.1, étape 209).
        let cimetiere = Zone::new(
            ZoneId::new("cimetiere-des-geants"),
            "Cimetière des Géants",
            (niveau(83), niveau(86)),
            5,
            3,
            vec![
                trouver("colosse-d-os"),
                trouver("charognard-des-geants"),
                trouver("ver-ossivore"),
            ],
            vec![EspeceRare {
                espece: trouver("titan-dechu"),
                pacifique: false,
            }],
            expedition_automatique(14, 772..=1502, 11630..=16110, 1..=3, 700..=1060),
            coffre(520..=790, Vec::new()),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| zone.avec_meteos(vec![Meteo::Tempete, Meteo::Pluie]))
        .expect("Cimetière des Géants conforme aux règles");
        let aire = annexe(
            Zone::new(
                ZoneId::new("aire-des-pegases"),
                "Aire des pégases",
                (niveau(81), niveau(84)),
                3,
                3,
                pics.especes().to_vec(),
                vec![EspeceRare {
                    espece: trouver("pegase-d-orage"),
                    pacifique: true,
                }],
                pics.expedition_automatique().clone(),
                coffre(680..=1000, capsules_de("pics-foudroyes")).avec_tresors(tresors_d_annexe()),
            ),
            &pics,
            meteos_des_pics,
            DECOUVERTE_DE_L_AIRE,
        )
        .expect("Aire des pégases conforme aux règles");
        // Au service de la Reine-mère de chair, elle garde le Palais, avec
        // deux monstres seulement, comme les autres Assassins (section 10.7).
        let favorite = Humain::new(
            HumainId::new("favorite"),
            "Favorite",
            vec![
                (trouver("devoreur-de-chair"), niveau(90)),
                (trouver("idole-de-chair"), niveau(91)),
            ],
            1400,
            AVANT_LA_FAVORITE,
            APRES_LA_FAVORITE,
        )
        .expect("Favorite conforme aux règles")
        .avec_heros(
            Classe::Assassin,
            Some(objet("dagues-foudroyees")),
            Some(objet("cuir-d-orage")),
        )
        .avec_niveau_du_heros(niveau(90))
        .avec_ia(IaHumain::Relevee);
        let meteos_du_palais = vec![Meteo::Pluie, Meteo::Canicule];
        // Ouvert par l'indice que laisse le Sénéchal des orages (section 9.4).
        let palais = Zone::new(
            ZoneId::new("palais-de-chair"),
            "Palais de Chair",
            (niveau(87), niveau(90)),
            5,
            3,
            vec![
                trouver("larve-de-chair"),
                trouver("devoreur-de-chair"),
                trouver("valet-ecorche"),
            ],
            vec![EspeceRare {
                espece: trouver("idole-de-chair"),
                pacifique: false,
            }],
            expedition_automatique(15, 816..=1593, 12760..=17600, 1..=3, 730..=1100),
            coffre(540..=820, capsules_de("palais-de-chair")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(favorite)
                .avec_meteos(meteos_du_palais.clone())
        })
        .expect("Palais de Chair conforme aux règles");
        // Entre la Reine-mère et le Roi sans visage, une faille où le monde s'arrête et où le néant regarde ceux qui s'en approchent (section 9.1, étape 209).
        let faille = Zone::new(
            ZoneId::new("faille-du-neant"),
            "Faille du Néant",
            (niveau(91), niveau(94)),
            5,
            3,
            vec![
                trouver("rampant-du-neant"),
                trouver("mangeur-d-etoiles"),
                trouver("spectre-hurlant"),
            ],
            vec![EspeceRare {
                espece: trouver("ange-du-neant"),
                pacifique: false,
            }],
            expedition_automatique(15, 858..=1686, 13940..=19120, 1..=3, 760..=1140),
            coffre(560..=850, Vec::new()),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| zone.avec_meteos(vec![Meteo::Tempete, Meteo::Grele]))
        .expect("Faille du Néant conforme aux règles");
        let jardin = annexe(
            Zone::new(
                ZoneId::new("jardin-de-la-reine"),
                "Jardin de la Reine",
                (niveau(89), niveau(92)),
                3,
                3,
                palais.especes().to_vec(),
                vec![EspeceRare {
                    espece: trouver("lys-de-sang"),
                    pacifique: true,
                }],
                palais.expedition_automatique().clone(),
                coffre(740..=1080, capsules_de("palais-de-chair")).avec_tresors(tresors_d_annexe()),
            ),
            &palais,
            meteos_du_palais,
            DECOUVERTE_DU_JARDIN,
        )
        .expect("Jardin de la Reine conforme aux règles");
        // Au service du Roi sans visage, il garde le Puits (section 10.7).
        let chevalier_crane = Humain::new(
            HumainId::new("chevalier-crane"),
            "Chevalier-Crâne",
            vec![
                (trouver("rampant-du-puits"), niveau(98)),
                (trouver("ver-des-abysses"), niveau(98)),
                (trouver("gardien-du-gouffre"), niveau(99)),
            ],
            1500,
            AVANT_LE_CHEVALIER_CRANE,
            APRES_LE_CHEVALIER_CRANE,
        )
        .expect("Chevalier-Crâne conforme aux règles")
        .avec_heros(
            Classe::Guerrier,
            Some(objet("hache-de-magma")),
            Some(objet("plates-de-magma")),
        )
        .avec_niveau_du_heros(niveau(98))
        .avec_ia(IaHumain::Relevee);
        let meteos_du_puits = vec![Meteo::Tempete, Meteo::Pluie];
        // Ouvert par l'indice que laisse la Reine-mère de chair (section 9.4).
        let puits = Zone::new(
            ZoneId::new("puits-du-monde"),
            "Puits du Monde",
            (niveau(95), niveau(98)),
            5,
            3,
            vec![
                trouver("rampant-du-puits"),
                trouver("echo-du-vide"),
                trouver("ver-des-abysses"),
            ],
            vec![EspeceRare {
                espece: trouver("gardien-du-gouffre"),
                pacifique: false,
            }],
            expedition_automatique(16, 899..=1778, 15110..=20630, 1..=3, 790..=1180),
            coffre(580..=880, capsules_de("puits-du-monde")),
        )
        .and_then(|zone| zone.avec_monstre_cache(schnoz()))
        .map(|zone| {
            zone.avec_humain(chevalier_crane)
                .avec_meteos(meteos_du_puits.clone())
        })
        .expect("Puits du Monde conforme aux règles");
        let racines = annexe(
            Zone::new(
                ZoneId::new("racines-du-monde"),
                "Racines du monde",
                (niveau(96), niveau(99)),
                3,
                3,
                puits.especes().to_vec(),
                vec![EspeceRare {
                    espece: trouver("germe-du-monde"),
                    pacifique: true,
                }],
                puits.expedition_automatique().clone(),
                coffre(800..=1160, capsules_de("puits-du-monde")).avec_tresors(tresors_d_annexe()),
            ),
            &puits,
            meteos_du_puits,
            DECOUVERTE_DES_RACINES,
        )
        .expect("Racines du monde conforme aux règles");
        // Le Baron pendu attend au fond de la Vallée, que le second indice,
        // donné par le Bourreau, ouvre. Premier Apôtre, il combat au niveau
        // maximal de la Vallée, 20, et ne connaît qu'une attaque Obscur
        // (section 9.4, étape 117).
        let baron = Apotre::new(trouver("baron-pendu"), &vallee, AVANT_LE_BARON, EPILOGUE)
            .and_then(|baron| baron.au_niveau_maximal_de_sa_zone(&vallee))
            .expect("Baron pendu conforme aux règles")
            .avec_recit_de_l_ame(AME_DU_BARON)
            .avec_ligne_du_carnet(CARNET_DU_BARON);
        let indice = |zone: &Zone, texte: &str| Indice {
            zone: zone.id().clone(),
            texte: texte.to_string(),
        };
        let chapitre_du_baron = Chapitre::new(
            vec![indice(&lisiere, INDICE_1), indice(&marais, INDICE_2)],
            baron,
        )
        .expect("chapitre du Baron conforme aux règles");
        // Le troisième indice se trouve au bout de l'expédition qui abat le
        // Baron, dans la Vallée, et ouvre l'Abbaye (section 9.4).
        let abbesse = Apotre::new(
            trouver("abbesse-ecorchee"),
            &abbaye,
            AVANT_L_ABBESSE,
            EPILOGUE_DE_L_ABBESSE,
        )
        .expect("Abbesse écorchée conforme aux règles")
        .avec_recit_de_l_ame(AME_DE_L_ABBESSE)
        .avec_ligne_du_carnet(CARNET_DE_L_ABBESSE);
        let chapitre_de_l_abbesse = Chapitre::new(
            vec![
                indice(&vallee, INDICE_3),
                indice(&bourg, INDICE_DU_BOURG),
                indice(&abbaye, INDICE_4),
            ],
            abbesse,
        )
        .expect("chapitre de l'Abbesse conforme aux règles");
        // Le cinquième indice se trouve au bout de l'expédition qui abat
        // l'Abbesse, dans l'Abbaye, et ouvre le Col (section 9.4).
        let margrave = Apotre::new(
            trouver("margrave-de-givre"),
            &col,
            AVANT_LE_MARGRAVE,
            EPILOGUE_DU_MARGRAVE,
        )
        .expect("Margrave de givre conforme aux règles")
        .avec_recit_de_l_ame(AME_DU_MARGRAVE)
        .avec_ligne_du_carnet(CARNET_DU_MARGRAVE);
        let chapitre_du_margrave = Chapitre::new(
            vec![
                indice(&abbaye, INDICE_5),
                indice(&bois, INDICE_DU_BOIS),
                indice(&col, INDICE_6),
            ],
            margrave,
        )
        .expect("chapitre du Margrave conforme aux règles");
        // Le septième indice se trouve au bout de l'expédition qui abat le
        // Margrave, dans le Col, et ouvre le Désert ; le huitième, laissé par
        // la Pythie, ouvre le Port, où le Contrebandier laisse le neuvième
        // (section 9.4).
        let amiral = Apotre::new(
            trouver("amiral-noye"),
            &port,
            AVANT_L_AMIRAL,
            EPILOGUE_DE_L_AMIRAL,
        )
        .expect("Amiral noyé conforme aux règles")
        .avec_recit_de_l_ame(AME_DE_L_AMIRAL)
        .avec_ligne_du_carnet(CARNET_DE_L_AMIRAL);
        let chapitre_de_l_amiral = Chapitre::new(
            vec![
                indice(&col, INDICE_7),
                indice(&desert, INDICE_8),
                indice(&port, INDICE_9),
            ],
            amiral,
        )
        .expect("chapitre de l'Amiral conforme aux règles");
        // Le dixième indice se trouve au bout de l'expédition qui abat
        // l'Amiral, au Port, et ouvre les Mines (section 9.4).
        let prevot = Apotre::new(
            trouver("prevot-des-fosses"),
            &mines,
            AVANT_LE_PREVOT,
            EPILOGUE_DU_PREVOT,
        )
        .expect("Prévôt des fosses conforme aux règles")
        .avec_recit_de_l_ame(AME_DU_PREVOT)
        .avec_ligne_du_carnet(CARNET_DU_PREVOT);
        let chapitre_du_prevot = Chapitre::new(
            vec![
                indice(&port, INDICE_10),
                indice(&mer, INDICE_DE_LA_MER),
                indice(&mines, INDICE_11),
            ],
            prevot,
        )
        .expect("chapitre du Prévôt conforme aux règles");
        // Le douzième indice se trouve au bout de l'expédition qui abat le
        // Prévôt, dans les Mines, et ouvre la Sylve ; la Chasseresse laisse
        // le treizième (section 9.4).
        let duchesse = Apotre::new(
            trouver("duchesse-sanguine"),
            &sylve,
            AVANT_LA_DUCHESSE,
            EPILOGUE_DE_LA_DUCHESSE,
        )
        .expect("Duchesse sanguine conforme aux règles")
        .avec_recit_de_l_ame(AME_DE_LA_DUCHESSE)
        .avec_ligne_du_carnet(CARNET_DE_LA_DUCHESSE);
        let chapitre_de_la_duchesse = Chapitre::new(
            vec![
                indice(&mines, INDICE_12),
                indice(&greve, INDICE_DE_LA_GREVE),
                indice(&sylve, INDICE_13),
            ],
            duchesse,
        )
        .expect("chapitre de la Duchesse conforme aux règles");
        // Le quatorzième indice se trouve au bout de l'expédition qui abat
        // la Duchesse, dans la Sylve, et ouvre les Catacombes (section 9.4).
        let eveque = Apotre::new(
            trouver("eveque-des-os"),
            &catacombes,
            AVANT_L_EVEQUE,
            EPILOGUE_DE_L_EVEQUE,
        )
        .expect("Évêque des os conforme aux règles")
        .avec_recit_de_l_ame(AME_DE_L_EVEQUE)
        .avec_ligne_du_carnet(CARNET_DE_L_EVEQUE);
        let chapitre_de_l_eveque = Chapitre::new(
            vec![
                indice(&sylve, INDICE_14),
                indice(&pavots, INDICE_DES_PAVOTS),
                indice(&catacombes, INDICE_15),
            ],
            eveque,
        )
        .expect("chapitre de l'Évêque conforme aux règles");
        // Le seizième indice se trouve au bout de l'expédition qui abat
        // l'Évêque, dans les Catacombes, et ouvre la Plaine ; le Capitaine
        // mercenaire laisse le dix-septième (section 9.4).
        let connetable = Apotre::new(
            trouver("connetable-boucher"),
            &plaine,
            AVANT_LE_CONNETABLE,
            EPILOGUE_DU_CONNETABLE,
        )
        .expect("Connétable boucher conforme aux règles")
        .avec_recit_de_l_ame(AME_DU_CONNETABLE)
        .avec_ligne_du_carnet(CARNET_DU_CONNETABLE);
        let chapitre_du_connetable = Chapitre::new(
            vec![
                indice(&catacombes, INDICE_16),
                indice(&remparts, INDICE_DES_REMPARTS),
                indice(&plaine, INDICE_17),
            ],
            connetable,
        )
        .expect("chapitre du Connétable conforme aux règles");
        // Le dix-huitième indice se trouve au bout de l'expédition qui abat
        // le Connétable, dans la Plaine, et ouvre la Haute-Cité (section 9.4).
        let chancelier = Apotre::new(
            trouver("chancelier-aux-mille-yeux"),
            &cite,
            AVANT_LE_CHANCELIER,
            EPILOGUE_DU_CHANCELIER,
        )
        .expect("Chancelier aux mille yeux conforme aux règles")
        .avec_recit_de_l_ame(AME_DU_CHANCELIER)
        .avec_ligne_du_carnet(CARNET_DU_CHANCELIER);
        let chapitre_du_chancelier = Chapitre::new(
            vec![
                indice(&plaine, INDICE_18),
                indice(&foret, INDICE_DE_LA_FORET),
                indice(&cite, INDICE_19),
            ],
            chancelier,
        )
        .expect("chapitre du Chancelier conforme aux règles");
        // Le vingtième indice se trouve au bout de l'expédition qui abat le
        // Chancelier, dans la Haute-Cité, et ouvre la Gorge ; le Pyromancien
        // laisse le vingt et unième (section 9.4).
        let prince = Apotre::new(
            trouver("prince-de-braise"),
            &gorge,
            AVANT_LE_PRINCE,
            EPILOGUE_DU_PRINCE,
        )
        .expect("Prince de braise conforme aux règles")
        .avec_recit_de_l_ame(AME_DU_PRINCE)
        .avec_ligne_du_carnet(CARNET_DU_PRINCE);
        let chapitre_du_prince = Chapitre::new(
            vec![
                indice(&cite, INDICE_20),
                indice(&sanctuaire, INDICE_DU_SANCTUAIRE),
                indice(&gorge, INDICE_21),
            ],
            prince,
        )
        .expect("chapitre du Prince conforme aux règles");
        // Le vingt-deuxième indice se trouve au bout de l'expédition qui abat
        // le Prince, dans la Gorge, et ouvre les Pics (section 9.4).
        let senechal = Apotre::new(
            trouver("senechal-des-orages"),
            &pics,
            AVANT_LE_SENECHAL,
            EPILOGUE_DU_SENECHAL,
        )
        .expect("Sénéchal des orages conforme aux règles")
        .avec_recit_de_l_ame(AME_DU_SENECHAL)
        .avec_ligne_du_carnet(CARNET_DU_SENECHAL);
        let chapitre_du_senechal = Chapitre::new(
            vec![
                indice(&gorge, INDICE_22),
                indice(&glacier, INDICE_DU_GLACIER),
                indice(&pics, INDICE_23),
            ],
            senechal,
        )
        .expect("chapitre du Sénéchal conforme aux règles");
        // Le vingt-quatrième indice se trouve au bout de l'expédition qui
        // abat le Sénéchal, dans les Pics, et ouvre le Palais ; la Favorite
        // laisse le vingt-cinquième (section 9.4).
        let reine = Apotre::new(
            trouver("reine-mere-de-chair"),
            &palais,
            AVANT_LA_REINE,
            EPILOGUE_DE_LA_REINE,
        )
        .expect("Reine-mère de chair conforme aux règles")
        .avec_recit_de_l_ame(AME_DE_LA_REINE)
        .avec_ligne_du_carnet(CARNET_DE_LA_REINE);
        let chapitre_de_la_reine = Chapitre::new(
            vec![
                indice(&pics, INDICE_24),
                indice(&cimetiere, INDICE_DU_CIMETIERE),
                indice(&palais, INDICE_25),
            ],
            reine,
        )
        .expect("chapitre de la Reine-mère conforme aux règles");
        // Le vingt-sixième indice se trouve au bout de l'expédition qui abat
        // la Reine-mère, dans le Palais, et ouvre le Puits (section 9.4).
        let roi = Apotre::new(
            trouver("roi-sans-visage"),
            &puits,
            AVANT_LE_ROI,
            EPILOGUE_DU_ROI,
        )
        .expect("Roi sans visage conforme aux règles")
        .avec_recit_de_l_ame(AME_DU_ROI)
        .avec_ligne_du_carnet(CARNET_DU_ROI);
        let chapitre_du_roi = Chapitre::new(
            vec![
                indice(&palais, INDICE_26),
                indice(&faille, INDICE_DE_LA_FAILLE),
                indice(&puits, INDICE_27),
            ],
            roi,
        )
        .expect("chapitre du Roi conforme aux règles");
        let piste = Piste::new(vec![
            chapitre_du_baron,
            chapitre_de_l_abbesse,
            chapitre_du_margrave,
            chapitre_de_l_amiral,
            chapitre_du_prevot,
            chapitre_de_la_duchesse,
            chapitre_de_l_eveque,
            chapitre_du_connetable,
            chapitre_du_chancelier,
            chapitre_du_prince,
            chapitre_du_senechal,
            chapitre_de_la_reine,
            chapitre_du_roi,
        ])
        .expect("piste conforme aux règles")
        // Le Béhérit attend les âmes des 13 Apôtres de la traque complète
        // (section 9.4).
        .avec_apotres_attendus(APOTRES_DE_LA_TRAQUE);
        let ressource = |id: &str| {
            especes
                .iter()
                .filter_map(Espece::ressource)
                .find(|ressource| ressource.id().as_str() == id)
                .cloned()
                .unwrap_or_else(|| panic!("ressource {id} absente du contenu"))
        };
        let plans_d_amelioration = PLANS_D_AMELIORATION
            .iter()
            .map(|(amelioration, niveau, cout, heures)| {
                let cout = cout
                    .iter()
                    .map(|(id, nombre)| (ressource(id), *nombre))
                    .collect();
                PlanDAmelioration::new(
                    *amelioration,
                    *niveau,
                    cout,
                    Duration::from_secs(heures * 3_600),
                )
                .expect("plan d'amélioration conforme aux règles")
            })
            .collect();
        let plans_de_fabrication = PLANS_DE_FABRICATION
            .iter()
            .map(|fiche| {
                let cout = fiche
                    .cout
                    .iter()
                    .map(|(id, nombre)| (ressource(id), *nombre))
                    .collect();
                PlanDeFabrication::new(
                    creer_objet(fiche),
                    cout,
                    Duration::from_secs(fiche.heures * 3_600),
                )
                .expect("plan de fabrication conforme aux règles")
            })
            .collect();
        // Le Démiurge attend seul dans l'Éclipse, deux niveaux au-dessus de
        // ce lieu d'une étape, comme un Apôtre : au niveau 100, en combat
        // final (section 9.4). Ses paramètres d'expédition automatique et de
        // coffre ne servent jamais.
        let eclipse = Zone::new(
            ZoneId::new("eclipse"),
            "Éclipse",
            (niveau(98), niveau(98)),
            1,
            1,
            vec![trouver("demiurge")],
            Vec::new(),
            expedition_automatique(5, 220..=450, 2200..=3400, 1..=3, 180..=320),
            coffre(140..=220, Vec::new()),
        )
        .expect("Éclipse conforme aux règles");
        let demiurge = Apotre::new(
            trouver("demiurge"),
            &eclipse,
            AVANT_LE_DEMIURGE,
            EPILOGUE_DU_DEMIURGE,
        )
        .and_then(|apotre| apotre.avec_seconde_forme(trouver("demiurge-eveille")))
        .expect("Démiurge conforme aux règles")
        .avec_ligne_du_carnet(CARNET_DU_DEMIURGE);
        let invocation = Invocation::new(eclipse, demiurge, FORGE_DU_BEHERIT, SACRIFICE);
        // Les zones légendaires, au niveau 100, s'ouvrent à la chute du
        // Démiurge ; leurs monstres sauvages reprennent des espèces des
        // zones de même thème (section 9.10). Aucune n'a d'expédition
        // automatique, faute de compter comme explorée.
        let zone_legendaire =
            |id: &str,
             nom: &str,
             especes: &[&str],
             rares: Vec<EspeceRare>,
             meteos: Vec<Meteo>,
             (gardien, avant, capture): (&str, &str, &str)| {
                let zone = Zone::new(
                    ZoneId::new(id),
                    nom,
                    (niveau(100), niveau(100)),
                    5,
                    3,
                    especes.iter().map(|espece| trouver(espece)).collect(),
                    rares,
                    expedition_automatique(16, 890..=1760, 14800..=20200, 1..=3, 790..=1180),
                    coffre(640..=960, Vec::new()),
                )
                .map(|zone| zone.avec_meteos(meteos))
                .expect("zone légendaire conforme aux règles");
                let legendaire = Legendaire::new(trouver(gardien), niveau(100), avant, capture)
                    .expect("légendaire conforme aux règles");
                ZoneLegendaire::new(zone, legendaire)
            };
        let zones_legendaires = vec![
            zone_legendaire(
                "abysses",
                "Mer sans rivage",
                &["sirene-putride", "crabe-necrophage"],
                vec![EspeceRare {
                    espece: trouver("hydre-des-marees"),
                    pacifique: false,
                }],
                vec![Meteo::Pluie, Meteo::Cyclone],
                ("leviathan", AVANT_LEVIATHAN, CAPTURE_DE_LEVIATHAN),
            ),
            zone_legendaire(
                "terres-arides",
                "Terres arides",
                &["crotale-des-salines", "ver-des-salines", "scarabee-solaire"],
                Vec::new(),
                vec![Meteo::TempeteDeSable, Meteo::Canicule],
                ("behemoth", AVANT_BEHEMOTH, CAPTURE_DE_BEHEMOTH),
            ),
            zone_legendaire(
                "ciel-dechire",
                "Ciel déchiré",
                &["harpie-des-orages", "aigle-tonnerre", "feu-de-saint-elme"],
                Vec::new(),
                vec![Meteo::Tempete, Meteo::Grele],
                ("ziz", AVANT_ZIZ, CAPTURE_DE_ZIZ),
            ),
        ];
        let zones = vec![
            lisiere, marais, vallee, bourg, abbaye, bois, col, desert, port, mer, mines, greve,
            sylve, pavots, catacombes, remparts, plaine, foret, cite, sanctuaire, gorge, glacier,
            pics, cimetiere, palais, faille, puits,
        ];
        // Les Abysses, au fond de la Tour : les rares hostiles de toutes les
        // zones y forment les groupes, et les rares pacifiques y sont les
        // monstres rares (section 9.5, étape 204).
        let mut especes_des_abysses: Vec<Espece> = Vec::new();
        let mut rares_des_abysses: Vec<EspeceRare> = Vec::new();
        for rare in zones.iter().flat_map(|zone| zone.rares()) {
            if rare.pacifique {
                if !rares_des_abysses
                    .iter()
                    .any(|deja| deja.espece.id() == rare.espece.id())
                {
                    rares_des_abysses.push(rare.clone());
                }
            } else if !especes_des_abysses
                .iter()
                .any(|deja| deja.id() == rare.espece.id())
            {
                especes_des_abysses.push(rare.espece.clone());
            }
        }
        let attitre = |id: &str, recit: &str| GardienAttitre {
            espece: trouver(id),
            recit: recit.to_string(),
        };
        let tour = TourSansFond::new(
            ZoneId::new("tour-sans-fond"),
            "Tour sans fond",
            &zones,
            schnoz(),
            ENTREE_DANS_LA_TOUR,
            AVANT_LE_GARDIEN,
            REPOS_DANS_LA_TOUR,
        )
        .expect("Tour sans fond conforme aux règles")
        .avec_abysses(
            especes_des_abysses,
            rares_des_abysses,
            ENTREE_DANS_LES_ABYSSES,
        )
        .avec_gardien_attitre(40, attitre("colosse-du-rempart", AVANT_LE_COLOSSE))
        .avec_gardien_attitre(50, attitre("hellequin", AVANT_LE_HELLEQUIN))
        .avec_elus(
            ELUS_DE_LA_MAIN
                .iter()
                .map(|(id, recit)| attitre(id, recit))
                .collect(),
        );
        let fosse = creer_la_fosse(&trouver);
        let matieres: Vec<(Ressource, u32)> = MATIERES_PECHEES
            .iter()
            .map(|(id, nom, prix, chance)| {
                (Ressource::new(RessourceId::new(*id), *nom, *prix), *chance)
            })
            .collect();
        let peche = ParametresPeche::new(
            Duration::from_secs(HEURES_DE_PECHE * 3_600),
            ESPECES_PECHEES.iter().map(|id| trouver(id)).collect(),
            trouver(ESPECE_PECHEE_RARE),
            &zones,
        )
        .and_then(|peche| peche.avec_matieres(matieres.clone()))
        .expect("pêche conforme aux règles");
        let matiere = |id: &str| {
            matieres
                .iter()
                .map(|(matiere, _)| matiere)
                .find(|matiere| matiere.id().as_str() == id)
                .cloned()
                .unwrap_or_else(|| panic!("matière {id} absente du contenu"))
        };
        let recette = |cout: &[(&str, u32)]| -> Vec<(Ressource, u32)> {
            cout.iter()
                .map(|(id, nombre)| (matiere(id), *nombre))
                .collect()
        };
        // Le prix des cannes à pêche (section 13.8), proposé par Claude
        // (Q123) : de l'argent, du métal pour la canne en fer, de l'ambre
        // pour la canne en argent.
        let peche = peche.avec_cannes(vec![
            (CanneAPeche::Bois, 50, Vec::new()),
            (
                CanneAPeche::Fer,
                300,
                vec![(ressource("fragment-d-armure"), 15)],
            ),
            (CanneAPeche::Argent, 1_000, recette(&[("ambre-noir", 3)])),
        ]);
        let atelier = AtelierDAnsgar::new(
            RECETTES_DES_APPATS
                .iter()
                .map(|(appat, cout)| (*appat, recette(cout)))
                .collect(),
            recette(&RECETTE_DU_PARFUM_DE_CHAROGNE),
            recette(&BASE_DU_PARFUM_D_ESPECE),
        )
        .expect("atelier d'Ansgar conforme aux règles");
        let plan_du_charme = PlanDuCharme::new(
            COUT_DU_CHARME
                .iter()
                .map(|(id, nombre)| (ressource(id), *nombre))
                .collect(),
            Duration::from_secs(MINUTES_DU_CHARME * 60),
        )
        .expect("plan du charme conforme aux règles");
        let plans_des_filets = PLANS_DES_FILETS
            .iter()
            .map(|(filet, cout, minutes)| {
                PlanDesFilets::new(
                    *filet,
                    FILETS_PAR_LOT,
                    cout.iter()
                        .map(|(id, nombre)| (ressource(id), *nombre))
                        .collect(),
                    Duration::from_secs(minutes * 60),
                )
                .expect("plan des filets conforme aux règles")
            })
            .collect();
        let mut contenu = Self {
            especes,
            zones,
            piste,
            capsules,
            plans_d_amelioration,
            plans_de_fabrication,
            armes_du_champion: ARMES_DU_CHAMPION.iter().map(creer_objet).collect(),
            plan_du_charme,
            plans_des_filets,
            recompenses_du_codex: creer_recompenses_du_codex(),
            peche,
            atelier,
            invocation,
            tour,
            fosse,
            annexes: vec![
                epave,
                filon,
                autel,
                crypte,
                tertre,
                bibliotheque,
                coeur,
                aire,
                jardin,
                racines,
            ],
            zones_legendaires,
            succes: Vec::new(),
            quetes_du_camp: Vec::new(),
        };
        contenu.succes = creer_succes(contenu.annexes.len(), PALIERS_DE_LA_FOSSE);
        contenu.quetes_du_camp = creer_quetes(&contenu.especes, &contenu.zones, &contenu.annexes);
        contenu
    }

    pub fn succes(&self) -> &[Succes] {
        &self.succes
    }

    pub fn quetes_du_camp(&self) -> &[Quete] {
        &self.quetes_du_camp
    }

    pub fn annexes(&self) -> &[Annexe] {
        &self.annexes
    }

    /// Pour les tests, avant que le contenu ait ses propres annexes.
    #[cfg(test)]
    pub(crate) fn avec_annexes(mut self, annexes: Vec<Annexe>) -> Self {
        self.annexes = annexes;
        self
    }

    /// Pour les tests : une quête dans une zone qu'une partie de test atteint.
    #[cfg(test)]
    pub(crate) fn avec_quetes_du_camp(mut self, quetes: Vec<Quete>) -> Self {
        self.quetes_du_camp = quetes;
        self
    }

    /// Pour les tests, avant que le contenu ait ses zones légendaires.
    #[cfg(test)]
    pub(crate) fn avec_zones_legendaires(mut self, zones: Vec<ZoneLegendaire>) -> Self {
        self.zones_legendaires = zones;
        self
    }

    pub fn zones_legendaires(&self) -> &[ZoneLegendaire] {
        &self.zones_legendaires
    }

    /// L'arène où le héros affronte seul des humains, de palier en palier
    /// (section 9.8).
    pub fn fosse(&self) -> &FosseAuxDamnes {
        &self.fosse
    }

    /// Le lieu hors de la carte où l'on descend le plus loin possible,
    /// seul ou à deux (section 9.5).
    pub fn tour(&self) -> &TourSansFond {
        &self.tour
    }

    /// L'Éclipse et le Démiurge, que le Béhérit pourpre fait venir (section
    /// 9.4).
    pub fn invocation(&self) -> &Invocation {
        &self.invocation
    }

    /// Toutes les espèces, dans l'ordre du codex.
    pub fn especes(&self) -> &[Espece] {
        &self.especes
    }

    /// Toutes les ressources, celles des espèces dans l'ordre du codex puis
    /// les matières de la pêche, chacune une fois : un ordre fixe, qui
    /// permet de les désigner par leur rang.
    pub fn ressources(&self) -> Vec<Ressource> {
        let mut ressources: Vec<Ressource> = Vec::new();
        for ressource in self
            .especes
            .iter()
            .filter_map(Espece::ressource)
            .chain(self.peche.matieres().iter().map(|(matiere, _)| matiere))
        {
            if !ressources
                .iter()
                .any(|presente| presente.id() == ressource.id())
            {
                ressources.push(ressource.clone());
            }
        }
        ressources
    }

    pub fn especes_de_depart(&self) -> &[Espece] {
        &self.especes[..NOMBRE_DE_DEPARTS]
    }

    /// Chaque classe a son monstre de départ, qui reprend son profil
    /// (section 6).
    pub fn espece_de_depart(&self, classe: Classe) -> &Espece {
        let id = match classe {
            Classe::Guerrier => "crapaud",
            Classe::Mage => "salamandre",
            Classe::Templier => "mandragore",
            Classe::Assassin => "chat-noir",
        };
        self.especes_de_depart()
            .iter()
            .find(|espece| espece.id().as_str() == id)
            .expect("chaque classe a son monstre de départ dans le contenu")
    }

    /// De la plus facile à la plus difficile, dans l'ordre de la carte.
    pub fn zones(&self) -> &[Zone] {
        &self.zones
    }

    pub fn piste(&self) -> &Piste {
        &self.piste
    }

    /// Ce que coûte chaque niveau d'amélioration du camp (section 13.4), dans
    /// l'ordre de docs/contenu-v1.md, section 9.
    pub fn plans_d_amelioration(&self) -> &[PlanDAmelioration] {
        &self.plans_d_amelioration
    }

    pub fn plan_d_amelioration(
        &self,
        amelioration: Amelioration,
        niveau: u8,
    ) -> Option<&PlanDAmelioration> {
        self.plans_d_amelioration
            .iter()
            .find(|plan| plan.amelioration() == amelioration && plan.niveau() == niveau)
    }

    /// Les objets d'équipement que Gunther fabrique (section 13.5) : les
    /// armes, puis les armures de chaque classe, de docs/contenu-v1.md,
    /// section 34, et les talismans de la section 11.
    pub fn plans_de_fabrication(&self) -> &[PlanDeFabrication] {
        &self.plans_de_fabrication
    }

    /// Ceux qu'il fabrique pour un héros de cette classe : ses armes, ses
    /// armures et les talismans (section 4.3).
    pub fn plans_de_fabrication_pour(&self, classe: Classe) -> Vec<&PlanDeFabrication> {
        self.plans_de_fabrication
            .iter()
            .filter(|plan| plan.objet().convient_a(classe))
            .collect()
    }

    /// L'arme du palier 4 de cette classe, que donne le dernier palier de la
    /// Fosse aux damnés (section 9.8).
    pub fn arme_du_champion(&self, classe: Classe) -> &ObjetDEquipement {
        self.armes_du_champion
            .iter()
            .find(|arme| arme.classe() == Some(classe))
            .expect("une arme du champion par classe")
    }

    /// Le charme de garde, que Gunther forge à l'unité (section 13.5).
    pub fn plan_du_charme(&self) -> &PlanDuCharme {
        &self.plan_du_charme
    }

    /// Les lots de filets que Gunther forge, du plus simple au plus
    /// efficace (section 13.5).
    pub fn plans_des_filets(&self) -> &[PlanDesFilets] {
        &self.plans_des_filets
    }

    /// Dans l'ordre des paliers (section 5.6).
    pub fn recompenses_du_codex(&self) -> &[RecompenseDuCodex] {
        &self.recompenses_du_codex
    }

    pub fn plan_des_filets(&self, filet: Filet) -> Option<&PlanDesFilets> {
        self.plans_des_filets
            .iter()
            .find(|plan| plan.filet() == filet)
    }

    pub fn plan_de_fabrication(&self, objet: &ObjetDEquipementId) -> Option<&PlanDeFabrication> {
        self.plans_de_fabrication
            .iter()
            .find(|plan| plan.objet().id() == objet)
    }

    /// Toutes les capsules d'attaque, que le marchand vend (section 14.2),
    /// dans l'ordre de docs/contenu-v1.md.
    /// Ce que remonte la ligne d'Ansgar (section 13.8).
    pub fn peche(&self) -> &ParametresPeche {
        &self.peche
    }

    /// Ce que prépare Ansgar (section 13.8).
    pub fn atelier(&self) -> &AtelierDAnsgar {
        &self.atelier
    }

    pub fn capsules(&self) -> &[Capsule] {
        &self.capsules
    }
}

impl Default for ContenuV1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Catalogue for ContenuV1 {
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

    /// Celle d'une espèce, ou une matière de la pêche (section 13.8).
    fn ressource(&self, id: &RessourceId) -> Option<Ressource> {
        self.especes
            .iter()
            .filter_map(Espece::ressource)
            .chain(self.peche.matieres().iter().map(|(matiere, _)| matiere))
            .find(|ressource| ressource.id() == id)
            .cloned()
    }

    fn objet_de_quete(&self, quete: &QueteId) -> Option<ObjetDeQuete> {
        self.quetes_du_camp
            .iter()
            .filter_map(Quete::exploration)
            .map(Exploration::objet)
            .find(|objet| objet.quete() == quete)
            .cloned()
    }

    /// Un objet fabriqué, un objet unique des récompenses du codex, ou une
    /// arme du palier 4, récompense de la Fosse aux damnés.
    fn objet_d_equipement(&self, id: &ObjetDEquipementId) -> Option<ObjetDEquipement> {
        self.plan_de_fabrication(id)
            .map(|plan| plan.objet().clone())
            .or_else(|| {
                self.recompenses_du_codex
                    .iter()
                    .filter_map(|recompense| recompense.objet_unique.as_ref())
                    .chain(&self.armes_du_champion)
                    .find(|objet| objet.id() == id)
                    .cloned()
            })
    }

    fn objet_de_classe(
        &self,
        classe: Classe,
        sorte: SorteDObjet,
        palier: u8,
    ) -> Option<ObjetDEquipement> {
        self.plans_de_fabrication
            .iter()
            .map(PlanDeFabrication::objet)
            .chain(&self.armes_du_champion)
            .find(|objet| {
                objet.classe() == Some(classe) && objet.sorte() == sorte && objet.palier() == palier
            })
            .cloned()
    }

    fn marchand_mort(&self, id: &MarchandId) -> Option<MarchandMort> {
        self.zones
            .iter()
            .filter_map(Zone::marchand_mort)
            .find(|marchand| marchand.id() == id)
            .cloned()
    }
}

struct FicheAttaque {
    id: &'static str,
    nom: &'static str,
    type_attaque: Type,
    categorie: Categorie,
    puissance: u16,
    precision: u8,
    effet: Option<(Statut, u8)>,
}

const fn fiche(
    id: &'static str,
    nom: &'static str,
    type_attaque: Type,
    categorie: Categorie,
    puissance: u16,
    precision: u8,
) -> FicheAttaque {
    FicheAttaque {
        id,
        nom,
        type_attaque,
        categorie,
        puissance,
        precision,
        effet: None,
    }
}

const fn avec_effet(fiche: FicheAttaque, statut: Statut, chance: u8) -> FicheAttaque {
    FicheAttaque {
        effet: Some((statut, chance)),
        ..fiche
    }
}

/// Section 34 de docs/contenu-v1.md : les attaques que les armes donnent au
/// héros, toutes de type Humain, classe par classe. Coup de poing, celle du
/// héros sans arme, est dans le module du héros.
const ATTAQUES_DES_HEROS: [FicheAttaque; 40] = [
    fiche("taille", "Taille", Type::Humain, Physique, 40, 100),
    fiche("estoc", "Estoc", Type::Humain, Physique, 50, 100),
    fiche("pourfendre", "Pourfendre", Type::Humain, Physique, 60, 100),
    fiche(
        "coup-du-champion",
        "Coup du champion",
        Type::Humain,
        Physique,
        75,
        100,
    ),
    fiche(
        "frappe-brutale",
        "Frappe brutale",
        Type::Humain,
        Physique,
        70,
        80,
    ),
    fiche("entaille", "Entaille", Type::Humain, Physique, 40, 100),
    fiche("coup-bas", "Coup bas", Type::Humain, Physique, 50, 100),
    fiche("egorgement", "Égorgement", Type::Humain, Physique, 60, 100),
    fiche("execution", "Exécution", Type::Humain, Physique, 75, 100),
    avec_effet(
        fiche(
            "lame-empoisonnee",
            "Lame empoisonnée",
            Type::Humain,
            Physique,
            30,
            100,
        ),
        Statut::Poison,
        50,
    ),
    fiche(
        "trait-arcanique",
        "Trait arcanique",
        Type::Humain,
        Speciale,
        40,
        100,
    ),
    fiche(
        "fleche-occulte",
        "Flèche occulte",
        Type::Humain,
        Speciale,
        50,
        100,
    ),
    fiche(
        "rayon-occulte",
        "Rayon occulte",
        Type::Humain,
        Speciale,
        60,
        100,
    ),
    fiche(
        "tempete-occulte",
        "Tempête occulte",
        Type::Humain,
        Speciale,
        75,
        100,
    ),
    avec_effet(
        fiche(
            "onde-engourdissante",
            "Onde engourdissante",
            Type::Humain,
            Speciale,
            30,
            100,
        ),
        Statut::Paralysie,
        40,
    ),
    fiche("chatiment", "Châtiment", Type::Humain, Speciale, 40, 100),
    fiche("verdict", "Verdict", Type::Humain, Speciale, 50, 100),
    fiche("anatheme", "Anathème", Type::Humain, Speciale, 60, 100),
    fiche(
        "courroux-sacre",
        "Courroux sacré",
        Type::Humain,
        Speciale,
        75,
        100,
    ),
    avec_effet(
        fiche(
            "sceau-d-entrave",
            "Sceau d'entrave",
            Type::Humain,
            Speciale,
            30,
            100,
        ),
        Statut::Paralysie,
        30,
    ),
    // L'attaque principale du palier 5 de chaque classe (étape 163,
    // section 4.3).
    fiche(
        "fend-armure",
        "Fend-armure",
        Type::Humain,
        Physique,
        85,
        100,
    ),
    fiche(
        "coup-mortel",
        "Coup mortel",
        Type::Humain,
        Physique,
        85,
        100,
    ),
    fiche(
        "tempete-arcanique",
        "Tempête arcanique",
        Type::Humain,
        Speciale,
        85,
        100,
    ),
    fiche(
        "colere-du-ciel",
        "Colère du ciel",
        Type::Humain,
        Speciale,
        85,
        100,
    ),
    // Celle du palier 6 (étape 165).
    fiche(
        "decapitation",
        "Décapitation",
        Type::Humain,
        Physique,
        95,
        100,
    ),
    fiche(
        "lame-fantome",
        "Lame fantôme",
        Type::Humain,
        Physique,
        95,
        100,
    ),
    fiche(
        "arcane-interdit",
        "Arcane interdit",
        Type::Humain,
        Speciale,
        95,
        100,
    ),
    fiche(
        "jugement-dernier",
        "Jugement dernier",
        Type::Humain,
        Speciale,
        95,
        100,
    ),
    // Celle du palier 7 (étape 166).
    fiche("brise-ost", "Brise-ost", Type::Humain, Physique, 105, 100),
    fiche(
        "mille-coupures",
        "Mille coupures",
        Type::Humain,
        Physique,
        105,
        100,
    ),
    fiche("cataclysme", "Cataclysme", Type::Humain, Speciale, 105, 100),
    fiche(
        "lumiere-divine",
        "Lumière divine",
        Type::Humain,
        Speciale,
        105,
        100,
    ),
    // Celle du palier 8 (étape 167).
    fiche("ravage", "Ravage", Type::Humain, Physique, 115, 100),
    fiche(
        "mort-silencieuse",
        "Mort silencieuse",
        Type::Humain,
        Physique,
        115,
        100,
    ),
    fiche("apocalypse", "Apocalypse", Type::Humain, Speciale, 115, 100),
    fiche(
        "glaive-celeste",
        "Glaive céleste",
        Type::Humain,
        Speciale,
        115,
        100,
    ),
    // L'attaque du second type de chaque classe, des armes des paliers 2 à
    // 5 (étapes 149 et 163, section 4.4).
    fiche(
        "coup-de-pommeau",
        "Coup de pommeau",
        Type::Combat,
        Physique,
        60,
        100,
    ),
    fiche(
        "lame-des-tenebres",
        "Lame des ténèbres",
        Type::Tenebres,
        Physique,
        60,
        100,
    ),
    fiche(
        "emprise-mentale",
        "Emprise mentale",
        Type::Psychique,
        Speciale,
        60,
        100,
    ),
    fiche(
        "rayon-de-foi",
        "Rayon de foi",
        Type::Lumiere,
        Speciale,
        60,
        100,
    ),
];

/// Section 1 de docs/contenu-v1.md, dans le même ordre.
const ATTAQUES: [FicheAttaque; 336] = [
    fiche(
        "souffle-chaud",
        "Souffle chaud",
        Type::Normal,
        Speciale,
        40,
        100,
    ),
    fiche(
        "crachat-de-braises",
        "Crachat de braises",
        Type::Feu,
        Speciale,
        40,
        100,
    ),
    fiche(
        "souffle-draconique",
        "Souffle draconique",
        Type::Dragon,
        Speciale,
        60,
        100,
    ),
    fiche("flamme-vive", "Flamme vive", Type::Feu, Speciale, 70, 90),
    fiche("fournaise", "Fournaise", Type::Feu, Speciale, 90, 85),
    fiche(
        "charge-gluante",
        "Charge gluante",
        Type::Normal,
        Physique,
        40,
        100,
    ),
    fiche("plongeon", "Plongeon", Type::Eau, Physique, 40, 100),
    avec_effet(
        fiche(
            "coup-de-langue",
            "Coup de langue",
            Type::Normal,
            Physique,
            55,
            95,
        ),
        Paralysie,
        10,
    ),
    avec_effet(
        fiche(
            "langue-venimeuse",
            "Langue venimeuse",
            Type::Poison,
            Physique,
            50,
            100,
        ),
        Poison,
        30,
    ),
    fiche("lame-de-fond", "Lame de fond", Type::Eau, Physique, 80, 85),
    fiche(
        "coup-de-racine",
        "Coup de racine",
        Type::Normal,
        Physique,
        40,
        100,
    ),
    fiche(
        "feuille-tranchante",
        "Feuille tranchante",
        Type::Plante,
        Speciale,
        40,
        100,
    ),
    avec_effet(
        fiche(
            "cri-strident",
            "Cri strident",
            Type::Normal,
            Speciale,
            45,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche("ronces", "Ronces", Type::Plante, Physique, 65, 95),
    avec_effet(
        fiche(
            "spores-putrides",
            "Spores putrides",
            Type::Poison,
            Speciale,
            55,
            100,
        ),
        Poison,
        30,
    ),
    fiche("griffure", "Griffure", Type::Normal, Physique, 40, 100),
    fiche("morsure", "Morsure", Type::Tenebres, Physique, 60, 90),
    fiche("curee", "Curée", Type::Normal, Physique, 75, 85),
    fiche("coup-de-bec", "Coup de bec", Type::Vol, Physique, 35, 100),
    fiche("pique", "Piqué", Type::Vol, Physique, 60, 90),
    avec_effet(
        fiche(
            "regard-funeste",
            "Regard funeste",
            Type::Tenebres,
            Speciale,
            50,
            100,
        ),
        Paralysie,
        10,
    ),
    avec_effet(
        fiche(
            "morsure-venimeuse",
            "Morsure venimeuse",
            Type::Poison,
            Physique,
            40,
            100,
        ),
        Poison,
        30,
    ),
    avec_effet(
        fiche(
            "regard-petrifiant",
            "Regard pétrifiant",
            Type::Normal,
            Speciale,
            30,
            100,
        ),
        Paralysie,
        30,
    ),
    fiche("etreinte", "Étreinte", Type::Normal, Physique, 60, 95),
    avec_effet(
        fiche(
            "lueur-trompeuse",
            "Lueur trompeuse",
            Type::Spectre,
            Speciale,
            40,
            100,
        ),
        Paralysie,
        10,
    ),
    fiche(
        "frisson-d-outre-tombe",
        "Frisson d'outre-tombe",
        Type::Spectre,
        Speciale,
        65,
        100,
    ),
    fiche(
        "coup-de-garde",
        "Coup de garde",
        Type::Acier,
        Physique,
        40,
        100,
    ),
    avec_effet(
        fiche(
            "lame-rouillee",
            "Lame rouillée",
            Type::Acier,
            Physique,
            65,
            95,
        ),
        Poison,
        20,
    ),
    fiche(
        "charge-spectrale",
        "Charge spectrale",
        Type::Spectre,
        Physique,
        70,
        90,
    ),
    fiche(
        "coup-de-bois",
        "Coup de bois",
        Type::Normal,
        Physique,
        40,
        100,
    ),
    fiche("ruade", "Ruade", Type::Normal, Physique, 60, 95),
    fiche(
        "lumiere-d-aube",
        "Lumière d'aube",
        Type::Feerique,
        Speciale,
        65,
        100,
    ),
    fiche(
        "souffle-putride",
        "Souffle putride",
        Type::Dragon,
        Speciale,
        70,
        95,
    ),
    avec_effet(
        fiche(
            "croc-de-venin",
            "Croc de venin",
            Type::Poison,
            Physique,
            75,
            90,
        ),
        Poison,
        30,
    ),
    avec_effet(
        fiche(
            "corde-du-gibet",
            "Corde du gibet",
            Type::Spectre,
            Physique,
            60,
            100,
        ),
        Paralysie,
        20,
    ),
    avec_effet(
        fiche(
            "lamentation",
            "Lamentation",
            Type::Spectre,
            Speciale,
            70,
            100,
        ),
        Paralysie,
        10,
    ),
    fiche("sentence", "Sentence", Type::Tenebres, Speciale, 80, 95),
    fiche(
        "trappe-du-gibet",
        "Trappe du gibet",
        Type::Tenebres,
        Physique,
        90,
        85,
    ),
    // Attaques des capsules, qu'aucune espèce n'apprend (section 6).
    fiche("brasier", "Brasier", Type::Feu, Speciale, 70, 100),
    fiche("deferlante", "Déferlante", Type::Eau, Physique, 65, 100),
    avec_effet(
        fiche(
            "lianes-etrangleuses",
            "Lianes étrangleuses",
            Type::Plante,
            Physique,
            55,
            100,
        ),
        Paralysie,
        20,
    ),
    // Attaques du Chat noir, monstre de l'Assassin.
    fiche(
        "coup-sournois",
        "Coup sournois",
        Type::Tenebres,
        Physique,
        40,
        100,
    ),
    avec_effet(
        fiche("feulement", "Feulement", Type::Normal, Speciale, 40, 100),
        Paralysie,
        20,
    ),
    fiche(
        "coup-de-grace",
        "Coup de grâce",
        Type::Tenebres,
        Physique,
        85,
        90,
    ),
    // Attaques du Schnoz, monstre caché.
    fiche(
        "eclat-astral",
        "Éclat astral",
        Type::Stellaire,
        Speciale,
        40,
        100,
    ),
    fiche(
        "rayon-astral",
        "Rayon astral",
        Type::Stellaire,
        Speciale,
        75,
        100,
    ),
    // Attaques des monstres de la Vallée des Brumes.
    fiche(
        "bourdonnement",
        "Bourdonnement",
        Type::Insecte,
        Speciale,
        40,
        100,
    ),
    fiche("malefice", "Maléfice", Type::Tenebres, Speciale, 65, 100),
    fiche(
        "poussiere-d-elfe",
        "Poussière d'elfe",
        Type::Feerique,
        Speciale,
        40,
        100,
    ),
    fiche("bourrasque", "Bourrasque", Type::Vol, Speciale, 55, 100),
    fiche(
        "frappe-d-os",
        "Frappe d'os",
        Type::Combat,
        Physique,
        40,
        100,
    ),
    fiche(
        "furie-draconique",
        "Furie draconique",
        Type::Dragon,
        Physique,
        85,
        90,
    ),
    // Attaques Obscur du Baron pendu.
    fiche(
        "etreinte-obscure",
        "Étreinte obscure",
        Type::Obscur,
        Physique,
        80,
        95,
    ),
    avec_effet(
        fiche(
            "hurlement-des-abysses",
            "Hurlement des abysses",
            Type::Obscur,
            Speciale,
            80,
            100,
        ),
        Paralysie,
        10,
    ),
    // Attaques d'évolution (section 5.4).
    fiche("jet-de-lave", "Jet de lave", Type::Feu, Speciale, 85, 90),
    fiche("trombe", "Trombe", Type::Eau, Physique, 85, 90),
    avec_effet(
        fiche(
            "hurlement-de-mandragore",
            "Hurlement de mandragore",
            Type::Plante,
            Speciale,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "griffe-d-ombre",
        "Griffe d'ombre",
        Type::Tenebres,
        Physique,
        80,
        95,
    ),
    fiche("croc-noir", "Croc noir", Type::Tenebres, Physique, 75, 95),
    fiche("bec-funeste", "Bec funeste", Type::Vol, Physique, 75, 95),
    avec_effet(
        fiche("venin-noir", "Venin noir", Type::Poison, Speciale, 75, 100),
        Poison,
        30,
    ),
    fiche("nuee", "Nuée", Type::Insecte, Speciale, 75, 100),
    fiche(
        "flamme-funebre",
        "Flamme funèbre",
        Type::Spectre,
        Speciale,
        80,
        100,
    ),
    fiche(
        "rayon-de-lune",
        "Rayon de lune",
        Type::Feerique,
        Speciale,
        85,
        100,
    ),
    fiche(
        "taille-d-acier",
        "Taille d'acier",
        Type::Acier,
        Physique,
        85,
        90,
    ),
    fiche(
        "epee-des-morts",
        "Épée des morts",
        Type::Combat,
        Physique,
        85,
        95,
    ),
    fiche(
        "regard-de-mort",
        "Regard de mort",
        Type::Poison,
        Speciale,
        85,
        90,
    ),
    // Abbaye des Cendres (étape 45).
    fiche(
        "griffe-de-pierre",
        "Griffe de pierre",
        Type::Roche,
        Physique,
        50,
        100,
    ),
    fiche(
        "chute-de-pierres",
        "Chute de pierres",
        Type::Roche,
        Physique,
        75,
        90,
    ),
    fiche(
        "festin-macabre",
        "Festin macabre",
        Type::Tenebres,
        Physique,
        80,
        90,
    ),
    avec_effet(
        fiche("ultrason", "Ultrason", Type::Normal, Speciale, 70, 100),
        Paralysie,
        10,
    ),
    fiche(
        "onde-psychique",
        "Onde psychique",
        Type::Psychique,
        Speciale,
        65,
        100,
    ),
    fiche("jugement", "Jugement", Type::Psychique, Speciale, 90, 90),
    fiche("bucher", "Bûcher", Type::Feu, Speciale, 90, 85),
    // Attaques Obscur de l'Abbesse écorchée (étape 46).
    fiche(
        "flagellation",
        "Flagellation",
        Type::Obscur,
        Physique,
        85,
        95,
    ),
    avec_effet(
        fiche(
            "psaume-impie",
            "Psaume impie",
            Type::Obscur,
            Speciale,
            90,
            95,
        ),
        Paralysie,
        20,
    ),
    // Attaque d'évolution de la Reine des ronces (étape 47).
    avec_effet(
        fiche(
            "ronces-maudites",
            "Ronces maudites",
            Type::Plante,
            Speciale,
            85,
            90,
        ),
        Poison,
        20,
    ),
    // Attaques du Col du Givre-Noir et du Margrave de givre (étape 79).
    avec_effet(
        fiche(
            "croc-de-givre",
            "Croc de givre",
            Type::Glace,
            Physique,
            60,
            100,
        ),
        Paralysie,
        10,
    ),
    fiche("avalanche", "Avalanche", Type::Glace, Physique, 85, 90),
    fiche(
        "souffle-glacial",
        "Souffle glacial",
        Type::Glace,
        Speciale,
        70,
        100,
    ),
    avec_effet(
        fiche("blizzard", "Blizzard", Type::Glace, Speciale, 90, 85),
        Paralysie,
        10,
    ),
    fiche("secousse", "Secousse", Type::Sol, Physique, 60, 100),
    fiche("seisme", "Séisme", Type::Sol, Physique, 90, 90),
    fiche("eboulement", "Éboulement", Type::Roche, Physique, 90, 85),
    avec_effet(
        fiche(
            "etincelle",
            "Étincelle",
            Type::Electrique,
            Speciale,
            45,
            100,
        ),
        Paralysie,
        10,
    ),
    avec_effet(
        fiche("foudre", "Foudre", Type::Electrique, Speciale, 90, 85),
        Paralysie,
        10,
    ),
    avec_effet(
        fiche(
            "onde-de-choc",
            "Onde de choc",
            Type::Electrique,
            Speciale,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    avec_effet(
        fiche(
            "linceul-de-givre",
            "Linceul de givre",
            Type::Obscur,
            Speciale,
            95,
            90,
        ),
        Paralysie,
        20,
    ),
    // Attaques d'évolution des stades 3 (étape 81).
    fiche(
        "souffle-de-cendre",
        "Souffle de cendre",
        Type::Feu,
        Speciale,
        95,
        90,
    ),
    avec_effet(
        fiche(
            "deluge-toxique",
            "Déluge toxique",
            Type::Poison,
            Physique,
            90,
            90,
        ),
        Poison,
        30,
    ),
    avec_effet(
        fiche(
            "racines-du-gibet",
            "Racines du gibet",
            Type::Spectre,
            Physique,
            85,
            95,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "griffes-spectrales",
        "Griffes spectrales",
        Type::Spectre,
        Physique,
        85,
        95,
    ),
    fiche(
        "dechiquetage",
        "Déchiquetage",
        Type::Combat,
        Physique,
        90,
        90,
    ),
    // Attaques du Démiurge (étape 90).
    avec_effet(
        fiche(
            "marque-du-sacrifice",
            "Marque du sacrifice",
            Type::Obscur,
            Physique,
            95,
            95,
        ),
        Poison,
        30,
    ),
    avec_effet(
        fiche(
            "soleil-noir",
            "Soleil noir",
            Type::Obscur,
            Speciale,
            100,
            90,
        ),
        Paralysie,
        20,
    ),
    // Types Son et Lumière, dans l'Abbaye des Cendres (étape 94).
    fiche("glas", "Glas", Type::Son, Speciale, 50, 100),
    avec_effet(
        fiche("tocsin", "Tocsin", Type::Son, Speciale, 75, 100),
        Paralysie,
        10,
    ),
    avec_effet(
        fiche(
            "carillon-funebre",
            "Carillon funèbre",
            Type::Son,
            Speciale,
            90,
            85,
        ),
        Paralysie,
        10,
    ),
    fiche(
        "lueur-sacree",
        "Lueur sacrée",
        Type::Lumiere,
        Speciale,
        50,
        100,
    ),
    fiche(
        "lance-de-lumiere",
        "Lance de lumière",
        Type::Lumiere,
        Physique,
        80,
        95,
    ),
    fiche(
        "aube-eclatante",
        "Aube éclatante",
        Type::Lumiere,
        Speciale,
        90,
        90,
    ),
    // Attaques d'œuf : elles ne s'obtiennent que par le Rite du sang mêlé
    // (docs/contenu-v1.md, section 28).
    fiche(
        "souffle-des-forges",
        "Souffle des forges",
        Type::Feu,
        Speciale,
        80,
        100,
    ),
    fiche("ressac", "Ressac", Type::Eau, Physique, 80, 100),
    avec_effet(
        fiche(
            "seve-maudite",
            "Sève maudite",
            Type::Plante,
            Speciale,
            75,
            100,
        ),
        Poison,
        20,
    ),
    fiche(
        "griffes-de-minuit",
        "Griffes de minuit",
        Type::Tenebres,
        Physique,
        80,
        100,
    ),
    fiche(
        "assaut-de-meute",
        "Assaut de meute",
        Type::Normal,
        Physique,
        80,
        100,
    ),
    fiche(
        "plongee-funebre",
        "Plongée funèbre",
        Type::Vol,
        Physique,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "etreinte-de-l-aspic",
            "Étreinte de l'aspic",
            Type::Poison,
            Physique,
            75,
            100,
        ),
        Poison,
        30,
    ),
    fiche(
        "lueur-des-marais",
        "Lueur des marais",
        Type::Spectre,
        Speciale,
        80,
        100,
    ),
    fiche(
        "coup-de-heaume",
        "Coup de heaume",
        Type::Acier,
        Physique,
        80,
        100,
    ),
    fiche(
        "essaim-maudit",
        "Essaim maudit",
        Type::Insecte,
        Speciale,
        80,
        100,
    ),
    fiche(
        "danse-macabre",
        "Danse macabre",
        Type::Combat,
        Physique,
        80,
        100,
    ),
    fiche(
        "brame-sacre",
        "Brame sacré",
        Type::Feerique,
        Speciale,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "souffle-du-basilic",
            "Souffle du basilic",
            Type::Dragon,
            Speciale,
            75,
            100,
        ),
        Poison,
        20,
    ),
    fiche(
        "sortilege-sylvestre",
        "Sortilège sylvestre",
        Type::Feerique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "pique-draconique",
        "Piqué draconique",
        Type::Dragon,
        Physique,
        80,
        100,
    ),
    fiche(
        "pluie-d-etoiles",
        "Pluie d'étoiles",
        Type::Stellaire,
        Speciale,
        80,
        100,
    ),
    fiche(
        "plongeon-de-granit",
        "Plongeon de granit",
        Type::Roche,
        Physique,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "griffes-putrides",
            "Griffes putrides",
            Type::Poison,
            Physique,
            75,
            100,
        ),
        Poison,
        30,
    ),
    fiche("vol-nocturne", "Vol nocturne", Type::Vol, Physique, 80, 100),
    fiche(
        "souffle-triple",
        "Souffle triple",
        Type::Dragon,
        Speciale,
        80,
        100,
    ),
    fiche(
        "litanie-des-dechus",
        "Litanie des déchus",
        Type::Psychique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "appel-des-profondeurs",
        "Appel des profondeurs",
        Type::Eau,
        Speciale,
        80,
        100,
    ),
    fiche(
        "morsure-des-tourbieres",
        "Morsure des tourbières",
        Type::Tenebres,
        Physique,
        80,
        100,
    ),
    avec_effet(
        fiche("saignee", "Saignée", Type::Poison, Physique, 75, 100),
        Poison,
        30,
    ),
    fiche(
        "galop-des-eaux",
        "Galop des eaux",
        Type::Eau,
        Physique,
        80,
        100,
    ),
    fiche(
        "griffes-de-famine",
        "Griffes de famine",
        Type::Glace,
        Physique,
        80,
        100,
    ),
    fiche(
        "coup-de-massue",
        "Coup de massue",
        Type::Sol,
        Physique,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "oeil-de-l-orage",
            "Œil de l'orage",
            Type::Electrique,
            Speciale,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "souffle-de-l-hiver",
        "Souffle de l'hiver",
        Type::Glace,
        Speciale,
        80,
        100,
    ),
    fiche(
        "valse-des-neiges",
        "Valse des neiges",
        Type::Feerique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "volee-de-cloches",
        "Volée de cloches",
        Type::Son,
        Speciale,
        80,
        100,
    ),
    fiche(
        "rosace-ardente",
        "Rosace ardente",
        Type::Lumiere,
        Speciale,
        80,
        100,
    ),
    // Attaques d'œuf des espèces de la Fosse aux damnés (section 36).
    fiche(
        "charge-des-fosses",
        "Charge des fosses",
        Type::Combat,
        Physique,
        80,
        100,
    ),
    fiche(
        "charge-d-airain",
        "Charge d'airain",
        Type::Acier,
        Physique,
        80,
        100,
    ),
    // Attaques du Désert de sel et de la Mélusine (section 40).
    avec_effet(
        fiche("crecelle", "Crécelle", Type::Son, Speciale, 60, 100),
        Paralysie,
        20,
    ),
    fiche(
        "chant-des-dunes",
        "Chant des dunes",
        Type::Son,
        Speciale,
        85,
        95,
    ),
    fiche(
        "rayon-solaire",
        "Rayon solaire",
        Type::Lumiere,
        Speciale,
        70,
        100,
    ),
    fiche(
        "eclat-du-zenith",
        "Éclat du zénith",
        Type::Lumiere,
        Speciale,
        85,
        95,
    ),
    fiche(
        "flamme-blanche",
        "Flamme blanche",
        Type::Lumiere,
        Speciale,
        95,
        90,
    ),
    avec_effet(
        fiche(
            "bandelettes",
            "Bandelettes",
            Type::Spectre,
            Physique,
            65,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "engloutissement",
        "Engloutissement",
        Type::Sol,
        Physique,
        95,
        85,
    ),
    fiche(
        "vent-de-sable",
        "Vent de sable",
        Type::Sol,
        Speciale,
        85,
        90,
    ),
    fiche(
        "souffle-des-brumes",
        "Souffle des brumes",
        Type::Dragon,
        Speciale,
        95,
        90,
    ),
    // Attaques d'œuf des espèces du Désert de sel (section 40).
    fiche(
        "morsure-des-salines",
        "Morsure des salines",
        Type::Poison,
        Physique,
        80,
        100,
    ),
    fiche(
        "disque-solaire",
        "Disque solaire",
        Type::Lumiere,
        Speciale,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "linceul-de-sel",
            "Linceul de sel",
            Type::Spectre,
            Physique,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "charge-des-dunes",
        "Charge des dunes",
        Type::Sol,
        Physique,
        80,
        100,
    ),
    fiche(
        "enigme-du-sphinx",
        "Énigme du sphinx",
        Type::Psychique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "aile-d-aurore",
        "Aile d'aurore",
        Type::Lumiere,
        Speciale,
        80,
        100,
    ),
    // Port des Noyés et Mines de Fer-Noir, section 44 (étape 163).
    fiche(
        "pince-broyeuse",
        "Pince broyeuse",
        Type::Eau,
        Physique,
        90,
        90,
    ),
    avec_effet(
        fiche(
            "chant-des-noyes",
            "Chant des noyés",
            Type::Spectre,
            Speciale,
            80,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche("maree-noire", "Marée noire", Type::Eau, Speciale, 90, 95),
    fiche("rapine", "Rapine", Type::Tenebres, Physique, 90, 95),
    fiche(
        "deluge-draconique",
        "Déluge draconique",
        Type::Dragon,
        Speciale,
        100,
        85,
    ),
    avec_effet(
        fiche("jet-d-encre", "Jet d'encre", Type::Eau, Speciale, 65, 100),
        Paralysie,
        10,
    ),
    fiche(
        "ancre-des-damnes",
        "Ancre des damnés",
        Type::Obscur,
        Physique,
        90,
        95,
    ),
    avec_effet(
        fiche(
            "abime-liquide",
            "Abîme liquide",
            Type::Obscur,
            Speciale,
            95,
            90,
        ),
        Paralysie,
        10,
    ),
    fiche(
        "marteau-de-forge",
        "Marteau de forge",
        Type::Acier,
        Physique,
        95,
        90,
    ),
    fiche(
        "coup-de-pioche",
        "Coup de pioche",
        Type::Combat,
        Physique,
        90,
        95,
    ),
    fiche(
        "coup-de-grisou",
        "Coup de grisou",
        Type::Feu,
        Speciale,
        100,
        85,
    ),
    avec_effet(
        fiche(
            "souffle-de-rouille",
            "Souffle de rouille",
            Type::Acier,
            Speciale,
            95,
            90,
        ),
        Poison,
        20,
    ),
    fiche(
        "eclat-de-gemme",
        "Éclat de gemme",
        Type::Roche,
        Speciale,
        90,
        95,
    ),
    avec_effet(
        fiche(
            "chaine-du-prevot",
            "Chaîne du prévôt",
            Type::Obscur,
            Physique,
            90,
            95,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "gouffre-sans-fond",
        "Gouffre sans fond",
        Type::Obscur,
        Speciale,
        95,
        90,
    ),
    // Leurs attaques d'œuf (section 13.11).
    fiche("pince-etau", "Pince-étau", Type::Acier, Physique, 80, 100),
    fiche(
        "lamento-des-abysses",
        "Lamento des abysses",
        Type::Spectre,
        Speciale,
        80,
        100,
    ),
    fiche(
        "pique-du-charognard",
        "Pique du charognard",
        Type::Vol,
        Physique,
        80,
        100,
    ),
    fiche(
        "morsure-des-sept-tetes",
        "Morsure des sept têtes",
        Type::Dragon,
        Physique,
        80,
        100,
    ),
    fiche(
        "spirale-abyssale",
        "Spirale abyssale",
        Type::Eau,
        Speciale,
        80,
        100,
    ),
    fiche(
        "poing-de-fer-noir",
        "Poing de fer noir",
        Type::Acier,
        Physique,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "hantise-du-filon",
            "Hantise du filon",
            Type::Spectre,
            Physique,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "etincelle-de-grisou",
        "Étincelle de grisou",
        Type::Feu,
        Speciale,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "ecaille-rouillee",
            "Écaille rouillée",
            Type::Acier,
            Physique,
            75,
            100,
        ),
        Poison,
        30,
    ),
    fiche(
        "prisme-feerique",
        "Prisme féerique",
        Type::Feerique,
        Speciale,
        80,
        100,
    ),
    // Sylve écarlate et Catacombes de Sainte-Ombre, section 45 (étape
    // 165).
    fiche(
        "etreinte-sanguine",
        "Étreinte sanguine",
        Type::Plante,
        Physique,
        90,
        95,
    ),
    fiche(
        "faux-de-mante",
        "Faux de mante",
        Type::Insecte,
        Physique,
        85,
        95,
    ),
    fiche(
        "charge-du-sanglier",
        "Charge du sanglier",
        Type::Normal,
        Physique,
        85,
        95,
    ),
    fiche(
        "seve-ecarlate",
        "Sève écarlate",
        Type::Plante,
        Speciale,
        95,
        90,
    ),
    avec_effet(
        fiche(
            "aile-poudree",
            "Aile poudrée",
            Type::Insecte,
            Speciale,
            90,
            95,
        ),
        Paralysie,
        10,
    ),
    fiche(
        "baiser-de-sang",
        "Baiser de sang",
        Type::Obscur,
        Speciale,
        90,
        95,
    ),
    avec_effet(
        fiche("peste-noire", "Peste noire", Type::Poison, Physique, 90, 95),
        Poison,
        30,
    ),
    fiche("requiem", "Requiem", Type::Son, Speciale, 95, 90),
    avec_effet(
        fiche(
            "morsure-de-l-araignee",
            "Morsure de l'araignée",
            Type::Insecte,
            Physique,
            80,
            100,
        ),
        Poison,
        20,
    ),
    fiche(
        "malediction-mortelle",
        "Malédiction mortelle",
        Type::Spectre,
        Speciale,
        85,
        95,
    ),
    fiche(
        "jugement-de-pierre",
        "Jugement de pierre",
        Type::Roche,
        Physique,
        95,
        90,
    ),
    fiche(
        "ossuaire-sacre",
        "Ossuaire sacré",
        Type::Obscur,
        Physique,
        95,
        90,
    ),
    // Leurs attaques d'œuf (section 13.11).
    fiche(
        "racines-assoiffees",
        "Racines assoiffées",
        Type::Plante,
        Physique,
        80,
        100,
    ),
    fiche(
        "danse-de-la-mante",
        "Danse de la mante",
        Type::Insecte,
        Physique,
        80,
        100,
    ),
    fiche(
        "coup-de-boutoir",
        "Coup de boutoir",
        Type::Sol,
        Physique,
        80,
        100,
    ),
    fiche(
        "chant-de-la-dryade",
        "Chant de la dryade",
        Type::Psychique,
        Speciale,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "poudre-d-aube",
            "Poudre d'aube",
            Type::Feerique,
            Speciale,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "nuee-de-rats",
        "Nuée de rats",
        Type::Normal,
        Physique,
        80,
        100,
    ),
    fiche("psalmodie", "Psalmodie", Type::Son, Speciale, 80, 100),
    avec_effet(
        fiche(
            "toile-des-cryptes",
            "Toile des cryptes",
            Type::Insecte,
            Physique,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "phylactere",
        "Phylactère",
        Type::Psychique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "aube-de-pierre",
        "Aube de pierre",
        Type::Lumiere,
        Speciale,
        80,
        100,
    ),
    // Plaine du Massacre et Haute-Cité déchue, section 46 (étape 166).
    fiche(
        "galop-funebre",
        "Galop funèbre",
        Type::Normal,
        Physique,
        85,
        95,
    ),
    fiche(
        "charge-des-morts",
        "Charge des morts",
        Type::Spectre,
        Physique,
        95,
        90,
    ),
    fiche(
        "serres-de-poussiere",
        "Serres de poussière",
        Type::Sol,
        Physique,
        85,
        95,
    ),
    fiche(
        "crocs-d-acier",
        "Crocs d'acier",
        Type::Acier,
        Physique,
        85,
        95,
    ),
    fiche(
        "plongeon-d-acier",
        "Plongeon d'acier",
        Type::Vol,
        Physique,
        95,
        90,
    ),
    fiche(
        "corne-de-cendre",
        "Corne de cendre",
        Type::Feu,
        Speciale,
        95,
        90,
    ),
    fiche("boucherie", "Boucherie", Type::Obscur, Physique, 95, 90),
    fiche(
        "decharge-mecanique",
        "Décharge mécanique",
        Type::Electrique,
        Physique,
        85,
        95,
    ),
    fiche(
        "festin-d-encre",
        "Festin d'encre",
        Type::Psychique,
        Speciale,
        85,
        95,
    ),
    avec_effet(
        fiche(
            "regard-hypnotique",
            "Regard hypnotique",
            Type::Psychique,
            Speciale,
            80,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "rayon-d-ether",
        "Rayon d'éther",
        Type::Psychique,
        Speciale,
        95,
        90,
    ),
    fiche(
        "eclair-de-l-archonte",
        "Éclair de l'archonte",
        Type::Electrique,
        Speciale,
        85,
        95,
    ),
    fiche(
        "chant-de-cristal",
        "Chant de cristal",
        Type::Son,
        Speciale,
        95,
        90,
    ),
    avec_effet(
        fiche(
            "regard-des-mille-yeux",
            "Regard des mille yeux",
            Type::Obscur,
            Speciale,
            90,
            95,
        ),
        Paralysie,
        10,
    ),
    fiche(
        "secret-interdit",
        "Secret interdit",
        Type::Obscur,
        Speciale,
        95,
        90,
    ),
    // Leurs attaques d'œuf (section 13.11).
    fiche(
        "sabots-fantomes",
        "Sabots fantômes",
        Type::Spectre,
        Physique,
        80,
        100,
    ),
    fiche(
        "curee-des-sables",
        "Curée des sables",
        Type::Sol,
        Physique,
        80,
        100,
    ),
    fiche(
        "morsure-de-fer",
        "Morsure de fer",
        Type::Acier,
        Physique,
        80,
        100,
    ),
    fiche(
        "serres-du-griffon",
        "Serres du griffon",
        Type::Vol,
        Physique,
        80,
        100,
    ),
    fiche(
        "flamme-de-licorne",
        "Flamme de licorne",
        Type::Feerique,
        Speciale,
        80,
        100,
    ),
    avec_effet(
        fiche("engrenage", "Engrenage", Type::Acier, Physique, 75, 100),
        Paralysie,
        20,
    ),
    fiche(
        "page-maudite",
        "Page maudite",
        Type::Psychique,
        Speciale,
        80,
        100,
    ),
    avec_effet(
        fiche("oeillade", "Œillade", Type::Psychique, Speciale, 75, 100),
        Paralysie,
        20,
    ),
    fiche(
        "arc-electrique",
        "Arc électrique",
        Type::Electrique,
        Speciale,
        80,
        100,
    ),
    fiche("harmonie", "Harmonie", Type::Son, Speciale, 80, 100),
    // Gorge de Braise et Pics Foudroyés, section 47 (étape 167).
    fiche(
        "coulee-de-lave",
        "Coulée de lave",
        Type::Feu,
        Physique,
        85,
        95,
    ),
    fiche(
        "crocs-de-braise",
        "Crocs de braise",
        Type::Feu,
        Physique,
        85,
        95,
    ),
    fiche(
        "marteau-ardent",
        "Marteau ardent",
        Type::Feu,
        Physique,
        85,
        95,
    ),
    fiche(
        "souffle-de-magma",
        "Souffle de magma",
        Type::Feu,
        Physique,
        95,
        90,
    ),
    fiche(
        "flamme-astrale",
        "Flamme astrale",
        Type::Psychique,
        Speciale,
        95,
        90,
    ),
    fiche(
        "couronne-de-braise",
        "Couronne de braise",
        Type::Obscur,
        Physique,
        95,
        90,
    ),
    fiche(
        "serres-foudroyantes",
        "Serres foudroyantes",
        Type::Electrique,
        Physique,
        85,
        95,
    ),
    fiche(
        "coup-de-cornes",
        "Coup de cornes",
        Type::Roche,
        Physique,
        85,
        95,
    ),
    fiche(
        "feu-errant",
        "Feu errant",
        Type::Electrique,
        Speciale,
        85,
        95,
    ),
    fiche(
        "poing-du-tonnerre",
        "Poing du tonnerre",
        Type::Electrique,
        Physique,
        95,
        90,
    ),
    fiche(
        "ailes-celestes",
        "Ailes célestes",
        Type::Feerique,
        Speciale,
        95,
        90,
    ),
    fiche(
        "tempete-noire",
        "Tempête noire",
        Type::Obscur,
        Speciale,
        95,
        90,
    ),
    // Leurs attaques d'œuf (section 13.11).
    fiche(
        "magma-bouillonnant",
        "Magma bouillonnant",
        Type::Feu,
        Physique,
        80,
        100,
    ),
    fiche(
        "croc-des-ombres",
        "Croc des ombres",
        Type::Tenebres,
        Physique,
        80,
        100,
    ),
    fiche("enclume", "Enclume", Type::Acier, Physique, 80, 100),
    fiche(
        "ecaille-ardente",
        "Écaille ardente",
        Type::Dragon,
        Physique,
        80,
        100,
    ),
    fiche(
        "mirage-ardent",
        "Mirage ardent",
        Type::Psychique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "cri-du-tonnerre",
        "Cri du tonnerre",
        Type::Electrique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "charge-du-belier",
        "Charge du bélier",
        Type::Combat,
        Physique,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "lueur-d-orage",
            "Lueur d'orage",
            Type::Spectre,
            Speciale,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche(
        "poing-du-geant",
        "Poing du géant",
        Type::Combat,
        Physique,
        80,
        100,
    ),
    fiche(
        "galop-celeste",
        "Galop céleste",
        Type::Vol,
        Physique,
        80,
        100,
    ),
    // Palais de Chair et Puits du Monde, section 48 (étape 168).
    avec_effet(
        fiche(
            "spore-de-chair",
            "Spore de chair",
            Type::Poison,
            Speciale,
            85,
            95,
        ),
        Poison,
        20,
    ),
    fiche(
        "festin-de-chair",
        "Festin de chair",
        Type::Normal,
        Physique,
        85,
        95,
    ),
    fiche("ecorchure", "Écorchure", Type::Combat, Physique, 85, 95),
    fiche(
        "extase-charnelle",
        "Extase charnelle",
        Type::Psychique,
        Speciale,
        95,
        90,
    ),
    fiche(
        "floraison-sanglante",
        "Floraison sanglante",
        Type::Plante,
        Speciale,
        95,
        90,
    ),
    fiche(
        "couvee-immonde",
        "Couvée immonde",
        Type::Obscur,
        Speciale,
        95,
        90,
    ),
    fiche(
        "chute-sans-fin",
        "Chute sans fin",
        Type::Roche,
        Physique,
        85,
        95,
    ),
    fiche("cri-du-neant", "Cri du néant", Type::Son, Speciale, 85, 95),
    fiche(
        "morsure-des-abysses",
        "Morsure des abysses",
        Type::Dragon,
        Physique,
        85,
        95,
    ),
    fiche(
        "couperet-du-gouffre",
        "Couperet du gouffre",
        Type::Acier,
        Physique,
        95,
        90,
    ),
    fiche(
        "seve-originelle",
        "Sève originelle",
        Type::Plante,
        Speciale,
        95,
        90,
    ),
    fiche("effacement", "Effacement", Type::Obscur, Speciale, 95, 90),
    // Leurs attaques d'œuf (section 13.11).
    fiche(
        "ponte-grouillante",
        "Ponte grouillante",
        Type::Insecte,
        Physique,
        80,
        100,
    ),
    fiche(
        "gueule-beante",
        "Gueule béante",
        Type::Normal,
        Physique,
        80,
        100,
    ),
    fiche(
        "lambeaux-de-peau",
        "Lambeaux de peau",
        Type::Spectre,
        Physique,
        80,
        100,
    ),
    fiche(
        "murmure-charnel",
        "Murmure charnel",
        Type::Psychique,
        Speciale,
        80,
        100,
    ),
    avec_effet(
        fiche(
            "parfum-de-sang",
            "Parfum de sang",
            Type::Feerique,
            Speciale,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche("reptation", "Reptation", Type::Tenebres, Physique, 80, 100),
    fiche("repercussion", "Répercussion", Type::Son, Speciale, 80, 100),
    fiche("fouissement", "Fouissement", Type::Sol, Physique, 80, 100),
    avec_effet(
        fiche(
            "garde-du-gouffre",
            "Garde du gouffre",
            Type::Acier,
            Physique,
            75,
            100,
        ),
        Paralysie,
        20,
    ),
    fiche("eclosion", "Éclosion", Type::Lumiere, Speciale, 80, 100),
    // Les légendaires, section 49 (étape 169).
    fiche(
        "abime-des-mers",
        "Abîme des mers",
        Type::Eau,
        Speciale,
        100,
        90,
    ),
    fiche(
        "pietinement-du-monde",
        "Piétinement du monde",
        Type::Sol,
        Physique,
        100,
        90,
    ),
    fiche(
        "ouragan-celeste",
        "Ouragan céleste",
        Type::Vol,
        Speciale,
        95,
        90,
    ),
    fiche(
        "aile-de-l-aube",
        "Aile de l'aube",
        Type::Lumiere,
        Speciale,
        100,
        90,
    ),
    // Leurs attaques d'œuf (section 13.11).
    fiche(
        "ecaille-abyssale",
        "Écaille abyssale",
        Type::Dragon,
        Physique,
        80,
        100,
    ),
    fiche(
        "corne-primordiale",
        "Corne primordiale",
        Type::Combat,
        Physique,
        80,
        100,
    ),
    fiche(
        "plume-celeste",
        "Plume céleste",
        Type::Vol,
        Speciale,
        80,
        100,
    ),
    // Attaques d'évolution des formes par échange (docs/contenu-v1.md,
    // section 53).
    fiche(
        "chevauchee-sans-tete",
        "Chevauchée sans tête",
        Type::Spectre,
        Physique,
        95,
        100,
    ),
    fiche(
        "presage-funeste",
        "Présage funeste",
        Type::Psychique,
        Speciale,
        90,
        100,
    ),
    avec_effet(
        fiche(
            "regard-de-pierre",
            "Regard de pierre",
            Type::Roche,
            Speciale,
            85,
            90,
        ),
        Paralysie,
        30,
    ),
    // L'attaque du Schnoz primordial, sa forme secrète du niveau 100
    // (section 5.4, étape 202).
    fiche(
        "genese-stellaire",
        "Genèse stellaire",
        Type::Stellaire,
        Speciale,
        120,
        90,
    ),
    // Les attaques d'évolution des formes de stade 4, au niveau 70
    // (section 5.4, étape 203).
    fiche(
        "apocalypse-de-cendre",
        "Apocalypse de cendre",
        Type::Feu,
        Speciale,
        110,
        90,
    ),
    fiche(
        "ecrasement-des-marais",
        "Écrasement des marais",
        Type::Eau,
        Physique,
        110,
        90,
    ),
    fiche(
        "etreinte-du-gibet",
        "Étreinte du gibet",
        Type::Plante,
        Speciale,
        100,
        100,
    ),
    fiche(
        "griffe-des-neuf-vies",
        "Griffe des neuf vies",
        Type::Tenebres,
        Physique,
        110,
        95,
    ),
    fiche(
        "curee-de-l-eclipse",
        "Curée de l'Éclipse",
        Type::Combat,
        Physique,
        110,
        90,
    ),
    fiche(
        "presage-ultime",
        "Présage ultime",
        Type::Psychique,
        Speciale,
        110,
        90,
    ),
    fiche(
        "marche-de-l-abime",
        "Marche de l'abîme",
        Type::Spectre,
        Physique,
        110,
        90,
    ),
    fiche(
        "couronne-d-epines",
        "Couronne d'épines",
        Type::Plante,
        Speciale,
        110,
        90,
    ),
    // Les attaques du Colosse du rempart et des Élus de la Main du Démiurge,
    // gardiens de la Tour sans fond (section 9.5, étape 204).
    fiche(
        "charge-du-rempart",
        "Charge du rempart",
        Type::Acier,
        Physique,
        110,
        90,
    ),
    fiche(
        "sentence-du-vide",
        "Sentence du vide",
        Type::Psychique,
        Speciale,
        120,
        90,
    ),
    fiche(
        "etreinte-des-supplices",
        "Étreinte des supplices",
        Type::Tenebres,
        Physique,
        120,
        90,
    ),
    fiche(
        "rire-de-l-ecorche",
        "Rire de l'écorché",
        Type::Son,
        Speciale,
        120,
        90,
    ),
    fiche(
        "fleau-grouillant",
        "Fléau grouillant",
        Type::Poison,
        Physique,
        120,
        90,
    ),
    fiche(
        "eclipse-totale",
        "Éclipse totale",
        Type::Tenebres,
        Speciale,
        120,
        90,
    ),
    // L'attaque d'oeuf du Colosse du rempart (section 13.11, étape 204).
    fiche(
        "mur-de-boucliers",
        "Mur de boucliers",
        Type::Acier,
        Physique,
        80,
        100,
    ),
    // Attaques d'oeuf des espèces de du Bourg des Lamentations et du Bois des Farfadets (section 13.11, étape 206).
    fiche(
        "voix-d-outre-tombe",
        "Voix d'outre-tombe",
        Type::Spectre,
        Speciale,
        80,
        100,
    ),
    fiche(
        "aumone-maudite",
        "Aumône maudite",
        Type::Tenebres,
        Physique,
        80,
        100,
    ),
    fiche(
        "paille-ensorcelee",
        "Paille ensorcelée",
        Type::Plante,
        Speciale,
        80,
        100,
    ),
    fiche(
        "linceul-blanc",
        "Linceul blanc",
        Type::Feerique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "hululement-lugubre",
        "Hululement lugubre",
        Type::Vol,
        Speciale,
        80,
        100,
    ),
    fiche(
        "spores-du-sommeil",
        "Spores du sommeil",
        Type::Poison,
        Speciale,
        80,
        100,
    ),
    fiche(
        "accolade-de-mousse",
        "Accolade de mousse",
        Type::Plante,
        Physique,
        80,
        100,
    ),
    fiche(
        "eclat-de-farfadet",
        "Éclat de farfadet",
        Type::Feerique,
        Speciale,
        80,
        100,
    ),
    // Attaques d'oeuf des espèces de de la Mer des Naufrages et de la Grève des Ossements (section 13.11, étape 207).
    fiche(
        "filaments-de-meduse",
        "Filaments de méduse",
        Type::Poison,
        Speciale,
        80,
        100,
    ),
    fiche(
        "machoire-abyssale",
        "Mâchoire abyssale",
        Type::Eau,
        Physique,
        80,
        100,
    ),
    fiche(
        "vol-du-mauvais-presage",
        "Vol du mauvais présage",
        Type::Vol,
        Physique,
        80,
        100,
    ),
    fiche(
        "tentacules-du-kraken",
        "Tentacules du kraken",
        Type::Eau,
        Physique,
        80,
        100,
    ),
    fiche(
        "coquille-d-os",
        "Coquille d'os",
        Type::Roche,
        Physique,
        80,
        100,
    ),
    fiche(
        "cri-des-marees",
        "Cri des marées",
        Type::Son,
        Speciale,
        80,
        100,
    ),
    fiche(
        "fanal-trompeur",
        "Fanal trompeur",
        Type::Feu,
        Speciale,
        80,
        100,
    ),
    fiche(
        "chant-d-ecume",
        "Chant d'écume",
        Type::Eau,
        Speciale,
        80,
        100,
    ),
    // Attaques d'oeuf des espèces de des Champs de Pavots noirs, des Remparts de Grisval, de la Forêt pétrifiée et du Sanctuaire d'Asphodèle (section 13.11, étape 208).
    fiche(
        "aile-du-crane",
        "Aile du crâne",
        Type::Insecte,
        Physique,
        80,
        100,
    ),
    fiche(
        "bond-onirique",
        "Bond onirique",
        Type::Psychique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "pollen-du-sommeil",
        "Pollen du sommeil",
        Type::Plante,
        Speciale,
        80,
        100,
    ),
    fiche(
        "sable-des-songes",
        "Sable des songes",
        Type::Psychique,
        Speciale,
        80,
        100,
    ),
    fiche(
        "tete-de-belier",
        "Tête de bélier",
        Type::Acier,
        Physique,
        80,
        100,
    ),
    fiche(
        "garde-immobile",
        "Garde immobile",
        Type::Roche,
        Physique,
        80,
        100,
    ),
    fiche(
        "carreau-funebre",
        "Carreau funèbre",
        Type::Spectre,
        Physique,
        80,
        100,
    ),
    fiche(
        "serment-du-chevalier",
        "Serment du chevalier",
        Type::Combat,
        Physique,
        80,
        100,
    ),
    fiche(
        "ramure-de-granit",
        "Ramure de granit",
        Type::Roche,
        Physique,
        80,
        100,
    ),
    fiche(
        "morsure-calcaire",
        "Morsure calcaire",
        Type::Roche,
        Physique,
        80,
        100,
    ),
    fiche(
        "seve-de-calcaire",
        "Sève de calcaire",
        Type::Plante,
        Speciale,
        80,
        100,
    ),
    fiche(
        "racines-de-granit",
        "Racines de granit",
        Type::Roche,
        Speciale,
        80,
        100,
    ),
    fiche(
        "mantra-solaire",
        "Mantra solaire",
        Type::Lumiere,
        Speciale,
        80,
        100,
    ),
    fiche(
        "griffes-de-cristal",
        "Griffes de cristal",
        Type::Lumiere,
        Physique,
        80,
        100,
    ),
    fiche(
        "lueur-du-ver",
        "Lueur du ver",
        Type::Insecte,
        Speciale,
        80,
        100,
    ),
    fiche(
        "souffle-d-asphodele",
        "Souffle d'Asphodèle",
        Type::Dragon,
        Speciale,
        80,
        100,
    ),
    // Attaques d'oeuf des espèces de du Glacier des Âmes, du Cimetière des Géants et de la Faille du Néant (section 13.11, étape 209).
    fiche(
        "etreinte-glaciale",
        "Étreinte glaciale",
        Type::Glace,
        Physique,
        80,
        100,
    ),
    fiche(
        "poing-de-givre",
        "Poing de givre",
        Type::Glace,
        Physique,
        80,
        100,
    ),
    fiche(
        "voile-de-neige",
        "Voile de neige",
        Type::Vol,
        Speciale,
        80,
        100,
    ),
    fiche(
        "pietinement-glacial",
        "Piétinement glacial",
        Type::Sol,
        Physique,
        80,
        100,
    ),
    fiche(
        "ossature-de-geant",
        "Ossature de géant",
        Type::Sol,
        Physique,
        80,
        100,
    ),
    fiche(
        "bec-putride",
        "Bec putride",
        Type::Poison,
        Physique,
        80,
        100,
    ),
    fiche("ronge-os", "Ronge-os", Type::Insecte, Physique, 80, 100),
    fiche(
        "chute-du-titan",
        "Chute du titan",
        Type::Combat,
        Physique,
        80,
        100,
    ),
    fiche(
        "murmure-du-neant",
        "Murmure du néant",
        Type::Tenebres,
        Speciale,
        80,
        100,
    ),
    fiche(
        "gueule-d-eclipse",
        "Gueule d'éclipse",
        Type::Dragon,
        Physique,
        80,
        100,
    ),
    fiche(
        "hurlement-du-vide",
        "Hurlement du vide",
        Type::Son,
        Speciale,
        80,
        100,
    ),
    fiche(
        "lumiere-noire",
        "Lumière noire",
        Type::Lumiere,
        Speciale,
        80,
        100,
    ),
];

struct FicheCapsule {
    attaque: &'static str,
    prix: u32,
    /// Identifiant de la zone dont les coffres la contiennent.
    zone: &'static str,
}

/// Section 6 de docs/contenu-v1.md, dans le même ordre.
const CAPSULES: [FicheCapsule; 48] = [
    FicheCapsule {
        attaque: "brasier",
        prix: 150,
        zone: "lisiere",
    },
    FicheCapsule {
        attaque: "deferlante",
        prix: 150,
        zone: "lisiere",
    },
    FicheCapsule {
        attaque: "lianes-etrangleuses",
        prix: 150,
        zone: "lisiere",
    },
    FicheCapsule {
        attaque: "croc-de-venin",
        prix: 200,
        zone: "marais",
    },
    FicheCapsule {
        attaque: "frisson-d-outre-tombe",
        prix: 200,
        zone: "marais",
    },
    FicheCapsule {
        attaque: "sentence",
        prix: 300,
        zone: "marais",
    },
    FicheCapsule {
        attaque: "malefice",
        prix: 300,
        zone: "vallee",
    },
    FicheCapsule {
        attaque: "bourrasque",
        prix: 300,
        zone: "vallee",
    },
    FicheCapsule {
        attaque: "furie-draconique",
        prix: 400,
        zone: "vallee",
    },
    FicheCapsule {
        attaque: "bucher",
        prix: 450,
        zone: "abbaye",
    },
    FicheCapsule {
        attaque: "festin-macabre",
        prix: 400,
        zone: "abbaye",
    },
    FicheCapsule {
        attaque: "ultrason",
        prix: 400,
        zone: "abbaye",
    },
    FicheCapsule {
        attaque: "blizzard",
        prix: 500,
        zone: "col-du-givre-noir",
    },
    FicheCapsule {
        attaque: "seisme",
        prix: 450,
        zone: "col-du-givre-noir",
    },
    FicheCapsule {
        attaque: "onde-de-choc",
        prix: 450,
        zone: "col-du-givre-noir",
    },
    FicheCapsule {
        attaque: "chant-des-dunes",
        prix: 550,
        zone: "desert-de-sel",
    },
    FicheCapsule {
        attaque: "eclat-du-zenith",
        prix: 550,
        zone: "desert-de-sel",
    },
    FicheCapsule {
        attaque: "vent-de-sable",
        prix: 500,
        zone: "desert-de-sel",
    },
    FicheCapsule {
        attaque: "maree-noire",
        prix: 600,
        zone: "port-des-noyes",
    },
    FicheCapsule {
        attaque: "pince-broyeuse",
        prix: 600,
        zone: "port-des-noyes",
    },
    FicheCapsule {
        attaque: "chant-des-noyes",
        prix: 600,
        zone: "port-des-noyes",
    },
    FicheCapsule {
        attaque: "marteau-de-forge",
        prix: 650,
        zone: "mines-de-fer-noir",
    },
    FicheCapsule {
        attaque: "coup-de-grisou",
        prix: 650,
        zone: "mines-de-fer-noir",
    },
    FicheCapsule {
        attaque: "coup-de-pioche",
        prix: 650,
        zone: "mines-de-fer-noir",
    },
    FicheCapsule {
        attaque: "faux-de-mante",
        prix: 700,
        zone: "sylve-ecarlate",
    },
    FicheCapsule {
        attaque: "charge-du-sanglier",
        prix: 700,
        zone: "sylve-ecarlate",
    },
    FicheCapsule {
        attaque: "etreinte-sanguine",
        prix: 700,
        zone: "sylve-ecarlate",
    },
    FicheCapsule {
        attaque: "peste-noire",
        prix: 750,
        zone: "catacombes-de-sainte-ombre",
    },
    FicheCapsule {
        attaque: "requiem",
        prix: 750,
        zone: "catacombes-de-sainte-ombre",
    },
    FicheCapsule {
        attaque: "morsure-de-l-araignee",
        prix: 750,
        zone: "catacombes-de-sainte-ombre",
    },
    FicheCapsule {
        attaque: "galop-funebre",
        prix: 800,
        zone: "plaine-du-massacre",
    },
    FicheCapsule {
        attaque: "serres-de-poussiere",
        prix: 800,
        zone: "plaine-du-massacre",
    },
    FicheCapsule {
        attaque: "crocs-d-acier",
        prix: 800,
        zone: "plaine-du-massacre",
    },
    FicheCapsule {
        attaque: "decharge-mecanique",
        prix: 850,
        zone: "haute-cite-dechue",
    },
    FicheCapsule {
        attaque: "festin-d-encre",
        prix: 850,
        zone: "haute-cite-dechue",
    },
    FicheCapsule {
        attaque: "regard-hypnotique",
        prix: 850,
        zone: "haute-cite-dechue",
    },
    FicheCapsule {
        attaque: "coulee-de-lave",
        prix: 900,
        zone: "gorge-de-braise",
    },
    FicheCapsule {
        attaque: "crocs-de-braise",
        prix: 900,
        zone: "gorge-de-braise",
    },
    FicheCapsule {
        attaque: "marteau-ardent",
        prix: 900,
        zone: "gorge-de-braise",
    },
    FicheCapsule {
        attaque: "serres-foudroyantes",
        prix: 950,
        zone: "pics-foudroyes",
    },
    FicheCapsule {
        attaque: "coup-de-cornes",
        prix: 950,
        zone: "pics-foudroyes",
    },
    FicheCapsule {
        attaque: "feu-errant",
        prix: 950,
        zone: "pics-foudroyes",
    },
    FicheCapsule {
        attaque: "spore-de-chair",
        prix: 1000,
        zone: "palais-de-chair",
    },
    FicheCapsule {
        attaque: "festin-de-chair",
        prix: 1000,
        zone: "palais-de-chair",
    },
    FicheCapsule {
        attaque: "ecorchure",
        prix: 1000,
        zone: "palais-de-chair",
    },
    FicheCapsule {
        attaque: "chute-sans-fin",
        prix: 1050,
        zone: "puits-du-monde",
    },
    FicheCapsule {
        attaque: "cri-du-neant",
        prix: 1050,
        zone: "puits-du-monde",
    },
    FicheCapsule {
        attaque: "morsure-des-abysses",
        prix: 1050,
        zone: "puits-du-monde",
    },
];

struct FicheEspece {
    id: &'static str,
    nom: &'static str,
    types: &'static [Type],
    role: Role,
    /// PV, Attaque, Défense, Attaque spéciale, Défense spéciale, Vitesse.
    stats_de_base: [u16; 6],
    rendement_experience: u16,
    /// Fixe le prix de revente (section 14.5).
    rarete: Rarete,
    /// Section 7 de docs/contenu-v1.md, dans le même ordre.
    habiletes: &'static [Habilete],
    /// Niveau d'apprentissage et identifiant de l'attaque.
    apprentissage: &'static [(u8, &'static str)],
    /// Section 8 de docs/contenu-v1.md.
    evolution: Option<FicheEvolution>,
}

struct FicheEvolution {
    vers: &'static str,
    condition: FicheCondition,
    /// Dans la liste d'apprentissage de l'espèce d'arrivée.
    attaque: &'static str,
}

enum FicheCondition {
    Niveau(u8),
    Pierre(PierreDEvolution),
    /// Identifiant de la zone et niveau.
    Lieu(&'static str, u8),
    /// Identifiant de l'Apôtre et niveau.
    ApotreVaincu(&'static str, u8),
    /// Niveau, de jour ou de nuit (section 9.6).
    Moment(JourOuNuit, u8),
    /// Niveau, sous une météo (section 9.7).
    Meteo(Meteo, u8),
    /// Reçu d'un autre joueur, ou avec une Chaîne de lien (section
    /// 5.4).
    Echange,
}

const fn par_pierre(
    vers: &'static str,
    pierre: PierreDEvolution,
    attaque: &'static str,
) -> Option<FicheEvolution> {
    Some(FicheEvolution {
        vers,
        condition: FicheCondition::Pierre(pierre),
        attaque,
    })
}

/// Reçu d'un autre joueur, ou avec une Chaîne de lien, à tout niveau
/// (section 5.4).
const fn par_echange(vers: &'static str, attaque: &'static str) -> Option<FicheEvolution> {
    Some(FicheEvolution {
        vers,
        condition: FicheCondition::Echange,
        attaque,
    })
}

const fn sous_condition(
    vers: &'static str,
    condition: FicheCondition,
    attaque: &'static str,
) -> Option<FicheEvolution> {
    Some(FicheEvolution {
        vers,
        condition,
        attaque,
    })
}

const fn par_niveau(
    vers: &'static str,
    niveau: u8,
    attaque: &'static str,
) -> Option<FicheEvolution> {
    Some(FicheEvolution {
        vers,
        condition: FicheCondition::Niveau(niveau),
        attaque,
    })
}

const fn par_niveau_sous_la_meteo(
    vers: &'static str,
    niveau: u8,
    meteo: Meteo,
    attaque: &'static str,
) -> Option<FicheEvolution> {
    Some(FicheEvolution {
        vers,
        condition: FicheCondition::Meteo(meteo, niveau),
        attaque,
    })
}

const fn par_niveau_au_moment(
    vers: &'static str,
    niveau: u8,
    jour_ou_nuit: JourOuNuit,
    attaque: &'static str,
) -> Option<FicheEvolution> {
    Some(FicheEvolution {
        vers,
        condition: FicheCondition::Moment(jour_ou_nuit, niveau),
        attaque,
    })
}

/// Grete relevée d'entre les morts : hors du codex, elle n'est pas parmi
/// les espèces (docs/contenu-v1.md, section 4).
const MARCHAND_ZOMBIE: FicheEspece = FicheEspece {
    id: "marchand-zombie",
    nom: "Marchand zombie",
    types: &[Type::Spectre, Type::Poison],
    role: Role::Offensif,
    stats_de_base: [60, 70, 60, 40, 55, 35],
    rendement_experience: 95,
    rarete: Rarete::Commune,
    habiletes: &[],
    apprentissage: &[
        (1, "coup-de-garde"),
        (1, "morsure-venimeuse"),
        (1, "charge-spectrale"),
    ],
    evolution: None,
};

/// Section 2 de docs/contenu-v1.md, dans l'ordre du codex : les monstres de
/// départ, les communs, les rares, l'Apôtre, le monstre caché, puis les
/// formes évoluées de la section 8.
static ESPECES: [FicheEspece; 190] = [
    FicheEspece {
        id: "salamandre",
        nom: "Salamandre",
        types: &[Type::Feu],
        role: Role::Offensif,
        stats_de_base: [45, 40, 40, 70, 55, 65],
        rendement_experience: 62,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Affinite, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "souffle-chaud"),
            (3, "crachat-de-braises"),
            (7, "souffle-draconique"),
            (10, "flamme-vive"),
            (14, "fournaise"),
        ],
        evolution: par_niveau("athanor", 16, "jet-de-lave"),
    },
    FicheEspece {
        id: "crapaud",
        nom: "Crapaud",
        types: &[Type::Eau],
        role: Role::Offensif,
        stats_de_base: [55, 68, 55, 35, 45, 55],
        rendement_experience: 63,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "charge-gluante"),
            (3, "plongeon"),
            (7, "coup-de-langue"),
            (10, "langue-venimeuse"),
            (14, "lame-de-fond"),
        ],
        evolution: par_niveau("crapaudine", 16, "trombe"),
    },
    FicheEspece {
        id: "mandragore",
        nom: "Mandragore",
        types: &[Type::Plante],
        role: Role::Soutien,
        stats_de_base: [70, 45, 70, 45, 65, 25],
        rendement_experience: 64,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-racine"),
            (3, "feuille-tranchante"),
            (7, "secousse"),
            (10, "ronces"),
            (14, "spores-putrides"),
        ],
        evolution: par_niveau("main-de-gloire", 16, "hurlement-de-mandragore"),
    },
    FicheEspece {
        id: "chat-noir",
        nom: "Chat noir",
        types: &[Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [40, 72, 35, 40, 38, 90],
        rendement_experience: 62,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Traqueur],
        apprentissage: &[
            (1, "griffure"),
            (3, "coup-sournois"),
            (7, "feulement"),
            (10, "morsure"),
            (14, "coup-de-grace"),
        ],
        evolution: par_niveau("monte-en-l-air", 16, "griffe-d-ombre"),
    },
    FicheEspece {
        id: "loup",
        nom: "Loup",
        types: &[Type::Normal],
        role: Role::Offensif,
        stats_de_base: [50, 62, 45, 30, 40, 58],
        rendement_experience: 70,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Depeceur, Habilete::Fouineur],
        apprentissage: &[(1, "griffure"), (4, "morsure"), (9, "curee")],
        evolution: par_niveau_au_moment("faux-agneau", 16, JourOuNuit::Nuit, "croc-noir"),
    },
    FicheEspece {
        id: "corbeau",
        nom: "Corbeau",
        types: &[Type::Vol, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [42, 57, 38, 45, 38, 72],
        rendement_experience: 66,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Pillard, Habilete::Fortune],
        apprentissage: &[(1, "coup-de-bec"), (4, "pique"), (8, "regard-funeste")],
        evolution: par_niveau_au_moment("bec-de-peste", 16, JourOuNuit::Nuit, "bec-funeste"),
    },
    FicheEspece {
        id: "vipere",
        nom: "Vipère",
        types: &[Type::Poison],
        role: Role::Offensif,
        stats_de_base: [40, 50, 40, 50, 45, 63],
        rendement_experience: 68,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Instinct],
        apprentissage: &[
            (1, "morsure-venimeuse"),
            (3, "regard-petrifiant"),
            (9, "etreinte"),
        ],
        evolution: par_niveau_au_moment("tentateur", 16, JourOuNuit::Jour, "venin-noir"),
    },
    FicheEspece {
        id: "feu-follet",
        nom: "Feu follet",
        types: &[Type::Feu, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [45, 30, 40, 70, 60, 75],
        rendement_experience: 65,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Eclaireur, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "lueur-trompeuse"),
            (1, "crachat-de-braises"),
            (10, "flamme-vive"),
            (12, "frisson-d-outre-tombe"),
        ],
        evolution: par_pierre(
            "lanterne-des-morts",
            PierreDEvolution::Braise,
            "flamme-funebre",
        ),
    },
    FicheEspece {
        id: "armure-hantee",
        nom: "Armure hantée",
        types: &[Type::Acier, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [60, 75, 85, 30, 50, 30],
        rendement_experience: 72,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "coup-de-garde"),
            (8, "lame-rouillee"),
            (11, "charge-spectrale"),
        ],
        evolution: sous_condition(
            "gisant",
            FicheCondition::Lieu("vallee", 12),
            "taille-d-acier",
        ),
    },
    FicheEspece {
        id: "fee-noire",
        nom: "Fée noire",
        types: &[Type::Insecte, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [40, 35, 40, 65, 50, 75],
        rendement_experience: 47,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Celerite],
        apprentissage: &[(1, "bourdonnement"), (4, "regard-funeste"), (8, "malefice")],
        evolution: par_niveau_sous_la_meteo("fee-sorciere", 12, Meteo::Tempete, "nuee"),
    },
    FicheEspece {
        id: "squelette",
        nom: "Squelette",
        types: &[Type::Spectre, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [50, 80, 60, 30, 45, 55],
        rendement_experience: 60,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "frappe-d-os"),
            (8, "lame-rouillee"),
            (11, "charge-spectrale"),
        ],
        evolution: sous_condition(
            "ankou",
            FicheCondition::ApotreVaincu("baron-pendu", 20),
            "epee-des-morts",
        ),
    },
    FicheEspece {
        id: "cerf-blanc",
        nom: "Cerf blanc",
        types: &[Type::Normal, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [70, 60, 60, 65, 70, 75],
        rendement_experience: 150,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[(1, "coup-de-bois"), (5, "ruade"), (8, "lumiere-d-aube")],
        evolution: par_pierre("cerf-candelabre", PierreDEvolution::Lune, "rayon-de-lune"),
    },
    FicheEspece {
        id: "basilic",
        nom: "Basilic",
        types: &[Type::Poison, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [75, 70, 65, 75, 60, 55],
        rendement_experience: 130,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "morsure-venimeuse"),
            (8, "regard-petrifiant"),
            (10, "souffle-putride"),
            (13, "croc-de-venin"),
        ],
        evolution: par_niveau_sous_la_meteo("cocatrix", 22, Meteo::Canicule, "regard-de-mort"),
    },
    FicheEspece {
        id: "elfe",
        nom: "Elfe",
        types: &[Type::Feerique, Type::Vol],
        role: Role::Soutien,
        stats_de_base: [55, 40, 50, 85, 80, 90],
        rendement_experience: 97,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[
            (1, "poussiere-d-elfe"),
            (5, "bourrasque"),
            (9, "lumiere-d-aube"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "vouivre",
        nom: "Vouivre",
        types: &[Type::Dragon, Type::Vol],
        role: Role::Offensif,
        stats_de_base: [70, 80, 60, 80, 55, 55],
        rendement_experience: 110,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "souffle-draconique"),
            (8, "pique"),
            (12, "furie-draconique"),
        ],
        evolution: par_niveau("melusine", 45, "souffle-des-brumes"),
    },
    FicheEspece {
        id: "baron-pendu",
        nom: "Baron pendu",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [85, 85, 75, 90, 80, 65],
        rendement_experience: 162,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "corde-du-gibet"),
            (1, "lamentation"),
            (3, "sentence"),
            (8, "trappe-du-gibet"),
            (11, "etreinte-obscure"),
            (14, "hurlement-des-abysses"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "schnoz",
        nom: "Schnoz",
        types: &[Type::Stellaire],
        role: Role::Soutien,
        stats_de_base: [60, 50, 60, 90, 90, 100],
        rendement_experience: 200,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Pillard, Habilete::Fouineur],
        apprentissage: &[
            (1, "eclat-astral"),
            (1, "lueur-trompeuse"),
            (10, "rayon-astral"),
        ],
        // Sa forme secrète, que le codex tait jusqu'à sa capture (section
        // 5.4, étape 202).
        evolution: par_niveau("schnoz-primordial", 100, "genese-stellaire"),
    },
    // Premières évolutions par niveau, section 8 de docs/contenu-v1.md.
    FicheEspece {
        id: "athanor",
        nom: "Athanor",
        types: &[Type::Feu],
        role: Role::Offensif,
        stats_de_base: [60, 50, 55, 92, 70, 83],
        rendement_experience: 86,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Affinite, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "souffle-chaud"),
            (3, "crachat-de-braises"),
            (7, "souffle-draconique"),
            (10, "flamme-vive"),
            (14, "fournaise"),
            (16, "jet-de-lave"),
            (16, "souffle-putride"),
        ],
        evolution: par_niveau("wyverne-de-cendre", 25, "souffle-de-cendre"),
    },
    FicheEspece {
        id: "crapaudine",
        nom: "Crapaudine",
        types: &[Type::Eau],
        role: Role::Offensif,
        stats_de_base: [72, 88, 70, 45, 58, 67],
        rendement_experience: 88,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "charge-gluante"),
            (3, "plongeon"),
            (7, "coup-de-langue"),
            (10, "langue-venimeuse"),
            (14, "lame-de-fond"),
            (16, "trombe"),
            (16, "deferlante"),
        ],
        evolution: par_niveau("crapaud-chaudron", 25, "deluge-toxique"),
    },
    FicheEspece {
        id: "main-de-gloire",
        nom: "Main de gloire",
        types: &[Type::Plante],
        role: Role::Soutien,
        stats_de_base: [90, 58, 90, 58, 84, 30],
        rendement_experience: 89,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-racine"),
            (3, "feuille-tranchante"),
            (7, "secousse"),
            (10, "ronces"),
            (14, "spores-putrides"),
            (16, "hurlement-de-mandragore"),
            (16, "lianes-etrangleuses"),
        ],
        evolution: par_niveau("arbre-pendu", 25, "racines-du-gibet"),
    },
    FicheEspece {
        id: "monte-en-l-air",
        nom: "Monte-en-l'air",
        types: &[Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [52, 92, 45, 50, 48, 113],
        rendement_experience: 86,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Traqueur],
        apprentissage: &[
            (1, "griffure"),
            (3, "coup-sournois"),
            (7, "feulement"),
            (10, "morsure"),
            (14, "coup-de-grace"),
            (16, "griffe-d-ombre"),
            (16, "malefice"),
        ],
        evolution: par_niveau("cait-sith", 25, "griffes-spectrales"),
    },
    FicheEspece {
        id: "faux-agneau",
        nom: "Faux-agneau",
        types: &[Type::Normal, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [65, 80, 58, 38, 52, 75],
        rendement_experience: 98,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Depeceur, Habilete::Fouineur],
        apprentissage: &[
            (1, "griffure"),
            (4, "morsure"),
            (9, "curee"),
            (16, "croc-noir"),
            (16, "coup-sournois"),
        ],
        evolution: par_niveau("loup-garou", 25, "dechiquetage"),
    },
    FicheEspece {
        id: "bec-de-peste",
        nom: "Bec-de-peste",
        types: &[Type::Vol, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [55, 72, 48, 58, 48, 90],
        rendement_experience: 92,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Pillard, Habilete::Fortune],
        apprentissage: &[
            (1, "coup-de-bec"),
            (4, "pique"),
            (8, "regard-funeste"),
            (16, "bec-funeste"),
            (16, "bourrasque"),
        ],
        evolution: par_echange("augure", "presage-funeste"),
    },
    FicheEspece {
        id: "tentateur",
        nom: "Tentateur",
        types: &[Type::Poison],
        role: Role::Offensif,
        stats_de_base: [52, 64, 52, 64, 58, 80],
        rendement_experience: 95,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Instinct],
        apprentissage: &[
            (1, "morsure-venimeuse"),
            (3, "regard-petrifiant"),
            (9, "etreinte"),
            (16, "venin-noir"),
            (16, "croc-de-venin"),
        ],
        evolution: par_echange("gorgone", "regard-de-pierre"),
    },
    FicheEspece {
        id: "fee-sorciere",
        nom: "Fée-sorcière",
        types: &[Type::Insecte, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [50, 44, 50, 84, 64, 96],
        rendement_experience: 65,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Celerite],
        apprentissage: &[
            (1, "bourdonnement"),
            (4, "regard-funeste"),
            (8, "malefice"),
            (12, "nuee"),
            (12, "sentence"),
        ],
        evolution: par_niveau_sous_la_meteo(
            "reine-des-ronces",
            25,
            Meteo::Pluie,
            "ronces-maudites",
        ),
    },
    FicheEspece {
        id: "cocatrix",
        nom: "Cocatrix",
        types: &[Type::Poison, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [90, 88, 80, 92, 75, 70],
        rendement_experience: 182,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "morsure-venimeuse"),
            (8, "regard-petrifiant"),
            (10, "souffle-putride"),
            (13, "croc-de-venin"),
            (22, "regard-de-mort"),
            (22, "furie-draconique"),
        ],
        evolution: None,
    },
    // Évolutions par pierre (étape 39).
    FicheEspece {
        id: "lanterne-des-morts",
        nom: "Lanterne des morts",
        types: &[Type::Feu, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [58, 38, 52, 90, 78, 96],
        rendement_experience: 91,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Eclaireur, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "lueur-trompeuse"),
            (1, "crachat-de-braises"),
            (1, "flamme-funebre"),
            (1, "brasier"),
            (10, "flamme-vive"),
            (12, "frisson-d-outre-tombe"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "cerf-candelabre",
        nom: "Cerf-candélabre",
        types: &[Type::Normal, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [85, 75, 75, 82, 88, 95],
        rendement_experience: 210,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[
            (1, "coup-de-bois"),
            (1, "rayon-de-lune"),
            (5, "ruade"),
            (8, "lumiere-d-aube"),
        ],
        evolution: None,
    },
    // Évolutions par lieu et par événement (étape 40).
    FicheEspece {
        id: "gisant",
        nom: "Gisant",
        types: &[Type::Acier, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [75, 95, 105, 38, 62, 38],
        rendement_experience: 100,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "coup-de-garde"),
            (8, "lame-rouillee"),
            (11, "charge-spectrale"),
            (18, "taille-d-acier"),
            (18, "frisson-d-outre-tombe"),
        ],
        evolution: par_echange("dullahan", "chevauchee-sans-tete"),
    },
    FicheEspece {
        id: "ankou",
        nom: "Ankou",
        types: &[Type::Spectre, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [62, 100, 75, 38, 56, 69],
        rendement_experience: 84,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "frappe-d-os"),
            (8, "lame-rouillee"),
            (11, "charge-spectrale"),
            (20, "epee-des-morts"),
            (20, "coup-de-garde"),
        ],
        evolution: None,
    },
    // Abbaye des Cendres, section 10 de docs/contenu-v1.md (étape 45).
    FicheEspece {
        id: "gargouille",
        nom: "Gargouille",
        types: &[Type::Roche, Type::Vol],
        role: Role::Soutien,
        stats_de_base: [60, 80, 90, 30, 50, 40],
        rendement_experience: 74,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "griffe-de-pierre"),
            (10, "pique"),
            (22, "chute-de-pierres"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "goule",
        nom: "Goule",
        types: &[Type::Poison, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [65, 78, 55, 40, 45, 57],
        rendement_experience: 72,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Depeceur, Habilete::Ferocite],
        apprentissage: &[
            (1, "morsure"),
            (1, "morsure-venimeuse"),
            (18, "croc-de-venin"),
            (24, "festin-macabre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "chauve-souris-de-crypte",
        nom: "Chauve-souris de crypte",
        types: &[Type::Vol, Type::Poison],
        role: Role::Offensif,
        stats_de_base: [50, 55, 45, 55, 45, 90],
        rendement_experience: 71,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Flair],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "cri-strident"),
            (8, "morsure-venimeuse"),
            (22, "ultrason"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "chimere",
        nom: "Chimère",
        types: &[Type::Feu, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [75, 80, 65, 85, 60, 55],
        rendement_experience: 131,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "crachat-de-braises"),
            (1, "souffle-draconique"),
            (15, "flamme-vive"),
            (25, "furie-draconique"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "ange-dechu",
        nom: "Ange déchu",
        types: &[Type::Psychique, Type::Vol],
        role: Role::Soutien,
        stats_de_base: [70, 50, 60, 95, 90, 55],
        rendement_experience: 128,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[(1, "onde-psychique"), (12, "bourrasque"), (24, "jugement")],
        evolution: None,
    },
    // Deuxième Apôtre (étape 46).
    FicheEspece {
        id: "abbesse-ecorchee",
        nom: "Abbesse écorchée",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [90, 75, 80, 100, 90, 75],
        rendement_experience: 198,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "lamentation"),
            (1, "malefice"),
            (7, "hurlement-des-abysses"),
            (12, "bucher"),
            (17, "flagellation"),
            (20, "psaume-impie"),
        ],
        evolution: None,
    },
    // Deuxième évolution de la Fée-sorcière (étape 47).
    FicheEspece {
        id: "reine-des-ronces",
        nom: "Reine des ronces",
        types: &[Type::Insecte, Type::Plante],
        role: Role::Offensif,
        stats_de_base: [70, 55, 65, 110, 85, 95],
        rendement_experience: 91,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Celerite],
        apprentissage: &[
            (1, "bourdonnement"),
            (4, "regard-funeste"),
            (8, "malefice"),
            (12, "nuee"),
            (12, "sentence"),
            (25, "ronces-maudites"),
            (25, "lianes-etrangleuses"),
        ],
        evolution: par_niveau("carabosse", 70, "couronne-d-epines"),
    },
    // Monstres aquatiques, qui ne se trouvent qu'à la pêche, section 15 de
    // docs/contenu-v1.md (étape 62).
    FicheEspece {
        id: "noye",
        nom: "Noyé",
        types: &[Type::Eau, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [60, 55, 55, 70, 60, 40],
        rendement_experience: 95,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "plongeon"),
            (1, "lueur-trompeuse"),
            (12, "deferlante"),
            (18, "frisson-d-outre-tombe"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "anguille-des-tourbieres",
        nom: "Anguille des tourbières",
        types: &[Type::Eau, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [50, 70, 45, 55, 45, 85],
        rendement_experience: 95,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Traqueur],
        apprentissage: &[
            (1, "plongeon"),
            (1, "morsure"),
            (8, "etreinte"),
            (16, "lame-de-fond"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "sangsue-geante",
        nom: "Sangsue géante",
        types: &[Type::Eau, Type::Poison],
        role: Role::Soutien,
        stats_de_base: [75, 55, 70, 40, 65, 35],
        rendement_experience: 92,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Instinct],
        apprentissage: &[
            (1, "charge-gluante"),
            (1, "morsure-venimeuse"),
            (15, "croc-de-venin"),
            (20, "deferlante"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "kelpie",
        nom: "Kelpie",
        types: &[Type::Eau, Type::Feerique],
        role: Role::Offensif,
        stats_de_base: [75, 85, 65, 65, 60, 70],
        rendement_experience: 180,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Ferocite, Habilete::Affinite],
        apprentissage: &[
            (1, "plongeon"),
            (1, "ruade"),
            (15, "lame-de-fond"),
            (25, "trombe"),
        ],
        evolution: None,
    },
    // Col du Givre-Noir et troisième Apôtre, section 21 de docs/contenu-v1.md
    // (étape 79).
    FicheEspece {
        id: "wendigo",
        nom: "Wendigo",
        types: &[Type::Glace, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [60, 85, 50, 45, 50, 70],
        rendement_experience: 79,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Traqueur],
        apprentissage: &[
            (1, "croc-de-givre"),
            (1, "charge-spectrale"),
            (20, "frisson-d-outre-tombe"),
            (30, "avalanche"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "troll-des-cavernes",
        nom: "Troll des cavernes",
        types: &[Type::Sol, Type::Roche],
        role: Role::Soutien,
        stats_de_base: [90, 85, 80, 30, 50, 30],
        rendement_experience: 80,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "griffe-de-pierre"),
            (1, "secousse"),
            (22, "chute-de-pierres"),
            (32, "eboulement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "harpie-des-orages",
        nom: "Harpie des orages",
        types: &[Type::Vol, Type::Electrique],
        role: Role::Offensif,
        stats_de_base: [55, 50, 45, 80, 50, 80],
        rendement_experience: 78,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Eclaireur],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "etincelle"),
            (20, "bourrasque"),
            (30, "foudre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "wyrm-de-givre",
        nom: "Wyrm de givre",
        types: &[Type::Dragon, Type::Glace],
        role: Role::Offensif,
        stats_de_base: [80, 85, 70, 85, 65, 60],
        rendement_experience: 140,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "souffle-draconique"),
            (1, "croc-de-givre"),
            (20, "souffle-glacial"),
            (30, "furie-draconique"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "sylphe-des-neiges",
        nom: "Sylphe des neiges",
        types: &[Type::Glace, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [65, 45, 60, 95, 90, 90],
        rendement_experience: 137,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[
            (1, "poussiere-d-elfe"),
            (1, "souffle-glacial"),
            (15, "lumiere-d-aube"),
            (30, "rayon-de-lune"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "margrave-de-givre",
        nom: "Margrave de givre",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [95, 100, 90, 95, 85, 65],
        rendement_experience: 216,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "charge-spectrale"),
            (1, "taille-d-acier"),
            (7, "etreinte-obscure"),
            (14, "blizzard"),
            (19, "hurlement-des-abysses"),
            (25, "linceul-de-givre"),
        ],
        evolution: None,
    },
    // Stades 3, au niveau 36 (étape 81).
    FicheEspece {
        id: "wyverne-de-cendre",
        nom: "Wyverne de cendre",
        types: &[Type::Feu, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [75, 68, 68, 110, 80, 90],
        rendement_experience: 120,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Affinite, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "souffle-chaud"),
            (3, "crachat-de-braises"),
            (7, "souffle-draconique"),
            (10, "flamme-vive"),
            (14, "fournaise"),
            (16, "jet-de-lave"),
            (16, "souffle-putride"),
            (25, "souffle-de-cendre"),
            (25, "furie-draconique"),
        ],
        evolution: par_niveau("ouroboros", 70, "apocalypse-de-cendre"),
    },
    FicheEspece {
        id: "crapaud-chaudron",
        nom: "Crapaud-chaudron",
        types: &[Type::Eau, Type::Poison],
        role: Role::Offensif,
        stats_de_base: [90, 105, 85, 55, 70, 75],
        rendement_experience: 123,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "charge-gluante"),
            (3, "plongeon"),
            (7, "coup-de-langue"),
            (10, "langue-venimeuse"),
            (14, "lame-de-fond"),
            (16, "trombe"),
            (16, "deferlante"),
            (25, "deluge-toxique"),
            (25, "croc-de-venin"),
        ],
        evolution: par_niveau("bombarde-des-marais", 70, "ecrasement-des-marais"),
    },
    FicheEspece {
        id: "arbre-pendu",
        nom: "Arbre-pendu",
        types: &[Type::Plante, Type::Spectre],
        role: Role::Soutien,
        stats_de_base: [110, 70, 110, 70, 100, 30],
        rendement_experience: 124,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-racine"),
            (3, "feuille-tranchante"),
            (7, "secousse"),
            (10, "ronces"),
            (14, "spores-putrides"),
            (16, "hurlement-de-mandragore"),
            (16, "lianes-etrangleuses"),
            (25, "racines-du-gibet"),
            (25, "frisson-d-outre-tombe"),
        ],
        evolution: par_niveau("chene-bourreau", 70, "etreinte-du-gibet"),
    },
    FicheEspece {
        id: "cait-sith",
        nom: "Cait sith",
        types: &[Type::Tenebres, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [65, 112, 55, 65, 58, 135],
        rendement_experience: 120,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Traqueur],
        apprentissage: &[
            (1, "griffure"),
            (3, "coup-sournois"),
            (7, "feulement"),
            (10, "morsure"),
            (14, "coup-de-grace"),
            (16, "griffe-d-ombre"),
            (16, "malefice"),
            (25, "griffes-spectrales"),
            (25, "charge-spectrale"),
        ],
        evolution: par_niveau("chat-a-neuf-queues", 70, "griffe-des-neuf-vies"),
    },
    FicheEspece {
        id: "loup-garou",
        nom: "Loup-garou",
        types: &[Type::Combat, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [85, 110, 75, 45, 65, 100],
        rendement_experience: 137,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Depeceur, Habilete::Fouineur],
        apprentissage: &[
            (1, "griffure"),
            (4, "morsure"),
            (9, "curee"),
            (16, "croc-noir"),
            (16, "coup-sournois"),
            (25, "dechiquetage"),
            (25, "griffe-d-ombre"),
        ],
        evolution: par_niveau("loup-de-l-eclipse", 70, "curee-de-l-eclipse"),
    },
    // Les deux formes du Démiurge, section 23 de docs/contenu-v1.md (étape
    // 90) ; combat final au niveau 100 depuis l'étape 162, aux stats bien
    // au-dessus de celles des Apôtres.
    FicheEspece {
        id: "demiurge",
        nom: "Démiurge",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [85, 93, 79, 93, 79, 71],
        rendement_experience: 420,
        rarete: Rarete::Demiurge,
        habiletes: &[Habilete::Ferocite, Habilete::Volonte],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (10, "hurlement-des-abysses"),
            (20, "flagellation"),
            (40, "marque-du-sacrifice"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "demiurge-eveille",
        nom: "Démiurge éveillé",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [97, 104, 86, 108, 86, 79],
        rendement_experience: 480,
        rarete: Rarete::Demiurge,
        habiletes: &[Habilete::Sorcellerie, Habilete::Carapace],
        apprentissage: &[
            (1, "hurlement-des-abysses"),
            (1, "psaume-impie"),
            (1, "marque-du-sacrifice"),
            (1, "soleil-noir"),
        ],
        evolution: None,
    },
    // Types Son et Lumière, dans l'Abbaye des Cendres (étape 94).
    FicheEspece {
        id: "cloche-hurlante",
        nom: "Cloche hurlante",
        types: &[Type::Acier, Type::Son],
        role: Role::Soutien,
        stats_de_base: [65, 50, 80, 65, 75, 25],
        rendement_experience: 76,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-garde"),
            (1, "glas"),
            (16, "tocsin"),
            (24, "carillon-funebre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "seraphin-de-vitrail",
        nom: "Séraphin de vitrail",
        types: &[Type::Lumiere],
        role: Role::Offensif,
        stats_de_base: [65, 50, 60, 105, 80, 60],
        rendement_experience: 128,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "lueur-sacree"),
            (12, "lumiere-d-aube"),
            (18, "lance-de-lumiere"),
            (24, "aube-eclatante"),
        ],
        evolution: None,
    },
    // Exclusives à la Fosse aux damnés, qui les donne aux paliers 5 et 10
    // (section 36).
    FicheEspece {
        id: "molosse-des-fosses",
        nom: "Molosse des fosses",
        types: &[Type::Combat],
        role: Role::Offensif,
        stats_de_base: [80, 100, 70, 35, 60, 85],
        rendement_experience: 170,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Ferocite, Habilete::Instinct],
        apprentissage: &[
            (1, "griffure"),
            (5, "frappe-d-os"),
            (12, "curee"),
            (20, "dechiquetage"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "taureau-d-airain",
        nom: "Taureau d'airain",
        types: &[Type::Acier, Type::Feu],
        role: Role::Offensif,
        stats_de_base: [90, 110, 100, 75, 70, 40],
        rendement_experience: 200,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "coup-de-garde"),
            (10, "brasier"),
            (25, "taille-d-acier"),
            (30, "bucher"),
        ],
        evolution: None,
    },
    // Espèces du Désert de sel (section 40), puis l'évolution de la Vouivre.
    FicheEspece {
        id: "crotale-des-salines",
        nom: "Crotale des salines",
        types: &[Type::Son, Type::Poison],
        role: Role::Offensif,
        stats_de_base: [60, 78, 52, 72, 52, 76],
        rendement_experience: 73,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Instinct],
        apprentissage: &[
            (1, "crecelle"),
            (1, "croc-de-venin"),
            (5, "tocsin"),
            (15, "venin-noir"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "scarabee-solaire",
        nom: "Scarabée solaire",
        types: &[Type::Lumiere, Type::Insecte],
        role: Role::Soutien,
        stats_de_base: [65, 50, 80, 75, 80, 40],
        rendement_experience: 72,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "bourdonnement"),
            (1, "rayon-solaire"),
            (5, "nuee"),
            (15, "aube-eclatante"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "momie-de-sel",
        nom: "Momie de sel",
        types: &[Type::Spectre, Type::Roche],
        role: Role::Soutien,
        stats_de_base: [85, 72, 95, 40, 78, 20],
        rendement_experience: 74,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "bandelettes"),
            (1, "chute-de-pierres"),
            (5, "lamentation"),
            (15, "eboulement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "ver-des-salines",
        nom: "Ver des salines",
        types: &[Type::Sol],
        role: Role::Offensif,
        stats_de_base: [95, 92, 68, 35, 55, 45],
        rendement_experience: 74,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Traqueur],
        apprentissage: &[
            (1, "secousse"),
            (1, "etreinte"),
            (5, "engloutissement"),
            (15, "seisme"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "sphinx-des-sables",
        nom: "Sphinx des sables",
        types: &[Type::Son, Type::Psychique],
        role: Role::Offensif,
        stats_de_base: [80, 70, 75, 100, 85, 55],
        rendement_experience: 130,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "onde-psychique"),
            (1, "glas"),
            (3, "carillon-funebre"),
            (13, "jugement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "phenix-blanc",
        nom: "Phénix blanc",
        types: &[Type::Lumiere, Type::Feu],
        role: Role::Soutien,
        stats_de_base: [75, 50, 65, 100, 95, 80],
        rendement_experience: 127,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[
            (1, "lueur-sacree"),
            (1, "brasier"),
            (1, "rayon-solaire"),
            (13, "flamme-blanche"),
        ],
        evolution: None,
    },
    // La Vouivre évolue au niveau 45 (section 5.4) ; son rendement d'XP
    // vaut 1,4 fois le sien.
    FicheEspece {
        id: "melusine",
        nom: "Mélusine",
        types: &[Type::Dragon, Type::Vol],
        role: Role::Offensif,
        stats_de_base: [90, 100, 80, 100, 75, 75],
        rendement_experience: 154,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "souffle-draconique"),
            (8, "pique"),
            (12, "furie-draconique"),
            (45, "souffle-des-brumes"),
            (45, "bec-funeste"),
        ],
        evolution: None,
    },
    // Espèces du Port des Noyés et de l'Épave du Léviathan, puis des Mines
    // de Fer-Noir et du Filon écarlate (section 44, étape 163).
    FicheEspece {
        id: "crabe-necrophage",
        nom: "Crabe nécrophage",
        types: &[Type::Eau, Type::Acier],
        role: Role::Soutien,
        stats_de_base: [72, 78, 90, 30, 60, 30],
        rendement_experience: 87,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "plongeon"),
            (1, "coup-de-garde"),
            (5, "deferlante"),
            (15, "taille-d-acier"),
            (38, "pince-broyeuse"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "sirene-putride",
        nom: "Sirène putride",
        types: &[Type::Eau, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [58, 35, 48, 92, 70, 64],
        rendement_experience: 87,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Instinct],
        apprentissage: &[
            (1, "lueur-trompeuse"),
            (1, "frisson-d-outre-tombe"),
            (10, "lamentation"),
            (20, "chant-des-noyes"),
            (38, "maree-noire"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "goeland-charognard",
        nom: "Goéland charognard",
        types: &[Type::Vol, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [61, 92, 48, 38, 47, 104],
        rendement_experience: 87,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Fortune],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "morsure"),
            (5, "pique"),
            (15, "croc-noir"),
            (25, "bec-funeste"),
            (38, "rapine"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "hydre-des-marees",
        nom: "Hydre des marées",
        types: &[Type::Eau, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [80, 85, 76, 88, 72, 64],
        rendement_experience: 149,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Ferocite],
        apprentissage: &[
            (1, "souffle-draconique"),
            (1, "lame-de-fond"),
            (10, "furie-draconique"),
            (20, "trombe"),
            (40, "deluge-draconique"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "nautile-des-abysses",
        nom: "Nautile des abysses",
        types: &[Type::Eau, Type::Roche],
        role: Role::Soutien,
        stats_de_base: [80, 62, 112, 76, 99, 36],
        rendement_experience: 146,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "jet-d-encre"),
            (1, "chute-de-pierres"),
            (15, "eboulement"),
            (40, "maree-noire"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "amiral-noye",
        nom: "Amiral noyé",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [96, 103, 90, 93, 86, 62],
        rendement_experience: 215,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (1, "lame-de-fond"),
            (1, "hurlement-des-abysses"),
            (15, "trombe"),
            (25, "ancre-des-damnes"),
            (32, "abime-liquide"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "golem-de-fer-noir",
        nom: "Golem de fer noir",
        types: &[Type::Acier, Type::Roche],
        role: Role::Soutien,
        stats_de_base: [76, 82, 90, 30, 55, 20],
        rendement_experience: 109,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-garde"),
            (1, "chute-de-pierres"),
            (12, "taille-d-acier"),
            (22, "eboulement"),
            (44, "marteau-de-forge"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "mineur-possede",
        nom: "Mineur possédé",
        types: &[Type::Spectre, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [67, 93, 59, 46, 54, 71],
        rendement_experience: 109,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Traqueur],
        apprentissage: &[
            (1, "charge-spectrale"),
            (1, "frappe-d-os"),
            (12, "corde-du-gibet"),
            (32, "griffes-spectrales"),
            (44, "coup-de-pioche"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "grisou-ardent",
        nom: "Grisou ardent",
        types: &[Type::Feu, Type::Poison],
        role: Role::Offensif,
        stats_de_base: [52, 40, 49, 92, 55, 84],
        rendement_experience: 109,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Celerite],
        apprentissage: &[
            (1, "crachat-de-braises"),
            (1, "spores-putrides"),
            (12, "brasier"),
            (22, "venin-noir"),
            (44, "coup-de-grisou"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "dragon-de-rouille",
        nom: "Dragon de rouille",
        types: &[Type::Dragon, Type::Acier],
        role: Role::Offensif,
        stats_de_base: [78, 85, 80, 66, 66, 55],
        rendement_experience: 187,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Affinite, Habilete::Carapace],
        apprentissage: &[
            (1, "souffle-draconique"),
            (1, "coup-de-garde"),
            (12, "furie-draconique"),
            (22, "taille-d-acier"),
            (46, "souffle-de-rouille"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "gemmame",
        nom: "Gemmâme",
        types: &[Type::Roche, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [66, 50, 100, 82, 100, 42],
        rendement_experience: 183,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[
            (1, "poussiere-d-elfe"),
            (1, "chute-de-pierres"),
            (17, "rayon-de-lune"),
            (46, "eclat-de-gemme"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "prevot-des-fosses",
        nom: "Prévôt des fosses",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [98, 108, 98, 84, 90, 62],
        rendement_experience: 272,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Ferocite, Habilete::Volonte],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (1, "chute-de-pierres"),
            (2, "hurlement-des-abysses"),
            (22, "seisme"),
            (32, "chaine-du-prevot"),
            (40, "gouffre-sans-fond"),
        ],
        evolution: None,
    },
    // Espèces de la Sylve écarlate et de l'Autel des ronces, puis des
    // Catacombes de Sainte-Ombre et de la Crypte des saints (section 45,
    // étape 165).
    FicheEspece {
        id: "ronce-vampire",
        nom: "Ronce vampire",
        types: &[Type::Plante, Type::Poison],
        role: Role::Soutien,
        stats_de_base: [84, 74, 88, 62, 72, 15],
        rendement_experience: 135,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "feuille-tranchante"),
            (1, "morsure-venimeuse"),
            (15, "ronces"),
            (25, "lianes-etrangleuses"),
            (35, "croc-de-venin"),
            (53, "etreinte-sanguine"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "mante-ecarlate",
        nom: "Mante écarlate",
        types: &[Type::Insecte, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [62, 100, 60, 40, 48, 75],
        rendement_experience: 135,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Celerite],
        apprentissage: &[
            (1, "bourdonnement"),
            (1, "frappe-d-os"),
            (15, "nuee"),
            (25, "epee-des-morts"),
            (30, "faux-de-mante"),
            (51, "dechiquetage"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "sanglier-enrage",
        nom: "Sanglier enragé",
        types: &[Type::Normal, Type::Sol],
        role: Role::Offensif,
        stats_de_base: [85, 95, 70, 30, 45, 60],
        rendement_experience: 135,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Volonte],
        apprentissage: &[
            (1, "ruade"),
            (1, "secousse"),
            (15, "curee"),
            (30, "charge-du-sanglier"),
            (51, "seisme"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "dryade-sanglante",
        nom: "Dryade sanglante",
        types: &[Type::Plante, Type::Psychique],
        role: Role::Offensif,
        stats_de_base: [78, 52, 74, 115, 88, 73],
        rendement_experience: 229,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Sorcellerie, Habilete::Affinite],
        apprentissage: &[
            (1, "feuille-tranchante"),
            (1, "onde-psychique"),
            (20, "hurlement-de-mandragore"),
            (30, "ronces-maudites"),
            (53, "seve-ecarlate"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "phalene-d-aube",
        nom: "Phalène d'aube",
        types: &[Type::Insecte, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [70, 45, 75, 95, 95, 70],
        rendement_experience: 225,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Instinct, Habilete::Volonte],
        apprentissage: &[
            (1, "bourdonnement"),
            (1, "poussiere-d-elfe"),
            (20, "lumiere-d-aube"),
            (30, "rayon-de-lune"),
            (55, "aile-poudree"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "duchesse-sanguine",
        nom: "Duchesse sanguine",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [96, 92, 88, 112, 92, 65],
        rendement_experience: 322,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Sorcellerie, Habilete::Ferocite],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (1, "morsure"),
            (5, "hurlement-des-abysses"),
            (20, "ronces-maudites"),
            (30, "baiser-de-sang"),
            (45, "psaume-impie"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "rat-des-ossuaires",
        nom: "Rat des ossuaires",
        types: &[Type::Normal, Type::Poison],
        role: Role::Offensif,
        stats_de_base: [70, 90, 58, 40, 52, 85],
        rendement_experience: 154,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Pillard],
        apprentissage: &[
            (1, "griffure"),
            (1, "morsure-venimeuse"),
            (16, "curee"),
            (26, "croc-de-venin"),
            (57, "peste-noire"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "moine-psalmodiant",
        nom: "Moine psalmodiant",
        types: &[Type::Son, Type::Spectre],
        role: Role::Soutien,
        stats_de_base: [80, 45, 75, 85, 85, 15],
        rendement_experience: 154,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Volonte, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "glas"),
            (1, "lueur-trompeuse"),
            (16, "lamentation"),
            (26, "tocsin"),
            (36, "flamme-funebre"),
            (57, "requiem"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "araignee-des-cryptes",
        nom: "Araignée des cryptes",
        types: &[Type::Insecte, Type::Roche],
        role: Role::Offensif,
        stats_de_base: [70, 92, 85, 35, 55, 48],
        rendement_experience: 154,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Traqueur, Habilete::Carapace],
        apprentissage: &[
            (1, "bourdonnement"),
            (1, "griffe-de-pierre"),
            (16, "chute-de-pierres"),
            (26, "morsure-de-l-araignee"),
            (57, "eboulement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "liche",
        nom: "Liche",
        types: &[Type::Psychique, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [66, 42, 62, 108, 90, 67],
        rendement_experience: 261,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "onde-psychique"),
            (1, "lueur-trompeuse"),
            (21, "flamme-funebre"),
            (36, "malediction-mortelle"),
            (58, "jugement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "ange-de-pierre",
        nom: "Ange de pierre",
        types: &[Type::Roche, Type::Lumiere],
        role: Role::Soutien,
        stats_de_base: [85, 75, 110, 60, 100, 20],
        rendement_experience: 258,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "lueur-sacree"),
            (1, "griffe-de-pierre"),
            (21, "lance-de-lumiere"),
            (31, "chute-de-pierres"),
            (60, "jugement-de-pierre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "eveque-des-os",
        nom: "Évêque des os",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [100, 110, 98, 95, 90, 57],
        rendement_experience: 368,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Volonte, Habilete::Carapace],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (1, "frappe-d-os"),
            (6, "hurlement-des-abysses"),
            (21, "epee-des-morts"),
            (36, "flagellation"),
            (51, "ossuaire-sacre"),
        ],
        evolution: None,
    },
    // Espèces de la Plaine du Massacre et du Tertre des héros, puis de la
    // Haute-Cité déchue et de la Bibliothèque interdite (section 46, étape
    // 166).
    FicheEspece {
        id: "destrier-spectral",
        nom: "Destrier spectral",
        types: &[Type::Normal, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [72, 86, 58, 40, 50, 68],
        rendement_experience: 172,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Ferocite],
        apprentissage: &[
            (1, "ruade"),
            (1, "charge-spectrale"),
            (17, "secousse"),
            (27, "griffes-spectrales"),
            (37, "galop-funebre"),
            (63, "charge-des-morts"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "vautour-des-charniers",
        nom: "Vautour des charniers",
        types: &[Type::Vol, Type::Sol],
        role: Role::Offensif,
        stats_de_base: [70, 80, 56, 36, 52, 66],
        rendement_experience: 172,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Depeceur, Habilete::Pillard],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "secousse"),
            (17, "pique"),
            (27, "bec-funeste"),
            (37, "serres-de-poussiere"),
            (63, "seisme"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "matin-de-guerre",
        nom: "Mâtin de guerre",
        types: &[Type::Combat, Type::Acier],
        role: Role::Offensif,
        stats_de_base: [78, 86, 80, 30, 52, 46],
        rendement_experience: 172,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "morsure"),
            (1, "coup-de-garde"),
            (17, "taille-d-acier"),
            (27, "epee-des-morts"),
            (37, "crocs-d-acier"),
            (63, "dechiquetage"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "griffon-cuirasse",
        nom: "Griffon cuirassé",
        types: &[Type::Vol, Type::Acier],
        role: Role::Offensif,
        stats_de_base: [75, 88, 78, 45, 60, 70],
        rendement_experience: 292,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Ferocite, Habilete::Affinite],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "coup-de-garde"),
            (22, "bec-funeste"),
            (32, "taille-d-acier"),
            (65, "plongeon-d-acier"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "licorne-des-cendres",
        nom: "Licorne des cendres",
        types: &[Type::Feu, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [75, 55, 70, 90, 95, 65],
        rendement_experience: 288,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[
            (1, "crachat-de-braises"),
            (1, "poussiere-d-elfe"),
            (22, "lumiere-d-aube"),
            (32, "brasier"),
            (42, "rayon-de-lune"),
            (67, "corne-de-cendre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "connetable-boucher",
        nom: "Connétable boucher",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [100, 105, 98, 85, 90, 62],
        rendement_experience: 411,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Ferocite, Habilete::Volonte],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (1, "frappe-d-os"),
            (7, "hurlement-des-abysses"),
            (22, "dechiquetage"),
            (37, "flagellation"),
            (57, "boucherie"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "automate-brise",
        nom: "Automate brisé",
        types: &[Type::Acier, Type::Electrique],
        role: Role::Soutien,
        stats_de_base: [78, 68, 86, 66, 58, 18],
        rendement_experience: 194,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-garde"),
            (1, "etincelle"),
            (19, "onde-de-choc"),
            (29, "taille-d-acier"),
            (39, "decharge-mecanique"),
            (69, "foudre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "bibliophage",
        nom: "Bibliophage",
        types: &[Type::Insecte, Type::Psychique],
        role: Role::Offensif,
        stats_de_base: [66, 55, 62, 102, 70, 45],
        rendement_experience: 194,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Fouineur],
        apprentissage: &[
            (1, "bourdonnement"),
            (1, "onde-psychique"),
            (19, "nuee"),
            (34, "festin-d-encre"),
            (69, "jugement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "oeil-errant",
        nom: "Œil errant",
        types: &[Type::Psychique, Type::Vol],
        role: Role::Offensif,
        stats_de_base: [58, 38, 52, 100, 66, 76],
        rendement_experience: 194,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Instinct, Habilete::Flair],
        apprentissage: &[
            (1, "regard-funeste"),
            (1, "onde-psychique"),
            (19, "bourrasque"),
            (29, "regard-hypnotique"),
            (69, "rayon-d-ether"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "archonte-foudroye",
        nom: "Archonte foudroyé",
        types: &[Type::Electrique, Type::Psychique],
        role: Role::Offensif,
        stats_de_base: [72, 48, 66, 105, 85, 72],
        rendement_experience: 327,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Sorcellerie, Habilete::Affinite],
        apprentissage: &[
            (1, "etincelle"),
            (1, "onde-psychique"),
            (24, "onde-de-choc"),
            (34, "eclair-de-l-archonte"),
            (71, "jugement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "muse-de-cristal",
        nom: "Muse de cristal",
        types: &[Type::Son, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [70, 45, 70, 95, 95, 65],
        rendement_experience: 322,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[
            (1, "glas"),
            (1, "poussiere-d-elfe"),
            (24, "tocsin"),
            (34, "rayon-de-lune"),
            (71, "chant-de-cristal"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "chancelier-aux-mille-yeux",
        nom: "Chancelier aux mille yeux",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [96, 80, 90, 110, 98, 66],
        rendement_experience: 456,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (1, "onde-psychique"),
            (9, "hurlement-des-abysses"),
            (24, "jugement"),
            (39, "regard-des-mille-yeux"),
            (64, "secret-interdit"),
        ],
        evolution: None,
    },
    // Espèces de la Gorge de Braise et du Cœur du volcan, puis des Pics
    // Foudroyés et de l'Aire des pégases (section 47, étape 167).
    FicheEspece {
        id: "golem-de-magma",
        nom: "Golem de magma",
        types: &[Type::Feu, Type::Roche],
        role: Role::Soutien,
        stats_de_base: [82, 74, 92, 60, 60, 18],
        rendement_experience: 218,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "crachat-de-braises"),
            (1, "griffe-de-pierre"),
            (22, "chute-de-pierres"),
            (32, "brasier"),
            (42, "coulee-de-lave"),
            (78, "eboulement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "chien-des-scories",
        nom: "Chien des scories",
        types: &[Type::Feu, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [70, 90, 58, 45, 50, 75],
        rendement_experience: 218,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Celerite],
        apprentissage: &[
            (1, "morsure"),
            (1, "crachat-de-braises"),
            (22, "croc-noir"),
            (32, "griffe-d-ombre"),
            (42, "crocs-de-braise"),
            (78, "rapine"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "forgeron-calcine",
        nom: "Forgeron calciné",
        types: &[Type::Feu, Type::Acier],
        role: Role::Offensif,
        stats_de_base: [78, 90, 82, 30, 52, 46],
        rendement_experience: 218,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "coup-de-garde"),
            (1, "crachat-de-braises"),
            (22, "taille-d-acier"),
            (32, "lame-rouillee"),
            (42, "marteau-ardent"),
            (78, "marteau-de-forge"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "wyrm-de-magma",
        nom: "Wyrm de magma",
        types: &[Type::Feu, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [80, 96, 76, 72, 64, 68],
        rendement_experience: 367,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Ferocite, Habilete::Affinite],
        apprentissage: &[
            (1, "crachat-de-braises"),
            (1, "souffle-draconique"),
            (27, "brasier"),
            (37, "furie-draconique"),
            (80, "souffle-de-magma"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "ifrit",
        nom: "Ifrit",
        types: &[Type::Feu, Type::Psychique],
        role: Role::Soutien,
        stats_de_base: [70, 50, 68, 92, 88, 64],
        rendement_experience: 362,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Sorcellerie, Habilete::Instinct],
        apprentissage: &[
            (1, "crachat-de-braises"),
            (1, "onde-psychique"),
            (27, "jet-de-lave"),
            (37, "festin-d-encre"),
            (80, "flamme-astrale"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "prince-de-braise",
        nom: "Prince de braise",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [100, 106, 96, 92, 90, 58],
        rendement_experience: 512,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Ferocite, Habilete::Volonte],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (1, "crachat-de-braises"),
            (12, "hurlement-des-abysses"),
            (27, "jet-de-lave"),
            (42, "flagellation"),
            (67, "couronne-de-braise"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "aigle-tonnerre",
        nom: "Aigle-tonnerre",
        types: &[Type::Vol, Type::Electrique],
        role: Role::Offensif,
        stats_de_base: [68, 86, 55, 50, 52, 82],
        rendement_experience: 242,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Traqueur, Habilete::Celerite],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "etincelle"),
            (25, "pique"),
            (35, "bec-funeste"),
            (45, "serres-foudroyantes"),
            (85, "plongeon-d-acier"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "belier-de-granit",
        nom: "Bélier de granit",
        types: &[Type::Roche, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [80, 90, 85, 30, 50, 46],
        rendement_experience: 242,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "griffe-de-pierre"),
            (1, "frappe-d-os"),
            (25, "chute-de-pierres"),
            (35, "epee-des-morts"),
            (45, "coup-de-cornes"),
            (85, "dechiquetage"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "feu-de-saint-elme",
        nom: "Feu de Saint-Elme",
        types: &[Type::Electrique, Type::Spectre],
        role: Role::Soutien,
        stats_de_base: [64, 38, 60, 92, 74, 58],
        rendement_experience: 242,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Instinct],
        apprentissage: &[
            (1, "etincelle"),
            (1, "lueur-trompeuse"),
            (25, "onde-de-choc"),
            (35, "flamme-funebre"),
            (45, "feu-errant"),
            (85, "foudre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "geant-des-orages",
        nom: "Géant des orages",
        types: &[Type::Electrique, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [85, 98, 78, 58, 60, 55],
        rendement_experience: 412,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Ferocite, Habilete::Volonte],
        apprentissage: &[
            (1, "etincelle"),
            (1, "frappe-d-os"),
            (30, "onde-de-choc"),
            (40, "epee-des-morts"),
            (87, "poing-du-tonnerre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "pegase-d-orage",
        nom: "Pégase d'orage",
        types: &[Type::Vol, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [70, 58, 66, 85, 90, 76],
        rendement_experience: 407,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Eclaireur],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "poussiere-d-elfe"),
            (30, "bourrasque"),
            (40, "rayon-de-lune"),
            (87, "ailes-celestes"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "senechal-des-orages",
        nom: "Sénéchal des orages",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [98, 82, 92, 110, 96, 64],
        rendement_experience: 564,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (1, "etincelle"),
            (15, "hurlement-des-abysses"),
            (30, "onde-de-choc"),
            (45, "foudre"),
            (70, "tempete-noire"),
        ],
        evolution: None,
    },
    // Espèces du Palais de Chair et du Jardin de la Reine, puis du Puits du
    // Monde et des Racines du monde (section 48, étape 168).
    FicheEspece {
        id: "larve-de-chair",
        nom: "Larve de chair",
        types: &[Type::Insecte, Type::Poison],
        role: Role::Soutien,
        stats_de_base: [84, 62, 86, 66, 74, 20],
        rendement_experience: 266,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "bourdonnement"),
            (1, "morsure-venimeuse"),
            (28, "nuee"),
            (38, "croc-de-venin"),
            (48, "spore-de-chair"),
            (92, "peste-noire"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "devoreur-de-chair",
        nom: "Dévoreur de chair",
        types: &[Type::Normal, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [76, 92, 60, 40, 52, 72],
        rendement_experience: 266,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Pillard],
        apprentissage: &[
            (1, "morsure"),
            (1, "griffure"),
            (28, "curee"),
            (38, "croc-noir"),
            (48, "festin-de-chair"),
            (92, "rapine"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "valet-ecorche",
        nom: "Valet écorché",
        types: &[Type::Combat, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [72, 92, 70, 40, 52, 64],
        rendement_experience: 266,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Traqueur],
        apprentissage: &[
            (1, "frappe-d-os"),
            (1, "charge-spectrale"),
            (28, "corde-du-gibet"),
            (38, "griffes-spectrales"),
            (48, "ecorchure"),
            (92, "charge-des-morts"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "idole-de-chair",
        nom: "Idole de chair",
        types: &[Type::Psychique, Type::Poison],
        role: Role::Offensif,
        stats_de_base: [78, 50, 70, 104, 88, 66],
        rendement_experience: 452,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Sorcellerie, Habilete::Affinite],
        apprentissage: &[
            (1, "onde-psychique"),
            (1, "morsure-venimeuse"),
            (33, "regard-hypnotique"),
            (43, "festin-d-encre"),
            (94, "extase-charnelle"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "lys-de-sang",
        nom: "Lys de sang",
        types: &[Type::Plante, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [72, 50, 70, 90, 96, 66],
        rendement_experience: 447,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Instinct],
        apprentissage: &[
            (1, "feuille-tranchante"),
            (1, "poussiere-d-elfe"),
            (33, "ronces"),
            (43, "rayon-de-lune"),
            (94, "floraison-sanglante"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "reine-mere-de-chair",
        nom: "Reine-mère de chair",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [104, 90, 100, 104, 98, 48],
        rendement_experience: 620,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Volonte, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (1, "morsure-venimeuse"),
            (18, "hurlement-des-abysses"),
            (33, "peste-noire"),
            (48, "baiser-de-sang"),
            (73, "couvee-immonde"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "rampant-du-puits",
        nom: "Rampant du Puits",
        types: &[Type::Roche, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [76, 92, 82, 36, 54, 50],
        rendement_experience: 288,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "griffe-de-pierre"),
            (1, "morsure"),
            (30, "chute-de-pierres"),
            (40, "griffe-d-ombre"),
            (50, "chute-sans-fin"),
            (99, "eboulement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "echo-du-vide",
        nom: "Écho du vide",
        types: &[Type::Son, Type::Psychique],
        role: Role::Soutien,
        stats_de_base: [68, 38, 64, 96, 80, 46],
        rendement_experience: 288,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Instinct],
        apprentissage: &[
            (1, "glas"),
            (1, "onde-psychique"),
            (30, "lamentation"),
            (40, "tocsin"),
            (50, "cri-du-neant"),
            (99, "requiem"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "ver-des-abysses",
        nom: "Ver des abysses",
        types: &[Type::Sol, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [84, 94, 74, 40, 52, 48],
        rendement_experience: 288,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Volonte],
        apprentissage: &[
            (1, "secousse"),
            (1, "souffle-draconique"),
            (30, "furie-draconique"),
            (50, "morsure-des-abysses"),
            (99, "engloutissement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "gardien-du-gouffre",
        nom: "Gardien du gouffre",
        types: &[Type::Acier, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [82, 94, 92, 44, 66, 50],
        rendement_experience: 490,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "coup-de-garde"),
            (1, "morsure"),
            (35, "taille-d-acier"),
            (45, "griffe-d-ombre"),
            (99, "couperet-du-gouffre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "germe-du-monde",
        nom: "Germe du monde",
        types: &[Type::Plante, Type::Lumiere],
        role: Role::Soutien,
        stats_de_base: [78, 50, 76, 92, 98, 56],
        rendement_experience: 485,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Carapace],
        apprentissage: &[
            (1, "feuille-tranchante"),
            (1, "lueur-sacree"),
            (35, "eclat-du-zenith"),
            (45, "ronces-maudites"),
            (99, "seve-originelle"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "roi-sans-visage",
        nom: "Roi sans visage",
        types: &[Type::Obscur],
        role: Role::Offensif,
        stats_de_base: [100, 98, 96, 102, 96, 60],
        rendement_experience: 677,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "etreinte-obscure"),
            (1, "onde-psychique"),
            (20, "hurlement-des-abysses"),
            (35, "jugement"),
            (50, "gouffre-sans-fond"),
            (75, "effacement"),
        ],
        evolution: None,
    },
    // Les légendaires des zones qui s'ouvrent à la chute du Démiurge, au
    // niveau 100 (section 49, étape 169). Ils comptent au codex.
    FicheEspece {
        id: "leviathan",
        nom: "Léviathan",
        types: &[Type::Eau, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [115, 90, 100, 100, 100, 95],
        rendement_experience: 500,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Carapace],
        apprentissage: &[
            (1, "deferlante"),
            (1, "souffle-draconique"),
            (40, "maree-noire"),
            (60, "souffle-des-brumes"),
            (80, "deluge-draconique"),
            (100, "abime-des-mers"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "behemoth",
        nom: "Béhémoth",
        types: &[Type::Sol, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [130, 125, 115, 55, 95, 80],
        rendement_experience: 500,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "secousse"),
            (1, "frappe-d-os"),
            (40, "coup-de-pioche"),
            (60, "engloutissement"),
            (80, "dechiquetage"),
            (100, "pietinement-du-monde"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "ziz",
        nom: "Ziz",
        types: &[Type::Vol, Type::Lumiere],
        role: Role::Offensif,
        stats_de_base: [95, 70, 85, 120, 95, 135],
        rendement_experience: 500,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Celerite, Habilete::Eclaireur],
        apprentissage: &[
            (1, "bourrasque"),
            (1, "lueur-sacree"),
            (40, "aube-eclatante"),
            (60, "flamme-blanche"),
            (80, "ouragan-celeste"),
            (100, "aile-de-l-aube"),
        ],
        evolution: None,
    },
    // Formes par échange (docs/contenu-v1.md, section 53) : elles gardent le
    // rôle, la rareté et les habiletés de leur forme d'avant, et leur
    // rendement d'XP vaut 1,4 fois le sien. Comme pour une pierre, leurs
    // deux attaques d'évolution s'apprennent au niveau 1.
    FicheEspece {
        id: "dullahan",
        nom: "Dullahan",
        types: &[Type::Acier, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [85, 120, 115, 45, 75, 60],
        rendement_experience: 140,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "coup-de-garde"),
            (1, "chevauchee-sans-tete"),
            (1, "epee-des-morts"),
            (8, "lame-rouillee"),
            (11, "charge-spectrale"),
            (18, "taille-d-acier"),
            (18, "frisson-d-outre-tombe"),
        ],
        evolution: par_niveau("hellequin", 70, "marche-de-l-abime"),
    },
    FicheEspece {
        id: "augure",
        nom: "Augure",
        types: &[Type::Vol, Type::Tenebres, Type::Psychique],
        role: Role::Offensif,
        stats_de_base: [70, 75, 65, 110, 75, 105],
        rendement_experience: 128,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Pillard, Habilete::Fortune],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "presage-funeste"),
            (1, "malefice"),
            (4, "pique"),
            (8, "regard-funeste"),
            (16, "bec-funeste"),
            (16, "bourrasque"),
        ],
        evolution: par_niveau("morrigane", 70, "presage-ultime"),
    },
    FicheEspece {
        id: "gorgone",
        nom: "Gorgone",
        types: &[Type::Poison, Type::Roche],
        role: Role::Offensif,
        stats_de_base: [80, 70, 95, 95, 85, 75],
        rendement_experience: 133,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Instinct],
        apprentissage: &[
            (1, "morsure-venimeuse"),
            (1, "regard-de-pierre"),
            (1, "eboulement"),
            (3, "regard-petrifiant"),
            (9, "etreinte"),
            (16, "venin-noir"),
            (16, "croc-de-venin"),
        ],
        evolution: None,
    },
    // La forme secrète du Schnoz, au niveau 100 : plus forte que les
    // légendaires, elle garde le rôle, la rareté et les habiletés du
    // Schnoz ; son
    // rendement d'XP vaut 1,4 fois le sien (section 5.4, étape 202,
    // proposé par Claude, Q124).
    FicheEspece {
        id: "schnoz-primordial",
        nom: "Schnoz primordial",
        types: &[Type::Stellaire],
        role: Role::Soutien,
        stats_de_base: [85, 75, 85, 135, 130, 150],
        rendement_experience: 280,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Pillard, Habilete::Fouineur],
        apprentissage: &[
            (1, "eclat-astral"),
            (1, "lueur-trompeuse"),
            (10, "rayon-astral"),
            (100, "genese-stellaire"),
        ],
        evolution: None,
    },
    // Les formes de stade 4, au niveau 70 : celles des quatre familles de
    // départ, puis du Loup, du Corbeau, de l'Armure hantée et de la Fée
    // noire. Elles gardent le rôle, la rareté, les habiletés et les attaques
    // de leur stade 3, vont à 570 de stats de base dans les mêmes
    // proportions, et leur rendement d'XP vaut 1,4 fois le sien (section
    // 5.4, étape 203, proposé par Claude, Q124).
    FicheEspece {
        id: "ouroboros",
        nom: "Ouroboros",
        types: &[Type::Feu, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [87, 79, 79, 128, 93, 104],
        rendement_experience: 168,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Affinite, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "souffle-chaud"),
            (3, "crachat-de-braises"),
            (7, "souffle-draconique"),
            (10, "flamme-vive"),
            (14, "fournaise"),
            (16, "jet-de-lave"),
            (16, "souffle-putride"),
            (25, "souffle-de-cendre"),
            (25, "furie-draconique"),
            (70, "apocalypse-de-cendre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "bombarde-des-marais",
        nom: "Bombarde des marais",
        types: &[Type::Eau, Type::Poison],
        role: Role::Offensif,
        stats_de_base: [107, 125, 101, 65, 83, 89],
        rendement_experience: 172,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "charge-gluante"),
            (3, "plongeon"),
            (7, "coup-de-langue"),
            (10, "langue-venimeuse"),
            (14, "lame-de-fond"),
            (16, "trombe"),
            (16, "deferlante"),
            (25, "deluge-toxique"),
            (25, "croc-de-venin"),
            (70, "ecrasement-des-marais"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "chene-bourreau",
        nom: "Chêne-bourreau",
        types: &[Type::Plante, Type::Spectre],
        role: Role::Soutien,
        stats_de_base: [129, 81, 128, 81, 116, 35],
        rendement_experience: 173,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-racine"),
            (3, "feuille-tranchante"),
            (7, "secousse"),
            (10, "ronces"),
            (14, "spores-putrides"),
            (16, "hurlement-de-mandragore"),
            (16, "lianes-etrangleuses"),
            (25, "racines-du-gibet"),
            (25, "frisson-d-outre-tombe"),
            (70, "etreinte-du-gibet"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "chat-a-neuf-queues",
        nom: "Chat à neuf queues",
        types: &[Type::Tenebres, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [76, 130, 64, 76, 67, 157],
        rendement_experience: 168,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Traqueur],
        apprentissage: &[
            (1, "griffure"),
            (3, "coup-sournois"),
            (7, "feulement"),
            (10, "morsure"),
            (14, "coup-de-grace"),
            (16, "griffe-d-ombre"),
            (16, "malefice"),
            (25, "griffes-spectrales"),
            (25, "charge-spectrale"),
            (70, "griffe-des-neuf-vies"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "loup-de-l-eclipse",
        nom: "Loup de l'Éclipse",
        types: &[Type::Combat, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [101, 131, 89, 53, 77, 119],
        rendement_experience: 191,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Depeceur, Habilete::Fouineur],
        apprentissage: &[
            (1, "griffure"),
            (4, "morsure"),
            (9, "curee"),
            (16, "croc-noir"),
            (16, "coup-sournois"),
            (25, "dechiquetage"),
            (25, "griffe-d-ombre"),
            (70, "curee-de-l-eclipse"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "morrigane",
        nom: "Morrigane",
        types: &[Type::Vol, Type::Tenebres, Type::Psychique],
        role: Role::Offensif,
        stats_de_base: [80, 86, 74, 124, 86, 120],
        rendement_experience: 179,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Pillard, Habilete::Fortune],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "presage-funeste"),
            (1, "malefice"),
            (4, "pique"),
            (8, "regard-funeste"),
            (16, "bec-funeste"),
            (16, "bourrasque"),
            (70, "presage-ultime"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "hellequin",
        nom: "Hellequin",
        types: &[Type::Acier, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [97, 137, 131, 51, 86, 68],
        rendement_experience: 196,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "coup-de-garde"),
            (1, "chevauchee-sans-tete"),
            (1, "epee-des-morts"),
            (8, "lame-rouillee"),
            (11, "charge-spectrale"),
            (18, "taille-d-acier"),
            (18, "frisson-d-outre-tombe"),
            (70, "marche-de-l-abime"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "carabosse",
        nom: "Carabosse",
        types: &[Type::Insecte, Type::Plante],
        role: Role::Offensif,
        stats_de_base: [83, 65, 77, 131, 101, 113],
        rendement_experience: 127,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Celerite],
        apprentissage: &[
            (1, "bourdonnement"),
            (4, "regard-funeste"),
            (8, "malefice"),
            (12, "nuee"),
            (12, "sentence"),
            (25, "ronces-maudites"),
            (25, "lianes-etrangleuses"),
            (70, "couronne-d-epines"),
        ],
        evolution: None,
    },
    // Les gardiens attitrés de la Tour sans fond (section 9.5, étape 204,
    // proposés par Claude, Q124) : le Colosse du rempart, à l'étape 40,
    // clin d'oeil aux chevaliers des tours des jeux Souls ; puis, dans les
    // Abysses, les cinq Élus de la Main du Démiurge, des Apôtres élevés au
    // rang de dieux, qui ne se capturent pas.
    FicheEspece {
        id: "colosse-du-rempart",
        nom: "Colosse du rempart",
        types: &[Type::Acier, Type::Roche],
        role: Role::Soutien,
        stats_de_base: [110, 125, 150, 45, 100, 50],
        rendement_experience: 260,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "eboulement"),
            (1, "marteau-de-forge"),
            (1, "jugement-de-pierre"),
            (1, "charge-du-rempart"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "cardinal-du-vide",
        nom: "Cardinal du Vide",
        types: &[Type::Psychique, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [100, 70, 110, 140, 120, 80],
        rendement_experience: 700,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "rayon-d-ether"),
            (1, "malediction-mortelle"),
            (1, "jugement"),
            (1, "sentence-du-vide"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "dame-des-supplices",
        nom: "Dame des supplices",
        types: &[Type::Tenebres, Type::Feerique],
        role: Role::Offensif,
        stats_de_base: [95, 125, 85, 110, 90, 115],
        rendement_experience: 700,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Ferocite, Habilete::Celerite],
        apprentissage: &[
            (1, "rapine"),
            (1, "ailes-celestes"),
            (1, "coup-de-grace"),
            (1, "etreinte-des-supplices"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "bouffon-ecorche",
        nom: "Bouffon écorché",
        types: &[Type::Son, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [90, 100, 90, 120, 95, 125],
        rendement_experience: 700,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Instinct, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "requiem"),
            (1, "cri-du-neant"),
            (1, "sentence"),
            (1, "rire-de-l-ecorche"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "seigneur-pestilence",
        nom: "Seigneur Pestilence",
        types: &[Type::Poison, Type::Insecte],
        role: Role::Offensif,
        stats_de_base: [130, 120, 120, 90, 110, 50],
        rendement_experience: 700,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "peste-noire"),
            (1, "faux-de-mante"),
            (1, "deluge-toxique"),
            (1, "fleau-grouillant"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "aile-de-l-eclipse",
        nom: "Aile de l'Éclipse",
        types: &[Type::Vol, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [100, 125, 90, 125, 90, 130],
        rendement_experience: 750,
        rarete: Rarete::Apotre,
        habiletes: &[Habilete::Affinite, Habilete::Celerite],
        apprentissage: &[
            (1, "plongeon-d-acier"),
            (1, "ouragan-celeste"),
            (1, "griffe-d-ombre"),
            (1, "eclipse-totale"),
        ],
        evolution: None,
    },
    // Espèces de du Bourg des Lamentations et du Bois des Farfadets (section 9.1, étape 206, proposées par
    // Claude, Q124).
    FicheEspece {
        id: "pleureuse",
        nom: "Pleureuse",
        types: &[Type::Spectre, Type::Son],
        role: Role::Offensif,
        stats_de_base: [50, 35, 45, 80, 65, 55],
        rendement_experience: 62,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Instinct],
        apprentissage: &[
            (1, "lueur-trompeuse"),
            (1, "glas"),
            (8, "lamentation"),
            (14, "crecelle"),
            (22, "chant-des-noyes"),
            (30, "tocsin"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "mendiant-possede",
        nom: "Mendiant possédé",
        types: &[Type::Tenebres, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [70, 80, 55, 30, 45, 50],
        rendement_experience: 62,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Instinct],
        apprentissage: &[
            (1, "frappe-d-os"),
            (1, "coup-sournois"),
            (9, "morsure"),
            (15, "croc-noir"),
            (24, "ecorchure"),
            (32, "rapine"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "epouvantail",
        nom: "Épouvantail",
        types: &[Type::Plante, Type::Spectre],
        role: Role::Soutien,
        stats_de_base: [65, 55, 70, 45, 65, 30],
        rendement_experience: 62,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "coup-de-bois"),
            (1, "feuille-tranchante"),
            (8, "lianes-etrangleuses"),
            (13, "frisson-d-outre-tombe"),
            (20, "ronces"),
            (30, "racines-du-gibet"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "dame-blanche",
        nom: "Dame blanche",
        types: &[Type::Spectre, Type::Feerique],
        role: Role::Soutien,
        stats_de_base: [70, 40, 65, 85, 90, 60],
        rendement_experience: 115,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "lueur-trompeuse"),
            (1, "poussiere-d-elfe"),
            (10, "lumiere-d-aube"),
            (16, "lamentation"),
            (28, "rayon-de-lune"),
            (40, "malediction-mortelle"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "hibou-blafard",
        nom: "Hibou blafard",
        types: &[Type::Vol, Type::Psychique],
        role: Role::Offensif,
        stats_de_base: [55, 40, 50, 80, 60, 60],
        rendement_experience: 74,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Eclaireur, Habilete::Instinct],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "regard-funeste"),
            (12, "pique"),
            (18, "onde-psychique"),
            (26, "bec-funeste"),
            (34, "regard-hypnotique"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "amanite-rampante",
        nom: "Amanite rampante",
        types: &[Type::Plante, Type::Poison],
        role: Role::Soutien,
        stats_de_base: [70, 50, 70, 60, 65, 30],
        rendement_experience: 74,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "coup-de-racine"),
            (1, "spores-putrides"),
            (10, "lianes-etrangleuses"),
            (18, "ronces"),
            (24, "venin-noir"),
            (32, "hurlement-de-mandragore"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "ours-moussu",
        nom: "Ours moussu",
        types: &[Type::Plante, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [85, 85, 70, 30, 45, 30],
        rendement_experience: 74,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "griffure"),
            (1, "coup-de-bois"),
            (11, "etreinte"),
            (17, "ronces"),
            (25, "ecorchure"),
            (33, "charge-du-sanglier"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "farfadet",
        nom: "Farfadet",
        types: &[Type::Feerique, Type::Vol],
        role: Role::Soutien,
        stats_de_base: [60, 40, 55, 90, 85, 80],
        rendement_experience: 120,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Celerite],
        apprentissage: &[
            (1, "poussiere-d-elfe"),
            (1, "bourrasque"),
            (12, "lumiere-d-aube"),
            (20, "pique"),
            (30, "rayon-de-lune"),
            (42, "ailes-celestes"),
        ],
        evolution: None,
    },
    // Espèces de de la Mer des Naufrages et de la Grève des Ossements (section 9.1, étape 207, proposées par
    // Claude, Q124).
    FicheEspece {
        id: "meduse-des-naufrages",
        nom: "Méduse des naufrages",
        types: &[Type::Eau, Type::Poison],
        role: Role::Soutien,
        stats_de_base: [70, 45, 60, 80, 80, 35],
        rendement_experience: 95,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Carapace],
        apprentissage: &[
            (1, "plongeon"),
            (1, "morsure-venimeuse"),
            (20, "jet-d-encre"),
            (30, "croc-de-venin"),
            (38, "venin-noir"),
            (46, "maree-noire"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "requin-noir",
        nom: "Requin noir",
        types: &[Type::Eau, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [70, 95, 60, 40, 45, 60],
        rendement_experience: 95,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Traqueur],
        apprentissage: &[
            (1, "plongeon"),
            (1, "morsure"),
            (22, "deferlante"),
            (30, "croc-noir"),
            (38, "lame-de-fond"),
            (48, "pince-broyeuse"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "albatros-maudit",
        nom: "Albatros maudit",
        types: &[Type::Vol, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [60, 75, 50, 55, 50, 80],
        rendement_experience: 95,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Eclaireur],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "lueur-trompeuse"),
            (20, "pique"),
            (28, "charge-spectrale"),
            (36, "bec-funeste"),
            (46, "ouragan-celeste"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "kraken",
        nom: "Kraken",
        types: &[Type::Eau, Type::Psychique],
        role: Role::Offensif,
        stats_de_base: [95, 100, 85, 75, 70, 40],
        rendement_experience: 170,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "lame-de-fond"),
            (1, "etreinte"),
            (26, "jet-d-encre"),
            (34, "trombe"),
            (42, "maree-noire"),
            (50, "abime-des-mers"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "pagure-a-crane",
        nom: "Pagure à crâne",
        types: &[Type::Eau, Type::Roche],
        role: Role::Soutien,
        stats_de_base: [70, 75, 95, 40, 65, 35],
        rendement_experience: 105,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "plongeon"),
            (1, "griffe-de-pierre"),
            (24, "deferlante"),
            (32, "chute-de-pierres"),
            (40, "pince-broyeuse"),
            (48, "eboulement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "mouette-hurleuse",
        nom: "Mouette hurleuse",
        types: &[Type::Vol, Type::Son],
        role: Role::Offensif,
        stats_de_base: [60, 70, 50, 70, 50, 80],
        rendement_experience: 105,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Instinct],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "glas"),
            (24, "pique"),
            (32, "crecelle"),
            (40, "bec-funeste"),
            (48, "carillon-funebre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "naufrageur",
        nom: "Naufrageur",
        types: &[Type::Feu, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [65, 50, 55, 95, 65, 50],
        rendement_experience: 105,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Affinite],
        apprentissage: &[
            (1, "crachat-de-braises"),
            (1, "lueur-trompeuse"),
            (24, "flamme-vive"),
            (32, "frisson-d-outre-tombe"),
            (40, "flamme-funebre"),
            (48, "bucher"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "sirene-d-ecume",
        nom: "Sirène d'écume",
        types: &[Type::Eau, Type::Lumiere],
        role: Role::Soutien,
        stats_de_base: [80, 45, 70, 95, 95, 80],
        rendement_experience: 195,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "deferlante"),
            (1, "lueur-sacree"),
            (28, "rayon-solaire"),
            (36, "chant-des-noyes"),
            (44, "eclat-du-zenith"),
            (52, "abime-des-mers"),
        ],
        evolution: None,
    },
    // Espèces de des Champs de Pavots noirs, des Remparts de Grisval, de la Forêt pétrifiée et du Sanctuaire d'Asphodèle (section 9.1, étape 208, proposées par
    // Claude, Q124).
    FicheEspece {
        id: "sphinx-tete-de-mort",
        nom: "Sphinx tête-de-mort",
        types: &[Type::Insecte, Type::Tenebres],
        role: Role::Offensif,
        stats_de_base: [64, 92, 60, 45, 50, 80],
        rendement_experience: 145,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Traqueur],
        apprentissage: &[
            (1, "bourdonnement"),
            (1, "coup-sournois"),
            (15, "nuee"),
            (25, "croc-noir"),
            (35, "faux-de-mante"),
            (53, "rapine"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "lievre-somnambule",
        nom: "Lièvre somnambule",
        types: &[Type::Normal, Type::Psychique],
        role: Role::Offensif,
        stats_de_base: [62, 50, 56, 90, 68, 64],
        rendement_experience: 145,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Instinct, Habilete::Celerite],
        apprentissage: &[
            (1, "griffure"),
            (1, "onde-psychique"),
            (15, "ruade"),
            (25, "regard-hypnotique"),
            (35, "curee"),
            (53, "jugement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "pavot-carnivore",
        nom: "Pavot carnivore",
        types: &[Type::Plante, Type::Psychique],
        role: Role::Soutien,
        stats_de_base: [84, 60, 80, 82, 80, 10],
        rendement_experience: 145,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Carapace],
        apprentissage: &[
            (1, "feuille-tranchante"),
            (1, "onde-psychique"),
            (15, "lianes-etrangleuses"),
            (25, "hurlement-de-mandragore"),
            (35, "regard-hypnotique"),
            (53, "ronces-maudites"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "marchand-de-sable",
        nom: "Marchand de sable",
        types: &[Type::Psychique, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [72, 48, 66, 112, 90, 74],
        rendement_experience: 245,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Sorcellerie, Habilete::Instinct],
        apprentissage: &[
            (1, "onde-psychique"),
            (1, "lueur-trompeuse"),
            (20, "regard-hypnotique"),
            (30, "flamme-funebre"),
            (40, "malediction-mortelle"),
            (54, "presage-funeste"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "belier-de-siege",
        nom: "Bélier de siège",
        types: &[Type::Acier, Type::Roche],
        role: Role::Offensif,
        stats_de_base: [80, 92, 90, 30, 50, 40],
        rendement_experience: 163,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Ferocite],
        apprentissage: &[
            (1, "coup-de-garde"),
            (1, "griffe-de-pierre"),
            (16, "lame-rouillee"),
            (26, "chute-de-pierres"),
            (36, "taille-d-acier"),
            (59, "eboulement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "sentinelle-de-pierre",
        nom: "Sentinelle de pierre",
        types: &[Type::Roche, Type::Combat],
        role: Role::Soutien,
        stats_de_base: [86, 70, 95, 50, 80, 10],
        rendement_experience: 163,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "griffe-de-pierre"),
            (1, "frappe-d-os"),
            (16, "chute-de-pierres"),
            (26, "epee-des-morts"),
            (36, "regard-de-pierre"),
            (59, "jugement-de-pierre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "baliste-hantee",
        nom: "Baliste hantée",
        types: &[Type::Acier, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [66, 96, 74, 35, 50, 70],
        rendement_experience: 163,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Traqueur, Habilete::Instinct],
        apprentissage: &[
            (1, "coup-de-garde"),
            (1, "charge-spectrale"),
            (16, "lame-rouillee"),
            (26, "griffes-spectrales"),
            (36, "taille-d-acier"),
            (59, "charge-des-morts"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "chevalier-errant",
        nom: "Chevalier errant",
        types: &[Type::Combat, Type::Acier],
        role: Role::Offensif,
        stats_de_base: [80, 108, 92, 40, 62, 66],
        rendement_experience: 276,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Ferocite],
        apprentissage: &[
            (1, "frappe-d-os"),
            (1, "coup-de-garde"),
            (21, "lame-rouillee"),
            (31, "epee-des-morts"),
            (41, "taille-d-acier"),
            (60, "marteau-de-forge"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "cerf-de-pierre",
        nom: "Cerf de pierre",
        types: &[Type::Roche, Type::Normal],
        role: Role::Offensif,
        stats_de_base: [74, 92, 84, 36, 52, 58],
        rendement_experience: 183,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Volonte, Habilete::Celerite],
        apprentissage: &[
            (1, "griffe-de-pierre"),
            (1, "ruade"),
            (18, "chute-de-pierres"),
            (28, "curee"),
            (38, "coup-de-cornes"),
            (65, "eboulement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "lezard-petrifie",
        nom: "Lézard pétrifié",
        types: &[Type::Roche, Type::Poison],
        role: Role::Offensif,
        stats_de_base: [70, 90, 80, 40, 50, 62],
        rendement_experience: 183,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Traqueur, Habilete::Carapace],
        apprentissage: &[
            (1, "griffe-de-pierre"),
            (1, "morsure-venimeuse"),
            (18, "croc-de-venin"),
            (28, "chute-de-pierres"),
            (38, "regard-de-pierre"),
            (65, "deluge-toxique"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "dryade-de-pierre",
        nom: "Dryade de pierre",
        types: &[Type::Roche, Type::Plante],
        role: Role::Soutien,
        stats_de_base: [82, 58, 88, 74, 82, 12],
        rendement_experience: 183,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "feuille-tranchante"),
            (1, "griffe-de-pierre"),
            (18, "lianes-etrangleuses"),
            (28, "hurlement-de-mandragore"),
            (38, "ronces-maudites"),
            (65, "eclat-de-gemme"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "arbre-roi-petrifie",
        nom: "Arbre-roi pétrifié",
        types: &[Type::Roche, Type::Psychique],
        role: Role::Soutien,
        stats_de_base: [100, 70, 105, 85, 95, 10],
        rendement_experience: 310,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Carapace],
        apprentissage: &[
            (1, "griffe-de-pierre"),
            (1, "onde-psychique"),
            (23, "chute-de-pierres"),
            (33, "regard-hypnotique"),
            (43, "regard-de-pierre"),
            (66, "jugement-de-pierre"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "moine-de-lumiere",
        nom: "Moine de lumière",
        types: &[Type::Lumiere, Type::Psychique],
        role: Role::Soutien,
        stats_de_base: [82, 48, 74, 86, 90, 12],
        rendement_experience: 206,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Volonte, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "lueur-sacree"),
            (1, "onde-psychique"),
            (20, "rayon-solaire"),
            (30, "regard-hypnotique"),
            (40, "eclat-du-zenith"),
            (73, "aube-eclatante"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "chimere-de-cristal",
        nom: "Chimère de cristal",
        types: &[Type::Lumiere, Type::Roche],
        role: Role::Offensif,
        stats_de_base: [70, 90, 82, 48, 54, 48],
        rendement_experience: 206,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "griffe-de-pierre"),
            (1, "lueur-sacree"),
            (20, "chute-de-pierres"),
            (30, "lance-de-lumiere"),
            (40, "coup-de-cornes"),
            (73, "eboulement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "ver-luisant-geant",
        nom: "Ver luisant géant",
        types: &[Type::Insecte, Type::Lumiere],
        role: Role::Offensif,
        stats_de_base: [70, 50, 64, 96, 72, 40],
        rendement_experience: 206,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Instinct, Habilete::Affinite],
        apprentissage: &[
            (1, "bourdonnement"),
            (1, "lueur-sacree"),
            (20, "nuee"),
            (30, "rayon-solaire"),
            (40, "eclat-du-zenith"),
            (73, "aile-poudree"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "gardien-d-asphodele",
        nom: "Gardien d'Asphodèle",
        types: &[Type::Lumiere, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [84, 62, 78, 100, 92, 64],
        rendement_experience: 347,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Volonte, Habilete::Affinite],
        apprentissage: &[
            (1, "lueur-sacree"),
            (1, "souffle-draconique"),
            (25, "rayon-solaire"),
            (35, "eclat-du-zenith"),
            (45, "souffle-des-brumes"),
            (74, "flamme-blanche"),
        ],
        evolution: None,
    },
    // Espèces de du Glacier des Âmes, du Cimetière des Géants et de la Faille du Néant (section 9.1, étape 209, proposées par
    // Claude, Q124).
    FicheEspece {
        id: "ours-des-glaces",
        nom: "Ours des glaces",
        types: &[Type::Glace, Type::Spectre],
        role: Role::Offensif,
        stats_de_base: [86, 96, 72, 38, 52, 48],
        rendement_experience: 230,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Volonte],
        apprentissage: &[
            (1, "croc-de-givre"),
            (1, "charge-spectrale"),
            (23, "avalanche"),
            (33, "griffes-spectrales"),
            (81, "charge-des-morts"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "yeti",
        nom: "Yéti",
        types: &[Type::Glace, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [84, 98, 74, 36, 50, 52],
        rendement_experience: 230,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Carapace],
        apprentissage: &[
            (1, "croc-de-givre"),
            (1, "frappe-d-os"),
            (23, "avalanche"),
            (33, "epee-des-morts"),
            (43, "ecorchure"),
            (81, "dechiquetage"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "esprit-du-blizzard",
        nom: "Esprit du blizzard",
        types: &[Type::Glace, Type::Vol],
        role: Role::Offensif,
        stats_de_base: [60, 45, 56, 96, 66, 70],
        rendement_experience: 230,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Celerite, Habilete::Sorcellerie],
        apprentissage: &[
            (1, "bourrasque"),
            (1, "croc-de-givre"),
            (23, "souffle-glacial"),
            (33, "ultrason"),
            (43, "bec-funeste"),
            (81, "blizzard"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "mammouth-fantome",
        nom: "Mammouth fantôme",
        types: &[Type::Glace, Type::Sol],
        role: Role::Offensif,
        stats_de_base: [100, 110, 90, 40, 62, 50],
        rendement_experience: 390,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Ferocite, Habilete::Volonte],
        apprentissage: &[
            (1, "croc-de-givre"),
            (1, "secousse"),
            (28, "avalanche"),
            (38, "charge-du-sanglier"),
            (48, "serres-de-poussiere"),
            (82, "seisme"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "colosse-d-os",
        nom: "Colosse d'os",
        types: &[Type::Sol, Type::Spectre],
        role: Role::Soutien,
        stats_de_base: [92, 80, 96, 40, 70, 14],
        rendement_experience: 254,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Carapace, Habilete::Volonte],
        apprentissage: &[
            (1, "secousse"),
            (1, "lueur-trompeuse"),
            (25, "charge-spectrale"),
            (35, "bandelettes"),
            (45, "griffes-spectrales"),
            (88, "engloutissement"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "charognard-des-geants",
        nom: "Charognard des géants",
        types: &[Type::Vol, Type::Poison],
        role: Role::Offensif,
        stats_de_base: [70, 90, 58, 40, 52, 82],
        rendement_experience: 254,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Depeceur, Habilete::Pillard],
        apprentissage: &[
            (1, "coup-de-bec"),
            (1, "morsure-venimeuse"),
            (25, "pique"),
            (35, "croc-de-venin"),
            (45, "bec-funeste"),
            (88, "deluge-toxique"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "ver-ossivore",
        nom: "Ver ossivore",
        types: &[Type::Sol, Type::Insecte],
        role: Role::Offensif,
        stats_de_base: [82, 94, 76, 36, 50, 54],
        rendement_experience: 254,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Traqueur],
        apprentissage: &[
            (1, "secousse"),
            (1, "bourdonnement"),
            (25, "morsure-de-l-araignee"),
            (35, "serres-de-poussiere"),
            (45, "faux-de-mante"),
            (88, "seisme"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "titan-dechu",
        nom: "Titan déchu",
        types: &[Type::Sol, Type::Combat],
        role: Role::Offensif,
        stats_de_base: [100, 115, 92, 40, 60, 48],
        rendement_experience: 432,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Ferocite, Habilete::Volonte],
        apprentissage: &[
            (1, "secousse"),
            (1, "frappe-d-os"),
            (30, "epee-des-morts"),
            (40, "ecorchure"),
            (50, "serres-de-poussiere"),
            (89, "seisme"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "rampant-du-neant",
        nom: "Rampant du néant",
        types: &[Type::Tenebres, Type::Psychique],
        role: Role::Offensif,
        stats_de_base: [68, 48, 62, 100, 72, 42],
        rendement_experience: 277,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Instinct],
        apprentissage: &[
            (1, "regard-funeste"),
            (1, "onde-psychique"),
            (28, "malefice"),
            (38, "sentence"),
            (48, "regard-hypnotique"),
            (95, "rayon-d-ether"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "mangeur-d-etoiles",
        nom: "Mangeur d'étoiles",
        types: &[Type::Tenebres, Type::Dragon],
        role: Role::Offensif,
        stats_de_base: [78, 94, 70, 52, 50, 48],
        rendement_experience: 277,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Ferocite, Habilete::Affinite],
        apprentissage: &[
            (1, "morsure"),
            (1, "souffle-draconique"),
            (28, "croc-noir"),
            (38, "furie-draconique"),
            (48, "griffe-d-ombre"),
            (95, "rapine"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "spectre-hurlant",
        nom: "Spectre hurlant",
        types: &[Type::Spectre, Type::Son],
        role: Role::Soutien,
        stats_de_base: [74, 40, 66, 92, 84, 36],
        rendement_experience: 277,
        rarete: Rarete::Commune,
        habiletes: &[Habilete::Sorcellerie, Habilete::Volonte],
        apprentissage: &[
            (1, "lueur-trompeuse"),
            (1, "glas"),
            (28, "lamentation"),
            (38, "tocsin"),
            (48, "malediction-mortelle"),
            (95, "requiem"),
        ],
        evolution: None,
    },
    FicheEspece {
        id: "ange-du-neant",
        nom: "Ange du néant",
        types: &[Type::Tenebres, Type::Lumiere],
        role: Role::Offensif,
        stats_de_base: [80, 60, 72, 106, 90, 62],
        rendement_experience: 470,
        rarete: Rarete::Rare,
        habiletes: &[Habilete::Sorcellerie, Habilete::Affinite],
        apprentissage: &[
            (1, "lueur-sacree"),
            (1, "regard-funeste"),
            (33, "malefice"),
            (43, "eclat-du-zenith"),
            (53, "sentence"),
            (96, "aube-eclatante"),
        ],
        evolution: None,
    },
];

/// Section 15 de docs/contenu-v1.md : la ligne est prête 2 h après avoir
/// été lancée.
const HEURES_DE_PECHE: u64 = 2;
const ESPECES_PECHEES: [&str; 3] = ["noye", "anguille-des-tourbieres", "sangsue-geante"];
const ESPECE_PECHEE_RARE: &str = "kelpie";

/// Section 53 de docs/contenu-v1.md : les matières que remonte chaque
/// ligne, avec leur prix chez Malvoise et leur chance sur 100.
const MATIERES_PECHEES: [(&str, &str, u32, u32); 3] = [
    ("vers-de-vase", "Vers de vase", 5, 60),
    ("chair-de-noye", "Chair de noyé", 10, 30),
    ("ambre-noir", "Ambre noir", 30, 10),
];
const RECETTES_DES_APPATS: [(Appat, &[(&str, u32)]); 3] = [
    (Appat::Vers, &[("vers-de-vase", 2)]),
    (Appat::Chair, &[("vers-de-vase", 1), ("chair-de-noye", 1)]),
    (Appat::Ambre, &[("chair-de-noye", 1), ("ambre-noir", 1)]),
];
const RECETTE_DU_PARFUM_DE_CHAROGNE: [(&str, u32); 2] = [("chair-de-noye", 3), ("vers-de-vase", 2)];
/// En plus de 3 de la ressource de l'espèce.
const BASE_DU_PARFUM_D_ESPECE: [(&str, u32); 1] = [("ambre-noir", 1)];

fn creer_espece(fiche: &FicheEspece) -> Espece {
    let [
        pv,
        attaque,
        defense,
        attaque_speciale,
        defense_speciale,
        vitesse,
    ] = fiche.stats_de_base;
    let apprentissage = fiche
        .apprentissage
        .iter()
        .map(|&(niveau_appris, id)| AttaqueApprise {
            niveau: niveau(niveau_appris),
            attaque: creer_attaque(id),
        })
        .collect();
    let espece = Espece::new(
        EspeceId::new(fiche.id),
        fiche.nom,
        TypesMonstre::new(fiche.types).expect("types du contenu valides"),
        fiche.role,
        Stats {
            pv,
            attaque,
            defense,
            attaque_speciale,
            defense_speciale,
            vitesse,
        },
        fiche.rendement_experience,
        apprentissage,
    )
    .avec_rarete(fiche.rarete)
    .avec_habiletes(fiche.habiletes.to_vec());
    let espece = match &fiche.evolution {
        Some(evolution) => espece.avec_evolution(creer_evolution(evolution)),
        None => espece,
    };
    let base = base_de_la_famille(fiche.id);
    let espece = match ATTAQUES_D_OEUF.iter().find(|(espece, _)| *espece == base) {
        Some((_, attaque)) => espece.avec_attaque_d_oeuf(creer_attaque(attaque)),
        None => espece,
    };
    let espece = match ACTIVITES.iter().find(|(espece, _)| *espece == fiche.id) {
        Some((_, activite)) => espece.avec_activite(*activite),
        None => espece,
    };
    match RESSOURCES.iter().find(|(espece, _, _)| *espece == fiche.id) {
        Some((_, id, nom)) => {
            let prix = match fiche.rarete {
                Rarete::Rare => PRIX_D_UNE_RESSOURCE_RARE,
                Rarete::Commune | Rarete::Apotre | Rarete::Demiurge => PRIX_D_UNE_RESSOURCE_COMMUNE,
            };
            let espece = espece.avec_ressource(Ressource::new(RessourceId::new(*id), *nom, prix));
            // Une espèce qui lâche une ressource a aussi son butin rare
            // (section 14.3).
            let butin_rare = ButinRare::de_l_espece(&espece);
            espece.avec_butin_rare(butin_rare)
        }
        None => espece,
    }
}

fn creer_evolution(fiche: &FicheEvolution) -> Evolution {
    let condition = match fiche.condition {
        FicheCondition::Niveau(valeur) => ConditionEvolution::Niveau(niveau(valeur)),
        FicheCondition::Pierre(pierre) => ConditionEvolution::Pierre(pierre),
        FicheCondition::Lieu(zone, valeur) => ConditionEvolution::Lieu {
            zone: ZoneId::new(zone),
            niveau: niveau(valeur),
        },
        FicheCondition::ApotreVaincu(apotre, valeur) => ConditionEvolution::ApotreVaincu {
            apotre: EspeceId::new(apotre),
            niveau: niveau(valeur),
        },
        FicheCondition::Moment(jour_ou_nuit, valeur) => ConditionEvolution::Moment {
            jour_ou_nuit,
            niveau: niveau(valeur),
        },
        FicheCondition::Meteo(meteo, valeur) => ConditionEvolution::Meteo {
            meteo,
            niveau: niveau(valeur),
        },
        FicheCondition::Echange => ConditionEvolution::Echange,
    };
    // Une pierre simule chaque condition de moment, de météo ou de lieu
    // (section 5.4) ; les météos qu'aucune évolution ne demande n'en ont
    // pas.
    let substitution = match &condition {
        ConditionEvolution::Moment { jour_ou_nuit, .. } => Some(match jour_ou_nuit {
            JourOuNuit::Nuit => PierreDEvolution::Nuit,
            JourOuNuit::Jour => PierreDEvolution::Soleil,
        }),
        ConditionEvolution::Meteo { meteo, .. } => match meteo {
            Meteo::Tempete => Some(PierreDEvolution::Orage),
            Meteo::Pluie => Some(PierreDEvolution::Pluie),
            Meteo::Canicule => Some(PierreDEvolution::Chaude),
            _ => None,
        },
        ConditionEvolution::Lieu { zone, .. } if zone.as_str() == "vallee" => {
            Some(PierreDEvolution::Brumes)
        }
        _ => None,
    };
    let evolution = Evolution::new(
        EspeceId::new(fiche.vers),
        condition,
        AttaqueId::new(fiche.attaque),
    );
    let evolution = if EVOLUTIONS_SECRETES.contains(&fiche.vers) {
        evolution.en_secret()
    } else {
        evolution
    };
    match substitution {
        Some(pierre) => evolution.avec_pierre_de_substitution(pierre),
        None => evolution,
    }
}

/// Les formes d'arrivée des évolutions que le codex tait jusqu'à leur
/// capture (section 5.4).
const EVOLUTIONS_SECRETES: [&str; 1] = ["schnoz-primordial"];

/// Section 9 de docs/contenu-v1.md : amélioration, niveau atteint,
/// ressources et durée en heures.
type FichePlan = (Amelioration, u8, &'static [(&'static str, u32)], u64);

/// Section 9 de docs/contenu-v1.md, puis les machines de la section 19.
const PLANS_D_AMELIORATION: [FichePlan; 17] = [
    (
        Amelioration::Enclos,
        1,
        &[
            ("croc-de-loup", 15),
            ("plume-de-corbeau", 15),
            ("ecaille-de-vipere", 15),
        ],
        1,
    ),
    (
        Amelioration::Enclos,
        2,
        &[("cendre-de-feu-follet", 18), ("fragment-d-armure", 18)],
        2,
    ),
    (
        Amelioration::Enclos,
        3,
        &[("aile-de-fee-noire", 18), ("os-de-squelette", 18)],
        4,
    ),
    (
        Amelioration::Filets,
        1,
        &[("croc-de-loup", 12), ("ecaille-de-vipere", 12)],
        1,
    ),
    (
        Amelioration::Filets,
        2,
        &[("cendre-de-feu-follet", 15), ("fragment-d-armure", 15)],
        3,
    ),
    (
        Amelioration::Filets,
        3,
        &[("aile-de-fee-noire", 15), ("os-de-squelette", 15)],
        6,
    ),
    (
        Amelioration::Equipe,
        1,
        &[
            ("croc-de-basilic", 2),
            ("ecaille-de-vouivre", 2),
            ("os-de-squelette", 24),
            ("aile-de-fee-noire", 24),
        ],
        8,
    ),
    (
        Amelioration::Sac,
        1,
        &[("plume-de-corbeau", 18), ("ecaille-de-vipere", 18)],
        1,
    ),
    (
        Amelioration::Sac,
        2,
        &[("cendre-de-feu-follet", 18), ("fragment-d-armure", 18)],
        3,
    ),
    (
        Amelioration::Sac,
        3,
        &[("aile-de-fee-noire", 18), ("os-de-squelette", 18)],
        6,
    ),
    (
        Amelioration::Pieges,
        1,
        &[("croc-de-loup", 15), ("ecaille-de-vipere", 15)],
        1,
    ),
    (
        Amelioration::Pieges,
        2,
        &[("cendre-de-feu-follet", 18), ("fragment-d-armure", 18)],
        3,
    ),
    (
        Amelioration::Pieges,
        3,
        &[
            ("aile-de-fee-noire", 18),
            ("os-de-squelette", 18),
            ("ecaille-de-vouivre", 2),
        ],
        6,
    ),
    (
        Amelioration::Cour,
        1,
        &[("croc-de-loup", 18), ("plume-de-corbeau", 18)],
        2,
    ),
    (
        Amelioration::Cour,
        2,
        &[("fragment-d-armure", 18), ("os-de-squelette", 18)],
        4,
    ),
    (
        Amelioration::Cour,
        3,
        &[("eclat-de-gargouille", 24), ("dent-de-goule", 18)],
        8,
    ),
    // Le bateau, pour la Mer des Naufrages, avec les ressources du Port des
    // Noyés (section 9.1, étape 207, proposé par Claude, Q124).
    (
        Amelioration::Bateau,
        1,
        &[
            ("carapace-de-crabe", 15),
            ("ecaille-de-sirene", 6),
            ("plume-de-goeland", 9),
        ],
        4,
    ),
];

/// Section 20 de docs/contenu-v1.md : le charme de garde, un par travail.
const COUT_DU_CHARME: [(&str, u32); 2] = [("os-de-squelette", 9), ("fragment-d-armure", 9)];
const MINUTES_DU_CHARME: u64 = 30;

/// Section 26 de docs/contenu-v1.md : chaque lot de 5 filets, son coût et
/// sa durée en minutes.
const FILETS_PAR_LOT: u32 = 5;
type FicheDesFilets = (Filet, &'static [(&'static str, u32)], u64);
const PLANS_DES_FILETS: [FicheDesFilets; 3] = [
    (
        Filet::Chanvre,
        &[("croc-de-loup", 9), ("plume-de-corbeau", 9)],
        30,
    ),
    (
        Filet::Leste,
        &[("fragment-d-armure", 9), ("os-de-squelette", 9)],
        60,
    ),
    (
        Filet::Chaines,
        &[("eclat-de-gargouille", 9), ("pierre-de-troll", 6)],
        120,
    ),
];

/// Sections 11 et 34 de docs/contenu-v1.md. Les bonus sont en pourcentage,
/// dans l'ordre Attaque, Défense, Attaque spéciale, Défense spéciale,
/// Vitesse : l'équipement ne donne pas de PV, que le combat ne majore pas.
/// Une arme ou une armure a sa classe et son palier ; une arme donne au
/// héros ses attaques (section 4.4).
struct FicheObjet {
    id: &'static str,
    nom: &'static str,
    sorte: SorteDObjet,
    bonus: [u8; 5],
    classe: Option<(Classe, u8)>,
    attaques: &'static [&'static str],
    cout: &'static [(&'static str, u32)],
    heures: u64,
}

const PLANS_DE_FABRICATION: [FicheObjet; 65] = [
    FicheObjet {
        id: "lame-de-milicien",
        nom: "Lame de milicien",
        sorte: SorteDObjet::Arme,
        bonus: [3, 0, 3, 0, 0],
        classe: Some((Classe::Guerrier, 1)),
        attaques: &["taille", "frappe-brutale"],
        cout: &[("croc-de-loup", 18), ("ecaille-de-vipere", 12)],
        heures: 1,
    },
    FicheObjet {
        id: "epee-du-pendu",
        nom: "Épée du pendu",
        sorte: SorteDObjet::Arme,
        bonus: [5, 0, 5, 0, 0],
        classe: Some((Classe::Guerrier, 2)),
        attaques: &["estoc", "frappe-brutale", "coup-de-pommeau"],
        cout: &[
            ("fragment-d-armure", 18),
            ("os-de-squelette", 18),
            ("croc-de-basilic", 2),
        ],
        heures: 3,
    },
    FicheObjet {
        id: "espadon-des-cendres",
        nom: "Espadon des cendres",
        sorte: SorteDObjet::Arme,
        bonus: [8, 0, 8, 0, 0],
        classe: Some((Classe::Guerrier, 3)),
        attaques: &["pourfendre", "frappe-brutale", "coup-de-pommeau"],
        cout: &[
            ("eclat-de-gargouille", 24),
            ("dent-de-goule", 18),
            ("crin-de-chimere", 2),
        ],
        heures: 6,
    },
    FicheObjet {
        id: "couteau-de-chasse",
        nom: "Couteau de chasse",
        sorte: SorteDObjet::Arme,
        bonus: [3, 0, 3, 0, 0],
        classe: Some((Classe::Assassin, 1)),
        attaques: &["entaille", "lame-empoisonnee"],
        cout: &[("croc-de-loup", 18), ("ecaille-de-vipere", 12)],
        heures: 1,
    },
    FicheObjet {
        id: "dague-des-brumes",
        nom: "Dague des brumes",
        sorte: SorteDObjet::Arme,
        bonus: [5, 0, 5, 0, 0],
        classe: Some((Classe::Assassin, 2)),
        attaques: &["coup-bas", "lame-empoisonnee", "lame-des-tenebres"],
        cout: &[
            ("fragment-d-armure", 18),
            ("os-de-squelette", 18),
            ("croc-de-basilic", 2),
        ],
        heures: 3,
    },
    FicheObjet {
        id: "lames-de-cendre",
        nom: "Lames de cendre",
        sorte: SorteDObjet::Arme,
        bonus: [8, 0, 8, 0, 0],
        classe: Some((Classe::Assassin, 3)),
        attaques: &["egorgement", "lame-empoisonnee", "lame-des-tenebres"],
        cout: &[
            ("eclat-de-gargouille", 24),
            ("dent-de-goule", 18),
            ("crin-de-chimere", 2),
        ],
        heures: 6,
    },
    FicheObjet {
        id: "baton-d-if",
        nom: "Bâton d'if",
        sorte: SorteDObjet::Arme,
        bonus: [3, 0, 3, 0, 0],
        classe: Some((Classe::Mage, 1)),
        attaques: &["trait-arcanique", "onde-engourdissante"],
        cout: &[("plume-de-corbeau", 18), ("ecaille-de-vipere", 12)],
        heures: 1,
    },
    FicheObjet {
        id: "grimoire-des-brumes",
        nom: "Grimoire des brumes",
        sorte: SorteDObjet::Arme,
        bonus: [5, 0, 5, 0, 0],
        classe: Some((Classe::Mage, 2)),
        attaques: &["fleche-occulte", "onde-engourdissante", "emprise-mentale"],
        cout: &[
            ("cendre-de-feu-follet", 18),
            ("aile-de-fee-noire", 18),
            ("croc-de-basilic", 2),
        ],
        heures: 3,
    },
    FicheObjet {
        id: "sceptre-des-cendres",
        nom: "Sceptre des cendres",
        sorte: SorteDObjet::Arme,
        bonus: [8, 0, 8, 0, 0],
        classe: Some((Classe::Mage, 3)),
        attaques: &["rayon-occulte", "onde-engourdissante", "emprise-mentale"],
        cout: &[
            ("aile-de-chauve-souris", 24),
            ("dent-de-goule", 18),
            ("crin-de-chimere", 2),
        ],
        heures: 6,
    },
    FicheObjet {
        id: "masse-de-pelerin",
        nom: "Masse de pèlerin",
        sorte: SorteDObjet::Arme,
        bonus: [3, 0, 3, 0, 0],
        classe: Some((Classe::Templier, 1)),
        attaques: &["chatiment", "sceau-d-entrave"],
        cout: &[("plume-de-corbeau", 18), ("ecaille-de-vipere", 12)],
        heures: 1,
    },
    FicheObjet {
        id: "fleau-du-pendu",
        nom: "Fléau du pendu",
        sorte: SorteDObjet::Arme,
        bonus: [5, 0, 5, 0, 0],
        classe: Some((Classe::Templier, 2)),
        attaques: &["verdict", "sceau-d-entrave", "rayon-de-foi"],
        cout: &[
            ("cendre-de-feu-follet", 18),
            ("aile-de-fee-noire", 18),
            ("croc-de-basilic", 2),
        ],
        heures: 3,
    },
    FicheObjet {
        id: "encensoir-maudit",
        nom: "Encensoir maudit",
        sorte: SorteDObjet::Arme,
        bonus: [8, 0, 8, 0, 0],
        classe: Some((Classe::Templier, 3)),
        attaques: &["anatheme", "sceau-d-entrave", "rayon-de-foi"],
        cout: &[
            ("aile-de-chauve-souris", 24),
            ("dent-de-goule", 18),
            ("crin-de-chimere", 2),
        ],
        heures: 6,
    },
    // Palier 5, aux ressources des Mines de Fer-Noir (étape 163).
    FicheObjet {
        id: "hache-de-fer-noir",
        nom: "Hache de fer noir",
        sorte: SorteDObjet::Arme,
        bonus: [12, 0, 12, 0, 0],
        classe: Some((Classe::Guerrier, 5)),
        attaques: &["fend-armure", "frappe-brutale", "coup-de-pommeau"],
        cout: &[
            ("eclat-de-fer-noir", 24),
            ("pioche-maudite", 18),
            ("ecaille-de-rouille", 2),
        ],
        heures: 12,
    },
    FicheObjet {
        id: "dagues-de-fer-noir",
        nom: "Dagues de fer noir",
        sorte: SorteDObjet::Arme,
        bonus: [12, 0, 12, 0, 0],
        classe: Some((Classe::Assassin, 5)),
        attaques: &["coup-mortel", "lame-empoisonnee", "lame-des-tenebres"],
        cout: &[
            ("eclat-de-fer-noir", 24),
            ("pioche-maudite", 18),
            ("ecaille-de-rouille", 2),
        ],
        heures: 12,
    },
    FicheObjet {
        id: "baton-de-grisou",
        nom: "Bâton de grisou",
        sorte: SorteDObjet::Arme,
        bonus: [12, 0, 12, 0, 0],
        classe: Some((Classe::Mage, 5)),
        attaques: &[
            "tempete-arcanique",
            "onde-engourdissante",
            "emprise-mentale",
        ],
        cout: &[
            ("fiole-de-grisou", 24),
            ("pioche-maudite", 18),
            ("ecaille-de-rouille", 2),
        ],
        heures: 12,
    },
    FicheObjet {
        id: "masse-de-fer-noir",
        nom: "Masse de fer noir",
        sorte: SorteDObjet::Arme,
        bonus: [12, 0, 12, 0, 0],
        classe: Some((Classe::Templier, 5)),
        attaques: &["colere-du-ciel", "sceau-d-entrave", "rayon-de-foi"],
        cout: &[
            ("fiole-de-grisou", 24),
            ("pioche-maudite", 18),
            ("ecaille-de-rouille", 2),
        ],
        heures: 12,
    },
    // Palier 6, aux ressources de la Sylve écarlate et des Catacombes
    // (étape 165).
    FicheObjet {
        id: "hache-du-bourreau-d-os",
        nom: "Hache du bourreau d'os",
        sorte: SorteDObjet::Arme,
        bonus: [14, 0, 14, 0, 0],
        classe: Some((Classe::Guerrier, 6)),
        attaques: &["decapitation", "frappe-brutale", "coup-de-pommeau"],
        cout: &[
            ("soie-des-cryptes", 24),
            ("lame-de-mante", 18),
            ("phylactere-brise", 2),
        ],
        heures: 14,
    },
    FicheObjet {
        id: "lames-de-soie",
        nom: "Lames de soie",
        sorte: SorteDObjet::Arme,
        bonus: [14, 0, 14, 0, 0],
        classe: Some((Classe::Assassin, 6)),
        attaques: &["lame-fantome", "lame-empoisonnee", "lame-des-tenebres"],
        cout: &[
            ("soie-des-cryptes", 24),
            ("lame-de-mante", 18),
            ("phylactere-brise", 2),
        ],
        heures: 14,
    },
    FicheObjet {
        id: "crosse-des-catacombes",
        nom: "Crosse des catacombes",
        sorte: SorteDObjet::Arme,
        bonus: [14, 0, 14, 0, 0],
        classe: Some((Classe::Mage, 6)),
        attaques: &["arcane-interdit", "onde-engourdissante", "emprise-mentale"],
        cout: &[
            ("chapelet-d-os", 24),
            ("epine-de-ronce", 18),
            ("phylactere-brise", 2),
        ],
        heures: 14,
    },
    FicheObjet {
        id: "masse-ecarlate",
        nom: "Masse écarlate",
        sorte: SorteDObjet::Arme,
        bonus: [14, 0, 14, 0, 0],
        classe: Some((Classe::Templier, 6)),
        attaques: &["jugement-dernier", "sceau-d-entrave", "rayon-de-foi"],
        cout: &[
            ("chapelet-d-os", 24),
            ("epine-de-ronce", 18),
            ("phylactere-brise", 2),
        ],
        heures: 14,
    },
    // Palier 7 (étape 166).
    FicheObjet {
        id: "fendoir-de-guerre",
        nom: "Fendoir de guerre",
        sorte: SorteDObjet::Arme,
        bonus: [16, 0, 16, 0, 0],
        classe: Some((Classe::Guerrier, 7)),
        attaques: &["brise-ost", "frappe-brutale", "coup-de-pommeau"],
        cout: &[
            ("collier-a-pointes", 24),
            ("crin-spectral", 18),
            ("eclat-d-archonte", 2),
        ],
        heures: 16,
    },
    FicheObjet {
        id: "dagues-du-massacre",
        nom: "Dagues du massacre",
        sorte: SorteDObjet::Arme,
        bonus: [16, 0, 16, 0, 0],
        classe: Some((Classe::Assassin, 7)),
        attaques: &["mille-coupures", "lame-empoisonnee", "lame-des-tenebres"],
        cout: &[
            ("collier-a-pointes", 24),
            ("crin-spectral", 18),
            ("eclat-d-archonte", 2),
        ],
        heures: 16,
    },
    FicheObjet {
        id: "sceptre-d-ether",
        nom: "Sceptre d'éther",
        sorte: SorteDObjet::Arme,
        bonus: [16, 0, 16, 0, 0],
        classe: Some((Classe::Mage, 7)),
        attaques: &["cataclysme", "onde-engourdissante", "emprise-mentale"],
        cout: &[
            ("encre-vivante", 24),
            ("cristallin", 18),
            ("eclat-d-archonte", 2),
        ],
        heures: 16,
    },
    FicheObjet {
        id: "marteau-du-griffon",
        nom: "Marteau du griffon",
        sorte: SorteDObjet::Arme,
        bonus: [16, 0, 16, 0, 0],
        classe: Some((Classe::Templier, 7)),
        attaques: &["lumiere-divine", "sceau-d-entrave", "rayon-de-foi"],
        cout: &[
            ("encre-vivante", 24),
            ("cristallin", 18),
            ("eclat-d-archonte", 2),
        ],
        heures: 16,
    },
    // Palier 8 (étape 167).
    FicheObjet {
        id: "hache-de-magma",
        nom: "Hache de magma",
        sorte: SorteDObjet::Arme,
        bonus: [18, 0, 18, 0, 0],
        classe: Some((Classe::Guerrier, 8)),
        attaques: &["ravage", "frappe-brutale", "coup-de-pommeau"],
        cout: &[
            ("corne-de-granit", 24),
            ("croc-calcine", 18),
            ("coeur-de-tonnerre", 2),
        ],
        heures: 18,
    },
    FicheObjet {
        id: "dagues-foudroyees",
        nom: "Dagues foudroyées",
        sorte: SorteDObjet::Arme,
        bonus: [18, 0, 18, 0, 0],
        classe: Some((Classe::Assassin, 8)),
        attaques: &["mort-silencieuse", "lame-empoisonnee", "lame-des-tenebres"],
        cout: &[
            ("corne-de-granit", 24),
            ("croc-calcine", 18),
            ("coeur-de-tonnerre", 2),
        ],
        heures: 18,
    },
    FicheObjet {
        id: "baton-des-orages",
        nom: "Bâton des orages",
        sorte: SorteDObjet::Arme,
        bonus: [18, 0, 18, 0, 0],
        classe: Some((Classe::Mage, 8)),
        attaques: &["apocalypse", "onde-engourdissante", "emprise-mentale"],
        cout: &[
            ("etincelle-captive", 24),
            ("scorie-de-magma", 18),
            ("coeur-de-tonnerre", 2),
        ],
        heures: 18,
    },
    FicheObjet {
        id: "marteau-de-lave",
        nom: "Marteau de lave",
        sorte: SorteDObjet::Arme,
        bonus: [18, 0, 18, 0, 0],
        classe: Some((Classe::Templier, 8)),
        attaques: &["glaive-celeste", "sceau-d-entrave", "rayon-de-foi"],
        cout: &[
            ("etincelle-captive", 24),
            ("scorie-de-magma", 18),
            ("coeur-de-tonnerre", 2),
        ],
        heures: 18,
    },
    FicheObjet {
        id: "gambison",
        nom: "Gambison",
        sorte: SorteDObjet::Armure,
        bonus: [0, 5, 0, 5, 0],
        classe: Some((Classe::Guerrier, 1)),
        attaques: &[],
        cout: &[("croc-de-loup", 18), ("plume-de-corbeau", 18)],
        heures: 2,
    },
    FicheObjet {
        id: "cotte-de-mailles",
        nom: "Cotte de mailles",
        sorte: SorteDObjet::Armure,
        bonus: [0, 10, 0, 10, 0],
        classe: Some((Classe::Guerrier, 2)),
        attaques: &[],
        cout: &[
            ("fragment-d-armure", 24),
            ("os-de-squelette", 12),
            ("ecaille-de-vouivre", 2),
        ],
        heures: 4,
    },
    FicheObjet {
        id: "plates-de-gargouille",
        nom: "Plates de gargouille",
        sorte: SorteDObjet::Armure,
        bonus: [0, 15, 0, 15, 0],
        classe: Some((Classe::Guerrier, 3)),
        attaques: &[],
        cout: &[
            ("eclat-de-gargouille", 30),
            ("os-de-squelette", 18),
            ("crin-de-chimere", 2),
        ],
        heures: 8,
    },
    FicheObjet {
        id: "brigandine-de-cuir",
        nom: "Brigandine de cuir",
        sorte: SorteDObjet::Armure,
        bonus: [0, 5, 0, 5, 0],
        classe: Some((Classe::Assassin, 1)),
        attaques: &[],
        cout: &[("croc-de-loup", 18), ("plume-de-corbeau", 18)],
        heures: 2,
    },
    FicheObjet {
        id: "cuir-des-brumes",
        nom: "Cuir des brumes",
        sorte: SorteDObjet::Armure,
        bonus: [0, 10, 0, 10, 0],
        classe: Some((Classe::Assassin, 2)),
        attaques: &[],
        cout: &[
            ("fragment-d-armure", 24),
            ("os-de-squelette", 12),
            ("ecaille-de-vouivre", 2),
        ],
        heures: 4,
    },
    FicheObjet {
        id: "manteau-de-cendre",
        nom: "Manteau de cendre",
        sorte: SorteDObjet::Armure,
        bonus: [0, 15, 0, 15, 0],
        classe: Some((Classe::Assassin, 3)),
        attaques: &[],
        cout: &[
            ("eclat-de-gargouille", 30),
            ("os-de-squelette", 18),
            ("crin-de-chimere", 2),
        ],
        heures: 8,
    },
    FicheObjet {
        id: "robe-de-lin",
        nom: "Robe de lin",
        sorte: SorteDObjet::Armure,
        bonus: [0, 5, 0, 5, 0],
        classe: Some((Classe::Mage, 1)),
        attaques: &[],
        cout: &[("croc-de-loup", 18), ("plume-de-corbeau", 18)],
        heures: 2,
    },
    FicheObjet {
        id: "robe-des-brumes",
        nom: "Robe des brumes",
        sorte: SorteDObjet::Armure,
        bonus: [0, 10, 0, 10, 0],
        classe: Some((Classe::Mage, 2)),
        attaques: &[],
        cout: &[
            ("fragment-d-armure", 24),
            ("os-de-squelette", 12),
            ("ecaille-de-vouivre", 2),
        ],
        heures: 4,
    },
    FicheObjet {
        id: "robe-des-cendres",
        nom: "Robe des cendres",
        sorte: SorteDObjet::Armure,
        bonus: [0, 15, 0, 15, 0],
        classe: Some((Classe::Mage, 3)),
        attaques: &[],
        cout: &[
            ("eclat-de-gargouille", 30),
            ("os-de-squelette", 18),
            ("crin-de-chimere", 2),
        ],
        heures: 8,
    },
    FicheObjet {
        id: "haubert-de-pelerin",
        nom: "Haubert de pèlerin",
        sorte: SorteDObjet::Armure,
        bonus: [0, 5, 0, 5, 0],
        classe: Some((Classe::Templier, 1)),
        attaques: &[],
        cout: &[("croc-de-loup", 18), ("plume-de-corbeau", 18)],
        heures: 2,
    },
    FicheObjet {
        id: "cotte-benie",
        nom: "Cotte bénie",
        sorte: SorteDObjet::Armure,
        bonus: [0, 10, 0, 10, 0],
        classe: Some((Classe::Templier, 2)),
        attaques: &[],
        cout: &[
            ("fragment-d-armure", 24),
            ("os-de-squelette", 12),
            ("ecaille-de-vouivre", 2),
        ],
        heures: 4,
    },
    FicheObjet {
        id: "plates-consacrees",
        nom: "Plates consacrées",
        sorte: SorteDObjet::Armure,
        bonus: [0, 15, 0, 15, 0],
        classe: Some((Classe::Templier, 3)),
        attaques: &[],
        cout: &[
            ("eclat-de-gargouille", 30),
            ("os-de-squelette", 18),
            ("crin-de-chimere", 2),
        ],
        heures: 8,
    },
    // Palier 4, aux ressources du Port des Noyés, puis palier 5, à celles
    // des Mines de Fer-Noir (étape 163).
    FicheObjet {
        id: "cuirasse-des-abysses",
        nom: "Cuirasse des abysses",
        sorte: SorteDObjet::Armure,
        bonus: [0, 20, 0, 20, 0],
        classe: Some((Classe::Guerrier, 4)),
        attaques: &[],
        cout: &[
            ("carapace-de-crabe", 30),
            ("ecaille-de-sirene", 18),
            ("croc-d-hydre", 2),
        ],
        heures: 10,
    },
    FicheObjet {
        id: "cuir-de-requin",
        nom: "Cuir de requin",
        sorte: SorteDObjet::Armure,
        bonus: [0, 20, 0, 20, 0],
        classe: Some((Classe::Assassin, 4)),
        attaques: &[],
        cout: &[
            ("carapace-de-crabe", 30),
            ("ecaille-de-sirene", 18),
            ("croc-d-hydre", 2),
        ],
        heures: 10,
    },
    FicheObjet {
        id: "robe-des-marees",
        nom: "Robe des marées",
        sorte: SorteDObjet::Armure,
        bonus: [0, 20, 0, 20, 0],
        classe: Some((Classe::Mage, 4)),
        attaques: &[],
        cout: &[
            ("carapace-de-crabe", 30),
            ("ecaille-de-sirene", 18),
            ("croc-d-hydre", 2),
        ],
        heures: 10,
    },
    FicheObjet {
        id: "plates-nacrees",
        nom: "Plates nacrées",
        sorte: SorteDObjet::Armure,
        bonus: [0, 20, 0, 20, 0],
        classe: Some((Classe::Templier, 4)),
        attaques: &[],
        cout: &[
            ("carapace-de-crabe", 30),
            ("ecaille-de-sirene", 18),
            ("croc-d-hydre", 2),
        ],
        heures: 10,
    },
    FicheObjet {
        id: "plates-de-fer-noir",
        nom: "Plates de fer noir",
        sorte: SorteDObjet::Armure,
        bonus: [0, 25, 0, 25, 0],
        classe: Some((Classe::Guerrier, 5)),
        attaques: &[],
        cout: &[
            ("eclat-de-fer-noir", 30),
            ("carapace-de-crabe", 18),
            ("ecaille-de-rouille", 2),
        ],
        heures: 14,
    },
    FicheObjet {
        id: "maille-de-fer-noir",
        nom: "Maille de fer noir",
        sorte: SorteDObjet::Armure,
        bonus: [0, 25, 0, 25, 0],
        classe: Some((Classe::Assassin, 5)),
        attaques: &[],
        cout: &[
            ("eclat-de-fer-noir", 30),
            ("carapace-de-crabe", 18),
            ("ecaille-de-rouille", 2),
        ],
        heures: 14,
    },
    FicheObjet {
        id: "robe-de-grisou",
        nom: "Robe de grisou",
        sorte: SorteDObjet::Armure,
        bonus: [0, 25, 0, 25, 0],
        classe: Some((Classe::Mage, 5)),
        attaques: &[],
        cout: &[
            ("eclat-de-fer-noir", 30),
            ("carapace-de-crabe", 18),
            ("ecaille-de-rouille", 2),
        ],
        heures: 14,
    },
    FicheObjet {
        id: "harnois-de-fer-noir",
        nom: "Harnois de fer noir",
        sorte: SorteDObjet::Armure,
        bonus: [0, 25, 0, 25, 0],
        classe: Some((Classe::Templier, 5)),
        attaques: &[],
        cout: &[
            ("eclat-de-fer-noir", 30),
            ("carapace-de-crabe", 18),
            ("ecaille-de-rouille", 2),
        ],
        heures: 14,
    },
    // Palier 6 (étape 165).
    FicheObjet {
        id: "plates-du-sanglier",
        nom: "Plates du sanglier",
        sorte: SorteDObjet::Armure,
        bonus: [0, 30, 0, 30, 0],
        classe: Some((Classe::Guerrier, 6)),
        attaques: &[],
        cout: &[
            ("defense-de-sanglier", 30),
            ("queue-de-rat", 18),
            ("seve-de-dryade", 2),
        ],
        heures: 16,
    },
    FicheObjet {
        id: "cuir-ecarlate",
        nom: "Cuir écarlate",
        sorte: SorteDObjet::Armure,
        bonus: [0, 30, 0, 30, 0],
        classe: Some((Classe::Assassin, 6)),
        attaques: &[],
        cout: &[
            ("defense-de-sanglier", 30),
            ("queue-de-rat", 18),
            ("seve-de-dryade", 2),
        ],
        heures: 16,
    },
    FicheObjet {
        id: "robe-de-ronces",
        nom: "Robe de ronces",
        sorte: SorteDObjet::Armure,
        bonus: [0, 30, 0, 30, 0],
        classe: Some((Classe::Mage, 6)),
        attaques: &[],
        cout: &[
            ("defense-de-sanglier", 30),
            ("queue-de-rat", 18),
            ("seve-de-dryade", 2),
        ],
        heures: 16,
    },
    FicheObjet {
        id: "harnois-des-saints",
        nom: "Harnois des saints",
        sorte: SorteDObjet::Armure,
        bonus: [0, 30, 0, 30, 0],
        classe: Some((Classe::Templier, 6)),
        attaques: &[],
        cout: &[
            ("defense-de-sanglier", 30),
            ("queue-de-rat", 18),
            ("seve-de-dryade", 2),
        ],
        heures: 16,
    },
    // Palier 7 (étape 166).
    FicheObjet {
        id: "cuirasse-du-griffon",
        nom: "Cuirasse du griffon",
        sorte: SorteDObjet::Armure,
        bonus: [0, 35, 0, 35, 0],
        classe: Some((Classe::Guerrier, 7)),
        attaques: &[],
        cout: &[
            ("rouage-brise", 30),
            ("plume-de-vautour", 18),
            ("serre-de-griffon", 2),
        ],
        heures: 18,
    },
    FicheObjet {
        id: "cuir-du-vautour",
        nom: "Cuir du vautour",
        sorte: SorteDObjet::Armure,
        bonus: [0, 35, 0, 35, 0],
        classe: Some((Classe::Assassin, 7)),
        attaques: &[],
        cout: &[
            ("rouage-brise", 30),
            ("plume-de-vautour", 18),
            ("serre-de-griffon", 2),
        ],
        heures: 18,
    },
    FicheObjet {
        id: "robe-des-archives",
        nom: "Robe des archives",
        sorte: SorteDObjet::Armure,
        bonus: [0, 35, 0, 35, 0],
        classe: Some((Classe::Mage, 7)),
        attaques: &[],
        cout: &[
            ("rouage-brise", 30),
            ("plume-de-vautour", 18),
            ("serre-de-griffon", 2),
        ],
        heures: 18,
    },
    FicheObjet {
        id: "plates-de-la-cite",
        nom: "Plates de la cité",
        sorte: SorteDObjet::Armure,
        bonus: [0, 35, 0, 35, 0],
        classe: Some((Classe::Templier, 7)),
        attaques: &[],
        cout: &[
            ("rouage-brise", 30),
            ("plume-de-vautour", 18),
            ("serre-de-griffon", 2),
        ],
        heures: 18,
    },
    // Palier 8 (étape 167).
    FicheObjet {
        id: "plates-de-magma",
        nom: "Plates de magma",
        sorte: SorteDObjet::Armure,
        bonus: [0, 40, 0, 40, 0],
        classe: Some((Classe::Guerrier, 8)),
        attaques: &[],
        cout: &[
            ("lingot-noirci", 30),
            ("plume-d-orage", 18),
            ("ecaille-de-magma", 2),
        ],
        heures: 20,
    },
    FicheObjet {
        id: "cuir-d-orage",
        nom: "Cuir d'orage",
        sorte: SorteDObjet::Armure,
        bonus: [0, 40, 0, 40, 0],
        classe: Some((Classe::Assassin, 8)),
        attaques: &[],
        cout: &[
            ("lingot-noirci", 30),
            ("plume-d-orage", 18),
            ("ecaille-de-magma", 2),
        ],
        heures: 20,
    },
    FicheObjet {
        id: "robe-de-cendre",
        nom: "Robe de cendre",
        sorte: SorteDObjet::Armure,
        bonus: [0, 40, 0, 40, 0],
        classe: Some((Classe::Mage, 8)),
        attaques: &[],
        cout: &[
            ("lingot-noirci", 30),
            ("plume-d-orage", 18),
            ("ecaille-de-magma", 2),
        ],
        heures: 20,
    },
    FicheObjet {
        id: "harnois-foudroye",
        nom: "Harnois foudroyé",
        sorte: SorteDObjet::Armure,
        bonus: [0, 40, 0, 40, 0],
        classe: Some((Classe::Templier, 8)),
        attaques: &[],
        cout: &[
            ("lingot-noirci", 30),
            ("plume-d-orage", 18),
            ("ecaille-de-magma", 2),
        ],
        heures: 20,
    },
    FicheObjet {
        id: "croc-tresse",
        nom: "Croc tressé",
        sorte: SorteDObjet::Talisman,
        bonus: [5, 0, 0, 0, 0],
        classe: None,
        attaques: &[],
        cout: &[("croc-de-loup", 24)],
        heures: 1,
    },
    FicheObjet {
        id: "plume-de-corbeau-sertie",
        nom: "Plume de corbeau sertie",
        sorte: SorteDObjet::Talisman,
        bonus: [0, 0, 0, 0, 5],
        classe: None,
        attaques: &[],
        cout: &[("plume-de-corbeau", 24)],
        heures: 1,
    },
    FicheObjet {
        id: "lanterne-de-feu-follet",
        nom: "Lanterne de feu follet",
        sorte: SorteDObjet::Talisman,
        bonus: [0, 0, 5, 0, 0],
        classe: None,
        attaques: &[],
        cout: &[("cendre-de-feu-follet", 24)],
        heures: 2,
    },
    FicheObjet {
        id: "os-grave",
        nom: "Os gravé",
        sorte: SorteDObjet::Talisman,
        bonus: [0, 5, 0, 0, 0],
        classe: None,
        attaques: &[],
        cout: &[("os-de-squelette", 24)],
        heures: 2,
    },
    FicheObjet {
        id: "aile-de-fee-sertie",
        nom: "Aile de fée sertie",
        sorte: SorteDObjet::Talisman,
        bonus: [0, 0, 0, 5, 0],
        classe: None,
        attaques: &[],
        cout: &[("aile-de-fee-noire", 24)],
        heures: 2,
    },
];

/// Section 34 de docs/contenu-v1.md : les armes du palier 4, récompense du
/// dernier palier de la Fosse aux damnés, qui ne se fabriquent pas
/// (section 9.8).
const ARMES_DU_CHAMPION: [FicheObjet; 4] = [
    FicheObjet {
        id: "glaive-du-champion",
        nom: "Glaive du champion",
        sorte: SorteDObjet::Arme,
        bonus: [10, 0, 10, 0, 0],
        classe: Some((Classe::Guerrier, 4)),
        attaques: &["coup-du-champion", "frappe-brutale", "coup-de-pommeau"],
        cout: &[],
        heures: 0,
    },
    FicheObjet {
        id: "crocs-du-champion",
        nom: "Crocs du champion",
        sorte: SorteDObjet::Arme,
        bonus: [10, 0, 10, 0, 0],
        classe: Some((Classe::Assassin, 4)),
        attaques: &["execution", "lame-empoisonnee", "lame-des-tenebres"],
        cout: &[],
        heures: 0,
    },
    FicheObjet {
        id: "grimoire-du-champion",
        nom: "Grimoire du champion",
        sorte: SorteDObjet::Arme,
        bonus: [10, 0, 10, 0, 0],
        classe: Some((Classe::Mage, 4)),
        attaques: &["tempete-occulte", "onde-engourdissante", "emprise-mentale"],
        cout: &[],
        heures: 0,
    },
    FicheObjet {
        id: "marteau-du-champion",
        nom: "Marteau du champion",
        sorte: SorteDObjet::Arme,
        bonus: [10, 0, 10, 0, 0],
        classe: Some((Classe::Templier, 4)),
        attaques: &["courroux-sacre", "sceau-d-entrave", "rayon-de-foi"],
        cout: &[],
        heures: 0,
    },
];

/// Section 30 de docs/contenu-v1.md : la récompense du codex complet, qui ne
/// se fabrique pas.
const SCEAU_DU_BESTIAIRE: FicheObjet = FicheObjet {
    id: "sceau-du-bestiaire",
    nom: "Sceau du bestiaire",
    sorte: SorteDObjet::Talisman,
    bonus: [5, 5, 5, 5, 5],
    classe: None,
    attaques: &[],
    cout: &[],
    heures: 0,
};

/// Section 30 de docs/contenu-v1.md : une récompense toutes les 10 espèces
/// capturées, puis au codex complet.
fn creer_recompenses_du_codex() -> Vec<RecompenseDuCodex> {
    let recompense = |palier, objets| RecompenseDuCodex {
        palier,
        objets,
        objet_unique: None,
    };
    let boosters = Booster::TOUS
        .into_iter()
        .fold(Inventaire::default(), |objets, booster| {
            objets.avec_boosters(booster, 1)
        });
    vec![
        recompense(
            PalierDuCodex::Especes(10),
            Inventaire::new(300, 0, Vec::new()).avec_filets(Filet::Leste, 10),
        ),
        recompense(
            PalierDuCodex::Especes(20),
            Inventaire::new(500, 0, Vec::new()).avec_filets(Filet::Chaines, 5),
        ),
        // Des objets d'expérience à la place du Philtre chromatique depuis
        // l'étape 211 (section 14.3).
        recompense(
            PalierDuCodex::Especes(30),
            Inventaire::default().avec_objets_d_experience(ObjetDExperience::Cristal, 2),
        ),
        recompense(
            PalierDuCodex::Especes(40),
            Inventaire::default().avec_capsules_d_habilete(3),
        ),
        recompense(PalierDuCodex::Especes(50), boosters),
        RecompenseDuCodex {
            palier: PalierDuCodex::Complet,
            objets: Inventaire::default()
                .avec_philtres_chromatiques(1)
                .avec_objets_d_experience(ObjetDExperience::Coeur, 3),
            objet_unique: Some(creer_objet(&SCEAU_DU_BESTIAIRE)),
        },
    ]
}

/// Section 36 de docs/contenu-v1.md : l'adversaire de chaque palier, avec
/// sa classe, son arme et son armure, son IA, son texte, l'argent de sa
/// récompense et le monstre qu'il donne, avec son niveau.
struct FicheDAdversaire {
    id: &'static str,
    nom: &'static str,
    classe: Classe,
    arme: Option<&'static str>,
    /// Fixé à part de l'équipement, de 2 ou 3 niveaux d'un palier à
    /// l'autre (section 9.8).
    niveau: u8,
    ia: IaHumain,
    presentation: &'static str,
    argent: u32,
    /// Au niveau du palier.
    monstre: Option<(&'static str, u8)>,
    /// Il prend la classe du héros qui l'affronte, avec l'arme de sa classe
    /// au palier de la sienne (section 9.8).
    miroir: bool,
}

const ADVERSAIRES_DE_LA_FOSSE: [FicheDAdversaire; 10] = [
    FicheDAdversaire {
        id: "gueux",
        nom: "Le Gueux",
        classe: Classe::Guerrier,
        arme: Some("lame-de-milicien"),
        niveau: 10,
        ia: IaHumain::Simple,
        presentation: "Un vagabond en haillons, un gourdin à la main. Il n'a plus rien à perdre.",
        argent: 40,
        monstre: None,
        miroir: false,
    },
    FicheDAdversaire {
        id: "coupe-jarret",
        nom: "Le Coupe-jarret",
        classe: Classe::Assassin,
        arme: Some("couteau-de-chasse"),
        niveau: 13,
        ia: IaHumain::Simple,
        presentation: "Il tourne autour de vous en souriant, une lame courte dans chaque main.",
        argent: 60,
        monstre: None,
        miroir: false,
    },
    FicheDAdversaire {
        id: "novice-de-l-ordre",
        nom: "Le Novice de l'ordre",
        classe: Classe::Templier,
        arme: Some("masse-de-pelerin"),
        niveau: 16,
        ia: IaHumain::Simple,
        presentation: "Un jeune frère en armure trop grande, qui prie à voix basse en levant sa masse.",
        argent: 80,
        monstre: Some(("basilic", 16)),
        miroir: false,
    },
    FicheDAdversaire {
        id: "apprenti-necromant",
        nom: "L'Apprenti nécromant",
        classe: Classe::Mage,
        arme: Some("grimoire-des-brumes"),
        niveau: 18,
        ia: IaHumain::Simple,
        presentation: "Sa robe sent la tombe ; une lueur malsaine danse au bout de ses doigts.",
        argent: 100,
        monstre: None,
        miroir: false,
    },
    FicheDAdversaire {
        id: "mercenaire",
        nom: "Le Mercenaire",
        classe: Classe::Guerrier,
        arme: Some("epee-du-pendu"),
        niveau: 21,
        ia: IaHumain::Avancee,
        presentation: "Il a vendu son épée à tous les seigneurs du royaume, et leur a survécu à tous.",
        argent: 120,
        monstre: Some(("molosse-des-fosses", 21)),
        miroir: false,
    },
    FicheDAdversaire {
        id: "lame-noire",
        nom: "La Lame noire",
        classe: Classe::Assassin,
        arme: Some("dague-des-brumes"),
        niveau: 24,
        ia: IaHumain::Avancee,
        presentation: "Une silhouette encapuchonnée, dont on dit qu'elle n'a jamais raté une gorge.",
        argent: 150,
        monstre: None,
        miroir: false,
    },
    FicheDAdversaire {
        id: "sorciere-des-cendres",
        nom: "La Sorcière des cendres",
        classe: Classe::Mage,
        arme: Some("sceptre-des-cendres"),
        niveau: 27,
        ia: IaHumain::Avancee,
        presentation: "Elle sort des ruines de l'Abbaye, la peau grise de cendre, les yeux brûlants.",
        argent: 180,
        monstre: Some(("chimere", 27)),
        miroir: true,
    },
    FicheDAdversaire {
        id: "templier-renegat",
        nom: "Le Templier renégat",
        classe: Classe::Templier,
        arme: Some("encensoir-maudit"),
        niveau: 29,
        ia: IaHumain::Avancee,
        presentation: "Il a renié son ordre, mais pas sa foi ni son armure.",
        argent: 210,
        monstre: None,
        miroir: false,
    },
    FicheDAdversaire {
        id: "gladiateur",
        nom: "Le Gladiateur",
        classe: Classe::Guerrier,
        arme: Some("espadon-des-cendres"),
        niveau: 32,
        ia: IaHumain::Avancee,
        presentation: "La foule scande son nom. Il salue les gradins avant de vous faire face.",
        argent: 250,
        monstre: None,
        miroir: true,
    },
    FicheDAdversaire {
        id: "champion-de-la-fosse",
        nom: "Le Champion de la Fosse",
        classe: Classe::Assassin,
        arme: Some("lames-de-cendre"),
        niveau: 35,
        ia: IaHumain::Avancee,
        presentation: "Personne ne l'a jamais vu saigner. Les gradins se taisent quand il entre.",
        argent: 400,
        monstre: Some(("taureau-d-airain", 35)),
        miroir: true,
    },
];

/// Le dernier palier donne aussi l'arme du palier 4 de la classe du héros ;
/// la défaite du brigand ouvre la Fosse (section 9.8).
fn creer_la_fosse(trouver: &dyn Fn(&str) -> Espece) -> FosseAuxDamnes {
    let adversaires = ADVERSAIRES_DE_LA_FOSSE
        .iter()
        .enumerate()
        .map(|(position, fiche)| {
            let recompense = RecompenseDeLArene {
                argent: fiche.argent,
                monstre: fiche
                    .monstre
                    .map(|(espece, niveau_offert)| (trouver(espece), niveau(niveau_offert))),
                arme_du_champion: position + 1 == ADVERSAIRES_DE_LA_FOSSE.len(),
            };
            let adversaire = AdversaireDeLArene::new(
                HumainId::new(fiche.id),
                fiche.nom,
                fiche.classe,
                fiche.arme.map(objet),
                niveau(fiche.niveau),
                fiche.ia,
                fiche.presentation,
                recompense,
            );
            if !fiche.miroir {
                return adversaire;
            }
            let palier = fiche.arme.map_or(1, |id| objet(id).palier());
            let armes = Classe::TOUTES
                .iter()
                .map(|classe| {
                    PLANS_DE_FABRICATION
                        .iter()
                        .find(|arme| {
                            arme.sorte == SorteDObjet::Arme
                                && arme.classe == Some((*classe, palier))
                        })
                        .map(creer_objet)
                        .expect("une arme par classe et par palier")
                })
                .collect();
            adversaire.en_miroir(armes)
        })
        .collect();
    FosseAuxDamnes::new(adversaires, HumainId::new("brigand"))
}

/// Une arme ou une armure de docs/contenu-v1.md, section 34, comme celles
/// des humains de l'IA (section 10.7).
fn objet(id: &str) -> ObjetDEquipement {
    PLANS_DE_FABRICATION
        .iter()
        .chain(&ARMES_DU_CHAMPION)
        .find(|fiche| fiche.id == id)
        .map(creer_objet)
        .unwrap_or_else(|| panic!("objet {id} absent du contenu"))
}

fn creer_objet(fiche: &FicheObjet) -> ObjetDEquipement {
    let [
        attaque,
        defense,
        attaque_speciale,
        defense_speciale,
        vitesse,
    ] = fiche.bonus;
    let objet = ObjetDEquipement::new(
        ObjetDEquipementId::new(fiche.id),
        fiche.nom,
        fiche.sorte,
        Stats {
            pv: 0,
            attaque,
            defense,
            attaque_speciale,
            defense_speciale,
            vitesse,
        },
    )
    .avec_attaques(fiche.attaques.iter().map(|id| creer_attaque(id)).collect());
    match fiche.classe {
        Some((classe, palier)) => objet.de_classe(classe, palier),
        None => objet,
    }
}

/// Ce que Malvoise donne d'une ressource, selon la rareté de son espèce
/// (section 14.3, Q82).
const PRIX_D_UNE_RESSOURCE_COMMUNE: u32 = 5;
const PRIX_D_UNE_RESSOURCE_RARE: u32 = 15;

/// Espèce, identifiant et nom de chaque ressource : seules les espèces
/// sauvages qui se combattent en ont une (section 14.3).
const RESSOURCES: [(&str, &str, &str); 109] = [
    ("loup", "croc-de-loup", "Croc de loup"),
    ("corbeau", "plume-de-corbeau", "Plume de corbeau"),
    ("vipere", "ecaille-de-vipere", "Écaille de vipère"),
    ("feu-follet", "cendre-de-feu-follet", "Cendre de feu follet"),
    ("armure-hantee", "fragment-d-armure", "Fragment d'armure"),
    ("fee-noire", "aile-de-fee-noire", "Aile de fée noire"),
    ("squelette", "os-de-squelette", "Os de squelette"),
    ("basilic", "croc-de-basilic", "Croc de basilic"),
    ("vouivre", "ecaille-de-vouivre", "Écaille de vouivre"),
    ("gargouille", "eclat-de-gargouille", "Éclat de gargouille"),
    ("goule", "dent-de-goule", "Dent de goule"),
    (
        "chauve-souris-de-crypte",
        "aile-de-chauve-souris",
        "Aile de chauve-souris",
    ),
    ("chimere", "crin-de-chimere", "Crin de chimère"),
    ("wendigo", "poil-de-wendigo", "Poil de wendigo"),
    ("troll-des-cavernes", "pierre-de-troll", "Pierre de troll"),
    ("harpie-des-orages", "plume-de-harpie", "Plume de harpie"),
    ("wyrm-de-givre", "ecaille-de-wyrm", "Écaille de wyrm"),
    // Formes évoluées sauvages du Col : la ressource de leur forme de base
    // (section 5.4).
    ("faux-agneau", "croc-de-loup", "Croc de loup"),
    ("gisant", "fragment-d-armure", "Fragment d'armure"),
    ("cloche-hurlante", "eclat-de-bronze", "Éclat de bronze"),
    (
        "crotale-des-salines",
        "sonnette-de-crotale",
        "Sonnette de crotale",
    ),
    ("scarabee-solaire", "elytre-solaire", "Élytre solaire"),
    ("momie-de-sel", "bandelette-de-sel", "Bandelette de sel"),
    ("ver-des-salines", "anneau-de-ver", "Anneau de ver"),
    ("sphinx-des-sables", "griffe-de-sphinx", "Griffe de sphinx"),
    ("crabe-necrophage", "carapace-de-crabe", "Carapace de crabe"),
    ("sirene-putride", "ecaille-de-sirene", "Écaille de sirène"),
    ("goeland-charognard", "plume-de-goeland", "Plume de goéland"),
    ("hydre-des-marees", "croc-d-hydre", "Croc d'hydre"),
    (
        "golem-de-fer-noir",
        "eclat-de-fer-noir",
        "Éclat de fer noir",
    ),
    ("mineur-possede", "pioche-maudite", "Pioche maudite"),
    ("grisou-ardent", "fiole-de-grisou", "Fiole de grisou"),
    (
        "dragon-de-rouille",
        "ecaille-de-rouille",
        "Écaille de rouille",
    ),
    ("ronce-vampire", "epine-de-ronce", "Épine de ronce"),
    ("mante-ecarlate", "lame-de-mante", "Lame de mante"),
    (
        "sanglier-enrage",
        "defense-de-sanglier",
        "Défense de sanglier",
    ),
    ("dryade-sanglante", "seve-de-dryade", "Sève de dryade"),
    ("rat-des-ossuaires", "queue-de-rat", "Queue de rat"),
    ("moine-psalmodiant", "chapelet-d-os", "Chapelet d'os"),
    (
        "araignee-des-cryptes",
        "soie-des-cryptes",
        "Soie des cryptes",
    ),
    ("liche", "phylactere-brise", "Phylactère brisé"),
    ("destrier-spectral", "crin-spectral", "Crin spectral"),
    (
        "vautour-des-charniers",
        "plume-de-vautour",
        "Plume de vautour",
    ),
    ("matin-de-guerre", "collier-a-pointes", "Collier à pointes"),
    ("griffon-cuirasse", "serre-de-griffon", "Serre de griffon"),
    ("automate-brise", "rouage-brise", "Rouage brisé"),
    ("bibliophage", "encre-vivante", "Encre vivante"),
    ("oeil-errant", "cristallin", "Cristallin"),
    ("archonte-foudroye", "eclat-d-archonte", "Éclat d'archonte"),
    ("golem-de-magma", "scorie-de-magma", "Scorie de magma"),
    ("chien-des-scories", "croc-calcine", "Croc calciné"),
    ("forgeron-calcine", "lingot-noirci", "Lingot noirci"),
    ("wyrm-de-magma", "ecaille-de-magma", "Écaille de magma"),
    ("aigle-tonnerre", "plume-d-orage", "Plume d'orage"),
    ("belier-de-granit", "corne-de-granit", "Corne de granit"),
    (
        "feu-de-saint-elme",
        "etincelle-captive",
        "Étincelle captive",
    ),
    ("geant-des-orages", "coeur-de-tonnerre", "Cœur de tonnerre"),
    ("larve-de-chair", "chair-grouillante", "Chair grouillante"),
    ("devoreur-de-chair", "dent-du-devoreur", "Dent du dévoreur"),
    ("valet-ecorche", "peau-ecorchee", "Peau écorchée"),
    ("idole-de-chair", "oeil-de-l-idole", "Œil de l'idole"),
    ("rampant-du-puits", "eclat-du-puits", "Éclat du puits"),
    ("echo-du-vide", "cristal-d-echo", "Cristal d'écho"),
    (
        "ver-des-abysses",
        "anneau-des-abysses",
        "Anneau des abysses",
    ),
    ("gardien-du-gouffre", "fer-du-gouffre", "Fer du gouffre"),
    ("pleureuse", "voile-de-pleureuse", "Voile de pleureuse"),
    ("mendiant-possede", "sebile-felee", "Sébile fêlée"),
    ("epouvantail", "paille-maudite", "Paille maudite"),
    (
        "dame-blanche",
        "larme-de-dame-blanche",
        "Larme de dame blanche",
    ),
    ("hibou-blafard", "plume-blafarde", "Plume blafarde"),
    ("amanite-rampante", "chapeau-d-amanite", "Chapeau d'amanite"),
    ("ours-moussu", "fourrure-moussue", "Fourrure moussue"),
    ("farfadet", "poudre-de-farfadet", "Poudre de farfadet"),
    (
        "meduse-des-naufrages",
        "ombrelle-de-meduse",
        "Ombrelle de méduse",
    ),
    ("requin-noir", "dent-de-requin", "Dent de requin"),
    ("albatros-maudit", "plume-d-albatros", "Plume d'albatros"),
    ("kraken", "bec-de-kraken", "Bec de kraken"),
    ("pagure-a-crane", "coquille-de-pagure", "Coquille de pagure"),
    ("mouette-hurleuse", "plume-de-mouette", "Plume de mouette"),
    ("naufrageur", "lanterne-rouillee", "Lanterne rouillée"),
    ("sirene-d-ecume", "perle-d-ecume", "Perle d'écume"),
    (
        "sphinx-tete-de-mort",
        "ecaille-de-sphinx",
        "Écaille de sphinx",
    ),
    ("lievre-somnambule", "patte-de-lievre", "Patte de lièvre"),
    ("pavot-carnivore", "graine-de-pavot", "Graine de pavot"),
    ("marchand-de-sable", "sablier-d-os", "Sablier d'os"),
    ("belier-de-siege", "corne-ferree", "Corne ferrée"),
    ("sentinelle-de-pierre", "eclat-de-statue", "Éclat de statue"),
    ("baliste-hantee", "carreau-rouille", "Carreau rouillé"),
    ("chevalier-errant", "ecu-brise", "Écu brisé"),
    ("cerf-de-pierre", "bois-petrifie", "Bois pétrifié"),
    ("lezard-petrifie", "ecaille-de-lezard", "Écaille de lézard"),
    ("dryade-de-pierre", "ecorce-de-pierre", "Écorce de pierre"),
    ("arbre-roi-petrifie", "seve-petrifiee", "Sève pétrifiée"),
    (
        "moine-de-lumiere",
        "chapelet-de-lumiere",
        "Chapelet de lumière",
    ),
    (
        "chimere-de-cristal",
        "prisme-de-chimere",
        "Prisme de chimère",
    ),
    ("ver-luisant-geant", "abdomen-luisant", "Abdomen luisant"),
    ("gardien-d-asphodele", "ecaille-doree", "Écaille dorée"),
    ("ours-des-glaces", "fourrure-givree", "Fourrure givrée"),
    ("yeti", "toison-de-yeti", "Toison de yéti"),
    ("esprit-du-blizzard", "flocon-eternel", "Flocon éternel"),
    (
        "mammouth-fantome",
        "defense-de-mammouth",
        "Défense de mammouth",
    ),
    ("colosse-d-os", "femur-de-geant", "Fémur de géant"),
    (
        "charognard-des-geants",
        "plume-de-charognard",
        "Plume de charognard",
    ),
    ("ver-ossivore", "mandibule-ossivore", "Mandibule ossivore"),
    ("titan-dechu", "phalange-de-titan", "Phalange de titan"),
    ("rampant-du-neant", "ichor-du-neant", "Ichor du néant"),
    ("mangeur-d-etoiles", "croc-stellaire", "Croc stellaire"),
    (
        "spectre-hurlant",
        "ectoplasme-hurlant",
        "Ectoplasme hurlant",
    ),
    ("ange-du-neant", "plume-du-neant", "Plume du néant"),
];

/// Section 32 de docs/contenu-v1.md : les météos de la Lisière, du Marais
/// et de l'Abbaye (section 9.7). Le cyclone attend une zone de mer ; la
/// tempête de sable est réservée au Désert de sel.
const METEOS_COMMUNES: [Meteo; 3] = [Meteo::Pluie, Meteo::Tempete, Meteo::Canicule];

/// Section 31 de docs/contenu-v1.md : les espèces qui ne sortent, dans les
/// groupes sauvages des zones, que la nuit ou que le jour (section 9.6).
/// Chaque zone en garde au moins deux à chaque moment.
const ACTIVITES: [(&str, Activite); 28] = [
    ("loup", Activite::Nocturne),
    ("feu-follet", Activite::Nocturne),
    ("fee-noire", Activite::Nocturne),
    ("goule", Activite::Nocturne),
    ("chauve-souris-de-crypte", Activite::Nocturne),
    ("troll-des-cavernes", Activite::Nocturne),
    ("wendigo", Activite::Nocturne),
    ("faux-agneau", Activite::Nocturne),
    ("vipere", Activite::Diurne),
    ("cloche-hurlante", Activite::Diurne),
    ("harpie-des-orages", Activite::Diurne),
    ("scarabee-solaire", Activite::Diurne),
    ("momie-de-sel", Activite::Nocturne),
    ("sirene-putride", Activite::Nocturne),
    ("goeland-charognard", Activite::Diurne),
    ("mineur-possede", Activite::Nocturne),
    ("mante-ecarlate", Activite::Diurne),
    ("sanglier-enrage", Activite::Nocturne),
    ("moine-psalmodiant", Activite::Nocturne),
    ("destrier-spectral", Activite::Nocturne),
    ("vautour-des-charniers", Activite::Diurne),
    ("oeil-errant", Activite::Nocturne),
    ("chien-des-scories", Activite::Nocturne),
    ("forgeron-calcine", Activite::Diurne),
    ("aigle-tonnerre", Activite::Diurne),
    ("feu-de-saint-elme", Activite::Nocturne),
    ("devoreur-de-chair", Activite::Nocturne),
    ("rampant-du-puits", Activite::Nocturne),
];

/// Section 28 de docs/contenu-v1.md : l'attaque d'œuf de chaque famille
/// d'évolution, par sa forme de base (section 13.11).
const ATTAQUES_D_OEUF: [(&str, &str); 138] = [
    ("salamandre", "souffle-des-forges"),
    ("crapaud", "ressac"),
    ("mandragore", "seve-maudite"),
    ("chat-noir", "griffes-de-minuit"),
    ("loup", "assaut-de-meute"),
    ("corbeau", "plongee-funebre"),
    ("vipere", "etreinte-de-l-aspic"),
    ("feu-follet", "lueur-des-marais"),
    ("armure-hantee", "coup-de-heaume"),
    ("fee-noire", "essaim-maudit"),
    ("squelette", "danse-macabre"),
    ("cerf-blanc", "brame-sacre"),
    ("basilic", "souffle-du-basilic"),
    ("elfe", "sortilege-sylvestre"),
    ("vouivre", "pique-draconique"),
    ("schnoz", "pluie-d-etoiles"),
    ("gargouille", "plongeon-de-granit"),
    ("goule", "griffes-putrides"),
    ("chauve-souris-de-crypte", "vol-nocturne"),
    ("chimere", "souffle-triple"),
    ("ange-dechu", "litanie-des-dechus"),
    ("noye", "appel-des-profondeurs"),
    ("anguille-des-tourbieres", "morsure-des-tourbieres"),
    ("sangsue-geante", "saignee"),
    ("kelpie", "galop-des-eaux"),
    ("wendigo", "griffes-de-famine"),
    ("troll-des-cavernes", "coup-de-massue"),
    ("harpie-des-orages", "oeil-de-l-orage"),
    ("wyrm-de-givre", "souffle-de-l-hiver"),
    ("sylphe-des-neiges", "valse-des-neiges"),
    ("cloche-hurlante", "volee-de-cloches"),
    ("seraphin-de-vitrail", "rosace-ardente"),
    ("molosse-des-fosses", "charge-des-fosses"),
    ("taureau-d-airain", "charge-d-airain"),
    ("crotale-des-salines", "morsure-des-salines"),
    ("scarabee-solaire", "disque-solaire"),
    ("momie-de-sel", "linceul-de-sel"),
    ("ver-des-salines", "charge-des-dunes"),
    ("sphinx-des-sables", "enigme-du-sphinx"),
    ("phenix-blanc", "aile-d-aurore"),
    ("crabe-necrophage", "pince-etau"),
    ("sirene-putride", "lamento-des-abysses"),
    ("goeland-charognard", "pique-du-charognard"),
    ("hydre-des-marees", "morsure-des-sept-tetes"),
    ("nautile-des-abysses", "spirale-abyssale"),
    ("golem-de-fer-noir", "poing-de-fer-noir"),
    ("mineur-possede", "hantise-du-filon"),
    ("grisou-ardent", "etincelle-de-grisou"),
    ("dragon-de-rouille", "ecaille-rouillee"),
    ("gemmame", "prisme-feerique"),
    ("ronce-vampire", "racines-assoiffees"),
    ("mante-ecarlate", "danse-de-la-mante"),
    ("sanglier-enrage", "coup-de-boutoir"),
    ("dryade-sanglante", "chant-de-la-dryade"),
    ("phalene-d-aube", "poudre-d-aube"),
    ("rat-des-ossuaires", "nuee-de-rats"),
    ("moine-psalmodiant", "psalmodie"),
    ("araignee-des-cryptes", "toile-des-cryptes"),
    ("liche", "phylactere"),
    ("ange-de-pierre", "aube-de-pierre"),
    ("destrier-spectral", "sabots-fantomes"),
    ("vautour-des-charniers", "curee-des-sables"),
    ("matin-de-guerre", "morsure-de-fer"),
    ("griffon-cuirasse", "serres-du-griffon"),
    ("licorne-des-cendres", "flamme-de-licorne"),
    ("automate-brise", "engrenage"),
    ("bibliophage", "page-maudite"),
    ("oeil-errant", "oeillade"),
    ("archonte-foudroye", "arc-electrique"),
    ("muse-de-cristal", "harmonie"),
    ("golem-de-magma", "magma-bouillonnant"),
    ("chien-des-scories", "croc-des-ombres"),
    ("forgeron-calcine", "enclume"),
    ("wyrm-de-magma", "ecaille-ardente"),
    ("ifrit", "mirage-ardent"),
    ("aigle-tonnerre", "cri-du-tonnerre"),
    ("belier-de-granit", "charge-du-belier"),
    ("feu-de-saint-elme", "lueur-d-orage"),
    ("geant-des-orages", "poing-du-geant"),
    ("pegase-d-orage", "galop-celeste"),
    ("larve-de-chair", "ponte-grouillante"),
    ("devoreur-de-chair", "gueule-beante"),
    ("valet-ecorche", "lambeaux-de-peau"),
    ("idole-de-chair", "murmure-charnel"),
    ("lys-de-sang", "parfum-de-sang"),
    ("rampant-du-puits", "reptation"),
    ("echo-du-vide", "repercussion"),
    ("ver-des-abysses", "fouissement"),
    ("gardien-du-gouffre", "garde-du-gouffre"),
    ("germe-du-monde", "eclosion"),
    ("leviathan", "ecaille-abyssale"),
    ("behemoth", "corne-primordiale"),
    ("ziz", "plume-celeste"),
    ("colosse-du-rempart", "mur-de-boucliers"),
    ("pleureuse", "voix-d-outre-tombe"),
    ("mendiant-possede", "aumone-maudite"),
    ("epouvantail", "paille-ensorcelee"),
    ("dame-blanche", "linceul-blanc"),
    ("hibou-blafard", "hululement-lugubre"),
    ("amanite-rampante", "spores-du-sommeil"),
    ("ours-moussu", "accolade-de-mousse"),
    ("farfadet", "eclat-de-farfadet"),
    ("meduse-des-naufrages", "filaments-de-meduse"),
    ("requin-noir", "machoire-abyssale"),
    ("albatros-maudit", "vol-du-mauvais-presage"),
    ("kraken", "tentacules-du-kraken"),
    ("pagure-a-crane", "coquille-d-os"),
    ("mouette-hurleuse", "cri-des-marees"),
    ("naufrageur", "fanal-trompeur"),
    ("sirene-d-ecume", "chant-d-ecume"),
    ("sphinx-tete-de-mort", "aile-du-crane"),
    ("lievre-somnambule", "bond-onirique"),
    ("pavot-carnivore", "pollen-du-sommeil"),
    ("marchand-de-sable", "sable-des-songes"),
    ("belier-de-siege", "tete-de-belier"),
    ("sentinelle-de-pierre", "garde-immobile"),
    ("baliste-hantee", "carreau-funebre"),
    ("chevalier-errant", "serment-du-chevalier"),
    ("cerf-de-pierre", "ramure-de-granit"),
    ("lezard-petrifie", "morsure-calcaire"),
    ("dryade-de-pierre", "seve-de-calcaire"),
    ("arbre-roi-petrifie", "racines-de-granit"),
    ("moine-de-lumiere", "mantra-solaire"),
    ("chimere-de-cristal", "griffes-de-cristal"),
    ("ver-luisant-geant", "lueur-du-ver"),
    ("gardien-d-asphodele", "souffle-d-asphodele"),
    ("ours-des-glaces", "etreinte-glaciale"),
    ("yeti", "poing-de-givre"),
    ("esprit-du-blizzard", "voile-de-neige"),
    ("mammouth-fantome", "pietinement-glacial"),
    ("colosse-d-os", "ossature-de-geant"),
    ("charognard-des-geants", "bec-putride"),
    ("ver-ossivore", "ronge-os"),
    ("titan-dechu", "chute-du-titan"),
    ("rampant-du-neant", "murmure-du-neant"),
    ("mangeur-d-etoiles", "gueule-d-eclipse"),
    ("spectre-hurlant", "hurlement-du-vide"),
    ("ange-du-neant", "lumiere-noire"),
];

/// La forme de base de la famille de cette espèce : celle qu'aucune autre
/// ne précède (section 5.4).
fn base_de_la_famille(id: &str) -> &str {
    let mut forme = id;
    while let Some(precedente) = ESPECES.iter().find(|fiche| {
        fiche
            .evolution
            .as_ref()
            .is_some_and(|evolution| evolution.vers == forme)
    }) {
        forme = precedente.id;
    }
    forme
}

fn creer_attaque(id: &str) -> Attaque {
    let fiche = ATTAQUES
        .iter()
        .chain(&ATTAQUES_DES_HEROS)
        .find(|fiche| fiche.id == id)
        .unwrap_or_else(|| panic!("attaque {id} absente du contenu"));
    let attaque = Attaque::new(
        AttaqueId::new(fiche.id),
        fiche.nom,
        fiche.type_attaque,
        fiche.categorie,
        fiche.puissance,
        fiche.precision,
    )
    .expect("attaque du contenu valide");
    match fiche.effet {
        Some((statut, chance)) => attaque
            .avec_effet(EffetSecondaire::new(statut, chance).expect("effet du contenu valide")),
        None => attaque,
    }
}

fn niveau(valeur: u8) -> Niveau {
    Niveau::new(valeur).expect("niveau du contenu valide")
}

/// Expéditions automatiques de la section 3 de docs/contenu-v1.md.
fn expedition_automatique(
    heures: u64,
    degats: RangeInclusive<u32>,
    experience: RangeInclusive<u32>,
    potions: RangeInclusive<u32>,
    argent: RangeInclusive<u32>,
) -> ParametresExpeditionAutomatique {
    ParametresExpeditionAutomatique::new(
        Duration::from_secs(heures * 3_600),
        degats,
        experience,
        potions,
        argent,
    )
    .expect("expédition automatique conforme aux règles")
}

/// Coffres de la section 3 de docs/contenu-v1.md.
fn coffre(argent: RangeInclusive<u32>, capsules: Vec<Capsule>) -> ParametresCoffre {
    ParametresCoffre::new(argent, capsules).expect("coffre conforme aux règles")
}

/// Section 44 de docs/contenu-v1.md : le trésor de chaque coffre d'une
/// annexe, avec son poids sur 100 (section 9.9). Depuis l'étape 211, le
/// Philtre chromatique y est plus rare, et des objets d'expérience y
/// entrent (section 14.3, valeurs proposées par Claude, Q125).
fn tresors_d_annexe() -> Vec<(Tresor, u32)> {
    let mut tresors = vec![
        (Tresor::PhiltreChromatique, 3),
        (Tresor::CapsuleDHabilete, 30),
        (Tresor::Pierre(PierreDEvolution::Braise), 13),
        (Tresor::Pierre(PierreDEvolution::Lune), 12),
        (Tresor::ObjetDExperience(ObjetDExperience::Cristal), 10),
        (Tresor::ObjetDExperience(ObjetDExperience::Coeur), 2),
    ];
    tresors.extend(Booster::TOUS.map(|booster| (Tresor::Booster(booster), 5)));
    tresors
}

/// Section 50 de docs/contenu-v1.md : les Apôtres, dans l'ordre de la
/// traque, avec l'identifiant du succès, le titre, l'Apôtre et son trophée.
const FLEAUX: [(&str, &str, &str, &str, &str); 13] = [
    (
        "fleau-du-baron",
        "Fléau du Baron",
        "baron-pendu",
        "abattre le Baron pendu",
        "La corde du Baron pendu, nouée au poteau du camp.",
    ),
    (
        "fleau-de-l-abbesse",
        "Fléau de l'Abbesse",
        "abbesse-ecorchee",
        "abattre l'Abbesse écorchée",
        "Le voile de l'Abbesse écorchée, tendu sur un cadre de bois.",
    ),
    (
        "fleau-du-margrave",
        "Fléau du Margrave",
        "margrave-de-givre",
        "abattre le Margrave de givre",
        "Un éclat de la couronne du Margrave de givre, qui ne fond jamais.",
    ),
    (
        "fleau-de-l-amiral",
        "Fléau de l'Amiral",
        "amiral-noye",
        "abattre l'Amiral noyé",
        "Le tricorne de l'Amiral noyé, toujours ruisselant, accroché à une lance.",
    ),
    (
        "fleau-du-prevot",
        "Fléau du Prévôt",
        "prevot-des-fosses",
        "abattre le Prévôt des fosses",
        "Le fouet du Prévôt des fosses, enroulé sur un clou.",
    ),
    (
        "fleau-de-la-duchesse",
        "Fléau de la Duchesse",
        "duchesse-sanguine",
        "abattre la Duchesse sanguine",
        "La coupe de la Duchesse sanguine, vide, posée sur une souche.",
    ),
    (
        "fleau-de-l-eveque",
        "Fléau de l'Évêque",
        "eveque-des-os",
        "abattre l'Évêque des os",
        "La crosse de l'Évêque des os, plantée près de l'entrée.",
    ),
    (
        "fleau-du-connetable",
        "Fléau du Connétable",
        "connetable-boucher",
        "abattre le Connétable boucher",
        "Le couperet du Connétable boucher, fiché dans un billot.",
    ),
    (
        "fleau-du-chancelier",
        "Fléau du Chancelier",
        "chancelier-aux-mille-yeux",
        "abattre le Chancelier aux mille yeux",
        "Le sceau du Chancelier aux mille yeux, dont aucun œil ne s'ouvre plus.",
    ),
    (
        "fleau-du-prince",
        "Fléau du Prince",
        "prince-de-braise",
        "abattre le Prince de braise",
        "Une braise du Prince, qui couve dans une lanterne de fer.",
    ),
    (
        "fleau-du-senechal",
        "Fléau du Sénéchal",
        "senechal-des-orages",
        "abattre le Sénéchal des orages",
        "Le cor du Sénéchal des orages, muet, pendu à une branche.",
    ),
    (
        "fleau-de-la-reine-mere",
        "Fléau de la Reine-mère",
        "reine-mere-de-chair",
        "abattre la Reine-mère de chair",
        "Une mèche de la Reine-mère de chair, tressée et séchée.",
    ),
    (
        "fleau-du-roi-sans-visage",
        "Fléau du Roi sans visage",
        "roi-sans-visage",
        "abattre le Roi sans visage",
        "Le masque vide du Roi sans visage, posé face contre terre.",
    ),
];

/// Section 50 : les légendaires capturés.
const LEGENDES: [(&str, &str, &str, &str, &str); 3] = [
    (
        "legende-des-abysses",
        "Légende de la Mer sans rivage",
        "leviathan",
        "capturer Léviathan",
        "Une écaille de Léviathan, grande comme un bouclier.",
    ),
    (
        "legende-des-terres-arides",
        "Légende des Terres arides",
        "behemoth",
        "capturer Béhémoth",
        "Une corne de Béhémoth, dressée comme une pierre levée.",
    ),
    (
        "legende-du-ciel-dechire",
        "Légende du Ciel déchiré",
        "ziz",
        "capturer Ziz",
        "Une plume de Ziz, plus haute qu'une tente.",
    ),
];

/// Section 50 de docs/contenu-v1.md, dans l'ordre de l'écran des succès.
/// Les seuils qui dépendent du contenu suivent ses annexes et la Fosse.
fn creer_succes(annexes: usize, paliers_de_la_fosse: u8) -> Vec<Succes> {
    let mut succes = vec![
        Succes::new(
            "nomade",
            "Nomade",
            "mener une expédition au bout",
            ConditionDeSucces::ExpeditionMenee,
        ),
        Succes::new(
            "naturaliste",
            "Naturaliste",
            "capturer 30 espèces du codex",
            ConditionDeSucces::EspecesCapturees(30),
        ),
        Succes::new(
            "encyclopediste",
            "Encyclopédiste",
            "compléter le codex",
            ConditionDeSucces::CodexComplet,
        )
        .avec_trophee(
            "Le bestiaire complet, relié de cuir noir, ouvert sur un lutrin près du feu.",
        ),
        Succes::new(
            "oeil-de-lynx",
            "Œil-de-lynx",
            "avoir un monstre shiny dans l'équipe ou au camp",
            ConditionDeSucces::MonstresShiny(1),
        ),
        Succes::new(
            "porte-lueur",
            "Porte-lueur",
            "avoir 5 monstres shiny à la fois dans l'équipe et au camp",
            ConditionDeSucces::MonstresShiny(5),
        )
        .avec_couleur(CouleurDuHeros::Nacre),
        Succes::new(
            "lueur-incarnee",
            "Lueur incarnée",
            "avoir un héros shiny",
            ConditionDeSucces::HerosShiny,
        ),
        Succes::new(
            "veteran",
            "Vétéran",
            "mener un monstre au niveau 50",
            ConditionDeSucces::Niveau(50),
        ),
        Succes::new(
            "legende-vivante",
            "Légende vivante",
            "mener un monstre au niveau 100",
            ConditionDeSucces::Niveau(100),
        ),
        Succes::new(
            "brise-chaines",
            "Brise-chaînes",
            "vaincre le Bourreau",
            ConditionDeSucces::HumainVaincu(HumainId::new("bourreau")),
        ),
    ];
    succes.extend(
        FLEAUX
            .iter()
            .map(|(id, titre, apotre, description, trophee)| {
                Succes::new(
                    id,
                    titre,
                    description,
                    ConditionDeSucces::ApotreAbattu(EspeceId::new(*apotre)),
                )
                .avec_trophee(trophee)
            }),
    );
    succes.push(
        Succes::new(
            "deicide",
            "Déicide",
            "abattre le Démiurge",
            ConditionDeSucces::DemiurgeAbattu,
        )
        .avec_trophee("Un fragment de l'Éclipse, pierre noire qui boit la lumière.")
        .avec_couleur(CouleurDuHeros::Cendre),
    );
    succes.extend(
        LEGENDES
            .iter()
            .map(|(id, titre, legendaire, description, trophee)| {
                Succes::new(
                    id,
                    titre,
                    description,
                    ConditionDeSucces::LegendaireCapture(EspeceId::new(*legendaire)),
                )
                .avec_trophee(trophee)
            }),
    );
    succes.extend([
        Succes::new(
            "idole-de-la-fosse",
            "Idole de la Fosse",
            "vaincre le dernier palier de la Fosse aux damnés",
            ConditionDeSucces::PaliersDeLaFosse(paliers_de_la_fosse),
        )
        .avec_trophee("La chaîne du Champion de la Fosse, jetée au pied du feu.")
        .avec_couleur(CouleurDuHeros::Sang),
        Succes::new(
            "ombre-de-la-tour",
            "Ombre de la Tour",
            "franchir 50 étapes de la Tour sans fond, seul",
            ConditionDeSucces::EtapesDeLaTour(50),
        )
        .avec_couleur(CouleurDuHeros::Abime),
        Succes::new(
            "duelliste",
            "Duelliste",
            "gagner 10 duels",
            ConditionDeSucces::DuelsGagnes(10),
        ),
        Succes::new(
            "main-d-or",
            "Main d'or",
            "avoir 10 000 pièces d'argent, sac et coffre réunis",
            ConditionDeSucces::Argent(10_000),
        ),
        Succes::new(
            "architecte",
            "Architecte",
            "mener toutes les améliorations du camp à leur niveau maximal",
            ConditionDeSucces::AmeliorationsAuMaximum,
        ),
        Succes::new(
            "fer-de-lance",
            "Fer de lance",
            "porter une arme et une armure du palier 8",
            ConditionDeSucces::PalierDEquipement(8),
        ),
        Succes::new(
            "cartographe",
            "Cartographe",
            &format!("ouvrir les {annexes} zones annexes"),
            ConditionDeSucces::AnnexesOuvertes(annexes),
        ),
        Succes::new(
            "complice-de-malvoise",
            "Complice de Malvoise",
            "accomplir les 4 quêtes de Malvoise",
            ConditionDeSucces::QuetesAccomplies(Personnage::Malvoise),
        )
        .avec_trophee("Le psautier de Frère Malvoise, ouvert à la page d'Isaure."),
        Succes::new(
            "eleve-de-gunther",
            "Élève de Gunther",
            "accomplir les 4 quêtes de Gunther",
            ConditionDeSucces::QuetesAccomplies(Personnage::Gunther),
        )
        .avec_trophee("La lame noire de Konrad, plantée au bord du camp."),
        Succes::new(
            "camarade-d-ansgar",
            "Camarade d'Ansgar",
            "accomplir les 4 quêtes d'Ansgar",
            ConditionDeSucces::QuetesAccomplies(Personnage::Ansgar),
        )
        .avec_trophee("Le filet de l'équipage d'Edwin, tendu entre deux pieux près de l'étang."),
    ]);
    succes
}

/// Une quête de la section 52 de docs/contenu-v1.md, avant que ses
/// références au contenu soient résolues.
struct FicheQuete {
    id: &'static str,
    personnage: Personnage,
    titre: &'static str,
    /// La zone qui l'ouvre ; une annexe si `ouverte_par_annexe`.
    ouverture: &'static str,
    ouverte_par_annexe: bool,
    demande: FicheDemande,
    argent: u32,
    objets: fn(Inventaire) -> Inventaire,
    texte_de_la_demande: &'static str,
    texte_de_la_remise: &'static str,
}

enum FicheDemande {
    /// L'espèce dont la ressource est demandée, et leur nombre.
    Rapporter(&'static str, u32),
    Explorer {
        zone: &'static str,
        etape: u32,
        objet: &'static str,
        gardien: &'static str,
        niveau: u8,
        rencontre: &'static str,
        prise: &'static str,
    },
}

const QUETES: [FicheQuete; 12] = [
    FicheQuete {
        id: "lueur-du-crane",
        personnage: Personnage::Malvoise,
        titre: "La lueur du crâne",
        ouverture: "marais",
        ouverte_par_annexe: false,
        demande: FicheDemande::Rapporter("feu-follet", 15),
        argent: 150,
        objets: |objets| objets.avec_soins(Soin::Potion, 3),
        texte_de_la_demande: "Malvoise fait tourner son crâne entre ses doigts. \"Il s'éteint. Les feux follets du Marais brûlent de ce qui reste des noyés : rapporte-m'en quinze poignées de cendre, et il verra de nouveau clair.\"",
        texte_de_la_remise: "Malvoise souffle la cendre dans les orbites du crâne, qui s'allument d'une lueur verte. \"Il voit. C'était le crâne de mon maître, à l'Abbaye des Cendres. Il m'a appris à lire les morts ; on l'a brûlé pour cela.\"",
    },
    FicheQuete {
        id: "psautier-de-malvoise",
        personnage: Personnage::Malvoise,
        titre: "Le psautier de Frère Malvoise",
        ouverture: "abbaye",
        ouverte_par_annexe: false,
        demande: FicheDemande::Explorer {
            zone: "abbaye",
            etape: 3,
            objet: "Psautier de Frère Malvoise",
            gardien: "goule",
            niveau: 19,
            rencontre: "Sous une dalle descellée du chœur, un psautier relié de cuir noir. Une goule, accroupie dessus, relève la tête et gronde.",
            prise: "Sur la garde du psautier, une main appliquée a écrit \"Frère Malvoise\". Vous le glissez dans votre sac.",
        },
        argent: 400,
        objets: |objets| objets.avec_objets_d_experience(ObjetDExperience::Fiole, 2),
        texte_de_la_demande: "\"L'Abbaye des Cendres est ouverte, dis-tu ? J'y ai été moine. Frère Malvoise, archiviste. J'ai caché sous les dalles du chœur un psautier noté de ma main. Les goules rôdent là où l'on enterre mal : rapporte-le-moi.\"",
        texte_de_la_remise: "Malvoise tourne les pages sans un mot, puis s'arrête sur un nom : Isaure. \"Ma sœur. Elle chantait au chœur quand l'Abbesse a commencé à écorcher les novices. Je n'ai rien fait. J'ai fui avec mes livres.\"",
    },
    FicheQuete {
        id: "prieres-pour-isaure",
        personnage: Personnage::Malvoise,
        titre: "Des prières pour Isaure",
        ouverture: "catacombes-de-sainte-ombre",
        ouverte_par_annexe: false,
        demande: FicheDemande::Rapporter("moine-psalmodiant", 18),
        argent: 1_000,
        objets: |objets| objets.avec_capsules_d_habilete(3),
        texte_de_la_demande: "\"Les moines des Catacombes psalmodient encore, morts depuis des siècles. Leurs chapelets d'os portent des prières que plus personne ne dit. Rapporte-m'en dix-huit : il m'en faut pour retenir une âme qui ne trouve pas le repos.\"",
        texte_de_la_remise: "Malvoise noue les chapelets autour du crâne, un à un. \"Isaure ne repose pas. Je l'entends, la nuit, qui chante sous le camp. Ces prières la retiendront, le temps que je trouve le reste.\"",
    },
    FicheQuete {
        id: "page-du-codex-mortis",
        personnage: Personnage::Malvoise,
        titre: "La page du Codex Mortis",
        ouverture: "haute-cite-dechue",
        ouverte_par_annexe: false,
        demande: FicheDemande::Explorer {
            zone: "haute-cite-dechue",
            etape: 3,
            objet: "Page du Codex Mortis",
            gardien: "oeil-errant",
            niveau: 66,
            rencontre: "Dans une salle de lecture éventrée, une page arrachée flotte au-dessus d'un pupitre, couverte d'une écriture qui bouge. Un œil errant se tourne vers vous.",
            prise: "L'écriture cesse de bouger dès que vous touchez la page. Vous la pliez dans votre sac.",
        },
        argent: 3_000,
        objets: |objets| objets.avec_philtres_chromatiques(1),
        texte_de_la_demande: "\"La Haute-Cité avait la plus grande bibliothèque du monde. Le Codex Mortis y était enchaîné ; une de ses pages dit comment rendre le repos à une âme liée. Trouve-la, et j'achèverai ce que j'aurais dû faire il y a trente ans.\"",
        texte_de_la_remise: "Malvoise lit la page à voix basse, puis la brûle au-dessus du crâne. Sous le camp, le chant s'arrête. \"Elle dort, enfin.\" Il reste longtemps silencieux. \"Je ne dois plus rien aux morts. Mes rites iront plus vite, désormais : mes mains ne tremblent plus.\"",
    },
    FicheQuete {
        id: "tenailles-neuves",
        personnage: Personnage::Gunther,
        titre: "Des tenailles neuves",
        ouverture: "vallee",
        ouverte_par_annexe: false,
        demande: FicheDemande::Rapporter("armure-hantee", 18),
        argent: 200,
        objets: |objets| {
            objets
                .avec_boosters(Booster::MoelleDeBete, 1)
                .avec_boosters(Booster::EcorceDeFer, 1)
        },
        texte_de_la_demande: "Gunther essuie ses mains sur son tablier. \"Le Bourreau m'a pris mes outils et ne m'a laissé que ses chaînes. Les armures hantées font un bon acier : rapporte-m'en dix-huit fragments, que je me refasse des tenailles dignes de ce nom.\"",
        texte_de_la_remise: "Gunther fond les fragments et martèle jusqu'à la nuit. Au matin, il vous montre des tenailles neuves. \"Mon fils aurait fait mieux. Konrad. Il avait la main plus fine que la mienne. Le Bourreau l'a gardé, lui aussi. Il n'est pas sorti.\"",
    },
    FicheQuete {
        id: "marteau-de-hautefer",
        personnage: Personnage::Gunther,
        titre: "Le marteau de Hautefer",
        ouverture: "col-du-givre-noir",
        ouverte_par_annexe: false,
        demande: FicheDemande::Explorer {
            zone: "col-du-givre-noir",
            etape: 3,
            objet: "Marteau de Hautefer",
            gardien: "troll-des-cavernes",
            niveau: 25,
            rencontre: "Dans les ruines d'une forge, sous la neige, un marteau repose sur une enclume fendue. Un troll des cavernes en a fait son oreiller ; il se lève, furieux.",
            prise: "Le manche du marteau porte une marque : trois traits et une flamme. Vous le rangez dans votre sac.",
        },
        argent: 600,
        objets: |objets| objets.avec_objets_d_experience(ObjetDExperience::Fiole, 2),
        texte_de_la_demande: "\"Mon village était au pied du Col : Hautefer. Les trolls l'ont pris, l'hiver où les Apôtres sont venus. Le marteau de mon père est resté sur l'enclume. S'il y est encore, rapporte-le-moi.\"",
        texte_de_la_remise: "Gunther soupèse le marteau et rit pour la première fois. \"La marque de Hautefer. Mon père, et son père avant lui.\" Il le pend au-dessus de l'enclume. \"Il ne servira qu'aux beaux ouvrages.\"",
    },
    FicheQuete {
        id: "lame-pour-konrad",
        personnage: Personnage::Gunther,
        titre: "Une lame pour Konrad",
        ouverture: "mines-de-fer-noir",
        ouverte_par_annexe: false,
        demande: FicheDemande::Rapporter("golem-de-fer-noir", 24),
        argent: 1_200,
        objets: |objets| objets.avec_capsules_d_habilete(3),
        texte_de_la_demande: "\"On dit que le fer des Mines ne rouille jamais, parce que les morts le gardent. Il m'en faut vingt-quatre éclats : je veux forger une lame pour la tombe de Konrad. Il n'en a pas, de tombe. Il aura une lame.\"",
        texte_de_la_remise: "Gunther forge trois jours. La lame est simple, sans ornement, noire comme les nuits du Col. Il la plante en terre au bord du camp. \"Voilà. Ici repose Konrad, apprenti de Hautefer.\"",
    },
    FicheQuete {
        id: "coeur-de-braise",
        personnage: Personnage::Gunther,
        titre: "Le cœur de braise",
        ouverture: "gorge-de-braise",
        ouverte_par_annexe: false,
        demande: FicheDemande::Explorer {
            zone: "gorge-de-braise",
            etape: 3,
            objet: "Cœur de braise",
            gardien: "golem-de-magma",
            niveau: 74,
            rencontre: "Au bord d'une coulée, une pierre rouge palpite comme un cœur. Un golem de magma la porte dans sa poitrine ouverte.",
            prise: "Arraché au golem, le cœur de braise continue de battre, tiède, presque doux. Vous l'enveloppez au fond de votre sac.",
        },
        argent: 3_000,
        objets: |objets| objets.avec_philtres_chromatiques(1),
        texte_de_la_demande: "\"Au fond de la Gorge bat un cœur de braise qui ne refroidit jamais. Une forge qui en a un ne s'éteint plus. Avec lui, je travaillerai plus vite que je ne l'ai jamais fait.\"",
        texte_de_la_remise: "Gunther pose le cœur de braise au fond du foyer. La forge s'embrase d'une flamme blanche. \"Konrad aurait aimé voir ça.\" Il reprend le marteau de Hautefer. \"Désormais, mes ouvrages iront plus vite.\"",
    },
    FicheQuete {
        id: "leurres-de-plumes",
        personnage: Personnage::Ansgar,
        titre: "Des leurres de plumes",
        ouverture: "marais",
        ouverte_par_annexe: false,
        demande: FicheDemande::Rapporter("corbeau", 15),
        argent: 150,
        objets: |objets| objets.avec_filets(Filet::Chanvre, 5),
        texte_de_la_demande: "Ansgar ravaude un filet, les doigts gourds. \"Mes leurres sont restés dans la cabane. Avec quinze plumes de corbeau, je m'en ferai de nouveaux, et le poisson de l'étang mordra mieux.\"",
        texte_de_la_remise: "Ansgar noue les plumes avec une patience infinie. \"Mon fils, Edwin, les faisait mieux que moi. Il est parti en mer, au Port. Les marins disaient que la mer y rend les noyés. Elle ne me l'a jamais rendu.\"",
    },
    FicheQuete {
        id: "medaillon-d-edwin",
        personnage: Personnage::Ansgar,
        titre: "Le médaillon d'Edwin",
        ouverture: "port-des-noyes",
        ouverte_par_annexe: false,
        demande: FicheDemande::Explorer {
            zone: "port-des-noyes",
            etape: 3,
            objet: "Médaillon d'Edwin",
            gardien: "sirene-putride",
            niveau: 31,
            rencontre: "Sur un ponton pourri, une sirène putride chante, un médaillon d'étain pendu à son cou.",
            prise: "Le médaillon est terni, mais le visage gravé ressemble à Ansgar, en plus jeune. Vous le glissez dans votre sac.",
        },
        argent: 800,
        objets: |objets| objets.avec_objets_d_experience(ObjetDExperience::Fiole, 2),
        texte_de_la_demande: "\"Le Port des Noyés. Edwin y a embarqué il y a vingt ans. Il portait un médaillon d'étain où j'avais gravé mon visage, tant bien que mal. Si la mer rend les noyés, elle rend peut-être aussi ce qu'ils portaient.\"",
        texte_de_la_remise: "Ansgar serre le médaillon dans son poing, longtemps. \"Elle l'avait, la sirène ? Alors il n'est pas resté seul, au fond.\" Il le porte désormais à son cou.",
    },
    FicheQuete {
        id: "barque-pour-edwin",
        personnage: Personnage::Ansgar,
        titre: "Une barque pour Edwin",
        ouverture: "mines-de-fer-noir",
        ouverte_par_annexe: false,
        demande: FicheDemande::Rapporter("crabe-necrophage", 24),
        argent: 1_200,
        objets: |objets| objets.avec_capsules_d_habilete(3),
        texte_de_la_demande: "\"Les gens du Port brûlaient une barque pour chaque noyé, pour qu'il trouve son chemin. Edwin n'en a pas eu. Avec vingt-quatre carapaces de crabe pour la coque, je lui en ferai une.\"",
        texte_de_la_remise: "Ansgar assemble une petite barque de carapaces et y couche une plume de corbeau. Au soir, il la pousse sur l'étang du camp et y met le feu. \"Bon vent, mon garçon.\"",
    },
    FicheQuete {
        id: "filet-de-l-equipage",
        personnage: Personnage::Ansgar,
        titre: "Le filet de l'équipage",
        ouverture: "epave-du-leviathan",
        ouverte_par_annexe: true,
        demande: FicheDemande::Explorer {
            zone: "epave-du-leviathan",
            etape: 2,
            objet: "Filet de l'équipage",
            gardien: "crabe-necrophage",
            niveau: 33,
            rencontre: "Au fond de la cale noyée, un filet aux mailles luisantes est roulé contre la coque. Un crabe nécrophage s'y est fait un nid ; il claque des pinces.",
            prise: "Pas une maille du filet n'a cédé. Vous le roulez au fond de votre sac.",
        },
        argent: 3_000,
        objets: |objets| objets.avec_philtres_chromatiques(1),
        texte_de_la_demande: "\"L'Épave du Léviathan, c'était son navire, j'en suis sûr à présent. Dans la cale, il y avait le filet de l'équipage, tressé de crin de kelpie : un filet qui ne casse jamais. Rapporte-le-moi.\"",
        texte_de_la_remise: "Ansgar déplie le filet de l'équipage d'Edwin. Ses mains ne tremblent pas. \"Avec ça, je ne perdrai plus une prise. Mes lignes iront plus vite au fond.\"",
    },
];

/// La dernière quête de chaque personnage améliore son service (section
/// 13.13).
fn creer_quetes(especes: &[Espece], zones: &[Zone], annexes: &[Annexe]) -> Vec<Quete> {
    let trouver = |id: &str| {
        especes
            .iter()
            .find(|espece| espece.id().as_str() == id)
            .unwrap_or_else(|| panic!("espèce {id} absente du contenu"))
    };
    let nom_de_zone = |id: &str| {
        zones
            .iter()
            .chain(annexes.iter().map(Annexe::zone))
            .find(|zone| zone.id().as_str() == id)
            .map(|zone| zone.nom().to_string())
            .unwrap_or_else(|| panic!("zone {id} absente du contenu"))
    };
    let mut quetes: Vec<Quete> = QUETES
        .iter()
        .map(|fiche| {
            let id = QueteId::new(fiche.id);
            let demande = match &fiche.demande {
                FicheDemande::Rapporter(espece, nombre) => DemandeDeQuete::Rapporter {
                    ressource: trouver(espece)
                        .ressource()
                        .cloned()
                        .unwrap_or_else(|| panic!("{espece} lâche une ressource")),
                    nombre: *nombre,
                },
                FicheDemande::Explorer {
                    zone,
                    etape,
                    objet,
                    gardien,
                    niveau: valeur,
                    rencontre,
                    prise,
                } => DemandeDeQuete::Explorer(Box::new(Exploration::new(
                    ZoneId::new(*zone),
                    &nom_de_zone(zone),
                    *etape,
                    ObjetDeQuete::new(id.clone(), objet),
                    trouver(gardien).clone(),
                    niveau(*valeur),
                    rencontre,
                    prise,
                ))),
            };
            let ouverture = if fiche.ouverte_par_annexe {
                OuvertureDeQuete::Annexe(ZoneId::new(fiche.ouverture))
            } else {
                OuvertureDeQuete::Zone(ZoneId::new(fiche.ouverture))
            };
            let recompense = (fiche.objets)(Inventaire::new(fiche.argent, 0, Vec::new()));
            Quete::new(
                id,
                fiche.personnage,
                fiche.titre,
                ouverture,
                demande,
                recompense,
                fiche.texte_de_la_demande,
                fiche.texte_de_la_remise,
            )
        })
        .collect();
    for personnage in Personnage::TOUS {
        if let Some(derniere) = quetes
            .iter_mut()
            .rev()
            .find(|quete| quete.personnage() == personnage)
        {
            *derniere = derniere.clone().qui_ameliore_le_service();
        }
    }
    quetes
}

#[cfg(test)]
mod tests {
    /// Les tests désignent les zones par leur identifiant : l'ajout de
    /// zones entre deux Apôtres décale leur position (section 9.1).
    fn zone_par_id<'a>(contenu: &'a ContenuV1, id: &str) -> &'a Zone {
        contenu
            .zones()
            .iter()
            .find(|zone| zone.id().as_str() == id)
            .unwrap_or_else(|| panic!("zone {id} absente"))
    }

    use super::*;
    use crate::monstre::Monstre;
    use crate::stats::ValeursInternes;

    /// Section 9.9 : chaque monstre de départ se montre dans deux annexes
    /// après la fin.
    #[test]
    fn chaque_monstre_de_depart_revient_dans_deux_annexes() {
        let contenu = ContenuV1::new();
        for depart in contenu.especes_de_depart() {
            let annexes = contenu
                .annexes()
                .iter()
                .filter(|annexe| {
                    annexe
                        .zone()
                        .rares_d_apres_la_fin()
                        .iter()
                        .any(|rare| rare.espece.id() == depart.id() && rare.pacifique)
                })
                .count();
            assert_eq!(annexes, 2, "{}", depart.id().as_str());
        }
        let toutes: usize = contenu
            .annexes()
            .iter()
            .map(|annexe| annexe.zone().rares_d_apres_la_fin().len())
            .sum();
        assert_eq!(toutes, DEPARTS_D_APRES_LA_FIN.len());
    }

    #[test]
    fn chaque_ressource_a_un_rang_et_un_seul() {
        let contenu = ContenuV1::new();
        let ressources = contenu.ressources();
        let attendues = contenu
            .especes()
            .iter()
            .filter_map(Espece::ressource)
            .chain(
                contenu
                    .peche()
                    .matieres()
                    .iter()
                    .map(|(matiere, _)| matiere),
            );
        for ressource in attendues {
            assert_eq!(
                ressources
                    .iter()
                    .filter(|presente| presente.id() == ressource.id())
                    .count(),
                1,
                "{}",
                ressource.id().as_str()
            );
        }
    }

    fn creer(contenu: &ContenuV1, id: &str, valeur: u8) -> Monstre {
        let espece = contenu.espece(&EspeceId::new(id)).unwrap();
        Monstre::creer(espece, niveau(valeur), ValeursInternes::default(), false).unwrap()
    }

    /// Deux espèces d'une même zone, à son niveau maximal, peuvent toujours
    /// se blesser l'une l'autre : sinon, quand le joueur a capturé l'une, le
    /// combat contre l'autre ne finit jamais (étape 166).
    #[test]
    fn deux_especes_d_une_meme_zone_peuvent_se_blesser() {
        use crate::types::{Efficacite, efficacite};

        let contenu = ContenuV1::new();
        let blesse = |attaquant: &Monstre, defenseur: &Monstre| {
            attaquant.attaques().liste().iter().any(|attaque| {
                attaque.puissance() > 0
                    && efficacite(attaque.type_attaque(), defenseur.espece().types())
                        != Efficacite::Nulle
            })
        };
        let annexes = contenu.annexes().iter().map(Annexe::zone);
        let legendaires = contenu.zones_legendaires().iter().map(ZoneLegendaire::zone);
        for zone in contenu.zones().iter().chain(annexes).chain(legendaires) {
            let (_, maximum) = zone.niveaux();
            let monstres: Vec<Monstre> = zone
                .especes()
                .iter()
                .chain(zone.rares().iter().map(|rare| &rare.espece))
                .map(|espece| creer(&contenu, espece.id().as_str(), maximum.valeur()))
                .collect();
            for un in &monstres {
                for autre in &monstres {
                    assert!(
                        blesse(un, autre) || blesse(autre, un),
                        "{} et {} dans {}",
                        un.espece().nom(),
                        autre.espece().nom(),
                        zone.nom()
                    );
                }
            }
        }
    }

    #[test]
    fn chaque_espece_sauvage_combattue_a_sa_ressource() {
        let contenu = ContenuV1::new();
        for (espece, id, _) in RESSOURCES {
            let ressource = contenu.espece(&EspeceId::new(espece)).unwrap();
            let ressource = ressource.ressource().expect("ressource attendue");
            assert_eq!(ressource.id().as_str(), id);
            assert_eq!(contenu.ressource(ressource.id()).as_ref(), Some(ressource));
        }
        let prix = |espece: &str| {
            let espece = contenu.espece(&EspeceId::new(espece)).unwrap();
            espece.ressource().map(Ressource::prix)
        };
        assert_eq!(prix("loup"), Some(5));
        assert_eq!(prix("vouivre"), Some(15));
        // Ni monstre de départ, ni espèce pacifique, ni Apôtre.
        for sans in ["salamandre", "cerf-blanc", "schnoz", "baron-pendu"] {
            assert_eq!(prix(sans), None, "{sans}");
        }
    }

    /// Monstre aux valeurs internes de 15, la moyenne des tirages.
    fn creer_moyen(contenu: &ContenuV1, id: &str, valeur: u8) -> Monstre {
        let espece = contenu.espece(&EspeceId::new(id)).unwrap();
        let moyennes = ValeursInternes::new(Stats {
            pv: 15,
            attaque: 15,
            defense: 15,
            attaque_speciale: 15,
            defense_speciale: 15,
            vitesse: 15,
        })
        .unwrap();
        Monstre::creer(espece, niveau(valeur), moyennes, false).unwrap()
    }

    /// Repères de la section 12.8 de docs/gamedesign.md.
    #[test]
    fn reperes_des_expeditions_automatiques() {
        let contenu = ContenuV1::new();
        let chances = |zone: usize, equipe: &[(&str, u8)]| {
            let monstres: Vec<Monstre> = equipe
                .iter()
                .map(|&(id, valeur)| creer_moyen(&contenu, id, valeur))
                .collect();
            contenu.zones()[zone]
                .expedition_automatique()
                .chances_de_reussite(
                    &monstres,
                    crate::expedition_automatique::Protection::default(),
                )
                .pourcentage()
        };
        let salamandre = ("salamandre", 8);
        assert_eq!(chances(0, &[salamandre]), 23);
        assert_eq!(chances(0, &[salamandre, ("loup", 6)]), 71);
        assert_eq!(chances(0, &[salamandre, ("loup", 6), ("vipere", 5)]), 100);
        let salamandre = ("salamandre", 12);
        assert_eq!(chances(1, &[salamandre]), 6);
        assert_eq!(chances(1, &[salamandre, ("crapaud", 11)]), 61);
        assert_eq!(
            chances(1, &[salamandre, ("crapaud", 11), ("mandragore", 10)]),
            100
        );
    }

    #[test]
    fn identifiants_uniques() {
        for (position, fiche) in ATTAQUES.iter().enumerate() {
            assert!(
                ATTAQUES[position + 1..]
                    .iter()
                    .all(|autre| autre.id != fiche.id),
                "attaque {} en double",
                fiche.id
            );
        }
        for (position, fiche) in ESPECES.iter().enumerate() {
            assert!(
                ESPECES[position + 1..]
                    .iter()
                    .all(|autre| autre.id != fiche.id),
                "espèce {} en double",
                fiche.id
            );
        }
    }

    #[test]
    fn chaque_attaque_est_apprise_par_une_espece_ou_une_capsule() {
        for fiche in &ATTAQUES {
            let apprise = ESPECES.iter().any(|espece| {
                espece
                    .apprentissage
                    .iter()
                    .any(|&(_, attaque)| attaque == fiche.id)
            });
            let en_capsule = CAPSULES.iter().any(|capsule| capsule.attaque == fiche.id);
            let d_oeuf = ATTAQUES_D_OEUF
                .iter()
                .any(|&(_, attaque)| attaque == fiche.id);
            assert!(
                apprise || en_capsule || d_oeuf,
                "attaque {} apprise par aucune espèce ni capsule, ni d'œuf",
                fiche.id
            );
        }
    }

    #[test]
    fn chaque_famille_a_son_attaque_d_oeuf_de_son_type_et_d_elle_seule() {
        let contenu = ContenuV1::new();
        let especes = contenu.especes();
        let mut vues = Vec::new();
        for espece in especes {
            if matches!(espece.rarete(), Rarete::Apotre | Rarete::Demiurge) {
                assert!(espece.attaque_d_oeuf().is_none(), "{}", espece.nom());
                continue;
            }
            let base = crate::rite::forme_de_base(espece, especes);
            let attaque = espece.attaque_d_oeuf().expect("attaque d'œuf");
            assert_eq!(base.attaque_d_oeuf(), Some(attaque), "{}", espece.nom());
            assert!(
                base.types().liste().contains(&attaque.type_attaque()),
                "{} : {}",
                base.nom(),
                attaque.nom()
            );
            // Ni par niveau, ni par capsule : par le rite seulement.
            assert!(
                especes
                    .iter()
                    .all(|autre| autre.attaque_apprise(attaque.id()).is_none()),
                "{}",
                attaque.nom()
            );
            assert!(contenu.capsule(attaque.id()).is_none(), "{}", attaque.nom());
            if base.id() == espece.id() {
                vues.push(attaque.id().clone());
            }
        }
        let mut uniques = vues.clone();
        uniques.sort_by(|a, b| a.as_str().cmp(b.as_str()));
        uniques.dedup();
        // Celle du Colosse du rempart depuis l'étape 204.
        assert_eq!(vues.len(), 138);
        assert_eq!(uniques.len(), 138);
    }

    #[test]
    fn les_recompenses_du_codex_suivent_les_paliers_et_le_sceau_ne_se_fabrique_pas() {
        use crate::recompenses_du_codex::{PalierDuCodex, especes_au_codex};

        let contenu = ContenuV1::new();
        let au_codex = especes_au_codex(contenu.especes());
        assert_eq!(au_codex, 170);
        let seuils: Vec<u32> = contenu
            .recompenses_du_codex()
            .iter()
            .map(|recompense| recompense.palier.seuil(au_codex))
            .collect();
        assert_eq!(seuils, vec![10, 20, 30, 40, 50, 170]);
        let complet = contenu.recompenses_du_codex().last().unwrap();
        assert_eq!(complet.palier, PalierDuCodex::Complet);
        let sceau = complet.objet_unique.as_ref().unwrap();
        assert!(contenu.plan_de_fabrication(sceau.id()).is_none());
        assert_eq!(contenu.objet_d_equipement(sceau.id()).as_ref(), Some(sceau));
    }

    #[test]
    fn chaque_capsule_sert_a_une_espece_qui_ne_l_apprend_pas() {
        let contenu = ContenuV1::new();
        assert_eq!(contenu.capsules().len(), 48);
        for capsule in contenu.capsules() {
            let utile = contenu.especes().iter().any(|espece| {
                capsule.convient_a(espece) && espece.attaque_apprise(capsule.id()).is_none()
            });
            assert!(utile, "capsule {} inutile", capsule.id().as_str());
            assert_eq!(contenu.capsule(capsule.id()).as_ref(), Some(capsule));
        }
    }

    #[test]
    fn les_coffres_de_chaque_zone_ont_leurs_capsules() {
        let contenu = ContenuV1::new();
        let ids = |zone: &Zone| -> Vec<String> {
            zone.coffre()
                .capsules()
                .iter()
                .map(|capsule| capsule.id().as_str().to_string())
                .collect()
        };
        assert_eq!(
            ids(zone_par_id(&contenu, "lisiere")),
            vec!["brasier", "deferlante", "lianes-etrangleuses"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "marais")),
            vec!["croc-de-venin", "frisson-d-outre-tombe", "sentence"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "vallee")),
            vec!["malefice", "bourrasque", "furie-draconique"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "abbaye")),
            vec!["bucher", "festin-macabre", "ultrason"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "col-du-givre-noir")),
            vec!["blizzard", "seisme", "onde-de-choc"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "desert-de-sel")),
            vec!["chant-des-dunes", "eclat-du-zenith", "vent-de-sable"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "port-des-noyes")),
            vec!["maree-noire", "pince-broyeuse", "chant-des-noyes"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "mines-de-fer-noir")),
            vec!["marteau-de-forge", "coup-de-grisou", "coup-de-pioche"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "sylve-ecarlate")),
            vec!["faux-de-mante", "charge-du-sanglier", "etreinte-sanguine"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "catacombes-de-sainte-ombre")),
            vec!["peste-noire", "requiem", "morsure-de-l-araignee"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "plaine-du-massacre")),
            vec!["galop-funebre", "serres-de-poussiere", "crocs-d-acier"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "haute-cite-dechue")),
            vec!["decharge-mecanique", "festin-d-encre", "regard-hypnotique"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "gorge-de-braise")),
            vec!["coulee-de-lave", "crocs-de-braise", "marteau-ardent"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "pics-foudroyes")),
            vec!["serres-foudroyantes", "coup-de-cornes", "feu-errant"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "palais-de-chair")),
            vec!["spore-de-chair", "festin-de-chair", "ecorchure"]
        );
        assert_eq!(
            ids(zone_par_id(&contenu, "puits-du-monde")),
            vec!["chute-sans-fin", "cri-du-neant", "morsure-des-abysses"]
        );
        // Les coffres d'une annexe gardent les capsules de sa zone.
        for annexe in contenu.annexes() {
            let zone = contenu.zone(annexe.de()).unwrap();
            assert_eq!(ids(annexe.zone()), ids(&zone), "{}", annexe.zone().nom());
        }
    }

    #[test]
    fn contenu_conforme_aux_regles() {
        let contenu = ContenuV1::new();
        assert_eq!(contenu.especes().len(), 190);
        assert_eq!(contenu.zones().len(), 27);
        assert_eq!(contenu.annexes().len(), 10);
        assert_eq!(contenu.piste().indices().count(), 38);
        // Toute espèce rencontrée en zone, annexes comprises, se retrouve au
        // chargement d'une sauvegarde.
        let annexes = contenu.annexes().iter().map(Annexe::zone);
        for zone in contenu.zones().iter().chain(annexes) {
            let rares = zone
                .rares()
                .iter()
                .chain(zone.monstre_cache())
                .map(|rare| &rare.espece);
            for espece in zone.especes().iter().chain(rares) {
                assert_eq!(contenu.espece(espece.id()).as_ref(), Some(espece));
            }
        }
    }

    /// Section 25 de docs/contenu-v1.md (étape 96).
    #[test]
    fn la_tour_sans_fond_descend_par_les_zones_de_la_carte() {
        let contenu = ContenuV1::new();
        let tour = contenu.tour();
        assert_eq!(tour.id().as_str(), "tour-sans-fond");
        assert_eq!(tour.ouverte_par().as_str(), "lisiere");
        assert!(contenu.zone(tour.id()).is_none());
        let especes = |etape: u32| -> Vec<String> {
            tour.zone_de_l_etape(etape, false, Vec::new())
                .especes()
                .iter()
                .map(|espece| espece.id().as_str().to_string())
                .collect()
        };
        // Niveau 6 à l'étape 1 : la Lisière et le Marais, redécoupé (étape
        // 205).
        assert_eq!(
            especes(1),
            vec!["loup", "corbeau", "vipere", "feu-follet", "armure-hantee"]
        );
        // Niveau 20 à l'étape 8 : jusqu'à l'Abbaye, sans le Col.
        let etape_10 = especes(8);
        assert!(etape_10.contains(&"cloche-hurlante".to_string()));
        assert!(!etape_10.contains(&"wendigo".to_string()));
        assert_eq!(
            etape_10
                .iter()
                .filter(|id| id.as_str() == "squelette")
                .count(),
            1
        );
        assert!(especes(9).contains(&"faux-agneau".to_string()));
        // Le Désert de sel, dès le niveau 24 : à l'étape 10, niveau 24.
        assert!(!especes(9).contains(&"crotale-des-salines".to_string()));
        assert!(especes(10).contains(&"crotale-des-salines".to_string()));
        assert!(
            tour.recit_du_gardien(10, "Basilic")
                .contains("Basilic garde")
        );
    }

    /// Section 24 de docs/contenu-v1.md (étape 94).
    #[test]
    fn l_abbaye_accueille_les_types_son_et_lumiere() {
        let contenu = ContenuV1::new();
        let abbaye = zone_par_id(&contenu, "abbaye");
        let cloche = abbaye
            .especes()
            .iter()
            .find(|espece| espece.id().as_str() == "cloche-hurlante")
            .unwrap();
        assert_eq!(cloche.types().liste(), &[Type::Acier, Type::Son]);
        let seraphin = abbaye
            .rares()
            .iter()
            .find(|rare| rare.espece.id().as_str() == "seraphin-de-vitrail")
            .unwrap();
        assert!(seraphin.pacifique);
        assert_eq!(seraphin.espece.types().liste(), &[Type::Lumiere]);
        assert_eq!(abbaye.rares().len(), 3);
        let bronze = cloche.ressource().unwrap();
        assert_eq!((bronze.nom(), bronze.prix()), ("Éclat de bronze", 5));
        assert_eq!(seraphin.espece.ressource(), None);
    }

    #[test]
    fn la_rarete_suit_la_place_de_l_espece_dans_le_jeu() {
        let contenu = ContenuV1::new();
        // Le Kelpie est le monstre rare de la pêche (section 13.8).
        let rares: Vec<&EspeceId> = contenu
            .zones()
            .iter()
            .chain(contenu.annexes().iter().map(Annexe::zone))
            .flat_map(|zone| zone.rares().iter().chain(zone.monstre_cache()))
            .map(|rare| rare.espece.id())
            .chain([contenu.peche().rare().id()])
            // Les espèces exclusives que donne la Fosse aux damnés (section
            // 9.8).
            .chain(
                contenu
                    .fosse()
                    .adversaires()
                    .iter()
                    .filter_map(|adversaire| adversaire.recompense().monstre.as_ref())
                    .map(|(espece, _)| espece.id()),
            )
            // Les légendaires des zones d'après le Démiurge (section 9.10).
            .chain(
                contenu
                    .zones_legendaires()
                    .iter()
                    .map(|zone| zone.legendaire().espece().id()),
            )
            .collect();
        let apotres: Vec<&EspeceId> = contenu
            .piste()
            .apotres()
            .map(|apotre| apotre.espece().id())
            .collect();
        let demiurge = contenu.invocation().demiurge();
        let formes_du_demiurge = [demiurge.espece()]
            .into_iter()
            .chain(demiurge.seconde_forme())
            .map(Espece::id)
            .collect::<Vec<_>>();
        for espece in contenu.especes() {
            // Une forme évoluée a la rareté de sa forme de base (section 5.4).
            let base = contenu.especes().iter().find(|base| {
                base.evolution()
                    .is_some_and(|evolution| evolution.vers() == espece.id())
            });
            // Les gardiens attitrés de la Tour : le Colosse du rempart,
            // rare, et les Élus, des Apôtres (section 9.5, étape 204).
            let elu = contenu
                .tour()
                .elus()
                .iter()
                .any(|elu| elu.espece.id() == espece.id());
            let colosse = espece.id().as_str() == "colosse-du-rempart";
            let attendue = if let Some(base) = base {
                base.rarete()
            } else if apotres.contains(&espece.id()) || elu {
                Rarete::Apotre
            } else if colosse {
                Rarete::Rare
            } else if formes_du_demiurge.contains(&espece.id()) {
                Rarete::Demiurge
            } else if rares.contains(&espece.id()) {
                Rarete::Rare
            } else {
                Rarete::Commune
            };
            assert_eq!(espece.rarete(), attendue, "{}", espece.id().as_str());
        }
    }

    #[test]
    fn chaque_niveau_d_amelioration_a_son_plan() {
        let contenu = ContenuV1::new();
        for amelioration in Amelioration::TOUTES {
            for niveau in 1..=amelioration.niveau_max() {
                let plan = contenu
                    .plan_d_amelioration(amelioration, niveau)
                    .unwrap_or_else(|| panic!("{} {niveau} sans plan", amelioration.nom()));
                // Seules des ressources de monstres sauvages, que Malvoise
                // rachète, se dépensent (section 13.4).
                for (ressource, _) in plan.cout() {
                    assert_eq!(contenu.ressource(ressource.id()).as_ref(), Some(ressource));
                }
            }
        }
        // Le bateau depuis l'étape 207.
        assert_eq!(contenu.plans_d_amelioration().len(), 17);
    }

    #[test]
    fn les_objets_d_equipement_suivent_les_sections_11_et_34() {
        let contenu = ContenuV1::new();
        let plans = contenu.plans_de_fabrication();
        let par_sorte = |sorte| {
            plans
                .iter()
                .filter(|plan| plan.objet().sorte() == sorte)
                .count()
        };
        assert_eq!(
            (
                par_sorte(SorteDObjet::Arme),
                par_sorte(SorteDObjet::Armure),
                par_sorte(SorteDObjet::Talisman)
            ),
            (28, 32, 5)
        );
        let ids: std::collections::HashSet<_> = plans
            .iter()
            .map(|plan| plan.objet().id())
            .chain(contenu.armes_du_champion.iter().map(ObjetDEquipement::id))
            .collect();
        assert_eq!(ids.len(), plans.len() + 4);
        for plan in plans {
            let objet = plan.objet();
            // Pas de PV : le combat ne majore pas les PV (section 4.3).
            assert_eq!(objet.bonus().pv, 0, "{}", objet.nom());
            assert_ne!(objet.bonus(), Stats::default(), "{}", objet.nom());
            let heures = plan.duree().as_secs() / 3_600;
            assert!((1..=20).contains(&heures), "{}", objet.nom());
            for (ressource, _) in plan.cout() {
                assert_eq!(contenu.ressource(ressource.id()).as_ref(), Some(ressource));
            }
            assert_eq!(contenu.objet_d_equipement(objet.id()).as_ref(), Some(objet));
        }
        // Pour chaque classe, une arme et une armure par palier, que la
        // conversion des anciens objets retrouve ; une arme du palier 4, qui
        // ne se forge pas, et depuis l'étape 163 une armure du palier 4.
        for classe in Classe::TOUTES {
            let siens = contenu.plans_de_fabrication_pour(classe);
            assert_eq!(siens.len(), 20, "{}", classe.nom());
            for palier in 1..=8 {
                for sorte in [SorteDObjet::Arme, SorteDObjet::Armure] {
                    let objet = contenu.objet_de_classe(classe, sorte, palier).unwrap();
                    let forge = siens.iter().any(|plan| plan.objet() == &objet);
                    assert_eq!(forge, (sorte, palier) != (SorteDObjet::Arme, 4));
                }
            }
            let champion = contenu.arme_du_champion(classe);
            assert_eq!(champion.palier(), 4);
            assert_eq!(
                contenu.objet_d_equipement(champion.id()).as_ref(),
                Some(champion)
            );
            assert_eq!(champion.bonus().attaque, 10);
        }
        let couteau = contenu
            .plan_de_fabrication(&ObjetDEquipementId::new("couteau-de-chasse"))
            .unwrap()
            .objet();
        assert_eq!(couteau.classe(), Some(Classe::Assassin));
        assert_eq!(
            (couteau.bonus().attaque, couteau.bonus().attaque_speciale),
            (3, 3)
        );
        let attaques: Vec<_> = couteau
            .attaques()
            .iter()
            .map(|attaque| attaque.id().as_str())
            .collect();
        assert_eq!(attaques, ["entaille", "lame-empoisonnee"]);
        let plates = contenu
            .plan_de_fabrication(&ObjetDEquipementId::new("plates-consacrees"))
            .unwrap();
        assert_eq!(plates.objet().bonus().defense_speciale, 15);
        assert_eq!(plates.duree().as_secs(), 8 * 3_600);
    }

    /// Section 34 de docs/contenu-v1.md : l'attaque principale de chaque
    /// palier, puis celle de la classe, de type Humain ; depuis l'étape 149,
    /// les armes des paliers 2 à 4 y ajoutent une attaque du second type de
    /// la classe.
    #[test]
    fn les_attaques_des_heros_sont_humain_ou_du_second_type() {
        let seconds = [Type::Combat, Type::Tenebres, Type::Psychique, Type::Lumiere];
        for fiche in &ATTAQUES_DES_HEROS {
            assert!(
                fiche.type_attaque == Type::Humain || seconds.contains(&fiche.type_attaque),
                "{}",
                fiche.id
            );
            assert!(
                ATTAQUES.iter().all(|autre| autre.id != fiche.id),
                "{}",
                fiche.id
            );
        }
        let contenu = ContenuV1::new();
        for classe in Classe::TOUTES {
            let puissances: Vec<u16> = (1..=8)
                .map(|palier| {
                    let arme = match palier {
                        4 => contenu.arme_du_champion(classe).clone(),
                        _ => contenu
                            .objet_de_classe(classe, SorteDObjet::Arme, palier)
                            .unwrap(),
                    };
                    let attaques = arme.attaques();
                    assert_eq!(attaques.len(), if palier == 1 { 2 } else { 3 });
                    assert!(
                        attaques[..2]
                            .iter()
                            .all(|attaque| attaque.type_attaque() == Type::Humain)
                    );
                    if let Some(troisieme) = attaques.get(2) {
                        assert_eq!(troisieme.type_attaque(), classe.second_type());
                        assert_eq!((troisieme.puissance(), troisieme.precision()), (60, 100));
                    }
                    attaques[0].puissance()
                })
                .collect();
            assert_eq!(
                puissances,
                [40, 50, 60, 75, 85, 95, 105, 115],
                "{}",
                classe.nom()
            );
        }
    }
    #[test]
    fn chaque_evolution_mene_a_une_forme_conforme() {
        let contenu = ContenuV1::new();
        let mut evoluees = 0;
        let mut sauvages = Vec::new();
        for base in contenu.especes() {
            let Some(evolution) = base.evolution() else {
                continue;
            };
            evoluees += 1;
            let vers = contenu
                .espece(evolution.vers())
                .unwrap_or_else(|| panic!("évolution de {} absente", base.id().as_str()));
            let nom = vers.id().as_str();
            assert!(
                vers.attaque_apprise(evolution.attaque()).is_some(),
                "{nom} n'apprend pas son attaque d'évolution"
            );
            for apprise in base.apprentissage() {
                assert!(
                    vers.apprentissage().contains(apprise),
                    "{nom} n'apprend pas {}",
                    apprise.attaque.id().as_str()
                );
            }
            assert_eq!(vers.habiletes(), base.habiletes(), "{nom}");
            assert_eq!(vers.role(), base.role(), "{nom}");
            // Seules les formes évoluées sauvages du Col lâchent une
            // ressource, celle de leur forme de base (section 5.4).
            let sauvage =
                |zone: &Zone| zone.especes().iter().any(|espece| espece.id() == vers.id());
            let zones_sauvages: Vec<&str> = contenu
                .zones()
                .iter()
                .filter(|zone| sauvage(zone))
                .map(|zone| zone.id().as_str())
                .collect();
            if zones_sauvages.is_empty() {
                assert_eq!(vers.ressource(), None, "{nom}");
            } else {
                assert_eq!(zones_sauvages, vec!["col-du-givre-noir"], "{nom}");
                assert_eq!(vers.ressource(), base.ressource(), "{nom}");
                sauvages.push(nom.to_string());
            }
            assert_eq!(
                vers.rendement_experience(),
                base.rendement_experience() * 14 / 10,
                "{nom}"
            );
            assert!(
                vers.total_des_stats_de_base() > base.total_des_stats_de_base(),
                "{nom}"
            );
        }
        // Le Schnoz évolue depuis l'étape 202, les stades 3 des familles de
        // départ et de quatre autres depuis l'étape 203.
        assert_eq!(evoluees, 32);
        assert_eq!(sauvages, vec!["faux-agneau", "gisant"]);
    }

    #[test]
    fn chaque_apotre_laisse_son_ame() {
        let contenu = ContenuV1::new();
        let recits: Vec<&str> = contenu
            .piste()
            .apotres()
            .map(Apotre::recit_de_l_ame)
            .collect();
        assert_eq!(recits.len(), 13);
        assert!(recits[0].starts_with("L'âme du Baron pendu"));
        assert!(recits[1].starts_with("L'âme de l'Abbesse écorchée"));
        assert!(recits[2].starts_with("L'âme du Margrave de givre"));
        assert!(recits[3].starts_with("L'âme de l'Amiral noyé"));
        assert!(recits[4].starts_with("L'âme du Prévôt des fosses"));
        assert!(recits[5].starts_with("L'âme de la Duchesse sanguine"));
        assert!(recits[6].starts_with("L'âme de l'Évêque des os"));
        assert!(recits[7].starts_with("L'âme du Connétable boucher"));
        assert!(recits[8].starts_with("L'âme du Chancelier aux mille yeux"));
        assert!(recits[9].starts_with("L'âme du Prince de braise"));
        assert!(recits[10].starts_with("L'âme du Sénéchal des orages"));
        assert!(recits[11].starts_with("L'âme de la Reine-mère de chair"));
        assert!(recits[12].starts_with("L'âme du Roi sans visage"));
        // Avec son âme, un objet d'expérience à la mesure de son rang, à la
        // place du Philtre chromatique depuis l'étape 211 (section 14.3).
        use ObjetDExperience::{Coeur, Cristal, Fiole};
        let objets: Vec<ObjetDExperience> = contenu
            .piste()
            .apotres()
            .map(|apotre| ObjetDExperience::d_un_apotre(apotre.niveau().valeur()))
            .collect();
        assert_eq!(
            objets,
            [
                Fiole, Fiole, Fiole, Fiole, Cristal, Cristal, Cristal, Cristal, Cristal, Coeur,
                Coeur, Coeur, Coeur
            ]
        );
    }

    #[test]
    fn la_peche_remonte_des_monstres_aquatiques_qui_ne_vivent_nulle_part_ailleurs() {
        let contenu = ContenuV1::new();
        let peche = contenu.peche();
        assert_eq!(peche.duree(), Duration::from_secs(7_200));
        // Ansgar attend à l'étape 2 du Marais, et nulle part ailleurs.
        let etapes: Vec<Option<u32>> = contenu
            .zones()
            .iter()
            .map(|zone| zone.pecheur().map(Pecheur::etape))
            .collect();
        let mut attendu = vec![None; contenu.zones().len()];
        attendu[1] = Some(2);
        assert_eq!(etapes, attendu);
        let ids: Vec<&str> = peche
            .especes()
            .iter()
            .chain([peche.rare()])
            .map(|espece| espece.id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec![
                "noye",
                "anguille-des-tourbieres",
                "sangsue-geante",
                "kelpie"
            ]
        );
        for espece in peche.especes().iter().chain([peche.rare()]) {
            assert!(espece.types().liste().contains(&Type::Eau));
            assert!(espece.ressource().is_none());
            assert!(espece.evolution().is_none());
            assert_eq!(contenu.espece(espece.id()).as_ref(), Some(espece));
            for zone in contenu.zones() {
                let ailleurs = zone
                    .especes()
                    .iter()
                    .chain(zone.rares().iter().map(|rare| &rare.espece));
                assert!(ailleurs.into_iter().all(|autre| autre.id() != espece.id()));
            }
        }
    }

    #[test]
    fn le_schnoz_seul_stellaire_se_cache_dans_chaque_zone() {
        let contenu = ContenuV1::new();
        for zone in contenu.zones() {
            let cache = zone.monstre_cache().unwrap();
            assert_eq!(cache.espece.id().as_str(), "schnoz");
            assert!(cache.pacifique);
        }
        let stellaires: Vec<&str> = contenu
            .especes()
            .iter()
            .filter(|espece| espece.types().liste().contains(&Type::Stellaire))
            .map(|espece| espece.id().as_str())
            .collect();
        // Sa forme secrète du niveau 100 est la seule autre (étape 202).
        assert_eq!(stellaires, vec!["schnoz", "schnoz-primordial"]);
    }

    #[test]
    fn chaque_espece_a_deux_habiletes_et_chaque_habilete_sert() {
        let contenu = ContenuV1::new();
        for espece in contenu.especes() {
            let habiletes = espece.habiletes();
            assert_eq!(habiletes.len(), 2, "{}", espece.id().as_str());
            assert_ne!(habiletes[0], habiletes[1], "{}", espece.id().as_str());
        }
        for habilete in Habilete::TOUTES {
            assert!(
                contenu
                    .especes()
                    .iter()
                    .any(|espece| espece.habiletes().contains(&habilete)),
                "{habilete:?} sans espèce"
            );
        }
    }

    /// Section 9.8 : les paliers se suivent de 2 ou 3 niveaux (étape 148) ;
    /// les adversaires prennent le second type de leur classe dès le niveau
    /// 25, aux paliers 7 à 10 (étape 145).
    #[test]
    fn les_paliers_de_la_fosse_se_suivent_de_2_ou_3_niveaux() {
        let contenu = ContenuV1::new();
        let paliers: Vec<(u8, usize)> = contenu
            .fosse()
            .adversaires()
            .iter()
            .map(|adversaire| {
                let heros = adversaire.generer_heros();
                let types = heros.espece().types().liste();
                assert_eq!(types[0], Type::Humain);
                if let Some(second) = types.get(1) {
                    assert_eq!(*second, adversaire.classe().second_type());
                }
                (heros.niveau().valeur(), types.len())
            })
            .collect();
        assert_eq!(
            paliers,
            vec![
                (10, 1),
                (13, 1),
                (16, 1),
                (18, 1),
                (21, 1),
                (24, 1),
                (27, 2),
                (29, 2),
                (32, 2),
                (35, 2)
            ]
        );
        // Les paliers 7, 9 et 10 prennent la classe du héros, avec l'arme
        // de sa classe au palier de la leur (étape 151).
        let miroirs: Vec<bool> = contenu
            .fosse()
            .adversaires()
            .iter()
            .map(AdversaireDeLArene::est_un_miroir)
            .collect();
        assert_eq!(
            miroirs,
            [
                false, false, false, false, false, false, true, false, true, true
            ]
        );
        let champion = contenu.fosse().adversaire(10).unwrap();
        for classe in Classe::TOUTES {
            let face = champion.face_a(classe);
            assert_eq!(face.classe(), classe);
            let heros = face.generer_heros();
            assert_eq!(heros.niveau().valeur(), 35);
            assert_eq!(heros.espece().types().liste()[1], classe.second_type());
            assert_eq!(heros.attaques().liste().len(), 3);
        }
        let novice = contenu.fosse().adversaire(3).unwrap();
        assert_eq!(novice.face_a(Classe::Mage).classe(), Classe::Templier);
        // Le monstre offert est au niveau de son palier.
        for adversaire in contenu.fosse().adversaires() {
            if let Some((_, niveau)) = &adversaire.recompense().monstre {
                assert_eq!(*niveau, adversaire.niveau(), "{}", adversaire.nom());
            }
        }
    }

    /// Section 10.7, décidée le 25/10/2026 pour l'étape 131 : chaque humain
    /// de fin de zone combat après ses monstres, au niveau de son équipement.
    #[test]
    fn les_humains_de_fin_de_zone_combattent_avec_leur_classe() {
        let contenu = ContenuV1::new();
        let heros: Vec<(&str, Classe, u8, Vec<String>)> = contenu
            .zones()
            .iter()
            .filter_map(Zone::humain)
            .map(|humain| {
                let heros = humain.generer_heros().unwrap();
                assert_eq!(heros.nom(), humain.nom());
                // Le second type de sa classe dès le niveau 25 (section
                // 4.4) : l'Inquisiteur, le Porte-étendard et la Pythie.
                let classe = humain.classe().unwrap();
                let attendus = if heros.niveau().valeur() >= 25 {
                    vec![Type::Humain, classe.second_type()]
                } else {
                    vec![Type::Humain]
                };
                assert_eq!(heros.espece().types().liste(), attendus);
                let attaques = heros
                    .attaques()
                    .liste()
                    .iter()
                    .map(|attaque| attaque.id().as_str().to_string())
                    .collect();
                (
                    humain.id().as_str(),
                    humain.classe().unwrap(),
                    heros.niveau().valeur(),
                    attaques,
                )
            })
            .collect();
        let niveaux: Vec<(&str, Classe, u8)> = heros
            .iter()
            .map(|(id, classe, niveau, _)| (*id, *classe, *niveau))
            .collect();
        assert_eq!(
            niveaux,
            vec![
                ("brigand", Classe::Assassin, 10),
                // Depuis l'étape 205, au niveau de leur zone redécoupée,
                // décalé comme elle.
                ("bourreau", Classe::Guerrier, 12),
                ("inquisiteur", Classe::Mage, 17),
                ("porte-etendard", Classe::Templier, 24),
                ("pythie-de-sel", Classe::Mage, 23),
                // Au niveau maximal de leur zone, à part de leur équipement
                // (étape 163).
                ("contrebandier", Classe::Assassin, 31),
                ("contremaitre", Classe::Guerrier, 39),
                ("chasseresse", Classe::Assassin, 47),
                ("fossoyeur", Classe::Mage, 53),
                ("capitaine-mercenaire", Classe::Guerrier, 59),
                ("archiviste", Classe::Mage, 66),
                ("pyromancien", Classe::Mage, 74),
                ("heraut-de-l-orage", Classe::Templier, 82),
                ("favorite", Classe::Assassin, 90),
                ("chevalier-crane", Classe::Guerrier, 98),
            ]
        );
        assert_eq!(heros[0].3, ["entaille", "lame-empoisonnee"]);
    }

    #[test]
    fn le_brigand_et_le_bourreau_attendent_au_bout_de_la_lisiere_et_du_marais() {
        let contenu = ContenuV1::new();
        let brigand = zone_par_id(&contenu, "lisiere").humain().unwrap();
        assert_eq!(brigand.id().as_str(), "brigand");
        assert_eq!(brigand.argent(), 60);
        let equipe: Vec<(String, u8)> = brigand
            .generer_equipe()
            .iter()
            .map(|monstre| {
                (
                    monstre.espece().id().as_str().to_string(),
                    monstre.niveau().valeur(),
                )
            })
            .collect();
        assert_eq!(
            equipe,
            vec![("loup".to_string(), 7), ("corbeau".to_string(), 8)]
        );
        let bourreau = zone_par_id(&contenu, "marais").humain().unwrap();
        assert_eq!(
            (bourreau.id().as_str(), bourreau.argent()),
            ("bourreau", 150)
        );
        let niveaux: Vec<u8> = bourreau
            .generer_equipe()
            .iter()
            .map(|monstre| monstre.niveau().valeur())
            .collect();
        // Le maximum du Marais et un niveau de plus, comme les humains
        // suivants (étape 117).
        assert_eq!(niveaux, vec![10, 10, 11]);
        assert_eq!(brigand.ia(), IaHumain::Simple);
        assert_eq!(bourreau.ia(), IaHumain::Relevee);
        assert!(zone_par_id(&contenu, "vallee").humain().is_none());
    }

    /// Toutes les espèces du Marais résistent à la Plante ou la frappent
    /// fort : la famille de la Mandragore apprend une attaque Sol au niveau
    /// 7 (étape 117).
    #[test]
    fn la_famille_de_la_mandragore_apprend_secousse_au_niveau_7() {
        let contenu = ContenuV1::new();
        for id in ["mandragore", "main-de-gloire", "arbre-pendu"] {
            let espece = contenu.espece(&EspeceId::new(id)).unwrap();
            let au_niveau_7: Vec<&str> = espece
                .apprentissage()
                .iter()
                .filter(|apprise| apprise.niveau == niveau(7))
                .map(|apprise| apprise.attaque.id().as_str())
                .collect();
            assert_eq!(au_niveau_7, vec!["secousse"], "{id}");
        }
        let mandragore = creer(&contenu, "mandragore", 7);
        assert!(mandragore.attaques().connait(&AttaqueId::new("secousse")));
    }

    /// Section 31 de docs/contenu-v1.md : chaque zone garde au moins deux
    /// espèces à chaque moment (section 9.6).
    #[test]
    fn chaque_zone_a_ses_especes_de_jour_et_de_nuit() {
        fn trier(mut ids: Vec<&str>) -> Vec<&str> {
            ids.sort_unstable();
            ids
        }
        let contenu = ContenuV1::new();
        let attendu: [(&str, &[&str], &[&str]); 15] = [
            ("lisiere", &["corbeau", "vipere"], &["loup", "corbeau"]),
            (
                "marais",
                &["armure-hantee", "vipere", "corbeau"],
                &["feu-follet", "armure-hantee", "corbeau"],
            ),
            (
                "vallee",
                &["squelette", "armure-hantee"],
                &["fee-noire", "squelette", "armure-hantee"],
            ),
            (
                "abbaye",
                &["squelette", "gargouille", "cloche-hurlante"],
                &[
                    "squelette",
                    "gargouille",
                    "goule",
                    "chauve-souris-de-crypte",
                ],
            ),
            (
                "col-du-givre-noir",
                &["harpie-des-orages", "gargouille", "gisant"],
                &[
                    "wendigo",
                    "troll-des-cavernes",
                    "gargouille",
                    "faux-agneau",
                    "gisant",
                ],
            ),
            (
                "port-des-noyes",
                &["crabe-necrophage", "goeland-charognard"],
                &["crabe-necrophage", "sirene-putride"],
            ),
            (
                "mines-de-fer-noir",
                &["golem-de-fer-noir", "grisou-ardent"],
                &["golem-de-fer-noir", "mineur-possede", "grisou-ardent"],
            ),
            (
                "sylve-ecarlate",
                &["ronce-vampire", "mante-ecarlate"],
                &["ronce-vampire", "sanglier-enrage"],
            ),
            (
                "catacombes-de-sainte-ombre",
                &["rat-des-ossuaires", "araignee-des-cryptes"],
                &[
                    "rat-des-ossuaires",
                    "moine-psalmodiant",
                    "araignee-des-cryptes",
                ],
            ),
            (
                "plaine-du-massacre",
                &["vautour-des-charniers", "matin-de-guerre"],
                &["destrier-spectral", "matin-de-guerre"],
            ),
            (
                "haute-cite-dechue",
                &["automate-brise", "bibliophage"],
                &["automate-brise", "bibliophage", "oeil-errant"],
            ),
            (
                "gorge-de-braise",
                &["golem-de-magma", "forgeron-calcine"],
                &["golem-de-magma", "chien-des-scories"],
            ),
            (
                "pics-foudroyes",
                &["aigle-tonnerre", "belier-de-granit"],
                &["belier-de-granit", "feu-de-saint-elme"],
            ),
            (
                "palais-de-chair",
                &["larve-de-chair", "valet-ecorche"],
                &["larve-de-chair", "devoreur-de-chair", "valet-ecorche"],
            ),
            (
                "puits-du-monde",
                &["echo-du-vide", "ver-des-abysses"],
                &["rampant-du-puits", "echo-du-vide", "ver-des-abysses"],
            ),
        ];
        for (id, de_jour, de_nuit) in attendu {
            let zone = contenu.zone(&ZoneId::new(id)).unwrap();
            let ids = |jour_ou_nuit| -> Vec<&str> {
                zone.especes_qui_sortent(Some(jour_ou_nuit))
                    .iter()
                    .map(|espece| espece.id().as_str())
                    .collect()
            };
            let (jour, nuit) = (ids(JourOuNuit::Jour), ids(JourOuNuit::Nuit));
            assert_eq!(
                trier(jour.clone()),
                trier(de_jour.to_vec()),
                "{id}, de jour"
            );
            assert_eq!(
                trier(nuit.clone()),
                trier(de_nuit.to_vec()),
                "{id}, de nuit"
            );
            assert!(jour.len() >= 2 && nuit.len() >= 2, "{id}");
        }
    }

    /// Section 32 de docs/contenu-v1.md ; le cyclone et la tempête de sable
    /// attendent une zone de mer et un désert (section 9.7).
    #[test]
    fn chaque_zone_a_ses_meteos() {
        use Meteo::*;
        let contenu = ContenuV1::new();
        let attendu: [(&str, &[Meteo]); 16] = [
            ("lisiere", &[Pluie, Tempete, Canicule]),
            ("marais", &[Pluie, Tempete, Canicule]),
            ("vallee", &[Pluie, Tempete, Grele]),
            ("abbaye", &[Pluie, Tempete, Canicule]),
            ("col-du-givre-noir", &[Grele, Tempete]),
            ("desert-de-sel", &[TempeteDeSable, Canicule]),
            ("port-des-noyes", &[Pluie, Tempete, Cyclone]),
            ("mines-de-fer-noir", &[Grele, Tempete]),
            ("sylve-ecarlate", &[Pluie, Tempete, Canicule]),
            ("catacombes-de-sainte-ombre", &[Pluie, Tempete]),
            ("plaine-du-massacre", &[Tempete, Canicule]),
            ("haute-cite-dechue", &[Tempete, Grele]),
            ("gorge-de-braise", &[Canicule, Tempete]),
            ("pics-foudroyes", &[Tempete, Grele]),
            ("palais-de-chair", &[Pluie, Canicule]),
            ("puits-du-monde", &[Tempete, Pluie]),
        ];
        for (id, meteos) in attendu {
            let zone = contenu.zone(&ZoneId::new(id)).unwrap();
            assert_eq!(zone.meteos(), meteos, "{id}");
        }
        // Une annexe a les météos de sa zone.
        for annexe in contenu.annexes() {
            let zone = contenu.zone(annexe.de()).unwrap();
            assert_eq!(annexe.zone().meteos(), zone.meteos());
        }
        // Le cyclone ne souffle qu'en mer, la tempête de sable que sur le
        // désert.
        let zones_ou = |meteo: Meteo| -> Vec<&str> {
            contenu
                .zones()
                .iter()
                .filter(|zone| zone.meteos().contains(&meteo))
                .map(|zone| zone.id().as_str())
                .collect()
        };
        assert_eq!(zones_ou(Cyclone), ["port-des-noyes", "mer-des-naufrages"]);
        assert_eq!(zones_ou(TempeteDeSable), ["desert-de-sel"]);
    }

    #[test]
    fn chaque_classe_a_son_monstre_de_depart() {
        let contenu = ContenuV1::new();
        let ids: Vec<&str> = Classe::TOUTES
            .iter()
            .map(|classe| contenu.espece_de_depart(*classe).id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec!["crapaud", "salamandre", "mandragore", "chat-noir"]
        );
    }

    #[test]
    fn chaque_depart_connait_une_attaque_normal_et_une_de_son_type_au_niveau_5() {
        let contenu = ContenuV1::new();
        let ids: Vec<&str> = contenu
            .especes_de_depart()
            .iter()
            .map(|espece| espece.id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec!["salamandre", "crapaud", "mandragore", "chat-noir"]
        );
        for espece in contenu.especes_de_depart() {
            let monstre = creer(&contenu, espece.id().as_str(), 5);
            let types: Vec<Type> = monstre
                .attaques()
                .liste()
                .iter()
                .map(Attaque::type_attaque)
                .collect();
            assert_eq!(types, vec![Type::Normal, espece.types().liste()[0]]);
        }
    }

    #[test]
    fn le_baron_pendu_obscur_attend_au_bout_de_la_vallee_au_niveau_13() {
        let contenu = ContenuV1::new();
        let apotre = contenu.piste().apotres().next().unwrap();
        assert_eq!(apotre.niveau(), niveau(13));
        assert_eq!(apotre.zone().as_str(), "vallee");
        assert_eq!(apotre.espece().types().liste(), &[Type::Obscur]);
        // Les quatre dernières de sa liste jusqu'à son niveau, dont une seule
        // attaque Obscur : Hurlement des abysses vient au niveau suivant.
        let baron = creer(&contenu, "baron-pendu", 13);
        let ids: Vec<&str> = baron
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec![
                "lamentation",
                "sentence",
                "trappe-du-gibet",
                "etreinte-obscure"
            ]
        );
    }

    #[test]
    fn l_abbesse_obscure_attend_au_bout_de_l_abbaye_au_niveau_21() {
        let contenu = ContenuV1::new();
        let abbesse = contenu.piste().apotres().nth(1).unwrap();
        assert_eq!(abbesse.niveau(), niveau(21));
        assert_eq!(abbesse.zone().as_str(), "abbaye");
        assert_eq!(abbesse.espece().types().liste(), &[Type::Obscur]);
        assert_eq!(abbesse.espece().rarete(), Rarete::Apotre);
        let monstre = creer(&contenu, "abbesse-ecorchee", 29);
        let ids: Vec<&str> = monstre
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec![
                "hurlement-des-abysses",
                "bucher",
                "flagellation",
                "psaume-impie"
            ]
        );
        let inquisiteur = zone_par_id(&contenu, "abbaye").humain().unwrap();
        assert_eq!(inquisiteur.id().as_str(), "inquisiteur");
        assert_eq!(inquisiteur.argent(), 300);
    }

    #[test]
    fn le_margrave_obscur_attend_au_bout_du_col_au_niveau_27() {
        let contenu = ContenuV1::new();
        let margrave = contenu.piste().apotres().nth(2).unwrap();
        assert_eq!(margrave.niveau(), niveau(27));
        assert_eq!(margrave.zone().as_str(), "col-du-givre-noir");
        assert_eq!(margrave.espece().types().liste(), &[Type::Obscur]);
        assert_eq!(margrave.espece().rarete(), Rarete::Apotre);
        let monstre = creer(&contenu, "margrave-de-givre", 38);
        let ids: Vec<&str> = monstre
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str())
            .collect();
        assert_eq!(
            ids,
            vec![
                "etreinte-obscure",
                "blizzard",
                "hurlement-des-abysses",
                "linceul-de-givre"
            ]
        );
        let col = zone_par_id(&contenu, "col-du-givre-noir");
        assert_eq!(col.niveaux().1, niveau(25));
        let porte_etendard = col.humain().unwrap();
        assert_eq!(porte_etendard.id().as_str(), "porte-etendard");
        assert_eq!(porte_etendard.argent(), 400);
        // Les stades 3 du niveau 36 s'atteignent dans le Col (section 5.4).
        let wyverne = contenu
            .espece(&EspeceId::new("athanor"))
            .and_then(|espece| espece.evolution().cloned())
            .unwrap();
        assert_eq!(wyverne.vers().as_str(), "wyverne-de-cendre");
        assert_eq!(
            wyverne.condition(),
            &ConditionEvolution::Niveau(col.niveaux().1)
        );
    }

    /// Section 40 de docs/contenu-v1.md (étape 143).
    #[test]
    fn le_desert_de_sel_suit_le_margrave_sans_apotre() {
        let contenu = ContenuV1::new();
        let desert = zone_par_id(&contenu, "desert-de-sel");
        assert_eq!(desert.id().as_str(), "desert-de-sel");
        assert_eq!(desert.niveaux(), (niveau(24), niveau(28)));
        let types = |id: &str| -> Vec<Type> {
            contenu
                .espece(&EspeceId::new(id))
                .unwrap()
                .types()
                .liste()
                .to_vec()
        };
        // Une commune et une rare Son, une commune et une rare Lumière.
        assert_eq!(types("crotale-des-salines"), [Type::Son, Type::Poison]);
        assert_eq!(types("scarabee-solaire"), [Type::Lumiere, Type::Insecte]);
        assert_eq!(types("sphinx-des-sables"), [Type::Son, Type::Psychique]);
        assert_eq!(types("phenix-blanc"), [Type::Lumiere, Type::Feu]);
        assert_eq!(desert.especes().len(), 4);
        let rares: Vec<(&str, bool)> = desert
            .rares()
            .iter()
            .map(|rare| (rare.espece.id().as_str(), rare.pacifique))
            .collect();
        assert_eq!(
            rares,
            vec![("sphinx-des-sables", false), ("phenix-blanc", true)]
        );
        // La Pythie annonce un autre Apôtre sans en garder aucun.
        let pythie = desert.humain().unwrap();
        assert_eq!(pythie.id().as_str(), "pythie-de-sel");
        assert_eq!(pythie.argent(), 500);
        let equipe: Vec<(String, u8)> = pythie
            .generer_equipe()
            .iter()
            .map(|monstre| {
                (
                    monstre.espece().id().as_str().to_string(),
                    monstre.niveau().valeur(),
                )
            })
            .collect();
        assert_eq!(
            equipe,
            vec![
                ("momie-de-sel".to_string(), 28),
                ("ver-des-salines".to_string(), 28),
                ("sphinx-des-sables".to_string(), 29),
            ]
        );
        assert!(
            contenu
                .piste()
                .apotres()
                .all(|apotre| apotre.zone() != desert.id())
        );
        // Hilde ajoute la panacée à l'étal de Malvoise.
        let hilde = desert.marchand_mort().unwrap();
        assert_eq!(
            (hilde.id().as_str(), hilde.soin()),
            ("hilde", Soin::Panacee)
        );
        // Le septième indice, au Col, une fois le Margrave abattu, ouvre le
        // Désert ; depuis l'étape 163, le huitième, au Désert, ouvre le Port.
        let piste = contenu.piste();
        let vaincus: Vec<EspeceId> = piste
            .apotres()
            .map(|apotre| apotre.espece().id().clone())
            .collect();
        assert_eq!(piste.indice_suivant(8, &vaincus[..2]), None);
        let septieme = piste.indice_suivant(8, &vaincus).unwrap();
        assert_eq!(septieme.zone.as_str(), "col-du-givre-noir");
        assert!(!piste.zone_ouverte(desert.id(), 8));
        assert!(piste.zone_ouverte(desert.id(), 9));
        let huitieme = piste.indice_suivant(9, &vaincus).unwrap();
        assert_eq!(huitieme.zone.as_str(), "desert-de-sel");
        assert!(huitieme.texte.contains("le Port des Noyés"));
        let port = ZoneId::new("port-des-noyes");
        assert!(!piste.zone_ouverte(&port, 9));
        assert!(piste.zone_ouverte(&port, 10));
        // La tempête de sable et la canicule renforcent le Sol, la Roche et
        // le Feu de ses espèces.
        assert_eq!(desert.meteos(), &[Meteo::TempeteDeSable, Meteo::Canicule]);
        let griffe = contenu
            .espece(&EspeceId::new("sphinx-des-sables"))
            .and_then(|espece| espece.ressource().cloned())
            .unwrap();
        assert_eq!((griffe.nom(), griffe.prix()), ("Griffe de sphinx", 15));
    }

    #[test]
    fn la_vouivre_evolue_en_dragon_des_brumes_au_niveau_45() {
        let contenu = ContenuV1::new();
        let evolution = contenu
            .espece(&EspeceId::new("vouivre"))
            .and_then(|espece| espece.evolution().cloned())
            .unwrap();
        assert_eq!(evolution.vers().as_str(), "melusine");
        assert_eq!(
            evolution.condition(),
            &ConditionEvolution::Niveau(niveau(45))
        );
        assert_eq!(evolution.attaque().as_str(), "souffle-des-brumes");
        let dragon = contenu.espece(evolution.vers()).unwrap();
        assert_eq!(dragon.types().liste(), &[Type::Dragon, Type::Vol]);
        assert_eq!(dragon.rarete(), Rarete::Rare);
    }

    #[test]
    fn chaque_apotre_et_le_demiurge_ont_leur_ligne_au_carnet() {
        let contenu = ContenuV1::new();
        for apotre in contenu
            .piste()
            .apotres()
            .chain([contenu.invocation().demiurge()])
        {
            assert!(
                apotre.ligne_du_carnet().starts_with(apotre.espece().nom()),
                "{}",
                apotre.espece().nom()
            );
        }
    }

    #[test]
    fn le_demiurge_attend_seul_dans_l_eclipse_sous_deux_formes_au_niveau_100() {
        let contenu = ContenuV1::new();
        let invocation = contenu.invocation();
        let eclipse = invocation.eclipse();
        assert!(eclipse.est_un_lieu_d_invocation());
        assert_eq!(eclipse.etapes(), 1);
        assert!(contenu.zones().iter().all(|zone| zone.id() != eclipse.id()));
        let demiurge = invocation.demiurge();
        assert_eq!(demiurge.niveau(), niveau(100));
        assert_eq!(contenu.piste().apotres_attendus(), 13);
        assert_eq!(demiurge.espece().id().as_str(), "demiurge");
        assert_eq!(
            demiurge.seconde_forme().map(|espece| espece.id().as_str()),
            Some("demiurge-eveille")
        );
        // Réglés à l'étape 169 pour qu'une équipe préparée l'emporte environ
        // une fois sur trois (docs/equilibrage.md, section 19).
        let totaux: Vec<u32> = [demiurge.espece()]
            .into_iter()
            .chain(demiurge.seconde_forme())
            .map(Espece::total_des_stats_de_base)
            .collect();
        assert_eq!(totaux, [500, 560]);
        let attaques = |id: &str| -> Vec<String> {
            creer(&contenu, id, 100)
                .attaques()
                .liste()
                .iter()
                .map(|attaque| attaque.id().as_str().to_string())
                .collect()
        };
        assert_eq!(
            attaques("demiurge"),
            vec![
                "etreinte-obscure",
                "hurlement-des-abysses",
                "flagellation",
                "marque-du-sacrifice"
            ]
        );
        assert_eq!(
            attaques("demiurge-eveille"),
            vec![
                "hurlement-des-abysses",
                "psaume-impie",
                "marque-du-sacrifice",
                "soleil-noir"
            ]
        );
        assert!(
            invocation
                .recit_du_sacrifice("Loup")
                .starts_with("Le Béhérit pourpre s'ouvre en un cri. Loup vous est arraché")
        );
    }

    #[test]
    fn la_piste_commence_dans_la_lisiere() {
        let contenu = ContenuV1::new();
        let zones: Vec<&str> = contenu
            .piste()
            .indices()
            .map(|indice| indice.zone.as_str())
            .collect();
        assert_eq!(
            zones,
            vec![
                "lisiere",
                "marais",
                "vallee",
                "bourg-des-lamentations",
                "abbaye",
                "abbaye",
                "bois-des-farfadets",
                "col-du-givre-noir",
                "col-du-givre-noir",
                "desert-de-sel",
                "port-des-noyes",
                "port-des-noyes",
                "mer-des-naufrages",
                "mines-de-fer-noir",
                "mines-de-fer-noir",
                "greve-des-ossements",
                "sylve-ecarlate",
                "sylve-ecarlate",
                "champs-de-pavots-noirs",
                "catacombes-de-sainte-ombre",
                "catacombes-de-sainte-ombre",
                "remparts-de-grisval",
                "plaine-du-massacre",
                "plaine-du-massacre",
                "foret-petrifiee",
                "haute-cite-dechue",
                "haute-cite-dechue",
                "sanctuaire-d-asphodele",
                "gorge-de-braise",
                "gorge-de-braise",
                "glacier-des-ames",
                "pics-foudroyes",
                "pics-foudroyes",
                "cimetiere-des-geants",
                "palais-de-chair",
                "palais-de-chair",
                "faille-du-neant",
                "puits-du-monde"
            ]
        );
    }

    /// Section 44 de docs/contenu-v1.md (étape 163) : le Contrebandier
    /// localise l'Amiral noyé, dont la chute ouvre les Mines, où le
    /// Contremaître localise le Prévôt des fosses.
    #[test]
    fn le_port_et_les_mines_menent_a_l_amiral_et_au_prevot() {
        let contenu = ContenuV1::new();
        let piste = contenu.piste();
        let vaincus: Vec<EspeceId> = piste
            .apotres()
            .map(|apotre| apotre.espece().id().clone())
            .collect();
        let port = zone_par_id(&contenu, "port-des-noyes");
        let mines = zone_par_id(&contenu, "mines-de-fer-noir");
        assert_eq!(port.niveaux(), (niveau(27), niveau(31)));
        assert_eq!(mines.niveaux(), (niveau(35), niveau(39)));
        let neuvieme = piste.indice_suivant(10, &vaincus[..3]).unwrap();
        assert_eq!(&neuvieme.zone, port.id());
        assert!(
            piste
                .apotre_en_attente(port.id(), 10, &vaincus[..3])
                .is_none()
        );
        let amiral = piste
            .apotre_en_attente(port.id(), 11, &vaincus[..3])
            .unwrap();
        assert_eq!(amiral.espece().id().as_str(), "amiral-noye");
        assert_eq!(amiral.niveau(), niveau(33));
        // Le dixième, au Port, attend la chute de l'Amiral.
        assert_eq!(piste.indice_suivant(11, &vaincus[..3]), None);
        let dixieme = piste.indice_suivant(11, &vaincus[..4]).unwrap();
        assert_eq!(&dixieme.zone, port.id());
        assert!(!piste.zone_ouverte(mines.id(), 11));
        assert!(piste.zone_ouverte(mines.id(), 13));
        let onzieme = piste.indice_suivant(13, &vaincus[..4]).unwrap();
        assert_eq!(&onzieme.zone, mines.id());
        let prevot = piste
            .apotre_en_attente(mines.id(), 14, &vaincus[..4])
            .unwrap();
        assert_eq!(prevot.espece().id().as_str(), "prevot-des-fosses");
        assert_eq!(prevot.niveau(), niveau(41));
        // Le douzième, aux Mines, attend la chute du Prévôt.
        assert_eq!(piste.indice_suivant(14, &vaincus[..4]), None);
        // Obscurs, ils combattent avec trois attaques Obscur et une de leur
        // zone.
        let attaques = |apotre: &Apotre| -> Vec<String> {
            assert_eq!(apotre.espece().types().liste(), &[Type::Obscur]);
            let monstre = creer(
                &contenu,
                apotre.espece().id().as_str(),
                apotre.niveau().valeur(),
            );
            monstre
                .attaques()
                .liste()
                .iter()
                .map(|attaque| attaque.id().as_str().to_string())
                .collect()
        };
        assert_eq!(
            attaques(amiral),
            [
                "hurlement-des-abysses",
                "trombe",
                "ancre-des-damnes",
                "abime-liquide"
            ]
        );
        assert_eq!(
            attaques(prevot),
            [
                "hurlement-des-abysses",
                "seisme",
                "chaine-du-prevot",
                "gouffre-sans-fond"
            ]
        );
        let humains: Vec<(&str, u32)> = [port, mines]
            .iter()
            .map(|zone| {
                let humain = zone.humain().unwrap();
                (humain.id().as_str(), humain.argent())
            })
            .collect();
        assert_eq!(humains, [("contrebandier", 600), ("contremaitre", 700)]);
    }

    /// Section 45 de docs/contenu-v1.md (étape 165) : la chute du Prévôt
    /// ouvre la Sylve, où la Chasseresse localise la Duchesse sanguine, dont
    /// la chute ouvre les Catacombes, où le Fossoyeur localise l'Évêque des
    /// os.
    #[test]
    fn la_sylve_et_les_catacombes_menent_a_la_duchesse_et_a_l_eveque() {
        let contenu = ContenuV1::new();
        let piste = contenu.piste();
        let vaincus: Vec<EspeceId> = piste
            .apotres()
            .map(|apotre| apotre.espece().id().clone())
            .collect();
        let sylve = zone_par_id(&contenu, "sylve-ecarlate");
        let catacombes = zone_par_id(&contenu, "catacombes-de-sainte-ombre");
        assert_eq!(sylve.niveaux(), (niveau(43), niveau(47)));
        assert_eq!(catacombes.niveaux(), (niveau(50), niveau(53)));
        let douzieme = piste.indice_suivant(14, &vaincus[..5]).unwrap();
        assert_eq!(douzieme.zone.as_str(), "mines-de-fer-noir");
        assert!(!piste.zone_ouverte(sylve.id(), 14));
        assert!(piste.zone_ouverte(sylve.id(), 16));
        let treizieme = piste.indice_suivant(16, &vaincus[..5]).unwrap();
        assert_eq!(&treizieme.zone, sylve.id());
        let duchesse = piste
            .apotre_en_attente(sylve.id(), 17, &vaincus[..5])
            .unwrap();
        assert_eq!(duchesse.espece().id().as_str(), "duchesse-sanguine");
        assert_eq!(duchesse.niveau(), niveau(49));
        assert_eq!(piste.indice_suivant(17, &vaincus[..5]), None);
        let quatorzieme = piste.indice_suivant(17, &vaincus[..6]).unwrap();
        assert_eq!(&quatorzieme.zone, sylve.id());
        assert!(piste.zone_ouverte(catacombes.id(), 19));
        let quinzieme = piste.indice_suivant(19, &vaincus[..6]).unwrap();
        assert_eq!(&quinzieme.zone, catacombes.id());
        let eveque = piste
            .apotre_en_attente(catacombes.id(), 20, &vaincus[..6])
            .unwrap();
        assert_eq!(eveque.espece().id().as_str(), "eveque-des-os");
        assert_eq!(eveque.niveau(), niveau(55));
        // Le seizième, aux Catacombes, attend la chute de l'Évêque.
        assert_eq!(piste.indice_suivant(20, &vaincus[..6]), None);
        let attaques = |apotre: &Apotre| -> Vec<String> {
            assert_eq!(apotre.espece().types().liste(), &[Type::Obscur]);
            creer(
                &contenu,
                apotre.espece().id().as_str(),
                apotre.niveau().valeur(),
            )
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str().to_string())
            .collect()
        };
        assert_eq!(
            attaques(duchesse),
            [
                "hurlement-des-abysses",
                "ronces-maudites",
                "baiser-de-sang",
                "psaume-impie"
            ]
        );
        assert_eq!(
            attaques(eveque),
            [
                "hurlement-des-abysses",
                "epee-des-morts",
                "flagellation",
                "ossuaire-sacre"
            ]
        );
        // La Chasseresse n'a que deux monstres, comme le Contrebandier.
        let equipes: Vec<(&str, usize, u32)> = [sylve, catacombes]
            .iter()
            .map(|zone| {
                let humain = zone.humain().unwrap();
                (
                    humain.id().as_str(),
                    humain.generer_equipe().len(),
                    humain.argent(),
                )
            })
            .collect();
        assert_eq!(equipes, [("chasseresse", 2, 800), ("fossoyeur", 3, 900)]);
    }

    /// Section 49 de docs/contenu-v1.md (étape 169) : trois zones légendaires
    /// au niveau 100, peuplées d'espèces existantes, chacune gardée par son
    /// légendaire, qui compte au codex.
    #[test]
    fn les_zones_legendaires_ont_leur_legendaire_au_niveau_100() {
        use crate::recompenses_du_codex::compte_au_codex;

        let contenu = ContenuV1::new();
        let legendaires: Vec<(&str, &str, Vec<Type>)> = contenu
            .zones_legendaires()
            .iter()
            .map(|legendaire| {
                let zone = legendaire.zone();
                assert_eq!(zone.niveaux(), (niveau(100), niveau(100)));
                assert_eq!(legendaire.legendaire().niveau(), niveau(100));
                let espece = legendaire.legendaire().espece();
                assert!(compte_au_codex(espece), "{}", espece.nom());
                assert_eq!(espece.rarete(), Rarete::Rare);
                assert_eq!(espece.total_des_stats_de_base(), 600, "{}", espece.nom());
                // Ses monstres sauvages viennent d'autres zones.
                for sauvage in zone.especes() {
                    assert!(
                        contenu
                            .zones()
                            .iter()
                            .any(|autre| autre.especes().contains(sauvage)),
                        "{}",
                        sauvage.nom()
                    );
                }
                assert!(!legendaire.legendaire().avant_combat().is_empty());
                assert!(!legendaire.legendaire().capture().is_empty());
                (
                    zone.id().as_str(),
                    espece.id().as_str(),
                    espece.types().liste().to_vec(),
                )
            })
            .collect();
        assert_eq!(
            legendaires,
            [
                ("abysses", "leviathan", vec![Type::Eau, Type::Dragon]),
                ("terres-arides", "behemoth", vec![Type::Sol, Type::Combat]),
                ("ciel-dechire", "ziz", vec![Type::Vol, Type::Lumiere]),
            ]
        );
        let attaques: Vec<String> = creer(&contenu, "leviathan", 100)
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str().to_string())
            .collect();
        assert_eq!(
            attaques,
            [
                "maree-noire",
                "souffle-des-brumes",
                "deluge-draconique",
                "abime-des-mers"
            ]
        );
    }

    /// Section 48 de docs/contenu-v1.md (étape 168) : la chute du Sénéchal
    /// ouvre le Palais de Chair, où la Favorite localise la Reine-mère de
    /// chair, dont la chute ouvre le Puits du Monde, où le Chevalier-Crâne
    /// localise le Roi sans visage, le treizième et dernier Apôtre, au
    /// niveau 100.
    #[test]
    fn le_palais_et_le_puits_menent_a_la_reine_mere_et_au_roi() {
        let contenu = ContenuV1::new();
        let piste = contenu.piste();
        let vaincus: Vec<EspeceId> = piste
            .apotres()
            .map(|apotre| apotre.espece().id().clone())
            .collect();
        assert_eq!(vaincus.len(), 13);
        assert_eq!(piste.apotres_attendus(), 13);
        let palais = zone_par_id(&contenu, "palais-de-chair");
        let puits = zone_par_id(&contenu, "puits-du-monde");
        assert_eq!(palais.niveaux(), (niveau(87), niveau(90)));
        assert_eq!(puits.niveaux(), (niveau(95), niveau(98)));
        let vingt_quatrieme = piste.indice_suivant(32, &vaincus[..11]).unwrap();
        assert_eq!(vingt_quatrieme.zone.as_str(), "pics-foudroyes");
        assert!(!piste.zone_ouverte(palais.id(), 32));
        assert!(piste.zone_ouverte(palais.id(), 34));
        let vingt_cinquieme = piste.indice_suivant(34, &vaincus[..11]).unwrap();
        assert_eq!(&vingt_cinquieme.zone, palais.id());
        let reine = piste
            .apotre_en_attente(palais.id(), 35, &vaincus[..11])
            .unwrap();
        assert_eq!(reine.espece().id().as_str(), "reine-mere-de-chair");
        assert_eq!(reine.niveau(), niveau(92));
        assert_eq!(piste.indice_suivant(35, &vaincus[..11]), None);
        let vingt_sixieme = piste.indice_suivant(35, &vaincus[..12]).unwrap();
        assert_eq!(&vingt_sixieme.zone, palais.id());
        assert!(piste.zone_ouverte(puits.id(), 37));
        let vingt_septieme = piste.indice_suivant(37, &vaincus[..12]).unwrap();
        assert_eq!(&vingt_septieme.zone, puits.id());
        let roi = piste
            .apotre_en_attente(puits.id(), 38, &vaincus[..12])
            .unwrap();
        assert_eq!(roi.espece().id().as_str(), "roi-sans-visage");
        assert_eq!(roi.niveau(), niveau(100));
        // Plus aucun indice après le dernier Apôtre : le Béhérit prend le
        // relais (section 9.4).
        assert_eq!(piste.indice_suivant(38, &vaincus), None);
        let attaques = |apotre: &Apotre| -> Vec<String> {
            assert_eq!(apotre.espece().types().liste(), &[Type::Obscur]);
            creer(
                &contenu,
                apotre.espece().id().as_str(),
                apotre.niveau().valeur(),
            )
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str().to_string())
            .collect()
        };
        assert_eq!(
            attaques(reine),
            [
                "hurlement-des-abysses",
                "peste-noire",
                "baiser-de-sang",
                "couvee-immonde"
            ]
        );
        assert_eq!(
            attaques(roi),
            [
                "hurlement-des-abysses",
                "jugement",
                "gouffre-sans-fond",
                "effacement"
            ]
        );
        // La Favorite, Assassin, n'a que deux monstres.
        let equipes: Vec<(&str, usize, u32)> = [palais, puits]
            .iter()
            .map(|zone| {
                let humain = zone.humain().unwrap();
                (
                    humain.id().as_str(),
                    humain.generer_equipe().len(),
                    humain.argent(),
                )
            })
            .collect();
        assert_eq!(
            equipes,
            [("favorite", 2, 1400), ("chevalier-crane", 3, 1500)]
        );
    }

    /// Section 47 de docs/contenu-v1.md (étape 167) : la chute du Chancelier
    /// ouvre la Gorge de Braise, où le Pyromancien localise le Prince de
    /// braise, dont la chute ouvre les Pics Foudroyés, où le Héraut de
    /// l'orage localise le Sénéchal des orages.
    #[test]
    fn la_gorge_et_les_pics_menent_au_prince_et_au_senechal() {
        let contenu = ContenuV1::new();
        let piste = contenu.piste();
        let vaincus: Vec<EspeceId> = piste
            .apotres()
            .map(|apotre| apotre.espece().id().clone())
            .collect();
        let gorge = zone_par_id(&contenu, "gorge-de-braise");
        let pics = zone_par_id(&contenu, "pics-foudroyes");
        assert_eq!(gorge.niveaux(), (niveau(71), niveau(74)));
        assert_eq!(pics.niveaux(), (niveau(79), niveau(82)));
        let vingtieme = piste.indice_suivant(26, &vaincus[..9]).unwrap();
        assert_eq!(vingtieme.zone.as_str(), "haute-cite-dechue");
        assert!(!piste.zone_ouverte(gorge.id(), 26));
        assert!(piste.zone_ouverte(gorge.id(), 28));
        let vingt_et_unieme = piste.indice_suivant(28, &vaincus[..9]).unwrap();
        assert_eq!(&vingt_et_unieme.zone, gorge.id());
        let prince = piste
            .apotre_en_attente(gorge.id(), 29, &vaincus[..9])
            .unwrap();
        assert_eq!(prince.espece().id().as_str(), "prince-de-braise");
        assert_eq!(prince.niveau(), niveau(76));
        assert_eq!(piste.indice_suivant(29, &vaincus[..9]), None);
        let vingt_deuxieme = piste.indice_suivant(29, &vaincus[..10]).unwrap();
        assert_eq!(&vingt_deuxieme.zone, gorge.id());
        assert!(piste.zone_ouverte(pics.id(), 31));
        let vingt_troisieme = piste.indice_suivant(31, &vaincus[..10]).unwrap();
        assert_eq!(&vingt_troisieme.zone, pics.id());
        let senechal = piste
            .apotre_en_attente(pics.id(), 32, &vaincus[..10])
            .unwrap();
        assert_eq!(senechal.espece().id().as_str(), "senechal-des-orages");
        assert_eq!(senechal.niveau(), niveau(84));
        // Le vingt-quatrième, dans les Pics, attend la chute du Sénéchal.
        assert_eq!(piste.indice_suivant(32, &vaincus[..10]), None);
        let attaques = |apotre: &Apotre| -> Vec<String> {
            assert_eq!(apotre.espece().types().liste(), &[Type::Obscur]);
            creer(
                &contenu,
                apotre.espece().id().as_str(),
                apotre.niveau().valeur(),
            )
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str().to_string())
            .collect()
        };
        assert_eq!(
            attaques(prince),
            [
                "hurlement-des-abysses",
                "jet-de-lave",
                "flagellation",
                "couronne-de-braise"
            ]
        );
        assert_eq!(
            attaques(senechal),
            [
                "hurlement-des-abysses",
                "onde-de-choc",
                "foudre",
                "tempete-noire"
            ]
        );
        let equipes: Vec<(&str, usize, u32)> = [gorge, pics]
            .iter()
            .map(|zone| {
                let humain = zone.humain().unwrap();
                (
                    humain.id().as_str(),
                    humain.generer_equipe().len(),
                    humain.argent(),
                )
            })
            .collect();
        assert_eq!(
            equipes,
            [("pyromancien", 3, 1200), ("heraut-de-l-orage", 3, 1300)]
        );
    }

    /// Section 46 de docs/contenu-v1.md (étape 166) : la chute de l'Évêque
    /// ouvre la Plaine du Massacre, où le Capitaine mercenaire localise le
    /// Connétable boucher, dont la chute ouvre la Haute-Cité déchue, où
    /// l'Archiviste localise le Chancelier aux mille yeux.
    #[test]
    fn la_plaine_et_la_haute_cite_menent_au_connetable_et_au_chancelier() {
        let contenu = ContenuV1::new();
        let piste = contenu.piste();
        let vaincus: Vec<EspeceId> = piste
            .apotres()
            .map(|apotre| apotre.espece().id().clone())
            .collect();
        let plaine = zone_par_id(&contenu, "plaine-du-massacre");
        let cite = zone_par_id(&contenu, "haute-cite-dechue");
        assert_eq!(plaine.niveaux(), (niveau(56), niveau(59)));
        assert_eq!(cite.niveaux(), (niveau(63), niveau(66)));
        let seizieme = piste.indice_suivant(20, &vaincus[..7]).unwrap();
        assert_eq!(seizieme.zone.as_str(), "catacombes-de-sainte-ombre");
        assert!(!piste.zone_ouverte(plaine.id(), 20));
        assert!(piste.zone_ouverte(plaine.id(), 22));
        let dix_septieme = piste.indice_suivant(22, &vaincus[..7]).unwrap();
        assert_eq!(&dix_septieme.zone, plaine.id());
        let connetable = piste
            .apotre_en_attente(plaine.id(), 23, &vaincus[..7])
            .unwrap();
        assert_eq!(connetable.espece().id().as_str(), "connetable-boucher");
        assert_eq!(connetable.niveau(), niveau(61));
        assert_eq!(piste.indice_suivant(23, &vaincus[..7]), None);
        let dix_huitieme = piste.indice_suivant(23, &vaincus[..8]).unwrap();
        assert_eq!(&dix_huitieme.zone, plaine.id());
        assert!(piste.zone_ouverte(cite.id(), 25));
        let dix_neuvieme = piste.indice_suivant(25, &vaincus[..8]).unwrap();
        assert_eq!(&dix_neuvieme.zone, cite.id());
        let chancelier = piste
            .apotre_en_attente(cite.id(), 26, &vaincus[..8])
            .unwrap();
        assert_eq!(
            chancelier.espece().id().as_str(),
            "chancelier-aux-mille-yeux"
        );
        assert_eq!(chancelier.niveau(), niveau(68));
        // Le vingtième, dans la Haute-Cité, attend la chute du Chancelier.
        assert_eq!(piste.indice_suivant(26, &vaincus[..8]), None);
        let attaques = |apotre: &Apotre| -> Vec<String> {
            assert_eq!(apotre.espece().types().liste(), &[Type::Obscur]);
            creer(
                &contenu,
                apotre.espece().id().as_str(),
                apotre.niveau().valeur(),
            )
            .attaques()
            .liste()
            .iter()
            .map(|attaque| attaque.id().as_str().to_string())
            .collect()
        };
        assert_eq!(
            attaques(connetable),
            [
                "hurlement-des-abysses",
                "dechiquetage",
                "flagellation",
                "boucherie"
            ]
        );
        assert_eq!(
            attaques(chancelier),
            [
                "hurlement-des-abysses",
                "jugement",
                "regard-des-mille-yeux",
                "secret-interdit"
            ]
        );
        let equipes: Vec<(&str, usize, u32)> = [plaine, cite]
            .iter()
            .map(|zone| {
                let humain = zone.humain().unwrap();
                (
                    humain.id().as_str(),
                    humain.generer_equipe().len(),
                    humain.argent(),
                )
            })
            .collect();
        assert_eq!(
            equipes,
            [("capitaine-mercenaire", 3, 1000), ("archiviste", 3, 1100)]
        );
    }

    /// Sections 9.9 et 44 de docs/contenu-v1.md (étape 163) : une annexe par
    /// zone d'Apôtre, un peu plus haute, avec les espèces de sa zone, une
    /// rare pacifique qui ne vit que là et un trésor dans chaque coffre.
    #[test]
    fn chaque_annexe_suit_sa_zone_avec_sa_rare_et_ses_tresors() {
        let contenu = ContenuV1::new();
        let annexes: Vec<(&str, &str, (u8, u8), &str)> = contenu
            .annexes()
            .iter()
            .map(|annexe| {
                let zone = annexe.zone();
                assert_eq!(zone.etapes(), 3);
                assert_eq!(zone.repartition(), Repartition::DES_ANNEXES);
                assert!(zone.humain().is_none());
                assert_eq!(zone.monstre_cache().unwrap().espece.id().as_str(), "schnoz");
                let de = contenu.zone(annexe.de()).unwrap();
                assert_eq!(zone.especes(), de.especes());
                let [rare] = zone.rares() else {
                    panic!("une seule rare dans {}", zone.nom());
                };
                assert!(rare.pacifique);
                assert!(rare.espece.ressource().is_none());
                let ailleurs = contenu
                    .zones()
                    .iter()
                    .flat_map(|zone| {
                        zone.especes()
                            .iter()
                            .chain(zone.rares().iter().map(|rare| &rare.espece))
                    })
                    .any(|espece| espece.id() == rare.espece.id());
                assert!(!ailleurs, "{}", rare.espece.nom());
                // Le Philtre chromatique, 3 fois sur 100 depuis l'étape 211.
                let tresors = zone.coffre().tresors();
                assert_eq!(tresors.iter().map(|(_, poids)| poids).sum::<u32>(), 100);
                assert!(tresors.contains(&(Tresor::PhiltreChromatique, 3)));
                assert!(!annexe.decouverte().is_empty());
                let (min, max) = zone.niveaux();
                (
                    zone.id().as_str(),
                    annexe.de().as_str(),
                    (min.valeur(), max.valeur()),
                    rare.espece.id().as_str(),
                )
            })
            .collect();
        assert_eq!(
            annexes,
            [
                (
                    "epave-du-leviathan",
                    "port-des-noyes",
                    (30, 33),
                    "nautile-des-abysses"
                ),
                ("filon-ecarlate", "mines-de-fer-noir", (38, 41), "gemmame"),
                (
                    "autel-des-ronces",
                    "sylve-ecarlate",
                    (46, 49),
                    "phalene-d-aube"
                ),
                (
                    "crypte-des-saints",
                    "catacombes-de-sainte-ombre",
                    (52, 55),
                    "ange-de-pierre"
                ),
                (
                    "tertre-des-heros",
                    "plaine-du-massacre",
                    (58, 61),
                    "licorne-des-cendres"
                ),
                (
                    "bibliotheque-interdite",
                    "haute-cite-dechue",
                    (65, 68),
                    "muse-de-cristal"
                ),
                ("coeur-du-volcan", "gorge-de-braise", (73, 76), "ifrit"),
                (
                    "aire-des-pegases",
                    "pics-foudroyes",
                    (81, 84),
                    "pegase-d-orage"
                ),
                (
                    "jardin-de-la-reine",
                    "palais-de-chair",
                    (89, 92),
                    "lys-de-sang"
                ),
                (
                    "racines-du-monde",
                    "puits-du-monde",
                    (96, 99),
                    "germe-du-monde"
                ),
            ]
        );
    }
}
