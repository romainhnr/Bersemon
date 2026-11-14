from dessin import *
from matieres import *
from atelier import sprite
import math

MER = ["1c3a6a", "2e5c98", "4a88c8", "84bce8", "d0ecfc"]


@sprite("farfadet", "Farfadet")
def farfadet(t):
    t.mat("c", PEAU)
    t.mat("v", ["1c5a2a", "2e8a40", "4cb85c", "88dc90", "c8f4cc"], ["5a1c4a", "8a2e74", "b84c9c", "dc88c8", "f4c8ec"])   # habit de feuilles
    t.mat("r", ["6a0c14", "a81c20", "e03a30", "f8806c", "ffd0c4"], OR)   # bonnet
    t.mat("a", ["6aa8c8", "9accec", "c4e6f8", "e4f4fc", "ffffff"], ["c86aa8", "ec9acc", "f8c4e6", "fce4f4", "ffffff"])   # ailes de libellule
    t.mat("h", "c86a30", "e8e0f0")   # cheveux roux
    t.mat("g", OR)
    t.mat("y", "2a8a4a")
    # quatre ailes fines
    for cote, dl in ((1, -1), (-1, 0)):
        c = lambda x: 24 + cote * x
        t.p(B([(c(2), 22), (c(8), 10), (c(18), 6), (c(20), 12), (c(10), 22)]), "a", trait=0, dl=dl)
        t.p(B([(c(2), 25), (c(12), 24), (c(18), 30), (c(10), 32), (c(3), 29)]), "a", trait=0, dl=dl - 1)
        t.p(L(c(3), 22, c(17), 9) | L(c(3), 26, c(15), 29), "a", trait=0, ton=0)
    # jambes repliees en tailleur, chausses a pointe
    t.p(K([(22, 34, 1.8), (16, 37, 1.5), (20, 40, 1.3)]) | K([(26, 34, 1.8), (32, 37, 1.5), (28, 40, 1.3)]), "v", trait=0, dl=-1)
    t.p(P([(19, 40), (15, 42), (21, 42)]) | P([(29, 40), (33, 42), (27, 42)]), "r", trait=0)
    # petit corps, tunique dentelee
    t.p(B([(20, 23), (24, 21), (28, 23), (30, 33), (27, 36), (24, 34), (21, 36), (18, 33)]), "v", trait=0)
    t.px([(24, 25), (24, 28), (24, 31)], "g", 3)
    # bras : un pied de nez, une piece d'or lancee en l'air
    t.p(K([(21, 24, 1.3), (16, 22, 1.1), (15, 17, 1)]), "c", trait=1)
    t.p(K([(27, 24, 1.3), (33, 23, 1.1), (36, 19, 1)]), "c", trait=0, dl=-1)
    t.p(E(38, 14, 2, 2), "g", trait=0); t.px((37, 13), "g", 4)
    # grosse tete rieuse, oreilles en pointe, bonnet rouge
    t.p(E(24, 15, 5.4, 5.2), "c", trait=0)
    t.p(P([(19, 14), (13, 11), (19, 17)]) | P([(29, 14), (35, 11), (29, 17)]), "c", trait=0)
    t.px([(21, 15), (27, 15)], "y", 2); t.px([(21, 14), (27, 14)], "o")
    t.p(LS([(21, 18), (23, 19), (25, 19), (27, 18)]), "o", trait=0, ton=2)
    t.px([(20, 17), (28, 17)], "r", 3)
    t.p(B([(18, 12), (20, 6), (27, 3), (32, 0), (30, 7), (30, 12), (24, 10)]), "r", trait=0)
    t.px((32, -1), "g", 4)
    t.px([(19, 12), (20, 13), (29, 13)], "h", 2)
    t.px([(8, 36), (42, 34), (6, 4), (44, 26)], "g", 3)


@sprite("meduse-des-naufrages", "Méduse des naufrages")
def meduse(t):
    t.mat("o2", ["3a2a6a", "6a4aa8", "9a7ad8", "c8b0f0", "f0e4ff"], ["2a6a5a", "4aa88c", "7ad8b8", "b0f0d8", "e4fff4"])   # ombrelle
    t.mat("f", ["5a2a7a", "9a4ab8", "d080e0", "f0b8f4", "ffe8ff"], FEU_VERT)   # filaments urticants
    t.mat("b", BOIS)
    t.mat("g", OR)
    # tentacules longs, ondulants
    for i, x in enumerate((13, 18, 23, 28, 33)):
        t.p(S([(x, 22, 1.4), (x + (2 if i % 2 else -2), 30, 1.1), (x + (-1 if i % 2 else 1), 38, 0.9), (x + (2 if i % 2 else -2), 46, 0.5)]),
            "f", trait=0, dl=-(i % 2))
    # bras oraux en dentelle
    t.p(B([(18, 21), (30, 21), (28, 29), (26, 25), (24, 32), (22, 25), (20, 29)]), "o2", trait=0, dl=1)
    # l'ombrelle
    t.p(B([(6, 20), (9, 9), (18, 3), (30, 3), (39, 9), (42, 20), (35, 23), (24, 24), (13, 23)]), "o2", trait=0)
    for x in (12, 18, 24, 30, 36):
        t.p(L(24, 5, x, 22), "o2", trait=0, ton=1, sur="o2")
    t.px([(13, 9), (15, 7), (18, 6)], "o2", 4)
    # elle porte l'epave en trophee : une figure de proue prise dans ses filaments
    t.p(P([(19, 9), (29, 9), (30, 16), (18, 16)]), "b", trait=0, dl=-1)
    t.p(E(24, 12, 2.6, 2.6), "g", trait=0); t.px([(23, 12), (25, 12)], "o")
    # points lumineux au bord
    for x in range(8, 42, 4):
        t.px((x, 21 + (x % 8 == 0)), "f", 4)
    t.px([(4, 34), (44, 30), (40, 44)], "f", 3)


@sprite("requin-noir", "Requin noir")
def requin(t):
    t.mat("n", ["0e1018", "1e222e", "343a4a", "565e72", "8c94a8"], ["18100e", "2e221e", "4a3a34", "725e56", "a8948c"])
    t.mat("v", "9aa0ac", "aca09a")
    t.mat("w", OS)
    t.mat("y", SANG, FEU_VERT)
    t.mat("r", SANG, FEU_VIOLET)
    # queue en croissant
    t.p(P([(40, 24), (47, 12), (45, 23), (47, 34), (41, 28)]), "n", trait=0, dl=-1)
    # corps en fuseau
    t.p(B([(1, 25), (8, 18), (20, 15), (32, 18), (42, 24), (42, 28), (32, 32), (18, 34), (7, 31)]), "n", trait=0)
    t.p(B([(3, 28), (10, 28), (20, 30), (32, 29), (40, 27), (32, 32), (18, 34), (8, 32)]), "v", trait=0)
    # ailerons
    t.p(P([(20, 15), (26, 3), (30, 17)]), "n", trait=0)
    t.p(P([(18, 32), (22, 42), (26, 32)]), "n", trait=0, dl=-1)
    t.p(P([(34, 19), (38, 14), (38, 21)]), "n", trait=0)
    # ouies, cicatrices, harpon casse plante dans le dos
    t.px([(15, 22), (15, 24), (15, 26), (17, 22), (17, 24), (17, 26), (19, 23), (19, 25)], "n", 0)
    t.px([(26, 21), (27, 22), (28, 23), (30, 20), (31, 21)], "r", 3)
    t.p(C(33, 18, 37, 8, 0.8), "w", trait=0, dl=-2)
    # gueule beante, rangees de dents
    t.p(B([(1, 27), (7, 26), (12, 28), (9, 33), (3, 32)]), "r", trait=1, ton=0)
    t.px([(2, 27), (4, 27), (6, 27), (8, 27), (10, 28), (3, 31), (5, 32), (7, 32), (9, 31)], "w", 4)
    t.px([(9, 21), (10, 21)], "y", 3); t.px((9, 21), "o")
    t.px([(4, 23), (5, 22)], "o")
    t.px([(12, 18), (20, 17), (28, 18)], "n", 4)


@sprite("albatros-maudit", "Albatros maudit")
def albatros(t):
    t.mat("p", SPECTRE, FEU_VIOLET)  # plumage de spectre
    t.mat("k", OS)
    t.mat("b", "c8b890", "9090a8")
    t.mat("c", "b89a64", "d8d0c0")   # corde
    t.mat("y", "f8f8ff", "f04848")
    # ailes tres longues et etroites, penne en os
    for (bx, by), (cx, cy), (px, py), dl in (((27, 22), (37, 13), (47, 16), -1), ((21, 22), (11, 12), (0, 15), 0)):
        t.p(P([(bx, by - 2), (cx, cy - 2), (px, py), (cx, cy + 4), (bx, by + 4)]), "p", trait=0, dl=dl)
        for f in (0.3, 0.5, 0.7, 0.9):
            x, y = cx + (px - cx) * f, cy + (py - cy) * f
            t.p(L(x, y - 1, x + (2 if px > 24 else -2), y + 4), "k", trait=0, ton=3)
    # corps maigre, cotes a jour
    t.p(B([(18, 20), (24, 17), (30, 20), (32, 28), (28, 35), (21, 35), (17, 28)]), "p", trait=0)
    for y in (23, 26, 29):
        t.px([(21, y), (22, y + 1), (26, y + 1), (27, y)], "k", 4)
    # la corde au cou : il porte un marin pendu en sautoir
    t.p(CS([(20, 20), (24, 24), (28, 20)]), "c", trait=0)
    t.p(R(24, 24, 24, 36), "c", trait=0)
    t.p(E(24, 38, 2, 2), "k", trait=0); t.px([(23, 38), (25, 38)], "o")
    t.p(R(23, 40, 25, 44), "p", trait=0, dl=-2)
    # tete, long bec crochu
    t.p(E(24, 13, 4.2, 4), "p", trait=0)
    t.p(B([(12, 13), (17, 11), (21, 12), (21, 16), (16, 16), (13, 18)]), "b", trait=0)
    t.px((13, 17), "b", 4)
    t.px([(23, 12), (24, 12)], "y", 3); t.px((23, 12), "o")
    t.px([(6, 26), (42, 28), (10, 36), (38, 38)], "p", 4)


@sprite("kraken", "Kraken")
def kraken(t):
    t.mat("c", ["4a1020", "7c2038", "b03c54", "d87888", "f4bcc4"], ["10204a", "20387c", "3c54b0", "7888d8", "bcc4f4"])
    t.mat("v", ["8a5a60", "b88890", "dcb4b8", "f2d8da", "fff2f2"])   # ventouses
    t.mat("y", ["7a6a10", "c0a820", "f0d840", "f8f090", "fffce0"], FEU_VERT)
    t.mat("b", BOIS)
    t.mat("o2", MER)
    # les flots
    t.p(B([(0, 41), (10, 37), (24, 40), (38, 37), (47, 41), (47, 46), (0, 46)]), "o2", trait=0)
    # huit tentacules dresses hors de l'eau
    bras = (((6, 40), (2, 28), (6, 18), -1), ((12, 40), (10, 30), (4, 34), 0), ((42, 40), (46, 28), (42, 18), -1),
            ((36, 40), (38, 30), (44, 34), 0), ((16, 40), (14, 24), (10, 10), -1), ((32, 40), (34, 24), (38, 10), -1))
    for (x0, y0), (x1, y1), (x2, y2), dl in bras:
        t.p(S([(x0, y0, 2.8), (x1, y1, 2.2), (x2, y2, 1)]), "c", trait=0, dl=dl)
        for f in (0.3, 0.5, 0.7):
            t.px((x0 + (x1 - x0) * f * 1.6 if f < 0.6 else x1 + (x2 - x1) * (f - 0.5) * 2,
                  y0 + (y1 - y0) * f * 1.6 if f < 0.6 else y1 + (y2 - y1) * (f - 0.5) * 2), "v", 3)
    # un mat brise serre dans un bras
    t.p(C(38, 14, 40, 2, 1.1), "b", trait=0)
    t.p(R(36, 5, 44, 6), "b", trait=0, dl=-1)
    # le manteau, en ogive
    t.p(B([(14, 30), (16, 16), (24, 6), (32, 16), (34, 30), (30, 40), (18, 40)]), "c", trait=0)
    t.px([(20, 12), (22, 10), (19, 16)], "c", 4)
    t.px([(27, 14), (29, 20), (26, 22), (21, 22)], "c", 0)
    # deux yeux enormes, pupille en barre
    t.p(E(19, 29, 3.2, 3.2) | E(29, 29, 3.2, 3.2), "y", trait=2, ton=3)
    t.p(R(17, 29, 21, 29) | R(27, 29, 31, 29), "o", trait=0, ton=2)
    t.px([(18, 27), (28, 27)], "y", 4)
    t.px([(8, 39), (24, 42), (40, 39), (14, 44), (34, 44)], "o2", 4)


@sprite("pagure-a-crane", "Pagure à crâne")
def pagure(t):
    t.mat("c", ["6a2a14", "a04a20", "d07838", "f0a870", "fcd8b4"], ["142a6a", "204aa0", "3878d0", "70a8f0", "b4d8fc"])   # carapace orange
    t.mat("k", OS)
    t.mat("y", "2a2630")
    t.mat("a", "3f7a44", "7a4a7a")
    # pattes marcheuses
    for i, (x1, y1, x2, y2) in enumerate(((14, 36, 8, 44), (17, 39, 13, 46), (10, 33, 3, 40))):
        t.p(K([(20, 34, 1.8), (x1, y1, 1.4), (x2, y2, 0.8)]), "c", trait=0, dl=-(i % 2))
    # le crane humain qui lui sert de coquille
    t.p(E(30, 23, 13, 12), "k", trait=0)
    t.p(B([(22, 30), (36, 30), (35, 38), (23, 38)]), "k", trait=0, dl=-1)
    t.p(E(25, 23, 3.4, 4) | E(35, 23, 3.4, 4), "o", trait=0, ton=2)
    t.p(P([(29, 27), (30, 31), (31, 27)]), "o", trait=0, ton=2)
    for x in (24, 27, 30, 33):
        t.p(R(x, 34, x, 37), "o", trait=0, ton=2)
    t.px([(24, 14), (27, 13), (22, 17)], "k", 4)
    t.p(LS([(36, 13), (38, 17), (36, 20)]), "k", trait=0, ton=0)
    # algues et anatifes sur le crane
    t.p(K([(34, 12, 1), (38, 8, 0.9), (37, 3, 0.7)]) | K([(30, 11, 1), (29, 6, 0.8)]), "a", trait=0)
    t.px([(40, 20), (41, 22), (40, 27)], "c", 3)
    # le bernard-l'ermite sort par l'orbite et la machoire : pinces, yeux pedoncules
    t.p(B([(14, 28), (20, 26), (24, 30), (22, 36), (16, 36)]), "c", trait=1)
    t.p(K([(17, 30, 2.2), (10, 26, 2), (6, 20, 1.8)]), "c", trait=1)
    t.p(B([(1, 19), (4, 12), (9, 12), (10, 18), (7, 22), (3, 22)]), "c", trait=0)
    t.p(P([(4, 15), (8, 14), (6, 18)]), "o", trait=0, ton=2)
    t.p(K([(20, 33, 1.8), (16, 30, 1.5), (13, 32, 1.3)]), "c", trait=1, dl=-1)
    for x in (18, 21):
        t.p(R(x, 20, x, 26), "c", trait=0)
        t.p(E(x, 19, 1.4, 1.4), "y", trait=0, ton=2); t.px((x, 18), "k", 4)
    t.px([(15, 30), (19, 32)], "c", 4)


@sprite("mouette-hurleuse", "Mouette hurleuse")
def mouette(t):
    t.mat("p", ["8a8a90", "b4b4ba", "dcdce0", "f2f2f4", "ffffff"], ["908a8a", "bab4b4", "e0dcdc", "f4f2f2", "ffffff"])
    t.mat("g", "8a909c", "9c908a")   # ailes grises
    t.mat("n", "2a2630")             # capuchon noir
    t.mat("b", "e05030", "c8c8d8")   # bec et pattes rouges
    t.mat("s", ["8a8ab0", "b0b0d0", "d0d0ea", "eaeaf8", "ffffff"])
    t.mat("l", BOIS)
    # le pieu d'amarrage
    t.p(R(20, 36, 28, 46), "l", trait=0)
    t.p(E(24, 36, 4.4, 1.6), "l", trait=0, dl=1)
    t.px([(22, 40), (25, 43), (26, 39)], "l", 0)
    # ailes a demi levees
    t.p(B([(27, 22), (34, 12), (43, 10), (46, 16), (40, 22), (32, 28)]), "g", trait=0, dl=-1)
    t.p(P([(42, 10), (47, 9), (46, 16)]), "n", trait=0, ton=2)
    t.p(B([(20, 24), (22, 12), (30, 6), (34, 10), (30, 18), (27, 28)]), "g", trait=0)
    t.p(P([(29, 6), (35, 3), (34, 10)]), "n", trait=0, ton=2)
    # corps
    t.p(B([(14, 24), (19, 19), (27, 21), (31, 28), (28, 34), (20, 35), (15, 30)]), "p", trait=0)
    t.p(K([(21, 34, 1), (20, 37, 0.8)]) | K([(25, 34, 1), (26, 37, 0.8)]), "b", trait=0)
    t.px([(18, 37), (19, 37), (21, 37), (25, 37), (27, 37), (28, 37)], "b", 2)
    # tete rejetee en arriere, bec grand ouvert sur un cri
    t.p(E(13, 16, 4.4, 4.2), "n", trait=0, ton=2)
    t.p(E(13, 16, 4.4, 4.2) & R(0, 8, 14, 22), "n", trait=0, ton=3)
    t.p(P([(9, 13), (2, 6), (11, 11)]), "b", trait=0)
    t.p(P([(9, 17), (3, 19), (10, 19)]), "b", trait=0, dl=-1)
    t.px([(13, 14), (14, 14)], "p", 4); t.px((13, 14), "o")
    # le cri
    for d, h in ((6, 5), (9, 8)):
        t.p(CS([(9 - d, 13 - h), (7 - d, 13), (9 - d, 13 + h)]), "s", trait=0, ton=2)


@sprite("naufrageur", "Naufrageur")
def naufrageur(t):
    t.mat("s", SPECTRE, FEU_VIOLET)  # corps de fantome
    t.mat("c", "3a3a44", "443a3a")   # cire, capote
    t.mat("f", FEU, FEU_VERT)        # le fanal trompeur
    t.mat("a", FER_NOIR)
    t.mat("b", BOIS)
    # la perche et le fanal qu'il balance pour attirer les navires sur les rochers
    t.p(C(10, 44, 12, 6, 1), "b", trait=0)
    t.p(R(12, 6, 20, 7), "b", trait=0)
    t.p(R(19, 8, 19, 10), "a", trait=0, ton=3)
    t.p(B([(15, 12), (19, 10), (23, 12), (23, 19), (15, 19)]), "a", trait=0)
    t.p(R(17, 13, 21, 18), "f", trait=0, ton=3)
    t.px([(19, 15), (19, 16)], "f", 4)
    for x, y in ((13, 15), (25, 15), (19, 22), (11, 11), (27, 11)):
        t.px((x, y), "f", 2)
    # suroit de marin sur un corps de brume
    t.p(B([(22, 22), (30, 19), (38, 22), (41, 34), (39, 42), (35, 46), (32, 41), (29, 46), (25, 41), (22, 45), (19, 36)]), "c",
        trait=0)
    for x in (24, 28, 34, 38):
        t.p(L(x, 28, x + (x - 30) // 4, 42), "c", trait=0, ton=0, sur="c")
    t.px([(30, 24), (30, 28), (30, 32)], "a", 4)
    # bras qui tient la perche
    t.p(K([(24, 24, 2.4), (17, 28, 2), (11, 27, 1.7)]), "c", trait=1)
    t.p(E(10, 26.5, 1.8, 1.8), "s", trait=0)
    # tete : un suroit, deux lueurs, sourire de flamme
    t.p(E(30, 15, 4.6, 4.8), "s", trait=0)
    t.px([(28, 14), (32, 14)], "f", 4); t.px([(28, 13), (32, 13)], "o")
    t.p(LS([(27, 17), (29, 18), (31, 18), (33, 17)]), "o", trait=0, ton=2)
    t.p(B([(22, 12), (25, 6), (31, 4), (36, 7), (40, 14), (34, 11), (27, 11)]), "c", trait=0, dl=1)
    t.px([(40, 28), (43, 38), (18, 42)], "s", 4)
