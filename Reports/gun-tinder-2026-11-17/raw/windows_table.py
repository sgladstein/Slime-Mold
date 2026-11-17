# windows_table.py DIR: the table of windows.sh's paired windows for Tinder.
# Per window and variant: walkers killed in the window (shots, rounds and
# fire), the fire's share of them, the trail bursts laid, the slime's mass at
# the end, soldiers at the end. Plain, torch and blast come from the element
# report's table (../../gun-element-2026-11-17/raw/windows.txt: the same
# scenes; windows.sh re-runs p1g2's plain and torch here, which must match
# it). Then each pairing: the paired counts of windows with more kills and
# with less slime at the end, with a two-sided sign-test p.
import os
import re
import sys
from math import comb

S = sys.argv[1]
HERE = os.path.dirname(os.path.abspath(__file__))
OWN = ['scorch', 'tinder-scorch', 'tinder-torch']
OLD = ['plain', 'torch', 'blast']
SCENES = ['p1g2', 'p1g3', 'p1s1', 'p2s1', 'p3g4']


def window(d):
    p = os.path.join(S, d, 'log.txt')
    if not os.path.exists(p):
        return None
    t = open(p).read()
    g = re.findall(r"t +[\d.]+ s: mould (\d+), soldiers (\d+), bank [\d.-]+, harvested [\d.]+, rendered \d+, shots (\d+), killed (\d+)", t)
    r = re.findall(r"rounds t +[\d.]+ s: .*?, (\d+) trail bursts.*?, (\d+) torches, (\d+) walkers killed by fire", t)
    if len(g) < 2:
        return None
    fire = [int(x[2]) for x in r]
    return dict(killed=int(g[-1][3]) - int(g[0][3]), mould=int(g[-1][0]), sold=int(g[-1][1]),
                trail=int(r[-1][0]) - (int(r[0][0]) if len(r) > 1 else 0) if r else 0,
                fire=fire[-1] - (fire[0] if len(fire) > 1 else 0) if fire else 0)


old = {}
for line in open(os.path.join(HERE, '../../gun-element-2026-11-17/raw/windows.txt')):
    m = re.match(r"(p\d\w\d-\d+) +(.*)", line)
    if not m:
        continue
    cells = re.findall(r"(\d+)/(\d+|-) */(\d+) +m(\d+) +s(\d+)", m.group(2))
    for v, c in zip(OLD, cells):
        old[(m.group(1), v)] = dict(killed=int(c[0]), fire=0 if c[1] == '-' else int(c[1]), mould=int(c[3]), sold=int(c[4]), trail=0)

for v in ['plain', 'torch']:
    for t in [25, 40]:
        w = window(f"p1g2-{t}-{v}")
        o = old[(f"p1g2-{t}", v)]
        same = w is not None and (w['killed'], w['mould']) == (o['killed'], o['mould'])
        print(f"check p1g2-{t} {v}: here {w and (w['killed'], w['mould'])}, element {(o['killed'], o['mould'])}: {'same' if same else 'DIFFERENT'}")
print()


def sign(a, b):
    n = a + b
    k = min(a, b)
    return min(1, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n) if n else 1


V = OLD + OWN
rows = {}
print(f"{'window':8} " + " ".join(f"{v + ' kill/fire m sold':>27}" for v in V))
for s in SCENES:
    for t in [25, 40]:
        n = f"{s}-{t}"
        w = {v: old.get((n, v)) for v in OLD}
        w.update({v: window(f"{n}-{v}") for v in OWN})
        if any(x is None for x in w.values()):
            print(n, 'missing')
            continue
        rows[n] = w
        print(f"{n:8} " + " ".join(f"{w[v]['killed']:>9}/{w[v]['fire']:<4} m{w[v]['mould']:<6} s{w[v]['sold']:<3}" for v in V))
print()
for v in V:
    k = sum(r[v]['killed'] for r in rows.values())
    f = sum(r[v]['fire'] for r in rows.values())
    m = sum(r[v]['mould'] for r in rows.values())
    print(f"{v:14} killed {k} ({f} by fire), slime at the ends {m}")
print()
for planet in ['p1', 'p2', 'p3']:
    print(planet + ': ' + ', '.join(f"{v} {sum(r[v]['killed'] for n, r in rows.items() if n.startswith(planet))}" for v in V))
print()
for a, b in [('tinder-torch', 'torch'), ('tinder-scorch', 'scorch'), ('tinder-torch', 'plain'), ('tinder-scorch', 'plain'), ('scorch', 'plain'), ('tinder-torch', 'blast')]:
    ku = sum(r[a]['killed'] > r[b]['killed'] for r in rows.values())
    kd = sum(r[a]['killed'] < r[b]['killed'] for r in rows.values())
    mu = sum(r[a]['mould'] < r[b]['mould'] for r in rows.values())
    md = sum(r[a]['mould'] > r[b]['mould'] for r in rows.values())
    print(f"{a} against {b}: more kills {ku}, fewer {kd} (sign p={sign(ku, kd):.3f}); less slime at the end {mu}, more {md} (sign p={sign(mu, md):.3f})")
