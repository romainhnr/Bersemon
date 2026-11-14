from dessin import *
from matieres import *
from atelier import sprite
from commun import _aile_membrane
import math

VIDE = ["0a0814", "181430", "2c2650", "4c4480", "8478c0"]
VIDE_SHINY = ["140a08", "30181c", "502c26", "80504c", "c08478"]


@sprite("ver-ossivore", "Ver ossivore")
def ver_ossivore(t):
    t.mat("c", ["4a4030", "74664c", "9c8c6c", "c4b494", "e8dcc0"], ["30404a", "4c6674", "6c8c9c", "94b4c4", "c0dce8"])   # chitine d'os jauni
    t.mat("k", OS)
    t.mat("n", "2a2420")
    t.mat("r", SANG, FEU_VERT)
    # un femur de geant, perce de part en part
    t.p(C(4, 38, 44, 34, 3.6), "k", trait=0, dl=-1)
    t.p(E(3, 36, 3, 2.6) | E(4, 41, 3, 2.6) | E(45, 32, 3, 2.6) | E(45, 37, 3, 2.6), "k", trait=0, dl=-1)
    t.p(E(31, 35, 2.6, 2.2), "o", trait=0, ton=2)
    t.px([(12, 37), (20, 36), (38, 34)], "k", 1)
    # le ver sort d'un trou et rentre par un autre : corps annele, cuirasse
    t.p(S([(31, 35, 3.4), (33, 26, 3.8), (29, 18, 4), (22, 14, 4), (15, 16, 3.8)]), "c", trait=0)
    for x0, y0, x1, y1 in ((29, 30, 37, 31), (29, 24, 37, 23), (26, 18, 33, 15), (23, 10, 24, 18), (18, 11, 18, 19)):
        t.p(L(x0, y0, x1, y1), "c", trait=0, ton=0, sur="c")
    for x, y in ((36, 27), (34, 19), (27, 13), (20, 11)):
        t.p(P([(x - 1, y), (x + 2, y - 3), (x + 1, y + 1)]), "c", trait=0, ton=4)
    # tete : une rape circulaire, mandibules en tenailles
    t.p(E(11, 18, 4.6, 5), "c", trait=0, dl=1)
    t.p(E(10, 18.5, 3, 3.4), "n", trait=0, ton=2)
    for a in range(0, 360, 45):
        t.px((10 + 2.6 * math.cos(math.radians(a)), 18.5 + 3 * math.sin(math.radians(a))), "k", 4)
    t.p(CS([(8, 14), (3, 15), (4, 20)]) | CS([(8, 23), (3, 23), (5, 18)]), "n", trait=0, ton=3)
    # poussiere d'os
    for x, y in ((6, 28), (9, 31), (4, 32), (12, 29), (27, 39), (34, 40)):
        t.px((x, y), "k", 3)


@sprite("titan-dechu", "Titan déchu")
def titan(t):
    t.mat("c", ["3a342c", "5e5646", "867c68", "aea48c", "d6ccb4"], ["2c343a", "46565e", "687c86", "8ca4ae", "b4ccd6"])   # chair de terre
    t.mat("g", PIERRE)
    t.mat("a", FER_NOIR, ACIER)      # chaines
    t.mat("y", FEU, FEU_BLEU)
    t.mat("l", "5a8a4a", "8a6aa8")
    # a genoux : une jambe pliee, l'autre genou a terre
    t.p(K([(31, 33, 4.4), (38, 37, 3.6), (36, 44, 3.2)]), "c", trait=0, dl=-1)
    t.p(K([(20, 34, 4.4), (14, 40, 3.8), (22, 44, 3.2)]), "c", trait=0)
    t.p(R(18, 44, 26, 45) | R(33, 44, 40, 45), "c", trait=0, dl=-1)
    # torse immense, voute, epaules de roche
    t.p(B([(10, 18), (17, 10), (31, 9), (39, 16), (40, 28), (35, 36), (16, 36), (10, 28)]), "c", trait=0)
    t.p(B([(26, 9), (33, 6), (41, 11), (42, 20), (37, 16), (30, 13)]), "g", trait=0)
    t.px([(31, 8), (36, 9), (40, 14)], "l", 2)
    t.px([(18, 18), (22, 24), (28, 20), (32, 28), (20, 31)], "c", 0)
    # les chaines brisees aux poignets et au cou
    t.p(K([(12, 20, 3.6), (7, 29, 3.2), (6, 38, 3)]), "c", trait=1)
    t.p(E(6, 41, 3.4, 3), "c", trait=0)
    t.p(R(3, 34, 9, 36), "a", trait=0)
    for y in (38, 41, 44):
        t.p(E(2, y, 1.3, 1.6) - E(2, y, 0.3, 0.6), "a", trait=0, ton=3)
    t.p(K([(38, 20, 3.6), (43, 28, 3.2), (43, 36, 3)]), "c", trait=0, dl=-1)
    t.p(R(40, 32, 46, 34), "a", trait=0)
    t.p(E(46, 37, 1.3, 1.6) - E(46, 37, 0.3, 0.6), "a", trait=0, ton=3)
    # tete baissee, un seul oeil encore allume, couronne arrachee
    t.p(B([(16, 13), (17, 6), (23, 3), (29, 6), (30, 14), (24, 18), (18, 17)]), "c", trait=0, dl=1)
    t.p(R(18, 10, 22, 11), "o", trait=0, ton=2); t.px([(19, 10), (20, 10)], "y", 4)
    t.p(R(25, 10, 28, 11), "o", trait=0, ton=2)
    t.px([(20, 14), (21, 15), (22, 15), (23, 15), (24, 14)], "o")
    t.p(R(16, 17, 31, 18), "a", trait=0)
    t.p(LS([(19, 5), (22, 2), (25, 4)]), "g", trait=0, ton=4)
    t.grain("g", 0.08, 22)
    t.grain("c", 0.04, 23)


@sprite("rampant-du-neant", "Rampant du néant")
def rampant_du_neant(t):
    t.mat("v", VIDE, VIDE_SHINY)
    t.mat("p", FEU_VIOLET, FEU_VERT)
    t.mat("w", ["8a8498", "b8b2c4", "dcd8e4", "f2f0f6", "ffffff"])
    # une ombre a plat ventre, qui se tire avec ses bras : pas de jambes, une traine
    t.p(B([(22, 34), (34, 30), (44, 34), (47, 40), (40, 44), (30, 42), (24, 44)]), "v", trait=0, ton=1)
    for x, y in ((38, 44), (44, 43), (33, 45), (46, 36)):
        t.px([(x, y), (x + 1, y + 1)], "v", 3)
    # torse arque
    t.p(B([(11, 28), (18, 22), (28, 24), (33, 31), (30, 39), (18, 40), (12, 36)]), "v", trait=0)
    t.px([(18, 25), (22, 24), (26, 26), (29, 30)], "v", 4)
    # bras demesures, mains a longs doigts plantees au sol
    t.p(K([(15, 30, 2.4), (9, 22, 1.8), (5, 34, 1.5)]), "v", trait=1)
    t.p(K([(26, 28, 2.4), (30, 18, 1.8), (24, 38, 1.5)]), "v", trait=1, dl=-1)
    for cx, cy in ((5, 36), (23, 40)):
        for dx in (-3, -1, 1, 3):
            t.p(L(cx, cy, cx + dx, cy + 6), "v", trait=0, ton=3)
    # tete : un masque blanc sans traits, fendu d'un sourire violet ; trois yeux
    t.p(E(11, 19, 5.4, 6), "v", trait=0)
    t.p(E(10, 19.5, 3.8, 4.6), "w", trait=0)
    t.px([(8, 18), (11, 18), (9, 15)], "p", 3); t.px([(8, 17), (11, 17)], "o")
    t.p(LS([(7, 21), (9, 23), (11, 23), (13, 21)]), "p", trait=0, ton=1)
    # des lambeaux de vide s'en detachent
    for x, y in ((18, 16), (22, 12), (36, 24), (40, 26), (3, 10), (16, 8)):
        t.px((x, y), "p", 3); t.px((x + 1, y - 1), "v", 3)


@sprite("mangeur-d-etoiles", "Mangeur d'étoiles")
def mangeur_d_etoiles(t):
    t.mat("n", ["06060e", "12122a", "24244c", "40407a", "7474b8"], ["0e0606", "2a1212", "4c2424", "7a4040", "b87474"])   # ecailles de nuit
    t.mat("e", OR, ["5a5a8a", "8a8ac0", "b8b8e8", "e0e0ff", "ffffff"])
    t.mat("m", ["140a2a", "2a1650", "44287c", "6c48a8", "a480d8"], ["2a0a14", "50162a", "7c2844", "a8486c", "d880a4"])
    t.mat("w", OS)
    _aile_membrane(t, "m", "n", (28, 22), (32, 6), [(42, 2), (47, 11), (45, 22), (38, 21)], dl=-1)
    # queue
    t.p(S([(34, 32, 3.4), (40, 37, 2.6), (44, 43, 1.6), (40, 47, 0.8)]), "n", trait=0)
    t.p(K([(31, 34, 2.8), (34, 40, 1.9), (32, 45, 1.7)]), "n", dl=-1, trait=0)
    # corps : le ciel etoile est dans son ventre
    t.p(B([(13, 27), (20, 22), (31, 22), (38, 27), (38, 35), (30, 38), (19, 38), (13, 34)]), "n", trait=0)
    for x, y in ((18, 26), (23, 30), (28, 26), (33, 31), (21, 35), (30, 35), (25, 24), (16, 31)):
        t.px((x, y), "e", 4)
    for x, y in ((26, 32), (34, 27)):
        t.px([(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)], "e", 3)
    t.p(K([(16, 34, 2.8), (14, 40, 1.9), (12, 45, 1.7)]), "n")
    t.p(K([(29, 35, 3.2), (29, 41, 2), (27, 45, 1.7)]), "n")
    t.px([(9, 46), (10, 46), (12, 46), (24, 46), (25, 46), (27, 46)], "w", 3)
    # cou tendu vers le haut, gueule ouverte sur une etoile qu'il avale
    t.p(S([(16, 27, 3.6), (13, 19, 3), (14, 12, 2.8)]), "n", trait=0)
    t.p(B([(8, 12), (9, 5), (15, 2), (20, 6), (20, 13), (14, 15)]), "n", trait=0)
    t.p(P([(9, 6), (2, 2), (8, 10)]), "n", trait=0, dl=1)
    t.p(P([(10, 11), (3, 12), (9, 14)]), "n", trait=0, dl=-1)
    t.px([(4, 4), (6, 6), (5, 11), (7, 12)], "w", 4)
    t.px([(3, 7), (2, 7), (4, 7), (3, 6), (3, 8), (1, 7), (5, 7), (3, 5), (3, 9)], "e", 3)
    t.px((3, 7), "e", 4)
    t.px([(15, 7), (16, 7)], "e", 4); t.px((15, 7), "o")
    t.p(P([(17, 3), (23, -1), (20, 6)]), "n", trait=0, ton=4)
    t.px([(6, 20), (2, 28), (44, 30), (24, 2), (36, 0)], "e", 3)


@sprite("spectre-hurlant", "Spectre hurlant")
def spectre_hurlant(t):
    t.mat("s", SPECTRE, FEU_VIOLET)
    t.mat("k", "0c1c24", "1c0c24")
    t.mat("o2", ["8a8ab0", "b0b0d0", "d0d0ea", "eaeaf8", "ffffff"])
    # un drap de brume, effile vers le bas, bras leves
    t.p(B([(14, 16), (24, 8), (34, 16), (37, 28), (34, 38), (30, 34), (27, 44), (24, 38), (20, 46), (18, 36), (13, 40), (11, 28)]),
        "s", trait=0)
    t.p(S([(14, 20, 2.6), (8, 14, 2), (5, 6, 1.4)]), "s", trait=0)
    t.p(S([(34, 20, 2.6), (40, 14, 2), (43, 6, 1.4)]), "s", trait=0, dl=-1)
    for x, y in ((3, 4), (5, 2), (7, 5), (41, 4), (43, 2), (45, 5)):
        t.px((x, y), "s", 3)
    t.px([(16, 30), (22, 34), (29, 30), (33, 26), (20, 24)], "s", 1)
    # visage : deux trous et une bouche demesuree, decrochee
    t.p(E(19.5, 17, 2.4, 3) | E(28.5, 17, 2.4, 3), "k", trait=0, ton=2)
    t.px([(19, 17), (28, 17)], "s", 4)
    t.p(B([(19, 23), (24, 21), (29, 23), (30, 31), (24, 36), (18, 31)]), "k", trait=0, ton=2)
    t.px([(21, 24), (23, 23), (25, 23), (27, 24)], "s", 4)
    # le hurlement : des cercles qui s'elargissent
    for r in (8, 12, 16):
        for a in list(range(150, 211, 12)) + list(range(-30, 31, 12)):
            t.px((24 + (r + 12) * math.cos(math.radians(a)), 28 + r * math.sin(math.radians(a))), "o2", 3 if r < 16 else 2)


@sprite("ange-du-neant", "Ange du néant")
def ange_du_neant(t):
    t.mat("v", VIDE, VIDE_SHINY)
    t.mat("l", ["a87a10", "e0b020", "f8dc50", "fff4a0", "ffffe8"], FEU_BLEU)   # lumiere
    t.mat("w", ["8a8a90", "b8b8be", "dcdce2", "f2f2f6", "ffffff"])
    # deux paires d'ailes : l'une de lumiere, l'autre de vide
    for i, (x, y) in enumerate(((7, 3), (3, 10), (2, 18), (5, 25))):
        t.p(C(20, 18, x, y, 3.2, 1.3), "w", trait=0, dl=-(i % 2))
    for i, (x, y) in enumerate(((41, 3), (45, 10), (46, 18), (43, 25))):
        t.p(C(28, 18, x, y, 3.2, 1.3), "v", trait=0, dl=(i % 2))
    for i, (x, y) in enumerate(((8, 32), (12, 38))):
        t.p(C(21, 24, x, y, 2.6, 1.1), "w", trait=0, dl=-1)
    for i, (x, y) in enumerate(((40, 32), (36, 38))):
        t.p(C(27, 24, x, y, 2.6, 1.1), "v", trait=0)
    # robe coupee en deux : moitie blanche, moitie nuit
    corps = B([(18, 18), (24, 15), (30, 18), (33, 32), (31, 42), (27, 46), (24, 41), (21, 46), (17, 42), (15, 32)])
    t.p(corps & R(0, 0, 23, 47), "w", trait=0)
    t.p(corps & R(24, 0, 47, 47), "v", trait=0)
    t.p(L(24, 16, 24, 41), "l", trait=0, ton=3)
    # bras ouverts, une etoile dans une main, un trou noir dans l'autre
    t.p(K([(19, 20, 1.8), (13, 26, 1.5), (9, 23, 1.3)]), "w", trait=1)
    t.p(K([(29, 20, 1.8), (35, 26, 1.5), (39, 23, 1.3)]), "v", trait=1)
    t.px([(7, 20), (6, 20), (8, 20), (7, 19), (7, 21), (5, 20), (9, 20), (7, 18), (7, 22)], "l", 3)
    t.px((7, 20), "l", 4)
    t.p(E(41, 20, 2.6, 2.6), "v", trait=0, ton=0)
    t.p(E(41, 20, 2.6, 2.6) - E(41, 20, 1.6, 1.6), "l", trait=0, ton=1)
    # visage partage, aureole : un anneau d'eclipse
    t.p(E(24, 10, 4, 4.4) & R(0, 0, 23, 47), "w", trait=0, ton=3)
    t.p(E(24, 10, 4, 4.4) & R(24, 0, 47, 47), "v", trait=0, ton=1)
    t.px((22, 10), "o"); t.px((26, 10), "l", 4)
    t.p(E(24, 3, 5.4, 1.8) - E(24, 3, 3.8, 0.8), "l", trait=0, ton=3)
    t.px([(12, 44), (36, 44), (24, 47)], "l", 3)


@sprite("marchand-zombie", "Marchand zombie", variantes=())
def marchand_zombie(t):
    t.mat("c", ["34402e", "566848", "7e9068", "a8b890", "d0dcbc"])   # chair de morte
    t.mat("r", "5a4a3a")             # robe d'herboriste
    t.mat("t", "7a4a5a")             # tablier
    t.mat("h", BOIS)                 # hotte d'osier
    t.mat("v", ["24401c", "406a2c", "66984a", "98c478", "ccecb4"])   # herbes
    t.mat("g", "8a8a94")             # cheveux gris
    t.mat("y", "f8f0a0")
    # la hotte d'herbes au dos
    t.p(B([(28, 14), (40, 12), (42, 30), (30, 32)]), "h", trait=0)
    for y in (17, 21, 25, 29):
        t.p(L(29, y + 1, 41, y - 1), "h", trait=0, ton=0)
    for x, h in ((30, 6), (33, 8), (36, 7), (39, 5)):
        t.p(K([(x, 13, 1), (x + 1, 13 - h, 0.7)]), "v", trait=0, dl=-(x % 2))
    t.px([(31, 7), (37, 6)], "v", 4)
    # robe longue, tablier tache, pas trainant
    t.p(B([(16, 20), (24, 16), (31, 20), (33, 32), (34, 45), (14, 45), (14, 32)]), "r", trait=0)
    t.p(B([(19, 24), (28, 24), (29, 42), (18, 42)]), "t", trait=0)
    t.px([(21, 30), (22, 31), (25, 36), (26, 37), (20, 38)], "v", 1)
    t.px([(16, 45), (20, 44), (26, 45), (31, 44)], "r", 0)
    t.p(R(13, 45, 19, 46) | R(26, 45, 32, 46), "c", trait=0, dl=-1)
    # bras tendus devant elle, une faucille dans la main
    t.p(K([(18, 21, 2.2), (11, 22, 1.9), (4, 21, 1.7)]), "r", trait=1)
    t.p(K([(24, 23, 2.2), (16, 26, 1.9), (9, 27, 1.7)]), "r", trait=1)
    t.p(E(3, 20.5, 1.8, 1.8) | E(8, 27, 1.8, 1.8), "c", trait=0)
    t.p(CS([(3, 19), (0, 14), (3, 10), (7, 11)]), "g", trait=0, ton=4)
    # tete penchee : coiffe, meches grises, machoire pendante, yeux vitreux
    t.p(B([(16, 18), (15, 9), (20, 4), (27, 5), (30, 11), (29, 18), (24, 14), (19, 15)]), "g", trait=0)
    t.p(E(21, 13, 4.4, 4.8), "c", trait=0)
    t.px([(19, 12), (22, 12)], "y", 3); t.px([(19, 11), (22, 11)], "o")
    t.p(E(20, 16.5, 1.6, 1.4), "o", trait=0, ton=2)
    t.px([(19, 18), (21, 18)], "c", 4)
    t.p(B([(16, 8), (20, 3), (27, 4), (29, 8), (22, 8)]), "t", trait=0, dl=1)
    t.px([(23, 20), (25, 19)], "v", 2)
