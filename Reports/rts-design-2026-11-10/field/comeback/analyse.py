"""The comeback test, from COMEBACK lines. Usage: analyse.py FILE

Named before the run (2026-11-10 20:33Z): per verb and dish, the trailing
side's food share at the bell, copy minus untouched control, over paired
seeds, sign test, Holm over every verb and dish. A verb overturns the
opening on a dish if it wins at least half of the seeds the control loses
and its planned use beats the same verb placed at random."""
import re, sys, math
from collections import defaultdict
rows = [dict(re.findall(r'(\w+)=([^\s]+)', l)) for l in open(sys.argv[1]) if l.startswith('COMEBACK')]
def sign_p(k, n):
    if n == 0: return 1.0
    m = min(k, n - k)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(m + 1)) / 2 ** n)
by = defaultdict(dict)
for r in rows:
    by[(r['dish'], r['seed'])][(r['verb'], r['mode'])] = r
tests = []
VERBS = [('salt', 'plan'), ('salt', 'random'), ('salt', 'x6'), ('lamp', 'plan'), ('lamp', 'random'), ('lamp', 'x6'),
         ('snip', 'plan'), ('taut', 'plan'), ('molt', 'hunter'), ('molt', 'random')]
for dish in sorted({k[0] for k in by}):
    seeds = sorted({k[1] for k in by if k[0] == dish}, key=int)
    lost = [s for s in seeds if float(by[(dish, s)][('none', 'control')]['bell']) < 0.5]
    gaps = sorted(0.5 - float(by[(dish, s)][('none', 'control')]['behind60']) for s in seeds)
    print(f"== {dish}: {len(seeds)} seeds; the side behind at 60 s trailed by {gaps[0]:.3f}-{gaps[-1]:.3f}; untouched, it still lost {len(lost)}/{len(seeds)}")
    for v in VERBS:
        d, wins, turned = [], 0, 0
        for s in seeds:
            g = by[(dish, s)]
            if v not in g: continue
            b = float(g[v]['bell']); c = float(g[('none', 'control')]['bell'])
            d.append(b - c); wins += b > 0.5
            turned += (s in lost) and b > 0.5
        n = len(d); up = sum(x > 0 for x in d)
        p = sign_p(up, n); tests.append((p, dish, v))
        line = f"  {v[0]}-{v[1]:7s} bell share up {up}/{n} p {p:.4f} median {sorted(d)[n//2]:+.3f} | ahead at the bell {wins}/{n}, turned {turned}/{len(lost)} lost games"
        if v[1] in ('plan', 'x6', 'hunter'):
            rnd = (v[0], 'random')
            pr = [float(by[(dish, s)][v]['bell']) - float(by[(dish, s)][rnd]['bell']) for s in seeds if rnd in by[(dish, s)] and v in by[(dish, s)]]
            if pr:
                k = sum(x > 0 for x in pr)
                line += f" | beats random {k}/{len(pr)} p {sign_p(k, len(pr)):.4f}"
        print(line)
tests.sort()
print(f"Holm over {len(tests)} tests (alpha 0.05):")
ok = True
for i, (p, dish, v) in enumerate(tests):
    bar = 0.05 / (len(tests) - i)
    ok = ok and p <= bar
    if ok or i < 8:
        print(f"  {dish:8s} {v[0]}-{v[1]:7s} p {p:.4f} bar {bar:.4f} {'survives' if ok else '-'}")
