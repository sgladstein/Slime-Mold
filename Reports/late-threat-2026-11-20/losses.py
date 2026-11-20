"""Soldier losses against army size in Scott's play logs.

Deaths in second k = soldiers[k-1] + buys in (k-1, k] - soldiers[k], kept when > 0.
(A buy the bank refused would show as a negative and is dropped; a death in
the same second as a refused buy would be missed, so this undercounts a little.)
"""
import re, sys, glob, os, collections
logs = sorted(glob.glob(os.path.join(sys.argv[1], '*planet-*.txt')))
bins = [(1, 4), (5, 8), (9, 15), (16, 25), (26, 40), (41, 999)]
expo = collections.Counter(); died = collections.Counter()
phase_expo = collections.Counter(); phase_died = collections.Counter()
per_game = []
for f in logs:
    rows = []; buys = collections.Counter(); end = None
    for line in open(f):
        if line.startswith('t='):
            d = dict(kv.split('=', 1) for kv in line.split())
            rows.append((int(float(d['t'])), int(d['soldiers']), int(d['mould']), int(d['killed'])))
        elif line.startswith('cmd') and ' buy' in line:
            t = float(line.split()[1][2:]); buys[int(t) + 1] += 1
        elif line.startswith('end'):
            end = line.strip()
    if len(rows) < 30: continue
    peak = max(r[2] for r in rows)
    T = rows[-1][0]
    total = 0; maxs = 0; late = 0
    for (t0, s0, m0, k0), (t1, s1, m1, k1) in zip(rows, rows[1:]):
        b = sum(buys[t] for t in range(t0 + 1, t1 + 1))
        d = max(0, s0 + b - s1)
        total += d; maxs = max(maxs, s1)
        for lo, hi in bins:
            if lo <= s0 <= hi:
                expo[(lo, hi)] += (t1 - t0); died[(lo, hi)] += d
        # phase by slime left: >=50% of peak, 20-50%, <20%
        frac = m0 / peak
        ph = 'slime >= half' if frac >= 0.5 else ('slime 20-50%' if frac >= 0.2 else 'slime < 20%')
        phase_expo[ph] += (t1 - t0); phase_died[ph] += d
        if t0 >= 0.75 * T: late += d
    name = os.path.basename(f)[:20] + ' P' + re.search(r'planet-(\d+)', f).group(1)
    per_game.append((name, T, maxs, total, late, rows[-1][2] / peak, end or ''))
print('game                       secs  max soldiers  deaths  deaths in last quarter  slime left/peak  end')
for g in per_game:
    print(f'{g[0]:26s} {g[1]:5d} {g[2]:6d} {g[3]:12d} {g[4]:10d} {g[5]:18.2f}   {g[6]}')
print()
print('army size   soldier-minutes  deaths  deaths per soldier-minute  deaths per minute')
for lo, hi in bins:
    e = expo[(lo, hi)]; d = died[(lo, hi)]
    if e == 0: continue
    # soldier-minutes needs the mean size; approximate by exposure-weighted midpoint is poor, so recompute below
    print(f'{lo:>3}-{hi:<4}  {e/60:8.1f} min  {d:6d}  {"":>10}  {d/(e/60):8.2f}')
print()
for ph in ['slime >= half', 'slime 20-50%', 'slime < 20%']:
    e = phase_expo[ph]; d = phase_died[ph]
    print(f'{ph:14s} {e/60:7.1f} min  deaths {d:4d}  per minute {d/(e/60) if e else 0:.2f}')
