import re,sys,statistics as st
from math import comb
R=re.compile(r'^\s*(\S+) seed\s+(\d+):\s+(.*?) at\s+(\d+) s, score ([+-][\d.]+); soldiers\s+(\d+), mould\s+(\d+)')
H=['idle','random','boom','claim','fort','strike','cut','take','boom:spend=2+turrets=8']
short={'boom:spend=2+turrets=8':'boom+tur'}
def p(a,b):
    n=a+b;k=min(a,b)
    return min(1,2*sum(comb(n,i) for i in range(k+1))/2**n) if n else 1
d=sys.argv[1]
tests=[];out=[]
for t in ['none','drifter','sprawler','trunk','sporer','sleeper','swarmer','grudge']:
    g={}
    for l in open(f'{d}/larderworld-{t}.txt'):
        m=R.match(l)
        if m: g[(m[1],int(m[2]))]=(m[3].strip(),float(m[5]),int(m[6]),int(m[7]))
    seeds=sorted({s for _,s in g})
    out.append(f'\n## {t}  ({len(seeds)} seeds)\n| plan | median | mean | mould wiped/cap/squad wiped/overrun | median mould left |')
    out.append('|---|---|---|---|---|')
    med={}
    for h in H:
        r=[g[(h,s)] for s in seeds]
        sc=[x[1] for x in r];med[h]=st.median(sc)
        e=[x[0] for x in r]
        out.append(f'| {short.get(h,h)} | {med[h]:+.3f} | {st.mean(sc):+.3f} | {sum("mould" in x for x in e)}/{sum("cap" in x for x in e)}/{sum("squad" in x for x in e)}/{sum("overrun" in x for x in e)} | {int(st.median([x[3] for x in r]))} |')
    rank=sorted(H[2:],key=lambda h:-med[h])
    def pair(a,b):
        hi=sum(g[(a,s)][1]>g[(b,s)][1] for s in seeds);lo=sum(g[(a,s)][1]<g[(b,s)][1] for s in seeds)
        return hi,lo,p(hi,lo)
    b,r2=rank[0],rank[1]
    hi,lo,pp=pair(b,r2);tests.append((pp,t,f'{t}: best {b} vs runner-up {r2}: {hi}-{lo}'))
    for h in H[2:]:
        for c in ['idle','random']:
            a,bb,pp2=pair(h,c);tests.append((pp2,t,f'{t}: {h} vs {c}: {a}-{bb}'))
    out.append(f'best by median: {b}; runner-up {r2}: {hi}-{lo}, p {pp:.3f}')
    for h in H[2:]:
        a,bb,_=pair(h,'random'); out.append(f'  {short.get(h,h)} vs random {a}-{bb}; vs strike {pair(h,"strike")[0]}-{pair(h,"strike")[1]}' if h!='strike' else f'  strike vs random {a}-{bb}')
# Holm
ts=sorted(tests);m=len(ts);surv=[]
for i,(pp,t,s) in enumerate(ts):
    if pp*(m-i)<=0.05: surv.append(s+f' p {pp:.4f}')
    else: break
out.append(f'\nHolm over {m} tests: {len(surv)} survive')
out+=['  '+s for s in surv if 'runner' in s]
out.append('  (every survivor that is a plan vs idle/random: %d)'%sum('runner' not in s for s in surv))
print('\n'.join(out))
