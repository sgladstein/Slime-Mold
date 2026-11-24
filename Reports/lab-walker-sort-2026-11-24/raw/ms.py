# ms.py: sorted vs not, paired by seed: walkers, kills, soldiers, spread (from the last occ dump)
import re, glob, os, numpy as np
B="/tmp/claude-0/lab/sort"
def one(d):
    log=open(f"{d}/log.txt").read()
    if "wall" not in log: return None
    m=re.findall(r"^  t (\d+) s: mould (\d+), soldiers (\d+), bank \d+, harvested (\d+), rendered \d+, shots (\d+), killed (\d+)", log, re.M)[-1]
    hd=open(f"{d}/dump/agar.bin","rb").read().split(b"\n",1)[0].split(); w,h=int(hd[0]),int(hd[1])
    o=np.fromfile(sorted(glob.glob(f"{d}/dump/occ-*.bin"))[-1],dtype=np.uint8).reshape(h,w).astype(float)
    ys,xs=np.nonzero(o); wt=o[ys,xs]; cx,cy=(xs*wt).sum()/wt.sum(),(ys*wt).sum()/wt.sum()
    rg=np.sqrt((((xs-cx)**2+(ys-cy)**2)*wt).sum()/wt.sum())
    loc=re.findall(r"tiles a warp ([\d.]+) -> ([\d.]+)", log)
    lb=np.mean([float(a) for a,_ in loc[1:]]) if len(loc)>1 else None
    return dict(t=int(m[0]),mould=int(m[1]),soldiers=int(m[2]),harvested=int(m[3]),shots=int(m[4]),killed=int(m[5]),cells=len(xs),rg=rg,before=lb)
for p,seeds in [(8,[1,2,3]),(2,[1,2,3,4])]:
    for s in seeds:
        a,b=[one(f"{B}/p{p}s{s}_{arm}") if os.path.exists(f"{B}/p{p}s{s}_{arm}/log.txt") else None for arm in ("off","on")]
        if not a or not b: continue
        f=lambda k: f"{k} {a[k]} / {b[k]} ({(b[k]-a[k])/max(a[k],1)*100:+.1f}%)"
        print(f"planet {p} seed {s} t {a['t']}/{b['t']}: "+"; ".join(f(k) for k in ["mould","killed","soldiers","harvested","cells"])+f"; rg {a['rg']:.1f} / {b['rg']:.1f}; tiles a warp 1 s after a sort {b['before']}")
