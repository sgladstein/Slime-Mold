import csv, statistics as st, math, itertools
import numpy as np
rows=list(csv.DictReader(open('/home/user/review-fun/maps/measures.tsv'),delimiter='\t'))
bio={1:'meadow',2:'barrens',3:'fen',4:'valley',5:'jungle',6:'highlands',7:'caves',8:'core'}
for r in rows:
    r['depth']=int(r['name'].split('-')[1])
    ns=[x.split('/') for x in r['nests(walk/hardwalk/detour/widest)'].split()]
    r['widest_min']=min(float(n[3]) for n in ns)
    r['detour']=st.mean(float(n[2]) for n in ns)
    r['wallsper']=float(r['wall_bins_per_1x'])
    r['area']=float(r['w'])*float(r['h'])/(1024*640)
    r['crys_per_area']=float(r['crystals'])/r['area']
# Play geometry: how routes, chokes, sight, room and the crystals lie.
geo=['widest_min','cut_rel','detour','sight','roomy','home_share','top_share','crys_per_area','wallsper','deep']
# What the slime feels: ground kinds.
slime=['rock','wet','sand','clay','barren']
def run(feats,label):
    X=np.array([[math.log1p(float(r[k])) if k in('widest_min','wallsper') else float(r[k]) for k in feats] for r in rows])
    X=(X-X.mean(0))/X.std(0).clip(1e-9)
    D=np.array([r['depth'] for r in rows])
    cent={d:X[D==d].mean(0) for d in range(1,9)}
    within={}
    for d in range(1,9):
        Y=X[D==d]
        within[d]=np.mean([np.linalg.norm(a-b) for a,b in itertools.combinations(Y,2)])
    print(f'== {label}: within-biome mean pairwise distance (seed to seed)')
    print('  '+'  '.join(f'{bio[d]} {within[d]:.2f}' for d in range(1,9)))
    pairs=[]
    for a,b in itertools.combinations(range(1,9),2):
        # mean distance between planets of a and planets of b, over the mean within spread
        between=np.mean([np.linalg.norm(x-y) for x in X[D==a] for y in X[D==b]])
        pairs.append((between/((within[a]+within[b])/2),between,a,b))
    pairs.sort()
    print('  most alike biome pairs (between / within; 1.0 = no more apart than two seeds of one biome):')
    for q,bt,a,b in pairs[:6]:
        print(f'    {bio[a]}-{bio[b]}: {q:.2f} (between {bt:.2f})')
    print('  most apart:', ', '.join(f'{bio[a]}-{bio[b]} {q:.2f}' for q,bt,a,b in pairs[-3:]))
run(geo,'play geometry')
run(geo+slime,'geometry + ground the slime feels')
