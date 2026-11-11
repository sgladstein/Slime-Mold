#!/usr/bin/env python3
"""Score Reports/species-sets-2026-11-11 runs: python3 ratios.py RAW_DIR.
For each seed, each species' own area (the `own` line) together over alone
at 30 s and 60 s, the Drifter's pieces together and alone, and the
together run's sorting line (`pure`, `left0`, `changed`). The deciding
measure: both ratios within 0.5-2 on 8 of 8 seeds."""
import glob, math, re, sys
d = sys.argv[1]; R = {}; S = {}
for f in glob.glob(d + '/*.txt'):
    for l in open(f):
        m = re.match(r'seed (\d+) (\S+) t\s+([\d.]+)s  own s0 walkers \d+ area ([\d.]+) pieces (\d+)  s1 walkers \d+ area ([\d.]+) pieces (\d+)', l)
        if m:
            R[(m[2], int(m[1]), float(m[3]))] = (float(m[4]), int(m[5]), float(m[6]), int(m[7]))
        m = re.match(r'seed (\d+) together t\s+([\d.]+)s  pure (\S+)  shares \S+  left0 (\S+)  changed (\S+)', l)
        if m:
            S[(int(m[1]), float(m[2]))] = (m[3], m[4], m[5])
def sign(w, n):
    k = max(w, n - w)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(k, n + 1)) / 2 ** n) if n else 1.0
for t in (30.0, 60.0):
    print(f"--- t {t:.0f} s")
    ok = 0; n = 0; fewer = more = 0
    for s in range(1, 33):
        if not all((a, s, t) in R for a in ('together', 'drifter', 'sprawler')):
            continue
        T, D, P = R[('together', s, t)], R[('drifter', s, t)], R[('sprawler', s, t)]
        rd, rs = T[0] / D[0], T[2] / P[2]
        n += 1; ok += (0.5 <= rd <= 2) and (0.5 <= rs <= 2)
        fewer += T[1] < D[1]; more += T[1] > D[1]
        pu, l0, ch = S.get((s, t), ('?', '?', '?'))
        print(f"seed {s}: Drifter {T[0]:.0f}/{D[0]:.0f} = {rd:.2f}; Sprawler {T[2]:.0f}/{P[2]:.0f} = {rs:.2f}; "
              f"Drifter pieces {T[1]} together, {D[1]} alone; pure {pu} left0 {l0} changed {ch}")
    print(f"both within 0.5-2 on {ok} of {n} seeds; Drifter pieces fewer together on {fewer}, more on {more}, "
          f"sign p {sign(fewer, fewer + more):.4f}")
