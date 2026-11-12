#!/usr/bin/env python3
"""Merge `larderworld census gpu=both` slices (one file per seed range) and
redo the census's comparisons over all their seeds.

    python3 merge.py raw/census-*.txt

Per engine: each hand's mean score and ends, and each plan against idle,
random and farm paired by seed (two-sided sign test, Holm), as the census
prints them. Across engines, by hand: the median CPU and GPU scores, the
median paired difference, a two-sided Mann-Whitney test (normal
approximation with ties and continuity, as `mann_whitney_p` in
examples/larderworld.rs), Holm over the hands; and the mould at the end,
GPU against CPU by seed (sign test), which the first look flagged.
"""
import math
import re
import sys

ROW = re.compile(r"^\s*(\S+) seed\s+(\d+) the (CPU|GPU engine):\s+(.+?) at\s+(\d+) s, score ([+-][\d.]+); soldiers\s+(\d+), mould\s+(\d+)")
SCRIPTED = {"farm", "rush", "wall"}


def sign_p(w, n):
    if n == 0:
        return 1.0
    k = min(w, n - w)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(k + 1)) / 2 ** n)


def mann_whitney_p(a, b):
    n1, n2 = len(a), len(b)
    allv = sorted([(x, 0) for x in a] + [(x, 1) for x in b])
    n = len(allv)
    r1, ties, i = 0.0, 0.0, 0
    while i < n:
        j = i
        while j + 1 < n and allv[j + 1][0] == allv[i][0]:
            j += 1
        rank = (i + j) / 2 + 1
        t = j - i + 1
        ties += t ** 3 - t
        r1 += sum(1 for x in allv[i:j + 1] if x[1] == 0) * rank
        i = j + 1
    u = r1 - n1 * (n1 + 1) / 2
    nn = n1 + n2
    var = n1 * n2 / 12 * ((nn + 1) - ties / (nn * (nn - 1)))
    if var <= 0:
        return 1.0
    z = max(abs(u - n1 * n2 / 2) - 0.5, 0) / math.sqrt(var)
    return min(1.0, 2 * (1 - 0.5 * (1 + math.erf(z / math.sqrt(2)))))


def holm(ps):
    order = sorted(range(len(ps)), key=lambda i: ps[i])
    holds = [False] * len(ps)
    for rank, i in enumerate(order):
        if ps[i] * (len(ps) - rank) <= 0.05:
            holds[i] = True
        else:
            break
    return holds


def median(v):
    v = sorted(v)
    n = len(v)
    return 0.0 if n == 0 else (v[n // 2] if n % 2 else (v[n // 2 - 1] + v[n // 2]) / 2)


rows = {}
for path in sys.argv[1:]:
    for line in open(path):
        m = ROW.match(line)
        if m:
            hand, seed, eng, end, t, score, alive, mould = m.groups()
            rows[(hand, int(seed), eng == "GPU engine")] = (end.strip(), float(score), int(mould))
hands = sorted({k[0] for k in rows}, key=lambda h: ["idle", "random", "farm", "rush", "wall"].index(h) if h in ["idle", "random", "farm", "rush", "wall"] else 9)
seeds = sorted({k[1] for k in rows})
missing = [(h, s, g) for h in hands for s in seeds for g in (False, True) if (h, s, g) not in rows]
print(f"{len(rows)} games, seeds {seeds[0]}-{seeds[-1]} ({len(seeds)}), hands {', '.join(hands)}; missing {len(missing)}")
seeds = [s for s in seeds if all((h, s, g) in rows for h in hands for g in (False, True))]

for gpu in (False, True):
    print(f"\n==== on the {'GPU engine' if gpu else 'CPU'} ({len(seeds)} seeds) ====")
    for h in hands:
        r = [rows[(h, s, gpu)] for s in seeds]
        ends = {e: sum(1 for x in r if x[0] == e) for e in ("mould wiped", "cap", "squad wiped")}
        print(f"  {h:>6}: mean {sum(x[1] for x in r) / len(r):+.3f}; mould wiped {ends['mould wiped']}, cap {ends['cap']}, squad wiped {ends['squad wiped']}")
    tests = []
    for h in [h for h in hands if h not in ("idle", "random")]:
        for base in ("idle", "random", "farm"):
            if (base == "farm" and h in SCRIPTED) or base not in hands:
                continue
            w = sum(1 for s in seeds if rows[(h, s, gpu)][1] > rows[(base, s, gpu)][1])
            l = sum(1 for s in seeds if rows[(h, s, gpu)][1] < rows[(base, s, gpu)][1])
            tests.append((f"{h} vs {base}", w, l, sign_p(w, w + l)))
    hold = holm([t[3] for t in tests])
    print(f"  paired by seed (sign test, Holm over {len(tests)}):")
    for t, ok in zip(tests, hold):
        print(f"    {t[0]:>16}: {t[1]} higher, {t[2]} lower, p {t[3]:.4f}{', holds' if ok else ''}")

print(f"\nCPU against GPU by hand ({len(seeds)} seeds; Mann-Whitney two-sided, Holm over {len(hands)}):")
res = []
for h in hands:
    a = [rows[(h, s, False)][1] for s in seeds]
    b = [rows[(h, s, True)][1] for s in seeds]
    res.append((h, median(a), median(b), median([y - x for x, y in zip(a, b)]), mann_whitney_p(a, b)))
for r, ok in zip(res, holm([r[4] for r in res])):
    print(f"  {r[0]:>6}: CPU {r[1]:+.3f}, GPU {r[2]:+.3f}, difference {r[3]:+.3f}; p {r[4]:.4f}{', differs' if ok else ''}")
print("\nThe mould at the end, GPU against CPU by seed (sign test; median GPU / CPU):")
ps = []
for h in hands:
    pairs = [(rows[(h, s, False)][2], rows[(h, s, True)][2]) for s in seeds]
    up = sum(1 for c, g in pairs if g > c)
    down = sum(1 for c, g in pairs if g < c)
    ratio = median([g / c for c, g in pairs if c > 0])
    ps.append(sign_p(up, up + down))
    print(f"  {h:>6}: GPU bigger {up}, smaller {down}, p {ps[-1]:.4f}; median ratio {ratio:.3f}")
print(f"  Holm over {len(hands)}: " + ", ".join(f"{h} {'holds' if ok else 'no'}" for h, ok in zip(hands, holm(ps))))
