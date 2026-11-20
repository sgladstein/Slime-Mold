"""How many gun decks exist (2026-11-19): every card a run can hold, each
payload in its sets, decks of 1..slots cards in order, within budget.
Costs and tiers copied from rounds.rs (Card::cost, Card::tier)."""
from itertools import product
from math import comb
cost = dict(plain=1,bomb=4,germ=4,blast=3,gas=3,scorch=3,mist=3,multicast=3,
 wide=2,scatter=2,pierce=2,split=2,bait=2,range=2,rate=2,smother=2,trigger=2,expire=2,heave=2,seek=2,
 blind=2,sour=2,spin=2,provoke=2,still=2,shatter=2,detonate=2,flare=2,torch=2,tinder=2,thrift=0,
 quiet=1,delay=1,thick=1,pull=1)
tier = {c:1 for c in cost}
for c in "bomb germ multicast detonate".split(): tier[c]=3
for c in "scatter thick split scorch mist thrift smother trigger expire heave pull seek blind sour spin provoke still shatter flare torch tinder".split(): tier[c]=2
payload_sets = {"plain","blast","bomb","gas","germ"}  # take all four sets
payload_special = {"blind","sour","spin","provoke","still","shatter","detonate","flare","torch"}
cards=[]
for c in cost:
    if c in payload_sets: cards += [(f"{c}.{s}",cost[c],tier[c]) for s in ("standard","slug","needle","buckshot")]
    else: cards.append((c,cost[c],tier[c]))
print("distinct cards", len(cards))
def count(slots,budget,maxtier,ordered):
    cs=[c for c in cards if c[2]<=maxtier]
    n=0
    if ordered:
        for k in range(1,slots+1):
            for d in product(cs,repeat=k):
                if sum(x[1] for x in d)<=budget: n+=1  # thrift ignored (upper bound)
    else:
        from itertools import combinations_with_replacement as cwr
        for k in range(1,slots+1):
            for d in cwr(cs,k):
                if sum(x[1] for x in d)<=budget: n+=1
    return n
for name,slots,budget,mt in [("rifle p1 (commons)",4,10,1),("rifle p2-3",4,10,2),("rifle p4 (all)",4,10,3),("carbine p4",5,8,3),("long p4",3,12,3),("scatter p4",3,9,3)]:
    print(name, "unordered", count(slots,budget,mt,False), "ordered", count(slots,budget,mt,True) if slots<=4 else "skip")
