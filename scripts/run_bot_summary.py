#!/usr/bin/env python3
"""The run bot's summary (the Engine and tools lane, 2026-11-20).

Reads what `larder-world league=run:SEED:TO plan=PLAN` printed (its
`league ...` lines) on stdin and writes a Markdown summary of the run: a
row a planet with the result, its length against the 8 to 12 minute target
(NOW.md: "each planet's length against 8 to 12 minutes"), the slime, the
squad, the kills and what the shop sold after it.

    python3 scripts/run_bot_summary.py SEED < bot.txt >> "$GITHUB_STEP_SUMMARY"
    python3 scripts/run_bot_summary.py --check SEED < bot.txt   # the gate
    python3 scripts/run_bot_summary.py --pace SEED=bot.txt ...  # all seeds

`--pace` (11-22, Direction: "each planet's minutes, its end (won, held
on, lost) and soldiers lost, per seed, plus the median by planet") reads
one `league` output a seed and writes a table a planet across the seeds,
with the median length of the planets won by their goal and the median
soldiers lost. The workflow's `pace` job runs it over the three seeds.
`scripts/play_log_summary.py` writes the same table from the owner's play
logs, so the bot's pace can be set beside his.

`ended=` (11-21, Direction) says how each planet ended: `goal` (won by its
goal), `held` (the slime held on to the stalemate limit: one star if the
squad cut it to half, else lost), `squad` (the last soldier fell),
`overrun`, `stuck` (no food to buy or fire with), `cleared` (the slime is
gone but the goal was not met), `open` (no verdict).

`--check` prints one line a planet (result and length) and fails on a
soft lock: a planet with no verdict by its clock and a minute (`open`),
or no line for a planet the run reached. It also fails a planet that ends
in under a minute (the lane brief's smoke check). A lost planet never
fails it: a loss is a measurement ("balance never blocks").

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


def result(p):
    """The planet's result in words, with held-on told apart from won."""
    how, ended = p.get("how", "?"), p.get("ended", "")
    stars = int(float(p.get("stars", 0)))
    s = f"{stars} star{'s' if stars != 1 else ''}"
    if how == "won" and ended == "held":
        return f"held on to the limit, {s} (cut to half)"
    if how == "won":
        return f"won, {s}"
    if how == "open":
        return "no verdict by the time limit"
    return {
        "held": "lost: the slime held on to the limit",
        "squad": "lost: the last soldier fell",
        "overrun": "lost: overrun with the bank empty",
        "stuck": "lost: no food to buy or fire with",
        "cleared": "lost: slime gone, goal not met",
    }.get(ended, "lost")


def end_word(p):
    """The planet's end in one word: won, held (on to the limit) or lost."""
    if p.get("how") == "won":
        return "held" if p.get("ended") == "held" else "won"
    return "open" if p.get("how") == "open" else "lost"


def lost(p):
    """Soldiers who fell on the planet (`fell_end`, else peak less alive)."""
    if "fell_end" in p:
        return int(p["fell_end"])
    try:
        return int(p["peak_alive"]) - int(p["alive"])
    except (KeyError, ValueError):
        return None


def median(xs):
    xs = sorted(xs)
    if not xs:
        return None
    n = len(xs)
    return xs[n // 2] if n % 2 else (xs[n // 2 - 1] + xs[n // 2]) / 2


def num(x):
    """A count or a median, without a needless `.0`; `-` for none."""
    return "-" if x is None else f"{x:g}"


def read(lines):
    """The planets (with the shop after each), the plan and the run lines."""
    planets, last, plan, runs = [], None, "?", []
    for line in lines:
        if not line.startswith("league "):
            continue
        kv = pairs(line)
        plan = kv.get("plan", plan)
        if "how" in kv and "depth" in kv:
            last = kv
            planets.append(kv)
        elif " shop " in line and last is not None:
            last["shop"] = kv.get("bought", "") or "nothing"
        elif "to" in kv and "won" in kv:
            runs.append(kv)
    return planets, plan, runs


def pace_table(title, seeds):
    """Markdown: a row a planet, a column a seed ("4:35 won, 3 lost"), then
    the median length of the planets won by their goal, the ends counted,
    and the median soldiers lost. `seeds` is [(name, [planet dicts])],
    each planet with depth, secs, how, ended and fell_end."""
    depths = sorted({int(p["depth"]) for _, ps in seeds for p in ps})
    print(f"### {title}")
    print()
    print("| Planet | " + " | ".join(str(n) for n, _ in seeds) + " | Median won by goal | Ends | Median soldiers lost |")
    print("|---" * (len(seeds) + 4) + "|")
    for d in depths:
        cells, won_secs, ends, losses = [], [], {}, []
        for _, ps in seeds:
            here = [p for p in ps if int(p["depth"]) == d]
            if not here:
                cells.append("-")
                continue
            parts = []
            for p in here:
                secs, word, fell = float(p.get("secs", 0)), end_word(p), lost(p)
                ends[word] = ends.get(word, 0) + 1
                if word == "won" and p.get("ended", "goal") == "goal":
                    won_secs.append(secs)
                if fell is not None:
                    losses.append(fell)
                parts.append(f"{clock(secs)} {word}" + (f", {fell} lost" if fell is not None else ""))
            cells.append("; ".join(parts))
        m, ml = median(won_secs), median(losses)
        tally = ", ".join(f"{n} {w}" for w, n in sorted(ends.items(), key=lambda e: ["won", "held", "lost", "open"].index(e[0])))
        print(
            f"| {d} | " + " | ".join(cells)
            + f" | {clock(m) if m is not None else '-'} | {tally} | {num(ml)} |"
        )
    print()


def check(seed, planets, runs):
    """The CI gate: fails on a soft lock or a planet under a minute."""
    bad = []
    for p in planets:
        secs = float(p.get("secs", 0))
        print(f"seed {seed} planet {p.get('depth', '?')} ({p.get('tier', '?')}): {result(p)}, {clock(secs)}")
        if p.get("how") == "open":
            bad.append(f"planet {p.get('depth')}: no verdict by its clock and a minute (a soft lock)")
        if secs < 60:
            bad.append(f"planet {p.get('depth')}: ended at {clock(secs)}, under a minute")
    if not planets or not runs:
        bad.append("the run printed no planet or no end line (the game stopped early)")
    for b in bad:
        print(f"FAIL: {b}")
    return 1 if bad else 0


def main():
    if sys.argv[1:2] == ["--pace"]:
        seeds, plan = [], "?"
        for a in sys.argv[2:]:
            name, _, path = a.rpartition("=")
            try:
                with open(path) as f:
                    planets, plan, _ = read(f)
            except OSError:
                planets = []
            seeds.append((f"seed {name or path}", planets))
        pace_table(f"The bot's pace by planet, plan `{plan}`", seeds)
        print("Length is a planet's game time; \"won\" is won by its goal, \"held\" held on to the limit (cut to half).")
        return 0
    args = [a for a in sys.argv[1:] if a != "--check"]
    gate = "--check" in sys.argv[1:]
    seed = args[0] if args else "?"
    planets, plan, runs = read(sys.stdin)
    if gate:
        return check(seed, planets, runs)
    print(f"### Run seed {seed}, plan `{plan}`")
    print()
    if not planets:
        print("No planet was played: see the job's log.")
        return
    print("| Planet | Slime | Result | Length | 8 to 12 min | Slime left of its peak | Soldiers (peak) | Soldiers lost | Killed | Shop after |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    inside = won = 0
    for p in planets:
        secs = float(p.get("secs", 0))
        # Only a planet won by its goal has a length that measures it: a
        # held one stopped at the stalemate limit, a lost one at its end.
        if p.get("how") == "won":
            won += 1
        if p.get("how") == "won" and p.get("ended", "goal") == "goal":
            if secs < TARGET[0]:
                band = "short"
            elif secs > TARGET[1]:
                band = "long"
            else:
                band = "inside"
                inside += 1
        else:
            band = "-"
        print(
            f"| {p.get('depth', '?')} | {p.get('tier', '?')} | {result(p)} | {clock(secs)} | {band} "
            f"| {p.get('mould', '?')} of {p.get('peak_mould', '?')} | {p.get('alive', '?')} ({p.get('peak_alive', '?')}) "
            f"| {lost(p) if lost(p) is not None else '?'} | {p.get('killed', '?')} | {p.get('shop', '-')} |"
        )
    print()
    print(f"{won} of {len(planets)} planets won or held; {inside} won by their goal took 8 to 12 minutes.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
