#!/usr/bin/env python3
"""Paired games (same SEED-PLANET folders in two census dirs): length and end, peak army, soldiers fallen by a mini-boss (bossfell lines) in all, in the last quarter, and against 30+ soldiers, late bosses, mortars, round-errand rows.

usage: bigarmy.py MAIN_DIR NEW_DIR
"""
import sys
import re,glob,os,sys
def game(f):
    t=open(f).read()
    end=re.search(r't (\d+) s: mould \d+, soldiers (\d+).*ended: ([^\n]+)',t)
    T=int(end.group(1)) if end else max([int(x) for x in re.findall(r'^STRAT t (\d+)',t,re.M)]+[0])+10
    how=end.group(3) if end else 'time'
    fell=[(float(a),int(n),int(al)) for a,n,al in re.findall(r'bossfell t ([\d.]+) s: (\d+) soldiers fell by a mini-boss \(\d+ in all\), (\d+) alive',t)]
    army=[(int(a),int(s)) for a,s in re.findall(r'^STRAT t (\d+) \S+ mould \d+ soldiers (\d+)',t,re.M)]
    peak=max([s for a,s in army]+[0])
    last=[x for x in fell if x[0]>=0.75*T]
    big=sum(n for tt,n,al in fell if al+n>=30)
    bigl=sum(n for tt,n,al in last if al+n>=30)
    mortars=len(re.findall(r'mortar formed',t)); rounds=len(re.findall(r'(hunter|pinwheel) Go\(',t))
    late=len(re.findall(r'formed.*; late',t))
    return dict(T=T,how=how,fell=sum(x[1] for x in fell),last=sum(x[1] for x in last),big=big,bigl=bigl,peak=peak,mortars=mortars,round_rows=rounds,late=late)
for m in sorted(os.listdir(sys.argv[1])):
    a=game(f'{sys.argv[1]}/{m}/log.txt'); b=game(f'{sys.argv[2]}/{m}/log.txt')
    print(f"{m}: main {a['T']}s {a['how']}, peak army {a['peak']}, boss kills {a['fell']} (last quarter {a['last']}, vs 30+ {a['big']}/{a['bigl']}), late bosses {a['late']}, mortars {a['mortars']} | A+B {b['T']}s {b['how']}, peak {b['peak']}, boss kills {b['fell']} (last q {b['last']}, vs 30+ {b['big']}/{b['bigl']}), late {b['late']}, mortars {b['mortars']}, round rows {b['round_rows']}")
