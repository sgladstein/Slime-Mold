#!/usr/bin/env python3
"""Fire on the way, paired (pathfinding thread, 2026-11-21).

    python3 fire.py BEFORE_DIR AFTER_DIR

Each DIR holds `examples/pathprobe ... fire=R` logs (`Game::path_rows`),
the same maps by name. Per map: health soldiers lost with no walker on them
(fire; the probe's slime is far) while their squad was on a plain move
over 10 cells from where it was sent (`burnt_way`), the same anywhere
(`burnt`), and the median seconds a finished move took (`secs`). Sign test
on maps lower against higher.
"""
import glob
import math
import os
import sys
from math import comb


def one(path, near=10.0, tps=15):
    sq = {}
    last = {}
    way = anywhere = 0.0
    trips = {}
    done = []
    rows = []
    for line in open(path):
        p = line.rstrip("\n").split(",")
        if "" in p:
            continue
        if p[0] == "q" and len(p) >= 12:
            t, q = int(p[1]), int(p[2])
            ax, ay, dx, dy, sx, sy = map(float, p[3:9])
            plain = not math.isnan(dx) and (sx, sy) == (0.0, 0.0) and p[11] == "0" and not math.isnan(ax)
            sq[(t, q)] = plain and math.hypot(dx - ax, dy - ay) > near
            cur = trips.get(q)
            if plain and (cur is None or cur[0] != (dx, dy)):
                trips[q] = ((dx, dy), t)
            elif math.isnan(dx) and cur is not None:
                done.append((t - cur[1]) / tps)
                trips.pop(q)
        elif p[0] == "s" and len(p) >= 14:
            rows.append((int(p[1]), int(p[2]), int(p[3]), int(p[10]), float(p[13])))
    for t, k, q, touching, hp in rows:
        if k in last and touching == 0 and last[k] > hp:
            anywhere += last[k] - hp
            if sq.get((t, q)):
                way += last[k] - hp
        last[k] = hp
    done.sort()
    return {"burnt_way": way, "burnt": anywhere, "secs": done[len(done) // 2] if done else 0.0, "moves": len(done)}


def sign(lo, hi):
    n = lo + hi
    if n == 0:
        return 1.0
    k = min(lo, hi)
    return min(1.0, 2 * sum(comb(n, i) for i in range(k + 1)) / 2**n)


def main():
    a, b = sys.argv[1], sys.argv[2]
    names = sorted(os.path.basename(p) for p in glob.glob(os.path.join(a, "*.csv")))
    names = [n for n in names if os.path.exists(os.path.join(b, n))]
    ra = {n: one(os.path.join(a, n)) for n in names}
    rb = {n: one(os.path.join(b, n)) for n in names}
    print(f"{len(names)} maps paired")
    print(f"{'measure':<10} {'before':>9} {'after':>9}  lower higher  same  sign p")
    for key in ["burnt_way", "burnt", "secs", "moves"]:
        av = sum(ra[n][key] for n in names) / len(names)
        bv = sum(rb[n][key] for n in names) / len(names)
        lo = sum(1 for n in names if rb[n][key] < ra[n][key] - 1e-9)
        hi = sum(1 for n in names if rb[n][key] > ra[n][key] + 1e-9)
        print(f"{key:<10} {av:>9.1f} {bv:>9.1f}  {lo:>5} {hi:>6} {len(names) - lo - hi:>5}  {sign(lo, hi):.3f}")
    for n in names:
        print(f"  {n:<20} burnt_way {ra[n]['burnt_way']:.0f}->{rb[n]['burnt_way']:.0f}  burnt {ra[n]['burnt']:.0f}->{rb[n]['burnt']:.0f}  secs {ra[n]['secs']:.1f}->{rb[n]['secs']:.1f}")


if __name__ == "__main__":
    main()
