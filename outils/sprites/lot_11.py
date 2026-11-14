from dessin import *
from matieres import *
from atelier import sprite
from commun import _aile_membrane
import math


@sprite("momie-de-sel", "Momie de sel")
def momie(t):
    t.mat("b", ["6a6258", "9a9080", "c8c0ac", "e6e0d0", "fbf8ee"], ["5a6a7a", "8496a8", "b0c0d0", "d6e2ec", "f6fbff"])   # bandelettes
    t.mat("s", ["8aa0b0", "b4c8d4", "d8e6ee", "eef6fa", "ffffff"], ["b08aa8", "d4b4cc", "eed8e8", "faeef6", "ffffff"])   # cristaux de sel
    t.mat("k", "2a2420", "2a2420")
    t.mat("y", SPECTRE, FEU)
    # jambes raides
    t.p(K([(27, 31, 2.8), (28, 38, 2.4), (28, 44, 2.2)]), "b", trait=0, dl=-1)
    t.p(K([(21, 31, 2.8), (20, 38, 2.4), (19, 44, 2.2)]), "b", trait=0)
    t.p(R(15, 44, 21, 45) | R(26, 44, 32, 45), "b", trait=0)
    # torse
    t.p(B([(17, 16), (24, 14), (31, 16), (32, 26), (30, 33), (18, 33), (16, 26)]), "b", trait=0)
    # bras tendus en avant
    t.p(K([(18, 18, 2.4), (11, 20, 2), (4, 19, 1.8)]), "b", trait=1)
    t.p(K([(30, 18, 2.4), (24, 23, 2), (17, 24, 1.8)]), "b", trait=1)
    t.px([(1, 18), (1, 20), (2, 21), (14, 23), (14, 25)], "b", 3)
    # tete bandee, un oeil a nu
    t.p(E(23, 9, 5.4, 5.6), "b", trait=0)
    t.p(R(19, 8, 24, 9), "k", trait=0, ton=2)
    t.px([(21, 8), (22, 8)], "y", 3)
    # les bandes, en travers
    for y, dx in ((5, 1), (11, -1), (18, 1), (22, -1), (26, 1), (30, -1), (36, 1), (40, -1)):
        t.p(L(15, y + dx, 33, y - dx), "b", trait=0, ton=1, sur="b")
    t.px([(8, 22), (8, 24), (9, 26), (21, 27), (21, 29)], "b", 1)
    # croute de sel aux epaules et au crane
    for x, y, h in ((17, 15, 4), (20, 14, 3), (30, 15, 5), (33, 17, 3), (25, 3, 4), (21, 4, 3), (28, 31, 3)):
        t.p(P([(x - 1, y), (x, y - h), (x + 2, y)]), "s", trait=0)
    t.px([(11, 44), (35, 43), (13, 41)], "s", 4)


@sprite("ver-des-salines", "Ver des salines")
def ver_des_salines(t):
    t.mat("c", ["5a3a34", "8a5c50", "b88474", "dcb09c", "f6dccc"], ["3a4a5a", "5c748a", "849eb8", "b0c8dc", "dceaf6"])
    t.mat("s", ["8a8478", "b4ae9e", "dcd6c6", "f2eee2", "ffffff"])   # croute de sel du sol
    t.mat("w", OS)
    t.mat("r", SANG, FEU_VIOLET)
    # le sol craquele qu'il creve
    t.p(B([(2, 42), (10, 39), (24, 40), (38, 39), (46, 42), (44, 46), (4, 46)]), "s", trait=0)
    t.p(LS([(8, 42), (13, 44), (12, 46)]) | LS([(32, 41), (36, 44), (41, 43)]) | LS([(20, 41), (22, 45)]), "s", trait=0, ton=0)
    for x, y in ((12, 38), (17, 36), (33, 37), (37, 39)):
        t.p(P([(x - 2, y + 3), (x, y - 2), (x + 2, y + 3)]), "s", trait=0)
    # le corps annele jaillit en arc
    corps = [(27, 42, 6.4), (30, 32, 6.2), (30, 22, 5.8), (25, 14, 5.4), (17, 11, 5)]
    t.p(S(corps), "c", trait=0)
    for x0, y0, x1, y1 in ((23, 37, 36, 37), (24, 31, 37, 31), (24, 25, 36, 24), (23, 19, 35, 18), (22, 9, 24, 20), (17, 6, 18, 17)):
        t.p(L(x0, y0, x1, y1), "c", trait=0, ton=0, sur="c")
    t.px([(26, 34), (27, 28), (27, 22), (25, 15), (20, 9)], "c", 4)
    # gueule ronde, trois rangs de dents
    t.p(E(12, 11, 5.6, 6), "c", trait=0, dl=1)
    t.p(E(11.5, 11, 4, 4.4), "r", trait=0, ton=0)
    t.p(E(11.5, 11, 2, 2.2), "o", trait=0, ton=2)
    for a in range(0, 360, 36):
        t.px((11.5 + 3.4 * math.cos(math.radians(a)), 11 + 3.8 * math.sin(math.radians(a))), "w", 4)
    for a in range(18, 360, 72):
        t.px((11.5 + 2 * math.cos(math.radians(a)), 11 + 2.2 * math.sin(math.radians(a))), "w", 3)
    t.px([(6, 17), (5, 20), (8, 21)], "s", 4)


@sprite("sphinx-des-sables", "Sphinx des sables")
def sphinx(t):
    t.mat("f", "c8a868", "9aa8b8")   # pelage de sable
    t.mat("c", PEAU)
    t.mat("n", ["1a2a5a", "2a4a9a", "4a78d0", "8aacf0", "d0e0ff"], ["5a1a2a", "9a2a4a", "d04a78", "f08aac", "ffd0e0"])   # nemes raye
    t.mat("g", OR)
    t.mat("p", "a8885a", "7a8898")   # ailes
    t.mat("y", FEU_VIOLET, FEU_VERT)
    # ailes repliees
    t.p(B([(22, 22), (28, 8), (40, 6), (44, 14), (38, 22), (30, 26)]), "p", trait=0)
    for a, b in (((28, 12), (40, 9)), ((27, 16), (41, 13)), ((26, 20), (39, 18))):
        t.p(L(a[0], a[1], b[0], b[1]), "p", trait=0, ton=0, sur="p")
    # queue de lion
    t.p(S([(40, 36, 1.6), (45, 33, 1.3), (46, 27, 1.1)]), "f", trait=0)
    t.p(E(46, 25, 1.8, 2.2), "p", trait=0, dl=-1)
    # couche, pattes allongees
    t.p(B([(12, 26), (20, 23), (32, 25), (41, 30), (42, 39), (34, 43), (16, 43), (11, 36)]), "f", trait=0)
    t.p(E(35, 37, 6.5, 6), "f", trait=2)
    t.p(K([(15, 38, 3), (8, 41, 2.6), (2, 42, 2.4)]), "f", trait=1)
    t.p(K([(23, 40, 2.8), (17, 43, 2.4)]), "f", trait=1)
    t.px([(0, 43), (2, 44), (4, 44), (14, 45), (16, 45)], "f", 0)
    # tete de femme sous le nemes, uraeus
    t.p(B([(7, 18), (9, 8), (16, 5), (23, 8), (25, 18), (24, 29), (18, 24), (13, 24), (8, 29)]), "n", trait=0)
    for y in (10, 14, 18, 22):
        t.p(L(8, y + 2, 12, y) | L(21, y, 25, y + 2), "g", trait=0, ton=3, sur="n")
    t.p(E(16, 15, 4.4, 5.2), "c", trait=0)
    t.px([(14, 14), (15, 14), (17, 14), (18, 14)], "y", 4); t.px([(13, 13), (14, 13), (15, 13), (17, 13), (18, 13), (19, 13)], "o")
    t.px([(15, 18), (16, 18), (17, 18)], "c", 0)
    t.p(R(15, 5, 17, 8), "g", trait=0, ton=3); t.px((16, 4), "g", 4)
    t.p(R(13, 20, 19, 23), "g", trait=0)
    t.px([(20, 28), (26, 30), (32, 30), (22, 34)], "f", 0)


@sprite("phenix-blanc", "Phénix blanc")
def phenix(t):
    t.mat("p", ["a89a78", "d4c8a8", "f0e8d0", "fbf8ec", "ffffff"], ["3a3048", "5a4c70", "8474a0", "b4a8cc", "e4dcf4"])   # plumes blanches
    t.mat("f", ["c87a10", "f0b020", "f8dc50", "fff4a0", "ffffe8"], FEU_BLEU)   # flammes d'or pale
    t.mat("b", "e8b030", "c0c0d0")
    t.mat("y", "3a7ad0", "d03a3a")
    # queue en longues flammes
    for x, y in ((38, 44), (44, 40), (46, 33), (42, 46)):
        t.p(S([(30, 30, 2.4), ((30 + x) / 2 + 2, (30 + y) / 2 - 2, 2), (x, y, 0.8)]), "f", trait=0, ton=2)
    t.p(S([(30, 30, 1.2), (36, 35, 1), (41, 41, 0.5)]), "f", trait=0, ton=4)
    # ailes levees, bordees de feu
    for (bx, by), pointes, dl in (((26, 22), ((30, 2), (37, 3), (42, 8), (45, 15)), -1), ((20, 22), ((17, 1), (10, 2), (5, 7), (3, 14)), 0)):
        for i, (x, y) in enumerate(pointes):
            t.p(C(bx, by, x, y, 3.2, 1.2), "p", trait=0, dl=dl - (i % 2))
            t.p(E(x, y, 1.4, 1.8), "f", trait=0, ton=3)
    # corps, poitrail bombe
    t.p(B([(15, 22), (20, 17), (27, 19), (32, 27), (30, 34), (22, 36), (16, 31)]), "p", trait=0)
    t.px([(19, 26), (22, 29), (25, 31), (20, 31), (26, 26)], "p", 1)
    t.p(K([(21, 35, 1.3), (20, 41, 0.9)]) | K([(26, 35, 1.3), (27, 41, 0.9)]), "b", trait=0)
    t.px([(18, 42), (19, 42), (21, 42), (22, 43), (25, 42), (26, 42), (28, 42), (29, 43)], "b", 2)
    # cou, tete, aigrette de feu
    t.p(S([(18, 21, 3), (15, 15, 2.4), (15, 10, 2.4)]), "p", trait=0)
    t.p(E(14, 9, 3.6, 3.4), "p", trait=0)
    t.p(P([(11, 8), (5, 10), (11, 11)]), "b", trait=0)
    t.px([(13, 8), (14, 8)], "y", 3); t.px((13, 8), "o")
    for x, y in ((15, 2), (18, 3), (20, 6)):
        t.p(C(15, 7, x, y, 1.2, 0.5), "f", trait=0, ton=2)
    t.px([(15, 3), (18, 4)], "f", 4)
    t.px([(8, 22), (36, 20), (10, 30), (40, 28)], "f", 4)


@sprite("crabe-necrophage", "Crabe nécrophage")
def crabe(t):
    t.mat("c", ["3a2028", "6a3a40", "9a5e5c", "c48c80", "e8c0b0"], ["20283a", "3a466a", "5e6c9a", "8c98c4", "c0c8e8"])   # carapace
    t.mat("a", ACIER)                # plaques de fer
    t.mat("k", OS)
    t.mat("y", "f8e040", "40f0e0")
    # pattes
    for cote in (1, -1):
        c = lambda x: 24 + cote * x
        for i, (x1, y1, x2, y2) in enumerate(((13, 30, 19, 38), (14, 34, 18, 43), (12, 37, 14, 45))):
            t.p(K([(c(8), 32, 2), (c(x1), y1, 1.5), (c(x2), y2, 0.8)]), "c", trait=0, dl=-1 if cote > 0 else 0)
    # carapace bombee, cuirassee
    t.p(B([(9, 30), (13, 22), (24, 19), (35, 22), (39, 30), (35, 37), (24, 39), (13, 37)]), "c", trait=0)
    t.p(B([(13, 24), (24, 20), (35, 24), (33, 28), (24, 26), (15, 28)]), "a", trait=0)
    t.px([(16, 25), (24, 22), (32, 25), (20, 23), (28, 23)], "a", 4)
    t.px([(14, 32), (19, 34), (29, 34), (34, 32), (24, 35)], "c", 0)
    # pinces : l'une enorme, qui tient un os
    t.p(K([(13, 27, 2.6), (7, 21, 2.4), (6, 14, 2.2)]), "c", trait=1)
    t.p(B([(1, 12), (4, 4), (10, 3), (12, 9), (9, 15), (4, 16)]), "c", trait=0)
    t.p(P([(4, 8), (9, 6), (7, 11)]), "o", trait=0, ton=2)
    t.px([(4, 5), (6, 4), (3, 9)], "c", 4)
    t.p(K([(35, 27, 2.2), (40, 22, 1.9), (41, 17, 1.7)]), "c", trait=1, dl=-1)
    t.p(B([(38, 16), (40, 10), (44, 10), (45, 15), (42, 19)]), "c", trait=0, dl=-1)
    t.p(C(37, 9, 46, 12, 1), "k", trait=1)
    t.p(E(36.5, 8.5, 1.6, 1.6) | E(46.5, 12.5, 1.6, 1.6), "k", trait=0)
    # yeux sur tiges
    for x in (21, 27):
        t.p(R(x, 15, x, 20), "c", trait=0)
        t.p(E(x, 14, 1.6, 1.6), "y", trait=0, ton=3); t.px((x, 14), "o")
    t.px([(22, 30), (23, 31), (24, 31), (25, 31), (26, 30)], "o")


@sprite("sirene-putride", "Sirène putride")
def sirene_putride(t):
    t.mat("c", ["34483e", "56705e", "7e9a84", "a8c0aa", "d4e4d2"], ["48343e", "70565e", "9a7e84", "c0a8aa", "e4d4d2"])   # chair verdie
    t.mat("e", ["1c3a34", "2e5c50", "488876", "78b4a0", "b8e0d0"], ["3a1c34", "5c2e50", "884876", "b478a0", "e0b8d0"])   # ecailles ternes
    t.mat("h", "3a4a34", "5a3a4a")   # cheveux d'algues
    t.mat("k", OS)
    t.mat("y", "f8f0a0", "f04848")
    # queue de poisson, aretes a nu
    t.p(S([(24, 30, 4.6), (29, 36, 4.2), (35, 41, 3.4), (40, 40, 2.4), (43, 35, 1.6)]), "e", trait=0)
    t.p(P([(42, 36), (47, 28), (45, 35), (47, 41)]), "e", trait=0, dl=-1)
    t.px([(31, 37), (33, 38), (35, 39), (37, 39)], "k", 4)
    t.px([(31, 39), (33, 40), (35, 41), (31, 35), (33, 36), (35, 37)], "k", 3)
    t.px([(26, 33), (28, 36), (39, 41)], "e", 0)
    # rocher
    t.p(B([(8, 40), (14, 35), (24, 36), (30, 41), (28, 45), (10, 45)]), "k", trait=0, dl=-2)
    # buste decharne
    t.p(B([(19, 19), (24, 17), (29, 19), (29, 27), (26, 33), (21, 33), (19, 27)]), "c", trait=0)
    t.px([(21, 23), (22, 24), (26, 24), (27, 23), (21, 26), (22, 27), (26, 27), (27, 26)], "c", 0)
    t.p(K([(20, 20, 1.5), (14, 25, 1.2), (10, 22, 1)]), "c", trait=1)
    t.p(K([(28, 20, 1.5), (33, 26, 1.2), (31, 32, 1)]), "c", trait=0, dl=-1)
    t.px([(8, 20), (8, 22), (9, 23)], "k", 3)
    # tete : cheveux d'algues, bouche qui chante, orbites creuses
    t.p(B([(18, 12), (21, 6), (27, 6), (30, 12), (33, 28), (28, 19), (20, 19), (15, 28)]), "h", trait=0)
    t.p(E(24, 13, 3.6, 4.2), "c", trait=0)
    t.p(E(22.5, 12, 1.1, 1.3) | E(25.5, 12, 1.1, 1.3), "o", trait=0, ton=2)
    t.px([(22, 12), (25, 12)], "y", 3)
    t.p(E(24, 15.5, 1.3, 1.2), "o", trait=0, ton=2)
    t.px([(16, 22), (32, 24), (17, 16)], "h", 4)


@sprite("goeland-charognard", "Goéland charognard")
def goeland(t):
    t.mat("p", ["8a8a90", "b4b4ba", "dcdce0", "f2f2f4", "ffffff"], ["4a3a34", "7a6258", "a88c80", "ccb4a8", "ecdcd4"])   # plumage sale
    t.mat("g", "5a5e6a", "3a3034")   # ailes grises
    t.mat("b", "e0b030", "c8c8d8")
    t.mat("r", SANG, FEU_VERT)
    t.mat("y", "f8f0d0", "f8f0d0")
    # ailes ouvertes, bout noir
    for (bx, by), (cx, cy), (px, py), dl in (((27, 22), (36, 12), (46, 14), -1), ((20, 22), (11, 11), (1, 12), 0)):
        t.p(P([(bx, by), (cx, cy - 3), (px, py), (cx + (2 if px > 24 else -2), cy + 5), (bx, by + 6)]), "g", trait=0, dl=dl)
        t.p(P([(px, py), (px + (-5 if px > 24 else 5), py - 1), (px + (-4 if px > 24 else 4), py + 4)]), "o", trait=0, ton=2)
        t.px([(cx, cy), (cx + (3 if px > 24 else -3), cy + 1)], "g", 4)
    # corps
    t.p(B([(16, 22), (22, 18), (29, 20), (33, 28), (30, 35), (22, 36), (17, 31)]), "p", trait=0)
    t.p(P([(29, 30), (38, 36), (31, 36)]), "g", trait=0)
    t.p(K([(21, 35, 1.2), (20, 41, 0.9)]) | K([(26, 35, 1.2), (27, 41, 0.9)]), "b", trait=0)
    t.px([(17, 42), (18, 42), (19, 42), (21, 42), (24, 42), (25, 42), (26, 42), (28, 42), (29, 42)], "b", 2)
    # tete, bec crochu ensanglante, lambeau
    t.p(E(16, 15, 4.8, 4.6), "p", trait=0)
    t.p(B([(4, 15), (8, 12), (13, 13), (13, 18), (8, 18), (5, 20)]), "b", trait=0)
    t.px([(5, 17), (6, 17), (7, 16), (8, 16), (9, 16), (10, 16)], "o")
    t.px([(5, 19), (5, 20), (6, 18)], "r", 2)
    t.p(K([(6, 20, 1), (5, 24, 0.9), (7, 27, 0.7)]), "r", trait=0, ton=2)
    t.px([(15, 13), (16, 13)], "y", 3); t.px((15, 13), "o"); t.px([(14, 12), (15, 12), (16, 12), (17, 11)], "o")
    t.px([(20, 26), (24, 29), (27, 25), (22, 32)], "p", 1)
    t.px([(23, 24), (24, 25)], "r", 2)


@sprite("hydre-des-marees", "Hydre des marées")
def hydre(t):
    t.mat("e", ["143a48", "24647a", "3e94a8", "78c4d0", "c0ecf0"], ["48143a", "7a2464", "a83e94", "d078c4", "f0c0ec"])
    t.mat("v", "c8e4dc", "f0dcec")
    t.mat("o2", ["1c3a6a", "2e5c98", "4a88c8", "84bce8", "d0ecfc"])
    t.mat("y", "f8e040", "f8e040")
    t.mat("w", "f4f0e6")
    # l'ecume ou elle se dresse
    t.p(B([(2, 42), (10, 37), (24, 39), (38, 37), (46, 42), (44, 46), (4, 46)]), "o2", trait=0)
    t.px([(8, 39), (12, 38), (36, 38), (40, 39), (22, 43), (30, 44)], "o2", 4)
    # corps
    t.p(B([(14, 34), (20, 27), (30, 27), (36, 34), (36, 41), (14, 41)]), "e", trait=0)
    t.p(B([(18, 36), (24, 33), (31, 36), (31, 41), (18, 41)]), "v", trait=0)
    # cinq cous, cinq tetes
    cous = (((20, 30), (12, 24), (6, 18), 0), ((22, 28), (17, 17), (13, 8), -1), ((25, 27), (25, 15), (24, 4), 0),
            ((28, 28), (33, 17), (36, 8), -1), ((31, 30), (39, 25), (43, 19), 0))
    for (x0, y0), (x1, y1), (x2, y2), dl in cous:
        t.p(S([(x0, y0, 2.8), (x1, y1, 2.3), (x2, y2, 2.1)]), "e", trait=1, dl=dl)
        t.p(B([(x2 - 5, y2), (x2 - 2, y2 - 4), (x2 + 2, y2 - 3), (x2 + 3, y2 + 1), (x2 - 1, y2 + 3), (x2 - 4, y2 + 3)]), "e",
            trait=0, dl=dl)
        t.px((x2 - 1, y2 - 1), "y", 3)
        t.px([(x2 - 4, y2 + 2), (x2 - 3, y2 + 2), (x2 - 2, y2 + 2)], "o")
        t.px((x2 - 3, y2 + 3), "w", 4)
        t.p(P([(x2 + 1, y2 - 3), (x2 + 3, y2 - 6), (x2 + 3, y2 - 2)]), "e", trait=0, dl=-1)
    t.px([(20, 32), (27, 31), (33, 34), (16, 36)], "e", 0)
