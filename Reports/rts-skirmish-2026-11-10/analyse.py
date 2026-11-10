#!/usr/bin/env python3
"""Round robin of the budget skirmish's scripted plans (harness `duel`).

Usage: analyse.py FILE...   (DUEL lines; each pairing played from both sides
on every seed). For each pairing: games the first plan won (its share of the
food eaten above half) out of all games, the two-sided sign test p, and
whether it survives a Holm correction over every pairing. Then each plan's
record, and every cycle of three among the pairings that survive.
"""
import itertools
import math
import re
import sys
from collections import defaultdict

LINE = re.compile(r"DUEL seed=(\d+) left=(\w+) right=(\w+) share=([\d.]+) at60=([\d.]+)")


def sign_p(k, n):
    """Two-sided sign test: k of n."""
    if n == 0:
        return 1.0
    lo = min(k, n - k)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(lo + 1)) / 2**n)


games = defaultdict(list)  # (a, b) with a < b -> [(seed, a's share, a's share at 60 s)]
plans = []
for f in sys.argv[1:]:
    for line in open(f):
        m = LINE.search(line)
        if not m:
            continue
        seed, left, right, share, at60 = int(m[1]), m[2], m[3], float(m[4]), float(m[5])
        for p in (left, right):
            if p not in plans:
                plans.append(p)
        a, b = sorted((left, right))
        sa = share if left == a else 1 - share
        sa60 = at60 if left == a else 1 - at60
        games[(a, b)].append((seed, sa, sa60))

rows = []
for (a, b), g in sorted(games.items()):
    n = len(g)
    ties = sum(1 for _, s, _ in g if s == 0.5)
    k = sum(1 for _, s, _ in g if s > 0.5)
    mean = sum(s for _, s, _ in g) / n
    win, lose = (a, b) if k * 2 >= n - ties else (b, a)
    kw = k if win == a else n - ties - k
    rows.append([win, lose, kw, n, sign_p(k, n - ties), mean if win == a else 1 - mean])

# Holm over every pairing.
order = sorted(range(len(rows)), key=lambda i: rows[i][4])
m = len(rows)
alive = True
for rank, i in enumerate(order):
    alive = alive and rows[i][4] <= 0.05 / (m - rank)
    rows[i].append(alive)

print(f"{len(rows)} pairings, {sum(len(g) for g in games.values())} games\n")
print(f"{'winner':>8} {'loser':>8}  games  p        Holm  winner's mean share")
for win, lose, kw, n, p, mean, holm in sorted(rows, key=lambda r: r[4]):
    print(f"{win:>8} {lose:>8}  {kw:>2}/{n:<2}  {p:<8.2g} {'yes' if holm else '-':>4}  {mean:.3f}")

print("\nEach plan: pairings won (surviving Holm), lost (surviving), mean share over all its games")
rec = {}
for p in plans:
    w = sum(1 for r in rows if r[0] == p and r[3] > 0 and r[2] * 2 > r[3])
    l = sum(1 for r in rows if r[1] == p and r[2] * 2 > r[3])
    ws = sum(1 for r in rows if r[0] == p and r[6])
    ls = sum(1 for r in rows if r[1] == p and r[6])
    shares = [s if p == a else 1 - s for (a, b), g in games.items() if p in (a, b) for _, s, _ in g]
    rec[p] = (w, l, ws, ls, sum(shares) / len(shares))
for p, (w, l, ws, ls, ms) in sorted(rec.items(), key=lambda x: -x[1][4]):
    print(f"{p:>8}: won {w} ({ws} survive), lost {l} ({ls} survive), mean share {ms:.3f}")

beats = {(r[0], r[1]) for r in rows if r[6]}
cycles = [c for c in itertools.permutations(plans, 3) if c[0] == min(c) and (c[0], c[1]) in beats and (c[1], c[2]) in beats and (c[2], c[0]) in beats]
print("\nCycles of three among surviving pairings:", cycles if cycles else "none")
beats_any = {(r[0], r[1]) for r in rows if r[2] * 2 > r[3]}
cycles = [c for c in itertools.permutations(plans, 3) if c[0] == min(c) and (c[0], c[1]) in beats_any and (c[1], c[2]) in beats_any and (c[2], c[0]) in beats_any]
print("Cycles of three on the majority alone:", cycles if cycles else "none")
