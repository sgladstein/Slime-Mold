import re,sys,math,glob
def rows(fs):
    d={}
    for f in fs:
        for l in open(f):
            m=re.match(r"\s*(\w+) seed\s+(\d+):.*?score ([+-][\d.]+);.*?mould\s+(\d+)",l)
            if m: d[(m[1],int(m[2]))]=(float(m[3]),int(m[4]))
    return d
def sign(w,l):
    n=w+l
    if n==0: return 1.0
    k=min(w,l); p=sum(math.comb(n,i) for i in range(k+1))/2**n*2
    return min(p,1.0)
base=rows(sys.argv[1].split(',')); body=rows(sys.argv[2].split(','))
hands=["idle","random","farm","rush","wall"]
print("hand  n  base_mean body_mean  body>base body<base  mould base->body")
for h in hands:
    ks=[k for k in base if k[0]==h and k in body]
    w=sum(body[k][0]>base[k][0] for k in ks); l=sum(body[k][0]<base[k][0] for k in ks)
    bm=sum(base[k][0] for k in ks)/len(ks); hm=sum(body[k][0] for k in ks)/len(ks)
    mb=sum(base[k][1] for k in ks)/len(ks); mh=sum(body[k][1] for k in ks)/len(ks)
    print(f"{h:6} {len(ks):2} {bm:+.3f} {hm:+.3f}  {w:2} {l:2} p {sign(w,l):.4f}  {mb:.0f} -> {mh:.0f}")
# gate within body arm
print("gate in body arm (plan vs idle/random by seed):")
for arm,name in ((base,'base'),(body,'body')):
    out=[]
    for p in ["farm","rush","wall"]:
        for c in ["idle","random"]:
            seeds=[s for (h,s) in arm if h==p and (c,s) in arm]
            w=sum(arm[(p,s)][0]>arm[(c,s)][0] for s in seeds); l=sum(arm[(p,s)][0]<arm[(c,s)][0] for s in seeds)
            out.append(f"{p}>{c} {w}-{l} p{sign(w,l):.3f}")
    print(name, "; ".join(out))
