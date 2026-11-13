"""Summarise cutprobe runs (slime-enemy thread, 2026-11-13): the share of
the cut piece alive over time, the largest 5 s drop, and the uncut mould at
3:00 (the grown line), paired by seed against the first run named.

    python3 -I summ.py news-c-shared news-n02 news-n03

reads NAME.txt and NAME-series.csv beside this script (one arm a run)."""
import re
import sys
from math import comb

def series(path):
    out = {}
    for line in open(path).read().splitlines()[1:]:
        f = line.split(',')
        key = (f[0], int(f[1]), int(f[2]))
        out.setdefault(key, {})[int(f[3])] = int(f[4])
    return out

def grown(path):
    g = {}
    for line in open(path):
        m = re.match(r'(\S+) (\d+) grown \| alive (\d+) .*eaten from (\d+) of', line)
        if m:
            g[int(m[2])] = (int(m[3]), int(m[4]))
    tail = [l for l in open(path) if 'cut_ms' not in l and re.match(r'\S+ \d+ \d+ \|', l)]
    ms = [float(l.split()[-1]) for l in tail]
    return g, (sum(ms) / len(ms) if ms else 0)

def sign_p(a, b):
    n = a + b
    if n == 0:
        return 1.0
    k = min(a, b)
    return min(1.0, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n)

T = [5, 10, 15, 20, 30, 45, 60, 90, 120]
names = sys.argv[1:]
import os; d = os.path.dirname(os.path.abspath(sys.argv[0]))
base_g = None
for n in names:
    s = series(f'{d}/{n}-series.csv')
    g, ms = grown(f'{d}/{n}.txt')
    share = {t: [] for t in T}
    drops = []
    for key, ts in s.items():
        n0 = ts[0]
        for t in T:
            share[t].append(ts.get(t, 0) / n0)
        drops.append(max((ts.get(t, 0) - ts.get(t + 5, 0)) / n0 for t in range(0, 116)))
    print(n, 'cuts', len(s), 'tick ms', round(ms, 2))
    print('  alive share', ' '.join(f'{t}s {sum(v)/len(v):.2f}' for t, v in share.items()))
    print('  max 5 s drop: mean %.2f max %.2f' % (sum(drops) / len(drops), max(drops)))
    seeds = sorted(g)
    print('  grown', ' '.join(f's{k}:{g[k][0]}/{g[k][1]}' for k in seeds))
    if base_g is None:
        base_g, base_n = g, n
    else:
        smaller = sum(1 for k in seeds if g[k][0] < base_g[k][0])
        bigger = sum(1 for k in seeds if g[k][0] > base_g[k][0])
        pct = [100 * (g[k][0] / base_g[k][0] - 1) for k in seeds]
        print('  vs %s: smaller %d bigger %d, p %.3f; change %s%%' % (base_n, smaller, bigger, sign_p(smaller, bigger), ' '.join(f'{p:+.1f}' for p in pct)))
