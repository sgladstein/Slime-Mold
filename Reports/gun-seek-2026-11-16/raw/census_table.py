import re,sys,os
S=sys.argv[1]; vs=sys.argv[2].split(','); scenes=sys.argv[3].split(',')
def read(n):
    p=f'{S}/{n}/log.txt'
    if not os.path.exists(p): return None
    t=open(p).read()
    r=[l for l in t.splitlines() if 'rounds t' in l]
    g=[l for l in t.splitlines() if re.match(r'^  t +[0-9]+ s',l)]
    if not r or not g: return None
    r=r[-1]; g=g[-1]
    m=re.search(r'(\d+) seeker turns, (\d+) soldier hits in (\d+) cells, (\d+) ran out of range and (\d+) hit a wall',r)
    turns,hits,cells,rng,wall=map(int,m.groups())
    sh=int(re.search(r'shots (\d+)',g).group(1)); k=int(re.search(r'killed (\d+)',g).group(1)); so=int(re.search(r'soldiers (\d+)',g).group(1)); tt=int(re.search(r't +(\d+) s',g).group(1))
    return dict(turns=turns,miss=rng+wall,shots=sh,killed=k,sold=so,t=tt)
tot={v:[0,0,0] for v in vs}
print('scene'.ljust(8),*[v.rjust(22) for v in vs])
for s in scenes:
    row=[]
    for v in vs:
        d=read(f'{s}-{v}')
        if d is None: row.append('-'.rjust(22)); continue
        if d['shots']: tot[v][0]+=d['miss']; tot[v][1]+=d['shots']
        if s!='hold': tot[v][2]+=d['killed']
        row.append(f"{d['miss']}/{d['shots']} k{d['killed']} s{d['sold']} t{d['t']}".rjust(22))
    print(s.ljust(8),*row)
print('total'.ljust(8),*[f"{a}/{b}={100*a/max(b,1):.1f}% k{c}".rjust(22) for a,b,c in tot.values()])
