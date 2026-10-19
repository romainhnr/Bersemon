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
| bourdonnement | Bourdonnement | Insecte | S | 40 | 100 | |
| malefice | Maléfice | Ténèbres | S | 65 | 100 | |
| poussiere-d-elfe | Poussière d'elfe | Féérique | S | 40 | 100 | |
| bourrasque | Bourrasque | Vol | S | 55 | 100 | |
| frappe-d-os | Frappe d'os | Combat | P | 40 | 100 | |
| furie-draconique | Furie draconique | Dragon | P | 85 | 90 | |
| etreinte-obscure | Étreinte obscure | Obscur | P | 80 | 95 | |
| hurlement-des-abysses | Hurlement des abysses | Obscur | S | 80 | 100 | paralysie 10 % |
| jet-de-lave | Jet de lave | Feu | S | 85 | 90 | |
| trombe | Trombe | Eau | P | 85 | 90 | |
| hurlement-de-mandragore | Hurlement de mandragore | Plante | S | 75 | 100 | paralysie 20 % |
| griffe-d-ombre | Griffe d'ombre | Ténèbres | P | 80 | 95 | |
| croc-noir | Croc noir | Ténèbres | P | 75 | 95 | |
| bec-funeste | Bec funeste | Vol | P | 75 | 95 | |
| venin-noir | Venin noir | Poison | S | 75 | 100 | poison 30 % |
| nuee | Nuée | Insecte | S | 75 | 100 | |
| flamme-funebre | Flamme funèbre | Spectre | S | 80 | 100 | |
| rayon-de-lune | Rayon de lune | Féérique | S | 85 | 100 | |
| taille-d-acier | Taille d'acier | Acier | P | 85 | 90 | |
| epee-des-morts | Épée des morts | Combat | P | 85 | 95 | |
| regard-de-mort | Regard de mort | Poison | S | 85 | 90 | |

Brasier, Déferlante et Lianes étrangleuses sont ajoutées après la V1, le 18/10/2026, pour les capsules d'attaque (section 6) : aucune espèce ne les apprend.

Coup sournois, Feulement, Coup de grâce, Éclat astral et Rayon astral sont ajoutées à l'étape 27, le 19/10/2026 : les trois premières pour le Chat noir, les deux autres pour le Schnoz. Le type Stellaire est neutre envers tous les types en attendant le type Obscur (gamedesign.md, section 5.2).

Les huit attaques de Maléfice à Hurlement des abysses sont ajoutées à l'étape 30, le 19/10/2026 : six pour les monstres de la Vallée des Brumes, deux, de type Obscur, pour le Baron pendu. Maléfice, Bourrasque et Furie draconique sont aussi les capsules de la Vallée (section 6).

Les treize dernières attaques, de Jet de lave à Regard de mort, sont ajoutées à l'étape 37, le 19/10/2026 : ce sont les attaques d'évolution (section 8).

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

Ajoutés à l'étape 30, le 19/10/2026, pour la Vallée des Brumes :

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP |
|---|---|---|---|---|---|
| fee-noire | Fée noire | Insecte, Ténèbres | offensif | 40 / 35 / 40 / 65 / 50 / 75 (305) | 72 |
| squelette | Squelette | Spectre, Combat | offensif | 50 / 80 / 60 / 30 / 45 / 55 (320) | 92 |

Attaques apprises :

- Loup : Griffure (niv. 1), Morsure (niv. 4), Curée (niv. 9).
- Corbeau : Coup de bec (niv. 1), Piqué (niv. 4), Regard funeste (niv. 8).
- Vipère : Morsure venimeuse (niv. 1), Regard pétrifiant (niv. 3), Étreinte (niv. 9).
- Feu follet : Lueur trompeuse (niv. 1), Crachat de braises (niv. 1), Flamme vive (niv. 10), Frisson d'outre-tombe (niv. 12).
- Armure hantée : Coup de garde (niv. 1), Lame rouillée (niv. 8), Charge spectrale (niv. 11).
- Fée noire : Bourdonnement (niv. 1), Regard funeste (niv. 4), Maléfice (niv. 8).
- Squelette : Frappe d'os (niv. 1), Lame rouillée (niv. 8), Charge spectrale (niv. 11).

### Monstres rares

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Comportement |
|---|---|---|---|---|---|---|
| cerf-blanc | Cerf blanc | Normal, Féérique | soutien | 70 / 60 / 60 / 65 / 70 / 75 (400) | 150 | pacifique : se laisse prendre |
| basilic | Basilic | Poison, Dragon | offensif | 75 / 70 / 65 / 75 / 60 / 55 (400) | 170 | hostile : attaque |
| elfe | Elfe | Féérique, Vol | soutien | 55 / 40 / 50 / 85 / 80 / 90 (400) | 150 | pacifique : se laisse prendre |
| vouivre | Vouivre | Dragon, Vol | offensif | 70 / 80 / 60 / 80 / 55 / 55 (400) | 170 | hostile : attaque |

Attaques apprises :

- Cerf blanc : Coup de bois (niv. 1), Ruade (niv. 5), Lumière d'aube (niv. 8).
- Basilic : Morsure venimeuse (niv. 1), Regard pétrifiant (niv. 8), Souffle putride (niv. 10), Croc de venin (niv. 13).
- Elfe, ajouté à l'étape 30 : Poussière d'elfe (niv. 1), Bourrasque (niv. 5), Lumière d'aube (niv. 9).
- Vouivre, ajoutée à l'étape 30 : Souffle draconique (niv. 1), Piqué (niv. 8), Furie draconique (niv. 12).

### Apôtre

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Niveau |
|---|---|---|---|---|---|---|
| baron-pendu | Baron pendu | Obscur | offensif | 85 / 85 / 75 / 90 / 80 / 65 (480) | 250 | 22, soit 2 de plus que le maximum de la Vallée des Brumes |

Attaques apprises : Corde du gibet (niv. 1), Lamentation (niv. 5), Sentence (niv. 10), Trappe du gibet (niv. 15), Étreinte obscure (niv. 18), Hurlement des abysses (niv. 21). Au niveau 22, il connaît les quatre dernières. Ses valeurs internes sont fixes, à 31 pour chaque stat (section 9.4).

Autrefois seigneur des terres du Marais, il a livré son propre fief au gibet pour devenir un Apôtre ; il s'est retiré au fond de la Vallée des Brumes. Depuis l'étape 30, il est de type Obscur seul : il résiste à la plupart des types, et seuls Féérique, Psychique, Stellaire et Obscur lui sont très efficaces. Jusqu'à l'étape 29, il était Ténèbres et Spectre, au niveau 15, au bout du Marais, et ne connaissait que ses quatre premières attaques.

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
| marais | Marais des Pendus | 8 à 13 | 1 à 3 | Feu follet, Armure hantée, Vipère, Corbeau | Basilic (hostile) | aucun depuis l'étape 30 |
| vallee | Vallée des Brumes | 14 à 20 | 1 à 3 | Fée noire, Squelette, Armure hantée | Elfe (pacifique), Vouivre (hostile) | Baron pendu |

Le Schnoz, monstre caché, se rencontre dans toutes les zones, à la place d'un monstre rare sur 20 (section 2).

La Lisière est accessible dès le départ. Depuis l'étape 30 (gamedesign.md, section 9.4) : le brigand attend au bout de la Lisière, et la première expédition menée au bout donne l'indice 1, qui ouvre le Marais ; le Bourreau attend au bout du Marais, et la première expédition menée au bout donne l'indice 2, qui ouvre la Vallée des Brumes ; chaque expédition dans la Vallée s'achève par le Baron pendu tant qu'il n'est pas vaincu. Jusqu'à l'étape 29, deux expéditions complètes dans le Marais donnaient les indices 2 et 3, et le Baron pendu attendait au bout du Marais.

Nombre d'étapes d'une expédition manuelle, fixe depuis l'étape 26 (décidé le 18/10/2026, gamedesign.md, section 9.2) : 3 dans la Lisière, 4 dans le Marais, 5 dans la Vallée. Dans la V1, il était tiré entre 4 et 6. Depuis l'étape 33 (19/10/2026), 5 dans chaque zone.

Expéditions automatiques, ajoutées après la V1 le 18/10/2026 (section 12) :

| Identifiant | Durée de base, pour un monstre | Dégâts | XP par monstre non KO | Potions | Argent |
|---|---|---|---|---|---|
| lisiere | 1 h | 15 à 60 | 100 à 200 | 0 à 1 | 15 à 30 |
| marais | 2 h | 40 à 120 | 300 à 600 | 1 à 2 | 40 à 80 |
| vallee | 3 h | 90 à 220 | 700 à 1300 | 1 à 3 | 80 à 150 |

L'argent est ajouté le 18/10/2026, avec l'économie (section 6).

Coffres des expéditions manuelles, après la V1 (section 6) :

| Identifiant | Potions | Argent | Capsules d'attaque |
|---|---|---|---|
| lisiere | 1 à 2 | 10 à 25 | Brasier, Déferlante, Lianes étrangleuses |
| marais | 1 à 2 | 25 à 50 | Croc de venin, Frisson d'outre-tombe, Sentence |
| vallee | 1 à 2 | 50 à 90 | Maléfice, Bourrasque, Furie draconique |

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

Depuis l'étape 30, le premier indice s'affiche au retour de la première expédition menée au bout dans la Lisière, le second au retour de la première expédition menée au bout dans le Marais, après le Bourreau :

> Indice 1 : au bout de la Lisière, des cordes neuves pendent encore à un chêne mort. Les traces d'une procession s'enfoncent vers le sud, là où la forêt se noie : le Marais des Pendus. Quelque chose y règne.

> Indice 2 : le bourreau vaincu a parlé : son maître, le Baron pendu, s'est retiré au fond de la Vallée des Brumes, au-delà du Marais, là où les morts ne dorment pas.

Jusqu'à l'étape 29, la piste comptait trois indices, les deux derniers trouvés dans le Marais :

> Indice 2 : des noyés flottent entre les roseaux, tous tournés vers le même point de la brume, comme s'ils écoutaient un appel.

> Indice 3 : un gibet se dresse sur une île de vase, plus haut que les arbres. À son pied, la terre est retournée par des pas trop lourds pour un homme. L'antre de l'Apôtre est là.

### Combat contre l'Apôtre

Depuis l'étape 30, au bout de la Vallée des Brumes. Avant le combat :

> Au fond de la Vallée, la brume se déchire sur un gibet dressé contre le ciel. Le Baron pendu, Apôtre du Démiurge, s'y balance encore, puis ouvre les yeux. Il n'y aura pas de fuite.

Épilogue, après la victoire :

> Le Baron pendu s'effondre au pied de son gibet. Sur sa poitrine, un sceau gravé d'une main ouverte : le signe du Démiurge. D'autres Apôtres portent ce sceau, au-delà des terres connues. Votre traque ne fait que commencer.

Jusqu'à l'étape 29, au bout du Marais, le texte avant le combat était : "Sous le gibet, une silhouette se déplie lentement, la corde encore au cou. Le Baron pendu, Apôtre du Démiurge, vous a senti venir. Il n'y aura pas de fuite." L'épilogue commençait par "Le Baron pendu s'effondre dans la vase."

### Combat contre le brigand

Ajouté à l'étape 29, le 19/10/2026 (gamedesign.md, section 10.7). Le brigand attend au bout de la Lisière, avec un Loup de niveau 7 et un Corbeau de niveau 8 (valeurs internes à 15, première habileté de leur espèce). Victoire : 60 pièces d'argent.

Avant le combat :

> Au sortir de la Lisière, un brigand vous barre la route, un loup et un corbeau à ses côtés. Ses bêtes ont faim, et lui aussi.

Après la victoire :

> Le brigand détale dans les fourrés, abandonnant ses bêtes et sa bourse.

### Combat contre le Bourreau

Ajouté à l'étape 30, le 19/10/2026 (gamedesign.md, section 10.7). Le Bourreau attend au bout du Marais, avec une Armure hantée de niveau 14, un Feu follet de niveau 14 et un Basilic de niveau 15 (valeurs internes à 15, première habileté de leur espèce). Victoire : 150 pièces d'argent, puis l'indice 2 au retour.

Avant le combat :

> Au cœur du Marais, sous un gibet chargé de cordes, un bourreau encagoulé aiguise sa hache. Trois bêtes attendent ses ordres. Il sert le Baron pendu, et il ne vous laissera pas passer.

Après la victoire :

> Le bourreau s'effondre dans la vase, sa hache brisée.

### Arrivée de Malvoise et âmes des marchands

Ajoutées à l'étape 31, le 19/10/2026 (gamedesign.md, sections 14.2 et 14.4).

Arrivée de Malvoise, au retour de la première expédition menée au bout dans la Lisière, après l'indice (remplacée à l'étape 34 par la rencontre de Malvoise, plus bas) :

> Au camp, un homme en robe noire vous attend près du feu, un crâne posé sur les genoux. Malvoise, nécromancien, a vu mourir bien des marchands sur ces routes. Rapportez-lui leur âme, et il vous vendra ce qu'ils vendaient de leur vivant.

| Marchand | Zone | Rencontre | Objet ajouté à l'étal |
|---|---|---|---|
| Bertrand, colporteur | Lisière du Bois-Sombre | un monstre de la zone le tue et attaque | Remède, 25 |
| Grete, herboriste | Marais des Pendus | elle s'est relevée en marchand zombie, qui attaque | Grande potion, 60 |
| Ulrich, alchimiste | Vallée des Brumes | il pend au gibet, gardé par un monstre de la zone | Élixir, 150 |

Le monstre qui tue Bertrand ou garde Ulrich est tiré parmi les espèces communes de la zone, au niveau maximal de la zone. Depuis le 19/10/2026, la bête du brigand qui attaque à la place de Bertrand est toujours un Loup de niveau 7, aux valeurs internes, shiny et habileté tirés.

Bertrand, avant le combat (le monstre n'est pas nommé : le combat le présente aussitôt, et son nom ne dit pas son genre) :

> Un cri déchire la Lisière : un monstre se jette sur un colporteur et l'abat sous vos yeux. La bête se tourne vers vous.

Après la victoire, suivi de "Malvoise vendra désormais le remède." (la grande potion pour Grete, l'élixir pour Ulrich) :

> L'âme de Bertrand, colporteur, flotte au-dessus de son corps. Vous la recueillez pour Malvoise.

Remplacé le 19/10/2026, pour l'étape 34 : Bertrand est abattu à l'étape 4 de la Lisière par le brigand qui attend à l'étape 5, et une bête qui le suivait attaque. Le texte avant le combat devient :

> Un cri déchire la Lisière : un brigand abat un colporteur sous vos yeux, lui arrache sa bourse et s'enfuit vers la sortie du bois. Une bête qui le suivait se tourne vers vous.

L'âme de Bertrand se recueille avant que le joueur connaisse Malvoise, et le soin ne s'ajoute à l'étal qu'au don de l'âme. Le texte après la victoire devient :

> L'âme de Bertrand, colporteur, flotte au-dessus de son corps. Vous la recueillez, sans trop savoir ce que c'est.

Ceux de Grete et d'Ulrich ne changent pas, puisque Malvoise est déjà au camp quand leur zone s'ouvre. La phrase "Malvoise vendra désormais ..." ne suit plus la victoire, mais le don de l'âme.

Grete, avant le combat :

> Une marchande titube vers vous entre les roseaux, la peau verdie, sa carriole renversée derrière elle. Elle n'est plus tout à fait morte.

Après la victoire :

> Le corps de Grete s'effondre enfin. Son âme flotte au-dessus de la vase : vous la recueillez pour Malvoise.

Ulrich, avant le combat (le monstre n'est pas nommé, comme pour Bertrand) :

> Au pied d'un gibet, un alchimiste pend au bout d'une corde, ses fioles brisées dans l'herbe. Un monstre garde le corps et se jette sur vous.

Après la victoire :

> L'âme d'Ulrich, alchimiste, s'échappe enfin de la corde. Vous la recueillez pour Malvoise.

Marchand zombie (hors du codex, ni filet ni capture) :

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Niveau |
|---|---|---|---|---|---|---|
| marchand-zombie | Marchand zombie | Spectre, Poison | offensif | 60 / 70 / 60 / 40 / 55 / 35 (320) | 95 | 13 |

Attaques : Coup de garde (niv. 1), Morsure venimeuse (niv. 1), Charge spectrale (niv. 1). Valeurs internes à 15, jamais shiny.

### Rencontre de Malvoise et don des âmes

Ajoutés à l'étape 34, le 19/10/2026 (gamedesign.md, section 14.2) : Malvoise est rencontré au bout de la première expédition menée au bout, avant le retour au camp, et remplace le texte de son arrivée au camp.

Si le joueur porte l'âme de Bertrand :

> Sur le chemin du camp, un homme en robe noire vous attend, un crâne posé au creux du bras. "Je t'ai vu recueillir cette âme. Elle m'intéresse, si tu ne sais pas quoi en faire. Je suis Malvoise, et les marchands morts sur ces routes ont encore des choses à vendre." Il vous suit jusqu'au camp et s'y installe.

Sinon :

> Sur le chemin du camp, un homme en robe noire vous attend, un crâne posé au creux du bras. "Je suis Malvoise. Les marchands meurent vite sur ces routes : rapporte-moi leur âme, et je te vendrai ce qu'ils vendaient de leur vivant." Il vous suit jusqu'au camp et s'y installe.

Don d'une âme, au camp, chez Malvoise (le remède pour Bertrand, la grande potion pour Grete, l'élixir pour Ulrich) :

> Vous remettez l'âme de Bertrand à Malvoise, qui l'enferme dans le crâne. Malvoise vendra désormais le remède.

### Rencontre du marchand

Ajoutée après la V1, le 18/10/2026 : affichée à l'étape où le marchand est rencontré, avant qu'il se réfugie au camp (gamedesign.md, section 14.2). Retirée à l'étape 31 : Malvoise remplace le marchand.

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
- Le Baron pendu est le combat le plus dur de la V1. Seul le type Féérique lui est très efficace : la Lumière d'aube du Cerf blanc, monstre rare pacifique de la Lisière, en fait une prise précieuse pour l'affronter. Depuis l'étape 30, Obscur au niveau 22, il est bien plus dur encore : le Cerf blanc, l'Elfe (Lumière d'aube et Poussière d'elfe) et le Schnoz sont les seuls à lui être très efficaces, aucun monstre n'étant Psychique. À équilibrer en jouant (gamedesign.md, Q76).
- Rythme estimé : une expédition dans la Lisière rapporte de l'ordre de 200 à 400 XP par participant, soit le passage du niveau 5 au niveau 8 environ en trois expéditions. Le Marais devient abordable vers le niveau 9, le Baron pendu vers le niveau 13 ou 14. À vérifier en jouant.
- Le Chat noir, de type Ténèbres, résiste aux attaques Spectre et Ténèbres : contre les Spectre du Marais, l'Assassin a le départ le plus facile. Depuis l'étape 30, le Baron pendu est Obscur : le Chat noir n'a plus d'avantage contre lui (gamedesign.md, Q76). En contrepartie, il a le moins de PV et de défenses des monstres de départ.
- Le Schnoz, très rare, est un cadeau : 450 de stats de base, sans faiblesse ni résistance tant que le type Stellaire est neutre.
- Place du combat contre l'Apôtre (Q57) : à la dernière étape de l'expédition qui suit le 3e indice, à la place de l'événement tiré ; rentrer avant cette étape le repousse à l'expédition suivante. Depuis l'étape 30, à la dernière étape de chaque expédition dans la Vallée des Brumes, tant qu'il n'est pas vaincu.
- La Vallée des Brumes (niveaux 14 à 20) compte deux espèces rares : l'Elfe, pacifique, et la Vouivre, hostile, chacune à une étape sur 40 environ. L'Armure hantée y revient, plus forte qu'au Marais.

## 6. Économie

Ajoutée après la V1, le 18/10/2026 (gamedesign.md, sections 13.10 et 14). Les montants se comptent en pièces d'argent.

Revente d'un monstre au marchand, depuis l'étape 26 : son niveau multiplié par le total des stats de base de son espèce, divisé par 50 en division entière, puis multiplié par le coefficient de rareté de l'espèce, et doublé pour un shiny. Avant l'étape 26, c'était son niveau multiplié par 5, 15 ou 40 selon la rareté.

| Rareté | Espèces | Coefficient | Exemple |
|---|---|---|---|
| commune | monstres de départ et monstres communs | 1 | Loup de niveau 5 (285 de stats de base) : 28 |
| rare | Cerf blanc, Basilic, Elfe, Vouivre, Schnoz | 2 | Cerf blanc de niveau 5 (400) : 80 ; Schnoz de niveau 10 (450) : 180 |
| Apôtre | Baron pendu | 4 | Baron pendu de niveau 15 (480) : 576 ; au niveau 22 : 844 |

Prix du marchand : une potion coûte 20. Il vend toutes les capsules d'attaque. Depuis l'étape 31, c'est Malvoise qui tient l'étal ; il vend aussi le remède (25), la grande potion (60) et l'élixir (150), chacun une fois l'âme de son marchand recueillie.

Pierres d'évolution, ajoutées à l'étape 39 le 19/10/2026 (gamedesign.md, section 5.4) : la Pierre de braise (identifiant pierre-de-braise) et la Pierre de lune (pierre-de-lune), 300 chacune, vendues par Malvoise, et trouvées dans un coffre sur 10, tirées entre les deux. Elles se consomment. Prix et taux sont des hypothèses (gamedesign.md, Q84).

Capsule d'habileté, ajoutée à l'étape 35 le 19/10/2026 (gamedesign.md, section 5.9) : 200, vendue par Malvoise, et trouvée dans un coffre sur 10. Elle fait passer un monstre à l'autre habileté de son espèce et se consomme.

Ressources, ajoutées à l'étape 36 le 19/10/2026 (gamedesign.md, section 14.3) : une par espèce sauvage qui peut être combattue, lâchée 1 fois sur 3 par un monstre sauvage mis KO et non capturé, après un combat gagné. Malvoise les rachète 5 pour une espèce commune, 15 pour une rare. Noms, taux et prix sont des hypothèses (Q82).

| Identifiant | Ressource | Espèce | Prix |
|---|---|---|---|
| croc-de-loup | Croc de loup | Loup | 5 |
| plume-de-corbeau | Plume de corbeau | Corbeau | 5 |
| ecaille-de-vipere | Écaille de vipère | Vipère | 5 |
| cendre-de-feu-follet | Cendre de feu follet | Feu follet | 5 |
| fragment-d-armure | Fragment d'armure | Armure hantée | 5 |
| aile-de-fee-noire | Aile de fée noire | Fée noire | 5 |
| os-de-squelette | Os de squelette | Squelette | 5 |
| croc-de-basilic | Croc de basilic | Basilic | 15 |
| ecaille-de-vouivre | Écaille de vouivre | Vouivre | 15 |

Capsules d'attaque : chacune enseigne son attaque à tout monstre qui a le type de cette attaque. Un coffre en contient une fois sur 5, tirée parmi les capsules de sa zone que le joueur n'a pas encore.

| Capsule | Type | Utile à | Zone des coffres | Prix |
|---|---|---|---|---|
| Brasier | Feu | Salamandre, Feu follet | lisiere | 150 |
| Déferlante | Eau | Crapaud | lisiere | 150 |
| Lianes étrangleuses | Plante | Mandragore | lisiere | 150 |
| Croc de venin | Poison | Vipère | marais | 200 |
| Frisson d'outre-tombe | Spectre | Armure hantée, Squelette | marais | 200 |
| Sentence | Ténèbres | Corbeau, Chat noir, Fée noire | marais | 300 |
| Maléfice | Ténèbres | Corbeau, Chat noir | vallee | 300 |
| Bourrasque | Vol | Corbeau, Vouivre | vallee | 300 |
| Furie draconique | Dragon | Basilic | vallee | 400 |

Les capsules de la Lisière servent aux monstres de départ, dont l'espèce apprend déjà toutes les attaques de leur type qui existaient dans la V1 ; celles du Marais donnent aux monstres communs une attaque d'une autre espèce de leur type, comme celles de la Vallée, ajoutées à l'étape 30. Depuis que le Baron pendu est Obscur, Frisson d'outre-tombe et Sentence ne lui servent plus.

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
| Fée noire | Sorcellerie, Célérité |
| Squelette | Férocité, Carapace |
| Elfe | Volonté, Instinct |
| Vouivre | Affinité, Férocité |

Valeurs : +10 % pour les habiletés de stats (Férocité, Carapace, Sorcellerie, Volonté, Célérité), +20 % de dégâts pour Affinité, +20 % d'XP pour Instinct, -20 % de durée pour Éclaireur, capture x1,25 pour Traqueur, +50 % d'argent pour Pillard, une potion 1 fois sur 4 pour Fouineur.

## 8. Évolutions

Ajoutées le 19/10/2026, pour les étapes 37 à 40 (gamedesign.md, section 5.4), contenu validé par l'utilisateur. Stats de base dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse.

Chaque forme évoluée garde le rôle, la rareté et les habiletés de sa forme de base (hypothèse, gamedesign.md, Q84), n'a pas de ressource, et son rendement d'XP vaut 1,4 fois celui de sa forme de base, arrondi à l'entier inférieur. Dans le codex, les formes évoluées viennent après les 17 espèces d'avant l'étape 37, dans l'ordre du tableau.

| Forme de base | Identifiant | Évolution | Condition | Types | Stats de base (total) | Rendement d'XP | Attaque d'évolution | Autre attaque, au même niveau |
|---|---|---|---|---|---|---|---|---|
| Salamandre | lezard-de-braise | Lézard de braise | niveau 16 | Feu | 60 / 50 / 55 / 92 / 70 / 83 (410) | 86 | Jet de lave | Souffle putride |
| Crapaud | crapaud-geant | Crapaud géant | niveau 16 | Eau | 72 / 88 / 70 / 45 / 58 / 67 (400) | 88 | Trombe | Déferlante |
| Mandragore | mandragore-ancienne | Mandragore ancienne | niveau 16 | Plante | 90 / 58 / 90 / 58 / 84 / 30 (410) | 89 | Hurlement de mandragore | Lianes étrangleuses |
| Chat noir | chat-des-ombres | Chat des ombres | niveau 16 | Ténèbres | 52 / 92 / 45 / 50 / 48 / 113 (400) | 86 | Griffe d'ombre | Maléfice |
| Loup | warg | Warg | niveau 16 | Normal, Ténèbres | 65 / 80 / 58 / 38 / 52 / 75 (368) | 98 | Croc noir | Coup sournois |
| Corbeau | corbeau-des-gibets | Corbeau des gibets | niveau 16 | Vol, Ténèbres | 55 / 72 / 48 / 58 / 48 / 90 (371) | 92 | Bec funeste | Bourrasque |
| Vipère | aspic | Aspic | niveau 16 | Poison | 52 / 64 / 52 / 64 / 58 / 80 (370) | 95 | Venin noir | Croc de venin |
| Fée noire | fee-sorciere | Fée-sorcière | niveau 12 (type Insecte) | Insecte, Ténèbres | 50 / 44 / 50 / 84 / 64 / 96 (388) | 100 | Nuée | Sentence |
| Basilic | basilic-royal | Basilic royal | niveau 22 (type Dragon) | Poison, Dragon | 90 / 88 / 80 / 92 / 75 / 70 (495) | 238 | Regard de mort | Furie draconique |
| Feu follet | lanterne-des-morts | Lanterne des morts | Pierre de braise | Feu, Spectre | 58 / 38 / 52 / 90 / 78 / 96 (412) | 119 | Flamme funèbre | Brasier |
| Cerf blanc | cerf-de-lune | Cerf de lune | Pierre de lune | Normal, Féérique | 85 / 75 / 75 / 82 / 88 / 95 (500) | 210 | Rayon de lune | aucune |
| Armure hantée | chevalier-noir | Chevalier noir | niveau 18, dans la Vallée des Brumes | Acier, Spectre | 75 / 95 / 105 / 38 / 62 / 38 (413) | 133 | Taille d'acier | Frisson d'outre-tombe |
| Squelette | chevalier-squelette | Chevalier squelette | niveau 20, Baron pendu vaincu | Spectre, Combat | 62 / 100 / 75 / 38 / 56 / 69 (400) | 128 | Épée des morts | Coup de garde |

Attaques apprises : celles de la forme de base, aux mêmes niveaux, puis l'attaque d'évolution et l'autre attaque au niveau de la condition ; au niveau 1 pour une évolution par pierre. Exemple, Lézard de braise : Souffle chaud (niv. 1), Crachat de braises (niv. 3), Souffle draconique (niv. 7), Flamme vive (niv. 10), Fournaise (niv. 14), Jet de lave (niv. 16), Souffle putride (niv. 16).

Stades suivants, nommés seulement, à coder avec des zones au-delà du niveau 20 (gamedesign.md, Q84) :

| Lignée | Deuxième évolution | Troisième évolution |
|---|---|---|
| Salamandre, Lézard de braise | Wyverne de cendre (Feu, Dragon), niveau 36 | Dragon de feu, niveau 70 |
| Crapaud, Crapaud géant | Crapaud-roi (Eau, Poison), niveau 36 | aucune |
| Mandragore, Mandragore ancienne | Arbre-pendu (Plante, Spectre), niveau 36 | aucune |
| Chat noir, Chat des ombres | Cait sith (Ténèbres, Spectre), niveau 36 | aucune |
| Loup, Warg | Loup-garou (Combat, Ténèbres), niveau 36 | aucune |
| Fée noire, Fée-sorcière | Reine des ronces, niveau 25 | aucune |

La Vouivre évoluera une fois, en Dragon des brumes, au niveau 45. L'Elfe, le Baron pendu, le Schnoz et le marchand zombie n'évoluent pas ; les autres espèces s'arrêtent à leur première évolution.

Textes, avec le nom affiché du monstre et le nom de l'espèce. Proposition, après un combat :

> Salamandre peut évoluer en Lézard de braise. Faire évoluer Salamandre ?

Les réponses sont "Évoluer" et "Pas maintenant". Au camp, la confirmation :

> Faire évoluer Salamandre en Lézard de braise ? L'évolution est définitive.

Après l'évolution, suivi de l'attaque apprise, comme à la montée de niveau, puis du rappel :

> Salamandre évolue en Lézard de braise !

> D'autres attaques peuvent s'apprendre au camp.

Refus après un combat :

> Salamandre n'évolue pas pour l'instant.
