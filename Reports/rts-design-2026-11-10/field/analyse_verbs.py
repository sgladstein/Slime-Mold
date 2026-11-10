"""The verb census, from VERB lines. Usage: analyse_verbs.py FILE

For each dish and verb: planned minus control food share at +30 s (seeds
where the plan gained / lost, sign p), planned minus random, the same on
territory, and the 5 s change near the action against the one-walker nudge.
Holm over every plan-v-control test."""
import re, sys, math
from collections import defaultdict
rows = [dict(re.findall(r'(\w+)=([^\s]+)', l)) for l in open(sys.argv[1]) if l.startswith('VERB')]
def sign_p(k, n):
    if n == 0: return 1.0
    m = min(k, n - k)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(m + 1)) / 2 ** n)
by = defaultdict(dict)
for r in rows:
    by[(r['dish'], r['seed'])][(r['verb'], r['mode'])] = r
tests = []
for dish in sorted({k[0] for k in by}):
    seeds = sorted({k[1] for k in by if k[0] == dish}, key=int)
    print(f"== {dish}, {len(seeds)} seeds")
    for verb in ['lamp', 'oats', 'salt', 'snip', 'taut', 'molt']:
        d_pc, d_pr, t_pc, vis_gt = [], [], [], 0
        for s in seeds:
            g = by[(dish, s)]
            if (verb, 'plan') not in g: continue
            p = g[(verb, 'plan')]
            d_pc.append(float(p['share30']) - float(p['ctl30']))
            t_pc.append(float(p['terr30']) - float(p['ctlterr30']))
            if (verb, 'random') in g:
                d_pr.append(float(p['share30']) - float(g[(verb, 'random')]['share30']))
            if float(p['vis5']) > float(g[('nudge', 'null')]['vis5']): vis_gt += 1
        n = len(d_pc)
        up = sum(x > 0 for x in d_pc); big = sum(x >= 0.02 for x in d_pc)
        p1 = sign_p(up, n); tests.append((p1, dish, verb))
        line = f"  {verb:5s} plan-ctl share: up {up}/{n} (>=0.02: {big}) p {p1:.4f} median {sorted(d_pc)[n//2]:+.3f} range {min(d_pc):+.3f}..{max(d_pc):+.3f}"
        if d_pr:
            ur = sum(x > 0 for x in d_pr)
            line += f" | plan>random {ur}/{len(d_pr)} p {sign_p(ur, len(d_pr)):.4f}"
        tu = sum(x > 0 for x in t_pc)
        line += f" | territory up {tu}/{n} | 5 s change > nudge {vis_gt}/{n}"
        print(line)
tests.sort()
m = len(tests)
print(f"Holm over {m} plan-v-control tests (alpha 0.05):")
ok = True
for i, (p, dish, verb) in enumerate(tests):
    bar = 0.05 / (m - i)
    surv = ok and p <= bar
    ok = surv
    print(f"  {dish:8s} {verb:5s} p {p:.4f} bar {bar:.4f} {'survives' if surv else '-'}")
