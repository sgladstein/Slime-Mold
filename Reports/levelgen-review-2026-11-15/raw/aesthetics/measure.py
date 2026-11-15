# Grey-luminance measures per planet from the whole-map frames at 1 s and 60 s,
# registered to the map file: ground classes, slime film/thick, crystals, cyan.
import sys, json
sys.path.insert(0, '/home/user/review-aesthetics/scripts')
import numpy as np
from PIL import Image, ImageDraw
from worldrect import rect
R = '/home/user/review-aesthetics/'
def luma(a): return 0.299 * a[..., 0] + 0.587 * a[..., 1] + 0.114 * a[..., 2]
def pct(v, q): return float(np.percentile(v, q)) if len(v) > 50 else None
res = {}
for d in range(1, 9):
    for s in (1, 2):
        name = f'planet-{d}-{s}'
        try:
            f1 = np.array(Image.open(R + f'frames/main/{name}-idle-001s-all.png').convert('RGB')).astype(float)
            f60 = np.array(Image.open(R + f'frames/main/{name}-idle-060s-all.png').convert('RGB')).astype(float)
        except FileNotFoundError:
            continue
        m = np.array(Image.open(R + f'maps/{name}.png').convert('RGBA')).astype(int)
        G, B = m[..., 1], m[..., 2]
        Hm, Wm = G.shape
        items = [l.split() for l in open(R + f'maps/{name}.txt') if l.strip()]
        x0, y0, z = rect(f1, Wm / 2, Hm / 2)
        ys, xs = np.mgrid[0:800, 0:1280]
        mx = ((xs - x0) / z * 2).astype(int); my = ((ys - y0) / z * 2).astype(int)
        inside = (mx >= 2) & (mx < Wm - 2) & (my >= 2) & (my < Hm - 2) & (ys > 60) & (ys < 700)
        mx = np.clip(mx, 0, Wm - 1); my = np.clip(my, 0, Hm - 1)
        g = G[my, mx]; b = B[my, mx]
        dens = (b & 63) / 63.0; pk = b >> 6
        rock = (g & 8) > 0; deep = (g >> 4) == 4; wet = ((g & 1) > 0) & ~deep; bridge = (g >> 4) == 5
        soil = g >> 4
        # walls and crystals rasterised on the map grid, then sampled
        wm = Image.new('L', (Wm, Hm), 0); dw = ImageDraw.Draw(wm)
        cm = Image.new('L', (Wm, Hm), 0); dc = ImageDraw.Draw(cm)
        cr = Image.new('L', (Wm, Hm), 0); dr = ImageDraw.Draw(cr)
        for p in items:
            if p[0] == 'wall':
                a0, b0, a1, b1, wd = map(float, p[1:6]); dw.line([a0, b0, a1, b1], fill=255, width=max(1, int(wd)))
            if p[0] == 'crystal':
                x, y, r = map(float, p[1:4])
                dc.ellipse([x - 1.5 * r, y - 1.5 * r, x + 1.5 * r, y + 1.5 * r], fill=255)
                dr.ellipse([x - 0.8 * r, y - 0.8 * r, x + 0.8 * r, y + 0.8 * r], fill=255)
        wall = np.array(wm)[my, mx] > 0; cry = np.array(cm)[my, mx] > 0; core = np.array(cr)[my, mx] > 0
        # boundary pixels: class changes within 6 map cells
        cls = np.where(rock, 1, np.where(deep, 2, np.where(wet, 3, 10 + soil)))
        clsm = cls[::1, ::1]
        def edge(mask):
            from numpy.lib.stride_tricks import sliding_window_view as sw
            pad = np.pad(mask, 4, mode='edge')
            v = sw(pad, (9, 9))
            return v.max((2, 3)) != v.min((2, 3))
        bd = edge(cls) | edge((dens > 0.3).astype(int)) | wall & False
        L1, L60 = luma(f1), luma(f60)
        warm1 = f1[..., 0] - f1[..., 2]; warm60 = f60[..., 0] - f60[..., 2]
        slime1 = warm1 > 45
        newslime = (warm60 - warm1 > 25) & ~slime1 & inside & ~cry & ~wall
        ok = inside & ~bd & ~cry & ~slime1
        classes = {
            'plain bare': ok & ~rock & ~wet & ~deep & (soil == 0) & (dens < 0.1) & ~wall,
            'sand bare': ok & ~rock & ~wet & (soil == 1) & (dens < 0.1) & ~wall,
            'clay bare': ok & ~rock & ~wet & (soil == 2) & (dens < 0.1) & ~wall,
            'barren': ok & ~rock & ~wet & (soil == 3) & ~wall,
            'grass': ok & ~rock & ~wet & (dens > 0.5) & (pk == 0) & ~wall,
            'jungle': ok & ~rock & ~wet & (dens > 0.5) & (pk == 1) & ~wall,
            'reeds': ok & ~rock & (dens > 0.5) & (pk == 2) & ~wall,
            'scrub': ok & ~rock & ~wet & (dens > 0.5) & (pk == 3) & ~wall,
            'shallow water': ok & wet & (dens < 0.1),
            'deep water': ok & deep,
            'rock': ok & rock & ~wall,
            'soft rock': inside & wall & ~cry & ~slime1 & ~rock,
        }
        out = {'zoom': z, 'classes': {}}
        for k, mk in classes.items():
            v = L1[mk]
            if len(v) > 80:
                out['classes'][k] = {'n': int(len(v)), 'p10': pct(v, 10), 'p50': pct(v, 50), 'p90': pct(v, 90)}
        sv = L60[newslime]
        under = L1[newslime]
        if len(sv) > 200:
            thin_cut = np.percentile(sv, 25)
            thin = newslime & (L60 <= thin_cut)
            out['slime'] = {'n': int(len(sv)), 'film_p10': pct(sv, 10), 'film_p25': pct(sv, 25), 'p50': pct(sv, 50), 'thick_p90': pct(sv, 90), 'thick_p99': pct(sv, 99),
                            'share_not_above_ground_by_10': float(np.mean(sv <= under + 10)),
                            'film_minus_ground_p50': pct((L60 - L1)[thin], 50)}
            # slime over each ground class: film (p10) vs that class's bare p90
            per = {}
            for k, mk in classes.items():
                sel = newslime & (mk | (inside & ~bd & ~cry & {'soft rock': wall}.get(k, mk)))
                sel = newslime & mk
                if sel.sum() > 80:
                    per[k] = {'n': int(sel.sum()), 'slime_p10': pct(L60[sel], 10), 'slime_p25': pct(L60[sel], 25), 'under_p50': pct(L1[sel], 50), 'share_le_under+10': float(np.mean(L60[sel] <= L1[sel] + 10))}
            out['slime_over'] = per
        # crystals: cyan pixels in crystal cores, ring around them; cyan elsewhere
        cyan = (f1[..., 2] - f1[..., 0] > 50) & (f1[..., 1] - f1[..., 0] > 30) & (f1[..., 2] > 90)
        cc = inside & core & cyan
        if cc.sum() > 50:
            out['crystal'] = {'p10': pct(L1[cc], 10), 'p50': pct(L1[cc], 50), 'p90': pct(L1[cc], 90)}
        stray = inside & ~cry & cyan
        out['cyan_outside_crystals_px'] = int(stray.sum())
        out['cyan_outside_crystals_share'] = float(stray.sum() / max(1, inside.sum()))
        res[name] = out
json.dump(res, open(R + 'measures.json', 'w'), indent=1)
for k, v in res.items():
    c = v['classes']
    print(k, 'z=%.2f' % v['zoom'], ' '.join(f"{n}:{int(x['p50'])}" for n, x in c.items()))
    if 'slime' in v:
        sl = v['slime']; print('   slime film p10 %.0f p25 %.0f p50 %.0f thick p90 %.0f p99 %.0f; share not >ground+10: %.2f' % (sl['film_p10'], sl['film_p25'], sl['p50'], sl['thick_p90'], sl['thick_p99'], sl['share_not_above_ground_by_10']))
        print('   over:', ' '.join(f"{n}:{int(x['slime_p10'])}/{int(x['under_p50'])}({x['share_le_under+10']:.2f})" for n, x in v['slime_over'].items()))
    print('   crystal', v.get('crystal'), 'cyan outside', v['cyan_outside_crystals_px'])
