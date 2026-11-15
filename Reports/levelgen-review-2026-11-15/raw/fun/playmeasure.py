"""Play measures of generated planets, from the map files `planets out=DIR` writes.

Run with PYTHONPATH=/home/user/review-fun/pylib python3 -I is not used (scipy is
in pylib). Usage: python3 playmeasure.py MAPDIR OUT.tsv [names...]

Everything is on 4-map-cell bins (one game "dish cell"), like levelgen::walkable.
Soldier speed is 10 dish cells a game second, gun range 24 dish cells, a wall
costs 20 food a dish cell (rules.rs), a soldier 400.

Per planet and per nest:
- walk: walking distance landing -> nest rim (dish cells), walls broken (soft)
  and walls kept (hard); straight line; detour = walk / line.
- widest: the widest route's narrowest point, as its width in dish cells
  (2 x clearance), walls kept. Under ~6 there is no open approach: every
  route funnels through a choke.
- cut: the least separator between the home (bins within 30% of the walk to
  the nearest nest) and all nests' surroundings (within 30% of their walk),
  in dish cells, walls kept as blocked (they are already a wall). The wall a
  turtle needs to hold the middle of the map. Divided by the map's short
  side, so 1.0 is a straight line across an open map.
- sight: mean free ray length from open bins, capped at 2 x range (48), over
  16 directions; rock and soft rock block, water does not.
- charge thirds: share of crystal energy (r^2 x charge) by walk position
  between the landing (0) and the nearest nest (1): home <1/3, middle, far
  >2/3, the nests' own home crystals excluded.
"""
import sys, os, math
import numpy as np
from PIL import Image
from scipy import ndimage
from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import maximum_flow, dijkstra

BIN = 4
G_ROCK, G_WET = 0x08, 0x01


def load(d, name):
    a = np.array(Image.open(os.path.join(d, name + '.png')))
    g = a[..., 1]
    b = a[..., 2]
    items = {'crystal': [], 'mould': [], 'wall': [], 'landing': None}
    for line in open(os.path.join(d, name + '.txt')):
        p = line.split()
        if not p:
            continue
        if p[0] == 'crystal':
            items['crystal'].append(tuple(map(float, p[1:5])))
        elif p[0] == 'mould':
            items['mould'].append(tuple(map(float, p[1:3])))
        elif p[0] == 'wall':
            items['wall'].append(tuple(map(float, p[1:7])))
        elif p[0] == 'landing':
            items['landing'] = (float(p[1]), float(p[2]))
    return g, b, items


def disc_mask(shape, c, r):
    bh, bw = shape
    yy, xx = np.mgrid[0:bh, 0:bw]
    return (xx * BIN + 2 - c[0]) ** 2 + (yy * BIN + 2 - c[1]) ** 2 <= r * r


def grids(g, items):
    h, w = g.shape
    bh, bw = h // BIN, w // BIN
    gg = g[:bh * BIN, :bw * BIN].reshape(bh, BIN, bw, BIN)
    stand = ((gg & G_ROCK) == 0) & ((gg >> 4) != 4)
    open_ = stand.all(axis=(1, 3))
    rock = ((gg & G_ROCK) != 0).any(axis=(1, 3))
    cryst = np.zeros((bh, bw), bool)
    yy, xx = np.mgrid[0:bh, 0:bw]
    px, py = xx * BIN + 2, yy * BIN + 2
    for (x, y, r, c) in items['crystal']:
        cryst |= (px - x) ** 2 + (py - y) ** 2 <= r * r
    walls = np.zeros((bh, bw), bool)
    for (x0, y0, x1, y1, t, hp) in items['wall']:
        l = math.hypot(x1 - x0, y1 - y0)
        k = max(1, math.ceil(l / 2))
        rr = t / 2 + 1
        for i in range(k + 1):
            u = i / k
            cx, cy = x0 + (x1 - x0) * u, y0 + (y1 - y0) * u
            bx0, bx1 = max(0, int((cx - rr) // BIN)), min(bw - 1, int((cx + rr) // BIN))
            by0, by1 = max(0, int((cy - rr) // BIN)), min(bh - 1, int((cy + rr) // BIN))
            sub = (px[by0:by1 + 1, bx0:bx1 + 1] - cx) ** 2 + (py[by0:by1 + 1, bx0:bx1 + 1] - cy) ** 2 <= rr * rr
            walls[by0:by1 + 1, bx0:bx1 + 1] |= sub
    soft = open_ & ~cryst          # walls broken
    hard = soft & ~walls           # walls kept
    return soft, hard, rock, walls, cryst


def graph(open_):
    """8-neighbour graph, 1 a side step and 1.4 a diagonal, no corner cutting."""
    bh, bw = open_.shape
    idx = -np.ones((bh, bw), int)
    idx[open_] = np.arange(open_.sum())
    rows, cols, vals = [], [], []
    for dy, dx, c in [(0, 1, 1.0), (1, 0, 1.0), (1, 1, 1.4142), (1, -1, 1.4142)]:
        a = open_[max(0, -dy):bh - max(0, dy), max(0, -dx):bw - max(0, dx)]
        bsl = open_[max(0, dy):bh - max(0, -dy) if dy < 0 else bh, max(0, dx):bw - max(0, -dx) if dx < 0 else bw]
        # explicit slices
        ys = slice(0, bh - dy)
        yd = slice(dy, bh)
        if dx >= 0:
            xs, xd = slice(0, bw - dx), slice(dx, bw)
        else:
            xs, xd = slice(-dx, bw), slice(0, bw + dx)
        m = open_[ys, xs] & open_[yd, xd]
        if dx != 0 and dy != 0:
            # no corner cutting
            m &= open_[yd, xs] & open_[ys, xd]
        u = idx[ys, xs][m]
        v = idx[yd, xd][m]
        rows += [u, v]
        cols += [v, u]
        vals += [np.full(len(u), c), np.full(len(u), c)]
    n = int(open_.sum())
    G = csr_matrix((np.concatenate(vals), (np.concatenate(rows), np.concatenate(cols))), shape=(n, n))
    return G, idx


def nearest_open(open_, p):
    bh, bw = open_.shape
    fx, fy = int(p[0] // BIN), int(p[1] // BIN)
    for r in range(0, 40):
        ys = range(max(0, fy - r), min(bh, fy + r + 1))
        xs = range(max(0, fx - r), min(bw, fx + r + 1))
        best = None
        for y in ys:
            for x in xs:
                if open_[y, x]:
                    d = (x - fx) ** 2 + (y - fy) ** 2
                    if best is None or d < best[0]:
                        best = (d, y, x)
        if best:
            return best[1], best[2]
    return None


def rim_bins(open_, c, rad):
    m = disc_mask(open_.shape, c, rad + 2 * BIN + 2) & open_
    return np.argwhere(m)


def walk_dist(open_, G, idx, p):
    s = nearest_open(open_, p)
    if s is None:
        return None
    d = dijkstra(G, indices=idx[s], directed=False)
    full = np.full(open_.shape, np.inf)
    full[open_] = d
    return full


def rim_dist(full, open_, c, rad):
    rb = rim_bins(open_, c, rad)
    if len(rb) == 0:
        return np.inf
    return full[rb[:, 0], rb[:, 1]].min()


def widest(open_, src, dst_bins):
    """Max over routes of the min clearance, as a width in bins (2 x edt)."""
    edt = ndimage.distance_transform_edt(open_)
    lo, hi = 0.0, float(edt.max())
    sy, sx = src
    best = 0.0
    levels = np.unique(np.round(edt[open_] * 2) / 2)
    levels = levels[levels > 0]
    # binary search over levels
    a, b = 0, len(levels) - 1
    while a <= b:
        mid = (a + b) // 2
        t = levels[mid]
        lab, _ = ndimage.label(edt >= t, structure=np.ones((3, 3)))
        ls = lab[sy, sx]
        ok = ls != 0 and (lab[dst_bins[:, 0], dst_bins[:, 1]] == ls).any()
        if ok:
            best = t
            a = mid + 1
        else:
            b = mid - 1
    return 2 * best


def min_cut(open_, home, away):
    """Least number of open bins whose removal separates home from away
    (8-connected moves, so the separator is 4-connected... we use 4-neighbour
    moves so an 8-connected wall of bins seals)."""
    bh, bw = open_.shape
    node = open_ & ~home & ~away
    n_in = -np.ones((bh, bw), int)
    k = int(node.sum())
    n_in[node] = np.arange(k)
    S, T = 2 * k, 2 * k + 1
    rows, cols, caps = [], [], []
    ids = n_in[node]
    rows.append(ids); cols.append(ids + k); caps.append(np.ones(k, np.int32))
    BIG = 10 ** 6
    for dy, dx in [(0, 1), (1, 0)]:
        ys, yd = slice(0, bh - dy), slice(dy, bh)
        xs, xd = slice(0, bw - dx), slice(dx, bw)
        A = (slice(None),)
        for (s1, s2) in [((ys, xs), (yd, xd)), ((yd, xd), (ys, xs))]:
            na, nb = node[s1], node[s2]
            ha, hb = home[s1] & open_[s1], home[s2] & open_[s2]
            wa, wb = away[s1] & open_[s1], away[s2] & open_[s2]
            ia, ib = n_in[s1], n_in[s2]
            m = na & nb
            rows.append(ia[m] + k); cols.append(ib[m]); caps.append(np.full(m.sum(), BIG, np.int32))
            m = ha & nb
            rows.append(np.full(m.sum(), S)); cols.append(ib[m]); caps.append(np.full(m.sum(), BIG, np.int32))
            m = na & wb
            rows.append(ia[m] + k); cols.append(np.full(m.sum(), T)); caps.append(np.full(m.sum(), BIG, np.int32))
            m = ha & wb
            if m.sum():
                return 0
    r = np.concatenate(rows); c = np.concatenate(cols); cap = np.concatenate(caps)
    M = csr_matrix((cap, (r, c)), shape=(2 * k + 2, 2 * k + 2))
    M.sum_duplicates()
    return int(maximum_flow(M, S, T).flow_value)


def sight(open_, blocked, cap=48, samples=600, rng=None):
    rng = rng or np.random.default_rng(1)
    pts = np.argwhere(open_)
    if len(pts) == 0:
        return 0
    pick = pts[rng.integers(0, len(pts), samples)]
    bh, bw = open_.shape
    tot = 0.0
    dirs = [(math.cos(2 * math.pi * k / 16), math.sin(2 * math.pi * k / 16)) for k in range(16)]
    for (y, x) in pick:
        for (dx, dy) in dirs:
            L = cap
            for s in range(1, cap + 1):
                xx, yy = int(x + 0.5 + dx * s), int(y + 0.5 + dy * s)
                if xx < 0 or yy < 0 or xx >= bw or yy >= bh or blocked[yy, xx]:
                    L = s - 1
                    break
            tot += L
    return tot / (len(pick) * 16)


def measure(d, name):
    g, b, it = load(d, name)
    h, w = g.shape
    soft, hard, rock, walls, cryst = grids(g, it)
    bh, bw = soft.shape
    land = it['landing']
    nests = it['mould']
    Gs, idxs = graph(soft)
    Gh, idxh = graph(hard)
    dsoft = walk_dist(soft, Gs, idxs, land)
    dhard = walk_dist(hard, Gh, idxh, land)
    out = {'name': name, 'w': w, 'h': h}
    nest_rows = []
    for k, q in enumerate(nests):
        ws = rim_dist(dsoft, soft, q, 40)
        wh = rim_dist(dhard, hard, q, 40)
        line = math.hypot(q[0] - land[0], q[1] - land[1]) / BIN
        src = nearest_open(hard, land)
        # within gun range (24 dish cells) of the nest's home crystal rim
        nr = next((c[2] for c in it['crystal'] if math.hypot(c[0]-q[0], c[1]-q[1]) < 2), 40)
        dst = np.argwhere(disc_mask(hard.shape, q, nr + 96) & hard)
        wd = widest(hard, src, dst) if np.isfinite(wh) and len(dst) else 0.0
        nest_rows.append({'walk': ws, 'walk_hard': wh, 'line': line, 'detour': ws / max(line, 1), 'widest': wd})
    out['nests'] = nest_rows
    near = min(r['walk'] for r in nest_rows)
    out['walk_near'] = near
    out['walk_far'] = max(r['walk'] for r in nest_rows)
    out['secs_far'] = out['walk_far'] / 10.0
    # home and away regions, walls kept; cut on the hard grid
    home = (dsoft <= 0.3 * near) & hard
    away = np.zeros_like(home)
    for q in nests:
        dq = walk_dist(soft, Gs, idxs, q)
        away |= (dq <= 0.3 * near) & hard
    cut = min_cut(hard, home, away)
    out['cut'] = cut
    out['cut_rel'] = cut / min(bh, bw)
    # crystals by position on the route
    tot = 0.0
    thirds = [0.0, 0.0, 0.0]
    first = None
    cr = []
    for (x, y, r, c) in it['crystal']:
        e = r * r * c
        is_home = any(math.hypot(x - q[0], y - q[1]) < 2 for q in nests)
        dl = rim_dist(dsoft, soft, (x, y), r)
        if is_home:
            continue
        if not np.isfinite(dl):
            continue
        dn = min(math.hypot(x - q[0], y - q[1]) for q in nests) / BIN
        t = dl / max(dl + dn, 1)
        cr.append((dl, e))
        tot += e
        thirds[min(2, int(t * 3))] += e
    cr.sort()
    out['crystals'] = len(it['crystal'])
    out['home_share'] = thirds[0] / max(tot, 1)
    out['mid_share'] = thirds[1] / max(tot, 1)
    out['far_share'] = thirds[2] / max(tot, 1)
    out['first_crystal'] = cr[0][0] if cr else np.inf
    out['second_crystal'] = cr[1][0] if len(cr) > 1 else np.inf
    # biggest single non-home crystal share
    out['top_share'] = max((e for _, e in cr), default=0) / max(tot, 1)
    out['sight'] = sight(hard, rock | walls)
    out['rock'] = rock.mean()
    gw = g & G_WET
    out['wet'] = (gw != 0).mean()
    out['deep'] = (g == 0x41).mean()
    out['sand'] = ((g >> 4) == 1).mean()
    out['clay'] = ((g >> 4) == 2).mean()
    out['barren'] = ((g >> 4) == 3).mean()
    out['plants'] = ((b & 63) > 0).mean()
    out['walls'] = len(it['wall'])
    out['wall_bins_per_1x'] = walls.sum() / (bw * bh) * 40960
    # open floor with room to build: bins with clearance >= 5 (a turret ring)
    edt = ndimage.distance_transform_edt(hard)
    out['roomy'] = (edt >= 6).sum() / max(hard.sum(), 1)
    return out


if __name__ == '__main__':
    d, outp = sys.argv[1], sys.argv[2]
    names = sys.argv[3:] or sorted(n[:-4] for n in os.listdir(d) if n.endswith('.png'))
    keys = ['name', 'w', 'h', 'crystals', 'walk_near', 'walk_far', 'secs_far', 'cut', 'cut_rel', 'home_share', 'mid_share', 'far_share', 'top_share', 'first_crystal', 'second_crystal', 'sight', 'roomy', 'rock', 'wet', 'deep', 'sand', 'clay', 'barren', 'plants', 'walls', 'wall_bins_per_1x']
    with open(outp, 'w') as f:
        f.write('\t'.join(keys) + '\tnests(walk/hardwalk/detour/widest)\n')
        for n in names:
            m = measure(d, n)
            row = []
            for k in keys:
                v = m[k]
                row.append(f'{v:.3f}' if isinstance(v, float) else str(v))
            ns = ' '.join(f"{r['walk']:.0f}/{r['walk_hard']:.0f}/{r['detour']:.2f}/{r['widest']:.1f}" for r in m['nests'])
            f.write('\t'.join(row) + '\t' + ns + '\n')
            f.flush()
            print(n, 'done', flush=True)
