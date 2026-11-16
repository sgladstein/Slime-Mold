import sys, collections
for f in sys.argv[1:]:
    pm=[]; mf=collections.defaultdict(list)
    for l in open(f):
        p=l.split()
        if p[0]=='PM': pm.append((int(p[1]),float(p[2]),int(p[3]),int(p[4]),int(p[5]),int(p[6]),int(p[7])))
        elif p[0]=='MF': mf[int(p[1])].append((int(p[2]),float(p[3]),int(p[4])))
    n=len(pm)
    busy=sum(1 for x in pm if x[2]!=-1 or x[3]!=-1)
    ms=sorted(x[1] for x in pm)
    # maps started: building goal changes
    starts=0; prev=-1
    for x in pm:
        if x[3]!=-1 and x[3]!=x[2]: starts+=1
    finished=sum(1 for x in pm if x[2]!=-1 and x[3]!=x[2])
    calls=sum(len(v) for v in mf.values())
    miss=sum(1 for v in mf.values() for c in v if c[2]==-1)
    distinct_goals=[len(set(c[0] for c in v)) for v in mf.values()]
    distinct_keys=[len(set(c[2] for c in v if c[2]!=-1)) for v in mf.values()]
    asks=[x[4] for x in pm]; maps=[x[5] for x in pm]; stale=[x[6] for x in pm]
    allgoals=set(c[0] for v in mf.values() for c in v)
    print(f"{f.split('/')[-1]}: steps {n}; pump busy {busy} steps; pump ms mean {sum(ms)/n:.2f} p50 {ms[n//2]:.2f} p95 {ms[n*95//100]:.2f} max {ms[-1]:.2f}")
    print(f"  maps started {starts} (finished {finished}); map_for calls {calls} ({calls/n:.1f}/step), unserved {miss} ({100*miss/max(calls,1):.1f}%)")
    print(f"  distinct goal cells asked per step: mean {sum(distinct_goals)/max(len(distinct_goals),1):.1f} max {max(distinct_goals,default=0)}; distinct maps used per step mean {sum(distinct_keys)/max(len(distinct_keys),1):.1f} max {max(distinct_keys,default=0)}; goal cells over the run {len(allgoals)}")
    print(f"  asked queue mean {sum(asks)/n:.1f} max {max(asks)}; maps in cache end {maps[-1]} max {max(maps)}; stale queue max {max(stale)}")
