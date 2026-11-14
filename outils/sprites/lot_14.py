from dessin import *
from matieres import *
from atelier import sprite
from commun import _aile_membrane
import math


@sprite("araignee-des-cryptes", "Araignée des cryptes")
def araignee(t):
    t.mat("c", ["2c2a30", "4c4a54", "74727c", "a09ea8", "cccad4"], ["30282a", "544850", "7c6c78", "a898a4", "d4c8d0"])   # chitine de pierre
    t.mat("y", SANG, FEU_VERT)
    t.mat("w", OS)
    t.mat("s", "e8e4f0")             # soie
    # fil de soie
    t.p(R(30, -4, 30, 16), "s", trait=0, ton=3)
    # huit pattes coudees
    for cote in (1, -1):
        c = lambda x: 22 + cote * x
        for i, (x1, y1, x2, y2) in enumerate(((9, 14, 17, 26), (12, 20, 21, 34), (12, 28, 20, 42), (8, 32, 12, 45))):
            t.p(K([(c(4), 28, 1.8), (c(x1), y1, 1.4), (c(x2), y2, 0.8)]), "c", trait=0, dl=-1 if cote > 0 else 0)
    # abdomen : une pierre tombale gravee d'une croix
    t.p(E(31, 26, 10, 9), "c", trait=0)
    t.p(R(30, 20, 31, 31) | R(27, 23, 34, 24), "c", trait=0, ton=0)
    t.px([(25, 20), (36, 22), (37, 30), (27, 33)], "c", 4)
    # cephalothorax, huit yeux, chelicere
    t.p(E(18, 29, 6.5, 6), "c", trait=1)
    for x, y in ((14, 27), (17, 26), (20, 27), (15, 30), (18, 29), (21, 30)):
        t.px((x, y), "y", 3)
    t.px([(13, 29), (12, 31)], "y", 2)
    t.p(P([(14, 33), (15, 37), (17, 34)]) | P([(18, 34), (19, 38), (21, 34)]), "w", trait=0)
    t.grain("c", 0.07, 4)


@sprite("liche", "Liche")
def liche(t):
    t.mat("r", ["1c1038", "34205e", "523688", "7e60b8", "b8a0e4"], ["38101c", "5e2034", "883652", "b8607e", "e4a0b8"])   # robe de mage
    t.mat("k", OS)
    t.mat("g", OR)
    t.mat("y", FEU_VERT, FEU_BLEU)
    t.mat("l", BOIS)
    # baton a crane
    t.p(C(39, 46, 38, 10, 1), "l", trait=0)
    t.p(E(38, 7, 3, 2.8), "k", trait=0)
    t.px([(37, 7), (39, 7)], "y", 3); t.px([(37, 9), (38, 9), (39, 9)], "o")
    # robe flottante, sans pieds
    t.p(B([(16, 18), (24, 15), (32, 18), (35, 32), (33, 42), (29, 46), (26, 41), (22, 46), (18, 41), (14, 45), (13, 32)]), "r", trait=0)
    for x in (17, 21, 27, 31):
        t.p(L(x, 24, x + (x - 24) // 5, 42), "r", trait=0, ton=0, sur="r")
    t.p(L(24, 17, 24, 40), "g", trait=0, ton=2)
    t.px([(23, 20), (25, 20), (23, 26), (25, 26), (23, 32), (25, 32)], "g", 3)
    # manches larges, mains d'os : l'une leve une flamme verte
    t.p(K([(18, 19, 2.6), (11, 22, 2.4), (8, 17, 2)]), "r", trait=1)
    t.p(E(8, 14.5, 1.8, 1.8), "k", trait=0)
    t.p(P([(5, 12), (8, 3), (11, 12)]) | E(8, 10, 2.6, 2.6), "y", trait=0, ton=2)
    t.px([(8, 9), (8, 10), (8, 7)], "y", 4)
    t.p(K([(30, 19, 2.6), (36, 24, 2.4), (38, 28, 2)]), "r", trait=1, dl=-1)
    t.p(E(38.5, 29, 1.8, 1.8), "k", trait=0)
    # crane couronne, sous un col haut
    t.p(B([(17, 18), (16, 9), (20, 13), (28, 13), (32, 9), (31, 18)]), "r", trait=0, dl=-1)
    t.p(E(24, 10, 4.2, 4.4), "k", trait=0)
    t.p(R(21, 12, 27, 15), "k", trait=0)
    t.p(E(22.2, 9.5, 1.3, 1.5) | E(25.8, 9.5, 1.3, 1.5), "o", trait=0, ton=2)
    t.px([(22, 9), (26, 9)], "y", 4)
    t.px([(22, 14), (24, 14), (26, 14)], "o"); t.px((24, 12), "o")
    for x, h in ((20, 3), (22, 5), (24, 6), (26, 5), (28, 3)):
        t.p(R(x, 6 - h, x, 6), "g", trait=0, ton=3)
    t.p(R(20, 5, 28, 6), "g", trait=0)
    t.px([(12, 34), (36, 38), (14, 10), (33, 5)], "y", 3)


@sprite("ange-de-pierre", "Ange de pierre")
def ange_de_pierre(t):
    t.mat("g", ["55535c", "7a7880", "a09ea6", "c4c2c8", "e6e4e8"], ["5c5548", "80786a", "a69e8e", "c8c2b4", "e8e4dc"])
    t.mat("l", "5a8a4a", "8a6aa8")   # lierre
    t.mat("y", ["c87a10", "f0b020", "f8dc50", "fff4a0", "ffffe8"], FEU_BLEU)   # lumiere
    # ailes de pierre repliees haut
    for cote, dl in ((1, -1), (-1, 0)):
        c = lambda x: 24 + cote * x
        t.p(B([(c(4), 16), (c(8), 3), (c(15), 1), (c(19), 10), (c(18), 26), (c(13), 36), (c(7), 30)]), "g", trait=0, dl=dl)
        for y in (10, 16, 22, 28):
            t.p(L(c(8), y - 2, c(17), y + 2), "g", trait=0, ton=0)
    # socle
    t.p(R(12, 41, 36, 45), "g", trait=0, dl=-1)
    t.p(R(14, 39, 34, 41), "g", trait=0)
    # robe de statue, plis droits
    t.p(B([(19, 18), (24, 16), (29, 18), (31, 30), (32, 40), (16, 40), (17, 30)]), "g", trait=0)
    for x in (19, 22, 26, 29):
        t.p(L(x, 26, x, 39), "g", trait=0, ton=0, sur="g")
    # mains jointes, tete inclinee, yeux de lumiere
    t.p(K([(20, 19, 1.8), (19, 25, 1.6), (23, 25, 1.4)]) | K([(28, 19, 1.8), (29, 25, 1.6), (25, 25, 1.4)]), "g", trait=1)
    t.p(P([(22, 26), (24, 20), (26, 26)]), "g", trait=0, ton=4)
    t.p(B([(19, 11), (24, 7), (29, 11), (29, 17), (19, 17)]), "g", trait=0, dl=-1)
    t.p(E(24, 12, 3.6, 4), "g", trait=0, dl=1)
    t.px([(22, 12), (23, 12), (25, 12), (26, 12)], "y", 3)
    t.px([(22, 13), (26, 13), (26, 14)], "y", 4)
    # fissure, lierre
    t.p(LS([(27, 20), (29, 25), (27, 30), (29, 35)]), "g", trait=0, ton=0)
    t.px([(15, 40), (16, 38), (17, 36), (18, 37), (33, 39), (34, 37), (32, 36), (12, 30), (13, 28), (36, 24), (35, 22)], "l", 2)
    t.px([(16, 37), (34, 38), (13, 29)], "l", 4)
    t.grain("g", 0.06, 9)
    # nimbe
    t.p(E(24, 5, 5, 1.6) - E(24, 5, 3.6, 0.7), "y", trait=0, ton=3)


@sprite("eveque-des-os", "Évêque des os")
def eveque(t):
    t.mat("r", ["3a0c1c", "6a1830", "9a2c48", "c85a70", "f0a4b0"], ["0c1c3a", "18306a", "2c489a", "5a70c8", "a4b0f0"])   # chasuble
    t.mat("g", OR)
    t.mat("k", OS)
    t.mat("y", FEU_VIOLET, FEU_VERT)
    # crosse faite d'une colonne vertebrale
    for y in range(12, 46, 3):
        t.p(E(9, y, 1.6, 1.2), "k", trait=0, dl=-(y % 2))
    t.p(CS([(9, 12), (9, 5), (14, 3), (16, 7), (13, 9)]), "k", trait=0)
    # chasuble en cloche, croix d'or
    t.p(B([(16, 18), (24, 15), (32, 18), (36, 32), (38, 45), (10, 45), (12, 32)]), "r", trait=0)
    t.p(R(23, 19, 25, 43) | R(17, 26, 31, 28), "g", trait=0)
    t.px([(24, 21), (24, 31), (24, 39), (19, 27), (29, 27)], "g", 4)
    t.px([(12, 45), (17, 44), (31, 44), (36, 45)], "g", 1)
    # bras d'os : l'un tient la crosse, l'autre benit
    t.p(K([(18, 20, 2.4), (12, 24, 2.2), (10, 22, 1.8)]), "r", trait=1)
    t.p(E(9, 21, 1.8, 1.8), "k", trait=0)
    t.p(K([(30, 20, 2.4), (36, 23, 2.2), (38, 17, 1.8)]), "r", trait=1, dl=-1)
    t.p(E(38.5, 14.5, 1.8, 2), "k", trait=0)
    t.px([(38, 11), (39, 11), (40, 12)], "k", 3)
    # crane, mitre haute
    t.p(E(24, 12, 4.2, 4.2), "k", trait=0)
    t.p(R(21, 14, 27, 17), "k", trait=0)
    t.p(E(22.2, 11.5, 1.3, 1.5) | E(25.8, 11.5, 1.3, 1.5), "o", trait=0, ton=2)
    t.px([(22, 11), (26, 11)], "y", 4)
    t.px([(22, 16), (24, 16), (26, 16)], "o"); t.px((24, 14), "o")
    t.p(P([(19, 9), (20, 2), (24, -3), (28, 2), (29, 9)]), "r", trait=0, dl=1)
    t.p(R(19, 8, 29, 9) | R(24, -1, 24, 8), "g", trait=0)
    # cotes qui percent l'etoffe
    t.px([(19, 22), (20, 23), (28, 23), (29, 22), (19, 31), (29, 31)], "k", 4)


@sprite("destrier-spectral", "Destrier spectral")
def destrier(t):
    t.mat("h", SPECTRE, FEU_VIOLET)
    t.mat("a", ACIER)                # chanfrein et caparacon
    t.mat("c", "4a2a5a", "2a4a5a")   # housse en lambeaux
    t.mat("y", "f8f8ff", "f8e040")
    # galop : anterieurs lances, posterieurs replies
    t.p(K([(17, 30, 2), (11, 33, 1.4), (7, 38, 1.2)]), "h", dl=-1, trait=0)
    t.p(K([(36, 30, 2.4), (41, 35, 1.5), (45, 32, 1.2)]), "h", dl=-1, trait=0)
    # queue de brume
    t.p(S([(39, 23, 2), (44, 21, 2.6), (47, 26, 1.8), (45, 31, 0.8)]), "h", trait=0, ton=1)
    t.p(B([(13, 23), (20, 19), (31, 19), (40, 23), (40, 30), (33, 33), (21, 33), (14, 31)]), "h", trait=0)
    # cotes visibles
    for x in (22, 25, 28, 31):
        t.p(L(x, 23, x - 1, 31), "h", trait=0, ton=0, sur="h")
    # housse dechiree
    t.p(B([(20, 19), (32, 19), (34, 28), (31, 25), (28, 30), (25, 26), (22, 30), (19, 25)]), "c", trait=0)
    t.p(K([(16, 31, 2.2), (10, 36, 1.5), (5, 41, 1.2)]), "h")
    t.p(K([(34, 31, 2.6), (38, 38, 1.5), (43, 41, 1.2)]), "h")
    t.px([(3, 42), (4, 42), (44, 42), (45, 42), (6, 39), (46, 31)], "y", 3)
    # encolure tendue, chanfrein de fer, criniere de flammes pales
    t.p(B([(13, 27), (9, 17), (12, 11), (17, 13), (20, 22)]), "h", trait=0)
    t.p(B([(2, 15), (5, 9), (11, 8), (14, 12), (10, 17), (4, 18)]), "h", trait=0)
    t.p(B([(3, 12), (6, 8), (11, 8), (12, 11), (6, 13)]), "a", trait=0)
    t.p(P([(10, 8), (12, 3), (13, 9)]), "h", trait=0)
    t.px([(7, 11), (8, 11)], "y", 4)
    for x, y, dx, dy in ((14, 11, 3, -4), (16, 14, 4, -3), (18, 18, 4, -3), (19, 21, 4, -2)):
        t.p(P([(x, y), (x + dx, y + dy), (x + 2, y + 2)]), "h", trait=0, ton=4)
    t.px([(2, 18), (1, 20), (3, 21)], "h", 4)


@sprite("vautour-des-charniers", "Vautour des charniers")
def vautour(t):
    t.mat("p", "4a3e36", "8a8a98")   # plumes brunes
    t.mat("c", ["7a3a3a", "a85c58", "d0847c", "ecb0a4", "fcd8cc"], ["3a4a7a", "5c70a8", "8498d0", "b0c0ec", "d8e0fc"])   # cou nu
    t.mat("w", "e8e0d0", "3a3a44")   # collerette
    t.mat("b", "3a3a40", "c8b060")
    t.mat("k", OS)
    t.mat("y", "f8e040", "f04848")
    # perche sur une cage thoracique
    for x in (14, 19, 24, 29, 34):
        t.p(CS([(x, 45), (x - 1, 39), (x + 2, 36)]), "k", trait=0)
    t.p(R(10, 44, 38, 45), "k", trait=0, dl=-1)
    # corps masse, ailes en cape
    t.p(B([(16, 20), (23, 15), (32, 17), (38, 25), (40, 36), (32, 38), (22, 38), (16, 32)]), "p", trait=0)
    t.p(B([(24, 18), (33, 20), (39, 28), (42, 40), (34, 38), (27, 30)]), "p", trait=2, dl=-1)
    for a, b in (((29, 24), (36, 34)), ((32, 23), (39, 33)), ((26, 26), (32, 36))):
        t.p(L(a[0], a[1], b[0], b[1]), "p", trait=0, ton=0)
    t.p(K([(21, 37, 1.5), (20, 42, 1)]) | K([(27, 37, 1.5), (27, 42, 1)]), "b", trait=0)
    t.px([(18, 43), (19, 43), (21, 43), (22, 43), (25, 43), (26, 43), (28, 43), (29, 43)], "b", 3)
    # collerette, cou nu en S, tete chauve
    t.p(E(18, 21, 5, 3.6), "w", trait=0)
    t.p(S([(17, 20, 2.2), (13, 15, 1.9), (14, 9, 1.9)]), "c", trait=0)
    t.p(E(13, 7.5, 3.6, 3.2), "c", trait=0)
    t.p(B([(4, 8), (9, 5), (11, 9), (8, 11), (6, 14)]), "b", trait=0)
    t.px((6, 13), "b", 4)
    t.px([(12, 6), (13, 6)], "y", 3); t.px((12, 6), "o"); t.px([(11, 5), (12, 5), (13, 5), (14, 4)], "o")
    t.px([(13, 13), (15, 16), (12, 17)], "c", 0)
    t.px([(20, 26), (23, 30), (19, 31)], "p", 3)


@sprite("matin-de-guerre", "Mâtin de guerre")
def matin(t):
    t.mat("f", "8a7a68", "3a3a44")   # poil gris-fauve
    t.mat("a", ACIER)                # barde de plates
    t.mat("r", "8a1a24", "1a4a8a")   # tapis d'armes
    t.mat("w", "f4f0e6")
    t.mat("y", "f8c040", "40e0f0")
    t.p(K([(20, 31, 2.4), (20, 38, 1.7), (19, 45, 1.6)]), "f", dl=-1, trait=0)
    t.p(K([(36, 30, 2.8), (39, 37, 1.8), (38, 45, 1.6)]), "f", dl=-1, trait=0)
    t.p(S([(40, 22, 1.5), (44, 20, 1.2), (46, 15, 0.9)]), "f", trait=0)
    t.p(B([(12, 22), (19, 18), (31, 19), (40, 22), (41, 29), (34, 32), (22, 33), (13, 32)]), "f", trait=0)
    # barde : dossiere articulee, tapis rouge dessous
    t.p(B([(18, 18), (31, 18), (38, 22), (38, 30), (32, 32), (20, 32), (18, 26)]), "r", trait=0)
    t.p(B([(17, 17), (31, 17), (39, 21), (38, 27), (30, 25), (19, 25)]), "a", trait=0)
    for x in (22, 27, 32):
        t.p(L(x, 18, x, 25), "a", trait=0, ton=1, sur="a")
    t.px([(20, 19), (25, 19), (30, 19), (35, 21)], "a", 4)
    for x in (21, 26, 31, 36):
        t.p(P([(x - 1, 17), (x, 13), (x + 1, 17)]), "a", trait=0, ton=4)
    t.p(K([(16, 31, 2.8), (15, 38, 1.9), (14, 45, 1.8)]), "f")
    t.p(K([(34, 31, 3), (35, 38, 1.9), (33, 45, 1.8)]), "f")
    t.px([(12, 46), (13, 46), (31, 46), (32, 46)], "w", 2)
    # tete : casque a nasal, machoire forte
    t.p(B([(3, 17), (7, 12), (14, 12), (18, 16), (17, 24), (10, 26), (4, 24)]), "f", trait=0)
    t.p(B([(5, 14), (8, 10), (15, 10), (18, 15), (12, 16), (7, 17)]), "a", trait=0)
    t.p(R(6, 16, 7, 20), "a", trait=0, ton=3)
    t.p(P([(11, 10), (12, 5), (14, 10)]), "a", trait=0, ton=4)
    t.px([(9, 17), (10, 17)], "y", 3); t.px((9, 17), "o")
    t.px([(4, 23), (5, 24), (6, 24), (7, 24), (8, 24), (9, 23), (10, 23)], "o")
    t.px([(5, 25), (7, 25), (9, 24)], "w", 4)
    t.px((3, 19), "o")


@sprite("griffon-cuirasse", "Griffon cuirassé")
def griffon(t):
    t.mat("f", "b88a48", "8a8a9a")   # pelage de lion
    t.mat("p", "e8dcc4", "4a4a56")   # plumes de la tete
    t.mat("a", ACIER)
    t.mat("b", "e8b030", "c8c8d8")
    t.mat("y", "f04830", "40e0f0")
    t.mat("m", "8a6a3a", "6a6a7a")   # ailes
    # aile dressee
    for i, (x, y) in enumerate(((26, 1), (33, 2), (39, 6), (43, 12), (44, 19))):
        t.p(C(26, 22, x, y, 3, 1.3), "m", trait=0, dl=-(i % 2))
    t.p(E(27, 19, 4.4, 3.4), "a", trait=0)
    # arriere-train de lion
    t.p(S([(40, 30, 1.5), (45, 28, 1.2), (46, 22, 1)]), "f", trait=0)
    t.p(E(46, 20, 1.8, 2.2), "m", trait=0, dl=-1)
    t.p(K([(36, 31, 2.8), (39, 38, 1.8), (37, 45, 1.6)]), "f", dl=-1, trait=0)
    t.p(K([(20, 31, 1.6), (20, 38, 1.2), (18, 44, 1)]), "b", dl=-1, trait=0)
    t.p(B([(13, 24), (20, 20), (32, 21), (40, 25), (40, 32), (32, 34), (20, 34), (14, 32)]), "f", trait=0)
    # barde de plates sur le poitrail et le dos
    t.p(B([(12, 22), (19, 19), (30, 20), (31, 26), (22, 28), (14, 32), (11, 27)]), "a", trait=0)
    t.px([(15, 22), (20, 21), (26, 22), (14, 27), (19, 26)], "a", 4)
    t.p(L(18, 20, 16, 30) | L(24, 20, 23, 27), "a", trait=0, ton=1, sur="a")
    t.p(K([(34, 32, 3), (35, 39, 1.9), (33, 45, 1.7)]), "f")
    # pattes de devant : des serres d'aigle
    t.p(K([(16, 31, 1.8), (15, 38, 1.3), (14, 43, 1.1)]), "b", trait=1)
    t.px([(11, 45), (12, 44), (13, 45), (15, 44), (16, 45), (16, 45), (20, 45), (18, 45)], "b", 3)
    # tete d'aigle, heaume a cimier
    t.p(S([(15, 24, 3.6), (13, 17, 3), (13, 12, 2.8)]), "p", trait=0)
    t.p(E(12, 10, 4.4, 4.2), "p", trait=0)
    t.p(B([(8, 8), (11, 4), (16, 5), (17, 10), (12, 9)]), "a", trait=0)
    t.p(P([(12, 5), (15, -1), (17, 5)]), "y", trait=0, ton=2)
    t.p(B([(2, 11), (6, 8), (9, 9), (9, 13), (5, 14), (3, 16)]), "b", trait=0)
    t.px((3, 15), "b", 4)
    t.px([(10, 10), (11, 10)], "y", 3); t.px((10, 10), "o")
    t.px([(14, 16), (12, 19), (15, 20)], "p", 1)
