#!/usr/bin/env python3
"""Tables for the squad movement report from measure.sh's before.txt and
after.txt: each move and squad size, the mean over seeds before -> after,
and for the planets (seeds 1-6 at 40 and 200) paired counts with a
two-sided sign test.

python3 summary.py raw/before.txt raw/after.txt"""
import re
import sys
from collections import defaultdict
from math import comb

KEYS = ["still", "jerk", "turn", "pace", "back", "settle", "arrive", "start", "waits", "mean", "p95", "worst"]


def parse(path):
    rows = {}
    for line in open(path):
        m = re.match(r"\s*(\S+) n=\s*(\d+) seed=(\d+) (.*)", line)
        if not m:
            continue
        name, n, seed, rest = m.group(1), int(m.group(2)), int(m.group(3)), m.group(4)
        rest = rest.replace("move ms mean", "mean")
        d = {}
        for k, v in re.findall(r"(\w+)=\s*([-\w.]+)", rest):
            v = v.rstrip("s")
            d[k] = None if v == "never" else float(v)
        rows[(name, n, seed)] = d
    return rows


def mean(xs):
    xs = [x for x in xs if x is not None]
    return sum(xs) / len(xs) if xs else None


def fmt(x, k):
    if x is None:
        return "never"
    if k in ("waits",):
        return f"{x:.0f}"
    if k in ("turn", "settle", "arrive"):
        return f"{x:.1f}"
    return f"{x:.2f}"


def sign_p(wins, losses):
    n = wins + losses
    if n == 0:
        return 1.0
    k = min(wins, losses)
    return min(1.0, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n)


b, a = parse(sys.argv[1]), parse(sys.argv[2])
groups = defaultdict(list)
for key in a:
    if key in b:
        groups[key[:2]].append(key[2])
show = ["still", "jerk", "turn", "pace", "back", "settle", "arrive", "waits", "mean", "p95"]
print("| move | n | seeds | " + " | ".join(show) + " |")
print("|---|---|---|" + "---|" * len(show))
for (name, n), seeds in sorted(groups.items(), key=lambda g: (list(dict.fromkeys(k[0] for k in a)).index(g[0][0]), g[0][1])):
    cells = []
    for k in show:
        bb = mean([b[(name, n, s)].get(k) for s in seeds]) if all(b[(name, n, s)].get(k) is not None for s in seeds) else None
        aa = mean([a[(name, n, s)].get(k) for s in seeds]) if all(a[(name, n, s)].get(k) is not None for s in seeds) else None
        cells.append(f"{fmt(bb, k)} → {fmt(aa, k)}")
    print(f"| {name} | {n} | {len(seeds)} | " + " | ".join(cells) + " |")
print()
# Planets: paired sign tests, lower is better except pace.
print("Planets 1 and 8, seeds 1-6, squads of 40 and 200 (24 pairs): after better / worse, sign test p")
for k in ["still", "jerk", "turn", "pace", "back", "settle", "arrive", "waits", "mean"]:
    w = l = 0
    for (name, n), seeds in groups.items():
        if not name.startswith("planet"):
            continue
        for s in seeds:
            x, y = b[(name, n, s)].get(k), a[(name, n, s)].get(k)
            if k == "arrive":
                x = 1e9 if x is None else x
                y = 1e9 if y is None else y
            if x is None or y is None or x == y:
                continue
            better = y > x if k == "pace" else y < x
            w += better
            l += not better
    print(f"  {k}: {w} better, {l} worse, p={sign_p(w, l):.4f}")
