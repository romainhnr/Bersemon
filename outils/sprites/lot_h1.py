from dessin import *
from matieres import *
from atelier import sprite
import commun
from commun import jambes, torse, robe, bras, tete, couronne

HEROS = dict(dossier="heros", couronne=True, variantes=("shiny", "cendre", "sang", "abime", "nacre", "royale"))


@sprite("heros-guerrier", "Guerrier", fichier="guerrier", **HEROS)
def guerrier(t):
    t.shiny_fixe = True
    t.mat("a", "565664", "b08a30", cendre="76726c", sang="7a1a22", abime="2a3e6a", nacre="d0ccd8", royale="d8b040")   # armure
    t.mat("r", "a02a2a", "2a4aa0", cendre="c8501a", sang="241418", abime="2a9a8a", nacre="b8a0d8", royale="6a2a9a")   # cape
    t.mat("c", PEAU)
    t.mat("h", "2e2428", "e8e4d8", cendre="3a3634", sang="2a1014", abime="1c2438", nacre="e8e4f0", royale="2e2428")   # cheveux, barbe
    t.mat("l", ACIER)                # lame
    t.mat("n", "3a2a20")             # cuir
    t.mat("or", OR)
    # cape
    t.p(B([(22, 16), (32, 15), (37, 26), (38, 42), (33, 40), (29, 43), (27, 30)]), "r", trait=0, dl=-1)
    jambes(t, "a", "n", pas=1)
    t.px([(20, 36), (19, 37), (28, 36), (29, 37)], "a", 4)
    torse(t, "a")
    t.px([(24, 17), (24, 20), (24, 23), (24, 26)], "a", 0)
    t.p(R(18, 29, 30, 30), "n", trait=0); t.px((24, 29), "or", 3)
    # grande epee portee sur l'epaule
    bras(t, [(30, 18), (34, 22), (32, 16)], "a", dl=-1, trait=0)
    t.p(P([(31, 14), (40, -3), (43, -1), (34, 16)]), "l", trait=1)
    t.px([(36, 6), (38, 2), (40, -1)], "l", 4)
    t.p(R(29, 14, 35, 15), "or", trait=0)
    # epaulieres, bras de devant, gantelet
    t.p(E(17, 17, 3.8, 3) | E(31, 17, 3.6, 2.8), "a", trait=0, dl=1)
    bras(t, [(18, 19), (14, 25), (15, 30)], "a", trait=1, main=False)
    t.p(E(15, 31, 2.2, 2), "a", trait=0, dl=1)
    # tete : cheveux courts, barbe
    t.p(B([(19, 8), (20, 3), (26, 2), (29, 6), (29, 12), (26, 8), (21, 7)]), "h", trait=0)
    tete(t)
    t.p(B([(18, 11), (20, 12), (24, 12), (26, 11), (25, 15), (21, 16), (19, 14)]), "h", trait=0)
    t.px([(20, 12), (21, 12)], "c", 1)
    if commun.COURONNE[0]:
        couronne(t, 24, 3)


@sprite("heros-mage", "Mage", fichier="mage", **HEROS)
def mage(t):
    t.shiny_fixe = True
    t.mat("r", "2c3c94", "6a2a8a", cendre="6a6662", sang="7a1a22", abime="1c4a5a", nacre="d8d4e4", royale="6a2a8a")   # robe
    t.mat("g", "d8b040", "d8b040", cendre="c8501a", sang="1e1418", abime="2ac0a8", nacre="b8a0d8", royale="f0c850")   # galons
    t.mat("k", "14102a")             # ombre du capuchon
    t.mat("y", ["1a6a8a", "30a8c8", "60d8f0", "b0f4ff", "ffffff"], ["8a5a10", "c89020", "f8d040", "ffec90", "ffffe0"],
          cendre=FEU, sang=["5a0a12", "961420", "d02c2c", "f06a50", "ffd0b0"], abime=FEU_VERT, nacre=FEU_VIOLET,
          royale=["8a5a10", "c89020", "f8d040", "ffec90", "ffffe0"])   # cristal, regard
    t.mat("l", BOIS)
    t.mat("c", PEAU)
    t.mat("or", OR)
    # baton a cristal
    t.p(C(10, 45, 11, 8, 1), "l", trait=0)
    t.p(P([(8, 8), (11, 0), (14, 8), (11, 11)]), "y", trait=0)
    t.px([(10, 4), (10, 6)], "y", 4)
    t.px([(6, 2), (16, 3), (11, -3)], "y", 3)
    robe(t, "r", large=1)
    t.p(R(23, 18, 25, 44), "g", trait=0, ton=2)
    t.p(R(12, 44, 36, 45), "g", trait=0, ton=1)
    t.p(R(17, 28, 31, 29), "g", trait=0)
    # manches larges
    bras(t, [(18, 19), (13, 24), (11, 22)], "r")
    bras(t, [(30, 19), (34, 26), (30, 30)], "r", dl=-1, trait=0)
    # capuchon pointu : deux lueurs dans l'ombre
    t.p(B([(17, 17), (17, 8), (22, 3), (27, 0), (28, 5), (30, 9), (31, 17), (27, 19), (21, 19)]), "r", trait=0, dl=1)
    t.p(E(23, 11.5, 4, 4.8), "k", trait=0, ton=2)
    t.px([(21, 11), (25, 11)], "y", 4); t.px([(20, 11), (24, 11)], "y", 3)
    if commun.COURONNE[0]:
        couronne(t, 23, 5)


@sprite("heros-templier", "Templier", fichier="templier", **HEROS)
def templier(t):
    t.shiny_fixe = True
    t.mat("s", ["8a8a90", "b8b8be", "dcdce2", "f2f2f6", "ffffff"], "2e2c38", cendre="76726c", sang="6a1820", abime="26365e",
          nacre="e4e0ec", royale="6a2a9a")   # surcot
    t.mat("x", "a82a2a", "d8b040", cendre="c8501a", sang="181014", abime="2ac0a8", nacre="b8a0d8", royale="f0c850")   # croix
    t.mat("a", ACIER, ["3a3020", "5e4e30", "867048", "b09a6c", "dcc8a0"])   # mailles et heaume
    t.mat("n", "3a2a20")
    t.mat("l", ACIER)
    t.mat("or", OR)
    # epee au cote
    t.p(C(32, 30, 40, 44, 0.9), "l", trait=0, ton=3)
    t.p(R(30, 28, 35, 29), "or", trait=0)
    jambes(t, "a", "a")
    # surcot long, croix sur la poitrine
    t.p(B([(17, 17), (24, 14), (31, 17), (32, 27), (33, 38), (16, 38), (16, 27)]), "s", trait=0)
    t.p(R(23, 19, 25, 34) | R(19, 24, 29, 26), "x", trait=0)
    t.p(R(17, 29, 31, 30), "n", trait=0)
    t.px([(17, 38), (20, 39), (24, 38), (28, 39), (31, 38)], "s", 1)
    bras(t, [(30, 19), (34, 25), (33, 29)], "a", dl=-1, trait=0, main=False)
    # grand ecu en amande, croix
    t.p(P([(3, 16), (17, 15), (18, 30), (10, 41), (3, 30)]), "a", trait=1, dl=-1)
    t.p(P([(5, 18), (15, 17), (16, 29), (10, 38), (5, 29)]), "s", trait=0)
    t.p(R(10, 19, 11, 35) | R(6, 24, 15, 25), "x", trait=0)
    # heaume a fente, croix d'aeration
    t.p(B([(18, 15), (18, 5), (23, 2), (29, 5), (29, 15)]), "a", trait=0)
    t.p(R(18, 8, 26, 9), "o", trait=0, ton=2)
    t.p(R(22, 5, 22, 13), "a", trait=0, ton=4)
    t.px([(19, 12), (21, 12), (24, 12), (26, 12)], "a", 0)
    if commun.COURONNE[0]:
        couronne(t, 23, 3)


@sprite("heros-assassin", "Assassin", fichier="assassin", **HEROS)
def assassin(t):
    t.shiny_fixe = True
    t.mat("n", "2c2a36", "e4e0d8", cendre="5e5a56", sang="5a141c", abime="1c2a4e", nacre="d8d4e4", royale="5a2480")   # tenue
    t.mat("r", "a02a34", "2a9a9a", cendre="c8501a", sang="181014", abime="2ac0a8", nacre="b8a0d8", royale="f0c850")   # foulard
    t.mat("c", PEAU)
    t.mat("l", ACIER)
    t.mat("b", "3a2a20")
    t.mat("or", OR)
    # foulard au vent
    t.p(S([(27, 14, 1.8), (34, 13, 1.6), (40, 16, 1.2), (44, 14, 0.7)]), "r", trait=0)
    # jambes flechies, pret a bondir
    t.p(K([(28, 30, 2.6), (34, 35, 2.2), (32, 43, 2)]), "n", trait=0, dl=-1)
    t.p(R(29, 43, 35, 45), "b", trait=0, dl=-1)
    t.p(K([(21, 30, 2.6), (16, 36, 2.2), (17, 43, 2)]), "n", trait=0)
    t.p(R(13, 43, 19, 45), "b", trait=0)
    # buste penche en avant
    t.p(B([(15, 19), (22, 14), (29, 16), (32, 24), (30, 32), (19, 32), (15, 26)]), "n", trait=0)
    t.p(L(18, 20, 28, 30) | L(19, 30, 29, 31), "b", trait=0, ton=2)
    t.px([(21, 23), (24, 26)], "l", 4)
    # deux dagues
    bras(t, [(29, 19), (34, 24), (38, 22)], "n", dl=-1, trait=0)
    t.p(P([(38, 21), (45, 17), (40, 23)]), "l", trait=0, ton=3)
    bras(t, [(17, 20), (11, 24), (7, 21)], "n")
    t.p(P([(7, 20), (0, 16), (5, 22)]), "l", trait=0, ton=4)
    # capuche, masque sur le bas du visage, regard
    t.p(B([(15, 15), (15, 7), (20, 3), (26, 4), (28, 10), (27, 16), (21, 18)]), "n", trait=0, dl=1)
    t.p(E(20, 11, 3.4, 3.6), "c", trait=0)
    t.p(R(16, 12, 23, 15), "r", trait=0)
    t.px([(18, 10), (21, 10)], "o")
    t.px([(17, 9), (18, 9), (20, 9), (21, 9)], "n", 0)
    if commun.COURONNE[0]:
        couronne(t, 21, 4)
