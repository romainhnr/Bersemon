from dessin import *
from matieres import *
from atelier import sprite
import math


@sprite("basilic", "Basilic")
def basilic(t):
    t.mat("e", "4f8a44", "8a4a8a")   # ecailles
    t.mat("v", "c8d49a", "f0d8f0")
    t.mat("r", SANG, OR)             # crete en couronne
    t.mat("y", "f8e040", "40e0f0")
    t.mat("w", "f4f0e6")
    t.p(S([(33, 33, 3.4), (40, 35, 2.6), (45, 31, 1.6), (44, 25, 0.8)]), "e", trait=0)
    t.p(K([(18, 34, 2), (15, 39, 1.5), (12, 43, 1.2)]), "e", dl=-1, trait=0)
    t.p(K([(31, 35, 2), (34, 40, 1.5), (37, 43, 1.2)]), "e", dl=-1, trait=0)
    t.p(B([(13, 30), (19, 26), (28, 27), (35, 30), (36, 35), (29, 38), (19, 38), (13, 35)]), "e", trait=0)
    t.p(B([(14, 35), (20, 37), (29, 37), (34, 35), (28, 39), (19, 39)]), "v", trait=0)
    # cou dresse, tete couronnee
    t.p(S([(17, 30, 3.6), (14, 23, 3), (14, 16, 2.8)]), "e", trait=0)
    t.p(S([(15, 31, 1.4), (12, 23, 1.2), (12, 17, 1.1)]), "v", trait=0, sur="e")
    t.p(B([(3, 16), (7, 10), (14, 9), (18, 13), (17, 18), (10, 20), (4, 19)]), "e", trait=0)
    for x, h in ((9, 4), (12, 6), (15, 5), (18, 3)):
        t.p(P([(x - 1, 10 + (x - 9) // 3), (x, 10 - h + (x - 9) // 3), (x + 2, 10 + (x - 9) // 3)]), "r", trait=0)
    for x, y in ((20, 25), (25, 26), (30, 28), (36, 31), (41, 33)):
        t.p(P([(x - 1, y), (x, y - 3), (x + 2, y)]), "r", trait=0, dl=-1)
    t.px([(9, 13), (10, 13), (9, 14), (10, 14)], "y", 3); t.px([(9, 14), (9, 13)], "o")
    t.px((10, 12), "y", 4)
    t.px([(4, 18), (5, 18), (6, 19), (7, 19), (8, 19), (9, 18)], "o")
    t.px([(5, 19), (8, 20)], "w", 4)
    t.p(K([(19, 35, 2.2), (16, 40, 1.6), (12, 44, 1.3)]), "e")
    t.p(K([(30, 36, 2.4), (33, 41, 1.7), (37, 44, 1.3)]), "e")
    t.px([(10, 45), (11, 45), (38, 45), (39, 45)], "w", 2)
    t.px([(22, 30), (26, 31), (30, 32), (21, 33), (27, 34)], "e", 0)


@sprite("cocatrix", "Cocatrix")
def cocatrix(t):
    t.mat("e", "4a7a4a", "7a4a8a")   # ecailles
    t.mat("p", "c89a3a", "d8d8e8")   # plumes du camail
    t.mat("r", SANG, FEU_BLEU)       # crete et barbillons
    t.mat("b", "e0b040", "f0d880")   # bec et pattes
    t.mat("m", "6a8a5a", "9a6aa8")   # membrane
    t.mat("y", "f8f0d0", "40e0f0")
    # aile de chauve-souris
    t.p(P([(24, 22), (27, 6), (37, 3), (45, 10), (44, 22), (38, 18), (33, 24)]), "m", trait=0)
    t.p(K([(24, 22, 1.2), (27, 6, 0.8)]) | K([(27, 6, 0.8), (45, 10, 0.5)]) | K([(27, 6, 0.8), (38, 18, 0.5)]), "e", trait=0)
    # queue de serpent enroulee
    t.p(S([(30, 30, 3.6), (37, 33, 3), (42, 38, 2.4), (38, 43, 2), (32, 41, 1.5), (34, 37, 0.9)]), "e", trait=0)
    # pattes de coq, ergots
    t.p(K([(23, 34, 1.3), (22, 41, 0.9)]) | K([(28, 34, 1.3), (29, 41, 0.9)]), "b", trait=0)
    t.px([(19, 42), (20, 42), (21, 42), (23, 42), (24, 43), (26, 42), (27, 42), (28, 42), (30, 42), (31, 43),
          (24, 39), (31, 39)], "b", 2)
    # corps dodu
    t.p(B([(16, 26), (22, 21), (30, 23), (33, 30), (30, 35), (22, 36), (17, 32)]), "e", trait=0)
    # camail de plumes, cou
    t.p(B([(14, 30), (12, 21), (15, 14), (20, 17), (22, 26), (19, 32)]), "p", trait=0)
    t.px([(14, 24), (16, 27), (18, 22), (15, 19), (19, 28), (17, 31)], "p", 0)
    # tete de coq
    t.p(E(15, 12, 4.4, 4.2), "p", trait=0)
    t.p(P([(11, 11), (4, 13), (11, 15)]), "b", trait=0)
    t.px([(6, 13), (7, 13), (8, 13), (9, 13), (10, 13)], "o")
    for x, h in ((12, 4), (15, 6), (18, 5)):
        t.p(B([(x - 2, 9), (x, 9 - h), (x + 2, 9)]), "r", trait=0)
    t.p(E(11.5, 17.5, 1.6, 2.6), "r", trait=0)
    t.p(E(14.5, 11.5, 1.5, 1.5), "y", trait=0, ton=3); t.px((14, 11), "o")
    t.px([(23, 26), (27, 27), (25, 30), (29, 31), (21, 30)], "e", 0)
    t.px([(36, 33), (40, 37), (38, 42)], "e", 4)


@sprite("elfe", "Elfe")
def elfe(t):
    t.mat("a", ["4a7aa8", "7aa8d0", "a8d0ec", "d4ecfa", "f6fcff"], ["a86a8a", "d09ab8", "ecc4d8", "fae0ec", "fff6fa"])   # ailes
    t.mat("c", PEAU)
    t.mat("h", "e8c860", "c8d8f0")   # cheveux blonds
    t.mat("r", "4a8a4a", "8a4a7a")   # tunique
    t.mat("b", BOIS)
    t.mat("y", "3a7ad0", "d03a7a")
    for cote in (1, -1):
        t.p(B([(24 + cote * 2, 20), (24 + cote * 10, 6), (24 + cote * 19, 5), (24 + cote * 18, 15), (24 + cote * 8, 22)]), "a",
            trait=0, dl=0 if cote < 0 else -1)
        t.p(B([(24 + cote * 2, 23), (24 + cote * 12, 22), (24 + cote * 16, 30), (24 + cote * 9, 33), (24 + cote * 3, 28)]), "a",
            trait=0, dl=-1)
    t.px([(12, 9), (10, 12), (37, 9), (39, 13), (13, 27), (36, 27)], "a", 4)
    # jambes, bottes
    t.p(K([(22, 33, 1.5), (21, 40, 1.2), (19, 45, 1)]) | K([(26, 33, 1.5), (28, 39, 1.2), (29, 44, 1)]), "c", trait=0)
    t.p(K([(21, 41, 1.4), (19, 45, 1.2)]) | K([(28, 40, 1.4), (29, 44, 1.2)]), "b", trait=0)
    # tunique de feuilles
    t.p(B([(20, 20), (24, 18), (28, 20), (30, 31), (27, 35), (24, 33), (21, 35), (18, 31)]), "r", trait=0)
    t.p(R(20, 26, 28, 27), "b", trait=0)
    # arc tendu
    t.p(CS([(11, 14), (7, 22), (11, 31)]), "b", trait=0)
    t.p(L(11, 14, 11, 31), "h", trait=0, ton=4)
    t.p(K([(21, 21, 1.3), (15, 23, 1.1), (9, 22, 1)]), "c", trait=1)
    t.p(K([(27, 21, 1.3), (30, 25, 1.1), (26, 24, 1)]), "c", trait=0, dl=-1)
    # tete : cheveux longs, oreilles pointues
    t.p(B([(19, 11), (24, 7), (29, 11), (31, 24), (27, 17), (21, 17), (18, 23)]), "h", trait=0)
    t.p(E(24, 13.5, 3.6, 4), "c", trait=0)
    t.p(P([(20, 13), (16, 10), (20, 15)]) | P([(28, 13), (32, 10), (28, 15)]), "c", trait=0)
    t.px([(22, 13), (26, 13)], "y", 3); t.px([(22, 12), (26, 12)], "o")
    t.px([(23, 16), (24, 16), (25, 16)], "c", 1)
    t.px([(21, 9), (23, 8), (26, 8)], "h", 4)


@sprite("vouivre", "Vouivre")
def vouivre(t):
    t.mat("e", "3f8a7a", "8a5a9a")   # ecailles d'eau verte
    t.mat("v", "c8e0c0", "f0e0f0")
    t.mat("m", "6aa890", "b08ac0")   # membrane
    t.mat("g", ["6a0a1a", "a8142a", "e03048", "f8808a", "ffe0e0"], FEU_BLEU)   # l'escarboucle
    t.mat("y", "f8e040", "f8e040")
    # ailes
    t.p(P([(27, 20), (30, 4), (40, 1), (46, 9), (45, 20), (39, 16), (34, 22)]), "m", trait=0)
    t.p(K([(27, 20, 1.2), (30, 4, 0.8)]) | K([(30, 4, 0.8), (46, 9, 0.5)]) | K([(30, 4, 0.8), (39, 16, 0.5)]), "e", trait=0)
    t.p(P([(22, 20), (16, 6), (8, 5), (4, 13), (10, 12), (15, 20)]), "m", trait=0, dl=-1)
    # corps de serpent love, sans pattes
    t.p(S([(24, 22, 4.2), (30, 28, 4.4), (36, 35, 4), (30, 42, 3.6), (19, 43, 3.2), (10, 40, 2.6), (6, 33, 1.6), (9, 28, 0.8)]),
        "e", trait=0)
    t.p(S([(23, 24, 1.6), (28, 30, 1.7), (33, 36, 1.5), (29, 41, 1.4), (19, 42, 1.2)]), "v", trait=0, sur="e")
    # cou, tete fine
    t.p(S([(24, 23, 3.8), (21, 16, 3), (17, 11, 2.6)]), "e", trait=0)
    t.p(B([(6, 12), (10, 6), (17, 5), (21, 9), (19, 14), (12, 15), (7, 15)]), "e", trait=0)
    t.p(P([(18, 6), (24, 2), (21, 9)]), "e", trait=0, dl=-1)
    t.px([(12, 9), (13, 9)], "y", 3); t.px((12, 9), "o")
    t.px([(7, 14), (8, 14), (9, 14), (10, 13), (11, 13)], "o")
    # l'escarboucle au front
    t.p(P([(14, 6), (16, 2), (18, 6), (16, 8)]), "g", trait=2)
    t.px((15, 4), "g", 4)
    t.px([(27, 27), (32, 32), (35, 38), (26, 43), (15, 42)], "e", 0)
    t.px([(13, 0), (19, 0), (16, -1)], "g", 3)


@sprite("melusine", "Mélusine")
def melusine(t):
    t.mat("e", "3a6a9a", "9a6a4a")   # ecailles
    t.mat("v", "c0d8e8", "f0e0c8")
    t.mat("m", "6a90b8", "c09a70")   # membrane
    t.mat("c", PEAU)
    t.mat("h", "c86a30", "e8e0f0")   # cheveux roux
    t.mat("g", OR)
    t.mat("r", "e8e4f0", "e8e4f0")   # voile
    # ailes de dragon dans le dos
    for cote, dl in ((1, -1), (-1, 0)):
        t.p(P([(24 + cote * 3, 20), (24 + cote * 8, 4), (24 + cote * 17, 2), (24 + cote * 22, 10), (24 + cote * 19, 20),
               (24 + cote * 14, 15), (24 + cote * 9, 22)]), "m", trait=0, dl=dl)
        t.p(K([(24 + cote * 3, 20, 1.1), (24 + cote * 8, 4, 0.7)]) | K([(24 + cote * 8, 4, 0.7), (24 + cote * 22, 10, 0.5)])
            | K([(24 + cote * 8, 4, 0.7), (24 + cote * 14, 15, 0.5)]), "e", trait=0)
    # queue de serpent, enroulee sous elle
    t.p(S([(24, 28, 4.6), (29, 35, 4.6), (35, 41, 3.8), (27, 45, 3.2), (16, 44, 2.8), (9, 40, 2), (7, 34, 1)]), "e", trait=0)
    t.p(S([(23, 30, 1.7), (27, 36, 1.7), (32, 41, 1.4), (26, 44, 1.2)]), "v", trait=0, sur="e")
    t.px([(30, 33), (33, 38), (22, 45), (13, 42)], "e", 0)
    t.p(P([(7, 34), (3, 30), (5, 36), (2, 38)]), "m", trait=0)
    # buste de femme, bras croises sur un miroir
    t.p(B([(19, 19), (24, 17), (29, 19), (29, 28), (24, 31), (19, 28)]), "c", trait=0)
    t.p(R(20, 22, 28, 24), "e", trait=0, ton=3)
    t.p(K([(20, 20, 1.5), (15, 25, 1.3), (17, 29, 1.2)]) | K([(28, 20, 1.5), (33, 25, 1.3), (30, 29, 1.2)]), "c", trait=1)
    t.p(E(14.5, 23, 2.4, 2.8), "g", trait=0); t.p(E(14.5, 23, 1.4, 1.8), "r", trait=0, ton=4)
    # tete, longue chevelure, hennin et voile
    t.p(B([(19, 11), (24, 8), (29, 11), (32, 26), (28, 17), (20, 17), (16, 26)]), "h", trait=0)
    t.p(E(24, 13, 3.6, 4), "c", trait=0)
    t.px([(22, 13), (26, 13)], "e", 0); t.px([(23, 16), (24, 16), (25, 16)], "c", 0)
    t.p(P([(21, 9), (26, 0), (28, 9)]), "g", trait=0)
    t.p(P([(26, 0), (32, 4), (33, 12), (29, 8)]), "r", trait=0)


@sprite("baron-pendu", "Baron pendu")
def baron_pendu(t):
    t.mat("p", "5a3a7a", "7a2a2a")   # habit
    t.mat("g", OR)
    t.mat("c", ["3a4a3a", "5e7058", "889a7c", "b0c0a0", "d8e4c8"], ["4a3a5a", "70587a", "9a7ca0", "c0a0c8", "e4c8ea"])   # chair morte
    t.mat("k", "b89a64", "d8d0c0")   # corde
    t.mat("y", SANG, FEU_VERT)
    t.mat("n", "2a2430", "2a2430")
    # la corde
    t.p(R(27, -8, 28, 12), "k", trait=0)
    t.px([(27, y) for y in range(-7, 12, 2)], "k", 0)
    t.p(R(21, 15, 28, 16), "k", trait=0, ton=3)
    # jambes qui pendent, pieds tombants
    for x, dl in ((27, -1), (21, 0)):
        t.p(K([(x, 32, 2.2), (x, 39, 1.7), (x - 0.5, 43, 1.5)]), "n", trait=0, dl=dl)
        t.p(K([(x - 0.5, 43, 1.5), (x - 2, 46, 1.2)]), "p", trait=0, dl=dl)
    # habit long, galons d'or
    t.p(B([(16, 18), (24, 16), (32, 18), (34, 28), (33, 36), (24, 34), (15, 36), (14, 28)]), "p", trait=0)
    t.px([(24, y) for y in range(19, 33)], "p", 0)
    t.px([(23, 20), (23, 23), (23, 26), (23, 29), (17, 34), (20, 34), (28, 34), (31, 34)], "g", 3)
    t.p(R(19, 27, 29, 28), "g", trait=0, ton=2)
    # bras ballants, mains grises
    t.p(K([(16, 20, 2.2), (13, 27, 1.9), (13, 33, 1.6)]), "p", trait=1)
    t.p(K([(32, 20, 2.2), (35, 27, 1.9), (35, 33, 1.6)]), "p", trait=0, dl=-1)
    t.p(E(13, 35, 1.8, 2) | E(35, 35, 1.8, 2), "c", trait=0)
    # tete penchee, col casse, couronne de baron
    t.p(E(22, 10, 5, 5.4, -18), "c", trait=0)
    t.p(E(19.5, 9.5, 1.5, 1.4) | E(23.5, 11, 1.5, 1.4), "o", trait=0, ton=2)
    t.px([(19, 9), (23, 11)], "y", 3)
    t.px([(19, 13), (20, 14), (21, 14), (22, 15)], "o")
    t.px((20, 15), "c", 4)
    t.p(P([(17, 6), (16, 1), (19, 3), (21, 0), (23, 4), (26, 2), (26, 8)]), "g", trait=0)
    t.px([(18, 4), (21, 3), (24, 5)], "y", 2)
    t.p(R(28, 15, 30, 17), "k", trait=0)


def _schnoz(t, cx, cy, r, c="b", n="n"):
    t.p(E(cx, cy, r, r * 0.9), c, trait=0)
    # petites pattes
    for dx in (-r * 0.55, r * 0.45):
        t.p(E(cx + dx, cy + r * 0.85, r * 0.26, r * 0.2), c, trait=0, dl=-1)
    # le nez : une trompe rose qui pend
    t.p(S([(cx - r * 0.55, cy - r * 0.1, r * 0.3), (cx - r * 1.1, cy + r * 0.05, r * 0.3), (cx - r * 1.45, cy + r * 0.45, r * 0.27),
           (cx - r * 1.5, cy + r * 0.85, r * 0.22)]), n, trait=1)
    t.p(E(cx - r * 0.1, cy - r * 0.3, r * 0.16, r * 0.2) | E(cx + r * 0.35, cy - r * 0.3, r * 0.16, r * 0.2), "o", trait=0, ton=2)


@sprite("schnoz", "Schnoz")
def schnoz(t):
    t.mat("b", "6a80d8", "e8c048")   # corps
    t.mat("n", "f0a0b8", "a0e0f0")   # nez
    t.mat("s", OR, ["5a5a8a", "8a8ac0", "b8b8e8", "e0e0ff", "ffffff"])   # etoiles
    _schnoz(t, 28, 28, 11)
    t.px([(26, 23), (32, 23)], "s", 4)
    for x, y in ((24, 33), (33, 30), (30, 36), (35, 24), (22, 21)):
        t.px((x, y), "b", 4)
    for x, y in ((12, 12), (40, 10), (44, 34), (20, 6)):
        t.px([(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)], "s", 3)
        t.px((x, y), "s", 4)
    t.px([(6, 20), (36, 4), (45, 22), (30, 44)], "s", 3)


@sprite("schnoz-primordial", "Schnoz primordial")
def schnoz_primordial(t):
    t.mat("b", ["0c0a2a", "1c1a52", "30307e", "5a5ab8", "a0a0f0"], ["2a1a0a", "5a3a12", "8a6020", "c89a40", "f8e0a0"])   # nuit etoilee
    t.mat("n", "f08ab0", "8ae0f0")
    t.mat("s", OR, ["5a5a8a", "8a8ac0", "b8b8e8", "e0e0ff", "ffffff"])
    t.mat("g", FEU_VIOLET, FEU_BLEU)   # la galaxie
    # anneau de poussiere d'etoiles, derriere
    t.p(E(27, 26, 21, 5.5, -14) - E(27, 26, 18, 3.4, -14), "g", trait=0, ton=2)
    _schnoz(t, 28, 26, 14)
    # devant, l'anneau repasse
    t.p((E(27, 26, 21, 5.5, -14) - E(27, 26, 18, 3.4, -14)) & R(0, 27, 47, 40), "g", trait=0, ton=3)
    # une galaxie en spirale dans son ventre
    for i in range(26):
        a = i * 0.5
        r = 0.8 + i * 0.33
        t.px((30 + r * math.cos(a), 27 + r * 0.8 * math.sin(a)), "g", 3 if i % 3 else 4)
    for x, y in ((22, 17), (36, 19), (38, 30), (24, 34), (33, 36), (20, 26)):
        t.px((x, y), "s", 4)
    t.px([(26, 20), (34, 20)], "s", 4)
    for x, y in ((6, 6), (44, 5), (4, 40), (44, 42), (24, 2)):
        t.px([(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1), (x - 2, y), (x + 2, y), (x, y - 2), (x, y + 2)], "s", 3)
        t.px((x, y), "s", 4)
