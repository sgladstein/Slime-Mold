import sys, numpy as np, glob
from PIL import Image
from scipy import ndimage as nd
fs=sorted(glob.glob(sys.argv[1]+'/*.png'))
def lab(f):
    a=np.array(Image.open(f).convert('L'))>0
    a=nd.binary_closing(a,iterations=2); l,n=nd.label(a); s=nd.sum(a,l,range(1,n+1))
    return l, [i+1 for i in range(n) if s[i]>30]
# overlap-based identity: body in frame t+1 continues a body in frame t if it overlaps exactly one body and that body overlaps only it
counts=[]; events=[]
prev=None
for t,f in enumerate(fs):
    l,keep=lab(f); counts.append(len(keep))
    if prev is not None and t%10==0:
        pl,pk=prev
        # map each current body to the set of previous labels it overlaps
        splits=merges=0
        pk_set=set(pk); ck=set(keep)
        pairs=set(zip(pl[(pl>0)&(l>0)].ravel(), l[(pl>0)&(l>0)].ravel()))
        pairs={(a,b) for a,b in pairs if a in pk_set and b in ck}
        from collections import Counter
        fa=Counter(a for a,b in pairs); fb=Counter(b for a,b in pairs)
        splits=sum(1 for a,c in fa.items() if c>1); merges=sum(1 for b,c in fb.items() if c>1)
        events.append((t/10, len(keep), splits, merges))
    prev=(l,keep)
print('bodies at 1,2,5,10 s:',counts[9],counts[19],counts[49],counts[99])
for e in events: print('t %.0f s bodies %d; vs 0.1 s earlier: splits %d merges %d'%e)
