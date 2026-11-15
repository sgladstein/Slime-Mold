import numpy as np
from PIL import Image
BACK = np.array([96, 98, 102])
def rect(a, Ww, Hh, formula=None):
    """(x0, y0, zoom) of the world in a whole-map frame."""
    if formula or Ww >= 800:
        # camera math: the band between the HUD insets is about 676 px from y=45
        z = min(1280 / Ww, 676 / Hh)
        return int(round(640 - z * Ww / 2)), int(round(45 + (676 - z * Hh) / 2)), z
    a = a.astype(int)
    nb = np.abs(a - BACK).max(2) > 12
    xs0, xs1, ys0 = [], [], []
    for y in (300, 350, 400, 450):
        xs = np.nonzero(nb[y, 60:1220])[0] + 60
        xs0.append(xs[0]); xs1.append(xs[-1] + 1)
    x0, x1 = int(np.median(xs0)), int(np.median(xs1))
    z = (x1 - x0) / Ww
    for x in (x0 + (x1 - x0) // 3, x0 + (x1 - x0) // 2, x0 + 2 * (x1 - x0) // 3):
        ys = np.nonzero(nb[30:300, x])[0] + 30
        ys0.append(ys[0])
    y0 = int(np.median(ys0))
    return x0, y0, z
