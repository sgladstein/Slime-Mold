# Body measure for the report. dense = cells whose 3x3 holds walkers in 5+
# of its 9 cells; the body's outline = dense closed by a disc of radius R
# (10 fine cells: gaps up to about 20 cells wide close over), gaps it
# encloses filled, pieces under 300 cells dropped. On that outline:
# strands = share of its cells holding a walker; holes = share whose 3x3
# holds none; stacked = share of the slime's walkers in cells holding 2+.
import sys, numpy as np
from scipy import ndimage as nd
sys.path.insert(0,'/tmp/claude-0/lab'); from measure import load, box
def measure(d,sec,R=10,x0=0,x1=None,y0=0,y1=None):
    ag,o=load(d,sec); op=ag>0; occ=(o>0)&op
    dense=box(occ,1)>=5
    g=np.arange(-R,R+1); disc=np.add.outer(g**2,g**2)<=R*R
    body=nd.binary_fill_holes(nd.binary_closing(np.pad(dense,R),disc)[R:-R,R:-R])&op
    lab,n=nd.label(body); sizes=nd.sum(body,lab,range(1,n+1)); body=np.isin(lab,1+np.nonzero(sizes>=300)[0])
    m=np.zeros_like(body); m[y0:y1,x0:x1]=True; body&=m
    e=body.sum(); hole=body&(box(occ,1)==0)
    return dict(walkers=int(o.sum()),body=int(e),strand=occ[body].mean() if e else 0,holes=hole.sum()/max(e,1),stacked=o[o>1].sum()/max(o.sum(),1))
if __name__=="__main__":
    d=sys.argv[1]
    for s in map(int,sys.argv[2:]):
        m=measure(d,s); print(f"{d:>8} t={s:4d} walkers={m['walkers']:6d} outline={m['body']:6d} cells, with a walker {m['strand']*100:5.1f}%, holes {m['holes']*100:5.1f}%, walkers stacked {m['stacked']*100:5.1f}%")
