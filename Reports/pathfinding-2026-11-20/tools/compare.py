#!/usr/bin/env python3
"""Paired comparison of two batch.sh folders: compare.py BEFORE AFTER.

Runs paths.py on every log (writing LOG.csv.json), then for each measure
the mean before and after, how many maps went down, up or stayed, and a
two-sided sign-test p over the maps that changed. Also counts the steered
squad's arrivals (`reached` lines in the .log)."""
import glob
import json
import math
import os
import subprocess
import sys

here = os.path.dirname(os.path.abspath(__file__))


def load(d):
    out = {}
    for c in sorted(glob.glob(os.path.join(d, "*.csv"))):
        name = os.path.basename(c)[:-4]
        if not os.path.exists(c + ".json") or os.path.getmtime(c + ".json") < os.path.getmtime(c):
            subprocess.run([sys.executable, os.path.join(here, "paths.py"), c], stdout=subprocess.DEVNULL, check=True)
        j = json.load(open(c + ".json"))
        log = c[:-4] + ".log"
        j["reached"] = sum(1 for l in open(log) if "reached" in l) if os.path.exists(log) else 0
        out[name] = j
    return out


def sign_p(up, down):
    n = up + down
    if n == 0:
        return 1.0
    k = min(up, down)
    p = sum(math.comb(n, i) for i in range(k + 1)) / 2 ** n
    return min(1.0, 2 * p)


def main():
    a, b = load(sys.argv[1]), load(sys.argv[2])
    names = sorted(set(a) & set(b))
    print(f"{len(names)} maps paired")
    keys = ["unplaced", "strays", "dither", "strung2x", "stalls", "unfinished", "ratio", "pace", "reached"]
    print(f"{'measure':<11} {'before':>8} {'after':>8} {'lower':>6} {'higher':>6} {'same':>5} {'sign p':>7}")
    for k in keys:
        x = [a[n][k] for n in names]
        y = [b[n][k] for n in names]
        lo = sum(1 for i, j in zip(x, y) if j < i - 1e-9)
        hi = sum(1 for i, j in zip(x, y) if j > i + 1e-9)
        print(f"{k:<11} {sum(x) / len(x):8.4f} {sum(y) / len(y):8.4f} {lo:6d} {hi:6d} {len(x) - lo - hi:5d} {sign_p(lo, hi):7.3f}")
    print()
    for n in names:
        print(f"  {n:<16} " + "  ".join(f"{k} {a[n][k]:.3g}->{b[n][k]:.3g}" for k in ["unplaced", "strays", "dither", "stalls", "reached"]))


if __name__ == "__main__":
    main()
