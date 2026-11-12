import sys, numpy as np
from PIL import Image
sys.path.insert(0,'.')
from specks import load, bright, box5, up2
from speck2 import shade
from scipy import ndimage as ndi
def amber(img):
    base=np.array([12,22,26.]); top=np.array([255,190,90.])
    return (base+(top-base)*np.clip(img,0,1)[...,None]).clip(0,255).astype(np.uint8)
def grey(img): return (np.clip(img,0,1)*255).astype(np.uint8)[...,None].repeat(3,-1)
out=sys.argv[1]; x0,y0,w,h,z=map(int,sys.argv[2].split(','))
rows=[]
for spec in sys.argv[3:]:
    p,n0=spec.split(':'); n0=float(n0)
    t,c=load(p); co=t.shape[1]==256
    tb=bright(t); g=tb*(1-np.exp(-box5(c)/n0))
    ims=[amber(tb), amber(g), grey(shade(g)), amber(np.clip(c,0,1))]
    if co: ims=[np.repeat(np.repeat(i,2,0),2,1) for i in ims]
    ims=[np.repeat(np.repeat(i[y0:y0+h,x0:x0+w],z,0),z,1) for i in ims]
    sep=np.full((ims[0].shape[0],3,3),255,np.uint8)
    rows.append(np.concatenate(sum([[i,sep] for i in ims],[])[:-1],axis=1))
    rows.append(np.full((3,rows[-1].shape[1],3),255,np.uint8))
Image.fromarray(np.concatenate(rows[:-1],axis=0)).save(out)
