# Emit close-view commands (t=1 s, hand idle) for every planet/seed.
import json
p = json.load(open('/home/user/review-aesthetics/points2.json'))
plan = [('ident', 12), ('landing', 12), ('nest', 6), ('shore', 24), ('shore', 6), ('rock', 24), ('plant', 24), ('hedge', 12), ('deep', 12), ('bridge', 24)]
for d in range(1, 9):
    for s in (1, 2):
        pts = p[f'planet-{d}-{s}']
        for k, z in plan:
            c = pts.get(k)
            if not c: continue
            out = f'frames/close/{d}-{s}-{k}-z{z}'
            print(f'/home/user/sm-review-bin/larder-world look={out} map=planet:{d}:{s} seed={s} hand=idle at=1 screen=1280x800 centre={c[0]},{c[1]},{z} > {out}.log 2>&1')
