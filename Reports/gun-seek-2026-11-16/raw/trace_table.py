# trace_table.py DIR [VARIANT...]: the table of trace-windows.sh's or
# line-windows.sh's paired windows (default variants: trace-windows.sh's). Per
# window and variant: hits/shots (a hit = a shot that struck slime; a miss
# ran out of range or hit a wall first), kills, soldiers at the end. Totals
# over the thin windows (all but hold, the Fork's thick front) and the thick
# ones, and per variant against plain the paired counts of windows with a
# higher hit rate and with more kills, with a two-sided sign-test p.
import re, sys, os
from math import comb
S = sys.argv[1]
V = sys.argv[2:] or ['plain', 'catch2', 'catch4', 'catch2turn']
def window(d):
    p = os.path.join(S, d, 'log.txt')
    if not os.path.exists(p): return None
    t = open(p).read()
    g = re.findall(r"t +[\d.]+ s: mould (\d+), soldiers (\d+), bank [\d.-]+, harvested [\d.]+, rendered \d+, shots (\d+), killed (\d+)", t)
    m = re.findall(r"(\d+) ran out of range and (\d+) hit a wall", t)
    tu = re.findall(r"(\d+) seeker turns", t)
    if len(g) < 2 or len(m) < 2: return None
    shots = int(g[-1][2]) - int(g[0][2])
    miss = int(m[-1][0]) + int(m[-1][1]) - int(m[0][0]) - int(m[0][1])
    return dict(shots=shots, miss=miss, hits=shots - miss, killed=int(g[-1][3]) - int(g[0][3]), sold=int(g[-1][1]),
                wall=int(m[-1][1]) - int(m[0][1]), turns=int(tu[-1]) - int(tu[0]))
names = sorted({d.rsplit('-', 1)[0] for d in os.listdir(S) if os.path.isdir(os.path.join(S, d))},
               key=lambda n: (n.split('-')[0], int(n.split('-')[1])))
def sign(a, b):
    n = a + b; k = min(a, b)
    return min(1, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n) if n else 1
tot = {lab: {v: [0, 0, 0] for v in V} for lab in ('thin', 'thick')}
pair = {v: [0, 0, 0, 0] for v in V[1:]}  # hit rate up, down; kills up, down (thin)
print(f"{'window':10} " + " ".join(f"{v + ' hits/shots kill sold':>30}" for v in V))
for n in names:
    w = {v: window(f"{n}-{v}") for v in V}
    if any(x is None for x in w.values()): print(n, 'missing'); continue
    print(f"{n:10} " + " ".join(f"{w[v]['hits']:>12}/{w[v]['shots']:<4} k{w[v]['killed']:<6} s{w[v]['sold']:<3}" for v in V))
    lab = 'thick' if n.startswith('hold') else 'thin'
    for v in V:
        a = tot[lab][v]; a[0] += w[v]['hits']; a[1] += w[v]['shots']; a[2] += w[v]['killed']
    if lab == 'thin' and w['plain']['shots'] > 0:
        rp = w['plain']['hits'] / w['plain']['shots']
        for v in V[1:]:
            rv = w[v]['hits'] / max(w[v]['shots'], 1)
            if rv > rp: pair[v][0] += 1
            elif rv < rp: pair[v][1] += 1
            if w[v]['killed'] > w['plain']['killed']: pair[v][2] += 1
            elif w[v]['killed'] < w['plain']['killed']: pair[v][3] += 1
print()
for lab in ('thin', 'thick'):
    for v in V:
        h, s, k = tot[lab][v]
        print(f"{lab:5} {v:10} hits {h}/{s} = {100 * h / max(s, 1):.1f}%, killed {k}")
print()
for v in V[1:]:
    u, d, ku, kd = pair[v]
    print(f"thin windows, {v} against plain: hit rate higher {u}, lower {d} (sign p={sign(u, d):.3f}); kills higher {ku}, lower {kd} (sign p={sign(ku, kd):.3f})")
