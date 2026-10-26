# Équilibrage de Bersemon

Ce document décrit l'outil d'équilibrage, la stratégie du joueur automatique et ses limites, puis les résultats mesurés le 25/10/2026 (étape 116), les constats qu'on en tire, les variantes mesurées et les réglages retenus à l'étape 117. Les décisions sont reportées dans docs/gamedesign.md (sections 5.3, 9.4 et 10.7, Q107 et Q108, section 17).

## 1. L'outil

- Le crate `bersemon-equilibrage` fait jouer des parties complètes par un joueur automatique, avec le noyau seul : ni interface, ni sauvegarde. Chaque partie va de la création du héros à la chute du troisième Apôtre, le Margrave de givre ; depuis l'étape 144, elle continue jusqu'à la chute de l'humain de la dernière zone de la carte, la Pythie de sel, au bout du Désert de sel (section 10) ; depuis l'étape 163, jusqu'à la chute du dernier Apôtre de la piste et de l'humain de la dernière zone, quel que soit le contenu (section 14).
- Commande : `cargo run --release -p bersemon-equilibrage -- [parties par classe] [classe]`. Sans argument, 25 parties par classe ; avec une classe, seule celle-ci est jouée. Le rapport s'écrit en Markdown sur la sortie standard.
- Une partie se rejoue à l'identique avec la même graine. En release, 100 parties par classe se jouent en 5 secondes environ.
- Pour mesurer un réglage, on change la valeur dans le noyau, on relance, et on compare les deux rapports. Toute modification du jeu change la suite des tirages : des écarts de 1 à 3 expéditions sur la médiane sont de l'ordre du bruit.
- Depuis l'étape 146, le rapport finit par la Fosse aux damnés, simulée à part : le héros de chaque classe, à chaque niveau de combat que donne son équipement, contre l'adversaire de chaque palier, 200 combats par case, avec et sans le second type des classes (section 11). Elle se joue en moins d'une seconde.
- Depuis l'étape 163, deux mesures à part suivent la Fosse : les humains de fin de zone contre des équipes tirées au hasard, et les espèces sauvages en un contre un (section 14).
- Le crate ne dépend que du noyau, sans crate externe. Il est hors de ce que joue le joueur : ni la console ni le bot Discord n'en dépendent.

## 2. Le joueur automatique

Sa stratégie est simple et constante, pour que les écarts d'une zone à l'autre viennent des valeurs du jeu :

- il part dans la zone la plus avancée où le niveau de ses meilleurs monstres a passé le premier tiers des niveaux de la zone ;
- il emmène ses monstres de plus haut niveau ; quand un Apôtre l'attend au bout de la zone, ceux qui valent le plus face à lui, selon la table des types, ses stats et ses PV. Depuis l'étape 163, il compose toujours son équipe ainsi, face à l'Apôtre, à l'équipe de l'humain gardien ou aux espèces sauvages de la zone, parmi ses monstres à 5 niveaux au plus du meilleur, 10 contre un Apôtre (section 14) ;
- en combat, il envoie le monstre qui vaut au moins deux fois plus que le monstre actif face à l'adversaire, attaque avec l'attaque aux dégâts attendus les plus hauts, lance un filet sur une espèce qu'il n'a pas quand la chance atteint 50 %, et fuit quand son dernier monstre debout a perdu la moitié de ses PV. Contre un humain ou un Apôtre, il donne l'ordre du héros et soigne son monstre sous 35 % de ses PV ;
- entre deux étapes, il soigne un monstre sous la moitié de ses PV, rentre quand la moitié de l'équipe est KO ou qu'il ne lui reste que la moitié de ses PV, et n'affronte le combat de fin de zone qu'avec tous ses monstres debout et 80 % de ses PV ;
- au camp, il donne les âmes à Malvoise, réclame les récompenses du codex, fait agrandir l'équipe puis forger l'objet d'équipement aux plus gros bonus, puis améliorer les filets, porte ses meilleurs objets, range argent et ressources au coffre, et emporte 6 potions et 10 filets.

Il n'emploie pas les expéditions automatiques, la cour d'entraînement, les pièges, le rite, les capsules, les habiletés, les pierres, les boosters ni la Tour sans fond, et ne revient pas dans une zone pour y chercher une ressource. Il mesure donc une progression sans ces aides : un joueur qui s'en sert ira plus vite. Hypothèse : chaque choix prend une minute de temps réel, pauses comprises, ce qui règle la durée des travaux de Gunther en nombre d'expéditions.

## 3. Constats

Mesurés sur 100 parties par classe à l'étape 116, avant les réglages de l'étape 117 ; le rapport de ce moment est dans l'historique git du document, celui d'après les réglages à la section 6.

1. Durée : il faut une médiane de 55 à 64 expéditions pour abattre le Margrave (Guerrier 55, Assassin 58, Mage 61, Templier 64), avec 9 à 13 défaites par partie, presque toutes contre les combats de fin de zone. Aucune durée cible n'est fixée (Q107).
2. Progression : l'équipe entre dans chaque zone vers le premier tiers de ses niveaux et bat chaque Apôtre à peu près à son niveau. Les zones demandent toutes 11 à 13 expéditions : la courbe d'expérience suit la difficulté.
3. Le Baron pendu est le mur de la partie : 4,8 tentatives, 12 % de victoires au premier essai, 19 % quand l'équipe est au niveau maximal de la Vallée, contre 2,0 à 2,2 tentatives pour l'Abbesse et le Margrave. La difficulté des Apôtres décroît donc d'un Apôtre à l'autre. Le Guerrier et le Mage y subissent 5,3 à 5,5 défaites, le Templier et l'Assassin 2,0 à 2,6 : la Salamandre et le Crapaud sont résistés par l'Obscur et le subissent fort, et les types qui frappent un Apôtre (Féérique, Psychique, Lumière, Stellaire) sont encore rares dans la Vallée.
4. Le Bourreau est l'humain le plus dur : 2,6 tentatives, 21 % au premier essai. Ses monstres ont 1 à 2 niveaux de plus que le maximum du Marais (14, 14 et 15), quand l'Inquisiteur et le Porte-étendard ont le maximum de leur zone et un niveau de plus (27, 27, 28 et 36, 36, 37).
5. Le Templier est la classe la plus lente : dans le Marais, 15,3 expéditions et 3,2 défaites, contre 10 à 11 expéditions et 1,5 à 1,8 défaite pour les autres. Toutes les espèces du Marais résistent aux attaques Plante ou les frappent fort, les monstres du Bourreau résistent tous à la Plante, et les attaques Normal de la Mandragore ne touchent pas les Spectres.
6. Les ressources rares sont trop rares pour la progression : 0,4 Croc de basilic, 0,2 Écaille de vouivre et 0,1 Crin de chimère par partie. L'équipe de 4 n'est jamais obtenue avant le Margrave, et les objets des paliers 2 et 3 presque jamais : 2 à 2,5 objets forgés, du palier 1. Même quand les monstres rares lâchent toujours leur ressource (1,4, 0,7 et 0,5 par partie), l'équipe de 4 n'arrive pas avant le Margrave : elle demande les deux ressources rares et 16 ressources communes.
7. L'argent ne bloque pas : 1 600 à 1 900 pièces restent à la fin, après 1 900 à 2 300 dépensées en potions et filets.
8. Les combats sauvages ne coûtent presque plus de défaites quand le joueur fuit à temps (4 % des expéditions dans la Lisière, 17 à 20 % ailleurs, combats de fin de zone compris).

## 4. Variantes mesurées

Chaque variante change une valeur du noyau, le temps de la mesure, sur 100 parties par classe. Les expéditions et les défaites sont données pour le Guerrier, le Mage, le Templier et l'Assassin.

| Variante | Expéditions (médiane) | Défaites (moyenne) | Baron pendu : tentatives, 1er essai | Bourreau : tentatives, 1er essai | Templier dans le Marais : expéditions (défaites) |
|---|---|---|---|---|---|
| Référence | 55, 61, 64, 58 | 11,5, 13,4, 10,0, 8,8 | 4,8, 12 % | 2,6, 21 % | 15,3 (3,2) |
| Baron pendu au niveau 20, le maximum de la Vallée | 54, 59, 62, 56 | 8,6, 10,0, 9,4, 7,2 | 2,7, 36 % | 2,6, 21 % | 15,3 (3,2) |
| Baron pendu à -15 % de stats de base (480 à 408) | 54, 59, 64, 58 | 9,9, 10,4, 9,9, 7,8 | 3,5, 24 % | 2,6, 21 % | 15,3 (3,2) |
| Bourreau un niveau plus bas (13, 13, 14) | 58, 60, 66, 58 | 12,6, 13,2, 9,2, 8,4 | 5,4, 12 % | 2,1, 36 % | 14,0 (2,4) |
| Mandragore : Secousse au niveau 7, à la place de Cri strident | 55, 61, 62, 58 | 11,5, 13,4, 9,2, 8,8 | 4,7, 12 % | 2,6, 24 % | 12,9 (2,5) |
| Monstres rares qui lâchent toujours leur ressource | 56, 60, 65, 58 | 11,1, 13,2, 10,4, 8,7 | 4,8, 13 % | 2,7, 22 % | 15,1 (3,3) |
| Baron au niveau 20, Bourreau à 13, 13, 14 et Secousse | 54, 58, 61, 56 | 7,9, 10,3, 7,6, 7,1 | 2,6, 36 % | 2,0, 38 % | 12,4 (1,7) |

Avec les trois réglages réunis, l'Abbesse demande 2,5 tentatives (53 % au premier essai) et le Margrave 2,0 (63 %) : les trois Apôtres se valent, et les classes vont de 54 à 61 expéditions. Avec des rares qui lâchent toujours leur ressource, 70 % des parties trouvent un Croc de basilic, 41 % une Écaille de vouivre et 33 % un Crin de chimère, contre 32 %, 20 % et 14 %.

## 5. Réglages retenus (étape 117)

Choix de l'utilisateur :

- le Baron pendu au niveau 20, le maximum de la Vallée, les Apôtres suivants restant 2 niveaux au-dessus du maximum de leur zone. À ce niveau, il ne connaît qu'une attaque Obscur, Étreinte obscure : Hurlement des abysses vient au niveau 21. C'est de là que vient l'essentiel du gain : au niveau 20 avec ses deux attaques Obscur (Hurlement des abysses au niveau 20), il demandait encore 3,8 tentatives, 22 % au premier essai ;
- les monstres du Bourreau aux niveaux 13, 13 et 14, comme ceux des autres humains de fin de zone ;
- Secousse (Sol, physique, 60, précision 100) au niveau 7 de la famille de la Mandragore, à la place du Cri strident ;
- les ressources rares inchangées : l'équipe de 4 et les objets des paliers 2 et 3 restent des objectifs à farmer.

Résultat, mesuré sur 100 parties par classe : 54 à 61 expéditions selon la classe (Guerrier 54, Assassin 56, Mage 58, Templier 61), 7 à 10 défaites par partie ; le Baron demande 2,6 tentatives (36 % au premier essai), l'Abbesse 2,5 (53 %), le Margrave 2,0 (63 %), le Bourreau 2,0 (38 %). Les chiffres sont ceux de la dernière ligne de la section 4 : le code retenu rejoue exactement la variante mesurée.

## 6. Rapport de référence, après l'étape 117

100 parties par classe, du héros à la chute du Margrave de givre, jouées par le joueur automatique (crates/bersemon-equilibrage). Le niveau de l'équipe est la moyenne des monstres de plus haut niveau, équipe et camp réunis, autant qu'en compte l'équipe.

### Parties

| Classe | Terminées | Expéditions (médiane) | Défaites (moyenne) | Niveau final de l'équipe | Achats chez Malvoise | Argent à la fin | Équipe de 4 dès l'expédition | Objets forgés | Choix refusés |
|---|---|---|---|---|---|---|---|---|---|
| Guerrier | 100/100 | 54 | 7.9 | 37.7 | 1925 | 1816 | jamais | 2.6 | 0.0 |
| Mage | 100/100 | 58 | 10.3 | 38.2 | 2238 | 1549 | jamais | 2.7 | 0.0 |
| Templier | 100/100 | 61 | 7.6 | 37.8 | 1988 | 1935 | jamais | 2.5 | 0.0 |
| Assassin | 100/100 | 56 | 7.1 | 37.8 | 1876 | 1985 | jamais | 2.7 | 0.0 |

### Zones

Moyennes par expédition, sauf mention. L'argent, les filets et les potions sont la variation du sac et du coffre : une défaite fait perdre le sac.

| Zone (niveaux) | Expéditions par partie | Menées au bout | Défaites | Rentrées | Niveau à la première | Niveaux gagnés | Argent | Filets | Potions | Nouvelles espèces |
|---|---|---|---|---|---|---|---|---|---|---|
| Lisière du Bois-Sombre (3 à 7) | 13.4 | 37 % | 4 % | 59 % | 5.0 | 0.31 | 13 | 0.33 | -0.45 | 0.30 |
| Marais des Pendus (8 à 13) | 11.3 | 56 % | 12 % | 32 % | 9.1 | 0.62 | 32 | 0.42 | 0.49 | 0.39 |
| Vallée des Brumes (14 à 20) | 11.3 | 69 % | 15 % | 17 % | 16.1 | 0.62 | 41 | 1.01 | 1.03 | 0.34 |
| Abbaye des Cendres (21 à 27) | 11.0 | 29 % | 22 % | 49 % | 23.1 | 0.69 | 77 | 2.22 | 2.58 | 0.57 |
| Col du Givre-Noir (28 à 36) | 11.5 | 17 % | 20 % | 63 % | 30.7 | 0.63 | 102 | 2.26 | 2.36 | 0.59 |

### Zones par classe

Expéditions par partie, et entre parenthèses les défaites par partie.

| Classe | Lisière du Bois-Sombre | Marais des Pendus | Vallée des Brumes | Abbaye des Cendres | Col du Givre-Noir |
|---|---|---|---|---|---|
| Guerrier | 12.4 (0.2) | 10.3 (1.2) | 11.4 (2.3) | 10.6 (2.1) | 10.6 (2.0) |
| Mage | 13.4 (0.4) | 11.5 (1.2) | 11.3 (2.7) | 10.5 (3.1) | 12.4 (2.9) |
| Templier | 15.0 (0.7) | 12.4 (1.7) | 11.8 (0.9) | 11.3 (2.3) | 11.6 (2.0) |
| Assassin | 12.7 (0.7) | 11.1 (1.3) | 10.8 (0.7) | 11.5 (2.2) | 11.2 (2.2) |

### Combats de fin de zone

| Combat | Niveau maximal de la zone | Tentatives par partie | Victoires au premier essai | Niveau de l'équipe à la victoire |
|---|---|---|---|---|
| Lisière du Bois-Sombre : humain | 7 | 1.4 | 70 % | 7.4 |
| Marais des Pendus : humain | 13 | 2.0 | 38 % | 13.1 |
| Vallée des Brumes : Apôtre | 20 | 2.6 | 36 % | 19.4 |
| Abbaye des Cendres : Apôtre | 27 | 2.5 | 53 % | 29.8 |
| Abbaye des Cendres : humain | 27 | 1.8 | 52 % | 27.0 |
| Col du Givre-Noir : Apôtre | 36 | 2.0 | 63 % | 37.6 |
| Col du Givre-Noir : humain | 36 | 1.9 | 53 % | 35.2 |

### Victoires selon le niveau de l'équipe

Part des tentatives gagnées, selon l'écart entre le niveau moyen de l'équipe au départ de l'expédition et le niveau maximal de la zone ; entre parenthèses, le nombre de tentatives. Un Apôtre a 2 niveaux de plus que le maximum de sa zone.

| Combat | -4 ou moins | -3 à -2 | -1 à 0 | +1 à +2 | +3 ou plus |
|---|---|---|---|---|---|
| Lisière du Bois-Sombre : humain | - | 31 % (74) | 73 % (332) | 99 % (136) | 100 % (2) |
| Marais des Pendus : humain | 25 % (125) | 31 % (147) | 52 % (366) | 77 % (151) | 93 % (15) |
| Vallée des Brumes : Apôtre | 20 % (249) | 31 % (197) | 46 % (455) | 57 % (134) | 100 % (2) |
| Abbaye des Cendres : Apôtre | 100 % (1) | 33 % (6) | 34 % (71) | 38 % (473) | 44 % (437) |
| Abbaye des Cendres : humain | 41 % (176) | 40 % (53) | 54 % (259) | 66 % (212) | 76 % (33) |
| Col du Givre-Noir : Apôtre | 0 % (5) | 47 % (17) | 49 % (234) | 50 % (414) | 51 % (137) |
| Col du Givre-Noir : humain | 45 % (141) | 39 % (180) | 59 % (303) | 61 % (109) | 50 % (38) |

### Ressources trouvées

Moyenne par partie, perdues ou non dans une défaite, et part des parties qui en trouvent au moins une.

| Ressource | Par partie | Parties |
|---|---|---|
| Aile de chauve-souris | 3.9 | 94 % |
| Aile de fée noire | 8.5 | 100 % |
| Cendre de feu follet | 5.2 | 98 % |
| Crin de chimère | 0.1 | 13 % |
| Croc de basilic | 0.6 | 44 % |
| Croc de loup | 7.7 | 100 % |
| Dent de goule | 3.9 | 95 % |
| Fragment d'armure | 16.5 | 100 % |
| Os de squelette | 12.4 | 100 % |
| Pierre de troll | 2.7 | 81 % |
| Plume de corbeau | 9.9 | 100 % |
| Plume de harpie | 2.8 | 82 % |
| Poil de wendigo | 2.7 | 84 % |
| Écaille de vipère | 10.4 | 100 % |
| Écaille de vouivre | 0.3 | 23 % |
| Écaille de wyrm | 0.2 | 15 % |
| Éclat de bronze | 3.6 | 94 % |
| Éclat de gargouille | 7.2 | 99 % |

## 7. Effet du cycle du jour et de la nuit (étape 118)

Le joueur automatique ne connaît pas le cycle : il part à toute heure, son horloge avançant d'une minute par choix. Mesuré sur 100 parties par classe, avant et après le cycle :

| Mesure | Après l'étape 117 | Avec le cycle |
|---|---|---|
| Expéditions (médiane : Guerrier, Mage, Templier, Assassin) | 54, 58, 61, 56 | 50, 52, 58, 54 |
| Défaites (moyenne) | 7,9, 10,3, 7,6, 7,1 | 9,3, 10,2, 6,8, 6,3 |
| Baron pendu : tentatives, 1er essai | 2,6, 36 % | 2,6, 38 % |
| Abbesse écorchée | 2,5, 53 % | 2,4, 52 % |
| Margrave de givre | 2,0, 63 % | 2,0, 66 % |
| Bourreau | 2,0, 38 % | 2,0, 37 % |

La nuit profite à ce joueur : ses monstres plus forts rapportent plus d'XP, et le bonus de 25 % s'y ajoute, sans beaucoup plus de défaites, puisqu'il fuit les combats mal engagés. Les combats de fin de zone, hors du cycle, ne changent pas.

## 8. Effet de la météo (étape 121)

Mesuré sur 100 parties par classe, cycle du jour et météo compris : 51, 54, 55 et 54 expéditions (Guerrier, Mage, Templier, Assassin), 9,0, 11,6, 7,2 et 6,5 défaites ; le Baron pendu demande 2,7 tentatives (34 % au premier essai), l'Abbesse 2,4 (53 %), le Margrave 2,0 (68 %), le Bourreau 2,0 (35 %). Les écarts avec la section 7 sont de l'ordre du bruit : une météo touche un créneau sur 3, renforce deux types pour les deux camps, et le joueur automatique n'en tient pas compte.

## 9. Effet du héros combattant et des armes de classe (étapes 128 à 134)

Le joueur automatique ne change pas de stratégie : son héros entre de lui-même quand l'équipe tombe, et il l'y fait attaquer comme un monstre ; il forge les objets de sa classe qui renforcent le plus ses monstres ; il n'entre pas dans la Fosse aux damnés ni ne joue l'arène entre joueurs. Mesuré sur 100 parties par classe, après l'étape 138 :

| Mesure | Section 8 (météo) | Avec le héros combattant |
|---|---|---|
| Expéditions (médiane : Guerrier, Mage, Templier, Assassin) | 51, 54, 55, 54 | 50, 54, 59, 54 |
| Défaites (moyenne) | 9,0, 11,6, 7,2, 6,5 | 8,7, 11,4, 7,0, 7,0 |
| Baron pendu : tentatives, 1er essai | 2,7, 34 % | 2,6, 36 % |
| Abbesse écorchée | 2,4, 53 % | 2,2, 58 % |
| Margrave de givre | 2,0, 68 % | 1,9, 69 % |
| Bourreau | 2,0, 35 % | 2,1, 34 % |
| Brigand, Inquisiteur, Porte-étendard | non mesurés | 1,5 et 63 %, 1,7 et 55 %, 2,0 et 50 % |
| Objets forgés (moyenne) | 2,6 (section 6) | 1,7 |

Le héros change peu la progression de ce joueur : au niveau de son équipement, il reste une proie face aux monstres des zones, comme voulu (section 4.4 de gamedesign.md), et ne sauve qu'une part des combats perdus. Les humains de fin de zone, qui combattent désormais après leurs monstres, restent dans les mêmes eaux ; le brigand, Assassin au niveau 10, est un peu plus dur qu'à l'étape 117 (63 % au premier essai au lieu de 70 %). Les armes de classe réduisent le choix de Gunther : 1,7 objet forgé par partie au lieu de 2,6. Le Templier, le plus lent, gagne quelques expéditions de plus, dans la marge du bruit d'une classe à l'autre.

## 10. Désert de sel (étapes 143 et 144)

Le joueur automatique continue désormais après le Margrave, jusqu'à la chute de la Pythie de sel, au bout du Désert de sel ; sa stratégie ne change pas : il entre dans le Désert une fois le niveau 39 passé (le premier tiers de ses niveaux), et n'emploie ni la panacée, qu'il n'achète pas, ni l'Éclipse. Les mesures jusqu'au Margrave ne se comparent donc plus aux sections précédentes que zone par zone. Mesuré sur 100 parties par classe, après l'étape 143 :

| Mesure | Guerrier | Mage | Templier | Assassin |
|---|---|---|---|---|
| Expéditions jusqu'à la Pythie (médiane) | 64 | 66 | 70 | 68 |
| Défaites (moyenne) | 10,8 | 13,8 | 8,7 | 10,0 |
| Expéditions dans le Désert (défaites) | 10,5 (2,1) | 12,5 (2,4) | 10,8 (1,6) | 14,5 (3,0) |

| Zone | Expéditions par partie | Menées au bout | Défaites | Rentrées | Niveau à la première |
|---|---|---|---|---|---|
| Col du Givre-Noir (28 à 36) | 14,9 | 26 % | 16 % | 58 % | 30,9 |
| Désert de sel (37 à 45) | 12,1 | 8 % | 19 % | 73 % | 39,6 |

| Combat | Tentatives par partie | Victoires au premier essai | Niveau de l'équipe à la victoire |
|---|---|---|---|
| Porte-étendard (Col, 36) | 2,0 | 50 % | 36,1 |
| Margrave de givre (Col, 38) | 1,9 | 69 % | 38,3 |
| Pythie de sel (Désert, 45) | 2,3 | 48 % | 45,2 |

Constats : la Pythie de sel est aussi difficile que le Porte-étendard (2,3 tentatives et 48 % au premier essai, contre 2,0 et 50 %), et ses victoires se gagnent au niveau maximal de la zone, comme celles des autres humains de fin de zone. Le Désert ajoute 12 expéditions environ à une partie, avec une part de défaites proche de celle du Col (19 % contre 16 %). Ses expéditions sont rarement menées au bout (8 %, contre 26 % dans le Col) : le joueur y entre au niveau 39, six niveaux sous le maximum, et rentre quand l'équipe faiblit. Hypothèse, non mesurée : ses espèces communes, de 390 au total au lieu de 360 dans le Col, y pèsent pour beaucoup. Aucun réglage n'est fait : les valeurs du Désert attendent la validation de l'utilisateur (gamedesign.md, Q113).

## 11. Fosse aux damnés et second type des classes (étapes 145 et 146)

Le joueur automatique n'entre pas dans la Fosse : elle est simulée à part. Le héros de chaque classe, à chaque niveau de combat (10 : arme du palier 1 sans armure ; 15 : 1 et 1 ; 20 : 2 et 1 ; 25 : 2 et 2 ; 30 : 3 et 2 ; 35 : 3 et 3 ; 40 : arme du palier 4 et armure du palier 3), affronte l'adversaire de chaque palier, 200 fois, seul et sans objet, en attaquant avec l'attaque aux dégâts attendus les plus hauts ; l'adversaire suit l'IA de son palier. Mesuré après l'étape 145, où les humains de niveau 25 ou plus ont le second type de leur classe :

| Palier | Adversaire (classe, niveau, types) | 10 | 15 | 20 | 25 | 30 | 35 | 40 | Tours |
|---|---|---|---|---|---|---|---|---|---|
| 1 | Le Gueux (Guerrier, 10, Humain) | 21 % | 100 % | 100 % | 100 % | 100 % | 100 % | 100 % | 1,6 |
| 2 | Le Coupe-jarret (Assassin, 15, Humain) | 0 % | 65 % | 100 % | 100 % | 100 % | 100 % | 100 % | 1,9 |
| 3 | Le Novice de l'ordre (Templier, 15, Humain) | 0 % | 58 % | 100 % | 100 % | 100 % | 100 % | 100 % | 2,6 |
| 4 | L'Apprenti nécromant (Mage, 20, Humain) | 0 % | 0 % | 55 % | 100 % | 100 % | 100 % | 100 % | 2,1 |
| 5 | Le Mercenaire (Guerrier, 20, Humain) | 0 % | 0 % | 28 % | 100 % | 100 % | 100 % | 100 % | 2,1 |
| 6 | La Lame noire (Assassin, 25, Humain, Ténèbres) | 0 % | 0 % | 0 % | 66 % | 100 % | 100 % | 100 % | 3,0 |
| 7 | La Sorcière des cendres (Mage, 30, Humain, Psychique) | 0 % | 0 % | 0 % | 0 % | 51 % | 100 % | 100 % | 3,2 |
| 8 | Le Templier renégat (Templier, 30, Humain, Lumière) | 0 % | 0 % | 0 % | 0 % | 40 % | 100 % | 100 % | 4,4 |
| 9 | Le Gladiateur (Guerrier, 35, Humain, Combat) | 0 % | 0 % | 0 % | 0 % | 0 % | 35 % | 100 % | 3,3 |
| 10 | Le Champion de la Fosse (Assassin, 35, Humain, Ténèbres) | 0 % | 0 % | 0 % | 0 % | 1 % | 66 % | 100 % | 2,9 |

Part de victoires, moyenne des quatre classes, selon le niveau de combat du héros ; dernière colonne, la durée moyenne d'un combat, en tours.

Effet du second type : la même mesure, tous les humains restant Humain seul, ne change la part de victoires qu'au niveau même de l'adversaire, de -14 à +14 points selon le palier (palier 7 au niveau 30 : +14 ; palier 9 au niveau 35 : -14), et rien ailleurs. Les combats des paliers 6 à 10 durent en moyenne 2,0 à 2,6 tours sans le second type, 2,9 à 4,4 avec.

Dans les expéditions, le second type de l'Inquisiteur, du Porte-étendard et de la Pythie ne change rien au-delà du bruit (100 parties par classe) : 1,7 tentative et 56 % au premier essai pour l'Inquisiteur (1,7 et 55 % avant), 1,9 et 53 % pour le Porte-étendard (2,0 et 50 %), 2,2 et 46 % pour la Pythie (2,3 et 48 %) ; 64, 64, 71 et 68 expéditions jusqu'à la Pythie (64, 66, 70 et 68 avant).

Constats :

- La Fosse est un contrôle de niveau : un héros de 5 niveaux au-dessus de l'adversaire gagne toujours, 5 niveaux au-dessous perd toujours ; au même niveau, il gagne une fois sur deux environ, selon les classes. Chaque palier attend donc un palier d'équipement de plus, ce qu'il récompense d'autant.
- Les combats sont très courts : 2 à 4 tours. Hypothèse, non mesurée : avec le bonus de type sur ses attaques Humain, neutres contre un humain à type simple, chaque coup emporte une grande part des PV adverses, et le plus rapide frappe le premier. Le second type allonge les combats de haut niveau de moitié environ, sans changer qui gagne.
- Le joueur automatique de la Fosse n'emploie que l'attaque la plus forte : le poison et la paralysie des attaques de classe n'y pèsent pas, alors qu'ils pèseraient davantage dans des combats plus longs. Aucun réglage n'est fait ; la durée des combats et l'écart de 5 niveaux d'un palier à l'autre sont à trancher (gamedesign.md, Q114).

## 12. PV du héros (étape 147)

Mesuré avant le choix de l'utilisateur, sur 100 parties par classe et 200 combats par case de la Fosse, en relevant les PV de base des quatre classes (90, 70, 95 et 70 jusqu'ici), ceux des humains de l'IA compris :

| Mesure | Avant | PV x1,5 | PV x2 (retenu) | PV x3 |
|---|---|---|---|---|
| Durée d'un combat de la Fosse, paliers 1, 4, 6, 8 et 10 (tours) | 1,6, 2,1, 3,0, 4,4, 2,9 | 2,0, 2,5, 3,6, 5,4, 3,5 | 2,2, 2,9, 4,2, 6,4, 4,2 | 2,7, 3,7, 5,3, 8,6, 5,4 |
| Fosse : part de victoires au niveau de l'adversaire, paliers 6 à 10 | 66, 51, 40, 35, 66 % | 65, 57, 38, 37, 66 % | 62, 57, 36, 37, 66 % | 64, 58, 36, 38, 64 % |
| Expéditions jusqu'à la Pythie (médiane : Guerrier, Mage, Templier, Assassin) | 64, 64, 71, 68 | 62, 63, 73, 68 | 64, 66, 70, 70 | 62, 65, 72, 68 |
| Défaites (moyenne) | 10,5, 13,1, 8,7, 9,4 | 11,2, 13,1, 9,2, 9,1 | 12,6, 14,6, 10,0, 9,6 | 11,3, 13,5, 9,1, 10,2 |
| Pythie de sel : tentatives, 1er essai | 2,2, 46 % | 2,2, 50 % | 2,7, 38 % | 2,5, 43 % |

Constats : plus de PV allongent les combats entre humains, un peu moins qu'en proportion des PV gagnés (avec x2, un héros de niveau 35 a 1,56 fois plus de PV et ses combats durent environ 1,45 fois plus longtemps) ; ils ne changent pas qui gagne à la Fosse, où un héros de 5 niveaux de plus gagne toujours et un héros de 5 niveaux de moins perd toujours, quels que soient les PV. Dans les expéditions, les écarts sont de l'ordre du bruit : les défaites un peu plus nombreuses et la Pythie un peu plus dure avec x2 ne se retrouvent ni avec x1,5 ni avec x3. L'utilisateur a retenu x2.

## 13. Paliers rapprochés, attaques du second type et adversaires miroirs (étapes 148 à 151)

Mesuré sur 100 parties par classe et 200 combats par case de la Fosse. Avec les niveaux fixés de l'étape 148 (10, 13, 16, 18, 21, 24, 27, 29, 32, 35) et les attaques du second type de l'étape 149, chaque palier d'équipement fait gagner un palier de plus ou deux, mais les affinités de types décidaient de certains combats d'avance, quel que soit le niveau :

| Palier | Classes qui ne le gagnaient pas au niveau 35 |
|---|---|
| 7, Sorcière des cendres (Mage, 27) | Templier (0 %, 88 % au niveau 40) ; le Guerrier gagnait 37 % |
| 9, Gladiateur (Guerrier, 32) | Templier et Assassin (0 %, même au niveau 40) |
| 10, Champion de la Fosse (Assassin, 35) | Mage et Templier (0 %, même au niveau 40) |

Le niveau 40 demande l'arme du palier 4, récompense du palier 10 : un Templier ne pouvait pas dépasser le palier 7. Après l'étape 151, où ces trois adversaires prennent la classe du héros, part de victoires selon le niveau de combat du héros, moyenne des quatre classes :

| Palier | Adversaire (classe, niveau, types) | 10 | 15 | 20 | 25 | 30 | 35 | 40 | Tours |
|---|---|---|---|---|---|---|---|---|---|
| 1 | Le Gueux (Guerrier, 10, Humain) | 18 % | 100 % | 100 % | 100 % | 100 % | 100 % | 100 % | 2,0 |
| 2 | Le Coupe-jarret (Assassin, 13, Humain) | 9 % | 100 % | 100 % | 100 % | 100 % | 100 % | 100 % | 2,1 |
| 3 | Le Novice de l'ordre (Templier, 16, Humain) | 0 % | 19 % | 100 % | 100 % | 100 % | 100 % | 100 % | 3,3 |
| 4 | L'Apprenti nécromant (Mage, 18, Humain) | 0 % | 0 % | 75 % | 100 % | 100 % | 100 % | 100 % | 2,0 |
| 5 | Le Mercenaire (Guerrier, 21, Humain) | 0 % | 0 % | 4 % | 99 % | 100 % | 100 % | 100 % | 2,3 |
| 6 | La Lame noire (Assassin, 24, Humain) | 0 % | 0 % | 1 % | 50 % | 73 % | 100 % | 100 % | 1,9 |
| 7 | La Sorcière des cendres (classe du héros, 27) | 0 % | 0 % | 0 % | 0 % | 100 % | 100 % | 100 % | 3,6 |
| 8 | Le Templier renégat (Templier, 29, Humain, Lumière) | 0 % | 0 % | 0 % | 75 % | 100 % | 100 % | 100 % | 4,0 |
| 9 | Le Gladiateur (classe du héros, 32) | 0 % | 0 % | 0 % | 0 % | 1 % | 100 % | 100 % | 3,8 |
| 10 | Le Champion de la Fosse (classe du héros, 35) | 0 % | 0 % | 0 % | 0 % | 0 % | 51 % | 100 % | 3,6 |

Niveau de combat à partir duquel chaque classe gagne au moins une fois sur deux :

| Palier | Guerrier | Mage | Templier | Assassin |
|---|---|---|---|---|
| 6 | 25 | 35 | 30 | 25 |
| 7 | 30 | 30 | 30 | 30 |
| 8 | 25 | 25 | 30 | 25 |
| 9 | 35 | 35 | 35 | 35 |
| 10 | 35 (52 %) | 35 (49 %) | 35 (48 %) | 35 (54 %) |

Dans les expéditions, à peine plus dur qu'à la section 12 : 13,3, 14,3, 10,0 et 10,3 défaites par partie ; 1,9 tentative et 46 % au premier essai pour l'Inquisiteur, 2,2 et 47 % pour le Porte-étendard, 2,8 et 38 % pour la Pythie, dont les héros ont désormais l'attaque du second type contre les monstres du joueur.

Constats :

- Chaque classe peut finir la Fosse : chaque palier se gagne au niveau 35 au plus, le dernier une fois sur deux environ, à chances égales entre les classes.
- Un combat miroir se décide presque entièrement au niveau : 3 niveaux d'avance le gagnent toujours, 2 de retard le perdent toujours. Les paliers à classe fixe restent marqués par les affinités : la Lame noire (palier 6) résiste au Mage jusqu'au niveau 35.
- Le Templier reste le plus faible face aux humains : son attaque Lumière ne fait que x0,5 à un humain, x0,25 à un Templier. C'est vrai aussi à l'arène entre joueurs, qui n'a pas d'adversaire miroir.


## 14. Port des Noyés et Mines de Fer-Noir (étape 163)

Premier lot de la suite de l'aventure (gamedesign.md, section 9.1 ; docs/contenu-v1.md, section 44). Cible : environ 12 expéditions par zone (choix de l'utilisateur, 27/10/2026).

Changements de l'outil :

- La partie va jusqu'à la chute du dernier Apôtre de la piste et de l'humain de la dernière zone de la carte, quel que soit le contenu : le Prévôt des fosses et le Contremaître depuis cette étape.
- Le joueur automatique compose son équipe selon ce qui l'attend dans la zone visée : l'Apôtre au bout, sinon l'équipe de l'humain qui la garde encore, sinon ses espèces sauvages à son niveau moyen, selon la valeur déjà employée contre les Apôtres (table des types, stats, PV). Seuls entrent dans ce choix les monstres à 5 niveaux au plus du meilleur, 10 contre un Apôtre. Avant, il emmenait ses monstres de plus haut niveau, sauf contre un Apôtre, où il prenait les meilleurs quels que soient leurs niveaux : au Port, il partait avec un Phénix de niveau 40 et une Sylphe de niveau 34, et rentrait dès la première étape. Les zones des sections précédentes en sont à peine changées (voir le tableau final).
- Deux mesures à part, écrites à la fin du rapport, qui ne dépendent pas des équipes du joueur automatique : chaque humain de fin de zone contre 100 équipes de trois monstres tirées au hasard parmi les espèces que le joueur a pu garder avant sa zone (formes évoluées comprises, 380 au moins au total), au niveau maximal de la zone, avec un héros Mage équipé au mieux sans dépasser ce niveau ; chaque espèce sauvage en un contre un, au niveau 50, contre toutes les espèces sauvages de 380 à 500 au total. Ce sont elles qui ont trouvé les causes, plus sûrement que les parties.

Mesures et réglages successifs (25 parties par classe, sauf la dernière ligne, 100) :

| Version | Expéditions au Port | Aux Mines | Contrebandier (tentatives, premier essai) | Cause relevée |
|---|---|---|---|---|
| Contenu proposé au lot 1 | 89,4 | 69,3 | 7,0 ; 10 % | communes de 445 à 465 au total, rares de 520 à 560 |
| Communes ramenées à 410-420, rendements d'XP à 3 fois le niveau | 66,1 | 45,9 | 6,5 ; 19 % | équipe du joueur automatique faible contre l'Eau |
| Équipe composée selon les types | 39,9 à 55 | 19,6 à 56 | 5,1 à 6,3 ; 16 à 20 % | grand bruit d'une version à l'autre |
| Attaques de 90 et plus après la zone ; espèces recalées sur le Désert | 39,9 | 31,1 | 4,7 ; 21 % | héros Assassin de niveau 51 en dernier |
| Contrebandier à deux monstres ; Amiral et Prévôt au gabarit du Margrave | 31,4 | 15,4 | 3,1 ; 39 % | Amiral |
| Final, 100 parties par classe | 29,3 | 15,2 | 2,7 ; 45 % | Amiral |

Constats :

1. Les espèces proposées au lot 1 étaient bien plus fortes que leurs aînées à niveau égal. En un contre un au niveau 50, le Crabe nécrophage, à 390 au total, gagnait 62 % de ses combats grâce à son typage Eau et Acier, et les rares du lot 61 à 85 %, contre 30 à 44 % pour les communes du Désert et 59 à 63 % pour ses rares. Recalées : communes de 353 à 390 au total (42 à 53 %), rares de 430 à 465 (53 à 66 %). Le typage Acier pèse plus que le total : le Golem de fer noir gagne 53 % avec 353.
2. Les attaques de puissance 90 et plus, apprises aux niveaux 45 à 55, durcissaient tous les combats de la zone. Elles s'apprennent désormais aux niveaux 58 à 64 : elles servent au joueur qui capture ces espèces, pas aux monstres sauvages de la zone.
3. Le niveau du héros d'un humain pèse peu (2 à 3 points de victoires pour 5 niveaux), sa classe beaucoup : à niveau 51, un héros Assassin en dernier, rapide et aux PV doublés, laissait 22 % de victoires contre des équipes au hasard, un Guerrier ou un Templier 37 %. Son arme ne change rien. Le Contrebandier garde sa classe et son niveau, mais n'a plus que deux monstres, comme le brigand : 47 % contre des équipes au hasard (Bourreau 53 %, Inquisiteur 58 %, Porte-étendard 63 %, Pythie 60 %), et 45 % au premier essai dans les parties.
4. Le Contremaître reste à 34 % contre des équipes au hasard, mais se gagne à 58 % au premier essai dans les parties (2,1 tentatives), où le joueur compose son équipe contre lui.
5. L'Amiral noyé reste le combat le plus dur du lot : 4,5 tentatives, 59 % au premier essai mais 18 à 30 % des tentatives suivantes, quel que soit l'écart de niveau. Le joueur automatique lui oppose ses monstres Lumière, dont le Phénix blanc, que son attaque Eau (Trombe) frappe x2, et ne change jamais d'équipe après une défaite. Le Prévôt des fosses suit (3,9 tentatives). Hypothèse, non vérifiable avec l'outil : un joueur qui change d'équipe après une défaite le bat en 2 ou 3 tentatives, comme le Margrave.
6. Le Port gagne 0,27 niveau par expédition, les Mines 0,30, contre 0,54 dans le Désert : ses espèces, surtout Eau, punissent les captures du Désert (Sol, Roche, Feu), et 69 % des expéditions y sont rentrées avant la fin. Les Mines, plus courtes à traverser après le Port, sont dans la cible.

| Zone | Expéditions par partie | Menées au bout | Défaites | Rentrées | Niveau à la première | Niveaux gagnés |
|---|---|---|---|---|---|---|
| Lisière du Bois-Sombre (3 à 7) | 12,1 | 22 % | 6 % | 73 % | 5,0 | 0,35 |
| Marais des Pendus (8 à 13) | 10,5 | 44 % | 14 % | 43 % | 9,3 | 0,66 |
| Vallée des Brumes (14 à 20) | 9,3 | 53 % | 20 % | 26 % | 16,2 | 0,75 |
| Abbaye des Cendres (21 à 27) | 12,2 | 24 % | 21 % | 55 % | 23,2 | 0,64 |
| Col du Givre-Noir (28 à 36) | 14,2 | 31 % | 20 % | 50 % | 31,0 | 0,61 |
| Désert de sel (37 à 45) | 14,6 | 31 % | 10 % | 59 % | 39,6 | 0,54 |
| Port des Noyés (46 à 51) | 29,3 | 12 % | 19 % | 69 % | 47,4 | 0,27 |
| Mines de Fer-Noir (52 à 57) | 15,2 | 13 % | 27 % | 59 % | 55,4 | 0,30 |

| Combat | Tentatives par partie | Victoires au premier essai | Niveau de l'équipe à la victoire |
|---|---|---|---|
| Pythie de sel (Désert, 45) | 2,0 | 49 % | 44,7 |
| Contrebandier (Port, 51) | 2,7 | 45 % | 51,7 |
| Amiral noyé (Port, 53) | 4,5 | 59 % | 54,3 |
| Contremaître (Mines, 57) | 2,1 | 58 % | 57,6 |
| Prévôt des fosses (Mines, 59) | 3,9 | 59 % | 59,6 |

Parties complètes, jusqu'au Prévôt : 104 à 115 expéditions (médiane selon la classe), 17 à 28 défaites, équipe finale au niveau 59 à 61.

Le Port dépasse donc la cible de 12 expéditions de plus du double avec le joueur automatique ; les Mines y sont. Aucun réglage n'est fait au-delà de ceux du tableau : le reste tient à l'Amiral et aux types de l'équipe que ce joueur emmène, à vérifier en jouant (gamedesign.md, Q118). Leviers possibles, mesurés ou non : l'Amiral sans attaque Eau (Trombe), ou un rendement d'XP plus haut au Port.
