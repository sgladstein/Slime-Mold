"""Fords and bridges by the nests, from a planets run with a debug line per river
(`DBG fords [d..] bridge Some(d)|None`: each crossing's distance to the nearest
nest, map cells, where the river is drawn). Only each planet's last build (the
one played); counts crossings within 320 and 220 of a nest."""
import sys, re
rows, block = [], []
for l in open(sys.argv[1]):
    if l.startswith('DBG'):
        block.append(l)
    elif l[:1].isdigit() and '[' in l:
        w = l.split()
        # a build prints its rivers twice (the planet's card and the level); keep the last build's lines
        k = len(block)
        last = block[k // 2:] if k % 2 == 0 and block[:k // 2] == block[k // 2:] else block
        ds = []
        for b in last:
            m = re.match(r'DBG fords \[(.*?)\] bridge (.*)', b)
            ds += [int(x) for x in m.group(1).split(',') if x.strip()]
            if m.group(2).startswith('Some'):
                ds.append(int(m.group(2)[5:-1]))
        if last:
            rows.append((int(w[0]), int(w[1]), w[8], ds))
        block = []
for depth in (4, 8):
    rr = [r for r in rows if r[0] == depth]
    allc = [d for r in rr for d in r[3]]
    print(f'planet {depth}: {len(rr)} planets with rivers, {len(allc)} fords and bridges; within 220 of a nest {sum(d < 220 for d in allc)}, within 320 {sum(d < 320 for d in allc)}; nearest {min(allc) if allc else "-"}; median {sorted(allc)[len(allc) // 2] if allc else "-"}')
