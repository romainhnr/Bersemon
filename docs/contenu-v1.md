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

Pierres d'évolution, ajoutées à l'étape 39 le 19/10/2026 (gamedesign.md, section 5.4) : la Pierre de braise (identifiant pierre-de-braise) et la Pierre de lune (pierre-de-lune), 300 chacune, vendues par Malvoise, et trouvées dans un coffre sur 10, tirées entre les deux. Elles se consomment. Un même tirage sur 100 décide de la capsule d'habileté (1 à 10), de la Pierre de braise (11 à 15) et de la Pierre de lune (16 à 20). Prix et taux sont des hypothèses (gamedesign.md, Q84).

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

Deuxième évolution décidée le 19/10/2026, pour les étapes 44 à 47 (gamedesign.md, section 5.4), contenu validé par l'utilisateur :

| Forme | Identifiant | Évolution | Condition | Types | Stats de base (total) | Rendement d'XP | Attaque d'évolution | Autre attaque, au même niveau |
|---|---|---|---|---|---|---|---|---|
| Fée-sorcière | reine-des-ronces | Reine des ronces | niveau 25 (type Insecte) | Insecte, Plante | 70 / 55 / 65 / 110 / 85 / 95 (480) | 140 | Ronces maudites | Lianes étrangleuses (hypothèse) |

Ronces maudites : Plante, spéciale, puissance 85, précision 90, poison 20 %. La Reine des ronces garde les habiletés de la Fée-sorcière (Sorcellerie, Célérité).

Stades suivants, nommés seulement, à coder avec des zones au-delà du niveau 20 (gamedesign.md, Q84) :

| Lignée | Deuxième évolution | Troisième évolution |
|---|---|---|
| Salamandre, Lézard de braise | Wyverne de cendre (Feu, Dragon), niveau 36 | Dragon de feu, niveau 70 |
| Crapaud, Crapaud géant | Crapaud-roi (Eau, Poison), niveau 36 | aucune |
| Mandragore, Mandragore ancienne | Arbre-pendu (Plante, Spectre), niveau 36 | aucune |
| Chat noir, Chat des ombres | Cait sith (Ténèbres, Spectre), niveau 36 | aucune |
| Loup, Warg | Loup-garou (Combat, Ténèbres), niveau 36 | aucune |
| Fée noire, Fée-sorcière | Reine des ronces, niveau 25, codée à l'étape 47 | aucune |

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

## 9. Améliorations du camp

Décidées le 19/10/2026, pour les étapes 41 à 43 (gamedesign.md, section 13.4), coûts et durées validés par l'utilisateur, à ajuster en jouant.

Libération de Gunther, après le texte de la victoire sur le Bourreau :

> Derrière le gibet, un homme enchaîné relève la tête : Gunther, forgeron, que le bourreau gardait pour forger ses chaînes. Libéré, il vous suit jusqu'au camp et y installe sa forge.

| Amélioration | Identifiant | Niveau | Effet | Coût | Durée |
|---|---|---|---|---|---|
| Enclos | enclos | 1 | camp de 30 à 40 places | 5 Croc de loup, 5 Plume de corbeau, 5 Écaille de vipère | 1 h |
| | | 2 | de 40 à 50 places | 6 Cendre de feu follet, 6 Fragment d'armure | 2 h |
| | | 3 | de 50 à 60 places | 6 Aile de fée noire, 6 Os de squelette | 4 h |
| Filets | filets | 1 | chance de capture x1,1 | 4 Croc de loup, 4 Écaille de vipère | 1 h |
| | | 2 | x1,2 | 5 Cendre de feu follet, 5 Fragment d'armure | 3 h |
| | | 3 | x1,3 | 5 Aile de fée noire, 5 Os de squelette | 6 h |
| Équipe | equipe | 1 | équipe de 3 à 4 monstres (étape 43) | 1 Croc de basilic, 1 Écaille de vouivre, 8 Os de squelette, 8 Aile de fée noire | 8 h |

Les ressources rares sont lentes à réunir : une expédition dans le Marais rapporte un Croc de basilic environ une fois sur 12. Elles ne sont demandées qu'à la dernière amélioration.

## 10. Abbaye des Cendres et deuxième Apôtre

Décidés le 19/10/2026, pour les étapes 44 à 47 (gamedesign.md, sections 5.2, 9.1 et 9.4), contenu validé par l'utilisateur. Codés aux étapes 45 (zone, espèces, capsules, ressources) et 46 (Inquisiteur, Abbesse écorchée, indices 3 et 4). L'Inquisiteur a l'IA relevée du Bourreau (hypothèse, gamedesign.md, Q86).

Zone :

| Identifiant | Nom | Niveaux | Taille des groupes | Espèces | Monstres rares | Humain | Apôtre |
|---|---|---|---|---|---|---|---|
| abbaye | Abbaye des Cendres | 21 à 27 | 1 à 3 | Squelette, Gargouille, Goule, Chauve-souris de crypte | Chimère (hostile), Ange déchu (pacifique) | Inquisiteur | Abbesse écorchée |

Expédition automatique : durée de base 4 h, dégâts 150 à 320, XP 1300 à 2200, potions 1 à 3, argent 120 à 220. Coffres : 1 à 2 potions et 90 à 150 pièces d'argent ; capsules Bûcher (450), Festin macabre (400) et Ultrason (400).

Espèces (stats dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse) :

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Rareté | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|---|
| gargouille | Gargouille | Roche, Vol | soutien | 60 / 80 / 90 / 30 / 50 / 40 (350) | 105 | commune | Carapace, Férocité | Griffe de pierre (1), Piqué (10), Chute de pierres (22) |
| goule | Goule | Poison, Ténèbres | offensif | 65 / 78 / 55 / 40 / 45 / 57 (340) | 102 | commune | Instinct, Férocité | Morsure (1), Morsure venimeuse (1), Croc de venin (18), Festin macabre (24) |
| chauve-souris-de-crypte | Chauve-souris de crypte | Vol, Poison | offensif | 50 / 55 / 45 / 55 / 45 / 90 (340) | 100 | commune | Célérité, Éclaireur | Coup de bec (1), Cri strident (1), Morsure venimeuse (8), Ultrason (22) |
| chimere | Chimère | Feu, Dragon | offensif | 75 / 80 / 65 / 85 / 60 / 55 (420) | 185 | rare, hostile | Affinité, Férocité | Crachat de braises (1), Souffle draconique (1), Flamme vive (15), Furie draconique (25) |
| ange-dechu | Ange déchu | Psychique, Vol | soutien | 70 / 50 / 60 / 95 / 90 / 55 (420) | 180 | rare, pacifique | Volonté, Instinct | Onde psychique (1), Bourrasque (12), Jugement (24) |
| abbesse-ecorchee | Abbesse écorchée | Obscur | offensif | 90 / 75 / 80 / 100 / 90 / 75 (510) | 280 | Apôtre, niveau 29 | Sorcellerie, Volonté | Lamentation (1), Maléfice (8), Hurlement des abysses (15), Bûcher (20), Flagellation (25), Psaume impie (28) |

Les rôles de la Gargouille et de l'Ange déchu, non précisés dans la proposition, sont des hypothèses (soutien), comme la répartition des habiletés.

Attaques nouvelles :

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| griffe-de-pierre | Griffe de pierre | Roche | P | 50 | 100 | |
| chute-de-pierres | Chute de pierres | Roche | P | 75 | 90 | |
| festin-macabre | Festin macabre | Ténèbres | P | 80 | 90 | |
| ultrason | Ultrason | Normal | S | 70 | 100 | paralysie 10 % |
| onde-psychique | Onde psychique | Psychique | S | 65 | 100 | |
| jugement | Jugement | Psychique | S | 90 | 90 | |
| bucher | Bûcher | Feu | S | 90 | 85 | |
| flagellation | Flagellation | Obscur | P | 85 | 95 | |
| psaume-impie | Psaume impie | Obscur | S | 90 | 95 | paralysie 20 % |
| ronces-maudites | Ronces maudites | Plante | S | 85 | 90 | poison 20 % |

Ressources : Éclat de gargouille, Dent de goule et Aile de chauve-souris (5 pièces d'argent chacune), Crin de chimère (15).

Depuis l'étape 94 (24/10/2026), l'Abbaye accueille aussi la Cloche hurlante, commune, et le Séraphin de vitrail, rare et pacifique (section 24).

L'Inquisiteur attend au bout de l'Abbaye avec une Gargouille de niveau 27, une Goule de niveau 27 et une Chimère de niveau 28 (valeurs internes à 15, première habileté de leur espèce). Victoire : 300 pièces d'argent, puis l'indice 4 au retour.

Textes :

> Indice 3 : parmi les reliques du Baron, une lettre scellée du même sceau, signée de l'Abbesse : à l'est, l'Abbaye des Cendres brûle sans jamais s'éteindre. Une autre Apôtre y attend.

> Indice 4 : l'inquisiteur vaincu a parlé : l'Abbesse écorchée se terre sous le chœur, dans la crypte où elle a livré ses sœurs au bûcher.

Inquisiteur, avant le combat :

> Sur le parvis de l'abbaye, un inquisiteur en robe rouge vous attend, entouré de trois bêtes. Il brûle au nom de l'Abbesse, et il compte bien vous brûler aussi.

Après la victoire :

> L'inquisiteur s'effondre sur les dalles, son livre de prières en cendres.

Abbesse écorchée, avant le combat :

> Au fond de la crypte, une silhouette sans peau prie devant un autel de chair. L'Abbesse écorchée, Apôtre du Démiurge, se retourne lentement. Il n'y aura pas de fuite.

Épilogue :

> L'Abbesse écorchée s'effondre sur l'autel. Le même sceau, une main ouverte, est gravé dans sa chair. Deux Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.

## 11. Équipement du héros

Décidé le 20/10/2026, pour les étapes 48 à 51 (gamedesign.md, sections 4.3 et 13.5), contenu validé par l'utilisateur, à ajuster en jouant. Chaque objet se fabrique une fois chez Gunther, contre des ressources seules, avec une durée. Les bonus s'ajoutent, en pourcentage, aux stats des monstres du joueur en combat. Aucun objet ne donne de PV.

| Identifiant | Objet | Sorte | Bonus | Coût | Durée |
|---|---|---|---|---|---|
| couteau-de-chasse | Couteau de chasse | arme | +5 % d'Attaque | 6 Croc de loup, 4 Écaille de vipère | 1 h |
| baton-d-if | Bâton d'if | arme | +5 % d'Attaque spéciale | 6 Plume de corbeau, 4 Écaille de vipère | 1 h |
| epee-du-pendu | Épée du pendu | arme | +10 % d'Attaque | 6 Fragment d'armure, 6 Os de squelette, 1 Croc de basilic | 3 h |
| grimoire-des-brumes | Grimoire des brumes | arme | +10 % d'Attaque spéciale | 6 Cendre de feu follet, 6 Aile de fée noire, 1 Croc de basilic | 3 h |
| espadon-des-cendres | Espadon des cendres | arme | +15 % d'Attaque | 8 Éclat de gargouille, 6 Dent de goule, 1 Crin de chimère | 6 h |
| encensoir-maudit | Encensoir maudit | arme | +15 % d'Attaque spéciale | 8 Aile de chauve-souris, 6 Dent de goule, 1 Crin de chimère | 6 h |
| brigandine-de-cuir | Brigandine de cuir | armure | +5 % de Défense et de Défense spéciale | 6 Croc de loup, 6 Plume de corbeau | 2 h |
| cotte-de-mailles | Cotte de mailles | armure | +10 % de Défense et de Défense spéciale | 8 Fragment d'armure, 4 Os de squelette, 1 Écaille de vouivre | 4 h |
| plates-de-gargouille | Plates de gargouille | armure | +15 % de Défense et de Défense spéciale | 10 Éclat de gargouille, 6 Os de squelette, 1 Crin de chimère | 8 h |
| croc-tresse | Croc tressé | talisman | +5 % d'Attaque | 8 Croc de loup | 1 h |
| plume-de-corbeau-sertie | Plume de corbeau sertie | talisman | +5 % de Vitesse | 8 Plume de corbeau | 1 h |
| lanterne-de-feu-follet | Lanterne de feu follet | talisman | +5 % d'Attaque spéciale | 8 Cendre de feu follet | 2 h |
| os-grave | Os gravé | talisman | +5 % de Défense | 8 Os de squelette | 2 h |
| aile-de-fee-sertie | Aile de fée sertie | talisman | +5 % de Défense spéciale | 8 Aile de fée noire | 2 h |

Paliers : le premier se fabrique avec les ressources de la Lisière et du Marais, le deuxième avec celles du Marais et de la Vallée, plus une ressource rare, le troisième avec celles de l'Abbaye des Cendres et un Crin de chimère.

## 12. Ordres du héros

Décidé le 20/10/2026, pour les étapes 52 à 54 (gamedesign.md, section 4.2), noms et textes validés par l'utilisateur. Un ordre par combat, au monstre actif, jusqu'à la fin du combat. {héros} est le nom du héros, {monstre} celui du monstre.

| Classe | Ordre | Bonus | Texte |
|---|---|---|---|
| Guerrier | Cri de guerre | +20 % d'Attaque | {héros} pousse un cri de guerre : {monstre} gagne 20 % d'Attaque jusqu'à la fin du combat. |
| Mage | Incantation | +20 % d'Attaque spéciale | {héros} murmure une incantation : {monstre} gagne 20 % d'Attaque spéciale jusqu'à la fin du combat. |
| Templier | Bouclier de foi | +20 % de Défense et de Défense spéciale | {héros} invoque un bouclier de foi : {monstre} gagne 20 % de Défense et de Défense spéciale jusqu'à la fin du combat. |
| Assassin | Pas de l'ombre | +20 % de Vitesse | {héros} souffle le pas de l'ombre : {monstre} gagne 20 % de Vitesse jusqu'à la fin du combat. |

## 13. Don et échange de monstres

Décidé le 20/10/2026, pour les étapes 55 à 58 (gamedesign.md, sections 15.1 et 15.2), textes validés par l'utilisateur. {héros} est le nom du héros de l'autre joueur, {pseudo} son pseudo Discord, {monstre} le nom d'un monstre.

| Moment | Texte |
|---|---|
| Combat, un monstre reçu désobéit | {monstre} n'en fait qu'à sa tête et ignore votre ordre. |
| Fiche d'un monstre reçu | Reçu de {héros}. Obéit jusqu'au niveau {limite}. |
| Message privé, offre d'échange | {héros} ({pseudo}) vous propose {son monstre} contre votre {monstre}. Ouvrez /bersemon, au camp, pour répondre. |
| Message privé, offre de don | {héros} ({pseudo}) vous offre {monstre}. Ouvrez /bersemon, au camp, pour répondre. |
| Message privé, offre acceptée | {héros} a accepté votre offre. |
| Message privé, offre refusée | {héros} a refusé votre offre. |
| Récit, pour chaque monstre cédé (hypothèse, Q89) | Vous cédez {monstre} à {héros}. |
| Récit, pour chaque monstre reçu (hypothèse, Q89) | Vous recevez {monstre} de {héros}. |

Au camp, le bouton s'appelle "Échanges", puis "Autres joueurs" depuis l'étape 60 (section 14). Limite d'obéissance : niveau maximal de la zone la plus avancée ouverte, plus 5 (12, 18, 25 puis 32).

## 14. Duel en temps réel

Décidé le 20/10/2026, pour les étapes 59 à 61 (gamedesign.md, section 15.3), textes validés par l'utilisateur. {héros} est le nom du héros de l'autre joueur, {pseudo} son pseudo Discord.

| Moment | Texte |
|---|---|
| Message privé, défi reçu | {héros} ({pseudo}) vous défie en duel. Ouvrez /bersemon, au camp, pour répondre. |
| Message privé, défi accepté | {héros} a accepté votre défi : le duel commence. Ouvrez /bersemon. |
| Message privé, défi refusé | {héros} a refusé votre défi. |
| Duel, après avoir choisi son action | En attente de l'action de {héros}. |
| Duel, l'autre abandonne | {héros} abandonne le duel. |
| Duel, victoire par forfait | {héros} n'a pas joué depuis 5 minutes : vous remportez le duel par forfait. |
| Fin du duel, victoire | Victoire : vous remportez le duel contre {héros}. |
| Fin du duel, défaite | Défaite : {héros} remporte le duel. |

Boutons : Défier en duel, Accepter le défi, Refuser le défi, Retirer le défi, Abandonner, Actualiser, Réclamer la victoire. Au camp, le bouton "Échanges" devient "Autres joueurs".

## 15. Pêche

Décidé le 20/10/2026, pour les étapes 62 à 65 (gamedesign.md, sections 5.3, 9.2 et 13.8), contenu validé par l'utilisateur, à ajuster en jouant.

Ansgar, vieux pêcheur, se rencontre à l'étape 2 de chaque expédition dans le Marais des Pendus tant qu'il n'est pas au camp. Il est attaqué par un Noyé de niveau 13. Avant le combat :

> Au bord d'une mare noire, un vieil homme s'arc-boute sur sa canne. Ce qu'il vient de ferrer surgit de l'eau et se jette sur vous.

Après la victoire :

> Le vieil homme reprend son souffle : Ansgar, pêcheur, chassé de sa cabane par ce qui remonte des eaux du Marais. Il vous suit jusqu'au camp et y tend ses lignes.

La ligne est prête 2 h après avoir été lancée. Prises, sur un tirage :

| Prise | Chance | Détail |
|---|---|---|
| Monstre aquatique commun | 55 % | Noyé, Anguille des tourbières ou Sangsue géante, au hasard |
| Kelpie | 5 % | rare |
| Potion | 20 % | 1 potion, au coffre |
| Bourse noyée | 10 % | l'argent d'un coffre de la zone ouverte la plus avancée, au coffre |
| Pierre d'évolution | 5 % | Pierre de braise ou Pierre de lune, pour moitié, au coffre |
| Capsule d'habileté | 5 % | au coffre |

Un monstre pêché a un niveau tiré parmi ceux de la zone ouverte la plus avancée. Le joueur le garde ou le relâche.

Espèces, qui ne vivent qu'à la pêche, sans ressource ni évolution (hypothèses, gamedesign.md, Q91). Stats dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse. Aucune attaque nouvelle.

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Rareté | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|---|
| noye | Noyé | Eau, Spectre | offensif | 60 / 55 / 55 / 70 / 60 / 40 (340) | 95 | commune | Carapace, Sorcellerie | Plongeon (1), Lueur trompeuse (1), Déferlante (12), Frisson d'outre-tombe (18) |
| anguille-des-tourbieres | Anguille des tourbières | Eau, Ténèbres | offensif | 50 / 70 / 45 / 55 / 45 / 85 (350) | 95 | commune | Célérité, Traqueur | Plongeon (1), Morsure (1), Étreinte (8), Lame de fond (16) |
| sangsue-geante | Sangsue géante | Eau, Poison | soutien | 75 / 55 / 70 / 40 / 65 / 35 (340) | 92 | commune | Carapace, Instinct | Charge gluante (1), Morsure venimeuse (1), Croc de venin (15), Déferlante (20) |
| kelpie | Kelpie | Eau, Féérique | offensif | 75 / 85 / 65 / 65 / 60 / 70 (420) | 180 | rare | Férocité, Affinité | Plongeon (1), Ruade (1), Lame de fond (15), Trombe (25) |

Textes proposés sans validation (gamedesign.md, Q91). {durée} est le temps restant, {monstre} le nom du monstre suivi de "sauvage", {objet} l'objet remonté.

| Moment | Texte |
|---|---|
| Ligne lancée | Vous lancez la ligne. Elle sera prête à relever dans {durée}. |
| Monstre pêché | Au bout de la ligne : {monstre}, de niveau {niveau} ! |
| Question | Garder {monstre} ? Sinon, la prise retourne à l'eau. |
| Monstre relâché | {monstre} retourne à l'eau. |
| Objet pêché | Vous remontez {objet}. Vous la rangez dans le coffre. ("une potion", "une bourse noyée, qui contient 30 pièces d'argent", "une Pierre de lune", "une capsule d'habileté") |
| État de la ligne | Aucune ligne à l'eau. / La ligne est à l'eau : prête dans {durée}. / La ligne est prête à relever. |
| Au camp | La ligne d'Ansgar est prête à relever. |
| Message privé sur Discord | La ligne d'Ansgar est prête. Ouvrez /bersemon pour la relever. |
| Refus, équipe et camp pleins | Votre équipe et votre camp sont pleins : faites de la place avant de relever la ligne. |

Libellés : "Pêche d'Ansgar" (console), "Pêche" (Discord), "Lancer la ligne", "Relever la ligne", "Garder", "Relâcher".

## 16. Âmes d'Apôtres

Décidé le 20/10/2026, pour l'étape 66 (gamedesign.md, section 9.4), texte validé par l'utilisateur. Les Apôtres ne se capturent plus : vaincu, chacun laisse son âme, recueillie après l'épilogue.

> L'âme du Baron pendu s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

> L'âme de l'Abbesse écorchée s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

Textes proposés sans validation (gamedesign.md, Q92) : au camp, "Âmes d'Apôtres : {noms}." (par exemple "Âmes d'Apôtres : Baron pendu et Abbesse écorchée."), et au codex, "{nom} ({types}) : Apôtre vaincu".

## 17. Boosters et Philtre chromatique

Décidé le 20/10/2026, pour les étapes 67 à 69 (gamedesign.md, sections 5.8, 13.7 et 14.3). Noms et taux validés par l'utilisateur ; les identifiants, l'ordre des tirages et les textes sont des hypothèses (gamedesign.md, Q93).

Chaque booster ajoute 5 à la valeur interne de sa stat, sans dépasser 31, et se consomme. On ne les achète pas.

| Identifiant | Booster | Stat | Rang dans le tirage du coffre |
|---|---|---|---|
| fiole-de-sang | Fiole de sang | PV | 21 |
| moelle-de-bete | Moelle de bête | Attaque | 22 |
| ecorce-de-fer | Écorce de fer | Défense | 23 |
| encre-d-arcane | Encre d'arcane | Attaque spéciale | 24 |
| cendre-benite | Cendre bénite | Défense spéciale | 25 |
| poudre-de-vif-argent | Poudre de vif-argent | Vitesse | 26 |

Le tirage sur 100 du coffre décide ainsi de la capsule d'habileté (1 à 10), de la Pierre de braise (11 à 15), de la Pierre de lune (16 à 20) et d'un booster (21 à 26) ; rien de plus au-delà.

Le Philtre chromatique, appelé Éclat d'astre jusqu'au 25/10/2026 (nom choisi par l'utilisateur pour l'étape 108) : chaque Apôtre vaincu en laisse un, et la pêche en remonte 1 fois sur 100. Malvoise s'en sert pour rendre un monstre shiny. Il ne s'achète pas et ne se revend pas. Le fichier de sauvegarde et les menus Discord gardent son ancien nom (eclats_d_astre, eclats-d-astre).

Pêche, depuis l'étape 67 (section 15) : monstre aquatique commun 55 %, Kelpie 5 %, potion 14 %, bourse noyée 10 %, pierre d'évolution 5 %, capsule d'habileté 5 %, booster 5 % (puis un tirage parmi les six, dans l'ordre du tableau), Philtre chromatique 1 %.

Textes proposés sans validation (gamedesign.md, Q93), sauf ceux du Philtre chromatique, réécrits et validés par l'utilisateur le 25/10/2026. {monstre} est le nom du monstre, {stat} le nom de la stat, {booster} celui du booster.

| Moment | Texte |
|---|---|
| Effet d'un booster, dans les listes | +5 à la valeur interne de {stat}, 31 au plus. |
| Booster donné | {monstre} : valeur interne de {stat} de {avant} à {après}. |
| Refus, valeur déjà au maximum | La valeur interne de {stat} de {monstre} est déjà à 31. |
| Refus, pas de booster | Vous n'avez pas de {booster}. |
| Philtre laissé par un Apôtre, après son âme | Dans la poussière de sa dépouille roule une fiole aux reflets changeants : un Philtre chromatique. Vous le ramassez. |
| Objet pêché | Vous remontez une Fiole de sang. Vous la rangez dans le coffre. / Vous remontez une fiole scellée : un Philtre chromatique. Vous le rangez dans le coffre. |
| Service, à l'étal | Rendre un monstre shiny (1 Philtre chromatique) |
| Confirmation | Faire rendre {monstre} shiny par Malvoise contre un Philtre chromatique ? Ses valeurs internes passeront toutes à 31. |
| Monstre rendu shiny | Malvoise fait boire le Philtre chromatique à {monstre} ; sa lignée s'altère, et sa robe prend une couleur nouvelle. {monstre} est désormais shiny. |
| Refus, pas de Philtre | Il vous faut un Philtre chromatique. |
| Refus, déjà shiny | {monstre} est déjà shiny. |

## 18. Amis et palmarès des duels

Décidé le 20/10/2026, pour les étapes 70 et 71 (gamedesign.md, section 15.4). Textes proposés sans validation (gamedesign.md, Q94). {bilan} est le nombre de victoires puis de défaites, au singulier jusqu'à 1 : "2 victoires, 1 défaite", "0 victoire, 0 défaite".

| Moment | Texte |
|---|---|
| Palmarès, en tête de l'écran "Autres joueurs" | Vos duels : {bilan}. |
| Joueur choisi | Duels de {héros} : {bilan}. |
| Ami, dans le menu des joueurs | description de l'option : "Ami - {pseudo}", ou "Ami" sans pseudo connu |
| Boutons | Ajouter aux amis / Retirer des amis / Classement / Retour aux autres joueurs |
| Classement | Classement des duels : puis une ligne par joueur, "1. {héros} ({pseudo}) : {bilan}", 10 au plus |
| Classement vide | Aucun duel n'a encore été livré. |
| Refus, rien à changer | Rien à changer à vos amis pour l'instant. |

Un ami ajouté ou retiré n'a pas de récit : l'écran garde le joueur choisi, dont le bouton passe de "Ajouter aux amis" à "Retirer des amis", ou l'inverse.

## 19. Machines du camp

Décidé le 20/10/2026, pour les étapes 72 à 74 (gamedesign.md, section 13.6), valeurs validées par l'utilisateur, à ajuster en jouant. Construites par Gunther, comme les améliorations (section 9).

| Machine | Identifiant | Niveau | Effet | Coût | Durée |
|---|---|---|---|---|---|
| Pièges | pieges | 1 | 1 ressource toutes les 2 h, stock de 6 | 5 Croc de loup, 5 Écaille de vipère | 1 h |
| | | 2 | 1 ressource par heure, stock de 12 | 6 Cendre de feu follet, 6 Fragment d'armure | 3 h |
| | | 3 | 1 ressource par heure, stock de 24, une ressource rare de la zone 1 fois sur 10 | 6 Aile de fée noire, 6 Os de squelette, 1 Écaille de vouivre | 6 h |
| Cour d'entraînement | cour | 1 | 1 place, 50 XP par heure | 6 Croc de loup, 6 Plume de corbeau | 2 h |
| | | 2 | 2 places, 50 XP par heure | 6 Fragment d'armure, 6 Os de squelette | 4 h |
| | | 3 | 3 places, 100 XP par heure | 8 Éclat de gargouille, 6 Dent de goule | 8 h |

Ressources des pièges, par zone : celles des espèces communes de la zone, au hasard ; au niveau 3, la ressource rare de la zone 1 fois sur 10 (Croc de basilic dans le Marais, Écaille de vouivre dans la Vallée, Crin de chimère dans l'Abbaye ; aucune dans la Lisière, dont le Cerf blanc ne lâche rien).

Plafond de l'entraînement : l'XP du niveau maximal de la zone ouverte la plus avancée (7 avec la seule Lisière, 13 avec le Marais, 20 avec la Vallée, 27 avec l'Abbaye).

Textes proposés sans validation (gamedesign.md, Q95) :

| Moment | Texte |
|---|---|
| Libellés | Machines ; Pièges ; Cour d'entraînement ; Poser les pièges dans {zone} ; Relever les pièges ; Mettre un monstre à l'entraînement ; Reprendre {monstre} |
| État des pièges | Pièges (niveau {n}, {effet}) dans {zone} : {stock}/{maximum} ressources, une de plus dans {durée}. / ..., pleins. / Pièges (niveau {n}, {effet}) : posés nulle part. |
| Au camp | Les pièges de Gunther sont pleins. |
| Pièges posés | Gunther pose ses pièges dans {zone}. |
| Relève | Vous relevez les pièges : {ressources}. Elles vont dans le coffre. |
| Pièges vides | Les pièges sont vides. |
| État de la cour | Cour d'entraînement (niveau {n}, {effet}), jusqu'au niveau {niveau} : {occupées}/{places} places occupées. Puis, pour chaque monstre, "{monstre} : {xp} XP gagnée depuis {durée}". |
| Monstre placé | {monstre} rejoint la cour d'entraînement. |
| Monstre repris | {monstre} quitte la cour d'entraînement avec {xp} XP. |
| Refus | Aucune zone explorée. / La cour est pleine. / Ce monstre est déjà à son plafond d'entraînement. |
| Accès direct (étape 109) | Cour d'entraînement ; Retour à la gestion ; Retour à la cour ; Gunther n'a pas encore construit de cour d'entraînement. |

## 20. Rôle des monstres et protections

Décidé le 20/10/2026, pour les étapes 75 à 78 (gamedesign.md, sections 4.2, 12.5, 12.6 et 13.5), valeurs validées par l'utilisateur.

| Rôle | En combat, monstres du joueur | En expédition automatique |
|---|---|---|
| soutien | +10 % de Défense et de Défense spéciale | 25 % de protection au monstre protégé |
| offensif | +10 % d'Attaque et d'Attaque spéciale | +10 % d'argent s'il rentre debout d'une réussite |

Charme de garde (identifiant charme-de-garde) : 50 % de protection au monstre protégé, un par expédition. Fabriqué par Gunther, un par travail, contre 3 Os de squelette et 3 Fragment d'armure, en 30 minutes. Il ne se revend pas.

Textes proposés sans validation (gamedesign.md, Q96) :

| Moment | Texte |
|---|---|
| Préparation | Monstre protégé : {monstre}, à {p} % (soutien 25 % chacun, charme 50 %). / Aucun monstre protégé. |
| Libellés | Protéger {monstre} ; Sans protection ; Avec un charme de garde ; Sans charme ; Fabriquer un charme de garde |
| Bilan | dans le bilan d'une expédition : "{monstre}, protégé à {p} %, subit {dégâts}". Bonus d'argent des offensifs : "dont {argent} grâce aux monstres offensifs". |
| Fiche d'un monstre | Rôle : soutien (+10 % de Défense et de Défense spéciale en combat). / Rôle : offensif (+10 % d'Attaque et d'Attaque spéciale en combat). |

## 21. Col du Givre-Noir, troisième Apôtre et stades 3

Décidés le 20/10/2026, pour les étapes 79 à 82 (gamedesign.md, sections 5.3, 5.4, 9.1 et 9.4), contenu proposé par Claude et validé par l'utilisateur. Le Porte-étendard a l'IA relevée de l'Inquisiteur ; la répartition des tirages suit celle de l'Abbaye (hypothèses, gamedesign.md, Q97).

Zone :

| Identifiant | Nom | Niveaux | Taille des groupes | Espèces | Monstres rares | Humain | Apôtre |
|---|---|---|---|---|---|---|---|
| col-du-givre-noir | Col du Givre-Noir | 28 à 36 | 1 à 3 | Wendigo, Troll des cavernes, Harpie des orages, Gargouille, Warg, Chevalier noir | Wyrm de givre (hostile), Sylphe des neiges (pacifique) | Porte-étendard | Margrave de givre |

Le Warg et le Chevalier noir sont des formes évoluées sauvages, une exception propre à cette zone (gamedesign.md, section 5.4) : ils lâchent la ressource de leur forme de base (Croc de loup, Fragment d'armure). Pas de marchand mort.

Expédition automatique : durée de base 5 h, dégâts 220 à 450, XP 2200 à 3400, potions 1 à 3, argent 180 à 320. Coffres : 1 à 2 potions et 140 à 220 pièces d'argent ; capsules Blizzard (500), Séisme (450) et Onde de choc (450).

Espèces (stats dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse) :

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Rareté | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|---|
| wendigo | Wendigo | Glace, Spectre | offensif | 60 / 85 / 50 / 45 / 50 / 70 (360) | 110 | commune | Férocité, Traqueur | Croc de givre (1), Charge spectrale (1), Frisson d'outre-tombe (20), Avalanche (30) |
| troll-des-cavernes | Troll des cavernes | Sol, Roche | soutien | 90 / 85 / 80 / 30 / 50 / 30 (365) | 112 | commune | Carapace, Volonté | Griffe de pierre (1), Secousse (1), Chute de pierres (22), Éboulement (32) |
| harpie-des-orages | Harpie des orages | Vol, Électrique | offensif | 55 / 50 / 45 / 80 / 50 / 80 (360) | 108 | commune | Célérité, Éclaireur | Coup de bec (1), Étincelle (1), Bourrasque (20), Foudre (30) |
| wyrm-de-givre | Wyrm de givre | Dragon, Glace | offensif | 80 / 85 / 70 / 85 / 65 / 60 (445) | 195 | rare, hostile | Affinité, Férocité | Souffle draconique (1), Croc de givre (1), Souffle glacial (20), Furie draconique (30) |
| sylphe-des-neiges | Sylphe des neiges | Glace, Féérique | soutien | 65 / 45 / 60 / 95 / 90 / 90 (445) | 190 | rare, pacifique | Volonté, Instinct | Poussière d'elfe (1), Souffle glacial (1), Lumière d'aube (15), Rayon de lune (30) |
| margrave-de-givre | Margrave de givre | Obscur | offensif | 95 / 100 / 90 / 95 / 85 / 65 (530) | 300 | Apôtre, niveau 38 | Férocité, Carapace | Charge spectrale (1), Taille d'acier (10), Étreinte obscure (18), Blizzard (25), Hurlement des abysses (30), Linceul de givre (36) |

Au codex, ces espèces viennent après les monstres aquatiques, dans l'ordre du tableau, puis les formes de stade 3.

Attaques nouvelles :

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| croc-de-givre | Croc de givre | Glace | P | 60 | 100 | paralysie 10 % |
| avalanche | Avalanche | Glace | P | 85 | 90 | |
| souffle-glacial | Souffle glacial | Glace | S | 70 | 100 | |
| blizzard | Blizzard | Glace | S | 90 | 85 | paralysie 10 % |
| secousse | Secousse | Sol | P | 60 | 100 | |
| seisme | Séisme | Sol | P | 90 | 90 | |
| eboulement | Éboulement | Roche | P | 90 | 85 | |
| etincelle | Étincelle | Électrique | S | 45 | 100 | paralysie 10 % |
| foudre | Foudre | Électrique | S | 90 | 85 | paralysie 10 % |
| onde-de-choc | Onde de choc | Électrique | S | 75 | 100 | paralysie 20 % |
| linceul-de-givre | Linceul de givre | Obscur | S | 95 | 90 | paralysie 20 % |
| souffle-de-cendre | Souffle de cendre | Feu | S | 95 | 90 | |
| deluge-toxique | Déluge toxique | Poison | P | 90 | 90 | poison 30 % |
| racines-du-gibet | Racines du gibet | Spectre | P | 85 | 95 | paralysie 20 % |
| griffes-spectrales | Griffes spectrales | Spectre | P | 85 | 95 | |
| dechiquetage | Déchiquetage | Combat | P | 90 | 90 | |

Ressources : Poil de wendigo (poil-de-wendigo), Pierre de troll (pierre-de-troll) et Plume de harpie (plume-de-harpie), 5 pièces d'argent chacune, Écaille de wyrm (ecaille-de-wyrm), 15. Ressource rare des pièges dans cette zone : l'Écaille de wyrm.

Le Porte-étendard attend au bout du Col avec un Wendigo de niveau 36, un Troll des cavernes de niveau 36 et un Wyrm de givre de niveau 37 (valeurs internes à 15, première habileté de leur espèce). Victoire : 400 pièces d'argent, puis l'indice 6 au retour.

Textes :

> Indice 5 : sous l'autel de l'Abbesse, une plaque de fer gravée du même sceau : au nord, le Col du Givre-Noir, où un margrave a mené son armée mourir dans la neige. Un troisième Apôtre y règne.

> Indice 6 : le porte-étendard vaincu a parlé : le Margrave attend au sommet, dans la forteresse où ses soldats ont gelé à sa place.

Porte-étendard, avant le combat :

> Au milieu du col, un soldat couvert de givre tient encore l'étendard d'une armée morte. Trois bêtes sortent de la tempête à son signal.

Après la victoire :

> Le porte-étendard tombe à genoux ; l'étendard gelé se brise dans la neige.

Margrave de givre, avant le combat :

> Au sommet, sur un trône de glace entouré de soldats figés, le Margrave de givre se lève. Apôtre du Démiurge, il a offert son armée au froid. Il n'y aura pas de fuite.

Épilogue :

> Le Margrave de givre se brise sur son trône. Le même sceau, une main ouverte, est gravé dans la glace. Trois Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.

Âme, sur le modèle de la section 16 (texte proposé sans validation, Q97) :

> L'âme du Margrave de givre s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

Stades 3, au niveau 36 (gamedesign.md, section 5.4). Chacun garde le rôle, la rareté et les habiletés de sa lignée, n'a pas de ressource, et son rendement d'XP vaut 1,4 fois celui de la forme précédente, arrondi à l'entier inférieur. Ses attaques sont celles de la forme précédente, puis l'attaque d'évolution et l'autre attaque au niveau 36.

| Forme | Identifiant | Évolution | Types | Stats de base (total) | Rendement d'XP | Attaque d'évolution | Autre attaque, au même niveau |
|---|---|---|---|---|---|---|---|
| Lézard de braise | wyverne-de-cendre | Wyverne de cendre | Feu, Dragon | 75 / 68 / 68 / 110 / 80 / 90 (491) | 120 | Souffle de cendre | Furie draconique |
| Crapaud géant | crapaud-roi | Crapaud-roi | Eau, Poison | 90 / 105 / 85 / 55 / 70 / 75 (480) | 123 | Déluge toxique | Croc de venin |
| Mandragore ancienne | arbre-pendu | Arbre-pendu | Plante, Spectre | 110 / 70 / 110 / 70 / 100 / 30 (490) | 124 | Racines du gibet | Frisson d'outre-tombe |
| Chat des ombres | cait-sith | Cait sith | Ténèbres, Spectre | 65 / 112 / 55 / 65 / 58 / 135 (490) | 120 | Griffes spectrales | Charge spectrale |
| Warg | loup-garou | Loup-garou | Combat, Ténèbres | 85 / 110 / 75 / 45 / 65 / 100 (480) | 137 | Déchiquetage | Griffe d'ombre |

Le Dragon de feu (stade 4, niveau 70) et le Dragon des brumes (Vouivre, niveau 45) restent seulement nommés.

## 22. Achat et location de monstres

Décidés le 20/10/2026, pour les étapes 83 à 86 (gamedesign.md, section 14.2), règles et prix choisis par l'utilisateur ; textes proposés sans validation (gamedesign.md, Q98).

- Achat : chaque espèce commune sauvage, à sa forme de base, des zones menées au bout, au niveau minimal de la zone où elle se rencontre (le plus bas si elle vit dans plusieurs zones). Prix : 4 fois le prix de revente de l'espèce à ce niveau, hors shiny. Exemples : Loup de niveau 3, 68 ; Feu follet de niveau 8, 204 ; Gargouille de niveau 21, 588 ; Wendigo de niveau 28, 804.
- Location : chaque espèce rare des zones menées au bout, Schnoz excepté, au niveau maximal de la zone ouverte la plus avancée, pour 3 expéditions manuelles. Prix : le prix de revente de l'espèce à ce niveau, hors shiny. Exemples, avec la Vallée ouverte (niveau 20) : Cerf blanc 320, Basilic 320 ; avec le Col (niveau 36) : Wyrm de givre 640.

| Moment | Texte |
|---|---|
| Libellés | Acheter ou louer ; Acheter un monstre ; Louer un monstre ; Rendre {monstre} à Malvoise |
| Offre d'achat | {espèce} de niveau {niveau} : {prix} |
| Offre de location | {espèce} de niveau {niveau}, pour 3 expéditions : {prix} |
| Achat | Malvoise ouvre une cage contre {prix} : {monstre} rejoint l'équipe. / ... rejoint le camp. |
| Location | Malvoise vous confie {monstre} pour 3 expéditions, contre {prix}. {monstre} rejoint l'équipe. / ... rejoint le camp. |
| Étal, avec un monstre loué | Monstre loué : {monstre}, encore {n} expéditions. (au singulier pour 1) |
| Fiche d'un monstre loué | Loué à Malvoise : encore {n} expéditions. (au singulier pour 1) |
| Vente, monstre loué | {monstre} (niv. {niveau}) : loué, ne se vend pas |
| Fin de location, au retour au camp | {monstre} retourne chez Malvoise : sa location est finie. |
| Rendu plus tôt | {monstre} retourne chez Malvoise. |
| Confirmation, rendre | Rendre {monstre} à Malvoise ? La location n'est pas remboursée. |
| Écran du bot | Malvoise vend des monstres communs des zones que vous avez explorées, et loue des monstres rares pour 3 expéditions : un monstre loué ne fait que combattre, puis repart. Vous avez {argent}. |
| Refus | Vous avez déjà un monstre loué. / Un monstre loué ne peut pas faire cela. / L'équipe et le camp sont pleins. / Malvoise ne propose pas ce monstre pour l'instant. / Vous n'avez pas de monstre loué. / Malvoise n'a encore aucun monstre à vendre : menez une expédition au bout. / Malvoise n'a encore aucun monstre à louer. |

## 23. Béhérit pourpre, Éclipse et Démiurge

Décidés le 21/10/2026, pour les étapes 88 à 92 (gamedesign.md, section 9.4), contenu proposé par Claude et validé par l'utilisateur.

Le Démiurge a deux formes, au niveau 42, à valeurs internes fixes de 31 ; la seconde se relève quand la première tombe KO. Une rareté à part, "Démiurge" ; elles n'ont pas de ressource et ne se capturent pas. Au codex, elles viennent après les stades 3.

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|
| demiurge | Démiurge | Obscur | offensif | 95 / 100 / 90 / 100 / 90 / 85 (560) | 320 | Férocité, Volonté | Étreinte obscure (1), Hurlement des abysses (10), Flagellation (20), Marque du sacrifice (40) |
| demiurge-eveille | Démiurge éveillé | Obscur | offensif | 110 / 115 / 100 / 120 / 100 / 95 (640) | 380 | Sorcellerie, Carapace | Hurlement des abysses (1), Psaume impie (1), Marque du sacrifice (1), Soleil noir (1) |

Attaques nouvelles :

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| marque-du-sacrifice | Marque du sacrifice | Obscur | P | 95 | 95 | poison 30 % |
| soleil-noir | Soleil noir | Obscur | S | 100 | 90 | paralysie 20 % |

L'Éclipse (identifiant eclipse) : une expédition d'une seule étape, le Démiurge, ouverte par le Béhérit. Dans les menus : "Éclipse", avec "Niveau 42 : le Démiurge".

Textes validés :

> Malvoise dépose les trois âmes au creux de ses mains. Elles se tordent, se mêlent et durcissent en un œuf de pierre pourpre, où des yeux et une bouche s'ouvrent et se referment. "Le Béhérit pourpre. Il t'ouvrira la porte du Démiurge, mais il réclamera son dû."

> Le Béhérit pourpre s'ouvre en un cri. {monstre} vous est arraché et disparaît dans une lumière rouge. Le ciel s'assombrit : l'Éclipse commence.

Avant le combat :

> Sous un soleil noir, la terre s'ouvre sur un abîme de chair. Une silhouette ailée descend lentement : le Démiurge, maître des Apôtres. Il n'y aura pas de fuite.

Quand la première forme tombe :

> Le Démiurge s'effondre, puis sa chair se déchire : sa forme véritable s'en arrache, immense.

Épilogue :

> Le Démiurge se disloque dans un hurlement qui fait trembler le ciel. Le soleil noir pâlit, et le jour revient sur le royaume. Votre vengeance est accomplie ; mais d'autres choses rôdent encore au bord des bois.

> Dans les cendres du Démiurge reposent trois Philtres chromatiques, aux reflets changeants. Vous les ramassez. (Réécrit le 25/10/2026, étape 108.)

Textes proposés sans validation (gamedesign.md, Q99) :

| Moment | Texte |
|---|---|
| Au camp, avec le Béhérit, à la place des âmes d'Apôtres | Béhérit pourpre : il attend son sacrifice. / Béhérit pourpre : il vous ouvre l'Éclipse. / Béhérit pourpre : l'Éclipse s'est refermée. |
| Étal, menu des âmes | Âmes des Apôtres : forger le Béhérit pourpre |
| Confirmation, Béhérit | Donner à Malvoise les âmes des trois Apôtres pour forger le Béhérit pourpre ? Elles seront perdues. |
| Écran du sacrifice | Le Béhérit pourpre réclame un sacrifice pour ouvrir l'Éclipse : choisissez un de vos monstres de niveau 30 ou plus. Il sera perdu pour toujours. |
| Confirmation, sacrifice | Sacrifier {monstre} au Béhérit pourpre ? Il sera perdu pour toujours. |
| Codex | {nom} ({types}) : vaincu |
| Libellés | Forger le Béhérit pourpre ; Sacrifier ; Partir vers l'Éclipse |
| Refus | Il vous faut les âmes des trois Apôtres. / Le Béhérit pourpre est déjà forgé. / Il vous faut le Béhérit pourpre. / Le Béhérit a déjà reçu son sacrifice. / Le Béhérit réclame un monstre de niveau 30 ou plus. / Ce sacrifice laisserait votre équipe vide. / L'Éclipse est fermée : le Béhérit n'a pas reçu son sacrifice, ou le Démiurge est vaincu. |

## 24. Types Lumière et Son

Décidés le 24/10/2026, pour les étapes 93 à 95 (gamedesign.md, sections 5.2 et 5.3), contenu proposé par Claude et validé par l'utilisateur. Le codex, la ressource et la place des espèces dans les tirages sont des hypothèses (gamedesign.md, Q100).

Les deux espèces vivent dans l'Abbaye des Cendres (section 10) : la Cloche hurlante parmi les espèces communes, le Séraphin de vitrail parmi les monstres rares, avec la Chimère et l'Ange déchu. Au codex, elles viennent après les deux formes du Démiurge, dans l'ordre du tableau.

Espèces (stats dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse) :

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Rareté | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|---|
| cloche-hurlante | Cloche hurlante | Acier, Son | soutien | 65 / 50 / 80 / 65 / 75 / 25 (360) | 108 | commune | Carapace, Volonté | Coup de garde (1), Glas (1), Tocsin (16), Carillon funèbre (24) |
| seraphin-de-vitrail | Séraphin de vitrail | Lumière | offensif | 65 / 50 / 60 / 105 / 80 / 60 (420) | 180 | rare, pacifique | Sorcellerie, Volonté | Lueur sacrée (1), Lumière d'aube (12), Lance de lumière (18), Aube éclatante (24) |

Attaques nouvelles :

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| glas | Glas | Son | S | 50 | 100 | |
| tocsin | Tocsin | Son | S | 75 | 100 | paralysie 10 % |
| carillon-funebre | Carillon funèbre | Son | S | 90 | 85 | paralysie 10 % |
| lueur-sacree | Lueur sacrée | Lumière | S | 50 | 100 | |
| lance-de-lumiere | Lance de lumière | Lumière | P | 80 | 95 | |
| aube-eclatante | Aube éclatante | Lumière | S | 90 | 90 | |

Pas de capsule : une capsule n'enseigne qu'aux monstres de son type, et chaque type n'a qu'une espèce, qui apprend déjà ces attaques. La Lance de lumière, d'abord prévue en capsule, est apprise par le Séraphin au niveau 18.

Ressource : Éclat de bronze (eclat-de-bronze), lâché par la Cloche hurlante, 5 pièces d'argent. Le Séraphin, pacifique, n'en a pas.

## 25. Tour sans fond

Décidée le 24/10/2026, pour les étapes 96 à 102 (gamedesign.md, sections 9.5 et 15.5).

Lieu : la Tour sans fond (identifiant tour-sans-fond), hors des zones de la carte. Niveau des monstres à l'étape n : 4 + 2 x n, 100 au plus. Groupes de 1 à 3 monstres seul, de 2 à 4 à deux. Gardien aux étapes multiples de 10, au niveau de l'étape + 3. Coffres : 1 à 2 potions, de 5 x n à 8 x n pièces d'argent.

Textes validés :

> Au milieu des ruines se dresse le sommet d'une tour engloutie. Ses marches s'enfoncent dans la terre, et nul n'en a trouvé le fond ; plus on descend, plus les bêtes sont terribles.

Gardien, avant le combat :

> Les marches débouchent sur un palier plus large que les autres. Au centre, {monstre} garde l'escalier qui s'enfonce plus bas. Il n'y aura pas de fuite.

Repos, après la victoire sur le gardien :

> Derrière le gardien, une source claire coule entre les pierres. Vos monstres y boivent et reprennent toutes leurs forces.

Textes proposés sans validation (gamedesign.md, Q101) :

| Moment | Texte |
|---|---|
| Retour au camp, nouveau record | Nouveau record dans la Tour sans fond : étape {n}. |
| Carte de la console | Tour sans fond (sans fin) : record, étape {n} / Tour sans fond (sans fin) : descendre le plus loin possible |
| Menu des expéditions du bot | Sans fin ; record, étape {n} / Sans fin ; descendre le plus loin possible |
| Compteur et récit | [Étape : {n}] ; Étape {n}. |
| Classement des records | Records de la Tour sans fond. / Vos records : seul, étape {n} ; à deux, aucun. / Seul : / À deux : / Aucun record pour l'instant. ; boutons Records de la Tour, Classement des duels |
| Message privé, invitation reçue | {héros} ({pseudo}) vous invite à descendre dans la Tour sans fond. Ouvrez /bersemon, au camp, pour répondre. |
| Message privé, invitation acceptée ou refusée | {héros} a accepté votre invitation : la descente commence. Ouvrez /bersemon. / {héros} a refusé votre invitation dans la Tour sans fond. |
| Écran des autres joueurs | {héros} vous invite à descendre dans la Tour sans fond. / Vous avez invité {héros} dans la Tour sans fond. En attente de sa réponse. ; boutons Inviter dans la Tour, Accepter l'invitation, Refuser l'invitation, Retirer l'invitation |
| Écrans de la descente | Descente avec {héros}. / En attente de {héros}. / Vous voulez continuer : vous attendez {héros}. / {héros} veut continuer. ; boutons Actualiser, Rentrer au camp, Ramener le groupe au camp |
| Refus | Vous ne pouvez pas vous inviter vous-même. / Vous avez déjà un défi ou une invitation en attente : un seul à la fois. / Ce joueur a déjà un défi ou une invitation en attente : réessayez plus tard. / Cette invitation n'est plus en attente. / Un des deux joueurs est déjà en duel ou en expédition. / Les deux joueurs doivent être au camp pour descendre ensemble : réessayez plus tard. / Chacun doit avoir ouvert la Tour sans fond, en menant la Lisière au bout. / C'est à votre compagnon d'agir. / Pas encore : votre compagnon a 5 minutes pour jouer. |

## 26. Filets de capture

Décidés le 25/10/2026, pour les étapes 103 à 106 (gamedesign.md, sections 9.2, 11.2, 13.5 et 14.2), valeurs proposées par Claude et validées par l'utilisateur, à ajuster en jouant.

| Identifiant | Nom | Chance de capture | Prix chez Malvoise | Fabrication par Gunther, par lot de 5 | Durée |
|---|---|---|---|---|---|
| filet-de-chanvre | Filet de chanvre | normale (x1) | 10 | 3 Croc de loup, 3 Plume de corbeau | 30 min |
| filet-leste | Filet lesté | x1,5 | 30 | 3 Fragment d'armure, 3 Os de squelette | 1 h |
| filet-de-chaines | Filet de chaînes | x2 | ne s'achète pas | 3 Éclat de gargouille, 2 Pierre de troll | 2 h |

Départ : 10 filets de chanvre dans le sac. Sauvegarde antérieure à la version 27 : 10 filets de chanvre au coffre.

Coffres d'expédition : dans le tirage sur 100 qui décide de la capsule d'habileté, des pierres et des boosters, 27 à 41 donnent 2 filets de chanvre, 42 à 46 un filet lesté.

Textes proposés sans validation (gamedesign.md, Q102) :

| Moment | Texte |
|---|---|
| Lancer | Vous lancez un filet de chanvre sur {monstre} ({n} % de chances). |
| Combat, console | Lancer un filet (10 filets de chanvre, 2 filets lestés) ; Quel filet lancer ? ; Filet de chanvre (10) : 45 % de chances ; Vous n'avez plus de filet. |
| Combat, Discord | Filet de chanvre x10 (45 %) |
| Sac et coffre | 10 filets de chanvre, 1 filet lesté, 2 filets de chaînes |
| Refus | Plus aucun filet de ce genre. / Vous n'avez plus de filet pour capturer. |
| Achat chez Malvoise | Vous achetez 2 filets de chanvre pour {prix}. Ils vont dans le coffre. |
| Refus d'achat | Malvoise ne vend pas ce filet : seul Gunther le forge. |
| Forge, question | Faire forger 5 filets lestés (chance de capture x1,5) ? Gunther prendra {coût}, et aura fini dans {durée}. Un travail lancé ne s'annule pas. |
| Forge, lancement | Vous remettez à Gunther {coût}. Il se met à forger 5 filets lestés. Il aura fini dans {durée}. |
| Forge, constat | Gunther a fini : 5 filets lestés. Ils vont dans le coffre. |
| Étal de Malvoise | Filets en vente : Filet lesté (chance de capture x1,5) : 30 pièces d'argent ; Acheter des filets ; Filets en vente (vous en avez 10 filets de chanvre) : |
| Forge, menu | Fabriquer des filets ; Vous avez 13 filets de chanvre, 2 filets lestés, entre votre sac et le coffre. ; 5 filets lestés (chance de capture x1,5) : {coût}, 1 h |
| Coffre | Prendre des filets ; Ranger des filets ; Il n'y a aucun filet. |
| Étal, Discord | Acheter 5 filets de chanvre (50 pièces d'argent) ; Acheter 5 filets lestés (150 pièces d'argent) |
| Forge, Discord | Menu "Fabriquer des filets" : 5 filets lestés (chance de capture x1,5), avec en description {coût}, {durée} ; vous en avez {n} |
| Camp, Discord | 17 sortes de ressources (à la place du détail des ressources du sac et du coffre) |

## 27. Carnet de traque

Décidé le 25/10/2026, pour l'étape 107 (gamedesign.md, section 5.6), textes validés par l'utilisateur.

Titre : "Carnet de traque :". Une entrée numérotée par victoire, dans l'ordre des victoires :

| Ennemi abattu | Entrée |
|---|---|
| Baron pendu | Baron pendu, abattu dans la Vallée des Brumes. |
| Abbesse écorchée | Abbesse écorchée, abattue dans l'Abbaye des Cendres. |
| Margrave de givre | Margrave de givre, abattu dans le Col du Givre-Noir. |
| Démiurge (ses deux formes) | Démiurge, abattu dans l'Éclipse. |

## 28. Rite du sang mêlé

Décidé le 25/10/2026, pour les étapes 110 à 112 (gamedesign.md, section 13.11). Nom et valeurs choisis par l'utilisateur, à ajuster en jouant.

| Valeur | |
|---|---|
| Durée | 2 h réelles |
| Coût | 3 ressources de l'espèce du petit et 50 pièces d'argent (l'argent seul pour une espèce qui ne lâche pas de ressource) |
| Niveau du petit | 1 |
| Valeurs internes du petit | pour chaque stat, la plus haute des deux parents |
| Shiny du petit | 1 chance sur 512, comme un monstre sauvage |

Textes proposés sans validation (gamedesign.md, Q105). {a} et {b} sont les parents, {petit} le nom du petit, {espèce} son espèce, {coût} les ressources et l'argent, {durée} le temps restant.

| Moment | Texte |
|---|---|
| Service, à l'étal | Rite du sang mêlé : 3 ressources de l'espèce du petit et 50 pièces d'argent, 2 h |
| Confirmation | Confier {a} et {b} à Malvoise pour le Rite du sang mêlé ? Il prendra {coût}. Dans 2 h, le petit naîtra, de l'espèce {espèce}, au niveau 1, avec la meilleure valeur interne de ses parents pour chaque stat. D'ici là, les deux restent auprès de Malvoise. |
| Rite lancé | Malvoise trace un cercle de sang autour de {a} et {b}, et entonne le Rite du sang mêlé. Le petit naîtra dans {durée}. |
| État, à l'étal | Rite du sang mêlé : {a} et {b}, achevé dans {durée}. / ..., achevé, le petit attend. |
| Au camp | Le Rite du sang mêlé est achevé : le petit attend auprès de Malvoise. |
| Naissance | Le cercle de sang se referme et le petit en naît : {petit}, au niveau 1. {a} et {b} vous rejoignent. |
| Refus | Malvoise accomplit déjà un rite : un seul à la fois. / Le rite n'unit que deux monstres d'une même famille d'évolution. / Il faut une place libre au camp pour le petit. / Il vous manque {ressources}. / Aucun rite n'est en cours. / Le rite n'est pas encore achevé. |
