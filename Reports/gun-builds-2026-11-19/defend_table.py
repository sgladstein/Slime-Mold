"""Defend table (2026-11-20): python3 defend_table.py raw/defend-pc.txt

arena=9:guard: nine soldiers hold the slimiest crystal with Guard and
Defend. Per deck and planet, medians over seeds paired against the empty
deck: slime removed while firing, walkers on the crystal (mean; lower is
better), health lost and soldiers alive at 90 s. W/L counts the seeds where
the deck removed more / less slime."""
import re, sys, statistics as st
from collections import defaultdict
rows = [dict(re.findall(r"(\w+)=(\S+)", l)) for l in open(sys.argv[1]) if l.startswith("gunlab")]
m = defaultdict(dict)
for r in rows:
    if r.get("how") in ("unfit", "error"): continue
    m[(r["depth"], r["deck"])][r["seed"]] = (int(r["start_mould"]) - int(r["mould_fire"]), float(r["on_crystal"]), float(r["hp_lost_fire"]), int(r["alive_fire"]))
decks = sorted({d for _, d in m})
def cell(depth, d):
    x, y = m.get((depth, d)), m.get((depth, "none"))
    if not x or not y: return None
    ss = [s for s in x if s in y]
    dk = [x[s][0] - y[s][0] for s in ss]
    return (st.median(dk), sum(v > 0 for v in dk), sum(v < 0 for v in dk), st.median(x[s][1] - y[s][1] for s in ss), st.median(x[s][2] - y[s][2] for s in ss), st.median(x[s][3] for s in ss))
out = []
for d in decks:
    if d == "none": continue
    cs = [cell(p, d) for p in "1234"]
    score = sum(c[0] for c in cs if c) / max(1, sum(1 for c in cs if c))
    out.append((score, d, cs))
out.sort(reverse=True)
print("| deck | " + " | ".join(f"planet {p}" for p in "1234") + " |")
print("|---|---|---|---|---|")
for score, d, cs in out:
    f = lambda c: "unfit" if c is None else f"{c[0]:+.0f} ({c[1]}/{c[2]}) crystal {c[3]:+.0f} hp {c[4]:+.1f} alive {c[5]:.0f}"
    print(f"| {d} | " + " | ".join(f(c) for c in cs) + " |")
print()
for p in "1234":
    y = m[(p, "none")]
    print(f"planet {p} empty deck: removed {st.median(v[0] for v in y.values()):.0f}, crystal {st.median(v[1] for v in y.values()):.0f}, hp lost {st.median(v[2] for v in y.values()):.1f}, alive {st.median(v[3] for v in y.values()):.0f}")
