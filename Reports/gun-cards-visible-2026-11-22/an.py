import sys,re,collections
def load(f):
    d=collections.defaultdict(dict)
    for l in open(f):
        kv=dict(x.split('=',1) for x in l.split()[1:] if '=' in x)
        if 'killed' not in kv: continue
        d[kv['deck']][kv['seed']]=kv
    return d
base=load('raw-main.txt')
for f in sys.argv[1:]:
    d=load(f)
    for deck,ss in d.items():
        diffs=[int(ss[s]['killed'])-int(base['none'][s]['killed']) for s in sorted(ss)]
        old=base.get(deck,{})
        od=[int(old[s]['killed'])-int(base['none'][s]['killed']) for s in sorted(ss) if s in old]
        extra=' '.join(f"{k}={sum(float(ss[s][k]) for s in ss)/len(ss):.0f}" for k in ['round_kills','fire_kills','hp_lost','alive'])
        print(f"{deck:24s} n={len(ss)} dkill={diffs} sum={sum(diffs)} old={sum(od)} {extra}")
