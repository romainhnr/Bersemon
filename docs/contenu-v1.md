# Contenu de la V1 - Bersemon

Statut : validé le 17/10/2026. Les règles qu'il applique sont celles de docs/gamedesign.md. Toute modification de ce contenu se reporte ici avant d'être codée.

Contraintes que le code impose au contenu :

- chaque espèce apprend au moins une attaque au niveau 1, pour pouvoir être créée à tout niveau ;
- une espèce n'apprend jamais deux fois la même attaque ;
- un monstre créé à un niveau donné connaît les 4 dernières attaques de sa liste jusqu'à ce niveau (section 7).

Repères de puissance : avec les formules de la V1, deux monstres de niveau 5 aux stats de base 50 s'infligent 4 à 5 dégâts sur 20 PV avec une attaque de puissance 40. Les stats de base des espèces communes vont de 285 à 330 au total, celles des rares de 400, et celles du Schnoz, monstre caché ajouté à l'étape 27, de 450.

## 1. Attaques

Catégorie : P pour physique, S pour spéciale. L'effet est la chance d'infliger un statut à la cible touchée (section 10.5).

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| souffle-chaud | Souffle chaud | Normal | S | 40 | 100 | |
| crachat-de-braises | Crachat de braises | Feu | S | 40 | 100 | |
| souffle-draconique | Souffle draconique | Dragon | S | 60 | 100 | |
| flamme-vive | Flamme vive | Feu | S | 70 | 90 | |
| fournaise | Fournaise | Feu | S | 90 | 85 | |
| charge-gluante | Charge gluante | Normal | P | 40 | 100 | |
| plongeon | Plongeon | Eau | P | 40 | 100 | |
| coup-de-langue | Coup de langue | Normal | P | 55 | 95 | paralysie 10 % |
| langue-venimeuse | Langue venimeuse | Poison | P | 50 | 100 | poison 30 % |
| lame-de-fond | Lame de fond | Eau | P | 80 | 85 | |
| coup-de-racine | Coup de racine | Normal | P | 40 | 100 | |
| feuille-tranchante | Feuille tranchante | Plante | S | 40 | 100 | |
| cri-strident | Cri strident | Normal | S | 45 | 100 | paralysie 20 % |
| ronces | Ronces | Plante | P | 65 | 95 | |
| spores-putrides | Spores putrides | Poison | S | 55 | 100 | poison 30 % |
| griffure | Griffure | Normal | P | 40 | 100 | |
| morsure | Morsure | Ténèbres | P | 60 | 90 | |
| curee | Curée | Normal | P | 75 | 85 | |
| coup-de-bec | Coup de bec | Vol | P | 35 | 100 | |
| pique | Piqué | Vol | P | 60 | 90 | |
| regard-funeste | Regard funeste | Ténèbres | S | 50 | 100 | paralysie 10 % |
| morsure-venimeuse | Morsure venimeuse | Poison | P | 40 | 100 | poison 30 % |
| regard-petrifiant | Regard pétrifiant | Normal | S | 30 | 100 | paralysie 30 % |
| etreinte | Étreinte | Normal | P | 60 | 95 | |
| lueur-trompeuse | Lueur trompeuse | Spectre | S | 40 | 100 | paralysie 10 % |
| frisson-d-outre-tombe | Frisson d'outre-tombe | Spectre | S | 65 | 100 | |
| coup-de-garde | Coup de garde | Acier | P | 40 | 100 | |
| lame-rouillee | Lame rouillée | Acier | P | 65 | 95 | poison 20 % |
| charge-spectrale | Charge spectrale | Spectre | P | 70 | 90 | |
| coup-de-bois | Coup de bois | Normal | P | 40 | 100 | |
| ruade | Ruade | Normal | P | 60 | 95 | |
| lumiere-d-aube | Lumière d'aube | Féérique | S | 65 | 100 | |
| souffle-putride | Souffle putride | Dragon | S | 70 | 95 | |
| croc-de-venin | Croc de venin | Poison | P | 75 | 90 | poison 30 % |
| corde-du-gibet | Corde du gibet | Spectre | P | 60 | 100 | paralysie 20 % |
| lamentation | Lamentation | Spectre | S | 70 | 100 | paralysie 10 % |
| sentence | Sentence | Ténèbres | S | 80 | 95 | |
| trappe-du-gibet | Trappe du gibet | Ténèbres | P | 90 | 85 | |
| brasier | Brasier | Feu | S | 70 | 100 | |
| deferlante | Déferlante | Eau | P | 65 | 100 | |
| lianes-etrangleuses | Lianes étrangleuses | Plante | P | 55 | 100 | paralysie 20 % |
| coup-sournois | Coup sournois | Ténèbres | P | 40 | 100 | |
| feulement | Feulement | Normal | S | 40 | 100 | paralysie 20 % |
| coup-de-grace | Coup de grâce | Ténèbres | P | 85 | 90 | |
| eclat-astral | Éclat astral | Stellaire | S | 40 | 100 | |
| rayon-astral | Rayon astral | Stellaire | S | 75 | 100 | |

Brasier, Déferlante et Lianes étrangleuses sont ajoutées après la V1, le 18/10/2026, pour les capsules d'attaque (section 6) : aucune espèce ne les apprend.

Les cinq dernières attaques sont ajoutées à l'étape 27, le 19/10/2026 : les trois premières pour le Chat noir, les deux autres pour le Schnoz. Le type Stellaire est neutre envers tous les types en attendant le type Obscur (gamedesign.md, section 5.2).

## 2. Espèces

Stats de base dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse. Le rôle n'a pas d'effet dans la V1 (section 5.1).

Rareté, ajoutée après la V1 le 18/10/2026 pour la revente au marchand (section 6) : les monstres de départ et les monstres communs sont d'espèce commune, les monstres rares et le Schnoz d'espèce rare, et le Baron pendu est un Apôtre.

### Monstres de départ

Trois profils distincts : la Salamandre frappe en attaque spéciale, le Crapaud en attaque physique, la Mandragore encaisse. Depuis le 18/10/2026, chacun est attitré à une classe : Salamandre au Mage, Crapaud au Guerrier, Mandragore au Templier (section 6). Le Chat noir, ajouté à l'étape 27 le 19/10/2026, est attitré à l'Assassin : il frappe vite et fort en attaque physique, mais encaisse mal.

| Identifiant | Nom | Types | Rôle | Profil | Stats de base (total) | Rendement d'XP |
|---|---|---|---|---|---|---|
| salamandre | Salamandre | Feu | offensif | offensif spécial | 45 / 40 / 40 / 70 / 55 / 65 (315) | 62 |
| crapaud | Crapaud | Eau | offensif | offensif physique | 55 / 68 / 55 / 35 / 45 / 55 (313) | 63 |
| mandragore | Mandragore | Plante | soutien | défensif | 70 / 45 / 70 / 45 / 65 / 25 (320) | 64 |
| chat-noir | Chat noir | Ténèbres | offensif | rapide et fragile, offensif physique | 40 / 72 / 35 / 40 / 38 / 90 (315) | 62 |

Attaques apprises :

- Salamandre, uniquement des attaques spéciales : Souffle chaud (niv. 1), Crachat de braises (niv. 3), Souffle draconique (niv. 7), Flamme vive (niv. 10), Fournaise (niv. 14).
- Crapaud, uniquement des attaques physiques : Charge gluante (niv. 1), Plongeon (niv. 3), Coup de langue (niv. 7), Langue venimeuse (niv. 10), Lame de fond (niv. 14).
- Mandragore, des attaques des deux catégories et deux effets de statut, pour user l'adversaire : Coup de racine (niv. 1), Feuille tranchante (niv. 3), Cri strident (niv. 7), Ronces (niv. 10), Spores putrides (niv. 14).
- Chat noir, surtout des attaques physiques : Griffure (niv. 1), Coup sournois (niv. 3), Feulement (niv. 7), Morsure (niv. 10), Coup de grâce (niv. 14). Griffure et Morsure sont aussi celles du Loup.

Au niveau 5, chaque monstre de départ connaît 2 attaques : une de type Normal et une de son type. La Mandragore est le départ le plus exigeant (section 5), ce qui est assumé.

### Monstres communs

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP |
|---|---|---|---|---|---|
| loup | Loup | Normal | offensif | 50 / 62 / 45 / 30 / 40 / 58 (285) | 70 |
| corbeau | Corbeau | Vol, Ténèbres | offensif | 42 / 57 / 38 / 45 / 38 / 72 (292) | 66 |
| vipere | Vipère | Poison | offensif | 40 / 50 / 40 / 50 / 45 / 63 (288) | 68 |
| feu-follet | Feu follet | Feu, Spectre | offensif | 45 / 30 / 40 / 70 / 60 / 75 (320) | 85 |
| armure-hantee | Armure hantée | Acier, Spectre | offensif | 60 / 75 / 85 / 30 / 50 / 30 (330) | 95 |

Attaques apprises :

- Loup : Griffure (niv. 1), Morsure (niv. 4), Curée (niv. 9).
- Corbeau : Coup de bec (niv. 1), Piqué (niv. 4), Regard funeste (niv. 8).
- Vipère : Morsure venimeuse (niv. 1), Regard pétrifiant (niv. 3), Étreinte (niv. 9).
- Feu follet : Lueur trompeuse (niv. 1), Crachat de braises (niv. 1), Flamme vive (niv. 10), Frisson d'outre-tombe (niv. 12).
- Armure hantée : Coup de garde (niv. 1), Lame rouillée (niv. 8), Charge spectrale (niv. 11).

### Monstres rares

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Comportement |
|---|---|---|---|---|---|---|
| cerf-blanc | Cerf blanc | Normal, Féérique | soutien | 70 / 60 / 60 / 65 / 70 / 75 (400) | 150 | pacifique : se laisse prendre |
| basilic | Basilic | Poison, Dragon | offensif | 75 / 70 / 65 / 75 / 60 / 55 (400) | 170 | hostile : attaque |

Attaques apprises :

- Cerf blanc : Coup de bois (niv. 1), Ruade (niv. 5), Lumière d'aube (niv. 8).
- Basilic : Morsure venimeuse (niv. 1), Regard pétrifiant (niv. 8), Souffle putride (niv. 10), Croc de venin (niv. 13).

### Apôtre

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Niveau |
|---|---|---|---|---|---|---|
| baron-pendu | Baron pendu | Ténèbres, Spectre | offensif | 85 / 85 / 75 / 90 / 80 / 65 (480) | 250 | 15, soit 2 de plus que le maximum du Marais |

Attaques apprises : Corde du gibet (niv. 1), Lamentation (niv. 5), Sentence (niv. 10), Trappe du gibet (niv. 15). Au niveau 15, il les connaît toutes les quatre. Ses valeurs internes sont fixes, à 31 pour chaque stat (section 9.4).

Autrefois seigneur des terres du Marais, il a livré son propre fief au gibet pour devenir un Apôtre. Ses types le rendent insensible aux attaques Normal et Combat ; seul le type Féérique lui est très efficace.

### Monstre caché

Ajouté à l'étape 27, le 19/10/2026 (gamedesign.md, sections 5.3 et 9.2).

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Comportement |
|---|---|---|---|---|---|---|
| schnoz | Schnoz | Stellaire | soutien | 60 / 50 / 60 / 90 / 90 / 100 (450) | 200 | pacifique : se laisse prendre |

Attaques apprises : Éclat astral (niv. 1), Lueur trompeuse (niv. 1), Rayon astral (niv. 10).

Seul monstre de type Stellaire. Dans toute zone, quand une étape tire un monstre rare, c'est le Schnoz une fois sur 20, au niveau de la zone ; il peut se rencontrer de nouveau.

## 3. Zones

| Identifiant | Nom | Niveaux | Taille des groupes | Espèces | Monstre rare | Apôtre |
|---|---|---|---|---|---|---|
| lisiere | Lisière du Bois-Sombre | 3 à 7 | 1 à 2 | Loup, Corbeau, Vipère | Cerf blanc (pacifique) | aucun |
| marais | Marais des Pendus | 8 à 13 | 1 à 3 | Feu follet, Armure hantée, Vipère, Corbeau | Basilic (hostile) | Baron pendu |

Le Schnoz, monstre caché, se rencontre dans les deux zones, à la place d'un monstre rare sur 20 (section 2).

La Lisière est accessible dès le départ. Le Marais s'ouvre avec le premier indice, au bout de la première expédition complète dans la Lisière ; deux expéditions complètes dans le Marais donnent les indices 2 et 3 ; l'expédition suivante dans le Marais s'achève par le Baron pendu (section 9.4).

Nombre d'étapes d'une expédition manuelle, fixe depuis l'étape 26 (décidé le 18/10/2026, gamedesign.md, section 9.2) : 3 dans la Lisière, 4 dans le Marais. Dans la V1, il était tiré entre 4 et 6.

Expéditions automatiques, ajoutées après la V1 le 18/10/2026 (section 12) :

| Identifiant | Durée de base, pour un monstre | Dégâts | XP par monstre non KO | Potions | Argent |
|---|---|---|---|---|---|
| lisiere | 1 h | 15 à 60 | 100 à 200 | 0 à 1 | 15 à 30 |
| marais | 2 h | 40 à 120 | 300 à 600 | 1 à 2 | 40 à 80 |

L'argent est ajouté le 18/10/2026, avec l'économie (section 6).

Coffres des expéditions manuelles, après la V1 (section 6) :

| Identifiant | Potions | Argent | Capsules d'attaque |
|---|---|---|---|
| lisiere | 1 à 2 | 10 à 25 | Brasier, Déferlante, Lianes étrangleuses |
| marais | 1 à 2 | 25 à 50 | Croc de venin, Frisson d'outre-tombe, Sentence |

## 4. Textes

### Introduction

Affichée après le choix de la classe (section 6). Dans la V1, elle précédait le choix du premier monstre et finissait par : "Le gardien du camp soulève le drap qui couvre trois cages. Choisissez la créature qui partagera votre route." Depuis le 18/10/2026, chaque classe a son monstre attitré, et le dernier paragraphe dépend de la classe (Q67).

> Le royaume s'est éteint sous un ciel de cendre. Sur les routes que les hommes ont désertées, les bêtes ont repris leurs droits, et des choses plus anciennes rôdent au bord des bois.
>
> Ces choses ont un nom : les Apôtres. Autrefois des hommes, ils ont sacrifié ce qu'ils aimaient pour une chair de monstre, et ils servent un maître que nul n'a vu en face, le Démiurge. Une nuit, ses Apôtres ont fondu sur les vôtres. Vous êtes le seul à vous être relevé.
>
> Vous atteignez le camp de base, dernier feu allumé avant les terres sauvages. Ceux qui survivent ici ne le doivent pas à leur seule lame : ils se lient aux créatures, les capturent au filet et les mènent au combat. Pour atteindre le Démiurge, il faudra abattre ses Apôtres, un par un.

Dernier paragraphe pour le Guerrier :

> Le gardien du camp jauge votre lame, puis ouvre une cage : un crapaud massif s'en extrait, la peau couverte de cicatrices. Il frappe fort et ne recule pas. Il partagera votre route.

Pour le Mage :

> Le gardien du camp remarque la cendre sur vos doigts, puis ouvre une cage : une salamandre s'en glisse, et l'air tremble autour d'elle. Ses flammes répondront aux vôtres. Elle partagera votre route.

Pour le Templier :

> Le gardien du camp observe votre bouclier, puis ouvre une cage : une mandragore en sort à pas lents, ses racines tressées comme une armure. Elle encaisse ce qui en abattrait d'autres. Elle partagera votre route.

Pour l'Assassin, depuis l'étape 27 :

> Le gardien du camp remarque la lame cachée dans votre manche, puis ouvre une cage : un chat noir s'en glisse sans un bruit, les yeux luisants. Il frappe avant qu'on le voie, mais ne supporte pas les coups. Il partagera votre route.

### Indices sur l'Apôtre

Le premier indice s'affiche au retour de la première expédition menée au bout dans la Lisière, les deux suivants au retour des expéditions menées au bout dans le Marais :

> Indice 1 : au bout de la Lisière, des cordes neuves pendent encore à un chêne mort. Les traces d'une procession s'enfoncent vers le sud, là où la forêt se noie : le Marais des Pendus. Quelque chose y règne.

> Indice 2 : des noyés flottent entre les roseaux, tous tournés vers le même point de la brume, comme s'ils écoutaient un appel.

> Indice 3 : un gibet se dresse sur une île de vase, plus haut que les arbres. À son pied, la terre est retournée par des pas trop lourds pour un homme. L'antre de l'Apôtre est là.

### Combat contre l'Apôtre

Avant le combat :

> Sous le gibet, une silhouette se déplie lentement, la corde encore au cou. Le Baron pendu, Apôtre du Démiurge, vous a senti venir. Il n'y aura pas de fuite.

Épilogue, après la victoire :

> Le Baron pendu s'effondre dans la vase. Sur sa poitrine, un sceau gravé d'une main ouverte : le signe du Démiurge. D'autres Apôtres portent ce sceau, au-delà des terres connues. Votre traque ne fait que commencer.

### Combat contre le brigand

Ajouté à l'étape 29, le 19/10/2026 (gamedesign.md, section 10.7). Le brigand attend au bout de la Lisière, avec un Loup de niveau 7 et un Corbeau de niveau 8 (valeurs internes à 15, première habileté de leur espèce). Victoire : 60 pièces d'argent.

Avant le combat :

> Au sortir de la Lisière, un brigand vous barre la route, un loup et un corbeau à ses côtés. Ses bêtes ont faim, et lui aussi.

Après la victoire :

> Le brigand détale dans les fourrés, abandonnant ses bêtes et sa bourse.

### Rencontre du marchand

Ajoutée après la V1, le 18/10/2026 : affichée à l'étape où le marchand est rencontré, avant qu'il se réfugie au camp (gamedesign.md, section 14.2).

> Un marchand ambulant surgit de la brume, sa carriole chargée de fioles. Les routes ne sont plus sûres pour lui : il part se réfugier au camp, où vous le retrouverez.

### Conseils du tutoriel

Chaque conseil n'apparaît qu'une fois par partie. Pour s'en souvenir d'une session à l'autre, la sauvegarde gardera la liste des conseils déjà montrés.

Au début du premier combat :

> Conseil : affaiblissez un monstre avant de lancer un filet, car plus il a perdu de PV, plus la capture a de chances de réussir. Fuir réussit toujours, mais le monstre adverse frappe une dernière fois.

Au premier arrêt entre deux étapes. Depuis l'étape 26 et le coffre du camp, le 19/10/2026, la fin du texte est « une défaite vous fait perdre tout votre sac, mais pas ce qui est rangé dans le coffre du camp » ; elle était « une défaite vous fait perdre toutes vos potions ».

> Conseil : vos monstres ne récupèrent leurs PV qu'au camp. Entre deux étapes, soignez-les avec une potion, changez l'ordre de votre équipe, ou rentrez avant qu'il ne soit trop tard : une défaite vous fait perdre tout votre sac, mais pas ce qui est rangé dans le coffre du camp.

Au premier retour au camp :

> Conseil : au camp, vos monstres sont soignés et la partie est sauvegardée. Trois monstres vous accompagnent ; les autres attendent ici, et vous pouvez les échanger à tout moment.

### Suicide du héros

Ajouté après la V1, le 19/10/2026 (gamedesign.md, section 13.9). Question de confirmation, avec le nom du héros :

> Vous suicider ? Votre partie sera effacée pour de bon : Aldric, vos monstres, votre sac, votre coffre, votre codex et vos expéditions automatiques en cours.

Après la confirmation, avant la création d'un nouveau héros :

> Vous retournez votre lame contre vous. Le feu du camp s'éteint, vos monstres se dispersent dans la nuit, et votre nom s'efface des routes. Ailleurs, une autre âme se relève parmi les cendres.

## 5. Points d'équilibrage

- La Mandragore est le départ le plus exigeant, ce qui est assumé : dans la Lisière, le Corbeau (Vol) et la Vipère (Poison) sont très efficaces contre elle, et la Vipère résiste à ses attaques Plante ; dans le Marais, l'Armure hantée (Acier) résiste aussi au type Plante. Son profil défensif et ses 320 de stats de base, 5 de plus que les deux autres, l'aident à tenir.
- Le type Spectre du Feu follet, de l'Armure hantée et du Baron pendu annule les attaques de type Normal : dans le Marais, chaque monstre de départ doit compter sur l'attaque de son type, apprise au niveau 3.
- Le Basilic, rare et hostile dans le Marais, est nettement plus fort que les monstres communs (400 de stats de base) : c'est un combat redoutable, qui rapporte beaucoup d'XP.
- Le Baron pendu est le combat le plus dur de la V1. Seul le type Féérique lui est très efficace : la Lumière d'aube du Cerf blanc, monstre rare pacifique de la Lisière, en fait une prise précieuse pour l'affronter.
- Rythme estimé : une expédition dans la Lisière rapporte de l'ordre de 200 à 400 XP par participant, soit le passage du niveau 5 au niveau 8 environ en trois expéditions. Le Marais devient abordable vers le niveau 9, le Baron pendu vers le niveau 13 ou 14. À vérifier en jouant.
- Le Chat noir, de type Ténèbres, résiste aux attaques Spectre et Ténèbres : contre les Spectre du Marais et le Baron pendu, l'Assassin a le départ le plus facile, ce qui est accepté jusqu'à l'Apôtre de l'étape 30 (gamedesign.md, Q76). En contrepartie, il a le moins de PV et de défenses des monstres de départ.
- Le Schnoz, très rare, est un cadeau : 450 de stats de base, sans faiblesse ni résistance tant que le type Stellaire est neutre.
- Place du combat contre l'Apôtre (Q57) : à la dernière étape de l'expédition qui suit le 3e indice, à la place de l'événement tiré ; rentrer avant cette étape le repousse à l'expédition suivante.

## 6. Économie

Ajoutée après la V1, le 18/10/2026 (gamedesign.md, sections 13.10 et 14). Les montants se comptent en pièces d'argent.

Revente d'un monstre au marchand, depuis l'étape 26 : son niveau multiplié par le total des stats de base de son espèce, divisé par 50 en division entière, puis multiplié par le coefficient de rareté de l'espèce, et doublé pour un shiny. Avant l'étape 26, c'était son niveau multiplié par 5, 15 ou 40 selon la rareté.

| Rareté | Espèces | Coefficient | Exemple |
|---|---|---|---|
| commune | monstres de départ et monstres communs | 1 | Loup de niveau 5 (285 de stats de base) : 28 |
| rare | Cerf blanc, Basilic, Schnoz | 2 | Cerf blanc de niveau 5 (400) : 80 ; Schnoz de niveau 10 (450) : 180 |
| Apôtre | Baron pendu | 4 | Baron pendu de niveau 15 (480) : 576 |

Prix du marchand : une potion coûte 20. Il vend toutes les capsules d'attaque.

Capsules d'attaque : chacune enseigne son attaque à tout monstre qui a le type de cette attaque. Un coffre en contient une fois sur 5, tirée parmi les capsules de sa zone que le joueur n'a pas encore.

| Capsule | Type | Utile à | Zone des coffres | Prix |
|---|---|---|---|---|
| Brasier | Feu | Salamandre, Feu follet | lisiere | 150 |
| Déferlante | Eau | Crapaud | lisiere | 150 |
| Lianes étrangleuses | Plante | Mandragore | lisiere | 150 |
| Croc de venin | Poison | Vipère | marais | 200 |
| Frisson d'outre-tombe | Spectre | Armure hantée, Baron pendu | marais | 200 |
| Sentence | Ténèbres | Corbeau, Chat noir | marais | 300 |

Les capsules de la Lisière servent aux monstres de départ, dont l'espèce apprend déjà toutes les attaques de leur type qui existaient dans la V1 ; celles du Marais donnent aux monstres communs une attaque d'une autre espèce de leur type.

## 7. Habiletés

Ajoutées à l'étape 28, le 19/10/2026 (gamedesign.md, section 5.9). Chaque espèce a 2 habiletés possibles ; un monstre tire la sienne à sa création. La première de la liste est donnée aux monstres des sauvegardes antérieures. La répartition suit le profil de chaque espèce ; elle est à ajuster en jouant.

| Espèce | Habiletés possibles |
|---|---|
| Salamandre | Affinité, Sorcellerie |
| Crapaud | Férocité, Carapace |
| Mandragore | Carapace, Volonté |
| Chat noir | Célérité, Traqueur |
| Loup | Traqueur, Fouineur |
| Corbeau | Pillard, Éclaireur |
| Vipère | Célérité, Instinct |
| Feu follet | Éclaireur, Sorcellerie |
| Armure hantée | Carapace, Férocité |
| Cerf blanc | Volonté, Instinct |
| Basilic | Affinité, Férocité |
| Baron pendu | Sorcellerie, Volonté |
| Schnoz | Pillard, Fouineur |

Valeurs : +10 % pour les habiletés de stats (Férocité, Carapace, Sorcellerie, Volonté, Célérité), +20 % de dégâts pour Affinité, +20 % d'XP pour Instinct, -20 % de durée pour Éclaireur, capture x1,25 pour Traqueur, +50 % d'argent pour Pillard, une potion 1 fois sur 4 pour Fouineur.
