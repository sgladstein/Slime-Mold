"""How each planet opens, from the maps `examples/planets out=DIR` writes: the crystals within 260 map cells
of the landing (nests' home crystals left out), their count, the nearest one's distance (map cells, straight),
radius and charge, and their energy (radius^2 x charge, the game's held measure, in thousands).
python3 opening.py DIR  ->  a line per biome: medians and ranges over the seeds."""
import sys, os, math, statistics as st
d = sys.argv[1]
names = ['meadow', 'barrens', 'fen', 'river valley', 'jungle', 'highlands', 'caves', 'core']
rows = {}
for f in sorted(os.listdir(d)):
    if not f.endswith('.txt'):
        continue
    depth = int(f.split('-')[1])
    it = [l.split() for l in open(os.path.join(d, f)) if l.strip()]
    land = next(tuple(map(float, r[1:3])) for r in it if r[0] == 'landing')
    nests = [tuple(map(float, r[1:3])) for r in it if r[0] == 'mould']
    cr = [tuple(map(float, r[1:5])) for r in it if r[0] == 'crystal']
    cr = [c for c in cr if all(math.hypot(c[0] - q[0], c[1] - q[1]) > 2 for q in nests)]
    near = sorted((math.hypot(c[0] - land[0], c[1] - land[1]), c) for c in cr)
    near = [(dd, c) for dd, c in near if dd <= 260]
    first = near[0] if near else (None, (0, 0, 0, 0))
    rows.setdefault(depth, []).append(dict(n=len(near), d=first[0] or 0, r=first[1][2], ch=first[1][3],
                                           e=sum(c[2] ** 2 * c[3] for _, c in near) / 1000))
print('planet | crystals within 260 | nearest: distance (map cells), radius, charge | energy within 260 (k)')
for depth in sorted(rows):
    r = rows[depth]
    rng = lambda k: f"{min(x[k] for x in r):.0f}-{max(x[k] for x in r):.0f}"
    med = lambda k: st.median(x[k] for x in r)
    print(f"{depth} {names[depth - 1]} | {med('n'):.0f} ({rng('n')}) | {med('d'):.0f} ({rng('d')}), r {med('r'):.0f} ({rng('r')}), charge {rng('ch')} | {med('e'):.0f} ({rng('e')})")
# One seed's planets, each crystal within 260 of the landing: python3 opening.py DIR SEED
if len(sys.argv) > 2:
    s = sys.argv[2]
    print(f'seed {s}: crystals within 260 map cells of the landing (distance, radius, charge)')
    for depth in range(1, 9):
        it = [l.split() for l in open(os.path.join(d, f'planet-{depth}-{s}.txt')) if l.strip()]
        land = next(tuple(map(float, r[1:3])) for r in it if r[0] == 'landing')
        nests = [tuple(map(float, r[1:3])) for r in it if r[0] == 'mould']
        cr = [tuple(map(float, r[1:5])) for r in it if r[0] == 'crystal']
        cr = [c for c in cr if all(math.hypot(c[0] - q[0], c[1] - q[1]) > 2 for q in nests)]
        near = sorted((math.hypot(c[0] - land[0], c[1] - land[1]), c) for c in cr)
        print(depth, names[depth - 1], '|', ', '.join(f'{dd:.0f}/{c[2]:.0f}/{c[3]:.0f}' for dd, c in near if dd <= 260))
