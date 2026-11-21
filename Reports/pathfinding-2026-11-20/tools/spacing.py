# Nearest-neighbour spacing of standing soldiers (speed under 0.5) from a
# LARDER_PATHLOG CSV, sampled from a third of the way in: median, share
# under elbow room (2.4 dish cells) and under a drawn figure (3.45).
# python3 spacing.py path.csv
import math, sys
from collections import defaultdict
rows=defaultdict(list)
for l in open(sys.argv[1]):
    p=l.split(',')
    if p[0]!='s': continue
    try: rows[int(p[1])].append((float(p[4]),float(p[5]),math.hypot(float(p[6]),float(p[7])),p[11]))
    except: pass
ts=sorted(rows)
nn=[]; orders=defaultdict(int)
for t in ts[len(ts)//3::15]:
    s=rows[t]
    for i,a in enumerate(s):
        if a[2]>0.5: continue
        d=min((math.hypot(a[0]-b[0],a[1]-b[1]) for j,b in enumerate(s) if j!=i), default=99)
        nn.append(d); orders[a[3]]+=1
nn.sort()
q=lambda f: nn[int(len(nn)*f)]
print('standing samples',len(nn),'nearest neighbour (dish cells): p10 %.2f p25 %.2f median %.2f p75 %.2f'%(q(.1),q(.25),q(.5),q(.75)), 'under 2.4: %.2f under 3.0: %.2f under 3.45: %.2f'%(sum(d<2.4 for d in nn)/len(nn),sum(d<3.0 for d in nn)/len(nn), sum(d<3.45 for d in nn)/len(nn)), dict(orders))
