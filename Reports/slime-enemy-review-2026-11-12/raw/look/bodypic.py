import sys, glob, os, re, numpy as np
from PIL import Image
from scipy import ndimage as ndi
sys.path.insert(0,'.')
from specks import load, bright, box5
from bodyfield import spread
seed=int(sys.argv[1]); x0,y0,w,h,z=map(int,sys.argv[2].split(','))
def tk(p):
    b=os.path.basename(p); m=re.search(r'-t0*(\d+)\.f32',b)
    return int(m.group(1)) if m else int(re.search(r'-0*(\d+)s\.f32',b).group(1))*120
files=sorted(glob.glob(f'rawT/big-ts-s{seed}-*.f32'), key=tk)
b=None
for p in files:
    t,c=load(p); agar=t>0
    if b is None: b=np.zeros_like(t)
    for _ in range(2): b=spread(b+5.0*c,agar)*0.98
t,c=load(files[-1])
M=bright(ndi.gaussian_filter(t,3))>=0.3
core=ndi.binary_erosion(M,iterations=2)
knee=np.median(b[core])/np.median(t[core])*6.0
def amber(img):
    base=np.array([12,22,26.]); top=np.array([255,190,90.])
    return (base+(top-base)*np.clip(img,0,1)[...,None]).clip(0,255).astype(np.uint8)
ims=[amber(bright(t)), amber(bright(t)*(1-np.exp(-box5(c)/6))), amber(1-np.exp(-b/knee)), amber(np.clip(c,0,1))]
full=np.concatenate([np.concatenate(ims[:2],1),np.concatenate(ims[2:],1)],0)
Image.fromarray(full).save(f'body_full_s{seed}.png')
ims=[np.repeat(np.repeat(i[y0:y0+h,x0:x0+w],z,0),z,1) for i in ims]
sep=np.full((ims[0].shape[0],3,3),255,np.uint8)
Image.fromarray(np.concatenate([ims[0],sep,ims[1],sep,ims[2],sep,ims[3]],1)).save(f'body_crop_s{seed}.png')
