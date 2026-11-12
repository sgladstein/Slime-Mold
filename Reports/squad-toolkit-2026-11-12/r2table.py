#!/usr/bin/env python3
"""Round 2's 32-seed plans table from kitplans per-game lines.

Reads compact lines `tier hand seed score end...` (the runners' sed form) or
kitplans' own per-game lines, from the files given. Prints, per tier, each
hand's mean, median and ends, then gate 1 (each plan against idle, random,
randkit) and gate 2 (the best plan against each other plan), every pair as a
sign count with its two-sided p and the median paired difference, Holm over
all the table's pairs together (the tech lead, 2026-11-12).
"""
import re, sys, math, statistics
from collections import defaultdict

PLANS = ["farm", "loud", "quiet", "plague", "siege"]
CONTROLS = ["idle", "random", "randkit"]
full = re.compile(r"^ *([a-z]+) +([a-z]+) seed +([0-9]+): +([a-z ]+?) +at .*score ([-+0-9.]+);")
compact = re.compile(r"^([a-z]+) ([a-z]+) ([0-9]+) ([-+0-9.]+) (.+)$")

games = {}
for path in sys.argv[1:]:
    for line in open(path):
        m = full.match(line)
        if m:
            t, h, s, end, sc = m.groups()
        else:
            m = compact.match(line.strip())
            if not m:
                continue
            t, h, s, sc, end = m.groups()
        games[(t, h, int(s))] = (float(sc), end.strip())

def sign_p(w, n):
    if n == 0:
        return 1.0
    k = min(w, n - w)
    p = sum(math.comb(n, i) for i in range(k + 1)) / 2 ** n * 2
    return min(1.0, p)

tiers = sorted({t for t, _, _ in games}, key=lambda t: ["none", "swarmer", "sleeper", "sporer", "sprawler", "trunk", "drifter", "grudge"].index(t) if t in ["none", "swarmer", "sleeper", "sporer", "sprawler", "trunk", "drifter", "grudge"] else 99)
tests = []
for t in tiers:
    seeds = sorted({s for tt, h, s in games if tt == t and all((t, x, s) in games for x in PLANS + CONTROLS)})
    print(f"\n{t}: {len(seeds)} seeds ({seeds[0]}-{seeds[-1]})" if seeds else f"\n{t}: no complete seeds")
    if not seeds:
        continue
    sc = lambda h: [games[(t, h, s)][0] for s in seeds]
    for h in CONTROLS + PLANS:
        ends = [games[(t, h, s)][1] for s in seeds]
        w = sum(e.startswith("mould") for e in ends)
        c = sum(e.startswith("cap") for e in ends)
        q = sum(e.startswith("squad") for e in ends)
        print(f"  {h:>8} mean {statistics.mean(sc(h)):+.3f} median {statistics.median(sc(h)):+.3f} ({w}/{c}/{q})")
    def pair(a, b, kind):
        d = [x - y for x, y in zip(sc(a), sc(b))]
        w, l = sum(x > 0 for x in d), sum(x < 0 for x in d)
        tests.append((t, kind, a, b, w, l, sign_p(w, w + l), statistics.median(d)))
    for h in PLANS:
        for c in CONTROLS:
            pair(h, c, "gate 1")
    best = max(PLANS, key=lambda h: statistics.mean(sc(h)))
    for h in PLANS:
        if h != best:
            pair(best, h, f"gate 2 (best {best})")

m = len(tests)
order = sorted(range(m), key=lambda i: tests[i][6])
holds = [False] * m
for rank, i in enumerate(order):
    if tests[i][6] <= 0.05 / (m - rank):
        holds[i] = True
    else:
        break
print(f"\npairs (higher / lower, sign p, median paired difference; Holm over all {m}):")
for i, (t, kind, a, b, w, l, p, med) in enumerate(tests):
    print(f"  {t:>8} {kind:<22} {a} vs {b}: {w} / {l}, p {p:.2g}, med {med:+.3f}{'  holds' if holds[i] else ''}")
