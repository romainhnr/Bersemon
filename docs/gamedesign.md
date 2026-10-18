# Game Design Document - Bersemon

Bersemon est le nom du projet et du jeu.

Statut : document de référence du projet. Il décrit ce qui est décidé ; les points non tranchés sont regroupés dans la section 16 "Questions ouvertes et valeurs à définir", et l'historique des décisions dans la section 17.

Dernière mise à jour : 18/10/2026

## Sommaire

1. Concept
2. Cadre technique
3. Périmètre de la V1
4. Le héros
5. Les monstres
6. Histoire et démarrage
7. Progression
8. Boucle de gameplay
9. Expéditions manuelles
10. Combat
11. Obtention des monstres
12. Expéditions automatiques
13. Camp de base
14. Économie, marchands, objets et ressources
15. Multijoueur (après l'adaptation à Discord)
16. Questions ouvertes et valeurs à définir
17. Journal des décisions

## 1. Concept

### 1.1 Pitch

Jeu mutualisant deux concepts : un RPG de type Dark Fantasy et un système de capture et de combat de monstres inspiré de Pokémon. L'ambition est celle d'un Pokémon sombre et mature.

Exemple : un guerrier capture une petite salamandre, qui évolue en dragon de feu et avec laquelle il combat.

### 1.2 Univers

- Jeu propre à l'univers Dark Fantasy, avec ses codes.
- Inspirations : Berserk, Dark Souls.

### 1.3 Objectifs de design

L'objectif du jeu est de proposer des boucles de gameplay intéressantes et stimulantes, qui poussent le joueur à continuer de jouer en se sentant progresser petit à petit :

- en partant en expédition pour découvrir de nouveaux lieux et progresser face à une difficulté croissante ;
- au camp de base, pour s'entraîner et optimiser son personnage et ses ressources.

### 1.4 Principes de règles

- Aucune valeur de dégâts, de gain ou de perte n'est fixe : elles sont toujours tirées dans des fourchettes aléatoires.

## 2. Cadre technique

- Langage : Rust.
- Architecture : un noyau métier en Rust, indépendant de l'interface, utilisé d'abord par une interface console, puis par un bot Discord.
- Genre : jeu solo, textuel, au tour par tour. Ce format est plus simple et adapté à une console comme à Discord.
- Plateformes, dans l'ordre :
  1. Console.
  2. Bot Discord (inspiration possible : le bot Discord Koya).
- Le déroulement de la partie (camp, expéditions, combat et leurs suites) est commun à la console et au bot Discord, qui ne font que l'afficher et transmettre les choix du joueur.
- Sur Discord, les mêmes commandes fonctionnent dans un salon de serveur, avec des réponses visibles du seul joueur, et en message privé avec le bot.
- Sur Discord, une seule commande, /bersemon, ouvre un message qui pose la question en cours ; le joueur répond par des boutons, et par un formulaire pour un texte comme le nom du héros.
- Chaque compte Discord a une seule partie, la même sur tous les serveurs et en message privé. Elle est séparée de la partie de la console.
- Sur Discord, une expédition en cours attend le joueur sans limite de temps, jusqu'à son prochain /bersemon ; un redémarrage du bot le ramène à sa dernière sauvegarde, comme quitter la console (section 13.9).
- Sur Discord, le bot prévient le joueur par message privé du retour de ses expéditions automatiques (section 12.7).
- Pour les tests, une option au lancement du bot rend /bersemon visible de tout le salon. Seul l'auteur d'un message peut alors se servir de ses boutons ; les autres joueurs ouvrent leur propre partie par /bersemon. Sans l'option, les réponses restent visibles du seul joueur.
- Les informations importantes du combat sont mises en valeur : coups critiques, statuts, efficacité des attaques, KO et captures. Elles s'affichent en gras sur Discord et en rouge dans la console.
- Hébergement du bot : pour commencer, sur le PC de l'utilisateur, lancé à la main quand il veut que le bot soit en ligne. Le bot est privé : seul l'utilisateur peut l'inviter dans un serveur.
- Multijoueur : après l'adaptation à Discord (section 15).
- Hors périmètre pour le moment : version mobile et vrai jeu PC.
- Sauvegarde : dans un fichier JSON pour commencer, puis migration vers une base de données, probablement SQLite. Dans la V1, le fichier se trouve dans le dossier de données de l'utilisateur, %APPDATA%\Bersemon sous Windows (section 13.9). Sur Discord, un fichier JSON par compte Discord, au même format, en attendant SQLite.

## 3. Périmètre de la V1

### 3.1 Inclus

- Interface console.
- Création du héros : un nom et une classe, Guerrier ou Mage (section 4).
- Choix du premier monstre parmi 3, au niveau 5.
- Une dizaine de monstres sauvages.
- 2 zones d'expédition manuelle, une facile puis une difficile (section 9).
- Potions de soin (section 14.4) et codex simple (section 5.6).
- Un Apôtre à traquer dans la zone difficile, au bout de la V1 (sections 6 et 9.4).
- Combat au tour par tour (format Pokémon, section 10), avec les statuts poison et paralysie, capture au filet et fuite.
- Camp de base réduit à l'essentiel : soin, gestion de l'équipe (3 monstres avec soi, 30 au camp) et sauvegarde.
- Valeurs internes et shiny des monstres (section 5.1).

### 3.2 Hors V1

- Interface Discord et multijoueur.
- Combat du héros (option hybride, section 4.2).
- Expéditions automatiques.
- Marchands, location de monstres, fabrication, améliorations, automatisation, pêche et services rares du camp.
- Les types Stellaire, Obscur, Lumière et Son, traités dans un second temps (section 5.2).
- Habiletés et évolution des monstres (sections 5.1 et 5.4).
- Effet du rôle soutien ou offensif (section 5.1).
- Classes Templier et Assassin ; niveau, stats et équipement du héros ; actions de soutien des classes (section 4).
- Combats contre des humains (sections 10.1 et 11.3).

### 3.3 À trancher pour la V1

- Aucun point du périmètre ne reste à trancher.

## 4. Le héros

### 4.1 Création

- Le joueur crée un héros de base à partir d'une classe : Guerrier, Mage, Templier, Assassin, etc.
- Chaque héros possède :
  - ses stats ;
  - son équipement : armes, armure, x talismans (nombre à définir, voir Q6) ;
  - ses monstres.
- Dans la V1, le héros se résume à un nom, choisi librement par le joueur (de 1 à 20 caractères), et à une classe, Guerrier ou Mage. Il n'a ni niveau, ni stats, ni équipement.
- Après la V1, le Templier rejoint les classes jouables : Guerrier, Mage ou Templier. Chaque classe a son monstre de départ attitré (section 6).

### 4.2 Rôle en combat

- Pour le moment, seuls les monstres combattent, un à un. Le héros ne combat pas et n'a pas de PV en combat.
- La classe et l'équipement du héros donnent des bonus aux monstres et quelques actions de soutien, par exemple des ordres ou l'utilisation d'objets.
- Bonus de classe : en combat, les monstres du joueur ont +10 % dans une stat, arrondi à l'entier inférieur. Guerrier : Attaque. Mage : Attaque spéciale. Après la V1, Templier : Défense et Défense spéciale, à chacune +10 % ; Assassin : Vitesse, plus tard. Les actions de soutien viendront après la V1.
- À terme, option hybride envisagée : le héros combat lui-même uniquement lorsque tous ses monstres sont KO.

## 5. Les monstres

### 5.1 Caractéristiques

Chaque monstre possède :

- un niveau, de 1 à 100 (section 7) ;
- un à trois types ;
- un rôle : soutien ou offensif, le soutien couvrant aussi les profils défensifs. Ce rôle s'ajoute aux types et il est propre à l'espèce ; il n'a pas d'effet dans la V1 ;
- six stats, comme dans Pokémon : PV, Attaque, Défense, Attaque spéciale, Défense spéciale, Vitesse (section 5.8) ;
- de 1 à 4 attaques distinctes, chacune d'un seul type, physique ou spéciale (section 10.4) ;
- des valeurs internes aléatoires, de 0 à 31 pour chaque stat, qui influent sur ses stats (similaires aux IV de Pokémon), ce qui rend chaque monstre unique. Ces valeurs sont visibles dans le résumé du monstre ;
- une habileté unique (équivalent des talents de Pokémon), qui a un effet secondaire lors des expéditions, des combats, etc. Les habiletés sont hors V1 ;
- une faible chance d'être "shiny", 1 sur 512 pour un monstre sauvage comme pour le monstre de départ : sa couleur change, sans incidence sur les stats. Dans la console, un shiny est signalé par la mention "(shiny)" après son nom ;
- un sprite (une image), pour rendre le jeu plus vivant. La console de la V1 n'affiche pas d'image, et le jeu reste textuel sur Discord pour l'instant : les sprites viendront plus tard, une fois décidées la source des images et leurs droits.

### 5.2 Types

- Les types reprennent ceux de Pokémon, avec l'ajout de nouveaux types.
- Liste des types : Normal, Combat, Vol, Poison, Sol, Roche, Insecte, Spectre, Acier, Feu, Eau, Plante, Électrique, Psychique, Glace, Dragon, Ténèbres, Féérique, Stellaire, Obscur, Lumière, Son.
- Table d'efficacité : celle de Pokémon, avec une modification : le type Glace résiste au type Eau.
- Une attaque a un seul type. Contre un monstre à 2 ou 3 types, les coefficients de chacun de ses types se multiplient, puis le résultat est borné entre x0,25 et x4. Une immunité (x0) l'emporte sur tout le reste. Exemple : une attaque Roche contre un monstre Feu, Vol et Glace vaudrait x8 ; elle inflige x4.
- Les types Stellaire, Obscur, Lumière et Son sont traités dans un second temps : leurs relations d'efficacité restent à définir et ils ne font pas partie de la V1 (voir Q5).

### 5.3 Bestiaire

- Les créatures ne reprennent pas directement les Pokémon, qui ne correspondent pas à l'univers. Elles s'en inspirent pour être adaptées à un univers assez sombre et médiéval.
- Exemples de familles : dragons, fées, elfes, créatures mythologiques ou folkloriques, animaux actuels ou disparus, objets possédés, etc.
- Volume initial : une dizaine de monstres, augmenté au fur et à mesure.
- Le contenu de la V1 (espèces, attaques, zones et textes) est décrit dans docs/contenu-v1.md, validé le 17/10/2026.

### 5.4 Évolution

- Un monstre peut évoluer ou non. Conditions possibles (comme dans Pokémon) :
  - l'utilisation d'un objet ;
  - un certain niveau atteint ;
  - une situation ou un lieu particulier.
- Une évolution augmente les stats, peut modifier le ou les types et change la forme du monstre.

### 5.5 Personnalisation

Le joueur peut :

- renommer son monstre : un surnom de 1 à 20 caractères, affiché à la place du nom de l'espèce ;
- ajuster ses attaques, au camp : réapprendre une attaque de son espèce ou en apprendre une nouvelle grâce à une capsule d'attaque (section 13.10) ;
- ajuster ses statistiques à l'aide d'objets ;
- consulter le codex, par exemple pour savoir quand il évolue.

### 5.6 Codex

- Le codex répertorie tous les monstres du jeu (comme le Pokédex de Pokémon).
- Une entrée est déverrouillée partiellement à la rencontre d'un monstre, et entièrement à sa capture.
- Dans la V1, le codex se consulte au camp. Une espèce rencontrée en expédition montre son nom et ses types ; une espèce capturée montre aussi ses stats de base et la liste des attaques qu'elle apprend. Le monstre de départ compte comme capturé.
- La fiche d'une espèce capturée rappelle aussi ses types.

### 5.7 KO (pas de mort définitive)

- Un monstre dont les PV tombent à 0 est KO, comme dans Pokémon. Il est soigné automatiquement au retour au camp.
- Une défaite totale, lorsque tous les monstres engagés sont KO, n'entraîne pas de mort : les monstres restent simplement KO, en expédition manuelle (section 9.3) comme en expédition automatique (section 12.3).
- Il n'existe pas de mort définitive pour le moment. Elle pourra être introduite plus tard, avec une ressource de réanimation (section 13.7).

### 5.8 Stats

- Six stats : PV, Attaque, Défense, Attaque spéciale, Défense spéciale, Vitesse.
- Une attaque physique oppose l'Attaque de l'attaquant à la Défense du défenseur ; une attaque spéciale oppose l'Attaque spéciale à la Défense spéciale.
- Chaque espèce a ses stats de base. Les stats d'un monstre en découlent par la formule de Pokémon, sans EV ni nature, en divisions entières :
  - PV = (2 x base + valeur interne) x niveau / 100 + niveau + 10 ;
  - autres stats = (2 x base + valeur interne) x niveau / 100 + 5.
- Les valeurs internes vont de 0 à 31. Elles sont tirées uniformément, une par stat, à la création du monstre.
- Exemple pour une stat de base 50 : au niveau 100, avec une valeur interne de 0, 210 PV ou 105 pour une autre stat ; avec une valeur interne de 31, 241 PV ou 136.

## 6. Histoire et démarrage

- Une histoire, simple pour le moment.
- Un petit tutoriel de départ.
- Fil de l'histoire, inspiré de Berserk : le héros veut se venger du Démiurge, maître des Apôtres, de puissants monstres. Pour l'atteindre, il traque les Apôtres un par un, de zone en zone : dans chaque zone, il réunit des indices sur la cachette d'un Apôtre, s'en approche peu à peu, puis l'affronte avant de passer au suivant (section 9.4).
- Dans la V1, un seul Apôtre est à vaincre, dans la zone difficile. Le Démiurge est annoncé dès l'introduction, mais il ne sera affronté que dans une version suivante. Après la victoire contre l'Apôtre, un court épilogue annonce la suite de la traque, puis le joueur peut continuer à explorer, capturer et compléter son codex.
- Dans la V1, après le choix de la classe, l'histoire amène le joueur à choisir son premier monstre parmi 3. Ce monstre est au niveau 5. Ses valeurs internes sont tirées au hasard et il a, comme un monstre sauvage, 1 chance sur 512 d'être shiny.
- Après la V1, le premier monstre n'est plus choisi : chaque classe a son monstre attitré, qui reprend son profil. Guerrier : Crapaud, offensif en attaque physique. Mage : Salamandre, offensive en attaque spéciale. Templier : Mandragore, défensive. Il reste au niveau 5, avec des valeurs internes tirées au hasard et 1 chance sur 512 d'être shiny. La fin de l'introduction, qui invitait à choisir parmi trois cages, est à réécrire (Q67).
- Dans la V1 : une courte introduction d'ambiance après le choix de la classe, jusqu'au choix du premier monstre ; un tutoriel fait de quelques conseils, chacun montré une seule fois par partie, au premier combat, au premier arrêt entre deux étapes et au premier retour au camp. Les textes sont proposés dans docs/contenu-v1.md.

## 7. Progression

- Les monstres gagnent de l'expérience et montent de niveau, du niveau 1 au niveau 100.
- Le monstre de départ est au niveau 5.
- Courbe d'expérience, la même pour toutes les espèces : il faut n x n x n XP au total pour atteindre le niveau n (125 au niveau 5, 216 au niveau 6, 1 000 000 au niveau 100). Un monstre créé au niveau n commence avec n x n x n XP. Au niveau 100, il ne gagne plus d'XP.
- À chaque niveau gagné, les stats sont recalculées et les PV actuels augmentent d'autant que les PV max.
- XP rapportée par un monstre sauvage vaincu, c'est-à-dire mis KO ou capturé : rendement de son espèce x son niveau / 7, multiplié par un pourcentage tiré de 90 % à 110 %, en divisions entières. Exemple : un monstre de niveau 5 et de rendement 64 rapporte de 40 à 49 XP.
- Chaque monstre de l'équipe qui a été actif face au monstre vaincu et qui n'est pas KO à la fin du combat reçoit cette XP en entier.
- L'XP est versée à la fin du combat, quelle que soit son issue ; les montées de niveau et l'apprentissage d'attaques se font donc après le combat, jamais pendant.
- Chaque espèce a une liste d'attaques apprises à des niveaux donnés. En atteignant un de ces niveaux, le monstre apprend l'attaque ; s'il en connaît déjà 4, le joueur choisit l'attaque à oublier ou renonce à la nouvelle. Un monstre créé à un niveau donné connaît les 4 dernières attaques de sa liste jusqu'à ce niveau.
- Une expédition automatique réussie rapporte aussi de l'XP (section 12.6).
- Le héros n'a pas de niveau dans la V1 (section 4.1).

## 8. Boucle de gameplay

Le jeu alterne entre deux phases :

- le camp de base, avant et après chaque expédition (section 13) ;
- les expéditions, manuelles (section 9) ou automatiques (section 12).

## 9. Expéditions manuelles

### 9.1 Carte du monde

- Pour rester dans le style propre à l'univers, le monde s'explore sur une carte qui se dévoile au fil des expéditions lancées par le joueur.
- Le joueur part en expédition vers un lieu choisi, avec ses monstres.
- Chaque zone a sa propre difficulté, visible sur la carte.
- La V1 compte 2 zones : une zone facile, accessible dès le départ, et une zone difficile, qui apparaît sur la carte avec le premier indice sur l'Apôtre, trouvé au bout d'une première expédition menée jusqu'à sa dernière étape dans la zone facile (section 9.4).
- Après la V1, chaque zone s'ouvrira avec la défaite de l'Apôtre de la zone précédente.

### 9.2 Événements

Lors d'une expédition, les événements suivants peuvent se produire :

- un combat contre un ou plusieurs monstres, ou contre un humain (section 10) ;
- la rencontre d'un marchand (section 14.2) ;
- rarement, la rencontre d'un monstre rare. Selon son espèce, il attaque (combat, capture possible) ou non (le joueur peut alors le prendre gratuitement ou non) ;
- rarement, la découverte d'un coffre, que le joueur peut ouvrir pour obtenir des ressources offertes.

Déroulement dans la V1 :

- Une expédition compte de 4 à 6 étapes, nombre tiré au départ et inconnu du joueur.
- Chaque étape tire un événement : combat contre un groupe sauvage (80 %), coffre (15 %) ou monstre rare (5 %). Le marchand est hors V1.
- Un groupe sauvage compte de 1 à 2 monstres dans la zone facile, de 1 à 3 dans la zone difficile ; la taille est tirée à chaque combat.
- Un coffre contient 1 à 2 potions (section 14.4).
- Entre deux étapes, le joueur continue, rentre au camp en gardant ses gains, utilise des potions ou change l'ordre de son équipe ; les échanges avec le camp restent impossibles. Les PV et les statuts sont conservés d'une étape à l'autre.
- Après la dernière étape, le joueur rentre au camp et la zone compte comme explorée.

### 9.3 Défaite

Si tous les monstres du joueur sont KO pendant une expédition manuelle :

- le joueur retourne automatiquement au camp ;
- il perd les objets qu'il avait sur lui, dont toutes ses potions ;
- ses monstres restent KO et sont soignés au camp (section 5.7) ;
- c'est aussi une défaite si son dernier monstre et le dernier monstre sauvage tombent KO au même tour ;
- l'expédition ne compte pas comme menée jusqu'au bout.

### 9.4 Traque d'un Apôtre

- Chaque expédition menée jusqu'à sa dernière étape révèle un indice sur la cachette de l'Apôtre traqué : un court texte d'histoire.
- Dans la V1, la piste commence dans la zone facile, qui n'a pas d'Apôtre : le premier indice, au bout de la première expédition complète dans la zone facile, désigne la zone difficile et l'ouvre. Les indices 2 et 3 viennent des expéditions menées au bout dans la zone difficile.
- Au 3e indice, l'antre est localisé : l'expédition suivante dans la zone s'achève par le combat contre l'Apôtre, à sa dernière étape et à la place de l'événement tiré. Rentrer au camp avant cette étape repousse le combat à l'expédition suivante.
- Combat contre un Apôtre : il combat seul, 2 niveaux au-dessus du niveau maximal de sa zone, avec des stats de base bien plus hautes que celles des monstres communs (environ 480). Ses valeurs internes sont fixes, au maximum : 31 pour chaque stat. Comme un monstre sauvage, il a 1 chance sur 512 d'être shiny. La fuite est impossible, et le filet aussi : on ne lance pas de filet sur un Apôtre pendant le combat.
- Une défaite contre l'Apôtre est une défaite ordinaire (section 9.3) ; les indices restent acquis et l'on peut retenter.
- Après la victoire, l'Apôtre, mis KO, peut être capturé, à coup sûr, comme tout monstre KO à la fin d'un combat gagné (section 11.2) : c'est la seule façon de le capturer. Capturé ou non, il ne réapparaît plus.

## 10. Combat

### 10.1 Principes

- Combat au tour par tour.
- Adversaires possibles : un ou plusieurs monstres, ou un humain (seul ou avec ses monstres, selon la difficulté).
- L'adversaire est contrôlé par une IA (l'ordinateur, et non un vrai joueur). Dans la V1, l'IA d'un monstre sauvage choisit son attaque au hasard.

### 10.2 Format

Le format reprend celui des combats de Pokémon, comme base de départ qui pourra être ajustée pour se différencier :

- un monstre actif de chaque côté, les autres en réserve ; ils combattent un à un, et les monstres d'un groupe sauvage entrent dans l'ordre de la rencontre ;
- possibilité de changer de monstre actif : un changement volontaire est l'action du tour, et l'adversaire attaque le nouveau monstre ; remplacer un monstre KO est gratuit et se fait avant le tour suivant ;
- ordre d'action déterminé par la vitesse ; en cas d'égalité, tirage au sort à chaque tour ;
- efficacité des attaques selon la table des types (section 5.2) ;
- effets de statut ;
- dégâts toujours tirés dans une fourchette aléatoire (section 1.4).

### 10.3 Fuite

- Le joueur peut fuir, et la fuite réussit toujours, sauf face à un Apôtre (section 9.4). En partant, le monstre actif du joueur subit une attaque du monstre adverse, résolue normalement (section 10.4), mais qui lui laisse au moins 1 PV : une fuite ne met jamais KO.

### 10.4 Résolution d'une attaque

- Chaque attaque a un type, une catégorie (physique ou spéciale), une puissance et une précision.
- Précision : une attaque de précision inférieure à 100 % peut rater ; un tirage en décide à chaque utilisation.
- Coup critique : 1 chance sur 24 pour chaque attaque qui touche ; dégâts x1,5.
- Bonus de type : x1,5 quand l'attaque a l'un des types du monstre qui l'utilise.
- Dégâts, selon la formule de Pokémon, en divisions entières de gauche à droite :
  - base = (2 x niveau / 5 + 2) x puissance x A / D / 50 + 2, où A et D sont l'Attaque de l'attaquant et la Défense du défenseur pour une attaque physique, l'Attaque spéciale et la Défense spéciale pour une attaque spéciale ;
  - puis, dans cet ordre et en arrondissant à l'entier inférieur à chaque étape : x1,5 en cas de coup critique, x un pourcentage tiré de 85 % à 100 %, x1,5 pour le bonus de type, x l'efficacité (section 5.2) ;
  - une attaque qui n'est pas sans effet inflige au moins 1 dégât.
- Exemple : au niveau 5, une attaque de puissance 40, quand l'Attaque de l'attaquant égale la Défense du défenseur, inflige de 4 à 5 dégâts sans autre modificateur.
- Le nombre d'utilisations des attaques n'est pas limité dans la V1 (pas d'équivalent des PP de Pokémon).
- Une attaque peut avoir un effet secondaire : une chance, en pourcentage, d'infliger un statut à la cible quand elle la touche sans la mettre KO (section 10.5).

### 10.5 Effets de statut

- La V1 compte deux statuts, avec les règles actuelles de Pokémon :
  - poison : le monstre perd 1/8 de ses PV max à la fin de chaque tour (au moins 1 PV) ; les types Poison et Acier y sont immunisés ;
  - paralysie : la vitesse du monstre est divisée par 2, et il a 1 chance sur 4 de ne pas pouvoir attaquer à chaque tour ; le type Électrique y est immunisé.
- Un monstre a au plus un statut à la fois : un nouveau statut ne remplace pas l'ancien.
- Un statut dure après le combat, pendant toute l'expédition, et il est guéri au retour au camp (section 13.1).

## 11. Obtention des monstres

### 11.1 Moyens d'obtention

- Le premier monstre, au démarrage (section 6).
- La capture d'un monstre lors d'un combat (section 11.2).
- La capture d'un monstre d'un humain contrôlé par l'IA, après l'avoir vaincu (section 11.3).
- L'achat ou la location auprès d'un marchand (section 14.2).
- La rencontre d'un monstre rare qui n'attaque pas, pris gratuitement (section 9.2).
- La pêche au camp de base, pour les monstres aquatiques (section 13.8).
- L'échange ou le don entre joueurs, avec le multijoueur (section 15).

### 11.2 Capture lors d'un combat

- Le joueur peut capturer un monstre mis KO ou affaibli lors d'un combat, en lançant un filet de capture.
- Chances de réussite :
  - monstre KO : capture garantie ;
  - monstre non KO : chances inversement proportionnelles au nombre de PV restants du monstre (moins il a de PV, plus la capture est probable) ;
  - plus le monstre est faible par rapport au joueur, plus la capture est facilitée ;
  - chaque niveau d'amélioration du filet augmente les chances de capture (section 13.5).
- Calcul pour un monstre non KO : chance = PV perdus / PV max x facteur, bornée à 100 %. Le facteur vaut le niveau du monstre actif du joueur divisé par celui du monstre sauvage, borné entre x0,5 et x2. Un monstre à PV pleins ne peut donc pas être capturé. Exemple : à niveau égal, un monstre à 2 PV sur 20 est capturé 9 fois sur 10.
- Lancer un filet sur le monstre adverse actif est l'action du tour : si la capture échoue, l'adversaire agit. Un monstre sauvage mis KO peut être capturé à la fin du combat, si le joueur l'emporte.
- Plusieurs monstres peuvent être capturés au cours d'un même combat : chaque monstre d'un groupe sauvage peut l'être, au filet pendant le combat ou, s'il est KO, à la fin d'un combat gagné. Un Apôtre ne se capture que KO, à la fin du combat (section 9.4).
- Les filets de capture sont illimités dans la V1.
- Chaque monstre capturé rejoint l'équipe s'il y reste une place, sinon le camp. Si le camp est plein lui aussi, le joueur relâche un monstre de son choix, du camp ou de l'équipe, et la capture prend sa place ; il peut aussi relâcher la capture elle-même, ce qui revient à y renoncer.

### 11.3 Capture après un combat contre un humain contrôlé par l'IA

- Après avoir vaincu un humain qui combat avec ses monstres, le joueur peut capturer le monstre de son choix parmi ceux de l'adversaire.
- La capture est garantie, puisque ses monstres sont forcément KO.
- Cette règle ne s'applique pas aux combats contre de vrais joueurs (section 15).

## 12. Expéditions automatiques

### 12.1 Principe

- Le joueur peut envoyer de 1 à 3 de ses monstres seuls en expédition, uniquement dans une zone déjà explorée manuellement, c'est-à-dire où il a mené une expédition jusqu'à sa dernière étape (section 9.2). Le départ se fait au camp.
- Les monstres envoyés viennent de l'équipe ou du camp. L'équipe garde toujours au moins un monstre (section 13.2) : il faut donc avoir au moins 2 monstres pour lancer une expédition automatique.
- Toute l'équipe peut partir, à condition qu'au moins un monstre reste au camp : le premier monstre du camp rejoint alors aussitôt l'équipe, qui n'est jamais vide, et le joueur peut repartir en expédition manuelle. Un bouton prépare directement le départ de toute l'équipe.
- Pendant l'expédition, les monstres envoyés sont inutilisables : on ne peut ni les relâcher, ni les renommer, ni les échanger. Ils gardent leur place au camp, dont la limite de 30 monstres les compte, et y reviennent.
- Le joueur peut partir en expédition manuelle avec ses autres monstres, y compris dans une zone où une expédition automatique est en cours.
- Le nombre d'expéditions automatiques simultanées n'est pas limité, mais une seule expédition automatique est possible par zone.
- Une expédition automatique n'a aucun effet sur la traque : elle ne donne ni indice ni combat contre un Apôtre, et ne rend pas une zone explorée. Elle ne rencontre ni ne capture aucun monstre : le codex ne change pas.

### 12.2 Durée

- La durée s'écoule en temps réel, y compris quand le jeu est fermé ou le bot éteint : elle se compte à partir de l'heure du départ.
- La durée est fixe, et non tirée au hasard (la section 1.4 vise les dégâts et les gains), pour être connue avant le départ.
- Chaque zone a une durée de base, pour un seul monstre (section 12.8) : plus la zone est difficile, plus l'expédition est longue.
- Chaque monstre envoyé en plus du premier réduit la durée de 25 % de la durée de base : 100 % avec 1 monstre, 75 % avec 2, 50 % avec 3.

### 12.3 Dégâts et résultat

- Les monstres partent avec tous leurs PV, puisque tous les monstres sont soignés au camp (section 13.1).
- Chaque zone a une fourchette de dégâts (section 12.8). Un total de dégâts est tiré dans cette fourchette, puis réparti à parts égales entre les monstres envoyés, en divisions entières : le reste de la division est perdu.
- Un monstre dont la part atteint ses PV tombe KO. Ce qui dépasse ses PV est réparti à parts égales entre les monstres encore debout, et ainsi de suite. L'expédition échoue donc à peu près quand le tirage atteint le total des PV des monstres envoyés : un monstre faible aide peu, un monstre robuste aide beaucoup.
- Exemple : une Salamandre de 26 PV et un Loup de 16 PV. Avec un tirage de 40, chacun reçoit 20 ; le Loup tombe KO et les 4 dégâts en trop passent à la Salamandre, qui en subit 24 et reste debout : c'est une réussite. Avec un tirage de 50, chacun reçoit 25 ; le Loup tombe KO et ses 9 dégâts en trop portent ceux de la Salamandre à 34 : elle tombe KO à son tour, c'est un échec.
- Les protections réduiront les dégâts subis ; elles sont reportées (section 12.5).
- Succès : au moins un monstre n'est pas KO. L'expédition rapporte de l'XP et des potions (section 12.6).
- Échec : tous les monstres envoyés sont KO. L'expédition ne rapporte rien (section 5.7).
- Le résultat n'est tiré et connu qu'au moment où le joueur accueille l'expédition (section 12.7). Au retour, tous les monstres sont soignés, comme au camp (section 13.1).

### 12.4 Information avant le départ

- Avant de lancer l'expédition, le joueur voit la difficulté de la zone (ses niveaux, section 9.1), sa fourchette de dégâts, la durée et les chances de réussite.
- Les chances de réussite sont calculées exactement à partir de la fourchette de dégâts, du nombre de monstres envoyés et de leurs PV, et plus tard des protections.

### 12.5 Protection

- Des objets peuvent protéger un monstre.
- Certains monstres de soutien peuvent protéger un autre monstre.
- Exemple :
  - une troupe de 3 monstres (a, b, c) est envoyée vers le lieu A ;
  - X monstres ou objets peuvent assurer une protection efficace à 100% à 1 monstre important ;
  - Y monstres ou objets assurent une protection de 75%.
- Les protections sont reportées : tant qu'il n'existe ni objet de protection ni effet du rôle (Q19), aucune protection ne s'applique (Q66).

### 12.6 Récompenses

- Chaque monstre qui n'est pas KO au retour d'une expédition réussie reçoit en entier le même gain d'XP, tiré une fois dans la fourchette de la zone (section 12.8) ; les monstres KO ne gagnent rien, comme en combat (section 7). Ce gain vaut environ la moitié de ce que rapporte une expédition manuelle dans la même zone.
- Un nombre de potions est tiré dans la fourchette de la zone et ajouté à celles du joueur (section 14.4).
- Les ressources et les autres objets (section 14.3) viendront avec l'économie (sections 13.4 et 14).

### 12.7 Retour, accueil et rappel

- Une expédition rentrée attend au camp que le joueur l'accueille. S'il est en expédition manuelle, il l'accueille à son retour au camp. Quand plusieurs expéditions sont rentrées, il les accueille une par une, chacune avec son bilan.
- À l'accueil, le joueur voit le bilan : dégâts subis, monstres KO, XP et potions. Un monstre qui monte de niveau et doit apprendre une attaque alors qu'il en connaît déjà 4 choisit comme après un combat (section 7). Les monstres reviennent au camp, soignés.
- La partie est sauvegardée dès l'accueil, pour que quitter le jeu ne permette pas de tirer de nouveau le résultat. Si le jeu s'arrête avant que le joueur ait choisi les attaques à oublier, ces choix lui sont reproposés à la reprise.
- Au camp, le joueur peut rappeler une expédition avant son terme, après confirmation : les monstres rentrent aussitôt, sans dégâts ni récompense. Une expédition déjà rentrée ne se rappelle plus, elle s'accueille.
- Sur Discord, le bot envoie un message privé au joueur à l'heure du retour, s'il est en ligne ; s'il était éteint, il prévient à son démarrage des expéditions rentrées entre-temps. Chaque retour n'est annoncé qu'une fois, même si le bot redémarre avant que le joueur l'accueille (Q65). Le joueur accueille ensuite l'expédition par /bersemon. La console affiche les expéditions rentrées au camp.

### 12.8 Valeurs par zone

| Zone | Durée de base | Dégâts | XP par monstre non KO | Potions |
|---|---|---|---|---|
| Lisière du Bois-Sombre | 1 h | 15 à 60 | 100 à 200 | 0 à 1 |
| Marais des Pendus | 2 h | 40 à 120 | 300 à 600 | 1 à 2 |

Repères, avec des valeurs internes de 15 : dans la Lisière, une Salamandre de niveau 8 seule réussit 23 fois sur 100, 71 fois avec un Loup de niveau 6, à coup sûr avec en plus une Vipère de niveau 5. Dans le Marais, une Salamandre de niveau 12 seule échoue toujours ; elle réussit 34 fois sur 100 avec un Crapaud de niveau 11, et 77 fois avec en plus une Mandragore de niveau 10. À vérifier en jouant.

## 13. Camp de base

### 13.1 Repos

- Le joueur se trouve au camp de base avant et après chaque expédition.
- Ses monstres y sont soignés automatiquement, y compris ceux qui sont KO.

### 13.2 Gestion des monstres

- Le joueur peut laisser certains monstres en sécurité au camp et en emmener d'autres avec lui.
- Limites de départ : 3 monstres avec le joueur et 30 monstres au camp. Ces limites peuvent être augmentées (section 13.4). Les monstres partis en expédition automatique comptent dans la limite du camp (section 12.1).
- Le joueur part en expédition avec 1 à 3 monstres. L'équipe garde toujours au moins un monstre, même au camp.
- Au camp, le joueur peut :
  - échanger des monstres entre son équipe et le camp ;
  - choisir l'ordre de son équipe : le premier monstre en état de combattre commence chaque combat ;
  - relâcher un monstre, définitivement et après confirmation, sauf son dernier monstre ;
  - renommer un monstre (section 5.5) ;
  - gérer les attaques d'un monstre (section 13.10) ;
  - une fois le marchand au camp, lui revendre un monstre (section 14.2).

### 13.3 Équipement et stockage

- Le joueur peut ajuster et modifier son équipement.
- L'inventaire étant limité en place, le joueur dispose d'un coffre pour stocker ses objets.

### 13.4 Améliorations

Avec des ressources trouvées en expédition (bois, métal, tissu, minerai rare, etc.), le joueur peut améliorer :

- son sac (inventaire) ;
- le nombre de monstres pouvant l'accompagner ;
- le nombre de monstres pouvant rester au camp ;
- etc.

### 13.5 Fabrication

- Avec les ressources obtenues, le joueur peut fabriquer des objets : filets de capture, armes, armures, talismans.
- Il peut aussi améliorer ses filets de capture : chaque niveau d'amélioration augmente les chances de capture.

### 13.6 Automatisation

Le joueur peut construire des machines pour automatiser certaines tâches, par exemple :

- récolter des ressources ;
- entraîner automatiquement ses monstres au camp ;
- etc.

### 13.7 Services rares

- Réanimer un monstre mort, contre une ressource très rare : en suspens tant qu'il n'existe pas de mort définitive (section 5.7).
- Rendre un de ses monstres shiny, contre un objet extrêmement rare.

### 13.8 Pêche

- Le joueur peut pêcher pour trouver des monstres aquatiques ou des objets plus ou moins rares.

### 13.9 Sauvegarde (V1)

- Un seul emplacement : une seule partie en cours, reprise au lancement du jeu. Commencer une nouvelle partie remplace l'ancienne, après confirmation.
- La sauvegarde est automatique : à la création de la partie, à chaque retour au camp et après chaque action de gestion au camp.
- On ne sauvegarde qu'au camp : quitter le jeu en pleine expédition ramène à la dernière sauvegarde.
- Une sauvegarde illisible, ou qui cite une espèce ou une attaque disparue du jeu, est refusée : le jeu l'explique et propose une nouvelle partie, sans rien effacer sans l'accord du joueur.
- La sauvegarde garde l'identifiant des espèces, des attaques et des zones, pas leurs données : un rééquilibrage s'applique aux monstres déjà capturés.
- Elle garde aussi l'avancée de la traque (indices trouvés, Apôtres vaincus) et les conseils du tutoriel déjà montrés.
- Après la V1, elle garde les expéditions automatiques en cours : zone, monstres envoyés, heures de départ et de retour (section 12). Elle garde aussi les attaques qui restent à choisir après l'accueil d'une expédition (section 12.7). Le départ, le rappel et l'accueil d'une expédition automatique sont des actions au camp, suivies d'une sauvegarde.

### 13.10 Attaques des monstres

- Au camp, un monstre peut réapprendre gratuitement toute attaque que son espèce apprend jusqu'à son niveau, y compris celles qu'il a oubliées ou refusées. S'il en connaît déjà 4, il en oublie une (section 7).
- Une capsule d'attaque enseigne une attaque précise à tout monstre qui a le type de cette attaque, de la même façon. Elle n'est pas consommée : une fois obtenue, elle sert autant de fois qu'on veut.
- Les capsules se trouvent rarement en expédition, ou s'achètent chez le marchand (section 14.2). Leur liste et leur rareté restent à définir (Q69).

## 14. Économie, marchands, objets et ressources

### 14.1 Monnaie

- La monnaie du jeu est l'argent : le même mot désigne le métal précieux et la monnaie.
- Après la V1, l'argent vient de la revente des monstres au marchand, des coffres des expéditions manuelles et des expéditions automatiques réussies. Les montants restent à définir (Q68).

### 14.2 Marchands

Les marchands sont rencontrés en expédition. Ils proposent :

- à l'achat : des objets (ressources, soins, boosters de stats, filets de capture, talismans, armes, armures, etc.) ;
- à l'achat ou à la location : des monstres. Une location coûte un prix donné et vaut pour un nombre donné d'expéditions.

Le joueur peut aussi vendre ses objets aux marchands contre de l'argent.

Premier marchand, après la V1 :

- Tant qu'aucun marchand n'est au camp, chaque étape d'une expédition manuelle peut tirer la rencontre d'un marchand, à la place d'un combat (environ 5 %, Q70).
- Le marchand rencontré se réfugie au camp et y reste, toujours accessible ; il ne se rencontre plus en expédition.
- Au camp, il vend des potions et des capsules d'attaque (section 13.10), et rachète les monstres capturés contre de l'argent. Revendre un monstre est définitif et suit les règles du relâcher : confirmation, et jamais le dernier monstre de l'équipe.
- Tuer un monstre pour en tirer des ressources est envisagé, mais reporté tant qu'aucune ressource n'a d'usage (Q71).

### 14.3 Objets et ressources cités dans ce document

- Ressources de fabrication et d'amélioration : bois, métal, tissu, minerai rare, etc.
- Consommables : soins, boosters de stats.
- Objets permettant d'ajuster les statistiques d'un monstre.
- Filets de capture : achetables, fabricables, améliorables.
- Équipement du héros : armes, armures, talismans.
- Argent (monnaie).
- Objets d'évolution.
- Objets de protection pour les expéditions automatiques.
- Capsules d'attaque, qui enseignent une attaque (section 13.10).
- Ressource très rare permettant de réanimer un monstre (en suspens, section 5.7).
- Objet extrêmement rare permettant de rendre un monstre shiny.

### 14.4 Potions (V1)

- Seul objet de la V1. Une potion rend à un monstre de l'équipe un pourcentage de ses PV max tiré de 40 % à 60 %, arrondi à l'entier inférieur, sans dépasser ses PV max. Elle ne relève pas un monstre KO et ne guérit pas les statuts. Sur un monstre qui a tous ses PV, le jeu demande confirmation : la potion serait consommée sans effet.
- Elle s'utilise en combat, comme action du tour (l'adversaire agit ensuite), ou entre deux étapes d'une expédition.
- Dans la V1, les potions se trouvent dans les coffres ; après la V1, les expéditions automatiques réussies en rapportent aussi (section 12.6), et le marchand en vend au camp (section 14.2). Il n'y a ni limite de sac ni coffre de stockage au camp : le joueur porte toutes ses potions et les perd toutes en cas de défaite (section 9.3).

## 15. Multijoueur (après l'adaptation à Discord)

- Combats contre d'autres vrais joueurs. Le vainqueur ne peut pas prendre de monstre à son adversaire.
- Échange et don de monstres entre joueurs.
- Expéditions en groupe avec un ami.
- Système d'amis.

## 16. Questions ouvertes et valeurs à définir

Les numéros des questions résolues ne sont pas réattribués, pour que les références restent stables.

- Q5. Types Stellaire, Obscur, Lumière et Son (après la V1) : relations d'efficacité, et différence entre les types Ténèbres et Obscur.
- Q6. Nombre de talismans équipables.

Les questions Q16 et suivantes ont été relevées le 17/10/2026 lors du découpage de la V1 en étapes. Chacune indique l'étape de développement qui en a besoin : 1 fondations (types, stats, monstres), 2 combat 1 contre 1, 3 combat complet (changement, statuts, fuite, capture), 4 progression, 5 héros, équipe et camp, 6 expédition manuelle, 7 sauvegarde, 8 contenu et boucle de jeu complète.

- Q19. (Après la V1) Effet du rôle soutien ou offensif, en combat et en expédition.

Les questions Q59 et suivantes ont été relevées le 18/10/2026 lors du découpage de l'adaptation à Discord en étapes : 9 déroulement de la partie commun à la console et au bot, 10 squelette du bot, 11 une partie par joueur Discord, 12 jeu complet sur Discord, 13 déploiement.

- Q64. (Après l'étape 12) Sprites des monstres (section 5.1) : source des images et droits d'utilisation.

Les questions Q65 et suivantes ont été relevées le 18/10/2026 lors de la conception des expéditions automatiques, découpées en étapes : 14 règles du noyau, 15 déroulement et sauvegarde en version 3, 16 console, 17 bot Discord.

- Q66. (Après l'étape 17) Protections des expéditions automatiques (section 12.5) : objets de protection, effet du rôle soutien (Q19), valeurs des protections et façon dont elles s'appliquent à la répartition des dégâts avec report.

Les questions Q67 et suivantes ont été relevées le 18/10/2026 après les premiers tests sur Discord, découpés en étapes : 18 corrections (types au codex, mise en valeur, messages publics pour les tests), 19 Templier et monstre de départ attitré, 20 toute l'équipe en expédition automatique, 21 et suivantes économie (argent, marchand, revente, capsules, attaques au camp).

- Q67. (Étape 19) Fin de l'introduction, maintenant que le monstre de départ n'est plus choisi : texte à proposer dans docs/contenu-v1.md.
- Q68. (Économie) Montants : argent des coffres et des expéditions automatiques, prix de revente des monstres, prix des potions et des capsules.
- Q69. (Économie) Capsules d'attaque : liste des attaques proposées, où et avec quelle chance on les trouve en expédition.
- Q70. (Économie) Chance de rencontrer le marchand à une étape (environ 5 % proposé, pris sur les combats).
- Q71. (Après l'économie) Tuer un monstre pour des ressources : reporté tant qu'aucune ressource n'a d'usage (fabrication, améliorations).

## 17. Journal des décisions

16/10/2026 :

- Interface console d'abord, puis bot Discord ; noyau métier indépendant de l'interface.
- Nom de code du projet : Bersemon.
- Les capsules de capture deviennent des filets de capture.
- Le terme "habileté" remplace "talent", pour se démarquer de Pokémon.
- La classe Tank devient Templier.
- Seuls les monstres combattent pour le moment ; le héros combattant (option hybride) est envisagé à terme.
- Premier monstre : choix parmi 3, au niveau 5.
- Limites de départ : 3 monstres avec le joueur, 30 au camp.
- Monstres : 1 à 3 types, rôle soutien ou offensif, niveau 1 à 100, valeurs internes visibles.
- Liste des 22 types fixée ; table d'efficacité de Pokémon, avec Glace qui résiste à Eau.
- Combat au format Pokémon.
- Capture : chances inversement proportionnelles aux PV restants, augmentées par l'amélioration des filets.
- Fuite : seuls les monstres perdent des PV.
- KO comme dans Pokémon ; mort uniquement en cas de défaite totale (interprétation à confirmer, Q1). Remplacé ci-dessous.
- Défaite en expédition manuelle : retour au camp, perte des objets portés, mort des monstres. La mort des monstres est remplacée ci-dessous.
- Valeurs de dégâts et de gains toujours tirées dans des fourchettes aléatoires.
- Expéditions automatiques : temps réel, une par zone, dégâts répartis à parts égales, échec si tous les monstres sont KO.
- Monnaie : l'argent ; le joueur peut vendre des objets aux marchands.
- Location de monstres : prix donné, valable pour un nombre donné d'expéditions.
- Sauvegarde dans un fichier texte plat, puis SQLite à terme. Format précisé ci-dessous.
- Multijoueur : pas de capture entre vrais joueurs ; échange, don, expéditions avec un ami et système d'amis.
- Périmètre de la V1 validé (section 3).

16/10/2026 (suite) :

- Aucune mort définitive pour le moment : une défaite totale, en expédition manuelle ou automatique, laisse simplement les monstres KO. Les autres conséquences d'une défaite en expédition manuelle sont inchangées (retour au camp, perte des objets portés).
- Les types Stellaire, Obscur, Lumière et Son sont traités dans un second temps et ne font pas partie de la V1.
- Sauvegarde au format JSON.
- Bersemon devient le nom définitif du jeu.

17/10/2026 :

- Q15 en partie : valeurs internes et shiny font partie de la V1 ; habiletés et évolution n'en font pas partie. Codex et objets de soin restent à trancher.
- Q16 : six stats, comme dans Pokémon (PV, Attaque, Défense, Attaque spéciale, Défense spéciale, Vitesse) ; chaque attaque est physique ou spéciale.
- Q17 : stats calculées par la formule de Pokémon, sans EV ni nature ; valeurs internes de 0 à 31, tirées uniformément pour chaque stat.
- Q18 : une attaque a un seul type ; contre plusieurs types, les coefficients se multiplient puis sont bornés entre x0,25 et x4 ; les immunités sont conservées.

17/10/2026 (suite) :

- Q4 : un monstre utilise au plus 4 attaques.
- Q20 : dégâts calculés par la formule de Pokémon, avec un pourcentage tiré de 85 % à 100 % ; au moins 1 dégât si l'attaque n'est pas sans effet.
- Q21 : chaque attaque a une précision et peut rater ; coups critiques à 1 chance sur 24, dégâts x1,5 ; bonus de type x1,5.
- Q22 : pas de limite au nombre d'utilisations des attaques dans la V1.
- Q23 : à vitesse égale, l'ordre d'action est tiré au sort à chaque tour.
- Q24 : dans la V1, l'IA d'un monstre sauvage choisit son attaque au hasard.

17/10/2026 (étape 3) :

- Q25 : un changement volontaire de monstre actif est l'action du tour ; remplacer un monstre KO est gratuit.
- Q26 : statuts poison et paralysie dans la V1, avec les règles actuelles de Pokémon ; un statut à la fois, infligé par l'effet secondaire d'une attaque, guéri au camp.
- Q27 : la fuite réussit toujours ; le monstre actif subit en partant une attaque du monstre adverse.
- Q13 : cette attaque laisse au moins 1 PV ; une fuite ne met jamais KO.
- Q28 : chance de capture = PV perdus / PV max x facteur de niveau (monstre actif du joueur contre monstre sauvage, borné entre x0,5 et x2), bornée à 100 %.
- Q29 : lancer un filet est l'action du tour ; un sauvage KO peut être capturé à la fin d'un combat gagné ; un seul monstre capturé par combat. La limite d'un monstre par combat est remplacée ci-dessous.
- Q30 : filets illimités dans la V1.
- Q12 : un monstre capturé va dans l'équipe s'il y a de la place, sinon au camp ; si le camp est plein, le joueur relâche un monstre pour le garder.
- Q31 : pas de combat contre des humains dans la V1.
- Q19 en partie : le rôle n'a pas d'effet dans la V1.
- Un monstre sauvage a 1 chance sur 512 d'être shiny.

17/10/2026 (étape 3, suite) :

- Q29 révisée : plusieurs monstres peuvent être capturés dans un même combat ; chaque monstre d'un groupe sauvage peut l'être, au filet pendant le combat ou KO à la fin d'un combat gagné.

17/10/2026 (étape 4) :

- Q10 : courbe d'expérience n x n x n pour toutes les espèces ; stats recalculées et PV actuels augmentés d'autant que les PV max à chaque niveau.
- Q32 : XP d'un monstre vaincu (KO ou capturé) = rendement de l'espèce x niveau / 7, x un pourcentage de 90 % à 110 % ; versée en entier à chaque participant non KO, à la fin du combat, quelle que soit l'issue.
- Q33 : attaques apprises à des niveaux fixés par espèce ; avec 4 attaques connues, le joueur oublie une attaque ou renonce à la nouvelle ; un monstre créé connaît les 4 dernières attaques de sa liste jusqu'à son niveau.

17/10/2026 (étape 5) :

- Q2 : les classes de la V1 sont Guerrier et Mage.
- Q34 : la classe donne +10 % à une stat des monstres du joueur en combat (Guerrier : Attaque ; Mage : Attaque spéciale ; plus tard Templier : Défense et Défense spéciale, Assassin : Vitesse) ; pas d'action de soutien dans la V1.
- Q9 et Q35 : dans la V1, le héros n'a ni niveau, ni stats, ni équipement ; il a un nom libre de 1 à 20 caractères et une classe.
- Q36 : départ en expédition avec 1 à 3 monstres ; au camp, le joueur peut relâcher un monstre (sauf le dernier), le renommer (surnom de 1 à 20 caractères) et choisir l'ordre de son équipe.
- Q12 précisée : avec l'équipe et le camp pleins, le joueur relâche n'importe lequel de ses monstres, la capture prenant sa place, ou relâche la capture.

17/10/2026 (étape 6) :

- Q3 : 2 zones dans la V1, une facile puis une difficile.
- Q40 : la zone difficile apparaît après une expédition menée jusqu'au bout dans la zone facile. Remplacé ci-dessous par la traque des Apôtres.
- Q37 et Q41 : une expédition compte de 4 à 6 étapes, nombre tiré au départ et inconnu du joueur ; entre deux étapes, le joueur continue ou rentre en gardant ses gains ; PV et statuts sont conservés ; après la dernière étape, la zone compte comme explorée.
- Q38 : à chaque étape, combat (80 %), coffre (15 %) ou monstre rare (5 %).
- Groupes sauvages de 1 à 2 monstres en zone facile, de 1 à 3 en zone difficile.
- Q15 : potions de soin dans la V1 (40 % à 60 % des PV max, ni réanimation ni soin des statuts), utilisables en combat ou entre deux étapes, toutes perdues en cas de défaite ; codex simple dans la V1.
- Q39 : un coffre contient 1 à 2 potions.

17/10/2026 (étape 7) :

- Q42 : un seul emplacement de sauvegarde ; une nouvelle partie remplace l'ancienne après confirmation.
- Q43 : sauvegarde automatique à la création, à chaque retour au camp et après chaque action au camp ; jamais en expédition.
- Fichier de sauvegarde JSON dans %APPDATA%\Bersemon.

17/10/2026 (étape 8) :

- Q44 et Q45 : un brouillon du contenu de la V1 est rédigé dans docs/contenu-v1.md, à valider avant d'être codé.
- Q46 : une courte introduction et un tutoriel de quelques conseils, chacun montré une fois par partie.
- Q47 : pas d'image dans la console ; un shiny est signalé par "(shiny)".
- Hypothèses validées : rôle propre à l'espèce (Q19), 1 à 4 attaques distinctes (Q48), entrée des sauvages dans l'ordre de la rencontre (Q49), défaite si les deux derniers monstres tombent au même tour (Q50), équipe jamais vide (Q51), monstre de départ capturé au codex (Q54), sauvegarde incompatible refusée sans rien effacer (Q56).
- Q52 révisée : le monstre de départ a des valeurs internes tirées au hasard et 1 chance sur 512 d'être shiny.
- Q53 révisée : une potion sur un monstre à PV pleins est permise après confirmation, et consommée sans effet.
- Q55 révisée : entre deux étapes, le joueur peut changer l'ordre de son équipe.
- Monstres de départ : Salamandre offensive en attaque spéciale, Crapaud offensif en attaque physique, Mandragore défensive ; un départ plus difficile que les autres est assumé. Noms du brouillon acceptés.
- Histoire inspirée de Berserk : le héros traque les Apôtres un par un pour se venger de leur maître, le Démiurge. Un seul Apôtre dans la V1, dans la zone difficile ; le Démiurge vient après la V1.
- Traque : un indice par expédition menée au bout ; le 1er, trouvé dans la zone facile, ouvre la zone difficile ; au 3e, l'antre est localisé et l'expédition suivante s'achève par le combat contre l'Apôtre.
- Q40 révisée : une zone s'ouvre désormais par la traque (indice ou Apôtre vaincu), et non plus par une simple expédition menée au bout.
- Combat contre un Apôtre : seul, 2 niveaux au-dessus du maximum de sa zone, stats bien plus hautes, sans fuite ; défaite ordinaire, indices conservés ; capture possible après la victoire ; il ne réapparaît plus.
- Après l'Apôtre de la V1 : un épilogue, puis jeu libre.
- Q44 et Q45 : contenu de la V1 validé dans docs/contenu-v1.md, avec la Mandragore à 320 de stats de base (5 PV de plus).
- Q57 : le combat contre l'Apôtre remplace l'événement de la dernière étape de l'expédition qui suit le 3e indice.
- Le rôle de soutien couvre aussi les profils défensifs ; pas de rôle supplémentaire.
- Q58 : un Apôtre a, comme un monstre sauvage, des valeurs internes tirées au hasard et 1 chance sur 512 d'être shiny ; pas de filet pendant le combat contre lui, il ne se capture que KO après la victoire.
- Q58 révisée : les valeurs internes d'un Apôtre ne sont pas tirées au hasard, elles sont fixes, à 31 pour chaque stat ; le shiny reste à 1 chance sur 512.

18/10/2026 (adaptation à Discord) :

- Après la V1, le chantier suivant est le bot Discord, avant d'enrichir le jeu ou de poursuivre l'histoire.
- Le déroulement de la partie est commun à la console et au bot, qui ne font que l'afficher et transmettre les choix du joueur.
- Sur Discord, les mêmes commandes fonctionnent dans un salon de serveur, avec des réponses visibles du seul joueur, et en message privé.
- Sauvegarde sur Discord : un fichier JSON par compte Discord, au format actuel ; SQLite plus tard.

18/10/2026 (étape 11) :

- Q59 : la partie Discord est séparée de la partie de la console.
- Q60 : une seule partie par compte Discord, la même sur tous les serveurs et en message privé.
- Sur Discord, une seule commande, /bersemon, ouvre un message qui pose la question en cours, avec des boutons et un formulaire pour le nom du héros.

18/10/2026 (étape 12) :

- Q61 : sur Discord, pas de délai d'inactivité ; une expédition en cours attend le joueur jusqu'à son prochain /bersemon, et un redémarrage du bot le ramène à sa dernière sauvegarde.
- Q62 : les sprites sont repoussés après l'étape 12 ; la source des images et leurs droits restent à décider (Q64).
- Sur Discord, chaque choix met à jour le même message, qui montre ce qui vient de se passer et la question suivante.

18/10/2026 (étape 13) :

- Q63 : le bot tourne sur le PC de l'utilisateur, lancé à la main ; il est privé, et seul l'utilisateur peut l'inviter dans un serveur. Un serveur permanent viendra plus tard si des joueurs veulent le bot en ligne en continu.

18/10/2026 (expéditions automatiques) :

- Chantier suivant : les expéditions automatiques, découpées en étapes 14 (règles du noyau), 15 (déroulement et sauvegarde en version 3), 16 (console) et 17 (bot Discord).
- Une expédition automatique emmène de 1 à 3 monstres, pris dans l'équipe ou au camp ; l'équipe garde au moins un monstre.
- Q7 : chaque monstre en plus du premier réduit la durée de 25 % de la durée de base (100 %, 75 %, 50 %).
- Durées de base : Lisière 1 h, Marais 2 h. La durée est fixe et se compte à partir de l'heure du départ, jeu fermé ou bot éteint compris.
- Dégâts répartis à parts égales, en divisions entières, avec report : ce qu'un monstre KO ne peut plus encaisser passe aux monstres encore debout. Remplace la répartition sans report de la section 12.3.
- Fourchettes de dégâts : Lisière 15 à 60, Marais 40 à 120.
- Récompenses d'une expédition réussie : de l'XP et des potions ; les ressources viendront avec l'économie.
- XP : chaque monstre non KO reçoit en entier un tirage de 100 à 200 (Lisière) ou de 300 à 600 (Marais), environ la moitié d'une expédition manuelle ; les monstres KO ne gagnent rien. Potions : 0 à 1 (Lisière), 1 à 2 (Marais).
- Protections reportées tant qu'il n'existe ni objet de protection ni effet du rôle (Q66).
- Une expédition rentrée attend que le joueur l'accueille au camp ; le résultat est tiré à l'accueil, et les attaques en trop se choisissent comme après un combat.
- Au camp, le joueur peut rappeler une expédition avant son terme, sans dégâts ni récompense.
- Précisions confirmées à l'étape 14 : le gain d'XP est tiré une seule fois et reçu en entier par chaque monstre non KO ; une expédition déjà rentrée ne se rappelle plus, elle s'accueille.

18/10/2026 (étape 15) :

- Rappeler une expédition automatique demande confirmation.
- La partie est sauvegardée dès l'accueil d'une expédition ; les attaques à oublier qui restent à choisir sont gardées par la sauvegarde et reproposées à la reprise.
- Les expéditions rentrées s'accueillent une par une, chacune avec son bilan.

18/10/2026 (étape 17) :

- Q65 : le message privé de retour n'est envoyé qu'une fois par expédition ; le bot garde la trace des messages envoyés, même après un redémarrage.

18/10/2026 (retours des premiers tests sur Discord) :

- La fiche d'une espèce capturée rappelle ses types.
- Coups critiques, statuts, efficacité, KO et captures sont mis en valeur : en gras sur Discord, en rouge dans la console.
- Une option au lancement du bot rend /bersemon visible de tout le salon, pour les tests ; seul l'auteur d'un message peut se servir de ses boutons.
- Le Templier devient jouable (+10 % en Défense et en Défense spéciale). Chaque classe a son monstre de départ attitré : Guerrier, Crapaud ; Mage, Salamandre ; Templier, Mandragore. Le choix du monstre de départ disparaît.
- Toute l'équipe peut partir en expédition automatique s'il reste au moins un monstre au camp ; le premier monstre du camp rejoint alors l'équipe, qui n'est jamais vide. Un bouton prépare ce départ.
- L'argent vient de la revente des monstres, des coffres et des expéditions automatiques réussies.
- Le premier marchand rencontré en expédition se réfugie au camp pour de bon ; il vend potions et capsules d'attaque et rachète les monstres capturés.
- Au camp, un monstre réapprend gratuitement les attaques de son espèce jusqu'à son niveau ; une capsule d'attaque, réutilisable, enseigne son attaque à tout monstre du même type.
- Tuer un monstre pour des ressources est reporté (Q71).
- La capture donne bien de l'XP (section 7), versée à la fin du combat : aucun changement.
- Sur Discord, le bot prévient le joueur par message privé du retour d'une expédition, et à son démarrage pour celles rentrées pendant qu'il était éteint.
- Hypothèses validées : une expédition automatique n'a aucun effet sur la traque, l'exploration ni le codex ; les monstres envoyés gardent leur place au camp, comptée dans sa limite, et ne peuvent être ni relâchés, ni renommés, ni échangés ; une expédition manuelle reste possible dans la même zone ; la durée n'est pas tirée au hasard.
