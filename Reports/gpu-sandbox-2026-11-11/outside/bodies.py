# bodies.py DIR TICKS_A_FRAME: drift.py (Reports/creatures-2026-11-11/) on the
# sandbox's frames: bodies in the last frame, their median size, and how far
# their centres moved over the last 2 s (240 ticks; the sandbox saves a
# frame every TICKS_A_FRAME ticks, 8 in outside/run.sh). A cell counts as a
# body cell if it is brighter than half (the sandbox draws grass as a dim
# green ground, under 21 of 255 in grey; a walker's cell is over 240).
# Needs numpy, pillow, scipy.
import sys, glob
import numpy as np
from PIL import Image
from scipy import ndimage as nd
fs = sorted(glob.glob(sys.argv[1] + '/*.png'))
back = round(240 / int(sys.argv[2]))
def cents(f):
    a = np.array(Image.open(f).convert('L')) > 128
    a = nd.binary_closing(a, iterations=2); l, n = nd.label(a); s = nd.sum(a, l, range(1, n + 1)); keep = [i + 1 for i in range(n) if s[i] > 30]
    return np.array(nd.center_of_mass(a, l, keep)), s[s > 30]
i, j = len(fs) - 1 - back, len(fs) - 1
c1, s1 = cents(fs[i]); c2, _ = cents(fs[j])
d = np.sqrt(((c1[:, None, :] - c2[None, :, :]) ** 2).sum(-1)).min(1)
print(len(c1), 'bodies, median size', np.median(s1), 'drift over', back, 'frames: median', round(np.median(d), 2), 'p90', round(np.percentile(d, 90), 2))
