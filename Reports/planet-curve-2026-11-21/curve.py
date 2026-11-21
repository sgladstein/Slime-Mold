"""Summarise a planet-curve league file: wins, slime size and how the losses
happened, by planet. Usage: python3 curve.py 00-b-curve.jsonl"""
import collections
import json
import statistics as st
import sys

rows = [json.loads(l) for l in open(sys.argv[1]) if l.startswith("{")]
for d in sorted({int(r["depth"]) for r in rows}):
    R = [r for r in rows if int(r["depth"]) == d]
    L = [r for r in R if r["how"] != "won"]
    big = [r for r in L if r["peak_alive"] >= 24]
    late = [r for r in big if r["mould"] / r["peak_mould"] < 0.6]
    heart = [r for r in big if r["mould"] / r["peak_mould"] < 0.2]
    print(f"planet {d} ({R[0]['tier']}): won {sum(r['how'] == 'won' for r in R)} of {len(R)}; "
          f"median start slime {st.median(r['start_mould'] for r in R):.0f}, peak {st.median(r['peak_mould'] for r in R):.0f}; "
          f"losses {len(L)} ({dict(collections.Counter(r['how'] for r in L))}), bitten {sum(r['bitten_end'] for r in L):.0f}, "
          f"burned {sum(r['burned_end'] for r in L):.0f}; big-squad losses {len(big)}, under 60% of peak {len(late)}, under 20% {len(heart)}")
    for plan in sorted({r["plan"] for r in R}):
        P = [r for r in R if r["plan"] == plan]
        w = [r["secs"] / 60 for r in P if r["how"] == "won"]
        print(f"   {plan[:60]}: {len(w)} of {len(P)}" + (f", mean win {sum(w) / len(w):.1f} min" if w else ""))
