import sys,statistics as st
def load(p):
    rows={}
    for l in open(p):
        f=l.split()
        if len(f)<2 or not (f[0].isdigit() and f[1].isdigit()): continue
        d,s=int(f[0]),int(f[1])
        att=int(l.split(']')[1].split()[0]); ms=int(l.split(']')[1].split()[1])
        parts=l.split('|')
        walk=parts[2].split(); oc=parts[3].strip().rstrip('%')
        rows[(d,s)]=dict(att=att,ms=ms,nest=int(walk[1]),far=int(walk[2]) if len(walk)>2 else 0,charge=int(oc),walls=int(parts[1].split()[-1]))
    return rows
b,a=load(sys.argv[1]),load(sys.argv[2])
print("depth | first-try passes | open charge median | far-nest walk median | build ms median (before -> after)")
for d in range(1,9):
    ks=[k for k in b if k[0]==d]
    def m(r,f): return st.median([r[k][f] for k in ks])
    ft=lambda r: sum(r[k]['att']==0 for k in ks)
    print(f"{d} | {ft(b)}/{len(ks)} -> {ft(a)}/{len(ks)} | {m(b,'charge'):.0f}% -> {m(a,'charge'):.0f}% | {m(b,'nest'):.0f} -> {m(a,'nest'):.0f} | {m(b,'ms'):.0f} -> {m(a,'ms'):.0f}")
