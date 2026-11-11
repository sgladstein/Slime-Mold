import re,sys
from math import comb
def sign(a,b):
    d=[x-y for x,y in zip(a,b) if x!=y]; n=len(d); k=sum(1 for x in d if x>0); kk=min(k,n-k)
    return k,n,min(1,2*sum(comb(n,i) for i in range(kk+1))/2**n)
def load(f):
    v={}
    for l in open(f):
        m=re.match(r'seed (\d+) (\S+) t +([\d.]+)s +walkers ([\d.]+),',l)
        if m: v[(int(m[1]),m[2],float(m[3]))]=float(m[4])
    return v
for f in sys.argv[1:]:
    v=load(f); tags=sorted({k[1] for k in v}); T=max(k[2] for k in v)
    print(f, tags, 'T',T)
    rs=sorted({t.rsplit('-',1)[0] for t in tags})
    for r in rs:
        seeds=sorted({k[0] for k in v if k[1]==r+'-A'})
        a=[v[(s,r+'-A',T)]-0.55 for s in seeds if (s,r+'-A',T) in v and (s,r+'-B',T) in v]
        b=[v[(s,r+'-B',T)]-0.5 for s in seeds if (s,r+'-A',T) in v and (s,r+'-B',T) in v]
        print(r,'A gain',[round(x,3) for x in a],'\n   B gain',[round(x,3) for x in b],'\n   sign',sign(a,b),'A range',round(min(a),3),round(max(a),3),'B range',round(min(b),3),round(max(b),3))
        a10=[v[(s,r+'-A',10.0)] for s in seeds if (s,r+'-A',10.0) in v]; print('   A share at 10s',min(a10),max(a10), 'A at T', min(v[(s,r+'-A',T)] for s in seeds), max(v[(s,r+'-A',T)] for s in seeds))
