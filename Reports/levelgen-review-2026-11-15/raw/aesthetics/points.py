# Pick close-view centres (world units = map cells / 2) per planet map.
import sys, json, glob
import numpy as np
from PIL import Image
out = {}
for f in sorted(glob.glob(sys.argv[1] + '/planet-*.png')):
    name = f.split('/')[-1][:-4]
    im = np.array(Image.open(f).convert('RGBA'))
    g = im[:, :, 1].astype(int); b = im[:, :, 2].astype(int)
    H, W = g.shape
    items = [l.split() for l in open(f[:-4] + '.txt') if l.strip()]
    crys = [(float(p[1]), float(p[2]), float(p[3])) for p in items if p[0] == 'crystal']
    mould = [(float(p[1]), float(p[2])) for p in items if p[0] == 'mould']
    land = [(float(p[1]), float(p[2])) for p in items if p[0] == 'landing'][0]
    walls = [tuple(map(float, p[1:5])) for p in items if p[0] == 'wall']
    rock = (g & 8) > 0; wet = (g & 1) > 0; deep = (g >> 4) == 4; bridge = (g >> 4) == 5
    kind = np.where(rock, 8, np.where(deep, 4, np.where(wet, 9, g >> 4)))
    dens = (b & 63) / 63.0
    plant = dens > 0.25
    # window of zoom 12 at 1280x800 -> 213 x 133 map cells; keep HUD margins
    hw, hh = 110, 70
    def clear(cx, cy, pad=1.0):
        for x, y, r in crys:
            if abs(x - cx) < hw + r and abs(y - cy) < hh + r: return False
        for x, y in mould + [land]:
            if abs(x - cx) < hw + 40 and abs(y - cy) < hh + 40: return False
        return True
    # whole map class histogram
    classes = [0, 1, 2, 3, 4, 8, 9]
    def hist(sub, p):
        hst = [np.mean(sub == c) for c in classes] + [np.mean(p)]
        return np.array(hst)
    whole = hist(kind, plant)
    best = None
    for cy in range(hh, H - hh, 16):
        for cx in range(hw, W - hw, 16):
            if not clear(cx, cy): continue
            sub = kind[cy-hh:cy+hh, cx-hw:cx+hw]; p = plant[cy-hh:cy+hh, cx-hw:cx+hw]
            d = np.abs(hist(sub, p) - whole).sum()
            nd = min(((x - cx) ** 2 + (y - cy) ** 2) ** 0.5 for x, y in mould)
            d += 2.0 * max(0.0, 0.5 * W - nd) / W
            if best is None or d < best[0]: best = (d, cx, cy)
    pts = {}
    if best: pts['ident'] = (best[1] / 2, best[2] / 2)
    def edge_point(mask, avoid=None):
        # cell on mask boundary nearest the map middle-ish but away from crystals
        m = mask
        e = m & ~(np.roll(m, 1, 0) & np.roll(m, -1, 0) & np.roll(m, 1, 1) & np.roll(m, -1, 1))
        e[:40, :] = False; e[-40:, :] = False; e[:, :60] = False; e[:, -60:] = False
        ys, xs = np.nonzero(e)
        if len(xs) == 0: return None
        # avoid crystals/mould within 40 cells
        ok = np.ones(len(xs), bool)
        for x, y, r in crys:
            ok &= (xs - x) ** 2 + (ys - y) ** 2 > (r + 40) ** 2
        for x, y in mould + [land]:
            ok &= (xs - x) ** 2 + (ys - y) ** 2 > 60 ** 2
        if avoid is not None:
            ok &= ~avoid[ys, xs]
        xs, ys = xs[ok], ys[ok]
        if len(xs) == 0: return None
        rng = np.random.default_rng(len(xs))
        i = rng.integers(len(xs))
        return (float(xs[i]) / 2, float(ys[i]) / 2)
    pts['shore'] = edge_point(wet | deep)
    pts['deep'] = edge_point(deep)
    pts['rock'] = edge_point(rock)
    pts['plant'] = edge_point(plant & ~wet & ~rock, avoid=None)
    pts['bridge'] = edge_point(bridge)
    pts['nest'] = (mould[0][0] / 2, mould[0][1] / 2)
    pts['landing'] = (land[0] / 2, land[1] / 2)
    if walls:
        x0, y0, x1, y1 = walls[len(walls) // 2]
        pts['hedge'] = ((x0 + x1) / 4, (y0 + y1) / 4)
    pts['world'] = (W / 2, H / 2)
    out[name] = {k: (None if v is None else [round(v[0], 1), round(v[1], 1)]) for k, v in pts.items()}
json.dump(out, open(sys.argv[2], 'w'), indent=1)
print(json.dumps(out, indent=0))
