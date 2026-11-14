from dessin import *
from matieres import *
from atelier import sprite
from commun import _aile_membrane
import math


@sprite("licorne-des-cendres", "Licorne des cendres")
def licorne(t):
    t.mat("h", ["3a3638", "625c5e", "8e8888", "bab4b2", "e2dedc"], ["38363a", "5e5c62", "88888e", "b2b4ba", "dcdee2"])   # robe de cendre
    t.mat("f", FEU, FEU_BLEU)
    t.mat("k", OS)
    t.mat("y", "f8c040", "40e0f0")
    t.p(K([(19, 32, 2), (18, 38, 1.3), (19, 45, 1.2)]), "h", dl=-1, trait=0)
    t.p(K([(36, 32, 2.4), (39, 38, 1.4), (38, 45, 1.2)]), "h", dl=-1, trait=0)
    # queue de braise
    t.p(S([(39, 24, 2), (44, 26, 2.8), (46, 34, 2), (44, 41, 0.9)]), "f", trait=0, ton=2)
    t.p(S([(40, 25, 1), (44, 29, 1.2), (45, 36, 0.6)]), "f", trait=0, ton=4)
    t.p(B([(13, 24), (19, 21), (31, 21), (40, 24), (40, 31), (33, 34), (21, 34), (14, 32)]), "h", trait=0)
    t.p(K([(16, 32, 2.2), (13, 38, 1.4), (14, 45, 1.2)]), "h")
    t.p(K([(34, 32, 2.8), (34, 38, 1.5), (32, 45, 1.2)]), "h")
    t.px([(12, 46), (13, 46), (14, 46), (30, 46), (31, 46), (32, 46), (18, 46), (37, 46)], "o")
    # encolure, tete, corne torsadee
    t.p(B([(12, 27), (9, 17), (12, 11), (17, 13), (19, 23)]), "h", trait=0)
    t.p(B([(3, 16), (6, 10), (12, 9), (15, 13), (11, 18), (5, 19)]), "h", trait=0)
    t.p(P([(12, 9), (14, 4), (15, 10)]), "h", trait=0)
    t.p(P([(7, 10), (3, -1), (10, 9)]), "k", trait=0)
    t.px([(6, 6), (5, 3), (7, 8)], "k", 1)
    t.px([(8, 13), (9, 13)], "y", 3); t.px((8, 13), "o")
    t.px([(3, 17), (3, 18)], "o")
    # criniere de flammes
    for x, y, dx, dy in ((13, 9, 4, -5), (15, 12, 5, -4), (17, 16, 5, -3), (18, 20, 5, -3)):
        t.p(P([(x, y), (x + dx, y + dy), (x + 3, y + 3)]), "f", trait=0, ton=2)
        t.px((x + 2, y), "f", 4)
    # braises sous la cendre
    t.px([(22, 26), (27, 24), (33, 27), (24, 31), (30, 30), (16, 28)], "f", 2)
    t.px([(4, 22), (2, 25), (5, 27)], "h", 3)


@sprite("connetable-boucher", "Connétable boucher")
def connetable(t):
    t.mat("a", FER_NOIR, ACIER)
    t.mat("t", ["5a504a", "8a7e74", "b4a89c", "d8cec2", "f4eee6"], ["4a505a", "747e8a", "9ca8b4", "c2ced8", "e6eef4"])   # tablier
    t.mat("s", SANG, FEU_VERT)
    t.mat("l", BOIS)
    t.mat("c", ACIER)
    t.mat("y", SANG, FEU_VERT)
    # grand couperet de boucher sur l'epaule
    t.p(C(8, 36, 12, 12, 1.1), "l", trait=0)
    t.p(B([(11, 14), (4, 12), (1, 4), (5, -1), (14, 2), (14, 12)]), "c", trait=0)
    t.px([(3, 6), (3, 8), (4, 10)], "c", 4)
    t.px([(6, 3), (8, 6), (10, 9), (7, 9), (9, 4)], "s", 2)
    # jambes d'armure
    t.p(R(26, 34, 31, 44), "a", trait=0, dl=-1); t.p(R(17, 34, 22, 44), "a", trait=0)
    t.p(R(15, 44, 23, 45) | R(25, 44, 33, 45), "a", trait=0)
    # corps massif, tablier ensanglante
    t.p(B([(13, 16), (24, 12), (35, 16), (37, 28), (34, 36), (14, 36), (11, 28)]), "a", trait=0)
    t.p(B([(17, 19), (31, 19), (33, 38), (15, 38)]), "t", trait=0)
    t.px([(19, 24), (20, 25), (21, 26), (27, 22), (28, 23), (24, 30), (25, 31), (25, 32), (19, 33), (29, 34), (30, 35)], "s", 2)
    t.px([(21, 27), (25, 33), (28, 24)], "s", 1)
    # spalieres a pointes, bras
    t.p(E(13, 16, 4.6, 3.8) | E(35, 16, 4.6, 3.8), "a", trait=0)
    t.p(P([(11, 13), (12, 8), (14, 13)]) | P([(34, 13), (36, 8), (37, 13)]), "a", trait=0, ton=4)
    t.p(K([(12, 19, 2.8), (9, 27, 2.4), (9, 34, 2.2)]), "a", trait=1)
    t.p(K([(36, 19, 2.8), (40, 27, 2.4), (39, 34, 2.2)]), "a", trait=0, dl=-1)
    # crochet de boucher a la ceinture, chaine
    t.p(CS([(39, 36), (41, 41), (38, 43), (36, 40)]), "c", trait=0, ton=3)
    # heaume a grille, cornes courtes
    t.p(B([(18, 14), (19, 5), (24, 2), (29, 5), (30, 14)]), "a", trait=0)
    t.p(R(20, 8, 28, 12), "o", trait=0, ton=2)
    for x in (21, 23, 25, 27):
        t.p(R(x, 8, x, 12), "a", trait=0, ton=3)
    t.px([(22, 9), (26, 9)], "y", 3)
    t.p(P([(18, 6), (14, 2), (19, 9)]) | P([(30, 6), (34, 2), (29, 9)]), "a", trait=0, ton=4)


@sprite("automate-brise", "Automate brisé")
def automate(t):
    t.mat("b", ["4a3414", "7a5a20", "b08a38", "dcb860", "f8e4a8"], ["2a3a4a", "4a647a", "7094b0", "a0c0dc", "d4e8f8"])   # laiton
    t.mat("n", FER_NOIR)
    t.mat("e", ["8a6a10", "c8a018", "f8d830", "fff080", "ffffd8"], FEU_BLEU)   # etincelles
    t.mat("v", ["0c4a3a", "189070", "30c8a0", "80f0d0", "e0fff8"], SANG)   # oeil de verre
    # jambes : une sur roue, l'autre faussee
    t.p(K([(27, 32, 2), (31, 38, 1.6), (29, 44, 1.5)]), "b", trait=0, dl=-1)
    t.p(R(26, 44, 32, 45), "b", trait=0, dl=-1)
    t.p(K([(20, 32, 2), (19, 39, 1.6)]), "b", trait=0)
    t.p(E(18, 42, 3.4, 3.4) - E(18, 42, 1.2, 1.2), "n", trait=0, ton=3)
    # coffre en tonneau, plaque ouverte sur les rouages
    t.p(B([(15, 16), (24, 14), (33, 16), (34, 28), (32, 34), (16, 34), (14, 28)]), "b", trait=0)
    t.p(R(20, 20, 29, 29), "n", trait=0, ton=1)
    for x, y, r in ((23, 23, 2.6), (27, 26, 2), (22, 27, 1.4)):
        t.p(E(x, y, r, r) - E(x, y, r * 0.35, r * 0.35), "b", trait=0, ton=3)
    t.p(P([(29, 19), (34, 17), (35, 26), (30, 29)]), "b", trait=1, dl=1)
    t.px([(16, 18), (32, 18), (16, 32), (32, 32), (18, 25)], "b", 4)
    # bras : l'un pend, arrache, cables a nu
    t.p(K([(15, 18, 2), (10, 24, 1.7), (9, 31, 1.5)]), "b", trait=1)
    t.p(P([(7, 31), (6, 36), (9, 33), (11, 37), (11, 31)]), "b", trait=0)
    t.p(K([(33, 18, 2), (37, 22, 1.6)]), "b", trait=0, dl=-1)
    t.p(CS([(38, 23), (41, 27), (39, 31)]) | CS([(38, 23), (43, 24), (44, 29)]), "n", trait=0, ton=3)
    for x, y in ((40, 33), (45, 31), (42, 36)):
        t.px([(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)], "e", 3)
    # tete : casque de cuivre penche, un oeil eteint, un ressort qui sort
    t.p(B([(18, 14), (17, 6), (23, 3), (30, 6), (30, 14)]), "b", trait=0)
    t.p(E(21, 9, 2.2, 2.2), "v", trait=2, ton=3); t.px((21, 9), "v", 4)
    t.p(E(27, 9, 2.2, 2.2), "n", trait=2, ton=1)
    t.px([(26, 8), (28, 10), (28, 8), (26, 10)], "n", 3)
    t.p(R(20, 12, 28, 12), "n", trait=0, ton=2)
    t.p(LS([(26, 3), (28, 0), (25, -1), (28, -3), (26, -5)]), "n", trait=0, ton=4)
    t.px([(30, 0), (32, -2), (23, -4)], "e", 3)


@sprite("bibliophage", "Bibliophage")
def bibliophage(t):
    t.mat("v", ["3a4a1c", "5e7a2c", "88a844", "b4d070", "e0f0b0"], ["4a1c3a", "7a2c5e", "a84488", "d070b4", "f0b0e0"])   # ver
    t.mat("p", ["8a7a5a", "b8a880", "dcd0a8", "f2ead0", "fffcf0"])   # pages
    t.mat("c", ["3a1420", "6a2434", "9a3a4c", "c06a78", "e8a8b0"], ["14203a", "24346a", "3a4c9a", "6a78c0", "a8b0e8"])   # reliure
    t.mat("g", OR)
    t.mat("y", FEU_VIOLET, FEU_VERT)
    # le grimoire ouvert
    t.p(P([(3, 36), (24, 39), (45, 36), (45, 42), (24, 46), (3, 42)]), "c", trait=0)
    t.p(P([(5, 32), (24, 36), (24, 43), (5, 39)]), "p", trait=0)
    t.p(P([(43, 32), (24, 36), (24, 43), (43, 39)]), "p", trait=0, dl=-1)
    for y in (34, 36, 38):
        t.p(L(8, y, 21, y + 3) | L(27, y + 3, 40, y), "p", trait=0, ton=1)
    t.px([(4, 40), (44, 40), (24, 45)], "g", 3)
    # un trou ronge dans la page
    t.p(E(33, 37, 3.4, 2), "o", trait=0, ton=2)
    # le ver, gras de savoir, dresse hors du trou
    corps = [(33, 37, 3.6), (36, 29, 4), (33, 21, 4.2), (26, 16, 4.2), (19, 14, 4)]
    t.p(S(corps), "v", trait=1)
    for x0, y0, x1, y1 in ((32, 31, 40, 32), (30, 26, 39, 24), (28, 20, 35, 16), (24, 12, 25, 20), (19, 10, 20, 18)):
        t.p(L(x0, y0, x1, y1), "v", trait=0, ton=0, sur="v")
    # des lettres luisent sous sa peau
    t.px([(35, 27), (33, 23), (29, 18), (23, 16), (37, 31)], "y", 3)
    # tete : lunettes, mandibules pleines de papier
    t.p(E(15, 14, 5, 4.6), "v", trait=0, dl=1)
    t.p(E(13, 13, 2, 2) | E(18, 13, 2, 2), "g", trait=0, ton=3)
    t.p(E(13, 13, 1.1, 1.1) | E(18, 13, 1.1, 1.1), "p", trait=0, ton=4)
    t.px([(13, 13), (18, 13)], "o")
    t.px([(15, 13), (16, 13)], "g", 3)
    t.p(P([(11, 17), (14, 20), (10, 21)]), "p", trait=1, ton=3)
    t.px([(12, 17), (13, 18), (15, 18)], "o")
    # pages qui volent
    for x, y in ((8, 22), (4, 12), (42, 12)):
        t.p(P([(x - 2, y), (x + 2, y - 1), (x + 3, y + 2), (x - 1, y + 3)]), "p", trait=0, ton=3)


@sprite("oeil-errant", "Œil errant")
def oeil_errant(t):
    t.mat("b", ["8a7a80", "c0b0b4", "e8dcdc", "f8f0f0", "ffffff"])   # blanc de l'oeil
    t.mat("i", ["5a2a08", "a85a10", "e89020", "f8c860", "fff0c0"], ["082a5a", "105aa8", "2090e8", "60c8f8", "c0f0ff"])   # iris
    t.mat("m", "6a4a7a", "4a7a6a")   # ailes
    t.mat("r", SANG, FEU_VIOLET)     # veines, nerf
    _aile_membrane(t, "m", "r", (32, 18), (38, 5), [(46, 4), (47, 15), (43, 22), (38, 21)], dl=-1)
    _aile_membrane(t, "m", "r", (16, 18), (10, 5), [(2, 4), (1, 15), (5, 22), (10, 21)])
    # le nerf optique, en queue
    t.p(S([(26, 32, 2.4), (29, 38, 1.8), (26, 43, 1.4), (29, 46, 0.8)]), "r", trait=0)
    # le globe
    t.p(E(24, 22, 12, 11.5), "b", trait=0)
    # veines
    for pts in ([(13, 20), (16, 22), (15, 26)], [(35, 19), (32, 22), (34, 26)], [(20, 32), (22, 29)], [(29, 12), (28, 15)],
                [(17, 14), (19, 17)]):
        t.p(LS(pts), "r", trait=0, ton=2)
    # iris, pupille fendue, reflet
    t.p(E(23, 22, 6.4, 6.4), "i", trait=0)
    t.p(E(23, 22, 6.4, 6.4) - E(23, 22, 5.4, 5.4), "i", trait=0, ton=0)
    t.p(E(23, 22, 1.6, 4.6), "o", trait=0, ton=2)
    t.px([(20, 19), (21, 19), (20, 20)], "b", 4)
    # paupiere du dessus
    t.p(B([(12, 18), (17, 11), (24, 9), (31, 11), (36, 18), (30, 14), (24, 13), (18, 14)]), "m", trait=0, dl=1)


@sprite("archonte-foudroye", "Archonte foudroyé")
def archonte(t):
    t.mat("r", ["1c2a5a", "30468c", "4c6cbc", "84a0e0", "c4d4f8"], ["5a1c2a", "8c3046", "bc4c6c", "e084a0", "f8c4d4"])   # toge
    t.mat("p", ["8a8a90", "b8b8be", "dcdce2", "f2f2f6", "ffffff"], ["3a3a44", "5c5c6a", "84849a", "b0b0c8", "dcdcf0"])   # ailes
    t.mat("c", PEAU)
    t.mat("g", OR)
    t.mat("e", ["8a6a10", "c8a018", "f8d830", "fff080", "ffffd8"], FEU_BLEU)
    t.mat("n", "2a2630")             # brulures
    # ailes : l'une intacte, l'autre calcinee
    for i, (x, y) in enumerate(((6, 6), (3, 13), (3, 20), (6, 26), (10, 31))):
        t.p(C(20, 18, x, y, 3, 1.3), "p", trait=0, dl=-(i % 2))
    for i, (x, y) in enumerate(((41, 7), (44, 14), (43, 21), (40, 26))):
        t.p(C(28, 18, x, y, 2.2, 0.9), "n", trait=0, dl=(i % 2))
    t.px([(40, 10), (43, 17), (41, 23)], "e", 2)
    # la foudre qui le traverse
    t.p(LS([(34, -2), (30, 6), (33, 7), (28, 16)]), "e", trait=0, ton=3)
    t.p(LS([(35, -2), (31, 6), (34, 7), (29, 16)]), "e", trait=0, ton=4)
    # toge de magistrat celeste
    t.p(B([(18, 18), (24, 15), (30, 18), (33, 32), (35, 45), (13, 45), (15, 32)]), "r", trait=0)
    for x in (17, 21, 27, 31):
        t.p(L(x, 26, x + (x - 24) // 4, 44), "r", trait=0, ton=0, sur="r")
    t.p(P([(19, 18), (24, 22), (29, 18), (30, 21), (24, 26), (18, 21)]), "g", trait=0)
    t.px([(15, 45), (20, 44), (28, 44), (33, 45)], "g", 2)
    # bras : sceptre de justice brise
    t.p(K([(19, 20, 2), (13, 25, 1.7), (11, 21, 1.5)]), "r", trait=1)
    t.p(E(10.5, 19.5, 1.6, 1.6), "c", trait=0)
    t.p(C(10, 22, 11, 8, 0.9), "g", trait=0)
    t.p(LS([(11, 8), (9, 6), (12, 5)]), "g", trait=0, ton=4)
    t.p(K([(29, 20, 2), (34, 26, 1.7), (33, 32, 1.5)]), "r", trait=0, dl=-1)
    # tete, laurier d'or, yeux blancs de foudre
    t.p(E(24, 11, 4, 4.4), "c", trait=0)
    t.px([(22, 11), (26, 11)], "e", 4); t.px([(22, 10), (26, 10)], "o")
    t.px([(23, 14), (24, 14), (25, 14)], "c", 0)
    t.p(E(24, 8, 4.6, 2.4) - E(24, 9.5, 4, 2.4), "g", trait=0, ton=3)
    t.px([(27, 12), (28, 13), (27, 15)], "n", 2)
    t.px([(14, 38), (36, 36), (30, 30)], "e", 3)


@sprite("muse-de-cristal", "Muse de cristal")
def muse(t):
    t.mat("c", ["3a6a8a", "6aa0c0", "a0d0e8", "d0ecf8", "f6feff"], ["8a3a6a", "c06aa0", "e8a0d0", "f8d0ec", "fff6fe"])   # cristal
    t.mat("g", OR)
    t.mat("s", ["8a8ab0", "b0b0d0", "d0d0ea", "eaeaf8", "ffffff"])
    # robe en prismes facettes
    t.p(P([(19, 20), (29, 20), (34, 34), (36, 45), (12, 45), (14, 34)]), "c", trait=0)
    for a, b in (((24, 20), (24, 45)), ((19, 20), (15, 45)), ((29, 20), (33, 45)), ((14, 34), (24, 30)), ((34, 34), (24, 30)),
                 ((17, 26), (31, 26))):
        t.p(L(a[0], a[1], b[0], b[1]), "c", trait=0, ton=1, sur="c")
    t.px([(16, 40), (20, 30), (22, 36), (18, 23), (21, 44)], "c", 4)
    # buste taille a facettes
    t.p(P([(20, 18), (24, 16), (28, 18), (29, 24), (24, 27), (19, 24)]), "c", trait=0, dl=1)
    t.p(L(24, 16, 24, 27), "c", trait=0, ton=1)
    # la lyre d'or
    t.p(CS([(6, 16), (4, 24), (8, 31)]) | CS([(16, 16), (18, 24), (14, 31)]), "g", trait=0)
    t.p(R(7, 31, 15, 32) | R(5, 15, 17, 16), "g", trait=0)
    for x in (8, 10, 12, 14):
        t.p(R(x, 17, x, 30), "s", trait=0, ton=3)
    t.p(K([(20, 20, 1.5), (16, 25, 1.2), (12, 24, 1.1)]), "c", trait=1)
    t.p(K([(28, 20, 1.5), (31, 27, 1.2), (27, 30, 1.1)]), "c", trait=0, dl=-1)
    # tete : chignon de cristal, visage lisse
    t.p(P([(20, 9), (22, 3), (27, 2), (30, 7), (29, 12)]), "c", trait=0, dl=-1)
    t.p(E(24, 11.5, 3.6, 4.2), "c", trait=0, dl=1)
    t.px([(22, 11), (26, 11)], "o"); t.px([(23, 14), (24, 14), (25, 14)], "c", 0)
    t.px((22, 9), "c", 4)
    # notes de cristal qui s'envolent
    for x, y in ((36, 10), (41, 18), (38, 3)):
        t.p(E(x, y + 3, 1.6, 1.3), "c", trait=0, ton=3)
        t.p(R(x + 1, y - 2, x + 1, y + 3), "c", trait=0, ton=3)
        t.px((x + 2, y - 2), "c", 4)
    t.px([(42, 30), (3, 8), (40, 40)], "s", 4)


@sprite("chancelier-aux-mille-yeux", "Chancelier aux mille yeux")
def chancelier(t):
    t.mat("r", ["241a10", "443420", "685234", "947a58", "c4ac8c"], ["101a24", "203444", "345268", "587a94", "8cacc4"])   # robe de chancellerie
    t.mat("f", "e8dcc8", "c8c0d8")   # col d'hermine
    t.mat("c", ["5a5068", "8a80a0", "b8b0cc", "dcd6ea", "f6f2ff"])
    t.mat("y", ["7a1010", "c02020", "f04a30", "f8a060", "fff0c0"], FEU_BLEU)
    t.mat("g", OR)
    t.mat("p", ["8a7a5a", "b8a880", "dcd0a8", "f2ead0", "fffcf0"])
    # robe tres ample, semee d'yeux ouverts
    t.p(B([(14, 18), (24, 14), (34, 18), (39, 32), (42, 45), (6, 45), (9, 32)]), "r", trait=0)
    for x, y in ((14, 28), (22, 25), (31, 27), (37, 34), (11, 37), (19, 34), (27, 35), (16, 42), (24, 41), (33, 42), (39, 41),
                 (9, 43)):
        t.p(E(x, y, 2.2, 1.5), "c", trait=0, ton=4)
        t.px((x, y), "y", 2); t.px((x, y - 1), "o")
    # col d'hermine, chaine d'office
    t.p(B([(15, 18), (24, 15), (33, 18), (31, 22), (24, 20), (17, 22)]), "f", trait=0)
    t.px([(18, 19), (22, 18), (26, 18), (30, 19)], "o")
    t.p(CS([(19, 22), (24, 27), (29, 22)]), "g", trait=0, ton=3)
    t.p(E(24, 28, 1.8, 1.8), "g", trait=0)
    # mains : un sceau et un rouleau
    t.p(K([(16, 21, 2.4), (10, 26, 2.2), (7, 24, 1.9)]), "r", trait=1)
    t.p(E(6, 23, 1.8, 1.8), "c", trait=0)
    t.p(R(3, 16, 9, 21), "p", trait=0); t.px([(4, 18), (6, 18), (8, 18), (5, 20), (7, 20)], "p", 0)
    t.p(E(6, 22, 1.2, 1.2), "y", trait=0, ton=1)
    t.p(K([(32, 21, 2.4), (38, 26, 2.2), (41, 23, 1.9)]), "r", trait=1, dl=-1)
    t.p(E(42, 22, 1.8, 1.8), "c", trait=0)
    # tete : crane haut, constelle d'yeux, mortier de juge
    t.p(B([(18, 14), (18, 6), (24, 2), (30, 6), (30, 14), (27, 17), (21, 17)]), "c", trait=0)
    for x, y in ((21, 12), (27, 12), (24, 8), (20, 7), (28, 7), (24, 14)):
        t.px((x, y), "y", 3); t.px((x, y - 1), "o")
    t.p(R(18, 1, 30, 3), "r", trait=0, dl=-1)
    t.p(R(20, -2, 28, 1), "r", trait=0)
    t.px([(24, -1), (24, 0)], "g", 3)
