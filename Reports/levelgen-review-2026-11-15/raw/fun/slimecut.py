"""The slime's cut: least separator between home and the nests' surroundings,
where deep water is open (the slime crawls through) and crystals are open
(it eats over them); rock and soft rock block. Compare with the soldiers' cut."""
import sys, os, math, csv
import numpy as np
sys.path.insert(0, '/home/user/review-fun/tools')
import playmeasure as pm
d = '/home/user/review-fun/maps'
out = open('/home/user/review-fun/maps/slimecut.tsv', 'w')
out.write('name\tcut_soldier\tcut_slime\n')
names = sys.argv[1:] or sorted(n[:-4] for n in os.listdir(d) if n.endswith('.png'))
for n in names:
    g, b, it = pm.load(d, n)
    soft, hard, rock, walls, cryst = pm.grids(g, it)
    bh, bw = soft.shape
    gg = g[:bh * 4, :bw * 4].reshape(bh, 4, bw, 4)
    slime = ~((gg & 8) != 0).any(axis=(1, 3)) & ~walls
    Gs, idxs = pm.graph(soft)
    dsoft = pm.walk_dist(soft, Gs, idxs, it['landing'])
    near = min(pm.rim_dist(dsoft, soft, q, 40) for q in it['mould'])
    home = (dsoft <= 0.3 * near) & hard
    away = np.zeros_like(home)
    for q in it['mould']:
        dq = pm.walk_dist(soft, Gs, idxs, q)
        away |= (dq <= 0.3 * near) & hard
    c1 = pm.min_cut(hard, home, away)
    c2 = pm.min_cut(slime, home & slime, away & slime)
    out.write(f'{n}\t{c1}\t{c2}\n'); out.flush()
