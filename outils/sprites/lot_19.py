from dessin import *
from matieres import *
from atelier import sprite
from commun import _aile_membrane
import math

VIDE = ["0a0814", "181430", "2c2650", "4c4480", "8478c0"]


@sprite("leviathan", "Léviathan")
def leviathan(t):
    t.mat("e", ["0c1e3a", "183864", "2c5c94", "5a8cc0", "a0c8ec"], ["3a0c1e", "641838", "942c5c", "c05a8c", "eca0c8"])
    t.mat("v", "b8d4e0", "e0c8d4")
    t.mat("n", ["0c3a4a", "18707a", "30a8a8", "78d8d0", "c8f8f0"], FEU)   # nageoires
    t.mat("o2", ["1c3a6a", "2e5c98", "4a88c8", "84bce8", "d0ecfc"])
    t.mat("w", OS)
    t.mat("y", "f8e040")
    # la mer
    t.p(B([(0, 40), (8, 36), (18, 39), (30, 36), (40, 39), (47, 37), (47, 46), (0, 46)]), "o2", trait=0)
    t.px([(6, 38), (9, 37), (28, 37), (31, 37), (44, 38), (14, 43), (36, 43)], "o2", 4)
    # deux arches du corps, puis la queue
    t.p(S([(46, 40, 3.4), (44, 31, 4.2), (39, 27, 4.4), (34, 31, 4.2), (33, 40, 3.8)]), "e", trait=0)
    t.p(P([(44, 26), (47, 18), (41, 24)]), "n", trait=0)
    # col dresse, enorme
    corps = [(22, 42, 6), (23, 32, 6.4), (26, 22, 6.4), (23, 13, 6), (16, 8, 5.4)]
    t.p(S(corps), "e", trait=0)
    t.p(S([(19, 42, 2.2), (19, 32, 2.4), (22, 22, 2.4), (19, 14, 2.2)]), "v", trait=0, sur="e")
    for y in (18, 23, 28, 33, 38):
        t.p(L(16, y, 23, y + 1), "e", trait=0, ton=1, sur="v")
    # crete en voile
    t.p(P([(27, 10), (36, 8), (33, 16), (37, 20), (32, 24), (36, 30), (30, 32), (29, 20)]), "n", trait=0)
    t.p(L(29, 14, 35, 10) | L(31, 22, 36, 21) | L(30, 28, 35, 30), "n", trait=0, ton=0)
    # tete : machoires de crocodile, cornes
    t.p(B([(0, 9), (5, 3), (14, 1), (21, 5), (21, 12), (13, 14), (4, 14)]), "e", trait=0)
    t.p(P([(17, 2), (25, -2), (21, 5)]) | P([(13, 1), (17, -4), (17, 2)]), "w", trait=0)
    t.p(P([(0, 12), (8, 13), (15, 13), (11, 18), (3, 17)]), "e", trait=1, dl=-1)
    t.px([(1, 13), (3, 13), (5, 13), (7, 13), (9, 13), (11, 13), (3, 16), (6, 16), (9, 16)], "w", 4)
    t.px([(10, 6), (11, 6)], "y", 3); t.px((10, 6), "o"); t.px([(9, 5), (10, 5), (11, 5), (12, 4)], "o")
    t.px([(27, 28), (28, 18), (25, 36)], "e", 4)


@sprite("behemoth", "Béhémoth")
def behemoth(t):
    t.mat("c", ["3a2a1c", "5e4630", "86684a", "ae906c", "d6bc98"], ["1c2a3a", "30465e", "4a6886", "6c90ae", "98bcd6"])   # cuir
    t.mat("g", PIERRE)
    t.mat("w", OS)
    t.mat("y", SANG, FEU_VERT)
    t.mat("l", "5a8a4a", "8a6aa8")
    t.p(K([(22, 33, 4), (22, 40, 3.2), (22, 44, 3.2)]), "c", dl=-1, trait=0)
    t.p(K([(40, 33, 4), (41, 40, 3.2), (41, 44, 3.2)]), "c", dl=-1, trait=0)
    # corps de montagne, echine de roc
    t.p(B([(10, 22), (17, 13), (30, 11), (42, 16), (46, 26), (44, 35), (34, 38), (18, 38), (10, 33)]), "c", trait=0)
    t.p(B([(18, 13), (24, 6), (33, 5), (41, 11), (43, 18), (36, 15), (27, 14)]), "g", trait=0)
    t.px([(24, 8), (30, 7), (36, 10), (40, 14)], "g", 0)
    t.px([(26, 6), (33, 6), (39, 11)], "l", 2)
    t.p(K([(16, 34, 4.4), (15, 40, 3.4), (15, 44, 3.4)]), "c")
    t.p(K([(35, 34, 4.4), (35, 40, 3.4), (35, 44, 3.4)]), "c")
    t.px([(12, 46), (14, 46), (16, 46), (18, 46), (32, 46), (34, 46), (36, 46), (38, 46)], "w", 3)
    # tete basse : mufle large, defenses d'ivoire, corne nasale
    t.p(B([(1, 25), (4, 17), (12, 14), (18, 19), (18, 30), (10, 34), (3, 32)]), "c", trait=0)
    t.p(P([(4, 18), (3, 10), (9, 16)]), "w", trait=0)
    t.p(CS([(8, 32), (4, 37), (1, 32)]) | CS([(12, 32), (10, 38), (5, 37)]), "w", trait=0, ton=4)
    t.px([(10, 21), (11, 21)], "y", 3); t.px((10, 21), "y", 4); t.px([(9, 20), (10, 20), (11, 20), (12, 19)], "o")
    t.px([(2, 27), (3, 28)], "o")
    t.px([(22, 22), (28, 26), (34, 22), (40, 28), (24, 32)], "c", 0)
    t.grain("g", 0.08, 14)


@sprite("ziz", "Ziz")
def ziz(t):
    t.mat("p", ["a89468", "d4c498", "f0e6c8", "fbf7e8", "ffffff"], ["3a3048", "5a4c70", "8474a0", "b4a8cc", "e4dcf4"])   # plumes d'ivoire
    t.mat("g", OR, FEU_VIOLET)
    t.mat("b", "e8b030", "c0c0d0")
    t.mat("y", "3a7ad0", "d03a3a")
    # le soleil qu'il cache
    t.p(E(24, 14, 13, 13), "g", trait=0, ton=3)
    t.p(E(24, 14, 13, 13) - E(24, 14, 11.6, 11.6), "g", trait=0, ton=2)
    # ailes immenses, d'un bord a l'autre
    for (bx, by), pointes, dl in (((27, 24), ((34, 6), (40, 7), (44, 11), (47, 16), (47, 22), (44, 28)), -1),
                                  ((21, 24), ((14, 6), (8, 7), (4, 11), (1, 16), (1, 22), (4, 28)), 0)):
        for i, (x, y) in enumerate(pointes):
            t.p(C(bx, by, x, y, 3.6, 1.2), "p", trait=0, dl=dl - (i % 2))
            t.px((x, y), "g", 3)
    # queue en traine
    for x, y in ((18, 45), (24, 47), (30, 45)):
        t.p(K([(24, 34, 2), (x, y, 0.8)]), "p", trait=0, dl=-1)
        t.px((x, y), "g", 4)
    # corps, poitrail d'or
    t.p(B([(18, 22), (24, 18), (30, 22), (31, 32), (27, 38), (21, 38), (17, 32)]), "p", trait=0)
    t.p(B([(20, 24), (24, 22), (28, 24), (27, 30), (24, 32), (21, 30)]), "g", trait=0)
    t.px([(20, 39), (21, 40), (22, 39), (26, 39), (27, 40), (28, 39)], "b", 3)
    # tete couronnee de plumes, bec d'or
    t.p(E(24, 14, 4.4, 4.4), "p", trait=0)
    t.p(P([(22, 15), (24, 21), (26, 15)]), "b", trait=0)
    t.px([(22, 13), (26, 13)], "y", 3); t.px([(22, 12), (26, 12)], "o")
    for x, h in ((21, 4), (24, 6), (27, 4)):
        t.p(P([(x - 1, 10), (x, 10 - h), (x + 1, 10)]), "g", trait=0, ton=3)


@sprite("colosse-du-rempart", "Colosse du rempart")
def colosse(t):
    t.mat("g", ["4a4850", "6e6c76", "94929c", "bab8c0", "dedce2"], ["504a48", "766e6c", "9c9492", "c0bab8", "e2dedc"])   # pierre de taille
    t.mat("a", ACIER)
    t.mat("y", FEU, FEU_BLEU)
    t.mat("l", "5a8a4a", "8a6aa8")
    # lance courte
    t.p(C(42, 46, 40, 2, 1), "a", trait=0, ton=1)
    t.p(P([(38, 4), (40, -3), (42, 4)]), "a", trait=0, ton=4)
    # jambes en tours
    t.p(R(27, 34, 34, 44), "g", trait=0, dl=-1); t.p(R(18, 34, 25, 44), "g", trait=0)
    t.p(R(16, 44, 26, 45) | R(26, 44, 36, 45), "g", trait=0)
    # torse : un pan de muraille, creneaux aux epaules
    t.p(B([(16, 16), (26, 13), (37, 16), (38, 28), (35, 36), (18, 36), (15, 28)]), "g", trait=0)
    for x in (30, 34, 38):
        t.p(R(x, 11, x + 2, 15), "g", trait=0)
    for y in (20, 25, 30):
        t.p(L(16, y, 38, y), "g", trait=0, ton=1, sur="g")
    for x, y in ((22, 20), (30, 20), (26, 25), (34, 25), (20, 30), (30, 30)):
        t.p(L(x, y, x, y + 5), "g", trait=0, ton=1, sur="g")
    t.p(K([(37, 18, 3), (41, 25, 2.6), (41, 30, 2.4)]), "g", trait=1)
    # le bouclier, haut comme une tour, garni de fer
    t.p(P([(1, 6), (17, 4), (18, 40), (9, 46), (2, 40)]), "a", trait=1, dl=-1)
    t.p(P([(3, 8), (15, 7), (16, 39), (9, 44), (4, 39)]), "g", trait=0)
    t.p(R(9, 8, 10, 43) | R(3, 22, 16, 23), "a", trait=0)
    t.px([(5, 10), (13, 10), (5, 36), (14, 36), (9, 22)], "a", 4)
    # heaume en casque de guet, fente rougeoyante
    t.p(B([(21, 14), (22, 5), (27, 2), (32, 5), (33, 14)]), "a", trait=0)
    t.p(R(23, 8, 31, 9), "y", trait=0, ton=2)
    t.px([(24, 8), (30, 8)], "y", 4)
    t.px([(26, 12), (28, 12)], "a", 0)
    t.px([(19, 35), (20, 34), (36, 17), (37, 16), (4, 40), (5, 41)], "l", 2)
    t.grain("g", 0.07, 15)


@sprite("cardinal-du-vide", "Cardinal du Vide")
def cardinal(t):
    t.mat("r", ["3a0810", "6c1420", "a02434", "d0505c", "f4a0a4"], ["08103a", "14206c", "2434a0", "505cd0", "a0a4f4"])   # pourpre cardinalice
    t.mat("c", ["6a5a68", "9a8a98", "c4b4c0", "e4d8e0", "fbf4f8"])   # crane livide
    t.mat("g", OR)
    t.mat("y", FEU_VIOLET, FEU_VERT)
    t.mat("v", VIDE)
    # robe sans pieds, flottante
    t.p(B([(14, 22), (24, 18), (34, 22), (38, 34), (36, 43), (31, 46), (28, 41), (24, 46), (20, 41), (17, 46), (12, 43), (10, 34)]),
        "r", trait=0)
    for x in (15, 20, 28, 33):
        t.p(L(x, 28, x + (x - 24) // 4, 42), "r", trait=0, ton=0, sur="r")
    # camail, croix pectorale renversee
    t.p(B([(14, 22), (24, 18), (34, 22), (32, 28), (24, 26), (16, 28)]), "r", trait=0, dl=1)
    t.p(R(23, 28, 25, 38) | R(20, 34, 28, 35), "g", trait=0)
    # mains jointes sur un calice ou tourne le vide
    t.p(K([(16, 24, 2.4), (14, 30, 2.2), (20, 32, 1.9)]) | K([(32, 24, 2.4), (34, 30, 2.2), (28, 32, 1.9)]), "r", trait=1)
    t.p(P([(20, 27), (28, 27), (26, 32), (22, 32)]), "g", trait=1)
    t.p(R(20, 27, 28, 27), "v", trait=0, ton=0)
    # tete : un crane demesure, chauve, ou des yeux s'ouvrent sans paupieres
    t.p(B([(13, 12), (15, 4), (24, 0), (33, 4), (35, 12), (31, 19), (24, 21), (17, 19)]), "c", trait=0)
    for x, y, r in ((20, 14, 1.8), (28, 14, 1.8), (24, 8, 2.2), (17, 8, 1.3), (31, 8, 1.3), (24, 3, 1.2)):
        t.p(E(x, y, r, r * 0.8), "y", trait=2, ton=3)
        t.px((x, y), "o")
    t.px([(22, 18), (23, 18), (25, 18), (26, 18)], "c", 0)
    t.p(E(24, 1, 5, 1.6) - R(19, 1, 29, 3), "r", trait=0)
    t.px([(6, 20), (42, 22), (8, 40), (40, 42)], "y", 3)


@sprite("dame-des-supplices", "Dame des supplices")
def dame_des_supplices(t):
    t.mat("c", ["6a5a70", "9a8aa4", "c8bcd0", "e8e0ec", "fffaff"])   # peau de nacre
    t.mat("n", ["1a1420", "30283c", "4c4260", "766c90", "aca4c4"], ["201a14", "3c3028", "604c42", "90766c", "c4aca4"])   # ailes, cheveux
    t.mat("a", ACIER)                # chaines
    t.mat("s", SANG, FEU_VERT)
    # ailes de plumes sombres, en manteau
    for cote, dl in ((1, -1), (-1, 0)):
        c = lambda x: 24 + cote * x
        for i, (dx, y) in enumerate(((17, 6), (20, 14), (20, 23), (17, 31), (12, 37))):
            t.p(C(c(4), 16, c(dx), y, 3.4, 1.4), "n", trait=0, dl=dl - (i % 2))
    # corps nu drape de chaines, hanches en fuseau
    t.p(B([(19, 19), (24, 17), (29, 19), (30, 28), (28, 36), (26, 45), (22, 45), (20, 36), (18, 28)]), "c", trait=0)
    t.p(R(22, 45, 26, 46), "c", trait=0, dl=-1)
    for (x0, y0), (x1, y1) in (((18, 22), (30, 27)), ((30, 22), (18, 28)), ((19, 31), (29, 35)), ((29, 31), (19, 36)),
                               ((21, 38), (27, 42))):
        t.p(L(x0, y0, x1, y1), "a", trait=0, ton=3, sur="c")
    t.px([(24, 24), (24, 33), (24, 40)], "a", 4)
    # bras ouverts : des chaines pendent de ses poignets, crochets au bout
    t.p(K([(20, 20, 1.5), (13, 22, 1.2), (8, 18, 1.1)]) | K([(28, 20, 1.5), (35, 22, 1.2), (40, 18, 1.1)]), "c", trait=1)
    for x in (8, 40):
        for y in range(20, 36, 3):
            t.p(E(x, y, 1.2, 1.5) - E(x, y, 0.3, 0.6), "a", trait=0, ton=3 if (y // 3) % 2 else 2)
        t.p(CS([(x, 36), (x + 2, 40), (x - 1, 41)]), "a", trait=0, ton=4)
    # tete : sourire, larmes de sang, longue chevelure noire
    t.p(B([(18, 12), (21, 5), (27, 5), (30, 12), (32, 26), (28, 18), (20, 18), (16, 26)]), "n", trait=0)
    t.p(E(24, 12, 3.6, 4.2), "c", trait=0)
    t.px([(22, 11), (26, 11)], "s", 3); t.px([(22, 10), (26, 10)], "o")
    t.px([(22, 12), (22, 13), (26, 12), (26, 13), (26, 14)], "s", 2)
    t.px([(22, 15), (23, 16), (24, 16), (25, 16), (26, 15)], "s", 1)
    t.px([(24, 21), (23, 28)], "s", 2)


@sprite("bouffon-ecorche", "Bouffon écorché")
def bouffon(t):
    t.mat("c", ["4a1420", "7a2a38", "aa4a58", "d08088", "f0bcc0"], ["14204a", "2a387a", "4a58aa", "8088d0", "bcc0f0"])   # chair a vif
    t.mat("v", ["1c5a2a", "2e8a40", "4cb85c", "88dc90", "c8f4cc"], ["5a1c4a", "8a2e74", "b84c9c", "dc88c8", "f4c8ec"])   # mi-parti vert
    t.mat("p", ["3a1a6a", "5c2ea0", "844cd0", "b488ec", "dcc8fc"], ["6a5a1a", "a08c2e", "d0bc4c", "ece088", "fcf4c8"])   # mi-parti violet
    t.mat("g", OR)
    t.mat("k", OS)
    # jambes sans peau, en pas de danse
    t.p(K([(27, 32, 2), (33, 37, 1.6), (31, 44, 1.4)]), "c", trait=0, dl=-1)
    t.p(K([(21, 32, 2), (17, 38, 1.6), (20, 44, 1.4)]), "c", trait=0)
    t.p(P([(18, 44), (14, 46), (22, 46)]) | P([(29, 44), (35, 46), (33, 43)]), "v", trait=0)
    t.px([(13, 45), (36, 45)], "g", 4)
    # pourpoint mi-parti, losanges
    t.p(B([(18, 18), (24, 16), (24, 34), (17, 34), (16, 26)]), "v", trait=0)
    t.p(B([(24, 16), (30, 18), (32, 26), (31, 34), (24, 34)]), "p", trait=0)
    for x, y in ((20, 22), (20, 29), (28, 22), (28, 29)):
        t.p(P([(x - 2, y), (x, y - 3), (x + 2, y), (x, y + 3)]), "g", trait=0, ton=1)
    for x in (17, 21, 25, 29):
        t.p(P([(x, 34), (x + 2, 38), (x + 4, 34)]), "p" if x % 8 == 1 else "v", trait=0)
        t.px((x + 2, 39), "g", 4)
    # bras : marotte a tete de mort, et l'autre main fait la reverence
    t.p(K([(18, 20, 1.8), (12, 17, 1.5), (9, 11, 1.3)]), "c", trait=1)
    t.p(C(9, 14, 8, 2, 0.8), "g", trait=0)
    t.p(E(8, 0, 2.6, 2.4), "k", trait=0); t.px([(7, 0), (9, 0)], "o")
    t.px([(5, -2), (11, -2)], "g", 4)
    t.p(K([(30, 20, 1.8), (36, 24, 1.5), (40, 20, 1.3)]), "c", trait=1, dl=-1)
    # tete ecorchee, rire fendu, bonnet a trois cornes et grelots
    t.p(E(24, 11, 4.2, 4.4), "c", trait=0)
    t.px([(22, 10), (26, 10)], "k", 4); t.px([(22, 10), (26, 10)], "o")
    t.p(P([(20, 12), (24, 16), (28, 12), (24, 14)]), "k", trait=0, ton=4)
    t.px([(21, 13), (23, 14), (25, 14), (27, 13)], "o")
    t.p(CS([(21, 8), (16, 5), (13, 9)]), "v", trait=0); t.p(CS([(27, 8), (32, 5), (35, 9)]), "p", trait=0)
    t.p(P([(21, 8), (24, 0), (27, 8)]), "g", trait=0, ton=1)
    t.px([(13, 10), (35, 10), (24, -1)], "g", 4)


@sprite("seigneur-pestilence", "Seigneur Pestilence")
def pestilence(t):
    t.mat("c", ["3a4420", "5e6c30", "869648", "b0c070", "d8e4a8"], ["44203a", "6c305e", "964886", "c070b0", "e4a8d8"])   # chair pourrie
    t.mat("b", ["6a5a20", "a08c30", "d0bc48", "ece080", "fcf4c0"])   # bubons
    t.mat("r", "5a4a34", "34445a")   # robe de bure
    t.mat("m", "2a2a30")             # mouches
    t.mat("p", FEU_VERT, FEU_VIOLET)
    t.mat("k", OS)
    # corps bouffi sous une bure trop petite
    t.p(B([(10, 24), (16, 15), (32, 15), (38, 24), (40, 36), (36, 45), (12, 45), (8, 36)]), "c", trait=0)
    t.p(B([(13, 30), (24, 27), (35, 30), (37, 45), (11, 45)]), "r", trait=0)
    t.px([(13, 45), (18, 43), (24, 45), (30, 43), (35, 45), (16, 36), (31, 37)], "r", 0)
    t.p(R(13, 30, 35, 31), "k", trait=0, ton=1)
    # bubons
    for x, y, r in ((14, 22), (20, 19), (30, 20), (35, 25), (24, 24), (17, 27))[:6] if False else ((14, 22, 2.2), (20, 19, 1.8), (30, 20, 2.4), (35, 26, 2), (25, 24, 1.6)):
        t.p(E(x, y, r, r), "b", trait=0)
        t.px((x - 1, y - 1), "b", 4)
    # bras : une clochette de lepreux, un encensoir qui fume vert
    t.p(K([(11, 24, 3), (6, 30, 2.6), (6, 36, 2.2)]), "c", trait=1)
    t.p(P([(3, 38), (6, 35), (9, 38), (9, 41), (3, 41)]), "b", trait=0, dl=-1); t.px((6, 42), "k", 3)
    t.p(K([(37, 24, 3), (42, 29, 2.6), (42, 33, 2.2)]), "c", trait=0, dl=-1)
    t.p(R(42, 34, 42, 39), "k", trait=0, ton=1)
    t.p(E(42, 42, 2.6, 2.4), "b", trait=0, dl=-1)
    for x, y in ((44, 36), (45, 31), (43, 27), (46, 24)):
        t.px((x, y), "p", 3)
    # tete sans cou : capuchon, visage ronge, sourire
    t.p(B([(16, 16), (17, 7), (24, 3), (31, 7), (32, 16), (28, 19), (20, 19)]), "r", trait=0, dl=1)
    t.p(E(24, 12, 4.6, 5), "c", trait=0)
    t.px([(22, 11), (26, 11)], "p", 4); t.px([(22, 10), (26, 10)], "o")
    t.px([(21, 14), (22, 15), (23, 15), (24, 15), (25, 15), (26, 15), (27, 14)], "o")
    t.px([(22, 16), (24, 16), (26, 16)], "k", 4)
    t.px((24, 12), "o")
    # nuee de mouches
    for x, y in ((6, 10), (10, 6), (38, 8), (42, 14), (34, 4), (4, 18), (14, 2), (44, 6)):
        t.px((x, y), "m", 2); t.px((x + 1, y - 1), "k", 4)
