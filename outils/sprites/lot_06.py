from dessin import *
from matieres import *
from atelier import sprite
import math


def _ailes_d_insecte(t, m, cx, cy, grand=1.0):
    """Quatre ailes nervurees, de part et d'autre du corps."""
    for cote in (1, -1):
        haut = B([(cx + cote * 2, cy), (cx + cote * 8 * grand, cy - 12 * grand), (cx + cote * 17 * grand, cy - 13 * grand),
                  (cx + cote * 19 * grand, cy - 5 * grand), (cx + cote * 10 * grand, cy + 1)])
        bas = B([(cx + cote * 2, cy + 2), (cx + cote * 12 * grand, cy + 3), (cx + cote * 16 * grand, cy + 10 * grand),
                 (cx + cote * 9 * grand, cy + 13 * grand), (cx + cote * 3, cy + 7)])
        t.p(bas, m, trait=0, dl=-1)
        t.p(haut, m, trait=0, dl=0 if cote < 0 else -1)
        for k in (5, 10, 15):
            t.px([(cx + cote * (2 + k * grand * i / 5), cy - (k * 0.7 * grand) * i / 5) for i in range(1, 6)], m, 0)


@sprite("fee-noire", "Fée noire")
def fee_noire(t):
    t.mat("a", "7a5a9a", "4a8a7a")   # ailes
    t.mat("n", "2e2838", "e8e0d0")   # corps et robe
    t.mat("c", ["5a5068", "8a80a0", "b8b0cc", "dcd6ea", "f6f2ff"], PEAU)   # peau blafarde
    t.mat("y", SANG, FEU_VERT)
    _ailes_d_insecte(t, "a", 24, 22)
    # jambes greles
    t.p(K([(22, 33, 1.3), (21, 40, 1), (19, 44, 0.8)]) | K([(26, 33, 1.3), (27, 40, 1), (29, 44, 0.8)]), "c", trait=0, dl=-1)
    # robe en petales sombres
    t.p(B([(20, 21), (24, 19), (28, 21), (31, 33), (27, 36), (24, 34), (21, 36), (17, 33)]), "n", trait=0)
    t.px([(19, 34), (22, 35), (26, 35), (29, 34)], "n", 0)
    # bras
    t.p(K([(21, 22, 1.2), (16, 26, 1), (14, 24, 0.9)]) | K([(27, 22, 1.2), (32, 26, 1), (34, 24, 0.9)]), "c", trait=0)
    # tete, cheveux noirs, antennes
    t.p(E(24, 14, 5, 5), "n", trait=0)
    t.p(E(24, 15, 3.6, 3.8), "c", trait=0)
    t.px([(22, 15), (26, 15)], "y", 3); t.px([(22, 14), (26, 14)], "o")
    t.px([(24, 18)], "y", 1)
    t.p(CS([(22, 10), (19, 5), (15, 4)]) | CS([(26, 10), (29, 5), (33, 4)]), "n", trait=0)
    t.px([(15, 4), (33, 4)], "a", 4)
    t.px([(9, 12), (38, 14), (11, 30), (37, 29)], "a", 4)


@sprite("fee-sorciere", "Fée-sorcière")
def fee_sorciere(t):
    t.mat("a", "6a4a8a", "4a7a8a")
    t.mat("n", "2a2434", "e8e0d0")
    t.mat("c", ["5a5068", "8a80a0", "b8b0cc", "dcd6ea", "f6f2ff"], PEAU)
    t.mat("y", FEU_VERT, FEU)
    t.mat("l", BOIS)
    _ailes_d_insecte(t, "a", 25, 24, 0.9)
    # robe longue, dechiquetee
    t.p(B([(21, 23), (25, 21), (29, 23), (33, 38), (30, 44), (27, 41), (24, 45), (21, 41), (18, 44), (16, 38)]), "n", trait=0)
    t.px([(20, 30), (22, 36), (27, 34), (30, 38), (24, 28)], "n", 3)
    # baguette noueuse, flamme verte
    t.p(C(12, 40, 14, 14, 0.9), "l", trait=0)
    t.p(E(14, 11, 2.8, 3.2), "y", trait=0, ton=2)
    t.p(E(14, 11.5, 1.6, 2), "y", trait=0, ton=4)
    t.px([(14, 6), (11, 8), (17, 8)], "y", 3)
    t.p(K([(22, 24, 1.3), (17, 28, 1.1), (13, 26, 1)]), "c", trait=1)
    t.p(K([(28, 24, 1.3), (33, 29, 1.1), (35, 33, 1)]), "c", trait=0, dl=-1)
    # tete sous un chapeau pointu a bord casse
    t.p(E(25, 17, 3.8, 4), "c", trait=0)
    t.px([(23, 17), (27, 17)], "y", 4); t.px([(23, 16), (27, 16)], "o")
    t.px([(24, 20), (25, 20), (26, 20)], "n", 2)
    t.p(E(25, 13.5, 8, 1.8), "n", trait=1)
    t.p(B([(20, 13), (24, 6), (29, 0), (32, 2), (29, 7), (30, 13)]), "n", trait=0)
    t.p(R(21, 11, 29, 12), "a", trait=0, ton=3)
    t.px([(25, 11), (25, 12)], "y", 3)


@sprite("reine-des-ronces", "Reine des ronces")
def reine_des_ronces(t):
    t.mat("a", "5a8a4a", "8a5a8a")   # ailes de feuille
    t.mat("v", "2f5a34", "5a2a5a")   # ronces
    t.mat("c", ["4a6a5a", "7a9a86", "a8c8b0", "d0e6d4", "f2fcf2"], PEAU)
    t.mat("r", ["5a0a1a", "96142a", "d02c44", "f06a78", "ffd0d4"], OR)   # roses
    t.mat("g", OR)
    _ailes_d_insecte(t, "a", 24, 21, 1.05)
    # robe de ronces en cloche
    t.p(B([(20, 21), (24, 19), (28, 21), (34, 34), (38, 44), (24, 46), (10, 44), (14, 34)]), "v", trait=0)
    for a, b in (((16, 30), (33, 44)), ((32, 30), (15, 44)), ((20, 24), (36, 38)), ((28, 24), (12, 38))):
        t.p(L(a[0], a[1], b[0], b[1]), "v", trait=0, ton=0, sur="v")
    for x, y in ((14, 40), (20, 44), (24, 35), (30, 42), (34, 37), (19, 29), (29, 30), (24, 25)):
        t.p(E(x, y, 1.6, 1.6), "r", trait=0)
        t.px((x, y), "r", 4)
    t.px([(11, 36), (37, 35), (13, 31), (35, 30), (9, 41), (39, 41)], "v", 4)
    # buste, bras ouverts, sceptre d'epine
    t.p(B([(20, 20), (24, 18), (28, 20), (27, 26), (21, 26)]), "c", trait=0)
    t.p(K([(21, 21, 1.3), (15, 25, 1.1), (12, 22, 1)]) | K([(27, 21, 1.3), (33, 25, 1.1), (36, 22, 1)]), "c", trait=0)
    t.p(C(36, 34, 36, 10, 0.8), "v", trait=0)
    t.p(E(36, 8, 2.2, 2.2), "r", trait=0); t.px((36, 8), "r", 4)
    t.px([(35, 14), (37, 18), (35, 22), (37, 26)], "v", 4)
    # tete, longs cheveux, couronne d'epines
    t.p(B([(19, 12), (24, 8), (29, 12), (30, 21), (27, 17), (21, 17), (18, 21)]), "v", trait=0, dl=-1)
    t.p(E(24, 14, 3.6, 4), "c", trait=0)
    t.px([(22, 14), (26, 14)], "r", 3); t.px([(22, 13), (26, 13)], "o")
    t.px([(23, 17), (24, 17), (25, 17)], "r", 2)
    for x, h in ((20, 3), (22, 5), (24, 6), (26, 5), (28, 3)):
        t.p(L(x, 10, x, 10 - h), "g", trait=0, ton=3)
    t.p(R(20, 9, 28, 10), "g", trait=0)


@sprite("carabosse", "Carabosse")
def carabosse(t):
    t.mat("a", "8a7a5a", "6a5a8a")   # ailes de phalene, fanees
    t.mat("v", "3a3044", "5a2a3a")   # chale et robe
    t.mat("c", ["4a5a4a", "7a8a70", "a4b494", "c8d4b8", "eaf2dc"], PEAU)
    t.mat("h", "c8c4cc", "e8d8a0")   # cheveux blancs
    t.mat("r", ["5a0a1a", "96142a", "d02c44", "f06a78", "ffd0d4"], OR)
    t.mat("l", BOIS)
    t.mat("f", "e8e0d0", "c8a0d0")   # fil du fuseau
    # ailes en lambeaux dans le dos
    for pts in ([(28, 18), (40, 4), (46, 10), (44, 20), (36, 22)], [(30, 24), (43, 24), (46, 32), (40, 38), (32, 32)]):
        t.p(B(pts), "a", trait=0, dl=-1)
    t.px([(41, 8), (43, 13), (39, 15), (42, 28), (40, 33), (37, 29)], "a", 0)
    t.p(E(42, 10, 1.6, 1.6) | E(41, 30, 1.6, 1.6), "v", trait=0, ton=1)
    # robe d'epines, dos voute
    t.p(B([(17, 20), (22, 14), (30, 13), (36, 20), (37, 32), (38, 44), (16, 45), (17, 32)]), "v", trait=0)
    t.px([(20, 40), (24, 43), (28, 40), (32, 43), (35, 39), (22, 34), (30, 34), (34, 28), (33, 20), (28, 15)], "v", 4)
    for x, y in ((19, 44), (27, 44), (35, 43), (37, 30), (36, 22)):
        t.p(P([(x, y), (x + 2, y - 3), (x + 3, y)]), "v", trait=0, ton=0)
    t.px([(23, 37), (31, 37)], "r", 3)
    # tete en avant : nez crochu, chignon, verrue
    t.p(E(19, 12, 5, 4.6), "h", trait=0)
    t.p(E(24, 9, 2.6, 2.4), "h", trait=0)
    t.p(B([(11, 12), (14, 8), (19, 9), (20, 16), (16, 19), (12, 17)]), "c", trait=0)
    t.p(P([(11, 12), (7, 15), (12, 15)]), "c", trait=0)
    t.px([(14, 11), (15, 11)], "r", 3); t.px((14, 11), "o")
    t.px([(13, 10), (14, 10), (15, 10), (16, 9)], "o")
    t.px([(12, 17), (13, 17), (14, 17)], "o")
    t.px((16, 14), "c", 0)
    # canne dans une main, fuseau dans l'autre
    t.p(C(9, 45, 11, 22, 1), "l", trait=0)
    t.p(K([(19, 22, 1.8), (14, 26, 1.5), (11, 25, 1.3)]), "c", trait=1)
    t.p(K([(24, 24, 1.8), (27, 30, 1.5), (24, 34, 1.3)]), "c", trait=1)
    t.p(C(24, 30, 22, 41, 0.7), "l", trait=0)
    t.p(E(23, 36, 2, 2.6), "f", trait=0)
    t.px([(22, 42), (21, 43), (20, 43)], "r", 3)
    t.px([(22, 35), (24, 37), (23, 34)], "f", 1)


def _crane(t, cx, cy, k="k", y="y", r=4.6):
    """Crane de profil trois quarts, tourne vers la gauche."""
    t.p(E(cx, cy, r, r - 0.4), k, trait=0)
    t.p(R(cx - r + 1, cy + r - 2, cx + 1, cy + r + 1), k, trait=0)
    t.p(E(cx - 2.2, cy, 1.4, 1.6) | E(cx + 1.6, cy, 1.2, 1.5), "o", trait=0, ton=2)
    t.px([(int(cx - 2), int(cy)), (int(cx + 2), int(cy))], y, 3)
    t.px([(int(cx - 1), int(cy + 2))], "o")
    t.px([(int(cx - r + 1) + i, int(cy + r)) for i in (0, 2, 4)], "o")


@sprite("squelette", "Squelette")
def squelette(t):
    t.mat("k", OS)
    t.mat("y", SANG, FEU_VERT)
    t.mat("c", "5a4a3a", "3a4a5a")   # lambeaux
    # jambes : femur, tibia, pied
    for x0, x1, x2, dl in ((27, 30, 29, -1), (21, 19, 20, 0)):
        t.p(K([(x0, 30, 1.3), (x1, 37, 1.1), (x2, 44, 1.1)]), "k", trait=0, dl=dl)
        t.p(E(x1, 37, 1.6, 1.6), "k", trait=0, dl=dl)
        t.p(R(x2 - 3, 44, x2 + 1, 45), "k", trait=0, dl=dl)
    # bassin, colonne, cotes
    t.p(B([(19, 27), (24, 26), (29, 27), (28, 31), (24, 32), (20, 31)]), "k", trait=0)
    t.p(R(23, 14, 24, 27), "k", trait=0)
    for y in (15, 18, 21, 24):
        t.p(E(24, y, 5.6 - (y - 15) * 0.2, 1) - R(23, y - 1, 25, y + 1), "k", trait=0)
    t.p(R(18, 13, 30, 14), "k", trait=0, ton=3)
    # bras en garde, poings d'os
    t.p(K([(18, 14, 1.3), (13, 19, 1.1), (11, 13, 1.1)]), "k", trait=1)
    t.p(E(11, 11, 2, 2), "k", trait=0)
    t.p(K([(30, 14, 1.3), (34, 20, 1.1), (30, 24, 1.1)]), "k", trait=0, dl=-1)
    t.p(E(29, 25, 2, 2), "k", trait=0, dl=-1)
    t.px([(13, 19), (34, 20)], "k", 0)
    _crane(t, 23, 7, r=5)
    t.px([(26, 28), (27, 30), (21, 29), (25, 31)], "c", 2)


@sprite("ankou", "Ankou")
def ankou(t):
    t.mat("k", OS)
    t.mat("n", "2e2a36", "5a2a2a")   # manteau
    t.mat("c", "3a3642", "4a3a2a")   # chapeau
    t.mat("h", "d0ccd4", "f0e0b0")   # cheveux blancs
    t.mat("a", ACIER)
    t.mat("l", BOIS)
    t.mat("y", SPECTRE, FEU)
    # la faux, lame tournee vers le dehors
    t.p(C(38, 46, 36, 4, 1), "l", trait=0)
    t.p(B([(36, 5), (40, 1), (45, 4), (47, 13), (44, 8), (40, 5), (37, 8)]), "a", trait=0)
    t.px([(40, 2), (43, 4), (45, 7), (46, 10)], "a", 4)
    # long manteau qui tombe jusqu'aux pieds
    t.p(B([(18, 18), (24, 15), (31, 18), (34, 30), (37, 45), (15, 45), (16, 30)]), "n", trait=0)
    t.px([(20, 28), (20, 34), (20, 40), (25, 26), (25, 33), (25, 41), (30, 28), (30, 36), (31, 42)], "n", 0)
    t.px([(18, 44), (22, 44), (27, 44), (33, 44)], "n", 3)
    t.p(R(19, 45, 23, 46) | R(28, 45, 32, 46), "k", trait=0)
    # un bras d'os tient le manche, l'autre montre du doigt
    t.p(K([(20, 20, 2), (14, 24, 1.7), (9, 22, 1.4)]), "n", trait=1)
    t.p(E(7.5, 21.5, 1.8, 1.6), "k", trait=0)
    t.px([(5, 21), (4, 21)], "k", 3)
    t.p(K([(29, 20, 2), (33, 26, 1.7), (36, 24, 1.4)]), "n", trait=1)
    t.p(E(37, 23.5, 1.8, 1.8), "k", trait=0)
    # crane, longs cheveux blancs, chapeau a larges bords
    t.p(B([(19, 11), (29, 11), (31, 21), (28, 17), (20, 17), (17, 21)]), "h", trait=0)
    _crane(t, 24, 12.5, r=4.2)
    t.p(E(24, 8.5, 10, 1.8), "c", trait=1)
    t.p(B([(19, 8), (20, 3), (28, 3), (29, 8)]), "c", trait=0)
    t.px([(20, 7), (22, 7), (24, 7), (26, 7), (28, 7)], "c", 4)
    t.px([(14, 30), (38, 33), (13, 38)], "y", 3)


def _cerf(t, m="b", v="v"):
    """Cerf debout de profil, tourne vers la gauche ; sans les bois."""
    t.p(K([(21, 31, 1.9), (22, 38, 1.2), (21, 45, 1.1)]), m, dl=-1, trait=0)
    t.p(K([(36, 31, 2.4), (39, 38, 1.3), (38, 45, 1.1)]), m, dl=-1, trait=0)
    t.p(B([(15, 24), (21, 21), (32, 22), (40, 24), (41, 30), (36, 33), (24, 33), (16, 31)]), m, trait=0)
    t.p(B([(18, 31), (26, 32), (36, 32), (30, 34), (22, 34)]), v, trait=0)
    t.p(K([(18, 31, 2), (16, 38, 1.2), (16, 45, 1.1)]), m)
    t.p(K([(34, 31, 2.6), (35, 38, 1.3), (33, 45, 1.1)]), m)
    t.px([(15, 46), (16, 46), (20, 46), (21, 46), (32, 46), (33, 46), (37, 46), (38, 46)], "o")
    t.p(P([(40, 23), (44, 22), (42, 27)]), v, trait=0)
    # cou dresse, tete fine
    t.p(B([(14, 27), (11, 17), (14, 12), (18, 14), (21, 23)]), m, trait=0)
    t.p(B([(5, 15), (8, 10), (14, 9), (16, 13), (12, 17), (6, 18)]), m, trait=0)
    t.p(P([(14, 10), (19, 8), (16, 13)]), m, trait=0, dl=-1)
    t.px([(5, 16), (5, 17)], "o")
    t.px([(10, 12), (11, 12)], "y", 3); t.px((10, 12), "o")
    t.px([(13, 20), (14, 22), (15, 25)], v, 2)


@sprite("cerf-blanc", "Cerf blanc")
def cerf_blanc(t):
    t.mat("b", "dedad2", "f0d8a0")
    t.mat("v", "f6f4ee", "fff6d8")
    t.mat("k", "b89a6a", "8a8aa8")   # bois
    t.mat("y", "6a8ad0", "d06a8a")
    _cerf(t)
    for pts in ([(13, 9), (11, 4), (7, 1)], [(11, 4), (12, 0)], [(9, 2), (5, 4)],
                [(15, 9), (18, 3), (23, 0)], [(18, 3), (16, -1)], [(20, 2), (24, 4)], [(22, 1), (26, -1)]):
        t.p(K([(x, y, 0.8) for x, y in pts]), "k", trait=0)


@sprite("cerf-candelabre", "Cerf-candélabre")
def cerf_candelabre(t):
    t.mat("b", "d6d2cc", "3a3a4a")
    t.mat("v", "f2f0ea", "5a5a6a")
    t.mat("k", OR, ACIER)            # bois en bras de chandelier
    t.mat("c", "f0e8d0", "f0e8d0")   # cierges
    t.mat("f", ["3a5a9a", "6a9ad8", "a8d0f8", "e0f2ff", "ffffff"], FEU)   # flammes pales
    t.mat("y", "6a8ad0", "d06a8a")
    _cerf(t)
    branches = ([(13, 9), (10, 6), (5, 6), (5, 3)], [(13, 9), (12, 4), (10, 2), (10, -1)],
                [(15, 9), (17, 5), (16, 1), (16, -2)], [(15, 9), (20, 6), (22, 3), (22, 0)],
                [(15, 9), (22, 8), (27, 6), (27, 3)])
    for pts in branches:
        t.p(CS(pts[:3]), "k", trait=0)
        x, y = pts[3]
        t.p(R(x - 1, y - 1, x + 1, y + 3), "c", trait=0)
        t.p(R(x - 2, y + 3, x + 2, y + 3), "k", trait=0, ton=3)
        t.p(P([(x - 1, y - 2), (x, y - 6), (x + 1, y - 2)]) | R(x - 1, y - 3, x + 1, y - 2), "f", trait=0, ton=2)
        t.px([(x, y - 3), (x, y - 2)], "f", 4)
    # cire qui coule sur l'encolure, lueur
    t.px([(12, 12), (13, 14), (16, 15), (17, 18)], "c", 2)
    t.px([(2, 8), (30, 9), (8, -5), (24, -6)], "f", 3)
