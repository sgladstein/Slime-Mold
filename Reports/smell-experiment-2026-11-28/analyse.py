#!/usr/bin/env python3
"""The smell experiment's batch, paired: the knowledge gate (arm `on-PLAN`)
against main's map-wide knowledge (`off-PLAN`), by planet, run seed and
plan (rows.txt from run-batch.ps1, or the Arms workflow's rows).

    python3 analyse.py rows.txt

For each planet and plan: wins off and on; soldiers lost (fell_end) paired
by seed, with how many seeds lost fewer, the same or more with the gate on
and a two-sided sign test p; buildings lost; raids sent by aim (building,
lure, soldiers, last stand) and waves; and the share of soldier-seconds and
building-seconds the slime smelled with the gate on.
"""
import sys
from collections import defaultdict
from math import comb


def sign_p(plus, minus):
    """Two-sided sign test p for `plus` against `minus` (ties dropped)."""
    n = plus + minus
    if n == 0:
        return 1.0
    k = min(plus, minus)
    return min(1.0, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n)


def parse(path):
    games = {}
    for line in open(path):
        if not line.startswith("ARMS ") or " league " not in line:
            continue
        kv = {}
        for tok in line.split():
            if "=" in tok:
                k, v = tok.split("=", 1)
                kv.setdefault(k, v)
        arm = kv.get("arm", "")
        if "-" not in arm or "depth" not in kv:
            continue
        side, plan = arm.split("-", 1)
        know = kv.get("know", "0/0/0/0:0/0/0/0:0").split(":")
        games[(int(kv["depth"]), int(kv["seed"]), plan, side)] = {
            "won": int(kv["won"]),
            "how": kv.get("how", "?"),
            "secs": float(kv["secs"]),
            "fell": int(kv["fell_end"]),
            "lost_b": int(kv["lost_buildings"]),
            "killed": int(kv["killed"]),
            "smelt": [int(x) for x in know[0].split("/")],
            "raids": [int(x) for x in know[1].split("/")],
            "waves": int(know[2]),
        }
    return games


def main():
    games = parse(sys.argv[1] if len(sys.argv) > 1 else "rows.txt")
    keys = sorted({(p, plan) for (p, _, plan, _) in games})
    print(f"{len(games)} games")
    for planet, plan in keys:
        seeds = sorted({s for (p, s, pl, _) in games if p == planet and pl == plan})
        pairs = [(games.get((planet, s, plan, "off")), games.get((planet, s, plan, "on"))) for s in seeds]
        pairs = [(s, a, b) for s, (a, b) in zip(seeds, pairs) if a and b]
        if not pairs:
            continue
        fewer = sum(b["fell"] < a["fell"] for _, a, b in pairs)
        more = sum(b["fell"] > a["fell"] for _, a, b in pairs)
        same = len(pairs) - fewer - more
        tot = lambda side, f: sum(f(a if side == "off" else b) for _, a, b in pairs)
        smelt = [sum(b["smelt"][i] for _, _, b in pairs) for i in range(4)]
        print(f"\nplanet {planet} {plan}: {len(pairs)} pairs")
        print(f"  won        off {tot('off', lambda g: g['won'])}  on {tot('on', lambda g: g['won'])}")
        print(f"  fell_end   off {tot('off', lambda g: g['fell'])}  on {tot('on', lambda g: g['fell'])}  "
              f"(on lost fewer {fewer}, same {same}, more {more}; sign p {sign_p(fewer, more):.2f})")
        print(f"  lost_bldg  off {tot('off', lambda g: g['lost_b'])}  on {tot('on', lambda g: g['lost_b'])}")
        for side in ("off", "on"):
            r = [sum((a if side == "off" else b)["raids"][i] for _, a, b in pairs) for i in range(4)]
            w = tot(side, lambda g: g["waves"])
            print(f"  raids {side:3} building {r[0]} lure {r[1]} soldiers {r[2]} last stand {r[3]}; waves {w}")
        if smelt[1] or smelt[3]:
            print(f"  smelled    soldiers {smelt[0] / max(smelt[1], 1):.0%} of soldier-seconds, "
                  f"buildings {smelt[2] / max(smelt[3], 1):.0%} of building-seconds")
        for s, a, b in pairs:
            print(f"    seed {s}: off {a['how']:5} {a['secs']:6.0f}s fell {a['fell']:3} | on {b['how']:5} {b['secs']:6.0f}s fell {b['fell']:3}")


if __name__ == "__main__":
    main()
