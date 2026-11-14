from dessin import *
from matieres import *
from atelier import sprite
from commun import _aile_membrane
import math


@sprite("sangsue-geante", "Sangsue géante")
def sangsue(t):
    t.mat("c", ["2a141c", "4e2430", "7a3a44", "a8626a", "d89a9c"], ["14262a", "24484e", "3a747a", "62a4a8", "9ad4d8"])
    t.mat("v", "b88a70", "c8d8c0")
    t.mat("w", "f4f0e6")
    t.mat("r", SANG, FEU_VERT)
    corps = [(42, 41, 2.4), (36, 40, 5), (27, 41, 6), (18, 38, 6), (13, 30, 5.6), (13, 21, 5.2), (16, 13, 4.8)]
    t.p(S(corps), "c", trait=0)
    t.p(S([(x - 1.5, y + 1.5, r * 0.35) for x, y, r in corps[1:]]), "v", trait=0, sur="c")
    # anneaux
    for x0, y0, x1, y1 in ((37, 35, 37, 45), (32, 35, 32, 46), (27, 35, 27, 47), (22, 34, 21, 45), (17, 32, 14, 41),
                           (8, 30, 19, 30), (8, 25, 18, 25), (9, 20, 18, 20), (11, 15, 20, 15)):
        t.p(L(x0, y0, x1, y1), "c", trait=0, ton=0, sur=("c", "v"))
    # ventouse dentee, tournee vers nous
    t.p(E(16, 9, 6, 5), "c", trait=0, dl=1)
    t.p(E(15.5, 9, 4.2, 3.4), "r", trait=0, ton=0)
    t.p(E(15.5, 9, 2.2, 1.6), "o", trait=0, ton=2)
    for a in range(0, 360, 45):
        t.px((15.5 + 3.3 * math.cos(math.radians(a)), 9 + 2.6 * math.sin(math.radians(a))), "w", 4)
    t.px([(22, 38), (30, 37), (36, 37), (12, 27)], "c", 4)
    t.px([(13, 15), (12, 17), (12, 19)], "r", 2)


@sprite("kelpie", "Kelpie")
def kelpie(t):
    t.mat("h", ["1c3a4a", "2e6478", "4a94a4", "84c4c8", "c4ecec"], ["3a2a5a", "5e4a8a", "8a74b8", "b8a8dc", "e4dcf8"])   # robe d'eau sombre
    t.mat("a", "3f8a54", "c06a9a")   # criniere d'algues
    t.mat("e", ["1c3a6a", "2e5c98", "4a88c8", "84bce8", "d0ecfc"])
    t.mat("y", "f8f0a0", "f8f0a0")
    # la vague qui le porte
    t.p(B([(2, 42), (8, 36), (16, 38), (24, 34), (33, 37), (41, 34), (46, 40), (44, 45), (4, 45)]), "e", trait=0)
    t.px([(6, 38), (9, 37), (22, 35), (25, 35), (39, 35), (42, 35), (14, 42), (30, 42)], "e", 4)
    # corps qui se cabre hors de l'eau
    t.p(B([(18, 22), (25, 19), (33, 23), (36, 32), (33, 40), (22, 40), (17, 32)]), "h", trait=0)
    t.p(K([(20, 28, 2.4), (13, 27, 1.7), (10, 32, 1.4)]), "h", trait=1)
    t.p(K([(21, 24, 2.4), (14, 21, 1.7), (9, 24, 1.4)]), "h", trait=0, dl=-1)
    t.px([(9, 34), (10, 34), (8, 26), (9, 26)], "o")
    # encolure, tete
    t.p(B([(18, 26), (15, 15), (18, 8), (24, 10), (26, 22)]), "h", trait=0)
    t.p(B([(3, 15), (7, 8), (14, 5), (20, 7), (21, 12), (15, 15), (10, 19), (5, 19)]), "h", trait=0)
    t.p(P([(16, 6), (18, 0), (20, 6)]), "h", trait=0)
    t.px([(13, 9), (14, 9)], "y", 3); t.px((13, 9), "o")
    t.px([(4, 16), (4, 17)], "o"); t.px([(5, 19), (6, 19), (7, 19)], "h", 0)
    # criniere d'algues ruisselantes
    for pts in ([(19, 5), (24, 9), (27, 16)], [(22, 9), (28, 14), (31, 22)], [(24, 14), (30, 21), (33, 30)],
                [(25, 20), (30, 28), (30, 36)]):
        t.p(S([(x, y, 1.5) for x, y in pts]), "a", trait=0)
    t.px([(27, 12), (31, 19), (33, 27)], "a", 4)
    t.px([(12, 37), (20, 34), (36, 33), (28, 30)], "e", 3)


@sprite("wendigo", "Wendigo")
def wendigo(t):
    t.mat("c", ["3a4252", "5c6878", "8492a0", "b0bcc8", "dce4ec"], ["4a3a3a", "786058", "a48a7c", "ccb4a4", "f0dcd0"])   # peau gelee
    t.mat("k", OS)
    t.mat("y", GLACE, FEU)
    t.mat("r", SANG)
    # bois de cerf
    for pts in ([(20, 9), (16, 3), (11, 1)], [(16, 3), (17, -2)], [(13, 2), (9, 5)],
                [(26, 9), (30, 3), (35, 1)], [(30, 3), (29, -2)], [(33, 2), (37, 5)]):
        t.p(K([(x, y, 0.9) for x, y in pts]), "k", trait=0)
    # jambes de cerf, decharnees
    t.p(K([(27, 32, 2), (30, 38, 1.3), (28, 45, 1.1)]), "c", trait=0, dl=-1)
    t.p(K([(21, 32, 2), (18, 38, 1.3), (20, 45, 1.1)]), "c", trait=0)
    t.px([(18, 46), (19, 46), (27, 46), (28, 46)], "o")
    # torse creuse, cotes saillantes
    t.p(B([(17, 17), (24, 15), (31, 17), (30, 26), (27, 33), (21, 33), (18, 26)]), "c", trait=0)
    for y in (19, 22, 25):
        t.px([(20, y), (21, y + 1), (22, y + 1), (26, y + 1), (27, y + 1), (28, y)], "c", 0)
    t.px([(24, 28), (24, 30)], "c", 0)
    # bras interminables, griffes
    t.p(K([(18, 18, 1.8), (12, 26, 1.3), (10, 36, 1.1)]), "c", trait=1)
    t.p(K([(30, 18, 1.8), (36, 26, 1.3), (38, 36, 1.1)]), "c", trait=0, dl=-1)
    t.px([(8, 38), (9, 39), (10, 40), (11, 38), (37, 38), (38, 40), (39, 39), (40, 38)], "k", 3)
    # crane de cerf pour tete
    t.p(B([(19, 8), (23, 6), (27, 8), (27, 13), (25, 19), (22, 19), (20, 13)]), "k", trait=0)
    t.p(E(21.5, 11, 1.3, 1.5) | E(25.5, 11, 1.3, 1.5), "o", trait=0, ton=2)
    t.px([(21, 11), (25, 11)], "y", 3)
    t.px([(23, 16), (24, 16), (23, 18), (24, 18)], "o")
    t.px([(22, 20), (23, 21), (25, 20)], "r", 2)
    t.px([(14, 12), (33, 13), (9, 30), (40, 28), (15, 42)], "y", 4)


@sprite("troll-des-cavernes", "Troll des cavernes")
def troll(t):
    t.mat("c", ["3a3a34", "5e5e52", "86866f", "aeae94", "d4d4bc"], ["4a3440", "765464", "a07a8c", "c8a4b4", "ecd0dc"])   # cuir gris
    t.mat("g", PIERRE)
    t.mat("b", BOIS)
    t.mat("w", OS)
    t.mat("y", "f8e040", "40e0f0")
    t.mat("l", "5a8a4a", "8a6aa8")
    # massue
    t.p(C(41, 40, 36, 12, 1.6, 3.4), "b", trait=0)
    t.px([(35, 12), (38, 15), (36, 19), (39, 22)], "w", 3)
    # jambes courtes
    t.p(K([(29, 34, 4), (31, 43, 3.4)]), "c", trait=0, dl=-1)
    t.p(K([(19, 34, 4), (17, 43, 3.4)]), "c", trait=0)
    t.px([(13, 45), (14, 46), (16, 46), (18, 46), (29, 46), (31, 46), (33, 46)], "w", 2)
    # torse enorme, dos de roche moussue
    t.p(B([(11, 20), (17, 12), (29, 11), (37, 18), (38, 30), (32, 37), (17, 37), (11, 30)]), "c", trait=0)
    t.p(B([(22, 11), (29, 8), (37, 14), (39, 24), (35, 20), (29, 14)]), "g", trait=0)
    t.px([(27, 10), (32, 11), (36, 16), (30, 9)], "l", 2)
    t.p(E(21, 30, 6.5, 5), "c", trait=0, dl=1)
    # bras qui trainent
    t.p(K([(14, 20, 3.6), (9, 29, 3), (9, 38, 2.6)]), "c", trait=1)
    t.p(E(9, 40, 3, 2.6), "c", trait=0)
    t.p(K([(36, 20, 3.4), (40, 29, 2.8), (40, 37, 2.4)]), "c", trait=0, dl=-1)
    # petite tete, gros nez, defenses
    t.p(B([(13, 14), (15, 8), (22, 6), (27, 10), (26, 17), (19, 20), (14, 19)]), "c", trait=0)
    t.p(E(14.5, 15, 2.6, 3), "c", trait=0, dl=1)
    t.px([(18, 11), (19, 11)], "y", 3); t.px((18, 11), "o"); t.px([(17, 10), (18, 10), (19, 10), (20, 9)], "o")
    t.px([(15, 19), (16, 19), (17, 19), (18, 19), (19, 19), (20, 18)], "o")
    t.px([(16, 18), (16, 17), (20, 17), (20, 16)], "w", 4)
    t.p(P([(25, 9), (29, 7), (27, 12)]), "c", trait=0, dl=-1)


@sprite("harpie-des-orages", "Harpie des orages")
def harpie(t):
    t.mat("p", "4a5a8a", "8a4a5a")   # plumes d'orage
    t.mat("c", PEAU)
    t.mat("h", "2e2c40", "e8e0d0")   # cheveux
    t.mat("b", "d8b040", "c0c0d0")   # serres
    t.mat("e", ["8a6a10", "c8a018", "f8d830", "fff080", "ffffd8"], FEU_BLEU)   # eclairs
    # ailes a la place des bras, grandes ouvertes
    for cote, dl in ((1, -1), (-1, 0)):
        c = lambda x: 24 + cote * x
        for i, (dx, y) in enumerate(((20, 4), (22, 11), (22, 18), (19, 24), (14, 28))):
            t.p(C(c(4), 18, c(dx), y, 3, 1.3), "p", trait=0, dl=dl - (i % 2))
        t.p(E(c(8), 16, 5, 3.6), "p", trait=0, dl=dl)
    # eclairs
    for cote in (1, -1):
        c = lambda x: 24 + cote * x
        t.p(LS([(c(13), 26), (c(16), 31), (c(14), 32), (c(18), 38)]), "e", trait=0, ton=3)
    # corps de femme, plumage au ventre
    t.p(B([(20, 18), (24, 16), (28, 18), (29, 26), (27, 33), (21, 33), (19, 26)]), "c", trait=0)
    t.p(B([(19, 25), (24, 23), (29, 25), (30, 33), (24, 37), (18, 33)]), "p", trait=0)
    t.px([(21, 28), (24, 30), (27, 28), (22, 33), (26, 33)], "p", 0)
    # pattes d'oiseau, serres ouvertes
    for x in (21, 27):
        t.p(K([(x, 35, 1.5), (x, 41, 1)]), "b", trait=0)
        t.px([(x - 2, 43), (x - 1, 42), (x, 43), (x + 1, 42), (x + 2, 43), (x, 44)], "b", 3)
    # tete qui hurle, cheveux au vent
    t.p(B([(18, 12), (20, 5), (27, 4), (33, 7), (36, 4), (33, 13), (29, 16), (20, 16)]), "h", trait=0)
    t.p(E(23.5, 12, 3.6, 4), "c", trait=0)
    t.px([(22, 11), (25, 11)], "e", 3); t.px([(22, 10), (25, 10)], "o")
    t.p(E(23.5, 14.5, 1.2, 1.3), "o", trait=0, ton=2)
    t.px([(23, 20), (24, 21)], "c", 1)


@sprite("wyrm-de-givre", "Wyrm de givre")
def wyrm_de_givre(t):
    t.mat("e", ["2a4a7a", "4a78b0", "7aa8d8", "b0d4f0", "e6f6ff"], ["5a3a7a", "8a60b0", "b490d8", "d8c0f0", "f6ecff"])
    t.mat("v", "dceaf4", "f4ecf8")
    t.mat("g", GLACE)
    t.mat("y", "f8f8ff", "f8e040")
    t.mat("w", "f4f0e6")
    # corps enorme sans pattes ni ailes, love en huit
    corps = [(8, 38, 1.4), (14, 42, 3), (23, 43, 4.4), (33, 41, 5), (40, 34, 5.2), (37, 26, 5), (29, 24, 5), (23, 19, 4.8),
             (20, 12, 4.4)]
    t.p(S(corps), "e", trait=1)
    t.p(S([(x - 1, y + 1.6, r * 0.4) for x, y, r in corps[1:]]), "v", trait=0, sur="e")
    # aiguilles de glace le long de l'echine
    for x, y, dx, dy in ((16, 39, -1, -5), (24, 38, 0, -5), (33, 35, 2, -5), (44, 31, 4, -3), (41, 23, 4, -4), (32, 19, 2, -5),
                         (27, 15, 4, -3), (24, 9, 4, -3)):
        t.p(P([(x - 1, y), (x + dx, y + dy), (x + 2, y + 1)]), "g", trait=0)
    # tete massive, cornes de glace, souffle de givre
    t.p(B([(5, 12), (9, 5), (17, 3), (23, 7), (23, 14), (16, 17), (8, 17)]), "e", trait=0)
    t.p(P([(18, 4), (26, -1), (22, 7)]) | P([(14, 3), (18, -3), (18, 4)]), "g", trait=0)
    t.px([(12, 8), (13, 8)], "y", 3); t.px((12, 8), "o"); t.px([(11, 7), (12, 7), (13, 7), (14, 6)], "o")
    t.px([(6, 14), (7, 15), (8, 15), (9, 15), (10, 15), (11, 14), (12, 14)], "o")
    t.px([(7, 16), (9, 16), (11, 15)], "w", 4)
    t.px((6, 11), "o")
    for x, y in ((2, 14), (0, 17), (3, 19), (-1, 21), (2, 23), (5, 21)):
        t.px((x, y), "g", 4 if (x + y) % 2 else 3)
    t.px([(30, 40), (38, 33), (33, 26), (25, 21)], "e", 0)


@sprite("sylphe-des-neiges", "Sylphe des neiges")
def sylphe(t):
    t.mat("a", ["6a8ab8", "9ab8dc", "c4dcf2", "e4f2fc", "ffffff"], ["b88aa8", "dcb0c8", "f2d0e2", "fce8f2", "ffffff"])   # ailes de givre
    t.mat("c", ["7a8aa8", "a4b4cc", "ccd8e8", "e8f0f8", "ffffff"], PEAU)
    t.mat("r", "a8c8e8", "e8b8d0")   # voile
    t.mat("h", "f0f4fc", "f8e8b0")
    t.mat("y", "4a7ad0", "d04a7a")
    # ailes en flocon : six branches
    for a in range(0, 360, 60):
        x, y = 24 + 19 * math.cos(math.radians(a + 30)), 20 + 17 * math.sin(math.radians(a + 30))
        t.p(K([(24, 20, 1.6), (x, y, 0.8)]), "a", trait=0)
        for f in (0.5, 0.75):
            mx, my = 24 + (x - 24) * f, 20 + (y - 20) * f
            for s in (1, -1):
                bx = mx + 4 * math.cos(math.radians(a + 30 + s * 55))
                by = my + 4 * math.sin(math.radians(a + 30 + s * 55))
                t.p(L(mx, my, bx, by), "a", trait=0, ton=3)
    # robe de voile qui s'effile en tourbillon
    t.p(B([(20, 20), (24, 18), (28, 20), (31, 32), (28, 40), (24, 45), (23, 39), (18, 33)]), "r", trait=0)
    t.p(CS([(17, 36), (22, 41), (28, 43), (33, 40)]), "a", trait=0, ton=4)
    t.px([(21, 28), (26, 30), (23, 35), (25, 40)], "r", 4)
    # bras ouverts, tete, cheveux de neige
    t.p(K([(21, 21, 1.2), (15, 24, 1), (12, 21, 0.9)]) | K([(27, 21, 1.2), (33, 24, 1), (36, 21, 0.9)]), "c", trait=0)
    t.p(B([(19, 12), (24, 8), (29, 12), (32, 22), (27, 17), (21, 17), (16, 22)]), "h", trait=0)
    t.p(E(24, 14, 3.4, 3.8), "c", trait=0)
    t.px([(22, 14), (26, 14)], "y", 3); t.px([(23, 17), (24, 17), (25, 17)], "c", 1)
    t.px([(8, 34), (40, 36), (36, 44), (11, 42), (42, 26)], "a", 4)


@sprite("margrave-de-givre", "Margrave de givre")
def margrave(t):
    t.mat("a", ["2a3a5a", "48608a", "7090b8", "a4c0dc", "dcecf8"], ["4a2a3a", "7a4860", "a8708c", "d0a4b8", "f4dce8"])   # armure prise dans la glace
    t.mat("g", GLACE)
    t.mat("c", ["3a4a6a", "5e7498", "8ca2c0", "bcccdc", "e8f0f8"])   # visage bleui
    t.mat("m", "c8d8ec", "ecd8e0")   # manteau de fourrure blanche
    t.mat("y", "f8f8ff", "f04848")
    # grande epee de glace, plantee
    t.p(P([(6, 12), (8, 4), (10, 12), (10, 44), (6, 44)]), "g", trait=0)
    t.px([(7, y) for y in range(8, 44, 3)], "g", 4)
    t.p(R(3, 22, 13, 23), "a", trait=0)
    # manteau
    t.p(B([(16, 16), (24, 13), (33, 16), (38, 30), (40, 45), (14, 45), (13, 30)]), "m", trait=0, dl=-1)
    t.px([(15, 44), (19, 45), (25, 44), (31, 45), (37, 44)], "m", 4)
    # armure
    t.p(B([(18, 17), (24, 15), (31, 17), (32, 28), (30, 36), (19, 36), (17, 28)]), "a", trait=0)
    t.p(R(19, 36, 23, 44) | R(26, 36, 30, 44), "a", trait=0, dl=-1)
    t.p(R(17, 44, 23, 45) | R(26, 44, 32, 45), "a", trait=0)
    t.px([(24, 18), (24, 21), (24, 24), (24, 27), (24, 30)], "a", 0)
    t.p(R(19, 30, 30, 31), "g", trait=0, ton=3)
    # stalactites aux epaules et aux coudes
    t.p(E(17, 17, 4, 3) | E(32, 17, 4, 3), "a", trait=0)
    for x, y, h in ((14, 19, 5), (17, 20, 4), (32, 20, 4), (35, 19, 5), (20, 31, 3), (28, 31, 4)):
        t.p(P([(x - 1, y), (x, y + h), (x + 1, y)]), "g", trait=0, ton=4)
    t.p(K([(17, 19, 2.2), (12, 23, 1.9), (9, 22, 1.7)]), "a", trait=1)
    t.p(K([(32, 19, 2.2), (36, 26, 1.9), (35, 32, 1.7)]), "a", trait=0, dl=-1)
    # tete : barbe de givre, couronne de glacons
    t.p(E(24.5, 10, 4.4, 4.8), "c", trait=0)
    t.p(B([(21, 12), (24, 13), (28, 12), (27, 18), (24, 20), (22, 18)]), "m", trait=0)
    t.px([(23, 9), (26, 9)], "y", 4); t.px([(22, 8), (23, 8), (26, 8), (27, 8)], "o")
    for x, h in ((20, 4), (22, 6), (24, 8), (26, 6), (28, 4)):
        t.p(P([(x, 6), (x + 0.5, 6 - h), (x + 1, 6)]), "g", trait=0, ton=4)
    t.p(R(20, 5, 29, 6), "g", trait=0, ton=2)
    t.px([(4, 30), (42, 24), (44, 38), (2, 40)], "g", 4)
