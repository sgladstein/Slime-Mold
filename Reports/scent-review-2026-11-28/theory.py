import numpy as np, math, json, sys
# 3x3 mean diffusion: D = 1/3 cell^2 per tick; 30 engine ticks a second at scale 2.
D = 1 / 3
TPS = 30
rows = [("Trail (plain)", 0.9), ("Trail (Drifter)", 0.93), ("Trail (Sprawler)", 0.8), ("Trail (Trunk)", 0.95),
        ("Food scent", 0.97), ("Alarm", 0.98)]
out = {}
for n, k in rows:
    mu = -math.log(k)
    L = math.sqrt(D / mu)
    hl = math.log(2) / mu / TPS
    out[n] = (hl, L)
    print(f"{n:18s} keep {k}: half-life {hl:5.2f} s, spread length {L:4.1f} fine cells ({L/2:4.1f} dish cells)")


def run(lay=20.0, keep=0.97, r=12, n=420, ticks=4000, ftz=True):
    s = np.zeros((n, n), np.float32)
    y, x = np.mgrid[:n, :n]
    src = ((x - n // 2) ** 2 + (y - n // 2) ** 2 <= r * r).astype(np.float32) * np.float32(lay)
    for t in range(ticks):
        a = s + src
        p = np.pad(a, 1)
        m = sum(p[1 + dy:1 + dy + n, 1 + dx:1 + dx + n] for dy in (-1, 0, 1) for dx in (-1, 0, 1)) / np.float32(9)
        s = (m * np.float32(keep)).astype(np.float32)
        if ftz:
            s[np.abs(s) < np.float32(1.1754944e-38)] = 0
        s[0, :] = s[-1, :] = s[:, 0] = s[:, -1] = 0
    return s


if len(sys.argv) > 1:
    s = run()
    c = s.shape[0] // 2
    prof = s[c, c:]
    np.save("profile.npy", prof)
    for d in [0, 12, 15, 20, 24, 30, 36, 48, 60, 90, 120, 150, 180]:
        print(d, prof[d])
    nz = np.nonzero(prof)[0]
    print("last nonzero at", nz.max())
