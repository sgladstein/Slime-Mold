"""The buildings gate over several census files of one world (the standing
gate's blocks, run by runners on slices of seeds): mean score per hand,
random's mould wipes, how often each plan is the best on a seed, every plan
against idle and random, and the building hands against army (the
spend-everything control), sign tests with Holm over all of them.
Usage: python3 gate.py NAME census-a.txt census-b.txt ..."""
import re, sys
from math import comb

PLANS = ["farm", "ring", "turtle", "killzone", "guard", "pumps", "army"]
BUILD = ["ring", "turtle", "killzone", "guard", "pumps"]


def sign_p(w, l):
    n = w + l
    if n == 0:
        return 1.0
    return min(1.0, 2 * sum(comb(n, i) for i in range(min(w, l) + 1)) / 2 ** n)


name, paths = sys.argv[1], sys.argv[2:]
rows, ends, prov = {}, {}, []
for path in paths:
    for line in open(path):
        if line.startswith("provenance:"):
            prov.append(line.strip())
        m = re.match(r"\s*(\w+) seed\s+(\d+):\s*(.*?) at .*score ([+-][\d.]+);", line)
        if m:
            rows[(m[1], int(m[2]))] = float(m[4])
            ends[(m[1], int(m[2]))] = m[3].strip()
hands = [h for h in ["idle", "random"] + PLANS if any(k[0] == h for k in rows)]
seeds = sorted(s for s in {s for _, s in rows} if all((h, s) in rows for h in hands))
print(f"== {name}: {len(seeds)} seeds ({seeds[0]}-{seeds[-1]}) from {len(paths)} files")
for p in prov:
    print("  " + p)
print("  mean: " + ", ".join(f"{h} {sum(rows[(h, s)] for s in seeds) / len(seeds):+.3f}" for h in hands))
wiped = sum(1 for s in seeds if ends[("random", s)].startswith("mould wiped"))
print(f"  random wipes the mould on {wiped} of {len(seeds)}")
best = {h: 0 for h in PLANS}
for s in seeds:
    sc = {h: rows[(h, s)] for h in PLANS if (h, s) in rows}
    top = max(sc.values())
    winners = [h for h, v in sc.items() if v == top]
    if len(winners) == 1:
        best[winners[0]] += 1
print("  best plan on a seed (ties dropped): " + ", ".join(f"{h} {n}" for h, n in best.items()))
tests = []
for h in PLANS:
    for base in ["idle", "random"]:
        tests.append((h, base))
for h in BUILD + ["farm"]:
    tests.append((h, "army"))
res = []
for a, b in tests:
    w = sum(1 for s in seeds if rows[(a, s)] > rows[(b, s)])
    l = sum(1 for s in seeds if rows[(a, s)] < rows[(b, s)])
    res.append((a, b, w, l, sign_p(w, l)))
order = sorted(range(len(res)), key=lambda i: res[i][4])
holds = [False] * len(res)
for rank, i in enumerate(order):
    if res[i][4] * (len(res) - rank) <= 0.05:
        holds[i] = True
    else:
        break
print(f"  paired by seed, sign test, Holm over {len(res)}:")
for i, (a, b, w, l, p) in enumerate(res):
    print(f"    {a:>8} vs {b:<6}: {w:2} higher, {l:2} lower, p {p:.4f}{', holds' if holds[i] else ''}")
