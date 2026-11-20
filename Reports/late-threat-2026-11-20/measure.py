"""The late-threat measure (2026-11-20, the balance thread; the brief
"Keeping a big squad under threat", https://claude.ai/artifact/YD9LSGAkAorDNKg5WA2zZH).

Reads strategy-league rows (one JSON a game, `league.py run`'s output) whose
jobs carry extra arguments (the arm), pairs them by map, and prints, per arm:

- late: soldiers the slime bit to death in the game's last quarter (the
  deciding measure), from `fell=` (every 30 s: fallen so far / bitten to
  death / burned) and `bitten_end=`. Bitten, not all fallen: on planet 3
  the strat bot's own blasts and fires kill many of its soldiers, which
  says nothing about the slime's threat;
- late_all: every soldier fallen in the last quarter, whatever the cause;
- collapse: seconds from the slime's first count under half its peak to its
  first under 5% (30 s marks; Scott's logs, 1 s counts: a median 82 s);
- won and the game's length.

Then each arm against the base arm, paired by map: how many maps rose, fell
or tied on `late`, with a two-sided sign test.

    python3 measure.py ROWS.jsonl [ROWS.jsonl ...] [--base ARM]
"""
import json, math, sys, collections

def marks(r):
    out = []
    for m in str(r.get("marks", "")).split(","):
        if ":" in m:
            t, rest = m.split(":", 1)
            k, mould, alive, bank = rest.split("/")
            out.append((int(float(t)), int(mould), int(alive)))
    return out

def fell(r):
    """(t, fallen, bitten) every 30 s."""
    out = []
    for m in str(r.get("fell", "")).split(","):
        if ":" in m:
            t, rest = m.split(":")
            n = rest.split("/")
            out.append((int(float(t)), int(n[0]), int(n[1]) if len(n) > 1 else 0))
    return out

def measure(r):
    secs = float(r["secs"])
    f = fell(r)
    end = int(r.get("fell_end", f[-1][1] if f else 0))
    bit = int(r.get("bitten_end", f[-1][2] if f else 0))
    q = 0.75 * secs
    before = max([n for t, n, _ in f if t <= q] or [0])
    bit_before = max([b for t, _, b in f if t <= q] or [0])
    ms = marks(r)
    peak = float(r.get("peak_mould") or max([m for _, m, _ in ms] or [1]))
    half = next((t for t, m, _ in ms if m < 0.5 * peak), None)
    five = next((t for t, m, _ in ms if m < 0.05 * peak), None)
    if five is None and float(r.get("mould", peak)) < 0.05 * peak:
        five = secs
    collapse = five - half if half is not None and five is not None else None
    return {"late": bit - bit_before, "late_all": end - before, "bitten": bit, "fallen": end, "collapse": collapse, "won": int(float(r.get("won", 0))), "secs": secs,
            "peak_alive": int(float(r.get("peak_alive", 0)))}

def sign_p(up, down):
    n = up + down
    if n == 0:
        return 1.0
    k = min(up, down)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(k + 1)) / 2 ** n)

def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    # --base ARM names the arm the others are paired against; the base arm
    # (no extra arguments) by default.
    base = sys.argv[sys.argv.index("--base") + 1] if "--base" in sys.argv else ""
    if base in args:
        args.remove(base)
    rows = []
    for p in args:
        for line in open(p):
            try:
                r = json.loads(line)
            except json.JSONDecodeError:
                continue
            if "error" not in r:
                rows.append(r)
    arms = collections.defaultdict(dict)
    for r in rows:
        arms[r.get("extra", "")][(int(r["depth"]), int(r["seed"]), r["plan"])] = measure(r)
    med = lambda xs: sorted(xs)[len(xs) // 2] if xs else None
    print(f"{'arm':28s} games  won  late bitten  late all  bitten  fallen  collapse s  length s   (medians)")
    for arm, games in sorted(arms.items()):
        g = list(games.values())
        print(f"{arm or '(base)':28s} {len(g):5d} {sum(x['won'] for x in g):4d}  {med([x['late'] for x in g])!s:>11}  {med([x['late_all'] for x in g])!s:>8}  {med([x['bitten'] for x in g])!s:>6}  {med([x['fallen'] for x in g])!s:>6}  "
              f"{med([x['collapse'] for x in g if x['collapse'] is not None])!s:>10}  {med([x['secs'] for x in g])!s:>8}")
    if base in arms and len(arms) > 1:
        print()
        for arm, games in sorted(arms.items()):
            if arm == base:
                continue
            both = [k for k in games if k in arms[base]]
            up = sum(games[k]["late"] > arms[base][k]["late"] for k in both)
            down = sum(games[k]["late"] < arms[base][k]["late"] for k in both)
            print(f"{arm or '(base)'} vs {base or '(base)'}: late bitten up on {up}, down on {down}, tied on {len(both) - up - down} of {len(both)} maps; sign test p = {sign_p(up, down):.3f}")

if __name__ == "__main__":
    main()
