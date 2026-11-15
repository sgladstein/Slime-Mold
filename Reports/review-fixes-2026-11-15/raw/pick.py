"""Pick each planet's flora window: pick.py MAPS (the planets example's
out= folder, seed 1). Prints PLANET X Y (world cells) for films.sh: the
160 x 100 map-pixel window (maps are 2 px a cell) richest in the flora's
typical kind, 220 px or more from every nest."""
import sys
import numpy as np
from PIL import Image
O = sys.argv[1]
# Kinds: 0 meadow, 1 jungle, 2 reeds, 3 scrub; None, any.
pref = {1: 0, 2: 3, 3: 0, 4: 0, 5: 1, 6: 0, 7: None, 8: None}
for p in range(1, 9):
    b = np.asarray(Image.open(f"{O}/planet-{p}-1.png").convert("RGB")).astype(int)[..., 2]
    k, d = b >> 6, (b & 63) / 63.0
    if pref[p] is not None:
        d = np.where(k == pref[p], d, 0)
    moulds = [tuple(map(float, l.split()[1:3])) for l in open(f"{O}/planet-{p}-1.txt") if l.startswith("mould")]
    H, W = b.shape; wx, wy = 160, 100; best = None
    for y0 in range(0, H - wy, 10):
        for x0 in range(0, W - wx, 10):
            cx, cy = x0 + wx / 2, y0 + wy / 2
            if any((cx - mx) ** 2 + (cy - my) ** 2 < 220 ** 2 for mx, my in moulds):
                continue
            sub = d[y0:y0 + wy, x0:x0 + wx]
            sc = sub.mean() + 0.5 * min((sub > 0.05).mean(), 0.9)
            if best is None or sc > best[0]:
                best = (sc, x0, y0)
    print(p, (best[1] + wx // 2) // 2, (best[2] + wy // 2) // 2)
