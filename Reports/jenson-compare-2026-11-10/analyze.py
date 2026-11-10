#!/usr/bin/env python3
"""Paired counts for run.sh: each pair on the same seed, two-sided sign
test, Bonferroni over the comparisons printed.

    python3 analyze.py raw > tables.txt
"""
import math
import os
import re
import statistics
import sys

LINE = re.compile(r"seed (\d+) (\S+) t\s+([\d.]+)s\s+tube\s+([\d.]+)\s+loops\s+(\d+)\s+width ([\d.]+)")
PAIRS = [
    ("one walker a cell (crowd 1) vs no limit (crowd 0), 100,000 walkers", "crowd1", "crowd0"),
    ("respawn 0 vs 0.01 a step, 1,000,000 walkers, no limit", "respawn0", "respawn0.01"),
]
MEASURES = [("loops", 1), ("width", 2), ("tube", 0)]


def load(folder):
    runs = {}
    for name in os.listdir(folder):
        m = re.fullmatch(r"(.+)-s(\d+)\.txt", name)
        if not m:
            continue
        for line in open(os.path.join(folder, name)):
            g = LINE.search(line)
            if g and round(float(g.group(3))) == 10:
                runs[(m.group(1), int(m.group(2)))] = {
                    "tube": float(g.group(4)), "loops": int(g.group(5)), "width": float(g.group(6))}
    return runs


def sign_p(up, down):
    n = up + down
    if n == 0:
        return 1.0
    k = min(up, down)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(k + 1)) / 2**n)


runs = load(sys.argv[1] if len(sys.argv) > 1 else "raw")
tests = len(PAIRS) * 2  # loops and width are the deciding measures
alpha = 0.05 / tests
print(f"At 10 s, seed 1 upward. Bonferroni over {tests} deciding comparisons (loops, width): p < {alpha:.4f} survives (*).")
for label, a, b in PAIRS:
    seeds = sorted(s for (arm, s) in runs if arm == a and (b, s) in runs)
    print(f"\n== {label}: {len(seeds)} seeds")
    if not seeds:
        continue
    for m, _ in MEASURES:
        va = [runs[(a, s)][m] for s in seeds]
        vb = [runs[(b, s)][m] for s in seeds]
        up = sum(y > x for x, y in zip(va, vb))
        down = sum(y < x for x, y in zip(va, vb))
        p = sign_p(up, down)
        mark = "*" if m != "tube" and p < alpha else " "
        print(f"  {m:6} {a} median {statistics.median(va):8.2f} ({min(va)}-{max(va)})  "
              f"{b} median {statistics.median(vb):8.2f} ({min(vb)}-{max(vb)})  "
              f"{b} higher on {up}, lower on {down}  p {p:.4f} {mark}")
