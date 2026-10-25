# Équilibrage de Bersemon

Ce document décrit l'outil d'équilibrage, la stratégie du joueur automatique et ses limites, puis les résultats mesurés le 25/10/2026 (étape 116), les constats qu'on en tire et les réglages proposés. Les décisions qui en découlent sont reportées dans docs/gamedesign.md (section 16, Q107, et section 17).

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

Mesurés sur 100 parties par classe ; le rapport complet est à la section 6.

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

## 5. Réglages proposés

En attente de la décision de l'utilisateur (Q107) :

- le Baron pendu au niveau 20, le maximum de la Vallée, les Apôtres suivants restant 2 niveaux au-dessus du maximum de leur zone ;
- les monstres du Bourreau aux niveaux 13, 13 et 14, comme ceux des autres humains de fin de zone ;
- la Secousse (Sol, physique, 60, précision 100) au niveau 7 de la famille de la Mandragore, à la place du Cri strident ;
- les ressources rares : les laisser lentes à réunir, ou faire lâcher leur ressource aux monstres rares à chaque victoire.

## 6. Rapport de référence

100 parties par classe, du héros à la chute du Margrave de givre, jouées par le joueur automatique (crates/bersemon-equilibrage). Le niveau de l'équipe est la moyenne des monstres de plus haut niveau, équipe et camp réunis, autant qu'en compte l'équipe.

### Parties

| Classe | Terminées | Expéditions (médiane) | Défaites (moyenne) | Niveau final de l'équipe | Achats chez Malvoise | Argent à la fin | Équipe de 4 dès l'expédition | Objets forgés | Choix refusés |
|---|---|---|---|---|---|---|---|---|---|
| Guerrier | 100/100 | 55 | 11.5 | 37.8 | 2074 | 1674 | jamais | 2.0 | 0.0 |
| Mage | 100/100 | 61 | 13.4 | 38.3 | 2313 | 1575 | jamais | 2.4 | 0.0 |
| Templier | 100/100 | 64 | 10.0 | 37.5 | 1908 | 1920 | jamais | 2.1 | 0.0 |
| Assassin | 100/100 | 58 | 8.8 | 38.0 | 2051 | 1883 | jamais | 2.5 | 0.0 |

### Zones

Moyennes par expédition, sauf mention. L'argent, les filets et les potions sont la variation du sac et du coffre : une défaite fait perdre le sac.

| Zone (niveaux) | Expéditions par partie | Menées au bout | Défaites | Rentrées | Niveau à la première | Niveaux gagnés | Argent | Filets | Potions | Nouvelles espèces |
|---|---|---|---|---|---|---|---|---|---|---|
| Lisière du Bois-Sombre (3 à 7) | 13.3 | 36 % | 4 % | 60 % | 5.0 | 0.31 | 13 | 0.33 | -0.43 | 0.30 |
| Marais des Pendus (8 à 13) | 12.0 | 45 % | 17 % | 38 % | 9.1 | 0.60 | 30 | 0.46 | 0.65 | 0.36 |
| Vallée des Brumes (14 à 20) | 11.8 | 44 % | 33 % | 23 % | 16.3 | 0.58 | 30 | 1.55 | 1.71 | 0.32 |
| Abbaye des Cendres (21 à 27) | 11.8 | 30 % | 18 % | 52 % | 23.1 | 0.62 | 78 | 1.40 | 1.89 | 0.51 |
| Col du Givre-Noir (28 à 36) | 11.6 | 17 % | 20 % | 63 % | 30.5 | 0.64 | 107 | 2.20 | 2.28 | 0.58 |

### Zones par classe

Expéditions par partie, et entre parenthèses les défaites par partie.

| Classe | Lisière du Bois-Sombre | Marais des Pendus | Vallée des Brumes | Abbaye des Cendres | Col du Givre-Noir |
|---|---|---|---|---|---|
| Guerrier | 12.4 (0.2) | 10.2 (1.7) | 12.1 (5.3) | 10.8 (2.0) | 10.7 (2.3) |
| Mage | 13.4 (0.4) | 11.4 (1.5) | 11.6 (5.5) | 12.1 (2.8) | 13.1 (3.2) |
| Templier | 14.8 (0.6) | 15.3 (3.2) | 11.4 (2.6) | 13.1 (2.1) | 10.2 (1.4) |
| Assassin | 12.7 (0.7) | 11.0 (1.8) | 11.9 (2.0) | 11.2 (1.9) | 12.3 (2.4) |

### Combats de fin de zone

| Combat | Niveau maximal de la zone | Tentatives par partie | Victoires au premier essai | Niveau de l'équipe à la victoire |
|---|---|---|---|---|
| Lisière du Bois-Sombre : humain | 7 | 1.3 | 71 % | 7.5 |
| Marais des Pendus : humain | 13 | 2.6 | 21 % | 13.8 |
| Vallée des Brumes : Apôtre | 20 | 4.8 | 12 % | 20.8 |
| Abbaye des Cendres : Apôtre | 27 | 2.2 | 57 % | 29.5 |
| Abbaye des Cendres : humain | 27 | 1.8 | 52 % | 27.0 |
| Col du Givre-Noir : Apôtre | 36 | 2.0 | 65 % | 37.6 |
| Col du Givre-Noir : humain | 36 | 2.0 | 49 % | 35.4 |

### Victoires selon le niveau de l'équipe

Part des tentatives gagnées, selon l'écart entre le niveau moyen de l'équipe au départ de l'expédition et le niveau maximal de la zone ; entre parenthèses, le nombre de tentatives. Un Apôtre a 2 niveaux de plus que le maximum de sa zone.

| Combat | -4 ou moins | -3 à -2 | -1 à 0 | +1 à +2 | +3 ou plus |
|---|---|---|---|---|---|
| Lisière du Bois-Sombre : humain | - | 32 % (74) | 74 % (311) | 97 % (147) | 100 % (5) |
| Marais des Pendus : humain | 16 % (121) | 25 % (156) | 38 % (406) | 47 % (266) | 58 % (108) |
| Vallée des Brumes : Apôtre | 3 % (216) | 10 % (309) | 19 % (768) | 33 % (566) | 51 % (59) |
| Abbaye des Cendres : Apôtre | 0 % (6) | 12 % (17) | 40 % (131) | 45 % (396) | 52 % (324) |
| Abbaye des Cendres : humain | 43 % (152) | 40 % (97) | 56 % (261) | 66 % (174) | 87 % (39) |
| Col du Givre-Noir : Apôtre | 25 % (12) | 26 % (23) | 55 % (211) | 52 % (397) | 50 % (142) |
| Col du Givre-Noir : humain | 38 % (141) | 42 % (195) | 55 % (310) | 58 % (118) | 52 % (48) |

### Ressources trouvées

Moyenne par partie, perdues ou non dans une défaite, et part des parties qui en trouvent au moins une.

| Ressource | Par partie | Parties |
|---|---|---|
| Aile de chauve-souris | 4.0 | 94 % |
| Aile de fée noire | 8.0 | 100 % |
| Cendre de feu follet | 5.1 | 98 % |
| Crin de chimère | 0.1 | 14 % |
| Croc de basilic | 0.4 | 32 % |
| Croc de loup | 7.8 | 100 % |
| Dent de goule | 4.0 | 97 % |
| Fragment d'armure | 16.3 | 100 % |
| Os de squelette | 12.6 | 100 % |
| Pierre de troll | 2.8 | 85 % |
| Plume de corbeau | 10.1 | 100 % |
| Plume de harpie | 3.1 | 88 % |
| Poil de wendigo | 2.9 | 88 % |
| Écaille de vipère | 10.3 | 100 % |
| Écaille de vouivre | 0.2 | 20 % |
| Écaille de wyrm | 0.2 | 17 % |
| Éclat de bronze | 3.9 | 96 % |
| Éclat de gargouille | 7.3 | 99 % |

