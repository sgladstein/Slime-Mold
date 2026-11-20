"""Stage 2 table (2026-11-19): python3 pairs_table.py FILE

For every deck, per planet, the median over seeds of the slime removed
while firing (start_mould - mould_fire) against the empty deck, and its
gain over the best single card it holds (the carrier, or the better of
two payloads), paired by seed, with the sign count. A gain over the best
part is the combination's own worth (the owner's rule: strong builds
should come from combinations)."""
import re, sys, statistics
from collections import defaultdict
rows = [dict(re.findall(r"(\w+)=(\S+)", l)) for l in open(sys.argv[1]) if l.startswith("gunlab")]
cut = defaultdict(dict); hurt = defaultdict(dict); unfit = set()
for r in rows:
    if r.get("how") in ("unfit", "error"):
        unfit.add(r["deck"]); continue
    cut[(r["depth"], r["deck"])][r["seed"]] = int(r["start_mould"]) - int(r["mould_fire"])
    hurt[(r["depth"], r["deck"])][r["seed"]] = float(r["hp_lost_fire"])
decks = sorted({d for _, d in cut})
def parts(d):
    return d.split("+")
def med(a, b, depth):
    x, y = cut.get((depth, a), {}), cut.get((depth, b), {})
    diffs = [x[s] - y[s] for s in x if s in y]
    if not diffs: return None
    return statistics.median(diffs), sum(v > 0 for v in diffs), sum(v < 0 for v in diffs), len(diffs)
out = []
for d in decks:
    if d == "none": continue
    total = 0; cells = []
    for depth in ("1", "3", "4"):
        v = med(d, "none", depth)
        singles = [p for p in parts(d) if (depth, p) in cut and p != d]
        best = None
        for p in singles:
            m = med(d, p, depth)
            if m and (best is None or m[0] < best[1][0]): best = (p, m)
        if v is None: cells.append("-"); continue
        total += v[0]
        g = f" vs {best[0]} {best[1][0]:+.0f} ({best[1][1]}/{best[1][3]})" if best else ""
        cells.append(f"{v[0]:+.0f} ({v[1]}/{v[3]}){g}")
    out.append((total, d, cells))
print("| deck | planet 1 | planet 3 | planet 4 |\n|---|---|---|---|")
for t, d, c in sorted(out, reverse=True):
    print(f"| {d} | " + " | ".join(c) + " |")
print("\nunfit:", ", ".join(sorted(unfit)))
