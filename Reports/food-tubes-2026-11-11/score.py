#!/usr/bin/env python3
"""Score Reports/food-tubes-2026-11-11 runs: python3 score.py RAW_DIR.
Crystals on the main body (`joined`, pile 0 the home not counted) at 15,
30, 60 and 90 s; crystals first reached by 60 s (`firsts`, pile 0 not
counted); and the share of those reached that are joined at 60 s. The
deciding comparison: joined at 60 s, race - droplets, sign test with ties
dropped. It was named with a second, racecry - droplets, Holm over the
two; racecry matched race to the hash and was dropped, so p is also
quoted doubled."""
import glob, math, re, sys
out = sys.argv[1]; J = {}; F = {}
for f in glob.glob(out + '/*.txt'):
    for l in open(f):
        m = re.match(r'seed (\d+) (\S+) t +([\d.]+)s  joined (\S+) body ([\d.]+)', l)
        if m:
            on = [] if m[4] == '-' else [int(v) for v in m[4].split(',')]
            J[(m[2], int(m[1]), float(m[3]))] = ([p for p in on if p != 0], float(m[5]))
        m = re.match(r'seed (\d+) (\S+) firsts (.*)', l)
        if m:
            F[(m[2], int(m[1]))] = [tuple(float(v) for v in p.split(':')) for p in m[3].split()]
def sign(w, n):
    if n == 0:
        return 1.0
    k = max(w, n - w)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(k, n + 1)) / 2 ** n)
arms = ['droplets', 'race']
seeds = sorted({s for (a, s) in F})
for a in arms:
    print(f"{a}:")
    for T in (15.0, 30.0, 60.0, 90.0):
        v = [len(J[(a, s, T)][0]) for s in seeds if (a, s, T) in J]
        print(f"  joined at {T:3.0f} s: " + ' '.join(map(str, v)) + f"  (median {sorted(v)[len(v)//2] if v else '-'})")
    body = [J[(a, s, 60.0)][1] for s in seeds if (a, s, 60.0) in J]
    print("  body at 60 s (dish cells): " + ' '.join(f"{b:.0f}" for b in body))
    reach = [sum(1 for p in F[(a, s)] if p[0] >= 1 and 0 <= p[3] <= 60) for s in seeds if (a, s) in F]
    print("  reached by 60 s: " + ' '.join(map(str, reach)))
    tot_r = tot_j = 0
    for s in seeds:
        if (a, s) in F and (a, s, 60.0) in J:
            r = {int(p[0]) for p in F[(a, s)] if p[0] >= 1 and 0 <= p[3] <= 60}
            tot_r += len(r); tot_j += len(r & set(J[(a, s, 60.0)][0]))
    print(f"  of crystals reached by 60 s, joined at 60 s: {tot_j} of {tot_r}")
rows = []
for a, b in [('race', 'droplets')]:
    for T in (60.0, 90.0):
        ss = [s for s in seeds if (a, s, T) in J and (b, s, T) in J]
        d = [len(J[(a, s, T)][0]) - len(J[(b, s, T)][0]) for s in ss]
        up, down = sum(x > 0 for x in d), sum(x < 0 for x in d)
        rows.append((T, a, b, up, down, len(d) - up - down, sign(up, up + down)))
for (T, a, b, up, down, ties, p) in rows:
    if T == 60.0:
        print(f"DECIDING joined at 60 s, {a} - {b}: more on {up}, fewer on {down}, ties {ties}; p {p:.5f}, doubled {min(1, 2 * p):.5f}")
for (T, a, b, up, down, ties, p) in rows:
    if T == 90.0:
        print(f"secondary joined at 90 s, {a} - {b}: more on {up}, fewer on {down}, ties {ties}; p {p:.5f}")
