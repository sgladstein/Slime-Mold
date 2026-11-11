#!/usr/bin/env python3
"""Per-arm totals for Reports/first-reach-2026-11-11: python3 summary.py RAW_DIR.
Scored piles (index >= 1, more than 66 dish cells from the start) reached by
30 s and 60 s, split by radius fraction (under 0.8, or 0.8 and up: near the
wall), the median first-reach time, the farthest pile reached, and the median
walker extent (r50, r90 in fine cells; the start disc is 200 fine cells, so a
still disc has r50 141 and r90 190) and ms a tick."""
import glob, re, statistics as st, sys
out = sys.argv[1]; R = {}; X = {}
for f in glob.glob(out + '/*.txt'):
    for l in open(f):
        m = re.match(r'seed (\d+) (\S+) firsts (.*)', l)
        if m:
            R[(m[2], int(m[1]))] = [tuple(float(v) for v in p.split(':')) for p in m[3].split()]
        m = re.match(r'seed (\d+) (\S+) t +([\d.]+)s +extent r50 (\S+) r90 (\S+)', l)
        if m:
            X.setdefault((m[2], float(m[3])), []).append((float(m[4]), float(m[5])))
        m = re.match(r'seed (\d+) (\S+) t +([\d.]+)s +tube .*ms/tick +(\S+)', l)
        if m:
            X.setdefault((m[2], float(m[3]), 'ms'), []).append(float(m[4]))
arms = ['droplets', 'dropscouts', 'dropfast', 'scoutsonly', 'grazers', 'grazescouts']
for a in arms:
    seeds = sorted(s for (n, s) in R if n == a)
    sc = [p for s in seeds for p in R[(a, s)] if p[0] >= 1 and p[1] > 66]
    r30 = [p for p in sc if 0 <= p[3] <= 30]; r60 = [p for p in sc if 0 <= p[3] <= 60]
    inn = [p for p in sc if p[2] < 0.8]; out_ = [p for p in sc if p[2] >= 0.8]
    i30 = sum(1 for p in inn if 0 <= p[3] <= 30); o30 = sum(1 for p in out_ if 0 <= p[3] <= 30)
    t = [p[3] for p in r60]
    far = max((p[1] for p in r60), default=0)
    print(f"{a:12s} seeds {len(seeds)}  scored {len(sc)}  by 30 s {len(r30)} ({len(r30)/len(sc):.2f})  by 60 s {len(r60)} ({len(r60)/len(sc):.2f})  "
          f"inner {i30}/{len(inn)} wall {o30}/{len(out_)} by 30 s  median first {st.median(t) if t else -1:.1f} s  farthest {far:.0f}")
    row = []
    for T in (10.0, 30.0, 60.0):
        e = X.get((a, T), [])
        ms = X.get((a, T, 'ms'), [])
        if e:
            row.append(f"{T:.0f} s r50 {st.median(v[0] for v in e):.0f} r90 {st.median(v[1] for v in e):.0f} ms/tick {st.median(ms):.1f}")
    print(' ' * 13 + ' | '.join(row))
