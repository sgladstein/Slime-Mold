"""Before (the map fixes) -> after (the doors), from two `planets` surveys:
first-try passes, median build ms, median rock share, median walk to the
far nest, median open charge; per planet."""
import sys, statistics as st
def load(p):
    rows = {}
    for l in open(p):
        f = l.split()
        if len(f) < 2 or not (f[0].isdigit() and f[1].isdigit()):
            continue
        d, s = int(f[0]), int(f[1])
        tail = l.split(']')[1].split()
        att = -1 if tail[0] == 'fallback' else int(tail[0])
        parts = l.split('|')
        g = parts[1].split()
        walk = parts[2].split()
        rows[(d, s)] = dict(att=att, ms=int(tail[1]), rock=float(g[1].rstrip('%')), far=int(walk[1]), charge=int(parts[3].strip().rstrip('%')), walls=int(g[-1]))
    return rows
b, a = load(sys.argv[1]), load(sys.argv[2])
print("planet | first-try passes | build ms median | rock median | far-nest walk median | open charge median | soft-rock pieces median (before -> after)")
tot = [0, 0, 0]
for d in range(1, 9):
    ks = [k for k in b if k[0] == d and k in a]
    m = lambda r, f: st.median([r[k][f] for k in ks])
    ft = lambda r: sum(r[k]['att'] == 0 for k in ks)
    tot[0] += ft(b); tot[1] += ft(a); tot[2] += len(ks)
    print(f"{d} | {ft(b)}/{len(ks)} -> {ft(a)}/{len(ks)} | {m(b,'ms'):.0f} -> {m(a,'ms'):.0f} | {m(b,'rock'):.0f}% -> {m(a,'rock'):.0f}% | {m(b,'far'):.0f} -> {m(a,'far'):.0f} | {m(b,'charge'):.0f}% -> {m(a,'charge'):.0f}% | {m(b,'walls'):.0f} -> {m(a,'walls'):.0f}")
print(f"all | {tot[0]}/{tot[2]} -> {tot[1]}/{tot[2]}")
print("fallbacks after:", sum(r['att'] == -1 for r in a.values()))
