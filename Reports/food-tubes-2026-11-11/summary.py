#!/usr/bin/env python3
"""Descriptive figures for Reports/food-tubes-2026-11-11 (not the deciding
measure): python3 summary.py RAW_DIR. Per arm, over seeds: when the
crystals were first reached (`firsts`, trail over the tube cut, pile 0 not
counted), the main body's share of all tube area, walkers alive, and the
cost a tick (one core, four runs at a time on four cores)."""
import re, sys, glob, statistics as st
raw = sys.argv[1]
for arm in ['droplets', 'race']:
    first, last, share, alive, ms = [], [], {}, {}, {}
    for f in sorted(glob.glob(f"{raw}/{arm}-s*.txt")):
        T, J = {}, {}
        for l in open(f):
            m = re.match(r'seed \d+ \S+ firsts (.*)', l)
            if m:
                ts = sorted(float(p.split(':')[3]) for p in m[1].split() if int(p.split(':')[0]) >= 1)
                first += ts; last.append(ts[-1])
            m = re.match(r'seed \d+ \S+ t +([\d.]+)s  tube +([\d.]+).*ms/tick ([\d.]+)', l)
            if m:
                T[float(m[1])] = float(m[2]); ms.setdefault(float(m[1]), []).append(float(m[3]))
            m = re.match(r'seed \d+ \S+ t +([\d.]+)s  joined \S+ body ([\d.]+)', l)
            if m:
                J[float(m[1])] = float(m[2])
            m = re.match(r'seed \d+ \S+ t +([\d.]+)s  alive (\d+)', l)
            if m:
                alive.setdefault(float(m[1]), []).append(int(m[2]))
        for t in J:
            if T.get(t):
                share.setdefault(t, []).append(J[t] / T[t])
    print(f"{arm}:")
    print(f"  first reached: median {st.median(first):.1f} s; every crystal reached by a median {st.median(last):.1f} s (seeds {min(last):.1f}-{max(last):.1f})")
    for t in (15.0, 30.0, 60.0, 90.0):
        s = share[t]
        print(f"  main body / all tube at {t:2.0f} s: median {st.median(s):.2f} ({min(s):.2f}-{max(s):.2f})")
    for t in (5.0, 15.0, 30.0, 60.0, 90.0):
        if t in alive:
            print(f"  alive at {t:2.0f} s: median {st.median(alive[t]):.0f} ({min(alive[t])}-{max(alive[t])})")
    print("  ms a tick, median at 30 / 60 / 90 s: " + ' / '.join(f"{st.median(ms[t]):.2f}" for t in (30.0, 60.0, 90.0)) + f" (all reports {min(min(v) for v in ms.values()):.2f}-{max(max(v) for v in ms.values()):.2f})")
