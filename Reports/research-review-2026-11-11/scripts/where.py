import sys, numpy as np
W,H=1024,640
yy,xx=np.mgrid[0:H,0:W]
r=np.hypot(xx+0.5-W/2,yy+0.5-H/2)
# normalised elliptical radius: 1 at the agar rim (rx=500, ry=308)
e=np.hypot((xx+0.5-W/2)/500,(yy+0.5-H/2)/308)
for f in sys.argv[1:]:
    a=np.fromfile(f,dtype='<f4'); t=a[:W*H].reshape(H,W); c=a[W*H:].reshape(H,W)
    tube=t>1.0
    n=c.sum()
    print(f.split('/')[-1], 'walkers',int(n), 'tube fine cells',int(tube.sum()),'= dish cells',tube.sum()/16)
    for lo,hi in [(0,0.3),(0.3,0.6),(0.6,0.85),(0.85,1.01)]:
        m=(e>=lo)&(e<hi)
        print(f'  ellip r {lo:.2f}-{hi:.2f}: walkers {int(c[m].sum()):8d} ({c[m].sum()/n:.4f})  tube cells {int((tube&m).sum()):7d}  max trail {t[m].max():9.2f}  p99.9 trail all {np.percentile(t[t>0],99.9):.1f}')
