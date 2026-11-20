#!/usr/bin/env python3
"""The run bot's summary (the Engine and tools lane, 2026-11-20).

Reads what `larder-world league=run:SEED:TO plan=PLAN` printed (its
`league ...` lines) on stdin and writes a Markdown summary of the run: a
row a planet with the result, its length against the 8 to 12 minute target
(NOW.md: "each planet's length against 8 to 12 minutes"), the slime, the
squad, the kills and what the shop sold after it.

    python3 scripts/run_bot_summary.py SEED < bot.txt >> "$GITHUB_STEP_SUMMARY"

A planet the bot loses plays to its clock, so its length is no measure of
the planet's: only a won planet's length is compared with the target. A
planet with no verdict when the game stopped (`how=open`) is not a win.
"""

import sys

TARGET = (8 * 60, 12 * 60)


def pairs(line):
    """The key=value words of a league line (no value holds a space)."""
    out = {}
    for w in line.split():
        k, sep, v = w.partition("=")
        if sep:
            out[k] = v
    return out


def clock(secs):
    s = int(round(float(secs)))
    return f"{s // 60}:{s % 60:02d}"


def main():
    seed = sys.argv[1] if len(sys.argv) > 1 else "?"
    planets, last, plan = [], None, "?"
    for line in sys.stdin:
        if not line.startswith("league "):
            continue
        kv = pairs(line)
        plan = kv.get("plan", plan)
        if "how" in kv and "depth" in kv:
            last = kv
            planets.append(kv)
        elif " shop " in line and last is not None:
            last["shop"] = kv.get("bought", "") or "nothing"
    print(f"### Run seed {seed}, plan `{plan}`")
    print()
    if not planets:
        print("No planet was played: see the job's log.")
        return
    print("| Planet | Slime | Result | Length | 8 to 12 min | Slime left of its peak | Soldiers (peak) | Killed | Shop after |")
    print("|---|---|---|---|---|---|---|---|---|")
    inside = won = 0
    for p in planets:
        how = p.get("how", "?")
        secs = float(p.get("secs", 0))
        if how == "won":
            won += 1
            stars = int(float(p.get("stars", 0)))
            result = f"won, {stars} star{'s' if stars != 1 else ''}"
            if secs < TARGET[0]:
                band = "short"
            elif secs > TARGET[1]:
                band = "long"
            else:
                band = "inside"
                inside += 1
        else:
            # `open`: no verdict when the game stopped. With no SECS (CI)
            # that is the planet's clock and a minute, so a soft lock; with
            # SECS it is only the time limit (a league job's 900).
            result = "lost" if how == "lost" else "no verdict by the time limit"
            band = "-"
        print(
            f"| {p.get('depth', '?')} | {p.get('tier', '?')} | {result} | {clock(secs)} | {band} "
            f"| {p.get('mould', '?')} of {p.get('peak_mould', '?')} | {p.get('alive', '?')} ({p.get('peak_alive', '?')}) "
            f"| {p.get('killed', '?')} | {p.get('shop', '-')} |"
        )
    print()
    print(f"{won} of {len(planets)} planets won; {inside} of the {won} won took 8 to 12 minutes.")


if __name__ == "__main__":
    main()
