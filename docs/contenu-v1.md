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
- Mandragore, des attaques des deux catégories et deux effets de statut, pour user l'adversaire : Coup de racine (niv. 1), Feuille tranchante (niv. 3), Secousse (niv. 7), Ronces (niv. 10), Spores putrides (niv. 14). Depuis l'étape 117 (gamedesign.md, section 5.3), Secousse remplace le Cri strident au niveau 7, pour la Mandragore comme pour la Mandragore ancienne et l'Arbre-pendu, qui suivent cette liste.
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
| baron-pendu | Baron pendu | Obscur | offensif | 85 / 85 / 75 / 90 / 80 / 65 (480) | 250 | 20, le maximum de la Vallée des Brumes, depuis l'étape 117 ; 22 avant |

Attaques apprises : Corde du gibet (niv. 1), Lamentation (niv. 5), Sentence (niv. 10), Trappe du gibet (niv. 15), Étreinte obscure (niv. 18), Hurlement des abysses (niv. 21). Au niveau 20, il connaît Lamentation, Sentence, Trappe du gibet et Étreinte obscure : depuis l'étape 117, Hurlement des abysses ne lui sert plus (gamedesign.md, section 9.4). Ses valeurs internes sont fixes, à 31 pour chaque stat (section 9.4).

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

Ajouté à l'étape 30, le 19/10/2026 (gamedesign.md, section 10.7). Le Bourreau attend au bout du Marais, avec une Armure hantée de niveau 13, un Feu follet de niveau 13 et un Basilic de niveau 14 (valeurs internes à 15, première habileté de leur espèce) ; jusqu'à l'étape 117, aux niveaux 14, 14 et 15. Victoire : 150 pièces d'argent, puis l'indice 2 au retour.

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
| Sac | sac | 1 | sac de 30 à 45 places (étape 125) | 6 Plume de corbeau, 6 Écaille de vipère | 1 h |
| | | 2 | de 45 à 60 places | 6 Cendre de feu follet, 6 Fragment d'armure | 3 h |
| | | 3 | de 60 à 80 places | 6 Aile de fée noire, 6 Os de squelette | 6 h |

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

Au camp, le bouton s'appelle "Échanges", puis "Autres joueurs" depuis l'étape 60 (section 14), puis "Multijoueur" depuis l'étape 150 ; l'écran commence alors par "Multijoueur : offres de monstres et duels." Limite d'obéissance : niveau maximal de la zone la plus avancée ouverte, plus 5 (12, 18, 25 puis 32).

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

Le Dragon de feu (stade 4, niveau 70) et le Dragon des brumes (Vouivre, niveau 45) restent seulement nommés. Le Dragon des brumes est codé à l'étape 143 (section 40).

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

Le Démiurge a deux formes, au niveau 42, à valeurs internes fixes de 31 ; la seconde se relève quand la première tombe KO. Depuis l'étape 162 (27/10/2026), combat final au niveau 100, après les 13 Apôtres, aux stats de base de 120 / 130 / 110 / 130 / 110 / 100 (700) et 135 / 145 / 120 / 150 / 120 / 110 (780), rendements d'XP 420 et 480 ; valeurs provisoires, à mesurer au dernier lot (gamedesign.md, Q118). Depuis l'étape 169, mesurées : 85 / 93 / 79 / 93 / 79 / 71 (500) et 97 / 104 / 86 / 108 / 86 / 79 (560), aux proportions d'avant (section 49 ; equilibrage.md, section 19). Dans les menus : "Niveau 100 : le Démiurge". Les textes du Béhérit ne disent plus "trois" âmes : "Malvoise dépose les âmes des Apôtres au creux de ses mains", "Donner à Malvoise les âmes des Apôtres pour forger le Béhérit pourpre ? Elles seront perdues.", "Il vous faut les âmes des 13 Apôtres." Une rareté à part, "Démiurge" ; elles n'ont pas de ressource et ne se capturent pas. Au codex, elles viennent après les stades 3.

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
| Shiny du petit | 1 chance sur 512 comme un monstre sauvage ; 1 sur 256 avec un parent shiny ; à coup sûr avec deux (étape 114) |
| Attaque d'œuf | 1 chance sur 4 ; à coup sûr si un parent la connaît (étape 114) |

Textes proposés sans validation (gamedesign.md, Q105). {a} et {b} sont les parents, {petit} le nom du petit, {espèce} son espèce, {coût} les ressources et l'argent, {durée} le temps restant.

| Moment | Texte |
|---|---|
| Service, à l'étal | Rite du sang mêlé : 3 ressources de l'espèce du petit et 50 pièces d'argent, 2 h |
| Confirmation | Confier {a} et {b} à Malvoise pour le Rite du sang mêlé ? Il prendra {coût}. Dans 2 h, le petit naîtra, de l'espèce {espèce}, au niveau 1, avec la meilleure valeur interne de ses parents pour chaque stat. D'ici là, les deux restent auprès de Malvoise. |
| Rite lancé | Malvoise trace un cercle de sang autour de {a} et {b}, et entonne le Rite du sang mêlé. Le petit naîtra dans {durée}. |
| État, à l'étal | Rite du sang mêlé : {a} et {b}, achevé dans {durée}. / ..., achevé, le petit attend. |
| Au camp | Le Rite du sang mêlé est achevé : le petit attend auprès de Malvoise. |
| Naissance | Le cercle de sang se referme et le petit en naît : {petit}, au niveau 1. {a} et {b} vous rejoignent. |
| Console | Quel premier parent confier à Malvoise ? ; Second parent, de la même famille : ; Aucun autre de vos monstres n'est de sa famille d'évolution. |
| Discord | Rite du sang mêlé (Malvoise) ; Choisissez le second parent, de la même famille d'évolution que {a}. ; Lui unir un monstre de l'équipe / du camp ; Retour au monstre ; Accueillir le petit (Malvoise) ; Choisissez d'abord un monstre dans la gestion. ; à l'étal : {service} : bouton "Rite du sang mêlé (Malvoise)" de la gestion des monstres. |
| Liste des monstres | 1 place du camp est gardée, pour un monstre absent ou à naître. / {n} places du camp sont gardées, pour des monstres absents ou à naître. |
| Confirmation, ajouts de l'étape 114 | Il a 1 chance sur 4 de connaître {attaque}, l'attaque d'œuf de sa famille. / Il connaîtra {attaque}, l'attaque d'œuf de sa famille, que connaît un parent. / Avec un parent shiny, il a 1 chance sur 256 de l'être. / Ses deux parents sont shiny : il le sera aussi. |
| Naissance, ajout de l'étape 114 | Il connaît {attaque}, l'attaque d'œuf de sa famille. |
| Fiche du codex | Attaque d'œuf (Rite du sang mêlé) : {attaque} ({type}, {catégorie}, puissance {n}, précision {n} %) |
| Refus | Malvoise accomplit déjà un rite : un seul à la fois. / Le rite n'unit que deux monstres d'une même famille d'évolution. / Il faut une place libre au camp pour le petit. / Il vous manque {ressources}. / Aucun rite n'est en cours. / Le rite n'est pas encore achevé. |

Attaques d'œuf, décidées le 25/10/2026 pour l'étape 114 (tableau proposé par Claude et validé par l'utilisateur ; 32 familles, voir gamedesign.md, section 13.11). Elles ne s'obtiennent que par le rite ; toute la famille partage celle de sa forme de base.

| Famille (forme de base) | Identifiant | Attaque | Type | Catégorie | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|---|
| Salamandre | souffle-des-forges | Souffle des forges | Feu | spéciale | 80 | 100 | aucun |
| Crapaud | ressac | Ressac | Eau | physique | 80 | 100 | aucun |
| Mandragore | seve-maudite | Sève maudite | Plante | spéciale | 75 | 100 | poison 20 % |
| Chat noir | griffes-de-minuit | Griffes de minuit | Ténèbres | physique | 80 | 100 | aucun |
| Loup | assaut-de-meute | Assaut de meute | Normal | physique | 80 | 100 | aucun |
| Corbeau | plongee-funebre | Plongée funèbre | Vol | physique | 80 | 100 | aucun |
| Vipère | etreinte-de-l-aspic | Étreinte de l'aspic | Poison | physique | 75 | 100 | poison 30 % |
| Feu follet | lueur-des-marais | Lueur des marais | Spectre | spéciale | 80 | 100 | aucun |
| Armure hantée | coup-de-heaume | Coup de heaume | Acier | physique | 80 | 100 | aucun |
| Fée noire | essaim-maudit | Essaim maudit | Insecte | spéciale | 80 | 100 | aucun |
| Squelette | danse-macabre | Danse macabre | Combat | physique | 80 | 100 | aucun |
| Cerf blanc | brame-sacre | Brame sacré | Féerique | spéciale | 80 | 100 | aucun |
| Basilic | souffle-du-basilic | Souffle du basilic | Dragon | spéciale | 75 | 100 | poison 20 % |
| Elfe | sortilege-sylvestre | Sortilège sylvestre | Féerique | spéciale | 80 | 100 | aucun |
| Vouivre | pique-draconique | Piqué draconique | Dragon | physique | 80 | 100 | aucun |
| Schnoz | pluie-d-etoiles | Pluie d'étoiles | Stellaire | spéciale | 80 | 100 | aucun |
| Gargouille | plongeon-de-granit | Plongeon de granit | Roche | physique | 80 | 100 | aucun |
| Goule | griffes-putrides | Griffes putrides | Poison | physique | 75 | 100 | poison 30 % |
| Chauve-souris de crypte | vol-nocturne | Vol nocturne | Vol | physique | 80 | 100 | aucun |
| Chimère | souffle-triple | Souffle triple | Dragon | spéciale | 80 | 100 | aucun |
| Ange déchu | litanie-des-dechus | Litanie des déchus | Psychique | spéciale | 80 | 100 | aucun |
| Noyé | appel-des-profondeurs | Appel des profondeurs | Eau | spéciale | 80 | 100 | aucun |
| Anguille des tourbières | morsure-des-tourbieres | Morsure des tourbières | Ténèbres | physique | 80 | 100 | aucun |
| Sangsue géante | saignee | Saignée | Poison | physique | 75 | 100 | poison 30 % |
| Kelpie | galop-des-eaux | Galop des eaux | Eau | physique | 80 | 100 | aucun |
| Wendigo | griffes-de-famine | Griffes de famine | Glace | physique | 80 | 100 | aucun |
| Troll des cavernes | coup-de-massue | Coup de massue | Sol | physique | 80 | 100 | aucun |
| Harpie des orages | oeil-de-l-orage | Œil de l'orage | Électrique | spéciale | 75 | 100 | paralysie 20 % |
| Wyrm de givre | souffle-de-l-hiver | Souffle de l'hiver | Glace | spéciale | 80 | 100 | aucun |
| Sylphe des neiges | valse-des-neiges | Valse des neiges | Féerique | spéciale | 80 | 100 | aucun |
| Cloche hurlante | volee-de-cloches | Volée de cloches | Son | spéciale | 80 | 100 | aucun |
| Séraphin de vitrail | rosace-ardente | Rosace ardente | Lumière | spéciale | 80 | 100 | aucun |

## 29. Messages privés de fin

Décidé le 25/10/2026, pour l'étape 113 (gamedesign.md, section 2). Les textes des expéditions automatiques, de la forge et de la pêche ne changent pas ; ceux-ci sont proposés sans validation. {monstre} est le nom du monstre.

| Ce qui se termine | Message |
|---|---|
| Rite du sang mêlé | Le Rite du sang mêlé est achevé : le petit attend auprès de Malvoise. Ouvrez /bersemon pour l'accueillir. |
| Pièges pleins | Les pièges de Gunther sont pleins. Ouvrez /bersemon pour les relever. |
| Monstre de la cour à son plafond | Entraînement terminé : {monstre} a atteint le plafond de la cour d'entraînement. Ouvrez /bersemon pour reprendre ce monstre. |

## 30. Récompenses du codex

Décidé le 25/10/2026, pour l'étape 115 (gamedesign.md, section 5.6). Paliers, récompenses et talisman validés par l'utilisateur, à ajuster en jouant.

| Palier | Récompense, au coffre |
|---|---|
| 10 espèces capturées | 10 filets lestés et 300 pièces d'argent |
| 20 espèces capturées | 5 filets de chaînes et 500 pièces d'argent |
| 30 espèces capturées | 1 Philtre chromatique |
| 40 espèces capturées | 3 capsules d'habileté |
| 50 espèces capturées | 1 booster de chaque sorte (6) |
| Codex complet (53 espèces depuis l'étape 135, 51 avant) | 3 Philtres chromatiques, et le Sceau du bestiaire dans l'équipement du héros |

Sceau du bestiaire (identifiant sceau-du-bestiaire) : talisman, +5 % à l'Attaque, la Défense, l'Attaque spéciale, la Défense spéciale et la Vitesse. Unique, il ne se fabrique pas.

Textes proposés sans validation (gamedesign.md, Q106) :

| Moment | Texte |
|---|---|
| Progression | Espèces capturées : {n} sur {total}. Récompense à réclamer pour 10 espèces capturées : {récompense}. / ... Prochaine récompense, à 20 espèces capturées : {récompense}. / ... Prochaine récompense, au codex complet : {récompense}. / ... Toutes les récompenses du codex sont réclamées. |
| Action | Réclamer la récompense : 10 espèces capturées / Réclamer la récompense : le codex complet |
| Récit | Récompense du codex, à 10 espèces capturées, au coffre : {objets}. / Récompense du codex complet, au coffre : {objets}. Le talisman {talisman} rejoint votre équipement. |
| Refus | Aucune récompense du codex à réclamer pour l'instant. |

## 31. Cycle du jour et de la nuit

Décidé le 25/10/2026, pour les étapes 118 à 120 (gamedesign.md, section 9.6). Valeurs choisies par l'utilisateur ; textes proposés sans validation (gamedesign.md, Q109).

| Moment | Durée | Attaques renforcées (+20 %) | Espèces qui ne sortent qu'à ce moment |
|---|---|---|---|
| Jour | 2 h | Lumière, Feu, Plante | Vipère, Cloche hurlante, Harpie des orages |
| Nuit | 1 h | Ténèbres, Spectre, Poison | Loup, Feu follet, Fée noire, Goule, Chauve-souris de crypte, Troll des cavernes, Wendigo, Warg |

De nuit : monstres des groupes sauvages et monstres rares à 2 niveaux de plus, XP des monstres vaincus et argent des coffres à +25 %.

Espèces par zone et par moment :

| Zone | Jour | Nuit |
|---|---|---|
| Lisière du Bois-Sombre | Corbeau, Vipère | Loup, Corbeau |
| Marais des Pendus | Armure hantée, Vipère, Corbeau | Feu follet, Armure hantée, Corbeau |
| Vallée des Brumes | Squelette, Armure hantée | Fée noire, Squelette, Armure hantée |
| Abbaye des Cendres | Squelette, Gargouille, Cloche hurlante | Squelette, Gargouille, Goule, Chauve-souris de crypte |
| Col du Givre-Noir | Harpie des orages, Gargouille, Chevalier noir | Wendigo, Troll des cavernes, Gargouille, Warg, Chevalier noir |

Évolutions selon le moment, au niveau 16 : Loup en Warg, de nuit ; Corbeau en Corbeau des gibets, de nuit ; Vipère en Aspic, de jour.

Textes :

| Moment | Texte |
|---|---|
| Camp et carte, de jour | Jour : la nuit tombe dans {durée}. |
| Camp et carte, de nuit | Nuit : l'aube dans {durée}. |
| Départ de jour | Il fait jour. Les créatures de la nuit se terrent. |
| Départ de nuit | La nuit est tombée. Les bêtes rôdent plus fortes, et d'autres sortent de l'ombre. |
| Compteur d'étapes | [Étape : 2/5] Jour / [Étape : 2/5] Nuit |
| Condition d'évolution, à la fiche | Warg dès le niveau 16, après un combat de nuit / Aspic dès le niveau 16, après un combat de jour |
| Fiche d'une espèce capturée | Ne sort que la nuit. / Ne sort que le jour. |

## 32. Météo

Décidé le 25/10/2026, pour les étapes 121 à 123 (gamedesign.md, section 9.7). Valeurs choisies par l'utilisateur ; textes proposés sans validation (gamedesign.md, Q110).

| Météo | Identifiant | Types renforcés (+20 %) | Zones | Texte au départ |
|---|---|---|---|---|
| Pluie | pluie | Eau, Plante | Lisière, Marais, Vallée, Abbaye, Port des Noyés, Sylve, Catacombes, Palais de Chair, Puits du Monde, Abysses | Une pluie froide tombe sans relâche. |
| Tempête | tempete | Électrique, Vol | toutes sauf le Désert de sel, le Palais de Chair, les Abysses et les Terres arides | L'orage gronde : le ciel se déchire d'éclairs. |
| Grêle | grele | Glace, Roche | Vallée, Col, Mines de Fer-Noir, Haute-Cité déchue, Pics Foudroyés, Ciel déchiré | La grêle crépite sur les pierres et les armures. |
| Tempête de sable | tempete-de-sable | Sol, Roche | Désert de sel, Terres arides | Le sable vole, cinglant comme une lame. |
| Cyclone | cyclone | Eau, Vol | Port des Noyés et Abysses, zones de mer | Le cyclone hurle, et les vagues montent à l'assaut des côtes. |
| Canicule | canicule | Feu, Sol | Lisière, Marais, Abbaye, Désert de sel, Sylve, Plaine du Massacre, Gorge de Braise, Palais de Chair, Terres arides | Une chaleur écrasante pèse sur la terre. |

Chaque zone a 1 chance sur 3, par créneau de 30 minutes, d'avoir une de ses météos ; sinon, temps calme.

Évolutions selon la météo (étape 124) : Fée noire en Fée-sorcière au niveau 12, sous la tempête ; Fée-sorcière en Reine des ronces au niveau 25, sous la pluie ; Basilic en Basilic royal au niveau 22, sous la canicule. À la fiche : "Basilic royal dès le niveau 22, après un combat sous la canicule" ("dans le cyclone" pour le cyclone).

Textes :

| Moment | Texte |
|---|---|
| Carte, console | Lisière du Bois-Sombre (niveaux 3 à 7), explorée : pluie, encore 42 min |
| Carte, Discord, description de la zone | Niveaux 3 à 7, explorée ; pluie, encore 42 min |
| Compteur d'étapes | [Étape : 2/5] Nuit, pluie |

## 33. Capacité du sac

Décidé le 25/10/2026, pour les étapes 125 à 127 (gamedesign.md, section 13.3). Valeurs choisies par l'utilisateur ; coûts de l'amélioration dans la section 9 ; textes proposés sans validation (gamedesign.md, Q111).

Places : 30 au départ, puis 45, 60 et 80 avec l'amélioration du sac. Chaque objet prend une place, l'argent aucune.

| Moment | Texte |
|---|---|
| Camp et étape | Sac (18/30) : {contenu}. |
| Sac qui déborde | Votre sac déborde : {objets} objets pour {places} places. Jetez-en {surplus} pour poursuivre ; ce que vous jetez est perdu. |
| Console, quantité | Combien en jeter ? |
| Discord, menus | Jeter un objet / Jeter une ressource ; chaque option : "En jeter un" |
| Récit | Vous jetez {objets}. |
| Refus de sortie du coffre | Le sac est plein : rangez des objets au coffre. / Le sac n'a plus que {n} places libres. |
| Refus de départ | Votre sac déborde de {surplus} objets : rangez-en au coffre avant de partir. |
| Forge, effet | sac de 45 places |

## 34. Héros combattant, type Humain et équipement de classe

Décidé le 25/10/2026, pour les étapes 128 à 133 (gamedesign.md, sections 4.3, 4.4 et 5.2), contenu proposé par Claude et validé par l'utilisateur, à ajuster en jouant.

Stats de base des classes, dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse, avec des valeurs internes fixes à 15 :

| Classe | Stats de base (total) |
|---|---|
| Guerrier | 180 / 100 / 85 / 40 / 65 / 70 (540) |
| Mage | 140 / 40 / 55 / 110 / 90 / 85 (520) |
| Templier | 190 / 45 / 100 / 75 / 95 / 40 (545) |
| Assassin | 140 / 105 / 55 / 40 / 60 / 120 (520) |

PV de base doublés le 26/10/2026, à l'étape 147 (choix de l'utilisateur, gamedesign.md, section 4.4) : ils étaient de 90, 70, 95 et 70, pour un total de 450 par classe. Un Guerrier a 33 PV au niveau 5, 128 au niveau 25 et 176 au niveau 35 (24, 83 et 113 avant).

Niveau de combat : 5, plus 5 par palier de l'arme et 5 par palier de l'armure (5 à 40 ; 55 depuis l'étape 163, avec les paliers 4 et 5 de la section 44 ; 65 depuis l'étape 165, avec le palier 6 de la section 45 ; 75 depuis l'étape 166, avec le palier 7 de la section 46 ; 85 depuis l'étape 167, avec le palier 8 de la section 47).

Attaques du héros, de type Humain :

| Identifiant | Attaque | Classe | Catégorie | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| coup-de-poing | Coup de poing | toutes, sans arme | P | 40 | 100 | |
| taille | Taille | Guerrier, palier 1 | P | 40 | 100 | |
| estoc | Estoc | Guerrier, palier 2 | P | 50 | 100 | |
| pourfendre | Pourfendre | Guerrier, palier 3 | P | 60 | 100 | |
| coup-du-champion | Coup du champion | Guerrier, palier 4 | P | 75 | 100 | |
| frappe-brutale | Frappe brutale | Guerrier, toute arme | P | 70 | 80 | |
| entaille | Entaille | Assassin, palier 1 | P | 40 | 100 | |
| coup-bas | Coup bas | Assassin, palier 2 | P | 50 | 100 | |
| egorgement | Égorgement | Assassin, palier 3 | P | 60 | 100 | |
| execution | Exécution | Assassin, palier 4 | P | 75 | 100 | |
| lame-empoisonnee | Lame empoisonnée | Assassin, toute arme | P | 30 | 100 | poison 50 % |
| trait-arcanique | Trait arcanique | Mage, palier 1 | S | 40 | 100 | |
| fleche-occulte | Flèche occulte | Mage, palier 2 | S | 50 | 100 | |
| rayon-occulte | Rayon occulte | Mage, palier 3 | S | 60 | 100 | |
| tempete-occulte | Tempête occulte | Mage, palier 4 | S | 75 | 100 | |
| onde-engourdissante | Onde engourdissante | Mage, toute arme | S | 30 | 100 | paralysie 40 % |
| chatiment | Châtiment | Templier, palier 1 | S | 40 | 100 | |
| verdict | Verdict | Templier, palier 2 | S | 50 | 100 | |
| anatheme | Anathème | Templier, palier 3 | S | 60 | 100 | |
| courroux-sacre | Courroux sacré | Templier, palier 4 | S | 75 | 100 | |
| sceau-d-entrave | Sceau d'entrave | Templier, toute arme | S | 30 | 100 | paralysie 30 % |

Attaques du second type, ajoutées le 26/10/2026 à l'étape 149 (choix de l'utilisateur, gamedesign.md, section 4.4 ; noms proposés sans validation, Q114). Les armes des paliers 2 à 4 de chaque classe les donnent en troisième attaque, après l'attaque principale et celle de la classe ; l'arme du palier 1 n'en donne pas :

| Identifiant | Attaque | Classe | Type | Catégorie | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|---|
| coup-de-pommeau | Coup de pommeau | Guerrier, paliers 2 à 4 | Combat | P | 60 | 100 | |
| lame-des-tenebres | Lame des ténèbres | Assassin, paliers 2 à 4 | Ténèbres | P | 60 | 100 | |
| emprise-mentale | Emprise mentale | Mage, paliers 2 à 4 | Psychique | S | 60 | 100 | |
| rayon-de-foi | Rayon de foi | Templier, paliers 2 à 4 | Lumière | S | 60 | 100 | |

Chaque arme donne l'attaque principale de son palier, puis l'attaque de sa classe. Précisé à l'étape 129, en codant : Saignée, nom validé pour l'attaque du palier 3 de l'Assassin, est déjà celui d'une attaque Poison de la Sangsue géante ; elle devient Égorgement, nom proposé sans validation (gamedesign.md, Q112).

Armes de classe. Bonus aux monstres : +3 %, +5 %, +8 % ou +10 % d'Attaque et d'Attaque spéciale selon le palier. Coûts et durées : ceux des anciennes armes d'Attaque pour le Guerrier et l'Assassin, d'Attaque spéciale pour le Mage et le Templier.

| Identifiant | Arme | Classe | Palier | Coût | Durée |
|---|---|---|---|---|---|
| lame-de-milicien | Lame de milicien | Guerrier | 1 | 6 Croc de loup, 4 Écaille de vipère | 1 h |
| epee-du-pendu | Épée du pendu | Guerrier | 2 | 6 Fragment d'armure, 6 Os de squelette, 1 Croc de basilic | 3 h |
| espadon-des-cendres | Espadon des cendres | Guerrier | 3 | 8 Éclat de gargouille, 6 Dent de goule, 1 Crin de chimère | 6 h |
| glaive-du-champion | Glaive du champion | Guerrier | 4 | récompense de la Fosse aux damnés | |
| couteau-de-chasse | Couteau de chasse | Assassin | 1 | 6 Croc de loup, 4 Écaille de vipère | 1 h |
| dague-des-brumes | Dague des brumes | Assassin | 2 | 6 Fragment d'armure, 6 Os de squelette, 1 Croc de basilic | 3 h |
| lames-de-cendre | Lames de cendre | Assassin | 3 | 8 Éclat de gargouille, 6 Dent de goule, 1 Crin de chimère | 6 h |
| crocs-du-champion | Crocs du champion | Assassin | 4 | récompense de la Fosse aux damnés | |
| baton-d-if | Bâton d'if | Mage | 1 | 6 Plume de corbeau, 4 Écaille de vipère | 1 h |
| grimoire-des-brumes | Grimoire des brumes | Mage | 2 | 6 Cendre de feu follet, 6 Aile de fée noire, 1 Croc de basilic | 3 h |
| sceptre-des-cendres | Sceptre des cendres | Mage | 3 | 8 Aile de chauve-souris, 6 Dent de goule, 1 Crin de chimère | 6 h |
| grimoire-du-champion | Grimoire du champion | Mage | 4 | récompense de la Fosse aux damnés | |
| masse-de-pelerin | Masse de pèlerin | Templier | 1 | 6 Plume de corbeau, 4 Écaille de vipère | 1 h |
| fleau-du-pendu | Fléau du pendu | Templier | 2 | 6 Cendre de feu follet, 6 Aile de fée noire, 1 Croc de basilic | 3 h |
| encensoir-maudit | Encensoir maudit | Templier | 3 | 8 Aile de chauve-souris, 6 Dent de goule, 1 Crin de chimère | 6 h |
| marteau-du-champion | Marteau du champion | Templier | 4 | récompense de la Fosse aux damnés | |

Armures de classe. Bonus aux monstres : +5 %, +10 % ou +15 % de Défense et de Défense spéciale selon le palier. Coûts et durées : ceux des anciennes armures du même palier.

| Identifiant | Armure | Classe | Palier | Coût | Durée |
|---|---|---|---|---|---|
| gambison | Gambison | Guerrier | 1 | 6 Croc de loup, 6 Plume de corbeau | 2 h |
| cotte-de-mailles | Cotte de mailles | Guerrier | 2 | 8 Fragment d'armure, 4 Os de squelette, 1 Écaille de vouivre | 4 h |
| plates-de-gargouille | Plates de gargouille | Guerrier | 3 | 10 Éclat de gargouille, 6 Os de squelette, 1 Crin de chimère | 8 h |
| brigandine-de-cuir | Brigandine de cuir | Assassin | 1 | 6 Croc de loup, 6 Plume de corbeau | 2 h |
| cuir-des-brumes | Cuir des brumes | Assassin | 2 | 8 Fragment d'armure, 4 Os de squelette, 1 Écaille de vouivre | 4 h |
| manteau-de-cendre | Manteau de cendre | Assassin | 3 | 10 Éclat de gargouille, 6 Os de squelette, 1 Crin de chimère | 8 h |
| robe-de-lin | Robe de lin | Mage | 1 | 6 Croc de loup, 6 Plume de corbeau | 2 h |
| robe-des-brumes | Robe des brumes | Mage | 2 | 8 Fragment d'armure, 4 Os de squelette, 1 Écaille de vouivre | 4 h |
| robe-des-cendres | Robe des cendres | Mage | 3 | 10 Éclat de gargouille, 6 Os de squelette, 1 Crin de chimère | 8 h |
| haubert-de-pelerin | Haubert de pèlerin | Templier | 1 | 6 Croc de loup, 6 Plume de corbeau | 2 h |
| cotte-benie | Cotte bénie | Templier | 2 | 8 Fragment d'armure, 4 Os de squelette, 1 Écaille de vouivre | 4 h |
| plates-consacrees | Plates consacrées | Templier | 3 | 10 Éclat de gargouille, 6 Os de squelette, 1 Crin de chimère | 8 h |

Les identifiants des anciens objets sont gardés pour l'objet de classe qui reprend leur nom. Les talismans de la section 11 et le Sceau du bestiaire ne changent pas.

Humains de fin de zone (gamedesign.md, section 10.7) :

| Humain | Classe | Arme | Armure | Niveau |
|---|---|---|---|---|
| Brigand | Assassin | palier 1 | aucune | 10 |
| Bourreau | Guerrier | palier 1 | palier 1 | 15 |
| Inquisiteur | Mage | palier 2 | palier 2 | 25 |
| Porte-étendard | Templier | palier 3 | palier 3 | 35 |

Textes proposés sans validation (gamedesign.md, Q112) :

| Moment | Texte |
|---|---|
| Entrée du héros en combat | Tous vos monstres sont à terre : {héros} entre lui-même dans le combat ! |
| Entrée du héros d'un humain | {humain} n'a plus de monstre debout : il entre lui-même dans le combat ! |
| Retour d'un monstre relevé | {monstre} se relève et reprend sa place : {héros} recule. |
| Défaite | {héros} s'effondre. |
| Fiche du héros | {héros}, {classe}, Humain, niveau de combat {n} : {pv}/{max} PV. / Stats en combat : {stats}. / Attaques : {attaques} / Il combat quand tous vos monstres sont KO. Son niveau vaut 5, plus 5 par palier de son arme et de son armure. |
| Ligne du héros, en combat et en expédition | {héros} (héros, niv. {n}) : {pv}/{max} PV |
| Héros adverse en attente | puis {humain} lui-même, en dernier |
| Forge, arme | ; attaques du héros : {attaques} |
| Soin | Soigner qui ? |
| Défaite, retour au camp | Votre héros et toute votre équipe sont KO : vous rentrez au camp, le sac vide. / ... et perdez tout votre sac : {objets}. |
| Retour au camp | Retour au camp : votre héros et vos monstres sont soignés. |

## 35. Grand élixir

Décidé le 25/10/2026, pour l'étape 134 (gamedesign.md, section 14.4) : grand élixir (identifiant grand-elixir), 400 pièces d'argent, relève un monstre KO avec tous ses PV max. Vendu par Malvoise une fois l'âme d'Ulrich recueillie, avec l'élixir.

Textes proposés sans validation (gamedesign.md, Q112) : Grand élixir ; 1 grand élixir, 3 grands élixirs ; Le grand élixir ; relève un monstre KO avec tous ses PV max ; Le grand élixir relève {monstre} avec {pv} PV. ; Vous n'avez plus de grand élixir.

## 36. Fosse aux damnés

Décidée le 25/10/2026, pour les étapes 135 à 137 (gamedesign.md, section 9.8), contenu proposé par Claude et validé par l'utilisateur.

Lieu : la Fosse aux damnés (identifiant fosse-aux-damnes), hors des zones de la carte, ouverte quand le brigand est vaincu. 10 paliers, un adversaire chacun, à un niveau fixé (au niveau donné par son équipement jusqu'à l'étape 148), valeurs internes fixes à 15.

| Palier | Identifiant | Adversaire | Classe | Arme | Niveau | IA | Récompense, à la première victoire |
|---|---|---|---|---|---|---|---|
| 1 | gueux | Le Gueux | Guerrier | palier 1 | 10 | simple | 40 pièces |
| 2 | coupe-jarret | Le Coupe-jarret | Assassin | palier 1 | 13 | simple | 60 pièces |
| 3 | novice-de-l-ordre | Le Novice de l'ordre | Templier | palier 1 | 16 | simple | 80 pièces, un Basilic de niveau 16 |
| 4 | apprenti-necromant | L'Apprenti nécromant | Mage | palier 2 | 18 | simple | 100 pièces |
| 5 | mercenaire | Le Mercenaire | Guerrier | palier 2 | 21 | avancée | 120 pièces, un Molosse des fosses de niveau 21 |
| 6 | lame-noire | La Lame noire | Assassin | palier 2 | 24 | avancée | 150 pièces |
| 7 | sorciere-des-cendres | La Sorcière des cendres | Mage | palier 3 | 27 | avancée | 180 pièces, une Chimère de niveau 27 |
| 8 | templier-renegat | Le Templier renégat | Templier | palier 3 | 29 | avancée | 210 pièces |
| 9 | gladiateur | Le Gladiateur | Guerrier | palier 3 | 32 | avancée | 250 pièces |
| 10 | champion-de-la-fosse | Le Champion de la Fosse | Assassin | palier 3 | 35 | avancée | 400 pièces, un Taureau d'airain de niveau 35, l'arme du palier 4 de la classe du héros |

Niveaux fixés le 26/10/2026, à l'étape 148 (choix de l'utilisateur, gamedesign.md, section 9.8) : ils venaient de l'équipement (10, 15, 15, 20, 20, 25, 30, 30, 35, 35), et les armures ne servaient qu'à les donner ; elles sont retirées. Les monstres offerts suivent le niveau de leur palier (le Basilic était de niveau 15, le Molosse 20, la Chimère 30). Second type de la classe aux paliers 7 à 10 (section 41).

Adversaires miroirs, décidés le 26/10/2026 à l'étape 151 (choix de l'utilisateur, gamedesign.md, section 9.8) : la Sorcière des cendres (palier 7), le Gladiateur (9) et le Champion de la Fosse (10) prennent la classe du héros qui les affronte, avec l'arme du palier 3 de cette classe (Espadon des cendres, Lames de cendre, Sceptre des cendres ou Encensoir maudit) ; la classe du tableau est celle qu'ils montrent hors combat. Leur texte de présentation ne nomme pas de classe.

L'argent va au coffre du camp.

Espèces exclusives à la Fosse (rares, ni sauvages, ni vendues, ni louées, sans évolution, sans ressource, sans capsule ; au codex après les espèces existantes) :

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Habiletés |
|---|---|---|---|---|---|---|
| molosse-des-fosses | Molosse des fosses | Combat | offensif | 80 / 100 / 70 / 35 / 60 / 85 (430) | 170 | Férocité, Instinct |
| taureau-d-airain | Taureau d'airain | Acier, Feu | offensif | 90 / 110 / 100 / 75 / 70 / 40 (485) | 200 | Carapace, Férocité |

Attaques apprises, toutes existantes :

- Molosse des fosses : Griffure (niv. 1), Frappe d'os (niv. 5), Curée (niv. 12), Déchiquetage (niv. 20).
- Taureau d'airain : Coup de garde (niv. 1), Brasier (niv. 10), Taille d'acier (niv. 25), Bûcher (niv. 30).

Attaques d'œuf (Rite du sang mêlé, section 28), ajoutées à l'étape 135, proposées par Claude sans validation (gamedesign.md, Q112), sur le modèle des autres :

| Famille | Identifiant | Attaque | Type | Catégorie | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|---|
| Molosse des fosses | charge-des-fosses | Charge des fosses | Combat | physique | 80 | 100 | aucun |
| Taureau d'airain | charge-d-airain | Charge d'airain | Acier | physique | 80 | 100 | aucun |

Textes validés avec les espèces, qui ne s'affichent nulle part pour l'instant : les fiches d'espèce n'ont pas de description.

> Molosse des fosses : chien de guerre élevé dans les fosses, nourri de chair de condamné.

> Taureau d'airain : taureau de bronze où l'on brûlait les condamnés ; leurs cris l'ont éveillé.

Textes proposés sans validation (gamedesign.md, Q112) :

| Moment | Texte |
|---|---|
| Lieu | Une fosse creusée dans la roche, cernée de gradins où se pressent des ombres avides. Ici, les humains s'entretuent pour le plaisir des spectateurs, et le sable ne sèche jamais. |
| Avant un palier | Palier {n} sur 10. Face à vous : {adversaire}, {classe}, niveau {niveau}. |
| Le Gueux | Un vagabond en haillons, un gourdin à la main. Il n'a plus rien à perdre. |
| Le Coupe-jarret | Il tourne autour de vous en souriant, une lame courte dans chaque main. |
| Le Novice de l'ordre | Un jeune frère en armure trop grande, qui prie à voix basse en levant sa masse. |
| L'Apprenti nécromant | Sa robe sent la tombe ; une lueur malsaine danse au bout de ses doigts. |
| Le Mercenaire | Il a vendu son épée à tous les seigneurs du royaume, et leur a survécu à tous. |
| La Lame noire | Une silhouette encapuchonnée, dont on dit qu'elle n'a jamais raté une gorge. |
| La Sorcière des cendres | Elle sort des ruines de l'Abbaye, la peau grise de cendre, les yeux brûlants. |
| Le Templier renégat | Il a renié son ordre, mais pas sa foi ni son armure. |
| Le Gladiateur | La foule scande son nom. Il salue les gradins avant de vous faire face. |
| Le Champion de la Fosse | Personne ne l'a jamais vu saigner. Les gradins se taisent quand il entre. |
| Victoire | {adversaire} tombe sur le sable. La foule hurle votre nom : palier {n} vaincu. |
| Récompense | Récompense du palier {n} : {récompenses}. |
| Défaite | {héros} s'effondre sous les huées. Vous pourrez revenir quand vous voudrez. |
| Abandon | Vous jetez votre arme sur le sable : la foule vous couvre d'insultes. |
| Fosse conquise | Vous avez vaincu le Champion : la Fosse aux damnés n'a plus d'adversaire à vous opposer. |
| Carte de la console | Fosse aux damnés (arène) : palier {n}/10 / Fosse aux damnés (arène) : conquise |
| Menu des expéditions du bot | Fosse aux damnés ; Arène ; prochain palier {n}/10 / Arène ; conquise |
| Boutons | Combattre ; Revenir au camp ; Abandonner |
| Abandon de l'adversaire, en duel d'arène | {adversaire} jette son arme sur le sable. |
| Après une défaite ou un abandon | Les huées vous raccompagnent hors de la Fosse. Vous pourrez revenir quand vous voudrez. |
| Refus | La Fosse aux damnés n'ouvre ses portes qu'à qui a vaincu le brigand de la Lisière. / Vous avez vaincu tous les adversaires de la Fosse aux damnés. / Ce palier offre un monstre : faites-lui de la place dans l'équipe ou au camp avant de combattre. |

## 37. Arène entre joueurs

Décidée le 25/10/2026, pour l'étape 138 (gamedesign.md, section 15.6). Cote de départ 1000 ; gain du vainqueur 32 x (1 - E), arrondi à l'entier le plus proche, avec E = 1 / (1 + 10^((cote du perdant - cote du vainqueur) / 400)) ; le perdant perd autant, sans descendre sous 0.

Textes proposés sans validation (gamedesign.md, Q112) :

| Moment | Texte |
|---|---|
| Écran des autres joueurs | Bouton Défier à l'arène. / {héros} vous défie à l'arène. / Vous avez défié {héros} à l'arène. En attente de sa réponse. |
| Message privé, défi reçu | {héros} ({pseudo}) vous défie à l'arène. Ouvrez /bersemon, au camp, pour répondre. |
| Fin du combat | Vous remportez le combat d'arène : votre cote passe à {cote} (+{gain}). / Vous perdez le combat d'arène : votre cote passe à {cote} (-{perte}). |
| Classement de l'arène | Classement de l'arène. / Votre cote : {cote}. / Aucun combat d'arène pour l'instant. / {rang}. {héros} ({pseudo}) : {cote} ; boutons Classement de l'arène, Classement des duels, Records de la Tour |
| Écran du combat | Combat d'arène contre {héros}. / Victoire : vous remportez le combat d'arène contre {héros}. / Défaite : {héros} remporte le combat d'arène. / {héros} jette son arme sur le sable. |

## 38. Fiche du codex

Décidée le 26/10/2026, pour l'étape 140 (gamedesign.md, section 5.6). Libellés proposés sans validation (gamedesign.md, Q113), dans l'ordre de la fiche, après les types, les habiletés et les stats de base :

| Ligne | Texte |
|---|---|
| Attaques apprises | Attaques apprises : / puis une ligne par attaque : "niv. 1 : Griffure (Normal, physique, puissance 40, précision 100 %)", et pour l'attaque d'évolution, sur la page de l'espèce d'arrivée : "niv. 16 (en évoluant depuis Loup) : Croc noir (Ténèbres, physique, puissance 75, précision 95 %)", depuis l'étape 150 ("(évolution)" avant ; "???" pour une espèce de départ pas encore capturée) |
| Évolution | Évolution : Warg dès le niveau 16, après un combat de nuit. / Évolution : ??? dès le niveau 16, après un combat de nuit (espèce d'arrivée pas encore capturée). / Évolution : ???, condition inconnue (Apôtre pas encore abattu) |
| Capsules | Capsules d'attaque possibles : / puis une ligne par capsule : "Sentence (Ténèbres, spéciale, puissance 80, précision 95 %)" ; ou "Capsules d'attaque possibles : aucune" |

L'attaque d'œuf et le moment de sortie suivent, comme avant.

## 39. Panacée

Décidée le 26/10/2026, pour l'étape 142 (gamedesign.md, section 14.4) : panacée (identifiant panacee), 120 pièces d'argent, rend tous les PV max et guérit le poison ou la paralysie, sans relever un monstre KO. Vendue par Malvoise une fois l'âme de Hilde recueillie (section 40).

Textes proposés sans validation (gamedesign.md, Q113) : Panacée ; 1 panacée, 3 panacées ; La panacée ; rend tous les PV max et guérit le poison ou la paralysie ; {monstre} récupère {pv} PV. ; {monstre} récupère {pv} PV et guérit du poison. (ou de la paralysie) ; La panacée guérit {monstre} du poison. (ou de la paralysie, quand il a tous ses PV) ; {monstre} a déjà tous ses PV, sans poison ni paralysie : la panacée serait perdue. L'utiliser quand même ? ; Vous n'avez plus de panacée. Sur Discord, le menu de l'étal : "Acheter des soins", options "5 potions : 100 pièces d'argent", avec l'effet du soin en description.

## 40. Désert de sel et Dragon des brumes

Décidés le 26/10/2026, pour l'étape 143 (gamedesign.md, sections 5.3, 5.4, 9.1 et 9.4). La place de la zone, ses niveaux, le nombre et les types de ses espèces, l'humain qui annonce la suite, les services et l'évolution de la Vouivre sont des choix de l'utilisateur ; le reste est proposé par Claude sans validation (gamedesign.md, Q113).

Zone :

| Identifiant | Nom | Niveaux | Taille des groupes | Espèces | Monstres rares | Humain | Apôtre |
|---|---|---|---|---|---|---|---|
| desert-de-sel | Désert de sel | 37 à 45 | 1 à 3 | Crotale des salines, Scarabée solaire, Momie de sel, Ver des salines | Sphinx des sables (hostile), Phénix blanc (pacifique) | Pythie de sel | aucun |

Marchand mort : Hilde (identifiant hilde), gardée par un monstre de la zone ; son âme fait vendre la panacée (section 39). Météos : tempête de sable et canicule. Le Scarabée solaire est diurne, la Momie de sel nocturne.

Expédition automatique : durée de base 6 h, dégâts 280 à 560, XP 3000 à 4600, potions 1 à 3, argent 230 à 400. Coffres : 1 à 2 potions et 180 à 280 pièces d'argent ; capsules Chant des dunes (550), Éclat du zénith (550) et Vent de sable (500).

Espèces (stats dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse) :

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Rareté | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|---|
| crotale-des-salines | Crotale des salines | Son, Poison | offensif | 60 / 78 / 52 / 72 / 52 / 76 (390) | 115 | commune | Célérité, Instinct | Crécelle (1), Croc de venin (1), Tocsin (22), Venin noir (32) |
| scarabee-solaire | Scarabée solaire | Lumière, Insecte | soutien | 65 / 50 / 80 / 75 / 80 / 40 (390) | 114 | commune | Carapace, Volonté | Bourdonnement (1), Rayon solaire (1), Nuée (22), Aube éclatante (32) |
| momie-de-sel | Momie de sel | Spectre, Roche | soutien | 85 / 72 / 95 / 40 / 78 / 20 (390) | 117 | commune | Carapace, Férocité | Bandelettes (1), Chute de pierres (1), Lamentation (22), Éboulement (32) |
| ver-des-salines | Ver des salines | Sol | offensif | 95 / 92 / 68 / 35 / 55 / 45 (390) | 116 | commune | Férocité, Traqueur | Secousse (1), Étreinte (1), Engloutissement (22), Séisme (32) |
| sphinx-des-sables | Sphinx des sables | Son, Psychique | offensif | 80 / 70 / 75 / 100 / 85 / 55 (465) | 205 | rare, hostile | Sorcellerie, Volonté | Onde psychique (1), Glas (1), Carillon funèbre (20), Jugement (30) |
| phenix-blanc | Phénix blanc | Lumière, Feu | soutien | 75 / 50 / 65 / 100 / 95 / 80 (465) | 200 | rare, pacifique | Volonté, Instinct | Lueur sacrée (1), Brasier (1), Rayon solaire (15), Flamme blanche (30) |
| dragon-des-brumes | Dragon des brumes | Dragon, Vol | offensif | 90 / 100 / 80 / 100 / 75 / 75 (520) | 238 | rare | Affinité, Férocité | celles de la Vouivre, puis Souffle des brumes (45, attaque d'évolution) et Bec funeste (45) |

La Vouivre évolue en Dragon des brumes au niveau 45. Au codex, ces espèces viennent après le Taureau d'airain, dans l'ordre du tableau. Les espèces du Désert n'évoluent pas.

Attaques nouvelles :

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| crecelle | Crécelle | Son | S | 60 | 100 | paralysie 20 % |
| chant-des-dunes | Chant des dunes | Son | S | 85 | 95 | |
| rayon-solaire | Rayon solaire | Lumière | S | 70 | 100 | |
| eclat-du-zenith | Éclat du zénith | Lumière | S | 85 | 95 | |
| flamme-blanche | Flamme blanche | Lumière | S | 95 | 90 | |
| bandelettes | Bandelettes | Spectre | P | 65 | 100 | paralysie 20 % |
| engloutissement | Engloutissement | Sol | P | 95 | 85 | |
| vent-de-sable | Vent de sable | Sol | S | 85 | 90 | |
| souffle-des-brumes | Souffle des brumes | Dragon | S | 95 | 90 | |

Attaques d'œuf, sur le modèle de la section 28 :

| Famille | Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|---|
| Crotale des salines | morsure-des-salines | Morsure des salines | Poison | P | 80 | 100 | |
| Scarabée solaire | disque-solaire | Disque solaire | Lumière | S | 80 | 100 | |
| Momie de sel | linceul-de-sel | Linceul de sel | Spectre | P | 75 | 100 | paralysie 20 % |
| Ver des salines | charge-des-dunes | Charge des dunes | Sol | P | 80 | 100 | |
| Sphinx des sables | enigme-du-sphinx | Énigme du sphinx | Psychique | S | 80 | 100 | |
| Phénix blanc | aile-d-aurore | Aile d'aurore | Lumière | S | 80 | 100 | |

Le Dragon des brumes est de la famille de la Vouivre, dont il garde l'attaque d'œuf, Piqué draconique.

Ressources : Sonnette de crotale (sonnette-de-crotale), Élytre solaire (elytre-solaire), Bandelette de sel (bandelette-de-sel) et Anneau de ver (anneau-de-ver), 5 pièces d'argent chacune, Griffe de sphinx (griffe-de-sphinx), 15. Ressource rare des pièges dans cette zone : la Griffe de sphinx.

La Pythie de sel (identifiant pythie-de-sel), de classe Mage, attend au bout du Désert avec une Momie de sel et un Ver des salines de niveau 45 et un Sphinx des sables de niveau 46 (valeurs internes à 15, première habileté de leur espèce). Son héros porte le Grimoire du champion et la Robe des cendres, et combat au niveau 40 ; IA relevée. Victoire : 500 pièces d'argent, puis l'indice 8 au retour.

Textes :

> Indice 7 : dans la forteresse du Margrave, une carte de cuir marquée du même sceau : au sud, au-delà des cols, le Désert de sel, où une ville entière a été changée en sel. Une prophétesse y attendait des nouvelles du Margrave.

> Indice 8 : la Pythie vaincue a parlé avant de se taire : un autre Apôtre a reçu la marque du Démiurge. Il attend au-delà du désert, là où la route s'efface sous le sel. Elle n'a pas su dire où.

Pythie de sel, avant le combat :

> Au cœur du désert, parmi les ruines d'une ville changée en sel, une femme aux yeux blancs vous attend. "Je t'ai vu venir depuis le col." Trois bêtes se dressent dans le sable.

Après la victoire :

> La Pythie s'effondre dans le sel, un dernier mot aux lèvres.

Hilde, avant le combat contre son gardien :

> Sous une dune, une caravane ensevelie : la marchande Hilde est morte assise contre sa charrette, une sacoche de baumes serrée contre elle. Un monstre rôde autour du corps et se jette sur vous.

Après la victoire :

> L'âme de Hilde, marchande de baumes, s'échappe enfin de la caravane. Vous la recueillez pour Malvoise.

## 41. Second type des classes

Décidé le 26/10/2026, pour l'étape 145 (gamedesign.md, section 4.4), choix de l'utilisateur. Dès le niveau de combat 25 (arme et armure du palier 2), un humain, héros du joueur ou de l'IA, a aussi le second type de sa classe :

| Classe | Types sous le niveau 25 | Types dès le niveau 25 |
|---|---|---|
| Guerrier | Humain | Humain, Combat |
| Assassin | Humain | Humain, Ténèbres |
| Mage | Humain | Humain, Psychique |
| Templier | Humain | Humain, Lumière |

Humains concernés aujourd'hui : l'Inquisiteur (niveau 25), le Porte-étendard (35) et la Pythie de sel (40) ; les adversaires des paliers 7 à 10 de la Fosse aux damnés (27 à 35), depuis l'étape 148 (paliers 6 à 10 avant). Le brigand (10), le Bourreau (15) et les paliers 1 à 6 (10 à 24) restent Humain seul.

Textes proposés sans validation (gamedesign.md, Q114) : la fiche du héros commence par "Aldric, Guerrier, Humain, Combat, niveau de combat 25 : {PV}/{PV max} PV." et, sous le niveau 25, finit par "Au niveau de combat 25, il prendra aussi le type Combat."

## 42. Sprites des monstres et des humains

Décidé le 26/10/2026, pour les étapes 152 à 154 (gamedesign.md, section 5.1), choix de l'utilisateur : du pixel art dessiné par Claude, style validé sur les cinq premières espèces.

Chaque espèce a un fichier texte, `crates/bersemon-discord/sprites/{identifiant}.txt`, modifiable à la main :

- des lignes de commentaire, qui commencent par `#` (le nom de l'espèce) ;
- `palette`, puis une couleur par ligne : un caractère et sa couleur en hexadécimal, par exemple `b 8a8a92` ;
- `shiny`, puis les mêmes caractères avec leurs couleurs shiny ;
- `dessin`, puis 24 lignes de 24 caractères ; `.` est transparent.

Le bot rend le dessin en PNG de 144 x 144 pixels, chaque pixel devenant un carré de 6 x 6. Pour voir les sprites hors de Discord : `cargo test -p bersemon-discord ecrire_les_png -- --ignored`, avec la variable BERSEMON_SPRITES qui donne le dossier où écrire les PNG.

Conventions de dessin : contour sombre d'un pixel, lumière venue d'en haut à gauche (une teinte claire en haut, une sombre en bas et à droite), monstre tourné vers la gauche, posé vers le bas de la grille. Le shiny ne change que les couleurs, comme dans Pokémon.

Espèces dessinées à l'étape 152, style validé par l'utilisateur : Salamandre, Crapaud, Mandragore, Chat noir, Loup. Les 60 autres à l'étape 154, sur le même modèle : les formes évoluées reprennent la silhouette et les couleurs de leur forme de base en plus imposant ; chaque shiny change de teinte, souvent vers la couleur complémentaire. Dessins et palettes proposés par Claude, à relire espèce par espèce (gamedesign.md, Q115). Un test vérifie que chaque espèce du contenu a son sprite, et qu'aucun sprite ne reste sans espèce : une espèce ajoutée plus tard demande son dessin.

Humains, décidé le 26/10/2026 pour les étapes 155 à 157 (gamedesign.md, section 5.1) : même format, sans section `shiny`, puisqu'un humain n'est jamais shiny.

- `crates/bersemon-discord/sprites/heros/{classe}.txt` : guerrier, mage, templier, assassin. En combat, le héros d'un joueur a l'espèce technique `heros-{classe}`. Depuis l'étape 160, ces quatre fichiers ont une section `shiny`, pour le héros shiny (section 43) : armure dorée et cape bleue pour le Guerrier, robe pourpre pour le Mage, armure noire et croix dorée pour le Templier, tenue blanche et foulard sarcelle pour l'Assassin.
- `crates/bersemon-discord/sprites/humains/{identifiant}.txt` : les 5 humains du récit et les 10 adversaires de la Fosse aux damnés. En combat, un humain de l'IA a l'espèce technique `humain-{identifiant}`, par exemple `humain-brigand`.

Les 4 classes et les 5 humains du récit sont dessinés à l'étape 156, style validé par l'utilisateur ; les 10 adversaires de la Fosse à l'étape 157, sur le même modèle. Dessins et couleurs proposés par Claude (gamedesign.md, Q116).

Images composées : en combat, les deux combattants actifs face à face, celui du joueur à gauche et retourné, avec un écart de 4 pixels de la grille (312 x 144 pixels) ; au choix de la classe, les quatre classes côte à côte (648 x 144 pixels). Chaque sprite garde sa palette, d'au plus 63 couleurs.

## 43. Choc chromatique et héros shiny

Décidé le 26/10/2026, pour les étapes 158 à 160 (gamedesign.md, sections 4.1, 5.1 et 13.7). Textes proposés par Claude sans validation (Q117). Renommée le 06/11/2026, pour l'étape 173 (choix de l'utilisateur) : l'attaque s'appelait Rayon chromatique, puis Choc stellaire le 06/11/2026 ; elle s'appelle Choc chromatique depuis le 07/11/2026, et son identifiant reste rayon-chromatique, pour les sauvegardes.

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| rayon-chromatique | Choc chromatique | Stellaire | P ou S | 80 | 100 | |

La catégorie suit le monstre qui l'utilise : physique si son Attaque dépasse son Attaque spéciale, spéciale sinon. Seul un monstre shiny la connaît.

Textes :

- héros né shiny : "Une lueur étrange émane de vous : vous êtes shiny, une chance sur 512." ;
- confirmation du service : "Faire rendre votre héros shiny par Malvoise contre un Philtre chromatique ? Seule sa couleur changera." ;
- héros rendu shiny : "Malvoise vous fait boire le Philtre chromatique ; votre sang s'altère, et une lueur nouvelle vous enveloppe. Vous êtes désormais shiny." ;
- monstre rendu shiny avec une place libre, à la suite du texte existant : "Il apprend Choc chromatique."

## 44. Port des Noyés, Mines de Fer-Noir et leurs annexes

Décidés le 27/10/2026, pour l'étape 163, premier lot de la suite de l'aventure (gamedesign.md, sections 4.3, 9.1, 9.4, 9.9 et 10.7). Les zones, leurs niveaux, les Apôtres, les humains gardiens, les noms des espèces et leurs types, les annexes et leur ouverture sont des choix de l'utilisateur ou ont été validés par lui (lot 1) ; les valeurs (stats, attaques, rendements, coûts), les textes et les réglages issus de la mesure sont proposés par Claude sans validation (gamedesign.md, Q118). Stats et attaques réglées avec l'outil d'équilibrage (equilibrage.md, section 14).

Zones :

| Identifiant | Nom | Niveaux | Étapes | Taille des groupes | Espèces | Monstre rare | Humain | Apôtre |
|---|---|---|---|---|---|---|---|---|
| port-des-noyes | Port des Noyés | 46 à 51 | 5 | 1 à 3 | Crabe nécrophage, Sirène putride, Goéland charognard | Hydre des marées (hostile) | Contrebandier | Amiral noyé (53) |
| mines-de-fer-noir | Mines de Fer-Noir | 52 à 57 | 5 | 1 à 3 | Golem de fer noir, Mineur possédé, Grisou ardent | Dragon de rouille (hostile) | Contremaître | Prévôt des fosses (59) |

Le Schnoz se cache dans les deux, comme partout. Pas de marchand mort ni de pêcheur. Météos : pluie, tempête et cyclone au Port, seule zone de mer ; grêle et tempête dans les Mines. La Sirène putride et le Mineur possédé sont nocturnes, le Goéland charognard diurne.

| Zone | Expédition automatique (durée de base, dégâts, XP, potions, argent) | Coffres (argent) | Capsules des coffres (prix chez Malvoise) |
|---|---|---|---|
| Port des Noyés | 7 h, 340 à 650, 3800 à 5600, 1 à 3, 280 à 460 | 220 à 340 | Marée noire, Pince broyeuse, Chant des noyés (600) |
| Mines de Fer-Noir | 8 h, 400 à 760, 4600 à 6800, 1 à 3, 330 à 540 | 260 à 400 | Marteau de forge, Coup de grisou, Coup de pioche (650) |

Espèces (stats dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse). Au codex, elles viennent après le Dragon des brumes, dans l'ordre du tableau. Aucune n'évolue.

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Rareté | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|---|
| crabe-necrophage | Crabe nécrophage | Eau, Acier | soutien | 72 / 78 / 90 / 30 / 60 / 30 (360) | 145 | commune | Carapace, Férocité | Plongeon (1), Coup de garde (1), Déferlante (25), Taille d'acier (35), Pince broyeuse (58) |
| sirene-putride | Sirène putride | Eau, Spectre | offensif | 58 / 35 / 48 / 92 / 70 / 64 (367) | 145 | commune | Sorcellerie, Instinct | Lueur trompeuse (1), Frisson d'outre-tombe (1), Lamentation (30), Chant des noyés (40), Marée noire (58) |
| goeland-charognard | Goéland charognard | Vol, Ténèbres | offensif | 61 / 92 / 48 / 38 / 47 / 104 (390) | 145 | commune | Célérité, Traqueur | Coup de bec (1), Morsure (1), Pique (25), Croc noir (35), Bec funeste (45), Rapine (58) |
| hydre-des-marees | Hydre des marées | Eau, Dragon | offensif | 80 / 85 / 76 / 88 / 72 / 64 (465) | 250 | rare, hostile | Affinité, Férocité | Souffle draconique (1), Lame de fond (1), Furie draconique (30), Trombe (40), Déluge draconique (60) |
| nautile-des-abysses | Nautile des abysses | Eau, Roche | soutien | 80 / 62 / 112 / 76 / 99 / 36 (465) | 245 | rare, pacifique (Épave) | Carapace, Volonté | Jet d'encre (1), Chute de pierres (1), Éboulement (35), Marée noire (60) |
| amiral-noye | Amiral noyé | Obscur | offensif | 96 / 103 / 90 / 93 / 86 / 62 (530) | 360 | Apôtre | Férocité, Carapace | Étreinte obscure (1), Lame de fond (1), Hurlement des abysses (20), Trombe (35), Ancre des damnés (45), Abîme liquide (52) |
| golem-de-fer-noir | Golem de fer noir | Acier, Roche | soutien | 76 / 82 / 90 / 30 / 55 / 20 (353) | 160 | commune | Carapace, Volonté | Coup de garde (1), Chute de pierres (1), Taille d'acier (30), Éboulement (40), Marteau de forge (62) |
| mineur-possede | Mineur possédé | Spectre, Combat | offensif | 67 / 93 / 59 / 46 / 54 / 71 (390) | 160 | commune | Férocité, Traqueur | Charge spectrale (1), Frappe d'os (1), Corde du gibet (30), Griffes spectrales (50), Coup de pioche (62) |
| grisou-ardent | Grisou ardent | Feu, Poison | offensif | 52 / 40 / 49 / 92 / 55 / 84 (372) | 160 | commune | Sorcellerie, Célérité | Crachat de braises (1), Spores putrides (1), Brasier (30), Venin noir (40), Coup de grisou (62) |
| dragon-de-rouille | Dragon de rouille | Dragon, Acier | offensif | 78 / 85 / 80 / 66 / 66 / 55 (430) | 275 | rare, hostile | Affinité, Carapace | Souffle draconique (1), Coup de garde (1), Furie draconique (30), Taille d'acier (40), Souffle de rouille (64) |
| gemmame | Gemmâme | Roche, Féérique | soutien | 66 / 50 / 100 / 82 / 100 / 42 (440) | 270 | rare, pacifique (Filon) | Volonté, Instinct | Poussière d'elfe (1), Chute de pierres (1), Rayon de lune (35), Éclat de gemme (64) |
| prevot-des-fosses | Prévôt des fosses | Obscur | offensif | 98 / 108 / 98 / 84 / 90 / 62 (540) | 400 | Apôtre | Férocité, Volonté | Étreinte obscure (1), Chute de pierres (1), Hurlement des abysses (20), Séisme (40), Chaîne du prévôt (50), Gouffre sans fond (58) |

Les attaques de puissance 90 et plus des communes et des rares ne s'apprennent qu'après leur zone (niveaux 58 à 64) : elles récompensent le joueur qui capture et entraîne ces espèces, sans durcir les combats de la zone (equilibrage.md, section 14). Le rendement d'XP suit la courbe en n x n x n : environ 3 fois le niveau moyen de la zone, contre 2,8 dans le Désert.

Attaques nouvelles :

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| pince-broyeuse | Pince broyeuse | Eau | P | 90 | 90 | |
| chant-des-noyes | Chant des noyés | Spectre | S | 80 | 100 | paralysie 20 % |
| maree-noire | Marée noire | Eau | S | 90 | 95 | |
| rapine | Rapine | Ténèbres | P | 90 | 95 | |
| deluge-draconique | Déluge draconique | Dragon | S | 100 | 85 | |
| jet-d-encre | Jet d'encre | Eau | S | 65 | 100 | paralysie 10 % |
| ancre-des-damnes | Ancre des damnés | Obscur | P | 90 | 95 | |
| abime-liquide | Abîme liquide | Obscur | S | 95 | 90 | paralysie 10 % |
| marteau-de-forge | Marteau de forge | Acier | P | 95 | 90 | |
| coup-de-pioche | Coup de pioche | Combat | P | 90 | 95 | |
| coup-de-grisou | Coup de grisou | Feu | S | 100 | 85 | |
| souffle-de-rouille | Souffle de rouille | Acier | S | 95 | 90 | poison 20 % |
| eclat-de-gemme | Éclat de gemme | Roche | S | 90 | 95 | |
| chaine-du-prevot | Chaîne du prévôt | Obscur | P | 90 | 95 | paralysie 20 % |
| gouffre-sans-fond | Gouffre sans fond | Obscur | S | 95 | 90 | |

Attaques d'œuf, sur le modèle de la section 28 :

| Famille | Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|---|
| Crabe nécrophage | pince-etau | Pince-étau | Acier | P | 80 | 100 | |
| Sirène putride | lamento-des-abysses | Lamento des abysses | Spectre | S | 80 | 100 | |
| Goéland charognard | pique-du-charognard | Pique du charognard | Vol | P | 80 | 100 | |
| Hydre des marées | morsure-des-sept-tetes | Morsure des sept têtes | Dragon | P | 80 | 100 | |
| Nautile des abysses | spirale-abyssale | Spirale abyssale | Eau | S | 80 | 100 | |
| Golem de fer noir | poing-de-fer-noir | Poing de fer noir | Acier | P | 80 | 100 | |
| Mineur possédé | hantise-du-filon | Hantise du filon | Spectre | P | 75 | 100 | paralysie 20 % |
| Grisou ardent | etincelle-de-grisou | Étincelle de grisou | Feu | S | 80 | 100 | |
| Dragon de rouille | ecaille-rouillee | Écaille rouillée | Acier | P | 75 | 100 | poison 30 % |
| Gemmâme | prisme-feerique | Prisme féerique | Féérique | S | 80 | 100 | |

Ressources : Carapace de crabe (carapace-de-crabe), Écaille de sirène (ecaille-de-sirene), Plume de goéland (plume-de-goeland), Éclat de fer noir (eclat-de-fer-noir), Pioche maudite (pioche-maudite) et Fiole de grisou (fiole-de-grisou), 5 pièces d'argent chacune ; Croc d'hydre (croc-d-hydre) et Écaille de rouille (ecaille-de-rouille), 15. Le Nautile et la Gemmâme, pacifiques, n'en ont pas.

Humains gardiens. Leur héros combat au niveau maximal de la zone, à part de son équipement, qui ne lui donne que ses attaques et ses bonus ; IA relevée ; valeurs internes de leurs monstres à 15, première habileté de leur espèce :

| Identifiant | Nom | Classe | Héros (niveau, arme, armure) | Monstres | Victoire |
|---|---|---|---|---|---|
| contrebandier | Contrebandier | Assassin | 51, Crocs du champion, Manteau de cendre | Sirène putride (51), Hydre des marées (52) | 600 pièces d'argent, puis l'indice 9 |
| contremaitre | Contremaître | Guerrier | 57, Hache de fer noir, Cuirasse des abysses | Golem de fer noir (57), Mineur possédé (57), Dragon de rouille (58) | 700 pièces d'argent, puis l'indice 11 |

Le Contrebandier n'a que deux monstres : avec trois, un Assassin de son niveau en dernier faisait de lui un mur (21 % de victoires au premier essai, contre 39 % avec deux ; equilibrage.md, section 14).

Annexes (gamedesign.md, section 9.9). Trois étapes, combat 60 %, coffre 30 %, monstre rare 10 % ; les espèces et les météos de leur zone ; le Schnoz caché ; ni humain, ni Apôtre, ni marchand mort, ni expédition automatique. Leurs coffres ont l'argent indiqué, les capsules de leur zone, et un trésor à coup sûr :

| Identifiant | Nom | Ouverte depuis | Niveaux | Monstre rare | Coffres (argent) |
|---|---|---|---|---|---|
| epave-du-leviathan | Épave du Léviathan | Port des Noyés | 50 à 53 | Nautile des abysses (pacifique) | 300 à 450 |
| filon-ecarlate | Filon écarlate | Mines de Fer-Noir | 56 à 59 | Gemmâme (pacifique) | 340 à 500 |

Trésor d'un coffre d'annexe, tiré selon son poids sur 100 : Philtre chromatique 5, capsule d'habileté 35, Pierre de braise 15, Pierre de lune 15, chacun des six boosters 5.

Équipement (gamedesign.md, section 4.3). Bonus aux monstres : +12 % d'Attaque et d'Attaque spéciale pour les armes du palier 5 ; +20 % puis +25 % de Défense et de Défense spéciale pour les armures des paliers 4 et 5. Chaque arme donne l'attaque principale de son palier, celle de sa classe et celle du second type de sa classe (section 41). Niveau de combat : 45 avec l'arme et l'armure du palier 4, 55 au palier 5.

| Identifiant | Objet | Classe | Palier | Coût | Durée |
|---|---|---|---|---|---|
| hache-de-fer-noir | Hache de fer noir (arme) | Guerrier | 5 | 8 Éclat de fer noir, 6 Pioche maudite, 1 Écaille de rouille | 12 h |
| dagues-de-fer-noir | Dagues de fer noir (arme) | Assassin | 5 | 8 Éclat de fer noir, 6 Pioche maudite, 1 Écaille de rouille | 12 h |
| baton-de-grisou | Bâton de grisou (arme) | Mage | 5 | 8 Fiole de grisou, 6 Pioche maudite, 1 Écaille de rouille | 12 h |
| masse-de-fer-noir | Masse de fer noir (arme) | Templier | 5 | 8 Fiole de grisou, 6 Pioche maudite, 1 Écaille de rouille | 12 h |
| cuirasse-des-abysses | Cuirasse des abysses (armure) | Guerrier | 4 | 10 Carapace de crabe, 6 Écaille de sirène, 1 Croc d'hydre | 10 h |
| cuir-de-requin | Cuir de requin (armure) | Assassin | 4 | idem | 10 h |
| robe-des-marees | Robe des marées (armure) | Mage | 4 | idem | 10 h |
| plates-nacrees | Plates nacrées (armure) | Templier | 4 | idem | 10 h |
| plates-de-fer-noir | Plates de fer noir (armure) | Guerrier | 5 | 10 Éclat de fer noir, 6 Carapace de crabe, 1 Écaille de rouille | 14 h |
| maille-de-fer-noir | Maille de fer noir (armure) | Assassin | 5 | idem | 14 h |
| robe-de-grisou | Robe de grisou (armure) | Mage | 5 | idem | 14 h |
| harnois-de-fer-noir | Harnois de fer noir (armure) | Templier | 5 | idem | 14 h |

Attaques principales du palier 5, de type Humain, puissance 85, précision 100 : Fend-armure (fend-armure, Guerrier, P), Coup mortel (coup-mortel, Assassin, P), Tempête arcanique (tempete-arcanique, Mage, S), Colère du ciel (colere-du-ciel, Templier, S).

Sprites : un par espèce, normal et shiny, et un par humain gardien (section 42).

Textes :

> Indice 8 (remplace le texte de la section 40) : la Pythie vaincue a parlé avant de se taire : un autre Apôtre a reçu la marque du Démiurge. Il attend au-delà du désert, sur la côte où la mer rend ses morts : le Port des Noyés.

> Indice 9 : le contrebandier vaincu a parlé : l'Amiral noyé attend au bout de la jetée, sur le pont d'un vaisseau coulé que la marée découvre chaque nuit.

> Indice 10 : dans la cabine de l'Amiral, un livre de bord scellé du même sceau : à l'est, les Mines de Fer-Noir, où les mineurs creusent encore après leur mort. Un cinquième Apôtre y règne.

> Indice 11 : le contremaître vaincu a parlé : le Prévôt des fosses juge les damnés au fond du dernier puits, là où le fer noir saigne.

Contrebandier, avant le combat, puis après la victoire :

> Sur le quai, entre des caisses éventrées, un contrebandier compte ses pièces à la lueur d'une lanterne. Il siffle : deux bêtes surgissent de l'eau noire. Personne n'approche de la jetée sans payer.

> Le contrebandier s'effondre sur les pavés mouillés, sa bourse répandue dans une flaque.

Contremaître, avant le combat, puis après la victoire :

> Au fond de la galerie, un contremaître au fouet de chaîne surveille des mineurs qui ne respirent plus. Trois bêtes sortent de l'ombre des boisages à son signal.

> Le contremaître tombe à genoux, et les mineurs morts lâchent enfin leurs pioches.

Amiral noyé, avant le combat, épilogue, âme et carnet :

> Au bout de la jetée, la marée se retire sur l'épave d'un vaisseau de guerre. Sur le pont, un officier aux poumons pleins d'eau se dresse : l'Amiral noyé, Apôtre du Démiurge, qui a mené son équipage par le fond. Il n'y aura pas de fuite.

> L'Amiral noyé s'effondre sur le pont, et la mer reprend son navire. Le même sceau, une main ouverte, est gravé sur la barre. Quatre Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.

> L'âme de l'Amiral noyé s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

> Amiral noyé, abattu au Port des Noyés.

Prévôt des fosses, avant le combat, épilogue, âme et carnet :

> Au fond du dernier puits, sur un trône taillé dans le fer noir, un juge enchaîné à ses condamnés lève la tête : le Prévôt des fosses, Apôtre du Démiurge. Il n'y aura pas de fuite.

> Le Prévôt des fosses s'effondre, et ses chaînes se brisent avec lui. Le même sceau, une main ouverte, est gravé dans le fer de son trône. Cinq Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.

> L'âme du Prévôt des fosses s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

> Prévôt des fosses, abattu dans les Mines de Fer-Noir.

Cartes des annexes, trouvées dans un coffre, suivies de "Un nouveau lieu s'ouvre : " et du nom de l'annexe :

> Épave du Léviathan : Au fond du coffre, une carte marine détrempée marque d'une croix une épave au large du port, là où une bête immense aurait sombré avec un navire.

> Filon écarlate : Au fond du coffre, un plan de mine griffonné montre une galerie murée, où le filon brille d'un rouge de sang.

## 45. Sylve écarlate, Catacombes de Sainte-Ombre et leurs annexes

Décidés le 01/11/2026, pour l'étape 165, deuxième lot de la suite de l'aventure (gamedesign.md, sections 4.3, 9.1, 9.4, 9.9 et 10.7). Les zones, leurs niveaux, les Apôtres et les humains gardiens viennent du fil validé ; les noms et les types des espèces, les rares, les annexes et les classes des humains ont été validés par l'utilisateur (lot 2) ; les valeurs, les textes et les réglages issus de la mesure sont proposés par Claude sans validation (gamedesign.md, Q118 ; equilibrage.md, section 15). Mêmes règles qu'au lot 1 (section 44) : espèces recalées à niveau égal sur les précédentes, attaques de puissance 90 et plus après la zone, rendement d'XP d'environ 3 fois le niveau moyen de la zone.

Zones :

| Identifiant | Nom | Niveaux | Étapes | Taille des groupes | Espèces | Monstre rare | Humain | Apôtre |
|---|---|---|---|---|---|---|---|---|
| sylve-ecarlate | Sylve écarlate | 58 à 62 | 5 | 1 à 3 | Ronce vampire, Mante écarlate, Sanglier enragé | Dryade sanglante (hostile) | Chasseresse | Duchesse sanguine (64) |
| catacombes-de-sainte-ombre | Catacombes de Sainte-Ombre | 63 à 67 | 5 | 1 à 3 | Rat des ossuaires, Moine psalmodiant, Araignée des cryptes | Liche (hostile) | Fossoyeur | Évêque des os (69) |

Le Schnoz se cache dans les deux. Pas de marchand mort ni de pêcheur. Météos : pluie, tempête et canicule dans la Sylve ; pluie et tempête dans les Catacombes. La Mante écarlate est diurne, le Sanglier enragé et le Moine psalmodiant nocturnes.

| Zone | Expédition automatique (durée de base, dégâts, XP, potions, argent) | Coffres (argent) | Capsules des coffres (prix chez Malvoise) |
|---|---|---|---|
| Sylve écarlate | 9 h, 460 à 870, 5600 à 8200, 1 à 3, 380 à 620 | 300 à 460 | Faux de mante, Charge du sanglier, Étreinte sanguine (700) |
| Catacombes de Sainte-Ombre | 10 h, 520 à 980, 6600 à 9600, 1 à 3, 430 à 700 | 340 à 520 | Peste noire, Requiem, Morsure de l'araignée (750) |

Espèces (stats dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse). Au codex, elles viennent après le Prévôt des fosses, dans l'ordre du tableau. Aucune n'évolue.

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Rareté | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|---|
| ronce-vampire | Ronce vampire | Plante, Poison | soutien | 84 / 74 / 88 / 62 / 72 / 15 (395) | 180 | commune | Carapace, Sorcellerie | Feuille tranchante (1), Morsure venimeuse (1), Ronces (30), Lianes étrangleuses (40), Croc de venin (50), Étreinte sanguine (68) |
| mante-ecarlate | Mante écarlate | Insecte, Combat | offensif | 62 / 100 / 60 / 40 / 48 / 75 (385) | 180 | commune | Férocité, Célérité | Bourdonnement (1), Frappe d'os (1), Nuée (30), Épée des morts (40), Faux de mante (45), Déchiquetage (66) |
| sanglier-enrage | Sanglier enragé | Normal, Sol | offensif | 85 / 95 / 70 / 30 / 45 / 60 (385) | 180 | commune | Férocité, Volonté | Ruade (1), Secousse (1), Curée (30), Charge du sanglier (45), Séisme (66) |
| dryade-sanglante | Dryade sanglante | Plante, Psychique | offensif | 78 / 52 / 74 / 115 / 88 / 73 (480) | 305 | rare, hostile | Sorcellerie, Affinité | Feuille tranchante (1), Onde psychique (1), Hurlement de mandragore (35), Ronces maudites (45), Sève écarlate (68) |
| phalene-d-aube | Phalène d'aube | Insecte, Féérique | soutien | 70 / 45 / 75 / 95 / 95 / 70 (450) | 300 | rare, pacifique (Autel) | Instinct, Volonté | Bourdonnement (1), Poussière d'elfe (1), Lumière d'aube (35), Rayon de lune (45), Aile poudrée (70) |
| duchesse-sanguine | Duchesse sanguine | Obscur | offensif | 96 / 92 / 88 / 112 / 92 / 65 (545) | 430 | Apôtre | Sorcellerie, Férocité | Étreinte obscure (1), Morsure (1), Hurlement des abysses (20), Ronces maudites (35), Baiser de sang (45), Psaume impie (60) |
| rat-des-ossuaires | Rat des ossuaires | Normal, Poison | offensif | 70 / 90 / 58 / 40 / 52 / 85 (395) | 195 | commune | Célérité, Pillard | Griffure (1), Morsure venimeuse (1), Curée (30), Croc de venin (40), Peste noire (71) |
| moine-psalmodiant | Moine psalmodiant | Son, Spectre | soutien | 80 / 45 / 75 / 85 / 85 / 15 (385) | 195 | commune | Volonté, Sorcellerie | Glas (1), Lueur trompeuse (1), Lamentation (30), Tocsin (40), Flamme funèbre (50), Requiem (71) |
| araignee-des-cryptes | Araignée des cryptes | Insecte, Roche | offensif | 70 / 92 / 85 / 35 / 55 / 48 (385) | 195 | commune | Traqueur, Carapace | Bourdonnement (1), Griffe de pierre (1), Chute de pierres (30), Morsure de l'araignée (40), Éboulement (71) |
| liche | Liche | Psychique, Spectre | offensif | 66 / 42 / 62 / 108 / 90 / 67 (435) | 330 | rare, hostile | Sorcellerie, Volonté | Onde psychique (1), Lueur trompeuse (1), Flamme funèbre (35), Malédiction mortelle (50), Jugement (72) |
| ange-de-pierre | Ange de pierre | Roche, Lumière | soutien | 85 / 75 / 110 / 60 / 100 / 20 (450) | 325 | rare, pacifique (Crypte) | Carapace, Volonté | Lueur sacrée (1), Griffe de pierre (1), Lance de lumière (35), Chute de pierres (45), Jugement de pierre (74) |
| eveque-des-os | Évêque des os | Obscur | offensif | 100 / 110 / 98 / 95 / 90 / 57 (550) | 465 | Apôtre | Volonté, Carapace | Étreinte obscure (1), Frappe d'os (1), Hurlement des abysses (20), Épée des morts (35), Flagellation (50), Ossuaire sacré (65) |

Attaques nouvelles :

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| etreinte-sanguine | Étreinte sanguine | Plante | P | 90 | 95 | |
| faux-de-mante | Faux de mante | Insecte | P | 85 | 95 | |
| charge-du-sanglier | Charge du sanglier | Normal | P | 85 | 95 | |
| seve-ecarlate | Sève écarlate | Plante | S | 95 | 90 | |
| aile-poudree | Aile poudrée | Insecte | S | 90 | 95 | paralysie 10 % |
| baiser-de-sang | Baiser de sang | Obscur | S | 90 | 95 | |
| peste-noire | Peste noire | Poison | P | 90 | 95 | poison 30 % |
| requiem | Requiem | Son | S | 95 | 90 | |
| morsure-de-l-araignee | Morsure de l'araignée | Insecte | P | 80 | 100 | poison 20 % |
| malediction-mortelle | Malédiction mortelle | Spectre | S | 85 | 95 | |
| jugement-de-pierre | Jugement de pierre | Roche | P | 95 | 90 | |
| ossuaire-sacre | Ossuaire sacré | Obscur | P | 95 | 90 | |

Attaques d'œuf, sur le modèle de la section 28 :

| Famille | Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|---|
| Ronce vampire | racines-assoiffees | Racines assoiffées | Plante | P | 80 | 100 | |
| Mante écarlate | danse-de-la-mante | Danse de la mante | Insecte | P | 80 | 100 | |
| Sanglier enragé | coup-de-boutoir | Coup de boutoir | Sol | P | 80 | 100 | |
| Dryade sanglante | chant-de-la-dryade | Chant de la dryade | Psychique | S | 80 | 100 | |
| Phalène d'aube | poudre-d-aube | Poudre d'aube | Féérique | S | 75 | 100 | paralysie 20 % |
| Rat des ossuaires | nuee-de-rats | Nuée de rats | Normal | P | 80 | 100 | |
| Moine psalmodiant | psalmodie | Psalmodie | Son | S | 80 | 100 | |
| Araignée des cryptes | toile-des-cryptes | Toile des cryptes | Insecte | P | 75 | 100 | paralysie 20 % |
| Liche | phylactere | Phylactère | Psychique | S | 80 | 100 | |
| Ange de pierre | aube-de-pierre | Aube de pierre | Lumière | S | 80 | 100 | |

Ressources : Épine de ronce (epine-de-ronce), Lame de mante (lame-de-mante), Défense de sanglier (defense-de-sanglier), Queue de rat (queue-de-rat), Chapelet d'os (chapelet-d-os) et Soie des cryptes (soie-des-cryptes), 5 pièces d'argent chacune ; Sève de dryade (seve-de-dryade) et Phylactère brisé (phylactere-brise), 15.

Humains gardiens. Héros au niveau maximal de la zone, à part de son équipement ; IA relevée ; valeurs internes de leurs monstres à 15, première habileté de leur espèce :

| Identifiant | Nom | Classe | Héros (niveau, arme, armure) | Monstres | Victoire |
|---|---|---|---|---|---|
| chasseresse | Chasseresse | Assassin | 62, Dagues de fer noir, Maille de fer noir | Mante écarlate (62), Dryade sanglante (63) | 800 pièces d'argent, puis l'indice 13 |
| fossoyeur | Fossoyeur | Mage | 67, Crosse des catacombes, Robe de grisou | Rat des ossuaires (67), Moine psalmodiant (67), Liche (68) | 900 pièces d'argent, puis l'indice 15 |

La Chasseresse, Assassin comme le Contrebandier, n'a que deux monstres (section 44).

Annexes (gamedesign.md, section 9.9), sur le modèle de la section 44 :

| Identifiant | Nom | Ouverte depuis | Niveaux | Monstre rare | Coffres (argent) |
|---|---|---|---|---|---|
| autel-des-ronces | Autel des ronces | Sylve écarlate | 61 à 64 | Phalène d'aube (pacifique) | 400 à 600 |
| crypte-des-saints | Crypte des saints | Catacombes de Sainte-Ombre | 66 à 69 | Ange de pierre (pacifique) | 450 à 680 |

Équipement du palier 6 (gamedesign.md, section 4.3) : +14 % d'Attaque et d'Attaque spéciale pour les armes, +30 % de Défense et de Défense spéciale pour les armures ; niveau de combat 65 avec l'arme et l'armure du palier 6.

| Identifiant | Objet | Classe | Coût | Durée |
|---|---|---|---|---|
| hache-du-bourreau-d-os | Hache du bourreau d'os (arme) | Guerrier | 8 Soie des cryptes, 6 Lame de mante, 1 Phylactère brisé | 14 h |
| lames-de-soie | Lames de soie (arme) | Assassin | 8 Soie des cryptes, 6 Lame de mante, 1 Phylactère brisé | 14 h |
| crosse-des-catacombes | Crosse des catacombes (arme) | Mage | 8 Chapelet d'os, 6 Épine de ronce, 1 Phylactère brisé | 14 h |
| masse-ecarlate | Masse écarlate (arme) | Templier | 8 Chapelet d'os, 6 Épine de ronce, 1 Phylactère brisé | 14 h |
| plates-du-sanglier | Plates du sanglier (armure) | Guerrier | 10 Défense de sanglier, 6 Queue de rat, 1 Sève de dryade | 16 h |
| cuir-ecarlate | Cuir écarlate (armure) | Assassin | idem | 16 h |
| robe-de-ronces | Robe de ronces (armure) | Mage | idem | 16 h |
| harnois-des-saints | Harnois des saints (armure) | Templier | idem | 16 h |

Attaques principales du palier 6, de type Humain, puissance 95, précision 100 : Décapitation (decapitation, Guerrier, P), Lame fantôme (lame-fantome, Assassin, P), Arcane interdit (arcane-interdit, Mage, S), Jugement dernier (jugement-dernier, Templier, S).

Sprites : un par espèce, normal et shiny, et un par humain gardien (section 42).

Textes :

> Indice 12 : dans la geôle du Prévôt, un registre de condamnés scellé du même sceau : au nord, la Sylve écarlate, où les arbres boivent le sang des voyageurs. Une sixième Apôtre y tient sa cour.

> Indice 13 : la chasseresse vaincue a parlé : la Duchesse sanguine attend au cœur de la Sylve, dans un manoir que les ronces ont dévoré.

> Indice 14 : parmi les parures de la Duchesse, une bague gravée du même sceau : sous la ville morte de Sainte-Ombre, des catacombes où un évêque bénit encore les os. Un septième Apôtre y prêche.

> Indice 15 : le fossoyeur vaincu a parlé : l'Évêque des os officie au fond des catacombes, dans une chapelle bâtie de crânes.

Chasseresse, avant le combat, puis après la victoire :

> Une flèche se plante dans l'écorce, à un doigt de votre visage. Entre les troncs rouges, une chasseresse encoche déjà la suivante. Deux bêtes surgissent des fourrés à son sifflement.

> La chasseresse tombe à genoux parmi les feuilles rouges, son arc brisé.

Fossoyeur, avant le combat, puis après la victoire :

> Entre deux rangées de niches funéraires, un fossoyeur appuyé sur sa pelle murmure des noms. Les morts qu'il a enterrés se relèvent, et trois bêtes sortent des tombes.

> Le fossoyeur s'effondre dans la fosse qu'il venait de creuser.

Duchesse sanguine, avant le combat, épilogue, âme et carnet :

> Au cœur du manoir dévoré par les ronces, une femme en robe de bal boit dans une coupe pleine de sang. La Duchesse sanguine, Apôtre du Démiurge, repose la coupe et sourit. Il n'y aura pas de fuite.

> La Duchesse sanguine s'effondre, et les ronces se flétrissent autour d'elle. Le même sceau, une main ouverte, est gravé sur sa coupe. Six Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.

> L'âme de la Duchesse sanguine s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

> Duchesse sanguine, abattue dans la Sylve écarlate.

Évêque des os, avant le combat, épilogue, âme et carnet :

> Dans la chapelle de crânes, un prélat aux os nus sous sa chasuble lève sa crosse : l'Évêque des os, Apôtre du Démiurge, bénit son troupeau de morts. Il n'y aura pas de fuite.

> L'Évêque des os se disloque au pied de son autel, et le silence retombe sur les catacombes. Le même sceau, une main ouverte, est gravé sur sa crosse. Sept Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.

> L'âme de l'Évêque des os s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

> Évêque des os, abattu dans les Catacombes de Sainte-Ombre.

Cartes des annexes :

> Autel des ronces : Au fond du coffre, un parchemin taché de sang montre le chemin d'une clairière cachée, où un autel de ronces attend ses offrandes.

> Crypte des saints : Au fond du coffre, une clé de bronze et un plan jauni mènent à une crypte scellée, où reposeraient les saints de Sainte-Ombre.

## 46. Plaine du Massacre, Haute-Cité déchue et leurs annexes

Décidés le 02/11/2026, pour l'étape 166, troisième lot de la suite de l'aventure (gamedesign.md, sections 4.3, 9.1, 9.4, 9.9 et 10.7). Les zones, leurs niveaux, les Apôtres et les humains gardiens viennent du fil validé ; les noms et les types des espèces, les rares, les annexes, les classes des humains et le palier 7 de l'équipement ont été validés par l'utilisateur (lot 3) ; les valeurs, les textes et les réglages issus de la mesure sont proposés par Claude sans validation (gamedesign.md, Q118 ; equilibrage.md, section 16). Mêmes règles qu'aux lots 1 et 2 (sections 44 et 45).

Zones :

| Identifiant | Nom | Niveaux | Étapes | Taille des groupes | Espèces | Monstre rare | Humain | Apôtre |
|---|---|---|---|---|---|---|---|---|
| plaine-du-massacre | Plaine du Massacre | 68 à 72 | 5 | 1 à 3 | Destrier spectral, Vautour des charniers, Mâtin de guerre | Griffon cuirassé (hostile) | Capitaine mercenaire | Connétable boucher (74) |
| haute-cite-dechue | Haute-Cité déchue | 73 à 77 | 5 | 1 à 3 | Automate brisé, Bibliophage, Œil errant | Archonte foudroyé (hostile) | Archiviste | Chancelier aux mille yeux (79) |

Le Schnoz se cache dans les deux. Pas de marchand mort ni de pêcheur. Météos : tempête et canicule dans la Plaine ; tempête et grêle dans la Haute-Cité. Le Destrier spectral et l'Œil errant sont nocturnes, le Vautour des charniers diurne.

| Zone | Expédition automatique (durée de base, dégâts, XP, potions, argent) | Coffres (argent) | Capsules des coffres (prix chez Malvoise) |
|---|---|---|---|
| Plaine du Massacre | 11 h, 590 à 1100, 7800 à 11200, 1 à 3, 490 à 780 | 380 à 580 | Galop funèbre, Serres de poussière, Crocs d'acier (800) |
| Haute-Cité déchue | 12 h, 650 à 1230, 9000 à 12800, 1 à 3, 550 à 860 | 420 à 640 | Décharge mécanique, Festin d'encre, Regard hypnotique (850) |

Espèces (stats dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse). Au codex, elles viennent après l'Évêque des os, dans l'ordre du tableau. Aucune n'évolue.

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Rareté | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|---|
| destrier-spectral | Destrier spectral | Normal, Spectre | offensif | 72 / 86 / 58 / 40 / 50 / 68 (374) | 210 | commune | Célérité, Férocité | Ruade (1), Charge spectrale (1), Secousse (30), Griffes spectrales (40), Galop funèbre (50), Charge des morts (76) |
| vautour-des-charniers | Vautour des charniers | Vol, Sol | offensif | 70 / 80 / 56 / 36 / 52 / 66 (360) | 210 | commune | Traqueur, Pillard | Coup de bec (1), Secousse (1), Piqué (30), Bec funeste (40), Serres de poussière (50), Séisme (76) |
| matin-de-guerre | Mâtin de guerre | Combat, Acier | offensif | 78 / 86 / 80 / 30 / 52 / 46 (372) | 210 | commune | Férocité, Carapace | Morsure (1), Coup de garde (1), Taille d'acier (30), Épée des morts (40), Crocs d'acier (50), Déchiquetage (76) |
| griffon-cuirasse | Griffon cuirassé | Vol, Acier | offensif | 75 / 88 / 78 / 45 / 60 / 70 (416) | 355 | rare, hostile | Férocité, Affinité | Coup de bec (1), Coup de garde (1), Bec funeste (35), Taille d'acier (45), Plongeon d'acier (78) |
| licorne-des-cendres | Licorne des cendres | Feu, Féérique | soutien | 75 / 55 / 70 / 90 / 95 / 65 (450) | 350 | rare, pacifique (Tertre) | Volonté, Instinct | Crachat de braises (1), Poussière d'elfe (1), Lumière d'aube (35), Brasier (45), Rayon de lune (55), Corne de cendre (80) |
| connetable-boucher | Connétable boucher | Obscur | offensif | 100 / 105 / 98 / 85 / 90 / 62 (540) | 500 | Apôtre | Férocité, Volonté | Étreinte obscure (1), Frappe d'os (1), Hurlement des abysses (20), Déchiquetage (35), Flagellation (50), Boucherie (70) |
| automate-brise | Automate brisé | Acier, Électrique | soutien | 78 / 68 / 86 / 66 / 58 / 18 (374) | 225 | commune | Carapace, Volonté | Coup de garde (1), Étincelle (1), Onde de choc (30), Taille d'acier (40), Décharge mécanique (50), Foudre (80) |
| bibliophage | Bibliophage | Insecte, Psychique | offensif | 66 / 55 / 62 / 102 / 70 / 45 (400) | 225 | commune | Sorcellerie, Fouineur | Bourdonnement (1), Onde psychique (1), Nuée (30), Festin d'encre (45), Jugement (80) |
| oeil-errant | Œil errant | Psychique, Vol | offensif | 58 / 38 / 52 / 100 / 66 / 76 (390) | 225 | commune | Instinct, Éclaireur | Regard funeste (1), Onde psychique (1), Bourrasque (30), Regard hypnotique (40), Rayon d'éther (80) |
| archonte-foudroye | Archonte foudroyé | Électrique, Psychique | offensif | 72 / 48 / 66 / 105 / 85 / 72 (448) | 380 | rare, hostile | Sorcellerie, Affinité | Étincelle (1), Onde psychique (1), Onde de choc (35), Éclair de l'archonte (45), Jugement (82) |
| muse-de-cristal | Muse de cristal | Son, Féérique | soutien | 70 / 45 / 70 / 95 / 95 / 65 (440) | 375 | rare, pacifique (Bibliothèque) | Volonté, Instinct | Glas (1), Poussière d'elfe (1), Tocsin (35), Rayon de lune (45), Chant de cristal (82) |
| chancelier-aux-mille-yeux | Chancelier aux mille yeux | Obscur | offensif | 96 / 80 / 90 / 110 / 98 / 66 (540) | 530 | Apôtre | Sorcellerie, Volonté | Étreinte obscure (1), Onde psychique (1), Hurlement des abysses (20), Jugement (35), Regard des mille yeux (50), Secret interdit (75) |

Le Destrier spectral, Normal et Spectre, est insensible aux deux types ; il apprend Secousse (Sol) au niveau 30, pour que deux Destriers puissent se blesser (étape 166, voir equilibrage.md, section 16).

Attaques nouvelles :

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| galop-funebre | Galop funèbre | Normal | P | 85 | 95 | |
| charge-des-morts | Charge des morts | Spectre | P | 95 | 90 | |
| serres-de-poussiere | Serres de poussière | Sol | P | 85 | 95 | |
| crocs-d-acier | Crocs d'acier | Acier | P | 85 | 95 | |
| plongeon-d-acier | Plongeon d'acier | Vol | P | 95 | 90 | |
| corne-de-cendre | Corne de cendre | Feu | S | 95 | 90 | |
| boucherie | Boucherie | Obscur | P | 95 | 90 | |
| decharge-mecanique | Décharge mécanique | Électrique | P | 85 | 95 | |
| festin-d-encre | Festin d'encre | Psychique | S | 85 | 95 | |
| regard-hypnotique | Regard hypnotique | Psychique | S | 80 | 100 | paralysie 20 % |
| rayon-d-ether | Rayon d'éther | Psychique | S | 95 | 90 | |
| eclair-de-l-archonte | Éclair de l'archonte | Électrique | S | 85 | 95 | |
| chant-de-cristal | Chant de cristal | Son | S | 95 | 90 | |
| regard-des-mille-yeux | Regard des mille yeux | Obscur | S | 90 | 95 | paralysie 10 % |
| secret-interdit | Secret interdit | Obscur | S | 95 | 90 | |

Attaques d'œuf, sur le modèle de la section 28 :

| Famille | Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|---|
| Destrier spectral | sabots-fantomes | Sabots fantômes | Spectre | P | 80 | 100 | |
| Vautour des charniers | curee-des-sables | Curée des sables | Sol | P | 80 | 100 | |
| Mâtin de guerre | morsure-de-fer | Morsure de fer | Acier | P | 80 | 100 | |
| Griffon cuirassé | serres-du-griffon | Serres du griffon | Vol | P | 80 | 100 | |
| Licorne des cendres | flamme-de-licorne | Flamme de licorne | Féérique | S | 80 | 100 | |
| Automate brisé | engrenage | Engrenage | Acier | P | 75 | 100 | paralysie 20 % |
| Bibliophage | page-maudite | Page maudite | Psychique | S | 80 | 100 | |
| Œil errant | oeillade | Œillade | Psychique | S | 75 | 100 | paralysie 20 % |
| Archonte foudroyé | arc-electrique | Arc électrique | Électrique | S | 80 | 100 | |
| Muse de cristal | harmonie | Harmonie | Son | S | 80 | 100 | |

Ressources : Crin spectral (crin-spectral), Plume de vautour (plume-de-vautour), Collier à pointes (collier-a-pointes), Rouage brisé (rouage-brise), Encre vivante (encre-vivante) et Cristallin (cristallin), 5 pièces d'argent chacune ; Serre de griffon (serre-de-griffon) et Éclat d'archonte (eclat-d-archonte), 15.

Humains gardiens. Héros au niveau maximal de la zone, à part de son équipement ; IA relevée ; valeurs internes de leurs monstres à 15, première habileté de leur espèce :

| Identifiant | Nom | Classe | Héros (niveau, arme, armure) | Monstres | Victoire |
|---|---|---|---|---|---|
| capitaine-mercenaire | Capitaine mercenaire | Guerrier | 72, Hache du bourreau d'os, Plates du sanglier | Vautour des charniers (72), Mâtin de guerre (72), Griffon cuirassé (73) | 1000 pièces d'argent, puis l'indice 17 |
| archiviste | Archiviste | Mage | 77, Crosse des catacombes, Robe de ronces | Automate brisé (77), Bibliophage (77), Archonte foudroyé (78) | 1100 pièces d'argent, puis l'indice 19 |

Annexes (gamedesign.md, section 9.9), sur le modèle de la section 44 :

| Identifiant | Nom | Ouverte depuis | Niveaux | Monstre rare | Coffres (argent) |
|---|---|---|---|---|---|
| tertre-des-heros | Tertre des héros | Plaine du Massacre | 71 à 74 | Licorne des cendres (pacifique) | 500 à 760 |
| bibliotheque-interdite | Bibliothèque interdite | Haute-Cité déchue | 76 à 79 | Muse de cristal (pacifique) | 560 à 840 |

Équipement du palier 7 (gamedesign.md, section 4.3) : +16 % d'Attaque et d'Attaque spéciale pour les armes, +35 % de Défense et de Défense spéciale pour les armures ; niveau de combat 75 avec l'arme et l'armure du palier 7.

| Identifiant | Objet | Classe | Coût | Durée |
|---|---|---|---|---|
| fendoir-de-guerre | Fendoir de guerre (arme) | Guerrier | 8 Collier à pointes, 6 Crin spectral, 1 Éclat d'archonte | 16 h |
| dagues-du-massacre | Dagues du massacre (arme) | Assassin | 8 Collier à pointes, 6 Crin spectral, 1 Éclat d'archonte | 16 h |
| sceptre-d-ether | Sceptre d'éther (arme) | Mage | 8 Encre vivante, 6 Cristallin, 1 Éclat d'archonte | 16 h |
| marteau-du-griffon | Marteau du griffon (arme) | Templier | 8 Encre vivante, 6 Cristallin, 1 Éclat d'archonte | 16 h |
| cuirasse-du-griffon | Cuirasse du griffon (armure) | Guerrier | 10 Rouage brisé, 6 Plume de vautour, 1 Serre de griffon | 18 h |
| cuir-du-vautour | Cuir du vautour (armure) | Assassin | idem | 18 h |
| robe-des-archives | Robe des archives (armure) | Mage | idem | 18 h |
| plates-de-la-cite | Plates de la cité (armure) | Templier | idem | 18 h |

Attaques principales du palier 7, de type Humain, puissance 105, précision 100 : Brise-ost (brise-ost, Guerrier, P), Mille coupures (mille-coupures, Assassin, P), Cataclysme (cataclysme, Mage, S), Lumière divine (lumiere-divine, Templier, S).

Sprites : un par espèce, normal et shiny, et un par humain gardien (section 42).

Textes :

> Indice 16 : sous l'autel de l'Évêque, une liste de recrues scellée du même sceau : à l'ouest, la Plaine du Massacre, où deux armées se battent encore après leur mort. Un huitième Apôtre y commande.

> Indice 17 : le capitaine mercenaire vaincu a parlé : le Connétable boucher dresse son camp au centre de la plaine, sur une colline de cadavres.

> Indice 18 : dans la tente du Connétable, un ordre de marche scellé du même sceau : au-delà de la plaine, la Haute-Cité déchue, dont les tours regardent encore le ciel. Un neuvième Apôtre y gouverne.

> Indice 19 : l'archiviste vaincu a parlé : le Chancelier aux mille yeux siège au sommet de la plus haute tour, dans une salle tapissée de regards.

Capitaine mercenaire, avant le combat, puis après la victoire :

> Au milieu des lances plantées dans la boue, un capitaine mercenaire compte ses gages sur le cadavre d'un chevalier. Il range sa bourse, lève sa hache, et trois bêtes de guerre se dressent derrière lui.

> Le capitaine mercenaire s'écroule sur sa bourse, que personne ne viendra réclamer.

Archiviste, avant le combat, puis après la victoire :

> Entre deux rayonnages effondrés, un archiviste aux yeux cousus feuillette un registre. Il referme le livre d'un coup sec : votre nom vient d'y être écrit. Trois créatures surgissent des étagères.

> L'archiviste s'effondre, et son registre se referme sur une page blanche.

Connétable boucher, avant le combat, épilogue, âme et carnet :

> Au sommet d'une colline de cadavres, un colosse en armure rouillée essuie son couperet sur une bannière déchirée. Le Connétable boucher, Apôtre du Démiurge, a fait de la guerre un abattoir. Il n'y aura pas de fuite.

> Le Connétable boucher s'effondre sur sa colline de cadavres, et les armées mortes se couchent enfin. Le même sceau, une main ouverte, est gravé sur son couperet. Huit Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.

> L'âme du Connétable boucher s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

> Connétable boucher, abattu dans la Plaine du Massacre.

Chancelier aux mille yeux, avant le combat, épilogue, âme et carnet :

> Dans la salle tapissée de regards, une silhouette drapée de pourpre lève la tête : sous sa capuche, des centaines d'yeux s'ouvrent à la fois. Le Chancelier aux mille yeux, Apôtre du Démiurge, sait déjà tout de vous. Il n'y aura pas de fuite.

> Le Chancelier aux mille yeux s'effondre, et tous les regards des murs se ferment d'un coup. Le même sceau, une main ouverte, est gravé sur sa chevalière. Neuf Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.

> L'âme du Chancelier aux mille yeux s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

> Chancelier aux mille yeux, abattu dans la Haute-Cité déchue.

Cartes des annexes :

> Tertre des héros : Au fond du coffre, une carte d'état-major indique un tertre où les héros de l'ancienne guerre ont été enterrés avec leurs montures.

> Bibliothèque interdite : Au fond du coffre, une clé gravée d'un œil ouvre, d'après son étiquette, la bibliothèque interdite de la Haute-Cité.

## 47. Gorge de Braise, Pics Foudroyés et leurs annexes

Décidés le 02/11/2026, pour l'étape 167, quatrième lot de la suite de l'aventure (gamedesign.md, sections 4.3, 9.1, 9.4, 9.9 et 10.7). Les zones, leurs niveaux, les Apôtres et les humains gardiens viennent du fil validé ; les noms et les types des espèces, les rares, les annexes, les classes des humains, les météos et le palier 8 de l'équipement (bonus, durées, noms des attaques) ont été validés par l'utilisateur (lot 4) ; les autres valeurs, les textes et les réglages issus de la mesure sont proposés par Claude sans validation (gamedesign.md, Q118 ; equilibrage.md, section 17). Mêmes règles qu'aux lots précédents (sections 44 à 46).

Zones :

| Identifiant | Nom | Niveaux | Étapes | Taille des groupes | Espèces | Monstre rare | Humain | Apôtre |
|---|---|---|---|---|---|---|---|---|
| gorge-de-braise | Gorge de Braise | 78 à 82 | 5 | 1 à 3 | Golem de magma, Chien des scories, Forgeron calciné | Wyrm de magma (hostile) | Pyromancien | Prince de braise (84) |
| pics-foudroyes | Pics Foudroyés | 83 à 87 | 5 | 1 à 3 | Aigle-tonnerre, Bélier de granit, Feu de Saint-Elme | Géant des orages (hostile) | Héraut de l'orage | Sénéchal des orages (89) |

Le Schnoz se cache dans les deux. Pas de marchand mort ni de pêcheur. Météos : canicule et tempête dans la Gorge ; tempête et grêle dans les Pics. Le Chien des scories et le Feu de Saint-Elme sont nocturnes, le Forgeron calciné et l'Aigle-tonnerre diurnes.

| Zone | Expédition automatique (durée de base, dégâts, XP, potions, argent) | Coffres (argent) | Capsules des coffres (prix chez Malvoise) |
|---|---|---|---|
| Gorge de Braise | 13 h, 710 à 1360, 10300 à 14500, 1 à 3, 610 à 940 | 460 à 700 | Coulée de lave, Crocs de braise, Marteau ardent (900) |
| Pics Foudroyés | 14 h, 770 à 1490, 11700 à 16300, 1 à 3, 670 à 1020 | 500 à 760 | Serres foudroyantes, Coup de cornes, Feu errant (950) |

Espèces (stats dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse). Au codex, elles viennent après le Chancelier aux mille yeux, dans l'ordre du tableau. Aucune n'évolue.

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Rareté | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|---|
| golem-de-magma | Golem de magma | Feu, Roche | soutien | 82 / 74 / 92 / 60 / 60 / 18 (386) | 240 | commune | Carapace, Volonté | Crachat de braises (1), Griffe de pierre (1), Chute de pierres (30), Brasier (40), Coulée de lave (50), Éboulement (86) |
| chien-des-scories | Chien des scories | Feu, Ténèbres | offensif | 70 / 90 / 58 / 45 / 50 / 75 (388) | 240 | commune | Férocité, Célérité | Morsure (1), Crachat de braises (1), Croc noir (30), Griffe d'ombre (40), Crocs de braise (50), Rapine (86) |
| forgeron-calcine | Forgeron calciné | Feu, Acier | offensif | 78 / 90 / 82 / 30 / 52 / 46 (378) | 240 | commune | Férocité, Carapace | Coup de garde (1), Crachat de braises (1), Taille d'acier (30), Lame rouillée (40), Marteau ardent (50), Marteau de forge (86) |
| wyrm-de-magma | Wyrm de magma | Feu, Dragon | offensif | 80 / 96 / 76 / 72 / 64 / 68 (456) | 405 | rare, hostile | Férocité, Affinité | Crachat de braises (1), Souffle draconique (1), Brasier (35), Furie draconique (45), Souffle de magma (88) |
| ifrit | Ifrit | Feu, Psychique | soutien | 70 / 50 / 68 / 92 / 88 / 64 (432) | 400 | rare, pacifique (Cœur du volcan) | Sorcellerie, Instinct | Crachat de braises (1), Onde psychique (1), Jet de lave (35), Festin d'encre (45), Flamme astrale (88) |
| prince-de-braise | Prince de braise | Obscur | offensif | 100 / 106 / 96 / 92 / 90 / 58 (542) | 565 | Apôtre | Férocité, Volonté | Étreinte obscure (1), Crachat de braises (1), Hurlement des abysses (20), Jet de lave (35), Flagellation (50), Couronne de braise (75) |
| aigle-tonnerre | Aigle-tonnerre | Vol, Électrique | offensif | 68 / 86 / 55 / 50 / 52 / 82 (393) | 255 | commune | Traqueur, Célérité | Coup de bec (1), Étincelle (1), Piqué (30), Bec funeste (40), Serres foudroyantes (50), Plongeon d'acier (90) |
| belier-de-granit | Bélier de granit | Roche, Combat | offensif | 80 / 90 / 85 / 30 / 50 / 46 (381) | 255 | commune | Férocité, Carapace | Griffe de pierre (1), Frappe d'os (1), Chute de pierres (30), Épée des morts (40), Coup de cornes (50), Déchiquetage (90) |
| feu-de-saint-elme | Feu de Saint-Elme | Électrique, Spectre | soutien | 64 / 38 / 60 / 92 / 74 / 58 (386) | 255 | commune | Sorcellerie, Instinct | Étincelle (1), Lueur trompeuse (1), Onde de choc (30), Flamme funèbre (40), Feu errant (50), Foudre (90) |
| geant-des-orages | Géant des orages | Électrique, Combat | offensif | 85 / 98 / 78 / 58 / 60 / 55 (434) | 435 | rare, hostile | Férocité, Volonté | Étincelle (1), Frappe d'os (1), Onde de choc (35), Épée des morts (45), Poing du tonnerre (92) |
| pegase-d-orage | Pégase d'orage | Vol, Féérique | soutien | 70 / 58 / 66 / 85 / 90 / 76 (445) | 430 | rare, pacifique (Aire) | Volonté, Éclaireur | Coup de bec (1), Poussière d'elfe (1), Bourrasque (35), Rayon de lune (45), Ailes célestes (92) |
| senechal-des-orages | Sénéchal des orages | Obscur | offensif | 98 / 82 / 92 / 110 / 96 / 64 (542) | 595 | Apôtre | Sorcellerie, Volonté | Étreinte obscure (1), Étincelle (1), Hurlement des abysses (20), Onde de choc (35), Foudre (50), Tempête noire (75) |

Le type Feu n'avait aucune attaque physique : le Golem de magma, le Chien des scories et le Forgeron calciné en reçoivent chacun une de puissance 85, le Wyrm de magma une de puissance 95.

Attaques nouvelles :

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| coulee-de-lave | Coulée de lave | Feu | P | 85 | 95 | |
| crocs-de-braise | Crocs de braise | Feu | P | 85 | 95 | |
| marteau-ardent | Marteau ardent | Feu | P | 85 | 95 | |
| souffle-de-magma | Souffle de magma | Feu | P | 95 | 90 | |
| flamme-astrale | Flamme astrale | Psychique | S | 95 | 90 | |
| couronne-de-braise | Couronne de braise | Obscur | P | 95 | 90 | |
| serres-foudroyantes | Serres foudroyantes | Électrique | P | 85 | 95 | |
| coup-de-cornes | Coup de cornes | Roche | P | 85 | 95 | |
| feu-errant | Feu errant | Électrique | S | 85 | 95 | |
| poing-du-tonnerre | Poing du tonnerre | Électrique | P | 95 | 90 | |
| ailes-celestes | Ailes célestes | Féérique | S | 95 | 90 | |
| tempete-noire | Tempête noire | Obscur | S | 95 | 90 | |

Attaques d'œuf, sur le modèle de la section 28 :

| Famille | Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|---|
| Golem de magma | magma-bouillonnant | Magma bouillonnant | Feu | P | 80 | 100 | |
| Chien des scories | croc-des-ombres | Croc des ombres | Ténèbres | P | 80 | 100 | |
| Forgeron calciné | enclume | Enclume | Acier | P | 80 | 100 | |
| Wyrm de magma | ecaille-ardente | Écaille ardente | Dragon | P | 80 | 100 | |
| Ifrit | mirage-ardent | Mirage ardent | Psychique | S | 80 | 100 | |
| Aigle-tonnerre | cri-du-tonnerre | Cri du tonnerre | Électrique | S | 80 | 100 | |
| Bélier de granit | charge-du-belier | Charge du bélier | Combat | P | 80 | 100 | |
| Feu de Saint-Elme | lueur-d-orage | Lueur d'orage | Spectre | S | 75 | 100 | paralysie 20 % |
| Géant des orages | poing-du-geant | Poing du géant | Combat | P | 80 | 100 | |
| Pégase d'orage | galop-celeste | Galop céleste | Vol | P | 80 | 100 | |

Ressources : Scorie de magma (scorie-de-magma), Croc calciné (croc-calcine), Lingot noirci (lingot-noirci), Plume d'orage (plume-d-orage), Corne de granit (corne-de-granit) et Étincelle captive (etincelle-captive), 5 pièces d'argent chacune ; Écaille de magma (ecaille-de-magma) et Cœur de tonnerre (coeur-de-tonnerre), 15.

Humains gardiens. Héros au niveau maximal de la zone, à part de son équipement ; IA relevée ; valeurs internes de leurs monstres à 15, première habileté de leur espèce :

| Identifiant | Nom | Classe | Héros (niveau, arme, armure) | Monstres | Victoire |
|---|---|---|---|---|---|
| pyromancien | Pyromancien | Mage | 82, Sceptre d'éther, Robe des archives | Forgeron calciné (82), Chien des scories (82), Wyrm de magma (83) | 1200 pièces d'argent, puis l'indice 21 |
| heraut-de-l-orage | Héraut de l'orage | Templier | 87, Marteau du griffon, Plates de la cité | Bélier de granit (87), Aigle-tonnerre (87), Géant des orages (88) | 1300 pièces d'argent, puis l'indice 23 |

Annexes (gamedesign.md, section 9.9), sur le modèle de la section 44 :

| Identifiant | Nom | Ouverte depuis | Niveaux | Monstre rare | Coffres (argent) |
|---|---|---|---|---|---|
| coeur-du-volcan | Cœur du volcan | Gorge de Braise | 81 à 84 | Ifrit (pacifique) | 620 à 920 |
| aire-des-pegases | Aire des pégases | Pics Foudroyés | 86 à 89 | Pégase d'orage (pacifique) | 680 à 1000 |

Équipement du palier 8, le dernier (gamedesign.md, section 4.3) : +18 % d'Attaque et d'Attaque spéciale pour les armes, +40 % de Défense et de Défense spéciale pour les armures ; niveau de combat 85 avec l'arme et l'armure du palier 8.

| Identifiant | Objet | Classe | Coût | Durée |
|---|---|---|---|---|
| hache-de-magma | Hache de magma (arme) | Guerrier | 8 Corne de granit, 6 Croc calciné, 1 Cœur de tonnerre | 18 h |
| dagues-foudroyees | Dagues foudroyées (arme) | Assassin | 8 Corne de granit, 6 Croc calciné, 1 Cœur de tonnerre | 18 h |
| baton-des-orages | Bâton des orages (arme) | Mage | 8 Étincelle captive, 6 Scorie de magma, 1 Cœur de tonnerre | 18 h |
| marteau-de-lave | Marteau de lave (arme) | Templier | 8 Étincelle captive, 6 Scorie de magma, 1 Cœur de tonnerre | 18 h |
| plates-de-magma | Plates de magma (armure) | Guerrier | 10 Lingot noirci, 6 Plume d'orage, 1 Écaille de magma | 20 h |
| cuir-d-orage | Cuir d'orage (armure) | Assassin | idem | 20 h |
| robe-de-cendre | Robe de cendre (armure) | Mage | idem | 20 h |
| harnois-foudroye | Harnois foudroyé (armure) | Templier | idem | 20 h |

Attaques principales du palier 8, de type Humain, puissance 115, précision 100 : Ravage (ravage, Guerrier, P), Mort silencieuse (mort-silencieuse, Assassin, P), Apocalypse (apocalypse, Mage, S), Glaive céleste (glaive-celeste, Templier, S).

Sprites : un par espèce, normal et shiny, et un par humain gardien (section 42).

Textes :

> Indice 20 : dans les archives du Chancelier, une carte scellée du même sceau : au sud, la Gorge de Braise, où la terre saigne du feu. Un dixième Apôtre y règne.

> Indice 21 : le pyromancien vaincu a parlé : le Prince de braise trône au fond de la gorge, sur un siège de lave refroidie.

> Indice 22 : dans les cendres du Prince, une lettre scellée du même sceau : au-delà de la gorge, les Pics Foudroyés, où l'orage ne s'arrête jamais. Un onzième Apôtre y monte la garde.

> Indice 23 : le héraut de l'orage vaincu a parlé : le Sénéchal des orages attend au sommet du plus haut pic, là où tombe la foudre.

Pyromancien, avant le combat, puis après la victoire :

> Au bord d'une coulée de lave, un pyromancien aux mains brûlées jusqu'à l'os fait danser les flammes. Il vous montre du doigt, et trois bêtes surgissent de la fournaise.

> Le pyromancien s'effondre, et les flammes qu'il tenait s'éteignent avec lui.

Héraut de l'orage, avant le combat, puis après la victoire :

> Sur un ressaut battu par l'orage, un héraut en armure lève une bannière foudroyée et sonne du cor. Trois bêtes répondent à son appel, portées par le vent.

> Le héraut s'écroule, et sa bannière s'envole dans la tempête.

Prince de braise, avant le combat, épilogue, âme et carnet :

> Sur un trône de lave refroidie, un jeune prince à la peau craquelée de braises vous regarde approcher. Le Prince de braise, Apôtre du Démiurge, sourit : tout ce qu'il touche finit en cendres. Il n'y aura pas de fuite.

> Le Prince de braise s'effondre, et la lave se fige autour de son trône. Le même sceau, une main ouverte, est gravé sur sa couronne. Dix Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.

> L'âme du Prince de braise s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

> Prince de braise, abattu dans la Gorge de Braise.

Sénéchal des orages, avant le combat, épilogue, âme et carnet :

> Au sommet du plus haut pic, un chevalier vêtu d'orage tient une lance où la foudre vient mourir. Le Sénéchal des orages, Apôtre du Démiurge, abaisse sa visière. Il n'y aura pas de fuite.

> Le Sénéchal des orages s'effondre, et pour la première fois le ciel des Pics se tait. Le même sceau, une main ouverte, est gravé sur sa lance. Onze Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.

> L'âme du Sénéchal des orages s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

> Sénéchal des orages, abattu dans les Pics Foudroyés.

Cartes des annexes :

> Cœur du volcan : Au fond du coffre, une carte roussie par le feu indique un passage vers le cœur du volcan, là où la lave est la plus ancienne.

> Aire des pégases : Au fond du coffre, une plume d'argent et un croquis mènent à une aire au-dessus des nuages, où se poseraient les pégases d'orage.

## 48. Palais de Chair, Puits du Monde et leurs annexes

Décidés le 02/11/2026, pour l'étape 168, cinquième et dernier lot de la suite de l'aventure (gamedesign.md, sections 9.1, 9.4, 9.9 et 10.7). Les zones, leurs niveaux, les Apôtres et les humains gardiens viennent du fil validé ; les noms et les types des espèces, les rares, les annexes, les classes des humains, les météos et l'absence de nouveau palier d'équipement ont été validés par l'utilisateur (lot 5) ; les autres valeurs, les textes et les réglages issus de la mesure sont proposés par Claude sans validation (gamedesign.md, Q118 ; equilibrage.md, section 18). Mêmes règles qu'aux lots précédents (sections 44 à 47). Le Roi sans visage, au niveau 100, est le treizième et dernier Apôtre de la piste.

Zones :

| Identifiant | Nom | Niveaux | Étapes | Taille des groupes | Espèces | Monstre rare | Humain | Apôtre |
|---|---|---|---|---|---|---|---|---|
| palais-de-chair | Palais de Chair | 88 à 92 | 5 | 1 à 3 | Larve de chair, Dévoreur de chair, Valet écorché | Idole de chair (hostile) | Favorite | Reine-mère de chair (94) |
| puits-du-monde | Puits du Monde | 93 à 98 | 5 | 1 à 3 | Rampant du Puits, Écho du vide, Ver des abysses | Gardien du gouffre (hostile) | Chevalier-Crâne | Roi sans visage (100) |

Le Schnoz se cache dans les deux. Pas de marchand mort ni de pêcheur. Météos : pluie et canicule dans le Palais ; tempête et pluie dans le Puits. Le Dévoreur de chair et le Rampant du Puits sont nocturnes.

| Zone | Expédition automatique (durée de base, dégâts, XP, potions, argent) | Coffres (argent) | Capsules des coffres (prix chez Malvoise) |
|---|---|---|---|
| Palais de Chair | 15 h, 830 à 1620, 13200 à 18200, 1 à 3, 730 à 1100 | 540 à 820 | Spore de chair, Festin de chair, Écorchure (1000) |
| Puits du Monde | 16 h, 890 à 1760, 14800 à 20200, 1 à 3, 790 à 1180 | 580 à 880 | Chute sans fin, Cri du néant, Morsure des abysses (1050) |

Espèces (stats dans l'ordre PV / Attaque / Défense / Attaque spéciale / Défense spéciale / Vitesse). Au codex, elles viennent après le Sénéchal des orages, dans l'ordre du tableau. Aucune n'évolue.

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Rareté | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|---|
| larve-de-chair | Larve de chair | Insecte, Poison | soutien | 84 / 62 / 86 / 66 / 74 / 20 (392) | 270 | commune | Carapace, Volonté | Bourdonnement (1), Morsure venimeuse (1), Nuée (30), Croc de venin (40), Spore de chair (50), Peste noire (94) |
| devoreur-de-chair | Dévoreur de chair | Normal, Ténèbres | offensif | 76 / 92 / 60 / 40 / 52 / 72 (392) | 270 | commune | Férocité, Pillard | Morsure (1), Griffure (1), Curée (30), Croc noir (40), Festin de chair (50), Rapine (94) |
| valet-ecorche | Valet écorché | Combat, Spectre | offensif | 72 / 92 / 70 / 40 / 52 / 64 (390) | 270 | commune | Férocité, Traqueur | Frappe d'os (1), Charge spectrale (1), Corde du gibet (30), Griffes spectrales (40), Écorchure (50), Charge des morts (94) |
| idole-de-chair | Idole de chair | Psychique, Poison | offensif | 78 / 50 / 70 / 104 / 88 / 66 (456) | 460 | rare, hostile | Sorcellerie, Affinité | Onde psychique (1), Morsure venimeuse (1), Regard hypnotique (35), Festin d'encre (45), Extase charnelle (96) |
| lys-de-sang | Lys de sang | Plante, Féérique | soutien | 72 / 50 / 70 / 90 / 96 / 66 (444) | 455 | rare, pacifique (Jardin) | Volonté, Instinct | Feuille tranchante (1), Poussière d'elfe (1), Ronces (35), Rayon de lune (45), Floraison sanglante (96) |
| reine-mere-de-chair | Reine-mère de chair | Obscur | offensif | 104 / 90 / 100 / 104 / 98 / 48 (544) | 630 | Apôtre | Volonté, Sorcellerie | Étreinte obscure (1), Morsure venimeuse (1), Hurlement des abysses (20), Peste noire (35), Baiser de sang (50), Couvée immonde (75) |
| rampant-du-puits | Rampant du Puits | Roche, Ténèbres | offensif | 76 / 92 / 82 / 36 / 54 / 50 (390) | 285 | commune | Férocité, Carapace | Griffe de pierre (1), Morsure (1), Chute de pierres (30), Griffe d'ombre (40), Chute sans fin (50), Éboulement (99) |
| echo-du-vide | Écho du vide | Son, Psychique | soutien | 68 / 38 / 64 / 96 / 80 / 46 (392) | 285 | commune | Sorcellerie, Instinct | Glas (1), Onde psychique (1), Lamentation (30), Tocsin (40), Cri du néant (50), Requiem (99) |
| ver-des-abysses | Ver des abysses | Sol, Dragon | offensif | 84 / 94 / 74 / 40 / 52 / 48 (392) | 285 | commune | Férocité, Volonté | Secousse (1), Souffle draconique (1), Furie draconique (30), Morsure des abysses (50), Engloutissement (99) |
| gardien-du-gouffre | Gardien du gouffre | Acier, Ténèbres | offensif | 82 / 94 / 92 / 44 / 66 / 50 (428) | 485 | rare, hostile | Carapace, Férocité | Coup de garde (1), Morsure (1), Taille d'acier (35), Griffe d'ombre (45), Couperet du gouffre (99) |
| germe-du-monde | Germe du monde | Plante, Lumière | soutien | 78 / 50 / 76 / 92 / 98 / 56 (450) | 480 | rare, pacifique (Racines) | Volonté, Carapace | Feuille tranchante (1), Lueur sacrée (1), Éclat du zénith (35), Ronces maudites (45), Sève originelle (99) |
| roi-sans-visage | Roi sans visage | Obscur | offensif | 100 / 98 / 96 / 102 / 96 / 60 (552) | 670 | Apôtre | Sorcellerie, Volonté | Étreinte obscure (1), Onde psychique (1), Hurlement des abysses (20), Jugement (35), Gouffre sans fond (50), Effacement (75) |

Attaques nouvelles :

| Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|
| spore-de-chair | Spore de chair | Poison | S | 85 | 95 | poison 20 % |
| festin-de-chair | Festin de chair | Normal | P | 85 | 95 | |
| ecorchure | Écorchure | Combat | P | 85 | 95 | |
| extase-charnelle | Extase charnelle | Psychique | S | 95 | 90 | |
| floraison-sanglante | Floraison sanglante | Plante | S | 95 | 90 | |
| couvee-immonde | Couvée immonde | Obscur | S | 95 | 90 | |
| chute-sans-fin | Chute sans fin | Roche | P | 85 | 95 | |
| cri-du-neant | Cri du néant | Son | S | 85 | 95 | |
| morsure-des-abysses | Morsure des abysses | Dragon | P | 85 | 95 | |
| couperet-du-gouffre | Couperet du gouffre | Acier | P | 95 | 90 | |
| seve-originelle | Sève originelle | Plante | S | 95 | 90 | |
| effacement | Effacement | Obscur | S | 95 | 90 | |

Attaques d'œuf, sur le modèle de la section 28 :

| Famille | Identifiant | Nom | Type | Cat. | Puissance | Précision | Effet |
|---|---|---|---|---|---|---|---|
| Larve de chair | ponte-grouillante | Ponte grouillante | Insecte | P | 80 | 100 | |
| Dévoreur de chair | gueule-beante | Gueule béante | Normal | P | 80 | 100 | |
| Valet écorché | lambeaux-de-peau | Lambeaux de peau | Spectre | P | 80 | 100 | |
| Idole de chair | murmure-charnel | Murmure charnel | Psychique | S | 80 | 100 | |
| Lys de sang | parfum-de-sang | Parfum de sang | Féérique | S | 75 | 100 | paralysie 20 % |
| Rampant du Puits | reptation | Reptation | Ténèbres | P | 80 | 100 | |
| Écho du vide | repercussion | Répercussion | Son | S | 80 | 100 | |
| Ver des abysses | fouissement | Fouissement | Sol | P | 80 | 100 | |
| Gardien du gouffre | garde-du-gouffre | Garde du gouffre | Acier | P | 75 | 100 | paralysie 20 % |
| Germe du monde | eclosion | Éclosion | Lumière | S | 80 | 100 | |

Ressources : Chair grouillante (chair-grouillante), Dent du dévoreur (dent-du-devoreur), Peau écorchée (peau-ecorchee), Éclat du puits (eclat-du-puits), Cristal d'écho (cristal-d-echo) et Anneau des abysses (anneau-des-abysses), 5 pièces d'argent chacune ; Œil de l'idole (oeil-de-l-idole) et Fer du gouffre (fer-du-gouffre), 15. Le palier 8 étant le dernier, aucune ne sert à la forge : elles se vendent à Malvoise.

Humains gardiens. Héros au niveau maximal de la zone, à part de son équipement ; IA relevée ; valeurs internes de leurs monstres à 15, première habileté de leur espèce :

| Identifiant | Nom | Classe | Héros (niveau, arme, armure) | Monstres | Victoire |
|---|---|---|---|---|---|
| favorite | Favorite | Assassin | 92, Dagues foudroyées, Cuir d'orage | Dévoreur de chair (92), Idole de chair (93) | 1400 pièces d'argent, puis l'indice 25 |
| chevalier-crane | Chevalier-Crâne | Guerrier | 98, Hache de magma, Plates de magma | Rampant du Puits (98), Ver des abysses (98), Gardien du gouffre (99) | 1500 pièces d'argent, puis l'indice 27 |

La Favorite, Assassin comme le Contrebandier et la Chasseresse, n'a que deux monstres.

Annexes (gamedesign.md, section 9.9), sur le modèle de la section 44 :

| Identifiant | Nom | Ouverte depuis | Niveaux | Monstre rare | Coffres (argent) |
|---|---|---|---|---|---|
| jardin-de-la-reine | Jardin de la Reine | Palais de Chair | 91 à 94 | Lys de sang (pacifique) | 740 à 1080 |
| racines-du-monde | Racines du monde | Puits du Monde | 96 à 99 | Germe du monde (pacifique) | 800 à 1160 |

Sprites : un par espèce, normal et shiny, et un par humain gardien (section 42).

Textes :

> Indice 24 : sous la lance du Sénéchal, une invitation scellée du même sceau : au cœur du royaume, le Palais de Chair, dont les murs respirent. Un douzième Apôtre y tient sa cour.

> Indice 25 : la favorite vaincue a parlé : la Reine-mère de chair repose dans la salle du trône, au milieu de sa couvée.

> Indice 26 : dans la couche de la Reine-mère, une carte scellée du même sceau : sous le palais, le Puits du Monde descend jusqu'au fond des choses. Le dernier Apôtre y attend.

> Indice 27 : le chevalier-crâne vaincu a parlé : le Roi sans visage règne au fond du Puits, sur un trône taillé dans la dernière pierre du monde.

Favorite, avant le combat, puis après la victoire :

> Sur un lit de soie tachée, une favorite aux yeux fardés vous sourit en remplissant deux coupes. Elle laisse tomber la sienne : deux créatures sortent des tentures.

> La favorite s'effondre sur les coussins, et son sourire ne la quitte pas.

Chevalier-Crâne, avant le combat, puis après la victoire :

> Sur la dernière corniche avant le gouffre, un chevalier dont le heaume est un crâne immense barre le passage, l'épée plantée devant lui. Trois bêtes montent du Puits à son côté.

> Le chevalier-crâne bascule dans le vide, et le Puits avale son épée.

Reine-mère de chair, avant le combat, épilogue, âme et carnet :

> Dans la salle du trône, une masse de chair coiffée d'une couronne se soulève, et cent bouches s'ouvrent à la fois. La Reine-mère de chair, Apôtre du Démiurge, couve le royaume entier. Il n'y aura pas de fuite.

> La Reine-mère de chair s'affaisse, et les murs du palais cessent de respirer. Le même sceau, une main ouverte, est gravé sur son sceptre. Douze Apôtres sont tombés ; le Démiurge, lui, attend toujours. Votre traque continue.

> L'âme de la Reine-mère de chair s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

> Reine-mère de chair, abattue dans le Palais de Chair.

Roi sans visage, avant le combat, épilogue, âme et carnet :

> Au fond du Puits, sur un trône taillé dans la dernière pierre du monde, un roi drapé d'ombre lève la tête : à la place du visage, il n'y a rien. Le Roi sans visage, dernier Apôtre du Démiurge, se lève. Il n'y aura pas de fuite.

> Le Roi sans visage s'effondre, et le Puits se tait. Le même sceau, une main ouverte, est gravé sur son trône. Les treize Apôtres sont tombés ; il ne reste que le Démiurge. Rapportez leurs âmes à Malvoise.

> L'âme du Roi sans visage s'arrache de sa dépouille, noire et brûlante. Vous la recueillez.

> Roi sans visage, abattu dans le Puits du Monde.

Cartes des annexes :

> Jardin de la Reine : Au fond du coffre, un ruban de soie rouge noue une clé de jardin : derrière le palais, la Reine cultiverait des fleurs qui boivent le sang.

> Racines du monde : Au fond du coffre, une lanterne de racines tressées éclaire un passage qui descend le long des racines du monde.

## 49. Le Démiurge réglé et les zones légendaires

Décidés le 02/11/2026, pour l'étape 169 (gamedesign.md, sections 5.6, 9.4 et 9.10). Validés par l'utilisateur : les trois légendaires, leurs types, environ 600 au total des stats, des zones peuplées d'espèces existantes de même thème, leur place au codex et la difficulté visée pour le Démiurge, environ 3 tentatives. Les autres valeurs et les textes sont proposés par Claude sans validation (gamedesign.md, Q118 ; equilibrage.md, section 19).

Le Démiurge garde ses types, ses attaques et ses rendements d'XP ; ses deux formes passent, aux mêmes proportions, de 700 et 780 à 500 et 560 au total des stats :

| Identifiant | Stats de base (total) |
|---|---|
| demiurge | 85 / 93 / 79 / 93 / 79 / 71 (500) |
| demiurge-eveille | 97 / 104 / 86 / 108 / 86 / 79 (560) |

Zones légendaires, ouvertes à la chute du Démiurge, dans cet ordre. Niveau 100, 5 étapes, groupes de 1 à 3, coffres de 640 à 960 pièces d'argent, sans capsule ; ni Schnoz, ni marchand mort, ni pêcheur, ni expédition automatique (elles ne comptent pas comme explorées). Le légendaire attend à la dernière étape (section 9.10) :

| Identifiant | Nom | Espèces | Monstre rare | Météos | Légendaire |
|---|---|---|---|---|---|
| abysses | Abysses | Sirène putride, Crabe nécrophage | Hydre des marées (hostile) | pluie, cyclone | Léviathan |
| terres-arides | Terres arides | Crotale des salines, Ver des salines, Scarabée solaire | aucun | tempête de sable, canicule | Béhémoth |
| ciel-dechire | Ciel déchiré | Harpie des orages, Aigle-tonnerre, Feu de Saint-Elme | aucun | tempête, grêle | Ziz |

Légendaires, au niveau 100, à valeurs internes fixes de 31, à l'IA avancée, 1 chance sur 512 d'être shiny (section 9.10). Rareté "rare", comptés au codex, après le Roi sans visage ; ni ressource, ni évolution :

| Identifiant | Nom | Types | Rôle | Stats de base (total) | Rendement d'XP | Habiletés | Attaques apprises |
|---|---|---|---|---|---|---|---|
| leviathan | Léviathan | Eau, Dragon | offensif | 115 / 90 / 100 / 100 / 100 / 95 (600) | 500 | Volonté, Carapace | Déferlante (1), Souffle draconique (1), Marée noire (40), Souffle des brumes (60), Déluge draconique (80), Abîme des mers (100) |
| behemoth | Béhémoth | Sol, Combat | offensif | 130 / 125 / 115 / 55 / 95 / 80 (600) | 500 | Férocité, Carapace | Secousse (1), Frappe d'os (1), Coup de pioche (40), Engloutissement (60), Déchiquetage (80), Piétinement du monde (100) |
| ziz | Ziz | Vol, Lumière | offensif | 95 / 70 / 85 / 120 / 95 / 135 (600) | 500 | Célérité, Éclaireur | Bourrasque (1), Lueur sacrée (1), Aube éclatante (40), Flamme blanche (60), Ouragan céleste (80), Aile de l'aube (100) |

Attaques nouvelles :

| Identifiant | Nom | Type | Cat. | Puissance | Précision |
|---|---|---|---|---|---|
| abime-des-mers | Abîme des mers | Eau | S | 100 | 90 |
| pietinement-du-monde | Piétinement du monde | Sol | P | 100 | 90 |
| ouragan-celeste | Ouragan céleste | Vol | S | 95 | 90 |
| aile-de-l-aube | Aile de l'aube | Lumière | S | 100 | 90 |

Attaques d'œuf, sur le modèle de la section 28, pour suivre la règle de chaque famille, bien qu'un légendaire, seul de son espèce, ne puisse pas passer par le rite :

| Famille | Identifiant | Nom | Type | Cat. | Puissance | Précision |
|---|---|---|---|---|---|---|
| Léviathan | ecaille-abyssale | Écaille abyssale | Dragon | P | 80 | 100 |
| Béhémoth | corne-primordiale | Corne primordiale | Combat | P | 80 | 100 |
| Ziz | plume-celeste | Plume céleste | Vol | S | 80 | 100 |

Sprites : un par légendaire, normal et shiny (section 42).

Textes, avant le combat puis à la capture :

> Léviathan : Au plus profond des Abysses, la mer elle-même se soulève : un serpent immense, couvert d'écailles noires, ouvre une gueule où s'engouffrent les courants. Léviathan, maître des eaux, vous barre la route.

> Léviathan s'effondre dans un remous et se laisse approcher. Le maître des eaux vous suit désormais.

> Béhémoth : Au cœur des Terres arides, le sol tremble à chaque pas : une bête colossale aux cornes de pierre se dresse entre les dunes. Béhémoth, premier-né de la terre, charge.

> Béhémoth plie les genoux dans la poussière et baisse ses cornes. Le premier-né de la terre vous suit désormais.

> Ziz : Au-dessus des nuages déchirés, une ombre couvre le soleil : un oiseau si grand que ses ailes cachent le ciel plonge vers vous. Ziz, roi des airs, pousse un cri.

> Ziz se pose devant vous, les ailes repliées, et incline la tête. Le roi des airs vous suit désormais.

## 50. Quêtes du jour et de la semaine

Proposé par Claude le 06/11/2026, pour les étapes 170 à 172, et codé sans validation, l'utilisateur n'étant pas là (gamedesign.md, section 13.12 et Q119).

Objectifs, avec l'identifiant gardé par la sauvegarde, et leurs quêtes ; une quête de la semaine demande 5 fois plus qu'une quête du jour :

| Objectif (identifiant) | Quête du jour | Quête de la semaine |
|---|---|---|
| vaincre-des-monstres-sauvages | Vaincre 15 monstres sauvages | Vaincre 75 monstres sauvages |
| capturer-des-monstres | Capturer 3 monstres | Capturer 15 monstres |
| ouvrir-des-coffres | Ouvrir 2 coffres en expédition | Ouvrir 10 coffres en expédition |
| recolter-des-ressources | Récolter 5 ressources sur les monstres vaincus | Récolter 25 ressources sur les monstres vaincus |
| mener-des-expeditions-au-bout | Mener 2 expéditions jusqu'au bout | Mener 10 expéditions jusqu'au bout |
| reussir-des-expeditions-automatiques | Réussir 1 expédition automatique | Réussir 5 expéditions automatiques |

Récompense, versée au coffre dès que la quête est accomplie :

| Quête | Récompense | Dans la Lisière | Dans le Puits du Monde |
|---|---|---|---|
| Du jour | l'argent de 2 coffres de la zone ouverte la plus avancée | de 20 à 50 pièces d'argent | de 1160 à 1760 pièces d'argent |
| De la semaine | l'argent de 5 coffres de cette zone et un booster, tiré parmi les six | de 50 à 125 pièces d'argent | de 2900 à 4400 pièces d'argent |

Textes proposés sans validation :

| Moment | Texte |
|---|---|
| Entrée du camp, console | Quêtes du jour et de la semaine |
| Bouton du camp, Discord | Quêtes |
| Écran, titres | Quêtes du jour (nouvelles dans {délai}) : / Quêtes de la semaine (nouvelles dans {délai}) : |
| Écran, quête en cours | {quête} : {progrès}/{cible}. Récompense : de {minimum} à {maximum} pièces d'argent. / ... et un booster. |
| Écran, quête accomplie | {quête} : accomplie. |
| Écran, rappel | Une quête accomplie verse aussitôt sa récompense au coffre du camp, même en expédition. |
| Récit | Quête du jour accomplie : {quête, en minuscule}. Récompense, au coffre : {objets}. / Quête de la semaine accomplie : ... |

Le délai se dit "5 h 12", ou "3 j 4 h" au-delà d'un jour. Exemple de récit : "Quête de la semaine accomplie : capturer 15 monstres. Récompense, au coffre : 34 pièces d'argent et 1 Fiole de sang."

## 51. Succès, titres, trophées et couleurs du héros

Décidé le 06/11/2026, pour les étapes 174, 176 et 177 (gamedesign.md, section 4.5). Le principe (titres, trophées des grands exploits, quatre couleurs et la royale couronnée) est un choix de l'utilisateur ; la liste, les titres, les conditions, les textes et les palettes sont proposés par Claude sans validation (Q120). Chaque succès donne son titre ; les titres conviennent à tout héros, sans accord de genre.

| Identifiant | Titre | Condition | Trophée | Couleur |
|---|---|---|---|---|
| nomade | Nomade | mener une expédition au bout | | |
| naturaliste | Naturaliste | capturer 30 espèces du codex | | |
| encyclopediste | Encyclopédiste | compléter le codex | oui | |
| oeil-de-lynx | Œil-de-lynx | avoir un monstre shiny dans l'équipe ou au camp | | |
| porte-lueur | Porte-lueur | avoir 5 monstres shiny à la fois dans l'équipe et au camp | | Nacre |
| lueur-incarnee | Lueur incarnée | avoir un héros shiny | | |
| veteran | Vétéran | mener un monstre au niveau 50 | | |
| legende-vivante | Légende vivante | mener un monstre au niveau 100 | | |
| brise-chaines | Brise-chaînes | vaincre le Bourreau | | |
| fleau-du-baron | Fléau du Baron | abattre le Baron pendu | oui | |
| fleau-de-l-abbesse | Fléau de l'Abbesse | abattre l'Abbesse écorchée | oui | |
| fleau-du-margrave | Fléau du Margrave | abattre le Margrave de givre | oui | |
| fleau-de-l-amiral | Fléau de l'Amiral | abattre l'Amiral noyé | oui | |
| fleau-du-prevot | Fléau du Prévôt | abattre le Prévôt des fosses | oui | |
| fleau-de-la-duchesse | Fléau de la Duchesse | abattre la Duchesse sanguine | oui | |
| fleau-de-l-eveque | Fléau de l'Évêque | abattre l'Évêque des os | oui | |
| fleau-du-connetable | Fléau du Connétable | abattre le Connétable boucher | oui | |
| fleau-du-chancelier | Fléau du Chancelier | abattre le Chancelier aux mille yeux | oui | |
| fleau-du-prince | Fléau du Prince | abattre le Prince de braise | oui | |
| fleau-du-senechal | Fléau du Sénéchal | abattre le Sénéchal des orages | oui | |
| fleau-de-la-reine-mere | Fléau de la Reine-mère | abattre la Reine-mère de chair | oui | |
| fleau-du-roi-sans-visage | Fléau du Roi sans visage | abattre le Roi sans visage | oui | |
| deicide | Déicide | abattre le Démiurge | oui | Cendre |
| legende-des-abysses | Légende des Abysses | capturer Léviathan | oui | |
| legende-des-terres-arides | Légende des Terres arides | capturer Béhémoth | oui | |
| legende-du-ciel-dechire | Légende du Ciel déchiré | capturer Ziz | oui | |
| idole-de-la-fosse | Idole de la Fosse | vaincre le dernier palier de la Fosse aux damnés | oui | Sang |
| ombre-de-la-tour | Ombre de la Tour | franchir 50 étapes de la Tour sans fond, seul | | Abîme |
| duelliste | Duelliste | gagner 10 duels | | |
| main-d-or | Main d'or | avoir 10 000 pièces d'argent, sac et coffre réunis | | |
| architecte | Architecte | mener toutes les améliorations du camp à leur niveau maximal | | |
| fer-de-lance | Fer de lance | porter une arme et une armure du palier 8 | | |
| cartographe | Cartographe | ouvrir les 10 zones annexes | | |
| complice-de-malvoise | Complice de Malvoise | accomplir les 4 quêtes de Malvoise | oui | |
| eleve-de-gunther | Élève de Gunther | accomplir les 4 quêtes de Gunther | oui | |
| camarade-d-ansgar | Camarade d'Ansgar | accomplir les 4 quêtes d'Ansgar | oui | |

La couleur royale, or et pourpre, avec une couronne, se débloque quand les 22 trophées sont réunis.

Trophées du camp :

| Succès | Trophée |
|---|---|
| encyclopediste | Le bestiaire complet, relié de cuir noir, ouvert sur un lutrin près du feu. |
| fleau-du-baron | La corde du Baron pendu, nouée au poteau du camp. |
| fleau-de-l-abbesse | Le voile de l'Abbesse écorchée, tendu sur un cadre de bois. |
| fleau-du-margrave | Un éclat de la couronne du Margrave de givre, qui ne fond jamais. |
| fleau-de-l-amiral | Le tricorne de l'Amiral noyé, toujours ruisselant, accroché à une lance. |
| fleau-du-prevot | Le fouet du Prévôt des fosses, enroulé sur un clou. |
| fleau-de-la-duchesse | La coupe de la Duchesse sanguine, vide, posée sur une souche. |
| fleau-de-l-eveque | La crosse de l'Évêque des os, plantée près de l'entrée. |
| fleau-du-connetable | Le couperet du Connétable boucher, fiché dans un billot. |
| fleau-du-chancelier | Le sceau du Chancelier aux mille yeux, dont aucun œil ne s'ouvre plus. |
| fleau-du-prince | Une braise du Prince, qui couve dans une lanterne de fer. |
| fleau-du-senechal | Le cor du Sénéchal des orages, muet, pendu à une branche. |
| fleau-de-la-reine-mere | Une mèche de la Reine-mère de chair, tressée et séchée. |
| fleau-du-roi-sans-visage | Le masque vide du Roi sans visage, posé face contre terre. |
| deicide | Un fragment de l'Éclipse, pierre noire qui boit la lumière. |
| legende-des-abysses | Une écaille de Léviathan, grande comme un bouclier. |
| legende-des-terres-arides | Une corne de Béhémoth, dressée comme une pierre levée. |
| legende-du-ciel-dechire | Une plume de Ziz, plus haute qu'une tente. |
| idole-de-la-fosse | La chaîne du Champion de la Fosse, jetée au pied du feu. |
| complice-de-malvoise | Le psautier de Frère Malvoise, ouvert à la page d'Isaure. |
| eleve-de-gunther | La lame noire de Konrad, plantée au bord du camp. |
| camarade-d-ansgar | Le filet de l'équipage d'Edwin, tendu entre deux pieux près de l'étang. |

Couleurs du héros, dessinées pour les quatre classes à l'étape 177 (même dessin, autre palette ; la royale ajoute une couronne d'or sertie d'une pierre pourpre) : Cendre, gris de cendre et braise ; Sang, rouge sombre et noir ; Abîme, bleu de nuit et vert d'eau ; Nacre, blanc nacré et lilas ; Royale, or et pourpre.

Textes :

| Moment | Texte |
|---|---|
| Succès obtenu | Succès obtenu : {titre} ({condition}). Vous pouvez porter ce titre. |
| Trophée en plus | Un nouveau trophée orne le camp : {trophée} |
| Couleur débloquée | Nouvelle couleur du héros : {couleur}. |
| Tous les trophées | Tous les trophées du camp sont réunis : la couleur royale vous revient. |
| Plusieurs à la fois (plus de 3) | {n} succès obtenus : voyez les succès, sur l'écran du héros. |
| Titre porté | Vous portez désormais le titre {titre}. / Vous ne portez plus de titre. |
| Couleur portée | Votre héros prend la couleur {couleur}. / Votre héros reprend sa couleur d'origine. |
| Écran des succès | Succès : {n}/36. Trophées du camp : {n}/22. Puis une ligne par succès : "{titre} : {condition}", suivie de " (obtenu)" pour un succès obtenu. |
| Libellés | Héros ; Succès ; Trophées du camp ; Porter un titre ; Ne porter aucun titre ; Couleur du héros ; Couleur d'origine |
| Refus | Ce succès n'est pas encore obtenu. / Cette couleur n'est pas encore débloquée. |

## 52. Quêtes des personnages du camp

Décidé le 06/11/2026, pour les étapes 175 à 177 (gamedesign.md, section 13.13). Les règles (chaîne de 4 quêtes par personnage, ressources à rapporter ou objet unique gardé, objet dans le sac, service 25 % plus court à la fin de chaque chaîne) sont des choix de l'utilisateur ; les quêtes, leurs zones, objets, gardiens, récompenses et textes sont proposés par Claude sans validation (Q121). Les récompenses vont au coffre.

| Identifiant | Personnage | Quête | Ouverture | Demande | Récompense |
|---|---|---|---|---|---|
| lueur-du-crane | Malvoise | La lueur du crâne | Marais des Pendus ouvert | 5 Cendre de feu follet | 150 pièces d'argent, 3 potions |
| psautier-de-malvoise | Malvoise | Le psautier de Frère Malvoise | Abbaye des Cendres ouverte | Psautier de Frère Malvoise, Abbaye, étape 3, gardé par une Goule de niveau 27 | 400, 1 Philtre chromatique |
| prieres-pour-isaure | Malvoise | Des prières pour Isaure | Catacombes de Sainte-Ombre ouvertes | 6 Chapelet d'os | 1 000, 3 capsules d'habileté |
| page-du-codex-mortis | Malvoise | La page du Codex Mortis | Haute-Cité déchue ouverte | Page du Codex Mortis, Haute-Cité, étape 3, gardée par un Œil errant de niveau 77 | 3 000, 1 Philtre chromatique ; rite en 1 h 30 |
| tenailles-neuves | Gunther | Des tenailles neuves | Vallée des Brumes ouverte | 6 Fragment d'armure | 200, 1 Moelle de bête, 1 Écorce de fer |
| marteau-de-hautefer | Gunther | Le marteau de Hautefer | Col du Givre-Noir ouvert | Marteau de Hautefer, Col, étape 3, gardé par un Troll des cavernes de niveau 36 | 600, 1 Philtre chromatique |
| lame-pour-konrad | Gunther | Une lame pour Konrad | Mines de Fer-Noir ouvertes | 8 Éclat de fer noir | 1 200, 3 capsules d'habileté |
| coeur-de-braise | Gunther | Le cœur de braise | Gorge de Braise ouverte | Cœur de braise, Gorge, étape 3, gardé par un Golem de magma de niveau 82 | 3 000, 1 Philtre chromatique ; travaux 25 % plus courts |
| leurres-de-plumes | Ansgar | Des leurres de plumes | Marais des Pendus ouvert | 5 Plume de corbeau | 150, 5 filets de chanvre |
| medaillon-d-edwin | Ansgar | Le médaillon d'Edwin | Port des Noyés ouvert | Médaillon d'Edwin, Port, étape 3, gardé par une Sirène putride de niveau 51 | 800, 1 Philtre chromatique |
| barque-pour-edwin | Ansgar | Une barque pour Edwin | Mines de Fer-Noir ouvertes | 8 Carapace de crabe | 1 200, 3 capsules d'habileté |
| filet-de-l-equipage | Ansgar | Le filet de l'équipage | Épave du Léviathan ouverte | Filet de l'équipage, Épave, étape 2, gardé par un Crabe nécrophage de niveau 53 | 3 000, 1 Philtre chromatique ; ligne en 1 h 30 |

Chaque objet de quête a l'identifiant de sa quête.

Textes, pour chaque quête : la demande, puis, pour un objet, sa rencontre et sa prise, puis la remise.

Malvoise :

> La lueur du crâne, demande : Malvoise fait tourner son crâne entre ses doigts. "Il s'éteint. Les feux follets du Marais brûlent de ce qui reste des noyés : rapporte-m'en cinq poignées de cendre, et il verra de nouveau clair."

> Remise : Malvoise souffle la cendre dans les orbites du crâne, qui s'allument d'une lueur verte. "Il voit. C'était le crâne de mon maître, à l'Abbaye des Cendres. Il m'a appris à lire les morts ; on l'a brûlé pour cela."

> Le psautier de Frère Malvoise, demande : "L'Abbaye des Cendres est ouverte, dis-tu ? J'y ai été moine. Frère Malvoise, archiviste. J'ai caché sous les dalles du chœur un psautier noté de ma main. Les goules rôdent là où l'on enterre mal : rapporte-le-moi."

> Rencontre : Sous une dalle descellée du chœur, un psautier relié de cuir noir. Une goule, accroupie dessus, relève la tête et gronde.

> Prise : Sur la garde du psautier, une main appliquée a écrit "Frère Malvoise". Vous le glissez dans votre sac.

> Remise : Malvoise tourne les pages sans un mot, puis s'arrête sur un nom : Isaure. "Ma sœur. Elle chantait au chœur quand l'Abbesse a commencé à écorcher les novices. Je n'ai rien fait. J'ai fui avec mes livres."

> Des prières pour Isaure, demande : "Les moines des Catacombes psalmodient encore, morts depuis des siècles. Leurs chapelets d'os portent des prières que plus personne ne dit. Rapporte-m'en six : il m'en faut pour retenir une âme qui ne trouve pas le repos."

> Remise : Malvoise noue les chapelets autour du crâne, un à un. "Isaure ne repose pas. Je l'entends, la nuit, qui chante sous le camp. Ces prières la retiendront, le temps que je trouve le reste."

> La page du Codex Mortis, demande : "La Haute-Cité avait la plus grande bibliothèque du monde. Le Codex Mortis y était enchaîné ; une de ses pages dit comment rendre le repos à une âme liée. Trouve-la, et j'achèverai ce que j'aurais dû faire il y a trente ans."

> Rencontre : Dans une salle de lecture éventrée, une page arrachée flotte au-dessus d'un pupitre, couverte d'une écriture qui bouge. Un œil errant se tourne vers vous.

> Prise : L'écriture cesse de bouger dès que vous touchez la page. Vous la pliez dans votre sac.

> Remise : Malvoise lit la page à voix basse, puis la brûle au-dessus du crâne. Sous le camp, le chant s'arrête. "Elle dort, enfin." Il reste longtemps silencieux. "Je ne dois plus rien aux morts. Mes rites iront plus vite, désormais : mes mains ne tremblent plus."

Gunther :

> Des tenailles neuves, demande : Gunther essuie ses mains sur son tablier. "Le Bourreau m'a pris mes outils et ne m'a laissé que ses chaînes. Les armures hantées font un bon acier : rapporte-m'en six fragments, que je me refasse des tenailles dignes de ce nom."

> Remise : Gunther fond les fragments et martèle jusqu'à la nuit. Au matin, il vous montre des tenailles neuves. "Mon fils aurait fait mieux. Konrad. Il avait la main plus fine que la mienne. Le Bourreau l'a gardé, lui aussi. Il n'est pas sorti."

> Le marteau de Hautefer, demande : "Mon village était au pied du Col : Hautefer. Les trolls l'ont pris, l'hiver où les Apôtres sont venus. Le marteau de mon père est resté sur l'enclume. S'il y est encore, rapporte-le-moi."

> Rencontre : Dans les ruines d'une forge, sous la neige, un marteau repose sur une enclume fendue. Un troll des cavernes en a fait son oreiller ; il se lève, furieux.

> Prise : Le manche du marteau porte une marque : trois traits et une flamme. Vous le rangez dans votre sac.

> Remise : Gunther soupèse le marteau et rit pour la première fois. "La marque de Hautefer. Mon père, et son père avant lui." Il le pend au-dessus de l'enclume. "Il ne servira qu'aux beaux ouvrages."

> Une lame pour Konrad, demande : "On dit que le fer des Mines ne rouille jamais, parce que les morts le gardent. Il m'en faut huit éclats : je veux forger une lame pour la tombe de Konrad. Il n'en a pas, de tombe. Il aura une lame."

> Remise : Gunther forge trois jours. La lame est simple, sans ornement, noire comme les nuits du Col. Il la plante en terre au bord du camp. "Voilà. Ici repose Konrad, apprenti de Hautefer."

> Le cœur de braise, demande : "Au fond de la Gorge bat un cœur de braise qui ne refroidit jamais. Une forge qui en a un ne s'éteint plus. Avec lui, je travaillerai plus vite que je ne l'ai jamais fait."

> Rencontre : Au bord d'une coulée, une pierre rouge palpite comme un cœur. Un golem de magma la porte dans sa poitrine ouverte.

> Prise : Arraché au golem, le cœur de braise continue de battre, tiède, presque doux. Vous l'enveloppez au fond de votre sac.

> Remise : Gunther pose le cœur de braise au fond du foyer. La forge s'embrase d'une flamme blanche. "Konrad aurait aimé voir ça." Il reprend le marteau de Hautefer. "Désormais, mes ouvrages iront plus vite."

Ansgar :

> Des leurres de plumes, demande : Ansgar ravaude un filet, les doigts gourds. "Mes leurres sont restés dans la cabane. Avec cinq plumes de corbeau, je m'en ferai de nouveaux, et le poisson de l'étang mordra mieux."

> Remise : Ansgar noue les plumes avec une patience infinie. "Mon fils, Edwin, les faisait mieux que moi. Il est parti en mer, au Port. Les marins disaient que la mer y rend les noyés. Elle ne me l'a jamais rendu."

> Le médaillon d'Edwin, demande : "Le Port des Noyés. Edwin y a embarqué il y a vingt ans. Il portait un médaillon d'étain où j'avais gravé mon visage, tant bien que mal. Si la mer rend les noyés, elle rend peut-être aussi ce qu'ils portaient."

> Rencontre : Sur un ponton pourri, une sirène putride chante, un médaillon d'étain pendu à son cou.

> Prise : Le médaillon est terni, mais le visage gravé ressemble à Ansgar, en plus jeune. Vous le glissez dans votre sac.

> Remise : Ansgar serre le médaillon dans son poing, longtemps. "Elle l'avait, la sirène ? Alors il n'est pas resté seul, au fond." Il le porte désormais à son cou.

> Une barque pour Edwin, demande : "Les gens du Port brûlaient une barque pour chaque noyé, pour qu'il trouve son chemin. Edwin n'en a pas eu. Avec huit carapaces de crabe pour la coque, je lui en ferai une."

> Remise : Ansgar assemble une petite barque de carapaces et y couche une plume de corbeau. Au soir, il la pousse sur l'étang du camp et y met le feu. "Bon vent, mon garçon."

> Le filet de l'équipage, demande : "L'Épave du Léviathan, c'était son navire, j'en suis sûr à présent. Dans la cale, il y avait le filet de l'équipage, tressé de crin de kelpie : un filet qui ne casse jamais. Rapporte-le-moi."

> Rencontre : Au fond de la cale noyée, un filet aux mailles luisantes est roulé contre la coque. Un crabe nécrophage s'y est fait un nid ; il claque des pinces.

> Prise : Pas une maille du filet n'a cédé. Vous le roulez au fond de votre sac.

> Remise : Ansgar déplie le filet de l'équipage d'Edwin. Ses mains ne tremblent pas. "Avec ça, je ne perdrai plus une prise. Mes lignes iront plus vite au fond."

Autres textes :

| Moment | Texte |
|---|---|
| Au camp, une demande à écouter | {personnage} a une demande pour vous. |
| Au camp, une quête à remettre | Vous pouvez remettre sa quête à {personnage}. |
| Récompense | Vous recevez {récompense}, rangés dans le coffre. |
| Service amélioré | Le Rite du sang mêlé dure désormais 1 h 30. / Les travaux de Gunther durent désormais 25 % de moins. / La ligne d'Ansgar est désormais prête en 1 h 30. |
| Écran des quêtes | Pour chaque personnage au camp : "{personnage} ({n}/4 quêtes accomplies)", puis sa quête en cours, "En cours : {quête}. {objectif}", "Une demande à écouter.", "Rien à vous demander pour l'instant." ou "Toutes ses quêtes sont accomplies." |
| Objectif | Rapporter 5 Cendre de feu follet (vous en avez 3). / Trouver le Psautier de Frère Malvoise dans l'Abbaye des Cendres. / Remettre le Psautier de Frère Malvoise. |
| Libellés | Quêtes du camp (console), Quêtes (Discord) ; Écouter {personnage} ; Remettre à {personnage} |
| Refus | Il vous manque 2 Cendre de feu follet. / Vous n'avez pas le Psautier de Frère Malvoise. / {personnage} n'a rien à vous demander. / {personnage} n'est pas au camp. |
