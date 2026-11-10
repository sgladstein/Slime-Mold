import re,sys,glob,collections
D=collections.defaultdict(dict)
for f in glob.glob(sys.argv[1]+'/*.txt'):
    m=re.search(r'/([^/]+)-(\d+)\.txt$',f); name,seed=m.group(1),int(m.group(2))
    for l in open(f):
        m=re.match(r'seed\s+\d+\s+\S+: (\d) stars\s+(.*?)\s+mould.*share ([\d.]+)',l)
        if m: D[name][seed]=(int(m.group(1)),float(m.group(3)),'DIED' in m.group(2))
order=[l.split('|')[0] for l in open(sys.argv[2]) if l.strip()]
for n in order:
    v=D.get(n,{}); ss=sorted(v)
    print(f"{n:16s} n={len(v)} stars {''.join(str(v[s][0]) for s in ss)} share {' '.join(('%.2f'%v[s][1])+('*' if v[s][2] else '') for s in ss)}")
def sign(a,b):
    A,B=D[a],D[b]; w=sum(B[s][1]>A[s][1] for s in A if s in B); l=sum(B[s][1]<A[s][1] for s in A if s in B)
    from math import comb
    n=w+l; k=min(w,l); p=min(1,2*sum(comb(n,i) for i in range(k+1))/2**n) if n else 1
    return f"{b} vs {a}: {w}-{l} p={p:.4f}"
for a,b in [('idle','random'),('idle','warden'),('random','warden'),('random','w-j6l2'),('w-j6l2','w-nosnip-j6l2'),('warden','w-nosnip'),('w-stop30','w-j6l2')]:
    if a in D and b in D: print(sign(a,b))
