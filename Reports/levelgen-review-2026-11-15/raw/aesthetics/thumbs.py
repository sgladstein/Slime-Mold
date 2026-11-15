# Grey thumbnail test (266x164 of the 1 s whole-map frame, world only) and a
# nearest-neighbour "name the biome" score between seeds 1 and 2.
import sys
sys.path.insert(0, '/home/user/review-aesthetics/scripts')
import numpy as np
from PIL import Image, ImageDraw
from worldrect import rect
R = '/home/user/review-aesthetics/'
BIOME = ['meadow', 'barrens', 'fen', 'river valley', 'jungle', 'highlands', 'caves', 'core']
def world_crop(f, d, s):
    a = np.array(Image.open(f).convert('RGB'))
    m = Image.open(R + f'maps/planet-{d}-{s}.png'); Wm, Hm = m.size
    x0, y0, z = rect(a, Wm / 2, Hm / 2)
    x1 = int(round(x0 + z * Wm / 2)); y1 = min(705, int(round(y0 + z * Hm / 2)))
    return Image.fromarray(a).crop((x0 + 2, y0 + 2, x1 - 2, y1))
def sheet(t, grey, label, out):
    cols = 4; tw, th = 266, 164
    S = Image.new('RGB', (cols * (tw + 6), 2 * 2 * (th + 16)), (16, 16, 16)); D = ImageDraw.Draw(S)
    k = 0
    for s in (1, 2):
        for d in range(1, 9):
            im = world_crop(R + f'frames/main/planet-{d}-{s}-idle-{t}-all.png', d, s).resize((tw, th), Image.LANCZOS)
            if grey: im = im.convert('L').convert('RGB')
            x = ((d - 1) % 4) * (tw + 6); y = ((s - 1) * 2 + (d - 1) // 4) * (th + 16)
            S.paste(im, (x, y + 16))
            D.text((x + 2, y + 2), f'P{d} {BIOME[d-1]} s{s}' if label else f'#{k}', fill=(230, 230, 230)); k += 1
    S.save(out)
def feats(im):
    g = np.array(im.convert('L')).astype(float) / 255
    hist = np.histogram(g, bins=16, range=(0, 1))[0] / g.size
    gx = np.abs(np.diff(g, axis=1)).mean(); gy = np.abs(np.diff(g, axis=0)).mean()
    small = np.array(im.convert('L').resize((33, 20), Image.BOX)).astype(float) / 255
    sgx = np.abs(np.diff(small, axis=1)).mean()
    return np.concatenate([np.cumsum(hist), [g.mean() * 2, g.std() * 4, gx * 20, gy * 20, sgx * 10]])
def nn_score(make):
    F = {(d, s): feats(make(d, s)) for d in range(1, 9) for s in (1, 2)}
    hits, pairs = 0, []
    for d in range(1, 9):
        dist = {e: np.abs(F[(d, 1)] - F[(e, 2)]).sum() for e in range(1, 9)}
        best = min(dist, key=dist.get); hits += best == d
        pairs.append((d, best))
    # biome-to-biome mean distances, both seeds
    M = np.zeros((8, 8))
    for a in range(1, 9):
        for b in range(1, 9):
            M[a-1, b-1] = np.mean([np.abs(F[(a, s)] - F[(b, t)]).sum() for s in (1, 2) for t in (1, 2)])
    return hits, pairs, M
if __name__ == '__main__':
    for t in ('001s', '060s'):
        sheet(t, False, True, R + f'thumbs-{t}-colour.png'); sheet(t, True, True, R + f'thumbs-{t}-grey.png')
    sheet('001s', True, False, R + 'thumbs-001s-grey-blind.png')
    th = lambda d, s: world_crop(R + f'frames/main/planet-{d}-{s}-idle-001s-all.png', d, s).resize((266, 164), Image.LANCZOS)
    hits, pairs, M = nn_score(th)
    print('zoom-1 grey thumbnails: seed-1 frame nearest seed-2 frame is the same biome', hits, '/ 8', pairs)
    np.set_printoptions(precision=2, suppress=True); print(M)
    iu = [(M[a, b], BIOME[a], BIOME[b]) for a in range(8) for b in range(a + 1, 8)]
    print('closest pairs zoom 1:', sorted(iu)[:4])
