"""The layout sweep: does the field's counter cycle (hunter > maze > cord >
hunter, on scatter) hold on other draws of the same pile sampler?
Usage: analyse.py FILE...

Named before the run: on each layout, each link is 32 games (16 seeds,
sides swapped); a link holds when its winner takes more than half the food
in a sign-test majority surviving Holm over the 3 links of that layout. The
cycle holds on a layout when all three hold in the cycle's direction. Also
reported: Holm over all 24 links at once."""
import re, sys, math
from collections import defaultdict
rows = [dict(re.findall(r'(\w+)=([^\s]+)', l)) for f in sys.argv[1:] for l in open(f) if l.startswith('FIELD')]
def sign_p(k, n):
    if n == 0: return 1.0
    m = min(k, n - k)
    return min(1.0, 2 * sum(math.comb(n, i) for i in range(m + 1)) / 2 ** n)
LINKS = [('hunter', 'maze'), ('maze', 'cord'), ('cord', 'hunter')]
alltests = []
held = {}
for lay in sorted({r['layout'] for r in rows}, key=lambda s: int(s[4:])):
    tests = []
    for a, b in LINKS:
        s = [float(r['share']) if r['self'] == a else 1 - float(r['share'])
             for r in rows if r['layout'] == lay and {r['self'], r['rival']} == {a, b}]
        w = sum(x > 0.5 for x in s); t = sum(x == 0.5 for x in s); n = len(s) - t
        tests.append((sign_p(w, n), a, b, w, n, sum(s) / len(s)))
    ok = True; res = []
    for i, (p, a, b, w, n, m) in enumerate(sorted(tests)):
        ok = ok and p <= 0.05 / (3 - i)
        res.append((a, b, w, n, p, m, ok and w * 2 > n))
        alltests.append((p, lay, a, b, w, n))
    res.sort(key=lambda x: LINKS.index((x[0], x[1])))
    held[lay] = all(r[6] for r in res)
    print(f"{lay}: cycle {'HOLDS' if held[lay] else 'does not hold'}")
    for a, b, w, n, p, m, h in res:
        print(f"  {a} v {b}: {a} won {w}/{n}, mean share {m:.3f}, p {p:.2g} {'holds' if h else ('reversed' if w * 2 < n and p < 0.05 else '-')}")
print(f"cycle holds on {sum(held.values())} of {len(held)} new layouts")
alltests.sort(); ok = True; n_ok = 0
for i, (p, lay, a, b, w, n) in enumerate(alltests):
    ok = ok and p <= 0.05 / (len(alltests) - i)
    n_ok += ok
print(f"Holm over all {len(alltests)} links: {n_ok} survive (either direction)")
