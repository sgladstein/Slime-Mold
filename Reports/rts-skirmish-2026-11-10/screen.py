#!/usr/bin/env python3
"""Round 4's screen: six rule sets, each a small round robin (harness `duel`
lines prefixed by the rule set's tag). For each rule set: soft's record
against each plan, the plans by mean share, majority cycles of three, and
the snowball (the side ahead at 1:00 won; games ended early by a wipe-out).

Usage: screen.py FILE...
"""
import itertools
import math
import re
import sys
from collections import defaultdict

LINE = re.compile(r"^(\w+) DUEL seed=(\d+) left=(\w+) right=(\w+) share=([\d.]+) at60=([\d.]+) .* t=(\d+).*?(?: bell=(\d+))?(?: \(|$)")


def sign_p(k, n):
    if n == 0:
        return 1.0
    lo = min(k, n - k)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(lo + 1)) / 2**n)


games = defaultdict(list)  # tag -> [(seed, left, right, share, at60, t)]
order = []
for f in sys.argv[1:]:
    for line in open(f):
        m = LINE.search(line)
        if not m:
            continue
        tag = m[1]
        if tag not in order:
            order.append(tag)
        # The bell is on the line from round 7; 150 before.
        games[tag].append((int(m[2]), m[3], m[4], float(m[5]), float(m[6]), int(m[7]), int(m[8] or 150)))

for tag in order:
    g = games[tag]
    plans = sorted({p for x in g for p in x[1:3]})
    wins = defaultdict(int)  # (a, b) -> games a won against b
    n = defaultdict(int)
    share = defaultdict(list)
    for seed, l, r, s, s60, t, _bell in g:
        n[(l, r)] += 1
        n[(r, l)] += 1
        share[l].append(s)
        share[r].append(1 - s)
        if s > 0.5:
            wins[(l, r)] += 1
        elif s < 0.5:
            wins[(r, l)] += 1
    lead = [(s60 > 0.5) == (s > 0.5) for _, _, _, s, s60, _, _ in g if s60 != 0.5 and s != 0.5]
    wiped = sum(1 for x in g if x[5] < x[6])
    print(f"== {tag}: {len(g)} games; ahead at 1:00 won {sum(lead)}/{len(lead)} ({sum(lead)/len(lead):.0%}); wipe-outs {wiped}")
    print("   soft v: " + ", ".join(f"{o} {wins[('soft', o)]}/{n[('soft', o)]}" for o in plans if o != "soft"))
    beats_soft = [o for o in plans if o != "soft" and wins[(o, "soft")] > n[(o, "soft")] / 2]
    print(f"   beat soft (majority): {beats_soft or 'none'}")
    rank = sorted(plans, key=lambda p: -sum(share[p]) / len(share[p]))
    print("   by mean share: " + ", ".join(f"{p} {sum(share[p]) / len(share[p]):.3f}" for p in rank))
    beat = {(a, b) for a in plans for b in plans if a != b and wins[(a, b)] > n[(a, b)] / 2}
    cyc = sorted({tuple(sorted(c)) for a, b, c in itertools.permutations(plans, 3)
                  if (a, b) in beat and (b, c) in beat and (c, a) in beat for c in [(a, b, c)]})
    print(f"   majority cycles of three: {len(cyc)} {cyc[:6]}")
    strong = [(a, b, wins[(a, b)]) for (a, b) in sorted(beat) if sign_p(wins[(a, b)], n[(a, b)]) < 0.05]
    print(f"   pairings won 7/8 or 8/8 (p < 0.05 unadjusted): {len(strong)} of {len(plans) * (len(plans) - 1) // 2}")
