#!/usr/bin/env python3
"""Round 5: compare rule sets game by game. Each rule set is a tag on the
harness `duel` lines; every rule set plays the same games (seed, left plan,
right plan), so each game is a pair across rule sets.

Per rule set: how often the side ahead at 1:00 won, comebacks, wipe-outs,
close finishes (the winner's share under 0.6) and the mean margin. Then, for
each other rule set against the first one named (the control), the paired
counts: games where only one of the two ended in a wipe-out, games where
only one was a comeback, and games where the margin was smaller, each with
a two-sided sign test p.

Last, the snowball against a null: among games that reach the bell, how
often the side ahead at 1:00 won, and how often it would if what happens
after 1:00 were shuffled between games (no snowball at all). Each game's
part of the score banked by 1:00 is its `by60` (harness lines from round 7
on); older lines use `BY_60`, from three traced games on seed 3
(`raw/trace-s3-*.txt`: 0.46-0.47). The bell is each line's `bell` (150
before round 7). To read the held-ground score instead, pass the lines
through `held.py` first.

Usage: compare.py CONTROL_TAG FILE...
"""
import math
import random
import re
import sys
from collections import defaultdict

BY_60 = 0.46
LINE = re.compile(r"^(\w+) DUEL seed=(\d+) left=(\w+) right=(\w+) share=([\d.]+) at60=([\d.]+) .* t=(\d+)(?: by60=([\d.]+))?.*?(?: bell=(\d+))?(?: \(|$)")


def sign_p(k, n):
    if n == 0:
        return 1.0
    lo = min(k, n - k)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(lo + 1)) / 2**n)


control = sys.argv[1]
games = defaultdict(dict)  # tag -> {(seed, left, right): (share, at60, t, by60, bell)}
order = []
for f in sys.argv[2:]:
    for line in open(f):
        m = LINE.search(line)
        if not m:
            continue
        if m[1] not in order:
            order.append(m[1])
        by60 = float(m[8]) if m[8] else BY_60
        bell = int(m[9]) if m[9] else 150
        games[m[1]][(int(m[2]), m[3], m[4])] = (float(m[5]), float(m[6]), int(m[7]), by60, bell)


def facts(share, at60, t, by60, bell):
    decided = at60 != 0.5 and share != 0.5
    return {
        "wiped": t < bell,
        "comeback": decided and (at60 > 0.5) != (share > 0.5),
        "decided": decided,
        "close": abs(share - 0.5) < 0.1,
        "margin": abs(share - 0.5),
    }


print(f"{'rules':>10} {'games':>5} {'ahead at 1:00 won':>18} {'comebacks':>9} {'wipe-outs':>9} {'close':>6} {'margin':>6}")
for tag in order:
    f = [facts(*v) for v in games[tag].values()]
    d = sum(x["decided"] for x in f)
    cb = sum(x["comeback"] for x in f)
    print(f"{tag:>10} {len(f):>5} {d - cb:>10}/{d} {(d - cb) / d:>4.0%} {cb:>9} {sum(x['wiped'] for x in f):>9}"
          f" {sum(x['close'] for x in f):>6} {sum(x['margin'] for x in f) / len(f):>6.3f}")

print(f"\nPaired against {control}, game by game (only games both rule sets played):")
for tag in order:
    if tag == control:
        continue
    keys = sorted(set(games[tag]) & set(games[control]))
    a = {k: facts(*games[tag][k]) for k in keys}
    c = {k: facts(*games[control][k]) for k in keys}
    print(f"  {tag} v {control} ({len(keys)} games)")
    for name in ("wiped", "comeback", "close"):
        only_t = sum(1 for k in keys if a[k][name] and not c[k][name])
        only_c = sum(1 for k in keys if c[k][name] and not a[k][name])
        print(f"    {name:>9}: only under {tag} {only_t}, only under {control} {only_c}, p={sign_p(only_t, only_t + only_c):.2g}")
    smaller = sum(1 for k in keys if a[k]["margin"] < c[k]["margin"])
    larger = sum(1 for k in keys if a[k]["margin"] > c[k]["margin"])
    print(f"    margin smaller under {tag} in {smaller}, larger in {larger}, p={sign_p(smaller, smaller + larger):.2g}")

print("\nGames that reach the bell: the side ahead at 1:00 won, and would win with the rest of the game shuffled between games (200 shuffles):")
rng = random.Random(1)
for tag in order:
    v = [(s, a, b) for s, a, t, b, bell in games[tag].values() if t >= bell and a != 0.5 and b < 1.0]
    if not v:
        print(f"  {tag:>10}: no game reached the bell")
        continue
    real = sum((a > 0.5) == (s > 0.5) for s, a, b in v)
    # What each game scored after 1:00, as a share; shuffled between games,
    # each keeping its own lead at 1:00 and its own part banked by then.
    rest = [(s - b * a) / (1 - b) for s, a, b in v]
    won = 0
    for _ in range(200):
        rng.shuffle(rest)
        won += sum((a > 0.5) == (b * a + (1 - b) * x > 0.5) for (s, a, b), x in zip(v, rest))
    banked = sum(b for _, _, b in v) / len(v)
    print(f"  {tag:>10}: {real}/{len(v)} ({real / len(v):.0%}); shuffled {won / (200 * len(v)):.0%}; banked by 1:00 {banked:.2f}")
