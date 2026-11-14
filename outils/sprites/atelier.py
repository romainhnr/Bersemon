# Atelier des sprites de Bersemon (docs/contenu-v1.md, section 42).
#   python3 atelier.py lot_01            dessine la planche de controle du lot
#   python3 atelier.py lot_01 loup       seulement ces sprites
#   python3 atelier.py lot_01 publier    ecrit les fichiers texte du depot
# Les fichiers texte de crates/bersemon-discord/sprites restent la reference :
# ces scripts ne servent qu'a les produire. Ils demandent Python 3 et Pillow.
import importlib, os, sys
from PIL import Image, ImageDraw
import dessin
from dessin import Toile

SPRITES = {}
ICI = os.path.dirname(os.path.abspath(__file__))
# Les planches de controle vont dans BERSEMON_PLANCHES, ou a cote des scripts.
SORTIE = os.environ.get("BERSEMON_PLANCHES", os.path.join(ICI, "rendu"))
DEPOT = os.path.join(ICI, "..", "..", "crates", "bersemon-discord", "sprites")

def sprite(ident, titre, dossier="", **opts):
    def deco(f):
        SPRITES[ident] = (titre, dossier, f, opts)
        return f
    return deco

def construire(ident):
    """Dessine le sprite ; s'il deborde de la grille, le reduit pas a pas."""
    titre, dossier, f, opts = SPRITES[ident]
    zoom = opts.get("zoom", 1.0)
    while True:
        t = Toile(ident)
        dessin.ZOOM[0] = zoom
        f(t)
        dessin.ZOOM[0] = 1.0
        try:
            if not opts.get("fini"):
                t.finir()
            if zoom != opts.get("zoom", 1.0):
                print(f"{ident} : zoom {zoom:.2f}")
            return t
        except ValueError:
            zoom -= 0.02
            if zoom < 0.7:
                raise


def planche(ids, sortie, cols=6, echelle=3, variante="normal", fond=(70, 90, 70)):
    c = 48 * echelle
    h = c + 12
    rows = (len(ids) + cols - 1) // cols
    im = Image.new("RGB", (cols * c, rows * h), fond)
    d = ImageDraw.Draw(im)
    for i, ident in enumerate(ids):
        try:
            t = construire(ident)
        except ValueError as erreur:
            print("ERREUR", erreur)
            continue
        s = t.image(variante, echelle)
        x, y = (i % cols) * c, (i // cols) * h
        im.paste(s, (x, y), s)
        d.text((x + 2, y + c), ident[:22], fill=(255, 255, 255))
    im.save(sortie)

def publier(ids):
    """Ecrit les sprites dans le depot."""
    for ident in ids:
        titre, dossier, f, opts = SPRITES[ident]
        t = construire(ident)
        variantes = opts.get("variantes", ("shiny",))
        chemin = os.path.join(DEPOT, dossier, opts.get("fichier", ident) + ".txt")
        with open(chemin, "w", encoding="utf-8", newline="\n") as sortie:
            sortie.write(t.texte(titre, variantes))

if __name__ == "__main__":
    import atelier as A
    for m in sys.argv[1].split(","):
        importlib.import_module(m)
    args = sys.argv[2:]
    if args and args[0] == "publier":
        ids = args[1:] or list(A.SPRITES)
        A.publier(ids)
        print("publies", len(ids))
        sys.exit(0)
    ids = args or list(A.SPRITES)
    os.makedirs(SORTIE, exist_ok=True)
    A.planche(ids, os.path.join(SORTIE, "planche.png"), cols=min(4, len(ids)), echelle=6)
    A.planche(ids, os.path.join(SORTIE, "planche-shiny.png"), cols=min(8, len(ids)), variante="shiny")
    print("ok", len(ids))
