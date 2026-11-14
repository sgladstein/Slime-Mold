# Usage: python3 plant_edges.py DIR planet-3-1 ... where DIR/old and DIR/new hold
# maps saved by `cargo run --release --example planets -- seeds=1 depths=3-5 out=DIR/new`
# (and the same build with the fades off into DIR/old); writes DIR/NAME-edges.png.
# Plant edges as the view draws them, roughly: the B byte at each fine cell
# (scale 2: map cell 2x+1), each kind's density interpolated bilinearly at
# zoom Z, a kind drawn where its density >= 0.2. Old vs new crops side by side.
import sys
import numpy as np
from PIL import Image
D = sys.argv[1]
Z = 12
COL = {0: (150, 160, 70), 1: (30, 80, 60), 2: (70, 120, 90), 3: (140, 120, 70)}
BARE = (90, 85, 60)
WATER = (40, 70, 90)
ROCK = (110, 110, 115)

def load(path):
    im = np.array(Image.open(path).convert('RGBA'))
    g = im[1::2, 1::2, 1].astype(int)
    b = im[1::2, 1::2, 2].astype(int)
    return g, b

def draw(g, b, x0, y0, w, h):
    gs = g[y0:y0 + h + 1, x0:x0 + w + 1]
    bs = b[y0:y0 + h + 1, x0:x0 + w + 1]
    kind = bs >> 6
    dens = (bs & 63) / 63.0
    H, W = h * Z, w * Z
    ys = (np.arange(H) + 0.5) / Z - 0.5
    xs = (np.arange(W) + 0.5) / Z - 0.5
    ys = np.clip(ys, 0, h - 1e-3); xs = np.clip(xs, 0, w - 1e-3)
    yi = ys.astype(int); xi = xs.astype(int); ty = (ys - yi)[:, None]; tx = (xs - xi)[None, :]
    def bil(a):
        a00 = a[yi][:, xi]; a01 = a[yi][:, xi + 1]; a10 = a[yi + 1][:, xi]; a11 = a[yi + 1][:, xi + 1]
        return (a00 * (1 - tx) + a01 * tx) * (1 - ty) + (a10 * (1 - tx) + a11 * tx) * ty
    out = np.zeros((H, W, 3), np.uint8)
    out[:] = BARE
    wet = bil(((gs & 1) != 0).astype(float)) > 0.5
    rock = bil(((gs & 8) != 0).astype(float)) > 0.5
    out[wet] = WATER
    best = np.zeros((H, W)); bk = np.full((H, W), -1)
    for k in range(4):
        d = bil(np.where((kind == k) & (bs != 0), dens, 0.0))
        m = d > best
        best = np.where(m, d, best); bk = np.where(m, k, bk)
    for k in range(4):
        out[(bk == k) & (best >= 0.2)] = COL[k]
    out[rock] = ROCK
    return Image.fromarray(out)

def busiest(b, w, h):
    kind = np.where(b != 0, b >> 6, -1)
    ch = (kind[:, 1:] != kind[:, :-1])[:-1, :] | (kind[1:, :] != kind[:-1, :])[:, :-1]
    best, at = -1, (0, 0)
    for y in range(0, ch.shape[0] - h, 10):
        for x in range(0, ch.shape[1] - w, 10):
            s = ch[y:y + h, x:x + w].sum()
            if s > best:
                best, at = s, (x, y)
    return at

for name in sys.argv[2:]:
    go, bo = load(f'{D}/old/{name}.png')
    gn, bn = load(f'{D}/new/{name}.png')
    w, h = 64, 36
    x0, y0 = busiest(bo, w, h)
    a = draw(go, bo, x0, y0, w, h); c = draw(gn, bn, x0, y0, w, h)
    sheet = Image.new('RGB', (a.width, a.height * 2 + 8), (255, 255, 255))
    sheet.paste(a, (0, 0)); sheet.paste(c, (0, a.height + 8))
    sheet.save(f'{D}/{name}-edges.png')
    print(name, 'crop at fine cell', (x0, y0), 'old top, new bottom')
