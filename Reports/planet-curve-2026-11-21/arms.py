import json,sys,math,collections
rows=[json.loads(l) for l in open(sys.argv[1]) if l.strip()]
def arm(r): return r.get('extra') or ''
def plan(r): return 'lure' if 'lure' in r['plan'] or 'switch' in r['plan'] else 'rush'
base={(plan(r),int(r['depth']),int(r['seed'])):r for r in rows if arm(r)==''}
def sign(a,b):
    n=a+b
    if n==0: return 1.0
    k=min(a,b); return min(1.0,2*sum(math.comb(n,i) for i in range(k+1))/2**n)
arms=sorted({arm(r) for r in rows})
print('arms',arms,'rows',len(rows))
for a in arms:
  for p in ('rush','lure'):
    for d in (4,5):
      g=[r for r in rows if arm(r)==a and plan(r)==p and int(r['depth'])==d]
      w=sum(int(r['won']) for r in g)
      if a=='': print(f"{'as-is':16} {p} P{d}: {w}/{len(g)}  med_secs_lost={sorted(r['secs'] for r in g if not r['won'])[len([1 for r in g if not r['won']])//2] if any(not r['won'] for r in g) else '-'}"); continue
      up=sum(1 for r in g if r['won'] and not base[(p,d,int(r['seed']))]['won'])
      dn=sum(1 for r in g if not r['won'] and base[(p,d,int(r['seed']))]['won'])
      print(f"{a:16} {p} P{d}: {w}/{len(g)}  +{up} -{dn} p={sign(up,dn):.2f}")
