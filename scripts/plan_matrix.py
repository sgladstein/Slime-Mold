#!/usr/bin/env python3
"""The standing plan-by-tier matrix: which plan beats which mould tier.

One command runs every plan against every tier on the CPU race world and
writes the matrix with the date and commit on it, so the question "do the
strategies counter each other?" has one current answer (design review,
2026-11-12 14:13Z). Re-run it after every rule PR.

    python3 scripts/plan_matrix.py run   OUT [seeds=201-216,5-20 tiers=... threads=4 cap=900]
    python3 scripts/plan_matrix.py table OUT        # the matrix from what has run

`run` resumes: a (harness, tier) whose raw file already holds every seed is
skipped, so a run can be split across machines by `tiers=` and the raw files
copied into one OUT. Raw output goes to OUT/raw/, the matrix to
OUT/matrix.md.

Plans come from two harnesses on the same world and seeds:
- `examples/larderworld.rs`: idle, random, the scripted farm hand and the
  player AI's styles (`mould::larder_world_ai`).
- `examples/kitplans.rs`: the squad toolkit's plans (loud, quiet, plague,
  siege) and randkit (random with the tools).

The statistics (the technical lead's review, 14:13Z): paired by seed, sign
test, Holm over the whole matrix. Per tier the family is every plan against
idle and against random, and the best plan by median against the runner-up.
A cell is called best only when it beats the runner-up after Holm; otherwise
the column is a tie between them. The median paired difference stands
beside each best-against-runner-up count.
"""
import os, re, subprocess, sys, datetime, statistics as st
from math import comb

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TIERS = ['none', 'sporer', 'sleeper', 'swarmer', 'drifter', 'sprawler', 'trunk', 'grudge']
WORLD_HANDS = ['idle', 'random', 'farm', 'boom', 'claim', 'fort', 'strike']
KIT_HANDS = ['randkit', 'loud', 'quiet', 'plague', 'siege']
CONTROLS = ['idle', 'random', 'randkit']
WORLD_ROW = re.compile(r'^\s*(\w+) seed\s+(\d+):\s+(.*?) at\s+(\d+) s, score ([+-][\d.]+); soldiers\s+(\d+), mould\s+(\d+)')
KIT_ROW = re.compile(r'^\s*(\w+)\s+(\w+) seed\s+(\d+):\s+(.*?) at\s+(\d+) s, score ([+-][\d.]+); soldiers\s+(\d+), mould\s+(\d+)')


def seeds_of(spec):
    out = []
    for part in spec.split(','):
        a, _, b = part.partition('-')
        out += list(range(int(a), int(b or a) + 1))
    return out


def spans(seeds):
    """Seeds back into the harnesses' a-b form, one span per call."""
    out, start = [], seeds[0]
    for prev, s in zip(seeds, seeds[1:] + [None]):
        if s != prev + 1:
            out.append(f'{start}-{prev}')
            start = s
    return out


def read(path):
    rows = {}
    if not os.path.exists(path):
        return rows
    for line in open(path):
        m = KIT_ROW.match(line)
        if m:
            rows[(m[2], int(m[3]))] = (float(m[6]), m[4], int(m[8]))
            continue
        m = WORLD_ROW.match(line)
        if m:
            rows[(m[1], int(m[2]))] = (float(m[5]), m[3], int(m[7]))
    return rows


def run(out, args):
    seeds = seeds_of(args.get('seeds', '201-216,5-20'))
    tiers = args.get('tiers', ','.join(TIERS)).split(',')
    threads, cap = args.get('threads', '4'), args.get('cap')
    os.makedirs(f'{out}/raw', exist_ok=True)
    for ex in ('larderworld', 'kitplans'):
        subprocess.run(['cargo', 'build', '--release', '--locked', '--example', ex], cwd=ROOT, check=True)
    with open(f'{out}/raw/commit.txt', 'w') as f:
        f.write(subprocess.run(['git', 'rev-parse', 'HEAD'], cwd=ROOT, capture_output=True, text=True).stdout)
    for tier in tiers:
        for ex, hands in (('larderworld', WORLD_HANDS), ('kitplans', KIT_HANDS)):
            path = f'{out}/raw/{ex}-{tier}.txt'
            have = read(path)
            if all((h, s) in have for h in hands for s in seeds):
                continue
            with open(path, 'a') as f:
                for span in spans(seeds):
                    cmd = [f'{ROOT}/target/release/examples/{ex}', 'census' if ex == 'larderworld' else '', f'seeds={span}',
                           f'hands={",".join(hands)}', f'threads={threads}']
                    cmd += [f'tier={tier}'] if ex == 'larderworld' and tier != 'none' else []
                    cmd += [f'tiers={tier}'] if ex == 'kitplans' else []
                    cmd += [f'cap={cap}'] if cap else []
                    cmd = [c for c in cmd if c]
                    print(' '.join(cmd), flush=True)
                    subprocess.run(cmd, cwd=ROOT, stdout=f, stderr=subprocess.STDOUT, check=True)
    table(out)


def sign(a, b):
    hi = sum(x > y for x, y in zip(a, b))
    lo = sum(x < y for x, y in zip(a, b))
    n, k = hi + lo, min(hi, lo)
    return hi, lo, (min(1.0, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n) if n else 1.0)


def table(out):
    d = {}
    for tier in TIERS:
        for ex in ('larderworld', 'kitplans'):
            for (h, s), v in read(f'{out}/raw/{ex}-{tier}.txt').items():
                d[(tier, h, s)] = v
    tiers = [t for t in TIERS if any(k[0] == t for k in d)]
    hands = [h for h in WORLD_HANDS + KIT_HANDS if any(k[1] == h for k in d)]
    seeds = sorted({k[2] for k in d})
    # A stopped run leaves a tier short of seeds. Each tier is paired on the
    # seeds every hand it has data for has finished; a harness with no rows
    # on that tier shows as missing. The tier's seed count is in its row.
    tseeds = {}
    for t in tiers:
        got = [{k[2] for k in d if k[0] == t and k[1] == h} for h in hands]
        got = [g for g in got if g]
        tseeds[t] = sorted(set.intersection(*got)) if got else []
    full = lambda t, h: bool(tseeds[t]) and all((t, h, s) in d for s in tseeds[t])
    sc = lambda t, h: [d[(t, h, s)][0] for s in tseeds[t]]
    plans = [h for h in hands if h not in CONTROLS]
    # The test family, Holm over all of it.
    tests = []
    best = {}
    for t in tiers:
        ps = [h for h in plans if full(t, h)]
        if len(ps) < 2:
            continue
        for h in ps:
            for c in ('idle', 'random'):
                if full(t, c):
                    tests.append(((t, h, c),) + sign(sc(t, h), sc(t, c)))
        order = sorted(ps, key=lambda h: -st.median(sc(t, h)))
        a, b = order[0], order[1]
        diff = st.median(x - y for x, y in zip(sc(t, a), sc(t, b)))
        best[t] = (a, b, diff)
        tests.append(((t, a, b),) + sign(sc(t, a), sc(t, b)))
    m = len(tests)
    idx = sorted(range(m), key=lambda i: tests[i][3])
    adj, run_max = {}, 0.0
    for k, i in enumerate(idx):
        run_max = max(run_max, min(1.0, (m - k) * tests[i][3]))
        adj[tests[i][0]] = run_max
    raw = {t[0]: t[1:] for t in tests}
    commit = open(f'{out}/raw/commit.txt').read().strip() if os.path.exists(f'{out}/raw/commit.txt') else '?'
    prov = ''
    for line in open(f'{out}/raw/larderworld-{tiers[0]}.txt'):
        if line.startswith('larderworld '):
            prov = line.strip()
            break
    L = []
    L.append(f'# Plan-by-tier matrix\n')
    L.append(f'Made {datetime.datetime.now(datetime.timezone.utc):%Y-%m-%d %H:%MZ} from commit `{commit[:9]}` by `scripts/plan_matrix.py`; seeds {spans(seeds)} ({len(seeds)}). Raw output in `raw/`.\n')
    L.append(f'Provenance: `{prov[:300]}…`\n')
    L.append('The number after each tier is how many paired seeds it has; a stopped run leaves some short. Each cell: median `Game::score` (v2), then the ends (mould wiped / cap / squad wiped). A win is above 1; a stalemate at the cap is about −0.4 to +0.5. **Bold** is the best plan by median when it beats the runner-up after Holm over the whole matrix; otherwise the column is a tie.\n')
    L.append('| Tier | ' + ' | '.join(hands) + ' | Best |')
    L.append('| --- | ' + ' | '.join('---' for _ in hands) + ' | --- |')
    for t in tiers:
        cells = []
        a, b, diff = best.get(t, (None, None, 0.0))
        sig = a is not None and adj.get((t, a, b), 1.0) < 0.05
        for h in hands:
            if not full(t, h):
                cells.append('–')
                continue
            e = [d[(t, h, s)][1] for s in tseeds[t]]
            c = f'{st.median(sc(t, h)):+.2f} {sum("mould" in x for x in e)}/{sum("cap" in x for x in e)}/{sum("squad" in x for x in e)}'
            cells.append(f'**{c}**' if sig and h == a else c)
        if a is None:
            verdict = '–'
        else:
            hi, lo, p = raw[(t, a, b)]
            verdict = f'{a} over {b} {hi}-{lo}, median {diff:+.2f}, Holm p {adj[(t, a, b)]:.3f}' + ('' if sig else ' (tie)')
        L.append(f'| {t} ({len(tseeds[t])}) | ' + ' | '.join(cells) + f' | {verdict} |')
    L.append('\n## Each plan against idle and random (higher-lower, Holm p)\n')
    L.append('| Tier | ' + ' | '.join(plans) + ' |')
    L.append('| --- | ' + ' | '.join('---' for _ in plans) + ' |')
    for t in tiers:
        row = []
        for h in plans:
            parts = [f'{c} {raw[(t, h, c)][0]}-{raw[(t, h, c)][1]} {adj[(t, h, c)]:.3f}' for c in ('idle', 'random') if (t, h, c) in raw]
            row.append(', '.join(parts) or '–')
        L.append(f'| {t} | ' + ' | '.join(row) + ' |')
    L.append('\n## Random\'s mould wipes per tier\n')
    L.append(', '.join(f'{t} {sum("mould" in d[(t, "random", s)][1] for s in tseeds[t])}/{len(tseeds[t])}' for t in tiers if full(t, 'random')))
    L.append(f'\n{m} tests in the Holm family.')
    open(f'{out}/matrix.md', 'w').write('\n'.join(L) + '\n')
    print('\n'.join(L))


if __name__ == '__main__':
    cmd, out = sys.argv[1], sys.argv[2]
    args = dict(a.split('=', 1) for a in sys.argv[3:])
    {'run': lambda: run(out, args), 'table': lambda: table(out)}[cmd]()
