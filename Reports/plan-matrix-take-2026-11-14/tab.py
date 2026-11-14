"""Tables for the take-specialty matrix from raw/ (python3 -I tab.py raw)."""
import re,sys,statistics as st
from math import comb
R=re.compile(r'^\s*(\S+) seed\s+(\d+):\s+(.*?) at\s+(\d+) s, score ([+-][\d.]+); soldiers\s+(\d+), mould\s+(\d+)')
H=['idle','random','strike','take','take_fort','take_boom','take_claim','take_strike']
SP=H[4:]
def p(a,b):
    n=a+b;k=min(a,b)
    return min(1,2*sum(comb(n,i) for i in range(k+1))/2**n) if n else 1
d=sys.argv[1];tests=[];out=[];best_sp={}
for t in ['none','drifter','sprawler','trunk','sporer','sleeper','swarmer','grudge']:
    g={}
    for l in open(f'{d}/larderworld-{t}.txt'):
        m=R.match(l)
        if m: g[(m[1],int(m[2]))]=(m[3].strip(),float(m[5]),int(m[6]),int(m[7]))
    seeds=sorted({s for _,s in g})
    out.append(f'\n## {t}  ({len(seeds)} seeds)\n| plan | median | mean | mould wiped/cap/squad wiped/overrun | median mould left | vs take |')
    out.append('|---|---|---|---|---|---|')
    def pair(a,b):
        hi=sum(g[(a,s)][1]>g[(b,s)][1] for s in seeds);lo=sum(g[(a,s)][1]<g[(b,s)][1] for s in seeds)
        return hi,lo,p(hi,lo)
    med={}
    for h in H:
        r=[g[(h,s)] for s in seeds];sc=[x[1] for x in r];med[h]=st.median(sc);e=[x[0] for x in r]
        vs='' if h=='take' else '%d-%d'%pair(h,'take')[:2]
        out.append(f'| {h} | {med[h]:+.3f} | {st.mean(sc):+.3f} | {sum("mould" in x for x in e)}/{sum("cap" in x for x in e)}/{sum("squad" in x for x in e)}/{sum("overrun" in x for x in e)} | {int(st.median([x[3] for x in r]))} | {vs} |')
    for h in SP:
        a,b,pp=pair(h,'take');tests.append((pp,f'{t}: {h} vs take {a}-{b}'))
    b=max(SP,key=lambda h:med[h]);best_sp[t]=b
    best=max(H,key=lambda h:med[h])
    out.append(f'best plan by median: {best}; best specialty: {b}, vs take {pair(b,"take")[0]}-{pair(b,"take")[1]} (p {pair(b,"take")[2]:.3f})')
ts=sorted(tests);m=len(ts);surv=[]
for i,(pp,s) in enumerate(ts):
    if pp*(m-i)<=0.05: surv.append(f'{s} p {pp:.4f}')
    else: break
out.append(f'\nHolm over {m} specialty-vs-take tests: {len(surv)} survive')
out+=['  '+s for s in surv]
out.append('smallest p: '+', '.join(f'{s} (p {pp:.4f})' for pp,s in ts[:4]))
print('\n'.join(out))
