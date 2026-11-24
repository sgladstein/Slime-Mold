# m4.py RUN...: waves of waking (awake share over 50% after under 30%), from the dump's life.txt
import sys
for d in sys.argv[1:]:
    rows=[list(map(int,l.split())) for l in open(f"{d}/dump/life.txt")]
    share=[(r[0]+1, r[1]/max(r[1]+r[3],1)) for r in rows]
    waves=[]; low=True
    for t,s in share:
        if low and s>0.5: waves.append(t); low=False
        elif s<0.3: low=True
    asleep=sum(1 for _,s in share if s<0.3)
    print(d, "waves at", waves, "seconds mostly asleep (awake under 30%):", asleep, "of", len(share))
