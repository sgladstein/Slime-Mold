# windows_table.py DIR: the table of windows.sh's paired windows for
# Provoke and Still. Per window and variant: walkers killed in the window,
# the slime's mass at its end, soldiers and their mean health at its end,
# raids sent in it, the Mind casts made. Then each card against plain: the
# paired counts of windows with more soldiers left (ties broken by health),
# with more kills and with less slime at the end, each with a two-sided
# sign-test p over the windows that differ.
import os
import re
import sys
from math import comb

S = sys.argv[1]
VARS = ['plain', 'provoke', 'still']
T_LINE = re.compile(r"^  t +([\d.]+) s: mould (\d+), soldiers (\d+), bank [\d.-]+, harvested [\d.]+, rendered \d+, shots (\d+), killed (\d+)", re.M)
SQUAD = re.compile(r"^  squad: touching [\d.]+ mean, \d+ most; health (\d+) mean", re.M)


def window(d):
    p = os.path.join(S, d, 'log.txt')
    if not os.path.exists(p):
        return None
    t = open(p).read()
    g = T_LINE.findall(t)
    h = SQUAD.findall(t)
    if len(g) < 2:
        return None
    t0, t1 = float(g[0][0]), float(g[-1][0])
    raids = [float(x) for x in re.findall(r"^  raid \d+ at t ([\d.]+) s", t, re.M)]
    mind = re.findall(r"(\d+) Mind casts", t)
    end = re.search(r"ended: ([a-z ]+)", t)
    return dict(killed=int(g[-1][4]) - int(g[0][4]), mould=int(g[-1][1]), sold=int(g[-1][2]),
                health=int(h[-1]) if h else 0, raids=sum(t0 <= r <= t1 for r in raids),
                mind=int(mind[-1]) - int(mind[0]) if len(mind) > 1 else 0, end=end.group(1) if end else '')


def sign(wins, losses):
    n = wins + losses
    if n == 0:
        return 1.0
    k = min(wins, losses)
    return min(1.0, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n)


names = sorted({re.sub(r"-(plain|provoke|still)$", "", d) for d in os.listdir(S) if re.match(r"p\d-\d+-", d)})
rows = {}
print(f"{'window':8} " + "  ".join(f"{v:>34}" for v in VARS))
print(f"{'':8} " + "  ".join(f"{'killed  slime  soldiers health raids mind':>34}" for v in VARS))
for n in names:
    cells = []
    for v in VARS:
        w = window(f"{n}-{v}")
        rows[(n, v)] = w
        cells.append(f"{'-':>34}" if w is None else f"{w['killed']:>6} {w['mould']:>6} {w['sold']:>8} {w['health']:>6} {w['raids']:>5} {w['mind']:>4}")
    print(f"{n:8} " + "  ".join(cells))
print()
for v in ['provoke', 'still']:
    pairs = [(rows[(n, 'plain')], rows[(n, v)]) for n in names if rows.get((n, 'plain')) and rows.get((n, v))]
    if not pairs:
        continue
    def tally(key):
        more = sum(key(b) > key(a) for a, b in pairs)
        less = sum(key(b) < key(a) for a, b in pairs)
        return more, less, sign(more, less)
    s = tally(lambda w: (w['sold'], w['health']))
    k = tally(lambda w: w['killed'])
    m = tally(lambda w: -w['mould'])
    tot = lambda key: (sum(key(a) for a, _ in pairs), sum(key(b) for _, b in pairs))
    print(f"{v} against plain, {len(pairs)} windows:")
    print(f"  soldiers left (then health): more in {s[0]}, fewer in {s[1]}, p {s[2]:.3f}; soldiers {tot(lambda w: w['sold'])}")
    print(f"  kills: more in {k[0]}, fewer in {k[1]}, p {k[2]:.3f}; totals {tot(lambda w: w['killed'])}")
    print(f"  slime at the end: less in {m[0]}, more in {m[1]}, p {m[2]:.3f}; totals {tot(lambda w: w['mould'])}")
    print(f"  raids sent: {tot(lambda w: w['raids'])}")
