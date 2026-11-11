import numpy as np, glob, re, sys, os
S=os.environ.get('S','/tmp/scr')
from math import comb
W,H=1024,640
yy,xx=np.mgrid[0:H,0:W]
r=np.hypot(xx+0.5-W/2,yy+0.5-H/2)/4  # dish cells
ann=(r>=35)&(r<=55)
def m(f):
    v=np.fromfile(f,dtype='<f4')[:W*H].reshape(H,W)
    pos=v[v>0]
    t=np.percentile(pos,99)
    top=v>=t
    share=(top&ann).sum()/top.sum()
    return share
def loops(txt):
    for l in open(txt):
        if ' t  20.0s' in l and 'loops' in l: return int(l.split('loops')[1].split()[0])
def sign(a,b):
    d=[x-y for x,y in zip(a,b) if x!=y]; n=len(d); k=sum(1 for x in d if x>0); kk=min(k,n-k)
    p=min(1,2*sum(comb(n,i) for i in range(kk+1))/2**n); return k,n,p
res={}
for arm in ['disc','flat']:  # 'nomod' (no step_mod) ran 3 seeds only
    sh=[];lp=[]
    for s in range(1,9):
        f=f'{S}/look/big-P-{arm}-s{s}-s{s}-020s.f32'
        sh.append(m(f)); lp.append(loops(f'{S}/P-{arm}-s{s}.txt'))
    res[arm]=(sh,lp)
    print(arm,'ring share',[round(x,3) for x in sh],'median',round(float(np.median(sh)),3),'loops',lp,'median',np.median(lp))
for a,b in [('disc','flat')]:
    print(a,'vs',b,'ring share more',sign(res[a][0],res[b][0]),'loops more',sign(res[a][1],res[b][1]))
