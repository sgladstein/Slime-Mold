"""How near each nest the river runs, from the maps `examples/planets out=DIR` writes.
python3 nestwater.py DIR SURVEY [pics]   (numpy, scipy, PIL: PYTHONPATH=/home/user/review-fun/pylib)

Only planets that rolled rivers (the survey's water dial). Per nest, straight
from its centre, in map cells: the nearest river water (wet, deep or a bridge
deck), the nearest bridge deck, and the nearest ford (a wet cell more than 20
map cells from deep water, in water that holds deep water somewhere: a ford, or
a road made shallow across). Water is cleared within 100 of a nest, so a river
by a nest runs dry there: that gap is a crossing at the nest."""
import sys, os, math
import numpy as np
from PIL import Image, ImageDraw
from scipy import ndimage

d, survey = sys.argv[1], sys.argv[2]
pics = len(sys.argv) > 3
rivers = []
for l in open(survey):
    w = l.split()
    if w and w[0].isdigit() and '[' in l and 'Rivers' in l.split('[')[1].split(']')[0]:
        rivers.append((int(w[0]), int(w[1])))
rows = []
for depth, seed in sorted(rivers):
    base = os.path.join(d, f'planet-{depth}-{seed}')
    g = np.array(Image.open(base + '.png'))[..., 1].astype(int)
    it = [l.split() for l in open(base + '.txt') if l.strip()]
    nests = [tuple(map(float, r[1:3])) for r in it if r[0] == 'mould']
    land = next(tuple(map(float, r[1:3])) for r in it if r[0] == 'landing')
    deep, wet, bridge = g == 0x41, (g & 0x0f) == 0x01, g == 0x50
    water = deep | wet | bridge
    lab, n = ndimage.label(water, structure=np.ones((3, 3)))
    rivery = np.zeros(n + 1, bool)
    rivery[np.unique(lab[deep | bridge])] = True
    rivery[0] = False
    river = rivery[lab]
    far_deep = ndimage.distance_transform_edt(~deep) > 20
    ford = river & wet & ~deep & far_deep
    def near(mask, q):
        ys, xs = np.nonzero(mask)
        return float(np.hypot(xs - q[0], ys - q[1]).min()) if len(xs) else float('inf')
    for k, q in enumerate(nests):
        rows.append((depth, seed, k, near(river, q), near(bridge, q), near(ford, q)))
    if pics:
        img = np.zeros(g.shape + (3,), np.uint8)
        img[...] = (60, 70, 50)
        img[(g & 8) != 0] = (110, 105, 100)
        img[wet] = (70, 110, 140)
        img[deep] = (20, 50, 110)
        img[bridge] = (200, 140, 60)
        img[ford] = (240, 200, 40)
        im = Image.fromarray(img)
        dr = ImageDraw.Draw(im)
        for q in nests:
            dr.ellipse([q[0] - 220, q[1] - 220, q[0] + 220, q[1] + 220], outline=(255, 60, 60), width=5)
        dr.ellipse([land[0] - 20, land[1] - 20, land[0] + 20, land[1] + 20], outline=(255, 255, 255), width=6)
        im.thumbnail((900, 900))
        os.makedirs('pics', exist_ok=True)
        im.save(f'pics/w{depth}-{seed}.png')
f = lambda v: 'none' if v == float('inf') else f'{v:.0f}'
for r in rows:
    print(f'{r[0]} {r[1]} nest {r[2]}: river {f(r[3])}, bridge {f(r[4])}, ford {f(r[5])}')
for depth in sorted({r[0] for r in rows}):
    rr = [r for r in rows if r[0] == depth]
    riv = sorted(r[3] for r in rr)
    cross = [min(r[4], r[5]) for r in rr]
    print(f'planet {depth}: {len(rr)} nests on {len({r[1] for r in rr})} planets | river within 220 of {sum(v < 220 for v in riv)}, '
          f'median nearest river {riv[len(riv) // 2]:.0f} (min {riv[0]:.0f}) | a ford or bridge within 300 of {sum(v < 300 for v in cross)}')
