"""Pilot, not a test: a Tero-style adaptive flow lattice that grows by eating.

One seed, guessed numbers. Question: does a flux-fed tube lattice give
(a) trunks between food with a finer mesh at the hungry front,
(b) a cut piece that visibly thins and starves while the fed side lives,
(c) a front slower than the transport, without any global solve
    (elastic tubes: pressure relaxes locally, explicit, GPU-shaped)?

Lattice W x H nodes (one node = 4 x 4 fine cells of a 512 x 320 race world).
Node volume V (elastic store), pressure p = V / C. Edge conductance D.
Per tick: V moves along edges by D (p_i - p_j) (conserved exactly);
D grows with |flux| (sigmoid, exponent gam) and decays at rate r;
food nodes inject (pulsing, random phase: shuttle flow between foods);
every live node pays upkeep; front nodes with spare V build a tube
into an empty neighbour (biased by food scent); nodes starve at V = 0.
"""
import sys, time
import numpy as np

W, H = 128, 80
rng = np.random.default_rng(int(sys.argv[1]) if len(sys.argv) > 1 else 1)
out = sys.argv[2] if len(sys.argv) > 2 else "."
TPS = 15                 # ticks a game second (the race world's clock)
SUB = 4                  # pressure substeps a tick
C = 1.0                  # node capacitance
DMAX = 0.2               # cap so SUB explicit substeps stay stable (4*DMAX/C*dt_sub < 1)
D0, DMIN = 0.02, 0.004   # new tube, death threshold
GAM, QS, A, R = 1.3, 0.02, 0.35, 0.25 / TPS   # adaptation; R: tube life without flow ~ ln(D0/DMIN)/0.25 = 6 s
KAPPA = 0.01             # the sheet: slow seep between any two live neighbours
UPKEEP = 0.0001          # volume a node a tick
GROW_V, GROW_COST = 0.6, 0.5
STARVE_TICKS = 6 * TPS   # empty that long -> node dies
FOOD_I = 0.08            # injection a tick (mean), pulsing
PULSE_T = 8 * TPS
CUT_AT, END = 150 * TPS, 200 * TPS

alive = np.zeros((H, W), bool)
V = np.zeros((H, W))
starve = np.zeros((H, W), int)
Dh = np.zeros((H, W - 1))
Dv = np.zeros((H - 1, W))
# food: home plus scattered crystals
foods = [(20, 40)] + [(int(x), int(y)) for x, y in zip(rng.integers(8, W - 8, 9), rng.integers(8, H - 8, 9))]
charge = {f: 400.0 for f in foods}
phase = {f: rng.uniform(0, 2 * np.pi) for f in foods}
yy, xx = np.mgrid[0:H, 0:W]
scent = np.zeros((H, W))
for (fx, fy) in foods[1:]:
    scent += np.exp(-np.hypot(xx - fx, yy - fy) / 12.0)
hx, hy = foods[0]
alive[hy - 2:hy + 3, hx - 2:hx + 3] = True
V[alive] = 1.0
for y in range(hy - 2, hy + 3):
    for x in range(hx - 2, hx + 2):
        Dh[y, x] = D0 * 3
for y in range(hy - 2, hy + 2):
    for x in range(hx - 2, hx + 3):
        Dv[y, x] = D0 * 3


def sig(q):
    a = np.abs(q) ** GAM
    return a / (QS ** GAM + a)


def loops(th):
    """Cyclomatic number E - V + C of tubes with D > th, and largest part share of live nodes."""
    par = np.arange(W * H)

    def find(a):
        while par[a] != a:
            par[a] = par[par[a]]
            a = par[a]
        return a
    E = 0
    for (D, dx, dy) in ((Dh, 1, 0), (Dv, 0, 1)):
        ys, xs = np.nonzero(D > th)
        for y, x in zip(ys, xs):
            a, b = find(y * W + x), find((y + dy) * W + x + dx)
            E += 1
            if a != b:
                par[a] = b
    nodes = np.nonzero(alive.ravel())[0]
    roots = {}
    for n in nodes:
        r_ = find(n)
        roots[r_] = roots.get(r_, 0) + 1
    comps = len(roots)
    big = max(roots.values()) / max(len(nodes), 1) if roots else 0
    return E - len(nodes) + comps, big


def draw(path):
    from PIL import Image, ImageDraw
    S = 6
    im = Image.new("RGB", (W * S, H * S), (12, 10, 8))
    d = ImageDraw.Draw(im)
    ys, xs = np.nonzero(alive)
    for y, x in zip(ys, xs):
        g = int(25 + 60 * min(V[y, x], 1.0))
        d.rectangle([x * S, y * S, x * S + S - 1, y * S + S - 1], fill=(g, g // 2, 0))
    for (D, dx, dy) in ((Dh, 1, 0), (Dv, 0, 1)):
        ys, xs = np.nonzero(D > 0)
        for y, x in zip(ys, xs):
            w = (D[y, x] / DMAX) ** 0.5
            col = (int(120 + 135 * w), int(110 + 120 * w), int(30 + 40 * w))
            d.line([x * S + S // 2, y * S + S // 2, (x + dx) * S + S // 2, (y + dy) * S + S // 2],
                   fill=col, width=max(1, int(1 + 4 * w)))
    for f in foods:
        d.ellipse([f[0] * S - 4, f[1] * S - 4, f[0] * S + 4, f[1] * S + 4], outline=(80, 200, 255))
    im.save(path)


log = []
front_r = []
t0 = time.time()
cutx = None
for t in range(END):
    # food injection, pulsing with random phase per crystal
    for f in foods:
        fx, fy = f
        if alive[fy, fx] and charge[f] > 0:
            amt = FOOD_I * (1 + 0.9 * np.sin(2 * np.pi * t / PULSE_T + phase[f]))
            amt = min(amt, charge[f])
            charge[f] -= amt
            V[fy, fx] += amt
    # elastic pressure relaxation, conservative
    for _ in range(SUB):
        p = V / C
        sh = KAPPA * (alive[:, :-1] & alive[:, 1:])
        sv = KAPPA * (alive[:-1, :] & alive[1:, :])
        Fh = (Dh + sh) * (p[:, :-1] - p[:, 1:]) / SUB
        Fv = (Dv + sv) * (p[:-1, :] - p[1:, :]) / SUB
        V[:, :-1] -= Fh; V[:, 1:] += Fh
        V[:-1, :] -= Fv; V[1:, :] += Fv
    # adapt tubes to the last substep's flux (per tick units)
    live_h, live_v = Dh > 0, Dv > 0
    Dh = np.where(live_h, np.minimum(DMAX, Dh + A * sig(Fh * SUB) * 0.01 - R * Dh), 0)
    Dv = np.where(live_v, np.minimum(DMAX, Dv + A * sig(Fv * SUB) * 0.01 - R * Dv), 0)
    Dh[Dh < DMIN] = 0; Dv[Dv < DMIN] = 0
    # upkeep and starving
    V[alive] -= UPKEEP
    empty = alive & (V <= 0)
    V[V < 0] = 0
    starve = np.where(empty, starve + 1, 0)
    # a node with no tube left (and not a food) or starved too long dies
    deg = np.zeros((H, W), int)
    deg[:, :-1] += Dh > 0; deg[:, 1:] += Dh > 0
    deg[:-1, :] += Dv > 0; deg[1:, :] += Dv > 0
    die = alive & (starve > STARVE_TICKS)
    for f in foods:
        if charge[f] > 0:
            die[f[1], f[0]] &= starve[f[1], f[0]] > STARVE_TICKS
    if die.any():
        alive[die] = False; V[die] = 0
        Dh[:, :][die[:, :-1] | die[:, 1:]] = 0
        Dv[die[:-1, :] | die[1:, :]] = 0
    # growth: a front node with spare volume builds one tube into an empty neighbour
    ys, xs = np.nonzero(alive & (V > GROW_V))
    order = rng.permutation(len(ys))
    for k in order:
        y, x = ys[k], xs[k]
        cands = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        cands = [(a, b) for a, b in cands if 0 <= a < W and 0 <= b < H and not alive[b, a]]
        if not cands:
            continue
        if cutx is not None and any(cutx[0] <= a <= cutx[1] for a, b in cands):
            cands = [(a, b) for a, b in cands if not (cutx[0] <= a <= cutx[1])]
            if not cands:
                continue
        wts = np.array([scent[b, a] + 0.15 for a, b in cands]) ** 2
        a, b = cands[rng.choice(len(cands), p=wts / wts.sum())]
        alive[b, a] = True
        V[y, x] -= GROW_COST
        V[b, a] = GROW_COST * 0.6
        if a != x:
            Dh[y, min(a, x)] = D0
        else:
            Dv[min(b, y), x] = D0
    # the cut: a band 2 nodes wide, held open
    if t == CUT_AT:
        cols = alive.any(axis=0)
        cutx = (hx + 14, hx + 15)
        alive[:, cutx[0]:cutx[1] + 1] = False; V[:, cutx[0]:cutx[1] + 1] = 0
        Dh[:, cutx[0] - 1:cutx[1] + 1] = 0; Dv[:, cutx[0]:cutx[1] + 1] = 0
    if t % TPS == 0:
        s = t // TPS
        r = np.hypot(xx[alive] - hx, yy[alive] - hy)
        front = np.percentile(r, 95) if r.size else 0
        far = alive & (xx > (cutx[1] if cutx else hx + 15))
        near = alive & (xx < (cutx[0] if cutx else hx + 14))
        Dsum = lambda m: Dh[m[:, :-1] & m[:, 1:]].sum() + Dv[m[:-1, :] & m[1:, :]].sum()
        log.append((s, int(alive.sum()), round(front, 1), int(far.sum()), round(Dsum(far), 2), int(near.sum()), round(Dsum(near), 2)))
    if t // TPS in (30, 90, 149, 160, 175, 199) and t % TPS == 0:
        L, big = loops(DMIN * 2)
        print(f"t={t // TPS:4d}s nodes={int(alive.sum())} loops={L} largest={big:.2f} trunks(D>0.05)={int((Dh > 0.05).sum() + (Dv > 0.05).sum())}", flush=True)
        draw(f"{out}/flow-s{t // TPS:03d}.png")
print("ms a tick (numpy, 1 core):", round(1000 * (time.time() - t0) / END, 2))
print("s nodes front95 far_nodes far_D near_nodes near_D")
for row in log[::10] + log[148:200:3]:
    print(*row)
