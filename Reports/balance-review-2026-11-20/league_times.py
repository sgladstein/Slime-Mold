"""League games by game version and planet: how many were won, how long a win
took, and how big the winning army was. Reads the strategy bot league's raw
rows (every *.jsonl under Reports/strategy-bots-2026-11-19/pc/, or under the
folder given). The review read the strategy thread's branch, which the worker
on the owner's PC keeps adding to:

    git fetch origin claude/project-thread-8grg6d
    mkdir /tmp/league && git archive FETCH_HEAD Reports/strategy-bots-2026-11-19/pc | tar -x -C /tmp/league
    python3 Reports/balance-review-2026-11-20/league_times.py /tmp/league/Reports/strategy-bots-2026-11-19/pc

The league stops a game at 720 s (12:00), so every win here is a real wipe,
never the clock's one-star "held on" (finding 2 of the review).
"""
import glob, json, os, statistics, sys

HERE = os.path.dirname(os.path.abspath(__file__))
DIR = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "..", "strategy-bots-2026-11-19", "pc")
# Code stamps in the rows. "old": before shots stopped at set slime;
# "set": shots stop at set slime (11-20); "now": later code (one burst per
# bullet). "first" is the first league's rows, whatever their stamp.
OLD = {None, "9a4b83191084", "3052b1da7102", "68f057590a7b", "de884269a412"}
SET = {"49cc46f036ba"}


def era(r):
    c = r.get("code")
    return "old" if c in OLD else "set" if c in SET else "now"


rows = []
for f in sorted(glob.glob(os.path.join(DIR, "**", "*.jsonl"), recursive=True)):
    for line in open(f):
        line = line.strip()
        if not line:
            continue
        try:
            r = json.loads(line)
        except json.JSONDecodeError:
            continue
        if "how" in r and "depth" in r:
            # The first league (gen*.jsonl beside the jobs, all planets, one
            # plan pool) is the only set that plays planets 1, 2 and 3 alike.
            r["_first"] = os.path.dirname(f) == os.path.normpath(DIR) and os.path.basename(f).startswith("gen")
            rows.append(r)


def mmss(s):
    return f"{int(s) // 60}:{int(s) % 60:02d}"


print(f"{len(rows)} games")
print("group  planet  games  won   median win  wins under 8:00  median peak army (wins)")
groups = [("first", lambda r: r["_first"])] + [(e, lambda r, e=e: era(r) == e) for e in ("old", "set", "now")]
for e, keep in groups:
    for d in sorted({int(r["depth"]) for r in rows}):
        g = [r for r in rows if keep(r) and int(r["depth"]) == d]
        if not g:
            continue
        w = [r for r in g if r["how"] == "won"]
        if not w:
            print(f"{e:5} {d:7} {len(g):6}  0%")
            continue
        secs = [r["secs"] for r in w]
        under = sum(1 for s in secs if s < 480) / len(w)
        army = statistics.median(r.get("peak_alive", 0) for r in w)
        print(f"{e:5} {d:7} {len(g):6} {100 * len(w) / len(g):4.0f}%  {mmss(statistics.median(secs)):>10}  {100 * under:14.0f}%  {army:10.0f}")
