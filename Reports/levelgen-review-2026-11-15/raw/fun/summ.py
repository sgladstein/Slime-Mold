import csv, statistics as st, math
rows=list(csv.DictReader(open('/home/user/review-fun/maps/measures.tsv'),delimiter='\t'))
bio={1:'meadow',2:'barrens',3:'fen',4:'valley',5:'jungle',6:'highlands',7:'caves',8:'core'}
for r in rows:
    r['depth']=int(r['name'].split('-')[1]); r['seed']=int(r['name'].split('-')[2])
    ns=[x.split('/') for x in r['nests(walk/hardwalk/detour/widest)'].split()]
    r['widest_min']=min(float(n[3]) for n in ns)
    r['widest_max']=max(float(n[3]) for n in ns)
    r['detour']=st.mean(float(n[2]) for n in ns)
    r['hardextra']=max(float(n[1])/float(n[0]) for n in ns)
keys=['walk_near','walk_far','widest_min','widest_max','cut','cut_rel','detour','sight','roomy','home_share','mid_share','far_share','top_share','first_crystal','second_crystal','crystals','rock','wet','deep','plants','walls']
print('depth biome '+' '.join(k[:10].rjust(10) for k in keys))
for d in range(1,9):
    rs=[r for r in rows if r['depth']==d]
    def f(k):
        v=sorted(float(r[k]) for r in rs)
        return f"{st.median(v):.2f}"
    print(d, bio[d].ljust(9), ' '.join(f(k).rjust(10) for k in keys))
print()
print('ranges (min-max over 8 seeds)')
for d in range(1,9):
    rs=[r for r in rows if r['depth']==d]
    print(d, bio[d].ljust(9), ' '.join((f"{min(float(r[k]) for r in rs):.2f}-{max(float(r[k]) for r in rs):.2f}").rjust(10) for k in ['widest_min','cut','cut_rel','sight','home_share','top_share','walk_far']))
