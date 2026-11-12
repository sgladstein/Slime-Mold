import sys, numpy as np
from PIL import Image
sys.path.insert(0,'.')
from specks import load, drawings, up2
from scipy import ndimage as ndi
def amber(img):
    base=np.array([12,22,26.]); top=np.array([255,190,90.])
    return (base+(top-base)*img[...,None]).clip(0,255).astype(np.uint8)
def panel(path, n0, coarse=False):
    t,c=load(path); d=drawings(t,c,n0)
    out=[]
    for k in ['trail','game','trail_blur','density']:
        im=d[k]
        if coarse: im=up2(im)
        out.append(amber(im))
    return out
rows=[]
spec=[(a.split(':')[0], float(a.split(':')[1])) for a in sys.argv[2:]]
for p,n0 in spec:
    rows.append(np.concatenate(panel(p,n0,'coarse' in p),axis=1))
    rows.append(np.full((4,rows[-1].shape[1],3),255,np.uint8))
Image.fromarray(np.concatenate(rows,axis=0)).save(sys.argv[1])
