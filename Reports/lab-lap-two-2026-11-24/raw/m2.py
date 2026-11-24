# m2.py: the smoke tests' table from each run's log and dump
import re, os, glob, struct
import numpy as np
B="/tmp/claude-0/lab/lap2"
def last(log, pat):
    m=re.findall(pat, log, re.M); return m[-1] if m else None
for pid in range(9,16):
    for arm in ["base","pair"]:
        d=f"{B}/p{pid}_{arm}"
        if not os.path.exists(f"{d}/log.txt") or "wall" not in open(f"{d}/log.txt").read(): continue
        log=open(f"{d}/log.txt").read()
        m=last(log, r"^  t (\d+) s: mould (\d+), soldiers (\d+), bank (\d+), harvested (\d+), rendered \d+, shots (\d+), killed (\d+)[^\n]*")
        ended = "ended" in (re.findall(r"^  t \d+ s: mould[^\n]*", log, re.M) or [""])[-1]
        life={}
        if os.path.exists(f"{d}/dump/life.txt"):
            for l in open(f"{d}/dump/life.txt"):
                s,a,sp,dm,_=map(int,l.split()); life[s+1]=(a,sp,dm)
        sp_max=max((v[1] for v in life.values()), default=0)
        awake = [round(v[0]/max(v[0]+v[2],1),2) for t,v in sorted(life.items()) if t in (30,60,90,120)]
        # walkers on scar over 100, last dump
        scar=""
        sc=sorted(glob.glob(f"{d}/dump/scar-*.bin")); oc=sorted(glob.glob(f"{d}/dump/occ-*.bin"))
        if sc and oc:
            s=np.fromfile(sc[-1],dtype=np.float32); o=np.fromfile(oc[-1],dtype=np.uint8)
            n=min(len(s),len(o)); scar=f"on scar>100 {int(o[:n][s[:n]>100].sum())}, scar max {s.max():.0f}"
        pumps=re.findall(r"pump at \([^)]*\): hp (\d+) / 200[^\n]*", log)
        stand=len(re.findall(r"last stand", log, re.I))
        wall=last(log, r"wall (\d+)s")
        print(f"p{pid} {arm}: t {m[0]} mould {m[1]} soldiers {m[2]} harvested {m[4]} shots {m[5]} killed {m[6]}{' ENDED' if ended else ''}; spores max {sp_max}; awake {awake}; {scar}; pumps hp {pumps[-4:]}; 'last stand' lines {stand}; wall {wall}")
