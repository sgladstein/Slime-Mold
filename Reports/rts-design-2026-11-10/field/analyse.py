"""Pairings, mirrors, the cycle and the opening, from FIELD lines.

Usage: analyse.py LAYOUT FILE...   e.g. analyse.py scatter raw/fgrid-first.txt raw/runner-*.txt

A line with no layout= is scatter (the first 144 games were run before the
second layout existed). Bead is left out of the grid: its first four seeds
grew about 1,100 walkers against 6,600 for the rest, so it never meets the
other mould (see README.md).

The deciding measure, named before the runners started: a link a > b holds
when a takes more than half the food in a sign-test majority of its 32 games
(16 seeds, sides swapped) and that p survives Holm over every pairing on the
layout. A cycle is three links that hold and close. The earlier version
counted "6 of 8" games and so read a 21/32 link as held both ways."""
import re, sys, itertools, math

layout, files = sys.argv[1], sys.argv[2:]
rows = []
for f in files:
    for l in open(f):
        if not l.startswith('FIELD'):
            continue
        d = dict(re.findall(r'(\w+)=([^\s]+)', l))
        if d.get('layout', 'scatter') == layout:
            rows.append(d)
G = [g for g in ['hunter', 'mesh', 'runner', 'cord', 'maze'] if any(r['self'] == g for r in rows)]


def sign_p(k, n):
    if n == 0:
        return 1.0
    m = min(k, n - k)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(m + 1)) / 2 ** n)


print(f'{layout}: {len(rows)} games')
print('mirrors (self side share; fought a side; walkers at 60 s, both moulds)')
for g in G:
    m = [r for r in rows if r['self'] == g and r['rival'] == g]
    sh = sorted(float(r['share']) for r in m)
    touched = sum(all(int(x) > 0 for x in r['fought'].split('/')) for r in m)
    w60 = sorted(sum(map(int, r['walkers60'].split('/'))) for r in m)
    print(f'  {g:7s} {len(m)} games, share {sh[0]:.3f}-{sh[-1]:.3f}, both sides fought in {touched}/{len(m)}, walkers60 {w60[0]}-{w60[-1]}')

print('pairings (wins of games for the first named, mean share, range, sign p)')
tests = []
for a, b in itertools.combinations(G, 2):
    s = []
    for r in rows:
        if r['self'] == a and r['rival'] == b:
            s.append(float(r['share']))
        if r['self'] == b and r['rival'] == a:
            s.append(1 - float(r['share']))
    if not s:
        continue
    w = sum(x > 0.5 for x in s)
    t = sum(x == 0.5 for x in s)
    n = len(s)
    p = sign_p(w, n - t)
    tests.append((p, a, b, w, n - t))
    tie = f' ({t} tie)' if t else ''
    print(f'  {a:7s} v {b:7s} {w}/{n}{tie}  mean {sum(s)/n:.3f}  {min(s):.3f}-{max(s):.3f}  p {p:.2g}')

print(f'Holm over {len(tests)} pairings (alpha 0.05)')
beats = set()
ok = True
for i, (p, a, b, w, n) in enumerate(sorted(tests)):
    bar = 0.05 / (len(tests) - i)
    ok = ok and p <= bar
    win, lose = (a, b) if w * 2 > n else (b, a)
    if ok:
        beats.add((win, lose))
    print(f'  {win} > {lose}  p {p:.2g}  bar {bar:.4f}  {"holds" if ok else "-"}')
cyc = {tuple(sorted(c)) for c in itertools.permutations(G, 3)
       if (c[0], c[1]) in beats and (c[1], c[2]) in beats and (c[2], c[0]) in beats}
print('3-cycles that hold:', sorted(cyc) or 'none')

print('opening: does the side ahead at 30 s / 60 s still lead at the end?')
for when in ['share30', 'share60']:
    for label, pick in [('all', lambda r: True), ('mirrors', lambda r: r['self'] == r['rival'])]:
        sel = [r for r in rows if when in r and pick(r) and float(r[when]) != 0.5 and float(r['share']) != 0.5]
        if not sel:
            continue
        kept = sum((float(r[when]) > 0.5) == (float(r['share']) > 0.5) for r in sel)
        print(f'  {when} {label:7s} leader keeps the lead {kept}/{len(sel)}  p {sign_p(kept, len(sel)):.2g}')
