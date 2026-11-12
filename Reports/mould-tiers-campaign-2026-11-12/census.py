"""The tier census (census.sh's C1-C3) from the runners' .rows files.
   python3 Reports/mould-tiers-campaign-2026-11-12/census.py DIR
Each row: hand seed score end... time soldiers mould (end may be two words)."""
import sys, glob, os, re
from math import comb
from collections import defaultdict
d = sys.argv[1]
def sign_p(w, n):
    if n == 0: return 1.0
    k = min(w, n - w)
    return min(1.0, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n)
games = {}  # (tier, hand, seed) -> (score, end, t, soldiers, mould)
for f in glob.glob(os.path.join(d, "*.rows")):
    tier = os.path.basename(f).split("-")[0]
    for line in open(f):
        p = line.split()
        if len(p) < 7: continue
        hand, seed, score = p[0], int(p[1]), float(p[2])
        t, sol, mould = float(p[-3]), int(p[-2]), int(p[-1])
        games[(tier, hand, seed)] = (score, " ".join(p[3:-3]), t, sol, mould)
tiers = [t for t in ["none", "drifter", "sprawler", "trunk", "wary", "grudge"] if any(k[0] == t for k in games)]
hands = []
for k in games:
    if k[1] not in hands: hands.append(k[1])
order = ["idle", "random", "farm", "wall", "boom", "claim", "fort", "strike"]
hands = [h for h in order if h in hands] + [h for h in hands if h not in order]
def seeds(t, h): return sorted(s for (a, b, s) in games if a == t and b == h)
tests = []
def pair(name, xs):
    w = sum(1 for a, b in xs if a > b); l = sum(1 for a, b in xs if a < b)
    tests.append([name, w, l, sign_p(w, w + l), False])
print("payoff table: mean score (mould wiped / cap / squad wiped / overrun), seeds per cell")
print("        " + "".join(f"{t:>26}" for t in tiers))
for h in hands:
    row = f"{h:>8}"
    for t in tiers:
        ss = seeds(t, h)
        if not ss: row += f"{'-':>26}"; continue
        sc = [games[(t, h, s)][0] for s in ss]; ends = [games[(t, h, s)][1] for s in ss]
        e = "/".join(str(sum(1 for x in ends if x.startswith(k))) for k in ["mould", "cap", "squad", "overrun"])
        row += f"{sum(sc)/len(sc):>+10.3f} ({e}) n{len(ss):<3}"
    print(row)
print("\nbest plan per tier (mean), and the hand best on the most seeds:")
for t in tiers:
    plans = [h for h in hands if h not in ("idle", "random") and seeds(t, h)]
    best = max(plans, key=lambda h: sum(games[(t, h, s)][0] for s in seeds(t, h)) / len(seeds(t, h)))
    wins = defaultdict(int)
    for s in seeds(t, "farm"):
        sc = {h: games[(t, h, s)][0] for h in plans if (t, h, s) in games}
        wins[max(sc, key=sc.get)] += 1
    print(f"  {t:>9}: best by mean {best}; best per seed {dict(sorted(wins.items(), key=lambda x: -x[1]))}")
    # C1 gate
    for h in plans:
        for base in ("idle", "random"):
            ss = [s for s in seeds(t, h) if (t, base, s) in games]
            pair(f"C1 {t} {h} vs {base}", [(games[(t, h, s)][0], games[(t, base, s)][0]) for s in ss])
    # each plan against the tier's best
    for h in plans:
        if h == best: continue
        ss = [s for s in seeds(t, h) if (t, best, s) in games]
        pair(f"C2 {t} {h} vs best {best}", [(games[(t, h, s)][0], games[(t, best, s)][0]) for s in ss])
print("\nC3, against the plain mould (paired by seed):")
for t in tiers:
    if t == "none": continue
    ss = [s for s in seeds(t, "idle") if ("none", "idle", s) in games]
    a = [games[(t, "idle", s)] for s in ss]; b = [games[("none", "idle", s)] for s in ss]
    print(f"  {t:>9} idle: ends {[x[1] for x in a].count('squad wiped')} squad wiped of {len(a)} (plain {[x[1] for x in b].count('squad wiped')}); end time median {sorted(x[2] for x in a)[len(a)//2]:.0f} s (plain {sorted(x[2] for x in b)[len(b)//2]:.0f}); mould at the end median {sorted(x[4] for x in a)[len(a)//2]} (plain {sorted(x[4] for x in b)[len(b)//2]})")
    pair(f"C3 {t} idle end time vs plain (later = easier)", [(x[2], y[2]) for x, y in zip(a, b)])
    for h in [h for h in hands if h not in ("idle",)]:
        ss2 = [s for s in seeds(t, h) if ("none", h, s) in games]
        if ss2: pair(f"C3 {t} {h} score vs plain", [(games[(t, h, s)][0], games[("none", h, s)][0]) for s in ss2])
m = len(tests); idx = sorted(range(m), key=lambda i: tests[i][3])
for r, i in enumerate(idx):
    if tests[i][3] * (m - r) <= 0.05: tests[i][4] = True
    else: break
print(f"\npaired tests (higher / lower; sign test; Holm over all {m}):")
for name, w, l, p, h in tests:
    print(f"  {name}: {w} / {l}, p {p:.4f}{' holds' if h else ''}")
