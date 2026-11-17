# windows_table.py DIR [VARIANTS]: the table of windows.sh's paired windows.
# Per window and variant: walkers killed in the window (shots, rounds and
# fire), the fire's share of them where the log has it, torch casts, the
# slime's mass at the end, soldiers at the end. Then the torch against each
# other variant: the paired counts of windows with more kills and with less
# slime at the end, with a two-sided sign-test p.
import re, sys, os
from math import comb
S = sys.argv[1]
V = sys.argv[2].split(',') if len(sys.argv) > 2 else ['plain', 'torch', 'blast']
def window(d):
    p = os.path.join(S, d, 'log.txt')
    if not os.path.exists(p): return None
    t = open(p).read()
    g = re.findall(r"t +[\d.]+ s: mould (\d+), soldiers (\d+), bank [\d.-]+, harvested [\d.]+, rendered \d+, shots (\d+), killed (\d+)", t)
    r = re.findall(r"rounds t +[\d.]+ s: .*?, (\d+) torches(?:, (\d+) walkers killed by fire)?", t)
    if len(g) < 2: return None
    fire = [int(x[1]) for x in r if x[1]]
    return dict(killed=int(g[-1][3]) - int(g[0][3]), mould=int(g[-1][0]), sold=int(g[-1][1]),
                torches=int(r[-1][0]) if r else 0, fire=(fire[-1] - (fire[0] if len(fire) > 1 else 0)) if fire else None)
names = sorted({d.rsplit('-', 1)[0] for d in os.listdir(S) if os.path.isdir(os.path.join(S, d))},
               key=lambda n: (n.split('-')[0], int(n.split('-')[1])))
def sign(a, b):
    n = a + b; k = min(a, b)
    return min(1, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n) if n else 1
pairs = {('torch', v): [0, 0, 0, 0] for v in V if v != 'torch' and 'torch' in V}
tot = {v: [0, 0, 0] for v in V}
print(f"{'window':10} " + " ".join(f"{v + ' kill/fire/torches mould sold':>34}" for v in V))
for n in names:
    w = {v: window(f"{n}-{v}") for v in V}
    if any(x is None for x in w.values()): print(n, 'missing'); continue
    f = lambda x: '-' if x is None else str(x)
    print(f"{n:10} " + " ".join(f"{w[v]['killed']:>10}/{f(w[v]['fire']):<5}/{w[v]['torches']:<3} m{w[v]['mould']:<6} s{w[v]['sold']:<3}" for v in V))
    for v in V:
        tot[v][0] += w[v]['killed']; tot[v][1] += w[v]['mould']; tot[v][2] += w[v]['torches']
    for (a, b), c in pairs.items():
        if w[a]['killed'] > w[b]['killed']: c[0] += 1
        elif w[a]['killed'] < w[b]['killed']: c[1] += 1
        if w[a]['mould'] < w[b]['mould']: c[2] += 1
        elif w[a]['mould'] > w[b]['mould']: c[3] += 1
print()
for v in V:
    print(f"{v:8} killed {tot[v][0]}, slime at the ends {tot[v][1]}, torches {tot[v][2]}")
print()
for (a, b), (ku, kd, mu, md) in pairs.items():
    print(f"{a} against {b}: more kills {ku}, fewer {kd} (sign p={sign(ku, kd):.3f}); less slime at the end {mu}, more {md} (sign p={sign(mu, md):.3f})")
