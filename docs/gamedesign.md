# Game Design Document - Bersemon

Bersemon est le nom du projet et du jeu.

Statut : document de référence du projet. Il décrit ce qui est décidé ; les points non tranchés sont regroupés dans la section 16 "Questions ouvertes et valeurs à définir", et l'historique des décisions dans la section 17.

Dernière mise à jour : 20/10/2026

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
- Décidé le 25/10/2026, pour l'étape 113 (choix de l'utilisateur) : le bot prévient par message privé, systématiquement, de tout ce qui se termine. Le retour d'une expédition automatique, tout travail de Gunther (amélioration, objet, charme de garde ou lot de filets), la ligne d'Ansgar prête, le Rite du sang mêlé achevé, les pièges pleins, qui ne prennent plus rien, et chaque monstre de la cour d'entraînement arrivé à son plafond, qui ne gagne plus d'XP. Comme avant, chaque fin n'est annoncée qu'une fois, même si le bot redémarre (Q65), et le bot prévient à son démarrage de ce qui s'est terminé pendant qu'il était éteint. Ce qui est à annoncer se déduit de la partie après chaque action du joueur : un plafond relevé par une zone ouverte, ou une cour améliorée, déplace l'annonce. Précisé en codant : les charmes de garde et les lots de filets lancés pendant que le bot tournait n'étaient pas annoncés ; ils le sont désormais. Textes dans docs/contenu-v1.md, section 29.
- Pour les tests, une option au lancement du bot rend /bersemon visible de tout le salon. Seul l'auteur d'un message peut alors se servir de ses boutons ; les autres joueurs ouvrent leur propre partie par /bersemon. Sans l'option, les réponses restent visibles du seul joueur.
- Les informations importantes du combat sont mises en valeur sur Discord, en gras : coups critiques, statuts, efficacité des attaques, KO et captures. La console les affiche en texte simple.
- Décidé le 25/10/2026, pour l'étape 116 (demande de l'utilisateur) : un outil d'équilibrage, hors de ce que joue le joueur, fait jouer des parties complètes par un joueur automatique, avec le noyau seul, de la création du héros à la chute du Margrave, et écrit leurs statistiques en Markdown : durée, défaites, combats de fin de zone, ressources. C'est un crate à part du workspace, bersemon-equilibrage, sans crate externe, dont ni la console ni le bot Discord ne dépendent. Méthode, limites du joueur automatique, résultats et réglages proposés dans docs/equilibrage.md (Q107).
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
- Depuis l'étape 27 (19/10/2026), l'Assassin aussi : Guerrier, Mage, Templier ou Assassin.
- Depuis les étapes 48 à 51 (20/10/2026), le héros a un équipement : une arme, une armure et 2 talismans (Q6), fabriqués par Gunther (section 4.3). Il n'a toujours ni niveau ni stats propres (hypothèse retenue, Q87).

### 4.2 Rôle en combat

- Pour le moment, seuls les monstres combattent, un à un. Le héros ne combat pas et n'a pas de PV en combat ; depuis l'étape 52, il donne un ordre par combat (ci-dessous).
- La classe et l'équipement du héros donnent des bonus aux monstres et quelques actions de soutien, par exemple des ordres ou l'utilisation d'objets.
- Bonus de classe : en combat, les monstres du joueur ont +10 % dans une stat, arrondi à l'entier inférieur. Guerrier : Attaque. Mage : Attaque spéciale. Après la V1, Templier : Défense et Défense spéciale, à chacune +10 % ; depuis l'étape 27, Assassin : Vitesse. Les actions de soutien viendront après la V1 ; depuis l'étape 52, ce sont les ordres (ci-dessous).
- Décidé le 18/10/2026, pour l'étape 27 : l'Assassin devient jouable, avec un monstre attitré rapide, qui frappe fort mais encaisse mal. Précisé le 19/10/2026 (Q72) : ce monstre est le Chat noir, de type Ténèbres, offensif en attaque physique (section 6, fiche dans docs/contenu-v1.md).
- Depuis l'étape 28, les habiletés de stats des monstres s'ajoutent à ce bonus (section 5.9).
- Décidé le 20/10/2026, pour l'étape 75 (choix de l'utilisateur, Q19) : en combat, les monstres du joueur ont aussi le bonus de leur rôle (section 5.1), +10 % de Défense et de Défense spéciale pour un soutien, +10 % d'Attaque et d'Attaque spéciale pour un offensif, qui s'ajoute aux autres bonus. Seuls les monstres du joueur en profitent : ni les monstres sauvages, ni ceux des humains, ni les Apôtres. En duel, chaque joueur en profite pour les siens, comme du bonus de classe (hypothèse retenue, Q96). Le rôle ne joue pas sur les PV. La fiche d'un monstre dit son rôle et son effet en combat (étape 75).
- Ordres du héros, décidés le 20/10/2026 pour les étapes 52 à 54 (choix de l'utilisateur) :
  - chaque classe a un ordre : Guerrier, Cri de guerre (+20 % d'Attaque) ; Mage, Incantation (+20 % d'Attaque spéciale) ; Templier, Bouclier de foi (+20 % de Défense et de Défense spéciale) ; Assassin, Pas de l'ombre (+20 % de Vitesse). Textes dans docs/contenu-v1.md, section 12 ;
  - le héros donne son ordre une seule fois par combat, au monstre actif, avant de choisir l'action du tour. L'ordre ne prend pas le tour : le joueur choisit ensuite l'action de son monstre, et l'adversaire n'agit pas entre-temps. Il n'est pas tiré au sort ;
  - le bonus s'ajoute, en pourcentage, à ceux de la classe, de l'équipement et de l'habileté (sections 4.3 et 5.9), et reste au monstre jusqu'à la fin du combat, même s'il sort puis revient ;
  - l'ordre se donne dans tous les combats, dès le premier, contre les Apôtres et les humains compris ; pas pendant le remplacement d'un monstre KO. Il ne joue pas sur les expéditions automatiques (hypothèses retenues, Q88) ;
  - l'utilisation d'objets en combat existe déjà : les soins sont l'action du tour (section 14.4).
  - Écrans : la console ajoute "Ordre du héros" au menu de combat (étape 53), le bot Discord un bouton dans la rangée du filet et de la fuite (étape 54), tant que l'ordre n'est pas donné.
- À terme, option hybride envisagée : le héros combat lui-même uniquement lorsque tous ses monstres sont KO.

### 4.3 Équipement

Décidé le 20/10/2026, pour les étapes 48 à 51 (choix de l'utilisateur) :

- Le héros porte une arme, une armure et 2 talismans, différents l'un de l'autre (Q6).
- Chaque objet porté ajoute des pourcentages aux stats des monstres du joueur en combat, comme le bonus de classe, auquel ils s'additionnent, avec celui des habiletés (sections 4.2 et 5.9). Les arrondis se font à l'entier inférieur. Un Guerrier qui porte une arme à +15 % d'Attaque et un talisman à +5 % d'Attaque, dont le monstre a Férocité, donne +40 % d'Attaque à ce monstre (à vérifier en jouant, Q87).
- Les armes renforcent l'Attaque ou l'Attaque spéciale, les armures la Défense et la Défense spéciale ensemble, et chaque talisman une stat. Aucun objet ne renforce les PV : les PV ne sont pas majorés en combat.
- Comme le bonus de classe, l'équipement ne joue qu'en combat, pas sur les expéditions automatiques (hypothèse retenue, Q87). Aucun objet n'est réservé à une classe (hypothèse retenue, Q87).
- Les objets se fabriquent chez Gunther (section 13.5), chacun une seule fois. Le héros garde tous les objets qu'il possède : ils ne vont ni dans le sac ni dans le coffre, et rien ne les lui fait perdre, pas même une défaite.
- Il les porte, les change ou les retire au camp seulement, sans confirmation. Un objet fabriqué n'est pas porté d'office.
- Liste, bonus, coûts et durées dans docs/contenu-v1.md, section 11 : 14 objets en trois paliers, liés aux ressources des zones.
- Écrans, à l'étape 50 (console) : une fois Gunther au camp, ou dès que le héros possède un objet, le camp propose "Équipement du héros". L'écran montre l'objet porté à chaque place (arme, armure, premier et second talisman), le bonus total des objets portés et les objets non portés ; le joueur choisit une place, puis l'objet à y porter, parmi ceux de sa sorte qui ne sont portés nulle part, ou de retirer l'objet porté. La forge propose "Fabriquer un objet" : la liste de tous les objets, avec leur coût et leur durée, ou "possédé", puis ceux qui ne sont pas encore possédés, même faute de ressources ou si Gunther est occupé (le refus dit pourquoi). Le lancement est confirmé, comme pour une amélioration.
- Écrans, à l'étape 51 (Discord) : un bouton "Équipement" s'ajoute au camp dans les mêmes cas (au plus 8 boutons, sur deux rangées). L'écran d'équipement a un menu par place, qui propose les objets à y porter, avec leur bonus, et "Retirer" pour l'objet porté ; une place sans objet à porter ni à retirer n'a pas de menu. La forge a un menu "Fabriquer un objet" : chaque objet non possédé, avec son bonus, son coût et sa durée. Le bot prévient par message privé de la fin d'une fabrication, comme d'une construction.

## 5. Les monstres

### 5.1 Caractéristiques

Chaque monstre possède :

- un niveau, de 1 à 100 (section 7) ;
- un à trois types ;
- un rôle : soutien ou offensif, le soutien couvrant aussi les profils défensifs. Ce rôle s'ajoute aux types et il est propre à l'espèce ; il n'a pas d'effet dans la V1. Depuis les étapes 75 et 76 (choix de l'utilisateur), il joue en combat pour les monstres du joueur (section 4.2) et en expédition automatique (sections 12.5 et 12.6) ;
- six stats, comme dans Pokémon : PV, Attaque, Défense, Attaque spéciale, Défense spéciale, Vitesse (section 5.8) ;
- de 1 à 4 attaques distinctes, chacune d'un seul type, physique ou spéciale (section 10.4) ;
- des valeurs internes aléatoires, de 0 à 31 pour chaque stat, qui influent sur ses stats (similaires aux IV de Pokémon), ce qui rend chaque monstre unique. Ces valeurs sont visibles dans le résumé du monstre ;
- une habileté unique (équivalent des talents de Pokémon), qui a un effet secondaire lors des expéditions, des combats, etc. Les habiletés sont hors V1 ; elles arrivent à l'étape 28 (section 5.9) ;
- une faible chance d'être "shiny", 1 sur 512 pour un monstre sauvage comme pour le monstre de départ : sa couleur change, sans incidence sur les stats. Dans la console, un shiny est signalé par la mention "(shiny)" après son nom. Depuis l'étape 28 (19/10/2026), un shiny a 31 dans chaque valeur interne : ses valeurs internes ne sont plus tirées, et les shiny des sauvegardes antérieures passent à 31 à leur reprise ;
- un sprite (une image), pour rendre le jeu plus vivant. La console de la V1 n'affiche pas d'image, et le jeu reste textuel sur Discord pour l'instant : les sprites viendront plus tard, une fois décidées la source des images et leurs droits.

### 5.2 Types

- Les types reprennent ceux de Pokémon, avec l'ajout de nouveaux types.
- Liste des types : Normal, Combat, Vol, Poison, Sol, Roche, Insecte, Spectre, Acier, Feu, Eau, Plante, Électrique, Psychique, Glace, Dragon, Ténèbres, Féérique, Stellaire, Obscur, Lumière, Son.
- Table d'efficacité : celle de Pokémon, avec une modification : le type Glace résiste au type Eau.
- Une attaque a un seul type. Contre un monstre à 2 ou 3 types, les coefficients de chacun de ses types se multiplient, puis le résultat est borné entre x0,25 et x4. Une immunité (x0) l'emporte sur tout le reste. Exemple : une attaque Roche contre un monstre Feu, Vol et Glace vaudrait x8 ; elle inflige x4.
- Les types Stellaire, Obscur, Lumière et Son sont traités dans un second temps : leurs relations d'efficacité restent à définir et ils ne font pas partie de la V1 (voir Q5).
- Type Obscur, décidé le 18/10/2026 pour l'étape 30 : il n'appartient qu'aux Apôtres, qui n'ont que ce type.
  - En défense : les attaques Féérique, Psychique et Stellaire lui sont très efficaces ; les attaques Ténèbres, Poison, Insecte, Dragon et Spectre sont neutres ; il résiste à toutes les autres.
  - En attaque : une attaque Obscur est très efficace contre tous les types, sauf Féérique et Stellaire, qui y résistent, et Psychique, Ténèbres, Poison, Insecte, Dragon et Spectre, contre qui elle est neutre.
  - Le type Stellaire, opposé de l'Obscur, est très rare : un seul monstre l'aura. Le type Féérique, autre opposé de l'Obscur, reste rare.
  - Depuis l'étape 27 (19/10/2026), ce monstre est le Schnoz (section 5.3). En attendant l'Obscur, le type Stellaire est neutre : ses attaques sont neutres contre tous les types, et il n'a ni faiblesse, ni résistance, ni immunité. Ses relations avec l'Obscur, décidées ci-dessus, seront codées à l'étape 30 ; ses autres relations restent ouvertes (Q76).
  - Restent à définir : Obscur contre Obscur, les autres relations du Stellaire, et l'équilibrage du combat contre un Apôtre (Q76). L'Apôtre combat seul contre les 3 monstres du joueur : on l'équilibrera en jouant, s'il le faut.
  - Décidé le 19/10/2026, pour les étapes 44 à 47 : l'Ange déchu, rare et pacifique de l'Abbaye des Cendres, est le premier monstre de type Psychique ; il est très efficace contre les Apôtres (docs/contenu-v1.md, section 10).
  - Précisé le 19/10/2026, pour l'étape 30 : une attaque Obscur est très efficace contre un monstre Obscur ; les Apôtres se déchirent entre eux. Le Stellaire garde ses relations avec l'Obscur et reste neutre envers les autres types. Le Baron pendu devient le premier Apôtre de type Obscur (section 9.4).
- Types Lumière et Son, décidés le 24/10/2026 pour les étapes 93 à 95 (choix de l'utilisateur) :
  - Lumière, en attaque : très efficace contre Ténèbres, Spectre et Obscur ; peu efficace contre Feu, Acier et Lumière. En défense : faible face à Ténèbres et Spectre ; résiste à Feu, Féérique, Lumière et Obscur. C'est le troisième opposé de l'Obscur, avec le Féérique et le Stellaire : une attaque Obscur ne lui est que peu efficace, et la Lumière s'ajoute aux types très efficaces contre les Apôtres.
  - Son, en attaque : très efficace contre Psychique, Glace et Vol ; peu efficace contre Sol et Son ; sans effet sur Spectre (un spectre n'a pas d'oreilles). En défense : faible face à Sol et Électrique ; résiste à Normal, Vol et Son. Envers l'Obscur, il suit la règle des types ordinaires : une attaque Obscur lui est très efficace, et l'Obscur résiste au Son.
  - Le Stellaire reste neutre envers la Lumière et le Son, dans les deux sens.
  - Ils arrivent par deux espèces nouvelles de l'Abbaye des Cendres (section 5.3) et six attaques nouvelles, sans capsule : les capsules n'enseignent qu'aux monstres de leur type (section 13.10), et chaque type n'a qu'une espèce. Les attaques existantes ne changent pas de type (docs/contenu-v1.md, section 24).
  - Précisé à l'étape 95, en vérifiant les écrans : la console et le bot Discord n'ont pas changé. Le codex de Discord, à 56 espèces, garde deux pages de 30 ; chez Malvoise, 14 espèces à acheter et 9 à louer quand toutes les zones sont menées au bout.

### 5.3 Bestiaire

- Les créatures ne reprennent pas directement les Pokémon, qui ne correspondent pas à l'univers. Elles s'en inspirent pour être adaptées à un univers assez sombre et médiéval.
- Exemples de familles : dragons, fées, elfes, créatures mythologiques ou folkloriques, animaux actuels ou disparus, objets possédés, etc.
- Volume initial : une dizaine de monstres, augmenté au fur et à mesure.
- Le contenu de la V1 (espèces, attaques, zones et textes) est décrit dans docs/contenu-v1.md, validé le 17/10/2026.
- Après la V1, chaque espèce a une rareté : commune (monstres de départ compris), rare ou Apôtre. Avec le total de ses stats de base, elle fixe son prix de revente (section 14.5).
- Décidé le 18/10/2026, pour l'étape 27 : de nouveaux monstres plus proches de l'univers de Berserk, parmi lesquels des fées, des elfes, des squelettes et des dragons (Q73).
- Précisé le 19/10/2026 : ces monstres peupleront la troisième zone, à l'étape 30, et non les deux zones actuelles (Q73).
- Décidé le 19/10/2026, pour l'étape 30 : la Vallée des Brumes accueille la Fée noire et le Squelette, communs, l'Elfe, rare et pacifique, et la Vouivre, rare et hostile. Le type Féérique y reste réservé à une espèce rare (section 5.2). Fiches dans docs/contenu-v1.md.
- Monstre caché, depuis l'étape 27 (19/10/2026) : le Schnoz, seul monstre de type Stellaire, d'espèce rare. Il se rencontre très rarement, dans toute zone (section 9.2). Fiche dans docs/contenu-v1.md.
- Décidé le 20/10/2026, pour les étapes 62 à 65 : quatre monstres aquatiques, qui ne se trouvent qu'à la pêche (section 13.8) : le Noyé, l'Anguille des tourbières et la Sangsue géante, communs, et le Kelpie, rare, de type Eau et Féérique. Au codex, ils viennent après la Reine des ronces. Fiches dans docs/contenu-v1.md, section 15.
- Décidé le 20/10/2026, pour les étapes 79 à 82 (contenu validé par l'utilisateur) : le Col du Givre-Noir accueille le Wendigo, le Troll des cavernes et la Harpie des orages, communs, la Gargouille, et deux formes évoluées sauvages, le Warg et le Chevalier noir (section 5.4) ; le Wyrm de givre, rare et hostile, et la Sylphe des neiges, rare et pacifique, de type Glace et Féérique. Les types Glace, Sol et Électrique y apparaissent pour la première fois. Au codex, ces espèces viennent après les monstres aquatiques, puis les formes de stade 3. Fiches dans docs/contenu-v1.md, section 21.
- Décidé le 24/10/2026, pour les étapes 93 à 95 (contenu validé par l'utilisateur) : l'Abbaye des Cendres accueille la Cloche hurlante, commune, de type Acier et Son, et le Séraphin de vitrail, rare et pacifique, de type Lumière, troisième monstre rare de l'Abbaye avec la Chimère et l'Ange déchu. Au codex, elles viennent après les deux formes du Démiurge (précisé en codant, hypothèse retenue, Q100). Fiches dans docs/contenu-v1.md, section 24.
- Décidé le 25/10/2026, pour l'étape 117 (choix de l'utilisateur, d'après docs/equilibrage.md) : la famille de la Mandragore (Mandragore, Mandragore ancienne, Arbre-pendu) apprend Secousse (Sol, physique, 60, précision 100) au niveau 7, à la place du Cri strident. Toutes les espèces du Marais résistent à la Plante ou la frappent fort, et ses attaques Normal ne touchent pas les Spectres : le Templier y était la classe la plus lente. Une Mandragore qui a déjà passé le niveau 7 garde le Cri strident et n'apprend pas Secousse (hypothèse retenue, Q108). Fiches dans docs/contenu-v1.md, section 2.

### 5.4 Évolution

- Un monstre peut évoluer ou non. Conditions possibles (comme dans Pokémon) :
  - l'utilisation d'un objet ;
  - un certain niveau atteint ;
  - une situation ou un lieu particulier.
- Une évolution augmente les stats, peut modifier le ou les types et change la forme du monstre.

Décidé le 19/10/2026, pour les étapes 37 à 40 :

- Une espèce évolue 0, 1, 2 ou 3 fois : une lignée compte au plus 4 stades. Chaque stade est une espèce à part, avec son nom, ses types, ses stats de base, ses attaques et ses habiletés.
- Niveaux d'évolution d'une espèce standard : 16 pour la première évolution, 36 pour la deuxième, 64 pour la troisième. Le type Insecte évolue plus tôt (12, 25, 50), le type Dragon plus tard (22, 45, 70). Ce sont les types de la forme qui évolue qui comptent ; une espèce à la fois Insecte et Dragon suivrait la grille standard (aucune aujourd'hui). Ces niveaux sont des repères : chaque espèce a le sien, fixé dans docs/contenu-v1.md, section 8.
- Conditions : un niveau atteint ; une pierre d'évolution utilisée au camp ; un niveau atteint dans un lieu donné ; un niveau atteint après un événement donné.
- Les zones s'arrêtent au niveau 20 : seules les premières évolutions sont jouables et codées. Les stades suivants sont seulement nommés (docs/contenu-v1.md, section 8) et seront codés avec des zones plus hautes.
- Une évolution est proposée au joueur, qui peut la refuser ou la repousser :
  - après un combat, à un monstre qui a gagné au moins un niveau pendant ce combat et qui remplit la condition, après ses attaques apprises. Refusée, elle sera proposée de nouveau au prochain niveau gagné ;
  - au camp, depuis la gestion des monstres, tout monstre qui remplit une condition de niveau ou d'événement peut évoluer à la demande, après confirmation. Une condition de lieu ne se remplit qu'en expédition, après un combat dans ce lieu ;
  - au retour d'une expédition automatique, rien n'est proposé : le monstre évoluera au camp, à la demande (hypothèse retenue, Q84).
- Ce que change l'évolution : l'espèce, donc les stats de base, les types et la liste des attaques apprises. Les PV actuels augmentent d'autant que les PV max, sauf pour un monstre KO, qui le reste. L'habileté garde sa position : la première habileté de l'espèce donne la première de l'évolution, la seconde la seconde. Les capsules d'habileté (section 5.9) restent valables.
- Ce qui ne change pas : les valeurs internes, le shiny, le surnom, le niveau, l'XP, le statut et les attaques connues.
- Attaque d'évolution : chaque évolution enseigne une attaque liée à la nouvelle forme. S'il en connaît déjà 4, le joueur choisit celle à oublier ou renonce, comme à la montée de niveau (section 7) ; au camp, ce choix se fait comme au retour d'une expédition automatique (section 12.7). Une attaque refusée se réapprend au camp.
- La liste d'attaques d'une forme évoluée reprend celle de sa forme de base, puis l'attaque d'évolution et une autre attaque au niveau de l'évolution : le récit de l'évolution rappelle que d'autres attaques peuvent s'apprendre au camp (section 13.10).
- La forme évoluée compte comme capturée au codex. Elle a la rareté de sa forme de base, ne lâche pas de ressource et se revend au prix de sa propre espèce (section 14.5). Hypothèse retenue (Q84) : elle a les mêmes habiletés que sa forme de base, dans le même ordre.
- Les monstres sauvages et ceux des humains restent à leur forme de base, quel que soit leur niveau. Un monstre capturé, ou d'une sauvegarde antérieure, déjà au-delà de son niveau d'évolution, pourra évoluer au camp ou au prochain niveau gagné.
- Pierres d'évolution (étape 39) : la Pierre de braise fait évoluer le Feu follet, la Pierre de lune le Cerf blanc, à tout niveau. Elles s'utilisent au camp, depuis la gestion des monstres, après confirmation, et se consomment. Elles se trouvent dans un coffre sur 10, tirées entre les deux, et Malvoise les vend 300 pièces d'argent. Elles se rangent dans le sac ou le coffre, se déplacent par lots et se perdent avec le sac (section 9.3). Taux et prix sont des hypothèses (Q84). Précisé à l'étape 39, en codant : un même tirage décide de la capsule d'habileté et de la pierre (1 à 10 sur 100 pour la capsule, 11 à 15 pour la Pierre de braise, 16 à 20 pour la Pierre de lune), si bien qu'un coffre ne contient jamais les deux ; les taux restent d'un coffre sur 10 pour chacune. Au camp, la pierre est prise dans le sac d'abord, puis dans le coffre, comme une capsule d'habileté.
- Écrans des pierres, à l'étape 39 : l'évolution par pierre passe par la même entrée que les autres ("Faire évoluer un monstre" dans la console, bouton "Évoluer en" sur Discord), proposée dès que le joueur a la pierre ; la confirmation dit que la pierre sera consommée. Dans la console, l'étal de Malvoise propose "Acheter des pierres d'évolution", au nombre choisi, et le coffre "Prendre des pierres d'évolution" et "Ranger des pierres d'évolution", au nombre choisi ; "Tout ranger dans le coffre" passe au numéro 13. Sur Discord, l'étal a un bouton d'achat à l'unité par pierre, et les menus des ressources du coffre proposent aussi les pierres, une ou toutes. Le coffre trouvé, le camp et le coffre disent les pierres du sac et du coffre.
- Lieu (étape 40) : l'Armure hantée évolue en Chevalier noir si elle gagne un niveau dans la Vallée des Brumes, à partir du niveau 18, en expédition manuelle.
- Événement (étape 40) : le Squelette évolue en Chevalier squelette s'il gagne un niveau, ou au camp, une fois le Baron pendu vaincu, à partir du niveau 20 : les morts de la Vallée s'éveillent à sa chute.
- L'Elfe, le Baron pendu, le Schnoz et le marchand zombie n'évoluent pas. La Vouivre évoluera une fois, au niveau 45, plus tard.
- Décidé le 20/10/2026, pour l'étape 81 (choix de l'utilisateur) : avec le Col du Givre-Noir (niveaux 28 à 36), les cinq stades 3 nommés au niveau 36 sont codés : Wyverne de cendre, Crapaud-roi, Arbre-pendu, Cait sith et Loup-garou (docs/contenu-v1.md, section 21). Le Dragon de feu (niveau 70) et le Dragon des brumes (niveau 45) restent nommés seulement.
- Décidé le 20/10/2026, pour l'étape 79 (choix de l'utilisateur, Q84) : exception à la règle des formes de base, le Warg et le Chevalier noir vivent à l'état sauvage dans le Col du Givre-Noir. Ils lâchent la ressource de leur forme de base, et le Warg sauvage capturé peut évoluer en Loup-garou. Partout ailleurs, les monstres sauvages et ceux des humains restent à leur forme de base.
- Décidé le 19/10/2026, pour les étapes 44 à 47 : la Fée-sorcière évolue en Reine des ronces au niveau 25, deuxième évolution atteignable dans l'Abbaye des Cendres (docs/contenu-v1.md, section 8). Codée à l'étape 47 ; elle apprend aussi Lianes étrangleuses au niveau 25 (hypothèse retenue, Q86).
- Précisé à l'étape 40, en codant : le Baron pendu compte comme vaincu une fois le bilan de son combat lu ; un Squelette qui gagne un niveau pendant ce combat ne se voit donc pas proposer l'évolution tout de suite, mais peut évoluer au camp, à la demande. Une Armure hantée de niveau 18 qui ne gagne pas de niveau dans la Vallée n'évolue pas, et ne peut pas évoluer au camp. La fiche du codex dit, par exemple, "Évolution : Chevalier noir dès le niveau 18, en gagnant un niveau dans la zone Vallée des Brumes".
- Écrans, à l'étape 38 : après un combat, la question "Salamandre peut évoluer en Lézard de braise. Faire évoluer Salamandre ?", par oui ou non dans la console, par les boutons "Évoluer" et "Pas maintenant" sur Discord. Au camp, dans la console, la gestion des monstres propose "Faire évoluer un monstre", qui liste les monstres qui le peuvent et leur évolution, et "Vendre un monstre à Malvoise" passe au numéro 12 ; sur Discord, la fiche d'un monstre qui le peut montre le bouton "Évoluer en" suivi de l'espèce. La confirmation dit que l'évolution est définitive. Le récit annonce l'évolution, l'attaque apprise, et rappelle que d'autres attaques s'apprennent au camp. La fiche du codex dit l'évolution et sa condition ; sur Discord, le menu des fiches se coupe en menus de 25 espèces.

### 5.5 Personnalisation

Le joueur peut :

- renommer son monstre : un surnom de 1 à 20 caractères, affiché à la place du nom de l'espèce ;
- ajuster ses attaques, au camp : réapprendre une attaque de son espèce ou en apprendre une nouvelle grâce à une capsule d'attaque (section 13.10) ;
- ajuster ses statistiques à l'aide d'objets ; depuis l'étape 67, au camp, avec les boosters, qui augmentent une valeur interne (section 14.3) ;
- depuis l'étape 35, changer son habileté pour l'autre de son espèce, au camp, avec une capsule d'habileté (section 5.9) ;
- consulter le codex, par exemple pour savoir quand il évolue. Depuis l'étape 38, la fiche d'une espèce capturée dit son évolution et sa condition (section 5.4) ;
- depuis l'étape 37, le faire évoluer au camp, depuis la gestion des monstres, quand il en remplit la condition (section 5.4).

### 5.6 Codex

- Le codex répertorie tous les monstres du jeu (comme le Pokédex de Pokémon).
- Une entrée est déverrouillée partiellement à la rencontre d'un monstre, et entièrement à sa capture.
- Dans la V1, le codex se consulte au camp. Une espèce rencontrée en expédition montre son nom et ses types ; une espèce capturée montre aussi ses stats de base et la liste des attaques qu'elle apprend. Le monstre de départ compte comme capturé.
- La fiche d'une espèce capturée rappelle aussi ses types. Depuis l'étape 28, elle montre aussi ses 2 habiletés possibles (section 5.9).
- Après la V1 : une espèce seulement rencontrée ne montre plus que son nom ; ses types n'apparaissent qu'à sa capture, comme en combat (section 10.6). La fiche d'une espèce capturée montre aussi le total de ses stats de base.
- Depuis l'étape 66, un Apôtre ne se capture plus : sa victoire ouvre sa fiche, et la liste du codex le dit "Apôtre vaincu" au lieu de "capturée" (section 9.4).
- Précisé à l'étape 65, en codant : avec les 41 espèces des monstres aquatiques, la liste du codex et une fiche ne tiennent plus ensemble dans un message Discord (2000 caractères). Sur Discord, une fiche ouverte remplace donc la liste, qu'un bouton "Liste du codex" rouvre ; la console garde la liste et la fiche.
- Précisé à l'étape 82, en codant : avec les 52 espèces du Col du Givre-Noir, la liste du codex ne tient plus dans un message Discord ; elle s'y lit par pages de 30 espèces, avec les boutons "Page précédente" et "Page suivante".
- Décidé le 25/10/2026, pour l'étape 107 (choix de l'utilisateur) : le codex a une section à part, le carnet de traque, pour ne rien dévoiler des Apôtres qui restent.
  - Les Apôtres et le Démiurge quittent la liste des espèces, où ils apparaissaient en "???" tant qu'ils n'étaient pas vaincus, ce qui révélait leur nombre. Ils n'apparaissent plus qu'au carnet.
  - Le carnet n'apparaît qu'au premier Apôtre abattu. Chaque victoire y ajoute une entrée, dans l'ordre des victoires, comme si le héros écrivait le nom de ses ennemis abattus l'un après l'autre : "1. Baron pendu, abattu dans la Vallée des Brumes." Le Démiurge y a une seule entrée pour ses deux formes, après les trois Apôtres qu'il faut abattre avant lui. Ni les Apôtres qui restent ni leur nombre n'y figurent. Textes dans docs/contenu-v1.md, section 27.
  - La fiche d'un Apôtre abattu reste consultable : dans la console, sous son entrée du carnet (les deux fiches, sous leur nom, pour le Démiurge) ; sur Discord, par le menu des fiches, comme avant. Sur Discord, le carnet suit la liste, à sa dernière page (hypothèse retenue, Q103).
- Décidé le 25/10/2026, pour l'étape 115 (choix de l'utilisateur) : la complétion du codex est récompensée.
  - Paliers : toutes les 10 espèces capturées (10, 20, 30, 40 et 50), puis le codex complet, aujourd'hui 51 espèces. Seules comptent les espèces que le codex dit capturées (le monstre de départ, les captures, les achats, la pêche, les petits du rite) ; les Apôtres et le Démiurge, hors de la liste, ne comptent pas.
  - Le joueur réclame chaque récompense depuis l'écran du codex, au camp, sans confirmation, un palier à la fois, dans l'ordre. Elle va dans le coffre ; le talisman du codex complet rejoint l'équipement du héros. Le codex montre le nombre d'espèces capturées sur le total, la récompense à réclamer ou celle du prochain palier.
  - Récompenses : 10 filets lestés et 300 pièces d'argent ; 5 filets de chaînes et 500 pièces d'argent ; 1 Philtre chromatique ; 3 capsules d'habileté ; un booster de chaque sorte ; puis 3 Philtres chromatiques et le Sceau du bestiaire, talisman unique (+5 % à l'Attaque, la Défense, l'Attaque spéciale, la Défense spéciale et la Vitesse), qui ne se fabrique pas (docs/contenu-v1.md, section 30).
  - Une sauvegarde antérieure n'a rien réclamé : les paliers qu'elle a déjà atteints se réclament à la reprise. Si des espèces s'ajoutent, le codex complet suit leur nombre, et une récompense reçue reste acquise (hypothèses retenues, Q106). Sauvegarde en version 29.
  - Écrans : dans la console, après la liste du codex, la progression, puis "Réclamer la récompense : 10 espèces capturées" quand un palier attend ; sur Discord, la progression sous le titre de chaque page de la liste, et un bouton du même libellé.

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
- Depuis l'étape 28, un shiny a 31 dans chaque valeur interne (section 5.1).
- Depuis l'étape 67, un booster ajoute 5 à une valeur interne, sans dépasser 31 (section 14.3).

### 5.9 Habiletés (après la V1)

Décidé le 19/10/2026, pour l'étape 28 :

- Chaque monstre a une habileté : un bonus passif. Chaque espèce a 2 habiletés possibles, et chaque monstre tire la sienne à sa création, monstres de départ, sauvages et Apôtres compris. La liste par espèce est dans docs/contenu-v1.md, section 7.
- À la reprise d'une sauvegarde antérieure, un monstre reçoit la première habileté de son espèce : une habileté tirée à la lecture changerait d'une reprise à l'autre.
- Les monstres sauvages et les Apôtres ont aussi leur habileté, mais seules les habiletés de combat jouent pour eux.
- Les habiletés de stats s'ajoutent au bonus de la classe (section 4.2) : le monstre Férocité d'un Guerrier a +20 % d'Attaque.
- Les habiletés d'équipe (Pillard, Éclaireur, Fouineur) ne se cumulent pas : deux monstres qui ont la même valent un seul.
- Les valeurs sont prudentes, à ajuster en jouant. Les arrondis se font à l'entier inférieur.

| Habileté | Effet |
|---|---|
| Férocité | +10 % d'Attaque en combat |
| Carapace | +10 % de Défense en combat |
| Sorcellerie | +10 % d'Attaque spéciale en combat |
| Volonté | +10 % de Défense spéciale en combat |
| Célérité | +10 % de Vitesse en combat |
| Affinité | ses attaques qui ont le bonus de type (section 10.4) infligent 20 % de dégâts en plus |
| Instinct | il gagne 20 % d'XP en plus, en combat comme en expédition automatique |
| Éclaireur | l'expédition automatique où il part dure 20 % de moins, après la réduction due au nombre de monstres (section 12.2) |
| Traqueur | quand il est le monstre actif, la chance de capture au filet est multipliée par 1,25, sans dépasser 100 % (section 11.2) |
| Pillard | s'il est dans l'équipe sans être KO, l'argent d'un coffre augmente de 50 % ; s'il rentre debout d'une expédition automatique réussie, l'argent qu'elle rapporte aussi |
| Fouineur | après un combat gagné, s'il est dans l'équipe sans être KO, 1 chance sur 4 de trouver une potion, qui va dans le sac |

- Affichage : la fiche d'un monstre, dans sa gestion et l'écran de ses attaques, montre son habileté et son effet ; la fiche d'une espèce capturée au codex montre ses 2 habiletés possibles. Le bilan d'un coffre ou d'une expédition automatique signale l'argent dû au Pillard, et la potion du Fouineur est annoncée après le combat.

Capsule d'habileté, décidée le 19/10/2026 pour l'étape 35 :

- Une capsule d'habileté, unique pour toutes les espèces, fait passer un monstre à l'autre habileté de son espèce. Elle se consomme, contrairement aux capsules d'attaque (section 13.10).
- Elle s'achète chez Malvoise, 200 pièces d'argent, et se trouve dans un coffre sur 10 (section 9.2). Elle se range dans le sac ou le coffre, se déplace par lots comme les potions et se perd avec le sac (section 9.3). Prix et taux sont des hypothèses, à confirmer (Q81).
- Elle s'utilise au camp, depuis la gestion des monstres, après confirmation, qui annonce l'ancienne et la nouvelle habileté. Elle se prend dans le sac d'abord, puis dans le coffre, comme l'argent (section 13.3). Elle est refusée pour un monstre dont l'espèce n'a pas deux habiletés différentes, ou pour un monstre parti en expédition automatique.
- Écrans, à l'étape 35 : dans la console, la gestion des monstres propose "Changer l'habileté d'un monstre", avec le nombre de capsules, l'étal de Malvoise "Acheter des capsules d'habileté" au nombre choisi, et le coffre les déplace au nombre choisi. Sur Discord, la fiche d'un monstre montre un bouton "Passer à" suivi de l'autre habileté quand le joueur a une capsule, l'étal un bouton d'achat à l'unité, et les menus du coffre un lot d'une capsule et le tout. Le coffre trouvé et le camp disent les capsules d'habileté du sac et du coffre.

## 6. Histoire et démarrage

- Une histoire, simple pour le moment.
- Un petit tutoriel de départ.
- Fil de l'histoire, inspiré de Berserk : le héros veut se venger du Démiurge, maître des Apôtres, de puissants monstres. Pour l'atteindre, il traque les Apôtres un par un, de zone en zone : dans chaque zone, il réunit des indices sur la cachette d'un Apôtre, s'en approche peu à peu, puis l'affronte avant de passer au suivant (section 9.4).
- Dans la V1, un seul Apôtre est à vaincre, dans la zone difficile. Le Démiurge est annoncé dès l'introduction, mais il ne sera affronté que dans une version suivante. Après la victoire contre l'Apôtre, un court épilogue annonce la suite de la traque, puis le joueur peut continuer à explorer, capturer et compléter son codex.
- Depuis les étapes 88 à 92 (21/10/2026), une fois les trois Apôtres vaincus, le Béhérit pourpre, forgé de leurs âmes, permet d'invoquer le Démiurge et de l'affronter dans l'Éclipse (section 9.4).
- Dans la V1, après le choix de la classe, l'histoire amène le joueur à choisir son premier monstre parmi 3. Ce monstre est au niveau 5. Ses valeurs internes sont tirées au hasard et il a, comme un monstre sauvage, 1 chance sur 512 d'être shiny.
- Après la V1, le premier monstre n'est plus choisi : chaque classe a son monstre attitré, qui reprend son profil. Guerrier : Crapaud, offensif en attaque physique. Mage : Salamandre, offensive en attaque spéciale. Templier : Mandragore, défensive. Depuis l'étape 27, Assassin : Chat noir, rapide et offensif en attaque physique, mais fragile. Il reste au niveau 5, avec des valeurs internes tirées au hasard et 1 chance sur 512 d'être shiny. L'écran du choix de la classe annonce le monstre de chacune. Le dernier paragraphe de l'introduction présente le monstre de la classe choisie (Q67, textes dans docs/contenu-v1.md).
- Dans la V1 : une courte introduction d'ambiance après le choix de la classe, jusqu'au choix du premier monstre ; un tutoriel fait de quelques conseils, chacun montré une seule fois par partie, au premier combat, au premier arrêt entre deux étapes et au premier retour au camp. Les textes sont proposés dans docs/contenu-v1.md.

## 7. Progression

- Les monstres gagnent de l'expérience et montent de niveau, du niveau 1 au niveau 100.
- Le monstre de départ est au niveau 5.
- Courbe d'expérience, la même pour toutes les espèces : il faut n x n x n XP au total pour atteindre le niveau n (125 au niveau 5, 216 au niveau 6, 1 000 000 au niveau 100). Un monstre créé au niveau n commence avec n x n x n XP. Au niveau 100, il ne gagne plus d'XP.
- Après la V1 (19/10/2026), la fiche d'un monstre et chaque gain d'XP montrent son XP totale et celle qu'il faut au total pour le niveau suivant : un Loup de niveau 5 qui a 130 XP affiche « XP : 130/216 », et un gain s'écrit « Loup gagne 45 XP (175/216). ». Au niveau 100, « niveau maximum » remplace l'XP du niveau suivant.
- À chaque niveau gagné, les stats sont recalculées et les PV actuels augmentent d'autant que les PV max.
- XP rapportée par un monstre sauvage vaincu, c'est-à-dire mis KO ou capturé : rendement de son espèce x son niveau / 7, multiplié par un pourcentage tiré de 90 % à 110 %, en divisions entières. Exemple : un monstre de niveau 5 et de rendement 64 rapporte de 40 à 49 XP.
- Chaque monstre de l'équipe qui a été actif face au monstre vaincu et qui n'est pas KO à la fin du combat reçoit cette XP en entier. Depuis l'étape 28, un monstre qui a l'habileté Instinct en reçoit 20 % de plus, sur le total du combat (section 5.9).
- L'XP est versée à la fin du combat, quelle que soit son issue ; les montées de niveau et l'apprentissage d'attaques se font donc après le combat, jamais pendant.
- Chaque espèce a une liste d'attaques apprises à des niveaux donnés. En atteignant un de ces niveaux, le monstre apprend l'attaque ; s'il en connaît déjà 4, le joueur choisit l'attaque à oublier ou renonce à la nouvelle. Un monstre créé à un niveau donné connaît les 4 dernières attaques de sa liste jusqu'à ce niveau.
- Une expédition automatique réussie rapporte aussi de l'XP (section 12.6).
- Depuis l'étape 37, après les attaques apprises, un monstre qui remplit la condition de son évolution se la voit proposer (section 5.4).
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
- Décidé le 19/10/2026, pour les étapes 44 à 47, après les améliorations du camp : une quatrième zone, l'Abbaye des Cendres (niveaux 21 à 27), s'ouvre avec la défaite du Baron pendu (section 9.4).
- Décidé le 20/10/2026, pour les étapes 79 à 82 (choix de l'utilisateur) : une cinquième zone, le Col du Givre-Noir (niveaux 28 à 36, 9 niveaux pour que les stades 3 du niveau 36 s'y atteignent), s'ouvre avec la défaite de l'Abbesse écorchée (section 9.4).

### 9.2 Événements

Lors d'une expédition, les événements suivants peuvent se produire :

- un combat contre un ou plusieurs monstres, ou contre un humain (section 10) ;
- la rencontre d'un marchand (section 14.2) ;
- rarement, la rencontre d'un monstre rare. Selon son espèce, il attaque (combat, capture possible) ou non (le joueur peut alors le prendre gratuitement ou non) ;
- rarement, la découverte d'un coffre, que le joueur peut ouvrir pour obtenir des ressources offertes.

Déroulement dans la V1 :

- Une expédition compte de 4 à 6 étapes, nombre tiré au départ et inconnu du joueur.
- Après la V1, le nombre d'étapes est fixe pour chaque zone et connu du joueur, affiché en haut de l'écran d'expédition sous la forme « [Étape : 2/3] ». Il suit la difficulté : 3 dans la première zone, 4 dans la deuxième, 5 dans la troisième et la quatrième, 6 dans la cinquième et la sixième, puis 7 au plus.
- Remplacé le 19/10/2026, pour l'étape 33 : toute zone compte 5 étapes, les zones à venir comprises.
- Chaque étape tire un événement : combat contre un groupe sauvage (80 %), coffre (15 %) ou monstre rare (5 %). Le marchand est hors V1.
- Depuis l'étape 27 (19/10/2026), quand une étape tire un monstre rare, c'est le Schnoz, monstre caché, une fois sur 20, dans toute zone ; sinon, c'est un monstre rare de la zone, comme avant. Il se rencontre donc environ une étape sur 400. Pacifique, il se laisse prendre ; son niveau est tiré parmi ceux de la zone, et il peut se rencontrer de nouveau. Au codex, il reste inconnu tant qu'il n'est pas rencontré, comme toute espèce.
- Après la V1, tant qu'aucun marchand n'est au camp, la rencontre d'un marchand prend 10 % sur les combats : combat 70 %, marchand 10 %, coffre 15 %, monstre rare 5 % (section 14.2). Une fois le marchand au camp, les combats retrouvent leurs 80 %.
- Depuis l'étape 31, le marchand ne se rencontre plus : c'est l'âme du marchand mort de la zone qui prend ces 10 %, tant qu'elle n'est pas recueillie et une fois Malvoise au camp (section 14.2, Q80).
- Remplacé le 19/10/2026, pour les étapes 33 et 34 (section 14.2) : l'âme d'un marchand se rencontre même sans Malvoise au camp. Celle de Bertrand est garantie à l'étape 4 de chaque expédition dans la Lisière, tant qu'elle n'est pas recueillie : le brigand de l'étape 5 l'abat sous les yeux du joueur. Celles de Grete et d'Ulrich gardent leurs 10 % tant qu'elles n'ont pas été rencontrées. Après une fuite ou une défaite face à leur gardien, elles sont à retrouver : la prochaine expédition dans leur zone les rencontre à coup sûr, à une étape tirée au départ parmi celles qui précèdent la dernière, sans tirage des 10 %.
- Un groupe sauvage compte de 1 à 2 monstres dans la zone facile, de 1 à 3 dans la zone difficile ; la taille est tirée à chaque combat.
- Un coffre contient 1 à 2 potions (section 14.4). Après la V1, il contient aussi de l'argent, selon la zone, et, une fois sur 5, une capsule d'attaque de la zone que le joueur n'a pas encore (sections 13.10 et 14.5). Depuis l'étape 28, l'habileté Pillard augmente cet argent de 50 %, et l'habileté Fouineur peut trouver une potion après un combat gagné (section 5.9).
- Depuis l'étape 35 (19/10/2026), un coffre contient aussi, une fois sur 10, une capsule d'habileté (section 5.9), tirée après la capsule d'attaque.
- Depuis l'étape 103 (25/10/2026), le même tirage donne des filets une fois sur 5 : 2 filets de chanvre (15 %) ou un filet lesté (5 %), dans les zones comme dans la Tour sans fond (section 11.2).
- Depuis l'étape 36 (19/10/2026), après un combat gagné, chaque monstre sauvage mis KO et non capturé lâche sa ressource 1 fois sur 3 ; elle va dans le sac (section 14.3).
- Entre deux étapes, le joueur continue, rentre au camp en gardant ses gains, utilise des potions ou change l'ordre de son équipe ; les échanges avec le camp restent impossibles. Les PV et les statuts sont conservés d'une étape à l'autre.
- Après la V1, l'écran entre deux étapes rappelle l'argent et les capsules trouvés pendant l'expédition, qu'une défaite ferait perdre (section 9.3). Avec le coffre du camp, il montre le contenu du sac (section 13.3).
- Décidé le 18/10/2026, pour l'étape 30 : un événement important (un indice, un humain à vaincre en fin de zone, un Apôtre) est d'abord présenté par un texte et un bouton (« Combattre », « Lire l'indice ») avant de se produire, pour que le joueur ne le rate pas en enchaînant les actions.
- Après la dernière étape, le joueur rentre au camp et la zone compte comme explorée.
- Décidé le 20/10/2026, pour l'étape 63 (choix de l'utilisateur) : tant qu'Ansgar, le pêcheur, n'est pas au camp, il se rencontre à coup sûr à l'étape 2 de chaque expédition dans le Marais des Pendus, à la place de l'événement tiré. Ce qu'il vient de ferrer, un Noyé de niveau 13 (le niveau maximal du Marais, comme le Loup de Bertrand), l'attaque : un combat ordinaire contre un monstre sauvage, fuite et filet compris, présenté par son texte comme celui d'une âme, sans bouton d'annonce. Une victoire l'installe au camp, acquis dès ce moment même si l'expédition finit en défaite, comme une âme recueillie ; une fuite ou une défaite le laisse à sauver à l'expédition suivante. Le Noyé ne lâche pas de ressource. L'âme de Grete garde ses 10 % sur les autres étapes ; à retrouver, son étape est tirée parmi les étapes 1, 3 et 4 (hypothèse retenue, Q91). Une sauvegarde qui a déjà passé le Marais rencontre Ansgar à sa prochaine expédition dans le Marais. Textes dans docs/contenu-v1.md, section 15.
- Annonces, depuis l'étape 30 : le combat contre un humain de fin de zone ou contre un Apôtre est d'abord présenté par son texte et un bouton "Combattre" ; un indice, au retour au camp, par un bouton "Lire l'indice". Dans la console, ce bouton est l'unique choix d'un menu.

### 9.3 Défaite

Si tous les monstres du joueur sont KO pendant une expédition manuelle :

- le joueur retourne automatiquement au camp ;
- il perd les objets qu'il avait sur lui, dont toutes ses potions ;
- après la V1, il perd aussi l'argent et les capsules trouvés pendant l'expédition, qui ne lui sont acquis qu'au retour au camp ; l'argent qu'il avait déjà au camp lui reste ;
- avec le coffre du camp (étape 26, section 13.3), il perd tout ce que contient son sac : potions, argent et capsules, qu'il les ait emportés ou trouvés en chemin. Ce qui est rangé dans le coffre est à l'abri. Il garde ses captures et l'XP gagnée ;
- ses monstres restent KO et sont soignés au camp (section 5.7) ;
- c'est aussi une défaite si son dernier monstre et le dernier monstre sauvage tombent KO au même tour ;
- depuis l'étape 34, il garde les âmes des marchands qu'il porte : elles ne sont pas dans le sac (section 14.2). Les capsules d'habileté et les ressources du sac se perdent avec lui (étapes 35 et 36) ;
- l'expédition ne compte pas comme menée jusqu'au bout.

### 9.4 Traque d'un Apôtre

- Chaque expédition menée jusqu'à sa dernière étape révèle un indice sur la cachette de l'Apôtre traqué : un court texte d'histoire.
- Dans la V1, la piste commence dans la zone facile, qui n'a pas d'Apôtre : le premier indice, au bout de la première expédition complète dans la zone facile, désigne la zone difficile et l'ouvre. Les indices 2 et 3 viennent des expéditions menées au bout dans la zone difficile.
- Au 3e indice, l'antre est localisé : l'expédition suivante dans la zone s'achève par le combat contre l'Apôtre, à sa dernière étape et à la place de l'événement tiré. Rentrer au camp avant cette étape repousse le combat à l'expédition suivante.
- Combat contre un Apôtre : il combat seul, 2 niveaux au-dessus du niveau maximal de sa zone (au niveau maximal pour le Baron pendu, depuis l'étape 117), avec des stats de base bien plus hautes que celles des monstres communs (environ 480). Ses valeurs internes sont fixes, au maximum : 31 pour chaque stat. Comme un monstre sauvage, il a 1 chance sur 512 d'être shiny. La fuite est impossible, et le filet aussi : on ne lance pas de filet sur un Apôtre pendant le combat.
- Une défaite contre l'Apôtre est une défaite ordinaire (section 9.3) ; les indices restent acquis et l'on peut retenter.
- Rappelé le 19/10/2026 par l'utilisateur : chaque combat contre un humain de fin de zone (le brigand, le Bourreau) ou contre un Apôtre est unique, comme la rencontre d'une âme de marchand. Une fois vaincu, il ne revient plus : à son étape, une expédition suivante dans la zone tire un événement ordinaire (combat contre un groupe sauvage, coffre, monstre rare).
- Après la victoire, l'Apôtre, mis KO, peut être capturé, à coup sûr, comme tout monstre KO à la fin d'un combat gagné (section 11.2) : c'est la seule façon de le capturer. Capturé ou non, il ne réapparaît plus. Remplacé le 20/10/2026, à l'étape 66 : voir les âmes d'Apôtres ci-dessous.
- Décidé le 20/10/2026, pour l'étape 66 (choix de l'utilisateur) : les Apôtres ne se capturent plus, même KO après la victoire. Vaincu, un Apôtre laisse son âme : après l'épilogue, un court texte dit que le joueur la recueille (docs/contenu-v1.md, section 16). Il ne réapparaît plus, comme avant.
  - Une âme d'Apôtre est gardée à part, ni dans le sac ni dans le coffre, et rien ne la fait perdre. Elle ne se donne pas à Malvoise : son usage est différent de celui des âmes des marchands et sera défini plus tard (Q92). Idée de l'utilisateur : réunir les âmes de tous les Apôtres pour créer un Béhérit pourpre, qui permettra d'invoquer le Démiurge et de l'affronter.
  - Le camp rappelle les âmes d'Apôtres gardées, sur une ligne à part de celles des marchands.
  - La victoire ouvre la fiche de l'Apôtre au codex, comme une capture ; la liste du codex le dit "Apôtre vaincu" (section 5.6). Remplacé le 25/10/2026, à l'étape 107 : l'Apôtre abattu n'est plus dans la liste, mais au carnet de traque (section 5.6).
  - Depuis l'étape 67 (choix de l'utilisateur), l'Apôtre vaincu laisse aussi un Philtre chromatique, après son âme, qui va dans le sac (section 14.3).
  - Choix de l'utilisateur pour les sauvegardes antérieures : un Apôtre déjà capturé quitte l'équipe, le camp ou son expédition automatique, et le joueur garde son âme, comme celle de tout Apôtre vaincu ; la fiche de chaque Apôtre vaincu s'ouvre au codex (section 13.9, hypothèses de Q92).
- Décidé le 18/10/2026, pour l'étape 30 : un indice par zone, trouvé à la première expédition menée au bout dans cette zone. La première zone s'achève par un combat contre un humain et ses monstres (étape 29). La deuxième s'achève par un humain plus fort, au service de l'Apôtre, qui donne l'indice de la troisième zone ; l'Apôtre attend à la fin de la troisième. Les détails sont dans Q75.
- Précisé le 19/10/2026, pour l'étape 30 (Q75), et remplace la piste de la V1 décrite plus haut :
  - la piste du Baron pendu compte deux indices. L'indice 1 vient de la première expédition menée au bout dans la Lisière, où attend le brigand (section 10.7) ; il ouvre le Marais. L'indice 2 vient de la première expédition menée au bout dans le Marais, où attend le Bourreau, au service du Baron pendu ; il ouvre la Vallée des Brumes (niveaux 14 à 20) ;
  - l'antre est alors localisé : chaque expédition dans la Vallée s'achève par le combat contre le Baron pendu, à sa dernière étape, tant qu'il n'est pas vaincu ;
  - le Baron pendu devient de type Obscur seul, au niveau 22 (le maximum de la Vallée plus 2). Ses stats ne changent pas ; il apprend deux attaques Obscur. Un Baron pendu déjà capturé suit ce changement (section 13.9). Remplacé le 25/10/2026, à l'étape 117 : voir ci-dessous ;
  - les sauvegardes de l'ancienne piste, qui ont trouvé de 1 à 3 indices, reprennent à 1 indice : le Marais reste ouvert, et il faut vaincre le Bourreau pour ouvrir la Vallée. Un Apôtre déjà vaincu le reste et n'attend plus au bout de la Vallée.
- Décidé le 25/10/2026, pour l'étape 117 (choix de l'utilisateur, d'après docs/equilibrage.md) : premier Apôtre, le Baron pendu combat au niveau maximal de la Vallée, 20, et non 2 niveaux au-dessus, pour que la difficulté monte d'un Apôtre à l'autre. Ses stats ne changent pas. À ce niveau, il ne connaît qu'une attaque Obscur, Étreinte obscure, avec Lamentation, Sentence et Trappe du gibet : Hurlement des abysses vient au niveau 21. Les Apôtres suivants restent 2 niveaux au-dessus du maximum de leur zone. Mesuré par l'outil d'équilibrage : 2,6 tentatives au lieu de 4,8, et 36 % de victoires au premier essai au lieu de 12 %.
- Décidé le 19/10/2026, pour les étapes 44 à 47 (contenu validé par l'utilisateur, docs/contenu-v1.md, section 10) :
  - la traque compte plusieurs Apôtres, chacun avec sa piste ; les indices se numérotent à la suite. Les indices de la piste d'un Apôtre ne se trouvent qu'une fois l'Apôtre précédent vaincu ;
  - l'indice 3 se trouve au bout de l'expédition de la Vallée qui abat le Baron pendu, et ouvre l'Abbaye des Cendres (niveaux 21 à 27). Une sauvegarde où le Baron est déjà vaincu le trouve au bout de sa prochaine expédition menée au bout dans la Vallée (hypothèse retenue, Q86) ;
  - l'Inquisiteur, humain au service du deuxième Apôtre, attend au bout de l'Abbaye ; la première expédition menée au bout donne l'indice 4, qui localise l'antre ;
  - ensuite, chaque expédition dans l'Abbaye s'achève par l'Abbesse écorchée, Apôtre de type Obscur, au niveau 29, tant qu'elle n'est pas vaincue ;
  - l'Abbaye n'a pas de marchand mort pour l'instant (hypothèse retenue, Q86).
  - précisé à l'étape 45, en codant : l'Abbaye et ses espèces existent dans le jeu, mais elle reste fermée sur la carte jusqu'à l'étape 46, qui ajoute l'indice 3. Au codex, les espèces de l'Abbaye viennent après les formes évoluées. Sur Discord, avec ses trois capsules et ses quatre ressources, le coffre ne propose plus de prendre une capsule d'attaque, qui sert où qu'elle soit (section 13.3), et les ressources s'y déplacent par sorte entière, pour que ses menus restent sous 25 options.
  - précisé à l'étape 44, en codant : la piste se lit chapitre par chapitre, un par Apôtre. Le combat contre un Apôtre suit l'Apôtre qui attend dans la zone de l'expédition ; une sauvegarde garde le nombre d'indices trouvés, qui se compte à la suite d'un chapitre à l'autre, sans changement de format.
- Décidé le 20/10/2026, pour les étapes 79 à 82 (contenu validé par l'utilisateur, docs/contenu-v1.md, section 21), sur le modèle du chapitre de l'Abbesse :
  - l'indice 5 se trouve au bout de l'expédition de l'Abbaye qui abat l'Abbesse écorchée, et ouvre le Col du Givre-Noir (niveaux 28 à 36). Une sauvegarde où l'Abbesse est déjà vaincue le trouve au bout de sa prochaine expédition menée au bout dans l'Abbaye (hypothèse retenue, Q97) ;
  - le Porte-étendard, humain au service du troisième Apôtre, attend au bout du Col ; la première expédition menée au bout donne l'indice 6, qui localise l'antre ;
  - ensuite, chaque expédition dans le Col s'achève par le Margrave de givre, Apôtre de type Obscur, au niveau 38, tant qu'il n'est pas vaincu. Vaincu, il laisse son âme et un Philtre chromatique, comme les autres Apôtres ;
  - le Col n'a pas de marchand mort (hypothèse retenue, Q97). Pas de changement de format de sauvegarde.

#### Béhérit pourpre et Démiurge

Décidé le 21/10/2026, pour les étapes 88 à 92 (choix de l'utilisateur, Q92 ; contenu validé, docs/contenu-v1.md, section 23) :

- Une fois les trois Apôtres vaincus, Malvoise fond leurs âmes en un Béhérit pourpre, au camp, depuis son étal, après confirmation. Les âmes sont consommées : le camp ne les rappelle plus, et rappelle le Béhérit à la place. Le Béhérit est gardé à part, ni dans le sac ni dans le coffre, et rien ne le fait perdre.
- Avec le Béhérit, le menu des expéditions propose un lieu à part, l'Éclipse : une expédition d'une seule étape, le combat contre le Démiurge. Ce n'est pas une zone : ni monstre sauvage, ni coffre, ni expédition automatique, ni pièges, ni marchand ; elle ne compte jamais comme explorée.
- Le premier départ vers l'Éclipse est l'invocation du Démiurge : le Béhérit réclame un sacrifice. Le joueur choisit un de ses monstres de niveau 30 ou plus, ni loué, et dont le départ ne laisse pas l'équipe vide ; après confirmation, ce monstre est perdu pour toujours, et l'expédition part aussitôt. Le sacrifice n'est demandé qu'une fois : l'Éclipse reste ensuite ouverte, et le joueur y retourne sans nouveau sacrifice.
- Le combat suit les règles de celui contre un Apôtre : le Démiurge combat seul, sans fuite ni filet, à valeurs internes fixes de 31, présenté par son texte et un bouton "Combattre". Il a deux formes, au niveau 42 : quand la première tombe KO, la seconde se relève à sa place, avec tous ses PV et d'autres attaques ; le joueur ne la voit pas avant. Ses monstres ne sont pas soignés entre les deux. Chaque forme vaincue rapporte son XP.
- Une défaite est une défaite ordinaire (section 9.3) : le sac est perdu, et l'Éclipse reste ouverte.
- Après la victoire : un épilogue, et le Démiurge laisse 3 Philtres chromatiques, dans le sac. Il ne se capture pas et ne revient plus ; l'Éclipse se referme. Ses deux formes s'ouvrent au codex, dites "vaincu". Le joueur continue d'explorer ; d'autres Apôtres pourront venir plus tard.
- Sauvegarde en version 25 : l'avancée du Béhérit (forgé, Éclipse ouverte, Démiurge vaincu).
- Précisé aux étapes 88 à 90, en codant : le sacrifice est sauvegardé aussitôt, avant le départ qu'il déclenche ; l'Éclipse n'est pas parmi les zones de la carte, et ses paramètres d'expédition automatique et de coffre ne servent jamais. En combat, la seconde forme n'est pas comptée parmi les monstres en attente, et son entrée est racontée par le texte de la métamorphose.
- Écrans, aux étapes 91 et 92 : dans la console, l'étal propose de forger le Béhérit après les âmes portées, dans "Donner une âme" ; la carte propose l'Éclipse après les zones, puis, avant le premier départ, la liste des monstres que le Béhérit accepte. Sur Discord, le menu "Donner une âme à Malvoise" propose "Âmes des Apôtres : forger le Béhérit pourpre" ; le menu "Partir en expédition" propose l'Éclipse, qui ouvre son écran : un menu des monstres à sacrifier, ou le bouton "Partir vers l'Éclipse". Le camp rappelle le Béhérit à la place des âmes d'Apôtres.

### 9.5 Tour sans fond

Décidé le 24/10/2026, pour les étapes 96 à 98 (choix de l'utilisateur) :

- La Tour sans fond est un lieu à part, hors des zones de la carte, comme l'Éclipse : une tour engloutie dont les marches s'enfoncent sous terre, sans fin. Le but est d'y descendre le plus loin possible. C'est aussi le seul lieu de l'expédition en groupe (section 15.5). Textes dans docs/contenu-v1.md, section 25.
- Elle se joue seul, dans la console comme sur Discord, et à deux sur Discord. Elle s'ouvre quand la Lisière est menée au bout pour la première fois ; à deux, il faut que les deux joueurs l'aient ouverte.
- Les étapes n'ont pas de fin : le compteur montre l'étape sans total, "[Étape : 12]". Le joueur rentre quand il veut entre deux étapes, avec ses gains, comme dans une zone.
- Difficulté : à l'étape n, les monstres sont au niveau 4 + 2 x n, 100 au plus (14 à l'étape 5, 24 à l'étape 10, 44 à l'étape 20, 100 dès l'étape 48). Les groupes comptent 1 à 3 monstres seul, 2 à 4 à deux. Les espèces sont les espèces communes des zones dont le niveau minimal est atteint (la Lisière dès l'étape 1, le Marais au niveau 8, la Vallée au niveau 14, l'Abbaye au niveau 21, le Col au niveau 28), et les monstres rares, ceux de ces zones, avec le Schnoz une fois sur 20, comme ailleurs.
- Événements : combat 80 %, coffre 15 %, monstre rare 5 %, comme dans une zone ; ni âme de marchand, ni Ansgar, ni humain, ni Apôtre, ni indice.
- Gardiens : aux étapes 10, 20, 30, et ainsi de suite, un gardien remplace l'événement, sans tirage de sa nature : un monstre rare hostile tiré parmi ceux des zones atteintes (Basilic, Vouivre, Chimère, Wyrm de givre), au niveau de l'étape + 3, 100 au plus, seul. Il est présenté par son texte et un bouton "Combattre", comme un humain de fin de zone. Ni fuite, ni filet, ni capture ; son IA est l'IA simple des humains (section 10.7). Une victoire est suivie d'un repos qui soigne tous les monstres, PV, statuts et KO compris. Une défaite est ordinaire.
- Coffres : 1 à 2 potions, de 5 x n à 8 x n pièces d'argent (25 à 40 à l'étape 5, 150 à 240 à l'étape 30), une capsule d'attaque d'une zone que le joueur a menée au bout, avec la chance habituelle, et les mêmes chances de capsule d'habileté, de pierre et de booster que dans une zone (sections 9.2 et 14.3).
- Pour le reste, la Tour suit les règles d'une expédition manuelle : fuite (sauf face au gardien), capture, ressources, sac, défaite (section 9.3). Elle compte une expédition pour un monstre loué (section 14.2) et pour le conseil du tutoriel. Elle n'est jamais explorée, ne joue pas sur l'obéissance (section 15.2), la cour d'entraînement ni les pièges (section 13.6), ne se fait pas en expédition automatique, et ne remplit aucune condition d'évolution par lieu (section 5.4).
- Record : la dernière étape franchie, c'est-à-dire la plus haute étape dont l'événement est résolu sans défaite (victoire, fuite, coffre, monstre rare). Une défaite à l'étape 14 donne 13. Chaque joueur garde dans sa sauvegarde son record seul et son record à deux ; à deux, chacun garde le record du groupe. Le retour au camp annonce un nouveau record. Sur Discord, un classement des records, seul et à deux, comme celui des duels (section 15.4). Sauvegarde en version 26.
- Écrans, à l'étape 97 : dans la console, la carte propose la Tour après les zones, une fois ouverte, avec le record seul : "Tour sans fond (sans fin) : record, étape 12", ou "descendre le plus loin possible" sans record. Le compteur montre "[Étape : 12]", le récit "Étape 12.". La console ne montre pas le record à deux.
- Écrans, à l'étape 98 : sur Discord, le menu "Partir en expédition" propose la Tour après les zones, avant l'Éclipse, avec "Sans fin ; record, étape 12" ou "Sans fin ; descendre le plus loin possible". L'écran du classement des duels a un bouton "Records de la Tour", qui ouvre le classement des records : les records du joueur, puis les 10 meilleurs seul et les 10 meilleurs à deux, avec un bouton "Classement des duels" pour revenir. L'écran "Autres joueurs" ne change pas, pour garder de la place aux invitations (section 15.5).

### 9.6 Cycle du jour et de la nuit

Décidé le 25/10/2026, pour les étapes 118 à 120 (choix de l'utilisateur) :

- Le monde alterne 2 heures de jour et 1 heure de nuit, en temps réel, jeu fermé ou bot éteint compris, les mêmes pour tous les joueurs, indépendamment de l'heure du jour réelle. Le cycle part de minuit UTC : la nuit tombe à 2 h, 5 h, 8 h, 11 h, 14 h, 17 h, 20 h et 23 h UTC, pour une heure (hypothèse retenue, Q109).
- Le cycle ne joue que dans les expéditions manuelles des zones de la carte. Ni la Tour sans fond, ni l'Éclipse, ni les expéditions automatiques, ni les pièges, ni les duels n'en dépendent. Dans une zone, les combats de fin de zone (brigand, Bourreau, Inquisiteur, Porte-étendard, Apôtres) n'en dépendent pas non plus : leur difficulté reste celle réglée par l'outil d'équilibrage.
- Une expédition est de jour ou de nuit selon l'heure de son départ, et le reste jusqu'au retour.
- La nuit est plus dangereuse : les monstres des groupes sauvages et les monstres rares ont 2 niveaux de plus que ceux tirés pour la zone, au-delà de son niveau maximal s'il le faut. En échange, chaque monstre vaincu rapporte 25 % d'XP en plus, avant l'habileté Instinct, et l'argent des coffres augmente de 25 %, avant l'habileté Pillard.
- Bonus de types : de nuit, les attaques Ténèbres, Spectre et Poison font 20 % de dégâts en plus ; de jour, les attaques Lumière, Feu et Plante. Le bonus vaut pour les deux camps, après celui de l'habileté Affinité et avant l'efficacité (section 10.4).
- Espèces nocturnes et diurnes : dans les groupes sauvages, une espèce nocturne ne sort que la nuit, une diurne que le jour. Nocturnes : Loup, Feu follet, Fée noire, Goule, Chauve-souris de crypte, Troll des cavernes, Wendigo, Warg. Diurnes : Vipère, Cloche hurlante, Harpie des orages. Les autres espèces, dont les monstres rares, sortent à toute heure. Chaque zone garde au moins deux espèces à chaque moment. Les monstres des âmes de marchands et le Noyé d'Ansgar, fixés à l'avance, ne changent pas, mais le bonus de types et celui d'XP jouent contre eux.
- Évolutions : le Loup évolue en Warg et le Corbeau en Corbeau des gibets au niveau 16, de nuit ; la Vipère en Aspic au niveau 16, de jour. Comme l'évolution par lieu (section 5.4), elle se propose après un combat d'une expédition dans une zone partie au bon moment ; jamais au camp ni dans la Tour.
- Affichage : au camp et sur la carte des expéditions, le moment et le temps avant le suivant ; en expédition, le moment à côté du compteur d'étapes, et un court texte au départ. Textes dans docs/contenu-v1.md, section 31.
- La sauvegarde ne change pas : le moment se déduit de l'heure, et une expédition en cours n'est pas sauvegardée.
- Précisé à l'étape 118, en codant : les zones vérifient que leurs espèces se créent jusqu'à 2 niveaux au-dessus de leur maximum, pour la nuit ; le moment n'entre pas dans les combats de groupe, qui n'ont lieu que dans la Tour.
- Écrans, à l'étape 119 (console) : au camp, sous l'en-tête, et en haut de la carte des expéditions, "Jour : la nuit tombe dans 1 h 12." ou "Nuit : l'aube dans 23 min." ; après "Vous partez pour ...", le texte du moment ; le compteur d'étapes devient "[Étape : 2/5] Nuit", sans moment dans la Tour ; la fiche d'une espèce capturée nocturne ou diurne finit par "Ne sort que la nuit." ou "Ne sort que le jour.", et la condition d'évolution dit "Warg dès le niveau 16, après un combat de nuit".
- Écrans, à l'étape 120 (Discord) : le camp, qui porte le menu "Partir en expédition", dit le moment sous son en-tête ; le texte du départ, le compteur d'étapes et la fiche du codex sont ceux de la console.

### 9.7 Météo

Décidé le 25/10/2026, pour les étapes 121 à 123 (choix de l'utilisateur) :

- Six météos : pluie, tempête, grêle, tempête de sable, cyclone et canicule. Chaque zone de la carte a les siennes. Le cyclone ne se produit que dans une zone de mer et la tempête de sable que dans un désert ; le jeu n'en ayant pas encore, ces deux météos ne se produisent nulle part pour l'instant.
- Météos par zone : Lisière du Bois-Sombre, Marais des Pendus et Abbaye des Cendres : pluie, tempête, canicule ; Vallée des Brumes : pluie, tempête, grêle ; Col du Givre-Noir : grêle, tempête. Zones à venir : une mer, avec pluie, tempête et cyclone ; un désert, avec canicule et tempête de sable.
- Le temps se découpe en créneaux de 30 minutes réelles, les mêmes pour tous les joueurs, jeu fermé ou bot éteint compris. À chaque créneau, chaque zone a 1 chance sur 3 d'avoir une météo, tirée parmi les siennes, et sinon un temps calme. Le tirage dépend seulement du créneau et de la zone : tous les joueurs voient le même ciel, sans sauvegarde. Deux créneaux de suite peuvent tirer la même météo, qui dure alors plus longtemps.
- Une expédition garde la météo de son départ jusqu'au retour, comme le moment du jour (section 9.6). La Tour sans fond, l'Éclipse et les expéditions automatiques n'ont pas de météo.
- Chaque météo renforce deux types : leurs attaques font 20 % de dégâts en plus, pour les deux camps. Pluie : Eau, Plante. Tempête : Électrique, Vol. Grêle : Glace, Roche. Tempête de sable : Sol, Roche. Cyclone : Eau, Vol. Canicule : Feu, Sol.
- Le bonus de la météo se cumule à celui du moment du jour, en se multipliant : une attaque Feu de jour sous la canicule fait x1,2 puis x1,2, soit x1,44, en divisions entières, après l'habileté Affinité et avant l'efficacité (section 10.4).
- Contrairement au moment du jour, la météo joue aussi dans les combats de fin de zone, contre les humains et les Apôtres.
- Affichage : sur la carte des expéditions, la météo de chaque zone et le temps qu'elle va encore durer ; en expédition, la météo à côté du moment, après le compteur d'étapes, et un court texte au départ. Textes dans docs/contenu-v1.md, section 32.
- La sauvegarde ne change pas.
- Précisé à l'étape 121, en codant : le tirage d'un créneau se fait sur un générateur propre, dont la graine mêle le numéro du créneau et l'identifiant de la zone ; il ne touche pas aux tirages de la partie. Le temps restant regarde au plus une journée de créneaux en avant. Mesuré par l'outil d'équilibrage, l'effet de la météo sur la durée de la partie et sur les combats de fin de zone reste de l'ordre du bruit (docs/equilibrage.md, section 8).
- Décidé le 25/10/2026, pour l'étape 124 (choix de l'utilisateur, parmi les évolutions proposées par Claude) : trois évolutions selon la météo. La Fée noire évolue en Fée-sorcière au niveau 12, sous la tempête ; la Fée-sorcière en Reine des ronces au niveau 25, sous la pluie ; le Basilic en Basilic royal au niveau 22, sous la canicule. Comme celles du moment du jour (section 9.6), elles se proposent après un combat d'une expédition partie sous cette météo, jamais au camp ni dans la Tour. La fiche dit "Basilic royal dès le niveau 22, après un combat sous la canicule".
- Écrans, à l'étape 122 (console) : sur la carte, une zone sous une météo finit par ": pluie, encore 42 min" ; le compteur d'étapes devient "[Étape : 2/5] Nuit, pluie", sans météo par temps calme ; le texte de la météo suit celui du moment, au départ.
- Écrans, à l'étape 123 (Discord) : dans le menu "Partir en expédition" du camp, la description d'une zone sous une météo finit par "; pluie, encore 42 min" ; le départ et le compteur d'étapes sont ceux de la console.

## 10. Combat

### 10.1 Principes

- Combat au tour par tour.
- Adversaires possibles : un ou plusieurs monstres, ou un humain (seul ou avec ses monstres, selon la difficulté).
- L'adversaire est contrôlé par une IA (l'ordinateur, et non un vrai joueur). Dans la V1, l'IA d'un monstre sauvage choisit son attaque au hasard.
- Décidé le 18/10/2026, pour les étapes 29 et 30 : les humains combattent avec leurs monstres et une IA qui n'agit pas au hasard. Un humain ordinaire a une IA simple ; l'humain qui termine la deuxième zone, une IA plus relevée ; l'Apôtre, une IA avancée et stratégique (Q74 et Q75). Les règles de ces IA sont dans la section 10.7.

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
  - puis, dans cet ordre et en arrondissant à l'entier inférieur à chaque étape : x1,5 en cas de coup critique, x un pourcentage tiré de 85 % à 100 %, x1,5 pour le bonus de type, x1,2 pour l'habileté Affinité quand l'attaque a le bonus de type (depuis l'étape 28, section 5.9), x l'efficacité (section 5.2) ;
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

### 10.6 Affichage du combat (après la V1)

Décidé le 18/10/2026, pour l'étape 25 :

- Chaque attaque proposée montre les dégâts estimés contre le monstre adverse actif (la fourchette du tirage de 85 % à 100 %, sans coup critique), sa précision, sa catégorie (Phy ou Spé) et son effet secondaire s'il y a la place.
- Sur Discord, la couleur du bouton d'une attaque dit son efficacité : vert pour une attaque très efficace, bleu pour une attaque qui a le bonus de type (section 10.4), gris sinon, rouge pour une attaque peu efficace ou sans effet. Très efficace l'emporte sur le bonus de type, et une attaque peu efficace reste rouge même avec le bonus.
- Les types d'un monstre adverse ne s'affichent que si son espèce est capturée au codex ; sinon, « ? ».
- Quand le joueur peut capturer, le nombre de monstres de son équipe et de son camp est rappelé. Le bouton qui renonce à la capture devient « Ne pas capturer ».
- Sur Discord, sont mis en gras, en plus de la section 2 : les noms des monstres, la fin d'une expédition menée au bout, les indices et les défaites.

### 10.7 Combats contre des humains (après la V1)

Décidé le 19/10/2026, pour l'étape 29 (Q74) :

- Le premier humain est un brigand, au bout de la Lisière du Bois-Sombre. Il remplace l'événement de la dernière étape de chaque expédition dans la Lisière tant qu'il n'est pas vaincu, comme un Apôtre (section 9.4). Un court texte le présente avant le combat.
- Le vaincre achève l'expédition : elle compte comme menée au bout, et la première fois donne l'indice 1, qui ouvre le Marais (section 9.4). Une partie qui a déjà cet indice croise quand même le brigand au bout de la Lisière, sans enjeu pour la traque. Une fois vaincu, il ne revient plus.
- Son équipe : un Loup de niveau 7 et un Corbeau de niveau 8, juste au-dessus des niveaux de la Lisière. Leurs valeurs internes sont fixes, à 15, ils ne sont jamais shiny et ont la première habileté de leur espèce : le combat est le même pour tous. Ses monstres entrent dans l'ordre de son équipe.
- Ni fuite ni filet pendant ce combat. Après la victoire, le joueur peut prendre un seul de ses monstres, au choix et à coup sûr, ou aucun (section 11.3).
- Victoire : 60 pièces d'argent dans le sac, en plus de l'XP habituelle de chaque monstre vaincu (section 7). Une défaite est une défaite ordinaire (section 9.3).
- IA simple d'un humain : son monstre actif choisit l'attaque qui inflige les dégâts estimés les plus hauts au monstre actif du joueur (le haut de la fourchette de la section 10.6, efficacité et bonus de type compris) ; à égalité, la première de sa liste. Il ne change pas de monstre et n'utilise pas de potion. Aucun tirage pour ce choix.
- En combat, ses monstres ne sont pas dits sauvages. Les textes sont dans docs/contenu-v1.md, section 4.

Décidé le 19/10/2026, pour l'étape 30 (Q75) :

- Le Bourreau attend au bout du Marais, de la même façon, avec une Armure hantée de niveau 14, un Feu follet de niveau 14 et un Basilic de niveau 15 (mêmes valeurs internes fixes). Victoire : 150 pièces d'argent ; le vaincre achève l'expédition et donne l'indice 2 la première fois (section 9.4).
- Décidé le 25/10/2026, pour l'étape 117 (choix de l'utilisateur, d'après docs/equilibrage.md) : les monstres du Bourreau passent aux niveaux 13, 13 et 14, le maximum du Marais et un niveau de plus, comme ceux de l'Inquisiteur et du Porte-étendard. Mesuré : 2,0 tentatives au lieu de 2,6, et 38 % de victoires au premier essai au lieu de 21 %.
- IA plus relevée du Bourreau : celle du brigand, mais si son monstre actif n'a que des attaques peu efficaces ou sans effet contre le monstre actif du joueur et qu'un autre de ses monstres, pas KO, a une attaque aux dégâts estimés plus hauts, il change pour celui qui a la meilleure : c'est l'action de son tour, qui passe avant les attaques, comme pour le joueur (section 10.2).
- IA avancée de l'Apôtre : s'il peut mettre KO le monstre actif du joueur, même avec le bas de la fourchette de dégâts, il prend la plus précise des attaques qui le font, la plus forte à précision égale. Sinon, il prend l'attaque aux dégâts moyens attendus les plus hauts (milieu de la fourchette x précision). Une attaque à effet de statut lui est préférée si elle vaut au moins 80 % de cette meilleure attaque et que la cible peut subir son statut. Aucun tirage pour ce choix.

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
  - chaque niveau d'amélioration du filet augmente les chances de capture (section 13.5). Depuis l'étape 41, Gunther améliore les filets : x1,1, x1,2 puis x1,3 (section 13.4).
- Calcul pour un monstre non KO : chance = PV perdus / PV max x facteur, bornée à 100 %. Depuis l'étape 28, elle est multipliée par 1,25 quand le monstre actif du joueur a l'habileté Traqueur, toujours bornée à 100 % (section 5.9). Le facteur vaut le niveau du monstre actif du joueur divisé par celui du monstre sauvage, borné entre x0,5 et x2. Un monstre à PV pleins ne peut donc pas être capturé. Exemple : à niveau égal, un monstre à 2 PV sur 20 est capturé 9 fois sur 10.
- Lancer un filet sur le monstre adverse actif est l'action du tour : si la capture échoue, l'adversaire agit. Un monstre sauvage mis KO peut être capturé à la fin du combat, si le joueur l'emporte.
- Plusieurs monstres peuvent être capturés au cours d'un même combat : chaque monstre d'un groupe sauvage peut l'être, au filet pendant le combat ou, s'il est KO, à la fin d'un combat gagné. Un Apôtre ne se capture que KO, à la fin du combat (section 9.4). Depuis l'étape 66, un Apôtre ne se capture plus : il laisse son âme (section 9.4).
- Les filets de capture sont illimités dans la V1.
- Remplacé le 25/10/2026, pour les étapes 103 à 106 (choix de l'utilisateur) : les filets s'épuisent. Trois qualités : le filet de chanvre (chance de capture normale), le filet lesté (x1,5) et le filet de chaînes (x2). Ce multiplicateur se multiplie avec l'amélioration des filets de Gunther (section 13.4) et l'habileté Traqueur, et la chance reste bornée à 100 %. Valeurs dans docs/contenu-v1.md, section 26.
  - Toute capture en combat consomme un filet : un filet lancé est perdu, que la capture réussisse ou non ; capturer un monstre KO après la victoire, ou prendre le monstre d'un humain vaincu (section 11.3), en coûte un aussi. Sans filet, pas de capture. Le monstre rare pacifique, la pêche, l'achat et la location n'en demandent pas.
  - En combat, le joueur choisit la qualité du filet qu'il lance, parmi celles du sac. La capture d'un monstre KO, garantie, prend le filet le plus simple qui reste (hypothèse retenue, Q102).
  - Les filets se rangent dans le sac et le coffre, comme les potions (section 13.3) : seuls ceux du sac servent en expédition, et une défaite les fait perdre avec le sac. Ceux qu'on achète ou fabrique vont au coffre, ceux qu'on trouve au sac.
  - Une nouvelle partie commence avec 10 filets de chanvre dans le sac. Une sauvegarde antérieure en trouve 10 au coffre (hypothèse retenue, Q102). Sauvegarde en version 27.
  - Sources : Malvoise vend le filet de chanvre et le filet lesté (section 14.2) ; Gunther fabrique les trois qualités par lots de 5 (section 13.5) ; un coffre d'expédition sur 5 en contient (section 9.2). Le filet de chaînes ne s'achète pas.
  - En expédition en groupe, chacun lance ses propres filets et en dépense un pour chaque capture ; un joueur sans filet ne capture plus (section 15.5).
  - Écrans, aux étapes 105 et 106 : dans la console, l'étal de Malvoise liste les filets en vente et propose "Acheter des filets", au nombre choisi (les entrées suivantes avancent d'un numéro) ; la forge propose "Fabriquer des filets", qui liste les trois lots avec leur coût et leur durée et dit les filets possédés ; le coffre propose "Prendre des filets" et "Ranger des filets", au nombre choisi, en 3 et 4 ("Tout ranger dans le coffre" passe au numéro 21). Sur Discord, l'étal a un bouton par filet en vente, qui en achète un lot de 5, à côté des boutons des soins ; la forge a un menu "Fabriquer des filets", qui dit pour chaque lot son coût, sa durée et les filets possédés ; les menus des autres objets du coffre proposent les filets, 5 ou tous. Pour que le camp tienne dans un message Discord quand le joueur a de tout, l'en-tête du camp y compte les ressources par sortes ("17 sortes de ressources") ; leur détail reste à l'écran du coffre, et la console garde l'en-tête complet (hypothèses retenues, Q102).
- Chaque monstre capturé rejoint l'équipe s'il y reste une place, sinon le camp. Si le camp est plein lui aussi, le joueur relâche un monstre de son choix, du camp ou de l'équipe, et la capture prend sa place ; il peut aussi relâcher la capture elle-même, ce qui revient à y renoncer.

### 11.3 Capture après un combat contre un humain contrôlé par l'IA

- Après avoir vaincu un humain qui combat avec ses monstres, le joueur peut capturer le monstre de son choix parmi ceux de l'adversaire. Depuis l'étape 29, il n'en prend qu'un, ou aucun ; pendant le combat, ni fuite ni filet (section 10.7).
- La capture est garantie, puisque ses monstres sont forcément KO.
- Cette règle ne s'applique pas aux combats contre de vrais joueurs (section 15).

## 12. Expéditions automatiques

### 12.1 Principe

- Le joueur peut envoyer de 1 à 3 de ses monstres seuls en expédition, uniquement dans une zone déjà explorée manuellement, c'est-à-dire où il a mené une expédition jusqu'à sa dernière étape (section 9.2). Le départ se fait au camp.
- Les monstres envoyés viennent de l'équipe ou du camp. L'équipe garde toujours au moins un monstre (section 13.2) : il faut donc avoir au moins 2 monstres pour lancer une expédition automatique.
- Toute l'équipe peut partir, à condition qu'au moins un monstre reste au camp : le premier monstre du camp rejoint alors aussitôt l'équipe, qui n'est jamais vide, et le joueur peut repartir en expédition manuelle. Un bouton prépare directement le départ de toute l'équipe ; il n'est proposé que si ce départ est possible.
- Chaque monstre pris dans l'équipe a besoin d'une place libre au camp, où il reviendra ; le monstre du camp qui rejoint l'équipe en libère une. Avec un camp plein, seuls des monstres du camp peuvent donc partir, puisqu'ils gardent leur propre place. Le refus l'explique au joueur.
- Remplacé le 18/10/2026, pour l'étape 26 : l'équipe peut partir tout entière et reste alors vide ; aucun monstre du camp ne la rejoint. Tant qu'elle est vide, aucune expédition manuelle n'est possible. Un seul monstre suffit donc pour une expédition automatique. Chaque monstre pris dans l'équipe a toujours besoin d'une place au camp, où il reviendra. Le bouton « Toute l'équipe » reste proposé quand ce départ est possible.
- Pendant l'expédition, les monstres envoyés sont inutilisables : on ne peut ni les relâcher, ni les renommer, ni les échanger. Ils gardent leur place au camp, dont la limite de 30 monstres les compte, et y reviennent.
- Depuis l'étape 32 (19/10/2026), ils reviennent d'abord dans l'équipe, s'il y a de la place, et sinon au camp (section 12.7).
- Le joueur peut partir en expédition manuelle avec ses autres monstres, y compris dans une zone où une expédition automatique est en cours.
- Le nombre d'expéditions automatiques simultanées n'est pas limité, mais une seule expédition automatique est possible par zone.
- Une expédition automatique n'a aucun effet sur la traque : elle ne donne ni indice ni combat contre un Apôtre, et ne rend pas une zone explorée. Elle ne rencontre ni ne capture aucun monstre : le codex ne change pas.

### 12.2 Durée

- La durée s'écoule en temps réel, y compris quand le jeu est fermé ou le bot éteint : elle se compte à partir de l'heure du départ.
- La durée est fixe, et non tirée au hasard (la section 1.4 vise les dégâts et les gains), pour être connue avant le départ.
- Chaque zone a une durée de base, pour un seul monstre (section 12.8) : plus la zone est difficile, plus l'expédition est longue.
- Chaque monstre envoyé en plus du premier réduit la durée de 25 % de la durée de base : 100 % avec 1 monstre, 75 % avec 2, 50 % avec 3.
- Depuis l'étape 28, si un des monstres envoyés a l'habileté Éclaireur, cette durée baisse encore de 20 % : dans la Lisière, 2 monstres dont un Éclaireur partent 36 minutes au lieu de 45 (section 5.9).

### 12.3 Dégâts et résultat

- Les monstres partent avec tous leurs PV, puisque tous les monstres sont soignés au camp (section 13.1).
- Chaque zone a une fourchette de dégâts (section 12.8). Un total de dégâts est tiré dans cette fourchette, puis réparti à parts égales entre les monstres envoyés, en divisions entières : le reste de la division est perdu.
- Un monstre dont la part atteint ses PV tombe KO. Ce qui dépasse ses PV est réparti à parts égales entre les monstres encore debout, et ainsi de suite. L'expédition échoue donc à peu près quand le tirage atteint le total des PV des monstres envoyés : un monstre faible aide peu, un monstre robuste aide beaucoup.
- Exemple : une Salamandre de 26 PV et un Loup de 16 PV. Avec un tirage de 40, chacun reçoit 20 ; le Loup tombe KO et les 4 dégâts en trop passent à la Salamandre, qui en subit 24 et reste debout : c'est une réussite. Avec un tirage de 50, chacun reçoit 25 ; le Loup tombe KO et ses 9 dégâts en trop portent ceux de la Salamandre à 34 : elle tombe KO à son tour, c'est un échec.
- Les protections réduiront les dégâts subis ; elles sont reportées (section 12.5). Depuis l'étape 76, un monstre protégé reporte une part de ses dégâts sur les autres (section 12.5).
- Succès : au moins un monstre n'est pas KO. L'expédition rapporte de l'XP et des potions (section 12.6).
- Échec : tous les monstres envoyés sont KO. L'expédition ne rapporte rien (section 5.7).
- Le résultat n'est tiré et connu qu'au moment où le joueur accueille l'expédition (section 12.7). Au retour, tous les monstres sont soignés, comme au camp (section 13.1).

### 12.4 Information avant le départ

- Avant de lancer l'expédition, le joueur voit la difficulté de la zone (ses niveaux, section 9.1), sa fourchette de dégâts, la durée et les chances de réussite.
- Les chances de réussite sont calculées exactement à partir de la fourchette de dégâts, du nombre de monstres envoyés et de leurs PV, et depuis l'étape 76 des protections (section 12.5).

### 12.5 Protection

- Des objets peuvent protéger un monstre.
- Certains monstres de soutien peuvent protéger un autre monstre.
- Exemple :
  - une troupe de 3 monstres (a, b, c) est envoyée vers le lieu A ;
  - X monstres ou objets peuvent assurer une protection efficace à 100% à 1 monstre important ;
  - Y monstres ou objets assurent une protection de 75%.
- Les protections sont reportées : tant qu'il n'existe ni objet de protection ni effet du rôle (Q19), aucune protection ne s'applique (Q66).

Décidé le 20/10/2026, pour les étapes 76 à 78 (choix de l'utilisateur, Q19 et Q66) :

- Au départ, le joueur peut désigner un des monstres envoyés comme protégé, s'il en envoie au moins deux (hypothèse retenue, Q96). Sa protection vaut 25 % par monstre de soutien envoyé avec lui (lui-même ne compte pas) et 50 % avec un charme de garde, 100 % au plus. Un seul charme par expédition, pris au départ, du sac d'abord, puis du coffre ; il est consommé même si l'expédition est rappelée (hypothèse retenue, Q96).
- La protection reporte une part des dégâts : à chaque répartition (section 12.3), tant qu'un autre monstre est debout, le protégé ne subit que sa part diminuée de sa protection, et ce qu'il ne subit pas est réparti à parts égales entre les autres monstres debout, en divisions entières, le reste étant perdu. Un monstre qui tombe KO n'encaisse que ses PV, et le surplus est réparti de nouveau, protection comprise. Quand le protégé reste seul debout, il subit tout ce qui reste.
- Exemple : un Loup protégé de 20 PV, une Mandragore de soutien de 30 PV et une Salamandre de 26 PV, sans charme : protection de 25 %. Avec un tirage de 60, chacun devait subir 20 : le Loup en subit 15, et les 5 autres se partagent entre la Mandragore et la Salamandre, 2 chacune, le reste étant perdu. La Mandragore subit 22 et la Salamandre 22.
- Les chances de réussite affichées avant le départ tiennent compte de la protection (section 12.4).
- L'objet de protection est le charme de garde, fabriqué par Gunther (section 13.5).
- Écrans, à l'étape 77 (console) : une fois les monstres choisis, s'il y en a au moins deux, le jeu demande lequel protéger, ou aucun, puis, si le joueur a un charme de garde, s'il le lui donne. La durée, les chances de réussite et la protection s'affichent avant la confirmation. La forge propose "Fabriquer un charme de garde", avec son coût, sa durée et le nombre de charmes possédés ; le coffre déplace les charmes. Le bilan dit le monstre protégé et sa protection, et la part d'argent due aux monstres offensifs.
- Écrans, à l'étape 78 (Discord) : avec au moins deux monstres choisis, l'écran de préparation a un bouton "Protéger {monstre}" pour chacun des autres monstres, puis, un monstre protégé, "Sans protection" et "Avec un charme de garde" ou "Sans charme" ; la protection s'affiche avec la durée et les chances. La forge a un bouton "Charme de garde", avec son coût et sa durée ; le coffre déplace les charmes, un ou tous.
- Précisé à l'étape 77, en jouant : la protection ne change presque pas les chances de réussite, puisque la part reportée reste à encaisser par les autres ; elle sert à garder debout un monstre précis, pour qu'il gagne son XP.

### 12.6 Récompenses

- Chaque monstre qui n'est pas KO au retour d'une expédition réussie reçoit en entier le même gain d'XP, tiré une fois dans la fourchette de la zone (section 12.8) ; les monstres KO ne gagnent rien, comme en combat (section 7). Ce gain vaut environ la moitié de ce que rapporte une expédition manuelle dans la même zone.
- Un nombre de potions est tiré dans la fourchette de la zone et ajouté à celles du joueur (section 14.4).
- Une somme d'argent est tirée dans la fourchette de la zone et ajoutée à celui du joueur (section 14.1).
- Depuis l'étape 28, l'habileté Instinct ajoute 20 % à l'XP du monstre qui l'a, et l'habileté Pillard 50 % à l'argent, si son monstre rentre debout (section 5.9).
- Les ressources et les autres objets (section 14.3) viendront plus tard (sections 13.4 et 14).
- Depuis l'étape 76 (choix de l'utilisateur, Q19) : chaque monstre offensif rentré debout d'une expédition réussie ajoute 10 % à l'argent rapporté. Ces pourcentages s'ajoutent à celui de l'habileté Pillard (hypothèse retenue, Q96) : un Pillard et deux offensifs debout, +70 %.

### 12.7 Retour, accueil et rappel

- Une expédition rentrée attend au camp que le joueur l'accueille. S'il est en expédition manuelle, il l'accueille à son retour au camp. Quand plusieurs expéditions sont rentrées, il les accueille une par une, chacune avec son bilan.
- À l'accueil, le joueur voit le bilan : dégâts subis, monstres KO, XP, potions et argent. Un monstre qui monte de niveau et doit apprendre une attaque alors qu'il en connaît déjà 4 choisit comme après un combat (section 7). Les monstres reviennent au camp, soignés.
- La partie est sauvegardée dès l'accueil, pour que quitter le jeu ne permette pas de tirer de nouveau le résultat. Si le jeu s'arrête avant que le joueur ait choisi les attaques à oublier, ces choix lui sont reproposés à la reprise.
- Au camp, le joueur peut rappeler une expédition avant son terme, après confirmation : les monstres rentrent aussitôt, sans dégâts ni récompense. Une expédition déjà rentrée ne se rappelle plus, elle s'accueille.
- Décidé le 19/10/2026, pour l'étape 32 : à l'accueil comme au rappel, les monstres rentrés rejoignent d'abord l'équipe, à la suite de ceux qui y sont, tant qu'elle compte moins de 3 monstres, dans l'ordre où ils sont partis ; les autres reprennent leur place à la fin du camp. Une équipe vide retrouve ainsi les 3 monstres d'une expédition. Le bilan dit où va chaque monstre. Une attaque à choisir au retour désigne le monstre là où il est allé, dans l'équipe ou au camp.
- Précisé le 19/10/2026 : chaque monstre parti garde sa place réservée au camp (section 12.1), si bien qu'un retour n'est jamais bloqué, même avec l'équipe et le camp pleins : il reprend cette place, ou la libère s'il rejoint l'équipe. Ce fonctionnement est gardé.
- Sur Discord, le bot envoie un message privé au joueur à l'heure du retour, s'il est en ligne ; s'il était éteint, il prévient à son démarrage des expéditions rentrées entre-temps. Chaque retour n'est annoncé qu'une fois, même si le bot redémarre avant que le joueur l'accueille (Q65). Le joueur accueille ensuite l'expédition par /bersemon. La console affiche les expéditions rentrées au camp.

### 12.8 Valeurs par zone

| Zone | Durée de base | Dégâts | XP par monstre non KO | Potions | Argent |
|---|---|---|---|---|---|
| Lisière du Bois-Sombre | 1 h | 15 à 60 | 100 à 200 | 0 à 1 | 15 à 30 |
| Marais des Pendus | 2 h | 40 à 120 | 300 à 600 | 1 à 2 | 40 à 80 |
| Vallée des Brumes (étape 30) | 3 h | 90 à 220 | 700 à 1300 | 1 à 3 | 80 à 150 |

Repères, avec des valeurs internes de 15 : dans la Lisière, une Salamandre de niveau 8 seule réussit 23 fois sur 100, 71 fois avec un Loup de niveau 6, à coup sûr avec en plus une Vipère de niveau 5. Dans le Marais, une Salamandre de niveau 12 seule échoue toujours ; elle réussit 34 fois sur 100 avec un Crapaud de niveau 11, et 77 fois avec en plus une Mandragore de niveau 10. À vérifier en jouant.

## 13. Camp de base

### 13.1 Repos

- Le joueur se trouve au camp de base avant et après chaque expédition.
- Ses monstres y sont soignés automatiquement, y compris ceux qui sont KO.

### 13.2 Gestion des monstres

- Le joueur peut laisser certains monstres en sécurité au camp et en emmener d'autres avec lui.
- Limites de départ : 3 monstres avec le joueur et 30 monstres au camp. Ces limites peuvent être augmentées (section 13.4) : depuis l'étape 41, l'enclos porte le camp à 40, 50 puis 60 places ; à l'étape 43, l'équipe passe à 4 monstres. Les monstres partis en expédition automatique comptent dans la limite du camp (section 12.1).
- Le joueur part en expédition avec 1 à 3 monstres. L'équipe garde toujours au moins un monstre, même au camp.
- Remplacé le 18/10/2026, pour l'étape 26 : au camp, l'équipe peut être vide, après le départ de toute l'équipe en expédition automatique ou quand le joueur la vide lui-même ; il ne peut alors pas partir en expédition manuelle. Il ne peut ni relâcher ni vendre son dernier monstre, en comptant l'équipe, le camp et les monstres partis en expédition automatique.
- Au camp, le joueur peut :
  - échanger des monstres entre son équipe et le camp ;
  - choisir l'ordre de son équipe : le premier monstre en état de combattre commence chaque combat ;
  - relâcher un monstre, définitivement et après confirmation, sauf son dernier monstre ;
  - renommer un monstre (section 5.5) ;
  - gérer les attaques d'un monstre (section 13.10) ;
  - une fois le marchand au camp, lui revendre un monstre (section 14.2) ;
  - après la V1 (étape 25), trier le camp par nom, niveau ou type : l'ordre du camp change pour de bon, et les numéros des monstres le suivent.
- Après la V1 (étape 25), la gestion d'un monstre et l'écran de ses attaques montrent ses types, ses stats et ses valeurs internes. Depuis l'étape 28, aussi son habileté et son effet (section 5.9). La liste « Vos monstres » disparaît du camp : la gestion des monstres la montre déjà.

### 13.3 Équipement et stockage

- Le joueur peut ajuster et modifier son équipement.
- L'inventaire étant limité en place, le joueur dispose d'un coffre pour stocker ses objets.
- Décidé le 18/10/2026, pour l'étape 26 : au camp, un coffre garde à l'abri l'argent, les potions et les capsules d'attaque ; le sac contient ce que le joueur emporte en expédition manuelle. Il les déplace librement entre le coffre et le sac, au camp seulement. Pas de limite de place pour l'instant.
  - Ce qui est gagné au camp (achats, ventes, expéditions automatiques) va dans le coffre. Ce qui est trouvé en expédition manuelle va dans le sac, et y reste au retour : le joueur range lui-même ce qu'il veut protéger.
  - Au camp, l'argent du sac et celui du coffre paient ensemble, celui du sac d'abord. Toutes les capsules possédées servent à apprendre des attaques, où qu'elles soient.
  - Une défaite en expédition manuelle fait perdre tout le sac (section 9.3). Une expédition automatique échouée ne rapporte rien, comme avant (section 12.3).
  - À la reprise d'une sauvegarde antérieure, l'argent, les potions et les capsules déjà possédés sont rangés dans le coffre.
  - Au camp, l'écran du coffre montre le sac et le coffre ; on y prend ou range des potions, de l'argent et des capsules, ou l'on range tout le sac d'un coup. Le camp rappelle le contenu des deux.
  - Depuis les étapes 35 et 36, le sac et le coffre gardent aussi les capsules d'habileté et les ressources, qui se déplacent par lots comme les potions. Les âmes des marchands n'y sont pas : elles restent avec le joueur, que rien ne lui fait perdre (section 14.2).
  - Précisé à l'étape 82, en codant : sur Discord, avec les ressources du Col du Givre-Noir, les ressources ne tiennent plus dans le menu des autres objets du coffre (pierres, boosters, Philtres chromatiques) : un bouton "Ressources" du coffre ouvre leur écran, avec un menu pour en prendre et un pour en ranger. Dans les textes du sac et du coffre, plusieurs capsules d'attaque se regroupent : "les capsules d'attaque (Brasier, Déferlante)", pour que le camp tienne dans un message quand le joueur les a toutes.
- Décidé le 25/10/2026, pour les étapes 125 à 127 (choix de l'utilisateur) : le sac a une capacité.
  - Chaque objet y prend une place : potion et autre soin, filet, ressource, capsule d'attaque ou d'habileté, pierre, booster, Philtre chromatique, charme de garde. L'argent n'en prend pas. Les monstres, les âmes et le Béhérit n'y sont pas.
  - 30 places au départ ; l'amélioration du sac, chez Gunther, les porte à 45, 60 puis 80 (section 13.4). Le coffre du camp n'a pas de limite.
  - En expédition, le butin entre dans le sac même s'il déborde. À la fin de l'étape, avant de continuer ou de rentrer au camp, le joueur jette des objets de son choix, parmi tout le sac, jusqu'à revenir à sa capacité ; ce qu'il jette est perdu. Après une défaite, le sac est perdu, et il n'y a rien à choisir.
  - Au camp, une sortie du coffre qui ferait déborder le sac est refusée. Les achats chez Malvoise vont au coffre (voir plus haut) et ne sont pas limités.
  - Le départ en expédition est refusé tant que le sac déborde : une sauvegarde antérieure dont le sac dépasse sa capacité ne perd rien, et le joueur range le surplus au coffre.
  - Sauvegarde en version 30 : le niveau de l'amélioration du sac ; une sauvegarde antérieure l'a à 0.
  - Précisé en codant : l'argent ne se jette pas ; dans la Tour à deux, chaque joueur vide son propre sac avant que le groupe continue ; le joueur automatique de l'outil d'équilibrage range son butin au coffre à chaque retour, et jette ce qu'il n'emploie pas d'abord.
  - Écrans, aux étapes 125 à 127 : le camp et l'écran entre deux étapes disent les places du sac, "Sac (18/30) : ...". À la fin d'une étape où le sac déborde, la question dit "Votre sac déborde : 32 objets pour 30 places. Jetez-en 2 pour poursuivre ; ce que vous jetez est perdu." Dans la console, le joueur choisit une sorte d'objet, puis combien en jeter ; sur Discord, un menu "Jeter un objet" et un menu "Jeter une ressource" jettent un exemplaire par choix. Le récit dit "Vous jetez 2 potions." La forge propose "Améliorer : Sac", dont l'effet est "sac de 45 places".

### 13.4 Améliorations

Avec des ressources trouvées en expédition (bois, métal, tissu, minerai rare, etc.), le joueur peut améliorer :

- son sac (inventaire) ;
- le nombre de monstres pouvant l'accompagner ;
- le nombre de monstres pouvant rester au camp ;
- etc.

Décidé le 19/10/2026, pour les étapes 41 à 43 :

- Les améliorations sont construites par un artisan, Gunther, forgeron, prisonnier du Bourreau. La victoire sur le Bourreau, au bout du Marais, le libère : il suit le joueur jusqu'au camp et y installe sa forge, définitivement. Une sauvegarde où le Bourreau est déjà vaincu a Gunther au camp dès la reprise (hypothèse retenue, Q85).
- Trois améliorations, chacune par niveaux, dont chacun demande le précédent :
  - l'enclos agrandit le camp : 40, 50 puis 60 places, au lieu de 30 (section 13.2) ;
  - les filets multiplient la chance de capture par 1,1, 1,2 puis 1,3, toujours bornée à 100 % ; le bonus se multiplie avec celui de l'habileté Traqueur (section 11.2) ;
  - l'équipe passe de 3 à 4 monstres, en un seul niveau (étape 43). Les expéditions automatiques restent de 1 à 3 monstres (section 12.1).
- Elles se paient en ressources seules, sans argent (section 14.3) : celles du sac d'abord, puis celles du coffre, prises au lancement. Coûts et durées dans docs/contenu-v1.md, section 9.
- Chaque niveau demande une durée de construction, de 1 h à 8 h. Gunther ne construit qu'un niveau à la fois ; une construction lancée ne s'annule pas (hypothèse retenue, Q85). Comme une expédition automatique, la construction se compte sur l'horloge réelle, et le joueur constate lui-même le travail fini, au camp : l'effet ne s'applique qu'à ce moment. Sur Discord, le bot prévient par message privé qu'une construction est finie, comme le retour d'une expédition automatique (section 12.7, hypothèse retenue, Q85).
- Les niveaux atteints et la construction en cours sont sauvegardés (section 13.9).
- Écrans, à l'étape 42 : une fois Gunther au camp, le camp propose sa forge ("Forge de Gunther", dans la console comme sur Discord). La forge montre, pour chaque amélioration, le niveau atteint, l'effet, le coût et la durée du niveau suivant, puis la construction en cours et son temps restant. Elle propose d'améliorer chaque amélioration qui n'est pas au niveau maximal, même faute de ressources (le refus dit ce qui manque), et de constater le travail une fois fini. Le lancement est confirmé ; la confirmation dit l'effet, le coût, la durée, et qu'une construction lancée ne s'annule pas. Le camp signale un travail fini à constater. Sur Discord, le bot prévient par message privé à la fin d'une construction.
- Précisé à l'étape 43, en codant : l'équipe de 4 compte dès que le travail est constaté ; une capture rejoint alors l'équipe en 4e place s'il en reste une. Les expéditions automatiques restent de 1 à 3 monstres : le bouton "Toute l'équipe" n'est proposé qu'à une équipe de 3 au plus. La sauvegarde passe en version 15, qui ajoute le niveau de l'amélioration de l'équipe ; une sauvegarde en version 14 l'a à 0.
- Précisé à l'étape 42, en codant : sur Discord, avec l'enclos agrandi, les menus de monstres numérotent l'équipe et le camp à la suite, 25 par menu, et la liste des monstres d'un message se résume, au-delà d'une trentaine de monstres, à l'équipe et au compte du camp, chaque monstre restant décrit dans les menus (limite de 2000 caractères).

### 13.5 Fabrication

- Avec les ressources obtenues, le joueur peut fabriquer des objets : filets de capture, armes, armures, talismans.
- Il peut aussi améliorer ses filets de capture : chaque niveau d'amélioration augmente les chances de capture.

Décidé le 20/10/2026, pour les étapes 48 à 51 (choix de l'utilisateur) :

- Gunther forge l'équipement du héros (section 4.3) dans le même atelier que les améliorations : un seul travail à la fois, amélioration ou objet. Comme une amélioration (section 13.4), un objet se paie en ressources seules, prises au lancement, du sac d'abord puis du coffre ; il demande une durée, de 1 h à 8 h, sur l'horloge réelle ; un travail lancé ne s'annule pas ; le joueur constate lui-même le travail fini, au camp, et l'objet rejoint alors son équipement. Sur Discord, le bot prévient par message privé de la fin d'une fabrication.
- Chaque objet ne se fabrique qu'une fois : un objet possédé n'est plus proposé.
- Les filets de capture restent une amélioration du camp (section 13.4) : ils ne se fabriquent pas à l'unité.
- Remplacé le 25/10/2026, pour l'étape 104 (choix de l'utilisateur) : Gunther fabrique aussi les filets, par lots de 5, aussi souvent que le joueur le veut, chaque lot étant un travail ordinaire, comme le charme de garde : 5 filets de chanvre contre 3 Croc de loup et 3 Plume de corbeau, en 30 minutes ; 5 filets lestés contre 3 Fragment d'armure et 3 Os de squelette, en 1 h ; 5 filets de chaînes contre 3 Éclat de gargouille et 2 Pierre de troll, en 2 h. Les filets vont au coffre quand le joueur constate le travail. L'amélioration des filets (section 13.4) continue de jouer sur toutes les qualités.
- Décidé le 20/10/2026, pour les étapes 76 à 78 (choix de l'utilisateur) : Gunther fabrique aussi le charme de garde, objet de protection des expéditions automatiques (section 12.5), un par travail, aussi souvent que le joueur le veut, contre 3 Os de squelette et 3 Fragment d'armure, en 30 minutes. C'est un travail ordinaire : un seul à la fois, avec les améliorations et l'équipement. Le charme va dans le coffre quand le joueur constate le travail. Il se range dans le sac et le coffre, se perd avec le sac et ne se revend pas.

### 13.6 Automatisation

Le joueur peut construire des machines pour automatiser certaines tâches, par exemple :

- récolter des ressources ;
- entraîner automatiquement ses monstres au camp ;
- etc.

Décidé le 20/10/2026, pour les étapes 72 à 74 (choix de l'utilisateur) :

- Deux machines, construites par Gunther comme des améliorations du camp (section 13.4), par niveaux, chacun demandant le précédent, contre des ressources et une durée, un travail à la fois : les pièges et la cour d'entraînement. Coûts, durées et rendements dans docs/contenu-v1.md, section 19.
- Les machines se gèrent depuis la forge de Gunther : elle mène à un écran "Machines". Le camp ne gagne pas de bouton ; il signale seulement des pièges pleins.
- Les pièges prennent des ressources dans une zone explorée (section 9.2), choisie par le joueur. Ils en prennent une à intervalle fixe, sur l'horloge réelle, même jeu fermé ou bot éteint, jusqu'à un stock maximal ; au-delà, ils n'en prennent plus. Le joueur relève le stock au camp quand il veut : les ressources sont tirées à ce moment, une par une, parmi celles des espèces communes de la zone (une ressource rare de la zone 1 fois sur 10 au niveau 3, s'il en existe une), puis vont dans le coffre, et la partie est sauvegardée aussitôt, comme pour la pêche (section 13.8).
- Après une relève, le compte reprend où il en était : le temps d'une ressource entamée n'est pas perdu, sauf si le stock était plein, auquel cas il repart de la relève (hypothèse retenue, Q95). Changer de zone relève d'abord le stock, puis le compte repart de zéro. Un niveau constaté vaut pour tout le temps écoulé depuis la dernière relève (hypothèse retenue, Q95).
- La cour d'entraînement reçoit 1 à 3 monstres, selon son niveau. Le joueur y place un monstre de l'équipe ou du camp, comme pour une expédition automatique (section 12.1) : il quitte l'équipe ou le camp, garde sa place au camp, et ne peut ni se relâcher, ni se renommer, ni s'échanger, ni partir ailleurs. Il y gagne une XP fixe par heure, comptée à la seconde près, versée quand le joueur le reprend, sans dépasser l'XP du niveau maximal de la zone ouverte la plus avancée (7, 13, 20 ou 27) : au-delà, il ne gagne plus rien. L'habileté Instinct ajoute ses 20 % (hypothèse retenue, Q95).
- Repris, le monstre rejoint l'équipe s'il y reste une place, sinon le camp, comme au retour d'une expédition automatique (section 12.7) ; les attaques à apprendre au-delà de 4 se choisissent de même, et il n'évolue pas d'office (section 5.4). Le niveau constaté de la cour vaut pour tout le temps passé (hypothèse retenue, Q95).
- Sur Discord, aucun message privé n'annonce des pièges pleins (hypothèse retenue, Q95). Remplacé le 25/10/2026, à l'étape 113 : le bot prévient des pièges pleins et d'un monstre de la cour à son plafond (section 2).
- Écrans, à l'étape 73 (console) : une fois une machine construite, la forge propose "Machines". L'écran montre les pièges (niveau, effet, zone, stock et temps avant la prochaine ressource) et la cour (niveau, effet, plafond, places occupées, puis chaque monstre avec l'XP gagnée et le temps passé). Il propose de relever les pièges s'ils ont un stock, de les poser dans chaque autre zone explorée, de mettre un monstre à l'entraînement s'il reste une place, puis de reprendre chaque monstre de la cour. Le camp signale des pièges pleins.
- Écrans, à l'étape 74 (Discord) : la forge a un bouton "Machines", une fois une machine construite. L'écran des machines a le même texte que dans la console, un menu des autres zones explorées où poser les pièges, et une rangée de boutons : "Relever les pièges", "Mettre un monstre à l'entraînement", qui ouvre un écran des monstres de l'équipe et du camp en menus numérotés, et "Reprendre {monstre}" pour chaque monstre de la cour. Le camp signale des pièges pleins.
- Décidé le 25/10/2026, pour l'étape 109 (choix de l'utilisateur, qui demandait un camp d'entraînement comme la pension de Pokémon, déjà couvert par la cour) : la cour, une fois construite, s'ouvre directement, en plus des machines de la forge, sans changement de règles. Dans la console, le menu du camp propose "Cour d'entraînement", avant "Options". Sur Discord, où le camp a déjà ses deux rangées de boutons, la gestion des monstres a un bouton "Cour d'entraînement", à côté des tris. L'écran de la cour n'en montre que l'état et propose de mettre un monstre à l'entraînement ou de le reprendre ; sur Discord, après le choix du monstre, le joueur revient à la cour, et l'écran a un bouton "Retour à la gestion" (hypothèses retenues, Q104).
- Idée de l'utilisateur, à concevoir plus tard : un système de reproduction des monstres, à partir de la cour (Q104).

### 13.7 Services rares

- Réanimer un monstre mort, contre une ressource très rare : en suspens tant qu'il n'existe pas de mort définitive (section 5.7).
- Rendre un de ses monstres shiny, contre un objet extrêmement rare.

Décidé le 20/10/2026, pour les étapes 67 à 69 (choix de l'utilisateur) :

- C'est Malvoise qui rend un monstre shiny, au camp, depuis son étal, contre un Philtre chromatique (section 14.3), sans argent. Le joueur choisit un monstre de l'équipe ou du camp qui n'est pas shiny ; le jeu demande confirmation, puis le Philtre est consommé, du sac d'abord, puis du coffre.
- Le monstre devient shiny, avec ce que cela implique déjà (section 5.1) : 31 dans chaque valeur interne, et une revente doublée (section 14.5). Il garde le reste : surnom, niveau, XP, attaques, habileté, marque "reçu de". Ses PV actuels augmentent d'autant que ses PV max, sauf s'il est KO (hypothèse retenue, Q93).
- L'étal propose le service même sans Philtre, comme la forge propose une amélioration faute de ressources : le refus dit ce qui manque (hypothèse retenue, Q93).
- Écrans, à l'étape 68 (console) : l'étal de Malvoise propose "Rendre un monstre shiny (1 Philtre chromatique)", avec le nombre de Philtres possédés, puis le choix du monstre et la confirmation. La gestion des monstres propose "Donner un booster à un monstre" : le booster, parmi ceux du sac et du coffre, avec son effet, puis le monstre. Le coffre déplace les boosters, par sorte et au nombre choisi, et les Philtres chromatiques.
- Écrans, à l'étape 69 (Discord), précisé en codant : la dernière rangée de l'étal de Malvoise est pleine. Le service se rend donc depuis la gestion des monstres : une fois Malvoise au camp et un Philtre chromatique possédé, un monstre choisi qui n'est pas shiny a un bouton "Rendre shiny (Malvoise)", suivi de la confirmation ; l'étal rappelle le service et le nombre de Philtres. Le monstre choisi a aussi un menu "Donner un booster", avec les boosters possédés qui lui servent encore (sa valeur interne n'est pas à 31). Dans le coffre, les boosters, par sorte entière, et les Philtres chromatiques, tous ensemble, s'ajoutent aux menus des ressources et des pierres, qui comptent alors 24 options au plus.

### 13.8 Pêche

- Le joueur peut pêcher pour trouver des monstres aquatiques ou des objets plus ou moins rares.

Décidé le 20/10/2026, pour les étapes 62 à 65 (choix de l'utilisateur) :

- La pêche se fait au camp, avec Ansgar, vieux pêcheur sauvé dans le Marais des Pendus (section 9.2). Tant qu'il n'est pas au camp, le camp ne propose pas la pêche.
- Une seule ligne à la fois. Le joueur la lance au camp, gratuitement et sans confirmation ; elle est prête 2 h plus tard, sur l'horloge réelle, comme une expédition automatique, et une ligne lancée ne se retire pas (hypothèse retenue, Q91). Le joueur la relève lui-même, au camp, une fois prête.
- La prise est tirée au moment de relever la ligne, une fois par ligne, et la partie est sauvegardée aussitôt, pour que quitter le jeu ne permette pas de la tirer de nouveau. Si l'équipe et le camp sont pleins, la ligne ne se relève pas, avant tout tirage : le refus dit de faire de la place (hypothèse retenue, Q91).
- Prises : monstre aquatique commun 55 %, Kelpie (rare) 5 %, potion 20 %, bourse noyée 10 %, pierre d'évolution 5 % (Pierre de braise ou de lune, pour moitié), capsule d'habileté 5 %. Depuis l'étape 67 (choix de l'utilisateur) : potion 14 %, booster 5 %, tiré parmi les six (section 14.3), Philtre chromatique 1 % (section 13.7). Un objet va dans le coffre, comme tout ce qui est gagné au camp (section 13.3). La bourse contient l'argent d'un coffre de la zone ouverte la plus avancée (section 14.5).
- Un monstre pêché est un monstre sauvage : son niveau est tiré parmi ceux de la zone ouverte la plus avancée, avec ses valeurs internes, son shiny (1 sur 512) et son habileté. Il entre au codex comme rencontré dès la prise. Choix de l'utilisateur : le joueur le garde ou le relâche ; gardé, il compte comme capturé et rejoint l'équipe s'il y a de la place, sinon le camp ; relâché, il retourne à l'eau, sans confirmation (hypothèse retenue, Q91). Tant qu'il n'a pas répondu, le camp n'accepte aucune autre action, comme pour une attaque à choisir (section 12.7), et la question revient à la reprise de la partie.
- Les espèces aquatiques ne vivent qu'à la pêche (le Noyé attaque aussi Ansgar, section 9.2). Elles n'ont ni ressource ni évolution (hypothèses retenues, Q91). Fiches dans docs/contenu-v1.md, section 15.
- La pêche ne joue pas sur les expéditions et ne touche pas au multijoueur.
- Écrans, à l'étape 64 (console) : une fois Ansgar au camp, le camp propose "Pêche d'Ansgar", et signale une ligne prête à relever. L'écran de la pêche montre l'état de la ligne et propose de la lancer, ou de la relever une fois prête. Le monstre pêché est présenté avec sa fiche, puis la question "Garder" ou "Relâcher".
- Écrans, à l'étape 65 (Discord) : un bouton "Pêche" s'ajoute au camp, le 10e et dernier possible sur ses deux rangées. Le bot prévient par message privé quand la ligne est prête, comme d'une construction finie.

### 13.9 Sauvegarde (V1)

- Un seul emplacement : une seule partie en cours, reprise au lancement du jeu. Commencer une nouvelle partie remplace l'ancienne, après confirmation.
- La sauvegarde est automatique : à la création de la partie, à chaque retour au camp et après chaque action de gestion au camp.
- On ne sauvegarde qu'au camp : quitter le jeu en pleine expédition ramène à la dernière sauvegarde.
- Une sauvegarde illisible, ou qui cite une espèce ou une attaque disparue du jeu, est refusée : le jeu l'explique et propose une nouvelle partie, sans rien effacer sans l'accord du joueur.
- La sauvegarde garde l'identifiant des espèces, des attaques et des zones, pas leurs données : un rééquilibrage s'applique aux monstres déjà capturés.
- Elle garde aussi l'avancée de la traque (indices trouvés, Apôtres vaincus) et les conseils du tutoriel déjà montrés. Depuis l'étape 31, en version 11, aussi les âmes recueillies et les nouveaux soins du sac et du coffre (section 14.4). Depuis l'étape 29, aussi les humains vaincus (section 10.7). Depuis l'étape 30, en version 10 : une sauvegarde antérieure qui a trouvé des indices de l'ancienne piste reprend à 1 indice (section 9.4).
- Après la V1, elle garde les expéditions automatiques en cours : zone, monstres envoyés, heures de départ et de retour (section 12). Elle garde aussi les attaques qui restent à choisir après l'accueil d'une expédition (section 12.7). Le départ, le rappel et l'accueil d'une expédition automatique sont des actions au camp, suivies d'une sauvegarde.
- Avec l'économie, elle garde aussi l'argent du joueur, la présence du marchand au camp et les capsules d'attaque obtenues (section 14). Les achats et les ventes sont des actions au camp, suivies d'une sauvegarde.
- Depuis l'étape 28, elle garde aussi l'habileté de chaque monstre (section 5.9).
- Depuis l'étape 41, en version 14 : Gunther au camp, les niveaux des améliorations et la construction en cours (section 13.4). Depuis l'étape 43, en version 15, aussi le niveau de l'amélioration de l'équipe.
- Depuis l'étape 49, en version 16 : l'équipement du héros, objets possédés et portés, et l'objet en fabrication (sections 4.3 et 13.5). Une sauvegarde antérieure n'a aucun objet. Une sauvegarde qui cite un objet disparu du jeu est refusée, comme une espèce disparue.
- Depuis l'étape 39, en version 13 : les pierres d'évolution du sac et du coffre (section 5.4). Une sauvegarde antérieure n'en a pas.
- Depuis l'étape 62, en version 18 : Ansgar au camp, l'heure où la ligne sera prête et le monstre pêché qui attend la réponse du joueur (section 13.8). Une sauvegarde antérieure n'a pas Ansgar au camp.
- Depuis l'étape 72, en version 22 : les niveaux des pièges et de la cour d'entraînement, la zone des pièges et l'heure du début de leur compte, et les monstres à l'entraînement avec l'heure où ils y sont entrés (section 13.6). Depuis l'étape 76, en version 23 : les charmes de garde du sac et du coffre, le charme en fabrication, et le monstre protégé et le charme de chaque expédition automatique (section 12.5).
- Depuis l'étape 66, en version 19, sans changement de structure : les âmes d'Apôtres sont celles des Apôtres vaincus, déjà gardés par la sauvegarde. À la lecture d'une sauvegarde antérieure, les Apôtres capturés quittent l'équipe, le camp et les expéditions automatiques, et la fiche de chaque Apôtre vaincu s'ouvre au codex (section 9.4).
- Depuis l'étape 32, en version 12 : les attaques à choisir au retour d'une expédition automatique désignent un monstre de l'équipe ou du camp ; celles des versions antérieures désignent un monstre du camp. Les étapes 33 à 36 y ajoutent, toujours en version 12, les âmes à retrouver, les âmes portées, les capsules d'habileté et les ressources du sac et du coffre. Les âmes recueillies des versions antérieures comptent comme données à Malvoise : leur soin reste en vente.
- Depuis l'étape 67, en version 20 : les boosters et les Philtres chromatiques du sac et du coffre (section 14.3). Une sauvegarde antérieure n'en a pas, sauf un Philtre chromatique dans le coffre par Apôtre déjà vaincu (hypothèse retenue, Q93).
- Depuis l'étape 70, en version 21 : la liste d'amis et le palmarès des duels du joueur (section 15.4), vides dans une sauvegarde antérieure.
- Après la V1 (19/10/2026), un sous-menu « Options » du camp permet au joueur de se suicider, après confirmation, pour recommencer sans attendre la reprise au lancement. La sauvegarde est effacée dès la confirmation, avec le héros, les monstres, le sac, le coffre, le codex et les expéditions automatiques en cours ; un court texte suit, puis la création d'un nouveau héros. Ce n'est possible qu'au camp. Le texte est dans docs/contenu-v1.md, section 4.

### 13.10 Attaques des monstres

- Au camp, un monstre peut réapprendre gratuitement toute attaque que son espèce apprend jusqu'à son niveau, y compris celles qu'il a oubliées ou refusées. S'il en connaît déjà 4, il en oublie une (section 7).
- Une capsule d'attaque enseigne une attaque précise à tout monstre qui a le type de cette attaque et ne la connaît pas déjà, de la même façon. Elle n'est pas consommée : une fois obtenue, elle sert autant de fois qu'on veut.
- Les capsules se trouvent dans les coffres des expéditions manuelles (section 9.2), ou s'achètent chez le marchand, qui les vend toutes (section 14.2). Chaque zone a ses capsules : un coffre en contient une fois sur 5, tirée parmi celles de la zone que le joueur n'a pas encore ; quand il les a toutes, le coffre n'en contient plus.
- Un monstre de départ n'a qu'un type, et son espèce apprend déjà toutes les attaques de ce type qui existaient dans la V1. Les capsules comptent donc des attaques nouvelles pour les types Feu, Eau et Plante, trouvées dans la zone facile, et des attaques d'autres espèces, trouvées dans la zone difficile. Liste, types et prix dans docs/contenu-v1.md (section 6).
- Le Chat noir, monstre de départ ajouté à l'étape 27, n'apprend pas toutes les attaques Ténèbres : la capsule Sentence lui sert aussi.

### 13.11 Rite du sang mêlé (reproduction)

Décidé le 25/10/2026, pour les étapes 110 à 112 (choix de l'utilisateur). Valeurs et textes dans docs/contenu-v1.md, section 28.

- Malvoise, une fois au camp, unit deux monstres par le Rite du sang mêlé, depuis son étal, après confirmation. Un rite à la fois (hypothèse retenue, Q105).
- Parents : deux monstres du joueur, de l'équipe ou du camp, de la même famille d'évolution (le Loup et le Warg, par exemple). Un monstre loué ne peut pas l'être, comme pour toute action autre que combattre (section 14.2). Les espèces rares sont admises (hypothèse retenue, Q105).
- Coût, payé au lancement, du sac d'abord puis du coffre : 3 ressources de l'espèce du petit et 50 pièces d'argent. Une espèce qui ne lâche pas de ressource ne coûte que l'argent.
- Le rite dure 2 heures réelles, même jeu fermé ou bot éteint. Les deux parents restent auprès de Malvoise jusqu'à la fin, comme dans la cour d'entraînement : ils quittent l'équipe ou le camp, gardent leur place au camp, et ne peuvent servir ailleurs. Le petit a aussi une place réservée au camp : le rite est refusé s'il n'en reste aucune (hypothèse retenue, Q105).
- Le rite fini, le joueur accueille le petit depuis l'étal ; le camp signale qu'il attend. Le petit naît au niveau 1, de la forme de base de la famille (le Loup, même d'un Warg et d'un Loup). Pour chaque stat, il reçoit la plus haute valeur interne de ses deux parents : un parent shiny, qui a 31 partout, donne donc des petits à 31 partout. Comme un monstre sauvage, il a 1 chance sur 512 d'être shiny et tire son habileté ; il connaît les attaques de son espèce au niveau 1. Son espèce s'ouvre au codex.
- Le petit rejoint l'équipe s'il y reste une place, sinon le camp, après ses parents, qui reviennent comme d'une expédition automatique (section 12.7).
- Sauvegarde en version 28 : le rite en cours, avec ses parents et l'heure de sa fin.
- Décidé le 25/10/2026, pour l'étape 114 (choix de l'utilisateur) :
  - Shiny : un seul parent shiny double les chances du petit, à 1 sur 256 ; deux parents shiny le rendent shiny à coup sûr. L'utilisateur avait écrit "divise le taux par 2" : Claude l'a lu comme une chance doublée, ce que l'utilisateur a confirmé.
  - Attaque d'œuf : chaque famille d'évolution a la sienne, de l'un de ses types, qui ne s'obtient que par le rite, ni par niveau, ni par capsule. Le petit naît en la connaissant 1 fois sur 4, et à coup sûr si un parent la connaît. Elle se garde à l'évolution, comme toute attaque. Puissance 80 et précision 100, ou 75 avec 20 à 30 % de chances d'empoisonner ou de paralyser : plus que les attaques courantes, sans atteindre les plus fortes. Liste dans docs/contenu-v1.md, section 28.
  - La confirmation du rite dit les chances du petit d'être shiny, quand un parent l'est, et de connaître l'attaque d'œuf ; la naissance dit s'il la connaît ; la fiche du codex de chaque espèce dit l'attaque d'œuf de sa famille.
  - Précisé en codant : il y a 32 familles, et non 35 comme Claude l'avait compté. La Lanterne des morts, le Chevalier noir et le Chevalier squelette sont les évolutions du Feu follet, de l'Armure hantée et du Squelette. Leurs familles gardent l'attaque de leur forme de base (Lueur des marais, Coup de heaume, Danse macabre), et les trois attaques proposées pour ces évolutions (Flamme des veilleurs, Estoc maudit, Charge du cavalier) n'ont pas été créées (hypothèse retenue, Q105).
- Écrans, à l'étape 111 (console) : l'étal de Malvoise propose le rite en 12, avec son coût et sa durée ("Rendre un monstre loué" passe en 13). Le joueur choisit le premier parent dans la liste de ses monstres, puis le second parmi ceux de la même famille, et confirme. Pendant le rite, la même entrée en donne l'état (les parents et le temps restant) ; une fois le rite achevé, elle accueille le petit, et le camp signale qu'il attend. Précisé en codant : la liste des monstres disait "en expédition automatique" toutes les places gardées au camp, celles de la cour comprises ; elle dit désormais combien de places sont gardées, pour des monstres absents ou à naître.
- Écrans, à l'étape 112 (Discord), précisé en codant : l'étal de Malvoise n'a plus de rangée libre. Le rite se lance donc depuis la gestion des monstres, comme le service shiny : un monstre choisi a un bouton "Rite du sang mêlé (Malvoise)" quand Malvoise est au camp, qu'aucun rite n'est en cours et qu'un autre monstre de sa famille peut s'unir à lui. Ce bouton ouvre le choix du second parent, en menus numérotés des monstres de sa famille, puis la confirmation. L'étal rappelle le service, ou donne l'état du rite en cours. Une fois le rite achevé, le camp le signale et gagne une rangée avec le bouton "Accueillir le petit (Malvoise)".

## 14. Économie, marchands, objets et ressources

### 14.1 Monnaie

- La monnaie du jeu est l'argent : le même mot désigne le métal précieux et la monnaie.
- Après la V1, l'argent vient de la revente des monstres au marchand, des coffres des expéditions manuelles et des expéditions automatiques réussies. Les montants sont en section 14.5.
- Le joueur commence sans argent. L'argent trouvé pendant une expédition manuelle ne lui est acquis qu'au retour au camp (section 9.3). Avec le coffre du camp, il va dans le sac et se perd avec lui en cas de défaite (section 13.3).
- Dans les textes du jeu, l'argent se compte en pièces d'argent.

### 14.2 Marchands

Les marchands sont rencontrés en expédition. Ils proposent :

- à l'achat : des objets (ressources, soins, boosters de stats, filets de capture, talismans, armes, armures, etc.). Depuis l'étape 104 (25/10/2026), Malvoise vend le filet de chanvre (10 pièces d'argent) et le filet lesté (30), sans confirmation, comme les soins ; ils vont au coffre. Il ne les rachète pas (hypothèse retenue, Q102) ;
- à l'achat ou à la location : des monstres. Une location coûte un prix donné et vaut pour un nombre donné d'expéditions.

Le joueur peut aussi vendre ses objets aux marchands contre de l'argent.

Premier marchand, après la V1 :

- Tant qu'aucun marchand n'est au camp, chaque étape d'une expédition manuelle peut tirer la rencontre d'un marchand, à la place d'un combat (10 % des étapes, section 9.2).
- Le marchand rencontré se réfugie au camp et y reste, toujours accessible ; il ne se rencontre plus en expédition. Il se réfugie au camp dès la rencontre, même si l'expédition finit en défaite ; quitter le jeu avant le retour au camp annule la rencontre, comme toute expédition non sauvegardée (section 13.9). On ne lui achète rien pendant l'expédition.
- Au camp, il vend des potions et des capsules d'attaque (section 13.10), et rachète les monstres capturés contre de l'argent. Il ne rachète ni potions ni capsules. Les achats se font sans confirmation.
- Revendre un monstre est définitif et suit les règles du relâcher : confirmation, et jamais le dernier monstre de l'équipe. Tout monstre se revend comme il se relâche, monstre de départ et Apôtre compris. Le prix dépend de son niveau, du total des stats de base et de la rareté de son espèce, et du shiny (section 14.5).
- Les prix du marchand, à l'achat comme à la revente, sont fixes et non tirés au hasard, pour être connus avant l'échange, comme la durée d'une expédition automatique (section 12.2) : la section 1.4 vise les gains des combats et des expéditions.
- Écrans : une fois le marchand au camp, le camp propose de le voir (une entrée du menu dans la console, un bouton sur Discord). Son étal montre l'argent, les potions et les capsules obtenues, vend des potions (au nombre choisi dans la console, par 1 ou 5 sur Discord) et les capsules que le joueur n'a pas encore. La vente montre le prix de chaque monstre ; on peut aussi vendre le monstre choisi depuis la gestion des monstres.
- Depuis l'étape 31, l'étal montre aussi les soins en vente, avec leur effet et leur prix, et les vend comme les potions : au nombre choisi dans la console, par 1 ou 5 sur Discord. En combat et entre deux étapes, le joueur choisit le soin puis le monstre : dans la console, par deux menus ; sur Discord, par un seul menu qui propose chaque soin du sac avec chaque monstre à qui il peut servir (l'élixir aux monstres KO, les autres soins aux monstres debout). Le coffre du camp déplace chaque soin par lots, comme les potions.
- Les attaques d'un monstre se gèrent depuis la gestion des monstres : ses attaques connues, puis celles qu'il peut apprendre au camp ; s'il en connaît déjà 4, le joueur choisit aussi celle à oublier (section 13.10).
- Tuer un monstre pour en tirer des ressources est envisagé, mais reporté tant qu'aucune ressource n'a d'usage (Q71).
- Décidé le 19/10/2026, pour l'étape 36 (Q71) : les monstres sauvages battus lâchent des ressources, que Malvoise rachète (section 14.3). C'est leur seul usage en attendant la fabrication et les améliorations (sections 13.4 et 13.5).
- Décidé le 19/10/2026, pour l'étape 31, à la place d'un marchand par zone : un nécromancien, unique, s'installe au camp. Sa rencontre est scriptée, juste après la première expédition menée au bout dans la première zone, et ne se reproduit plus. En expédition, le joueur trouve des marchands tués, par exemple sous ses yeux par un monstre, ou un cadavre de marchand qui l'attaque ; il rapporte leur âme au nécromancien, dont l'étal s'enrichit des articles de chaque marchand. Chaque marchand a son nom. Ce que vend chacun, la façon de trouver les âmes et le sort du marchand actuel restent à définir (Q77).
- Précisé le 19/10/2026, pour l'étape 31 (Q77), et remplace le premier marchand décrit plus haut :
  - le nécromancien s'appelle Malvoise. Il arrive au camp au retour de la première expédition menée au bout dans la Lisière, après l'indice, avec un court texte, et y reste ;
  - il remplace le marchand ambulant, qui ne se rencontre plus en expédition : Malvoise vend les potions et les capsules d'attaque et rachète les monstres, aux mêmes prix et selon les mêmes règles. Une sauvegarde où le marchand est au camp a Malvoise à sa place ; une sauvegarde qui a déjà mené la Lisière au bout l'a au camp dès la reprise (hypothèse retenue, à confirmer) ;
  - chaque zone a son marchand mort : Bertrand, colporteur, dans la Lisière ; Grete, herboriste, dans le Marais ; Ulrich, alchimiste, dans la Vallée des Brumes. Tant que l'âme du marchand d'une zone n'est pas recueillie, la rencontre de son âme prend 10 % des étapes de cette zone, à la place d'un combat, une fois par expédition au plus, comme le marchand d'avant (section 9.2) ;
  - dans la Lisière, un monstre de la zone, au niveau maximal de la zone, tue Bertrand sous les yeux du joueur et l'attaque : un combat ordinaire contre un monstre sauvage, fuite et filet compris. Dans le Marais, Grete s'est relevée en marchand zombie, qui attaque : fuite possible, mais ni filet ni capture, et il n'entre pas au codex. Dans la Vallée, Ulrich pend au gibet du Baron, et un monstre de la Vallée garde son corps et attaque, comme dans la Lisière ;
  - une victoire recueille l'âme, acquise dès ce moment même si l'expédition finit en défaite (hypothèse retenue, comme la rencontre du marchand d'avant). Une fuite ou une défaite laisse l'âme à trouver ;
  - chaque âme recueillie ajoute à l'étal de Malvoise l'objet de son marchand (section 14.4) : le remède pour Bertrand, la grande potion pour Grete, l'élixir pour Ulrich.
- Précisé à l'étape 31, en codant :
  - Malvoise arrive au retour de la première expédition menée au bout, qui ne peut être que dans la Lisière, seule zone ouverte avant ;
  - l'âme d'un marchand ne se trouve qu'une fois Malvoise au camp, pour que le texte "Vous la recueillez pour Malvoise" ait un sens (hypothèse retenue, Q80) ;
  - le combat pour une âme suit tout de suite son texte, sans bouton d'annonce : ce n'est pas un combat de fin de zone ;
  - le monstre qui tue Bertrand ou garde Ulrich n'est pas nommé dans leur texte : le combat le présente aussitôt, et son nom ne dit pas son genre ("un Loup", mais "une Vipère") ;
  - le marchand zombie n'est pas dit sauvage en combat ; une âme dont le marchand disparaîtrait du jeu serait oubliée par la sauvegarde.
- Décidé le 19/10/2026, pour les étapes 33 et 34, et remplace l'hypothèse de Q80 et le recueil de l'âme pour Malvoise :
  - une âme est un objet rare et indestructible : la victoire sur son gardien la donne au joueur, qui la porte dans un inventaire à part, ni sac ni coffre. Une défaite ne la lui fait pas perdre (section 9.3) ;
  - l'âme de Bertrand se trouve dès la première expédition dans la Lisière, sans Malvoise au camp, à l'étape 4, garantie tant qu'elle n'est pas recueillie. Précisé le 19/10/2026 par l'utilisateur : le brigand de l'étape 5 abat Bertrand sous les yeux du joueur, s'enfuit, et une bête qui le suivait attaque. Précisé le 19/10/2026 par l'utilisateur (Q83) : cette bête est toujours un Loup de niveau 7 ; ses valeurs internes, son shiny et son habileté sont tirés comme pour tout monstre sauvage, et le combat est ordinaire, fuite et filet compris. Le joueur recueille l'âme sans savoir ce que c'est ;
  - Malvoise est rencontré au bout de la première expédition menée au bout, après le brigand, et rentre au camp avec le joueur, où il s'installe. S'il porte l'âme de Bertrand, il lui dit l'avoir vu la recueillir et qu'elle l'intéresse ; sinon, il lui demande de rapporter les âmes des marchands morts. Il vient dans les deux cas, puisqu'il tient l'étal ;
  - au camp, chez Malvoise, le joueur lui donne l'âme qu'il veut, quand il veut, sans confirmation (hypothèse retenue, à confirmer) ; il peut aussi la garder sans limite de temps. Chaque âme donnée ajoute à l'étal l'objet de son marchand. Il en va de même pour les âmes de Grete et d'Ulrich ;
  - après une fuite ou une défaite face au gardien d'une âme, celui-ci revient à coup sûr à la prochaine expédition dans sa zone, à une étape tirée parmi celles qui précèdent la dernière (section 9.2) ; quitter l'expédition avant cette étape la repousse à la suivante. Cela vaut pour Grete, le marchand zombie, et pour Ulrich ; Bertrand, lui, revient toujours à l'étape 4.
- Plus tard, d'autres personnages pourront être rencontrés de la même façon et enrichir le camp, comme un pêcheur qui apprend à pêcher (section 13.8). Depuis l'étape 41, Gunther, forgeron, libéré du Bourreau, construit les améliorations du camp (section 13.4). Depuis l'étape 63, Ansgar, pêcheur, sauvé dans le Marais, tend sa ligne au camp (section 13.8).
- Décidé le 20/10/2026, pour les étapes 83 à 86 (choix de l'utilisateur), achat et location de monstres chez Malvoise, au camp (prix et textes dans docs/contenu-v1.md, section 22) :
  - achat : chaque espèce commune sauvage, à sa forme de base, des zones menées au bout, au niveau minimal de la zone où elle se rencontre, pour 4 fois son prix de revente à ce niveau. Ses valeurs internes, son habileté et son shiny (1 chance sur 512) sont tirés à l'achat, comme pour un monstre sauvage. Le monstre acheté est au joueur comme une capture : il rejoint l'équipe s'il y reste une place, sinon le camp ; l'achat est refusé si les deux sont pleins. Son espèce passe capturée au codex ;
  - location : chaque espèce rare des zones menées au bout, Schnoz excepté, au niveau maximal de la zone ouverte la plus avancée, pour 3 expéditions manuelles, au prix de sa revente à ce niveau. Un seul monstre loué à la fois. Ses valeurs internes, son habileté et son shiny sont tirés à la location ;
  - un monstre loué combat, gagne de l'XP et obéit toujours. Il ne se vend, ne se donne, ne s'échange et ne se relâche pas, n'évolue pas, ne reçoit ni booster, ni service shiny, ni capsule, ni surnom, et ne part ni en expédition automatique ni à l'entraînement. Le joueur peut le rendre plus tôt à Malvoise, après confirmation, sans remboursement ;
  - chaque retour au camp d'une expédition manuelle, menée au bout, interrompue ou perdue, compte une expédition de la location, que le monstre loué y soit parti ou non (hypothèse retenue, Q98). Au retour de la troisième, il repart chez Malvoise avec tout ce qu'il a gagné ;
  - sauvegarde en version 24 : la location en cours et le nombre d'expéditions qui lui restent.
  - Précisé aux étapes 83 à 86, en codant : l'achat et la location se font sans confirmation ; le retour d'un monstre loué est confirmé. Un monstre loué ne compte pas parmi les autres monstres quand le joueur vend, donne ou relâche (Q98) ; il peut réapprendre au camp les attaques de son espèce, mais rien d'une capsule. La fiche d'un monstre loué et l'étal disent les expéditions qui restent ; dans la vente, il est dit "loué, ne se vend pas". Une sauvegarde qui aurait plus d'un monstre loué, ou un monstre loué pour seul monstre, est illisible.
  - Écrans : dans la console, l'étal de Malvoise propose "Acheter un monstre" (9), "Louer un monstre" (10) et, avec un monstre loué, "Rendre {monstre} à Malvoise" (11). Sur Discord, la dernière rangée de l'étal étant pleine, l'écran de vente a un bouton "Acheter ou louer", qui ouvre un écran avec un menu d'achat, un menu de location et, avec un monstre loué, le bouton "Rendre {monstre} à Malvoise". Les menus ne disent pas les types des espèces, cachés jusqu'à leur capture (section 5.6).
  - Précisé à l'étape 87 : sur Discord, la gestion d'un monstre loué ne propose ni "Renommer", ni "Relâcher", ni la vente, ni le service shiny, ni la capsule d'habileté, ni les boosters ; elle propose "Rendre à Malvoise", confirmé comme depuis l'écran des achats.

### 14.3 Objets et ressources cités dans ce document

- Ressources de fabrication et d'amélioration : bois, métal, tissu, minerai rare, etc.
- Consommables : soins, boosters de stats.
- Décidé le 20/10/2026, pour les étapes 67 à 69 (choix de l'utilisateur) : six boosters, un par stat, qui ajoutent chacun 5 à la valeur interne de leur stat, sans dépasser 31 (section 5.8). Noms : Fiole de sang (PV), Moelle de bête (Attaque), Écorce de fer (Défense), Encre d'arcane (Attaque spéciale), Cendre bénite (Défense spéciale), Poudre de vif-argent (Vitesse). Détails dans docs/contenu-v1.md, section 17.
  - Ils ne s'achètent pas : on les trouve dans les coffres (6 %, 1 % par booster) et à la pêche (5 %, section 13.8). Malvoise ne les vend ni ne les rachète.
  - Ils se rangent dans le sac et le coffre, se déplacent par lots et se perdent avec le sac, comme les pierres d'évolution.
  - Au camp, depuis la gestion des monstres, le joueur donne un booster à un monstre de l'équipe ou du camp, sans confirmation (hypothèse retenue, Q93). Le booster est consommé, du sac d'abord, puis du coffre. Un booster sur une valeur interne déjà à 31 est refusé, sans être consommé ; une valeur à 28 passe à 31 et le surplus est perdu. Un shiny a déjà 31 partout : aucun booster ne lui sert. Les PV actuels augmentent d'autant que les PV max, sauf pour un monstre KO (hypothèse retenue, Q93).
- Objets permettant d'ajuster les statistiques d'un monstre.
- Filets de capture : achetables, fabricables, améliorables. Depuis les étapes 103 et 104, trois qualités qui s'épuisent (section 11.2).
- Équipement du héros : armes, armures, talismans. Depuis les étapes 48 à 51, fabriqués par Gunther et gardés par le héros, hors du sac et du coffre (sections 4.3 et 13.5).
- Depuis l'étape 36 (19/10/2026, Q71), une ressource par espèce sauvage qui peut être combattue, lâchée par un monstre sauvage mis KO et non capturé, après un combat gagné, 1 fois sur 3. Les monstres d'un humain, les Apôtres et le marchand zombie n'en lâchent pas, et les espèces pacifiques ne se combattent pas. Elle va dans le sac, se range au coffre, se perd avec le sac et se revend à Malvoise, à un prix fixe selon la rareté de l'espèce : 5 pièces d'argent pour une espèce commune, 15 pour une rare. Au camp, Malvoise rachète toutes les ressources d'une sorte d'un coup, celles du sac puis celles du coffre, sans confirmation. Liste dans docs/contenu-v1.md, section 6. Noms, taux et prix sont des hypothèses, à confirmer (Q82).
- Écrans, à l'étape 36 : les ressources s'écrivent avec leur nombre entre parenthèses, "Croc de loup (3)". Après une victoire, le récit dit celles que le joueur ramasse. Dans la console, l'étal de Malvoise propose "Vendre des ressources", par sorte, et le coffre les déplace au nombre choisi. Sur Discord, la vente des monstres devient "Vendre monstres et ressources", avec un menu des ressources, et le coffre a deux menus à part pour elles, une ou toutes de chaque sorte.
- Capsule d'habileté, depuis l'étape 35 (section 5.9).
- Argent (monnaie).
- Objets d'évolution : depuis l'étape 39, la Pierre de braise et la Pierre de lune (section 5.4).
- Capsules d'attaque, qui enseignent une attaque (section 13.10).
- Ressource très rare permettant de réanimer un monstre (en suspens, section 5.7).
- Objets de protection pour les expéditions automatiques : depuis l'étape 76, le charme de garde, fabriqué par Gunther (sections 12.5 et 13.5).
- Objet extrêmement rare permettant de rendre un monstre shiny. Décidé le 20/10/2026, pour les étapes 67 à 69 (choix de l'utilisateur) : le Philtre chromatique. Chaque Apôtre vaincu en laisse un, après son âme (section 9.4), et la pêche en remonte 1 fois sur 100 (section 13.8). Il ne s'achète pas et ne se revend pas. Il se range dans le sac et le coffre et se perd avec le sac, comme tout objet trouvé en expédition (hypothèse retenue, Q93 : la victoire sur un Apôtre achève l'expédition, il rentre donc au camp avec le joueur). Il sert au service de Malvoise (section 13.7).
- Renommé le 25/10/2026, pour l'étape 108 (choix de l'utilisateur) : l'Éclat d'astre devient le Philtre chromatique, une fiole aux reflets changeants qui altère la lignée du monstre pour en changer la couleur, comme chez les animaux albinos. Seuls le nom et les textes changent ; les règles restent les mêmes. Les sauvegardes gardent l'ancien nom dans le fichier, sans changement de version. Textes dans docs/contenu-v1.md, section 17.

### 14.4 Potions (V1)

- Seul objet de la V1. Une potion rend à un monstre de l'équipe un pourcentage de ses PV max tiré de 40 % à 60 %, arrondi à l'entier inférieur, sans dépasser ses PV max. Elle ne relève pas un monstre KO et ne guérit pas les statuts. Sur un monstre qui a tous ses PV, le jeu demande confirmation : la potion serait consommée sans effet.
- Elle s'utilise en combat, comme action du tour (l'adversaire agit ensuite), ou entre deux étapes d'une expédition.
- Dans la V1, les potions se trouvent dans les coffres ; après la V1, les expéditions automatiques réussies en rapportent aussi (section 12.6), et le marchand en vend au camp, à 20 pièces d'argent (sections 14.2 et 14.5). Il n'y a ni limite de sac ni coffre de stockage au camp : le joueur porte toutes ses potions et les perd toutes en cas de défaite (section 9.3). Avec le coffre du camp (étape 26, section 13.3), seules les potions emportées dans le sac se perdent.
- Depuis l'étape 31 (19/10/2026, Q77), trois soins s'ajoutent à la potion, vendus par Malvoise une fois l'âme de leur marchand recueillie (section 14.2) :
  - le remède, 25 pièces d'argent : il guérit le poison ou la paralysie d'un monstre qui n'est pas KO ;
  - la grande potion, 60 pièces d'argent : comme une potion, mais elle rend de 80 % à 100 % des PV max ;
  - l'élixir, 150 pièces d'argent : il relève un monstre KO avec la moitié de ses PV max, au moins 1.
- Ces soins s'utilisent comme la potion : en combat, comme action du tour, ou entre deux étapes. Ils se rangent dans le sac et le coffre, se perdent avec le sac et ne se trouvent pas dans les coffres. Comme une potion sur un monstre qui a tous ses PV, un soin sans effet (un remède sur un monstre sans statut, un élixir sur un monstre qui n'est pas KO) demande confirmation et serait consommé pour rien. Une potion, une grande potion ou un remède ne relève pas un monstre KO.

### 14.5 Montants (après la V1)

| Élément | Valeur |
|---|---|
| Argent au départ | 0 |
| Coffre, Lisière du Bois-Sombre | 10 à 25 pièces d'argent, en plus de 1 à 2 potions |
| Coffre, Marais des Pendus | 25 à 50, en plus de 1 à 2 potions |
| Coffre, Vallée des Brumes (étape 30) | 50 à 90, en plus de 1 à 2 potions |
| Capsule dans un coffre | un coffre sur 5, parmi les capsules de la zone que le joueur n'a pas encore |
| Expédition automatique réussie | Lisière 15 à 30, Marais 40 à 80 (section 12.8) |
| Revente d'un monstre | niveau x total des stats de base / 50, en division entière, puis x 1 pour une espèce commune, x 2 pour une rare, x 4 pour un Apôtre, et x 2 pour un shiny (depuis l'étape 26 ; avant : niveau x 5, 15 ou 40 selon la rareté) |
| Potion | 20 |
| Remède, grande potion, élixir (étape 31) | 25, 60 et 150 (section 14.4) |
| Capsule d'attaque | 150 pour celles de la Lisière, 200 à 300 pour celles du Marais, 300 à 400 pour celles de la Vallée (docs/contenu-v1.md, section 6) |
| Victoire contre un humain (étapes 29 et 30) | brigand 60, Bourreau 150 (section 10.7) |
| Rencontre du marchand | 10 % des étapes d'une expédition manuelle, tant qu'il n'est pas au camp ; remplacée à l'étape 31 par l'âme du marchand de la zone, tant qu'elle n'est pas recueillie (section 14.2) |
| Capsule d'habileté (étape 35) | 200 ; un coffre sur 10 (section 5.9) |
| Ressource rachetée par Malvoise (étape 36) | 5 pour une espèce commune, 15 pour une rare ; lâchée 1 fois sur 3 (section 14.3) |
| Pierre d'évolution (étape 39) | 300 ; un coffre sur 10 (section 5.4) |

Repères : un Loup de niveau 5 se revend 28 pièces d'argent, un Feu follet de niveau 10 64, un Cerf blanc de niveau 5 80, un Schnoz de niveau 10 180, le Baron pendu au niveau 15 576. Une expédition dans la Lisière rapporte de l'ordre de 40 à 60 pièces d'argent avec une ou deux captures revendues, une expédition dans le Marais de 100 à 130. Ces estimations datent des expéditions de 4 à 6 étapes ; depuis l'étape 26, elles en comptent 3 dans la Lisière et 4 dans le Marais, et 5 partout depuis l'étape 33. Une expédition croise le marchand 27 fois sur 100 dans la Lisière, 34 dans le Marais. À vérifier en jouant.

## 15. Multijoueur (après l'adaptation à Discord)

- Combats contre d'autres vrais joueurs. Le vainqueur ne peut pas prendre de monstre à son adversaire. Depuis les étapes 59 à 61 : le duel en temps réel (section 15.3).
- Échange et don de monstres entre joueurs : depuis les étapes 55 à 58 (section 15.1).
- Expéditions en groupe avec un ami : décidées le 24/10/2026, dans la Tour sans fond seulement (sections 9.5 et 15.5).
- Système d'amis : remplacé pour l'instant par la liste des joueurs du bot (section 15.1).

### 15.1 Don et échange de monstres

Décidé le 20/10/2026, pour les étapes 55 à 58 (choix de l'utilisateur) :

- Le multijoueur ne concerne que le bot Discord : la console reste une partie seule. Ses règles sont dans le noyau.
- Au camp, un bouton "Échanges" ouvre l'écran des échanges ; depuis l'étape 60, il s'appelle "Autres joueurs" et sert aussi aux défis (section 15.3). Le joueur y choisit un autre joueur dans la liste des joueurs qui ont une partie sur le bot (nom du héros et pseudo Discord), puis un de ses monstres, et enfin un monstre de l'autre joueur, dont il voit la liste, pour un échange, ou aucun, pour un don.
- L'autre joueur accepte ou refuse l'offre depuis son écran des échanges. Celui qui l'a faite peut la retirer tant qu'elle n'est pas acceptée. Chaque joueur a au plus une offre envoyée et une offre reçue en attente. Le bot prévient par message privé de l'arrivée d'une offre, puis de son acceptation ou de son refus. Textes dans docs/contenu-v1.md, section 13.
- Au moment de l'acceptation, les deux joueurs doivent être au camp, sans question en attente ; sinon, l'offre reste en attente. Elle n'est plus valable si l'un des monstres a changé ou changé de place depuis : elle est alors retirée.
- Un échange : chaque monstre prend la place, dans l'équipe ou au camp, de celui qu'il remplace. Un don : le monstre rejoint l'équipe s'il y reste une place, sinon le camp ; si les deux sont pleins, le don est refusé, sans relâcher de monstre.
- On ne donne pas son dernier monstre, en comptant ceux partis en expédition automatique (section 13.2). Un monstre parti en expédition automatique ne s'offre pas. Un Apôtre ne se donne ni ne s'échange, et ne se demande pas : il reste à celui qui l'a vaincu (Q78).
- Le monstre garde son surnom, son niveau, son XP, ses attaques, son habileté, son statut shiny et ses valeurs internes. Son espèce passe "capturée" au codex de celui qui le reçoit (section 5.6).
- Le monstre reçu porte la marque "reçu de" suivie du nom du héros qui l'a cédé, gardée même après une évolution ; un nouveau don la remplace.

### 15.2 Obéissance des monstres reçus

Décidé le 20/10/2026, pour l'étape 55 (choix de l'utilisateur) :

- Seul un monstre reçu d'un autre joueur peut désobéir. Il obéit jusqu'à sa limite : le niveau maximal de la zone la plus avancée que le joueur a ouverte, plus 5. Soit 12 avec la seule Lisière (7), 18 avec le Marais (13), 25 avec la Vallée (20) et 32 avec l'Abbaye des Cendres (27).
- Au-dessus de sa limite, chaque fois que le joueur lui ordonne d'attaquer, il a 10 % de chances par niveau au-dessus de la limite, 50 % au plus, d'ignorer l'ordre : il ne fait rien ce tour, comme un monstre bloqué par la paralysie (section 10.5). Ce tirage n'a lieu que pour un monstre au-dessus de sa limite, avant celui de la paralysie. Changer de monstre, le soigner, lancer un filet ou fuir ne sont pas touchés.
- La fiche d'un monstre reçu rappelle de qui il vient et sa limite.
- L'obéissance ne joue pas sur les expéditions automatiques (hypothèse retenue, Q89).

### 15.3 Duel en temps réel

Décidé le 20/10/2026, pour les étapes 59 à 61 (choix de l'utilisateur) :

- Sur le bot Discord seulement. Depuis l'écran "Autres joueurs" du camp, le joueur choisit un autre joueur et le défie en duel. L'autre accepte ou refuse ; celui qui a lancé le défi peut le retirer. Un défi à la fois par joueur, envoyé ou reçu. Le bot prévient par message privé du défi, puis de son acceptation ou de son refus. Textes dans docs/contenu-v1.md, section 14.
- Le défi s'accepte si les deux joueurs sont au camp, sans question en attente, et ont chacun au moins un monstre dans l'équipe. Pendant le duel, le message de chacun montre le duel à la place du camp.
- Duel amical : chacun combat avec une copie de son équipe, soignée. Aucun gain ni perte : ni XP, ni argent, ni monstre KO après le duel, et rien n'est sauvegardé. Les monstres gardent leur niveau. Un duel en cours est perdu, sans conséquence, si le bot redémarre.
- Les règles sont celles du combat contre un humain (section 10) : attaquer, changer de monstre, donner l'ordre de son héros (section 4.2). Chaque côté a le bonus de classe et d'équipement de son héros (sections 4.2 et 4.3), et ses monstres reçus peuvent lui désobéir (section 15.2). Ni soin, ni filet, ni fuite.
- À chaque tour, les deux joueurs choisissent leur action sans voir celle de l'autre ; le tour se joue quand les deux ont choisi. Les changements de monstre passent avant les attaques, celui du joueur qui a lancé le défi le premier ; l'ordre des attaques suit la vitesse, avec un tirage à égalité (section 10.2). Chacun remplace lui-même son monstre KO avant le tour suivant.
- Le joueur qui a choisi attend l'autre : son message le dit, avec un bouton Actualiser qui montre le tour une fois joué. Si l'autre n'a toujours pas joué 5 minutes après lui, il peut réclamer la victoire par forfait. Chacun peut abandonner à tout moment, ce qui donne la victoire à l'autre.
- Le duel se gagne quand toute l'équipe de l'autre est KO. Si les deux équipes tombent au même tour, le joueur dont le dernier monstre est tombé le premier perd.
- Pas de palmarès des duels pour l'instant. Remplacé le 20/10/2026, pour les étapes 70 et 71 : voir la section 15.4.

### 15.4 Amis et palmarès des duels

Décidé le 20/10/2026, pour les étapes 70 et 71 (choix de l'utilisateur) :

- Sur le bot Discord seulement, comme le reste du multijoueur.
- Chaque joueur tient sa propre liste d'amis : il ajoute ou retire un autre joueur, depuis l'écran "Autres joueurs", une fois ce joueur choisi, sans demande à accepter ni message à l'autre. La liste est gardée dans sa sauvegarde.
- Les amis ne restreignent rien : ils sont proposés en tête du menu des joueurs (qui en montre 25 au plus), marqués "ami", puis viennent les autres, toujours triés par nom du héros. Les offres et les défis restent ouverts à tous.
- Le palmarès compte les victoires et les défaites en duel de chaque joueur, gardées dans sa sauvegarde. Un abandon ou un forfait compte comme une défaite pour celui qui le subit et une victoire pour l'autre (hypothèse retenue, Q94). Seul le palmarès est enregistré à la fin d'un duel : le duel lui-même ne change toujours rien d'autre.
- L'écran "Autres joueurs" montre en tête le palmarès du joueur, et propose un classement de tous les joueurs du bot qui ont livré au moins un duel, par victoires, puis par défaites, les moins nombreuses d'abord, puis par nom du héros (hypothèse retenue, Q94).
- La console reste une partie seule : elle garde la liste d'amis et le palmarès dans la sauvegarde sans les montrer.
- Précisé à l'étape 71, en codant : le palmarès est noté dans la sauvegarde de chaque joueur dès la fin du duel, une seule fois, sans attendre que chacun revienne au camp. Les boutons "Ajouter aux amis" ou "Retirer des amis" et le palmarès de l'autre joueur apparaissent une fois ce joueur choisi dans le menu ; comme le choix d'un joueur, ils ne sont donc pas proposés tant qu'une offre envoyée attend sa réponse. Le classement est un écran à part, ouvert par un bouton "Classement" de l'écran "Autres joueurs".

### 15.5 Expédition en groupe

Décidé le 24/10/2026, pour les étapes 99 à 102 (choix de l'utilisateur) :

- Sur le bot Discord seulement, et uniquement dans la Tour sans fond (section 9.5).
- Depuis l'écran "Autres joueurs", un joueur invite un autre joueur à descendre avec lui ; l'autre accepte ou refuse, et celui qui invite peut retirer l'invitation. Une invitation à la fois par joueur, envoyée ou reçue. Le bot prévient par message privé de l'invitation, puis de son acceptation ou de son refus. L'invitation s'accepte si les deux joueurs sont au camp, sans question en attente, ont ouvert la Tour et ont au moins un monstre dans l'équipe (hypothèse retenue, sur le modèle des défis, Q101).
- Chacun descend avec ses vrais monstres et son propre sac : contrairement au duel, l'expédition en groupe change les parties, comme une expédition seul.
- Combat à équipe commune : les deux équipes forment un seul camp, celle de l'hôte puis celle de l'invité, avec un monstre actif à la fois. Le propriétaire du monstre actif choisit l'action : attaquer, faire entrer un autre de ses monstres, soigner un monstre du groupe avec ses propres potions, lancer un filet (la capture lui revient) ou fuir (le groupe fuit). L'autre attend. Quand un monstre tombe, son propriétaire fait entrer un des siens ; s'il n'en a plus, l'autre joueur fait entrer un des siens. Chaque héros donne son ordre une fois par combat, à un de ses monstres actif. Chaque monstre a le bonus de classe, d'équipement et de rôle de son propriétaire, et obéit selon la limite de son propriétaire (sections 4.2, 4.3 et 15.2).
- Chaque monstre gagne son XP comme d'habitude ; chacun règle les attaques à apprendre et les évolutions de ses monstres.
- Butin : chacun le sien. Chaque coffre est tiré pour chacun et va dans son sac ; chaque monstre sauvage KO lâche sa ressource, avec un tirage pour chacun. Un monstre rare pacifique peut être pris par les deux, chacun son exemplaire.
- Entre deux étapes, chacun peut soigner ses monstres et changer l'ordre de son équipe. Continuer exige l'accord des deux : si l'un choisit de rentrer, le groupe rentre au camp, chacun avec ses gains.
- Défaite : quand tous les monstres des deux joueurs sont KO, chacun subit une défaite ordinaire et perd son sac (section 9.3).
- Absence : 5 minutes après le moment où l'autre aurait dû jouer, celui qui attend peut ramener le groupe au camp, chacun gardant ses gains ; en combat, cela compte comme une fuite réussie.
- Comme une expédition seul, l'expédition en groupe n'est pas sauvegardée en cours : si le bot redémarre, chacun reprend à sa dernière sauvegarde.
- Précisé aux étapes 99 et 100, en codant (hypothèses retenues, Q101) : l'expédition de l'hôte tire les étapes et les monstres, celle de l'invité les suit ; chacun tire son propre coffre, avec les capsules des zones qu'il a menées au bout. Un monstre rare pacifique est le même pour les deux : chacun prend ou laisse son exemplaire. Après une victoire, chacun capture pour soi les monstres KO qu'il veut, le premier qui choisit l'emporte, et le combat se termine quand chacun a fini de capturer. Chacun lance ses filets avec les améliorations de sa forge. Le gardien n'a pas de bouton "Combattre" en groupe : son texte précède aussitôt le combat. Un soin sans effet n'est pas confirmé en combat commun. Pendant ses attaques à apprendre ou ses évolutions, un joueur ne bloque l'autre que pour continuer ; quand le groupe rentre, chacun rentre une fois ses suites réglées.
- Précisé aux étapes 101 et 102, en codant : pour que l'écran "Autres joueurs" tienne dans un message, un joueur n'a qu'un défi ou qu'une invitation en attente à la fois, envoyé ou reçu (hypothèse retenue, Q101). Une fois un joueur choisi, l'écran propose "Inviter dans la Tour" à côté de "Défier en duel" ; l'invitation reçue ou envoyée y apparaît avec "Accepter l'invitation" et "Refuser l'invitation", ou "Retirer l'invitation". Pendant la descente, chaque écran commence par "Descente avec {héros}." ; celui qui agit voit les écrans d'un combat seul, où il ne fait entrer que ses monstres ; l'autre voit l'état du combat, "En attente de {héros}." et un bouton "Actualiser" ; celui qui veut continuer voit "Vous voulez continuer : vous attendez {héros}.", avec "Actualiser" et "Rentrer au camp". Entre deux étapes, "{héros} veut continuer." quand le compagnon a déjà choisi. Après 5 minutes d'attente, un bouton "Ramener le groupe au camp" apparaît. Les suites des choix du compagnon s'affichent au prochain affichage ; un récit trop long perd ses plus anciens passages, pour que la question reste entière, pour tous les écrans du bot.

## 16. Questions ouvertes et valeurs à définir

Les numéros des questions résolues ne sont pas réattribués, pour que les références restent stables.

- Q5. Types Stellaire, Obscur, Lumière et Son (après la V1) : relations d'efficacité, et différence entre les types Ténèbres et Obscur. L'Obscur, réservé aux Apôtres, est tranché depuis l'étape 30, la Lumière et le Son depuis le 24/10/2026 (section 5.2). Restent les autres relations du Stellaire (Q76).

Les questions Q16 et suivantes ont été relevées le 17/10/2026 lors du découpage de la V1 en étapes. Chacune indique l'étape de développement qui en a besoin : 1 fondations (types, stats, monstres), 2 combat 1 contre 1, 3 combat complet (changement, statuts, fuite, capture), 4 progression, 5 héros, équipe et camp, 6 expédition manuelle, 7 sauvegarde, 8 contenu et boucle de jeu complète.

- Q19. (Après la V1) Effet du rôle soutien ou offensif, en combat et en expédition.

Les questions Q59 et suivantes ont été relevées le 18/10/2026 lors du découpage de l'adaptation à Discord en étapes : 9 déroulement de la partie commun à la console et au bot, 10 squelette du bot, 11 une partie par joueur Discord, 12 jeu complet sur Discord, 13 déploiement.

- Q64. (Après l'étape 12) Sprites des monstres (section 5.1) : source des images et droits d'utilisation.

Les questions Q65 et suivantes ont été relevées le 18/10/2026 lors de la conception des expéditions automatiques, découpées en étapes : 14 règles du noyau, 15 déroulement et sauvegarde en version 3, 16 console, 17 bot Discord.

- Q66. (Après l'étape 17) Protections des expéditions automatiques (section 12.5) : objets de protection, effet du rôle soutien (Q19), valeurs des protections et façon dont elles s'appliquent à la répartition des dégâts avec report.

Les questions Q67 et suivantes ont été relevées le 18/10/2026 après les premiers tests sur Discord, découpés en étapes : 18 corrections (types au codex, mise en valeur, messages publics pour les tests), 19 Templier et monstre de départ attitré, 20 toute l'équipe en expédition automatique, puis l'économie : 21 argent et sauvegarde en version 4, 22 marchand, rencontre et revente (version 5), 23 attaques au camp et capsules (version 6), 24 écrans de la console et du bot.

Les questions Q72 et suivantes ont été relevées le 18/10/2026 après les essais de l'économie, découpés en étapes : 25 interface, 26 règles (coffre et sac, équipe vide, revente, étapes fixes), 27 classe Assassin et nouveaux monstres, 28 combats contre des humains, 29 troisième zone, Baron et Apôtre de type Obscur, 30 marchands par zone. Découpage révisé le 19/10/2026 : 28 shiny et habiletés, 29 combats contre des humains, 30 troisième zone, Baron et Apôtre de type Obscur, 31 nécromancien et âmes de marchands.

- Q76. (À vérifier en jouant) Équilibrage du combat contre le Baron pendu, Apôtre Obscur : seuls les types Féérique (Cerf blanc, Elfe), Psychique (aucun monstre ne l'a encore) et Stellaire (Schnoz) lui sont très efficaces. Autres relations du type Stellaire, neutre envers les types autres que l'Obscur (voir Q5). Le Chat noir, de type Ténèbres, n'a plus d'avantage contre lui : à vérifier.
- Q79. (Plus tard) Le Schnoz, surnom donné par les fans au chimimoryo, créature astrale croisée près du Qliphoth dans l'épisode 215 du manga (page https://berserk.fandom.com/wiki/Chimimoryo transmise par l'utilisateur), est une créature de l'œuvre, protégée : sans souci pour un jeu privé, mais si le bot devient public, mieux vaudra une créature qui s'en inspire, avec son propre nom et sa propre apparence (voir aussi Q64). Repris de Q73, tranchée.

Les questions Q81 et suivantes ont été relevées le 19/10/2026 lors de la conception des étapes 32 retour des expéditions automatiques dans l'équipe, 33 retour du gardien d'une âme, 34 âmes devenues objets et rencontre de Malvoise, 35 capsules d'habileté, 36 ressources.

- Q81. (Étape 35) Capsule d'habileté : prix de 200 pièces d'argent et un coffre sur 10, hypothèses retenues, à confirmer en jouant.
- Q82. (Étape 36) Ressources : leurs noms, la chance de 1 sur 3, les prix de 5 et 15 et le rachat de toute une sorte d'un coup sont des hypothèses retenues. Leurs usages futurs (fabrication, améliorations, sections 13.4 et 13.5) restent à définir.
Les questions Q84 et suivantes ont été relevées le 19/10/2026 lors de la conception des étapes 37 règles de l'évolution et premières évolutions par niveau, 38 écrans de l'évolution, 39 pierres d'évolution et sauvegarde en version 13, 40 évolutions par lieu et par événement.

- Q84. (Étapes 37 à 40) Hypothèses retenues, à confirmer : les formes évoluées ont les mêmes habiletés que leur forme de base ; au retour d'une expédition automatique, aucune évolution n'est proposée, le monstre évolue au camp à la demande ; une condition de lieu ne se remplit qu'en expédition manuelle ; les pierres d'évolution valent 300 et se trouvent dans un coffre sur 10 ; le rendement d'XP d'une forme évoluée vaut 1,4 fois celui de sa forme de base ; la fiche du codex nomme l'évolution, même pas encore rencontrée. Restent à définir : le stade 4 (Dragon de feu, niveau 70) et le Dragon des brumes (niveau 45), à coder avec des zones plus hautes. Depuis les étapes 79 et 81, les stades 3 du niveau 36 sont codés et le Col du Givre-Noir a deux formes évoluées sauvages (section 5.4).

Les questions Q85 et suivantes ont été relevées le 19/10/2026 lors de la conception des étapes 41 améliorations du camp et sauvegarde en version 14, 42 écrans de la forge, 43 équipe de 4 monstres, puis 44 à 47 traque à plusieurs Apôtres, Abbaye des Cendres, Inquisiteur et Abbesse écorchée, Reine des ronces.

- Q85. (Étapes 41 à 43) Hypothèses retenues, à confirmer : une sauvegarde où le Bourreau est déjà vaincu a Gunther au camp dès la reprise ; une construction lancée ne s'annule pas ; le bot prévient par message privé qu'une construction est finie. Coûts et durées sont à ajuster en jouant ; l'équipe de 4 facilite les combats contre les Apôtres, à vérifier en jouant.
- Q86. (Étapes 44 à 47) Hypothèses retenues, à confirmer : la Reine des ronces apprend Lianes étrangleuses au niveau 25, en plus de Ronces maudites ; l'Inquisiteur a l'IA relevée du Bourreau ; une sauvegarde où le Baron est déjà vaincu trouve l'indice 3 au bout de sa prochaine expédition menée au bout dans la Vallée ; l'Abbaye n'a pas de marchand mort ; les nouvelles espèces de l'Abbaye n'évoluent pas pour l'instant.

Les questions Q87 et suivantes ont été relevées le 20/10/2026 lors de la conception des étapes 48 équipement du héros, 49 fabrication par Gunther et sauvegarde en version 16, 50 écrans de la console, 51 écrans du bot Discord.

- Q87. (Étapes 48 à 51) Hypothèses retenues, à confirmer : le héros n'a ni niveau ni stats propres, et n'a pas encore d'actions de soutien (section 4.2) ; aucun objet n'est réservé à une classe ; l'équipement ne joue pas sur les expéditions automatiques ; un objet disparu du jeu rend la sauvegarde illisible. Le cumul des bonus (jusqu'à +40 % sur une stat) est à vérifier en jouant, comme les coûts et les durées.

Les questions Q88 et suivantes ont été relevées le 20/10/2026 lors de la conception des étapes 52 ordres du héros dans le noyau, 53 écran de la console, 54 écran du bot Discord.

- Q88. (Étapes 52 à 54) Hypothèses retenues, à confirmer : l'ordre est disponible dès le premier combat, contre les Apôtres et les humains compris ; il ne joue pas sur les expéditions automatiques ; l'état du combat ne rappelle pas quel monstre l'a reçu. Le cumul des bonus (jusqu'à +60 % sur une stat, par exemple un Guerrier avec l'Espadon des cendres, le Croc tressé, Férocité et le Cri de guerre) est à vérifier en jouant. Le niveau et les stats propres du héros restent pour plus tard (Q87).

Les questions Q89 et suivantes ont été relevées le 20/10/2026 lors de la conception des étapes 55 monstres reçus et obéissance, avec la sauvegarde en version 17, 56 règles du don et de l'échange, 57 offres sur le bot Discord, 58 écran des échanges.

- Q89. (Étapes 55 à 58) Hypothèses retenues, à confirmer : les offres en attente sont gardées en mémoire, sans délai d'expiration, et perdues si le bot redémarre ; la sauvegarde de celui qui reçoit est écrite la première, si bien qu'un incident entre les deux écritures dupliquerait le monstre plutôt que de le perdre ; la liste des joueurs montre au plus 25 joueurs, triés par nom du héros, avec le dernier pseudo Discord connu du bot ; un monstre rendu à celui qui l'avait cédé reste marqué "reçu de" et peut lui désobéir ; l'obéissance ne joue pas sur les expéditions automatiques, et l'ordre du héros renforce aussi un monstre qui désobéit. Textes proposés sans validation : le récit d'un monstre cédé ou reçu (docs/contenu-v1.md, section 13), les refus de l'écran des échanges et ses boutons (Accepter l'offre, Refuser l'offre, Retirer l'offre, Le donner sans contrepartie, Annuler l'offre). Celui qui a fait l'offre n'en voit pas le récit : le message privé le prévient. Restent à faire : combats entre joueurs, expéditions en groupe, système d'amis (section 15).

Les questions Q90 et suivantes ont été relevées le 20/10/2026 lors de la conception des étapes 59 combat symétrique et duel dans le noyau, 60 défis sur le bot Discord et écran "Autres joueurs", 61 écran du duel.

- Q90. (Étapes 59 à 61) Hypothèses retenues, à confirmer : le délai du forfait court aussi pendant le remplacement d'un monstre KO ; l'ordre du héros se donne à tout moment avant de choisir son action, et l'autre le voit en actualisant ; un joueur en duel ne peut ni lancer ni recevoir de défi, mais les offres de don et d'échange restent possibles ; le joueur qui a lancé le défi est le premier côté du combat. Textes proposés sans validation : l'en-tête "Duel contre {héros}.", "{héros} vous défie en duel." et "Vous avez défié {héros} en duel. En attente de sa réponse." sur l'écran des autres joueurs, son en-tête "Autres joueurs : offres de monstres et duels.", et les refus des défis et du duel. Restent à faire : expéditions en groupe, système d'amis (section 15).

Les questions Q91 et suivantes ont été relevées le 20/10/2026 lors de la conception des étapes 62 règles de la pêche et sauvegarde en version 18, 63 rencontre d'Ansgar dans le Marais, 64 écran de la console, 65 bot Discord.

- Q91. (Étapes 62 à 65) Hypothèses retenues, à confirmer : une ligne lancée ne se retire pas ; si l'équipe et le camp sont pleins, la ligne ne se relève pas, avant tout tirage ; un monstre pêché se relâche sans confirmation ; les espèces aquatiques n'ont ni ressource ni évolution et ne vivent qu'à la pêche ; l'âme de Grete à retrouver ne tombe jamais à l'étape 2 du Marais, réservée à Ansgar ; la durée de 2 h, les chances des prises et le Kelpie, deuxième monstre Féérique accessible et très efficace contre les Apôtres, sont à vérifier en jouant. Textes proposés sans validation : le lancement ("Vous lancez la ligne. Elle sera prête à relever dans {durée}."), la prise ("Au bout de la ligne : {monstre}, de niveau {niveau} !", "Vous remontez {objet}. Vous la rangez dans le coffre."), la question ("Garder {monstre} ? Sinon, la prise retourne à l'eau."), le relâcher ("{monstre} retourne à l'eau."), l'état de la ligne, l'avis au camp ("La ligne d'Ansgar est prête à relever."), le message privé ("La ligne d'Ansgar est prête. Ouvrez /bersemon pour la relever."), les refus, et les libellés "Pêche d'Ansgar", "Pêche", "Lancer la ligne", "Relever la ligne", "Garder" et "Relâcher", ainsi que le bouton "Liste du codex" sur Discord (section 5.6).

Les questions Q92 et suivantes ont été relevées le 20/10/2026 lors de la conception de l'étape 66, âmes d'Apôtres.

- Q92. (Étape 66) Usage des âmes d'Apôtres tranché le 21/10/2026 : le Béhérit pourpre (section 9.4, Q99). Hypothèses retenues, à confirmer : les âmes d'Apôtres sont celles des Apôtres vaincus, sans champ à part dans la sauvegarde, tant que le Béhérit n'est pas forgé ; à la lecture d'une sauvegarde antérieure, les Apôtres capturés disparaissent sans message ; un joueur qui n'aurait plus aucun autre monstre, en comptant le camp et les expéditions automatiques, garde ses Apôtres ; une attaque qu'un Apôtre attendait de choisir est oubliée ; une expédition automatique dont l'Apôtre partait seul disparaît, sans rien rapporter ; les règles qui citent les Apôtres de la collection (revente, don et échange interdits) restent pour ce dernier cas. Textes proposés sans validation : "Âmes d'Apôtres : {noms}." au camp et la mention "Apôtre vaincu" au codex.

Les questions Q93 et suivantes ont été relevées le 20/10/2026 lors de la conception des étapes 67 règles des boosters et du Philtre chromatique, avec la sauvegarde en version 20, 68 écrans de la console, 69 écrans du bot Discord, 70 amis et palmarès dans le noyau, avec la sauvegarde en version 21, 71 écran "Autres joueurs" du bot.

- Q93. (Étapes 67 à 69) Hypothèses retenues, à confirmer : un booster se donne sans confirmation ; sur un monstre, il augmente ses PV actuels d'autant que ses PV max, sauf s'il est KO, comme le service de Malvoise ; l'étal propose le service même sans Philtre chromatique, et le refus dit ce qui manque ; le Philtre chromatique laissé par un Apôtre va dans le sac et se perd avec lui ; une sauvegarde antérieure reçoit, dans le coffre, un Philtre chromatique par Apôtre déjà vaincu ; dans le tirage du coffre, les boosters prennent les rangs 21 à 26, un par booster, dans l'ordre des stats ; à la pêche, la potion passe de 20 % à 14 %, au profit des boosters (5 %, puis un tirage parmi les six) et du Philtre chromatique (1 %). Textes proposés sans validation : ceux de docs/contenu-v1.md, section 17.
- Q94. (Étapes 70 et 71) Hypothèses retenues, à confirmer : un abandon ou un forfait compte dans le palmarès ; le classement montre au plus les 10 premiers, et ne compte que les joueurs qui ont livré au moins un duel ; un joueur qui recommence sa partie (suicide du héros) perd sa liste d'amis et son palmarès, gardés dans sa sauvegarde ; un ami dont la partie n'existe plus n'est plus montré. Textes proposés sans validation : ceux de docs/contenu-v1.md, section 18.

Les questions Q95 et suivantes ont été relevées le 20/10/2026 lors de la conception des étapes 72 machines dans le noyau, avec la sauvegarde en version 22, 73 écrans de la console, 74 bot Discord, 75 rôle en combat, 76 protections, effet offensif et charme de garde dans le noyau, avec la sauvegarde en version 23, 77 écrans de la console, 78 bot Discord.

- Q95. (Étapes 72 à 74) Hypothèses retenues, à confirmer : après une relève, le temps d'une ressource entamée est gardé, sauf si le stock était plein ; un niveau constaté des pièges ou de la cour vaut pour tout le temps écoulé depuis la dernière relève ou l'entrée du monstre ; l'habileté Instinct joue à l'entraînement ; un monstre placé à l'entraînement suit les règles d'un monstre parti en expédition automatique (il garde sa place au camp, l'équipe peut se vider) ; aucun message privé n'annonce des pièges pleins (remplacé à l'étape 113 : le bot les annonce, section 2). Rendements, coûts et durées à vérifier en jouant. Textes proposés sans validation : ceux de docs/contenu-v1.md, section 19.
- Q96. (Étapes 75 à 78) Hypothèses retenues, à confirmer : en duel, chaque joueur a le bonus de rôle de ses monstres ; un monstre ne se protège que s'il part avec au moins un autre ; le charme est consommé même si l'expédition est rappelée ; les bonus d'argent des offensifs et de Pillard s'additionnent ; le monstre protégé ne compte pas dans sa propre protection s'il est de soutien. Textes proposés sans validation : ceux de docs/contenu-v1.md, section 20.

Les questions Q97 et suivantes ont été relevées le 20/10/2026 lors de la conception des étapes 79 contenu du Col du Givre-Noir, 80 troisième chapitre de la traque, 81 stades 3 d'évolution, 82 écrans de la console et du bot, 83 achat de monstres, 84 location et sauvegarde en version 24, 85 écrans de la console, 86 bot Discord.

- Q97. (Étapes 79 à 82) Hypothèses retenues, à confirmer : le Porte-étendard a l'IA relevée ; une sauvegarde où l'Abbesse est déjà vaincue trouve l'indice 5 au bout de sa prochaine expédition menée au bout dans l'Abbaye ; le Col n'a pas de marchand mort ; le Warg et le Chevalier noir sauvages lâchent la ressource de leur forme de base. Contenu proposé par Claude et validé par l'utilisateur (docs/contenu-v1.md, section 21), dont l'équilibrage reste à vérifier en jouant, en particulier le Margrave au niveau 38 et la zone de 9 niveaux. Texte proposé sans validation : l'âme du Margrave ("L'âme du Margrave de givre s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.").
- Q98. (Étapes 83 à 86) Hypothèses retenues, à confirmer : chaque retour au camp d'une expédition manuelle compte une expédition de la location, que le monstre loué y soit parti ou non, et les expéditions automatiques ne comptent pas ; le monstre loué ne compte pas comme capturé au codex (son espèce y passe seulement rencontrée) ; il ne compte pas parmi les autres monstres quand le joueur vend, donne ou relâche, pour qu'on ne garde jamais un monstre loué pour seul monstre ; une espèce qui vit dans plusieurs zones se vend au niveau minimal le plus bas ; l'achat et la location se font sans confirmation, comme les autres achats. Textes proposés sans validation : ceux de docs/contenu-v1.md, section 22.

Les questions Q99 et suivantes ont été relevées le 21/10/2026 lors de la conception des étapes 87 actions du monstre loué sur Discord, 88 Béhérit pourpre et sauvegarde en version 25, 89 Éclipse et sacrifice, 90 combat en deux phases contre le Démiurge, 91 console, 92 bot Discord.

- Q99. (Étapes 88 à 92) Hypothèses retenues, à confirmer : le Béhérit se forge sans autre coût que les âmes ; le monstre sacrifié ne doit pas laisser l'équipe vide, et un monstre reçu d'un autre joueur peut être sacrifié ; une attaque qu'il attendait de choisir est oubliée ; la seconde forme n'entre au codex, comme rencontrée, qu'une fois apparue en combat ; le Démiurge a, comme un Apôtre, 1 chance sur 512 d'être shiny, forme par forme ; l'Éclipse n'est jamais explorée, ne compte pas pour l'obéissance ni pour la cour d'entraînement ; une sauvegarde antérieure n'a pas de Béhérit. Une rareté à part, "Démiurge", sert au codex ("vaincu"). Le nom "Béhérit" vient de Berserk, comme le Schnoz : à revoir si le bot devient public (Q79). L'équilibrage des deux formes est à vérifier en jouant.

Les questions Q100 et suivantes ont été relevées le 24/10/2026 lors de la conception des étapes 93 types Lumière et Son dans le noyau, 94 contenu de l'Abbaye des Cendres, 95 vérification des écrans de la console et du bot.

- Q100. (Étapes 93 à 95) Hypothèses retenues, à confirmer : au codex, la Cloche hurlante et le Séraphin de vitrail viennent après les deux formes du Démiurge ; la Cloche lâche l'Éclat de bronze (5 pièces d'argent), qui entre aussi dans les pièges de l'Abbaye, et Malvoise la vend, comme les autres communes d'une zone menée au bout ; le Séraphin se loue comme les autres rares ; les trois rares de l'Abbaye sont tirés à parts égales, si bien que l'Ange déchu devient plus rare (1 monstre rare sur 3 au lieu de 1 sur 2) ; l'Inquisiteur garde son équipe ; la Cloche et le Séraphin n'évoluent pas. Pas de capsule Son ni Lumière, faute d'une deuxième espèce de chaque type. À vérifier en jouant : l'immunité du Spectre au Son (le Squelette et le Wendigo ne craignent rien de la Cloche), la Lumière contre les Apôtres (le Séraphin, comme l'Ange déchu, facilite le combat contre le Margrave et le Démiurge), et la Lance de lumière, attaque physique d'un monstre dont l'Attaque est faible. Aucun texte affiché n'est ajouté, en dehors des noms validés.

Les questions Q101 et suivantes ont été relevées le 24/10/2026 lors de la conception des étapes 96 Tour sans fond dans le noyau, avec la sauvegarde en version 26, 97 écrans de la console, 98 bot Discord seul et classement des records, 99 combat à équipe commune, 100 expédition en groupe dans le noyau, 101 invitations sur le bot Discord, 102 écrans de l'expédition en groupe.

- Q101. (Étapes 96 à 102) Hypothèses retenues, à confirmer : le gardien ne lâche pas de ressource, comme un humain ou un Apôtre ; les espèces de plusieurs zones ne comptent qu'une fois dans les tirages de la Tour ; le Warg et le Chevalier noir, communs du Col, y vivent dès le niveau 28 ; les capsules de la Tour sont celles des zones menées au bout par le joueur, au moment du départ ; l'invitation suit les règles des défis (au camp, sans question en attente, au moins un monstre dans l'équipe), et un joueur en expédition de groupe ne peut ni lancer ni recevoir de défi ou d'invitation. Précisé en codant : voir la fin de la section 15.5 (tirage des étapes par l'hôte, rare pacifique en double, captures après la victoire, filets, gardien sans bouton, soin non confirmé, un défi ou une invitation à la fois). Un joueur en expédition en groupe peut recevoir un défi, mais ne l'accepte qu'une fois au camp. Équilibrage à vérifier en jouant : la montée de 2 niveaux par étape, les gardiens à +3 niveaux et les coffres. Textes proposés et validés : l'entrée, le gardien et le repos (docs/contenu-v1.md, section 25) ; textes proposés sans validation : le nouveau record, les libellés, le classement des records, les invitations et les écrans de la descente, listés dans la même section.

Les questions Q102 et suivantes ont été relevées le 25/10/2026 lors de la conception des étapes 103 filets dans le noyau, avec la sauvegarde en version 27, 104 vente par Malvoise et fabrication par Gunther, 105 écrans de la console, 106 écrans du bot Discord.

Les questions Q103 et suivantes ont été relevées le 25/10/2026 lors de la conception des étapes 107 carnet de traque du codex et 108 Philtre chromatique.

Les questions Q111 et suivantes ont été relevées le 25/10/2026 lors de la conception des étapes 124, évolutions selon la météo, et 125 à 127, capacité du sac.

- Q111. (Étapes 124 à 127) Hypothèses retenues, à confirmer : une Fée noire, une Fée-sorcière ou un Basilic qui a déjà passé son niveau d'évolution attend la bonne météo, comme le Loup, le Corbeau et la Vipère attendent le bon moment ; la fin d'une étape est l'arrêt entre deux étapes, ou le retour au camp après la dernière ; dans la Tour à deux, chaque joueur jette dans son propre sac ; les filets lancés et les soins utilisés en combat libèrent leur place aussitôt ; une capsule d'attaque prend une place, comme tout objet. Textes proposés sans validation (docs/contenu-v1.md, sections 32 et 33).

Les questions Q110 et suivantes ont été relevées le 25/10/2026 lors de la conception des étapes 121 à 123, météo.

- Q110. (Étapes 121 à 123) Hypothèses retenues, à confirmer : les créneaux de 30 minutes partent de minuit UTC, comme le cycle du jour ; une météo n'a pas d'autre effet que le bonus de types (ni sur les espèces, ni sur les niveaux, ni sur les récompenses) ; le temps restant affiché court jusqu'au changement de météo de la zone, sur les créneaux suivants s'ils tirent la même ; une zone sans météo possible reste au calme. Le cyclone et la tempête de sable attendent une zone de mer et un désert, à concevoir. Textes proposés sans validation (docs/contenu-v1.md, section 32).

Les questions Q109 et suivantes ont été relevées le 25/10/2026 lors de la conception des étapes 118 à 120, cycle du jour et de la nuit.

- Q109. (Étapes 118 à 120) Hypothèses retenues, à confirmer : le cycle part de minuit UTC ; les monstres rares, même pacifiques, ont aussi leurs 2 niveaux de plus la nuit ; le bonus de types et celui d'XP jouent contre les monstres des âmes de marchands et le Noyé d'Ansgar, dont l'espèce et le niveau ne changent pas ; si une zone n'avait aucune espèce à un moment, toutes y sortiraient ; la fiche d'une espèce nocturne ou diurne le dit, comme toute fiche, une fois l'espèce capturée. Textes proposés sans validation (docs/contenu-v1.md, section 31).

Les questions Q108 et suivantes ont été relevées le 25/10/2026 lors de l'étape 117, réglages issus de l'équilibrage.

- Q108. (Étape 117) Hypothèses retenues, à confirmer : une Mandragore, une Mandragore ancienne ou un Arbre-pendu qui a déjà passé le niveau 7 garde le Cri strident s'il le connaît, et n'apprend pas Secousse, sauf à naître du rite ou à être capturé de nouveau ; la sauvegarde ne change pas de format, les niveaux du Baron et du Bourreau n'y étant pas gardés.

Les questions Q107 et suivantes ont été relevées le 25/10/2026 lors de l'étape 116, équilibrage.

- Q107. (Étape 116) Les réglages proposés sont tranchés à l'étape 117 (section 17). Restent ouverts : aucune durée cible de partie n'est fixée, alors que le joueur automatique abat le Margrave en 54 à 61 expéditions, sans expéditions automatiques ni cour d'entraînement ; l'équipe de 4 et les objets des paliers 2 et 3 restent des objectifs à farmer, faute de ressources rares (choix de l'utilisateur), à revoir si cela lasse en jouant. Hypothèse de l'outil, à confirmer : un choix par minute de temps réel, pauses comprises.

Les questions Q106 et suivantes ont été relevées le 25/10/2026 lors de la conception de l'étape 115, récompenses du codex, avec la sauvegarde en version 29.

- Q106. (Étape 115) Hypothèses retenues, à confirmer : les paliers atteints avant l'étape se réclament à la reprise ; une récompense se réclame un palier à la fois, dans l'ordre ; si des espèces s'ajoutent, le codex complet suit leur nombre et aucun palier au-delà de 50 n'est encore défini ; le Sceau du bestiaire ne se perd pas et ne se fabrique pas. Textes proposés sans validation : la progression, le libellé de la réclamation, le récit et le refus (docs/contenu-v1.md, section 30).

Les questions Q105 et suivantes ont été relevées le 25/10/2026 lors de la conception des étapes 110 Rite du sang mêlé dans le noyau, avec la sauvegarde en version 28, 111 écrans de la console, 112 bot Discord.

- Q105. (Étapes 110 à 112) Hypothèses retenues, à confirmer : un seul rite à la fois ; les espèces rares peuvent se reproduire ; le petit a une place réservée au camp dès le lancement, et le rite est refusé s'il n'en reste aucune ; un rite lancé ne s'annule pas ; le petit ne garde ni surnom ni marque "reçu de", et ses parents ne gagnent rien. Pour l'attaque d'œuf (étape 114) : une attaque d'œuf oubliée ne se réapprend pas au camp ; si le petit connaissait déjà 4 attaques au niveau 1, elle prendrait la place de la plus ancienne (aucune forme de base ne l'atteint aujourd'hui) ; les familles du Feu follet, de l'Armure hantée et du Squelette gardent l'attaque de leur forme de base. Textes proposés sans validation : la confirmation, le lancement, l'état du rite, l'avis au camp, la naissance et les refus (docs/contenu-v1.md, section 28).

Les questions Q104 et suivantes ont été relevées le 25/10/2026 lors de la conception de l'étape 109, accès direct à la cour d'entraînement.

- Q104. (Étape 109) Hypothèses retenues, à confirmer : l'accès direct n'apparaît qu'une fois la cour construite, comme Malvoise ou Gunther n'apparaissent qu'une fois au camp ; dans la console, l'entrée vient après celles des personnages, avant "Options", pour ne pas décaler les numéros des entrées courantes. À concevoir plus tard : la reproduction des monstres (idée de l'utilisateur), dont il faudra décider le lieu (la cour ou un nouveau bâtiment), les parents admis, ce qui se transmet (espèce, valeurs internes, attaques, shiny) et la durée.

- Q103. (Étapes 107 et 108) Hypothèses retenues, à confirmer : sur Discord, le carnet de traque suit la liste du codex, à sa dernière page, plutôt que d'avoir son propre bouton ; le Démiurge est toujours la dernière entrée, car l'ordre des victoires n'est gardé que pour les Apôtres (un Apôtre ajouté plus tard et abattu après le Démiurge viendrait avant lui). Relevé sans être corrigé : la fiche d'une espèce dont l'évolution demande un Apôtre vaincu le nomme, même s'il n'est pas encore abattu ("une fois l'Apôtre Baron pendu vaincu").

- Q102. (Étapes 103 à 106) Hypothèses retenues, à confirmer : la capture garantie d'un monstre KO, après une victoire ou sur un humain vaincu, prend le filet le plus simple qui reste, sans demander lequel ; une sauvegarde antérieure reçoit 10 filets de chanvre au coffre ; Malvoise ne rachète pas les filets ; les filets trouvés dans un coffre ne dépendent pas de la zone. Sur Discord, l'étal vend les filets par lots de 5, et l'en-tête du camp compte les ressources par sortes au lieu de les détailler (section 11.2). À vérifier en jouant : les prix, les recettes, les 10 filets du départ et la part des coffres. Noms et effets validés (docs/contenu-v1.md, section 26) ; textes proposés sans validation : le lancer ("Vous lancez un filet lesté sur ..."), l'achat, la fabrication, les libellés et les refus, listés dans la même section.

- Q83. (Étapes 33 et 34) Hypothèses retenues, à confirmer : le gardien d'une âme à retrouver attaque à une étape tirée parmi celles qui précèdent la dernière ; le don d'une âme à Malvoise se fait sans confirmation ; une partie antérieure qui n'a pas recueilli l'âme de Bertrand le rencontre à l'étape 4 de sa prochaine expédition dans la Lisière, même avec Malvoise déjà au camp ; le texte du brigand de l'étape 5 ne change pas, même s'il a perdu son Loup à l'étape 4.

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
- Sur Discord, le bot prévient le joueur par message privé du retour d'une expédition, et à son démarrage pour celles rentrées pendant qu'il était éteint.
- Hypothèses validées : une expédition automatique n'a aucun effet sur la traque, l'exploration ni le codex ; les monstres envoyés gardent leur place au camp, comptée dans sa limite, et ne peuvent être ni relâchés, ni renommés, ni échangés ; une expédition manuelle reste possible dans la même zone ; la durée n'est pas tirée au hasard.

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

18/10/2026 (étape 18) :

- Pas de couleur dans la console, qui afficherait des caractères parasites dans la vieille console de Windows 10 : la mise en valeur ne vaut que sur Discord. Remplace le rouge prévu pour la console.

18/10/2026 (étape 19) :

- Q67 : le dernier paragraphe de l'introduction présente le monstre attitré de la classe choisie, un texte par classe (docs/contenu-v1.md).
- L'écran du choix de la classe annonce le monstre de départ de chaque classe.

18/10/2026 (étape 20) :

- Avec un camp plein, la règle de la section 12.1 est gardée : un monstre pris dans l'équipe a besoin d'une place au camp, où il reviendra. Le refus l'explique, et le raccourci "toute l'équipe" n'est proposé que si ce départ est possible.

18/10/2026 (économie) :

- Découpage par couches : 21 argent et sauvegarde en version 4 ; 22 marchand, rencontre et revente, version 5 ; 23 attaques au camp et capsules, version 6 ; 24 écrans de la console et du bot. Une version de sauvegarde par étape qui change le format.
- Q68 : montants de la section 14.5. Coffres : 10 à 25 pièces d'argent dans la Lisière, 25 à 50 dans le Marais. Expéditions automatiques réussies : 15 à 30 et 40 à 80. Revente : niveau x 5, 15 ou 40 selon la rareté de l'espèce, doublée pour un shiny. Potion : 20. Capsules : 150 à 300. Départ sans argent.
- Chaque espèce a une rareté (commune, rare, Apôtre), qui fixe son prix de revente.
- Q69 : six capsules, trois attaques nouvelles pour les types des monstres de départ (Lisière) et trois attaques d'autres espèces (Marais) ; un coffre sur 5 contient une capsule de sa zone que le joueur n'a pas encore ; le marchand les vend toutes.
- Q70 : tant qu'il n'est pas au camp, le marchand se rencontre à 10 % des étapes, pris sur les combats (combat 70 %, marchand 10 %, coffre 15 %, monstre rare 5 %).
- Une défaite en expédition manuelle fait perdre l'argent et les capsules trouvés pendant l'expédition, acquis seulement au retour au camp ; l'argent déjà au camp reste.
- Les prix du marchand sont fixes, pour être connus avant l'échange ; la section 1.4 vise les gains des combats et des expéditions.
- Hypothèses retenues, présentées avec la conception : le marchand reste au camp même après une défaite, mais quitter le jeu avant le retour annule la rencontre ; on ne lui achète rien en expédition ; les achats se font sans confirmation ; tout monstre se revend comme il se relâche, monstre de départ et Apôtre compris ; il ne rachète ni potions ni capsules ; l'argent se compte en pièces d'argent.
- Après un essai sur Discord de l'étape 22 (marchand rencontré mais invisible au camp, faute d'écran), l'ordre des étapes est gardé : capsules à l'étape 23, écrans à l'étape 24.

18/10/2026 (étape 24) :

- Écrans de l'économie, dans la console et sur Discord : le marchand au camp (étal, achat de potions et de capsules, vente avec le prix de chaque monstre, vente aussi depuis la gestion), les attaques d'un monstre depuis la gestion, et le rappel, entre deux étapes, de ce qui a été trouvé en chemin. Sur Discord, les potions s'achètent par 1 ou 5.

18/10/2026 (retours des essais de l'économie) :

- Découpage : 25 interface, 26 règles, 27 classe Assassin et nouveaux monstres, 28 combats contre des humains, 29 troisième zone, Baron et Apôtre de type Obscur, 30 marchands par zone.
- Interface (étape 25) : types, stats et valeurs internes d'un monstre dans sa gestion et l'écran de ses attaques ; total des stats de base au codex ; plus de liste « Vos monstres » au camp ; tri du camp par nom, niveau ou type, qui change l'ordre pour de bon ; affichage du combat de la section 10.6.
- Les types d'une espèce n'apparaissent, au codex comme en combat, qu'à sa capture. Remplace la règle de la section 5.6 pour une espèce seulement rencontrée.
- Coffre du camp (étape 26) : argent, potions et capsules à l'abri ; le sac contient ce que le joueur emporte ; une défaite en expédition manuelle fait perdre tout le sac, captures et XP gardées. Ce qui est gagné au camp va dans le coffre, ce qui est trouvé en expédition dans le sac, où il reste au retour. Au camp, sac et coffre paient ensemble, le sac d'abord, et toutes les capsules servent. Les sauvegardes antérieures rangent tout dans le coffre. Remplace la règle du butin perdu de la section 9.3.
- Équipe vide (étape 26) : toute l'équipe peut partir en expédition automatique et reste alors vide ; pas d'expédition manuelle tant qu'elle l'est ; un seul monstre suffit pour une expédition automatique ; on ne relâche ni ne vend son dernier monstre, en comptant ceux partis. Remplace la règle de l'étape 20 et Q51.
- Revente (étape 26) : niveau x total des stats de base / 50, puis x 1, 2 ou 4 selon la rareté, et x 2 pour un shiny. Remplace la formule de Q68.
- Étapes (étape 26) : fixes par zone et affichées, 3, 4, 5, 5, 6, 6, puis 7 au plus. Remplace Q37.
- Principes décidés pour plus tard : classe Assassin (Q72), monstres inspirés de Berserk (Q73), combats contre des humains et IA (Q74), troisième zone, Baron et un indice par zone (Q75), annonce des événements importants avec un bouton, type Obscur des Apôtres (Q76), marchands par zone (Q77), événement des Apôtres réunis (Q78).

19/10/2026 (précisions sur les retours) :

- Équipe vide : la décision du 18/10/2026 est confirmée, car refaire son équipe après chaque expédition automatique est trop gênant.
- Troisième zone et nouvelle traque : l'utilisateur accepte de renommer l'Apôtre ou le Baron et de convertir les sauvegardes qui ont déjà trouvé 3 indices.
- Marchands (étape 30) : un nécromancien unique, rencontré au camp juste après la première zone menée au bout, à qui l'on rapporte l'âme des marchands tués en expédition ; son étal s'enrichit à chaque âme. Remplace le principe d'un marchand par zone.
- Les types d'une espèce restent cachés tant qu'elle n'est pas capturée, même rencontrée.
- Type Obscur : il est gardé tel quel ; l'Apôtre combat seul contre 3 monstres, et l'équilibrage se fera en jouant.
- Schnoz : monstre caché, obtenu par un easter egg ou extrêmement rare, peut-être le seul de type Stellaire (Q73).
- Le rendu des listes sur Discord doit être propre (étape 25).

19/10/2026 (étape 26) :

- Écran du coffre au camp, dans la console (« Coffre du camp », entre la gestion des monstres et le codex) et sur Discord (bouton « Coffre du camp ») : prendre ou ranger des potions, de l'argent ou une capsule, ou tout ranger. La console demande la quantité ; Discord propose des lots dans ses menus : 1, 5 ou toutes les potions, 10, 50, 100, 500 ou tout l'argent.
- Le compteur « [Étape : 2/3] » est en haut de chaque écran d'expédition, combats compris ; l'annonce d'une étape devient « Étape 2/3. ». Entre deux étapes, l'écran montre le contenu du sac, qu'une défaite ferait perdre.
- Le conseil du premier arrêt entre deux étapes parle du sac et du coffre, à la place des potions perdues.
- Sauvegarde en version 7, avec le sac et le coffre ; les versions 1 à 6 se lisent, tout rangé au coffre.

19/10/2026 (XP et suicide, à la suite de l'étape 26, sur la même branche) :

- XP : la fiche d'un monstre (gestion, attaques) et chaque gain d'XP, en combat comme au retour d'une expédition automatique, montrent l'XP totale et celle du niveau suivant, « XP : 130/216 » (section 7). Les listes de monstres restent sans XP.
- Suicide : sous-menu « Options » au camp, dans la console et sur Discord, avec « Se suicider » et une confirmation ; la sauvegarde est effacée dès la confirmation, puis un nouveau héros est créé (section 13.9). Le bot n'annonce plus les retours des expéditions de la partie effacée.

19/10/2026 (étape 27) :

- Q72 : le monstre attitré de l'Assassin est le Chat noir, de type Ténèbres, offensif en attaque physique : rapide, il frappe fort mais encaisse mal (315 de stats de base). Le dernier paragraphe de l'introduction le présente. L'Assassin donne +10 % en Vitesse, comme prévu (Q34). Son avantage contre le Baron pendu, dont il résiste aux attaques, est accepté ; il sera revu avec l'Apôtre de l'étape 29 (Q76).
- Q73 en partie : les nouveaux monstres inspirés de Berserk peupleront la troisième zone, à l'étape 29 ; un premier jet de quatre espèces est noté dans Q73.
- Q73 en partie : le Schnoz est un monstre caché, de type Stellaire, d'espèce rare. Dans toute zone, un monstre rare sur 20 est le Schnoz ; pacifique, il se laisse prendre, au niveau de la zone, et peut se rencontrer de nouveau.
- En attendant l'Obscur, le type Stellaire est neutre envers tous les types, en attaque comme en défense.

19/10/2026 (shiny et habiletés, étape 28) :

- Découpage révisé : 28 shiny et habiletés, 29 combats contre des humains, 30 troisième zone, Baron et Apôtre de type Obscur, 31 nécromancien. Les habiletés passent avant la troisième zone, pour que ses espèces en aient dès leur création.
- Un shiny a 31 dans chaque valeur interne ; les shiny des sauvegardes antérieures passent à 31 à leur reprise. Remplace "sans incidence sur les stats" (section 5.1).
- Habiletés (section 5.9) : 2 possibles par espèce, une tirée pour chaque monstre à sa création ; la première de l'espèce pour les monstres des sauvegardes antérieures. Catalogue de onze habiletés d'après la liste de l'utilisateur : cinq bonus de stats de +10 %, Affinité (+20 % de dégâts avec le bonus de type), Instinct (+20 % d'XP), Éclaireur (-20 % de durée d'expédition automatique), Traqueur (capture x1,25), Pillard (+50 % d'argent des coffres et des expéditions automatiques) et Fouineur (une potion 1 fois sur 4 après un combat gagné). Valeurs à ajuster en jouant.
- "Booste le taux de drop" est compris comme plus de butin : c'est l'habileté Pillard.
- Les habiletés d'équipe ne se cumulent pas ; celles de stats s'ajoutent au bonus de la classe ; seules les habiletés de combat jouent pour les monstres sauvages et les Apôtres.

19/10/2026 (étape 29) :

- Q74 : le premier humain est un brigand, au bout de la Lisière, qui remplace l'événement de la dernière étape tant qu'il n'est pas vaincu ; le vaincre achève l'expédition et, la première fois, donne l'indice 1 qui ouvre le Marais. Une partie qui a déjà cet indice le croise aussi, sans enjeu pour la traque.
- Son équipe : Loup de niveau 7 et Corbeau de niveau 8, valeurs internes à 15, sans shiny, première habileté de leur espèce. Victoire : 60 pièces d'argent dans le sac et l'XP habituelle.
- Ni fuite ni filet contre un humain ; après la victoire, un seul de ses monstres au choix, à coup sûr, ou aucun.
- IA simple : l'attaque aux dégâts estimés les plus hauts contre le monstre actif du joueur, la première à égalité ; ni changement de monstre ni potion.
- Sauvegarde en version 9, avec les humains vaincus.

19/10/2026 (étape 30) :

- Q75 : l'humain au bout du Marais est le Bourreau, au service du Baron pendu, qui garde son nom et son identifiant. Équipe : Armure hantée 14, Feu follet 14, Basilic 15 ; victoire : 150 pièces d'argent et l'indice 2.
- Q75 : la piste compte deux indices, Lisière puis Marais ; la Vallée des Brumes (niveaux 14 à 20, 5 étapes) s'ouvre avec l'indice 2, et le Baron pendu attend au bout de chaque expédition dans la Vallée jusqu'à sa défaite.
- Q75 : les sauvegardes de l'ancienne piste reprennent à 1 indice ; un Apôtre vaincu le reste. Sauvegarde en version 10.
- Q75 : IA du Bourreau (la plus forte attaque, et un changement de monstre quand le sien ne peut que peu ou pas blesser) et IA de l'Apôtre (achever d'abord, sinon les dégâts moyens attendus, avec une préférence pour une attaque à statut presque aussi bonne).
- Q73 : Fée noire, Squelette, Elfe et Vouivre, d'après le premier jet, dans la Vallée des Brumes ; l'Armure hantée y vit aussi. La note sur le Schnoz devient Q79.
- Q76 en partie : une attaque Obscur est très efficace contre un monstre Obscur. Le Baron pendu devient Obscur, au niveau 22, avec deux attaques Obscur ; son équilibrage se fera en jouant.
- Valeurs de la Vallée : expédition automatique de 3 h, 90 à 220 dégâts, 700 à 1300 XP, 1 à 3 potions, 80 à 150 pièces d'argent ; coffres de 50 à 90 pièces ; capsules Maléfice (300), Bourrasque (300) et Furie draconique (400).
- Les annonces décidées le 18/10/2026 arrivent : bouton "Combattre" avant un humain de fin de zone ou un Apôtre, "Lire l'indice" avant un indice.

19/10/2026 (étape 31) :

- Q77 : le nécromancien Malvoise remplace le marchand ambulant ; il arrive au camp au retour de la première expédition menée au bout dans la Lisière, vend les potions et les capsules et rachète les monstres.
- Q77 : trois marchands morts, un par zone : Bertrand, colporteur (Lisière), tué sous les yeux du joueur par un monstre de la zone qui l'attaque ; Grete, herboriste (Marais), relevée en marchand zombie, qui attaque ; Ulrich, alchimiste (Vallée), pendu au gibet et gardé par un monstre de la zone. Leur âme se rencontre à 10 % des étapes de leur zone tant qu'elle n'est pas recueillie.
- Q77 : chaque âme ajoute un soin à l'étal : remède (25), grande potion (60), élixir (150). Ils s'utilisent comme la potion.
- Le marchand zombie : Spectre et Poison, niveau 13, ni filet ni capture, hors du codex.
- Hypothèses retenues, à confirmer : une âme est acquise dès la victoire, même si l'expédition finit en défaite ; une sauvegarde qui a mené la Lisière au bout a Malvoise au camp dès la reprise.
- Le texte du marchand zombie est accordé au féminin, puisque c'est Grete.
- Sauvegarde en version 11.

19/10/2026 (étape 31, en codant) :

- Q80 : hypothèse retenue, les âmes ne se rencontrent qu'une fois Malvoise au camp ; Malvoise arrive au retour de la première expédition menée au bout, forcément dans la Lisière.
- Les textes de Bertrand et d'Ulrich ne nomment plus le monstre, pour éviter l'accord en genre ; le combat le présente aussitôt.
- Écrans des soins : choix du soin puis du monstre dans la console ; un menu unique sur Discord, qui propose chaque soin du sac avec chaque monstre à qui il peut servir.

19/10/2026 (conception des étapes 32 à 36) :

- Retour des expéditions automatiques : à l'accueil comme au rappel, les monstres rejoignent d'abord l'équipe, dans l'ordre de départ, jusqu'à 3 ; les autres vont à la fin du camp (section 12.7). Sauvegarde en version 12.
- Gardien d'une âme : après une fuite ou une défaite, il revient à coup sûr à la prochaine expédition dans sa zone, à une étape tirée, pour les trois marchands (sections 9.2 et 14.2).
- Âmes : objets rares et indestructibles, portés à part du sac. L'âme de Bertrand est garantie à l'étape 2 de la première expédition dans la Lisière. Malvoise est rencontré au bout de la première expédition menée au bout et rentre au camp avec le joueur, qui lui donne les âmes quand il veut ; chaque âme donnée ajoute son soin à l'étal. Remplace Q80, qui est retirée (section 14.2).
- Capsule d'habileté : vendue par Malvoise et trouvée dans les coffres, consommée à l'usage ; elle fait passer un monstre à l'autre habileté de son espèce (section 5.9). Prix et taux en hypothèse (Q81).
- Ressources : lâchées par les monstres sauvages battus, revendues à Malvoise (section 14.3). Q71 est retirée ; détails en hypothèse (Q82).

19/10/2026 (conception des étapes 32 à 36, précisions de l'utilisateur) :

- Le retour d'une expédition automatique garde la place réservée au camp : il n'est jamais bloqué, même équipe et camp pleins.
- Toute zone compte 5 étapes (section 9.2), au lieu de 3, 4, 5, puis jusqu'à 7.
- Bertrand est à l'étape 4 de la Lisière, au lieu de l'étape 2 : le brigand de l'étape 5 l'abat sous les yeux du joueur, et une de ses bêtes attaque. Il revient à l'étape 4 tant que son âme n'est pas recueillie ; la règle de l'étape tirée ne vaut que pour Grete et Ulrich.

19/10/2026 (après les étapes 32 à 36) :

- Q83 en partie : la bête du brigand qui attaque à l'étape 4 de la Lisière, à la place de Bertrand, est toujours un Loup de niveau 7, tiré comme un monstre sauvage ; combat ordinaire, fuite et filet compris.
- Rappel : les combats contre un humain de fin de zone ou un Apôtre sont uniques ; vaincus, ils laissent la place à un événement ordinaire. C'était déjà la règle (sections 9.4 et 10.7) et le code la suivait.

19/10/2026 (conception des étapes 37 à 40, évolution) :

- Évolution : 0 à 3 évolutions par espèce, chaque stade étant une espèce ; repères de niveau 16 / 36 / 64, 12 / 25 / 50 pour le type Insecte, 22 / 45 / 70 pour le type Dragon (section 5.4).
- Seules les premières évolutions sont codées, les zones s'arrêtant au niveau 20 ; les stades suivants sont nommés dans docs/contenu-v1.md.
- Choix de l'utilisateur : l'évolution est proposée au joueur, qui peut la refuser ; elle est proposée de nouveau au prochain niveau gagné, et possible à la demande au camp.
- Valeurs internes, shiny, surnom, niveau et attaques connues ne changent pas ; l'habileté garde sa position ; une attaque d'évolution est apprise, avec choix de l'attaque à oublier.
- Contenu validé par l'utilisateur : 13 premières évolutions, dont 9 par niveau, 2 par pierre (Pierre de braise, Pierre de lune), 1 par lieu (Armure hantée dans la Vallée) et 1 par événement (Squelette après la chute du Baron pendu), et 13 attaques d'évolution (docs/contenu-v1.md, section 8).
- Hypothèses retenues, à confirmer : Q84.

19/10/2026 (conception des étapes 41 à 47) :

- Choix de l'utilisateur : les améliorations du camp passent avant la quatrième zone.
- Améliorations : enclos (40, 50 puis 60 places au camp), filets (x1,1, x1,2 puis x1,3 sur la chance de capture) et équipe de 4 monstres, payées en ressources seules, avec une durée de construction, à constater au camp (section 13.4). Coûts et durées validés par l'utilisateur.
- Gunther, forgeron prisonnier du Bourreau, libéré à sa défaite, construit les améliorations (sections 13.4 et 14.2). Sauvegarde en version 14.
- Quatrième zone, validée par l'utilisateur pour les étapes 44 à 47 : l'Abbaye des Cendres, l'Inquisiteur, l'Abbesse écorchée, l'Ange déchu (premier monstre Psychique) et la Reine des ronces (sections 5.2, 5.4 et 9.4).
- Hypothèses retenues, à confirmer : Q85 et Q86.

20/10/2026 (conception des étapes 48 à 51, équipement du héros) :

- Choix de l'utilisateur : l'équipement du héros passe avant les autres fonctionnalités en attente (fabrication seule, protections des expéditions automatiques, multijoueur, suite de la traque, services rares).
- Q6 tranchée : 2 talismans, différents, en plus d'une arme et d'une armure (section 4.3).
- Choix de l'utilisateur : l'équipement donne des bonus de stats en pourcentage aux monstres du joueur en combat, comme la classe ; il n'est jamais perdu ; Gunther le forge dans le même atelier que les améliorations, avec une durée (section 13.5).
- Contenu validé par l'utilisateur : 14 objets, 6 armes, 3 armures et 5 talismans, en trois paliers (docs/contenu-v1.md, section 11). Sauvegarde en version 16.
- Hypothèses retenues, à confirmer : Q87.

20/10/2026 (conception des étapes 52 à 54, ordres du héros) :

- Choix de l'utilisateur : les actions de soutien du héros passent avant les autres fonctionnalités en attente (protections des expéditions automatiques, suite de la traque, multijoueur).
- Choix de l'utilisateur : un ordre par classe, gratuit, une fois par combat, sur le monstre actif, +20 % dans la stat du bonus de classe jusqu'à la fin du combat (section 4.2). Noms et textes validés (docs/contenu-v1.md, section 12).
- Choix de l'utilisateur : pas encore de niveau ni de stats propres du héros. Pas de changement de sauvegarde.
- Hypothèses retenues, à confirmer : Q88.

20/10/2026 (conception des étapes 55 à 58, don et échange de monstres) :

- Choix de l'utilisateur : le don et l'échange de monstres passent avant les combats entre joueurs, sur le bot Discord seulement (section 15.1).
- Choix de l'utilisateur : l'autre joueur se choisit dans la liste des joueurs du bot ; une offre, don ou échange, se fait puis s'accepte ou se refuse ; les règles du transfert et les textes sont validés (section 15.1, docs/contenu-v1.md, section 13).
- Choix de l'utilisateur : pas de limite de niveau à la réception, mais un monstre reçu désobéit au-dessus du niveau maximal de la zone la plus avancée ouverte plus 5, avec 10 % de chances par niveau au-dessus, 50 % au plus (section 15.2). Un Apôtre ne s'échange pas.
- Sauvegarde en version 17 : la marque "reçu de" de chaque monstre.
- Hypothèses retenues, à confirmer : Q89.

20/10/2026 (conception des étapes 59 à 61, duel en temps réel) :

- Choix de l'utilisateur : le duel en temps réel passe avant le duel contre une copie menée par l'IA, les expéditions en groupe et le système d'amis.
- Choix de l'utilisateur : duel amical sur des copies soignées, sans gain ni perte ; règles du combat contre un humain avec l'ordre du héros, sans soin ni filet ni fuite ; attente avec un bouton Actualiser et forfait après 5 minutes ; défis lancés depuis l'écran "Autres joueurs", qui remplace "Échanges" (section 15.3). Si les deux équipes tombent au même tour, la première tombée perd. Pas de palmarès. Textes validés (docs/contenu-v1.md, section 14).
- Hypothèses retenues, à confirmer : Q90.

20/10/2026 (conception des étapes 62 à 65, pêche) :

- Choix de l'utilisateur : la pêche passe avant les autres services rares, le système d'amis et le palmarès des duels.
- Choix de l'utilisateur : une ligne au camp, prête en 2 h sur l'horloge réelle, relevée par le joueur ; la prise est tirée à la relève (section 13.8).
- Choix de l'utilisateur : Ansgar, pêcheur, se sauve à l'étape 2 du Marais, contre un Noyé de niveau 13, et s'installe au camp (section 9.2).
- Contenu validé par l'utilisateur : quatre espèces aquatiques, la table des prises, le nom d'Ansgar et ses deux textes (docs/contenu-v1.md, section 15).
- Choix de l'utilisateur : le joueur garde ou relâche le monstre pêché.
- Sauvegarde en version 18.
- Hypothèses retenues, à confirmer : Q91.

20/10/2026 (étape 66, âmes d'Apôtres) :

- Choix de l'utilisateur : les Apôtres ne se capturent plus ; vaincus, ils laissent une âme, à l'usage différent de celui des âmes des marchands, à définir plus tard (idée : un Béhérit pourpre pour invoquer le Démiurge, Q92).
- Choix de l'utilisateur : dans les sauvegardes antérieures, un Apôtre capturé est changé en son âme ; au codex, la victoire ouvre la fiche de l'Apôtre, dit vaincu. Texte de l'âme validé (docs/contenu-v1.md, section 16).
- Sauvegarde en version 19, sans changement de structure.
- Hypothèses retenues, à confirmer : Q92.

20/10/2026 (conception des étapes 67 à 71, boosters, service shiny, amis et palmarès) :

- Choix de l'utilisateur : les trois petits lots, boosters de stats, service shiny et système d'amis avec palmarès des duels.
- Choix de l'utilisateur : les boosters sont permanents et ajoutent 5 à une valeur interne, 31 au plus ; ils ne se trouvent que dans les coffres (6 %) et à la pêche (5 %) ; noms proposés acceptés, la Poudre de vif-argent remplaçant la Plume de corbeau, déjà prise par une ressource (section 14.3, docs/contenu-v1.md, section 17).
- Choix de l'utilisateur : Malvoise rend un monstre shiny contre un Éclat d'astre, laissé par chaque Apôtre vaincu et pêché 1 fois sur 100 (sections 13.7 et 14.3).
- Choix de l'utilisateur : une liste d'amis personnelle, qui les met en tête du menu des joueurs sans rien restreindre ; un palmarès des victoires et des défaites, avec un classement de tous les joueurs, sur l'écran "Autres joueurs" (section 15.4).
- Sauvegarde en version 20 pour les boosters et l'Éclat d'astre, en version 21 pour les amis et le palmarès.
- Hypothèses retenues, à confirmer : Q93 et Q94.

20/10/2026 (conception des étapes 72 à 78, automatisation du camp, rôle et protections) :

- Choix de l'utilisateur : l'automatisation du camp, puis l'effet du rôle et les protections des expéditions automatiques.
- Choix de l'utilisateur : deux machines construites par Gunther, gérées depuis sa forge : des pièges qui prennent des ressources dans une zone explorée, en continu jusqu'à un stock maximal, et une cour d'entraînement qui donne une XP horaire plafonnée au niveau maximal de la zone ouverte la plus avancée (section 13.6). Valeurs proposées acceptées (docs/contenu-v1.md, section 19).
- Choix de l'utilisateur : le rôle joue en combat, pour les monstres du joueur seulement (+10 % de Défense et de Défense spéciale pour un soutien, d'Attaque et d'Attaque spéciale pour un offensif), et en expédition automatique (protection par les soutiens, +10 % d'argent par offensif debout) (sections 4.2, 12.5 et 12.6).
- Choix de l'utilisateur : un monstre protégé reporte une part de ses dégâts sur les autres, 25 % par soutien et 50 % avec un charme de garde ; le charme est fabriqué par Gunther, un par travail, contre 3 Os de squelette et 3 Fragment d'armure, en 30 minutes (sections 12.5 et 13.5).
- Sauvegarde en version 22 pour les machines, en version 23 pour les protections et le charme.
- Hypothèses retenues, à confirmer : Q95 et Q96.

20/10/2026 (conception des étapes 79 à 86, cinquième zone, troisième Apôtre, stades 3, achat et location de monstres) :

- Choix de l'utilisateur : la cinquième zone et le troisième Apôtre, avec les stades 3 d'évolution, puis l'achat et la location de monstres.
- Choix de l'utilisateur : le Col du Givre-Noir, niveaux 28 à 36, et le Margrave de givre au niveau 38, pour que les stades 3 du niveau 36 s'atteignent dans la zone (sections 9.1 et 9.4). Contenu proposé accepté : espèces, attaques, capsules, ressources, Porte-étendard, textes et cinq stades 3 (docs/contenu-v1.md, section 21).
- Choix de l'utilisateur : des formes évoluées sauvages, le Warg et le Chevalier noir, dans le Col seulement (section 5.4).
- Choix de l'utilisateur : Malvoise vend les espèces communes des zones menées au bout, au niveau minimal de leur zone, pour 4 fois leur revente, et loue les espèces rares, au niveau maximal de la zone ouverte la plus avancée, pour 3 expéditions manuelles, au prix de leur revente ; un monstre loué ne fait que combattre (section 14.2, docs/contenu-v1.md, section 22). Sauvegarde en version 24.
- Hypothèses retenues, à confirmer : Q97 et Q98.

21/10/2026 (conception des étapes 87 à 92, Béhérit pourpre et Démiurge) :

- Pas de préférence de l'utilisateur pour le grand lot : retenu l'option recommandée, le Béhérit pourpre et le Démiurge (Q92). Petit lot choisi par l'utilisateur : masquer sur Discord les actions qu'un monstre loué ne peut pas faire (étape 87).
- Choix de l'utilisateur : Malvoise forge le Béhérit pourpre des trois âmes d'Apôtres, consommées ; l'invocation exige le sacrifice définitif d'un monstre de niveau 30 ou plus, une seule fois ; le Démiurge s'affronte dans un lieu à part, l'Éclipse, en deux phases ; une défaite est ordinaire ; après la victoire, un épilogue, et le jeu continue (section 9.4).
- Contenu proposé accepté : les deux formes du Démiurge, deux attaques, l'Éclipse, les textes et 3 Éclats d'astre (docs/contenu-v1.md, section 23). Q78 est tranchée par le Béhérit.
- Sauvegarde en version 25.
- Hypothèses retenues, à confirmer : Q99.

24/10/2026 (conception des étapes 93 à 95, types Lumière et Son) :

- Choix de l'utilisateur : les types Lumière et Son. La Lumière est très efficace contre les Ténèbres, les Spectres et l'Obscur, dont elle est le troisième opposé ; le Son est très efficace contre les types Psychique, Glace et Vol, et sans effet sur le Spectre (section 5.2). Q5 est tranchée pour ces deux types.
- Choix de l'utilisateur : deux espèces nouvelles dans l'Abbaye des Cendres, la Cloche hurlante (Acier et Son, commune) et le Séraphin de vitrail (Lumière, rare et pacifique), et des attaques nouvelles ; les attaques existantes ne changent pas (section 5.3, docs/contenu-v1.md, section 24).
- Choix de l'utilisateur, après avoir relevé que les capsules n'enseignent qu'aux monstres de leur type : pas de capsule Son ni Lumière ; le Séraphin apprend la Lance de lumière au niveau 18.
- Pas de changement du format de sauvegarde : espèces et attaques y sont désignées par leur identifiant.
- Hypothèses retenues, à confirmer : Q100.

24/10/2026 (conception des étapes 96 à 102, Tour sans fond et expédition en groupe) :

- Choix de l'utilisateur : l'expédition en groupe, uniquement dans une zone infinie où le but est d'aller le plus loin possible, la Tour sans fond, qu'on descend (sections 9.5 et 15.5).
- Choix de l'utilisateur : la Tour se joue seul ou à deux, s'ouvre avec la Lisière menée au bout, monte de 2 niveaux par étape, a un gardien toutes les 10 étapes suivi d'un repos, des coffres selon la profondeur, un monstre de plus par groupe à deux, et un record avec classement (dernière étape franchie).
- Choix de l'utilisateur : en groupe, une équipe commune où le propriétaire du monstre actif choisit l'action, chacun son butin, l'accord des deux pour continuer, et le retour du groupe au camp après 5 minutes d'absence.
- Sauvegarde en version 26 pour les records.
- Hypothèses retenues, à confirmer : Q101.

25/10/2026 (conception des étapes 103 à 106, filets de capture et leur fabrication) :

- Demande de l'utilisateur : la fabrication, notamment des filets, pour qu'ils ne soient plus infinis.
- Choix de l'utilisateur : trois qualités de filets, chanvre, lesté (x1,5) et chaînes (x2), rangées dans le sac et le coffre ; toute capture en combat consomme un filet ; 10 filets de chanvre au départ ; Malvoise vend le chanvre (10) et le lesté (30) ; Gunther fabrique les trois qualités par lots de 5 ; un coffre d'expédition sur 5 en contient (sections 9.2, 11.2, 13.5 et 14.2).
- Sauvegarde en version 27.
- Précisé en codant les étapes 105 et 106 : sur Discord, achat des filets par lots de 5 et en-tête du camp qui compte les ressources par sortes, pour rester sous les 2000 caractères (section 11.2).
- Hypothèses retenues, à confirmer : Q102.

25/10/2026 (conception des étapes 107 carnet de traque et 108 Philtre chromatique) :

- Demande de l'utilisateur : une section du codex avec seulement les Apôtres de la traque, chacun ajouté une fois abattu, sans dévoiler ceux qui restent ni leur nombre.
- Choix de l'utilisateur : section à part, le carnet de traque, qui n'apparaît qu'au premier Apôtre abattu ; les Apôtres et le Démiurge quittent la liste des espèces ; le Démiurge y figure en dernier ; une ligne par victoire, "Baron pendu, abattu dans la Vallée des Brumes." (section 5.6).
- Demande de l'utilisateur : un nom autour de l'ADN et de sa manipulation pour l'objet qui rend un monstre shiny, comme les animaux albinos. Claude a relevé que le mot ADN détonne dans un univers dark fantasy et proposé Brin d'albâtre, Hélice pâle, Germe albinos et Philtre de lignée.
- Choix de l'utilisateur : le Philtre chromatique, avec des textes réécrits autour d'une fiole et d'une lignée altérée (section 14.3, docs/contenu-v1.md, section 17).
- Hypothèses retenues, à confirmer : Q103.

25/10/2026 (conception de l'étape 109, accès direct à la cour d'entraînement) :

- Demande de l'utilisateur : un camp d'entraînement où les monstres gagnent de l'XP en attendant, comme la pension de Pokémon, avec plus tard un système de reproduction.
- Claude a relevé que la cour d'entraînement de Gunther le fait déjà (section 13.6), mais qu'elle n'est accessible que par les machines de la forge.
- Choix de l'utilisateur : la rendre plus visible, sans changer ses règles ; la reproduction reste à concevoir.
- Hypothèses retenues, à confirmer : Q104.

25/10/2026 (conception des étapes 110 à 112, reproduction) :

- Demande de l'utilisateur : ajouter la reproduction des monstres.
- Choix de l'utilisateur : un rituel de Malvoise, le Rite du sang mêlé, qui dure 2 heures réelles pendant lesquelles les parents restent auprès de lui ; deux parents de la même famille d'évolution ; un petit de la forme de base, au niveau 1, qui reçoit la meilleure valeur interne de ses parents pour chaque stat ; un coût de 3 ressources de l'espèce du petit et de 50 pièces d'argent (section 13.11).
- Sauvegarde en version 28.
- Hypothèses retenues, à confirmer : Q105.

25/10/2026 (étape 113, messages privés) :

- Demande de l'utilisateur : prévenir systématiquement par message privé quand quelque chose est terminé.
- Interprétation retenue et signalée par Claude : en plus des expéditions automatiques, des travaux de Gunther et de la ligne d'Ansgar, déjà annoncés, le Rite du sang mêlé achevé, les pièges pleins et un monstre de la cour à son plafond ; la Q95 est remplacée pour les pièges (section 2).

25/10/2026 (étape 114, shiny et attaques d'œuf du rite) :

- Demande de l'utilisateur : un parent shiny change les chances du petit, deux le garantissent ; une attaque d'œuf par espèce, de son type, obtenue seulement par reproduction, intéressante et assez puissante sans être abusive.
- Choix de l'utilisateur : 1 chance sur 256 avec un parent shiny ; une attaque nouvelle par famille ; 1 chance sur 4, à coup sûr si un parent la connaît ; le tableau des attaques proposé par Claude (section 13.11, docs/contenu-v1.md, section 28).
- Hypothèses retenues, à confirmer : Q105.

25/10/2026 (étape 115, récompenses du codex) :

- Demande de l'utilisateur : récompenser la complétion du codex, parmi les idées proposées par Claude.
- Choix de l'utilisateur : une récompense toutes les 10 espèces capturées et au codex complet, à réclamer depuis le codex ; des objets rares et un talisman unique, le Sceau du bestiaire, selon la répartition proposée par Claude ; seules les espèces capturées comptent (section 5.6).
- Sauvegarde en version 29.
- Hypothèses retenues, à confirmer : Q106.

25/10/2026 (étape 116, équilibrage) :

- Demande de l'utilisateur : réaliser l'équilibrage proposé par Claude.
- Fait : l'outil d'équilibrage et son joueur automatique (section 2), mesurés sur 100 parties par classe, puis des variantes des valeurs qui posent problème (docs/equilibrage.md).
- Constats : le Baron pendu est le mur de la partie, plus dur que les deux Apôtres suivants, surtout pour le Guerrier et le Mage ; le Bourreau est l'humain le plus dur ; le Templier est la classe la plus lente, à cause du Marais ; l'équipe de 4 et les objets des paliers 2 et 3 n'arrivent pas avant le Margrave, faute de ressources rares ; l'argent ne bloque pas.
- Aucune valeur du jeu n'est changée sans l'accord de l'utilisateur : Q107.
25/10/2026 (étape 117, réglages issus de l'équilibrage) :

- Choix de l'utilisateur, parmi les réglages proposés par Claude et mesurés par l'outil d'équilibrage : le Baron pendu au niveau 20, le maximum de la Vallée (section 9.4) ; les monstres du Bourreau aux niveaux 13, 13 et 14 (section 10.7) ; Secousse au niveau 7 de la famille de la Mandragore, à la place du Cri strident (section 5.3) ; les ressources rares inchangées, lentes à réunir.
- Claude a relevé qu'au niveau 20, le Baron perd Hurlement des abysses, appris au niveau 21, et que l'essentiel du gain mesuré en vient : s'il gardait ses deux attaques Obscur, il demanderait encore 3,8 tentatives. Choix de l'utilisateur : une seule attaque Obscur.
- Mesuré après les réglages : 54 à 61 expéditions selon la classe, au lieu de 55 à 64 ; 2,6, 2,5 et 2,0 tentatives pour les trois Apôtres (docs/equilibrage.md).
- Hypothèses retenues, à confirmer : Q108.
25/10/2026 (conception des étapes 118 à 120, cycle du jour et de la nuit) :

- Demande de l'utilisateur : ajouter le cycle jour/nuit.
- Choix de l'utilisateur : un cycle accéléré en temps réel, 2 heures de jour puis 1 heure de nuit ; une expédition garde le moment de son départ ; le cycle ne joue que dans les zones, sans les combats de fin de zone ; de nuit, des monstres sauvages de 2 niveaux de plus, qui rapportent 25 % d'XP et d'argent des coffres en plus ; un bonus de 20 % aux attaques Ténèbres, Spectre et Poison de nuit, Lumière, Feu et Plante de jour ; des espèces exclusivement nocturnes ou diurnes, selon la liste ajustée par Claude ; trois évolutions selon le moment ; l'affichage au camp, sur la carte et en expédition (section 9.6).
- Claude a relevé qu'avec des espèces exclusives, sa première liste laissait une seule espèce de jour dans la Vallée et l'Abbaye, et de nuit dans la Lisière ; la liste ajustée en garde au moins deux par zone et par moment.
- Hypothèses retenues, à confirmer : Q109.
25/10/2026 (conception des étapes 121 à 123, météo) :

- Demande de l'utilisateur : une météo, pluie, tempête, grêle, tempête de sable, cyclone et canicule, en événements aléatoires de 30 minutes, certaines exclusives à des zones (cyclone en mer, tempête de sable au désert), qui renforcent des types et se cumulent au cycle du jour.
- Claude a relevé que le jeu n'a ni zone de mer ni désert. Choix de l'utilisateur : le cyclone et la tempête de sable sont réservés à ces zones futures.
- Choix de l'utilisateur : une météo par zone, la même pour tous les joueurs, une chance sur 3 par créneau de 30 minutes ; fixée au départ de l'expédition ; deux types renforcés de 20 % par météo, selon la répartition proposée par Claude, en se multipliant au bonus du moment ; les météos de chaque zone proposées par Claude ; la météo joue aussi contre les humains et les Apôtres de fin de zone (section 9.7).
- Hypothèses retenues, à confirmer : Q110.
25/10/2026 (conception de l'étape 124, évolutions selon la météo, et des étapes 125 à 127, capacité du sac) :

- Demande de l'utilisateur : des évolutions selon la météo. Choix de l'utilisateur, parmi celles proposées par Claude : la Fée noire sous la tempête, la Fée-sorcière sous la pluie, le Basilic sous la canicule (section 9.7).
- Demande de l'utilisateur : une limite au sac, améliorable avec des ressources. Choix de l'utilisateur : chaque objet prend une place, sauf l'argent ; 30 places, puis 45, 60 et 80 avec l'amélioration de Gunther, aux coûts proposés par Claude ; le joueur choisit quoi jeter à la fin de l'étape où le sac déborde ; au camp, une sortie du coffre qui ferait déborder le sac est refusée ; le départ est refusé tant que le sac déborde (sections 13.3 et 13.4).
- Claude a relevé que les achats chez Malvoise vont au coffre : seule la sortie du coffre est limitée au camp.
- Hypothèses retenues, à confirmer : Q111.

