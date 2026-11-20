#!/usr/bin/env python3
"""The Mortar's landed shells and the slime's share of its peak at each boss.

usage: shells.py 'census/shells-*.txt'   (a glob, quoted)
Per shell (from the "shell t" rows): walkers and median distance from its
target when nearest, and the most soldiers within 15 units while it stood.
Per boss formed: the slime's mass over its peak so far (the STRAT rows, every
10 s), which `BossRules::late` compares.
"""
import glob, re, statistics as st, sys

shells, forms = [], []
for f in sorted(glob.glob(sys.argv[1])):
    text = open(f).read()
    rows = {}
    for m in re.finditer(r"shell t (\d+) s: lump (\d+) at ([\d,]+), (\d+) members, median ([\d.]+) from it; (\d+) soldiers within 15", text):
        t, l, at, n, med, sol = m.groups()
        rows.setdefault((l, at), []).append((int(n), float(med), int(sol)))
    shells += rows.values()
    mass = [(int(a), int(b)) for a, b in re.findall(r"^STRAT t (\d+) \S+ mould (\d+)", text, re.M)]
    for m in re.finditer(r"boss t ([\d.]+) s: \w+ (\w+) formed", text):
        t = float(m.group(1))
        seen = [v for tt, v in mass if tt <= t]
        if seen:
            forms.append((f, m.group(2), t, seen[-1] / max(seen)))
land = [min(r, key=lambda x: x[1]) for r in shells]
if land:
    print(f"shells {len(shells)}; walkers when nearest, median {st.median(x[0] for x in land)}; "
          f"distance then, median {st.median(x[1] for x in land)}; "
          f"with a soldier within 15: {sum(1 for r in shells if max(x[2] for x in r) > 0)}; "
          f"most soldiers within 15, median {st.median(max(x[2] for x in r) for r in shells)}")
for f, kind, t, share in forms:
    print(f"{f.split('/')[-1]}: {kind} at {t:.0f} s, slime {share:.2f} of its peak")
