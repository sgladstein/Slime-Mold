"""Paired counts and sign tests for world/run.sh (2026-11-11). Usage: pairs.py RAWDIR"""
import re, sys, glob, math, os

def life(path, t):
    for line in open(path):
        m = re.search(r"t +%s\s+life walk (\d+) spore (\d+) dormant (\d+) dead (\d+)\s+held (\d+)/\d+\s+bodies (\d+) big \d+\s+centre \S+\s+near (\d+)" % re.escape(t), line)
        if m:
            return dict(zip(["walk", "spore", "dormant", "dead", "held", "bodies", "near"], map(int, m.groups())))
    raise SystemExit(f"{path}: no life line at {t}")

def alive(path, t):
    for line in open(path):
        m = re.search(r"t +%s\s+alive (\d+)" % re.escape(t), line)
        if m:
            return int(m.group(1))
    raise SystemExit(f"{path}: no alive line at {t}")

def follow_up_kills(path):
    # Every strike but the first (10 s): the 10 that meet the answer to it.
    ks = [(float(m.group(1)), int(m.group(2))) for m in re.finditer(r"t +([\d.]+)s\s+strike at \S+ r \S+: killed (\d+)", open(path).read())]
    return sum(k for t, k in ks if t > 10.01)

def sign_p(more, less):
    n = more + less
    if n == 0:
        return 1.0
    k = min(more, less)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(k + 1)) / 2 ** n)

med = lambda v: sorted(v)[len(v) // 2] if len(v) % 2 else (sorted(v)[len(v) // 2 - 1] + sorted(v)[len(v) // 2]) / 2

D = sys.argv[1]
seeds = sorted({int(re.search(r"-s(\d+)\.txt", f).group(1)) for f in glob.glob(os.path.join(D, "plain-s*.txt"))})
tests = [
    ("W1 threat: walkers alive at 60 s", "sporer", "plain", lambda f: alive(f, "60.0s"), True),
    ("W2 burst: piles held at 40 s", "burst-on", "burst-off", lambda f: life(f, "40.0s")["held"], True),
    ("W3 bait: walkers near the strike at 16 s", "bait-on", "bait-off", lambda f: life(f, "16.0s")["near"], True),
    ("W4 swarm: walkers killed by the 10 follow-up strikes", "swarm-on", "swarm-off", follow_up_kills, True),
    ("(shown) Sporer piles held at 10 s", "sporer", "plain", lambda f: life(f, "10.0s")["held"], False),
    ("(shown) Sporer piles held at 60 s", "sporer", "plain", lambda f: life(f, "60.0s")["held"], False),
    ("(shown) Sleeper walkers alive at 60 s", "sleeper", "plain", lambda f: alive(f, "60.0s"), False),
    ("(shown) Sleeper piles held at 60 s", "sleeper", "plain", lambda f: life(f, "60.0s")["held"], False),
    ("(shown) Sleeper dormant at 60 s", "sleeper", "plain", lambda f: life(f, "60.0s")["dormant"], False),
    ("(shown) Sporer crystals left at 60 s (per mille)", "sporer", "plain", lambda f: crystals(f, "60.0s"), False),
]

def crystals(path, t):
    for line in open(path):
        m = re.search(r"t +%s\s+tube.*crystals ([\d.]+)" % re.escape(t), line)
        if m:
            return round(1000 * float(m.group(1)))
    raise SystemExit(f"{path}: no tube line at {t}")

for title, a, b, f, test in tests:
    on = [f(os.path.join(D, f"{a}-s{s}.txt")) for s in seeds]
    off = [f(os.path.join(D, f"{b}-s{s}.txt")) for s in seeds]
    more = sum(x > y for x, y in zip(on, off))
    less = sum(x < y for x, y in zip(on, off))
    tie = len(seeds) - more - less
    p = sign_p(more, less)
    tag = f", sign p {p:.4f}{' (survives x4)' if p * 4 < 0.05 else ''}" if test else ""
    print(f"{title}: {a} {min(on)}-{max(on)} (median {med(on)}) vs {b} {min(off)}-{max(off)} (median {med(off)}); "
          f"higher {more}, lower {less}, tie {tie} of {len(seeds)}{tag}")
    print(f"   {a:9} {on}\n   {b:9} {off}")
