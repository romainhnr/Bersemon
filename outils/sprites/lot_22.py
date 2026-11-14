from dessin import *
from matieres import *
from atelier import sprite
import math

MER = ["1c3a6a", "2e5c98", "4a88c8", "84bce8", "d0ecfc"]


@sprite("sirene-d-ecume", "Sirène d'écume")
def sirene_d_ecume(t):
    t.mat("c", PEAU)
    t.mat("e", ["1c6a6a", "30a0a0", "58d0c8", "a0f0e4", "e0fff8"], ["6a1c5a", "a03090", "d058c0", "f0a0e0", "ffe0f8"])   # ecailles nacrees
    t.mat("h", ["a87a10", "e0b020", "f8dc50", "fff4a0", "ffffe8"], ["8a8ab0", "b0b0d0", "d0d0ea", "eaeaf8", "ffffff"])   # cheveux d'or
    t.mat("o2", MER)
    t.mat("w", ["a8c8e0", "c8e0f0", "e0f0fa", "f2fbff", "ffffff"])   # ecume
    # la vague et son ecume
    t.p(B([(0, 40), (10, 36), (22, 39), (36, 35), (47, 40), (47, 46), (0, 46)]), "o2", trait=0)
    for x, y in ((6, 37), (10, 35), (14, 37), (32, 35), (36, 33), (40, 35), (44, 38)):
        t.p(E(x, y, 2.2, 1.6), "w", trait=0)
    # queue relevee, nageoire en voile
    t.p(S([(24, 30, 4.4), (30, 36, 4), (37, 34, 3), (41, 26, 2), (40, 19, 1.2)]), "e", trait=0)
    t.p(B([(40, 20), (34, 12), (40, 15), (46, 10), (44, 20)]), "e", trait=0, dl=1)
    for x, y in ((27, 33), (31, 36), (35, 35), (39, 30)):
        t.px([(x, y), (x + 1, y)], "e", 4)
    # buste, bras leve qui tient un coquillage a l'oreille
    t.p(B([(19, 19), (24, 17), (29, 19), (29, 27), (26, 32), (21, 32), (19, 27)]), "c", trait=0)
    t.p(R(20, 22, 28, 23), "e", trait=0, ton=3)
    t.p(K([(28, 20, 1.5), (33, 16, 1.2), (31, 11, 1.1)]), "c", trait=1, dl=-1)
    t.p(E(30.5, 9, 2.2, 2), "w", trait=0, ton=3)
    t.p(K([(20, 20, 1.5), (15, 26, 1.2), (17, 31, 1.1)]), "c", trait=1)
    # tete, longue chevelure d'or, etoile de mer
    t.p(B([(18, 12), (21, 5), (27, 5), (30, 12), (33, 28), (28, 19), (20, 19), (14, 30)]), "h", trait=0)
    t.p(E(24, 12.5, 3.6, 4.2), "c", trait=0)
    t.px([(22, 12), (26, 12)], "e", 1); t.px([(22, 11), (26, 11)], "o")
    t.px([(23, 15), (24, 15), (25, 15)], "c", 0)
    t.px([(20, 6), (19, 7), (21, 7), (20, 8), (20, 7)], "w", 4)
    t.px([(4, 28), (10, 22), (44, 28)], "w", 4)


@sprite("sphinx-tete-de-mort", "Sphinx tête-de-mort")
def sphinx_tete_de_mort(t):
    t.mat("a", ["2a2018", "4a3a28", "70583c", "9c8058", "c8ac80"], ["18202a", "283a4a", "3c5870", "58809c", "80acc8"])   # ailes brunes
    t.mat("j", ["8a5a08", "d09010", "f8c830", "ffe880", "fffcd8"], FEU_VIOLET)   # bandes jaunes
    t.mat("k", OS)
    t.mat("n", "241c20")
    for cote in (1, -1):
        c = lambda x: 24 + cote * x
        haut = B([(c(3), 18), (c(9), 8), (c(19), 5), (c(23), 12), (c(19), 22), (c(9), 24)])
        bas = B([(c(3), 25), (c(11), 25), (c(17), 31), (c(13), 39), (c(6), 38), (c(3), 32)])
        t.p(bas, "j", trait=0, dl=0 if cote < 0 else -1)
        t.p(haut, "a", trait=0, dl=0 if cote < 0 else -1)
        for a, b in (((c(5), 18), (c(20), 9)), ((c(6), 21), (c(21), 15)), ((c(8), 23), (c(19), 21))):
            t.p(L(a[0], a[1], b[0], b[1]), "a", trait=0, ton=0)
        t.p(L(c(5), 30, c(15), 34) | L(c(5), 34, c(13), 38), "n", trait=0, ton=2)
    # abdomen raye
    t.p(E(24, 33, 3.6, 10), "j", trait=0)
    for y in (27, 31, 35, 39):
        t.p(R(21, y, 27, y + 1), "n", trait=0, ton=2, sur="j")
    # thorax velu : le crane dessine dans le poil
    t.p(E(24, 19, 5.6, 6.4), "a", trait=0, dl=1)
    t.p(E(24, 18, 3.6, 3.6), "k", trait=0)
    t.p(R(22, 21, 26, 23), "k", trait=0)
    t.px([(22, 18), (23, 18), (25, 18), (26, 18)], "o")
    t.px((24, 20), "o"); t.px([(23, 23), (25, 23)], "o")
    # tete, antennes, trompe enroulee
    t.p(E(24, 11, 3.4, 3), "n", trait=0, ton=3)
    t.px([(22, 10), (26, 10)], "j", 3)
    t.p(CS([(22, 9), (19, 4), (16, 2)]) | CS([(26, 9), (29, 4), (32, 2)]), "n", trait=0, ton=3)
    t.px([(24, 13), (24, 14)], "j", 1)


@sprite("lievre-somnambule", "Lièvre somnambule")
def lievre(t):
    t.mat("f", "a8927c", "8c94ac")   # pelage
    t.mat("v", "e8dcc8", "dce0ec")   # ventre
    t.mat("r", "d09a9a", "a0b0d0")   # interieur des oreilles
    t.mat("p", FEU_VIOLET, FEU_VERT) # reve
    t.mat("b", "f0f0f8")             # bonnet de nuit
    # debout sur ses pattes arriere, bras tendus du dormeur
    t.p(K([(29, 36, 3.4), (32, 42, 2.4), (27, 45, 1.8)]), "f", dl=-1, trait=0)
    t.p(E(34, 36, 2.6, 2.6), "v", trait=0)
    t.p(B([(17, 22), (24, 19), (31, 23), (33, 33), (30, 41), (20, 41), (17, 33)]), "f", trait=0)
    t.p(B([(19, 26), (24, 24), (27, 30), (25, 39), (20, 38)]), "v", trait=0)
    t.p(K([(22, 38, 3.6), (19, 43, 2.6), (13, 45, 2)]), "f")
    t.p(K([(20, 25, 1.9), (13, 25, 1.6), (6, 24, 1.4)]), "f", trait=1)
    t.p(K([(24, 27, 1.9), (17, 29, 1.6), (10, 29, 1.4)]), "f", trait=1)
    # tete, yeux fermes, bonnet de nuit a pompon
    t.p(B([(11, 15), (15, 9), (23, 9), (27, 14), (25, 21), (17, 22), (12, 20)]), "f", trait=0)
    t.p(B([(9, 17), (13, 15), (16, 18), (14, 22), (10, 21)]), "v", trait=0)
    t.px((9, 17), "r", 0)
    t.p(LS([(15, 14), (17, 15), (19, 14)]), "o", trait=0, ton=2)
    t.p(P([(18, 10), (24, -4), (22, 10)]), "f", trait=0)
    t.p(P([(19, 9), (23, -1), (21, 9)]), "r", trait=0)
    t.p(P([(22, 10), (31, -1), (26, 12)]), "f", trait=0, dl=-1)
    t.p(B([(17, 10), (25, 8), (30, 10), (37, 16), (33, 16), (27, 13), (18, 12)]), "b", trait=0)
    t.p(E(37, 18, 1.8, 1.8), "p", trait=0, ton=3)
    # le reve qui le mene : une spirale, des Z
    for i in range(14):
        a = i * 0.7
        t.px((38 + (1 + i * 0.35) * math.cos(a), 6 + (1 + i * 0.3) * math.sin(a)), "p", 3 if i % 2 else 4)
    t.p(LS([(2, 8), (6, 8), (2, 12), (6, 12)]), "p", trait=0, ton=3)
    t.p(LS([(8, 2), (11, 2), (8, 5), (11, 5)]), "p", trait=0, ton=2)


@sprite("pavot-carnivore", "Pavot carnivore")
def pavot(t):
    t.mat("p", ["2a0a2a", "501850", "7c2c7a", "aa5ca8", "d8a0d4"], ["2a1a0a", "503418", "7c542c", "aa845c", "d8bca0"])   # petales noirs-violets
    t.mat("v", "4a6a44", "6a446a")
    t.mat("k", "1c1014")
    t.mat("w", OS)
    t.mat("s", ["8a8ab0", "b0b0d0", "d0d0ea", "eaeaf8", "ffffff"])   # pollen du sommeil
    # tige velue, feuilles decoupees, capsule
    t.p(S([(24, 46, 2.2), (22, 38, 2), (25, 30, 1.8), (24, 24, 1.6)]), "v", trait=0)
    for pts in ([(23, 40), (14, 37), (6, 39), (4, 35), (13, 33)], [(24, 38), (33, 34), (41, 36), (43, 32), (34, 30)]):
        t.p(B(pts), "v", trait=0, dl=-(pts[1][0] > 24))
    t.px([(9, 36), (38, 33), (12, 35), (36, 32)], "v", 0)
    t.p(S([(25, 34, 1), (32, 28, 0.9), (35, 22, 0.8)]), "v", trait=0)
    t.p(E(35, 19, 2.8, 3.2), "v", trait=0, dl=1)
    t.p(R(33, 15, 37, 16), "v", trait=0, ton=0)
    # la fleur : quatre grands petales froisses, ouverts sur une gueule
    for x, y, rx, ry, dl in ((14, 13, 8, 7, 0), (33, 12, 8, 7, -1), (17, 22, 7, 5, -1), (30, 22, 7, 5, -1), (24, 8, 7, 6, 0)):
        t.p(E(x, y, rx, ry), "p", trait=0, dl=dl)
    t.p(E(24, 16, 6, 5.4), "k", trait=0, ton=2)
    for a in range(0, 360, 36):
        x, y = 24 + 5 * math.cos(math.radians(a)), 16 + 4.4 * math.sin(math.radians(a))
        t.p(L(x, y, 24 + 2.6 * math.cos(math.radians(a)), 16 + 2.4 * math.sin(math.radians(a))), "w", trait=0, ton=4)
    t.p(E(24, 16, 1.6, 1.4), "p", trait=0, ton=4)
    t.px([(8, 10), (11, 16), (38, 8), (40, 15), (22, 4)], "p", 4)
    # pollen
    for x, y in ((6, 24), (42, 26), (3, 14), (45, 6), (12, 2), (30, 0)):
        t.px((x, y), "s", 3)


@sprite("marchand-de-sable", "Marchand de sable")
def marchand_de_sable(t):
    t.mat("r", ["101838", "202e64", "344a94", "6078c0", "a0b4e8"], ["381018", "64202e", "94344a", "c06078", "e8a0b4"])   # manteau de nuit
    t.mat("e", ["a87a10", "e0b020", "f8dc50", "fff4a0", "ffffe8"], ["8a8ab0", "b0b0d0", "d0d0ea", "eaeaf8", "ffffff"])   # etoiles, sable
    t.mat("s", SPECTRE, FEU_VIOLET)
    t.mat("t", "8a7454", "546a8a")   # sac de toile
    t.mat("k", "0c0c1c")
    # manteau en cloche, sans pieds
    t.p(B([(15, 18), (24, 14), (33, 18), (37, 32), (35, 43), (30, 46), (27, 41), (24, 46), (21, 41), (17, 46), (12, 43), (11, 32)]),
        "r", trait=0)
    for x, y in ((16, 26), (22, 31), (30, 26), (33, 36), (19, 39), (27, 37), (14, 34), (24, 22)):
        t.px((x, y), "e", 4)
    for x, y in ((20, 25), (28, 32), (17, 42)):
        t.px([(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)], "e", 3)
    # sac de sable a l'epaule, qui fuit
    t.p(B([(31, 14), (37, 10), (44, 13), (45, 22), (40, 27), (33, 24)]), "t", trait=1)
    t.p(R(35, 9, 38, 11), "t", trait=0, dl=-1)
    t.px([(36, 16), (40, 19), (42, 15)], "t", 0)
    for x, y in ((42, 28), (43, 31), (41, 34), (44, 37), (42, 41), (43, 45)):
        t.px((x, y), "e", 3)
    # main tendue qui laisse couler le sable
    t.p(K([(17, 21, 2.4), (10, 23, 2), (5, 21, 1.7)]), "r", trait=1)
    t.p(E(4, 20, 1.8, 1.8), "s", trait=0)
    for x, y in ((3, 23), (4, 26), (3, 29), (5, 32), (4, 36), (3, 40), (5, 43)):
        t.px((x, y), "e", 3 if y % 2 else 4)
    # capuchon pointu : dedans, un croissant de lune pour visage
    t.p(B([(16, 18), (17, 8), (22, 2), (26, -2), (27, 5), (31, 9), (32, 18), (27, 20), (21, 20)]), "r", trait=0, dl=1)
    t.p(E(24, 12, 4.2, 5), "k", trait=0, ton=2)
    t.p(E(24, 12, 3.4, 4) - E(26, 11, 3.2, 3.8), "e", trait=0, ton=3)
    t.px((26, 12), "e", 4)
    t.px((26, -3), "e", 4)


@sprite("belier-de-siege", "Bélier de siège")
def belier_de_siege(t):
    t.mat("b", BOIS)
    t.mat("a", ACIER)
    t.mat("g", PIERRE)
    t.mat("y", FEU, FEU_VERT)
    t.mat("c", "b89a64", "d8d0c0")
    # le chassis roulant
    t.p(R(8, 34, 42, 38), "b", trait=0, dl=-1)
    for x in (13, 36):
        t.p(E(x, 41, 5, 5), "b", trait=0)
        t.p(E(x, 41, 5, 5) - E(x, 41, 3.8, 3.8), "a", trait=0, ton=1)
        t.p(R(x - 4, 41, x + 4, 41) | R(x, 37, x, 45), "b", trait=0, ton=0)
        t.px((x, 41), "a", 4)
    # montants et toit de peaux
    t.p(R(12, 12, 14, 34) | R(35, 12, 37, 34), "b", trait=0)
    t.p(P([(8, 13), (24, 4), (41, 13), (41, 16), (24, 8), (8, 16)]), "g", trait=0, dl=-1)
    t.px([(14, 11), (20, 8), (28, 8), (34, 11)], "g", 4)
    # les chaines et la poutre suspendue
    for x in (17, 31):
        t.p(R(x, 13, x, 22), "a", trait=0, ton=3)
        t.px([(x, 15), (x, 18), (x, 21)], "a", 1)
    t.p(R(8, 22, 44, 28), "b", trait=0)
    t.px([(12, 23), (20, 23), (28, 23), (36, 23)], "b", 4)
    t.px([(16, 27), (24, 26), (32, 27), (40, 26)], "b", 0)
    t.p(R(14, 22, 15, 28) | R(33, 22, 34, 28), "a", trait=0, ton=1)
    # la tete de belier en fer, vivante
    t.p(B([(0, 23), (3, 18), (10, 18), (12, 22), (12, 29), (8, 32), (2, 31)]), "a", trait=1)
    t.p(E(9, 21, 4, 4) - E(8, 22, 2, 2) - R(2, 22, 7, 27), "a", trait=1, dl=1)
    t.px([(4, 23), (5, 23)], "y", 3); t.px((4, 23), "y", 4)
    t.px([(1, 27), (2, 28), (3, 28)], "o")
    t.px([(1, 30), (0, 33)], "y", 2)
    t.grain("g", 0.1, 16)


@sprite("sentinelle-de-pierre", "Sentinelle de pierre")
def sentinelle(t):
    t.mat("g", ["504c48", "76726c", "9c9892", "c2beb6", "e4e0d8"], ["48504c", "6c7672", "929c98", "b6c2be", "d8e4e0"])
    t.mat("y", FEU_BLEU, FEU)
    t.mat("l", "5a8a4a", "8a6aa8")
    # hallebarde de pierre
    t.p(R(39, 4, 40, 46), "g", trait=0, dl=-1)
    t.p(B([(40, 6), (46, 4), (47, 12), (43, 16), (40, 14)]), "g", trait=0, dl=1)
    t.p(P([(38, 4), (39, -2), (41, 4)]), "g", trait=0, ton=4)
    # jambes soudees au socle
    t.p(R(26, 34, 32, 43), "g", trait=0, dl=-1); t.p(R(17, 34, 23, 43), "g", trait=0)
    t.p(R(13, 43, 36, 46), "g", trait=0, dl=-1)
    # corps de statue de garde : cotte d'ecailles sculptee
    t.p(B([(15, 16), (24, 13), (34, 16), (35, 28), (33, 36), (16, 36), (14, 28)]), "g", trait=0)
    for y in (20, 24, 28, 32):
        for x in range(17 + (y % 8) // 4 * 2, 33, 4):
            t.p(CS([(x, y), (x + 2, y + 2), (x + 4, y)]), "g", trait=0, ton=1)
    # bras : l'un tient la hallebarde, l'autre un bouclier rond
    t.p(K([(34, 18, 2.6), (38, 23, 2.4), (39, 27, 2.2)]), "g", trait=1)
    t.p(K([(15, 18, 2.6), (11, 24, 2.4)]), "g", trait=1)
    t.p(E(9, 27, 7, 7.6), "g", trait=1, dl=1)
    t.p(E(9, 27, 7, 7.6) - E(9, 27, 5.4, 6), "g", trait=0, ton=1)
    t.p(E(9, 27, 2, 2.2), "g", trait=0, ton=4)
    # tete : casque a nasal, visage use, yeux bleus
    t.p(B([(19, 14), (20, 5), (24, 2), (29, 5), (30, 14)]), "g", trait=0, dl=1)
    t.p(R(24, 6, 25, 13), "g", trait=0, ton=4)
    t.px([(21, 9), (22, 9), (27, 9), (28, 9)], "y", 3)
    t.px([(22, 12), (27, 12)], "g", 0)
    t.px([(14, 44), (15, 43), (34, 44), (17, 17), (33, 17), (5, 31)], "l", 2)
    t.grain("g", 0.07, 17)


@sprite("baliste-hantee", "Baliste hantée")
def baliste(t):
    t.mat("b", BOIS)
    t.mat("a", ACIER)
    t.mat("c", "b89a64", "d8d0c0")
    t.mat("s", SPECTRE, FEU_VIOLET)
    # trepied
    t.p(K([(24, 28, 1.6), (12, 45, 1.3)]) | K([(24, 28, 1.6), (36, 45, 1.3)]) | K([(24, 28, 1.6), (26, 45, 1.3)]), "b", trait=0, dl=-1)
    t.p(R(10, 45, 14, 46) | R(34, 45, 38, 46) | R(24, 45, 28, 46), "a", trait=0, ton=1)
    # l'arbrier, pointe vers la gauche
    t.p(P([(4, 20), (42, 22), (42, 27), (4, 25)]), "b", trait=0)
    t.px([(10, 22), (18, 22), (26, 23), (34, 23)], "b", 4)
    t.p(R(36, 21, 39, 28), "a", trait=0, ton=1)
    # l'arc, grand ouvert, et sa corde
    t.p(CS([(14, 4), (8, 12), (10, 22)]) | CS([(15, 4), (9, 12), (11, 22)]), "a", trait=0)
    t.p(CS([(14, 42), (8, 34), (10, 25)]) | CS([(15, 42), (9, 34), (11, 25)]), "a", trait=0, dl=-1)
    t.p(L(14, 4, 34, 23) | L(14, 42, 34, 24), "c", trait=0, ton=3)
    # le carreau, pret a partir
    t.p(R(2, 22, 34, 23), "a", trait=0, ton=4)
    t.p(P([(-3, 22), (3, 19), (3, 26)]), "a", trait=0, ton=3)
    # le treuil tourne tout seul
    t.p(E(41, 31, 3, 3), "b", trait=0, dl=1)
    for a in range(0, 360, 90):
        t.p(L(41, 31, 41 + 5 * math.cos(math.radians(a + 20)), 31 + 5 * math.sin(math.radians(a + 20))), "b", trait=0, ton=3)
    # le servant mort qui la hante : un buste de brume, mains sur le treuil
    t.p(B([(32, 16), (36, 8), (42, 8), (45, 16), (44, 22), (34, 21)]), "s", trait=0, ton=1)
    t.p(E(39, 11, 3.4, 3.6), "s", trait=0)
    t.px([(38, 11), (40, 11)], "o"); t.px((39, 13), "o")
    t.p(K([(42, 17, 1.4), (44, 24, 1.1), (42, 29, 1)]), "s", trait=0, ton=2)
    t.px([(30, 10), (46, 6), (33, 4)], "s", 4)
