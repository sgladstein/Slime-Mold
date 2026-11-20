"""Tables from gunlab lines (2026-11-19): python3 table.py FILE...

Per planet and deck, each measure paired by seed against the empty deck
(plain bullets): the median difference and the sign count (better/worse)
with a two-sided sign-test p. Measures (higher is better unless noted):
  cut   slime removed by the end of the fire (start_mould - mould_fire)
  cutT  slime removed after the tail (start_mould - mould): slow killers
  area  mean slime during the fire, lower is better (reported as -)
  lost  soldiers lost by the end of the fire, lower is better
  hurt  health lost by the end of the fire (soldiers' worth), lower is better
  best  kills in the best 10 s
  rng   mean range of a gun kill, dish cells
"""
import sys, re, statistics
from math import comb
from collections import defaultdict

def parse(paths):
    rows = []
    for p in paths:
        for line in open(p):
            if not line.startswith("gunlab "):
                continue
            kv = dict(re.findall(r"(\w+)=(\S+)", line))
            rows.append(kv)
    return rows

def sign_p(w, l):
    n = w + l
    if n == 0:
        return 1.0
    k = min(w, l)
    return min(1.0, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n)

def measures(r):
    start = int(r["start_mould"]); n = int(r["arena"]) or int(r["peak"])
    return {
        "cut": start - int(r["mould_fire"]),
        "cutT": start - int(r["mould"]),
        "area": -float(r["mould_mean"]),
        "lost": -(n - int(r["alive_fire"])),
        "hurt": -float(r["hp_lost_fire"]),
        "best": int(r["best10"]),
        "rng": float(r["kill_range"]),
    }

def main():
    rows = [r for r in parse(sys.argv[1:]) if r.get("how") not in ("unfit", "error")]
    unfit = sorted({(r["depth"], r["deck"]) for r in parse(sys.argv[1:]) if r.get("how") == "unfit"})
    by = defaultdict(dict)
    for r in rows:
        by[(r["depth"], r["deck"])][r["seed"]] = measures(r)
    keys = ["cut", "cutT", "area", "lost", "hurt", "best", "rng"]
    for depth in sorted({d for d, _ in by}):
        base = by.get((depth, "none"), {})
        print(f"\n## Planet {depth} (paired with the empty deck over {len(base)} seeds)\n")
        print("| deck | " + " | ".join(f"{k} med diff (+/-)" for k in keys) + " |")
        print("|---|" + "---|" * len(keys))
        lines = []
        for (d, deck), seeds in by.items():
            if d != depth or deck == "none":
                continue
            cells = []
            score = None
            for k in keys:
                diffs = [seeds[s][k] - base[s][k] for s in seeds if s in base]
                if not diffs:
                    cells.append("-"); continue
                w = sum(x > 0 for x in diffs); l = sum(x < 0 for x in diffs)
                med = statistics.median(diffs)
                if k == "cut":
                    score = med
                cells.append(f"{med:+.0f} ({w}/{l}, p {sign_p(w, l):.2f})" if abs(med) >= 10 or k in ("cut", "cutT", "area") else f"{med:+.2f} ({w}/{l})")
            lines.append((score if score is not None else -1e9, deck, cells))
        for _, deck, cells in sorted(lines, reverse=True):
            print(f"| {deck} | " + " | ".join(cells) + " |")
    if unfit:
        print("\nUnfit decks (over budget, slots or a repeated card):", ", ".join(f"p{d} {k}" for d, k in unfit))

main()
