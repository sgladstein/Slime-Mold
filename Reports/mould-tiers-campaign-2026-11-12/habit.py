"""Pairs habit.sh's runs: H1-H3 (named in habit.sh), sign tests, Holm."""
import re, sys, glob, os
from math import comb
out = sys.argv[1]
def sign_p(w, n):
    if n == 0: return 1.0
    k = min(w, n - w)
    return min(1.0, 2 * sum(comb(n, i) for i in range(k + 1)) / 2 ** n)
def read(name, s):
    t = open(os.path.join(out, f"{name}-s{s}.txt")).read()
    near = {float(m.group(1)): int(m.group(2)) for m in re.finditer(r"t +([\d.]+)s  life .* near (\d+) ", t)}
    kills = [int(k) for k in re.findall(r"killed (\d+)", t)]
    return near, kills
seeds = sorted({int(re.search(r"-s(\d+)\.txt", f).group(1)) for f in glob.glob(os.path.join(out, "flee-s*.txt"))})
tests = [("H1 wary vs flee, near at 60 s (fewer)", "wary", "flee", lambda r: r[0][60.0], -1),
         ("H2 grudge vs swarm, near at 60 s (more)", "grudge", "swarm", lambda r: r[0][60.0], 1),
         ("H3 habit vs flee, killed by the last 8 strikes (more)", "habit", "flee", lambda r: sum(r[1][-8:]), 1)]
res = []
for name, a, b, f, want in tests:
    rows = [(f(read(a, s)), f(read(b, s))) for s in seeds]
    w = sum(1 for x, y in rows if (x - y) * want > 0); l = sum(1 for x, y in rows if (x - y) * want < 0)
    res.append((name, rows, w, l, sign_p(w, w + l)))
order = sorted(range(len(res)), key=lambda i: res[i][4]); holds = set()
for rank, i in enumerate(order):
    if res[i][4] * (len(res) - rank) <= 0.05: holds.add(i)
    else: break
for i, (name, rows, w, l, p) in enumerate(res):
    xs, ys = sorted(r[0] for r in rows), sorted(r[1] for r in rows)
    print(f"{name}: tier {xs[0]}-{xs[-1]} (median {xs[len(xs)//2]}), control {ys[0]}-{ys[-1]} (median {ys[len(ys)//2]}); as expected on {w}, against on {l}, of {len(rows)}; p {p:.4f}{' holds (Holm over 3)' if i in holds else ''}")
    print("   per seed:", " ".join(f"{x}/{y}" for x, y in rows))
