import re,sys,os
S=sys.argv[1]
def window(d):
    t=open(os.path.join(S,d,'log.txt')).read()
    g=re.findall(r"t +[\d.]+ s: mould (\d+), soldiers (\d+), bank [\d.-]+, harvested [\d.]+, rendered \d+, shots (\d+), killed (\d+)",t)
    m=re.findall(r"(\d+) ran out of range and (\d+) hit a wall",t)
    tu=re.findall(r"(\d+) seeker turns",t)
    if len(g)<2 or len(m)<2: return None
    return dict(shots=int(g[-1][2])-int(g[0][2]),killed=int(g[-1][3])-int(g[0][3]),sold=int(g[-1][1]),
                miss=int(m[-1][0])+int(m[-1][1])-int(m[0][0])-int(m[0][1]),wall=int(m[-1][1])-int(m[0][1]),turns=int(tu[-1])-int(tu[0]))
names=sorted({d.rsplit('-',1)[0] for d in os.listdir(S) if os.path.isdir(os.path.join(S,d))}, key=lambda n:(n.split('-')[0],int(n.split('-')[1])))
T={'plain':[0,0,0],'seek':[0,0,0]}; Tt={'plain':[0,0,0],'seek':[0,0,0]}
fewer=more=same=0; kup=kdn=keq=0
print(f"{'window':12} {'plain miss/shots kill sold':>28} {'seek miss/shots kill sold turns(wall)':>40}")
for n in names:
    a=window(n+'-plain'); b=window(n+'-seek')
    if not a or not b: print(n,'missing'); continue
    print(f"{n:12} {a['miss']:>6}/{a['shots']:<4} k{a['killed']:<6} s{a['sold']:<3}       {b['miss']:>6}/{b['shots']:<4} k{b['killed']:<6} s{b['sold']:<3} {b['turns']} ({b['wall']} wall; plain {a['wall']})")
    thin=not n.startswith('hold')
    for k,v in (('plain',a),('seek',b)):
        acc=Tt if thin else T
        acc[k][0]+=v['miss']; acc[k][1]+=v['shots']; acc[k][2]+=v['killed']
    if thin and a['shots']+b['shots']>0:
        ra=a['miss']/max(a['shots'],1); rb=b['miss']/max(b['shots'],1)
        if rb<ra: fewer+=1
        elif rb>ra: more+=1
        else: same+=1
        if b['killed']>a['killed']: kup+=1
        elif b['killed']<a['killed']: kdn+=1
        else: keq+=1
for lab,acc in (('thin',Tt),('thick',T)):
    print(lab, {k:f"{v[0]}/{v[1]}={100*v[0]/max(v[1],1):.1f}% killed {v[2]}" for k,v in acc.items()})
from math import comb
def sign(a,b):
    n=a+b; k=min(a,b); return min(1,2*sum(comb(n,i) for i in range(k+1))/2**n) if n else 1
print(f"thin windows: miss rate lower with seek {fewer}, higher {more}, same {same}; sign p={sign(fewer,more):.3f}")
print(f"thin windows: kills higher with seek {kup}, lower {kdn}, same {keq}; sign p={sign(kup,kdn):.3f}")
