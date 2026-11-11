#!/usr/bin/env python3
"""Score Reports/first-reach-2026-11-11 runs: python3 score.py OUT_DIR [T=30].
Scored piles: index >= 1 and distance from the start > 66 dish cells."""
import glob, math, re, sys
out = sys.argv[1]; T = float(sys.argv[2]) if len(sys.argv) > 2 else 30.0
R = {}
for f in glob.glob(out + '/*.txt'):
    for l in open(f):
        m = re.match(r'seed (\d+) (\S+) firsts (.*)', l)
        if not m:
            continue
        piles = [tuple(float(v) for v in p.split(':')) for p in m[3].split()]
        scored = [p for p in piles if p[0] >= 1 and p[1] > 66]
        R[(m[2], int(m[1]))] = (sum(1 for p in scored if 0 <= p[3] <= T), len(scored))
def sign(w, n):
    if n == 0:
        return 1.0
    k = max(w, n - w)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(k, n + 1)) / 2 ** n)
pairs = [('grazers', 'droplets'), ('grazescouts', 'grazers'), ('dropscouts', 'droplets'), ('dropfast', 'droplets'), ('scoutsonly', 'droplets')]
names = sorted({k[0] for k in R})
for n in names:
    v = [R[(n, s)] for s in range(1, 33) if (n, s) in R]
    print(f"{n:12s} seeds {len(v):2d}  reached by {T:.0f}s: " + ' '.join(f"{a}/{b}" for a, b in v))
rows = []
for a, b in pairs:
    seeds = [s for s in range(1, 33) if (a, s) in R and (b, s) in R]
    d = [R[(a, s)][0] - R[(b, s)][0] for s in seeds]
    up, down = sum(x > 0 for x in d), sum(x < 0 for x in d)
    rows.append((sign(up, up + down), a, b, up, down, len(d) - up - down, sum(d)))
rows.sort()
m = len(rows)
for i, (p, a, b, up, down, ties, tot) in enumerate(rows):
    holm = min(1.0, p * (m - i))
    print(f"{a} - {b}: more on {up}, fewer on {down}, ties {ties}, total {tot:+d}; p {p:.4f}, Holm {holm:.4f}")
