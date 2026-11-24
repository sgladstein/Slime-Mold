# The body's measure from LAB_DUMP: envelope = open cells whose 21x21 box is
# at least 20% walkers (the body's region at a coarse scale); fill = share
# of envelope cells holding a walker; holes = share of envelope cells whose
# 3x3 holds none; cover = envelope cells; walkers = sum.
import sys, numpy as np
def load(d, sec):
    a = open(f"{d}/agar.bin","rb").read()
    nl = a.index(b"\n"); w,h = map(int,a[:nl].split()); ag = np.frombuffer(a[nl+1:],np.uint8).reshape(h,w)
    o = np.frombuffer(open(f"{d}/occ-{sec:04d}.bin","rb").read(),np.uint8).reshape(h,w)
    return ag,o
def box(x,r):
    c=np.pad(x.astype(np.int64),((r+1,r),(r+1,r))).cumsum(0).cumsum(1)
    k=2*r+1
    return c[k:,k:]-c[:-k,k:]-c[k:,:-k]+c[:-k,:-k]
def measure(d,sec):
    ag,o=load(d,sec); occ=(o>0)&(ag>0); op=ag>0
    R=6; n_open=box(op,R); env=op&(box(occ,R)>=0.3*np.maximum(n_open,1))
    hole=env&(box(occ,1)==0)
    e=env.sum()
    return dict(sec=sec,walkers=int(o.sum()),cover=int(e),fill=occ[env].mean() if e else 0,holes=hole.sum()/max(e,1),multi=int((o>1).sum()))
if __name__=="__main__":
    d=sys.argv[1]
    for s in map(int,sys.argv[2:]):
        m=measure(d,s); print(f"{d.split('/')[-1]:>12} t={m['sec']:4d} walkers={m['walkers']:6d} body={m['cover']:6d} cells fill={m['fill']*100:5.1f}% holes={m['holes']*100:5.1f}%")
