"""Plan-by-tier matrix from larderworld census output: python3 matrix.py DIR [seeds a-b].
Per tier: median score per plan, ends, the plan best on most seeds, and the
best plan's paired count against the runner-up (sign test). Per plan, its
paired count against the same plan on the plain mould."""
import re, sys, glob, os, statistics as st
from math import comb
DIR = sys.argv[1]
pat = re.compile(r'^\s*(\w+) seed\s+(\d+):\s+(.*?) at\s+(\d+) s, score ([+-][\d.]+); soldiers\s+(\d+), mould\s+(\d+)')
stuck = re.compile(r'stuck (\d+) s')
ORDER = ['none', 'sporer', 'sleeper', 'swarmer', 'drifter', 'sprawler', 'trunk', 'grudge']  # wary ran (main/wary.txt) but the tiers thread dropped it (13:26Z, e359fac)
d = {}
for t in ORDER:
    f = f'{DIR}/{t}.txt'
    if not os.path.exists(f):
        continue
    for l in open(f):
        m = pat.match(l)
        if m:
            s = stuck.search(l)
            d[(t, m[1], int(m[2]))] = (float(m[5]), m[3], int(m[7]), int(s[1]) if s else 0)
T = [t for t in ORDER if any(k[0] == t for k in d)]
S = sorted({k[2] for k in d})
H = [h for h in ['idle', 'random', 'farm', 'boom', 'claim', 'fort', 'strike'] if any(k[1] == h for k in d)]
PLANS = [h for h in H if h not in ('idle', 'random')]
def full(t, h): return all((t, h, s) in d for s in S)
def sign(a, b):
    hi = sum(x > y for x, y in zip(a, b)); lo = sum(x < y for x, y in zip(a, b)); n = hi + lo; k = min(hi, lo)
    return hi, lo, (min(1, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n) if n else 1.0)
sc = lambda t, h: [d[(t, h, s)][0] for s in S]
print(f'seeds {S[0]}-{S[-1]} ({len(S)}); median score per plan; ends wiped/cap/lost; stuck = mean seconds stuck')
print('tier     ' + ''.join(f'{h:>16}' for h in H))
for t in T:
    row = f'{t:9}'
    for h in H:
        if not full(t, h): row += f'{"-":>16}'; continue
        e = [d[(t, h, s)][1] for s in S]
        row += f'{st.median(sc(t, h)):+7.3f} {sum("mould" in x for x in e)}/{sum("cap" in x for x in e)}/{sum("squad" in x for x in e):<5}'
    print(row)
print('\nbest plan per tier (best on N seeds; best vs runner-up by median, paired)')
for t in T:
    ps = [h for h in PLANS if full(t, h)]
    best = [max(ps, key=lambda h: d[(t, h, s)][0]) for s in S]
    cnt = {h: best.count(h) for h in ps}
    by_med = sorted(ps, key=lambda h: -st.median(sc(t, h)))
    a, b = by_med[0], by_med[1]
    hi, lo, p = sign(sc(t, a), sc(t, b))
    print(f'{t:9} best-on {cnt}  top by median {a} vs {b}: {hi}-{lo} p {p:.3f}')
if 'none' in T:
    print('\neach plan against the same plan on plain (higher-lower, p)')
    for t in T[1:]:
        print(f'{t:9} ' + '  '.join(f'{h} {sign(sc(t,h), sc("none",h))[0]}-{sign(sc(t,h), sc("none",h))[1]} p {sign(sc(t,h), sc("none",h))[2]:.3f}' for h in H if full(t, h)))
print('\nmean seconds stuck a game (dead time)')
for t in T:
    print(f'{t:9} ' + '  '.join(f'{h} {st.mean(d[(t,h,s)][3] for s in S):.0f}' for h in H if full(t, h)))
