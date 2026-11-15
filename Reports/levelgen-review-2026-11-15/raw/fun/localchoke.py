"""Direct way vs way round: for each nest, the widest route whose walk is at
most 15% longer than the shortest (the direct way's narrowest point), next to
the widest route of any length. Direct narrow and a wide way round is the
'both' the owner asked for: a kill box on the direct way, an open approach
round it. Walls (soft rock) kept as blocked; widths in dish cells (2 x clearance)."""
import sys, os, math
import numpy as np
from scipy import ndimage
from scipy.sparse.csgraph import dijkstra
sys.path.insert(0, '/home/user/review-fun/tools')
import playmeasure as pm
d = '/home/user/review-fun/maps'
out = open('/home/user/review-fun/maps/localchoke.tsv', 'w')
out.write('name\tnest\tdirect_width\twidest\tshortest\twidest_len\n')

def best_len(hard, edt, t, s, dst):
    ok = hard & (edt >= t)
    if not ok[s]:
        return np.inf
    G, idx = pm.graph(ok)
    dist = dijkstra(G, indices=idx[s], directed=False)
    sel = ok[dst[:, 0], dst[:, 1]]
    if not sel.any():
        return np.inf
    return dist[idx[dst[sel, 0], dst[sel, 1]]].min()

for n in sorted(x[:-4] for x in os.listdir(d) if x.endswith('.png')):
    g, b, it = pm.load(d, n)
    soft, hard, rock, walls, cryst = pm.grids(g, it)
    edt = ndimage.distance_transform_edt(hard)
    s = pm.nearest_open(hard, it['landing'])
    levels = np.unique(np.round(edt[hard] * 2) / 2)
    levels = levels[levels > 0]
    for k, q in enumerate(it['mould']):
        nr = next((c[2] for c in it['crystal'] if math.hypot(c[0] - q[0], c[1] - q[1]) < 2), 40)
        dst = np.argwhere(pm.disc_mask(hard.shape, q, nr + 96) & hard)
        L0 = best_len(hard, edt, 0.5, s, dst)
        if not np.isfinite(L0):
            out.write(f'{n}\t{k}\t0\t0\tinf\tinf\n'); continue
        # widest within 15% of the shortest, and widest at all
        def search(limit):
            a, bb, best = 0, len(levels) - 1, 0.0
            while a <= bb:
                mid = (a + bb) // 2
                L = best_len(hard, edt, levels[mid], s, dst)
                if L <= limit:
                    best = levels[mid]; a = mid + 1
                else:
                    bb = mid - 1
            return best
        direct = search(1.15 * L0)
        wide = search(np.inf)
        Lw = best_len(hard, edt, wide, s, dst)
        out.write(f'{n}\t{k}\t{2*direct:.0f}\t{2*wide:.0f}\t{L0:.0f}\t{Lw:.0f}\n'); out.flush()
