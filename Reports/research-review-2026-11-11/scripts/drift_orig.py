# drift.py DIR: bodies in the last frame of DIR (count-style PNGs), their size, and how far their centres moved over the last 20 frames (nearest centre). Needs numpy, pillow, scipy.
import sys, numpy as np, glob
from PIL import Image
from scipy import ndimage as nd
fs=sorted(glob.glob(sys.argv[1]+'/*.png'))
def cents(f):
    a=np.array(Image.open(f).convert('L'))>0
    a=nd.binary_closing(a,iterations=2); l,n=nd.label(a); s=nd.sum(a,l,range(1,n+1)); keep=[i+1 for i in range(n) if s[i]>30]
    return np.array(nd.center_of_mass(a,l,keep)), s[s>30]
i,j=len(fs)-21,len(fs)-1
c1,s1=cents(fs[i]); c2,_=cents(fs[j])
d=np.sqrt(((c1[:,None,:]-c2[None,:,:])**2).sum(-1)).min(1)
print(len(c1),'bodies, median size',np.median(s1),'drift over',j-i,'frames: median',round(np.median(d),2),'p90',round(np.percentile(d,90),2))
