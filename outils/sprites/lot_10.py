from dessin import *
from matieres import *
from atelier import sprite
from commun import _aile_membrane
import math


@sprite("demiurge", "Démiurge")
def demiurge(t):
    t.mat("r", "2c2238", "e8e0d0")   # robe de nuit
    t.mat("m", "4a2a5a", "c8b070")   # ailes
    t.mat("c", ["5a5068", "8a80a0", "b8b0cc", "dcd6ea", "f6f2ff"])   # visage de cire
    t.mat("g", OR, FEU_VIOLET)       # le sceau : une main ouverte
    t.mat("y", SANG, FEU_BLEU)
    _aile_membrane(t, "m", "r", (28, 16), (34, 2), [(45, 1), (46, 12), (42, 22), (36, 20)], dl=-1)
    _aile_membrane(t, "m", "r", (20, 16), (14, 2), [(3, 1), (2, 12), (6, 22), (12, 20)])
    # robe droite, en colonne
    t.p(B([(17, 16), (24, 13), (31, 16), (34, 30), (36, 45), (12, 45), (14, 30)]), "r", trait=0)
    for x in (16, 20, 28, 32):
        t.p(L(x, 26, x + (x - 24) // 4, 44), "r", trait=0, ton=0, sur="r")
    t.px([(14, 45), (18, 44), (24, 45), (30, 44), (34, 45)], "r", 3)
    # bras ouverts, paumes offertes
    t.p(K([(18, 18, 2.2), (12, 24, 1.8), (8, 22, 1.5)]) | K([(30, 18, 2.2), (36, 24, 1.8), (40, 22, 1.5)]), "r", trait=1)
    t.p(E(7, 20.5, 2, 2) | E(41, 20.5, 2, 2), "c", trait=0)
    # le sceau sur la poitrine
    t.p(E(24, 27, 3.2, 3.4), "g", trait=0)
    for x, h in ((21, 3), (23, 5), (25, 5), (27, 3)):
        t.p(R(x, 24 - h, x, 24), "g", trait=0, ton=3)
    t.px((24, 27), "y", 2)
    # capuchon, visage lisse, regard rouge
    t.p(B([(17, 14), (19, 5), (24, 2), (29, 5), (31, 14), (27, 18), (21, 18)]), "r", trait=0)
    t.p(E(24, 10.5, 3.6, 4.6), "c", trait=0)
    t.px([(22, 10), (26, 10)], "y", 3); t.px([(22, 9), (26, 9)], "o")
    t.px([(23, 13), (24, 13), (25, 13)], "c", 0)


@sprite("demiurge-eveille", "Démiurge éveillé")
def demiurge_eveille(t):
    t.mat("c", ["30101c", "5a1c30", "8a3048", "b85a6c", "e09aa0"], ["101c30", "1c305a", "30488a", "5a6cb8", "9aa0e0"])   # chair du dieu
    t.mat("m", "4a2a5a", "c8b070")
    t.mat("y", ["7a1010", "c02020", "f04a30", "f8a060", "fff0c0"], FEU_BLEU)   # yeux
    t.mat("g", OR, FEU_VIOLET)
    _aile_membrane(t, "m", "c", (30, 18), (37, 3), [(46, 2), (47, 13), (44, 23), (38, 21)], dl=-1)
    _aile_membrane(t, "m", "c", (18, 18), (11, 3), [(2, 2), (1, 13), (4, 23), (10, 21)])
    # huit pattes en etoile
    for cote in (1, -1):
        c = lambda x: 24 + cote * x
        for (dx1, y1), (dx2, y2) in (((12, 20), (20, 28)), ((13, 28), (21, 38)), ((11, 34), (16, 45)), ((7, 38), (9, 46))):
            t.p(K([(c(5), 28, 2.2), (c(dx1), y1, 1.6), (c(dx2), y2, 1)]), "c", trait=0, dl=-1 if cote > 0 else 0)
    # corps : une sphere couverte d'yeux
    t.p(E(24, 26, 12, 11.5), "c", trait=0)
    for x, y, r in ((24, 24, 3.4), (17, 20, 1.8), (31, 20, 1.8), (16, 29, 1.6), (32, 29, 1.6), (21, 33, 1.5), (28, 33, 1.5),
                    (24, 17, 1.4)):
        t.p(E(x, y, r, r * 0.8), "y", trait=2, ton=3)
        t.px((x, y), "o")
    t.px((23, 23), "y", 4)
    # couronne : le sceau de la main, ouverte au-dessus de lui
    for x, h in ((20, 5), (22, 8), (24, 9), (26, 8), (28, 5)):
        t.p(R(x, 13 - h, x, 13), "g", trait=0, ton=3)
    t.px([(19, 13), (29, 13)], "g", 2)
    t.px([(14, 24), (34, 25), (20, 37), (27, 37)], "c", 0)


@sprite("cloche-hurlante", "Cloche hurlante")
def cloche(t):
    t.mat("b", ["4a3410", "7a5a1c", "b08a30", "dcb858", "f8e4a0"], ["1c3a34", "30645a", "50988a", "84c4b4", "c4ece0"])   # bronze
    t.mat("l", BOIS)
    t.mat("k", "1e1618", "1e1618")
    t.mat("y", FEU, FEU_VIOLET)
    t.mat("s", ["8a8ab0", "b0b0d0", "d0d0ea", "eaeaf8", "ffffff"])   # ondes
    # le joug de bois et ses ferrures
    t.p(R(13, 3, 35, 7), "l", trait=0)
    t.px([(15, 5), (33, 5), (24, 5)], "b", 4)
    t.p(R(22, 7, 26, 10), "b", trait=0, dl=-1)
    # la robe de la cloche
    t.p(B([(18, 12), (24, 9), (30, 12), (33, 24), (38, 36), (40, 40), (8, 40), (10, 36), (15, 24)]), "b", trait=0)
    t.p(R(8, 39, 40, 41), "b", trait=0, ton=3)
    t.px([(x, 15) for x in range(18, 31)], "b", 1)
    t.px([(x, 35) for x in range(11, 38)], "b", 1)
    t.px([(19, 13), (18, 17), (17, 21), (16, 25)], "b", 4)
    # visage qui hurle, fondu dans le bronze
    t.p(E(19, 23, 2, 2.6) | E(29, 23, 2, 2.6), "k", trait=0, ton=2)
    t.px([(19, 23), (29, 23)], "y", 3)
    t.p(B([(19, 29), (24, 27), (29, 29), (29, 35), (24, 37), (19, 35)]), "k", trait=0, ton=2)
    t.px([(21, 29), (24, 28), (27, 29)], "b", 4)
    # le battant, en langue
    t.p(R(23, 34, 25, 43), "b", trait=0, dl=-1)
    t.p(E(24, 44, 2.4, 2.2), "b", trait=0)
    # ondes du cri
    for cote in (1, -1):
        for d, h in ((20, 5), (23, 8)):
            t.p(CS([(24 + cote * d, 28 - h), (24 + cote * (d + 2), 28), (24 + cote * d, 28 + h)]), "s", trait=0, ton=2)


@sprite("seraphin-de-vitrail", "Séraphin de vitrail")
def seraphin(t):
    t.mat("p", "2a2630", "4a4050")   # plombs
    t.mat("r", ["6a0a1a", "a8142a", "e03048", "f8808a", "ffe0e0"], ["0a4a6a", "1484a8", "30b8e0", "80dcf8", "e0f8ff"])
    t.mat("b", ["0a2a7a", "1448b8", "3078e8", "80b0f8", "e0ecff"], ["6a4a0a", "a87814", "e0a830", "f8d080", "fff4e0"])
    t.mat("g", OR, ["3a6a2a", "5a9a3a", "88c858", "b8e890", "e8fcd0"])
    t.mat("w", ["a8a898", "d0d0c0", "ecece0", "f8f8f0", "ffffff"])
    # six ailes de verre, cernees de plomb
    for i, (dx, y0, y1, m) in enumerate(((20, 2, 14, "b"), (22, 14, 26, "r"), (18, 26, 38, "g"))):
        for cote in (1, -1):
            c = lambda x: 24 + cote * x
            aile = P([(c(4), y0 + 8), (c(dx - 6), y0), (c(dx), y0 + 4), (c(dx - 2), y1), (c(5), y1 - 2)])
            t.p(aile, "p", trait=0, ton=2)
            t.p(reduire(aile), m, trait=0)
            t.p(L(c(5), y0 + 8, c(dx - 2), y0 + 5) | L(c(6), y1 - 3, c(dx - 4), y0 + 6), "p", trait=0, ton=2)
    # corps : une lancette de vitrail
    corps = B([(19, 16), (24, 12), (29, 16), (30, 34), (27, 44), (21, 44), (18, 34)])
    t.p(corps, "p", trait=0, ton=2)
    t.p(reduire(corps), "w", trait=0)
    t.p(R(19, 24, 29, 24) | R(19, 33, 29, 33) | R(24, 17, 24, 43), "p", trait=0, ton=2)
    t.p(R(20, 25, 23, 32), "r", trait=0, ton=2); t.p(R(25, 25, 28, 32), "b", trait=0, ton=2)
    t.p(R(21, 34, 23, 41), "b", trait=0, ton=3); t.p(R(25, 34, 27, 41), "r", trait=0, ton=3)
    t.px([(21, 18), (22, 20), (26, 19)], "w", 4)
    # tete et nimbe
    t.p(E(24, 8, 6.4, 6.4), "g", trait=0, ton=3)
    t.p(E(24, 8, 6.4, 6.4) - E(24, 8, 5.2, 5.2), "p", trait=0, ton=2)
    t.p(E(24, 9, 3.4, 3.8), "w", trait=0)
    t.px([(22, 9), (26, 9)], "b", 2); t.px([(23, 11), (24, 11), (25, 11)], "r", 2)
    t.px([(3, 8), (44, 10), (6, 32), (42, 34)], "g", 4)


@sprite("molosse-des-fosses", "Molosse des fosses")
def molosse(t):
    t.mat("f", "7a5a40", "5a5a6a")   # robe fauve
    t.mat("n", "3a2c24", "2a2a34")   # masque noir
    t.mat("a", ACIER)
    t.mat("r", SANG, FEU_VIOLET)
    t.mat("w", "f4f0e6")
    t.mat("y", "f8c040", "40e0f0")
    t.p(K([(20, 30, 2.8), (19, 38, 2), (18, 44, 1.8)]), "f", dl=-1, trait=0)
    t.p(K([(36, 29, 3.2), (39, 36, 2), (38, 44, 1.8)]), "f", dl=-1, trait=0)
    t.p(S([(40, 22, 1.6), (44, 19, 1.3), (45, 14, 0.9)]), "f", trait=0)
    # corps trapu, poitrail large
    t.p(B([(11, 20), (18, 16), (30, 18), (40, 21), (41, 29), (34, 32), (22, 34), (13, 33), (10, 27)]), "f", trait=0)
    t.p(K([(15, 31, 3.2), (14, 38, 2.2), (13, 44, 2)]), "f")
    t.p(K([(34, 30, 3.6), (35, 37, 2.2), (33, 44, 2)]), "f")
    t.px([(11, 45), (12, 45), (31, 45), (32, 45), (16, 45), (36, 45)], "w", 2)
    # cicatrices
    t.px([(24, 22), (25, 23), (26, 24), (30, 21), (31, 22), (20, 27), (21, 28)], "r", 3)
    # tete carree, babines pendantes, oreilles coupees
    t.p(B([(3, 14), (7, 9), (15, 9), (19, 14), (18, 22), (11, 24), (4, 22)]), "f", trait=0)
    t.p(B([(2, 17), (7, 15), (12, 17), (12, 24), (6, 26), (2, 23)]), "n", trait=0)
    t.p(P([(13, 10), (15, 6), (17, 11)]), "n", trait=0)
    t.px([(9, 13), (10, 13)], "y", 3); t.px((9, 13), "o"); t.px([(8, 12), (9, 12), (10, 12), (11, 11)], "o")
    t.px([(2, 17), (3, 17)], "o")
    t.px([(3, 23), (4, 24), (5, 24), (6, 24), (7, 24), (8, 23), (9, 23)], "o")
    t.px([(4, 25), (6, 25), (8, 24)], "w", 4)
    t.px([(5, 27), (5, 29)], "w", 3)
    # collier de fer a pointes, chaine brisee
    t.p(C(14, 21, 20, 30, 1.6), "a", trait=1)
    for x, y in ((12, 22), (14, 26), (17, 30), (21, 31)):
        t.p(P([(x, y), (x - 2, y + 2), (x + 1, y + 2)]), "a", trait=0, ton=4)
    t.px([(19, 33), (19, 35), (20, 36), (20, 38)], "a", 3)


@sprite("taureau-d-airain", "Taureau d'airain")
def taureau(t):
    t.mat("b", ["4a2c10", "7a4c1c", "b07c34", "dca85c", "f8dca0"], ["2a3a3a", "486464", "709898", "a0c4c4", "d4ecec"])   # airain
    t.mat("f", FEU, FEU_BLEU)
    t.mat("k", "e8e0c8", "e8e0c8")
    t.mat("n", FER_NOIR)
    t.p(K([(21, 32, 2.8), (21, 39, 2), (20, 45, 2)]), "b", dl=-1, trait=0)
    t.p(K([(37, 32, 3), (39, 39, 2), (38, 45, 2)]), "b", dl=-1, trait=0)
    t.p(S([(41, 22, 1.3), (45, 25, 1.1), (45, 32, 1)]), "b", trait=0)
    t.p(P([(44, 32), (46, 36), (43, 36)]), "f", trait=0, ton=2)
    # corps en tonneau, rivete
    t.p(B([(12, 19), (20, 15), (34, 16), (42, 21), (42, 31), (35, 34), (21, 34), (13, 32)]), "b", trait=0)
    t.px([(18, 18), (24, 17), (30, 17), (36, 19), (18, 32), (24, 33), (30, 33), (36, 31)], "b", 4)
    t.p(L(21, 16, 21, 34) | L(34, 17, 34, 34), "b", trait=0, ton=0, sur="b")
    # la trappe du ventre, d'ou sort la lueur du brasier
    t.p(R(24, 22, 31, 29), "n", trait=0, ton=1)
    t.p(R(25, 23, 30, 28), "f", trait=0, ton=1)
    t.p(P([(25, 28), (26, 25), (27, 27), (28, 23), (29, 26), (30, 28)]), "f", trait=0, ton=3)
    t.px([(27, 28), (28, 27), (28, 28)], "f", 4)
    t.p(K([(17, 32, 3.2), (16, 39, 2.2), (15, 45, 2.1)]), "b")
    t.p(K([(35, 32, 3.4), (35, 39, 2.2), (33, 45, 2.1)]), "b")
    t.px([(13, 46), (14, 46), (15, 46), (31, 46), (32, 46), (33, 46)], "n", 2)
    # tete basse, cornes, fumee aux naseaux
    t.p(B([(3, 21), (6, 14), (13, 12), (18, 16), (18, 24), (11, 28), (5, 27)]), "b", trait=0)
    t.p(CS([(8, 13), (4, 9), (5, 4)]) | CS([(14, 12), (17, 7), (15, 2)]), "k", trait=0)
    t.px([(9, 18), (10, 18)], "f", 3); t.px((9, 18), "f", 4)
    t.px([(4, 24), (6, 25)], "o")
    t.p(E(5, 27, 2.4, 1.2) - E(5, 27, 1.2, 0.4), "n", trait=0, ton=3)
    t.px([(2, 22), (1, 20), (2, 18), (0, 16)], "f", 2)
    t.px([(1, 21), (1, 17)], "f", 4)


@sprite("crotale-des-salines", "Crotale des salines")
def crotale(t):
    t.mat("e", "c8b89a", "9a8ab8")   # ecailles couleur de sel
    t.mat("z", "8a6a4a", "5a4a7a")   # losanges
    t.mat("v", "f0e8d4", "f0e8f4")
    t.mat("k", OS)                   # sonnette
    t.mat("y", "f04830", "40e0f0")
    t.mat("s", ["8a8ab0", "b0b0d0", "d0d0ea", "eaeaf8", "ffffff"])
    # anneaux a plat
    t.p(E(25, 40, 17, 4.4), "e", trait=0)
    t.p(E(25, 34.5, 13, 4), "e", trait=1)
    # la queue dressee et sa sonnette
    t.p(S([(38, 36, 2.6), (42, 28, 2.2), (41, 20, 1.8)]), "e", trait=1)
    for i, (x, y) in enumerate(((41, 17), (40, 14), (40, 11), (39, 8), (39, 5))):
        t.p(E(x, y, 2 - i * 0.2, 1.7), "k", trait=1)
    for d in (4, 7):
        t.p(CS([(39 - d, 4), (39 - d - 1, 8), (39 - d, 12)]) | CS([(40 + d, 4), (41 + d, 8), (40 + d, 12)]), "s", trait=0, ton=2)
    # cou en S, pret a frapper
    t.p(S([(20, 33, 3.8), (15, 27, 3.4), (18, 20, 3.1), (13, 14, 3)]), "e", trait=1)
    t.p(S([(19, 34, 1.4), (13, 27, 1.3), (16, 20, 1.2), (11, 15, 1.1)]), "v", trait=0, sur="e")
    t.p(B([(1, 14), (6, 8), (14, 8), (17, 12), (15, 17), (7, 18), (2, 17)]), "e", trait=0)
    for x, y in ((12, 38), (18, 40), (24, 38), (30, 40), (36, 38), (16, 34), (22, 35), (28, 34), (34, 35), (19, 24),
                 (17, 18)):
        t.p(P([(x - 2, y), (x, y - 2), (x + 2, y), (x, y + 2)]), "z", trait=0, ton=2, sur="e")
    # gueule ouverte, crochets
    t.p(B([(1, 16), (6, 16), (11, 17), (8, 21), (3, 21)]), "v", trait=1, ton=1)
    t.px([(3, 17), (3, 18), (7, 17), (7, 18)], "k", 4)
    t.px([(8, 11), (9, 11)], "y", 3); t.px((8, 11), "o"); t.px([(7, 10), (8, 10), (9, 10), (10, 9)], "o")
    t.px((3, 13), "o")


@sprite("scarabee-solaire", "Scarabée solaire")
def scarabee(t):
    t.mat("c", ["0c2a2e", "1a5a5a", "2e9088", "6cc8b4", "c0f4e0"], ["3a0c3a", "6a1a6a", "a03aa0", "d07ad0", "f8c8f8"])   # carapace irisee
    t.mat("n", "24222c", "24222c")   # pattes
    t.mat("s", ["a84a08", "e88a10", "f8c830", "ffe880", "fffcd8"], FEU_BLEU)   # le disque du soleil
    # le soleil qu'il pousse au-dessus de lui
    for a in range(0, 360, 30):
        t.p(C(24 + 9 * math.cos(math.radians(a)), 12 + 9 * math.sin(math.radians(a)), 24 + 12.5 * math.cos(math.radians(a)),
              12 + 12.5 * math.sin(math.radians(a)), 1, 0.3), "s", trait=0, ton=2)
    t.p(E(24, 12, 9.4, 9.4), "s", trait=0)
    t.p(E(22, 10, 4, 4), "s", trait=0, ton=4)
    # pattes dentelees
    for cote in (1, -1):
        c = lambda x: 24 + cote * x
        for (x1, y1), (x2, y2) in (((11, 22), (9, 15)), ((13, 32), (17, 34)), ((11, 40), (15, 45))):
            t.p(K([(c(6), y1 + 4 if y1 < 30 else y1, 1.6), (c(x1), y1, 1.2), (c(x2), y2, 0.8)]), "n", trait=0)
    # corps : pronotum, elytres fendus
    t.p(B([(15, 30), (17, 25), (24, 23), (31, 25), (33, 30), (33, 39), (28, 45), (20, 45), (15, 39)]), "c", trait=0)
    t.p(L(24, 30, 24, 45), "c", trait=0, ton=0)
    t.p(B([(16, 26), (24, 22), (32, 26), (31, 30), (24, 31), (17, 30)]), "c", trait=1, dl=1)
    for x in (18, 21, 27, 30):
        t.p(L(x, 32, x + (x - 24) // 6, 43), "c", trait=0, ton=1, sur="c")
    t.px([(18, 33), (19, 32), (17, 36)], "c", 4)
    # tete en pelle, cornes
    t.p(B([(19, 23), (21, 19), (27, 19), (29, 23), (24, 25)]), "n", trait=0)
    t.px([(20, 19), (19, 18), (28, 19), (29, 18)], "n", 3)
    t.px([(22, 22), (26, 22)], "s", 3)
