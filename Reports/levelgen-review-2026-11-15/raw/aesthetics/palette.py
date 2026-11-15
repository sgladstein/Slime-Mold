# Per planet: the ground's mean colour and spread at 1 s (no slime, no crystal),
# and the slime's share of the world at 1 s and 60 s.
import sys, colorsys
sys.path.insert(0, '/home/user/review-aesthetics/scripts')
import numpy as np
from PIL import Image
from worldrect import rect
R = '/home/user/review-aesthetics/'
B = ['meadow', 'barrens', 'fen', 'valley', 'jungle', 'highlands', 'caves', 'core']
print('planet  ground mean RGB   luma p10/p50/p90  sat  | slime share 1s 60s')
for d in range(1, 9):
    for s in (1, 2):
        try:
            f1 = np.array(Image.open(R + f'frames/main/planet-{d}-{s}-idle-001s-all.png').convert('RGB')).astype(float)
            f60 = np.array(Image.open(R + f'frames/main/planet-{d}-{s}-idle-060s-all.png').convert('RGB')).astype(float)
        except FileNotFoundError:
            continue
        Wm, Hm = Image.open(R + f'maps/planet-{d}-{s}.png').size
        x0, y0, z = rect(f1, Wm / 2, Hm / 2)
        x1 = int(x0 + z * Wm / 2); y1 = min(700, int(y0 + z * Hm / 2))
        a1 = f1[y0 + 4:y1, x0 + 4:x1 - 4]; a60 = f60[y0 + 4:y1, x0 + 4:x1 - 4]
        def slime(a): return (a[..., 0] - a[..., 2]) > 45
        cyan = (a1[..., 2] - a1[..., 0] > 40)
        g = a1[~slime(a1) & ~cyan & (a1.sum(2) < 400)]
        L = 0.299 * g[:, 0] + 0.587 * g[:, 1] + 0.114 * g[:, 2]
        mx = g.max(1); mn = g.min(1); sat = np.where(mx > 0, (mx - mn) / np.maximum(mx, 1), 0)
        m = g.mean(0)
        print(f'P{d} s{s} {B[d-1]:9s} ({m[0]:.0f},{m[1]:.0f},{m[2]:.0f})  {np.percentile(L,10):.0f}/{np.percentile(L,50):.0f}/{np.percentile(L,90):.0f}  {sat.mean():.2f} | {slime(a1).mean():.2f} {slime(a60).mean():.2f}')
