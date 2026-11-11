# stats.py: the census in raw/ (run.sh), as paired counts and sign tests.
# Living walkers at the last second of each run; fed against starved at
# 30 s, then regrowth 0.0005 < 0.001 < 0.002 at 60 s, each by seed. Holm
# over the three tests. Run from the repo root: python3 -I
# Reports/growth-2026-11-11/stats.py
import glob, re, math

D = 'Reports/growth-2026-11-11/raw'

def last(tag, sec):
    out = {}
    for f in glob.glob(f'{D}/{tag}-s*.txt'):
        s = int(re.search(r'-s(\d+)\.txt$', f).group(1))
        for line in open(f):
            m = re.search(rf't\s+{sec}\.0s\s+alive (\d+)\s+energy ([\d.]+)\s+born (\d+)\s+starved (\d+)\s+grass ([\d.]+)', line)
            if m:
                out[s] = tuple(float(x) for x in m.groups())
    return out

def series(tag):
    out = {}
    for f in glob.glob(f'{D}/{tag}-s*.txt'):
        s = int(re.search(r'-s(\d+)\.txt$', f).group(1))
        out[s] = [(float(m.group(1)), int(m.group(2))) for m in re.finditer(r't\s+([\d.]+)s\s+alive (\d+)', open(f).read())]
    return out

def sign(a, b):
    seeds = sorted(set(a) & set(b))
    up = sum(a[s][0] > b[s][0] for s in seeds)
    down = sum(a[s][0] < b[s][0] for s in seeds)
    n = up + down
    k = min(up, down)
    p = min(1.0, 2 * sum(math.comb(n, i) for i in range(k + 1)) / 2 ** n) if n else 1.0
    return up, down, len(seeds), p

fed, starved = last('fed', 30), last('starved', 30)
print('fed against starved, living walkers at 30 s (start 10,000; 100,000 slots)')
for s in sorted(fed):
    print(f'  seed {s}: fed {int(fed[s][0])} (grass {fed[s][4]:.3f}, born {int(fed[s][2])}, starved {int(fed[s][3])})  starved {int(starved[s][0])} (grass {starved[s][4]:.3f})')
for tag in ['fed', 'starved']:
    ser = series(tag)
    peaks = {s: max(series_v for _, series_v in ser[s]) for s in ser}
    print(f'  {tag}: alive at 5, 10, 20, 30 s by seed:', {s: [a for t, a in ser[s] if t in (5.0, 10.0, 20.0, 30.0)] for s in sorted(ser)})
tests = []
up, down, n, p = sign(fed, starved)
tests.append(('fed over starved at 30 s', up, down, n, p))
print(f'  fed over starved on {up} of {n} seeds, sign test p {p:.4f}')
print(f'  fed above its start on {sum(fed[s][0] > 10000 for s in fed)} of {len(fed)}; starved below on {sum(starved[s][0] < 10000 for s in starved)} of {len(starved)}')
r = {x: last(f'regrow{x}', 60) for x in ['0.0005', '0.001', '0.002']}
print('regrowth against living walkers at 60 s (512 x 320, start 2,500; 50,000 slots)')
for s in sorted(r['0.002']):
    print(f'  seed {s}: ' + '  '.join(f'{x}: {int(r[x][s][0])} (grass {r[x][s][4]:.3f})' for x in r if s in r[x]))
for a, b in [('0.001', '0.0005'), ('0.002', '0.001')]:
    up, down, n, p = sign(r[a], r[b])
    tests.append((f'regrow {a} over {b} at 60 s', up, down, n, p))
    med = sorted(r[a][s][0] / r[b][s][0] for s in r[a] if s in r[b])
    print(f'  {a} over {b} on {up} of {n} seeds, p {p:.4f}, ratio median {med[len(med) // 2]:.2f} (range {med[0]:.2f}-{med[-1]:.2f})')
print('Holm over the three:')
m = len(tests)
for i, (name, up, down, n, p) in enumerate(sorted(tests, key=lambda t: t[4])):
    print(f'  {name}: p {p:.4f} against {0.05 / (m - i):.4f}: {"survives" if p <= 0.05 / (m - i) else "does not survive"}')
