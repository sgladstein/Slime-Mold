import sys, json
import numpy as np
from PIL import Image
from scipy import ndimage
out = {}
for tag in ("before", "after"):
    for d, s in ((8, 1), (8, 2), (5, 1), (4, 1), (4, 2), (4, 6), (8, 5)):
        base = f"{tag}/planet-{d}-{s}"
        g = np.array(Image.open(base + ".png").convert("RGBA"))[:, :, 1].astype(int)
        nests = [tuple(map(float, l.split()[1:3])) for l in open(base + ".txt") if l.startswith("mould")]
        br = (g >> 4) == 5
        lab, n = ndimage.label(br)
        bridge = None
        if n:
            sizes = ndimage.sum(br, lab, range(1, n + 1))
            k = int(np.argmax(sizes)) + 1
            ys, xs = np.nonzero(lab == k)
            bridge = (float(xs.mean()) / 2, float(ys.mean()) / 2)
        out[f"{tag}-{d}-{s}"] = {"nests": [(x / 2, y / 2) for x, y in nests], "bridge": bridge, "size": [g.shape[1], g.shape[0]]}
json.dump(out, open("points.json", "w"), indent=1)
for k, v in out.items():
    print(k, v)
