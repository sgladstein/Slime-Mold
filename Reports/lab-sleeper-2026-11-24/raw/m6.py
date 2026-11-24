# m6.py RUN...: walkers awake/asleep a second (life.txt) and the log's squad lines
import sys, re
for d in sys.argv[1:]:
    life = {}
    for l in open(f"{d}/dump/life.txt"):
        s, a, sp, dm, _ = map(int, l.split()); life[s + 1] = (a, dm)
    log = open(f"{d}/log.txt").read()
    sq = {int(m[0]): m for m in re.findall(r"^  t (\d+) s: mould (\d+), soldiers (\d+), bank (\d+), harvested (\d+), rendered \d+, shots (\d+), killed (\d+)", log, re.M)}
    print(d)
    for t in [1, 30, 60, 90, 120, 125, 130, 140, 150, 160, 170, 180]:
        if t in life:
            a, dm = life[t]
            m = sq.get(t)
            extra = f" soldiers {m[2]} harvested {m[4]} shots {m[5]} killed {m[6]}" if m else ""
            print(f"  {t:4d}s awake {a:7d} asleep {dm:7d} all {a+dm:7d}{extra}")
