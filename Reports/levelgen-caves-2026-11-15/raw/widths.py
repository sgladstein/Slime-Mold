"""Tunnel widths on cave planets, from the maps `examples/planets out=DIR` writes.
python3 widths.py DIR [NAME...]   (needs numpy and scipy: PYTHONPATH=/home/user/review-fun/pylib)

Standable ground is not rock, not deep water and not a hedge (soft rock). Its
distance to the nearest unstandable cell (scipy's exact EDT, map cells) peaks
along each passage's middle; at those ridge cells (a 3x3 local maximum) twice
the distance is the passage's width in map cells, which is its width in dish
cells. Ridge cells under 30 dish wide are tunnels (caverns are 115+ across, a
nest's road 40+). Per planet: tunnel ridge cells' width percentiles, the share
under 8 and over 14, and the share of the map that is cavern (a cell at least
20 dish cells from rock, a room to fight in). Also the biggest such room's area,
in dish cells squared."""
import sys, os, math
import numpy as np
from PIL import Image
from scipy import ndimage

def walls_mask(shape, txt):
    m = Image.new('L', (shape[1], shape[0]), 0)
    from PIL import ImageDraw
    d = ImageDraw.Draw(m)
    for line in open(txt):
        w = line.split()
        if w and w[0] == 'wall':
            a, b, c, e, t = map(float, w[1:6])
            d.line([(a, b), (c, e)], fill=255, width=max(2, int(t)))
    return np.array(m) > 0

def measure(base):
    g = np.array(Image.open(base + '.png'))[..., 1].astype(int)
    stand = ((g & 8) == 0) & (g != 0x41) & ~walls_mask(g.shape, base + '.txt')
    edt = ndimage.distance_transform_edt(stand)
    ridge = stand & (edt >= ndimage.maximum_filter(edt, size=3)) & (edt > 1)
    wd = edt[ridge]  # width in dish cells = 2 * edt map cells / 2
    tun = wd[wd < 30]
    room = edt >= 40  # 20 dish cells of clearance, map cells
    lab, n = ndimage.label(room)
    big = 0
    if n:
        # a room's area: the cells within 40 map cells of its core, roughly; count the core's own area grown back
        sizes = ndimage.sum(np.ones_like(lab), lab, range(1, n + 1))
        k = int(np.argmax(sizes)) + 1
        grown = ndimage.binary_dilation(lab == k, iterations=40) & stand
        big = grown.sum() / 4
    p = np.percentile(tun, [10, 50, 90]) if len(tun) else [0, 0, 0]
    return dict(p10=p[0], p50=p[1], p90=p[2], lt8=(tun < 8).mean() if len(tun) else 0, gt14=(tun > 14).mean() if len(tun) else 0,
                room=room.mean(), big=big, stand=stand.mean())

d = sys.argv[1]
names = sys.argv[2:] or sorted(x[:-4] for x in os.listdir(d) if x.startswith('planet-7-') and x.endswith('.png'))
print('planet | tunnel width p10 p50 p90 (dish) | under 8 | over 14 | rooms (share of map) | biggest room (dish^2) | standable')
rows = []
for n in names:
    m = measure(os.path.join(d, n))
    rows.append(m)
    print(f"{n} | {m['p10']:.0f} {m['p50']:.0f} {m['p90']:.0f} | {m['lt8']:.0%} | {m['gt14']:.0%} | {m['room']:.1%} | {m['big']:.0f} | {m['stand']:.0%}")
med = lambda k: np.median([r[k] for r in rows])
print(f"median | {med('p10'):.0f} {med('p50'):.0f} {med('p90'):.0f} | {med('lt8'):.0%} | {med('gt14'):.0%} | {med('room'):.1%} | {med('big'):.0f} | {med('stand'):.0%}")
