# Morceaux d'anatomie partages par plusieurs lots.
from dessin import *


def _aile_membrane(t, m, os_, base, coude, pointes, dl=0):
    """Aile de chauve-souris : un bras, des doigts, la membrane tendue entre."""
    pts = [base, coude] + list(pointes)
    t.p(P(pts), m, trait=0, dl=dl)
    for p in pointes:
        t.p(K([(coude[0], coude[1], 0.8), (p[0], p[1], 0.4)]), os_, trait=0, dl=dl)
    t.p(K([(base[0], base[1], 1.3), (coude[0], coude[1], 0.9)]), os_, trait=0, dl=dl)


# ---------------------------------------------------------------- humains
# Vrai pendant le second dessin d'un heros : celui de la couleur royale.
COURONNE = [False]


def jambes(t, m="b", bottes="n", pas=0):
    """Deux jambes, celle du fond plus sombre ; `pas` les ecarte."""
    t.p(K([(27, 30, 2.6), (28 + pas, 37, 2.2), (28 + pas, 43, 2)]), m, trait=0, dl=-1)
    t.p(R(25 + pas, 43, 31 + pas, 45), bottes, trait=0, dl=-1)
    t.p(K([(21, 30, 2.6), (20 - pas, 37, 2.2), (19 - pas, 43, 2)]), m, trait=0)
    t.p(R(15 - pas, 43, 21 - pas, 45), bottes, trait=0)


def torse(t, m="h", large=0, bas=32):
    t.p(B([(17 - large, 17), (24, 14), (31 + large, 17), (32 + large, 26), (30, bas), (18, bas), (16 - large, 26)]), m, trait=0)


def robe(t, m="h", large=0, bas=45):
    """Robe longue qui cache les jambes."""
    t.p(B([(17, 17), (24, 14), (31, 17), (33 + large, 30), (35 + large, bas), (13 - large, bas), (15 - large, 30)]), m, trait=0)
    for x in (17, 21, 27, 31):
        t.p(L(x, 28, x + (x - 24) // 4, bas - 1), m, trait=0, ton=0, sur=m)


def bras(t, pts, m="h", peau="c", dl=0, trait=1, main=True):
    """Bras : epaule, coude, poignet ; la main au bout."""
    t.p(K([(p[0], p[1], r) for p, r in zip(pts, (2.2, 1.9, 1.7))]), m, trait=trait, dl=dl)
    if main:
        t.p(E(pts[-1][0], pts[-1][1], 1.8, 1.8), peau, trait=0, dl=dl)


def tete(t, peau="c", cx=23, cy=9, yeux=True, bouche=True):
    """Tete de trois quarts, tournee vers la gauche."""
    t.p(E(cx, cy, 4.2, 4.6), peau, trait=0)
    t.px((cx - 5, cy + 1), peau, 2)
    if yeux:
        t.px([(cx - 3, cy), (cx, cy)], "o")
    if bouche:
        t.px([(cx - 3, cy + 3), (cx - 2, cy + 3)], peau, 0)


def couronne(t, cx=23, y=4, m="or"):
    t.p(R(cx - 4, y, cx + 4, y + 1), m, trait=0)
    for dx, h in ((-4, 2), (-2, 3), (0, 4), (2, 3), (4, 2)):
        t.p(R(cx + dx, y - h, cx + dx, y), m, trait=0, ton=3)
    t.px([(cx - 2, y + 1), (cx + 2, y + 1)], m, 4)
