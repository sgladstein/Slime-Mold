"""Payoff table from larderworld census files: mean score per hand, how
often each plan is the best plan on a seed, and each building hand paired
with farm (sign test). Usage: python3 table.py census-*.txt"""
import re, sys
from math import comb

PLANS = ["farm", "ring", "turtle", "killzone", "guard", "army", "pumps"]

def sign_p(w, l):
    n = w + l
    if n == 0:
        return 1.0
    k = min(w, l)
    return min(1.0, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n)

for path in sys.argv[1:]:
    rows = {}
    for line in open(path):
        m = re.match(r"\s*(\w+) seed\s+(\d+):.*score ([+-][\d.]+);.*killed\s+(\d+)", line)
        if m:
            rows[(m[1], int(m[2]))] = (float(m[3]), int(m[4]))
            continue
        # The runners' compact form: hand seed score.
        m = re.match(r"\s*([a-z]+) (\d+) ([+-][\d.]+)\s*$", line)
        if m:
            rows[(m[1], int(m[2]))] = (float(m[3]), 0)
    head = next((l.strip() for l in open(path) if l.startswith("larderworld")), path)
    # Only seeds every hand finished (a runner can stop short).
    allh = {h for h, _ in rows}
    seeds = sorted(s for s in {s for _, s in rows} if all((h, s) in rows for h in allh))
    hands = sorted({h for h, _ in rows}, key=lambda h: (["idle", "random"] + PLANS).index(h) if h in ["idle", "random"] + PLANS else 99)
    print(f"== {path}: {head[:80]}")
    print("  mean: " + ", ".join(f"{h} {sum(rows[(h, s)][0] for s in seeds) / len(seeds):+.3f}" for h in hands if all((h, s) in rows for s in seeds)))
    best = {h: 0 for h in PLANS}
    for s in seeds:
        sc = {h: rows[(h, s)][0] for h in PLANS if (h, s) in rows}
        if sc:
            best[max(sc, key=sc.get)] += 1
    print("  best plan on a seed: " + ", ".join(f"{h} {n}" for h, n in best.items()))
    for h in PLANS[1:]:
        w = sum(rows[(h, s)][0] > rows[("farm", s)][0] for s in seeds if (h, s) in rows and ("farm", s) in rows)
        l = sum(rows[(h, s)][0] < rows[("farm", s)][0] for s in seeds if (h, s) in rows and ("farm", s) in rows)
        print(f"  {h} vs farm: {w} higher, {l} lower, p {sign_p(w, l):.4f}")
