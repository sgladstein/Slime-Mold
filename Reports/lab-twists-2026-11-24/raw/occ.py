# Per-second body measures from a LAB_DUMP folder: occupied fine cells,
# centroid, and the body's reach (90th percentile distance from its centroid).
import sys, os, glob
import numpy as np
d = sys.argv[1]
secs = [int(x) for x in (sys.argv[2].split(',') if len(sys.argv) > 2 else [])]
hdr = open(os.path.join(d, 'agar.bin'), 'rb').read()
nl = hdr.index(b'\n'); w, h = map(int, hdr[:nl].split())
files = sorted(glob.glob(os.path.join(d, 'occ-*.bin')))
yy, xx = np.mgrid[0:h, 0:w]
for f in files:
    s = int(f[-8:-4])
    if secs and s not in secs: continue
    o = np.frombuffer(open(f, 'rb').read(), dtype=np.uint8).reshape(h, w)
    m = o > 0
    n = int(m.sum())
    if n == 0: print(s, 0); continue
    wt = o.astype(np.float64)
    cx = (xx * wt).sum() / wt.sum(); cy = (yy * wt).sum() / wt.sum()
    r = np.sqrt((xx[m] - cx) ** 2 + (yy[m] - cy) ** 2)
    print(f"{s} cells {n} walkers {int(o.sum())} centre {cx:.1f},{cy:.1f} reach90 {np.percentile(r, 90):.1f} xmin {xx[m].min()} xmax {xx[m].max()} ymin {yy[m].min()} ymax {yy[m].max()}")
