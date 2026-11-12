import sys, glob, os, re, numpy as np
from scipy import ndimage as ndi
sys.path.insert(0,'.')
from specks import load, bright
res={}
for d in sys.argv[1:]:
    for p in sorted(glob.glob(d+'/*.f32')):
        b=os.path.basename(p); tag=re.match(r'big-(\w+?)-s\d',b).group(1)
        t,c=load(p); s=2 if t.shape[1]==256 else 1
        M=(bright(ndi.gaussian_filter(t,3/s))>=0.3)&~ndi.binary_dilation(t==0,iterations=1)
        rho=ndi.gaussian_filter(c,3/s)/(s*s)   # walkers a fine cell
        thin=M&(rho<0.3)
        # body area in fine cells, thin share, walkers on thin parts share
        res.setdefault(tag,[]).append((M.sum()*s*s, thin.sum()/max(M.sum(),1), c[thin].sum()/max(c.sum(),1), c.sum()))
for k,v in res.items():
    v=np.array(v); print(f'{k:<12} body {v[:,0].mean():8.0f} fine cells  thin share of body area {v[:,1].mean():.2f} ({v[:,1].min():.2f}-{v[:,1].max():.2f})  walkers on thin {v[:,2].mean():.3f}  walkers {v[:,3].mean():.0f}')
