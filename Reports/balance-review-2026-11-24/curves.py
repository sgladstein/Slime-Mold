"""Per-planet curves from the owner's play logs (play-logs branch).

git fetch origin play-logs; python3 Reports/balance-review-2026-11-24/curves.py [SINCE]
For each log: peak slime and when, minutes the slime spent at >= half its
peak, the minutes from half to 5% of peak (the collapse), peak soldiers,
soldiers when the slime first fell under half, food banked at the end,
soldiers lost (drops in the soldier count), end line.
"""
import re, subprocess, sys
since = sys.argv[1] if len(sys.argv) > 1 else "2026-11-22"
names = subprocess.run(["git", "ls-tree", "-r", "--name-only", "origin/play-logs"],
                       capture_output=True, text=True).stdout.split()
rows = []
for n in sorted(names):
    if "planet-" not in n or n.split("/")[-1] < "logs/"[:0] + since:
        continue
    if n.split("/")[-1][:10] < since:
        continue
    txt = subprocess.run(["git", "show", "origin/play-logs:" + n], capture_output=True, text=True).stdout
    head = txt.split("\n", 1)[0]
    kv = dict(re.findall(r"(\w+)=(\S+)", head))
    ts = []
    for m in re.finditer(r"^t=(\d+) mould=(\d+) .*?soldiers=(\d+) .*?bank=(\d+) income=(\d+)", txt, re.M):
        ts.append(tuple(int(x) for x in m.groups()))
    if len(ts) < 30:
        continue
    end = re.search(r"^end (.*)$", txt, re.M)
    peak = max(ts, key=lambda r: r[1])
    pk, pt = peak[1], peak[0]
    above = sum(1 for r in ts if r[1] >= pk / 2)
    half = next((r for r in ts if r[0] > pt and r[1] < pk / 2), None)
    five = next((r for r in ts if r[0] > pt and r[1] < pk * 0.05), None)
    lost = sum(max(0, a[2] - b[2]) for a, b in zip(ts, ts[1:]))
    bursts = max((ts[i][2] - ts[i + 5][2]) for i in range(len(ts) - 5))
    bosses = len(re.findall(r"^boss .* formed", txt, re.M))
    broken = len(re.findall(r"^boss .*ended=Broken", txt, re.M))
    raids = len(re.findall(r"^raid ", txt, re.M))
    rows.append(dict(
        when=kv.get("when", "")[:16], build=kv.get("build", "").split(":")[0], planet=kv.get("planet"),
        tier=kv.get("tier"), len=f"{ts[-1][0]//60}:{ts[-1][0]%60:02d}",
        peak=pk, peak_at=f"{pt/60:.1f}", above=f"{above/60:.1f}",
        collapse=f"{(five[0]-half[0])/60:.1f}" if half and five else "-",
        sold_peak=max(r[2] for r in ts), sold_half=half[2] if half else "-",
        lost=lost, burst5s=bursts, bank_end=ts[-1][3], bosses=f"{broken}/{bosses}", raids=raids,
        end=(end.group(1) if end else "(none)")[:28], bought=kv.get("bought", "")))
cols = ["when", "build", "planet", "tier", "len", "peak", "peak_at", "above", "collapse",
        "sold_peak", "sold_half", "lost", "burst5s", "bank_end", "bosses", "raids", "end"]
print("| " + " | ".join(cols) + " |")
print("|" + "---|" * len(cols))
for r in rows:
    print("| " + " | ".join(str(r[c]) for c in cols) + " |")
print()
for r in rows:
    print(r["when"], "P" + str(r["planet"]), r["bought"])
