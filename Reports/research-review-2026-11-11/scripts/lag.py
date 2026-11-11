# Track bodies by nearest-centre chaining frame to frame, then net displacement over lags (frames are 0.1 s apart).
import sys, numpy as np, glob
from PIL import Image
from scipy import ndimage as nd
fs=sorted(glob.glob(sys.argv[1]+'/*.png'))
def cents(f):
    a=np.array(Image.open(f).convert('L'))>0
    a=nd.binary_closing(a,iterations=2); l,n=nd.label(a); s=nd.sum(a,l,range(1,n+1)); keep=[i+1 for i in range(n) if s[i]>30]
    return np.array(nd.center_of_mass(a,l,keep))
C=[cents(f) for f in fs]
start=len(C)-81  # last 8 s
# chain each body from frame `start` forward by nearest centre each frame (0.1 s steps)
pos=C[start].copy(); traj=[pos.copy()]; ok=np.ones(len(pos),bool)
for t in range(start+1,len(C)):
    d=np.sqrt(((pos[:,None,:]-C[t][None,:,:])**2).sum(-1)); j=d.argmin(1); step=d.min(1)
    ok&=step<8   # drop a chain that jumps (split/merge)
    pos=C[t][j]; traj.append(pos.copy())
traj=np.array(traj)
print('bodies tracked over 8 s without a jump:',ok.sum(),'of',len(ok))
for lag in [5,10,20,40,80]:
    disp=np.sqrt(((traj[lag:,ok]-traj[:-lag,ok])**2).sum(-1))
    path=np.sqrt(((traj[1:,ok]-traj[:-1,ok])**2).sum(-1))
    cum=np.array([path[i:i+lag].sum(0) for i in range(len(path)-lag+1)])
    print(f'lag {lag/10:4.1f} s: median net displacement {np.median(disp):6.2f} fine cells; median path length {np.median(cum):6.2f}; straightness {np.median(disp)/np.median(cum):.2f}')
