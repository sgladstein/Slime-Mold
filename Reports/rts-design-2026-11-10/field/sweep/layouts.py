"""The field's pile draws. Seed 7 gave scatter, 11 gave scatter2; the sweep
uses 21-28. Prints one line per seed: LABEL x,y x,y ... (the left half; the
harness mirrors each pile and adds the centre line and the homes)."""
import random, math, sys
def draw(sd):
    random.seed(sd)
    inside = lambda x, y, m=0.80: ((x-128)/125)**2 + ((y-80)/77)**2 <= m
    pts = []; c = 0
    while len(pts) < 9 and c < 100000:
        c += 1
        x = random.uniform(14, 112); y = random.uniform(10, 150)
        if not inside(x, y): continue
        if math.hypot(x-40, y-80) < 24: continue
        if any(math.hypot(x-a, y-b) < 28 for a, b in pts): continue
        if any(math.hypot(x-128, y-cc) < 22 for cc in (40, 80, 120)): continue
        pts.append((round(x), round(y)))
    return pts
for sd in map(int, sys.argv[1:]):
    p = draw(sd)
    print(f"draw{sd}", len(p), " ".join(f"{x},{y}" for x, y in p))
