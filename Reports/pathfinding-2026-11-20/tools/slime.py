#!/usr/bin/env python3
"""What sent squads walk through on the way (pathfinding thread, 2026-11-20).

    python3 slime.py LOG.csv [--tps 15] [--near 10]

LOG.csv is `Game::path_rows` (examples/pathprobe or LARDER_PATHLOG). For
each squad on a plain move (a destination, not an attack-move, not
steered), while its anchor is over `near` dish cells from the destination
(on the way, not at the crystal it was sent to):
- touch: share of member-steps with walkers on the soldier
- hurt: health its members lost, per member and trip
- fell: members that died on the way
- secs: seconds a finished trip took, median
Writes LOG.csv.slime.json.
"""
import json
import math
import sys
from collections import defaultdict


def main():
    a = sys.argv[1:]
    path = a[0]
    tps, near = 15, 10.0
    for i, x in enumerate(a):
        if x == "--tps":
            tps = int(a[i + 1])
        if x == "--near":
            near = float(a[i + 1])
    sq = {}  # (t, q) -> (ax, ay, dx, dy, steered, fight, holding)
    mem = defaultdict(list)  # (t, q) -> [(k, touching, hp)]
    for line in open(path):
        p = line.rstrip("\n").split(",")
        if p[0] == "s" and len(p) >= 14 and "" not in p:
            mem[(int(p[1]), int(p[3]))].append((int(p[2]), int(p[10]), float(p[13])))
        elif p[0] == "q" and len(p) >= 12 and "" not in p:
            ax, ay, dx, dy, sx, sy = map(float, p[3:9])
            sq[(int(p[1]), int(p[2]))] = (ax, ay, dx, dy, (sx, sy) != (0.0, 0.0), p[11] == "1", p[9] == "1")
    steps = touched = 0
    hurt = 0.0
    trips = []
    last_hp = {}
    cur = {}
    seen = {}
    for (t, q) in sorted(sq, key=lambda k: (k[0], k[1])):
        ax, ay, dx, dy, steered, fight, hold = sq[(t, q)]
        plain = not math.isnan(dx) and not steered and not fight and not math.isnan(ax)
        on_way = plain and math.hypot(dx - ax, dy - ay) > near
        c = cur.get(q)
        if plain and (c is None or c["d"] != (dx, dy)):
            c = cur[q] = {"d": (dx, dy), "t0": t, "t1": t, "done": False, "hurt": 0.0, "n": 0}
            trips.append(c)
        elif not plain and c is not None:
            c["done"] = math.isnan(dx)
            cur.pop(q)
            c = None
        if c is not None:
            c["t1"] = t
        here = mem.get((t, q), [])
        alive_now = [k for k, _, _ in here]
        for k, touching, hp in here:
            if on_way:
                steps += 1
                touched += touching > 0
                lost = max(0.0, last_hp.get(k, hp) - hp)
                hurt += lost
                if c is not None:
                    c["hurt"] += lost
            last_hp[k] = hp
        for k in alive_now:
            seen[k] = (t, on_way)
        if c is not None and on_way:
            c["n"] = max(c["n"], len(here))
    t_end = max((t for t, _ in sq), default=0)
    fell = sum(1 for t, w in seen.values() if w and t < t_end)
    done = sorted((c["t1"] - c["t0"]) / tps for c in trips if c["done"] and c["n"])
    per = [c["hurt"] / c["n"] for c in trips if c["n"]]
    out = {
        "touch": touched / max(steps, 1),
        "hurt": sum(per) / max(len(per), 1),
        "fell": fell,
        "secs": done[len(done) // 2] if done else 0.0,
        "trips": len(per),
        "steps": steps,
    }
    with open(path + ".slime.json", "w") as f:
        json.dump(out, f)
    print(f"{path}: {out['trips']} move trips, {steps} member-steps on the way: walkers on {out['touch'] * 100:.1f}%, health lost {out['hurt']:.1f} a member a trip, {fell} fell, trip median {out['secs']:.1f} s")


if __name__ == "__main__":
    main()
