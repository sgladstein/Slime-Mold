#!/usr/bin/env python3
"""The strategy league's driver (2026-11-19, the thread "Build a smarter bot
that explores strategies"). It runs `larder-world league=...` games, many at a
time, and reads their `league` lines into JSON lines; it also searches plans.

  league.py run JOBS OUT [--workers N] [--shard I/N] [--bin PATH]
      JOBS: one game a line, "PLAN DEPTH SEED SECS" (PLAN has no spaces).
      Appends one JSON object a game to OUT; a job already in OUT is skipped,
      so a stopped run picks up where it was.
  league.py jobs PLANS OUT --depths 1,2,3 --seeds 1-6 --secs 720
      Writes the cross of plans (one a line) by depths by seeds as JOBS.
  league.py table OUT...
      Prints each plan's wins, mean time to win and mean progress by planet.
  league.py breed OUT... --top K --kids N --seed S > PLANS
      The next generation: the K best plans by score, kept, and N children
      made by moving one or two dials of a parent (and crossing two parents).

Score of one game: a win is 1 plus up to 1 for speed ((cap - secs) / cap);
a game not won is its progress, 1 - slime left / the slime's peak, times
0.9, so any win beats any loss. A plan's score is the mean over its games.
"""
import argparse, json, os, random, re, subprocess, sys, time
from concurrent.futures import ThreadPoolExecutor
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))

def parse_line(line):
    """A `league ...` line into a dict (numbers as numbers)."""
    out = {}
    for w in line.split()[1:]:
        if "=" not in w:
            continue
        k, v = w.split("=", 1)
        try:
            out[k] = float(v) if re.fullmatch(r"-?\d+(\.\d+)?", v) else v
        except ValueError:
            out[k] = v
    return out

def key(job):
    return f"{job[0]}|{job[1]}|{job[2]}|{job[3]}"

def run_one(binary, job, timeout):
    plan, depth, seed, secs = job
    cmd = [binary, f"league={depth}:{seed}:{secs}", f"plan={plan}"]
    t0 = time.time()
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, cwd=ROOT)
    except subprocess.TimeoutExpired:
        return {"job": key(job), "plan": plan, "depth": int(depth), "seed": int(seed), "error": "timeout", "wall": time.time() - t0}
    rows = [parse_line(l) for l in p.stdout.splitlines() if l.startswith("league ")]
    if p.returncode != 0 or not rows:
        return {"job": key(job), "plan": plan, "depth": int(depth), "seed": int(seed), "error": (p.stderr or "")[-400:], "wall": time.time() - t0}
    r = rows[-1]
    r["job"] = key(job)
    r["plan"] = plan
    r["cap"] = float(secs)
    return r

def cmd_run(a):
    jobs = [tuple(l.split()) for l in open(a.jobs) if l.strip() and not l.startswith("#")]
    if a.shard:
        i, n = map(int, a.shard.split("/"))
        jobs = jobs[i::n]
    done = set()
    if os.path.exists(a.out):
        for l in open(a.out):
            try:
                r = json.loads(l)
                if "error" not in r:
                    done.add(r["job"])
            except json.JSONDecodeError:
                pass
    todo = [j for j in jobs if key(j) not in done]
    print(f"league: {len(todo)} of {len(jobs)} games to play, {a.workers} at once", file=sys.stderr)
    with open(a.out, "a") as f, ThreadPoolExecutor(a.workers) as ex:
        for r in ex.map(lambda j: run_one(a.bin, j, a.timeout), todo):
            f.write(json.dumps(r) + "\n")
            f.flush()
            tag = r.get("how", r.get("error", "?"))
            print(f"  {r['plan'][:60]} planet {r.get('depth')} seed {r.get('seed')}: {tag} {r.get('secs', '')} ({r.get('wall', 0):.0f} s)", file=sys.stderr)

def seeds_of(s):
    out = []
    for part in s.split(","):
        if "-" in part:
            lo, hi = part.split("-")
            out += list(range(int(lo), int(hi) + 1))
        else:
            out.append(int(part))
    return out

def cmd_jobs(a):
    plans = [l.strip() for l in open(a.plans) if l.strip() and not l.startswith("#")]
    with open(a.out, "w") as f:
        for s in seeds_of(a.seeds):
            for d in a.depths.split(","):
                for p in plans:
                    f.write(f"{p} {d} {s} {a.secs}\n")

def load(paths):
    rows = []
    for p in paths:
        for l in open(p):
            try:
                r = json.loads(l)
            except json.JSONDecodeError:
                continue
            if "error" not in r:
                rows.append(r)
    return rows

def score(r):
    cap = r.get("cap") or 720.0
    if r.get("how") == "won":
        return 1.0 + max(0.0, (cap - r["secs"]) / cap)
    peak = max(r.get("peak_mould", 1), 1)
    return 0.9 * max(0.0, 1.0 - r.get("mould", peak) / peak)

def cmd_table(a):
    rows = load(a.out)
    by = defaultdict(list)
    for r in rows:
        by[r["plan"]].append(r)
    depths = sorted({int(r["depth"]) for r in rows})
    lines = []
    for plan, rs in by.items():
        s = sum(score(r) for r in rs) / len(rs)
        cells = []
        for d in depths:
            ds = [r for r in rs if int(r["depth"]) == d]
            if not ds:
                cells.append("-")
                continue
            w = [r for r in ds if r["how"] == "won"]
            t = sum(r["secs"] for r in w) / len(w) if w else 0
            prog = sum(1 - r["mould"] / max(r["peak_mould"], 1) for r in ds) / len(ds)
            cells.append(f"{len(w)}/{len(ds)} {t/60:4.1f}m {prog:.2f}")
        lines.append((s, plan, cells))
    lines.sort(reverse=True)
    print("score  " + "  ".join(f"planet {d:<12}" for d in depths) + "  plan")
    for s, plan, cells in lines:
        print(f"{s:5.3f}  " + "  ".join(f"{c:<19}" for c in cells) + "  " + plan)

# The dials the search moves, and the values each may take.
SPACE = {
    "pumps": [0, 1, 2, 3, 4, 6],
    "walk": [1, 2, 4, 8],
    "up": [0, 1, 2],
    "guard": [0, 1, 2, 3],
    "turrets": [0, 2, 3, 4, 6],
    "walls": [0, 1, 2],
    "lure": [0, 1],
    "first": ["guard", "pumps"],
    "hold": [0, 4, 8, 12],
    "go": [0, 10, 20, 30, 40, 60, 80],
    "cap": [40, 80, 120, 200],
    "back": [0.3, 0.5, 0.7],
    "per": [100, 150, 200, 300],
    "defend": [0, 1],
    "more": [0, 2, 4, 8],
    "take": [0, 1],
    "cards": [0, 1, 2],
}
SHOP_KEYS = ["strike", "rate", "hp", "squad", "harvest", "range", "bank", "turret", "turret+", "pump+", "mortar", "draw"]

def dials_of(plan):
    d = {}
    for w in plan.split(","):
        if "=" in w:
            k, v = w.split("=", 1)
            d[k] = v
        elif w:
            d["@"] = w
    return d

def plan_of(d):
    head = [d["@"]] if "@" in d else []
    return ",".join(head + [f"{k}={v}" for k, v in sorted(d.items()) if k != "@"])

def mutate(d, rng):
    d = dict(d)
    for _ in range(rng.choice([1, 1, 2])):
        k = rng.choice(list(SPACE) + ["shop"])
        if k == "shop":
            shop = d.get("shop", "strike/rate/hp/squad/harvest/range/bank").split("/")
            op = rng.choice(["swap", "front", "drop", "add"])
            if op == "swap" and len(shop) > 1:
                i, j = rng.sample(range(len(shop)), 2)
                shop[i], shop[j] = shop[j], shop[i]
            elif op == "front":
                x = rng.choice(SHOP_KEYS)
                shop = [x] + [s for s in shop if s != x]
            elif op == "drop" and len(shop) > 2:
                shop.pop(rng.randrange(len(shop)))
            else:
                x = rng.choice([s for s in SHOP_KEYS if s not in shop] or SHOP_KEYS)
                shop.insert(rng.randrange(len(shop) + 1), x)
            d["shop"] = "/".join(shop)
        else:
            d[k] = str(rng.choice(SPACE[k]))
    return d

def cross(a, b, rng):
    keys = set(a) | set(b)
    return {k: (a if rng.random() < 0.5 else b).get(k, (b if k in b else a).get(k)) for k in keys if (k in a or k in b)}

def cmd_breed(a):
    rng = random.Random(a.seed)
    rows = load(a.out)
    by = defaultdict(list)
    for r in rows:
        by[r["plan"]].append(score(r))
    ranked = sorted(by, key=lambda p: -sum(by[p]) / len(by[p]))
    top = ranked[: a.top]
    out = list(top)
    seen = set(out)
    tries = 0
    while len(out) < a.top + a.kids and tries < 10000:
        tries += 1
        pa = dials_of(rng.choice(top))
        if rng.random() < 0.3 and len(top) > 1:
            pa = cross(pa, dials_of(rng.choice(top)), rng)
        kid = plan_of(mutate(pa, rng))
        if kid not in seen:
            seen.add(kid)
            out.append(kid)
    print("\n".join(out))

def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    r = sub.add_parser("run")
    r.add_argument("jobs")
    r.add_argument("out")
    r.add_argument("--workers", type=int, default=1)
    r.add_argument("--shard")
    r.add_argument("--timeout", type=float, default=3600)
    r.add_argument("--bin", default=os.path.join(ROOT, "target", "release", "larder-world"))
    j = sub.add_parser("jobs")
    j.add_argument("plans")
    j.add_argument("out")
    j.add_argument("--depths", default="1,2,3")
    j.add_argument("--seeds", default="1-4")
    j.add_argument("--secs", default="720")
    t = sub.add_parser("table")
    t.add_argument("out", nargs="+")
    b = sub.add_parser("breed")
    b.add_argument("out", nargs="+")
    b.add_argument("--top", type=int, default=6)
    b.add_argument("--kids", type=int, default=12)
    b.add_argument("--seed", type=int, default=1)
    a = ap.parse_args()
    {"run": cmd_run, "jobs": cmd_jobs, "table": cmd_table, "breed": cmd_breed}[a.cmd](a)

if __name__ == "__main__":
    main()
