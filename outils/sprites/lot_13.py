from dessin import *
from matieres import *
from atelier import sprite
import math

ROSE = ["5a0a1a", "96142a", "d02c44", "f06a78", "ffd0d4"]


@sprite("ronce-vampire", "Ronce vampire")
def ronce_vampire(t):
    t.mat("v", "3a5a34", "5a3a5a")   # tiges
    t.mat("r", ROSE, FEU_VIOLET)     # fleur-gueule
    t.mat("w", OS)
    t.mat("s", SANG, FEU_VERT)
    # ronces qui rampent au sol
    for pts in ([(24, 42, 2.2), (14, 43, 1.8), (6, 40, 1.4), (2, 34, 0.8)], [(24, 42, 2.2), (34, 43, 1.8), (42, 40, 1.4), (46, 33, 0.8)],
                [(22, 40, 1.8), (12, 34, 1.4), (8, 26, 0.9)], [(26, 40, 1.8), (36, 34, 1.4), (41, 25, 0.9)]):
        t.p(S(pts), "v", trait=0, dl=-1)
    # tige maitresse en S
    t.p(S([(24, 43, 3), (27, 34, 2.6), (21, 26, 2.4), (24, 18, 2.2)]), "v", trait=0)
    # epines
    for x, y, dx in ((28, 37, 3), (20, 30, -3), (26, 23, 3), (9, 30, -2), (39, 30, 2), (5, 37, -2), (43, 37, 2), (16, 39, 0),
                     (33, 39, 0)):
        t.p(P([(x, y - 1), (x + dx, y - 3), (x, y + 1)]), "v", trait=0, ton=4)
    # la fleur : une gueule de petales, crocs en couronne
    for a in range(0, 360, 60):
        x, y = 24 + 8 * math.cos(math.radians(a)), 12 + 7 * math.sin(math.radians(a))
        t.p(E(x, y, 4.6, 4), "r", trait=0, dl=-(a // 60 % 2))
    t.p(E(24, 12, 5, 4.4), "s", trait=0, ton=0)
    t.p(E(24, 12, 2.4, 2), "o", trait=0, ton=2)
    for a in range(0, 360, 40):
        t.px((24 + 4.2 * math.cos(math.radians(a)), 12 + 3.6 * math.sin(math.radians(a))), "w", 4)
    # sang qui perle
    t.px([(22, 18), (22, 20), (26, 19), (24, 22), (11, 28), (38, 27)], "s", 2)
    t.px([(22, 21), (24, 23)], "s", 3)
    t.p(B([(18, 30), (12, 27), (10, 30), (15, 33)]) | B([(29, 27), (35, 24), (37, 27), (32, 30)]), "v", trait=0, dl=1)


@sprite("mante-ecarlate", "Mante écarlate")
def mante(t):
    t.mat("c", ["5a0c14", "961c20", "d03a30", "f07a60", "fcc4a8"], ["0c2a5a", "1c4a96", "3a78d0", "7aa8f0", "c4dcfc"])
    t.mat("a", "e8a070", "a0c8f0")   # ailes
    t.mat("y", "f8f0a0", "f8f0a0")
    # ailes en etui sur le dos
    t.p(B([(22, 22), (32, 22), (44, 30), (46, 36), (34, 34), (24, 28)]), "a", trait=0)
    t.px([(28, 25), (33, 27), (38, 30), (42, 33)], "a", 0)
    # abdomen long
    t.p(B([(24, 26), (34, 28), (43, 34), (40, 38), (30, 36), (23, 32)]), "c", trait=0)
    for x in (28, 32, 36, 40):
        t.p(L(x, 28, x - 1, 37), "c", trait=0, ton=0, sur="c")
    # quatre pattes greles
    for (x0, x1, x2), dl in (((27, 31, 30), -1), ((33, 39, 40), -1), ((25, 26, 23), 0), ((31, 35, 34), 0)):
        t.p(K([(x0, 32, 1.2), (x1, 39, 0.9), (x2, 46, 0.7)]), "c", trait=0, dl=dl)
    # thorax dresse
    t.p(S([(25, 30, 3), (21, 22, 2.4), (20, 14, 2.2)]), "c", trait=0)
    # pattes ravisseuses, en faux dentelees
    for dx, dl in ((0, 0), (4, -1)):
        t.p(K([(21 + dx, 20, 1.6), (13 + dx, 24, 1.5), (9 + dx, 16, 1.3)]), "c", trait=1 if dx == 0 else 0, dl=dl)
        t.p(P([(8 + dx, 17), (10 + dx, 5), (12 + dx, 16)]), "c", trait=0, dl=dl + 1)
        t.px([(11 + dx, 8), (11 + dx, 11), (11 + dx, 14)], "y", 3)
    # tete triangulaire, gros yeux, antennes
    t.p(P([(15, 9), (24, 8), (20, 16)]), "c", trait=0)
    t.p(E(16, 10, 2, 2.2) | E(23, 9.5, 2, 2.2), "y", trait=0, ton=3)
    t.px([(16, 10), (23, 10)], "o")
    t.p(CS([(18, 8), (17, 3), (13, 1)]) | CS([(21, 8), (23, 3), (27, 1)]), "c", trait=0)
    t.px([(19, 14), (20, 15)], "o")


@sprite("sanglier-enrage", "Sanglier enragé")
def sanglier(t):
    t.mat("f", "5a4638", "4a4a56")   # soies
    t.mat("n", "3a2c26", "2e2e3a")
    t.mat("w", OS)
    t.mat("y", SANG, FEU_VERT)
    t.mat("r", "c88a8a", "a8b8c8")   # groin
    t.mat("b", "e8e4dc")             # bave
    t.p(K([(21, 33, 2.4), (21, 39, 1.6), (20, 45, 1.4)]), "n", dl=0, trait=0)
    t.p(K([(37, 32, 2.6), (39, 39, 1.6), (38, 45, 1.4)]), "n", dl=0, trait=0)
    t.p(CS([(42, 24), (45, 22), (44, 19), (46, 17)]), "f", trait=0)
    # corps en tonneau, garrot haut
    t.p(B([(12, 22), (18, 14), (28, 14), (39, 19), (43, 26), (41, 33), (32, 36), (20, 36), (13, 32)]), "f", trait=0)
    # criniere herissee
    for x in range(15, 38, 3):
        y = 15 + abs(x - 22) // 4
        t.p(P([(x - 1, y + 1), (x + 1, y - 4), (x + 2, y + 1)]), "n", trait=0)
    t.p(K([(17, 33, 2.8), (16, 39, 1.8), (15, 45, 1.5)]), "f")
    t.p(K([(34, 33, 3), (35, 39, 1.8), (33, 45, 1.5)]), "f")
    t.px([(14, 46), (15, 46), (32, 46), (33, 46), (19, 46), (37, 46)], "o")
    # hure basse, groin, defenses
    t.p(B([(2, 28), (5, 21), (12, 18), (18, 22), (18, 31), (11, 35), (4, 34)]), "f", trait=0)
    t.p(E(3.5, 30, 2.6, 3), "r", trait=0)
    t.px([(3, 29), (3, 31)], "o")
    t.p(P([(13, 18), (16, 13), (17, 20)]), "n", trait=0)
    t.p(CS([(7, 33), (6, 28), (9, 25)]) | CS([(10, 34), (10, 29), (13, 27)]), "w", trait=0, ton=4)
    t.px([(10, 23), (11, 23)], "y", 3); t.px((10, 23), "y", 4); t.px([(9, 22), (10, 22), (11, 22), (12, 21)], "o")
    t.px([(5, 35), (5, 37), (7, 36), (4, 39)], "b", 3)
    t.px([(24, 24), (28, 27), (33, 24), (22, 30), (36, 30)], "f", 0)


@sprite("dryade-sanglante", "Dryade sanglante")
def dryade_sanglante(t):
    t.mat("b", "7a5a44", "6a6a80")   # ecorce
    t.mat("l", ["5a0a14", "961824", "c8303a", "e86a60", "fcb8a8"], ["0a5a4a", "189680", "30c8b0", "6ae8d0", "b8fcf0"])   # feuillage rouge
    t.mat("c", ["5a4a3a", "8a7458", "b89c78", "dcc49c", "f6e4c4"])
    t.mat("s", SANG, FEU_VERT)
    t.mat("y", FEU_VIOLET, OR)
    # jambes-racines
    for pts in ([(22, 36, 2.6), (18, 42, 1.8), (12, 45, 0.9)], [(26, 36, 2.6), (30, 42, 1.8), (36, 45, 0.9)],
                [(24, 38, 2), (24, 45, 1)]):
        t.p(S(pts), "b", trait=0, dl=-1)
    # corps de femme pris dans l'ecorce
    t.p(B([(19, 20), (24, 18), (29, 20), (31, 30), (28, 38), (20, 38), (17, 30)]), "b", trait=0)
    t.p(B([(20, 20), (24, 19), (28, 20), (27, 27), (24, 29), (21, 27)]), "c", trait=0)
    t.px([(19, 31), (22, 34), (27, 33), (29, 30)], "b", 0)
    # bras-branches leves
    t.p(S([(20, 21, 1.8), (13, 17, 1.5), (9, 10, 1.1)]), "b", trait=1)
    t.p(S([(28, 21, 1.8), (35, 17, 1.5), (39, 10, 1.1)]), "b", trait=0, dl=-1)
    t.p(S([(11, 13, 0.8), (6, 13, 0.6)]) | S([(37, 13, 0.8), (42, 13, 0.6)]), "b", trait=0)
    # chevelure de feuilles rouges
    for x, y, r in ((24, 5, 6), (16, 8, 5), (32, 8, 5), (10, 8, 4), (38, 8, 4), (19, 14, 3.4), (30, 14, 3.4), (6, 12, 3),
                    (42, 12, 3)):
        t.p(E(x, y, r, r * 0.8), "l", trait=0)
    t.px([(22, 3), (14, 6), (33, 5), (9, 6), (39, 6)], "l", 4)
    t.px([(26, 7), (18, 10), (35, 10)], "l", 0)
    # visage, larmes de seve rouge
    t.p(E(24, 13, 3.6, 4), "c", trait=0)
    t.px([(22, 13), (26, 13)], "y", 4); t.px([(22, 12), (26, 12)], "o")
    t.px([(22, 14), (22, 15), (26, 14), (26, 15), (26, 16)], "s", 2)
    t.px([(23, 16), (24, 16), (25, 16)], "c", 0)
    t.px([(24, 24), (24, 26), (23, 30), (25, 33), (10, 20), (38, 21)], "s", 2)


@sprite("phalene-d-aube", "Phalène d'aube")
def phalene(t):
    t.mat("a", ["a86a4a", "d89a70", "f4c8a0", "fce8d0", "fffaf0"], ["4a6aa8", "709ad8", "a0c8f4", "d0e8fc", "f0faff"])   # ailes d'aurore
    t.mat("r", ["b84a6a", "e87a90", "f8a8b4", "fcd4d8", "fff0f0"], ["6a4ab8", "907ae8", "b4a8f8", "d8d4fc", "f0f0ff"])
    t.mat("f", "e8dcc8", "dcd8e8")   # corps duveteux
    t.mat("y", "2a2430")
    t.mat("g", OR, ["5a5a8a", "8a8ac0", "b8b8e8", "e0e0ff", "ffffff"])
    for cote in (1, -1):
        c = lambda x: 24 + cote * x
        haut = B([(c(2), 20), (c(6), 6), (c(16), 2), (c(22), 8), (c(20), 20), (c(10), 24)])
        bas = B([(c(2), 24), (c(12), 24), (c(19), 30), (c(16), 40), (c(8), 42), (c(3), 34)])
        t.p(bas, "r", trait=0, dl=0 if cote < 0 else -1)
        t.p(haut, "a", trait=0, dl=0 if cote < 0 else -1)
        # ocelles
        t.p(E(c(14), 12, 3.4, 3.4), "g", trait=0, ton=3)
        t.p(E(c(14), 12, 1.8, 1.8), "y", trait=0, ton=2)
        t.px((c(14) - 1, 11), "g", 4)
        t.p(E(c(11), 33, 2.2, 2.2), "g", trait=0, ton=2)
        for k in (7, 12, 17):
            t.px([(c(3 + k * i / 5), 20 - (k * 0.7) * i / 5) for i in range(1, 6)], "a", 1)
        t.px([(c(20), 12), (c(21), 16), (c(17), 37), (c(13), 41)], "g", 3)
    # corps velu, antennes plumeuses
    t.p(E(24, 22, 3, 5), "f", trait=0)
    t.p(E(24, 31, 2.4, 7), "f", trait=0)
    t.px([(23, 27), (25, 27), (23, 31), (25, 31), (23, 35), (25, 35)], "f", 0)
    t.p(E(24, 15, 2.8, 2.6), "f", trait=0)
    t.px([(22, 15), (26, 15)], "y", 2)
    for cote in (1, -1):
        c = lambda x: 24 + cote * x
        t.p(CS([(c(1), 13), (c(4), 7), (c(8), 3)]), "f", trait=0)
        t.px([(c(4), 5), (c(6), 6), (c(6), 3), (c(8), 5)], "f", 1)


@sprite("duchesse-sanguine", "Duchesse sanguine")
def duchesse(t):
    t.mat("r", ["3a0410", "70101e", "a82030", "d84a50", "f8a09c"], ["10043a", "1e1070", "3020a8", "504ad8", "9ca0f8"])   # robe rouge sang
    t.mat("n", "24202c", "e8e4dc")
    t.mat("c", ["7a7088", "a89eb4", "d0c8d8", "ece6f0", "fffaff"])   # peau de marbre
    t.mat("g", OR)
    t.mat("s", SANG, FEU_VERT)
    # grand col de vampire
    t.p(B([(12, 22), (14, 8), (20, 14), (24, 16), (28, 14), (34, 8), (36, 22), (30, 20), (18, 20)]), "n", trait=0)
    t.p(B([(14, 20), (16, 11), (20, 16), (24, 18), (28, 16), (32, 11), (34, 20)]), "r", trait=0, dl=-1)
    # robe a panier
    t.p(B([(19, 20), (24, 18), (29, 20), (28, 28), (36, 36), (40, 45), (8, 45), (12, 36), (20, 28)]), "r", trait=0)
    for x in (13, 19, 29, 35):
        t.p(L(x, 34 + abs(x - 24) // 6, x + (x - 24) // 3, 45), "r", trait=0, ton=0, sur="r")
    t.px([(10, 45), (16, 44), (24, 45), (32, 44), (38, 45)], "n", 2)
    t.p(R(20, 26, 28, 27), "g", trait=0)
    t.p(B([(21, 20), (24, 22), (27, 20), (26, 25), (22, 25)]), "c", trait=0)
    # bras : une coupe d'or pleine
    t.p(K([(20, 21, 1.5), (14, 25, 1.3), (10, 21, 1.1)]), "c", trait=1)
    t.p(K([(28, 21, 1.5), (33, 27, 1.3), (31, 33, 1.1)]), "c", trait=1, dl=-1)
    t.p(P([(6, 13), (13, 13), (11, 18), (8, 18)]), "g", trait=0)
    t.p(R(9, 18, 10, 21) | R(7, 21, 12, 21), "g", trait=0)
    t.p(R(7, 13, 12, 13), "s", trait=0, ton=2)
    t.px([(7, 15), (6, 17)], "s", 2)
    # tete : chignon haut, diademe, levres rouges
    t.p(B([(19, 12), (20, 4), (24, 0), (28, 4), (29, 12), (27, 16), (21, 16)]), "n", trait=0)
    t.p(E(24, 11, 3.6, 4.2), "c", trait=0)
    t.px([(22, 10), (26, 10)], "s", 3); t.px([(22, 9), (26, 9)], "o")
    t.px([(23, 13), (24, 13), (25, 13)], "s", 2)
    t.px([(23, 14), (25, 14)], "c", 4)
    t.p(R(20, 6, 28, 6), "g", trait=0, ton=3); t.px((24, 5), "s", 3)
    t.px((24, 15), "s", 2)


@sprite("rat-des-ossuaires", "Rat des ossuaires")
def rat(t):
    t.mat("f", "6a5e58", "c8c0b0")
    t.mat("r", "d09a9a", "e8b8c0")   # peau nue : oreilles, pattes, queue
    t.mat("k", OS)
    t.mat("y", SANG, FEU_VERT)
    t.mat("w", "f4f0e6")
    # un tas d'os
    t.p(B([(4, 42), (12, 38), (26, 40), (40, 38), (46, 43), (42, 46), (6, 46)]), "k", trait=0, dl=-1)
    t.p(E(38, 40, 3.4, 3), "k", trait=0)
    t.px([(37, 40), (39, 40)], "o"); t.px([(37, 42), (38, 42), (39, 42)], "o")
    t.p(C(8, 42, 18, 44, 1), "k", trait=0, dl=1)
    # queue annelee
    t.p(S([(36, 34, 1.6), (42, 32, 1.3), (45, 26, 1.1), (42, 20, 0.8)]), "r", trait=0)
    t.px([(40, 33), (43, 30), (45, 25), (43, 21)], "r", 0)
    # corps bossu
    t.p(B([(14, 30), (20, 22), (30, 21), (38, 27), (38, 36), (30, 40), (18, 39)]), "f", trait=0)
    t.p(E(32, 34, 5.5, 5), "f", trait=2)
    t.p(K([(33, 38, 1.6), (29, 41, 1.2)]) | K([(18, 36, 1.4), (15, 40, 1.1)]), "r", trait=0)
    t.px([(13, 41), (14, 41), (27, 42), (28, 42)], "r", 4)
    # tete pointue, incisives jaunes, oreilles rondes
    t.p(E(21, 22, 3.4, 3.6), "r", trait=0, dl=-1)
    t.p(B([(3, 29), (9, 23), (16, 22), (20, 27), (18, 33), (10, 34)]), "f", trait=0)
    t.p(E(17, 21, 3.2, 3.4), "r", trait=0)
    t.px([(11, 27), (12, 27)], "y", 3); t.px((11, 27), "y", 4)
    t.px([(3, 29), (3, 30)], "r", 1)
    t.px([(5, 33), (6, 33), (5, 34), (6, 34)], "w", 3)
    t.px([(1, 27), (0, 30), (1, 32)], "w", 3)
    t.px([(24, 26), (28, 29), (22, 32), (34, 29)], "f", 0)
    # il ronge une phalange
    t.p(C(5, 36, 11, 38, 1), "k", trait=1)


@sprite("moine-psalmodiant", "Moine psalmodiant")
def moine_psalmodiant(t):
    t.mat("b", "5a4a3a", "3a3a5a")   # bure
    t.mat("s", SPECTRE, FEU_VIOLET)  # corps de spectre
    t.mat("k", "1c1418")
    t.mat("g", OR)
    t.mat("o2", ["8a8ab0", "b0b0d0", "d0d0ea", "eaeaf8", "ffffff"])
    # bure sans jambes, qui s'effiloche en fumee
    t.p(B([(15, 18), (24, 15), (33, 18), (35, 32), (33, 41), (29, 45), (27, 40), (24, 46), (21, 40), (17, 45), (14, 38), (13, 30)]),
        "b", trait=0)
    for x in (17, 21, 27, 31):
        t.p(L(x, 24, x, 40), "b", trait=0, ton=0, sur="b")
    t.p(R(15, 28, 33, 29), "g", trait=0, ton=1)
    t.px([(26, 30), (26, 32), (26, 34), (27, 35)], "g", 2)
    # manches jointes sur un psautier ouvert
    t.p(K([(17, 20, 2.6), (15, 26, 2.4), (20, 27, 2.2)]) | K([(31, 20, 2.6), (33, 26, 2.4), (28, 27, 2.2)]), "b", trait=1)
    t.p(P([(18, 24), (24, 25), (30, 24), (30, 28), (24, 29), (18, 28)]), "o2", trait=1, ton=3)
    t.p(L(24, 25, 24, 29), "o2", trait=0, ton=1)
    t.px([(20, 26), (21, 26), (26, 26), (27, 26), (20, 27), (27, 27)], "o2", 0)
    # capuchon profond : rien dedans que deux lueurs et une bouche ouverte
    t.p(B([(16, 17), (17, 7), (24, 2), (31, 7), (32, 17), (28, 20), (20, 20)]), "b", trait=0, dl=1)
    t.p(E(24, 11.5, 4.4, 5.4), "k", trait=0, ton=2)
    t.px([(22, 10), (26, 10)], "s", 4)
    t.p(E(24, 14, 1.4, 1.8), "s", trait=0, ton=2)
    # le psaume : des notes qui montent
    for x, y in ((38, 12), (42, 6), (8, 10)):
        t.p(E(x, y + 3, 1.6, 1.3), "o2", trait=0, ton=3)
        t.p(R(x + 1, y - 3, x + 1, y + 3), "o2", trait=0, ton=3)
        t.px([(x + 2, y - 3), (x + 3, y - 2)], "o2", 3)
    t.px([(12, 42), (36, 43), (24, 47)], "s", 3)
