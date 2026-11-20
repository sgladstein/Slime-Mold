#!/usr/bin/env python3
"""Each mini-boss that met the squad (nearest soldier under 20): the army then, seconds it lived after, soldiers lost meanwhile (any cause).

usage: contact.py CENSUS_DIR... (folders of SEED-PLANET/log.txt)
"""
import re,glob,sys
for a in sys.argv[1:]:
    out=[]
    for f in sorted(glob.glob(a+'/*/log.txt')):
        rows={}
        for m in re.finditer(r'boss t (\d+) s: lump (\d) (\w+) (\S+(?: \S+)?), (\d+) members .*?nearest soldier ([\d.]+), (\d+) alive',open(f).read()):
            t,l,k,ph,n,near,alive=m.groups()
            rows.setdefault(l,[]).append((int(t),int(n),float(near),int(alive),k))
        for l,r in rows.items():
            segs=[[r[0]]]
            for x in r[1:]:
                (segs[-1].append(x) if x[0]-segs[-1][-1][0]<=2 else segs.append([x]))
            for s in segs:
                c=[x for x in s if x[2]<20 and x[1]>0]
                if c:
                    i=s.index(c[0]); d=sum(max(0,p[3]-q[3]) for p,q in zip(s[i:],s[i+1:]))
                    out.append((c[0][3],f"{f.split('/')[1]} {s[0][4]} t{c[0][0]} members {max(x[1] for x in s)} army {c[0][3]} lived {s[-1][0]-c[0][0]}s lost {d}",s[-1][0]-c[0][0],d))
    big=[o for o in out if o[0]>=30]
    print(a, 'contacts',len(out),'with army>=30:',len(big),'lived',[o[2] for o in big],'lost',[o[3] for o in big])
    print('\n'.join(o[1] for o in out))
