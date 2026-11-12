"""Speckle inside the body (second measure; the first, small pieces per
1,000 cells, counted stray walkers' dots off the network and did not track
the eye: dense scored only ~40% below base while its picture is plainly
smoother).

Body region M: the trail blurred with sigma 3 cells, brightness >= 0.3
(where a smooth body would be drawn); its core: M eroded by 2 cells.
For a drawing I (0-1):
 SC   speckle contrast in the core: rms of (I - I blurred sigma 4) over core
      cells, divided by the mean of I over the core. 0 is a smooth body.
 GAP  share of core cells where I is under half its 9x9 local max (after
      a sigma 1 blur): breaks and gaps between beads.
Shaded: an approximation of view.wgsl (height 4 th^0.65, slope over +-1.5
cells, light from the top left, gloss 0.08 + 0.7 sqrt th, specular ^28),
measured on luminance.
Clumping: Fano of 5x5 walker counts over core cells, divided by the same
with walker counts shuffled among core cells inside each 16x16 block (so
large-scale lumps vs streams are kept; only sub-16-cell bunching differs).
"""
import sys, glob, os, re, json
import numpy as np
from scipy import ndimage as ndi
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from specks import load, bright, box5, up2

RNG = np.random.default_rng(11)
LIGHT = np.array([-0.42, -0.58, 0.70]); LIGHT = LIGHT / np.linalg.norm(LIGHT)

def shade(th):
    h = 4.0 * np.power(np.clip(th, 0, 1), 0.65)
    hi = ndi.zoom(h, 1, order=1)
    # slope over +-1.5 cells: average of 1 and 2 cell differences
    gx = (np.roll(hi, -1, 1) - np.roll(hi, 1, 1)) / 2.0 * 0.5 + (np.roll(hi, -2, 1) - np.roll(hi, 2, 1)) / 4.0 * 0.5
    gy = (np.roll(hi, -1, 0) - np.roll(hi, 1, 0)) / 2.0 * 0.5 + (np.roll(hi, -2, 0) - np.roll(hi, 2, 0)) / 4.0 * 0.5
    n = np.stack([-gx, -gy, np.ones_like(gx)], -1)
    n /= np.linalg.norm(n, axis=-1, keepdims=True)
    s = 1.0 + 1.25 * (n @ LIGHT - LIGHT[2])
    half = LIGHT + np.array([0, 0, 1.0]); half /= np.linalg.norm(half)
    spec = np.power(np.clip(n @ half, 0, None), 28) * (0.08 + 0.7 * np.sqrt(np.clip(th, 0, 1)))
    soil = 57.0  # mid soil luminance
    alb = soil + (0.6 * 190 * (0.55 + 0.45 * th) - soil) * (1 - np.exp(-5 * th)) * 0.85
    lum = alb * np.maximum(s, 0.15) + 245 * spec * 0.7
    return lum / 255.0

def measures(I, core):
    if core.sum() < 30:
        return np.nan, np.nan
    m = I[core].mean()
    hp = I - ndi.gaussian_filter(I, 4)
    sc = np.sqrt((hp[core] ** 2).mean()) / max(m, 1e-9)
    loc = ndi.maximum_filter(ndi.gaussian_filter(I, 1), 9)
    gap = (I[core] < 0.5 * loc[core]).mean()
    return sc, gap

def clump(count, core):
    if core.sum() < 30:
        return np.nan
    win = box5(count)[core]
    f_obs = win.var() / max(win.mean(), 1e-9)
    sh = count.copy()
    h, w = count.shape
    for by in range(0, h, 16):
        for bx in range(0, w, 16):
            blk = core[by:by + 16, bx:bx + 16]
            if blk.sum() < 2:
                continue
            sub = sh[by:by + 16, bx:bx + 16]
            vals = sub[blk].copy(); RNG.shuffle(vals); sub[blk] = vals
    wn = box5(sh)[core]
    f_null = wn.var() / max(wn.mean(), 1e-9)
    return f_obs / f_null

def n0_of(tag, cells):
    start = {'dense': 25000}.get(tag, 5000)
    m = re.match(r'fix(\d+)', tag)
    if m: start = int(m.group(1)) * 1000
    return 6.0 * max((start / cells) / (5000 / (512 * 320)), 0.05)

def one(p):
    b = os.path.basename(p)
    m = re.search(r'big-(\w+)-s(\d+)-t0*(\d+)\.f32', b)
    if m: tag, seed, tick = m.group(1), int(m.group(2)), int(m.group(3))
    else:
        m = re.search(r'big-(\w+)-s(\d+)-0*(\d+)s\.f32', b)
        tag, seed, tick = m.group(1), int(m.group(2)), int(m.group(3)) * 120
    trail, count = load(p)
    coarse = trail.shape[1] == 256
    cells = trail.size
    n0 = n0_of(tag, cells)
    M = bright(ndi.gaussian_filter(trail, 3)) >= 0.3
    core = ndi.binary_erosion(M, iterations=2)
    # Rock, walls and the rim hold no trail at all (exactly 0): drop cells
    # within 3 of them, so terrain holes are not counted as specks.
    rock = ndi.binary_dilation(trail == 0.0, iterations=3)
    core &= ~rock
    # Thin tubes only: drop packed lumps (walkers blurred sigma 3 >= 0.3 a
    # cell, the crystals' discs), dilated by 3 cells.
    lump = ndi.binary_dilation(ndi.gaussian_filter(count, 3) >= 0.3, iterations=3)
    core &= ~lump
    thinM = M & ~lump & ~rock
    r = dict(tag=tag, seed=seed, sec=tick / 15.0, alive=int(count.sum()), dens=count.sum() / cells,
             core=int(core.sum()), clump=clump(count, core),
             core_occ=count[thinM].sum() / max(thinM.sum(), 1),
             lump_share=count[lump].sum() / max(count.sum(), 1))
    tb = bright(trail)
    nb = box5(count)
    game = tb * (1 - np.exp(-nb / n0))
    rho = ndi.gaussian_filter(count, 2.0)
    ref = np.median(rho[core]) if core.any() else 1.0
    dens = 1 - np.exp(-rho / max(ref, 1e-6) * np.log(2) / 0.7)
    draws = dict(trail=tb, game=game, blur=bright(ndi.gaussian_filter(trail, 1.5)), dens=dens,
                 game_sh=shade(game), trail_sh=shade(tb), blur_sh=shade(bright(ndi.gaussian_filter(trail, 1.5))))
    if coarse:
        core = ndi.zoom(core.astype(float), 2, order=0) > 0.5
        draws = {k: up2(v) for k, v in draws.items()}
    for k, I in draws.items():
        r['sc_' + k], r['gap_' + k] = measures(I, core)
    return r

if __name__ == '__main__':
    rows = []
    for d in sys.argv[1:]:
        for p in sorted(glob.glob(os.path.join(d, '*.f32'))):
            rows.append(one(p))
    keys = ['tag', 'seed', 'sec', 'alive', 'dens', 'lump_share', 'core_occ', 'clump', 'sc_trail', 'sc_game', 'sc_blur', 'sc_dens', 'sc_trail_sh', 'sc_game_sh', 'sc_blur_sh', 'gap_trail', 'gap_game', 'gap_blur']
    print(' '.join(f'{k[:9]:>9}' for k in keys))
    for r in rows:
        print(' '.join(f'{r[k]:>9.3f}' if isinstance(r[k], float) else f'{r[k]:>9}' for k in keys))
    json.dump(rows, open('speck2-' + '-'.join(os.path.basename(d.rstrip('/')) for d in sys.argv[1:]) + '.json', 'w'), default=float, indent=0)
