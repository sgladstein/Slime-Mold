#!/usr/bin/env python3
"""Tables for the scaling-rule census (run.sh): each arm's medians, and
each 4x-finer arm paired against today on the same seed with a two-sided
sign test, Bonferroni over every comparison printed.

    python3 analyze.py raw > tables.txt
"""
import math
import os
import re
import statistics
import sys

ARMS = ["today", "naive", "steps", "smell", "reach", "turn", "nose"]
LINE = re.compile(
    r"seed (\d+) (\S+) t\s+([\d.]+)s\s+tube\s+([\d.]+)\s+loops\s+(\d+)\s+width ([\d.]+)\s+"
    r"reach\s+([\d.]+)\s+piles (\d+)/(\d+)\s+strays ([\d.]+)\s+ms/tick ([\d.]+)"
)

# (start, time, measure, label, higher is "more")
DECIDING = [
    ("blob", 20, "piles", "piles reached at 20 s (blob)"),
    ("blob", 5, "reach", "reach at 5 s, dish cells (blob)"),
    ("uniform", 20, "loops", "loops at 20 s (uniform)"),
    ("uniform", 20, "width", "tube width at 20 s, dish cells (uniform)"),
    ("blob", 20, "strays", "strays at 20 s (blob)"),
    ("uniform", 20, "strays", "strays at 20 s (uniform)"),
]
DESCRIBE = [
    ("blob", 20, "tube"),
    ("blob", 20, "loops"),
    ("blob", 20, "width"),
    ("blob", 20, "reach"),
    ("uniform", 20, "tube"),
    ("uniform", 20, "piles"),
    ("uniform", 5, "reach"),
]


def load(folder):
    runs = {}
    for name in sorted(os.listdir(folder)):
        m = re.fullmatch(r"(\w+)-(blob|uniform)-s(\d+)\.txt", name)
        if not m:
            continue
        arm, start, seed = m.group(1), m.group(2), int(m.group(3))
        rows = {}
        hash_ = None
        for line in open(os.path.join(folder, name)):
            g = LINE.search(line)
            if g:
                t = round(float(g.group(3)))
                rows[t] = {
                    "tube": float(g.group(4)),
                    "loops": int(g.group(5)),
                    "width": float(g.group(6)),
                    "reach": float(g.group(7)),
                    "piles": int(g.group(8)),
                    "of": int(g.group(9)),
                    "strays": float(g.group(10)),
                    "ms": float(g.group(11)),
                }
            if "hash=" in line:
                hash_ = line.split("hash=")[1].strip()
        if hash_ is None:
            continue  # unfinished
        runs[(arm, start, seed)] = rows
    return runs


def sign_p(up, down):
    n = up + down
    if n == 0:
        return 1.0
    k = min(up, down)
    tail = sum(math.comb(n, i) for i in range(k + 1)) / 2**n
    return min(1.0, 2 * tail)


def fmt(v):
    return f"{v:.3f}" if isinstance(v, float) and v < 1 else (f"{v:.1f}" if isinstance(v, float) else str(v))


def main():
    folder = sys.argv[1] if len(sys.argv) > 1 else "raw"
    runs = load(folder)
    seeds = sorted({s for (_, _, s) in runs})
    print(f"Scaling-rule census: {len(runs)} runs, seeds {seeds[0]}-{seeds[-1]} ({len(seeds)})")
    print()
    print("Medians (low-high over seeds). Per-arm cost: median ms a tick, one thread,")
    print("four runs at once on four cores (contended; see cost.txt for clean timings).")
    print()
    cols = DECIDING + [(s, t, m, f"{m} at {t} s ({s})") for (s, t, m) in DESCRIBE]
    for start in ["blob", "uniform"]:
        print(f"== {start} start")
        for arm in ARMS:
            ss = [s for s in seeds if (arm, start, s) in runs]
            if not ss:
                continue
            parts = []
            for (st, t, m, label) in cols:
                if st != start:
                    continue
                vals = [runs[(arm, start, s)][t][m] for s in ss]
                parts.append(f"{m}@{t}s {fmt(statistics.median(vals))} ({fmt(min(vals))}-{fmt(max(vals))})")
            ms = statistics.median(runs[(arm, start, s)][20]["ms"] for s in ss)
            print(f"  {arm:6} n={len(ss):2}  " + "  ".join(parts) + f"  ms/tick {ms:.2f}")
        print()
    comparisons = [(arm, d) for arm in ARMS[1:] for d in DECIDING]
    alpha = 0.05 / len(comparisons)
    print(f"Paired against today, same seed: arm above / below / tied, two-sided sign test.")
    print(f"Bonferroni over {len(comparisons)} comparisons: survives if p < {alpha:.5f} (marked *).")
    print()
    for (start, t, m, label) in DECIDING:
        print(f"== {label}")
        for arm in ARMS[1:]:
            ss = [s for s in seeds if (arm, start, s) in runs and ("today", start, s) in runs]
            up = sum(runs[(arm, start, s)][t][m] > runs[("today", start, s)][t][m] for s in ss)
            down = sum(runs[(arm, start, s)][t][m] < runs[("today", start, s)][t][m] for s in ss)
            tie = len(ss) - up - down
            p = sign_p(up, down)
            mark = "*" if p < alpha else " "
            print(f"  {arm:6} above {up:2} below {down:2} tied {tie:2}  p {p:.5f} {mark}")
        print()
    # The follow-up arm against the arm it changes, its own correction.
    alpha2 = 0.05 / len(DECIDING)
    print("Follow-up: nose paired against naive (the one change: scent smelled 36 fine cells out).")
    print(f"Bonferroni over {len(DECIDING)} comparisons: survives if p < {alpha2:.5f} (marked *).")
    for (start, t, m, label) in DECIDING:
        ss = [s for s in seeds if ("nose", start, s) in runs and ("naive", start, s) in runs]
        up = sum(runs[("nose", start, s)][t][m] > runs[("naive", start, s)][t][m] for s in ss)
        down = sum(runs[("nose", start, s)][t][m] < runs[("naive", start, s)][t][m] for s in ss)
        p = sign_p(up, down)
        mark = "*" if p < alpha2 else " "
        print(f"  {label:42} above {up:2} below {down:2} tied {len(ss) - up - down:2}  p {p:.5f} {mark}")


if __name__ == "__main__":
    main()
