"""The owner's play logs, planet by planet: how long the slime stays big and
how fast it breaks, and the army and food at the end.

    git fetch origin play-logs
    mkdir /tmp/logs && git archive FETCH_HEAD | tar -x -C /tmp/logs
    python3 Reports/balance-review-2026-11-20/play_logs.py /tmp/logs/logs

A log is one planet: a header line, then a `t=` line a second (mould,
soldiers, bank, killed, ...). A planet counts as won when the slime ends
under 1% of its peak; the time-out win (1 star at the clock, finding 2 of the
review) is caught separately by its end line. `--rows FILE` also writes the
review's chart rows: the last ten minutes of each won planet, every 15 s, as
slime left in % of its peak.
"""
import glob, json, os, statistics, sys


def kv(line):
    return dict(t.split("=", 1) for t in line.split() if "=" in t)


def mmss(s):
    return f"{int(s) // 60}:{int(s) % 60:02d}"


args = [a for a in sys.argv[1:] if not a.startswith("--")]
rows_out = sys.argv[sys.argv.index("--rows") + 1] if "--rows" in sys.argv else None
if rows_out in args:
    args.remove(rows_out)
D = args[0]

won, chart = [], []
for f in sorted(glob.glob(os.path.join(D, "*planet*.txt"))):
    L = open(f).read().splitlines()
    h = kv(L[0])
    ts = [kv(l) for l in L if l.startswith("t=")]
    if len(ts) < 100:  # a restart a few seconds in
        continue
    T = [float(d["t"]) for d in ts]
    M = [float(d["mould"]) for d in ts]
    S = [float(d["soldiers"]) for d in ts]
    B = [float(d["bank"]) for d in ts]
    end, pk = T[-1], max(M)
    ip = M.index(pk)
    above = sum(1 for m in M if m >= 0.5 * pk) / len(M)
    half = next((T[i] for i in range(ip, len(M)) if M[i] < 0.5 * pk), None)
    five = next((T[i] for i in range(ip, len(M)) if M[i] < 0.05 * pk), None)
    name = f"P{h['planet']} {os.path.basename(f)[:10]} {mmss(end)}"
    if M[-1] > 0.01 * pk:
        print(f"{name}: not won (slime {int(M[-1])} of peak {int(pk)}, soldiers {int(S[-1])})")
        continue
    collapse = (five - half) / 60 if half is not None and five is not None else None
    won.append(dict(name=name, planet=int(h["planet"]), end=end, above=above, collapse=collapse, army=int(S[-1]), food=int(B[-1])))
    print(f"{name}: peak {int(pk)}, above half the peak {100 * above:.0f}% of the planet, "
          f"half to 5% in {collapse:.1f} min, army {int(S[-1])}, food left {int(B[-1])}")
    day = os.path.basename(f)[5:10]
    for i, t in enumerate(T):
        rel = (t - end) / 60
        if rel < -10.01 or (int(t) % 15 != 0 and i != len(T) - 1):
            continue
        chart.append({"p": f"Planet {h['planet']}, {mmss(end)}", "slug": f"p{h['planet']}-{day}-{int(end)}", "t": round(rel, 2), "s": round(100 * M[i] / pk)})

print()
print(f"{len(won)} won planets")
print(f"above half the peak: median {100 * statistics.median(w['above'] for w in won):.0f}% of the planet")
print(f"half to 5%: median {statistics.median(w['collapse'] for w in won if w['collapse'] is not None):.2f} min")
for p in sorted({w["planet"] for w in won}):
    a = [w["army"] for w in won if w["planet"] == p]
    print(f"planet {p}: army at the win {min(a)} to {max(a)}; times {', '.join(mmss(w['end']) for w in won if w['planet'] == p)}")
food = [w["food"] for w in won]
print(f"food left at the win: {min(food)} to {max(food)}, over 3,500 on {sum(1 for x in food if x > 3500)} of {len(food)}")
if rows_out:
    json.dump(chart, open(rows_out, "w"))
    print(f"{len(chart)} chart rows to {rows_out}")
