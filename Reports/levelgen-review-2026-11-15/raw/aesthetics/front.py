# Pick a point on the 60 s slime front near water and plants (world units).
import sys
sys.path.insert(0, '/home/user/review-aesthetics/scripts')
import numpy as np
from PIL import Image
from worldrect import rect
R = '/home/user/review-aesthetics/'
for name in sys.argv[1:]:
    f60 = np.array(Image.open(R + f'frames/main/{name}-idle-060s-all.png').convert('RGB')).astype(int)
    m = np.array(Image.open(R + f'maps/{name}.png').convert('RGBA')).astype(int)
    G, B = m[..., 1], m[..., 2]; Hm, Wm = G.shape
    items = [l.split() for l in open(R + f'maps/{name}.txt') if l.strip()]
    crys = [tuple(map(float, p[1:4])) for p in items if p[0] == 'crystal']
    x0, y0, z = rect(f60, Wm / 2, Hm / 2)
    slime = (f60[..., 0] - f60[..., 2]) > 45
    best = None
    for py in range(80, 690, 4):
        for px in range(x0 + 60, int(x0 + z * Wm / 2) - 60, 4):
            if not slime[py, px] or slime[py, px + 4] == slime[py, px] and slime[py + 4, px] == slime[py, px] and slime[py, px - 4] and slime[py - 4, px]:
                continue
            mx, my = (px - x0) / z * 2, (py - y0) / z * 2
            if any((mx - x) ** 2 + (my - y) ** 2 < (1.5 * r + 60) ** 2 for x, y, r in crys): continue
            a, b = int(max(0, my - 60)), int(max(0, mx - 100))
            gw = G[a:int(my + 60), b:int(mx + 100)]; bw = B[a:int(my + 60), b:int(mx + 100)]
            if gw.size == 0: continue
            water = np.mean((gw & 1) > 0); plants = np.mean((bw & 63) > 20); rock = np.mean((gw & 8) > 0)
            sc = min(water, 0.4) + min(plants, 0.4) - rock
            if best is None or sc > best[0]: best = (sc, mx / 2, my / 2, water, plants)
    print(name, 'front %.0f,%.0f' % (best[1], best[2]), 'water %.2f plants %.2f' % (best[3], best[4]))
