import re,math,statistics as st
from math import comb
T=['none','sporer','sleeper','swarmer']
d={}
pat=re.compile(r'^\s*(\w+) seed (\d+):\s+(.*?) at\s+(\d+) s, score ([+-][\d.]+); soldiers\s+(\d+), mould\s+(\d+)')
for t in T:
    for l in open(f'/tmp/claude-0/aiw/tiers/{t}.txt'):
        m=pat.match(l)
        if m: d[(t,m[1],int(m[2]))]=(float(m[5]),m[3],int(m[4]),int(m[6]),int(m[7]))
S=range(53,61);H=['idle','random','farm','boom','claim','strike'];AI=['farm','boom','claim','strike']
def sign(a,b):
    h=sum(x>y for x,y in zip(a,b));lo=sum(x<y for x,y in zip(a,b));n=h+lo
    if n==0: return h,lo,1.0
    k=min(h,lo);p=min(1,2*sum(comb(n,i) for i in range(k+1))/2**n);return h,lo,p
sc=lambda t,h:[d[(t,h,s)][0] for s in S]
tests=[]
print('medians / means, ends (wiped/cap/lost), median mould at end')
for t in T:
    for h in H:
        v=sc(t,h);e=[d[(t,h,s)][1] for s in S]
        print(f'{t:8}{h:7} med {st.median(v):+.3f} mean {st.mean(v):+.3f} ends {sum("mould" in x for x in e)}/{sum("cap" in x for x in e)}/{sum("squad" in x for x in e)} mould med {st.median([d[(t,h,s)][4] for s in S])}')
for t in T:
    for h in ['boom','claim','strike']:
        for c in ['idle','random']:
            tests.append((f'D1 {t} {h}>{c}',)+sign(sc(t,h),sc(t,c)))
for t in T[1:]:
    for h in H:
        tests.append((f'D2 {t} {h} vs plain',)+sign(sc(t,h),sc('none',h)))
pd=[a-b for a,b in zip(sc('none','strike'),sc('none','boom'))]
for t in T[1:]:
    td=[a-b for a,b in zip(sc(t,'strike'),sc(t,'boom'))]
    tests.append((f'D3 {t} (strike-boom) vs plain',)+sign(td,pd))
    print(t,'strike-boom',[f'{x:+.2f}' for x in td],'plain',[f'{x:+.2f}' for x in pd])
# also style vs farm per tier
for t in T:
    for h in ['boom','claim','strike']:
        tests.append((f'X {t} {h}>farm',)+sign(sc(t,h),sc(t,'farm')))
n=len(tests);o=sorted(range(n),key=lambda i:tests[i][3]);adj=[0]*n;r=0
for k,i in enumerate(o): r=max(r,min(1,(n-k)*tests[i][3]));adj[i]=r
print('Holm over',n)
for i,tt in enumerate(tests): print(f'{tt[0]:34} {tt[1]}-{tt[2]} p {tt[3]:.4f} holm {adj[i]:.3f}')
# best per seed
for t in T:
    b=[max(['boom','claim','strike'],key=lambda h:d[(t,h,s)][0]) for s in S]
    print(t,'best',{h:b.count(h) for h in set(b)})
