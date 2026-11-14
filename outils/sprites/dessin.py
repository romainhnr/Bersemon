# Outil de dessin des sprites de Bersemon : des formes pleines, ombrees
# d'apres une lumiere venue d'en haut a gauche, puis un contour sombre.
# Le resultat est ecrit au format texte de crates/bersemon-discord/sprites.
import colorsys
import math
import random

LUMIERE = (-0.6, -0.8)
CARACTERES = (
    "abcdefghijklmnpqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
)


# Reduction appliquee a tout le dessin autour du centre de la grille, pour
# faire tenir un sprite trop grand sans reprendre chaque coordonnee.
ZOOM = [1.0]


def _z(v):
    return 24 + (v - 24) * ZOOM[0]


# ---------------------------------------------------------------- masques
def E(cx, cy, rx, ry, rot=0.0):
    """Ellipse pleine, tournee de `rot` degres."""
    cx, cy, rx, ry = _z(cx), _z(cy), rx * ZOOM[0], ry * ZOOM[0]
    a = math.radians(rot)
    ca, sa = math.cos(a), math.sin(a)
    r = int(max(rx, ry)) + 2
    m = set()
    for y in range(int(cy) - r, int(cy) + r + 1):
        for x in range(int(cx) - r, int(cx) + r + 1):
            dx, dy = x - cx, y - cy
            u = (dx * ca + dy * sa) / rx
            v = (-dx * sa + dy * ca) / ry
            if u * u + v * v <= 1.0:
                m.add((x, y))
    return m


def P(pts):
    """Polygone plein."""
    pts = [(_z(p[0]), _z(p[1])) for p in pts]
    ys = [p[1] for p in pts]
    xs = [p[0] for p in pts]
    m = set()
    n = len(pts)
    for y in range(int(math.floor(min(ys))), int(math.ceil(max(ys))) + 1):
        for x in range(int(math.floor(min(xs))), int(math.ceil(max(xs))) + 1):
            dedans = False
            j = n - 1
            for i in range(n):
                xi, yi = pts[i]
                xj, yj = pts[j]
                if (yi > y) != (yj > y):
                    xc = (xj - xi) * (y - yi) / (yj - yi) + xi
                    if x < xc:
                        dedans = not dedans
                j = i
            if dedans:
                m.add((x, y))
    # Les sommets et les aretes comptent : sans eux les pointes disparaissent.
    for i in range(n):
        m |= _ligne(pts[i][0], pts[i][1], pts[(i + 1) % n][0], pts[(i + 1) % n][1])
    return m


def _spline(pts, ferme, pas=8):
    n = len(pts)
    out = []
    rng = range(n) if ferme else range(n - 1)
    for i in rng:
        if ferme:
            p0, p1, p2, p3 = (pts[(i - 1) % n], pts[i], pts[(i + 1) % n],
                              pts[(i + 2) % n])
        else:
            p0 = pts[max(i - 1, 0)]
            p1, p2 = pts[i], pts[i + 1]
            p3 = pts[min(i + 2, n - 1)]
        for k in range(pas):
            t = k / pas
            t2, t3 = t * t, t * t * t
            out.append(tuple(
                0.5 * ((2 * p1[d]) + (-p0[d] + p2[d]) * t
                       + (2 * p0[d] - 5 * p1[d] + 4 * p2[d] - p3[d]) * t2
                       + (-p0[d] + 3 * p1[d] - 3 * p2[d] + p3[d]) * t3)
                for d in range(len(p1))))
    if not ferme:
        out.append(tuple(pts[-1]))
    return out


def B(pts):
    """Forme fermee aux bords lisses, qui passe par les points."""
    return P([(p[0], p[1]) for p in _spline(pts, True)])


def C(x0, y0, x1, y1, r0, r1=None):
    """Capsule : un segment epais, dont le rayon va de r0 a r1."""
    if r1 is None:
        r1 = r0
    x0, y0, x1, y1, r0, r1 = _z(x0), _z(y0), _z(x1), _z(y1), r0 * ZOOM[0], r1 * ZOOM[0]
    m = set()
    r = max(r0, r1) + 1
    dx, dy = x1 - x0, y1 - y0
    d2 = dx * dx + dy * dy
    for y in range(int(min(y0, y1) - r), int(max(y0, y1) + r) + 2):
        for x in range(int(min(x0, x1) - r), int(max(x0, x1) + r) + 2):
            t = 0.0 if d2 == 0 else max(0.0, min(1.0, ((x - x0) * dx + (y - y0) * dy) / d2))
            px, py = x0 + t * dx, y0 + t * dy
            rr = r0 + (r1 - r0) * t
            if (x - px) ** 2 + (y - py) ** 2 <= rr * rr:
                m.add((x, y))
    return m


def S(pts):
    """Trait lisse et epais qui passe par des points (x, y, rayon)."""
    c = _spline(pts, False)
    m = set()
    for a, b in zip(c, c[1:]):
        m |= C(a[0], a[1], b[0], b[1], a[2], b[2])
    return m


def K(pts):
    """Chaine de capsules qui passe par des points (x, y, rayon), sans lissage :
    un membre et ses articulations."""
    m = set()
    for a, b in zip(pts, pts[1:]):
        m |= C(a[0], a[1], b[0], b[1], a[2], b[2])
    return m


def L(x0, y0, x1, y1):
    """Ligne d'un pixel."""
    return _ligne(_z(x0), _z(y0), _z(x1), _z(y1))


def _ligne(x0, y0, x1, y1):
    m = set()
    n = int(max(abs(x1 - x0), abs(y1 - y0))) + 1
    for i in range(n + 1):
        t = i / n
        m.add((int(round(x0 + (x1 - x0) * t)), int(round(y0 + (y1 - y0) * t))))
    return m


def LS(pts):
    """Ligne brisee d'un pixel."""
    m = set()
    for a, b in zip(pts, pts[1:]):
        m |= L(a[0], a[1], b[0], b[1])
    return m


def CS(pts):
    """Courbe lisse d'un pixel qui passe par les points."""
    return LS(_spline(pts, False))


def R(x0, y0, x1, y1):
    x0, y0, x1, y1 = (int(round(_z(v))) for v in (x0, y0, x1, y1))
    return {(x, y) for x in range(x0, x1 + 1) for y in range(y0, y1 + 1)}


def mir(m, axe):
    """Symetrique du masque par rapport a la colonne `axe` (peut etre x.5)."""
    return {(int(round(2 * axe - x)), y) for x, y in m}


def dec(m, dx, dy):
    return {(x + dx, y + dy) for x, y in m}


def bord(m):
    """Pixels du masque qui touchent l'exterieur."""
    return {(x, y) for x, y in m
            if not {(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)} <= m}


def grossir(m, n=1):
    for _ in range(n):
        m = m | {(x + dx, y + dy) for x, y in m for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))}
    return m


def reduire(m, n=1):
    for _ in range(n):
        m = m - bord(m)
    return m


# ---------------------------------------------------------------- couleurs
def _hexa(c):
    c = c.lstrip("#")
    return tuple(int(c[i:i + 2], 16) / 255 for i in (0, 2, 4))


def _vers_hexa(rgb):
    return "".join(f"{max(0, min(255, int(round(v * 255)))):02x}" for v in rgb)


def _tourner(h, cible, part):
    d = ((cible - h + 0.5) % 1.0) - 0.5
    return (h + d * part) % 1.0


def rampe(base):
    """Cinq tons d'une couleur : les ombres tirent vers le violet, les
    lumieres vers le jaune."""
    r, g, b = _hexa(base)
    h, l, s = colorsys.rgb_to_hls(r, g, b)
    tons = []
    for fl, fs, cible, part in ((0.42, 1.05, 0.72, 0.22), (0.68, 1.02, 0.72, 0.10),
                                (1.0, 1.0, 0.0, 0.0), (None, 0.92, 0.14, 0.10),
                                (None, 0.75, 0.14, 0.18)):
        if fl is None:
            k = 0.36 if len(tons) == 3 else 0.68
            nl = l + (1 - l) * k
        else:
            nl = l * fl
        nh = _tourner(h, cible, part) if s > 0.05 else h
        tons.append(_vers_hexa(colorsys.hls_to_rgb(nh, nl, min(1.0, s * fs))))
    return tons


def shiny_auto(base, decalage, teinte):
    """Couleur shiny tiree de la couleur normale : la teinte tourne ; un
    gris, sans teinte a tourner, se mele a la couleur `teinte`."""
    r, g, b = _hexa(base)
    h, l, s = colorsys.rgb_to_hls(r, g, b)
    if s < 0.14:
        tr, tg, tb = _hexa(teinte)
        th, tl, ts = colorsys.rgb_to_hls(tr, tg, tb)
        return _vers_hexa(colorsys.hls_to_rgb(th, l * 0.85 + tl * 0.15, 0.38))
    return _vers_hexa(colorsys.hls_to_rgb((h + decalage) % 1.0, l, s))


# ------------------------------------------------------------------ toile
class Toile:
    def __init__(self, nom, cote=48):
        self.nom = nom
        self.n = cote
        self.pix = {}          # (x, y) -> (matiere, ton)
        self.matieres = {}     # nom -> {"normal": [5 tons], variante: [5 tons]}
        self.ordre = []
        self.variantes = ["shiny"]
        # Le shiny des matieres qui n'en disent rien : teinte tournee, ou
        # gris mele a cette couleur.
        self.decalage_shiny = 0.5
        self.teinte_shiny = "c8a040"
        self.mat("o", "1b1420", "1b1420")

    # -- matieres
    def mat(self, nom, base, shiny=None, **variantes):
        """Declare une matiere : sa couleur de base (ou ses cinq tons), sa
        couleur shiny, et celles des autres variantes eventuelles."""
        def tons(c):
            return list(c) if isinstance(c, (list, tuple)) else rampe(c)
        normal = tons(base)
        if shiny is None:
            shiny = [shiny_auto(c, self.decalage_shiny, self.teinte_shiny) for c in normal]
        m = {"normal": normal, "shiny": tons(shiny)}
        for v, c in variantes.items():
            m[v] = tons(c)
        if nom not in self.matieres:
            self.ordre.append(nom)
        self.matieres[nom] = m
        return self

    # -- peinture
    def p(self, masque, matiere, trait=1, dl=0, ton=None, s=(0.38, -0.30, -0.72), sur=None):
        """Peint le masque avec la matiere, ombree d'apres la lumiere.
        trait : 0 aucun, 1 trait sombre du cote de l'ombre sur ce qui est
        dessous, 2 tout autour. dl decale les tons (-1 plus sombre).
        ton impose un ton uni. sur : ne peint que sur ces matieres."""
        masque = set(masque)
        if sur is not None:
            sur = (sur,) if isinstance(sur, str) else sur
            masque = {q for q in masque if q in self.pix and self.pix[q][0] in sur}
        if not masque:
            return masque
        if trait:
            for (x, y) in masque:
                voisins = ((x + 1, y), (x, y + 1)) if trait == 1 else (
                    (x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1))
                for q in voisins:
                    if q not in masque and q in self.pix and self.pix[q][0] != "o":
                        self.pix[q] = (self.pix[q][0], 0)
        for q in masque:
            if ton is not None:
                t = ton
            else:
                v = self._eclairage(q, masque)
                t = 3 if v > s[0] else (0 if v < s[2] else (1 if v < s[1] else 2))
                t = max(0, min(4, t + dl))
            self.pix[q] = (matiere, t)
        return masque

    def _eclairage(self, q, masque):
        def marche(sens):
            x, y = q
            d = 0.0
            while d < 80:
                d += 0.5
                xx = int(round(x + LUMIERE[0] * d * sens))
                yy = int(round(y + LUMIERE[1] * d * sens))
                if (xx, yy) not in masque:
                    return d
            return d
        vers, loin = marche(1), marche(-1)
        return (loin - vers) / (loin + vers)

    def px(self, pts, matiere, ton=2):
        """Pose des pixels d'un ton donne. pts : un point ou une liste."""
        if isinstance(pts, tuple) and len(pts) == 2 and not isinstance(pts[0], tuple):
            pts = [pts]
        for q in pts:
            self.pix[(int(round(_z(q[0]))), int(round(_z(q[1]))))] = (matiere, ton)

    def oeil(self, x, y, iris="y", large=2, pupille=True, reflet=False):
        """Un oeil a (x, y), `large` pixels de large : iris clair, pupille
        sombre du cote du regard (a gauche), reflet au besoin."""
        x, y = int(round(_z(x))), int(round(_z(y)))
        for i in range(large):
            self.pix[(x + i, y)] = (iris, 3)
        if pupille:
            self.pix[(x, y)] = ("o", 2)
        if reflet:
            self.pix[(x + large - 1, y - 1)] = (iris, 4)

    def ton(self, masque, t, sur=None):
        """Change le ton des pixels deja peints du masque."""
        for q in masque:
            if q in self.pix and self.pix[q][0] != "o":
                if sur is None or self.pix[q][0] == sur:
                    self.pix[q] = (self.pix[q][0], t)

    def ombre(self, masque, d=-1, sur=None):
        """Assombrit (ou eclaire, d > 0) les pixels deja peints du masque."""
        for q in masque:
            if q in self.pix and self.pix[q][0] != "o":
                if sur is None or self.pix[q][0] == sur:
                    m, t = self.pix[q]
                    self.pix[q] = (m, max(0, min(4, t + d)))

    def efface(self, masque):
        for q in masque:
            self.pix.pop(q, None)

    def art(self, x, y, lignes, legende):
        """Tamponne un petit dessin : legende = {car: (matiere, ton)}."""
        for j, ligne in enumerate(lignes):
            for i, c in enumerate(ligne):
                if c in legende:
                    self.pix[(x + i, y + j)] = legende[c]

    def grain(self, matiere, part, graine=1, de=2, vers=1):
        """Texture : passe au hasard une part des pixels d'un ton a un autre."""
        rnd = random.Random(graine)
        for q in sorted(self.pix):
            if self.pix[q] == (matiere, de) and rnd.random() < part:
                self.pix[q] = (matiere, vers)

    def masque(self, matiere=None):
        return {q for q, (m, _) in self.pix.items() if matiere is None or m == matiere}

    # -- finition
    def cadrer(self, bas=True, centre=True):
        """Pose le dessin en bas de la grille et le centre."""
        xs = [q[0] for q in self.pix]
        ys = [q[1] for q in self.pix]
        dx = ((self.n - 1 - max(xs) - min(xs)) // 2) if centre else 0
        dy = (self.n - 2 - max(ys)) if bas else 0
        self.pix = {(x + dx, y + dy): v for (x, y), v in self.pix.items()}

    def contour(self):
        ext = set()
        for (x, y) in self.pix:
            for q in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                if q not in self.pix:
                    ext.add(q)
        for q in ext:
            self.pix[q] = ("o", 2)

    def finir(self, cadrer=True, contour=True):
        if cadrer:
            self.cadrer()
        if contour:
            self.contour()
        hors = [q for q in self.pix if not (0 <= q[0] < self.n and 0 <= q[1] < self.n)]
        if hors:
            xs = [q[0] for q in self.pix]
            ys = [q[1] for q in self.pix]
            raise ValueError(f"{self.nom} : dessin de {max(xs) - min(xs) + 1} x "
                             f"{max(ys) - min(ys) + 1} avec son contour, au plus {self.n} x {self.n}")
        return self

    # -- sorties
    def _cles(self):
        utilises = sorted({v for v in self.pix.values()},
                          key=lambda v: (self.ordre.index(v[0]), v[1]))
        cles = {}
        libres = iter(CARACTERES)
        for v in utilises:
            cles[v] = "o" if v[0] == "o" else next(libres)
        return cles

    def grille(self):
        cles = self._cles()
        return ["".join(cles[self.pix[(x, y)]] if (x, y) in self.pix else "."
                        for x in range(self.n)) for y in range(self.n)]

    def texte(self, titre, variantes=("shiny",), autre_dessin=None):
        cles = self._cles()
        out = [f"# {titre}"]
        for v in ("normal",) + tuple(variantes):
            out.append("palette" if v == "normal" else v)
            for (m, t), c in cles.items():
                out.append(f"{c} {self.matieres[m][v][t]}")
        out.append("dessin")
        out += self.grille()
        if autre_dessin is not None:
            out.append(autre_dessin[0])
            out += autre_dessin[1]
        return "\n".join(out) + "\n"

    def image(self, variante="normal", echelle=3):
        from PIL import Image
        im = Image.new("RGBA", (self.n, self.n), (0, 0, 0, 0))
        for (x, y), (m, t) in self.pix.items():
            c = self.matieres[m][variante][t]
            im.putpixel((x, y), tuple(int(c[i:i + 2], 16) for i in (0, 2, 4)) + (255,))
        return im.resize((self.n * echelle, self.n * echelle), Image.NEAREST)
