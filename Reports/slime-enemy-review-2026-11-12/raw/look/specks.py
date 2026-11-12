"""Speck census over bigdish raw fields (trail, living walkers a cell).

Deciding measure (named before the runs were read): DOTS = connected pieces
(8-connected) of the drawn image above a threshold whose area is 25 fine
cells or less (a bead at most 5 x 5), per 1,000 fine cells of drawn mould
(image > 0.1), averaged over thresholds 0.2, 0.35, 0.5. Lower is smoother.
A coarse (scale 1) world is upsampled 2x bilinearly first, as the view
interpolates, so every arm is measured in fine cells of world.

Sim-side measures on tube cells (trail brightness >= 0.5):
 occ   walkers a tube cell
 fano  variance / mean of living walkers in 5x5 windows centred on tube cells,
       divided by the same for the same walkers shuffled at random over the
       same tube cells (1 = no clumping beyond chance; >1 clumped).
 tcv   coefficient of variation of the trail over tube cells.
"""
import sys, glob, os, re
import numpy as np
from scipy import ndimage as ndi

KNEE = 6.0
RNG = np.random.default_rng(7)

def load(path):
    a = np.fromfile(path, dtype='<f4')
    n = a.size // 2
    w, h = (512, 320) if n == 512 * 320 else (256, 160)
    return a[:n].reshape(h, w), a[n:].reshape(h, w)

def bright(v):
    return 1.0 - np.exp(-np.maximum(v, 0) / KNEE)

def box5(a):
    return ndi.uniform_filter(a, size=5, mode='constant') * 25.0

def up2(a):
    return ndi.zoom(a, 2, order=1)

def dots(img):
    area = (img > 0.1).sum()
    if area == 0:
        return float('nan'), float('nan')
    ds, ps = [], []
    for t in (0.2, 0.35, 0.5):
        lab, n = ndi.label(img > t, structure=np.ones((3, 3)))
        if n == 0:
            ds.append(0); ps.append(0); continue
        sizes = np.bincount(lab.ravel())[1:]
        ds.append((sizes <= 25).sum())
        ps.append(n)
    return 1000 * np.mean(ds) / area, 1000 * np.mean(ps) / area

def drawings(trail, count, n0):
    d = {}
    d['trail'] = bright(trail)
    n = box5(count)
    d['game'] = bright(trail) * (1 - np.exp(-n / n0))
    d['trail_blur'] = bright(ndi.gaussian_filter(trail, 1.5))
    # Walker density, Gaussian sigma 2 cells, scaled by its tube median.
    rho = ndi.gaussian_filter(count, 2.0)
    tube = bright(trail) >= 0.5
    ref = np.median(rho[tube]) if tube.any() else 1.0
    d['density'] = 1 - np.exp(-rho / max(ref, 1e-6) * np.log(2) / 0.7)
    return d

def simside(trail, count):
    tube = bright(trail) >= 0.5
    if tube.sum() < 50:
        return dict(occ=np.nan, fano=np.nan, tcv=np.nan, tube=int(tube.sum()))
    occ = count[tube].sum() / tube.sum()
    win = box5(count)[tube]
    f_obs = win.var() / max(win.mean(), 1e-9)
    # Null: the walkers on tube cells shuffled over tube cells (keeps the
    # tube geometry and inhomogeneity of where tubes lie, not of who is where).
    sh = np.zeros_like(count)
    idx = np.flatnonzero(tube)
    vals = count.ravel()[idx].copy()
    RNG.shuffle(vals)
    sh.ravel()[idx] = vals
    # Walkers off the tube stay where they are.
    sh[~tube] = count[~tube]
    wn = box5(sh)[tube]
    f_null = wn.var() / max(wn.mean(), 1e-9)
    t = trail[tube]
    return dict(occ=occ, fano=f_obs / f_null, tcv=t.std() / t.mean(), tube=int(tube.sum()))

def main(dirs):
    rows = []
    for d in dirs:
        for p in sorted(glob.glob(os.path.join(d, '*.f32'))):
            b = os.path.basename(p)
            m = re.search(r'big-(\w+)-s(\d+)-t0*(\d+)\.f32', b)
            if m:
                tag, seed, tick = m.group(1), int(m.group(2)), int(m.group(3))
            else:
                m = re.search(r'big-(\w+)-s(\d+)-0*(\d+)s\.f32', b)
                tag, seed, tick = m.group(1), int(m.group(2)), int(m.group(3)) * 120
            trail, count = load(p)
            coarse = trail.shape[1] == 256
            cells = trail.size
            alive = count.sum()
            # The view's n0: BODY_N x (start living walkers a cell / 5000 on 512x320).
            start = {'dense': 25000}.get(tag, 5000)
            if tag.startswith('fix'):
                start = int(re.search(r'fix(\d+)', tag).group(1)) * 1000
            n0 = 6.0 * max((start / cells) / (5000 / (512 * 320)), 0.05)
            s = simside(trail, count)
            dr = drawings(trail, count, n0)
            r = dict(tag=tag, seed=seed, sec=tick / 15.0 if not coarse else tick / 15.0, alive=int(alive),
                     dens=alive / cells, **s)
            for k, img in dr.items():
                if coarse:
                    img = up2(img)
                r['dots_' + k], r['pieces_' + k] = dots(img)
            rows.append(r)
    return rows

if __name__ == '__main__':
    rows = main(sys.argv[1:])
    keys = ['tag', 'seed', 'sec', 'alive', 'dens', 'tube', 'occ', 'fano', 'tcv', 'dots_trail', 'dots_game', 'dots_trail_blur', 'dots_density']
    print(' '.join(f'{k:>10}' for k in keys))
    for r in rows:
        print(' '.join(f'{r[k]:>10.3f}' if isinstance(r[k], float) else f'{r[k]:>10}' for k in keys))
    import json
    json.dump(rows, open('specks.json', 'w'), default=float, indent=0)
