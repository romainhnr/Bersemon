from dessin import *
from matieres import *
from atelier import sprite
from commun import jambes, torse, robe, bras, tete

H = dict(dossier="humains", variantes=())


def _h(nom):
    return sprite("humain-" + nom, {}, fichier=nom, **H)


@sprite("humain-brigand", "Brigand", fichier="brigand", **H)
def brigand(t):
    t.mat("c", PEAU); t.mat("v", "7a5a3a"); t.mat("b", "4a3c30"); t.mat("n", "2e2420")
    t.mat("r", "9a2a2a"); t.mat("h", "3a2a22"); t.mat("l", ACIER)
    jambes(t, "b", "n", pas=1)
    torse(t, "v")
    t.p(L(18, 19, 29, 29), "n", trait=0, ton=2)
    t.p(R(18, 29, 30, 30), "n", trait=0)
    bras(t, [(30, 19), (34, 25), (33, 30)], "v", dl=-1, trait=0)
    bras(t, [(18, 19), (12, 23), (7, 21)], "v")
    t.p(P([(7, 20), (0, 17), (5, 23)]), "l", trait=0, ton=4)
    tete(t)
    t.p(B([(18, 11), (20, 12), (24, 12), (27, 10), (26, 15), (21, 16)]), "h", trait=0)
    t.p(B([(18, 7), (20, 3), (27, 3), (29, 7), (24, 6)]), "r", trait=0)
    t.p(K([(29, 6, 1), (33, 9, 0.8), (32, 13, 0.6)]), "r", trait=0)


@sprite("humain-bourreau", "Bourreau", fichier="bourreau", **H)
def bourreau(t):
    t.mat("c", PEAU); t.mat("n", "24202a"); t.mat("b", "4a3a2e"); t.mat("l", ACIER); t.mat("m", BOIS)
    t.mat("r", SANG); t.mat("y", SANG)
    # grande hache
    t.p(C(9, 45, 10, 6, 1.1), "m", trait=0)
    t.p(B([(9, 8), (2, 4), (0, 13), (3, 21), (9, 18)]), "l", trait=0)
    t.px([(1, 9), (1, 13), (2, 17)], "l", 4); t.px([(3, 19), (4, 21), (2, 22)], "r", 2)
    jambes(t, "b", "n")
    torse(t, "c", large=2)
    t.px([(22, 20), (26, 20), (24, 24), (21, 27), (27, 27)], "c", 0)
    t.p(B([(16, 27), (32, 27), (33, 34), (15, 34)]), "b", trait=0)
    bras(t, [(31, 19), (36, 26), (35, 32)], "c", dl=-1, trait=0)
    bras(t, [(17, 19), (12, 24), (10, 22)], "c")
    # cagoule noire
    t.p(B([(18, 16), (18, 6), (23, 1), (28, 6), (29, 16), (24, 18)]), "n", trait=0, dl=1)
    t.px([(20, 9), (21, 9), (24, 9), (25, 9)], "y", 3)


@sprite("humain-inquisiteur", "Inquisiteur", fichier="inquisiteur", **H)
def inquisiteur(t):
    t.mat("c", PEAU); t.mat("r", "8a1c24"); t.mat("n", "26222c"); t.mat("g", OR); t.mat("m", BOIS); t.mat("f", FEU)
    # torche
    t.p(C(10, 34, 10, 14, 1), "m", trait=0)
    t.p(B([(7, 14), (8, 8), (10, 3), (13, 8), (13, 14)]), "f", trait=0, ton=2)
    t.px([(10, 9), (10, 11), (10, 13)], "f", 4)
    robe(t, "r")
    t.p(R(23, 18, 25, 30) | R(20, 22, 28, 23), "g", trait=0)
    t.p(R(17, 30, 31, 31), "n", trait=0)
    bras(t, [(30, 19), (34, 26), (31, 31)], "r", dl=-1, trait=0)
    bras(t, [(18, 19), (13, 23), (10, 21)], "r")
    tete(t)
    t.px([(19, 13), (20, 13), (21, 13)], "c", 0)
    # chapeau noir a large bord
    t.p(E(23, 5.5, 9, 1.8), "n", trait=1)
    t.p(B([(18, 5), (19, -1), (27, -1), (28, 5)]), "n", trait=0)
    t.p(R(19, 3, 27, 4), "g", trait=0, ton=1)


@sprite("humain-porte-etendard", "Porte-étendard", fichier="porte-etendard", **H)
def porte_etendard(t):
    t.mat("a", ACIER); t.mat("c", PEAU); t.mat("d", ["1c3470", "2e52a8", "4a7ad8", "8cb0f0", "d0e0fc"]); t.mat("w", "f0f0f8")
    t.mat("m", BOIS); t.mat("n", "2e2a30")
    # hampe et etendard
    t.p(R(9, -4, 10, 45), "m", trait=0)
    t.p(P([(11, -3), (30, -2), (26, 4), (30, 10), (11, 11)]), "d", trait=0)
    t.p(R(17, 1, 18, 8) | R(14, 4, 21, 5), "w", trait=0, ton=3)
    jambes(t, "a", "n")
    torse(t, "a")
    t.p(B([(18, 18), (24, 16), (30, 18), (31, 34), (17, 34)]), "d", trait=0)
    t.p(R(18, 28, 30, 29), "n", trait=0)
    bras(t, [(30, 19), (34, 25), (33, 30)], "a", dl=-1, trait=0, main=False)
    bras(t, [(18, 19), (13, 23), (10, 20)], "a", main=False)
    t.p(E(10, 19, 2, 2), "a", trait=0, dl=1)
    # chapel de fer
    tete(t, cy=11)
    t.p(E(23, 7.5, 7.4, 1.6), "a", trait=1)
    t.p(B([(18, 7), (20, 2), (26, 2), (28, 7)]), "a", trait=0)


@sprite("humain-pythie-de-sel", "Pythie de sel", fichier="pythie-de-sel", **H)
def pythie(t):
    t.mat("r", ["8a8478", "b8b2a4", "dcd8cc", "f2f0e8", "ffffff"]); t.mat("c", PEAU); t.mat("g", OR)
    t.mat("s", ["8aa0b0", "b4c8d4", "d8e6ee", "eef6fa", "ffffff"]); t.mat("y", SPECTRE)
    robe(t, "r", large=1)
    t.p(R(17, 28, 31, 29), "g", trait=0)
    t.px([(14, 44), (20, 43), (28, 43), (34, 44)], "g", 2)
    bras(t, [(30, 19), (35, 22), (38, 17)], "r", dl=-1, trait=0)
    bras(t, [(18, 19), (13, 22), (10, 17)], "r")
    # cristaux de sel dans les mains et sur les epaules
    for x, y in ((10, 14), (38, 14)):
        t.p(P([(x - 2, y + 1), (x, y - 5), (x + 2, y + 1)]), "s", trait=0)
        t.px((x, y - 2), "s", 4)
    for x, y, h in ((17, 16, 4), (31, 16, 4), (14, 30, 3), (34, 31, 3)):
        t.p(P([(x - 1, y), (x, y - h), (x + 1, y)]), "s", trait=0, ton=3)
    # long voile blanc, yeux revulses
    t.p(B([(17, 16), (17, 6), (23, 2), (29, 6), (30, 16), (33, 30), (28, 20), (20, 19)]), "r", trait=0, dl=1)
    t.p(E(22, 10, 3.6, 4), "c", trait=0)
    t.px([(20, 10), (23, 10)], "y", 4)
    t.px([(21, 13), (22, 13)], "c", 0)
    t.p(R(19, 5, 26, 5), "g", trait=0, ton=3)


@sprite("humain-contrebandier", "Contrebandier", fichier="contrebandier", **H)
def contrebandier(t):
    t.mat("c", PEAU); t.mat("v", "1e2e48"); t.mat("w", "e8e4dc"); t.mat("r", "a82a2a"); t.mat("b", "3a3440")
    t.mat("n", "2a2024"); t.mat("l", ACIER); t.mat("f", FEU); t.mat("a", FER_NOIR); t.mat("h", "2a2024")
    jambes(t, "b", "n", pas=1)
    torse(t, "w")
    for y in (19, 22, 25, 28):
        t.p(R(19, y, 29, y), "r", trait=0, ton=2, sur="w")
    # caban ouvert
    t.p(B([(16, 17), (20, 16), (20, 32), (15, 34), (15, 26)]) | B([(28, 16), (32, 17), (33, 26), (33, 34), (28, 32)]), "v", trait=0)
    # coutelas a la ceinture, lanterne sourde
    bras(t, [(31, 19), (35, 25), (36, 30)], "v", dl=-1, trait=0)
    t.p(CS([(36, 31), (41, 38), (44, 44)]), "l", trait=0, ton=3)
    bras(t, [(17, 19), (12, 24), (10, 28)], "v")
    t.p(R(10, 30, 10, 32), "a", trait=0, ton=3)
    t.p(R(7, 33, 13, 39), "a", trait=0)
    t.p(R(9, 34, 11, 38), "f", trait=0, ton=3)
    tete(t)
    t.p(B([(18, 11), (21, 12), (25, 12), (27, 10), (26, 14), (21, 15)]), "h", trait=0)
    t.p(B([(18, 7), (20, 3), (27, 3), (29, 8), (24, 6)]), "r", trait=0)
    t.p(K([(29, 7, 1), (33, 10, 0.8)]), "r", trait=0)
    t.px((27, 11), "f", 3)


@sprite("humain-contremaitre", "Contremaître", fichier="contremaitre", **H)
def contremaitre(t):
    t.mat("c", PEAU); t.mat("v", "8a7a5a"); t.mat("t", "5a4030"); t.mat("b", "4a4038"); t.mat("n", "2e2420")
    t.mat("p", ["6a5a20", "a08c30", "d0bc48", "ece080", "fcf4c0"]); t.mat("h", "3a2a22")
    jambes(t, "b", "n")
    torse(t, "v", large=1)
    t.p(B([(19, 20), (29, 20), (30, 36), (18, 36)]), "t", trait=0)
    bras(t, [(31, 19), (35, 25), (34, 31)], "v", dl=-1, trait=0)
    bras(t, [(17, 19), (12, 23), (8, 20)], "v")
    # le fouet
    t.p(C(8, 22, 8, 17, 0.9), "n", trait=0, ton=3)
    t.p(CS([(8, 17), (3, 22), (5, 30), (1, 36), (4, 42)]), "n", trait=0, ton=3)
    tete(t)
    t.p(B([(18, 11), (21, 12), (25, 12), (27, 10), (26, 15), (21, 16)]), "h", trait=0)
    # chapeau de paille
    t.p(E(23, 5.5, 8.6, 1.8), "p", trait=1)
    t.p(B([(18, 5), (20, 1), (26, 1), (28, 5)]), "p", trait=0)
    t.p(R(19, 4, 27, 4), "n", trait=0, ton=3)


@sprite("humain-chasseresse", "Chasseresse", fichier="chasseresse", **H)
def chasseresse(t):
    t.mat("c", PEAU); t.mat("v", "3f5a34"); t.mat("r", "8a3a2a"); t.mat("b", "5a4a38"); t.mat("n", "2e2420")
    t.mat("m", BOIS); t.mat("h", "6a3a20"); t.mat("w", "e8e4dc")
    # carquois dans le dos
    t.p(P([(30, 14), (35, 12), (36, 26), (32, 27)]), "b", trait=0)
    t.px([(32, 10), (34, 9), (33, 11)], "w", 3)
    jambes(t, "b", "n", pas=1)
    torse(t, "r")
    t.p(R(18, 28, 30, 29), "n", trait=0)
    # arc bande, fleche encochee
    t.p(CS([(8, 4), (3, 20), (8, 36)]), "m", trait=0)
    t.p(L(8, 4, 20, 20) | L(8, 36, 20, 20), "w", trait=0, ton=3)
    t.p(R(2, 20, 20, 20), "m", trait=0, ton=3); t.px([(1, 20), (0, 20)], "w", 4)
    bras(t, [(18, 19), (12, 20), (5, 20)], "r")
    bras(t, [(30, 19), (26, 22), (21, 20)], "r", dl=-1)
    tete(t)
    # capuchon vert, tresse rousse
    t.p(B([(18, 16), (18, 6), (23, 2), (28, 5), (30, 14), (26, 9), (21, 8)]), "v", trait=0, dl=1)
    t.p(K([(27, 13, 1.2), (29, 19, 1), (28, 24, 0.8)]), "h", trait=0)
    t.p(B([(16, 17), (24, 15), (31, 17), (29, 21), (24, 19), (18, 21)]), "v", trait=0)


@sprite("humain-fossoyeur", "Fossoyeur", fichier="fossoyeur", **H)
def fossoyeur(t):
    t.mat("c", PEAU); t.mat("v", "4a4640"); t.mat("b", "3a3632"); t.mat("n", "24201e"); t.mat("m", BOIS)
    t.mat("l", ACIER); t.mat("g", "b8b4b0"); t.mat("f", FEU); t.mat("a", FER_NOIR)
    # pelle
    t.p(R(9, 8, 10, 38), "m", trait=0)
    t.p(B([(6, 38), (13, 38), (13, 44), (9, 47), (6, 44)]), "l", trait=0)
    t.p(R(7, 7, 12, 8), "m", trait=0, dl=-1)
    jambes(t, "b", "n")
    # long manteau
    t.p(B([(17, 17), (24, 14), (31, 17), (33, 28), (34, 40), (15, 40), (15, 28)]), "v", trait=0)
    t.p(L(24, 17, 24, 39), "n", trait=0, ton=2)
    bras(t, [(31, 19), (35, 26), (36, 31)], "v", dl=-1, trait=0)
    t.p(R(36, 33, 36, 34), "a", trait=0, ton=3)
    t.p(R(33, 35, 39, 41), "a", trait=0)
    t.p(R(35, 36, 37, 40), "f", trait=0, ton=3)
    bras(t, [(17, 19), (12, 24), (10, 22)], "v")
    tete(t)
    t.p(B([(18, 11), (21, 12), (25, 12), (27, 10), (27, 17), (22, 20), (19, 16)]), "g", trait=0)
    t.p(E(23, 5.5, 9, 1.8), "n", trait=1)
    t.p(B([(18, 5), (19, 0), (27, 0), (28, 5)]), "n", trait=0)


@sprite("humain-capitaine-mercenaire", "Capitaine mercenaire", fichier="capitaine-mercenaire", **H)
def capitaine(t):
    t.mat("c", PEAU); t.mat("v", "6a4a34"); t.mat("b", "3e3a44"); t.mat("n", "2a2024"); t.mat("r", "a82a2a")
    t.mat("l", ACIER); t.mat("m", BOIS); t.mat("g", OR); t.mat("h", "3a2a22")
    # hache d'armes sur l'epaule
    t.p(C(34, 30, 38, 4, 1), "m", trait=0)
    t.p(B([(38, 6), (45, 2), (47, 10), (44, 16), (38, 14)]), "l", trait=0)
    jambes(t, "b", "n", pas=1)
    torse(t, "v", large=1)
    t.p(L(17, 18, 30, 30), "r", trait=0, ton=2); t.p(L(17, 19, 29, 30), "r", trait=0, ton=1)
    t.p(R(17, 29, 31, 30), "n", trait=0)
    t.p(E(31, 33, 2.4, 2.8), "g", trait=0, dl=-1); t.px((31, 30), "n", 2)
    bras(t, [(31, 19), (35, 24), (34, 28)], "v", dl=-1, trait=0)
    bras(t, [(17, 19), (12, 25), (13, 30)], "v")
    tete(t)
    t.p(B([(18, 11), (21, 12), (25, 12), (27, 10), (26, 15), (21, 16)]), "h", trait=0)
    # chapeau a plume
    t.p(E(23, 5.5, 9, 1.8), "n", trait=1)
    t.p(B([(18, 5), (20, 0), (27, 0), (28, 5)]), "n", trait=0)
    t.p(CS([(27, 3), (33, -2), (37, 2)]), "r", trait=0, ton=3)
    t.p(CS([(27, 4), (33, -1), (37, 3)]), "r", trait=0, ton=2)


@sprite("humain-archiviste", "Archiviste", fichier="archiviste", **H)
def archiviste(t):
    t.mat("c", ["6a5a68", "9a8a98", "c4b4c0", "e4d8e0", "fbf4f8"]); t.mat("r", "5a5660"); t.mat("g", OR)
    t.mat("p", ["8a7a5a", "b8a880", "dcd0a8", "f2ead0", "fffcf0"]); t.mat("y", SANG); t.mat("n", "2a2630")
    robe(t, "r")
    t.p(R(17, 29, 31, 30), "g", trait=0, ton=1)
    t.px([(27, 31), (27, 33), (27, 35)], "g", 2)
    # un rouleau deroule devant lui, une plume
    bras(t, [(30, 19), (33, 25), (27, 27)], "r", dl=-1, trait=0)
    bras(t, [(18, 19), (13, 24), (10, 22)], "r")
    t.p(R(2, 18, 12, 30), "p", trait=1)
    t.p(E(7, 18, 5.4, 1.6) | E(7, 30, 5.4, 1.6), "p", trait=0, dl=-1)
    for y in (21, 23, 25, 27):
        t.p(R(4, y, 10, y), "n", trait=0, ton=3)
    t.p(L(27, 26, 33, 16), "p", trait=0, ton=4)
    # tonsure, yeux rouges de trop lire
    tete(t, yeux=False)
    t.px([(20, 9), (23, 9)], "y", 3)
    t.p(B([(19, 8), (20, 5), (26, 5), (28, 9), (27, 12), (25, 7), (21, 7)]), "r", trait=0, dl=-1)
    t.p(B([(16, 17), (24, 15), (31, 17), (29, 21), (24, 19), (18, 21)]), "r", trait=0, dl=1)


@sprite("humain-pyromancien", "Pyromancien", fichier="pyromancien", **H)
def pyromancien(t):
    t.mat("c", PEAU); t.mat("r", "a8301c"); t.mat("g", OR); t.mat("f", FEU); t.mat("n", "2a2024")
    robe(t, "r")
    t.p(R(17, 29, 31, 30), "g", trait=0)
    t.p(P([(21, 17), (24, 22), (27, 17)]), "g", trait=0, ton=1)
    t.px([(14, 44), (20, 43), (28, 43), (34, 44)], "f", 2)
    # deux mains levees, une flamme dans chaque paume
    bras(t, [(30, 19), (36, 20), (39, 14)], "r", dl=-1, trait=0)
    bras(t, [(18, 19), (12, 20), (9, 14)], "r")
    for x in (9, 39):
        t.p(B([(x - 3, 11), (x - 2, 5), (x, 0), (x + 2, 5), (x + 3, 11)]), "f", trait=0, ton=2)
        t.px([(x, 6), (x, 8), (x, 10)], "f", 4)
    # crane rase, tatouages de braise, sourcils brules
    tete(t)
    t.px([(22, 5), (24, 6), (26, 5), (25, 8)], "f", 1)
    t.px([(19, 8), (20, 8), (22, 8), (23, 8)], "n", 2)
    t.px([(4, 16), (44, 16), (6, 2), (42, 3)], "f", 3)


@sprite("humain-heraut-de-l-orage", "Héraut de l'orage", fichier="heraut-de-l-orage", **H)
def heraut(t):
    t.mat("a", ACIER); t.mat("c", PEAU); t.mat("d", ["1c2a5a", "30468c", "4c6cbc", "84a0e0", "c4d4f8"])
    t.mat("e", ["8a6a10", "c8a018", "f8d830", "fff080", "ffffd8"]); t.mat("m", BOIS); t.mat("n", "2a2630"); t.mat("g", OR)
    # banniere etoilee
    t.p(R(9, -4, 10, 45), "m", trait=0)
    t.p(P([(11, -3), (26, -3), (26, 12), (18, 9), (11, 12)]), "d", trait=0)
    t.px([(14, 0), (22, 1), (18, 4), (14, 7), (23, 7)], "e", 4)
    jambes(t, "a", "n")
    # tabard frappe d'un eclair
    t.p(B([(17, 17), (24, 14), (31, 17), (32, 27), (32, 37), (16, 37), (16, 27)]), "d", trait=0)
    t.p(LS([(26, 18), (22, 25), (26, 26), (22, 34)]), "e", trait=0, ton=3)
    t.p(LS([(25, 18), (21, 25), (25, 26), (21, 34)]), "e", trait=0, ton=2)
    # une trompe d'appel
    bras(t, [(30, 19), (35, 22), (36, 16)], "a", dl=-1, trait=0, main=False)
    t.p(S([(35, 14, 0.8), (40, 10, 1.2), (45, 5, 2.4)]), "g", trait=0)
    bras(t, [(18, 19), (13, 23), (10, 20)], "a", main=False)
    t.p(E(10, 19, 2, 2), "a", trait=0, dl=1)
    # salade a visiere levee
    tete(t, cy=11)
    t.p(B([(18, 9), (19, 3), (25, 1), (29, 5), (29, 10), (24, 7)]), "a", trait=0)
    t.p(R(17, 7, 23, 8), "a", trait=0, ton=4)


@sprite("humain-favorite", "Favorite", fichier="favorite", **H)
def favorite(t):
    t.mat("c", PEAU); t.mat("r", ["3a0410", "70101e", "a82030", "d84a50", "f8a09c"]); t.mat("g", OR); t.mat("h", "241c22")
    t.mat("l", ACIER); t.mat("s", SANG)
    # robe a traine
    t.p(B([(19, 18), (24, 16), (29, 18), (28, 27), (34, 36), (42, 45), (10, 45), (14, 36), (20, 27)]), "r", trait=0)
    for x in (16, 21, 28, 34):
        t.p(L(x, 34, x + (x - 24) // 3, 44), "r", trait=0, ton=0, sur="r")
    t.p(R(20, 26, 28, 27), "g", trait=0)
    t.p(B([(21, 18), (24, 20), (27, 18), (26, 23), (22, 23)]), "c", trait=0)
    t.px((24, 19), "g", 4)
    # une coupe de vin dans une main, un stylet cache derriere le dos
    bras(t, [(28, 19), (33, 25), (36, 29)], "r", dl=-1, trait=0)
    t.p(P([(36, 30), (41, 36), (37, 33)]), "l", trait=0, ton=4)
    bras(t, [(20, 19), (14, 23), (10, 20)], "r")
    t.p(P([(6, 13), (13, 13), (11, 17), (8, 17)]), "g", trait=0)
    t.p(R(9, 17, 10, 19), "g", trait=0); t.p(R(7, 13, 12, 13), "s", trait=0, ton=2)
    # chevelure noire relevee, peigne d'or
    t.p(B([(19, 12), (19, 4), (24, 0), (29, 4), (30, 13), (33, 24), (28, 16), (26, 8), (21, 7)]), "h", trait=0)
    tete(t, cy=10)
    t.px([(20, 13), (21, 13)], "s", 3)
    t.p(R(25, 2, 28, 3), "g", trait=0, ton=3)
