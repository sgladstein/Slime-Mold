#!/usr/bin/env python3
"""Paired tables for the GPU plan's phase 1 arms.

    python3 Reports/gpu-plan-2026-11-10/census/analyze.py [ARM ...]

Reads raw/ARM-{puzzles,alive,wild}-SEED.txt (written by run.sh) and prints,
for each arm against today's rules, per dish and hand: stars per arm, the
dish's own measure per arm (median and range), and the arm against today
paired by seed (better / worse / same, two-sided sign-test p), then how many
seeds meet mouldprobe's ship bar under each. With no ARM, every arm found.

Deciding measure, named before each run: each dish's ship bar (doing nothing
0 stars, random at most 1, the plan hand its stars) holds as often under the
arm as under today's rules, and no hand's own measure moves on a paired sign
test that survives Bonferroni over that arm's comparisons. The arms:

- late: the census every 30 steps (4 a second), landing 4 steps (two
  frames) late (`Params::census_every`, `Params::census_lag`).
- together: walkers move all at once (`Params::all_at_once`).
- fixed: positions, headings, trail and scent rounded as an all-integer
  simulation rounds them (`Params::fixed_point`).
- gpu: all three at once, as the GPU design would run the dish.
"""

import glob
import math
import os
import re
import statistics
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RAW = os.path.join(HERE, "raw")
DISHES = {1: "Behind the Wall", 2: "Beat the Ruler", 3: "Running Dry", 4: "Keep It Alive", 5: "The Wild One"}
LINE = re.compile(r"^seed (\d+)\s+(\S+): (\d+) stars\s+(.*?)\s+\|")
DISH = re.compile(r"^== dish (\d+) ")
BAR = re.compile(r"^dish (\d+) .*-> (MEETS|FAILS) THE BAR")


def clock(s):
    m, sec = s.split(":")
    return int(m) * 60 + int(sec)


def measure(dish, amount, line):
    """The dish's own number, larger is better."""
    if dish == 1:  # reached the food sooner; not reached ranks below any reach,
        # strays-only above any miss, and a nearer miss above a farther one
        m = re.search(r"REACHED THE FOOD AT (\d+:\d+)", amount)
        if m:
            return -clock(m.group(1))
        if "ONLY STARVING STRAYS" in amount:
            return -1000
        m = re.search(r"WITHIN (\d+) CELLS", amount)
        return -1000 - int(m.group(1)) if m else -2000
    if dish == 2:  # foods joined, then a shorter network against the ruler
        m = re.search(r"ruler (\d+\.\d+)x", line)
        if m:
            return 10 - float(m.group(1))
        m = re.search(r"(\d) OF 3 FOODS JOINED", amount)
        return int(m.group(1)) if m else 0
    if dish == 3:  # mould alive at the bell; died earlier ranks lower
        m = re.search(r"(\d+) MOULD ALIVE AT THE BELL", amount)
        if m:
            return int(m.group(1))
        m = re.search(r"DIED AT (\d+:\d+)", amount)
        return clock(m.group(1)) - 1000 if m else -2000
    if dish == 4:  # seconds lived
        m = re.search(r"LIVED (\d+:\d+)", amount)
        return clock(m.group(1)) if m else 0
    if dish == 5:  # share of the food eaten
        m = re.search(r"share (\d\.\d+)", line)
        return float(m.group(1)) if m else 0.0
    raise ValueError(dish)


def show(dish, v):
    if dish == 1:
        if v > -1000:
            return f"{-v // 60:.0f}:{-v % 60:02.0f}"
        return "strays only" if v == -1000 else f"not ({-1000 - v:.0f} cells)"
    if dish == 2:
        return f"{10 - v:.2f}x" if v > 3 else f"{v:.0f}/3 joined"
    if dish == 3:
        return f"{v:.0f} alive" if v >= 0 else f"died {v + 1000:.0f}s"
    if dish == 4:
        return f"{v // 60:.0f}:{v % 60:02.0f}"
    return f"{v:.3f}"


def sign_p(w, l):
    n = w + l
    if n == 0:
        return 1.0
    k = min(w, l)
    p = sum(math.comb(n, i) for i in range(k + 1)) / 2**n * 2
    return min(1.0, p)


def read():
    runs = {}  # (arm, dish, hand, seed) -> (stars, measure, hash)
    bars = {}  # (arm, dish, seed) -> met the ship bar
    for path in glob.glob(os.path.join(RAW, "*.txt")):
        arm = os.path.basename(path).split("-")[0]
        seed = int(os.path.basename(path).rsplit("-", 1)[1].split(".")[0])
        dish = None
        for line in open(path):
            m = BAR.match(line)
            if m:
                bars[(arm, int(m.group(1)), seed)] = m.group(2) == "MEETS"
                continue
            m = DISH.match(line)
            if m:
                dish = int(m.group(1))
                continue
            m = LINE.match(line)
            if m and dish:
                seed, hand, stars, amount = int(m.group(1)), m.group(2), int(m.group(3)), m.group(4)
                h = re.search(r"hash=([0-9a-f]+)", line)
                runs[(arm, dish, hand, seed)] = (stars, measure(dish, amount, line), h.group(1) if h else "")
    return runs, bars


def main():
    runs, bars = read()
    if not runs:
        sys.exit("no runs in raw/")
    arms = sys.argv[1:] or sorted({k[0] for k in runs} - {"today"})
    for arm in arms:
        compare(runs, bars, arm)


def compare(runs, bars, arm):
    seeds = sorted({k[3] for k in runs if k[0] == arm})
    print(f"\n######## {arm} against today ({len(seeds)} seeds) ########")
    comparisons = []
    for dish in sorted({k[1] for k in runs}):
        hands = sorted({k[2] for k in runs if k[1] == dish}, key=lambda h: ["idle", "random", "plan", "fresh", "intended", "sleeper", "warden"].index(h))
        print(f"\n== dish {dish} {DISHES[dish]} ({len(seeds)} seeds) ==")
        print(f"{'hand':>9} | stars today | stars {arm} | measure today (median; worst to best) | measure {arm} | {arm} vs today: better-worse-same, p")
        for hand in hands:
            t = [runs.get(("today", dish, hand, s)) for s in seeds]
            l = [runs.get((arm, dish, hand, s)) for s in seeds]
            pairs = [(a, b) for a, b in zip(t, l) if a and b]
            if not pairs:
                continue
            st = sum(a[0] for a, _ in pairs)
            sl = sum(b[0] for _, b in pairs)
            mt = [a[1] for a, _ in pairs]
            ml = [b[1] for _, b in pairs]
            w = sum(b[1] > a[1] for a, b in pairs)
            lo = sum(b[1] < a[1] for a, b in pairs)
            same = len(pairs) - w - lo
            p = sign_p(w, lo)
            comparisons.append((dish, hand, w, lo, p))
            starsets = lambda xs: "".join(str(x[0]) for x in xs)
            print(
                f"{hand:>9} | {st:>3} ({starsets([a for a, _ in pairs])}) | {sl:>3} ({starsets([b for _, b in pairs])}) | "
                f"{show(dish, statistics.median(mt))} ({show(dish, min(mt))} to {show(dish, max(mt))}) | "
                f"{show(dish, statistics.median(ml))} ({show(dish, min(ml))} to {show(dish, max(ml))}) | {w}-{lo}-{same}, p {p:.3f}"
            )
        met = lambda a: sum(bars.get((a, dish, s), False) for s in seeds)
        print(f"{'ship bar':>9} | met on {met('today')} of {len(seeds)} seeds today, {met(arm)} under {arm}")
    k = len(comparisons)
    bar = 0.05 / k if k else 0.05
    print(f"\n{k} paired comparisons; Bonferroni bar p < {bar:.4f}")
    for dish, hand, w, lo, p in comparisons:
        if p < bar:
            print(f"  survives: dish {dish} {hand}: {w}-{lo}, p {p:.5f}")
    if not any(p < bar for *_, p in comparisons):
        print("  none survives")
    same = sum(1 for k, v in runs.items() if k[0] == arm and runs.get(("today",) + k[1:], (0, 0, ""))[2] == v[2])
    print(f"{same} of {sum(1 for k in runs if k[0] == arm)} runs end on the same hash= as today's")


if __name__ == "__main__":
    main()
