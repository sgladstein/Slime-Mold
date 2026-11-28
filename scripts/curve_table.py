#!/usr/bin/env python3
"""The run bot's curve by step and pip (Milestone 2 test 3; Engine and
tools, 2026-11-28): reads rows from scripts/pc-curve.ps1 (`CURVE arm=...
step=... seed=... pip=... wall=... league ...`) or the curve workflow, and
prints a Markdown table a arm: for each step and pip, the games, the wins,
the median time to win, the median peak squad, the median income and the
median soldiers lost, then the checks Direction's page states: no step a
wall for every arm (no win in any game), later steps costing more than
earlier ones, Hard more than Fair and Fair more than Safe (paired by step
and seed, time to win with a loss costlier than any win; sign test).

    python3 scripts/curve_table.py curve-*.txt
"""

import re
import sys
from math import comb

PIPS = ("safe", "fair", "hard")


def rows(paths):
    out = []
    for path in paths:
        with open(path) as f:
            for line in f:
                line = line[line.find("CURVE "):] if "CURVE " in line else line
                if not line.startswith("CURVE ") or " league " not in line:
                    continue
                # The league line may sit after other output on the row (the
                # first PC run wrote "failed <game output> league ..."): the
                # row's own keys from its head, the game's from its league
                # line, which win (its `pip=` is the planet played).
                head, _, league = line.partition(" league ")
                kv = dict(re.findall(r"(\w+)=(\S+)", head.split(" wall=")[0]))
                kv.update(dict(re.findall(r"(\w+)=(\S+)", league)))
                if "how" in kv:
                    out.append(kv)
    return out


def median(xs):
    xs = sorted(xs)
    if not xs:
        return None
    n = len(xs)
    return xs[n // 2] if n % 2 else (xs[n // 2 - 1] + xs[n // 2]) / 2


def clock(s):
    return "-" if s is None else f"{int(s) // 60}:{int(s) % 60:02d}"


def num(x):
    return "-" if x is None else f"{x:.0f}"


def cost(g):
    """Time to win, a loss costlier than any win."""
    won = g["how"] == "won"
    return (0, float(g["secs"])) if won else (1, -float(g["secs"]))


def sign_p(k, n):
    return sum(comb(n, i) for i in range(k, n + 1)) / 2 ** n if n else 1.0


def main(paths):
    games = rows(paths)
    if not games:
        print("no CURVE rows")
        return 1
    arms = sorted({g["arm"] for g in games})
    for arm in arms:
        mine = [g for g in games if g["arm"] == arm]
        print(f"### Arm `{arm}`")
        print()
        print("| Step | Pip | Tier | Games | Won | Median time to win | Median peak squad | Median income | Median soldiers lost |")
        print("|---|---|---|---|---|---|---|---|---|")
        for step in sorted({int(g["step"]) for g in mine}):
            for pip in PIPS:
                here = [g for g in mine if int(g["step"]) == step and g["pip"] == pip]
                if not here:
                    continue
                won = [g for g in here if g["how"] == "won"]
                tier = "/".join(sorted({g["tier"] for g in here}))
                print(f"| {step} | {pip} | {tier} | {len(here)} | {len(won)} | {clock(median([float(g['secs']) for g in won]))} "
                      f"| {num(median([float(g['peak_alive']) for g in here]))} | {num(median([float(g['income']) for g in here]))} "
                      f"| {num(median([float(g['fell_end']) for g in here]))} |")
        print()
        walls = [s for s in sorted({int(g["step"]) for g in mine}) if not any(g["how"] == "won" for g in mine if int(g["step"]) == s)]
        if walls:
            print(f"**No win at step {', '.join(map(str, walls))} for this arm.**")
            print()
        by = {(int(g["step"]), g["seed"], g["pip"]): g for g in mine}
        for a, b in (("safe", "fair"), ("fair", "hard")):
            pairs = [(by[(s, d, a)], by[(s, d, b)]) for (s, d, p) in by if p == a and (s, d, b) in by]
            up = sum(cost(y) > cost(x) for x, y in pairs)
            down = sum(cost(y) < cost(x) for x, y in pairs)
            print(f"- {b} costs more than {a}: {up} pairs, less: {down}, tied {len(pairs) - up - down}; sign p {sign_p(up, up + down):.3f}")
        steps = sorted({int(g["step"]) for g in mine})
        for pip in PIPS:
            pairs = [(by[(s, d, pip)], by[(t, d, pip)]) for s, t in zip(steps, steps[1:]) for (x, d, p) in by if x == s and p == pip and (t, d, pip) in by]
            up = sum(cost(y) > cost(x) for x, y in pairs)
            down = sum(cost(y) < cost(x) for x, y in pairs)
            if pairs:
                print(f"- {pip}, each step against the one before: costs more {up}, less {down}, tied {len(pairs) - up - down}; sign p {sign_p(up, up + down):.3f}")
        print()
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
