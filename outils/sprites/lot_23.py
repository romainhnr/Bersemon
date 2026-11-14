from dessin import *
from matieres import *
from atelier import sprite
import math

GRES = ["4c4844", "726c66", "98928a", "beb8ae", "e2ded4"]
GRES_SHINY = ["444c48", "66726c", "8a9892", "aebeb8", "d4e2de"]
LUEUR = ["a87a10", "e0b020", "f8dc50", "fff4a0", "ffffe8"]
CRISTAL = ["3a6a8a", "6aa0c0", "a0d0e8", "d0ecf8", "f6feff"]


@sprite("chevalier-errant", "Chevalier errant")
def chevalier_errant(t):
    t.mat("a", ["3a3a34", "62625a", "8c8c82", "b6b6aa", "deded4"], ["34343a", "5a5a62", "82828c", "aaaab6", "d4d4de"])   # armure terne, cabossee
    t.mat("c", ["3a1c14", "64301e", "8c4a2c", "b47a54", "dcb494"], ["141c3a", "1e3064", "2c4a8c", "547ab4", "94b4dc"])   # cape rapiecee
    t.mat("r", ["3a1c10", "6a3418", "a05424", "d08440", "f0bc80"])   # rouille
    t.mat("b", BOIS)
    # cape en loques
    t.p(B([(22, 16), (32, 15), (38, 24), (40, 38), (36, 34), (33, 40), (29, 34), (26, 38)]), "c", trait=0, dl=-1)
    # jambes en marche
    t.p(K([(27, 32, 2.6), (31, 38, 2.2), (30, 44, 2)]), "a", trait=0, dl=-1)
    t.p(K([(21, 32, 2.6), (17, 38, 2.2), (17, 44, 2)]), "a", trait=0)
    t.p(R(13, 44, 19, 45) | R(28, 44, 34, 45), "a", trait=0, dl=-1)
    # cuirasse, taches de rouille
    t.p(B([(16, 17), (24, 14), (31, 17), (32, 27), (29, 34), (18, 34), (15, 27)]), "a", trait=0)
    t.px([(19, 20), (20, 21), (27, 25), (28, 26), (22, 30), (18, 27)], "r", 2)
    t.px([(24, 17), (24, 20), (24, 23), (24, 26)], "a", 0)
    # ecu fendu au bras, epee ebrechee qui traine
    t.p(K([(17, 19, 2.4), (12, 25, 2), (10, 30, 1.8)]), "a", trait=1)
    t.p(P([(3, 18), (13, 17), (14, 28), (8, 35), (3, 28)]), "b", trait=1)
    t.p(LS([(8, 18), (9, 24), (7, 28), (8, 34)]), "o", trait=0, ton=2)
    t.px([(4, 19), (12, 19), (5, 27)], "a", 4)
    t.p(K([(31, 19, 2.4), (35, 26, 2), (37, 31, 1.8)]), "a", trait=0, dl=-1)
    t.p(C(37, 32, 45, 45, 0.9), "a", trait=0, ton=3)
    t.p(R(35, 31, 40, 32), "r", trait=0)
    t.px([(41, 38), (43, 41)], "o")
    # heaume ferme, plumet casse, tete basse
    t.p(B([(18, 15), (18, 7), (23, 4), (28, 6), (29, 15), (24, 17)]), "a", trait=0)
    t.p(R(18, 10, 25, 11), "o", trait=0, ton=2)
    t.px([(19, 13), (21, 13), (23, 13)], "a", 0)
    t.p(LS([(26, 5), (31, 2), (33, 6)]), "c", trait=0, ton=3)


@sprite("cerf-de-pierre", "Cerf de pierre")
def cerf_de_pierre(t):
    t.mat("g", GRES, GRES_SHINY)
    t.mat("l", "5a8a4a", "8a6aa8")
    t.mat("y", LUEUR, FEU_BLEU)
    # couche, pattes repliees sous lui
    t.p(B([(10, 30), (18, 24), (32, 24), (42, 28), (44, 37), (38, 42), (16, 42), (9, 38)]), "g", trait=0)
    t.p(K([(15, 38, 2.4), (8, 41, 2), (3, 43, 1.6)]), "g", trait=1)
    t.p(K([(36, 38, 3), (28, 42, 2.2), (22, 43, 1.8)]), "g", trait=1)
    t.px([(1, 44), (2, 44), (20, 44), (21, 44)], "o")
    # cassures de la pierre
    t.p(LS([(20, 25), (23, 31), (20, 37), (23, 42)]) | LS([(33, 25), (31, 32), (35, 38)]) | LS([(23, 31), (31, 32)]), "g",
        trait=0, ton=0)
    # encolure droite, tete levee qui ecoute
    t.p(B([(10, 32), (8, 20), (11, 12), (17, 14), (19, 26)]), "g", trait=0)
    t.p(B([(2, 14), (5, 8), (12, 7), (15, 11), (11, 16), (4, 17)]), "g", trait=0)
    t.p(P([(12, 8), (17, 6), (14, 11)]), "g", trait=0, dl=-1)
    t.px([(7, 11), (8, 11)], "y", 3); t.px((7, 11), "y", 4)
    t.px([(2, 15), (2, 16)], "o")
    # bois : des branches de pierre, l'une cassee
    for pts in ([(11, 7), (9, 1), (4, -2)], [(9, 1), (10, -4)], [(6, -1), (2, 2)], [(13, 7), (17, 1), (23, -2)], [(17, 1), (15, -4)],
                [(20, -1), (24, 2)]):
        t.p(K([(x, y, 1) for x, y in pts]), "g", trait=0, dl=1)
    t.px([(24, 3), (25, 4)], "o")
    # lichen sur le dos
    for x, y in ((20, 24), (26, 23), (33, 24), (39, 27), (14, 27)):
        t.px([(x, y), (x + 1, y), (x, y - 1)], "l", 2)
    t.px([(26, 22), (39, 26)], "l", 4)
    t.grain("g", 0.08, 18)


@sprite("lezard-petrifie", "Lézard pétrifié")
def lezard_petrifie(t):
    t.mat("g", GRES, GRES_SHINY)
    t.mat("v", ["24401c", "406a2c", "66984a", "98c478", "ccecb4"], ["401c40", "6a2c6a", "984a98", "c478c4", "ecb4ec"])   # chair encore vive
    t.mat("p", FEU_VERT, FEU_VIOLET)
    t.mat("y", "f8e040", "f04848")
    # queue de pierre, cassee au bout
    t.p(S([(34, 34, 3.4), (41, 34, 2.8), (45, 29, 2)]), "g", trait=0)
    t.p(P([(44, 27), (47, 24), (46, 30)]), "g", trait=0, ton=0)
    # pattes ecartees : celles de derriere sont deja de pierre
    t.p(K([(31, 36, 2.4), (36, 40, 1.8), (40, 44, 1.4)]), "g", trait=0, dl=-1)
    t.p(K([(17, 35, 2.2), (12, 39, 1.6), (8, 43, 1.3)]), "v", trait=0, dl=-1)
    # corps trapu : la pierre gagne depuis l'arriere
    t.p(B([(10, 30), (17, 25), (28, 25), (36, 29), (37, 36), (29, 39), (17, 39), (10, 36)]), "v", trait=0)
    t.p(B([(23, 25), (28, 25), (36, 29), (37, 36), (29, 39), (24, 39), (26, 32)]), "g", trait=0)
    t.p(LS([(23, 25), (26, 29), (24, 33), (26, 36), (24, 39)]), "o", trait=0, ton=2)
    t.px([(29, 28), (33, 31), (30, 35)], "g", 0)
    t.p(K([(29, 37, 2.6), (33, 41, 1.9), (37, 45, 1.4)]), "g")
    t.p(K([(15, 36, 2.4), (10, 40, 1.8), (5, 44, 1.3)]), "v")
    t.px([(3, 45), (4, 45), (38, 46), (39, 46)], "o")
    # tete levee, collerette deployee, il crache du venin
    t.p(S([(13, 30, 3.4), (11, 23, 2.8), (12, 17, 2.6)]), "v", trait=0)
    t.p(E(13, 15, 7.5, 6.5), "v", trait=0, dl=-1)
    for a in range(-150, 60, 30):
        t.p(L(13, 15, 13 + 7.4 * math.cos(math.radians(a)), 15 + 6.4 * math.sin(math.radians(a))), "v", trait=0, ton=0)
    t.p(B([(3, 16), (7, 11), (14, 11), (17, 15), (15, 19), (8, 20)]), "v", trait=0, dl=1)
    t.px([(9, 14), (10, 14)], "y", 3); t.px((9, 14), "o")
    t.px([(4, 17), (5, 18), (6, 18), (7, 18)], "o")
    t.px([(2, 19), (0, 21), (1, 24)], "p", 3)
    t.grain("g", 0.08, 19)


@sprite("dryade-de-pierre", "Dryade de pierre")
def dryade_de_pierre(t):
    t.mat("g", GRES, GRES_SHINY)
    t.mat("l", ["24401c", "406a2c", "66984a", "98c478", "ccecb4"], ["401c40", "6a2c6a", "984a98", "c478c4", "ecb4ec"])   # dernieres feuilles
    t.mat("y", LUEUR, FEU_BLEU)
    # elle est assise, genoux ramenes, changee en statue au pied de son arbre
    t.p(B([(26, 10), (32, 6), (40, 8), (43, 20), (44, 40), (40, 45), (30, 45), (28, 30)]), "g", trait=0, dl=-1)
    for a, b in (((33, 8), (35, 44)), ((38, 9), (40, 44)), ((29, 14), (31, 44))):
        t.p(L(a[0], a[1], b[0], b[1]), "g", trait=0, ton=0)
    t.p(S([(40, 9, 1.6), (45, 4, 1.1), (46, -1, 0.7)]) | S([(32, 7, 1.6), (28, 1, 1.1)]), "g", trait=0, dl=-1)
    # racines
    t.p(S([(30, 44, 2), (22, 45, 1.5), (16, 46, 0.8)]) | S([(42, 44, 2), (46, 46, 1)]), "g", trait=0, dl=-1)
    # corps replie
    t.p(B([(16, 24), (22, 21), (28, 24), (30, 34), (27, 42), (16, 42), (13, 34)]), "g", trait=0)
    t.p(K([(18, 34, 3), (11, 33, 2.6), (9, 40, 2.2)]), "g", trait=1)
    t.p(R(6, 41, 12, 43), "g", trait=0)
    # bras noues autour des genoux
    t.p(K([(18, 25, 2), (12, 30, 1.7), (8, 32, 1.5)]), "g", trait=1)
    t.p(K([(26, 26, 2), (20, 32, 1.7), (13, 34, 1.5)]), "g", trait=1)
    # tete penchee sur les genoux, chevelure de pierre, une larme de lumiere
    t.p(B([(15, 18), (17, 11), (24, 9), (29, 13), (31, 24), (26, 19), (18, 20), (13, 24)]), "g", trait=0, dl=-1)
    t.p(E(20, 17, 4, 4.4), "g", trait=0, dl=1)
    t.p(LS([(17, 17), (19, 18), (21, 17)]), "o", trait=0, ton=2)
    t.px([(18, 19), (18, 21), (17, 23)], "y", 3); t.px((18, 20), "y", 4)
    t.px([(19, 21), (20, 21)], "g", 0)
    # quelques feuilles vertes tiennent encore
    for x, y in ((24, 8), (28, 10), (30, 4), (44, 3), (36, 5)):
        t.p(P([(x - 2, y), (x, y - 2), (x + 2, y), (x, y + 2)]), "l", trait=0)
    t.px([(24, 7), (36, 4)], "l", 4)
    t.grain("g", 0.08, 20)


@sprite("arbre-roi-petrifie", "Arbre-roi pétrifié")
def arbre_roi(t):
    t.mat("g", GRES, GRES_SHINY)
    t.mat("c", CRISTAL, ["8a3a6a", "c06aa0", "e8a0d0", "f8d0ec", "fff6fe"])   # feuillage change en cristal
    t.mat("y", FEU_VIOLET, FEU_VERT)
    t.mat("o2", OR)
    # racines en griffes
    for pts in ([(18, 39, 3.4), (10, 43, 2.4), (3, 45, 1)], [(30, 39, 3.4), (38, 43, 2.4), (45, 45, 1)], [(24, 41, 2.8), (24, 46, 1.4)]):
        t.p(S(pts), "g", trait=0, dl=-1)
    # branches maitresses en bras leves
    t.p(S([(17, 22, 3), (10, 16, 2.4), (6, 8, 1.6)]), "g", trait=0)
    t.p(S([(31, 22, 3), (38, 16, 2.4), (42, 8, 1.6)]), "g", trait=0, dl=-1)
    # houppier : des bouquets de cristaux
    for x, y, h in ((6, 8, 9), (11, 6, 7), (2, 12, 6), (42, 8, 9), (37, 6, 7), (46, 12, 6), (18, 6, 8), (30, 6, 8), (24, 3, 9)):
        t.p(P([(x - 2, y + 1), (x - 2, y - h + 3), (x, y - h), (x + 2, y - h + 3), (x + 2, y + 1)]), "c", trait=0, dl=-(x % 2))
        t.px((x - 1, y - h + 4), "c", 4)
    # tronc enorme, visage de vieux roi dans l'ecorce de pierre
    t.p(B([(14, 41), (15, 28), (13, 20), (18, 13), (30, 13), (35, 20), (33, 28), (34, 41)]), "g", trait=0)
    t.p(P([(19, 21), (22, 22), (20, 24)]) | P([(29, 21), (26, 22), (28, 24)]), "o", trait=0, ton=2)
    t.px([(20, 22), (28, 22)], "y", 4)
    t.p(P([(23, 23), (24, 28), (25, 23)]), "g", trait=0, ton=4)
    t.p(LS([(20, 31), (22, 30), (26, 30), (28, 31)]), "o", trait=0, ton=2)
    for x in (19, 22, 25, 28):
        t.p(L(x, 32, x + (x - 24) // 3, 39), "g", trait=0, ton=1)
    # couronne
    t.p(R(18, 13, 30, 15), "o2", trait=0)
    for x, h in ((18, 3), (21, 2), (24, 4), (27, 2), (30, 3)):
        t.p(R(x, 13 - h, x, 13), "o2", trait=0, ton=3)
    t.px([(16, 30), (17, 34), (32, 27), (31, 36)], "g", 0)
    t.grain("g", 0.07, 21)


@sprite("moine-de-lumiere", "Moine de lumière")
def moine_de_lumiere(t):
    t.mat("r", ["8a7a48", "c0b070", "e8dca0", "f8f0cc", "fffcf0"], ["485a8a", "7088c0", "a0b8e8", "ccdcf8", "f0f6ff"])   # robe blanche et or
    t.mat("y", LUEUR, FEU_BLEU)
    t.mat("g", OR)
    t.mat("c", PEAU)
    # rayons derriere lui
    for a in range(-170, 0, 20):
        t.p(C(24, 12, 24 + 20 * math.cos(math.radians(a)), 12 + 14 * math.sin(math.radians(a)), 1.2, 0.3), "y", trait=0, ton=2)
    t.p(E(24, 11, 8, 8), "y", trait=0, ton=3)
    # en levitation, jambes croisees
    t.p(B([(12, 34), (18, 30), (30, 30), (36, 34), (34, 39), (24, 41), (14, 39)]), "r", trait=0, dl=-1)
    t.px([(16, 36), (22, 38), (27, 38), (32, 36)], "r", 0)
    t.p(E(24, 45, 9, 1.4), "y", trait=0, ton=2)
    # robe
    t.p(B([(17, 19), (24, 16), (31, 19), (33, 30), (30, 35), (18, 35), (15, 30)]), "r", trait=0)
    t.p(R(23, 19, 25, 34), "g", trait=0, ton=2)
    # mains ouvertes, une sphere de lumiere dans chaque paume
    t.p(K([(18, 21, 2.2), (12, 26, 1.9), (8, 24, 1.7)]) | K([(30, 21, 2.2), (36, 26, 1.9), (40, 24, 1.7)]), "r", trait=1)
    for x in (6, 42):
        t.p(E(x, 21, 2.6, 2.6), "y", trait=0, ton=3); t.px((x, 21), "y", 4)
    # tete rasee, yeux clos qui rayonnent
    t.p(E(24, 11, 4.2, 4.6), "c", trait=0)
    t.p(LS([(21, 11), (22, 12), (23, 11)]) | LS([(25, 11), (26, 12), (27, 11)]), "y", trait=0, ton=4)
    t.px([(23, 14), (24, 14), (25, 14)], "c", 0)
    t.px((24, 8), "y", 4)
    t.px([(8, 38), (40, 38), (4, 30), (44, 30)], "y", 3)


@sprite("chimere-de-cristal", "Chimère de cristal")
def chimere_de_cristal(t):
    t.mat("c", CRISTAL, ["8a3a6a", "c06aa0", "e8a0d0", "f8d0ec", "fff6fe"])
    t.mat("y", LUEUR, FEU_BLEU)
    # assise comme un sphinx de verre : arriere-train, patte posee
    t.p(S([(38, 40, 1.6), (44, 37, 1.4), (46, 30, 1.2), (43, 25, 1)]), "c", trait=0)
    t.p(P([(41, 26), (43, 20), (46, 25)]), "c", trait=0, ton=4)
    t.p(P([(18, 22), (30, 20), (38, 28), (40, 44), (16, 44), (14, 32)]), "c", trait=0)
    t.p(P([(27, 32), (36, 30), (40, 44), (28, 44)]), "c", trait=1, dl=1)
    t.p(P([(15, 30), (21, 30), (22, 44), (14, 44)]), "c", trait=1)
    for a, b in (((18, 22), (22, 34)), ((30, 20), (27, 32)), ((22, 34), (27, 32)), ((22, 34), (24, 44))):
        t.p(L(a[0], a[1], b[0], b[1]), "c", trait=0, ton=1)
    t.px([(16, 34), (30, 34), (20, 24)], "c", 4)
    # trois tetes facettees : lion, chevre, serpent
    t.p(P([(6, 14), (12, 8), (20, 10), (22, 18), (18, 24), (9, 24), (5, 19)]), "c", trait=0)
    t.p(P([(4, 19), (10, 17), (12, 22), (7, 24)]), "c", trait=0, ton=4)
    t.px([(13, 14), (14, 14)], "y", 3); t.px((13, 14), "y", 4)
    for x, y in ((10, 8), (15, 6), (20, 8), (23, 13)):
        t.p(P([(x - 1, y + 2), (x, y - 3), (x + 2, y + 2)]), "c", trait=0, dl=-1)
    t.p(P([(24, 12), (28, 6), (33, 8), (34, 14), (29, 18)]), "c", trait=0, dl=-1)
    t.p(L(29, 6, 27, -1) | L(32, 7, 34, 0), "c", trait=0, ton=4)
    t.px((30, 11), "y", 4)
    t.p(S([(34, 22, 1.8), (39, 16, 1.6), (38, 9, 1.4)]), "c", trait=0, dl=-1)
    t.p(P([(35, 8), (38, 4), (41, 8), (38, 11)]), "c", trait=0)
    t.px((38, 7), "y", 4)
    # la lumiere s'y decompose
    t.px([(24, 28), (25, 29), (26, 30)], "y", 2)
    t.px([(2, 8), (44, 12), (4, 34)], "y", 4)


@sprite("ver-luisant-geant", "Ver luisant géant")
def ver_luisant(t):
    t.mat("c", ["2a2018", "4a3a28", "70583c", "9c8058", "c8ac80"], ["18202a", "283a4a", "3c5870", "58809c", "80acc8"])   # segments bruns
    t.mat("l", ["3a7a10", "7ac020", "c0f040", "ecff90", "fbffe0"], ["7a3a10", "c07a20", "f0c040", "ffec90", "fffbe0"])   # lanterne
    t.mat("n", "24202a")
    # la lanterne de l'abdomen, halo
    t.p(E(36, 34, 9.5, 8.5), "l", trait=0, ton=1)
    t.p(E(36, 34, 7, 6), "l", trait=0, ton=2)
    t.p(E(36, 34, 4.4, 3.8), "l", trait=0, ton=3)
    t.p(E(36, 34, 2, 1.8), "l", trait=0, ton=4)
    # corps plat en ecailles qui se chevauchent
    corps = [(34, 32, 5.6), (27, 30, 6), (20, 29, 5.6), (13, 30, 5)]
    for i, (x, y, r) in enumerate(corps):
        t.p(E(x, y, r, r * 0.9), "c", trait=1, dl=-(i % 2))
        t.px((x - 2, y - 3), "c", 4)
    t.p(E(36, 36, 5, 3.4), "l", trait=0, ton=3)
    t.px((36, 36), "l", 4)
    # six pattes courtes
    for x in (12, 17, 22, 27):
        t.p(K([(x, 34, 1), (x - 2, 39, 0.8)]), "n", trait=0, ton=3)
    # tete sous un bouclier, antennes, mandibules
    t.p(B([(3, 30), (6, 24), (12, 23), (15, 28), (13, 33), (6, 34)]), "c", trait=0, dl=1)
    t.p(E(6, 31, 2.6, 2.2), "n", trait=0, ton=3)
    t.px((5, 30), "l", 4)
    t.p(CS([(6, 25), (3, 20), (0, 19)]) | CS([(9, 24), (8, 18), (5, 15)]), "n", trait=0, ton=3)
    t.px([(3, 33), (4, 34)], "n", 2)
    # lucioles qui l'accompagnent
    for x, y in ((14, 10), (28, 6), (40, 14), (6, 44), (22, 44), (44, 46)):
        t.px((x, y), "l", 4); t.px([(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)], "l", 2)
