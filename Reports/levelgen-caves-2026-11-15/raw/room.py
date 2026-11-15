import sys, numpy as np
from PIL import Image
from scipy import ndimage
for d in sys.argv[1:]:
    out = []
    for s in range(1, 21):
        b = f"{d}/planet-7-{s}"
        it = [l.split() for l in open(b + '.txt') if l.strip()]
        homes = [tuple(map(float, r[1:3])) for r in it if r[0] in ('landing', 'mould')]
        g = np.array(Image.open(b + '.png'))[..., 1].astype(int)
        e = ndimage.distance_transform_edt((g & 8) == 0)
        yy, xx = np.mgrid[0:g.shape[0], 0:g.shape[1]]
        for q in homes:
            e[(xx - q[0]) ** 2 + (yy - q[1]) ** 2 < 260 ** 2] = 0
        out.append(int(e.max()))
    print(d, sorted(out))
