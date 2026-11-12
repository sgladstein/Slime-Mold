# Re-tally of raw census lines already in the repo (no new runs).
# usage: python3 -I tally.py FILE...
import re, sys, statistics as st
rows = {}
pat = re.compile(r"\s*(\w+) seed\s+(\d+)\S*:\s+(.+?) at\s+(\d+) s, score ([-+.\d]+); soldiers\s+(\d+), mould\s+(\d+), bank\s+([\d.]+), harvested\s+([\d.]+)")
for f in sys.argv[1:]:
    for line in open(f):
        m = pat.match(line)
        if m:
            rows.setdefault(m.group(1), []).append((m.group(3).strip(), float(m.group(5)), float(m.group(8)), float(m.group(9))))
for h, r in rows.items():
    ends = {}
    for e, *_ in r:
        ends[e] = ends.get(e, 0) + 1
    un = [b / hv for _, _, b, hv in r if hv > 0]
    print(f"{h:7} n={len(r)} ends={ends} median score {st.median([s for _, s, _, _ in r]):+.3f} unspent/harvest {st.median(un) if un else float('nan'):.2f}")
