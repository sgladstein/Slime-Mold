#!/usr/bin/env python3
"""Pathing faults from a `LARDER_PATHLOG` CSV (pathfinding thread, 2026-11-20).

    python3 paths.py LOG.csv [--tps 15] [--speed 10] [--list N]

Rows (dish cells, a game step each, 15 a game second):
  s,tick,k,squad,x,y,vx,vy,gx,gy,touching,order,cover,hp
  q,tick,q,ax,ay,dx,dy,steerx,steery,holding,column,fight

What it counts, each a soldier-step unless said:
- unplaced: a goal more than 2 cells off, its squad not standing to fight, and
  under a tenth of full speed, for 3 game seconds or more in a row (episodes).
- strays: over 3 game seconds standing far from its squad's median.
- dither: over 2 s, walked more than 3 cells and got less than a quarter of
  that nearer its goal or further on (back and forth).
- strung: while a squad's anchor moves (not in a column), its members'
  90th-percentile distance from their median, against the blob's own size.
- stalls: a squad with a destination and not standing to fight whose anchor
  moved under 1 cell in 3 s (episodes).
- trips: each destination, from set to cleared: straight distance, the
  anchor's walked length, seconds.
- slimed: soldier-steps walking (over a third of full speed) with walkers on it.
"""
import math
import sys
from collections import defaultdict


def main():
    args = sys.argv[1:]
    path = args[0]
    tps = 15
    speed = 10.0
    nlist = 12
    for i, a in enumerate(args):
        if a == "--tps":
            tps = int(args[i + 1])
        if a == "--speed":
            speed = float(args[i + 1])
        if a == "--list":
            nlist = int(args[i + 1])
    sold = defaultdict(list)  # k -> [(t, sq, x, y, v, g, d, touching, order)]
    squads = defaultdict(list)  # q -> [(t, ax, ay, dx, dy, steer, hold, col)]
    members = defaultdict(lambda: defaultdict(list))  # t -> q -> [(x, y)]
    hold_at = {}
    for line in open(path):
        p = line.rstrip("\n").split(",")
        if len(p) < 12 or "" in p:
            continue  # a cut last line
        if p[0] == "s":
            t, k, q = int(p[1]), int(p[2]), int(p[3])
            x, y, vx, vy, gx, gy = map(float, p[4:10])
            touching, order = int(p[10]), p[11]
            d = math.hypot(gx - x, gy - y) if not math.isnan(gx) else float("nan")
            sold[k].append((t, q, x, y, math.hypot(vx, vy), (gx, gy), d, touching, order))
            members[t][q].append((x, y))
        elif p[0] == "q":
            t, q = int(p[1]), int(p[2])
            ax, ay, dx, dy, sx, sy = map(float, p[3:9])
            hold, col = int(p[9]), int(p[10])
            squads[q].append((t, ax, ay, dx, dy, (sx, sy) != (0.0, 0.0), hold, col))
            hold_at[(t, q)] = hold
    if not sold:
        print("no rows")
        return
    t_end = max(r[0] for v in sold.values() for r in v)
    steps = sum(len(v) for v in sold.values())
    print(f"{path}: {len(sold)} soldiers, {steps} soldier-steps, {t_end / tps:.0f} s")

    # Stuck episodes.
    slow = 0.1 * speed
    stuck = []
    stuck_steps = 0
    for k, rows in sold.items():
        run = []
        for r in rows + [None]:
            bad = r is not None and not math.isnan(r[6]) and r[6] > 2.0 and r[4] < slow and not hold_at.get((r[0], r[1]), 0) and r[8] in "mfgc"
            if bad and run and r[0] - run[-1][0] <= 1:
                run.append(r)
                continue
            if len(run) >= 3 * tps:
                stuck.append((len(run) / tps, k, run[0]))
                stuck_steps += len(run)
            run = [r] if bad else []
    stuck.sort(reverse=True)
    print(f"unplaced: {len(stuck)} episodes of 3 s+ standing over 2 cells from their goal,, {stuck_steps / max(steps, 1) * 100:.2f}% of soldier-steps")
    for secs, k, r in stuck[:nlist]:
        print(f"   soldier {k} squad {r[1]} from {r[0] / tps:.1f} s for {secs:.1f} s at {r[2]:.1f},{r[3]:.1f}, goal {r[5][0]:.1f},{r[5][1]:.1f} ({r[6]:.1f} off), order {r[8]}, touching {r[7]}")

    # Strays: standing still 3 s+ far from their squad (over twice its
    # blob's radius, at least 8 cells, from its members' median), wanting to
    # be elsewhere.
    med = {}
    for t, qs in members.items():
        for q, m in qs.items():
            xs = sorted(p[0] for p in m)
            ys = sorted(p[1] for p in m)
            med[(t, q)] = (xs[len(xs) // 2], ys[len(ys) // 2], max(8.0, 2 * 2.52 * math.sqrt(len(m))))
    strays = []
    stray_steps = 0
    for k, rows in sold.items():
        run = []
        for r in rows + [None]:
            bad = False
            if r is not None and not math.isnan(r[6]) and r[6] > 2.0 and r[4] < slow and not hold_at.get((r[0], r[1]), 0):
                mx, my, far = med[(r[0], r[1])]
                bad = math.hypot(r[2] - mx, r[3] - my) > far
            if bad and run and r[0] - run[-1][0] <= 1:
                run.append(r)
                continue
            if len(run) >= 3 * tps:
                strays.append((len(run) / tps, k, run[0]))
                stray_steps += len(run)
            run = [r] if bad else []
    strays.sort(reverse=True)
    print(f"strays: {len(strays)} episodes of 3 s+ standing far from their squad, {stray_steps / max(steps, 1) * 100:.2f}% of soldier-steps")
    for secs, k, r in strays[:nlist]:
        mx, my, far = med[(r[0], r[1])]
        print(f"   soldier {k} squad {r[1]} from {r[0] / tps:.1f} s for {secs:.1f} s at {r[2]:.1f},{r[3]:.1f}, squad's median {mx:.0f},{my:.0f}, goal {r[5][0]:.1f},{r[5][1]:.1f}, order {r[8]}")

    # Dither: over 2 s windows.
    w = 2 * tps
    dither = 0
    windows = 0
    dith_where = []
    for k, rows in sold.items():
        for i in range(0, len(rows) - w, tps // 3):
            a, b = rows[i], rows[i + w]
            if b[0] - a[0] != w or math.isnan(a[6]) or math.isnan(b[6]):
                continue
            walked = sum(math.hypot(rows[j + 1][2] - rows[j][2], rows[j + 1][3] - rows[j][3]) for j in range(i, i + w))
            if walked < 3.0:
                continue
            windows += 1
            net = math.hypot(b[2] - a[2], b[3] - a[3])
            gained = a[6] - b[6] if a[5] == b[5] else net
            if net < 0.25 * walked and gained < 0.25 * walked:
                dither += 1
                dith_where.append((a[0] / tps, k, a[1], a[2], a[3], walked, net))
    print(f"dither: {dither} of {windows} walking 2 s windows ({dither / max(windows, 1) * 100:.1f}%) walked 3+ cells and got under a quarter of it on")
    seen = set()
    shown = 0
    for t, k, q, x, y, walked, net in dith_where:
        key = (round(x / 10), round(y / 10), int(t / 10))
        if key in seen:
            continue
        seen.add(key)
        print(f"   soldier {k} squad {q} at {t:.1f} s near {x:.0f},{y:.0f}: walked {walked:.1f}, net {net:.1f}")
        shown += 1
        if shown >= nlist:
            break

    # Strung out while moving.
    strung = []
    for q, rows in squads.items():
        for (t, ax, ay, dx, dy, steer, hold, col) in rows:
            if math.isnan(ax) or col:
                continue
            m = members[t][q]
            if len(m) < 4:
                continue
            xs = sorted(p[0] for p in m)
            ys = sorted(p[1] for p in m)
            mx, my = xs[len(xs) // 2], ys[len(ys) // 2]
            ds = sorted(math.hypot(p[0] - mx, p[1] - my) for p in m)
            p90 = ds[int(0.9 * (len(ds) - 1))]
            # The blob's sunflower radius: ring 2.52 x sqrt(n).
            own = 2.52 * math.sqrt(len(m))
            strung.append((p90 / own, t, q, len(m), p90, own, mx, my))
    if strung:
        rs = sorted(s[0] for s in strung)
        over = sum(1 for s in rs if s > 2.0)
        print(f"strung: {len(rs)} squad-steps moving; p90 spread / blob radius median {rs[len(rs) // 2]:.2f}, 90% {rs[int(0.9 * (len(rs) - 1))]:.2f}, max {rs[-1]:.2f}; over 2x in {over} ({over / len(rs) * 100:.1f}%)")
        worst = sorted(strung, reverse=True)
        seen = set()
        shown = 0
        for r, t, q, n, p90, own, mx, my in worst:
            key = (q, int(t / tps / 5))
            if key in seen:
                continue
            seen.add(key)
            print(f"   squad {q} at {t / tps:.1f} s, {n} members round {mx:.0f},{my:.0f}: 90% within {p90:.1f} (blob {own:.1f})")
            shown += 1
            if shown >= nlist // 2:
                break

    # Squad stalls and trips.
    stalls = []
    trips = []
    for q, rows in squads.items():
        cur = None
        for i, (t, ax, ay, dx, dy, steer, hold, col) in enumerate(rows):
            has = not math.isnan(dx)
            if has and (cur is None or (dx, dy) != cur["d"]):
                if cur is not None:
                    trips.append(cur)
                cur = {"q": q, "t0": t, "d": (dx, dy), "from": (ax, ay), "len": 0.0, "last": (ax, ay), "t1": t, "done": False, "hold": 0}
            elif not has and cur is not None:
                cur["done"] = True
                trips.append(cur)
                cur = None
            if cur is not None:
                if not math.isnan(ax):
                    if math.isnan(cur["from"][0]):
                        cur["from"] = (ax, ay)
                    if not math.isnan(cur["last"][0]):
                        cur["len"] += math.hypot(ax - cur["last"][0], ay - cur["last"][1])
                    cur["last"] = (ax, ay)
                cur["t1"] = t
                cur["hold"] += hold
        if cur is not None:
            trips.append(cur)
        # Stalls: dest set, not holding, anchor moved < 1 in 3 s.
        w3 = 3 * tps
        i = 0
        while i + w3 < len(rows):
            a, b = rows[i], rows[i + w3]
            ok = b[0] - a[0] == w3 and not math.isnan(a[3]) and a[3:5] == b[3:5] and not any(r[6] for r in rows[i:i + w3 + 1]) and not math.isnan(a[1]) and not math.isnan(b[1])
            if ok and math.hypot(b[1] - a[1], b[2] - a[2]) < 1.0 and math.hypot(a[3] - a[1], a[4] - a[2]) > 3.0:
                stalls.append((a[0] / tps, q, a[1], a[2], a[3], a[4], len(members[a[0]][q])))
                i += w3
            else:
                i += 1
    print(f"stalls: {len(stalls)} squad windows of 3 s with a goal more than 3 cells off, not fighting, anchor moved under 1 cell")
    for t, q, ax, ay, dx, dy, n in stalls[:nlist]:
        print(f"   squad {q} ({n}) at {t:.1f} s: anchor {ax:.0f},{ay:.0f}, goal {dx:.0f},{dy:.0f}")
    long = [tr for tr in trips if not math.isnan(tr["from"][0])]
    for tr in long:
        tr["straight"] = math.hypot(tr["d"][0] - tr["from"][0], tr["d"][1] - tr["from"][1])
        tr["secs"] = (tr["t1"] - tr["t0"]) / tps
    far = [tr for tr in long if tr["straight"] > 20]
    if far:
        ratio = sorted(tr["len"] / tr["straight"] for tr in far)
        pace = sorted(tr["len"] / max(tr["secs"], 1e-3) for tr in far if tr["done"])
        print(f"trips: {len(long)} ({len(far)} over 20 cells); walked / straight median {ratio[len(ratio) // 2]:.2f}, max {ratio[-1]:.2f}; finished pace median {pace[len(pace) // 2] if pace else float('nan'):.2f} cells/s (soldier {speed}, anchor 0.75 of it)")
        for tr in sorted(far, key=lambda tr: -tr["len"] / tr["straight"])[: nlist // 2]:
            print(f"   squad {tr['q']} at {tr['t0'] / tps:.1f} s: {tr['from'][0]:.0f},{tr['from'][1]:.0f} -> {tr['d'][0]:.0f},{tr['d'][1]:.0f}, straight {tr['straight']:.0f}, walked {tr['len']:.0f} in {tr['secs']:.1f} s{'' if tr['done'] else ' (not there)'}")

    walking = [r for v in sold.values() for r in v if r[4] > speed / 3]
    slimed = sum(1 for r in walking if r[7] > 0)
    import json
    summary = {
        "unplaced": stuck_steps / max(steps, 1),
        "strays": stray_steps / max(steps, 1),
        "dither": dither / max(windows, 1),
        "strung2x": (sum(1 for s in strung if s[0] > 2.0) / len(strung)) if strung else 0.0,
        "stalls": len(stalls),
        "ratio": (sorted(tr["len"] / tr["straight"] for tr in far)[len(far) // 2]) if far else 0.0,
        "unfinished": sum(1 for tr in far if not tr["done"]),
        "pace": (sorted(tr["len"] / max(tr["secs"], 1e-3) for tr in far if tr["done"]) or [0.0])[len([tr for tr in far if tr["done"]]) // 2] if far else 0.0,
    }
    with open(path + ".json", "w") as jf:
        json.dump(summary, jf)
    print(f"slimed: {slimed} of {len(walking)} walking soldier-steps ({slimed / max(len(walking), 1) * 100:.1f}%) had walkers on them")


if __name__ == "__main__":
    main()
