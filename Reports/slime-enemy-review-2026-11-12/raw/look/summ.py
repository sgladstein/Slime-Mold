import json, sys, numpy as np
rows=json.load(open(sys.argv[1])); ref=sys.argv[2]
keys=['dens','lump_share','core_occ','clump','sc_trail','sc_game','sc_blur','sc_dens','sc_trail_sh','sc_game_sh','sc_blur_sh','gap_trail','gap_game','gap_blur']
tags=[]
for r in rows:
    if r['tag'] not in tags: tags.append(r['tag'])
print('arm        '+' '.join(f'{k[:10]:>10}' for k in keys))
base={ (r['seed'],r['sec']):r for r in rows if r['tag']==ref}
for t in tags:
    rs=[r for r in rows if r['tag']==t]
    print(f'{t:<10} '+' '.join(f'{np.mean([r[k] for r in rs]):>10.3f}' for k in keys))
    if t!=ref:
        cnt=[]
        for k in keys[3:]:
            lo=sum(1 for r in rs if (r['seed'],r['sec']) in base and r[k]<base[(r['seed'],r['sec'])][k])
            cnt.append(f'{lo}/{len(rs)}')
        print(f'{"  lower":<10} '+' '.join(f'{"":>10}' for _ in keys[:3])+' '+' '.join(f'{c:>10}' for c in cnt))
