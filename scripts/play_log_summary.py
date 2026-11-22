#!/usr/bin/env python3
"""The owner's pace by planet, from his play logs (Engine and tools, 2026-11-22).

Direction (11-22) asked for the bot's planet pace beside the owner's: each
planet's minutes, its end (won, held on, lost) and the soldiers lost.
`scripts/run_bot_summary.py --pace` writes the bot's; this writes the
same from the play logs the game uploads to the `play-logs` branch
(`logs/<when>-planet-N-<run>-<seed>.txt`: a `log ...` head line, then a
`t=N mould=... soldiers=...` line a second and, since 2026-11-18, an
`end t=N <verdict>` line).

    git fetch origin play-logs
    python3 scripts/play_log_summary.py                 # every log on the branch
    python3 scripts/play_log_summary.py --since 2026-11-21 FILE ...

Until 2026-11-22 a planet won in the step that finished on the GPU wrote no
`end` line (main.rs `log_end`), so 21 of the first 30 logs stop with no
verdict. A log with no `end` line is read from its last second: the slime
at 50 walkers or fewer is `won` (the goal is met long before the slime is
gone: those logs stop with 0 to 40 walkers and the next planet follows),
one soldier left with the slime alive is `lost`, else `quit` (the owner
left or restarted, often in the first minute). Those ends are marked `*`.

Soldiers lost are the drops in `soldiers=` from second to second (a soldier
bought in the same second as one falls hides that fall), so the count is a
floor.
"""

import os
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from run_bot_summary import clock, median, num, pairs  # noqa: E402

REF = "origin/play-logs"


def end_of(text):
    """The end word for an `end` line's verdict."""
    if text.startswith("Goal met") or text == "Mission won":
        return "won"
    if text.startswith("The slime held on") or text == "Time: held on":
        return "held" if "1 star" in text else "lost"
    return "lost"


def read_log(name, lines):
    head, secs, end, guessed = {}, [], None, False
    for line in lines:
        if line.startswith("log "):
            head = pairs(line)
        elif line.startswith("t="):
            kv = pairs(line)
            try:
                secs.append((int(kv["t"]), int(kv["mould"]), int(kv["soldiers"])))
            except (KeyError, ValueError):
                pass
        elif line.startswith("end "):
            t, _, text = line[4:].strip().partition(" ")
            end = (float(t.partition("=")[2] or 0), end_of(text), text)
    if not head or not secs:
        return None
    lost = sum(max(0, a[2] - b[2]) for a, b in zip(secs, secs[1:]))
    last = secs[-1]
    if end is None:
        guessed = True
        if last[1] <= 50:
            word = "won"
        elif last[2] <= 1:
            word = "lost"
        else:
            word = "quit"
        end = (float(last[0]), word, "")
    return {
        "name": name,
        "when": head.get("when", "?")[:16].replace("T", " "),
        "build": head.get("build", "?").partition(":")[0],
        "run": head.get("run", "?"),
        "depth": int(head.get("planet", 0)),
        "tier": head.get("tier", "?"),
        "secs": end[0],
        "end": end[1],
        "said": end[2],
        "guessed": guessed,
        "lost": lost,
        "peak": max(s[2] for s in secs),
    }


def from_branch(since):
    names = subprocess.run(
        ["git", "ls-tree", "--name-only", REF, "logs/"], capture_output=True, text=True, check=True
    ).stdout.split()
    out = []
    for n in names:
        if "-planet-" not in n or os.path.basename(n)[:10] < since:
            continue
        text = subprocess.run(["git", "show", f"{REF}:{n}"], capture_output=True, text=True, check=True).stdout
        out.append((n, text.splitlines()))
    return out


def main():
    args = sys.argv[1:]
    since = ""
    if "--since" in args:
        i = args.index("--since")
        since = args[i + 1]
        del args[i : i + 2]
    if args:
        logs = []
        for p in args:
            with open(p) as f:
                logs.append((p, f.read().splitlines()))
    else:
        logs = from_branch(since)
    planets = [p for p in (read_log(n, ls) for n, ls in logs) if p]
    print("### The owner's planets, from his play logs")
    print()
    print("| When (UTC) | Build | Run | Planet | Slime | Length | End | Soldiers lost (most at once) |")
    print("|---|---|---|---|---|---|---|---|")
    for p in planets:
        end = p["end"] + ("*" if p["guessed"] else "")
        print(
            f"| {p['when']} | {p['build']} | {p['run']} | {p['depth']} | {p['tier']} | {clock(p['secs'])} "
            f"| {end} | {p['lost']} ({p['peak']}) |"
        )
    print()
    print("`*` no `end` line: read from the last second (see the script's notes).")
    print()
    print("| Planet | Played | Ends | Median won by goal | Won lengths | Median soldiers lost, won |")
    print("|---|---|---|---|---|---|")
    for d in sorted({p["depth"] for p in planets}):
        here = [p for p in planets if p["depth"] == d]
        ends = {}
        for p in here:
            ends[p["end"]] = ends.get(p["end"], 0) + 1
        won = [p for p in here if p["end"] == "won"]
        secs = sorted(p["secs"] for p in won)
        m, ml = median(secs), median([p["lost"] for p in won])
        tally = ", ".join(f"{n} {w}" for w, n in sorted(ends.items(), key=lambda e: ["won", "held", "lost", "quit"].index(e[0])))
        print(
            f"| {d} | {len(here)} | {tally} | {clock(m) if m is not None else '-'} "
            f"| {', '.join(clock(s) for s in secs) or '-'} | {num(ml)} |"
        )
    return 0


if __name__ == "__main__":
    sys.exit(main())
