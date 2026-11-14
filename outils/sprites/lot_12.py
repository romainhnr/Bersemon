from dessin import *
from matieres import *
from atelier import sprite
from commun import _aile_membrane
import math


@sprite("nautile-des-abysses", "Nautile des abysses")
def nautile(t):
    t.mat("c", ["6a5a48", "a08a6c", "d0bc98", "ece0c4", "fcf8ec"], ["3a4a6a", "5c74a0", "8ca4cc", "bcd0ec", "e8f2fc"])   # coquille
    t.mat("z", "8a3a24", "2a3a7a")   # flammes de la coquille
    t.mat("t", ["5a2a3a", "8a4a5c", "b87484", "dca4ae", "f6d4d8"], ["2a3a5a", "4a5c8a", "7484b8", "a4aedc", "d4d8f6"])   # tentacules
    t.mat("y", "f8f0d0", "f8f0d0")
    t.mat("g", GLACE, FEU_VIOLET)    # lueur des fonds
    # tentacules
    for i, (x1, y1, x2, y2) in enumerate(((10, 30, 3, 26), (9, 34, 1, 33), (10, 38, 3, 41), (13, 41, 8, 46), (17, 42, 15, 47),
                                          (11, 26, 6, 20))):
        t.p(S([(17, 33, 1.8), (x1, y1, 1.4), (x2, y2, 0.7)]), "t", trait=0, dl=-(i % 2))
    # la coquille en spirale
    t.p(E(28, 24, 17, 17), "c", trait=0)
    pts = []
    for i in range(60):
        a = i * 0.24
        r = 15.5 * math.exp(-0.085 * i * 0.6)
        pts.append((28 + r * math.cos(a + 2.2), 24 + r * math.sin(a + 2.2)))
    t.p(LS(pts), "c", trait=0, ton=0, sur="c")
    for a in range(-60, 150, 26):
        x0, y0 = 28 + 8 * math.cos(math.radians(a)), 24 + 8 * math.sin(math.radians(a))
        x1, y1 = 28 + 16 * math.cos(math.radians(a - 12)), 24 + 16 * math.sin(math.radians(a - 12))
        t.p(C(x0, y0, x1, y1, 0.6, 1.5), "z", trait=0, sur="c")
    # le capuchon et l'oeil a fente
    t.p(B([(11, 25), (17, 22), (22, 28), (22, 37), (17, 41), (12, 37)]), "t", trait=1)
    t.p(E(17, 30, 2.6, 2.6), "y", trait=2, ton=3)
    t.px([(16, 30), (17, 30), (18, 30)], "o")
    t.px([(4, 22), (2, 37), (10, 45), (44, 8), (45, 40)], "g", 4)
    t.px([(24, 10), (22, 12), (21, 15)], "c", 4)


@sprite("amiral-noye", "Amiral noyé")
def amiral(t):
    t.mat("u", ["101a34", "1e3058", "324c84", "5a78b0", "98b4dc"], ["341a10", "58301e", "844c32", "b0785a", "dcb498"])   # habit d'amiral
    t.mat("g", OR)
    t.mat("c", ["2a4a52", "487a80", "74a8a8", "a4d0cc", "d4f0ea"])   # chair de noye
    t.mat("a", "3f7a44", "7a4a7a")
    t.mat("s", ACIER)
    t.mat("k", OS)
    t.mat("y", SPECTRE, FEU)
    # sabre leve
    t.p(CS([(8, 24), (5, 14), (7, 4)]), "s", trait=0, ton=3)
    t.p(CS([(9, 24), (6, 14), (8, 5)]), "s", trait=0, ton=1)
    t.p(R(6, 24, 11, 25), "g", trait=0)
    # jambes, bottes pleines d'eau, une jambe de bois
    t.p(K([(27, 34, 2.4), (28, 44, 1.2)]), "k", trait=0, dl=-2)
    t.p(K([(21, 34, 2.6), (20, 41, 2.4)]), "u", trait=0, dl=-1)
    t.p(R(16, 41, 22, 45), "u", trait=0, dl=-2)
    # redingote longue, epaulettes, boutons
    t.p(B([(16, 17), (24, 14), (32, 17), (34, 28), (36, 40), (27, 37), (24, 36), (21, 37), (13, 40), (14, 28)]), "u", trait=0)
    t.px([(24, y) for y in range(18, 36)], "u", 0)
    t.px([(22, 20), (22, 24), (22, 28), (22, 32), (26, 20), (26, 24), (26, 28), (26, 32)], "g", 3)
    t.p(R(13, 16, 19, 18) | R(29, 16, 35, 18), "g", trait=0)
    t.px([(13, 19), (15, 19), (17, 19), (31, 19), (33, 19), (35, 19)], "g", 1)
    t.p(K([(17, 19, 2.2), (11, 24, 1.9), (8, 26, 1.7)]), "u", trait=1)
    t.p(K([(31, 19, 2.2), (36, 26, 1.9), (36, 33, 1.7)]), "u", trait=0, dl=-1)
    t.p(E(36, 35, 1.8, 2), "c", trait=0)
    # tete gonflee, barbe d'algues, bicorne
    t.p(E(24, 10.5, 4.8, 5), "c", trait=0)
    t.p(E(22, 10, 1.2, 1.3) | E(26, 10, 1.2, 1.3), "o", trait=0, ton=2)
    t.px([(22, 10), (26, 10)], "y", 4)
    for pts in ([(21, 13), (20, 18), (22, 22)], [(24, 14), (24, 20)], [(27, 13), (28, 18), (27, 23)]):
        t.p(K([(x, y, 0.9) for x, y in pts]), "a", trait=0)
    t.p(B([(13, 7), (18, 2), (24, 0), (30, 2), (35, 7), (29, 7), (24, 5), (19, 7)]), "u", trait=0, dl=-1)
    t.p(L(14, 7, 34, 7), "g", trait=0, ton=3)
    t.px((24, 2), "g", 4)
    t.px([(12, 34), (12, 38), (38, 30), (30, 42), (18, 40)], "y", 3)


@sprite("golem-de-fer-noir", "Golem de fer noir")
def golem_de_fer(t):
    t.mat("n", FER_NOIR)
    t.mat("g", PIERRE)
    t.mat("f", FEU, FEU_VIOLET)
    # jambes en blocs
    t.p(R(26, 33, 33, 44), "n", trait=0, dl=-1)
    t.p(R(15, 33, 22, 44), "n", trait=0)
    t.p(R(13, 43, 23, 45) | R(25, 43, 35, 45), "g", trait=0)
    # torse : un bloc de minerai, veine de feu
    t.p(B([(12, 14), (18, 10), (30, 10), (36, 14), (37, 28), (33, 34), (15, 34), (11, 28)]), "n", trait=0)
    t.p(LS([(20, 14), (23, 19), (21, 24), (25, 28), (24, 33)]), "f", trait=0, ton=2)
    t.px([(23, 19), (25, 28), (22, 22)], "f", 4)
    t.px([(15, 16), (16, 15), (33, 16), (14, 26), (34, 26), (28, 14), (30, 22), (17, 30)], "n", 4)
    # epaules de roche, bras-masses
    t.p(E(11, 15, 5, 4.4) | E(37, 15, 5, 4.4), "g", trait=0)
    t.p(K([(10, 18, 3.4), (7, 27, 3.2), (7, 35, 3.8)]), "n", trait=1)
    t.p(K([(38, 18, 3.4), (41, 27, 3.2), (41, 35, 3.8)]), "n", trait=0, dl=-1)
    t.p(R(3, 34, 11, 40), "g", trait=0)
    t.p(R(37, 34, 45, 40), "g", trait=0, dl=-1)
    t.px([(4, 36), (8, 38), (39, 37), (43, 35)], "g", 0)
    # tete enfoncee, fente ardente
    t.p(B([(18, 11), (20, 4), (28, 4), (30, 11)]), "n", trait=0)
    t.p(R(20, 7, 28, 8), "f", trait=0, ton=1)
    t.px([(21, 7), (22, 7), (26, 7), (27, 7)], "f", 4)
    t.grain("n", 0.06, 5)
    t.grain("g", 0.1, 7)


@sprite("mineur-possede", "Mineur possédé")
def mineur(t):
    t.mat("c", ["3a4444", "5c6c6a", "869896", "b0c0bc", "d8e4e0"], ["443a44", "6c5c6a", "988696", "c0b0bc", "e4d8e0"])   # chair grise
    t.mat("v", "5a4a3a", "3a4a5a")   # vetements
    t.mat("b", BOIS)
    t.mat("a", ACIER)
    t.mat("l", FEU, FEU_VERT)        # la lampe
    t.mat("y", SPECTRE, FEU_VIOLET)
    # pioche levee
    t.p(C(36, 30, 39, 4, 1), "b", trait=0)
    t.p(B([(28, 6), (36, 2), (45, 5), (46, 9), (39, 6), (31, 9)]), "a", trait=0)
    # jambes
    t.p(K([(27, 32, 2.6), (29, 39, 2.2), (28, 44, 2)]), "v", trait=0, dl=-1)
    t.p(K([(21, 32, 2.6), (19, 39, 2.2), (19, 44, 2)]), "v", trait=0)
    t.p(R(15, 44, 21, 45) | R(26, 44, 32, 45), "o", trait=0, ton=2)
    # torse voute, tablier de cuir
    t.p(B([(16, 18), (23, 15), (31, 18), (32, 27), (30, 34), (18, 34), (16, 27)]), "v", trait=0)
    t.p(B([(19, 22), (28, 22), (29, 34), (19, 34)]), "b", trait=0, dl=-1)
    t.p(K([(30, 19, 2.2), (35, 23, 1.9), (36, 28, 1.8)]), "v", trait=1)
    t.p(K([(17, 20, 2.2), (12, 26, 1.9), (11, 32, 1.7)]), "v", trait=1)
    t.p(E(11, 34, 1.8, 2) | E(36, 30, 1.8, 1.8), "c", trait=0)
    # tete : barbe de poussiere, yeux revulses, barrette et chandelle
    t.p(E(22, 11, 4.8, 5), "c", trait=0)
    t.p(B([(18, 13), (22, 15), (26, 13), (25, 19), (22, 21), (19, 19)]), "v", trait=0, dl=1)
    t.p(E(20, 10.5, 1.2, 1.2) | E(24, 10.5, 1.2, 1.2), "y", trait=2, ton=4)
    t.p(E(22, 7, 6, 2.6), "b", trait=0, dl=-1)
    t.p(R(21, 2, 22, 5), "a", trait=0, ton=4)
    t.p(P([(20, 2), (21, -2), (23, 2)]), "l", trait=0, ton=2); t.px((21, 1), "l", 4)
    # l'esprit qui le tient
    t.px([(13, 14), (12, 17), (14, 20), (30, 12), (32, 9), (15, 10)], "y", 3)
    t.px([(13, 16), (31, 11)], "y", 4)


@sprite("grisou-ardent", "Grisou ardent")
def grisou(t):
    t.mat("g", ["2a4a1c", "4a7a2c", "74a844", "a4d070", "d8f0b0"], ["3a1c4a", "5c2c7a", "8644a8", "b470d0", "e0b0f0"])   # nuee de gaz
    t.mat("f", FEU, FEU_BLEU)
    t.mat("k", "16240e", "1c0e24")
    # nuee en volutes
    for x, y, r in ((14, 18, 8), (24, 13, 9), (34, 18, 8), (11, 28, 7.5), (37, 28, 7.5), (17, 36, 7), (31, 36, 7), (24, 26, 12)):
        t.p(E(x, y, r, r * 0.9), "g", trait=0)
    for x, y in ((9, 16), (18, 9), (30, 9), (40, 17), (6, 27), (42, 30), (14, 40), (34, 40)):
        t.px([(x, y), (x + 1, y), (x, y + 1)], "g", 4)
    for x, y in ((17, 22), (32, 22), (13, 32), (36, 34), (24, 39)):
        t.px([(x, y), (x + 1, y + 1)], "g", 0)
    # la flamme qui l'embrase par le coeur
    t.p(B([(24, 15), (28, 22), (30, 29), (27, 34), (21, 34), (18, 29), (20, 22)]), "f", trait=0, ton=1)
    t.p(B([(24, 20), (27, 26), (27, 31), (24, 33), (21, 31), (21, 26)]), "f", trait=0, ton=3)
    t.px([(24, 29), (24, 30), (24, 31)], "f", 4)
    # deux yeux fendus, mauvais
    t.p(P([(14, 22), (19, 24), (15, 26)]) | P([(34, 22), (29, 24), (33, 26)]), "k", trait=0, ton=2)
    t.px([(16, 24), (32, 24)], "f", 3)
    # etincelles
    t.px([(4, 12), (44, 10), (2, 36), (46, 38), (24, 2), (10, 44), (38, 45)], "f", 3)


@sprite("dragon-de-rouille", "Dragon de rouille")
def dragon_de_rouille(t):
    t.mat("r", ["3a1c10", "6a3418", "a05424", "d08440", "f0bc80"], ["10303a", "18546a", "2484a0", "40b4d0", "80dcf0"])   # plaques rouillees
    t.mat("a", ACIER)
    t.mat("m", "5a4a44", "445a5a")   # ailes de tole
    t.mat("y", FEU_VERT, FEU)
    t.mat("w", OS)
    _aile_membrane(t, "m", "a", (27, 20), (31, 3), [(42, 1), (46, 11), (43, 21), (36, 19)], dl=-1)
    t.px([(38, 8), (41, 14), (36, 13)], "o")
    # queue en chaine
    t.p(S([(34, 32, 3.2), (40, 36, 2.4), (44, 41, 1.6), (46, 45, 0.8)]), "r", trait=0)
    t.p(K([(31, 33, 2.8), (33, 39, 2), (32, 45, 1.8)]), "r", dl=-1, trait=0)
    t.p(K([(18, 33, 2.4), (18, 39, 1.8), (17, 45, 1.6)]), "r", dl=-1, trait=0)
    # corps en plaques rivetees
    t.p(B([(12, 26), (19, 21), (30, 21), (37, 26), (37, 34), (29, 37), (18, 37), (12, 33)]), "r", trait=0)
    for x in (17, 22, 27, 32):
        t.p(L(x, 22, x - 1, 36), "r", trait=0, ton=0, sur="r")
    t.px([(15, 26), (20, 24), (25, 23), (30, 24), (35, 27), (19, 34), (24, 35), (29, 34)], "a", 4)
    t.p(K([(15, 33, 2.8), (13, 39, 2), (12, 45, 1.8)]), "r")
    t.p(K([(29, 34, 3.2), (29, 40, 2), (27, 45, 1.8)]), "r")
    t.px([(9, 46), (10, 46), (12, 46), (24, 46), (25, 46), (27, 46)], "w", 3)
    # cou articule, tete de fer
    t.p(S([(15, 26, 3.4), (12, 19, 2.8), (12, 13, 2.6)]), "r", trait=0)
    t.px([(9, 22), (10, 22), (14, 21), (9, 17), (10, 17), (14, 16)], "r", 0)
    t.p(B([(1, 13), (5, 7), (13, 6), (17, 10), (15, 15), (8, 17), (2, 16)]), "r", trait=0)
    t.p(P([(13, 7), (20, 3), (16, 10)]) | P([(10, 6), (14, 0), (14, 6)]), "a", trait=0)
    t.px([(7, 10), (8, 10)], "y", 3); t.px((7, 10), "o"); t.px([(6, 9), (7, 9), (8, 9), (9, 8)], "o")
    t.px([(2, 15), (3, 15), (4, 15), (5, 15), (6, 15), (7, 14)], "o")
    t.px([(3, 16), (5, 16)], "w", 4)
    # ecailles qui tombent en poussiere
    t.px([(22, 40), (24, 43), (36, 41), (20, 44), (39, 30)], "r", 3)


@sprite("gemmame", "Gemmâme")
def gemmame(t):
    t.mat("c", ["5a0a2a", "9a1848", "d83c70", "f888a8", "ffd8e4"], ["0a2a5a", "18489a", "3c70d8", "88a8f8", "d8e4ff"])   # cristal
    t.mat("g", PIERRE)
    t.mat("s", SPECTRE, FEU_VIOLET)  # ames
    # la gangue de roche
    t.p(B([(8, 38), (14, 33), (24, 34), (34, 33), (40, 38), (38, 45), (10, 45)]), "g", trait=0)
    t.px([(13, 40), (20, 43), (30, 42), (35, 39)], "g", 0)
    # la druse : sept prismes
    for x, bas, h, l, dl in ((12, 36, 13, 3, -1), (36, 36, 14, 3, -1), (17, 37, 21, 3.4, 0), (31, 37, 19, 3.4, -1),
                             (24, 38, 30, 4.4, 0), (21, 38, 12, 2.4, 1), (28, 38, 10, 2.4, 0)):
        t.p(P([(x - l, bas), (x - l, bas - h + l), (x, bas - h - 1), (x + l, bas - h + l), (x + l, bas)]), "c", trait=1, dl=dl)
        t.px([(x - l + 1, y) for y in range(int(bas - h + l), bas, 2)], "c", 4)
        t.p(L(x, bas - h, x, bas), "c", trait=0, ton=1, sur="c")
    # l'ame prisonniere : un visage dans le grand cristal
    t.p(E(22.5, 21, 1, 1.4) | E(25.5, 21, 1, 1.4), "s", trait=0, ton=4)
    t.p(E(24, 25, 1.2, 1.6), "s", trait=0, ton=3)
    # feux follets qui tournent autour
    for x, y in ((6, 14), (42, 12), (40, 26)):
        t.p(E(x, y, 2, 2.6), "s", trait=0, ton=3)
        t.px([(x, y - 4), (x, y)], "s", 4)
    t.px([(9, 24), (44, 20), (4, 22), (38, 6)], "s", 3)


@sprite("prevot-des-fosses", "Prévôt des fosses")
def prevot(t):
    t.mat("n", "26222e", "e4dcd0")   # robe et cagoule noires
    t.mat("c", ["4a4a44", "74746a", "9c9c90", "c4c4b4", "e8e8d8"])   # chair morte
    t.mat("a", FER_NOIR, ACIER)
    t.mat("g", OR)
    t.mat("y", SANG, FEU_VERT)
    # chaines qui pendent de ses bras
    for x, y0 in ((6, 24), (10, 26), (40, 25), (43, 23)):
        for y in range(y0, 44, 3):
            t.p(E(x, y, 1.3, 1.6) - E(x, y, 0.3, 0.6), "a", trait=0, ton=3 if (y // 3) % 2 else 2)
    # robe de juge, large aux epaules
    t.p(B([(15, 16), (24, 13), (33, 16), (36, 30), (38, 45), (10, 45), (12, 30)]), "n", trait=0)
    for x in (16, 21, 27, 32):
        t.p(L(x, 26, x + (x - 24) // 4, 44), "n", trait=0, ton=0, sur="n")
    t.px([(12, 45), (17, 44), (24, 45), (31, 44), (36, 45)], "n", 3)
    # bras ecartes
    t.p(K([(17, 18, 2.6), (10, 21, 2.2), (5, 22, 1.9)]) | K([(31, 18, 2.6), (38, 21, 2.2), (43, 21, 1.9)]), "n", trait=1)
    t.p(E(4, 22, 1.8, 2) | E(44, 21, 1.8, 2), "c", trait=0)
    # la grande cle des fosses, pendue au cou
    t.p(R(23, 20, 24, 34), "g", trait=0)
    t.p(E(23.5, 19, 2.6, 2.6) - E(23.5, 19, 1, 1), "g", trait=0)
    t.p(R(25, 30, 27, 31) | R(25, 33, 27, 34), "g", trait=0, ton=3)
    # cagoule de bourreau, haute, deux trous rouges
    t.p(B([(17, 16), (18, 6), (24, 0), (30, 6), (31, 16), (27, 19), (21, 19)]), "n", trait=0, dl=1)
    t.p(E(21.5, 10, 1.4, 1.2) | E(26.5, 10, 1.4, 1.2), "y", trait=2, ton=3)
    t.px([(21, 10), (26, 10)], "y", 4)
    t.px([(22, 15), (24, 15), (26, 15)], "n", 0)
