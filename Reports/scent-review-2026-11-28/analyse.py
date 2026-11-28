"""Reads one planet's field dumps (scratch SCENT_DUMP patch) and prints the
numbers the scent review quotes; writes field pictures.

usage: analyse.py DIR MS   (MS: the dump's game time in ms, e.g. 60000)
"""
import sys, os, glob, re, math
import numpy as np
from PIL import Image

d, ms = sys.argv[1], int(sys.argv[2])
meta = open(f"{d}/meta-{ms:06d}.txt").read()
w = int(re.search(r"w=(\d+)", meta).group(1)); h = int(re.search(r"h=(\d+)", meta).group(1))
done = int(re.search(r"done=(\d+)", meta).group(1))
sold = [tuple(map(float, p.split(","))) for p in re.search(r"soldiers=(.*)", meta).group(1).split() if p]
agar = np.fromfile(f"{d}/agar.u8", np.uint8).reshape(h, w).astype(bool)
scent = np.fromfile(f"{d}/scent-{ms:06d}.f32", np.float32)[: w * h].reshape(h, w)
trail = np.fromfile(f"{d}/trail-{ms:06d}.f32", np.float32)[: w * h].reshape(h, w)
alarm = np.fromfile(f"{d}/alarm-{ms:06d}.f32", np.float32)[: w * h].reshape(h, w)
occ = np.fromfile(f"{d}/occ-{ms:06d}.u32", np.uint32)[: w * h].reshape(h, w)
B = 2
bw, bh = (w + B - 1) // B, (h + B - 1) // B


def latest(kind):
    fs = sorted(glob.glob(f"{d}/{kind}-*.u32"))
    fs = [f for f in fs if int(re.search(r"-(\d+)\.u32", f).group(1)) <= done]
    if not fs:
        return None, None
    f = fs[-1]
    v = np.fromfile(f, np.uint32)[: bw * bh].reshape(bh, bw)
    return v, int(re.search(r"-(\d+)\.u32", f).group(1))


food, food_at = latest("food")
hunt, hunt_at = latest("hunt")
print(f"planet dir {d}, t {ms/1000:.0f} s, world {w} x {h} fine cells, engine tick {done}, soldiers {len(sold)}")
op = agar
body = occ > 0
print(f"open cells {op.sum()}, walkers' cells {body.sum()} ({body.sum()/op.sum():.1%} of open)")

# Scent bands over open cells.
s = scent[op].astype(np.float64)
bands = [(">=1", s >= 1), ("1e-3..1", (s >= 1e-3) & (s < 1)), ("1e-10..1e-3", (s >= 1e-10) & (s < 1e-3)),
         ("1e-30..1e-10", (s >= 1e-30) & (s < 1e-10)), ("0..1e-30", (s > 0) & (s < 1e-30)), ("== 0", s == 0)]
print("scent over open cells: " + ", ".join(f"{n} {m.mean():.1%}" for n, m in bands))
pos = s[s > 0]
print(f"scent max {s.max():.1f}, smallest above 0 {pos.min():.3e} (f32 normal floor 1.18e-38; subnormals below it: {(pos < 1.1754944e-38).sum()})")
vis = (1 - np.exp(-s / 60)) * 255 >= 1
print(f"cells the F2 view can show (byte >= 1, scent >= {-60*math.log(1-1/255):.2f}): {vis.mean():.1%} of open cells")
t_on = trail[body]
print(f"trail under walkers: median {np.median(t_on):.1f}, p90 {np.percentile(t_on,90):.1f}, max {trail.max():.1f}")
print(f"trail off the body: share of open cells off the body with trail > 1e-3: {((trail > 1e-3) & op & ~body).sum()/max(1,(op & ~body).sum()):.1%}")
print(f"alarm max {alarm.max():.2f}, cells over 0.1: {(alarm > 0.1).sum()}")

# Log-scent against the far smell's walking distance (fine cells).
if food is not None:
    sb = scent[: bh * B, : bw * B].reshape(bh, B, bw, B).max(axis=(1, 3)).astype(np.float64)
    ob = agar[: bh * B, : bw * B].reshape(bh, B, bw, B).all(axis=(1, 3))
    ok = ob & (food != 0xFFFFFFFF) & (sb > 0)
    x = food[ok].astype(np.float64); y = -np.log(sb[ok])
    rx = np.argsort(np.argsort(x)); ry = np.argsort(np.argsort(y))
    rho = np.corrcoef(rx, ry)[0, 1]
    A = np.vstack([x, np.ones_like(x)]).T
    slope, icpt = np.linalg.lstsq(A, y, rcond=None)[0]
    print(f"far smell field (tick {food_at}): blocks compared {ok.sum()}, Spearman(-ln scent, walking distance) {rho:.3f}; fit -ln s = {slope:.3f} d + {icpt:.1f} (theory 1/L = {1/3.31:.3f} per fine cell)")
    # Gradient directions over blocks: steepest descent of the far smell vs steepest ascent of ln scent.
    f = food.astype(np.float64); f[food == 0xFFFFFFFF] = np.nan
    ls = np.where(sb > 0, np.log(np.maximum(sb, 1e-300)), np.nan)
    def grad(a):
        gx = (np.roll(a, -1, 1) - np.roll(a, 1, 1)) / 2; gy = (np.roll(a, -1, 0) - np.roll(a, 1, 0)) / 2
        return gx, gy
    fx, fy = grad(f); lx, ly = grad(ls)
    dot = (-fx) * lx + (-fy) * ly
    n1 = np.hypot(fx, fy); n2 = np.hypot(lx, ly)
    cos = dot / (n1 * n2)
    good = np.isfinite(cos) & (n1 > 0) & (n2 > 0)
    # only blocks off the body (no walkers): where the far smell matters
    occb = occ[: bh * B, : bw * B].reshape(bh, B, bw, B).sum(axis=(1, 3))
    off = good & (occb == 0)
    for nm, m in [("all open blocks", good), ("off the body", off)]:
        c = cos[m]
        print(f"  {nm}: {m.sum()} blocks, ln-scent uphill within 45 deg of the far smell's downhill on {np.mean(c > math.cos(math.pi/4)):.1%}, within 90 deg on {np.mean(c > 0):.1%}")
    by = [(lo, hi) for lo, hi in [(0, 25), (25, 50), (50, 100), (100, 200), (200, 400), (400, 10**6)]]
    for lo, hi in by:
        m = off & (f >= lo) & (f < hi)
        if m.sum() > 20:
            print(f"    walking distance {lo}-{hi} fine cells: {m.sum()} blocks, within 45 deg {np.mean(cos[m] > math.cos(math.pi/4)):.1%}, median scent {np.median(sb[m]):.2e}")

if hunt is not None:
    hv = hunt != 0xFFFFFFFF
    print(f"hunt field (tick {hunt_at}): blocks reached {hv.sum()} ({hv.sum()/(bw*bh):.1%} of the map), nearest {hunt[hv].min() if hv.any() else '-'} to farthest {hunt[hv].max() if hv.any() else '-'} fine cells")

# Pictures, one pixel a fine cell.
os.makedirs(f"{d}/pics", exist_ok=True)
bg = np.where(agar, 18, 52).astype(np.uint8)
def save(name, rgb):
    Image.fromarray(rgb).save(f"{d}/pics/{name}-{ms//1000:03d}s.png")
# 1. What F2 shows: the game's byte.
byte = np.clip((1 - np.exp(-scent.astype(np.float64) / 60)), 0, 1)
rgb = np.stack([bg, bg, bg], -1).astype(np.float64)
rgb = rgb * (1 - byte[..., None] * 0.8) + np.array([70, 170, 255]) * byte[..., None] * 0.8
save("scent-seen", rgb.clip(0, 255).astype(np.uint8))
# 2. What the walkers steer by: log10 scent, banded every decade.
lg = np.log10(np.maximum(scent.astype(np.float64), 1e-45))
v = np.clip((lg + 38) / (3 + 38), 0, 1)
band = (np.floor(lg) % 2 == 0) & (scent > 0)
rgb = np.stack([bg, bg, bg], -1).astype(np.float64)
col = np.stack([40 + 60 * v, 70 + 140 * v, 120 + 135 * v], -1)
rgb = np.where((scent > 0)[..., None] & agar[..., None], col * np.where(band, 1.0, 0.78)[..., None], rgb)
rgb[body] = [255, 150, 40]
for (sx, sy) in sold:
    xi, yi = int(sx), int(sy)
    rgb[max(0, yi - 1):yi + 2, max(0, xi - 1):xi + 2] = [255, 255, 255]
save("scent-log", rgb.clip(0, 255).astype(np.uint8))
# 3. The hunt's trail (walking distance + age), if any.
if hunt is not None:
    hv = hunt != 0xFFFFFFFF
    hf = np.kron(np.where(hv, hunt, 0), np.ones((B, B), np.uint32))[:h, :w].astype(np.float64)
    hvf = np.kron(hv, np.ones((B, B), bool))[:h, :w]
    mx = max(1.0, hf[hvf].max()) if hvf.any() else 1.0
    k = np.clip(1 - hf / mx, 0, 1)
    rgb = np.stack([bg, bg, bg], -1).astype(np.float64)
    rgb = np.where(hvf[..., None], rgb * (1 - k[..., None]) + np.array([150, 255, 200]) * k[..., None], rgb)
    rgb[body] = rgb[body] * 0.4 + np.array([255, 150, 40]) * 0.6
    for (sx, sy) in sold:
        xi, yi = int(sx), int(sy)
        rgb[max(0, yi - 1):yi + 2, max(0, xi - 1):xi + 2] = [255, 255, 255]
    save("hunt", rgb.clip(0, 255).astype(np.uint8))
# 4. The far smell's walking distance, contoured every 20 fine cells.
if food is not None:
    ff = np.kron(np.where(food != 0xFFFFFFFF, food, 0), np.ones((B, B), np.uint32))[:h, :w].astype(np.float64)
    fv = np.kron(food != 0xFFFFFFFF, np.ones((B, B), bool))[:h, :w]
    ring = (np.floor(ff / 20) % 2 == 0)
    rgb = np.stack([bg, bg, bg], -1).astype(np.float64)
    rgb = np.where(fv[..., None], np.where(ring[..., None], [60, 60, 110], [35, 35, 70]), rgb)
    rgb[body] = [255, 150, 40]
    save("far", rgb.clip(0, 255).astype(np.uint8))
print("pictures in", f"{d}/pics")
