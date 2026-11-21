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
  league.py rank OUT... [--games 12]
      One row a distinct plan, with only plans that played every game.
  league.py worker [--workers 2] [--push]
      Runs for good on a spare machine (Scott's PC): plays the job files
      in queue/pc/ (one game a line), then the search queue/pc-evolve.txt
      sets (name, plans, depths, seeds, secs, top, kids, util_share,
      always: plans every generation, also: more result files in pc/ to
      breed from), 40 games at a time; between, pushes results in pc/, takes
      the branch's new commits, rebuilds the game when src/ changed and
      restarts itself when this file changed. Waits while the owner's game
      or editor is open (other bots' games don't count). queue/pc-stop on
      the branch stops it.
  league.py breed OUT... --top K --kids N --seed S > PLANS
  league.py evolve PLANS DIR --until UTC_ISO [--workers N] [--push]
      Generations on one machine until a time: plays PLANS (genN.jsonl in
      DIR), breeds the next from every game in DIR (the strategy bot's and
      the utility bot's plans bred apart, so neither crowds the other out),
      and again; with --push, commits DIR after each and pushes it to the
      branch it was fetched from.
      The next generation: the K best plans by score, kept, and N children
      made by moving one or two dials of a parent (and crossing two parents).

Score of one game: a win is 1 plus up to 1 for speed ((cap - secs) / cap);
a game not won is its progress, 1 - slime left / the slime's peak, times
0.9, so any win beats any loss. A plan's score is the mean over its games.
"""
import argparse, glob, json, math, os, random, re, subprocess, sys, time
from concurrent.futures import ThreadPoolExecutor
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
BIN = os.path.join(ROOT, "target", "release", "larder-world" + (".exe" if os.name == "nt" else ""))

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
    # Words after SECS are extra game arguments (`set=late_at=0.4`), part
    # of the job's name (2026-11-20, the balance thread's late-threat
    # measure): a job of four words keys as it always did.
    return "|".join(job)

DEADLINE = None

# A whole run's planets keep these fields in its row's `planets`.
RUN_KEYS = ("depth", "tier", "how", "secs", "mould", "start_mould", "peak_mould", "alive", "peak_alive", "bought", "killed", "pumps", "turrets", "walls", "t_rush", "t_regroup", "fell_end", "bitten_end", "stand")

def run_one(binary, job, timeout):
    """One job: `PLAN DEPTH SEED SECS` plays one planet; `PLAN run SEED
    SECS` a whole run from planet 1 until a planet is lost (depth 0 in
    its row, with each planet in `planets` and the planets won in
    `won`)."""
    if DEADLINE and time.time() > DEADLINE:
        return None
    plan, depth, seed, secs, *extra = job
    whole = depth == "run"
    cmd = [binary, f"league=run:{seed}:8:{secs}" if whole else f"league={depth}:{seed}:{secs}", f"plan={plan}", *extra]
    d = 0 if whole else int(depth)
    t0 = time.time()
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout * (8 if whole else 1), cwd=ROOT)
    except subprocess.TimeoutExpired:
        return {"job": key(job), "plan": plan, "depth": d, "seed": int(seed), "error": "timeout", "wall": time.time() - t0, "code": CODE}
    lines = [l for l in p.stdout.splitlines() if l.startswith("league ")]
    rows = [parse_line(l) for l in lines]
    if whole:
        planets = [x for x in rows if "how" in x and "tier" in x]
        final = [x for x in rows if "to" in x and "how" not in x]
        rows = [{"depth": 0, "seed": int(seed), "won": final[-1]["won"], "secs": final[-1]["secs"], "reached": max((int(x["depth"]) for x in planets), default=0),
                 "planets": [{k: x.get(k) for k in RUN_KEYS} for x in planets],
                 "shops": [x.get("bought", "") for x in rows if "bought" in x and "how" not in x]}] if final else []
    if p.returncode != 0 or not rows:
        return {"job": key(job), "plan": plan, "depth": d, "seed": int(seed), "error": (p.stderr or "")[-400:], "wall": time.time() - t0, "code": CODE}
    r = rows[-1]
    if CODE:
        r["code"] = CODE
    r["job"] = key(job)
    r["plan"] = plan
    r["cap"] = float(secs)
    if extra:
        r["extra"] = " ".join(extra)
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
            if r is None:
                continue
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
            prog = sum(1 - r.get("mould", 1) / max(r.get("peak_mould", 1), 1) for r in ds) / len(ds)
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
    "aim": [1, 2, 3, 6, 10],
    "kite": [0, 0.3, 0.5, 0.7],
    "split": [0, 1, 2, 3, 4, 6],
    "group": [2, 3, 4, 6, 8],
    "mop": [0, 100, 300, 1000, 3000],
    # 11-20: switching in the game (strat.rs `grow`, `regroup`, `spare`).
    "grow": [0, 1.2, 1.3, 1.5, 2],
    "regroup": [0, 2, 3, 4, 8],
    "spare": [0, 1, 2],
}
SHOP_KEYS = ["strike", "rate", "hp", "squad", "harvest", "range", "bank", "turret", "turret+", "pump+", "mortar", "draw"]

def dials_of(plan):
    d = {}
    if plan.startswith("util:"):
        d["@"] = "util:"
        plan = plan[5:]
    for w in plan.split(","):
        if "=" in w:
            k, v = w.split("=", 1)
            d[k] = v
        elif w:
            d["@"] = w
    return d

def plan_of(d):
    body = [f"{k}={v}" for k, v in sorted(d.items()) if k != "@"]
    if d.get("@") == "util:":
        return "util:" + ",".join(body)
    head = [d["@"]] if "@" in d else []
    return ",".join(head + body)

_WEIGHTS = {}

def util_weights():
    """The utility bot's weights and defaults, read from utility.rs once (a
    long search keeps the weights its binary was built with, even when a
    push it rebases on brings new ones)."""
    if not _WEIGHTS:
        src = open(os.path.join(ROOT, "src", "bin", "larder-world", "utility.rs")).read()
        _WEIGHTS.update({k: float(v) for k, v in re.findall(r'\("([a-z_]+)", (-?[0-9.]+), "', src)})
    return _WEIGHTS

def mutate_util(d, rng):
    """Moves three to six of a utility plan's weights by a factor of about
    e^N(0, 0.5) (or sets a zero one to a small value), and sometimes its shop."""
    d = dict(d)
    ws = util_weights()
    for k in rng.sample(sorted(ws), rng.randint(3, 6)):
        v = float(d.get(k, ws[k]))
        if k in ("per", "cap", "threat_r", "threat_n"):
            v *= math.exp(rng.gauss(0, 0.3))
        elif abs(v) < 0.05:
            v = rng.uniform(0.05, 0.5)
        else:
            v *= math.exp(rng.gauss(0, 0.5))
            if rng.random() < 0.05:
                v = 0.0
        d[k] = f"{v:.3g}"
    if rng.random() < 0.4:
        d = mutate_shop(d, rng)
    if rng.random() < 0.2:
        d["cards"] = str(rng.choice([0, 1, 2]))
    if rng.random() < 0.2:
        d["aim"] = str(rng.choice(SPACE["aim"]))
    if rng.random() < 0.2:
        d["kite"] = str(rng.choice(SPACE["kite"]))
    return d

def mutate_shop(d, rng):
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
    d = dict(d)
    d["shop"] = "/".join(shop)
    return d

def mutate(d, rng):
    if d.get("@") == "util:":
        return mutate_util(d, rng)
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
    if (a.get("@") == "util:") != (b.get("@") == "util:"):
        return a
    keys = set(a) | set(b)
    return {k: (a if rng.random() < 0.5 else b).get(k, (b if k in b else a).get(k)) for k in keys if (k in a or k in b)}

STRAT_DEFAULTS = {"pumps": "0", "walk": "4", "up": "0", "guard": "0", "turrets": "0", "walls": "0", "lure": "0", "first": "guard", "hold": "0", "go": "30", "cap": "80", "back": "0.5", "per": "200", "defend": "0", "more": "0", "take": "0", "shop": "strike/rate/hp/squad/harvest/range/bank", "cards": "1", "aim": "1", "kite": "0", "split": "0", "group": "4", "want": "", "mop": "0", "grow": "0", "regroup": "0", "spare": "0"}

def presets():
    """The strategy bot's named families, read from strat.rs."""
    src = open(os.path.join(ROOT, "src", "bin", "larder-world", "strat.rs")).read()
    return dict(re.findall(r'\("([a-z]+)", "([^"]*)"\)', src))

def canon(plan, _p={}):
    """What a plan plays, with its preset and defaults filled in, so two
    spellings of one plan (`lure` and `lure,lure=1`) count as one."""
    if not _p:
        _p.update(presets())
    d = dials_of(plan)
    d.pop("name", None)
    if d.get("@") == "util:":
        full = {k: f"{v:.3g}" for k, v in util_weights().items()}
        full.update({"shop": STRAT_DEFAULTS["shop"], "cards": "1", "aim": "1", "kite": "0", "split": "0", "group": "4", "want": "", "mop": "0", "grow": "0", "regroup": "0", "spare": "0"})
    else:
        full = dict(STRAT_DEFAULTS)
        full.update({k: v for k, v in dials_of(_p.get(d.get("@", ""), "")).items() if k != "@"})
    for k, v in d.items():
        if k == "@":
            continue
        try:
            full[k] = f"{float(v):.3g}"
        except ValueError:
            full[k] = v
    for k, v in list(full.items()):
        try:
            full[k] = f"{float(v):.3g}"
        except ValueError:
            pass
    return ("util" if d.get("@") == "util:" else "strat", tuple(sorted(full.items())))

def breed(rows, top_n, kids, seed, only=None):
    """The best `top_n` plans by mean score (of those `only` keeps) and
    `kids` new ones bred from them."""
    rng = random.Random(seed)
    by = defaultdict(list)
    for r in rows:
        if only is None or only(r["plan"]):
            by[r["plan"]].append(score(r))
    ranked = sorted(by, key=lambda p: -sum(by[p]) / len(by[p]))
    top = ranked[:top_n]
    out = list(top)
    seen = {canon(p) for p in by}
    tries = 0
    while top and len(out) < len(top) + kids and tries < 10000:
        tries += 1
        pa = dials_of(rng.choice(top))
        if rng.random() < 0.3 and len(top) > 1:
            pa = cross(pa, dials_of(rng.choice(top)), rng)
        kid = plan_of(mutate(pa, rng))
        if canon(kid) not in seen:
            seen.add(canon(kid))
            out.append(kid)
    return out

def cmd_evolve(a):
    global DEADLINE
    import datetime
    DEADLINE = datetime.datetime.fromisoformat(a.until.replace("Z", "+00:00")).timestamp()
    os.makedirs(a.dir, exist_ok=True)
    plans = [l.strip() for l in open(a.plans) if l.strip() and not l.startswith("#")]
    gen = a.gen
    while time.time() < DEADLINE:
        tag = os.path.join(a.dir, f"gen{gen}")
        open(tag + "-plans.txt", "w").write("\n".join(plans) + "\n")
        cmd_jobs(argparse.Namespace(plans=tag + "-plans.txt", out=tag + "-jobs.txt", depths=a.depths, seeds=a.seeds, secs=a.secs))
        cmd_run(argparse.Namespace(jobs=tag + "-jobs.txt", out=tag + ".jsonl", shard=None, workers=a.workers, bin=a.bin, timeout=a.timeout))
        if a.push:
            push(a.dir, f"Strategy league: generation {gen} on {os.environ.get('COMPUTERNAME', 'this machine')}")
        rows = load([os.path.join(a.dir, f) for f in os.listdir(a.dir) if f.endswith(".jsonl")] + a.also)
        half = a.kids // 2
        plans = breed(rows, a.top // 2, half, gen, lambda p: not p.startswith("util:")) + breed(rows, a.top - a.top // 2, a.kids - half, gen + 1000, lambda p: p.startswith("util:"))
        gen += 1

def push(d, msg, publish=True):
    """Commits folder `d` and pushes it to the branch the worktree came from,
    rebasing on whatever was pushed meanwhile. A machine with no git name
    commits as "Strategy league" (11-19: Scott-Desktop has none, so the
    first night's commits failed and nothing was pushed until morning).
    With publish False it only commits; the next push carries it."""
    br = "claude/project-thread-8grg6d"
    run = lambda *c: subprocess.run(c, cwd=ROOT, capture_output=True, text=True)
    who = []
    if not run("git", "config", "user.email").stdout.strip():
        who = ["-c", "user.name=Strategy league", "-c", "user.email=strategy-league@localhost.invalid"]
    run("git", "add", d)
    c = run("git", *who, "commit", "-q", "-m", msg + "\n\nNothing-new: strategy league results only.")
    if c.returncode != 0 and "nothing to commit" not in c.stdout + c.stderr and "nothing added to commit" not in c.stdout + c.stderr:
        print("league: commit failed: " + (c.stderr or c.stdout)[-300:], file=sys.stderr)
    if not publish:
        return
    for _ in range(4):
        run("git", "fetch", "-q", "origin", br)
        if run("git", *who, "rebase", "-q", "FETCH_HEAD").returncode != 0:
            run("git", "rebase", "--abort")
        p = run("git", "push", "-q", "origin", f"HEAD:refs/heads/{br}")
        if p.returncode == 0:
            return
        print("league: push failed: " + p.stderr[-300:], file=sys.stderr)
        time.sleep(10)
    print("league: could not push the results; they are in " + d, file=sys.stderr)

def cmd_rank(a):
    """Like `table`, but one row a distinct plan (spellings that play the
    same are one, and a game played twice counts once) and only plans
    with every game of the grid (`--games`, 12: planets 1-3, seeds 1-4)."""
    names, by = defaultdict(set), defaultdict(dict)
    for r in load(a.out):
        c = canon(r["plan"])
        names[c].add(r["plan"])
        by[c][(int(r["depth"]), int(r["seed"]))] = r
    res = []
    for c, g in by.items():
        if len(g) < a.games:
            continue
        sc = sum(score(r) for r in g.values()) / len(g)
        cells = []
        for d in sorted({d for d, _ in g}):
            won = [r["secs"] / 60 for (dd, _), r in g.items() if dd == d and r.get("how") == "won"]
            n = sum(1 for dd, _ in g if dd == d)
            cells.append(f"{len(won)}/{n} {sum(won) / len(won) if won else 0:4.1f}m")
        res.append((sc, cells, min(names[c], key=len)))
    print(f"score  {'  '.join(f'planet {d}    ' for d in range(1, 4))}plan")
    for sc, cells, n in sorted(res, key=lambda x: -x[0]):
        print(f"{sc:.3f}  {'  '.join(cells)}  {n}")

# ---- The PC worker (2026-11-19, Scott 14:08Z: "You can run it all day and
# night"). One process that never needs a person: it takes new bot code and
# new jobs from the branch, plays them, and pushes only results.

BRANCH = "claude/project-thread-8grg6d"
QUEUE = os.path.join(HERE, "queue", "pc")
OUT = os.path.join(HERE, "pc")

def git(*c):
    return subprocess.run(("git",) + c, cwd=ROOT, capture_output=True, text=True)

def log(msg):
    """To stderr, and to pc/worker-MACHINE.log, which goes up with the
    results so the log can be read from the branch."""
    line = time.strftime("%Y-%m-%d %H:%M:%S ") + msg
    print(line, file=sys.stderr, flush=True)
    try:
        os.makedirs(OUT, exist_ok=True)
        with open(os.path.join(OUT, f"worker-{os.environ.get('COMPUTERNAME', 'local')}.log"), "a") as f:
            f.write(line + "\n")
    except OSError:
        pass

BOT_KEYS = ("league=", "gunlab=", "runbot=", "look=", "bench=")

def owner_playing(binary):
    """True while the owner's game or editor is open: a larder-edit, or a
    larder-world not started with a bot's key (`BOT_KEYS`; 11-19: the gun
    lab's own batch ran four `gunlab=` games here, and those are not him).
    The search then waits and leaves his GPU to him."""
    if os.name == "nt":
        ps = "Get-CimInstance Win32_Process -Filter \"Name='larder-world.exe' OR Name='larder-edit.exe'\" | ForEach-Object { $_.Name + '|' + $_.CommandLine }"
        out = subprocess.run(["powershell", "-NoProfile", "-Command", ps], capture_output=True, text=True).stdout
        procs = [l.strip().split("|", 1) for l in out.splitlines() if "|" in l]
    else:
        out = subprocess.run(["ps", "-eo", "args="], capture_output=True, text=True).stdout
        procs = [[os.path.basename(l.split()[0]), l] for l in out.splitlines() if l.split() and os.path.basename(l.split()[0]) in ("larder-world", "larder-edit")]
    return any(name.startswith("larder-edit") or not any(k in cmd for k in BOT_KEYS) for name, cmd in procs)

def wait_for_owner(binary):
    if not owner_playing(binary):
        return
    log("worker: the game is open on this PC; waiting until it closes")
    while owner_playing(binary):
        time.sleep(30)
    log("worker: the game closed; going on")

def built_from():
    """What a build depends on, as the branch's HEAD has it."""
    return " ".join(git("rev-parse", f"HEAD:{x}").stdout.strip()[:12] for x in ("src", "assets", "Cargo.toml", "Cargo.lock"))

def own_text():
    return open(os.path.abspath(__file__), "rb").read()

SELF = own_text()

# Seconds between the worker's pushes. Until 11-20 it pushed after every
# chunk (about every 15 min), and each push re-ran the whole CI of the
# branch's pull request and cancelled the run before: 26 runs from 01:45 to
# 05:03Z, two of them finished, on runners the account shares with every
# lane (20 at once). Between pushes the results are committed locally.
PUSH_EVERY = 3600
LAST_PUSH = 0.0

def sync(binary, push_results, now=False):
    """Pushes results, then takes what the branch has: rebuilds the game
    when what it is built from differs from the last build, and restarts
    this script when its file differs from the one running. Both compare
    states, not commits: until 11-19 19:00Z it compared the commit before
    and after its fetch, but push() had already rebased onto the branch,
    so it never saw a change, and the PC played the kiting test on the old
    binary (27 errors) and never restarted. Returns False when the branch
    asks the worker to stop. Results are pushed at most every PUSH_EVERY
    seconds, or `now` (a job file just finished)."""
    global LAST_PUSH
    if push_results:
        go = now or time.time() - LAST_PUSH >= PUSH_EVERY
        push(OUT, f"Strategy league: results from {os.environ.get('COMPUTERNAME', 'the worker')}", publish=go)
        if go:
            LAST_PUSH = time.time()
    if git("fetch", "-q", "origin", BRANCH).returncode == 0:
        # A machine with no git name cannot replay its own result commits
        # (11-20: since results are pushed hourly there are some to replay,
        # and the PC took no new commits from 15:19Z until a push, which
        # passes a name). Same name as push().
        who = [] if git("config", "user.email").stdout.strip() else ["-c", "user.name=Strategy league", "-c", "user.email=strategy-league@localhost.invalid"]
        r = git(*who, "rebase", "-q", "FETCH_HEAD")
        if r.returncode != 0:
            git("rebase", "--abort")
            log("worker: could not take the branch's new commits: " + (r.stderr or r.stdout)[-300:])
    if built_from() != BUILT:
        log(f"worker: the game's code changed (now {git('rev-parse', '--short', 'HEAD').stdout.strip()}); building")
        build()
    if own_text() != SELF:
        log("worker: league.py changed; restarting")
        subprocess.Popen([sys.executable] + sys.argv, cwd=ROOT, stdout=sys.stdout, stderr=sys.stderr)
        os._exit(0)
    return not os.path.exists(os.path.join(HERE, "queue", "pc-stop"))

CODE = None
BUILT = None

def build():
    """Builds the game; CODE becomes the tree of src/ it was built from,
    stamped on every game so a result is never reused across code."""
    global CODE, BUILT
    t0 = time.time()
    want = built_from()
    b = subprocess.run(["cargo", "build", "--release", "--locked", "--bin", "larder-world"], cwd=ROOT, capture_output=True, text=True)
    if b.returncode == 0:
        CODE = git("rev-parse", "HEAD:src").stdout.strip()[:12]
        BUILT = want
        log(f"worker: built the game in {time.time() - t0:.0f} s (src {CODE})")
    else:
        BUILT = want  # no retry until the code changes again
        log("worker: the build failed; playing on with the last binary: " + b.stderr[-600:])

def pending(jobs_path, out_path):
    """The games of a job file not yet played. A file whose comment lines
    say `code-bound` counts only games played on the current build (11-20:
    main's Heavier shots fix and new clock came in while jobs were queued;
    a job whose base and variants straddle two games compares nothing),
    so a new build plays it again and its rows are told apart by `code`.
    Codes named on that line (`# code-bound: 1eec8cf8f4b3`) count too: a
    build that changes only a dial its jobs leave at its default plays the
    same games, so the rows it has keep (11-20 18:00Z, `fire` became a
    share)."""
    lines = [l for l in open(jobs_path) if l.strip()]
    bound = any(l.startswith("#") and "code-bound" in l for l in lines)
    also = {w for l in lines if l.startswith("#") and "code-bound" in l for w in re.findall(r"\b[0-9a-f]{12}\b", l)}
    jobs = [tuple(l.split()) for l in lines if not l.startswith("#")]
    done = set()
    if os.path.exists(out_path):
        for l in open(out_path):
            try:
                r = json.loads(l)
            except json.JSONDecodeError:
                continue
            if bound and r.get("code") != CODE and r.get("code") not in also:
                continue
            if "error" not in r or r.get("code") == CODE:
                done.add(r["job"])
    return [j for j in jobs if key(j) not in done]

def play(todo, out_path, binary, workers, timeout):
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    def one(j):
        wait_for_owner(binary)
        return run_one(binary, j, timeout)
    with open(out_path, "a") as f, ThreadPoolExecutor(workers) as ex:
        for r in ex.map(one, todo):
            if r is None:
                continue
            f.write(json.dumps(r) + "\n")
            f.flush()
            tag = r.get("how", r.get("error", "?")[:200])
            log(f"  {r['plan'][:60]} planet {r.get('depth')} seed {r.get('seed')}: {tag} {r.get('secs', '')} ({r.get('wall', 0):.0f} s)")

def read_cfg(path):
    cfg = {}
    for l in open(path):
        l = l.split("#")[0].strip()
        if "=" in l:
            k, v = l.split("=", 1)
            cfg[k.strip()] = v.strip()
    return cfg

EVOLVE_NEXT = 0

def next_work(chunk):
    """The next games: the first job file in queue/pc/ (by name) with games
    left, else the next chunk of one of the searches that the files
    queue/pc-evolve*.txt set, taking them in turn (11-20: planet 2 and
    planet 3 side by side). Returns (games, results file) or None."""
    global EVOLVE_NEXT
    if os.path.isdir(QUEUE):
        for name in sorted(os.listdir(QUEUE)):
            if name.endswith(".txt"):
                out = os.path.join(OUT, name[:-4] + ".jsonl")
                todo = pending(os.path.join(QUEUE, name), out)
                if todo:
                    return todo[:chunk], out
    cfgs = sorted(glob.glob(os.path.join(HERE, "queue", "pc-evolve*.txt")))
    for i in range(len(cfgs)):
        k = (EVOLVE_NEXT + i) % len(cfgs)
        w = search_work(cfgs[k], chunk)
        if w:
            EVOLVE_NEXT = (k + 1) % len(cfgs)
            return w
    return None

def search_work(cfg_path, chunk):
    """The next chunk of the search that file `cfg_path` sets, breeding its
    next generation when the last one is played. None when it has nothing
    new to play."""
    c = read_cfg(cfg_path)
    d = os.path.join(OUT, c["name"])
    os.makedirs(d, exist_ok=True)
    gens = sorted(int(m.group(1)) for f in os.listdir(d) for m in [re.fullmatch(r"gen(\d+)-jobs\.txt", f)] if m)
    gen = gens[-1] if gens else 0
    if gen:
        todo = pending(os.path.join(d, f"gen{gen}-jobs.txt"), os.path.join(d, f"gen{gen}.jsonl"))
        if todo:
            return todo[:chunk], os.path.join(d, f"gen{gen}.jsonl")
    depths = {int(x) for x in c.get("depths", "3").split(",")}
    also = [os.path.join(OUT, f) for f in c.get("also", "").split(",") if f and os.path.exists(os.path.join(OUT, f))]
    rows = [r for r in load([os.path.join(d, f) for f in os.listdir(d) if f.endswith(".jsonl")] + also) if int(r["depth"]) in depths]
    if gen == 0:
        plans = [l.strip() for l in open(os.path.join(ROOT, c["plans"])) if l.strip() and not l.startswith("#")]
    else:
        share = float(c.get("util_share", 0.25))
        top, kids = int(c.get("top", 8)), int(c.get("kids", 24))
        ut, uk = round(top * share), round(kids * share)
        plans = breed(rows, top - ut, kids - uk, gen, lambda p: not p.startswith("util:"))
        if ut:
            plans += breed(rows, ut, uk, gen + 1000, lambda p: p.startswith("util:"))
    plans += [p for p in c.get("always", "").split(";") if p and p not in plans]
    # A plan this code already played on a seed plays it the same again
    # (one machine, one binary): the kept parents' old games stand in.
    seen = {(canon(r["plan"]), int(r["depth"]), int(r["seed"])) for r in rows if r.get("code") == CODE}
    jobs = [(p, dd, s, c.get("secs", "720")) for s in seeds_of(c.get("seeds", "1-8")) for dd in sorted(depths) for p in plans if (canon(p), dd, s) not in seen]
    if not jobs:
        log(f"worker: search {c['name']} has no new games to play")
        return None
    gen += 1
    tag = os.path.join(d, f"gen{gen}")
    open(tag + "-plans.txt", "w").write("\n".join(plans) + "\n")
    open(tag + "-jobs.txt", "w").write("".join(f"{p} {dd} {s} {secs}\n" for p, dd, s, secs in jobs))
    log(f"worker: search {c['name']} generation {gen}: {len(plans)} plans, {len(jobs)} new games")
    return pending(tag + "-jobs.txt", tag + ".jsonl")[:chunk], tag + ".jsonl"

def cmd_worker(a):
    log(f"worker: started in {ROOT}, {a.workers} games at once")
    build()
    last = False
    while sync(a.bin, a.push, now=last):
        work = next_work(a.chunk)
        if not work:
            log("worker: nothing queued; looking again in 5 minutes")
            time.sleep(300)
            continue
        todo, out = work
        play(todo, out, a.bin, a.workers, a.timeout)
        # A short chunk was the file's last: push it now, not within the hour.
        last = len(todo) < a.chunk
    log("worker: queue/pc-stop is on the branch; stopping")

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
    r.add_argument("--bin", default=BIN)
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
    k = sub.add_parser("rank")
    k.add_argument("out", nargs="+")
    k.add_argument("--games", type=int, default=12)
    e = sub.add_parser("evolve")
    e.add_argument("plans")
    e.add_argument("dir")
    e.add_argument("--until", required=True)
    e.add_argument("--gen", type=int, default=1)
    e.add_argument("--depths", default="1,2,3")
    e.add_argument("--seeds", default="1-4")
    e.add_argument("--secs", default="720")
    e.add_argument("--workers", type=int, default=2)
    e.add_argument("--top", type=int, default=8)
    e.add_argument("--kids", type=int, default=24)
    e.add_argument("--timeout", type=float, default=1800)
    e.add_argument("--also", nargs="*", default=[])
    e.add_argument("--push", action="store_true")
    e.add_argument("--bin", default=BIN)
    w = sub.add_parser("worker")
    w.add_argument("--workers", type=int, default=2)
    w.add_argument("--chunk", type=int, default=40)
    w.add_argument("--timeout", type=float, default=1800)
    w.add_argument("--push", action="store_true")
    w.add_argument("--bin", default=BIN)
    a = ap.parse_args()
    {"run": cmd_run, "jobs": cmd_jobs, "table": cmd_table, "breed": cmd_breed, "evolve": cmd_evolve, "rank": cmd_rank, "worker": cmd_worker}[a.cmd](a)

if __name__ == "__main__":
    main()
