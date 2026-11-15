"""Where to point the camera on a cave planet, in world cells (map cells / 2):
the landing, halfway to the first nest, and the after map's widest room away
from the landing and nests (an arena or the jackpot's cavern).
python3 spots.py AFTER_DIR SEED   ->  'landing X,Y' 'mid X,Y' 'room X,Y'"""
import sys
import numpy as np
from PIL import Image
from scipy import ndimage
d, s = sys.argv[1], sys.argv[2]
base = f"{d}/planet-7-{s}"
items = [l.split() for l in open(base + '.txt') if l.strip()]
land = next(tuple(map(float, r[1:3])) for r in items if r[0] == 'landing')
nests = [tuple(map(float, r[1:3])) for r in items if r[0] == 'mould']
g = np.array(Image.open(base + '.png'))[..., 1].astype(int)
edt = ndimage.distance_transform_edt((g & 8) == 0)
yy, xx = np.mgrid[0:g.shape[0], 0:g.shape[1]]
for q in [land] + nests:
    edt[(xx - q[0]) ** 2 + (yy - q[1]) ** 2 < 260 ** 2] = 0
y, x = np.unravel_index(np.argmax(edt), edt.shape)
mid = ((land[0] + nests[0][0]) / 2, (land[1] + nests[0][1]) / 2)
print(f"landing {land[0]/2:.0f},{land[1]/2:.0f}")
print(f"mid {mid[0]/2:.0f},{mid[1]/2:.0f}")
print(f"room {x/2:.0f},{y/2:.0f}")
