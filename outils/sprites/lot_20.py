from dessin import *
from matieres import *
from atelier import sprite
from commun import _aile_membrane
import math


@sprite("aile-de-l-eclipse", "Aile de l'Éclipse")
def aile_de_l_eclipse(t):
    t.mat("n", ["08070e", "16141f", "282634", "444250", "74728a"], ["0e0807", "1f1614", "342826", "504442", "8a7472"])   # cuir de nuit
    t.mat("m", ["1a1028", "30204a", "4a3470", "7458a0", "aa90d0"], ["28101a", "4a2030", "70344a", "a05874", "d090aa"])   # membrane
    t.mat("e", ["3a0a0a", "8a1a14", "e04a1c", "f8b03c", "fff4c0"], FEU_BLEU)   # couronne de l'eclipse
    t.mat("y", SANG, FEU_BLEU)
    # l'astre noir derriere sa tete
    t.p(E(24, 9, 8.6, 8.6), "e", trait=0, ton=3)
    t.p(E(24, 9, 7.2, 7.2), "n", trait=0, ton=0)
    # ailes de faucon de nuit, immenses
    _aile_membrane(t, "m", "n", (28, 17), (36, 3), [(46, 2), (47, 14), (45, 26), (40, 34), (33, 24)], dl=-1)
    _aile_membrane(t, "m", "n", (20, 17), (12, 3), [(2, 2), (1, 14), (3, 26), (8, 34), (15, 24)])
    # corps mince, cuirasse lisse, jambes jointes en pointe
    t.p(B([(19, 16), (24, 14), (29, 16), (30, 26), (27, 36), (25, 46), (23, 46), (21, 36), (18, 26)]), "n", trait=0)
    t.px([(24, 18), (24, 22), (24, 26), (22, 30), (26, 30), (24, 36)], "n", 4)
    t.p(K([(20, 18, 1.6), (15, 24, 1.3), (17, 30, 1.1)]) | K([(28, 18, 1.6), (33, 24, 1.3), (31, 30, 1.1)]), "n", trait=1)
    # heaume en bec de rapace, regard rouge
    t.p(B([(19, 12), (21, 5), (24, 3), (27, 5), (29, 12), (24, 16)]), "n", trait=0, dl=1)
    t.p(P([(22, 10), (24, 15), (26, 10)]), "n", trait=0, ton=4)
    t.px([(21, 9), (22, 9), (26, 9), (27, 9)], "y", 3)
    t.p(P([(21, 5), (18, -1), (23, 4)]) | P([(27, 5), (30, -1), (25, 4)]), "n", trait=0, ton=3)
    t.px([(4, 40), (44, 40), (10, 44), (38, 44)], "e", 2)


@sprite("pleureuse", "Pleureuse")
def pleureuse(t):
    t.mat("v", ["14141c", "282834", "40404e", "64647a", "9a9ab0"], ["1c1414", "342828", "4e4040", "7a6464", "b09a9a"])   # voiles de deuil
    t.mat("s", SPECTRE, FEU_VIOLET)
    t.mat("l", ["3a6a9a", "6a9ac8", "a0c8ec", "d0e8fa", "f4fbff"])   # larmes
    # robe de deuil qui se dissout
    t.p(B([(16, 18), (24, 14), (32, 18), (36, 32), (34, 42), (30, 46), (27, 41), (24, 47), (21, 41), (18, 46), (14, 42), (12, 32)]),
        "v", trait=0)
    for x in (16, 20, 28, 32):
        t.p(L(x, 24, x + (x - 24) // 5, 42), "v", trait=0, ton=0, sur="v")
    # long voile sur la tete et les epaules
    t.p(B([(15, 20), (16, 8), (24, 3), (32, 8), (33, 20), (37, 34), (30, 26), (24, 24), (18, 26), (11, 34)]), "v", trait=0, dl=1)
    # visage de spectre sous le voile, bouche ouverte sur une plainte
    t.p(E(24, 13, 4.4, 5.2), "s", trait=0)
    t.p(E(22, 12, 1.2, 1.6) | E(26, 12, 1.2, 1.6), "o", trait=0, ton=2)
    t.p(E(24, 16.5, 1.4, 1.8), "o", trait=0, ton=2)
    # mains jointes, mouchoir
    t.p(K([(18, 22, 2.2), (17, 28, 2), (22, 30, 1.7)]) | K([(30, 22, 2.2), (31, 28, 2), (26, 30, 1.7)]), "v", trait=1)
    t.p(E(24, 30, 2.4, 2), "s", trait=0, dl=1)
    t.p(P([(22, 31), (26, 31), (25, 37), (23, 35)]), "l", trait=0, ton=4)
    # larmes qui tombent en pluie
    t.px([(22, 14), (22, 16), (26, 14), (26, 16), (26, 18)], "l", 3)
    for x, y in ((8, 20), (10, 28), (6, 36), (40, 18), (42, 28), (38, 38), (12, 42), (36, 44)):
        t.px([(x, y), (x, y + 1)], "l", 3); t.px((x, y - 1), "l", 4)


@sprite("mendiant-possede", "Mendiant possédé")
def mendiant(t):
    t.mat("h", "5a4e40", "404e5a")   # haillons
    t.mat("c", ["4a4038", "766858", "a0907c", "c8b8a0", "ece0cc"])   # peau grise
    t.mat("y", FEU_VIOLET, FEU_VERT)
    t.mat("b", BOIS)
    t.mat("a", ACIER)
    # bequille brandie comme une massue
    t.p(C(38, 28, 42, 2, 1.2), "b", trait=0)
    t.p(R(38, 0, 46, 2), "b", trait=0, dl=-1)
    # une jambe, un moignon bande
    t.p(K([(22, 32, 2.6), (20, 39, 2.2), (19, 44, 2)]), "h", trait=0)
    t.p(R(15, 44, 21, 45), "c", trait=0, dl=-1)
    t.p(K([(28, 32, 2.6), (30, 37, 2.4)]), "h", trait=0, dl=-1)
    t.p(E(30, 39, 2.6, 2.2), "c", trait=0, dl=1)
    # corps casse en deux, guenilles
    t.p(B([(15, 20), (22, 14), (31, 15), (35, 22), (34, 30), (30, 36), (18, 36), (14, 28)]), "h", trait=0)
    t.px([(17, 36), (20, 38), (24, 36), (27, 38), (31, 36), (20, 24), (27, 22), (23, 30)], "h", 0)
    t.px([(18, 22), (30, 27)], "c", 3)
    # bras : l'un tend la sebile, l'autre leve la bequille
    t.p(K([(17, 21, 2.2), (10, 26, 1.8), (5, 24, 1.5)]), "h", trait=1)
    t.p(E(4, 23, 1.6, 1.6), "c", trait=0)
    t.p(P([(0, 20), (7, 20), (6, 23), (1, 23)]), "a", trait=0)
    t.p(K([(33, 20, 2.2), (38, 22, 1.8), (39, 26, 1.6)]), "h", trait=1)
    # tete penchee, capuche trouee, les yeux d'un autre
    t.p(B([(13, 18), (14, 9), (20, 5), (26, 8), (27, 16), (22, 20)]), "h", trait=0, dl=1)
    t.p(E(19, 13, 3.8, 4.2), "c", trait=0)
    t.px([(17, 12), (20, 12)], "y", 4); t.px([(17, 11), (20, 11)], "o")
    t.px([(16, 16), (17, 15), (18, 15), (19, 15), (20, 16)], "o")
    t.px([(17, 17), (19, 17)], "c", 4)
    # la chose qui le possede lui sort du dos
    t.p(S([(28, 15, 1.6), (31, 9, 1.3), (29, 4, 1), (32, 1, 0.6)]), "y", trait=0, ton=2)
    t.px([(31, 9), (29, 4)], "y", 4)
    t.px([(34, 10), (26, 2)], "y", 3)


@sprite("epouvantail", "Épouvantail")
def epouvantail(t):
    t.mat("p", ["6a5a20", "a08c30", "d0bc48", "ece080", "fcf4c0"], ["4a4a4a", "767676", "a0a0a0", "c8c8c8", "ececec"])   # paille
    t.mat("t", "5a4a3a", "3a3a5a")   # toile de sac
    t.mat("v", "3a4a2e", "4a2e3a")   # veste
    t.mat("b", BOIS)
    t.mat("y", FEU, FEU_VERT)
    t.mat("n", "2a2630")
    # le pieu et la traverse
    t.p(R(23, 14, 25, 46), "b", trait=0)
    t.p(R(3, 18, 45, 19), "b", trait=0, dl=-1)
    # veste rapiecee, paille qui deborde
    t.p(B([(16, 17), (24, 15), (32, 17), (33, 30), (30, 37), (18, 37), (15, 30)]), "v", trait=0)
    t.p(R(20, 24, 23, 27), "t", trait=0); t.px([(20, 24), (23, 27), (21, 25)], "o")
    for x in (17, 20, 23, 26, 29):
        t.p(L(x, 37, x + (x - 24) // 3, 42), "p", trait=0, ton=2)
    t.px([(18, 41), (22, 43), (26, 43), (30, 41)], "p", 4)
    # bras en croix, mains de paille
    t.p(K([(17, 19, 2.6), (8, 18, 2.2)]) | K([(31, 19, 2.6), (40, 18, 2.2)]), "v", trait=1)
    for cx, s in ((6, -1), (42, 1)):
        for dy in (-3, 0, 3):
            t.p(L(cx, 18, cx + s * 4, 18 + dy), "p", trait=0, ton=3)
    # tete en sac de toile : sourire cousu, yeux de braise, chapeau troue
    t.p(E(24, 10, 5.4, 5.6), "t", trait=0)
    t.p(P([(20, 8), (23, 9), (21, 11)]) | P([(28, 8), (25, 9), (27, 11)]), "o", trait=0, ton=2)
    t.px([(21, 9), (27, 9)], "y", 3)
    t.p(LS([(20, 13), (22, 14), (24, 13), (26, 14), (28, 13)]), "o", trait=0, ton=2)
    t.px([(21, 12), (23, 15), (25, 12), (27, 15)], "o")
    t.p(E(24, 6, 8.6, 1.6), "n", trait=1)
    t.p(B([(19, 6), (21, 0), (27, -1), (29, 6)]), "n", trait=0)
    t.px((26, 2), "t", 2)
    # un corbeau sur l'epaule
    t.p(E(37, 14, 2.6, 2.2), "n", trait=0); t.p(E(35, 12, 1.6, 1.5), "n", trait=0)
    t.px((33, 12), "p", 2); t.px((35, 12), "y", 3)


@sprite("dame-blanche", "Dame blanche")
def dame_blanche(t):
    t.mat("w", ["7a84a0", "aab4cc", "d4dcec", "eef2fa", "ffffff"], ["a07a84", "ccaab4", "ecd4dc", "faeef2", "ffffff"])   # robe de brume
    t.mat("s", ["a8c8e0", "c8e0f0", "e0f0fa", "f2fbff", "ffffff"])
    t.mat("h", "c8d0e4", "e4d0c8")   # cheveux
    t.mat("l", ["3a6a9a", "6a9ac8", "a0c8ec", "d0e8fa", "f4fbff"], FEU)
    # traine de brume
    t.p(B([(18, 20), (24, 17), (30, 20), (34, 32), (40, 42), (34, 46), (28, 43), (22, 46), (15, 43), (10, 45), (13, 34)]), "w", trait=0)
    for x in (17, 21, 27, 31):
        t.p(L(x, 26, x + (x - 24) // 3, 43), "w", trait=0, ton=1, sur="w")
    t.px([(11, 44), (20, 45), (30, 45), (38, 43)], "w", 4)
    # corsage, bras tendu qui montre le chemin, lanterne pale
    t.p(B([(20, 19), (24, 17), (28, 19), (28, 26), (24, 28), (20, 26)]), "w", trait=0, dl=1)
    t.p(K([(20, 20, 1.5), (13, 21, 1.2), (7, 18, 1)]), "s", trait=1)
    t.p(R(5, 19, 5, 22), "h", trait=0, ton=1)
    t.p(E(5, 25, 2.6, 3), "l", trait=0, ton=3); t.px((5, 25), "l", 4)
    t.p(K([(28, 20, 1.5), (32, 27, 1.2), (30, 32, 1)]), "s", trait=0, dl=-1)
    # tete, cheveux denoues qui flottent, hennin de voile
    t.p(B([(19, 12), (24, 8), (29, 12), (34, 24), (38, 20), (33, 30), (28, 18), (20, 18), (16, 26)]), "h", trait=0)
    t.p(E(24, 13, 3.6, 4.2), "s", trait=0)
    t.px([(22, 13), (26, 13)], "o"); t.px([(23, 16), (24, 16), (25, 16)], "w", 0)
    t.p(P([(21, 9), (25, 0), (27, 9)]), "w", trait=0, dl=1)
    t.p(P([(25, 0), (31, 3), (33, 12), (29, 8)]), "s", trait=0, ton=3)
    t.px([(40, 30), (8, 34), (42, 12), (3, 10)], "l", 4)


@sprite("hibou-blafard", "Hibou blafard")
def hibou(t):
    t.mat("p", ["6a6258", "9a9284", "c4bcac", "e4ded0", "faf6ee"], ["58626a", "84929a", "acbcc4", "d0dee4", "eef6fa"])   # plumage pale
    t.mat("t", "8a7a66", "66788a")   # taches
    t.mat("y", ["8a5a08", "d09010", "f8c830", "ffe880", "fffcd8"], FEU_VIOLET)
    t.mat("b", "4a4640", "c8b060")
    t.mat("l", BOIS)
    # la branche
    t.p(S([(2, 41, 2), (16, 40, 2.2), (32, 41, 2.2), (46, 39, 1.8)]), "l", trait=0)
    t.p(S([(36, 40, 1), (42, 35, 0.7)]), "l", trait=0)
    # corps droit, en oeuf
    t.p(B([(14, 20), (16, 8), (24, 4), (32, 8), (34, 20), (33, 32), (28, 38), (20, 38), (15, 32)]), "p", trait=0)
    # ailes repliees
    t.p(B([(28, 16), (34, 18), (36, 30), (32, 37), (29, 28)]), "t", trait=1)
    t.p(B([(20, 16), (14, 18), (12, 30), (16, 37), (19, 28)]), "t", trait=1, dl=1)
    for x, y in ((21, 24), (24, 27), (27, 24), (22, 31), (26, 31), (24, 34)):
        t.p(P([(x - 1, y), (x, y + 2), (x + 1, y)]), "t", trait=0, ton=2)
    t.px([(20, 39), (21, 40), (22, 39), (26, 39), (27, 40), (28, 39)], "b", 2)
    # disque facial en coeur, grands yeux fixes
    t.p(B([(16, 12), (20, 7), (24, 10), (28, 7), (32, 12), (30, 19), (24, 22), (18, 19)]), "p", trait=0, dl=1)
    t.p(E(20, 13, 3.2, 3.2) | E(28, 13, 3.2, 3.2), "y", trait=2, ton=3)
    t.p(E(20, 13, 1.6, 1.6) | E(28, 13, 1.6, 1.6), "o", trait=0, ton=2)
    t.px([(19, 12), (27, 12)], "y", 4)
    t.p(P([(23, 16), (24, 20), (25, 16)]), "b", trait=0)
    t.p(P([(16, 9), (15, 3), (20, 7)]) | P([(32, 9), (33, 3), (28, 7)]), "t", trait=0)


@sprite("amanite-rampante", "Amanite rampante")
def amanite(t):
    t.mat("c", ["6a0c14", "a81c20", "e03a30", "f8806c", "ffd0c4"], ["0c146a", "1c20a8", "3a30e0", "806cf8", "d0c4ff"])   # chapeau rouge
    t.mat("p", ["8a8478", "b8b2a4", "dcd8cc", "f2f0e8", "ffffff"])   # pied, verrues blanches
    t.mat("v", "3f6a3c", "6a3a6a")
    t.mat("s", FEU_VERT, FEU_VIOLET) # spores
    t.mat("k", "2a1e22")
    # filaments qui lui servent de pattes
    for cote in (1, -1):
        c = lambda x: 24 + cote * x
        for x1, y1, x2, y2 in ((8, 40, 14, 45), (10, 36, 18, 42), (5, 43, 7, 47)):
            t.p(K([(c(4), 37, 1.6), (c(x1), y1, 1.1), (c(x2), y2, 0.6)]), "p", trait=0, dl=-1 if cote > 0 else 0)
    # pied renfle, volve, anneau
    t.p(B([(18, 24), (30, 24), (31, 34), (33, 41), (24, 44), (15, 41), (17, 34)]), "p", trait=0)
    t.p(B([(17, 27), (24, 25), (31, 27), (33, 30), (24, 29), (15, 30)]), "p", trait=1, dl=1)
    # visage dans le pied
    t.p(E(21, 34, 1.4, 1.8) | E(27, 34, 1.4, 1.8), "k", trait=0, ton=2)
    t.px([(21, 33), (27, 33)], "s", 4)
    t.p(LS([(21, 39), (23, 40), (25, 40), (27, 39)]), "k", trait=0, ton=2)
    # grand chapeau a verrues
    t.p(B([(3, 23), (8, 12), (17, 5), (31, 5), (40, 12), (45, 23), (36, 27), (24, 28), (12, 27)]), "c", trait=0)
    for x, y, r in ((12, 15, 2.2), (20, 9, 2), (29, 9, 2.4), (37, 16, 2), (24, 17, 2.6), (16, 22, 1.6), (33, 22, 1.8), (8, 21, 1.4),
                    (41, 22, 1.3)):
        t.p(E(x, y, r, r * 0.8), "p", trait=0)
    # lamelles
    for x in range(8, 42, 3):
        t.px((x, 26), "p", 1)
    # spores
    for x, y in ((6, 32), (10, 36), (40, 31), (43, 37), (2, 28), (46, 28)):
        t.px((x, y), "s", 3)


@sprite("ours-moussu", "Ours moussu")
def ours_moussu(t):
    t.mat("f", "5e4a38", "4a4a56")   # pelage brun
    t.mat("m", ["2a4a20", "447430", "6a9c48", "9cc474", "d0e8b0"], ["4a204a", "743074", "9c489c", "c474c4", "e8b0e8"])   # mousse
    t.mat("n", "3a2e26")
    t.mat("w", OS)
    t.mat("y", "f8e040", "40e0f0")
    t.mat("r", ROSE if False else ["5a0a1a", "96142a", "d02c44", "f06a78", "ffd0d4"], FEU_BLEU)
    t.p(K([(21, 33, 3.4), (21, 40, 2.8), (20, 44, 2.8)]), "f", dl=-1, trait=0)
    t.p(K([(38, 33, 3.6), (40, 40, 2.8), (39, 44, 2.8)]), "f", dl=-1, trait=0)
    # corps massif, bosse des epaules
    t.p(B([(10, 24), (16, 15), (27, 14), (40, 19), (45, 27), (43, 35), (34, 38), (19, 38), (11, 34)]), "f", trait=0)
    # tapis de mousse sur le dos, champignons, une fleur
    t.p(B([(15, 16), (27, 12), (40, 17), (45, 25), (38, 22), (28, 18), (18, 21)]), "m", trait=0)
    for x, y in ((20, 16), (27, 14), (34, 16), (40, 20), (24, 18), (31, 18)):
        t.px([(x, y), (x + 1, y - 1)], "m", 4)
    t.p(R(31, 9, 31, 13), "w", trait=0, ton=3); t.p(E(31, 8, 2.4, 1.6), "r", trait=0)
    t.p(R(37, 13, 37, 16), "w", trait=0, ton=3); t.p(E(37, 12, 1.8, 1.3), "r", trait=0)
    t.px([(22, 11), (22, 12), (22, 13)], "m", 1); t.px([(21, 10), (23, 10), (22, 9), (22, 11)], "y", 3)
    t.p(K([(16, 34, 3.8), (15, 40, 3), (14, 44, 3)]), "f")
    t.p(K([(34, 34, 4), (34, 40, 3), (33, 44, 3)]), "f")
    t.px([(11, 46), (13, 46), (15, 46), (30, 46), (32, 46), (34, 46)], "w", 3)
    # tete : museau clair, petites oreilles rondes
    t.p(E(13, 15, 2.4, 2.4), "f", trait=0, dl=-1)
    t.p(B([(2, 24), (5, 17), (12, 15), (17, 20), (17, 29), (10, 32), (4, 30)]), "f", trait=0)
    t.p(E(7, 16, 2.4, 2.4), "f", trait=0)
    t.p(B([(0, 25), (4, 22), (8, 24), (8, 29), (3, 30)]), "w", trait=0, dl=-1)
    t.px([(0, 25), (1, 25), (0, 26)], "o")
    t.px([(2, 29), (3, 30), (4, 30), (5, 30), (6, 29)], "o")
    t.px([(9, 21), (10, 21)], "y", 3); t.px((9, 21), "o")
    t.px([(22, 26), (28, 30), (36, 27), (25, 34)], "f", 0)
