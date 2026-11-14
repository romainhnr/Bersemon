from dessin import *
from matieres import *
from atelier import sprite
import math

VIANDE = ["4a1420", "7a2a38", "aa4a58", "d08088", "f0bcc0"]
VIANDE_SHINY = ["14204a", "2a387a", "4a58aa", "8088d0", "bcc0f0"]
VIDE = ["0a0814", "181430", "2c2650", "4c4480", "8478c0"]


@sprite("lys-de-sang", "Lys de sang")
def lys(t):
    t.mat("p", ["8a8088", "c0b4bc", "e8dce0", "f8f0f2", "ffffff"], ["88808a", "bcb4c0", "e0dce8", "f2f0f8", "ffffff"])   # petales blancs
    t.mat("s", SANG, FEU_VIOLET)
    t.mat("v", "3f6a3c", "6a3a6a")
    t.mat("g", OR)
    t.mat("l", ["c84a6a", "f07a98", "f8a8c0", "fcd0dc", "fff0f4"], FEU_BLEU)   # lueur de fee
    # tige et feuilles en lames
    t.p(S([(24, 46, 2.2), (25, 38, 2), (23, 30, 1.8), (24, 22, 1.6)]), "v", trait=0)
    for pts in ([(24, 42), (14, 34), (8, 22), (15, 30)], [(25, 42), (35, 35), (41, 24), (33, 31)], [(24, 44), (16, 44), (8, 40), (15, 40)]):
        t.p(B(pts), "v", trait=0, dl=-(pts[1][0] > 24))
    # six petales en trompette, taches de sang
    for a, l in ((-150, 14), (-110, 15), (-70, 15), (-30, 14), (10, 11), (170, 11)):
        x, y = 24 + l * math.cos(math.radians(a)), 20 + l * math.sin(math.radians(a))
        mx, my = 24 + l * 0.55 * math.cos(math.radians(a)), 20 + l * 0.55 * math.sin(math.radians(a))
        t.p(K([(24, 20, 2.4), (mx, my, 3.4), (x, y, 1)]), "p", trait=0, dl=-(a > -80 and a < 100))
        t.px([(mx, my), (mx + 1, my + 1)], "s", 2)
    t.p(E(24, 20, 3.4, 3), "p", trait=0, dl=-1)
    # etamines d'or, pistil qui goutte
    for a in (-130, -100, -70, -40):
        x, y = 24 + 8 * math.cos(math.radians(a)), 19 + 8 * math.sin(math.radians(a))
        t.p(L(24, 19, x, y), "g", trait=0, ton=2)
        t.px((x, y), "g", 4)
    t.px([(24, 22), (24, 24), (24, 27), (23, 31)], "s", 2)
    # visage de fee au coeur de la fleur
    t.px([(23, 20), (25, 20)], "o")
    t.px([(4, 10), (44, 8), (40, 36), (6, 34)], "l", 3)


@sprite("reine-mere-de-chair", "Reine-mère de chair")
def reine_mere(t):
    t.mat("c", VIANDE, VIANDE_SHINY)
    t.mat("v", ["8a5a50", "b88478", "dcb0a4", "f2d4c8", "fff0e8"])
    t.mat("g", OR)
    t.mat("y", ["7a6a10", "c0a820", "f0d840", "f8f090", "fffce0"], FEU_BLEU)
    t.mat("w", OS)
    # abdomen de reine pondeuse, enorme, annele
    t.p(B([(18, 26), (30, 22), (41, 26), (46, 35), (42, 44), (28, 46), (16, 42), (14, 33)]), "v", trait=0)
    for x in (22, 27, 32, 37, 41):
        t.p(CS([(x, 24 + abs(x - 32) // 4), (x + 2, 34), (x, 45 - abs(x - 30) // 4)]), "v", trait=0, ton=0, sur="v")
    t.px([(24, 30), (30, 28), (36, 31), (41, 36)], "v", 4)
    # oeufs translucides
    for x, y in ((38, 44), (43, 42), (33, 46)):
        t.p(E(x, y, 2, 1.8), "y", trait=0, ton=4)
        t.px((x, y), "c", 2)
    # buste de femme ecorchee
    t.p(B([(13, 20), (18, 16), (24, 19), (25, 28), (20, 33), (13, 30)]), "c", trait=0)
    t.px([(16, 22), (17, 24), (18, 26), (21, 22), (20, 24), (19, 28)], "c", 0)
    # quatre bras
    t.p(K([(14, 21, 1.8), (8, 24, 1.5), (4, 20, 1.3)]) | K([(15, 26, 1.8), (9, 31, 1.5), (5, 36, 1.3)]), "c", trait=1)
    t.p(K([(23, 21, 1.8), (28, 16, 1.5), (32, 11, 1.3)]) | K([(24, 26, 1.8), (30, 24, 1.5)]), "c", trait=0, dl=-1)
    t.px([(2, 18), (3, 18), (4, 37), (5, 38), (33, 9), (34, 10)], "w", 3)
    # tete : pas de visage, une couronne enfoncee dans la chair
    t.p(E(17, 11, 4.6, 5), "c", trait=0)
    t.px([(15, 11), (19, 11)], "y", 4); t.px([(15, 10), (19, 10)], "o")
    t.px([(15, 14), (16, 14), (17, 14), (18, 14), (19, 14)], "w", 4)
    t.p(R(13, 5, 21, 6), "g", trait=0)
    for x, h in ((13, 3), (15, 4), (17, 6), (19, 4), (21, 3)):
        t.p(R(x, 5 - h, x, 5), "g", trait=0, ton=3)
    t.px([(14, 8), (20, 8)], "c", 0)
    # pattes
    for x in (20, 26, 32):
        t.p(K([(x, 43, 1.4), (x - 3, 46, 0.9)]), "c", trait=0, dl=-1)


@sprite("rampant-du-puits", "Rampant du puits")
def rampant_du_puits(t):
    t.mat("g", ["1e1c26", "363444", "565468", "7e7c92", "b0aec4"], ["261c1e", "443436", "685456", "927c7e", "c4aeb0"])   # peau d'ardoise
    t.mat("y", FEU_VIOLET, FEU_VERT)
    t.mat("w", OS)
    # queue longue
    t.p(S([(36, 32, 2.8), (42, 34, 2), (46, 30, 1.2), (45, 24, 0.6)]), "g", trait=0)
    # membres ecartes, comme un lezard sur un mur
    t.p(K([(18, 30, 2), (15, 23, 1.5), (9, 20, 1.2)]), "g", dl=-1, trait=0)
    t.p(K([(32, 30, 2), (36, 23, 1.5), (42, 21, 1.2)]), "g", dl=-1, trait=0)
    # corps plat et long, plaques de pierre sur l'echine
    t.p(B([(11, 31), (18, 27), (30, 27), (38, 30), (38, 35), (30, 37), (18, 37), (11, 35)]), "g", trait=0)
    for x in range(15, 37, 4):
        t.p(P([(x - 2, 28), (x, 24), (x + 2, 28)]), "g", trait=0, ton=4)
    t.p(K([(18, 35, 2.2), (13, 40, 1.6), (7, 43, 1.2)]), "g")
    t.p(K([(32, 35, 2.2), (36, 41, 1.6), (42, 44, 1.2)]), "g")
    for x, y in ((5, 43), (7, 19), (43, 45), (44, 20)):
        t.px([(x, y), (x - 1, y + 1), (x + 1, y + 1), (x - 2, y)], "w", 3)
    # tete en coin, sans yeux : des fentes qui luisent
    t.p(B([(1, 32), (5, 27), (12, 27), (15, 32), (13, 37), (6, 38), (2, 36)]), "g", trait=0)
    t.px([(5, 30), (6, 30), (8, 31), (9, 31), (5, 33), (6, 33)], "y", 3)
    t.px([(2, 35), (3, 36), (4, 36), (5, 36), (6, 36), (7, 36), (8, 35)], "o")
    t.px([(3, 37), (5, 37), (7, 37)], "w", 4)
    t.px([(22, 31), (27, 33), (33, 32)], "y", 2)
    t.grain("g", 0.06, 11)


@sprite("echo-du-vide", "Écho du vide")
def echo(t):
    t.mat("m", ["8a8498", "b8b2c4", "dcd8e4", "f2f0f6", "ffffff"])   # masque
    t.mat("v", VIDE, ["140a08", "30181c", "502c26", "80504c", "c08478"])
    t.mat("p", FEU_VIOLET, FEU_VERT)
    # ondes : des anneaux brises autour de lui
    for r in (21, 17):
        for a in range(0, 360, 12):
            if (a // 12) % 3 != 2:
                t.px((24 + r * math.cos(math.radians(a)), 23 + r * 0.95 * math.sin(math.radians(a))), "p", 2 if r == 21 else 3)
    # un lambeau de nuit
    t.p(B([(14, 16), (24, 10), (34, 16), (36, 30), (31, 40), (28, 36), (24, 44), (20, 36), (17, 40), (12, 30)]), "v", trait=0)
    # le masque : un visage qui crie sans bouche
    t.p(B([(16, 14), (24, 9), (32, 14), (33, 26), (28, 33), (24, 35), (20, 33), (15, 26)]), "m", trait=0)
    t.p(P([(18, 18), (22, 20), (20, 24), (17, 22)]) | P([(30, 18), (26, 20), (28, 24), (31, 22)]), "v", trait=0, ton=0)
    t.px([(19, 21), (29, 21)], "p", 4)
    t.p(E(24, 29, 2.4, 3.4), "v", trait=0, ton=0)
    # fissure
    t.p(LS([(24, 9), (25, 14), (23, 17), (25, 21)]), "v", trait=0, ton=1)
    t.px([(22, 11), (21, 13)], "m", 4)
    t.px([(4, 22), (44, 24), (24, 1), (24, 47)], "p", 4)


@sprite("ver-des-abysses", "Ver des abysses")
def ver_des_abysses(t):
    t.mat("c", ["14142c", "282850", "44447c", "6c6cac", "a4a4dc"], ["2c1414", "502828", "7c4444", "ac6c6c", "dca4a4"])   # cuir bleu-noir
    t.mat("l", ["0c4a5a", "189aa8", "40d8e0", "a0f4f4", "f0ffff"], FEU)   # lumieres des fonds
    t.mat("w", OS)
    t.mat("r", SANG, FEU_VERT)
    # corps de dragon sans pattes, en boucles montantes
    t.p(S([(44, 40, 2), (40, 44, 3.4), (32, 44, 4.4), (27, 38, 4.8), (31, 31, 5), (37, 25, 5), (35, 17, 5), (27, 13, 5),
           (19, 14, 4.8)]), "c", trait=1)
    # rangee de lumieres le long du flanc
    for x, y in ((40, 43), (33, 43), (28, 38), (31, 32), (36, 26), (35, 19), (29, 14), (22, 14)):
        t.px((x, y), "l", 3); t.px((x, y - 1), "l", 4)
    # crete dorsale
    for x, y, dx, dy in ((37, 39, 3, -3), (32, 27, -4, -2), (41, 24, 4, -1), (38, 13, 3, -3), (28, 8, 0, -3)):
        t.p(P([(x - 1, y), (x + dx, y + dy), (x + 1, y + 1)]), "c", trait=0, ton=4)
    # gueule de baudroie, dents en aiguilles, leurre lumineux
    t.p(B([(3, 12), (8, 5), (17, 5), (22, 10), (21, 18), (14, 22), (6, 20)]), "c", trait=0)
    t.p(B([(3, 13), (10, 12), (17, 14), (14, 20), (7, 19)]), "r", trait=0, ton=0)
    for x in (4, 7, 10, 13, 16):
        t.p(R(x, 12, x, 15), "w", trait=0, ton=4)
    for x in (6, 9, 12):
        t.p(R(x, 17, x, 19), "w", trait=0, ton=3)
    t.px([(15, 8), (16, 8)], "l", 4); t.px((15, 8), "o")
    t.p(CS([(13, 5), (10, 0), (5, 1)]), "c", trait=0)
    t.p(E(4, 2, 2, 2), "l", trait=0, ton=3); t.px((4, 2), "l", 4)
    t.px([(1, 6), (7, -2), (0, -1)], "l", 3)


@sprite("gardien-du-gouffre", "Gardien du gouffre")
def gardien(t):
    t.mat("a", FER_NOIR, ACIER)
    t.mat("p", FEU_VIOLET, FEU_VERT)
    t.mat("g", PIERRE)
    t.mat("l", BOIS)
    # grande hache
    t.p(C(40, 46, 38, 6, 1.1), "l", trait=0)
    t.p(B([(38, 8), (45, 4), (47, 12), (45, 20), (39, 17)]), "a", trait=0, dl=1)
    t.px([(46, 8), (46, 12), (46, 16)], "a", 4)
    # jambes en piliers
    t.p(R(25, 34, 31, 44), "a", trait=0, dl=-1); t.p(R(16, 34, 22, 44), "a", trait=0)
    t.p(R(14, 44, 23, 45) | R(24, 44, 33, 45), "a", trait=0)
    # torse de geant, runes violettes
    t.p(B([(13, 16), (24, 12), (34, 16), (36, 28), (33, 36), (15, 36), (12, 28)]), "a", trait=0)
    t.p(R(23, 18, 24, 32) | R(19, 22, 28, 23), "p", trait=0, ton=2)
    t.px([(23, 18), (23, 26), (21, 22), (27, 22)], "p", 4)
    # pavois de pierre, porte au bras gauche
    t.p(P([(1, 14), (15, 12), (16, 34), (8, 44), (2, 36)]), "g", trait=1)
    t.p(P([(3, 16), (13, 15), (14, 33), (8, 41), (4, 35)]), "g", trait=0, dl=1)
    t.p(E(8.5, 26, 3, 3) - E(8.5, 26, 1.4, 1.4), "p", trait=0, ton=2)
    t.px([(4, 18), (12, 17), (6, 36)], "g", 0)
    # epaules, bras droit
    t.p(E(34, 17, 5, 4), "a", trait=0)
    t.p(K([(35, 20, 3), (39, 27, 2.6), (39, 32, 2.4)]), "a", trait=1)
    # heaume sans visage, cornes de taureau, lueur dans la fente
    t.p(B([(18, 14), (19, 5), (24, 2), (29, 5), (30, 14)]), "a", trait=0)
    t.p(R(20, 8, 28, 9), "p", trait=0, ton=3)
    t.px([(21, 8), (27, 8)], "p", 4)
    t.p(CS([(19, 6), (15, 4), (14, -1)]) | CS([(29, 6), (33, 4), (34, -1)]), "g", trait=0, ton=3)
    t.grain("g", 0.08, 13)


@sprite("germe-du-monde", "Germe du monde")
def germe(t):
    t.mat("s", ["a87a10", "e0b020", "f8dc50", "fff4a0", "ffffe8"], FEU_BLEU)   # la graine de lumiere
    t.mat("v", "4f9a3c", "3a9aa8")
    t.mat("r", BOIS)
    t.mat("t", ["3a2a1c", "5a4430", "7e6448", "a48a68", "ccb494"])   # motte de terre
    # racines qui pendent dans le vide
    for pts in ([(20, 36, 1.8), (15, 41, 1.2), (12, 46, 0.6)], [(24, 37, 1.8), (24, 43, 1.2), (22, 47, 0.6)],
                [(28, 36, 1.8), (33, 41, 1.2), (36, 46, 0.6)], [(17, 34, 1.4), (10, 37, 0.9), (6, 42, 0.5)],
                [(31, 34, 1.4), (38, 37, 0.9), (42, 42, 0.5)]):
        t.p(S(pts), "r", trait=0, dl=-1)
    # motte de terre flottante
    t.p(B([(13, 30), (24, 28), (35, 30), (33, 36), (24, 39), (15, 36)]), "t", trait=0)
    t.px([(17, 33), (22, 36), (28, 35), (31, 32)], "t", 0)
    # la graine : un petit soleil fendu
    for a in range(0, 360, 30):
        t.p(C(24 + 9 * math.cos(math.radians(a)), 20 + 9 * math.sin(math.radians(a)), 24 + 12.5 * math.cos(math.radians(a)),
              20 + 12.5 * math.sin(math.radians(a)), 0.8, 0.2), "s", trait=0, ton=3)
    t.p(E(24, 21, 8, 9), "s", trait=0)
    t.p(E(22, 19, 3.4, 4), "s", trait=0, ton=4)
    t.p(LS([(24, 12), (25, 17), (23, 21), (25, 26), (24, 30)]), "s", trait=0, ton=0)
    # la pousse : deux feuilles neuves
    t.p(S([(24, 13, 1.2), (24, 7, 1), (25, 2, 0.8)]), "v", trait=0)
    t.p(B([(25, 5), (30, 0), (35, 1), (31, 6)]), "v", trait=0)
    t.p(B([(24, 7), (18, 3), (14, 5), (19, 9)]), "v", trait=0, dl=-1)
    t.px([(31, 2), (17, 5)], "v", 4)
    t.px([(6, 14), (42, 12), (4, 28), (44, 30)], "s", 3)


@sprite("roi-sans-visage", "Roi sans visage")
def roi(t):
    t.mat("r", ["0c0a14", "1c182c", "302a48", "4c446c", "7c74a0"], ["140a0c", "2c181c", "482a30", "6c444c", "a0747c"])   # manteau d'ombre
    t.mat("g", OR)
    t.mat("f", "e8dcc8", "c8c0d8")   # hermine
    t.mat("v", VIDE)
    t.mat("p", FEU_VIOLET, FEU_VERT)
    # trone de pierre derriere lui
    t.p(R(13, 6, 35, 44), "v", trait=0, ton=2)
    t.p(R(13, 6, 15, 44) | R(33, 6, 35, 44) | R(13, 6, 35, 8), "v", trait=0, ton=3)
    t.p(P([(13, 6), (14, 1), (16, 6)]) | P([(32, 6), (34, 1), (35, 6)]), "v", trait=0, ton=3)
    # manteau royal en cloche
    t.p(B([(15, 20), (24, 16), (33, 20), (38, 32), (41, 45), (7, 45), (10, 32)]), "r", trait=0)
    for x in (13, 19, 29, 35):
        t.p(L(x, 28, x + (x - 24) // 3, 44), "r", trait=0, ton=0, sur="r")
    t.p(B([(16, 20), (24, 17), (32, 20), (30, 24), (24, 22), (18, 24)]), "f", trait=0)
    t.px([(19, 21), (23, 20), (27, 20), (30, 22)], "o")
    t.px([(9, 45), (15, 44), (24, 45), (33, 44), (39, 45)], "g", 2)
    t.p(R(23, 23, 25, 44), "g", trait=0, ton=1)
    # mains gantees posees sur un sceptre et un orbe
    t.p(K([(17, 23, 2.4), (11, 28, 2.2), (9, 26, 1.9)]), "r", trait=1)
    t.p(C(8, 44, 9, 12, 0.9), "g", trait=0)
    t.p(E(9, 10, 2.4, 2.4), "p", trait=0, ton=3); t.px((8, 9), "p", 4)
    t.p(K([(31, 23, 2.4), (37, 28, 2.2), (38, 26, 1.9)]), "r", trait=1, dl=-1)
    t.p(E(39, 24, 3, 3), "g", trait=0)
    t.p(R(39, 19, 39, 21) | R(38, 20, 40, 20), "g", trait=0, ton=4)
    # capuchon : a la place du visage, rien
    t.p(B([(17, 18), (17, 9), (24, 5), (31, 9), (31, 18), (27, 20), (21, 20)]), "r", trait=0, dl=1)
    t.p(E(24, 13, 4.4, 5), "v", trait=0, ton=0)
    # la couronne
    t.p(R(18, 5, 30, 7), "g", trait=0)
    for x, h in ((18, 3), (21, 4), (24, 6), (27, 4), (30, 3)):
        t.p(R(x, 5 - h, x, 5), "g", trait=0, ton=3)
    t.px([(21, 6), (24, 6), (27, 6)], "p", 3)
