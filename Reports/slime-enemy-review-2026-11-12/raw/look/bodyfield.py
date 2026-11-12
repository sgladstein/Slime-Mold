"""A second, long-memory field laid by the same walkers (the sensed trail
unchanged): b <- keep_b * mean3x3(b) twice a tick (two fine steps), plus
DEP x walkers on the cell each fine step (positions known at tick ends only,
so deposits are 2 cells apart: conservative). Measured at the last tick in
the sparse body (thin tubes, as speck2.py) against the trail and the game's
body term. Brightness knee for b: chosen so b's median over the body core
draws as the trail's median does."""
import sys, glob, os, re, numpy as np
from scipy import ndimage as ndi
sys.path.insert(0,'.')
from specks import load, bright, box5
from speck2 import measures
def spread(a, agar):
    s = ndi.uniform_filter(a*agar, 3, mode='constant')*9
    n = ndi.uniform_filter(agar.astype(float), 3, mode='constant')*9
    return np.where(agar, s/np.maximum(n,1), 0)
out=[]
for seed in (1,2,3):
    def tk(p):
        b=os.path.basename(p); m=re.search(r'-t0*(\d+)\.f32',b)
        return int(m.group(1)) if m else int(re.search(r'-0*(\d+)s\.f32',b).group(1))*120
    files=sorted(glob.glob(f'rawT/big-ts-s{seed}-*.f32'), key=tk)
    assert len(files)==151, len(files)
    res={}
    for keepb in (0.9, 0.95, 0.98):
        b=None
        for p in files:
            t,c=load(p); agar=t>0
            if b is None: b=np.zeros_like(t)
            for _ in range(2):
                b=spread(b+5.0*c, agar)*keepb
        res[keepb]=b
    t,c=load(files[-1])
    M=bright(ndi.gaussian_filter(t,3))>=0.3
    core=ndi.binary_erosion(M,iterations=2)&~ndi.binary_dilation(t==0,iterations=3)
    lump=ndi.binary_dilation(ndi.gaussian_filter(c,3)>=0.3,iterations=3)
    thin=core&~lump
    tb=bright(t); game=tb*(1-np.exp(-box5(c)/6.0))
    row={'seed':seed,'thin_cells':int(thin.sum())}
    for name,I in [('trail',tb),('game',game)]:
        row[name]=measures(I,thin)
    for k,b in res.items():
        knee=np.median(b[core])/max(np.median(t[core]),1e-9)*6.0
        I=1-np.exp(-b/knee)
        row[f'body{k}']=measures(I,thin)
    out.append(row)
    print(row)
for k in ['trail','game','body0.9','body0.95','body0.98']:
    sc=np.mean([r[k][0] for r in out]); gp=np.mean([r[k][1] for r in out])
    lower=sum(r[k][0]<r['trail'][0] for r in out)
    print(f'{k:<10} SC {sc:.3f}  gaps {gp:.3f}  SC below trail on {lower}/3')
