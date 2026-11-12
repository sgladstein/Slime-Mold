"""Variant of bodyfield.py: long memory, little spread:
b <- keep * ((1-mix) b + mix mean3x3(b)) + 5 x walkers, two fine steps a tick."""
import sys, glob, os, re, numpy as np
from PIL import Image
from scipy import ndimage as ndi
sys.path.insert(0,'.')
from specks import load, bright, box5
from speck2 import measures
def spread(a, agar):
    s = ndi.uniform_filter(a*agar, 3, mode='constant')*9
    n = ndi.uniform_filter(agar.astype(float), 3, mode='constant')*9
    return np.where(agar, s/np.maximum(n,1), 0)
def tk(p):
    b=os.path.basename(p); m=re.search(r'-t0*(\d+)\.f32',b)
    return int(m.group(1)) if m else int(re.search(r'-0*(\d+)s\.f32',b).group(1))*120
arms=[(0.98,1.0),(0.98,0.25),(0.97,0.25),(0.99,0.1)]
rows=[]
for seed in (1,2,3):
    files=sorted(glob.glob(f'rawT/big-ts-s{seed}-*.f32'), key=tk)
    t,c=load(files[-1])
    M=bright(ndi.gaussian_filter(t,3))>=0.3
    core=ndi.binary_erosion(M,iterations=2)&~ndi.binary_dilation(t==0,iterations=3)
    lump=ndi.binary_dilation(ndi.gaussian_filter(c,3)>=0.3,iterations=3)
    thin=core&~lump
    row={'trail':measures(bright(t),thin),'game':measures(bright(t)*(1-np.exp(-box5(c)/6)),thin)}
    pics=[]
    for keep,mix in arms:
        b=np.zeros_like(t)
        for p in files:
            tt,cc=load(p); agar=tt>0
            for _ in range(2):
                b=b+5.0*cc
                b=((1-mix)*b+mix*spread(b,agar))*keep*agar
        knee=np.median(b[core])/np.median(t[core])*6.0
        I=1-np.exp(-b/knee)
        row[(keep,mix)]=measures(I,thin)
        pics.append(I)
    rows.append(row)
    if seed==1:
        def amber(img):
            base=np.array([12,22,26.]); top=np.array([255,190,90.])
            return (base+(top-base)*np.clip(img,0,1)[...,None]).clip(0,255).astype(np.uint8)
        x0,y0,w,h,z=230,10,150,110,3
        ims=[amber(bright(t))]+[amber(I) for I in pics[1:3]]
        ims=[np.repeat(np.repeat(i[y0:y0+h,x0:x0+w],z,0),z,1) for i in ims]
        sep=np.full((ims[0].shape[0],3,3),255,np.uint8)
        Image.fromarray(np.concatenate([ims[0],sep,ims[1],sep,ims[2]],1)).save('body2_crop_s1.png')
        Image.fromarray(np.concatenate([amber(bright(t)),amber(pics[1])],1)).save('body2_full_s1.png')
for k in ['trail','game']+arms:
    print(k, 'SC %.3f gaps %.3f' % (np.mean([r[k][0] for r in rows]), np.mean([r[k][1] for r in rows])), 'SC<trail %d/3' % sum(r[k][0]<r['trail'][0] for r in rows))
