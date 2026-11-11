"""Paired counts and sign tests for run.sh (2026-11-11). Usage: pairs.py RAWDIR"""
import re, sys, glob, math, os

def life(path, t):
    for line in open(path):
        m = re.search(r"t +%s\s+life walk (\d+) spore (\d+) dormant (\d+) dead (\d+)\s+held (\d+)/\d+\s+bodies (\d+) big \d+\s+centre \S+\s+near (\d+)" % re.escape(t), line)
        if m:
            return dict(zip(["walk", "spore", "dormant", "dead", "held", "bodies", "near"], map(int, m.groups())))
    raise SystemExit(f"{path}: no life line at {t}")

def killed(path):
    return sum(int(m.group(1)) for m in re.finditer(r"killed (\d+)", open(path).read()))

def sign_p(more, less):
    n = more + less
    if n == 0:
        return 1.0
    k = min(more, less)
    p = 2 * sum(math.comb(n, i) for i in range(k + 1)) / 2 ** n
    return min(1.0, p)

D = sys.argv[1]
seeds = sorted({int(re.search(r"-s(\d+)\.txt", f).group(1)) for f in glob.glob(os.path.join(D, "t1-on-s*.txt"))})
tests = [
    ("T1 sporer: piles held at 20 s", "t1-on", "t1-off", lambda f: life(f, "20.0s")["held"]),
    ("T2 burst: piles held at 20 s", "t2-on", "t2-off", lambda f: life(f, "20.0s")["held"]),
    ("T3 bait: walkers near the strike at 16 s", "t3-on", "t3-off", lambda f: life(f, "16.0s")["near"]),
    ("T4 swarm: walkers killed by the fire", "t4-on", "t4-off", killed),
    ("T5 flee: tube bodies at 20 s", "t5-on", "t4-off", lambda f: life(f, "20.0s")["bodies"]),
]
for title, a, b, f in tests:
    on = [f(os.path.join(D, f"{a}-s{s}.txt")) for s in seeds]
    off = [f(os.path.join(D, f"{b}-s{s}.txt")) for s in seeds]
    more = sum(x > y for x, y in zip(on, off))
    less = sum(x < y for x, y in zip(on, off))
    tie = len(seeds) - more - less
    med = lambda v: sorted(v)[len(v) // 2] if len(v) % 2 else (sorted(v)[len(v) // 2 - 1] + sorted(v)[len(v) // 2]) / 2
    print(f"{title}: on {min(on)}-{max(on)} (median {med(on)}) vs off {min(off)}-{max(off)} (median {med(off)}); "
          f"higher {more}, lower {less}, tie {tie} of {len(seeds)}, sign p {sign_p(more, less):.4f}")
    print(f"   on  {on}\n   off {off}")
