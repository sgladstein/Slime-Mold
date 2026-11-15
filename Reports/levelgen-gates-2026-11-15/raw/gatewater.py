"""Soft-rock gates standing in water, from the maps `examples/planets out=DIR` writes.
python3 gatewater.py DIR DEPTH [SEEDS]   (numpy, PIL: PYTHONPATH=/home/user/review-fun/pylib)

A gate (or ring arc, or hedge) is a chain of wall pieces end to end. A piece is on
water if any point along it (every 4 map cells) is wet or deep ground. Per planet:
chains, chains with a piece on water, pieces, pieces on water."""
import sys, os
import numpy as np
from PIL import Image
d, depth = sys.argv[1], int(sys.argv[2])
seeds = range(1, 21) if len(sys.argv) < 4 else [int(s) for s in sys.argv[3].split(',')]
tot = np.zeros(4, int)
for s in seeds:
    base = os.path.join(d, f'planet-{depth}-{s}')
    if not os.path.exists(base + '.png'):
        continue
    g = np.array(Image.open(base + '.png'))[..., 1].astype(int)
    h, w = g.shape
    wet = (g & 1) != 0
    walls = [tuple(map(float, l.split()[1:6])) for l in open(base + '.txt') if l.startswith('wall')]
    chains, cur = [], []
    for wl in walls:
        if cur and (abs(cur[-1][2] - wl[0]) > 0.5 or abs(cur[-1][3] - wl[1]) > 0.5):
            chains.append(cur); cur = []
        cur.append(wl)
    if cur:
        chains.append(cur)
    def on(wl):
        ax, ay, bx, by, _ = wl
        n = max(2, int(np.hypot(bx - ax, by - ay) / 4) + 1)
        for k in range(n):
            t = k / (n - 1)
            x, y = int(ax + (bx - ax) * t), int(ay + (by - ay) * t)
            if 0 <= x < w and 0 <= y < h and wet[y, x]:
                return True
        return False
    pw = sum(on(wl) for wl in walls)
    cw = sum(any(on(wl) for wl in c) for c in chains)
    row = np.array([len(chains), cw, len(walls), pw])
    tot += row
    print(f'{depth}-{s}: chains {len(chains)}, on water {cw}; pieces {len(walls)}, on water {pw}')
print(f'planet {depth}, {len(list(seeds))} seeds: chains {tot[0]}, on water {tot[1]}; pieces {tot[2]}, on water {tot[3]}')
