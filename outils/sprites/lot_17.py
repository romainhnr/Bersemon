from dessin import *
from matieres import *
from atelier import sprite
import math

ECLAIR = ["8a6a10", "c8a018", "f8d830", "fff080", "ffffd8"]
VIANDE = ["4a1420", "7a2a38", "aa4a58", "d08088", "f0bcc0"]
VIANDE_SHINY = ["14204a", "2a387a", "4a58aa", "8088d0", "bcc0f0"]


@sprite("feu-de-saint-elme", "Feu de Saint-Elme")
def saint_elme(t):
    t.mat("f", ["2a2a8a", "4a5ad0", "7a9af8", "b8d4ff", "f4fbff"], ["8a2a6a", "d04aa0", "f87ac8", "ffb8e4", "fff4fb"])   # feu electrique
    t.mat("b", BOIS)
    t.mat("c", "c8c0a8", "c8c0a8")   # voile
    t.mat("k", "101028")
    # un bout de mat et sa vergue, voile en lambeaux
    t.p(R(23, 22, 25, 46), "b", trait=0)
    t.p(R(10, 26, 38, 27), "b", trait=0, dl=-1)
    t.p(P([(12, 28), (22, 28), (21, 38), (17, 35), (14, 40)]), "c", trait=0)
    t.p(P([(26, 28), (36, 28), (35, 36), (31, 33), (28, 39)]), "c", trait=0, dl=-1)
    # la flamme bleue, en aigrette au sommet du mat
    t.p(B([(24, 0), (28, 8), (32, 14), (31, 20), (24, 23), (17, 20), (16, 14), (20, 8)]), "f", trait=0, ton=1)
    t.p(B([(24, 6), (27, 12), (28, 18), (24, 21), (20, 18), (21, 12)]), "f", trait=0, ton=3)
    t.p(E(24, 17, 2.2, 2.6), "f", trait=0, ton=4)
    t.px([(22, 14), (26, 14)], "k", 2)
    t.px([(23, 18), (24, 18), (25, 18)], "k", 2)
    # aigrettes aux bouts de la vergue, arcs
    for x in (10, 38):
        t.p(P([(x - 2, 26), (x, 19), (x + 2, 26)]), "f", trait=0, ton=2)
        t.px((x, 23), "f", 4)
    t.p(LS([(12, 20), (15, 17), (14, 14), (17, 12)]) | LS([(36, 20), (33, 17), (34, 14), (31, 12)]), "f", trait=0, ton=3)
    t.px([(6, 12), (42, 10), (4, 30), (44, 32), (24, -3)], "f", 3)


@sprite("geant-des-orages", "Géant des orages")
def geant(t):
    t.mat("c", ["2a3a5a", "48608c", "7090b8", "a4c0dc", "dcecf8"], ["5a3a2a", "8c6048", "b89070", "dcc0a4", "f8ecdc"])   # peau de nuee
    t.mat("n", ["3a3a48", "5c5c70", "84849c", "b0b0c8", "dcdcf0"])   # barbe de nuages
    t.mat("e", ECLAIR, FEU_BLEU)
    t.mat("p", "4a3a2a", "2a3a4a")   # pagne
    # la foudre qu'il brandit comme un javelot
    t.p(LS([(40, -2), (36, 6), (40, 8), (35, 16), (38, 17), (34, 24)]), "e", trait=0, ton=3)
    t.p(LS([(41, -2), (37, 6), (41, 8), (36, 16), (39, 17), (35, 24)]), "e", trait=0, ton=4)
    # jambes
    t.p(K([(28, 33, 3.4), (30, 40, 2.8), (30, 44, 2.6)]), "c", trait=0, dl=-1)
    t.p(K([(20, 33, 3.4), (18, 40, 2.8), (17, 44, 2.6)]), "c", trait=0)
    t.p(R(13, 44, 20, 45) | R(27, 44, 34, 45), "c", trait=0, dl=-1)
    # torse de lutteur
    t.p(B([(13, 15), (24, 11), (35, 15), (36, 25), (32, 33), (16, 33), (12, 25)]), "c", trait=0)
    t.px([(24, 15), (24, 18), (24, 21), (20, 24), (28, 24), (19, 20), (29, 20)], "c", 0)
    t.p(B([(16, 30), (32, 30), (33, 37), (28, 35), (24, 38), (20, 35), (15, 37)]), "p", trait=0)
    # bras : l'un leve la foudre, l'autre poing ferme
    t.p(K([(34, 16, 3.2), (38, 22, 2.8), (35, 26, 2.6)]), "c", trait=1)
    t.p(K([(14, 16, 3.2), (9, 23, 2.8), (9, 30, 2.6)]), "c", trait=1)
    t.p(E(9, 32, 3, 2.8), "c", trait=0)
    # tete : barbe et chevelure de nuages d'orage
    for x, y, r in ((18, 6, 3.6), (24, 3, 4), (30, 6, 3.6), (19, 13, 3), (24, 16, 3.4), (29, 13, 3)):
        t.p(E(x, y, r, r * 0.85), "n", trait=0)
    t.p(E(24, 9, 4, 4), "c", trait=0)
    t.px([(22, 9), (26, 9)], "e", 4); t.px([(21, 8), (22, 8), (26, 8), (27, 8)], "o")
    t.px([(23, 12), (24, 12), (25, 12)], "o")
    t.px([(20, 16), (27, 17), (17, 4)], "e", 3)


@sprite("pegase-d-orage", "Pégase d'orage")
def pegase(t):
    t.mat("h", ["3a3a4a", "5e5e74", "888aa0", "b4b6c8", "e0e2ee"], ["4a3a3a", "745e5e", "a08888", "c8b4b4", "eee0e0"])   # robe gris d'orage
    t.mat("p", ["6a6a80", "9a9ab0", "c8c8dc", "e8e8f4", "ffffff"])   # ailes
    t.mat("e", ECLAIR, FEU_BLEU)
    t.mat("y", "f8f8ff")
    # aile du fond
    for i, (x, y) in enumerate(((28, 0), (34, 2), (39, 6))):
        t.p(C(27, 22, x, y, 3, 1.3), "p", trait=0, dl=-1)
    t.p(K([(19, 32, 2), (18, 38, 1.3), (19, 45, 1.2)]), "h", dl=-1, trait=0)
    t.p(K([(36, 32, 2.4), (39, 38, 1.4), (38, 45, 1.2)]), "h", dl=-1, trait=0)
    t.p(S([(39, 24, 2), (44, 27, 2.6), (45, 35, 1.6), (43, 41, 0.8)]), "p", trait=0, dl=-1)
    t.p(B([(13, 24), (19, 21), (31, 21), (40, 24), (40, 31), (33, 34), (21, 34), (14, 32)]), "h", trait=0)
    t.p(K([(16, 32, 2.2), (12, 37, 1.4), (9, 42, 1.2)]), "h")
    t.p(K([(34, 32, 2.8), (34, 38, 1.5), (32, 45, 1.2)]), "h")
    t.px([(7, 43), (8, 43), (30, 46), (31, 46), (32, 46), (18, 46), (37, 46)], "o")
    # aile de devant, grande ouverte
    for i, (x, y) in enumerate(((20, -1), (27, 0), (33, 3), (38, 8), (41, 14))):
        t.p(C(23, 23, x, y, 3.2, 1.3), "p", trait=0, dl=-(i % 2))
    t.p(E(23, 22, 4, 3), "p", trait=1)
    # encolure, tete, criniere dressee par la foudre
    t.p(B([(12, 27), (9, 17), (12, 11), (17, 13), (19, 23)]), "h", trait=0)
    t.p(B([(3, 16), (6, 10), (12, 9), (15, 13), (11, 18), (5, 19)]), "h", trait=0)
    t.p(P([(12, 9), (14, 4), (15, 10)]), "h", trait=0)
    t.px([(8, 13), (9, 13)], "y", 3); t.px((8, 13), "o")
    for x, y, dx, dy in ((13, 9, 3, -5), (15, 12, 4, -4), (17, 16, 4, -3)):
        t.p(P([(x, y), (x + dx, y + dy), (x + 2, y + 3)]), "e", trait=0, ton=3)
    # eclairs sous les sabots
    t.p(LS([(9, 44), (6, 46), (9, 47)]) | LS([(32, 47), (36, 47), (34, 49)]), "e", trait=0, ton=3)
    t.px([(44, 20), (2, 26), (42, 44)], "e", 4)


@sprite("senechal-des-orages", "Sénéchal des orages")
def senechal(t):
    t.mat("a", ["1c2444", "34447a", "5470b0", "8ca8dc", "d0e0f8"], ["441c24", "7a3444", "b05470", "dc8ca8", "f8d0e0"])   # armure bleu d'orage
    t.mat("e", ECLAIR, FEU_BLEU)
    t.mat("m", ["2a2a3a", "46465c", "686884", "9494b0", "c8c8dc"])   # cape de nuee
    t.mat("g", OR)
    # la lance ou la foudre vient mourir
    t.p(C(9, 46, 11, 6, 1), "g", trait=0)
    t.p(P([(9, 7), (11, -2), (13, 7)]), "a", trait=0, ton=4)
    t.p(LS([(11, -2), (14, -5), (13, -8)]) | LS([(11, -2), (7, -4), (8, -7)]) | LS([(12, 4), (16, 2), (15, -1)]), "e", trait=0, ton=3)
    # cape en nuage
    for x, y, r in ((30, 20, 6), (35, 27, 6), (36, 35, 6), (33, 42, 5), (26, 44, 4), (38, 42, 4)):
        t.p(E(x, y, r, r * 0.85), "m", trait=0, dl=-1)
    # armure
    t.p(B([(17, 17), (24, 15), (31, 17), (32, 28), (30, 36), (18, 36), (16, 28)]), "a", trait=0)
    t.p(R(19, 36, 23, 44) | R(26, 36, 30, 44), "a", trait=0, dl=-1)
    t.p(R(17, 44, 23, 45) | R(26, 44, 32, 45), "a", trait=0)
    t.p(LS([(21, 19), (25, 24), (22, 25), (26, 31)]), "e", trait=0, ton=3)
    t.p(R(18, 32, 30, 33), "g", trait=0)
    t.p(E(17, 17, 4, 3.2) | E(31, 17, 4, 3.2), "a", trait=0)
    t.p(K([(17, 19, 2.2), (12, 24, 1.9), (10, 22, 1.7)]), "a", trait=1)
    t.p(K([(31, 19, 2.2), (35, 26, 1.9), (34, 32, 1.7)]), "a", trait=0, dl=-1)
    # heaume a visiere baissee, ailettes
    t.p(B([(19, 14), (20, 6), (24, 3), (28, 6), (29, 14)]), "a", trait=0)
    t.p(R(20, 9, 28, 10), "o", trait=0, ton=2)
    t.px([(21, 9), (22, 9), (26, 9), (27, 9)], "e", 4)
    t.px([(22, 12), (24, 12), (26, 12)], "a", 0)
    t.p(P([(19, 8), (14, 2), (20, 5)]) | P([(29, 8), (34, 2), (28, 5)]), "g", trait=0)
    t.px([(40, 14), (42, 22), (44, 34)], "e", 3)


@sprite("larve-de-chair", "Larve de chair")
def larve(t):
    t.mat("c", VIANDE, VIANDE_SHINY)
    t.mat("v", ["8a5a50", "b88478", "dcb0a4", "f2d4c8", "fff0e8"])   # anneaux pales
    t.mat("w", OS)
    t.mat("p", FEU_VERT, FEU_VIOLET)
    # corps gras, annele, a ras de terre
    corps = [(42, 38, 3.4), (36, 36, 6), (28, 36, 7.4), (19, 36, 7.4), (11, 35, 6.2)]
    t.p(S(corps), "v", trait=0)
    for x in (15, 20, 25, 30, 35, 39):
        t.p(L(x, 28, x, 44), "v", trait=0, ton=0, sur="v")
    # plaques de chair a vif sur le dos
    for x in (12, 17, 22, 27, 32, 37):
        t.p(E(x + 1, 31 + (x % 2), 2, 2.6), "c", trait=0, sur="v")
    # fausses pattes
    for x in (14, 19, 24, 29, 34):
        t.p(R(x, 43, x + 1, 45), "v", trait=0, dl=-1)
    # stigmates suintants
    for x in (17, 22, 27, 32):
        t.px((x, 38), "p", 3)
    t.px([(22, 40), (32, 41)], "p", 2)
    # tete ronde, mandibules, ocelles
    t.p(E(8, 34, 5.4, 6), "c", trait=0)
    t.p(P([(3, 37), (2, 42), (6, 39)]) | P([(8, 39), (9, 44), (11, 39)]), "w", trait=0)
    t.px([(5, 32), (8, 31), (10, 33), (6, 35)], "o")
    t.px([(4, 31), (9, 30)], "v", 4)
    t.px([(7, 44), (5, 46), (10, 46)], "p", 3)


@sprite("devoreur-de-chair", "Dévoreur de chair")
def devoreur(t):
    t.mat("c", VIANDE, VIANDE_SHINY)
    t.mat("w", OS)
    t.mat("n", "2a141c", "141c2a")
    t.mat("r", SANG, FEU_VERT)
    # quatre pattes arquees
    t.p(K([(19, 30, 2.6), (15, 36, 2), (15, 44, 1.8)]), "c", dl=-1, trait=0)
    t.p(K([(35, 30, 2.6), (40, 36, 2), (39, 44, 1.8)]), "c", dl=-1, trait=0)
    # corps : une boule de muscle, sans yeux, toute en gueule
    t.p(E(25, 23, 15, 13), "c", trait=0)
    t.p(K([(14, 30, 3), (9, 37, 2.2), (9, 44, 2)]), "c")
    t.p(K([(32, 32, 3), (34, 38, 2.2), (32, 44, 2)]), "c")
    t.px([(6, 45), (7, 46), (9, 46), (11, 46), (30, 46), (32, 46), (34, 46), (13, 46), (38, 46)], "w", 3)
    # la gueule fend tout le corps
    t.p(B([(10, 22), (18, 20), (28, 20), (37, 23), (34, 30), (24, 33), (14, 31)]), "n", trait=0, ton=2)
    t.p(B([(16, 25), (24, 24), (32, 26), (28, 30), (20, 30)]), "r", trait=0, ton=1)
    for x in range(12, 36, 3):
        t.p(P([(x, 21 + abs(x - 24) // 8), (x + 1, 25 + abs(x - 24) // 8), (x + 2, 21 + abs(x - 24) // 8)]), "w", trait=0, ton=4)
    for x in range(15, 33, 3):
        t.p(P([(x, 32 - abs(x - 24) // 6), (x + 1, 28 - abs(x - 24) // 6), (x + 2, 32 - abs(x - 24) // 6)]), "w", trait=0, ton=3)
    # fibres des muscles, bave
    for pts in ([(16, 13), (22, 15), (30, 14)], [(13, 17), (10, 20)], [(36, 16), (39, 20)], [(20, 34), (26, 35)]):
        t.p(LS(pts), "c", trait=0, ton=0)
    t.px([(20, 12), (26, 11), (32, 13)], "c", 4)
    t.px([(22, 33), (22, 36), (27, 33), (27, 35), (27, 38)], "r", 2)


@sprite("valet-ecorche", "Valet écorché")
def valet(t):
    t.mat("c", VIANDE, VIANDE_SHINY)
    t.mat("l", ["2a2a4a", "44447a", "6464a8", "9494d0", "ccccf0"], ["4a2a2a", "7a4444", "a86464", "d09494", "f0cccc"])   # livree
    t.mat("g", OR)
    t.mat("k", OS)
    t.mat("a", ACIER)
    t.mat("y", "f8f8ff")
    # jambes ecorchees
    t.p(K([(27, 33, 2.2), (29, 39, 1.8), (28, 44, 1.6)]), "c", trait=0, dl=-1)
    t.p(K([(21, 33, 2.2), (19, 39, 1.8), (19, 44, 1.6)]), "c", trait=0)
    t.p(R(15, 44, 21, 45) | R(26, 44, 32, 45), "o", trait=0, ton=2)
    # livree de valet, ouverte sur les cotes a nu
    t.p(B([(17, 17), (24, 15), (31, 17), (33, 28), (31, 35), (17, 35), (15, 28)]), "l", trait=0)
    t.p(B([(21, 18), (27, 18), (28, 32), (20, 32)]), "c", trait=0)
    for y in (21, 24, 27, 30):
        t.px([(21, y), (22, y), (26, y), (27, y)], "k", 4)
    t.px([(17, 22), (17, 26), (17, 30), (31, 22), (31, 26), (31, 30)], "g", 3)
    # bras : un plateau d'argent, et sur le plateau un couteau
    t.p(K([(18, 19, 2.2), (12, 22, 1.9), (8, 18, 1.7)]), "l", trait=1)
    t.p(E(8, 16, 1.8, 1.8), "c", trait=0)
    t.p(E(8, 13, 6, 1.6), "a", trait=0)
    t.p(R(5, 10, 6, 12), "l", trait=0, dl=-1); t.p(P([(4, 10), (5, 4), (7, 10)]), "a", trait=0, ton=4)
    t.p(K([(30, 19, 2.2), (34, 26, 1.9), (33, 33, 1.7)]), "l", trait=0, dl=-1)
    t.p(E(33, 35, 1.8, 2), "c", trait=0)
    # tete sans peau : muscles, dents, yeux ronds sans paupieres
    t.p(E(24, 10, 4.4, 4.8), "c", trait=0)
    t.p(E(22, 9, 1.4, 1.4) | E(26, 9, 1.4, 1.4), "y", trait=0, ton=3)
    t.px([(22, 9), (26, 9)], "o")
    t.px([(22, 13), (23, 13), (24, 13), (25, 13), (26, 13)], "k", 4)
    t.px([(22, 14), (24, 14), (26, 14)], "o")
    t.px([(20, 7), (24, 6), (28, 8), (24, 11)], "c", 0)
    t.p(R(20, 15, 28, 16), "y", trait=0, ton=3)


@sprite("idole-de-chair", "Idole de chair")
def idole(t):
    t.mat("c", VIANDE, VIANDE_SHINY)
    t.mat("g", PIERRE)
    t.mat("y", ["7a6a10", "c0a820", "f0d840", "f8f090", "fffce0"], FEU_BLEU)
    t.mat("o2", OR)
    # piedestal
    t.p(R(12, 40, 36, 45), "g", trait=0, dl=-1)
    t.p(R(14, 37, 34, 40), "g", trait=0)
    t.px([(17, 42), (24, 43), (31, 42)], "o2", 3)
    # six bras en rayons
    for cote in (1, -1):
        c = lambda x: 24 + cote * x
        for (x1, y1), (x2, y2) in (((12, 12), (18, 6)), ((14, 20), (21, 20)), ((12, 28), (18, 34))):
            t.p(K([(c(5), 20, 2.2), (c(x1), y1, 1.7), (c(x2), y2, 1.4)]), "c", trait=0, dl=-1 if cote > 0 else 0)
            t.p(E(c(x2), y2, 2, 2), "c", trait=0)
    # corps : une colonne de chair assise, ventre rond
    t.p(B([(17, 12), (24, 8), (31, 12), (33, 24), (32, 36), (16, 36), (15, 24)]), "c", trait=0)
    t.p(E(24, 29, 6.5, 6), "c", trait=0, dl=1)
    # des yeux partout
    for x, y, r in ((24, 14, 2.6), (19, 20, 1.6), (29, 20, 1.6), (24, 28, 2.2), (19, 32, 1.4), (29, 32, 1.4)):
        t.p(E(x, y, r, r * 0.75), "y", trait=2, ton=3)
        t.px((x, y), "o")
    # une bouche au nombril, couronne de dents en diademe
    t.px([(22, 22), (23, 23), (24, 23), (25, 23), (26, 22)], "o")
    for x in (19, 22, 25, 28):
        t.p(P([(x, 9 + abs(x - 24) // 3), (x + 1, 5 + abs(x - 24) // 3), (x + 2, 9 + abs(x - 24) // 3)]), "o2", trait=0)
    t.px([(17, 16), (31, 17), (20, 26), (28, 26)], "c", 0)
