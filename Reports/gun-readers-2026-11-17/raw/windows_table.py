# windows_table.py DIR: the table of windows.sh's paired windows. Per window
# and variant: walkers killed in the window (shots and rounds), the rounds'
# kills, reader casts, the slime's mass at the end, soldiers at the end.
# Then each reader against its plain pair: the paired counts of windows with
# more kills and with less slime at the end, with a two-sided sign-test p.
# An optional second argument names the variants (comma-separated), for a
# re-run of only some (the germ pair's, 11-17, after the GPU germ fix).
import re, sys, os
from math import comb
S = sys.argv[1]
V = sys.argv[2].split(',') if len(sys.argv) > 2 else ['gas-plain', 'gas-shatter', 'germ-plain', 'germ-detonate']
def window(d):
    p = os.path.join(S, d, 'log.txt')
    if not os.path.exists(p): return None
    t = open(p).read()
    g = re.findall(r"t +[\d.]+ s: mould (\d+), soldiers (\d+), bank [\d.-]+, harvested [\d.]+, rendered \d+, shots (\d+), killed (\d+)", t)
    r = re.findall(r"rounds t +[\d.]+ s: \d+ casts .*?, (\d+) walkers killed, .*, (\d+) reader casts", t)
    if len(g) < 2: return None
    return dict(killed=int(g[-1][3]) - int(g[0][3]), mould=int(g[-1][0]), sold=int(g[-1][1]),
                rounds=int(r[-1][0]) - (int(r[0][0]) if len(r) > 1 else 0) if r else 0, reads=int(r[-1][1]) if r else 0)
names = sorted({d.rsplit('-', 2)[0] for d in os.listdir(S) if os.path.isdir(os.path.join(S, d))},
               key=lambda n: (n.split('-')[0], int(n.split('-')[1])))
def sign(a, b):
    n = a + b; k = min(a, b)
    return min(1, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n) if n else 1
pairs = {k: [0, 0, 0, 0] for k in [('gas-shatter', 'gas-plain'), ('germ-detonate', 'germ-plain')] if set(k) <= set(V)}
tot = {v: [0, 0, 0] for v in V}
print(f"{'window':10} " + " ".join(f"{v + ' kill/rounds/reads mould sold':>36}" for v in V))
for n in names:
    w = {v: window(f"{n}-{v}") for v in V}
    if any(x is None for x in w.values()): print(n, 'missing'); continue
    print(f"{n:10} " + " ".join(f"{w[v]['killed']:>10}/{w[v]['rounds']:<5}/{w[v]['reads']:<3} m{w[v]['mould']:<6} s{w[v]['sold']:<3}" for v in V))
    for v in V:
        tot[v][0] += w[v]['killed']; tot[v][1] += w[v]['mould']; tot[v][2] += w[v]['reads']
    for (a, b), c in pairs.items():
        if w[a]['killed'] > w[b]['killed']: c[0] += 1
        elif w[a]['killed'] < w[b]['killed']: c[1] += 1
        if w[a]['mould'] < w[b]['mould']: c[2] += 1
        elif w[a]['mould'] > w[b]['mould']: c[3] += 1
print()
for v in V:
    print(f"{v:14} killed {tot[v][0]}, slime at the ends {tot[v][1]}, reader casts {tot[v][2]}")
print()
for (a, b), (ku, kd, mu, md) in pairs.items():
    print(f"{a} against {b}: more kills {ku}, fewer {kd} (sign p={sign(ku, kd):.3f}); less slime at the end {mu}, more {md} (sign p={sign(mu, md):.3f})")
