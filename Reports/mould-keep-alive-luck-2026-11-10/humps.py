import re,glob,sys
rows=[]
for f in sorted(glob.glob(sys.argv[1]+'/*_*.txt' if len(sys.argv)>1 else '*_*.txt')):
    hand,seed=f.split('/')[-1][:-4].split('_'); seed=int(seed)
    s=open(f).read()
    m=re.search(r'LIVED (\d+):(\d+)',s)
    if not m: continue
    lived=int(m[1])*60+int(m[2])
    piles={}
    for l in re.finditer(r'PILE n=(\d+) t=([\d.]+)s at=\S+ gap=(\S+) walkers=(\d+) asleep=(\d+) near_awake=(\S+) near_asleep=(\S+)',s):
        piles[int(l[1])]=dict(t=float(l[2]),gap=float(l[3]),w=int(l[4]),a=int(l[5]),na=float(l[6]),ns=float(l[7]))
    rows.append((hand,seed,lived,piles))
for hand,seed,lived,p in sorted(rows,key=lambda r:r[2]):
    cells=[]
    for n in range(9,17):
        if n in p: q=p[n]; cells.append(f"{n}:{q['w']}/{q['a']} {q['na']:.0f}|{q['ns']:.0f}")
    print(f"{hand:8} s{seed:<2} {lived//60}:{lived%60:02d} "+"  ".join(cells))

# The deciding measure (named after the first look, see the report): the
# first pile that lands while more than half the mould is asleep (20+
# walkers), and whether a sleeper is within 45 cells of it, the water hand's
# own reach.
print()
for hand,seed,lived,p in sorted(rows,key=lambda r:r[2]):
    first=[(n,q) for n,q in sorted(p.items()) if q['a']>0.5*q['w'] and q['w']>=20]
    tag="none" if not first else ("NEAR" if first[0][1]['ns']<45 else "far")
    print(f"{hand:8} s{seed:<2} {lived//60}:{lived%60:02d} {'long ' if lived>=330 else 'short'} first mostly-asleep pile: {tag} {first[0][0] if first else ''} sleeper at {first[0][1]['ns'] if first else ''}")
