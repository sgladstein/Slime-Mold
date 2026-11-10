import glob,re,sys,collections
from math import comb
D=sys.argv[1]
d=collections.defaultdict(dict); st=collections.defaultdict(dict)
for f in glob.glob(D+'/a16-*.txt'):
    for l in open(f):
        m=re.match(r'seed (\d+)\s+(\w+): (\d) stars\s+(.*?)\s+\|',l)
        if not m: continue
        a=m[4]; mine=wild=None
        mm=re.match(r'YOU (\d+), WILD (\d+)',a)
        if mm: mine,wild=int(mm[1]),int(mm[2])
        mm=re.match(r'THE WILD ONE DIED AT .*; YOU (\d+)',a)
        if mm: mine,wild=int(mm[1]),0
        mm=re.match(r'NONE OF YOURS LEFT AT .*; WILD (\d+)',a)
        if mm: mine,wild=0,int(mm[1])
        d[m[2]][int(m[1])]=(mine,wild); st[m[2]][int(m[1])]=int(m[3])
for h in sorted(d):
    v=d[h]; ks=sorted(v)
    wins=sum(v[k][0]>v[k][1] for k in ks)
    print(f"{h:8} n={len(ks)} wins {wins} stars {[st[h][k] for k in ks]} share {[round(v[k][0]/max(1,v[k][0]+v[k][1]),2) for k in ks]}")
def sign(a,b):
    sh=lambda x:x[0]/max(1,x[0]+x[1])
    w=sum(sh(d[a][k])>sh(d[b][k]) for k in d[a] if k in d[b]); l=sum(sh(d[a][k])<sh(d[b][k]) for k in d[a] if k in d[b]); n=w+l
    return f"{w}-{l} p={2*sum(comb(n,i) for i in range(min(w,l)+1))/2**n:.4f}" if n else "n/a"
hs=sorted(d)
for a in hs:
    for b in hs:
        if a<b: print(a,'vs',b,'share',sign(a,b))
