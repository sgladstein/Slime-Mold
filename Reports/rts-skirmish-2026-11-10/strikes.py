#!/usr/bin/env python3
"""The economy sketch's test 1: does Fortify answer a strike? Harness `duel`
lines with `def=` (round 8 on): for each plan, the strikes that landed on a
pile it held, and how many it still held 10 s later, in all and among those
it had fortified or rallied when the strike landed. Then each plan's games
against soft, and each pair of plans game by game against soft (same seed,
same side): in how many games the first took more of the score, with a
two-sided sign test p.

Usage: strikes.py FILE...
"""
import math
import re
import sys
from collections import defaultdict

LINE = re.compile(r"DUEL seed=(\d+) left=(\w+) right=(\w+) share=([\d.]+) .* def=([^ ]+);([^ ]+) ")
DEF = re.compile(r"(\d+)/(\d+),f(\d+)/(\d+),r(\d+)/(\d+)")

held = defaultdict(lambda: [0] * 6)  # plan -> held, all, fort held, fort all, rally held, rally all
games = defaultdict(lambda: [0, 0])  # plan v soft -> [won, played]
share = defaultdict(dict)  # plan -> {(seed, side): its share against soft}
for f in sys.argv[1:]:
    for line in open(f):
        m = LINE.search(line)
        if not m:
            continue
        left, right, x = m[2], m[3], float(m[4])
        for side, plan, d, foe, mine in ((0, left, m[5], right, x), (1, right, m[6], left, 1 - x)):
            if foe != "soft":
                continue
            k = DEF.match(d)
            for j in range(6):
                held[plan][j] += int(k[j + 1])
            if plan != "soft" or left != right:
                games[plan][0] += mine > 0.5
                games[plan][1] += 1
                share[plan][(int(m[1]), side)] = mine


def sign_p(k, n):
    if n == 0:
        return 1.0
    lo = min(k, n - k)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(lo + 1)) / 2**n)


print("Strikes by soft on a pile the defender held, held 10 s later:")
for plan, v in sorted(held.items()):
    print(f"  {plan:>6}: all {v[0]}/{v[1]}, fortified {v[2]}/{v[3]}, rallied {v[4]}/{v[5]}")
print("Games against soft (won/played):")
for plan, (w, n) in sorted(games.items()):
    print(f"  {plan:>6}: {w}/{n}")
print("Game by game against soft (same seed and side):")
plans = sorted(share)
for i, a in enumerate(plans):
    for b in plans[i + 1:]:
        keys = set(share[a]) & set(share[b])
        more = sum(share[a][k] > share[b][k] for k in keys)
        less = sum(share[a][k] < share[b][k] for k in keys)
        print(f"  {a} more than {b} in {more}, less in {less} of {len(keys)}, p={sign_p(more, more + less):.2g}")
