from dessin import *
from matieres import *
from atelier import sprite


@sprite("mandragore", "Mandragore")
def mandragore(t):
    t.mat("r", "b8875a", "c9a0c8")   # racine
    t.mat("l", "4f9a3c", "3a9aa8")   # feuilles
    t.mat("b", "8a3f6a", "e0c040")   # fleur
    t.mat("k", "2a1e22", "2a1e22")   # creux des yeux et de la bouche
    ax = 23.5
    for pts in ([(23, 14), (16, 4), (12, 5), (18, 12)], [(22, 14), (10, 9), (7, 12), (17, 15)], [(24, 14), (23, 3), (26, 4), (26, 13)]):
        f = B(pts)
        t.p(f, "l", trait=0); t.p(mir(f, ax), "l", trait=0, dl=-1)
    t.p(B([(22, 13), (23, 2), (25, 2), (26, 13)]), "l", trait=1)
    t.p(E(23.5, 12.5, 3, 2.2), "b")
    t.p(B([(16, 18), (23, 14), (31, 18), (33, 27), (30, 35), (24, 37), (17, 35), (14, 27)]), "r", trait=0)
    g = S([(16, 24, 2.2), (10, 26, 1.8), (6, 31, 1.4), (4, 36, 0.8)])
    t.p(g, "r", trait=0); t.p(mir(g, ax), "r", trait=0, dl=-1)
    t.p(S([(8, 29, 1), (5, 28, 0.6)]) | mir(S([(8, 29, 1), (5, 28, 0.6)]), ax), "r", trait=0)
    j = S([(19, 34, 2.6), (17, 39, 2.0), (13, 43, 1.2), (10, 44, 0.7)])
    t.p(j, "r"); t.p(mir(j, ax), "r", dl=-1)
    t.p(S([(22, 36, 1.6), (22, 41, 1.0), (21, 44, 0.6)]) | S([(26, 36, 1.6), (27, 41, 1.0), (28, 44, 0.6)]), "r", dl=-1)
    t.p(E(19.5, 23, 2, 2.4) | E(27.5, 23, 2, 2.4), "k", ton=2, trait=0)
    t.p(E(23.5, 30, 2.6, 3.4), "k", ton=2, trait=0)
    t.px([(19, 22), (27, 22)], "b", 3)
    t.px([(16, 28), (17, 29), (30, 21), (31, 22), (18, 33), (29, 32), (22, 17), (25, 17)], "r", 1)
    t.px([(18, 19), (19, 18), (20, 18)], "r", 4)


@sprite("main-de-gloire", "Main de gloire")
def main_de_gloire(t):
    t.mat("r", "a8805c", "b89ac0")   # racine en forme de main
    t.mat("f", FEU, FEU_VERT)
    t.mat("c", "e8dcc0", "e8dcc0")   # cire
    t.mat("k", "2a1e22", "2a1e22")
    t.mat("l", "4f9a3c", "3a9aa8")
    # racines du poignet
    for pts in ([(21, 37, 2.4), (17, 40, 1.6), (12, 42, 0.8)], [(27, 37, 2.4), (31, 40, 1.6), (36, 42, 0.8)],
                [(24, 38, 2), (24, 41, 1.2), (23, 42, 0.6)]):
        t.p(S(pts), "r", trait=0, dl=-1)
    # doigts dresses, chacun coiffe d'une flamme
    doigts = ((16, 21, 14, 9, 2.1), (21, 19, 20, 5, 2.3), (26, 19, 27, 4, 2.3), (31, 21, 33, 8, 2.1))
    for x0, y0, x1, y1, r in doigts:
        t.p(C(x0, y0, x1, y1, r, r - 0.5), "r", trait=0)
    t.p(K([(15, 29, 2.6), (9, 25, 2.2), (6, 19, 1.7)]), "r", trait=0)
    t.p(B([(13, 22), (18, 18), (30, 18), (34, 23), (33, 33), (28, 39), (20, 39), (14, 33)]), "r", trait=0)
    for x, y in ((14, 9), (20, 5), (27, 4), (33, 8), (6, 19)):
        t.p(R(x - 1, y - 2, x + 1, y - 1), "c", trait=0, ton=3)
        t.p(P([(x - 1, y - 3), (x, y - 7), (x + 1, y - 3)]) | R(x - 1, y - 4, x + 1, y - 3), "f", trait=0, ton=2)
        t.px([(x, y - 4), (x, y - 3)], "f", 4)
        t.px((x - 1, y), "c", 3)
    # phalanges, visage dans la paume
    t.px([(13, 15), (14, 15), (15, 15), (20, 12), (21, 12), (26, 11), (27, 11), (28, 11), (32, 14), (33, 14), (34, 14),
          (8, 23), (9, 22)], "r", 0)
    t.p(E(19.5, 25.5, 2, 2.4) | E(28.5, 25.5, 2, 2.4), "k", ton=2, trait=0)
    t.p(E(24, 32.5, 2.6, 3), "k", ton=2, trait=0)
    t.px([(19, 25), (28, 25)], "f", 3)
    t.px([(16, 30), (17, 31), (31, 29), (22, 21), (26, 21), (30, 35)], "r", 1)
    t.p(B([(18, 40), (13, 37), (11, 39), (16, 42)]) | B([(30, 40), (35, 37), (37, 39), (32, 42)]), "l", trait=0)


@sprite("arbre-pendu", "Arbre-pendu")
def arbre_pendu(t):
    t.mat("b", "6a5644", "8a7a9a")   # ecorce morte
    t.mat("s", SPECTRE, FEU_VIOLET)
    t.mat("c", "b89a64", "d8d0c0")   # corde
    t.mat("k", "1e1618", "1e1618")
    # racines
    for pts in ([(20, 38, 3), (14, 42, 2), (8, 44, 1)], [(28, 38, 3), (34, 42, 2), (40, 44, 1)],
                [(24, 40, 2.4), (23, 44, 1.4), (20, 46, 0.7)]):
        t.p(S(pts), "b", trait=0, dl=-1)
    # branche de droite : la potence
    t.p(S([(28, 18, 2.6), (34, 13, 2.2), (41, 11, 1.6), (46, 12, 0.9)]), "b", trait=0)
    t.p(S([(36, 12, 1.2), (38, 6, 0.9), (41, 3, 0.5)]), "b", trait=0)
    # branche de gauche, doigts de brindilles
    t.p(S([(19, 20, 2.6), (12, 15, 2.0), (7, 9, 1.3), (5, 4, 0.6)]), "b", trait=0)
    t.p(S([(10, 13, 1.1), (5, 13, 0.8), (2, 11, 0.5)]) | S([(8, 10, 1), (10, 5, 0.6)]), "b", trait=0)
    # tronc tordu, tete brisee
    t.p(B([(17, 40), (18, 28), (16, 18), (20, 11), (25, 8), (30, 12), (31, 20), (29, 30), (31, 40)]), "b", trait=0)
    t.p(P([(21, 10), (22, 4), (24, 9), (26, 3), (28, 10)]), "b", trait=0)
    # visage creux, lueur des morts
    t.p(E(20.5, 18, 1.8, 2.3) | E(27, 18, 1.8, 2.3), "k", ton=2, trait=0)
    t.px([(20, 18), (27, 18)], "s", 3); t.px([(20, 17), (27, 17)], "s", 4)
    t.p(B([(20, 25), (24, 23), (28, 25), (26, 30), (22, 30)]), "k", ton=2, trait=0)
    t.px([(22, 25), (25, 25), (23, 29), (26, 28)], "b", 3)
    # la corde et son noeud coulant
    t.px([(40, 13), (40, 14), (40, 15), (40, 16), (40, 17), (40, 18), (40, 19), (40, 20), (40, 21)], "c", 2)
    t.px([(41, 13), (41, 15), (41, 17), (41, 19)], "c", 1)
    t.p(E(40.5, 26, 3.2, 4.4) - E(40.5, 26, 1.8, 3), "c", trait=0)
    t.px([(39, 21), (41, 21), (40, 22)], "c", 3)
    # ecorce
    t.px([(19, 33), (20, 34), (27, 33), (28, 35), (22, 13), (26, 14), (18, 24), (29, 23)], "b", 0)
    t.px([(19, 13), (18, 30), (19, 36)], "b", 4)
    # ame qui s'echappe
    t.p(E(33, 30, 2, 2.4), "s", trait=0, ton=2)
    t.px([(33, 27), (34, 26), (33, 30)], "s", 4)


@sprite("chene-bourreau", "Chêne-bourreau", zoom=0.93)
def chene_bourreau(t):
    t.mat("b", "5c4634", "7a6a8c")
    t.mat("l", "3f6e34", "8a5a2a")   # feuillage
    t.mat("n", "2a2630", "5a1a20")   # cagoule
    t.mat("a", ACIER)
    t.mat("c", "b89a64", "d8d0c0")
    t.mat("y", SANG, FEU_VERT)
    # racines
    for pts in ([(17, 39, 3.6), (10, 43, 2.4), (4, 45, 1)], [(31, 39, 3.6), (38, 43, 2.4), (44, 45, 1)],
                [(24, 41, 3), (24, 45, 1.6)]):
        t.p(S(pts), "b", trait=0, dl=-1)
    # bras qui porte les cordes
    t.p(S([(31, 22, 3.2), (38, 19, 2.6), (44, 21, 1.6)]), "b", trait=0)
    for x, bas in ((38, 31), (43, 28)):
        t.px([(x, y) for y in range(22, bas)], "c", 2)
        t.p(E(x, bas + 2, 2, 2.6) - E(x, bas + 2, 0.9, 1.5), "c", trait=0)
    # feuillage des epaules
    for x, y, rx, ry in ((13, 17, 7, 5), (35, 15, 7, 5), (24, 10, 9, 5)):
        t.p(E(x, y, rx, ry), "l", trait=0)
    # tronc trapu
    t.p(B([(13, 41), (14, 28), (12, 20), (17, 15), (31, 15), (36, 20), (34, 28), (35, 41)]), "b", trait=0)
    # la cagoule du bourreau
    t.p(B([(15, 24), (16, 12), (24, 4), (32, 12), (33, 24), (28, 29), (24, 31), (20, 29)]), "n", trait=2)
    t.p(E(20, 18, 1.8, 1.4) | E(28, 18, 1.8, 1.4), "y", trait=0, ton=3)
    t.px([(20, 18), (28, 18)], "y", 4)
    t.px([(24, 6), (23, 9), (25, 12), (22, 22), (26, 24)], "n", 1)
    # bras qui tient la hache
    t.p(S([(16, 23, 3.2), (10, 27, 2.6), (7, 32, 2.2)]), "b", trait=1)
    t.p(C(8, 44, 5, 10, 1.1), "b", trait=0, dl=1)
    t.p(B([(5, 11), (0, 8), (-2, 15), (0, 22), (5, 19)]), "a", trait=2)
    t.px([(-1, 12), (-1, 13), (-1, 14), (-1, 15), (-1, 16), (-1, 17), (-1, 18)], "a", 4)
    t.p(E(7, 32, 2.8, 2.6), "b", trait=1)
    # ecorce, feuilles claires
    t.px([(17, 33), (18, 35), (30, 34), (31, 36), (22, 37), (26, 39), (15, 30)], "b", 0)
    t.px([(10, 15), (13, 13), (33, 12), (37, 13), (20, 7), (27, 7)], "l", 4)
    t.px([(9, 19), (16, 20), (32, 18), (39, 17)], "l", 0)


def _tete_de_chat(t, cx, cy, f="n", y="y", r=5.6):
    """Tete de chat tournee vers la gauche : oreilles pointues, oeil fendu."""
    t.p(P([(cx + 1, cy - 3), (cx + 5, cy - r - 5), (cx + 6, cy - 1)]), f, trait=0, dl=-1)
    t.p(E(cx, cy, r, r - 0.8), f, trait=0)
    t.p(P([(cx - 4, cy - 3), (cx - 3, cy - r - 5), (cx, cy - 4)]), f, trait=0)
    t.px([(cx - 3, cy - r - 2), (cx - 2, cy - r - 1)], "r", 2)
    t.p(E(cx - 2.5, cy - 0.5, 1.6, 1.3), y, trait=0, ton=3)
    t.px((int(cx - 3), int(cy - 0.5)), "o")
    t.px((int(cx - 3), int(cy - 1.5)), y, 4)
    t.px([(int(cx - r), cy + 1), (int(cx - r) + 1, cy + 2)], "r", 2)        # truffe
    t.px([(int(cx - r) - 2, cy + 2), (int(cx - r) - 1, cy + 3), (int(cx - r) - 2, cy + 4)], "w", 3)   # moustaches


@sprite("chat-noir", "Chat noir")
def chat_noir(t):
    t.mat("n", "35313f", "e8e4dc")   # pelage
    t.mat("y", "f0c030", "50b8f0")   # oeil
    t.mat("r", "c8707a", "e89aa0")   # truffe, oreille
    t.mat("w", "e8e4dc", "9a94a8")   # moustaches
    # queue dressee en point d'interrogation
    t.p(S([(33, 40, 2.2), (39, 37, 2.0), (42, 30, 1.8), (40, 23, 1.6), (36, 21, 1.3)]), "n", trait=0)
    # corps assis
    t.p(B([(15, 24), (22, 20), (30, 23), (35, 31), (36, 40), (30, 44), (18, 44), (14, 38)]), "n", trait=0)
    t.p(E(31, 38, 5.5, 6), "n", trait=2)
    t.p(K([(18, 30, 2.4), (16, 38, 1.8), (15, 43, 1.7)]), "n", trait=1)
    t.p(K([(22, 31, 2.4), (21, 38, 1.8), (20, 43, 1.7)]), "n", trait=1)
    t.px([(13, 44), (14, 44), (18, 44), (19, 44), (27, 44), (28, 44)], "n", 3)
    _tete_de_chat(t, 16, 17)
    t.px([(20, 26), (21, 27), (20, 28)], "n", 3)


@sprite("monte-en-l-air", "Monte-en-l'air", zoom=0.95)
def monte_en_l_air(t):
    t.mat("n", "3a3646", "e8e4dc")
    t.mat("y", "f0c030", "50b8f0")
    t.mat("r", "c8707a", "e89aa0")
    t.mat("w", "e8e4dc", "9a94a8")
    t.mat("c", "2f5560", "6a2a48")   # capuche et cape
    t.mat("s", "a88858", "b8a890")   # sac de butin
    t.mat("g", OR)
    # queue basse, en balancier
    t.p(S([(36, 30, 2), (41, 31, 1.8), (45, 27, 1.5), (45, 21, 1.2)]), "n", trait=0)
    # pattes du fond
    t.p(K([(16, 33, 2), (12, 38, 1.6), (9, 43, 1.5)]), "n", dl=-1, trait=0)
    t.p(K([(33, 33, 2.4), (37, 38, 1.7), (35, 43, 1.5)]), "n", dl=-1, trait=0)
    # corps allonge, a pas de loup
    t.p(B([(13, 27), (20, 24), (30, 25), (37, 28), (37, 34), (29, 36), (18, 35), (13, 33)]), "n", trait=0)
    # sac de butin noue sur le dos
    t.p(B([(24, 24), (25, 15), (31, 12), (37, 16), (37, 25), (31, 27)]), "s", trait=2)
    t.px([(27, 18), (30, 21), (34, 17), (33, 23)], "s", 0)
    t.px([(30, 11), (31, 10), (32, 11)], "s", 3)
    t.p(C(24, 26, 36, 27, 0.8), "c", trait=0)
    t.px([(29, 14), (33, 14)], "g", 3)
    # pattes de devant
    t.p(K([(19, 33, 2.2), (17, 39, 1.7), (14, 44, 1.6)]), "n")
    t.p(K([(31, 33, 2.8), (33, 39, 1.8), (30, 44, 1.6)]), "n")
    t.px([(12, 45), (13, 45), (28, 45), (29, 45)], "n", 3)
    _tete_de_chat(t, 12, 22, r=5.4)
    # capuche a oreilles et loup sur les yeux
    t.p(B([(9, 17), (14, 15), (19, 19), (19, 27), (15, 29), (16, 22)]), "c", trait=1)
    t.p(R(6, 20, 13, 22), "c", trait=0, ton=1)
    t.px([(8, 21), (9, 21)], "y", 3); t.px((8, 21), "o")
    # la cle d'or entre les dents
    t.px([(5, 26), (4, 26), (3, 26), (2, 26), (2, 27), (4, 27)], "g", 3)
    t.p(E(1, 24.5, 1.6, 1.6) - {(1, 24), (1, 25)}, "g", trait=0, ton=3)


@sprite("cait-sith", "Cait sith", zoom=0.93)
def cait_sith(t):
    t.mat("n", "2f2b3a", "e4e0e8")
    t.mat("y", "58e0a0", "f070b0")
    t.mat("r", "c8707a", "e89aa0")
    t.mat("w", "eeeaf2", "6a5a8a")
    t.mat("s", SPECTRE, FEU_VIOLET)
    # queue fourchue, pointes de feu follet
    t.p(S([(30, 36, 2.2), (37, 33, 2), (41, 26, 1.7), (40, 19, 1.2)]), "n", trait=0)
    t.p(S([(39, 30, 1.6), (44, 28, 1.3), (46, 23, 0.9)]), "n", trait=0)
    t.p(E(40, 16, 1.8, 2.4) | E(46, 20, 1.6, 2.2), "s", trait=0, ton=3)
    t.px([(40, 15), (46, 19)], "s", 4)
    # debout sur ses pattes arriere
    t.p(K([(27, 36, 3), (29, 41, 2.2), (27, 45, 1.9)]), "n", dl=-1, trait=0)
    t.p(B([(16, 20), (24, 17), (30, 22), (32, 32), (29, 40), (20, 40), (16, 32)]), "n", trait=0)
    t.p(K([(20, 37, 3.2), (19, 42, 2.3), (16, 45, 2)]), "n")
    t.px([(13, 46), (14, 46), (24, 46), (25, 46)], "n", 3)
    # tache blanche du poitrail
    t.p(B([(17, 24), (21, 23), (23, 28), (20, 33), (17, 30)]), "w", trait=0)
    # patte qui tient une ame volee
    t.p(K([(20, 25, 2.2), (13, 28, 1.8), (9, 26, 1.6)]), "n")
    t.p(E(6, 22, 3, 3.6), "s", trait=0, ton=2)
    t.p(E(6, 23, 1.8, 2.2), "s", trait=0, ton=4)
    t.px([(6, 17), (7, 18), (5, 18)], "s", 3)
    t.px([(5, 23), (7, 23)], "o")
    t.p(K([(27, 25, 2.2), (31, 29, 1.7)]), "n", dl=-1, trait=0)
    _tete_de_chat(t, 21, 12, r=6)


@sprite("chat-a-neuf-queues", "Chat à neuf queues", zoom=0.93)
def chat_a_neuf_queues(t):
    import math
    t.mat("n", "2c2836", "e4e0e8")
    t.mat("y", "f04040", "40d0f0")
    t.mat("r", "c8707a", "e89aa0")
    t.mat("w", "eeeaf2", "6a5a8a")
    t.mat("c", "7a4a30", "5a5a7a")   # lanieres de cuir
    t.mat("s", FEU_VIOLET, FEU_VERT)
    # neuf queues en lanieres, nouees au bout
    for i in range(9):
        a = math.radians(-150 + i * 17.5)
        x1, y1 = 30 + 9 * math.cos(a), 30 + 9 * math.sin(a)
        x2, y2 = 30 + 19 * math.cos(a + 0.18), 31 + 21 * math.sin(a + 0.18)
        x3, y3 = 30 + 25 * math.cos(a + 0.05), 31 + 27 * math.sin(a + 0.05)
        x3, y3 = min(x3, 46), max(y3, 2)
        t.p(S([(31, 31, 1.6), (x1, y1, 1.4), (x2, y2, 1.1), (x3, y3, 0.8)]), "n" if i % 2 else "c", trait=0)
        t.p(E(x3, y3, 1.5, 1.5), "s", trait=0, ton=3 if i % 2 else 2)
    # corps ramasse, pret a bondir
    t.p(K([(17, 36, 2.2), (13, 41, 1.7), (9, 44, 1.6)]), "n", dl=-1, trait=0)
    t.p(B([(11, 30), (18, 27), (28, 27), (35, 31), (35, 38), (27, 40), (17, 39), (11, 36)]), "n", trait=0)
    t.p(E(31, 37, 5.5, 5.5), "n", trait=2)
    t.p(K([(33, 40, 2.6), (30, 44, 2), (26, 45, 1.7)]), "n")
    t.p(K([(18, 36, 2.6), (15, 41, 1.9), (11, 45, 1.7)]), "n")
    t.px([(8, 45), (9, 46), (10, 46), (23, 46), (24, 46)], "w", 3)
    _tete_de_chat(t, 11, 26, r=5.8)
    # gueule ouverte, crocs
    t.px([(5, 29), (6, 30), (7, 30), (8, 30)], "o")
    t.px([(6, 31), (8, 31)], "w", 4)
    t.px([(20, 30), (24, 29), (28, 31)], "n", 3)
