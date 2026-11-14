from dessin import *
from matieres import *
from atelier import sprite
from commun import jambes, torse, robe, bras, tete

H = dict(dossier="humains", variantes=())
CAMP = dict(dossier="camp", variantes=())


@sprite("humain-chevalier-crane", "Chevalier-Crâne", fichier="chevalier-crane", **H)
def chevalier_crane(t):
    t.mat("a", FER_NOIR); t.mat("k", OS); t.mat("r", "3a1c1c"); t.mat("y", SANG); t.mat("l", ACIER); t.mat("g", OR)
    # cape sombre
    t.p(B([(20, 16), (34, 15), (40, 26), (42, 44), (36, 41), (31, 45), (28, 30)]), "r", trait=0, dl=-1)
    # grande epee plantee devant lui
    t.p(R(8, 12, 10, 45), "l", trait=0)
    t.px([(8, y) for y in range(14, 44, 2)], "l", 4)
    t.p(R(4, 19, 14, 20), "g", trait=0); t.p(R(8, 7, 10, 11), "a", trait=0)
    jambes(t, "a", "a")
    torse(t, "a", large=1)
    t.px([(24, 17), (24, 20), (24, 23), (24, 26)], "a", 4)
    t.p(E(17, 17, 4, 3) | E(32, 17, 4, 3), "a", trait=0, dl=1)
    bras(t, [(31, 19), (35, 25), (34, 30)], "a", dl=-1, trait=0, main=False)
    bras(t, [(17, 19), (12, 22), (9, 20)], "a", main=False)
    t.p(E(9, 20, 2.2, 2), "a", trait=0, dl=1)
    # le heaume : un crane immense
    t.p(E(23.5, 8, 7, 6.6), "k", trait=0)
    t.p(R(19, 12, 28, 16), "k", trait=0)
    t.p(E(20.5, 8, 2, 2.4) | E(26.5, 8, 2, 2.4), "o", trait=0, ton=2)
    t.px([(20, 8), (26, 8)], "y", 3)
    t.p(P([(23, 10), (23.5, 13), (24, 10)]), "o", trait=0, ton=2)
    for x in (20, 22, 24, 26):
        t.p(R(x, 14, x, 16), "o", trait=0, ton=2)


@sprite("humain-gueux", "Gueux", fichier="gueux", **H)
def gueux(t):
    t.mat("c", PEAU); t.mat("v", "7a6a54"); t.mat("b", "5a4a3a"); t.mat("m", BOIS); t.mat("h", "5a4630")
    # gourdin leve
    t.p(C(34, 22, 38, 4, 1.4, 2.6), "m", trait=0)
    t.px([(37, 5), (39, 8)], "m", 0)
    # jambes nues, pieds nus
    jambes(t, "c", "c", pas=1)
    # tunique dechiree
    t.p(B([(17, 17), (24, 14), (31, 17), (32, 28), (31, 36), (28, 33), (25, 37), (22, 33), (18, 36), (16, 28)]), "v", trait=0)
    t.px([(20, 22), (27, 26), (23, 30)], "v", 0)
    t.p(R(17, 28, 31, 28), "b", trait=0)
    bras(t, [(30, 19), (35, 22), (35, 18)], "c", dl=-1, trait=0)
    bras(t, [(18, 19), (13, 25), (12, 31)], "c")
    tete(t)
    t.p(B([(18, 8), (20, 3), (27, 3), (29, 8), (29, 13), (26, 8), (21, 7)]), "h", trait=0)
    t.p(B([(19, 12), (22, 13), (26, 11), (25, 15), (21, 16)]), "h", trait=0)


@sprite("humain-coupe-jarret", "Coupe-jarret", fichier="coupe-jarret", **H)
def coupe_jarret(t):
    t.mat("c", PEAU); t.mat("v", "5a4a3a"); t.mat("b", "3e3a36"); t.mat("n", "24201e"); t.mat("g", "3f6a3c")
    t.mat("l", ACIER); t.mat("w", "f4f0e6")
    jambes(t, "b", "n", pas=2)
    torse(t, "v")
    t.p(R(18, 28, 30, 29), "n", trait=0)
    # deux lames, bras ecartes
    bras(t, [(30, 19), (36, 23), (41, 21)], "v", dl=-1, trait=0)
    t.p(P([(41, 20), (47, 24), (42, 23)]), "l", trait=0, ton=3)
    bras(t, [(18, 19), (12, 22), (7, 20)], "v")
    t.p(P([(7, 19), (0, 15), (5, 21)]), "l", trait=0, ton=4)
    tete(t, bouche=False)
    # rictus, bonnet vert
    t.px([(19, 12), (20, 12), (21, 12), (22, 12)], "w", 4)
    t.px([(18, 11), (23, 11)], "o")
    t.p(B([(18, 7), (19, 2), (25, 0), (29, 4), (29, 8), (24, 6)]), "g", trait=0)
    t.p(K([(28, 3, 1.4), (32, 6, 1)]), "g", trait=0, dl=-1)


@sprite("humain-novice-de-l-ordre", "Novice de l'Ordre", fichier="novice-de-l-ordre", **H)
def novice(t):
    t.mat("c", PEAU); t.mat("s", ["8a8a90", "b8b8be", "dcdce2", "f2f2f6", "ffffff"]); t.mat("x", "a82a2a"); t.mat("a", ACIER)
    t.mat("n", "3a2a20"); t.mat("m", BOIS)
    # masse d'armes levee
    t.p(C(35, 24, 38, 8, 1), "m", trait=0)
    t.p(E(38.5, 5, 3.4, 3.4), "a", trait=0)
    t.px([(35, 2), (42, 2), (35, 8), (42, 8), (38, 1)], "a", 4)
    jambes(t, "a", "n")
    t.p(B([(17, 17), (24, 14), (31, 17), (32, 27), (32, 36), (16, 36), (16, 27)]), "s", trait=0)
    t.p(R(23, 19, 25, 32) | R(20, 23, 28, 25), "x", trait=0)
    t.p(R(17, 29, 31, 30), "n", trait=0)
    bras(t, [(30, 19), (35, 22), (35, 19)], "a", dl=-1, trait=0)
    bras(t, [(18, 19), (13, 24), (14, 30)], "a")
    # visage jeune, chapel de fer trop grand
    tete(t, cy=11)
    t.px([(20, 13), (22, 13)], "x", 3)
    t.p(E(23, 7, 8, 1.8), "a", trait=1)
    t.p(B([(18, 7), (19, 2), (27, 2), (28, 7)]), "a", trait=0)


@sprite("humain-apprenti-necromant", "Apprenti nécromant", fichier="apprenti-necromant", **H)
def apprenti(t):
    t.mat("c", ["5a6a50", "86987a", "aec0a0", "d0dcc4", "ecf4e4"]); t.mat("r", "2e3e30"); t.mat("f", FEU_VERT)
    t.mat("k", OS); t.mat("n", "1c2420"); t.mat("p", ["8a7a5a", "b8a880", "dcd0a8", "f2ead0", "fffcf0"])
    robe(t, "r")
    t.p(R(17, 29, 31, 30), "k", trait=0, ton=1)
    t.px([(26, 31), (26, 33), (27, 35)], "k", 3)
    # un grimoire sous le bras, une flamme verte mal tenue dans la main
    bras(t, [(30, 19), (33, 26), (29, 29)], "r", dl=-1, trait=0)
    t.p(R(25, 25, 32, 32), "p", trait=1); t.p(R(25, 25, 26, 32), "n", trait=0, ton=3)
    bras(t, [(18, 19), (12, 22), (9, 18)], "r")
    t.p(B([(6, 15), (7, 9), (9, 4), (12, 9), (12, 15)]), "f", trait=0, ton=2)
    t.px([(9, 9), (9, 12), (9, 14)], "f", 4)
    t.px([(5, 5), (13, 3), (9, 0)], "f", 3)
    # capuche, teint malade, cernes
    t.p(B([(17, 17), (17, 7), (23, 2), (29, 6), (30, 17), (26, 19), (21, 19)]), "r", trait=0, dl=1)
    t.p(E(22, 11, 3.6, 4.2), "c", trait=0)
    t.px([(20, 10), (23, 10)], "o"); t.px([(20, 11), (23, 11)], "c", 0)
    t.px([(21, 14), (22, 14)], "c", 0)


@sprite("humain-mercenaire", "Mercenaire", fichier="mercenaire", **H)
def mercenaire(t):
    t.mat("c", PEAU); t.mat("v", "6a5a44"); t.mat("a", ACIER); t.mat("b", "4a4038"); t.mat("n", "2a2420")
    t.mat("l", ACIER); t.mat("h", "5a4630"); t.mat("m", BOIS)
    # rondache dans le dos
    t.p(E(31, 22, 6, 7), "m", trait=0, dl=-1)
    t.p(E(31, 22, 6, 7) - E(31, 22, 4.8, 5.8), "a", trait=0, ton=1)
    jambes(t, "b", "n", pas=1)
    torse(t, "v")
    # broigne d'ecailles
    for y in (19, 22, 25):
        for x in range(19 + (y % 2), 30, 3):
            t.px([(x, y), (x + 1, y)], "a", 3)
    t.p(R(18, 29, 30, 30), "n", trait=0)
    bras(t, [(30, 19), (34, 25), (33, 30)], "v", dl=-1, trait=0)
    # epee tenue basse
    bras(t, [(18, 19), (13, 25), (11, 30)], "v")
    t.p(C(10, 31, 2, 43, 1), "l", trait=0, ton=3)
    t.p(R(8, 29, 14, 30), "a", trait=0, ton=1)
    tete(t, cy=10)
    t.p(B([(18, 12), (21, 13), (25, 13), (27, 11), (26, 16), (21, 17)]), "h", trait=0)
    # cervelliere, cicatrice
    t.p(B([(18, 8), (19, 3), (27, 3), (28, 8)]), "a", trait=0)
    t.p(R(18, 7, 28, 8), "a", trait=0, ton=1)
    t.px([(25, 9), (25, 10), (26, 11)], "c", 0)


@sprite("humain-lame-noire", "Lame noire", fichier="lame-noire", **H)
def lame_noire(t):
    t.mat("n", ["08070e", "16141f", "282634", "444250", "74728a"]); t.mat("y", "f4f4ff"); t.mat("l", ACIER)
    # manteau d'un seul bloc, capuche pointue
    t.p(B([(17, 17), (20, 6), (24, 0), (28, 6), (31, 17), (35, 32), (37, 45), (11, 45), (13, 32)]), "n", trait=0)
    for x in (15, 20, 28, 33):
        t.p(L(x, 26, x + (x - 24) // 3, 44), "n", trait=0, ton=0, sur="n")
    t.p(E(24, 11, 3.6, 4.4), "n", trait=0, ton=0)
    t.px([(22, 11), (26, 11)], "y", 3)
    # une seule main sort du manteau : une lame noire a fil blanc
    bras(t, [(18, 21), (12, 26), (8, 24)], "n", main=False)
    t.p(P([(8, 23), (1, 13), (6, 25)]), "n", trait=0, ton=2)
    t.p(L(2, 14, 7, 23), "y", trait=0, ton=3)
    t.px([(13, 45), (19, 44), (24, 45), (30, 44), (35, 45)], "n", 3)


@sprite("humain-sorciere-des-cendres", "Sorcière des cendres", fichier="sorciere-des-cendres", **H)
def sorciere(t):
    t.mat("c", ["6a625e", "9a908a", "c4bab2", "e2dad2", "f8f2ec"]); t.mat("r", "55525a"); t.mat("h", "e4e0e4")
    t.mat("f", FEU); t.mat("m", BOIS)
    # baton noueux, braise au sommet
    t.p(C(10, 45, 11, 10, 1), "m", trait=0)
    t.p(B([(8, 10), (9, 4), (11, -1), (14, 4), (14, 10)]), "f", trait=0, ton=2)
    t.px([(11, 4), (11, 7), (11, 9)], "f", 4)
    robe(t, "r")
    # braises prises dans l'etoffe
    for x, y in ((18, 26), (22, 34), (28, 30), (31, 40), (16, 40), (25, 42), (20, 20), (29, 22)):
        t.px((x, y), "f", 2)
    t.px([(22, 35), (31, 41)], "f", 4)
    bras(t, [(30, 19), (34, 26), (31, 31)], "r", dl=-1, trait=0)
    bras(t, [(18, 19), (13, 23), (11, 21)], "r")
    # longs cheveux blancs, visage de cendre
    t.p(B([(18, 10), (20, 4), (26, 3), (30, 8), (33, 26), (29, 18), (27, 9), (21, 7)]), "h", trait=0)
    tete(t)
    t.p(K([(19, 13, 1), (17, 20, 0.9), (18, 26, 0.7)]), "h", trait=0)
    t.px([(4, 12), (16, 2), (6, 4)], "f", 3)


@sprite("humain-templier-renegat", "Templier renégat", fichier="templier-renegat", **H)
def templier_renegat(t):
    t.mat("a", FER_NOIR); t.mat("s", "2e2a34"); t.mat("x", "a82a2a"); t.mat("y", SANG); t.mat("l", ACIER); t.mat("m", BOIS)
    t.mat("v", FEU_VIOLET)
    # fleau d'armes : manche, chaine, boule a pointes
    t.p(C(34, 26, 37, 10, 1), "m", trait=0)
    for x, y in ((38, 8), (40, 6), (42, 5)):
        t.px((x, y), "l", 3)
    t.p(E(44, 4, 3, 3), "a", trait=0, dl=1)
    t.px([(41, 1), (47, 1), (41, 7), (47, 7), (44, 0)], "l", 4)
    t.px([(40, 12), (43, 10)], "v", 3)
    jambes(t, "a", "a")
    t.p(B([(17, 17), (24, 14), (31, 17), (32, 27), (32, 37), (29, 34), (26, 38), (22, 34), (19, 38), (16, 34), (16, 27)]), "s",
        trait=0)
    # la croix de l'ordre, renversee
    t.p(R(23, 19, 25, 32) | R(20, 27, 28, 29), "x", trait=0)
    bras(t, [(30, 19), (34, 23), (34, 26)], "a", dl=-1, trait=0, main=False)
    bras(t, [(18, 19), (13, 24), (13, 30)], "a", main=False)
    t.p(E(13, 31, 2.2, 2), "a", trait=0, dl=1)
    # heaume noir, fente rouge
    t.p(B([(18, 15), (18, 5), (23, 2), (29, 5), (29, 15)]), "a", trait=0, dl=1)
    t.p(R(18, 8, 26, 9), "y", trait=0, ton=1)
    t.px([(19, 8), (20, 8), (23, 8), (24, 8)], "y", 3)
    t.px([(19, 12), (21, 12), (24, 12), (26, 12)], "a", 0)


@sprite("humain-gladiateur", "Gladiateur", fichier="gladiateur", **H)
def gladiateur(t):
    t.mat("c", PEAU); t.mat("r", "a82a2a"); t.mat("a", ["4a3410", "7a5a1c", "b08a30", "dcb858", "f8e4a0"]); t.mat("n", "5a4030")
    t.mat("l", ACIER)
    # jambes nues, jambiere, sandales
    jambes(t, "c", "n", pas=1)
    t.p(R(17, 36, 21, 42), "a", trait=0)
    # torse nu, baudrier, pagne rouge
    torse(t, "c", large=1)
    t.px([(22, 20), (26, 20), (24, 24), (21, 26), (27, 26)], "c", 0)
    t.p(L(17, 18, 30, 28), "n", trait=0, ton=2)
    t.p(B([(16, 27), (32, 27), (33, 36), (28, 33), (24, 37), (20, 33), (15, 36)]), "r", trait=0)
    t.p(R(16, 27, 32, 28), "a", trait=0)
    # glaive leve, petit bouclier rond
    bras(t, [(31, 19), (37, 17), (38, 11)], "c", dl=-1, trait=0)
    t.p(P([(37, 10), (39, -2), (41, 10)]), "l", trait=0, ton=3)
    t.p(R(36, 10, 42, 11), "a", trait=0)
    bras(t, [(17, 19), (12, 24), (10, 27)], "c")
    t.p(E(8, 27, 5.4, 6), "a", trait=1)
    t.p(E(8, 27, 2, 2.2), "a", trait=0, ton=4)
    # casque a crete rouge, visiere grillee
    tete(t, cy=10)
    t.p(B([(18, 9), (18, 4), (23, 2), (28, 4), (29, 12), (26, 9)]), "a", trait=0)
    t.p(R(18, 8, 22, 12), "a", trait=0, ton=1)
    t.px([(19, 9), (21, 9), (19, 11), (21, 11)], "o")
    t.p(B([(20, 3), (22, -2), (27, -2), (30, 3), (30, 8), (27, 3)]), "r", trait=0)


@sprite("humain-champion-de-la-fosse", "Champion de la Fosse", fichier="champion-de-la-fosse", **H)
def champion(t):
    t.mat("a", FER_NOIR); t.mat("r", ["3a0410", "70101e", "a82030", "d84a50", "f8a09c"]); t.mat("g", OR); t.mat("f", FEU)
    t.mat("c", PEAU); t.mat("y", FEU)
    # manteau rouge sur une epaule
    t.p(B([(20, 15), (34, 15), (40, 28), (41, 45), (34, 42), (29, 45), (26, 30)]), "r", trait=0, dl=-1)
    jambes(t, "a", "a", pas=1)
    torse(t, "a", large=1)
    t.p(R(17, 28, 31, 29), "g", trait=0)
    t.p(P([(21, 18), (24, 23), (27, 18)]), "g", trait=0, ton=1)
    t.p(E(17, 17, 4.2, 3.2), "g", trait=0)
    # deux epees de flamme
    bras(t, [(31, 19), (36, 23), (40, 20)], "a", dl=-1, trait=0, main=False)
    t.p(LS([(40, 19), (42, 13), (40, 8), (43, 2)]), "f", trait=0, ton=3)
    t.p(LS([(41, 19), (43, 13), (41, 8), (44, 2)]), "f", trait=0, ton=2)
    bras(t, [(17, 19), (11, 23), (7, 20)], "a", main=False)
    t.p(LS([(7, 19), (5, 13), (7, 8), (4, 2)]), "f", trait=0, ton=3)
    t.p(LS([(6, 19), (4, 13), (6, 8), (3, 2)]), "f", trait=0, ton=2)
    t.px([(4, 1), (44, 1)], "f", 4)
    # masque d'or sans traits, heaume a cimier
    t.p(B([(18, 15), (18, 5), (23, 2), (29, 5), (29, 15)]), "a", trait=0, dl=1)
    t.p(E(22, 10, 3.4, 4), "g", trait=0)
    t.px([(20, 9), (23, 9)], "o")
    t.p(P([(22, 3), (24, -3), (27, 4)]), "r", trait=0)


@sprite("personnage-malvoise", "Malvoise", fichier="malvoise", **CAMP)
def malvoise(t):
    t.mat("r", ["1c1028", "34204a", "50346e", "7c5c9c", "b498cc"]); t.mat("c", ["6a5a68", "9a8a98", "c4b4c0", "e4d8e0", "fbf4f8"])
    t.mat("k", OS); t.mat("g", OR); t.mat("y", FEU_VIOLET)
    robe(t, "r", large=1)
    t.p(R(23, 18, 25, 44), "r", trait=0, ton=1)
    t.p(CS([(19, 20), (24, 26), (29, 20)]), "g", trait=0, ton=2)
    t.p(R(23, 26, 25, 30), "g", trait=0)
    # un crane dans la main, qu'il regarde ; l'autre main cachee dans la manche
    bras(t, [(30, 19), (34, 26), (30, 30)], "r", dl=-1, trait=0, main=False)
    bras(t, [(18, 19), (12, 24), (9, 21)], "r")
    t.p(E(7, 17, 3.6, 3.4), "k", trait=0)
    t.p(R(5, 19, 9, 21), "k", trait=0)
    t.px([(6, 17), (8, 17)], "o"); t.px([(6, 20), (8, 20)], "o")
    # capuchon de moine, visage creuse, sourire fin
    t.p(B([(17, 17), (17, 7), (23, 2), (29, 6), (30, 17), (26, 19), (21, 19)]), "r", trait=0, dl=1)
    t.p(E(22, 11, 3.6, 4.4), "c", trait=0)
    t.px([(20, 10), (23, 10)], "y", 3); t.px([(20, 9), (23, 9)], "o")
    t.p(LS([(20, 14), (21, 15), (23, 14)]), "o", trait=0, ton=2)


@sprite("personnage-gunther", "Gunther", fichier="gunther", **CAMP)
def gunther(t):
    t.mat("c", PEAU); t.mat("t", "5a3a24"); t.mat("v", "7a6a58"); t.mat("b", "4a4038"); t.mat("n", "2a2420")
    t.mat("h", "b8501c"); t.mat("a", ACIER); t.mat("m", BOIS); t.mat("f", FEU)
    # marteau de forge leve
    t.p(C(36, 20, 38, 4, 1), "m", trait=0)
    t.p(R(33, -1, 44, 4), "a", trait=0)
    t.px([(34, 0), (36, 0), (38, 0)], "a", 4)
    jambes(t, "b", "n")
    torse(t, "v", large=2)
    # tablier de cuir, brule par endroits
    t.p(B([(19, 19), (29, 19), (31, 38), (17, 38)]), "t", trait=0)
    t.px([(21, 26), (26, 31), (22, 35)], "n", 2)
    bras(t, [(32, 19), (37, 22), (36, 19)], "c", dl=-1, trait=0)
    bras(t, [(16, 19), (11, 25), (11, 31)], "c")
    # pince et fer rouge
    t.p(R(4, 32, 11, 32), "a", trait=0, ton=1)
    t.p(R(0, 31, 4, 33), "f", trait=0, ton=3)
    # crane chauve, grande barbe rousse
    tete(t)
    t.p(B([(17, 11), (20, 12), (25, 12), (28, 10), (28, 18), (23, 22), (19, 18)]), "h", trait=0)
    t.px([(20, 12), (21, 13)], "c", 1)


@sprite("personnage-ansgar", "Ansgar", fichier="ansgar", **CAMP)
def ansgar(t):
    t.mat("c", PEAU); t.mat("v", "3a5a7a"); t.mat("b", "4a4438"); t.mat("n", "2a2420"); t.mat("g", "c4c0bc")
    t.mat("p", ["6a5a20", "a08c30", "d0bc48", "ece080", "fcf4c0"]); t.mat("m", BOIS); t.mat("l", "e8e4f0")
    t.mat("e", ["3a5a7a", "6088a8", "90b8d0", "c0dcec", "ecf8fc"])
    # canne a peche, ligne, poisson au bout
    t.p(C(34, 26, 46, -2, 0.8), "m", trait=0)
    t.p(L(46, -2, 44, 30), "l", trait=0, ton=3)
    t.p(B([(41, 30), (44, 28), (47, 31), (44, 35)]), "e", trait=0)
    t.p(P([(43, 35), (41, 39), (46, 39)]), "e", trait=0, dl=-1)
    t.px((44, 30), "o")
    # cuissardes
    jambes(t, "n", "n")
    torse(t, "v")
    t.p(L(20, 17, 21, 31) | L(28, 17, 27, 31), "b", trait=0, ton=3)
    t.p(R(18, 30, 30, 31), "b", trait=0)
    bras(t, [(30, 19), (35, 23), (34, 26)], "v", dl=-1, trait=0)
    # un panier d'osier au bras
    bras(t, [(18, 19), (13, 25), (13, 30)], "v")
    t.p(B([(7, 31), (17, 31), (16, 39), (8, 39)]), "p", trait=1, dl=-1)
    t.p(L(8, 34, 16, 34) | L(8, 37, 16, 37), "p", trait=0, ton=0)
    tete(t, cy=10)
    t.p(B([(18, 12), (21, 13), (25, 13), (27, 11), (27, 17), (22, 20), (19, 17)]), "g", trait=0)
    # chapeau de paille a large bord
    t.p(E(23, 6.5, 9.6, 2), "p", trait=1)
    t.p(B([(18, 6), (20, 1), (26, 1), (28, 6)]), "p", trait=0)
    t.p(R(19, 5, 27, 5), "v", trait=0, ton=1)
