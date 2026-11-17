# windows_table.py DIR: the table of windows.sh's paired windows. Per window
# and variant: walkers killed in the window (shots and rounds), flares cast,
# the slime's mass at the end, soldiers at the end. Then each route card
# against the plain gun: paired counts of windows with more kills, with less
# slime at the end and with more soldiers left, each with a two-sided
# sign-test p. Four deciding comparisons were named before the run (flare:
# soldiers, slime; road: kills, soldiers), so p below 0.0125 survives a
# Bonferroni correction.
import re, sys, os
from math import comb
S = sys.argv[1]
V = ['plain', 'flare', 'road']
def window(d):
    p = os.path.join(S, d, 'log.txt')
    if not os.path.exists(p): return None
    t = open(p).read()
    g = re.findall(r"t +[\d.]+ s: mould (\d+), soldiers (\d+), bank [\d.-]+, harvested [\d.]+, rendered \d+, shots (\d+), killed (\d+)", t)
    r = re.findall(r"rounds t +[\d.]+ s: .*?, (\d+) trail bursts, .*, (\d+) flares", t)
    if len(g) < 2: return None
    return dict(killed=int(g[-1][3]) - int(g[0][3]), mould=int(g[-1][0]), sold=int(g[-1][1]),
                trails=int(r[-1][0]) if r else 0, flares=int(r[-1][1]) if r else 0)
names = sorted({d.rsplit('-', 1)[0] for d in os.listdir(S) if os.path.isdir(os.path.join(S, d))},
               key=lambda n: (n.split('-')[0], int(n.split('-')[1])))
def sign(a, b):
    n = a + b; k = min(a, b)
    return min(1, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n) if n else 1
pairs = {v: [0] * 6 for v in ['flare', 'road']}
tot = {v: [0, 0, 0, 0] for v in V}
print(f"{'window':10} " + " ".join(f"{v + ' kill/flares/trails mould sold':>38}" for v in V))
for n in names:
    w = {v: window(f"{n}-{v}") for v in V}
    if any(x is None for x in w.values()): print(n, 'missing'); continue
    print(f"{n:10} " + " ".join(f"{w[v]['killed']:>10}/{w[v]['flares']:<4}/{w[v]['trails']:<5} m{w[v]['mould']:<6} s{w[v]['sold']:<3}" for v in V))
    for v in V:
        tot[v][0] += w[v]['killed']; tot[v][1] += w[v]['mould']; tot[v][2] += w[v]['sold']; tot[v][3] += w[v]['flares']
    for a, c in pairs.items():
        b = w['plain']
        if w[a]['killed'] > b['killed']: c[0] += 1
        elif w[a]['killed'] < b['killed']: c[1] += 1
        if w[a]['mould'] < b['mould']: c[2] += 1
        elif w[a]['mould'] > b['mould']: c[3] += 1
        if w[a]['sold'] > b['sold']: c[4] += 1
        elif w[a]['sold'] < b['sold']: c[5] += 1
print()
for v in V:
    print(f"{v:6} killed {tot[v][0]}, slime at the ends {tot[v][1]}, soldiers at the ends {tot[v][2]}, flares {tot[v][3]}")
print()
for a, (ku, kd, mu, md, su, sd) in pairs.items():
    print(f"{a} against plain: more kills {ku}, fewer {kd} (p={sign(ku, kd):.3f}); less slime {mu}, more {md} (p={sign(mu, md):.3f}); more soldiers left {su}, fewer {sd} (p={sign(su, sd):.3f})")
