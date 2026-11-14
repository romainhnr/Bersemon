from dessin import *
from matieres import *
from atelier import sprite
from commun import _aile_membrane
import math

LUEUR = ["a87a10", "e0b020", "f8dc50", "fff4a0", "ffffe8"]


@sprite("gardien-d-asphodele", "Gardien d'Asphodèle")
def gardien_d_asphodele(t):
    t.mat("e", ["8a7a48", "c0b070", "e8dca0", "f8f0cc", "fffcf0"], ["485a8a", "7088c0", "a0b8e8", "ccdcf8", "f0f6ff"])   # ecailles d'ivoire
    t.mat("g", OR)
    t.mat("y", LUEUR, FEU_BLEU)
    t.mat("f", ["8a8a90", "b8b8be", "dcdce2", "f2f2f6", "ffffff"])   # asphodeles
    t.mat("v", "4f9a3c", "3a9aa8")
    # ailes de plumes blanches
    for i, (x, y) in enumerate(((28, 0), (35, 2), (41, 6), (45, 12), (46, 19))):
        t.p(C(27, 22, x, y, 3.2, 1.3), "f", trait=0, dl=-(i % 2))
    # queue
    t.p(S([(36, 32, 3.4), (42, 36, 2.6), (45, 42, 1.6), (41, 46, 0.8)]), "e", trait=0)
    t.p(K([(32, 34, 2.8), (35, 40, 1.9), (33, 45, 1.7)]), "e", dl=-1, trait=0)
    t.p(K([(19, 34, 2.4), (20, 40, 1.7), (18, 45, 1.5)]), "e", dl=-1, trait=0)
    # corps de dragon couche en lion, collier d'or
    t.p(B([(12, 27), (19, 22), (31, 22), (39, 27), (39, 35), (31, 38), (19, 38), (12, 34)]), "e", trait=0)
    for x in (18, 23, 28, 33):
        t.p(CS([(x, 24), (x + 2, 30), (x, 37)]), "e", trait=0, ton=1, sur="e")
    t.p(K([(16, 34, 2.8), (14, 40, 1.9), (12, 45, 1.7)]), "e")
    t.p(K([(30, 35, 3.2), (30, 41, 2), (28, 45, 1.7)]), "e")
    t.px([(9, 46), (10, 46), (12, 46), (25, 46), (26, 46), (28, 46)], "g", 3)
    # cou dresse, tete couronnee de cornes d'or
    t.p(S([(15, 27, 3.6), (12, 20, 3), (12, 13, 2.8)]), "e", trait=0)
    t.p(C(11, 24, 18, 26, 1.4), "g", trait=0)
    t.p(B([(1, 13), (5, 7), (13, 6), (17, 10), (15, 15), (8, 17), (2, 16)]), "e", trait=0)
    t.p(CS([(13, 7), (18, 4), (20, -1)]) | CS([(10, 6), (13, 1), (12, -3)]), "g", trait=0)
    t.px([(7, 10), (8, 10)], "y", 3); t.px((7, 10), "y", 4); t.px([(6, 9), (7, 9), (8, 9), (9, 8)], "o")
    t.px([(2, 15), (3, 15), (4, 15), (5, 15), (6, 15)], "o")
    # un champ d'asphodeles a ses pieds
    for x in (4, 22, 38, 44):
        t.p(R(x, 41, x, 46), "v", trait=0)
        t.px([(x, 40), (x - 1, 41), (x + 1, 41), (x, 39)], "f", 4)


@sprite("ours-des-glaces", "Ours des glaces")
def ours_des_glaces(t):
    t.mat("f", ["8a94a8", "b4bccc", "d8dee8", "f0f3f8", "ffffff"], ["a8948a", "ccbcb4", "e8ded8", "f8f3f0", "ffffff"])   # fourrure blanche
    t.mat("g", GLACE, FEU_VIOLET)
    t.mat("s", SPECTRE, FEU)
    t.mat("n", "2a2a34")
    # dresse sur ses pattes arriere
    t.p(K([(29, 36, 4.2), (32, 42, 3.4), (30, 45, 3.2)]), "f", dl=-1, trait=0)
    t.p(B([(14, 18), (23, 12), (33, 15), (37, 26), (36, 36), (29, 41), (19, 41), (14, 32)]), "f", trait=0)
    t.p(K([(20, 38, 4.4), (18, 43, 3.6), (16, 45, 3.4)]), "f")
    t.px([(12, 46), (14, 46), (16, 46), (27, 46), (29, 46), (31, 46)], "n", 2)
    # pattes avant levees, griffes de glace
    t.p(K([(17, 20, 3.4), (10, 22, 3), (5, 17, 2.8)]), "f", trait=1)
    t.p(K([(32, 19, 3.4), (39, 22, 3), (43, 17, 2.8)]), "f", trait=0, dl=-1)
    for x in (3, 5, 7, 41, 43, 45):
        t.p(P([(x - 1, 14), (x, 9), (x + 1, 14)]), "g", trait=0, ton=4)
    # flanc a jour : on voit a travers, des cotes de glace
    t.p(E(27, 29, 5, 6), "s", trait=0, ton=1)
    for y in (25, 28, 31):
        t.p(R(23, y, 31, y), "g", trait=0, ton=3)
    t.p(R(27, 23, 27, 34), "g", trait=0, ton=4)
    # tete : gueule ouverte, souffle de givre
    t.p(E(27, 8, 2.2, 2.2), "f", trait=0, dl=-1)
    t.p(B([(13, 12), (16, 6), (23, 4), (29, 8), (29, 15), (22, 18), (15, 17)]), "f", trait=0)
    t.p(E(21, 5, 2.2, 2.2), "f", trait=0)
    t.p(B([(9, 13), (14, 11), (17, 14), (16, 19), (11, 19)]), "f", trait=0, dl=-1)
    t.px([(9, 13), (10, 13), (9, 14)], "n", 2)
    t.px([(11, 17), (12, 18), (13, 18), (14, 18)], "o")
    t.px([(12, 19), (14, 19)], "g", 4)
    t.px([(19, 10), (20, 10)], "s", 4); t.px((19, 10), "o")
    for x, y in ((6, 20), (4, 23), (7, 25), (2, 26)):
        t.px((x, y), "g", 4)
    t.px([(18, 24), (20, 30), (33, 22), (16, 34)], "f", 1)


@sprite("yeti", "Yéti")
def yeti(t):
    t.mat("f", ["7a8494", "a4acb8", "ccd2da", "e8ecf0", "ffffff"], ["947a84", "b8a4ac", "daccd2", "f0e8ec", "ffffff"])
    t.mat("c", ["2a3a5a", "48608a", "7088b0", "a0b4d4", "d0dcf0"], ["5a2a3a", "8a4860", "b07088", "d4a0b4", "f0d0dc"])   # face et mains bleues
    t.mat("g", GLACE)
    t.mat("w", OS)
    t.mat("y", "f8e040", "f04848")
    # jambes courtes, grands pieds
    t.p(K([(29, 34, 4), (31, 41, 3.4)]), "f", trait=0, dl=-1)
    t.p(K([(19, 34, 4), (17, 41, 3.4)]), "f", trait=0)
    t.p(R(11, 43, 20, 45) | R(28, 43, 37, 45), "c", trait=0)
    # torse en poire, poils qui pendent
    t.p(B([(11, 18), (17, 9), (31, 9), (37, 18), (38, 30), (33, 38), (15, 38), (10, 30)]), "f", trait=0)
    for x in (13, 17, 21, 25, 29, 33):
        t.p(P([(x, 36), (x + 1, 40), (x + 3, 36)]), "f", trait=0)
    t.px([(15, 22), (19, 28), (24, 24), (29, 30), (33, 24), (22, 33)], "f", 1)
    # bras interminables, poings au sol ; l'un brandit un bloc de glace
    t.p(K([(12, 19, 3.6), (7, 28, 3.2), (6, 37, 3)]), "f", trait=1)
    t.p(E(6, 40, 3.2, 2.8), "c", trait=0)
    t.p(K([(36, 19, 3.6), (41, 12, 3.2), (40, 5, 3)]), "f", trait=1, dl=-1)
    t.p(P([(34, 2), (38, -4), (45, -2), (46, 4), (41, 7), (35, 6)]), "g", trait=1)
    t.px([(37, -1), (39, -2), (36, 2)], "g", 4)
    # face bleue dans la fourrure, crocs du bas
    t.p(B([(18, 14), (20, 8), (28, 8), (30, 14), (27, 19), (21, 19)]), "c", trait=0)
    t.px([(21, 12), (22, 12), (26, 12), (27, 12)], "y", 3); t.px([(21, 12), (26, 12)], "o")
    t.px([(20, 10), (21, 10), (22, 11), (26, 11), (27, 10), (28, 10)], "o")
    t.px([(21, 16), (22, 17), (23, 17), (24, 17), (25, 17), (26, 17), (27, 16)], "o")
    t.px([(22, 16), (26, 16)], "w", 4)
    t.p(P([(19, 9), (24, 3), (29, 9)]), "f", trait=0)


@sprite("esprit-du-blizzard", "Esprit du blizzard")
def esprit_du_blizzard(t):
    t.mat("g", ["4a6a9a", "7a9ac8", "a8c8e8", "d4e8f8", "f6fcff"], ["9a4a7a", "c87aa8", "e8a8d0", "f8d4ec", "fff6fc"])   # vent glace
    t.mat("w", ["a8c0d8", "c8dcec", "e4f0f8", "f4fafe", "ffffff"])
    t.mat("k", "1a2a4a")
    # tourbillon : des rubans de vent qui s'enroulent vers le bas
    for i, (r, y) in enumerate(((17, 14), (14, 22), (10, 30), (6, 37), (3, 43))):
        t.p(E(24, y, r, 4.6), "g", trait=0, dl=-(i % 2))
        t.p(CS([(24 - r, y), (24 - r / 2, y + 3), (24 + r / 2, y + 3), (24 + r, y)]), "w", trait=0, ton=4)
    t.p(E(24, 46, 1.6, 1.2), "g", trait=0, ton=1)
    # tete de nuee, bouche qui souffle
    t.p(B([(12, 12), (16, 4), (24, 1), (32, 4), (36, 12), (32, 18), (16, 18)]), "w", trait=0)
    t.p(P([(17, 9), (22, 11), (18, 13)]) | P([(31, 9), (26, 11), (30, 13)]), "k", trait=0, ton=2)
    t.px([(19, 11), (29, 11)], "g", 4)
    t.p(E(24, 15, 2.2, 1.6), "k", trait=0, ton=2)
    # bras de vent
    t.p(S([(12, 16, 2.4), (5, 20, 1.8), (2, 27, 1)]), "g", trait=0)
    t.p(S([(36, 16, 2.4), (43, 20, 1.8), (46, 27, 1)]), "g", trait=0, dl=-1)
    # cristaux de neige emportes
    for x, y in ((4, 8), (44, 6), (40, 34), (8, 36), (2, 44), (46, 42)):
        t.px([(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)], "w", 4)
    for x, y in ((10, 2), (38, 0), (36, 44), (12, 44)):
        t.px((x, y), "w", 3)


@sprite("mammouth-fantome", "Mammouth fantôme")
def mammouth(t):
    t.mat("s", ["3a5a7a", "5c88a8", "8cb8d0", "bcdcec", "ecf8fc"], ["7a3a5a", "a85c88", "d08cb8", "ecbcdc", "fcecf8"])   # spectre de glace
    t.mat("k", OS)
    t.mat("y", "f8f8ff", "f8e040")
    t.p(K([(23, 32, 3.4), (23, 40, 2.8), (23, 44, 2.8)]), "s", dl=-1, trait=0)
    t.p(K([(41, 32, 3.4), (42, 40, 2.8), (42, 44, 2.8)]), "s", dl=-1, trait=0)
    # corps en dome, laine qui pend en glacons
    t.p(B([(14, 20), (20, 9), (32, 8), (42, 14), (46, 24), (45, 34), (38, 37), (20, 37), (14, 32)]), "s", trait=0)
    for x in range(18, 44, 4):
        t.p(P([(x, 36), (x + 1, 41), (x + 3, 36)]), "s", trait=0, ton=3)
    # le squelette se voit a travers
    for x in (26, 30, 34, 38):
        t.p(CS([(x, 16), (x + 2, 24), (x, 32)]), "k", trait=0, ton=3)
    t.p(L(22, 15, 42, 17), "k", trait=0, ton=4)
    t.p(K([(18, 33, 3.6), (17, 40, 2.8), (17, 44, 2.8)]), "s")
    t.p(K([(36, 33, 3.8), (36, 40, 2.8), (36, 44, 2.8)]), "s")
    t.px([(14, 46), (16, 46), (18, 46), (33, 46), (35, 46), (37, 46)], "k", 3)
    # tete bombee, trompe, grandes defenses recourbees
    t.p(B([(5, 18), (8, 9), (16, 6), (22, 11), (22, 22), (15, 27), (8, 25)]), "s", trait=0)
    t.p(S([(8, 23, 2.6), (5, 31, 2.2), (6, 38, 1.8), (10, 41, 1.4)]), "s", trait=1)
    t.p(CS([(10, 25), (4, 34), (2, 42), (8, 46), (14, 42)]), "k", trait=0, ton=4)
    t.p(CS([(14, 26), (10, 35), (10, 43), (16, 46), (20, 41)]), "k", trait=0, ton=3)
    t.px([(12, 15), (13, 15)], "y", 3); t.px((12, 15), "y", 4)
    t.p(E(20, 14, 2.6, 4), "s", trait=0, dl=-1)
    t.px([(44, 8), (2, 12), (46, 42), (28, 2)], "s", 4)


@sprite("colosse-d-os", "Colosse d'os")
def colosse_d_os(t):
    t.mat("k", OS)
    t.mat("t", ["3a2a1c", "5a4430", "7e6448", "a48a68", "ccb494"])   # terre
    t.mat("y", SPECTRE, FEU_VIOLET)
    # jambes : des femurs de geant lies ensemble
    for x, dl in ((29, -1), (19, 0)):
        t.p(K([(x, 33, 2.6), (x + 1, 43, 2.4)]), "k", trait=0, dl=dl)
        t.p(E(x - 1, 33, 2.2, 2) | E(x + 2, 33, 2.2, 2) | E(x, 44, 3.4, 2), "k", trait=0, dl=dl)
    # cage thoracique enorme, une lueur dedans
    t.p(E(24, 22, 12, 11), "t", trait=0, ton=0)
    t.p(E(24, 23, 4, 4.4), "y", trait=0, ton=2); t.p(E(24, 23, 2, 2.2), "y", trait=0, ton=4)
    for y, r in ((14, 9), (18, 11.5), (22, 12), (26, 11.5), (30, 9.5)):
        t.p(E(24, y, r, 1.3), "k", trait=0)
    t.p(R(23, 11, 25, 33), "k", trait=0, ton=3)
    t.p(E(24, 33, 7, 2.6), "k", trait=0, dl=-1)
    # epaules : des cranes ; bras : des colonnes vertebrales, poings en tas d'os
    for x, s in ((11, 1), (37, -1)):
        t.p(E(x, 13, 3.6, 3.4), "k", trait=0)
        t.px([(x - 1, 13), (x + 1, 13)], "o"); t.px([(x - 1, 15), (x, 15), (x + 1, 15)], "o")
        for y in range(18, 34, 3):
            t.p(E(x - s * (y - 18) * 0.15, y, 2, 1.4), "k", trait=0, dl=-(y % 2))
        t.p(E(x - s * 3, 37, 4, 3.6), "k", trait=0)
        t.px([(x - s * 4, 36), (x - s * 2, 38), (x - s * 3, 39)], "k", 0)
    # tete : un crane de geant sans machoire, cornu
    t.p(E(24, 6, 5.6, 5.2), "k", trait=0)
    t.p(E(21.5, 6, 1.6, 1.8) | E(26.5, 6, 1.6, 1.8), "o", trait=0, ton=2)
    t.px([(21, 6), (26, 6)], "y", 4)
    t.px((24, 9), "o")
    for x in (21, 23, 25, 27):
        t.p(R(x, 11, x, 12), "k", trait=0, ton=4)
    t.px([(12, 44), (14, 45), (34, 45), (36, 44)], "t", 2)


@sprite("charognard-des-geants", "Charognard des géants")
def charognard(t):
    t.mat("p", ["24202a", "3e3848", "5c546a", "84789a", "b4a8c8"], ["2a2420", "483e38", "6a5c54", "9a8478", "c8b4a8"])   # plumes d'ardoise
    t.mat("c", ["4a5a2a", "748a40", "9cb45c", "c4d88c", "e8f4c4"], ["5a2a4a", "8a4074", "b45c9c", "d88cc4", "f4c4e8"])   # peau verdatre
    t.mat("k", OS)
    t.mat("b", "3a3a40", "c8b060")
    t.mat("y", FEU_VERT, FEU_VIOLET)
    # un crane de geant pour perchoir
    t.p(E(24, 39, 16, 8), "k", trait=0, dl=-1)
    t.p(E(16, 39, 3.4, 3) | E(30, 39, 3.4, 3), "o", trait=0, ton=2)
    t.p(P([(22, 41), (23, 45), (24, 41)]), "o", trait=0, ton=2)
    t.px([(12, 33), (18, 32), (34, 34)], "k", 4)
    # ailes en voute au-dessus de lui
    for (bx, by), pointes, dl in (((28, 20), ((38, 6), (44, 10), (46, 17), (44, 24)), -1), ((20, 20), ((10, 6), (4, 10), (2, 17), (4, 24)), 0)):
        for i, (x, y) in enumerate(pointes):
            t.p(C(bx, by, x, y, 3.4, 1.3), "p", trait=0, dl=dl - (i % 2))
    # corps voute
    t.p(B([(17, 18), (24, 14), (31, 18), (32, 27), (28, 33), (20, 33), (16, 27)]), "p", trait=0)
    t.px([(20, 22), (24, 25), (28, 22), (22, 29), (26, 29)], "p", 0)
    t.px([(19, 34), (20, 35), (21, 34), (27, 34), (28, 35), (29, 34)], "b", 3)
    # cou nu a goitre, tete chauve, bec qui degoutte de poison
    t.p(E(24, 18, 4.4, 3), "k", trait=0, dl=-1)
    t.p(S([(24, 17, 2.2), (22, 11, 1.9), (24, 6, 1.9)]), "c", trait=0)
    t.p(E(21.5, 13, 2.2, 2.4), "c", trait=0, dl=1)
    t.p(E(24.5, 5, 3.4, 3), "c", trait=0)
    t.p(B([(16, 6), (21, 3), (22, 7), (19, 9), (17, 12)]), "b", trait=0)
    t.px((17, 11), "b", 4)
    t.px([(24, 4), (25, 4)], "y", 3); t.px((24, 4), "o")
    t.px([(17, 14), (17, 17), (16, 20)], "y", 3)
