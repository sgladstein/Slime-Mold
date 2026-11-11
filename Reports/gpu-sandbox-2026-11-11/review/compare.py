#!/usr/bin/env python3
"""The sandbox's dishes against the CPU's, seed by seed (run.sh's output).

For each arm and measure: the CPU's range over its seeds, the GPU's, and the
paired count of seeds where the GPU is higher, with a two-sided sign-test p
(ties dropped). Bonferroni over every arm x measure tested. Run from the
repo root: python3 Reports/gpu-sandbox-2026-11-11/review/compare.py
"""
import glob, math, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, '..', '..', '..'))
JENSON = os.path.join(ROOT, 'Reports/jenson-compare-2026-11-10/raw')
SCALE = os.path.join(ROOT, 'Reports/gpu-plan-2026-11-10/scale/raw')
ARMS = [  # name, GPU raw prefix, CPU raw glob, seeds
    ('100k, one a cell, no food (preset 1, food off)', 'crowd1', os.path.join(JENSON, 'crowd1-s{}.txt'), 8),
    ('100k, no limit (preset 3)', 'crowd0', os.path.join(JENSON, 'crowd0-s{}.txt'), 8),
    ('1M, Jenson settings (preset 4)', 'jenson', os.path.join(JENSON, 'respawn0-s{}.txt'), 8),
    ('1M, respawn 1% (preset 5)', 'respawn', os.path.join(JENSON, 'respawn0.01-s{}.txt'), 8),
    ('32k, food on (preset 2)', 'fed32k', os.path.join(SCALE, 'nose-uniform-s{}.txt'), 16),
]
MEASURES = ['loops', 'width', 'tube', 'strays']
PAT = {m: re.compile(m + r'\s+([0-9.]+)') for m in MEASURES}

def read(path, cpu):
    for line in open(path):
        if (cpu and ' 10.0s ' in line) or (not cpu and line.startswith('dish at 10.0s')):
            return {m: float(PAT[m].search(line).group(1)) for m in MEASURES}
    return None

def sign_p(up, down):
    n = up + down
    if n == 0:
        return 1.0
    k = min(up, down)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(k + 1)) / 2 ** n)

def main(gpu_dir):
    rows, tests = [], 0
    for name, prefix, cpu_glob, seeds in ARMS:
        pairs = []
        for s in range(1, seeds + 1):
            g = os.path.join(gpu_dir, f'{prefix}-s{s}.txt')
            if not os.path.exists(g) or not os.path.exists(cpu_glob.format(s)):
                continue
            gm, cm = read(g, False), read(cpu_glob.format(s), True)
            if gm and cm:
                pairs.append((gm, cm))
        if not pairs:
            continue
        print(f'== {name}: {len(pairs)} seeds')
        for m in MEASURES:
            gs = [p[0][m] for p in pairs]
            cs = [p[1][m] for p in pairs]
            up = sum(g > c for g, c in zip(gs, cs))
            down = sum(g < c for g, c in zip(gs, cs))
            p = sign_p(up, down)
            tests += 1
            rows.append((name, m, p))
            mean = lambda v: sum(v) / len(v)
            print(f'  {m:6s}  CPU {min(cs):g}-{max(cs):g} (mean {mean(cs):.4g})   GPU {min(gs):g}-{max(gs):g} (mean {mean(gs):.4g})   GPU higher {up}, lower {down}, sign p {p:.3g}')
    if tests:
        print(f'Bonferroni over {tests} tests: p < {0.05 / tests:.2g} survives:', [f'{n} / {m}' for n, m, p in rows if p < 0.05 / tests] or 'none')

if __name__ == '__main__':
    import sys
    main(sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, 'raw'))
