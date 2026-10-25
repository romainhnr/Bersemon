# Équilibrage de Bersemon

Ce document décrit l'outil d'équilibrage, la stratégie du joueur automatique et ses limites, puis les résultats mesurés le 25/10/2026 (étape 116), les constats qu'on en tire, les variantes mesurées et les réglages retenus à l'étape 117. Les décisions sont reportées dans docs/gamedesign.md (sections 5.3, 9.4 et 10.7, Q107 et Q108, section 17).

## 1. L'outil

- Le crate `bersemon-equilibrage` fait jouer des parties complètes par un joueur automatique, avec le noyau seul : ni interface, ni sauvegarde. Chaque partie va de la création du héros à la chute du troisième Apôtre, le Margrave de givre.
- Commande : `cargo run --release -p bersemon-equilibrage -- [parties par classe] [classe]`. Sans argument, 25 parties par classe ; avec une classe, seule celle-ci est jouée. Le rapport s'écrit en Markdown sur la sortie standard.
- Une partie se rejoue à l'identique avec la même graine. En release, 100 parties par classe se jouent en 5 secondes environ.
- Pour mesurer un réglage, on change la valeur dans le noyau, on relance, et on compare les deux rapports. Toute modification du jeu change la suite des tirages : des écarts de 1 à 3 expéditions sur la médiane sont de l'ordre du bruit.
- Le crate ne dépend que du noyau, sans crate externe. Il est hors de ce que joue le joueur : ni la console ni le bot Discord n'en dépendent.

## 2. Le joueur automatique

Sa stratégie est simple et constante, pour que les écarts d'une zone à l'autre viennent des valeurs du jeu :

- il part dans la zone la plus avancée où le niveau de ses meilleurs monstres a passé le premier tiers des niveaux de la zone ;
- il emmène ses monstres de plus haut niveau ; quand un Apôtre l'attend au bout de la zone, ceux qui valent le plus face à lui, selon la table des types, ses stats et ses PV ;
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
