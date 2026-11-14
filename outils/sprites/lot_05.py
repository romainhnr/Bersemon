from dessin import *
from matieres import *
from atelier import sprite
import math


@sprite("vipere", "Vipère", zoom=0.94)
def vipere(t):
    t.mat("e", "7a7446", "8a4a6a")   # ecailles
    t.mat("z", "3a3424", "2a1a3a")   # zigzag du dos
    t.mat("v", "d8cfa0", "f0d8e0")   # ventre
    t.mat("y", "e8a020", "60e0a0")
    t.mat("r", SANG)
    # anneaux empiles
    t.p(E(27, 40, 16, 4.6), "e", trait=0)
    t.p(E(27, 34, 13, 4.4), "e", trait=1)
    t.p(E(26, 28.5, 9.5, 4), "e", trait=1)
    t.p(S([(40, 41, 2.2), (45, 39, 1.4), (46, 35, 0.7)]), "e", trait=0)
    # cou dresse en S, tete triangulaire
    t.p(S([(24, 27, 3.6), (19, 22, 3.2), (20, 15, 2.9), (15, 10, 2.8)]), "e", trait=1)
    t.p(S([(23, 27, 1.4), (17, 22, 1.3), (18, 15, 1.2), (13, 11, 1.1)]), "v", trait=0, sur="e")
    t.p(B([(3, 11), (8, 5), (15, 5), (18, 9), (16, 14), (9, 14)]), "e", trait=0)
    # zigzag
    for x, y in ((14, 39), (18, 41), (22, 39), (26, 41), (30, 39), (34, 41), (38, 39), (17, 33), (21, 35), (25, 33),
                 (29, 35), (33, 33), (37, 35), (20, 28), (24, 30), (28, 28), (32, 30)):
        t.p(P([(x - 2, y), (x, y - 2), (x + 2, y), (x, y + 1)]), "z", trait=0, ton=2, sur="e")
    t.px([(22, 21), (23, 20), (22, 17), (23, 16), (19, 11), (18, 12)], "z", 2)
    t.px([(10, 7), (11, 7)], "y", 3); t.px((10, 7), "o"); t.px([(9, 6), (10, 6), (11, 6), (12, 6)], "z", 2)
    t.px([(4, 12), (5, 12), (6, 13), (7, 13), (8, 13)], "o")
    t.px([(2, 13), (1, 14), (0, 13), (0, 15)], "r", 2)
    t.px((5, 9), "o")


@sprite("tentateur", "Tentateur")
def tentateur(t):
    t.mat("e", "3f6a3c", "6a3a7a")
    t.mat("z", "1e3a22", "2a1438")
    t.mat("v", "c8d890", "f0d0f0")
    t.mat("y", "f0d020", "f04848")
    t.mat("p", ["5a0a12", "961420", "d02c2c", "f06a50", "ffd0b0"], OR)   # la pomme
    t.mat("l", "4f9a3c", "3a9aa8")
    t.mat("b", BOIS)
    t.mat("r", SANG)
    # la pomme, lourde et luisante
    t.p(B([(12, 28), (17, 23), (24, 25), (31, 23), (36, 28), (36, 37), (30, 44), (24, 43), (18, 44), (12, 37)]), "p", trait=0)
    t.px([(16, 27), (17, 26), (16, 29), (15, 31)], "p", 4)
    t.p(C(24, 25, 25, 20, 0.9), "b", trait=0)
    t.p(B([(25, 21), (30, 17), (34, 19), (30, 23)]), "l", trait=0)
    # le serpent l'enlace
    t.p(S([(44, 44, 1), (40, 40, 2.4), (33, 36, 3), (24, 35, 3.1), (15, 37, 3), (10, 33, 2.9)]), "e", trait=1)
    t.p(S([(10, 33, 2.9), (8, 27, 2.8), (12, 21, 2.8), (19, 20, 2.7), (22, 15, 2.6), (19, 10, 2.5)]), "e", trait=1)
    t.p(S([(10, 33, 1.1), (7, 27, 1.1), (11, 20, 1.1), (18, 19, 1), (21, 15, 1), (18, 11, 1)]), "v", trait=0, sur="e")
    for x, y in ((38, 39), (31, 35), (24, 34), (17, 36), (11, 30), (13, 22), (20, 18)):
        t.px([(x, y), (x + 1, y), (x, y + 1)], "z", 2)
    # tete penchee, qui offre le fruit
    t.p(B([(6, 10), (11, 4), (18, 4), (21, 8), (19, 13), (12, 13)]), "e", trait=0)
    t.px([(13, 6), (14, 6)], "y", 3); t.px((13, 6), "o"); t.px([(12, 5), (13, 5), (14, 5), (15, 5)], "z", 2)
    t.px([(7, 11), (8, 11), (9, 12), (10, 12), (11, 12)], "o")
    t.px([(5, 12), (4, 13), (3, 12), (3, 14)], "r", 2)
    # la morsure dans la pomme
    t.p(E(33, 28, 2.6, 2.6), "v", trait=0, ton=3, sur="p")
    t.px([(32, 27), (34, 29)], "p", 1)


@sprite("gorgone", "Gorgone", zoom=0.89)
def gorgone(t):
    t.mat("e", "5a7a4a", "7a5a8a")   # ecailles de la queue
    t.mat("c", ["4a5a4a", "7a8a72", "a8b89a", "ccd8bc", "eef4e0"], PIERRE)   # peau
    t.mat("s", "3f8a3c", "c8a030")   # serpents de la chevelure
    t.mat("y", "f0e040", "f04848")
    t.mat("g", PIERRE)
    t.mat("r", SANG)
    # queue de serpent lovee
    t.p(S([(24, 30, 4.4), (30, 36, 4.4), (36, 41, 3.8), (28, 44, 3.4), (16, 43, 3), (8, 40, 2.2), (4, 35, 1)]), "e", trait=0)
    for x, y in ((28, 34), (33, 39), (30, 43), (22, 44), (14, 42), (9, 39)):
        t.px([(x, y), (x + 1, y + 1)], "e", 0)
    t.px([(27, 32), (32, 37), (20, 42)], "e", 4)
    # buste
    t.p(B([(19, 20), (24, 18), (29, 20), (30, 28), (27, 33), (21, 33), (18, 28)]), "c", trait=0)
    t.p(R(20, 22, 28, 24), "s", trait=0, ton=1)
    # bras : une main levee, changee en pierre
    t.p(K([(20, 21, 1.9), (13, 25, 1.6), (9, 20, 1.4)]), "c", trait=1)
    t.p(E(8, 18, 2.2, 2.4), "g", trait=0)
    t.px([(7, 15), (9, 15), (6, 17)], "g", 3)
    t.p(K([(28, 21, 1.9), (34, 26, 1.6), (37, 31, 1.4)]), "c", trait=1, dl=-1)
    # tete, serpents dresses
    for x0, x1, y1, r in ((20, 12, 5, 1.5), (22, 18, 1, 1.5), (25, 25, 0, 1.5), (27, 32, 2, 1.5), (29, 37, 7, 1.5),
                          (19, 10, 11, 1.4), (30, 39, 14, 1.4)):
        t.p(S([(x0, 11, r), ((x0 + x1) / 2 + 1, (11 + y1) / 2 - 1, r - 0.2), (x1, y1, r - 0.5)]), "s", trait=0,
            dl=-1 if x1 % 2 else 0)
        t.px((x1, y1), "y", 3)
    t.p(E(24.5, 13, 4.4, 4.8), "c", trait=0)
    t.px([(22, 12), (23, 12), (26, 12), (27, 12)], "y", 4)
    t.px([(22, 11), (23, 11), (26, 11), (27, 11)], "o")
    t.px([(23, 16), (24, 16), (25, 16), (26, 16)], "r", 1)
    t.px([(23, 17), (26, 17)], "c", 4)


@sprite("feu-follet", "Feu follet")
def feu_follet(t):
    t.mat("f", ["1a4a6a", "2a8ab0", "50c8e0", "a8f0f0", "f0ffff"], FEU_VIOLET)
    t.mat("k", "10202c", "1a1030")
    t.p(B([(24, 4), (29, 13), (35, 21), (37, 31), (32, 40), (24, 43), (16, 40), (11, 31), (13, 21), (19, 13)]), "f", trait=0, ton=1)
    t.p(B([(24, 11), (28, 18), (33, 25), (33, 33), (29, 39), (24, 41), (19, 39), (15, 33), (15, 25), (20, 18)]), "f", trait=0, ton=2)
    t.p(B([(24, 20), (29, 27), (29, 34), (24, 39), (19, 34), (19, 27)]), "f", trait=0, ton=3)
    t.p(E(24, 34, 3.4, 3.4), "f", trait=0, ton=4)
    # langues de flamme
    t.p(P([(14, 22), (9, 13), (17, 17)]) | P([(34, 22), (39, 14), (31, 17)]), "f", trait=0, ton=1)
    t.p(P([(24, 5), (22, 0), (26, 2)]), "f", trait=0, ton=2)
    # regard vide, bouche en soupir
    t.p(E(20, 28, 1.8, 2.4) | E(28, 28, 1.8, 2.4), "k", trait=0, ton=2)
    t.px([(20, 27), (28, 27)], "f", 4)
    t.p(E(24, 35, 1.6, 2), "k", trait=0, ton=2)
    # etincelles
    t.px([(7, 30), (40, 27), (9, 38), (41, 37), (6, 20), (42, 18)], "f", 3)
    t.px([(8, 31), (41, 28)], "f", 2)


@sprite("lanterne-des-morts", "Lanterne des morts")
def lanterne_des_morts(t):
    t.mat("n", FER_NOIR)
    t.mat("f", ["1a4a6a", "2a8ab0", "50c8e0", "a8f0f0", "f0ffff"], FEU_VIOLET)
    t.mat("k", "10202c", "1a1030")
    t.mat("g", OR)
    # chaine et anneau
    t.p(E(24, 3, 2.6, 2.6) - E(24, 3, 1.2, 1.2), "n", trait=0, ton=3)
    t.px([(24, 6), (24, 7), (23, 8), (25, 8), (24, 9)], "n", 3)
    # toit, cage de fer, socle
    t.p(P([(13, 16), (24, 9), (35, 16)]), "n", trait=0)
    t.p(R(12, 16, 36, 18), "n", trait=0, ton=3)
    t.p(R(15, 19, 33, 38), "f", trait=0, ton=0)
    t.p(R(12, 38, 36, 41), "n", trait=0)
    t.p(R(15, 42, 33, 43), "n", trait=0, dl=-1)
    # le feu follet prisonnier
    t.p(B([(24, 20), (29, 27), (31, 33), (27, 37), (21, 37), (17, 33), (19, 27)]), "f", trait=0, ton=2)
    t.p(B([(24, 25), (27, 30), (27, 35), (21, 35), (21, 30)]), "f", trait=0, ton=3)
    t.px([(24, 34), (24, 35)], "f", 4)
    t.px([(21, 30), (22, 30), (26, 30), (27, 30)], "k", 2)
    t.px([(23, 33), (24, 33), (25, 33)], "k", 2)
    # barreaux
    for x in (15, 21, 27, 33):
        t.p(R(x, 19, x, 38), "n", trait=0, ton=2 if x in (15, 33) else 3)
    t.px([(24, 12), (22, 14), (26, 14)], "g", 3)
    t.px([(13, 39), (35, 39), (24, 39)], "g", 3)
    # flammes qui s'echappent entre les barreaux
    t.p(P([(11, 30), (8, 24), (13, 26)]) | P([(37, 28), (41, 21), (36, 24)]), "f", trait=0, ton=2)
    t.px([(9, 26), (40, 23), (6, 34), (42, 33)], "f", 4)


def _heaume(t, cx, cy, a="a", y="y", cimier=None):
    """Heaume ferme, fente des yeux eclairee de l'interieur, tourne a gauche."""
    t.p(B([(cx - 5, cy - 2), (cx - 3, cy - 6), (cx + 3, cy - 6), (cx + 5, cy - 2), (cx + 5, cy + 5), (cx - 5, cy + 5)]), a, trait=0)
    t.p(R(cx - 5, cy - 1, cx + 2, cy), "o", trait=0, ton=2)
    t.px([(cx - 4, cy - 1), (cx - 3, cy - 1), (cx, cy - 1), (cx + 1, cy - 1)], y, 3)
    t.px([(cx - 3, cy + 2), (cx - 1, cy + 2), (cx - 3, cy + 4), (cx - 1, cy + 4)], a, 0)
    t.px([(cx - 2, cy - 5), (cx - 1, cy - 5)], a, 4)


@sprite("armure-hantee", "Armure hantée", zoom=0.96)
def armure_hantee(t):
    t.mat("a", ACIER)
    t.mat("y", FEU_VIOLET, FEU_VERT)
    t.mat("l", BOIS)
    t.mat("c", "6a2a3a", "2a4a6a")   # etoffe
    # epee plantee, tenue a deux mains
    t.p(R(8, 12, 9, 44), "a", trait=0, ton=3)
    t.px([(9, y) for y in range(13, 44)], "a", 1)
    t.p(R(5, 22, 12, 23), "a", trait=0, ton=1)
    t.p(R(8, 8, 9, 11), "l", trait=0)
    # jambieres, vides aux genoux
    for x, dl in ((27, -1), (20, 0)):
        t.p(R(x - 2, 31, x + 2, 36), "a", trait=0, dl=dl)
        t.p(R(x - 2, 38, x + 2, 43), "a", trait=0, dl=dl)
        t.p(R(x - 4, 44, x + 2, 45), "a", trait=0, dl=dl)
    # tassette et plastron
    t.p(B([(16, 26), (31, 26), (33, 31), (15, 31)]), "c", trait=0)
    t.p(B([(16, 13), (24, 11), (31, 13), (32, 22), (29, 27), (19, 27), (15, 22)]), "a", trait=0)
    t.px([(23, 14), (23, 16), (23, 18), (23, 20), (23, 22), (23, 24)], "a", 1)
    # spalieres et bras
    t.p(E(31, 14, 3.6, 3), "a", trait=0, dl=-1)
    t.p(K([(31, 16, 2), (33, 23, 1.7)]), "a", trait=0, dl=-1)
    t.p(E(16, 14, 3.8, 3.2), "a", trait=1)
    t.p(K([(15, 17, 2.2), (12, 21, 1.9)]), "a", trait=1)
    t.p(E(10, 22, 2.2, 2), "a", trait=1, dl=1)
    _heaume(t, 23, 6)
    t.p(P([(24, 0), (30, -2), (28, 3)]), "c", trait=0)
    # l'esprit qui l'habite fuit par les jointures
    t.px([(20, 37), (27, 37), (22, 29), (34, 26), (26, 12)], "y", 3)
    t.px([(19, 37), (28, 37), (35, 25), (14, 24)], "y", 2)


@sprite("gisant", "Gisant", zoom=0.95)
def gisant(t):
    t.mat("g", ["3a4440", "5c6a62", "84948a", "aebcb0", "d8e2d8"], ["5a3a2a", "8a5a3a", "b8845a", "d8b088", "f4dcc0"])   # bronze et pierre
    t.mat("m", "5a8a4a", "8a6aa8")   # mousse
    t.mat("y", SPECTRE, FEU_VIOLET)
    # la dalle du tombeau, portee en pavois
    t.p(P([(2, 12), (16, 10), (17, 44), (3, 45)]), "g", trait=0, dl=-1)
    t.p(P([(4, 14), (14, 13), (15, 42), (5, 43)]), "g", trait=0)
    t.p(R(9, 17, 10, 38), "g", trait=0, ton=0)
    t.p(R(6, 22, 13, 23), "g", trait=0, ton=0)
    t.px([(5, 15), (13, 40), (6, 41), (12, 15)], "g", 4)
    t.px([(4, 30), (5, 31), (6, 32), (7, 33), (12, 28), (13, 29)], "g", 0)
    # jambes raides de statue
    for x, dl in ((30, -1), (23, 0)):
        t.p(R(x - 2, 31, x + 2, 43), "g", trait=0, dl=dl)
        t.p(R(x - 4, 44, x + 2, 45), "g", trait=0, dl=dl)
    # cotte longue, mains jointes en priere
    t.p(B([(18, 14), (26, 12), (34, 14), (35, 30), (33, 34), (19, 34), (17, 30)]), "g", trait=0)
    t.px([(22, 26), (22, 29), (22, 32), (27, 25), (27, 28), (27, 31), (31, 26), (31, 30)], "g", 1)
    t.p(K([(33, 16, 2.2), (30, 22, 1.9), (25, 20, 1.6)]), "g", trait=1)
    t.p(K([(19, 16, 2.2), (21, 22, 1.9), (25, 20, 1.6)]), "g", trait=1)
    t.p(P([(23, 21), (25, 15), (27, 21)]), "g", trait=0, ton=4)
    # epee au cote
    t.p(C(36, 20, 40, 42, 0.9), "g", trait=0, ton=3)
    t.p(R(34, 19, 38, 20), "g", trait=0, ton=1)
    # tete sculptee : camail, yeux clos qui s'eclairent
    t.p(B([(20, 8), (22, 2), (30, 2), (32, 8), (32, 14), (20, 14)]), "g", trait=0)
    t.p(E(25, 8, 3.2, 3.6), "g", trait=0, dl=1)
    t.px([(23, 8), (24, 8), (26, 8), (27, 8)], "y", 3)
    t.px([(24, 11), (25, 11), (26, 11)], "g", 0)
    # mousse et lichen
    t.px([(18, 15), (19, 14), (20, 14), (33, 13), (34, 15), (21, 3), (22, 2), (3, 13), (4, 12), (16, 43), (29, 44)], "m", 2)
    t.px([(19, 15), (34, 14), (5, 13)], "m", 3)


def _cheval(t, m, pas=True):
    """Cheval au pas, de profil, tourne vers la gauche."""
    t.p(K([(18, 33, 2), (17, 39, 1.4), (18, 45, 1.3)]), m, dl=-1, trait=0)
    t.p(K([(35, 33, 2.4), (38, 39, 1.5), (37, 45, 1.3)]), m, dl=-1, trait=0)
    t.p(S([(38, 26, 2), (43, 28, 2.4), (45, 35, 1.6), (44, 41, 0.8)]), m, trait=0, dl=-1)
    t.p(B([(12, 25), (18, 22), (30, 22), (39, 25), (39, 32), (32, 35), (20, 35), (13, 33)]), m, trait=0)
    t.p(K([(15, 33, 2.2), (12, 39, 1.5), (13, 45, 1.3)]), m)
    t.p(K([(33, 33, 2.8), (33, 39, 1.6), (31, 45, 1.3)]), m)
    t.p(B([(11, 28), (8, 18), (11, 12), (16, 14), (18, 24)]), m, trait=0)
    t.p(B([(3, 16), (6, 10), (12, 9), (14, 14), (10, 19), (5, 20)]), m, trait=0)
    t.p(P([(10, 10), (12, 5), (13, 10)]), m, trait=0)


@sprite("dullahan", "Dullahan", zoom=0.96)
def dullahan(t):
    t.mat("h", "34303e", "d8d4e0")   # cheval noir
    t.mat("a", FER_NOIR, ACIER)
    t.mat("c", "6a1a2a", "1a4a6a")   # cape
    t.mat("y", FEU_VERT, FEU_VIOLET)
    t.mat("k", OS)
    _cheval(t, "h")
    t.px([(7, 13), (8, 13)], "y", 3)
    t.px([(4, 18), (3, 19)], "y", 2)
    t.px([(11, 45), (12, 46), (30, 46), (17, 46), (36, 46)], "k", 1)
    # criniere
    t.px([(13, 12), (14, 14), (15, 16), (16, 19), (17, 22)], "h", 0)
    # cavalier sans tete, cape au vent
    t.p(B([(28, 10), (36, 12), (44, 9), (42, 18), (45, 24), (36, 23), (30, 20)]), "c", trait=0)
    t.p(K([(27, 22, 2.4), (24, 30, 2), (23, 35, 1.7)]), "a", trait=1)
    t.p(B([(22, 10), (27, 8), (32, 10), (32, 20), (27, 23), (22, 20)]), "a", trait=0)
    t.p(E(27, 8.5, 3.4, 1.6), "a", trait=0, ton=0)
    t.px([(26, 8), (27, 7), (28, 8)], "y", 2); t.px((27, 5), "y", 3)
    t.px([(27, 12), (27, 14), (27, 16), (27, 18)], "a", 4)
    # il porte sa tete a bout de bras
    t.p(K([(23, 12, 2), (18, 9, 1.7), (16, 5, 1.5)]), "a", trait=1)
    t.p(E(15.5, 2.5, 3.4, 3.2), "k", trait=0)
    t.px([(14, 2), (16, 2)], "y", 3); t.px([(14, 4), (15, 4), (16, 4)], "o")
    t.p(K([(31, 12, 2), (34, 18, 1.7)]), "a", trait=0, dl=-1)


@sprite("hellequin", "Hellequin", zoom=0.84)
def hellequin(t):
    t.mat("h", SPECTRE, FEU_VIOLET)  # monture spectrale
    t.mat("a", FER_NOIR, ACIER)
    t.mat("k", OS)
    t.mat("y", FEU, FEU_VERT)
    t.mat("g", OR)
    t.mat("c", "3a2a4a", "5a1a2a")
    # cheval cabre : arriere-train au sol, anterieurs leves
    t.p(S([(36, 34, 2), (42, 36, 2.6), (45, 42, 1.4)]), "h", trait=0, dl=-1)
    t.p(K([(31, 36, 2.6), (35, 41, 1.6), (33, 47, 1.3)]), "h", dl=-1, trait=0)
    t.p(B([(17, 19), (24, 20), (33, 27), (38, 34), (35, 39), (27, 38), (19, 31), (15, 24)]), "h", trait=0)
    t.p(K([(29, 37, 3), (28, 42, 1.7), (27, 47, 1.4)]), "h")
    t.p(K([(17, 26, 2.2), (10, 28, 1.5), (8, 33, 1.3)]), "h", trait=1)
    t.p(K([(18, 22, 2.2), (11, 22, 1.5), (7, 26, 1.3)]), "h", trait=0, dl=-1)
    t.p(B([(15, 24), (12, 14), (15, 8), (20, 10), (22, 20)]), "h", trait=0)
    t.p(B([(6, 12), (9, 6), (15, 5), (17, 10), (13, 15), (8, 16)]), "h", trait=0)
    t.p(P([(13, 6), (15, 1), (16, 6)]), "h", trait=0)
    t.px([(10, 9), (11, 9)], "y", 3); t.px([(7, 14), (6, 15)], "y", 2)
    # cotes a jour de la monture
    t.px([(22, 27), (24, 29), (26, 31), (28, 33), (22, 30), (24, 32), (26, 34)], "h", 0)
    # cavalier d'acier noir
    t.p(B([(30, 14), (38, 16), (45, 12), (44, 24), (37, 28), (31, 24)]), "c", trait=0)
    t.p(K([(27, 26, 2.4), (23, 32, 2), (20, 35, 1.7)]), "a", trait=1)
    t.p(B([(23, 13), (28, 11), (33, 13), (33, 23), (28, 26), (23, 23)]), "a", trait=0)
    t.px([(28, 15), (28, 17), (28, 19), (28, 21)], "a", 4)
    # heaume couronne de bois de cerf
    t.p(B([(24, 5), (26, 2), (31, 2), (33, 5), (33, 11), (24, 11)]), "a", trait=0)
    t.p(R(24, 6, 30, 7), "o", trait=0, ton=2)
    t.px([(25, 6), (26, 6), (28, 6), (29, 6)], "y", 3)
    for pts in ([(26, 2), (23, -3), (20, -5)], [(23, -3), (25, -7)], [(22, -4), (18, -2)],
                [(31, 2), (34, -3), (38, -5)], [(34, -3), (33, -7)], [(36, -4), (40, -1)]):
        t.p(K([(x, y, 0.9) for x, y in pts]), "k", trait=0)
    # il sonne du cor
    t.p(K([(24, 15, 2), (19, 13, 1.7), (17, 9, 1.5)]), "a", trait=1)
    t.p(S([(23, 8, 0.8), (19, 5, 1.2), (15, 0, 2), (12, -3, 2.6)]), "g", trait=1)
    t.px([(11, -5), (12, -5), (10, -4)], "g", 4)
