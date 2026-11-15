# False-colour layout of a WM1 map PNG: soil kinds, wet, rock, plants, items.
import sys
from PIL import Image, ImageDraw
import numpy as np
src, out = sys.argv[1], sys.argv[2]
im = np.array(Image.open(src).convert('RGBA'))
g = im[:, :, 1].astype(int); b = im[:, :, 2].astype(int)
h, w = g.shape
col = np.zeros((h, w, 3), np.uint8)
kind = g >> 4
col[:] = (60, 60, 60)
col[kind == 1] = (200, 180, 110)   # sand
col[kind == 2] = (150, 100, 80)    # clay
col[kind == 3] = (110, 105, 100)   # barren
col[(g & 1) == 1] = (60, 100, 160)  # wet
col[kind == 4] = (20, 40, 110)     # deep
col[kind == 5] = (160, 110, 50)    # bridge
col[(g & 8) == 8] = (180, 180, 180)  # rock
dens = (b & 63) / 63.0
pk = b >> 6
plantc = {0: (90, 170, 70), 1: (30, 120, 40), 2: (120, 150, 60), 3: (80, 160, 140)}
for k, c in plantc.items():
    m = (dens > 0) & (pk == k)
    a = dens[m][:, None] * 0.8
    col[m] = (col[m] * (1 - a) + np.array(c) * a).astype(np.uint8)
img = Image.fromarray(col)
d = ImageDraw.Draw(img)
for line in open(src[:-4] + '.txt'):
    p = line.split()
    if not p: continue
    if p[0] == 'crystal':
        x, y, r = float(p[1]), float(p[2]), float(p[3]); d.ellipse([x-r, y-r, x+r, y+r], outline=(0, 255, 255), width=3)
    elif p[0] == 'mould':
        x, y = float(p[1]), float(p[2]); d.ellipse([x-14, y-14, x+14, y+14], fill=(255, 160, 0))
    elif p[0] == 'landing':
        x, y = float(p[1]), float(p[2]); d.rectangle([x-12, y-12, x+12, y+12], fill=(255, 255, 255))
    elif p[0] == 'wall':
        x0, y0, x1, y1, wd = map(float, p[1:6]); d.line([x0, y0, x1, y1], fill=(230, 120, 200), width=int(wd/2))
img.save(out)
