# Morceaux d'anatomie partages par plusieurs lots.
from dessin import *


def _aile_membrane(t, m, os_, base, coude, pointes, dl=0):
    """Aile de chauve-souris : un bras, des doigts, la membrane tendue entre."""
    pts = [base, coude] + list(pointes)
    t.p(P(pts), m, trait=0, dl=dl)
    for p in pointes:
        t.p(K([(coude[0], coude[1], 0.8), (p[0], p[1], 0.4)]), os_, trait=0, dl=dl)
    t.p(K([(base[0], base[1], 1.3), (coude[0], coude[1], 0.9)]), os_, trait=0, dl=dl)
