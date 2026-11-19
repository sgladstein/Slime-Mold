#!/usr/bin/env python3
"""The soldiers' Armour looks (owner, 2026-11-19 14:41Z: "our base soldier
should be olive that upgraded to slate that upgrades to heavy", then
15:11Z: "Replace heavy with dark camo"): cuts three
designs from `soldiers-2026-11-19.jpg` (the owner's image-model sheet, 3 x 2
on magenta: 1 olive, 2 slate, 3 cloak, 4 radio, 5 heavy, 6 dark camo) into
the game's frame and writes them over tiles 17-22 of `sprites.png`, body
then helmet: olive, slate, dark camo (`view.wgsl`, `skin_body`), and tiles
23 and 24, the fallen in slate and dark camo, recoloured from sprite A's
fallen olive (tile 3; `fallen`, `skin_dead`). Run after
`render.js`, from the repo root: python3 src/world/sprites/cut.py

The frame: a tile covers -2..2 soldier radii, facing +x; the helmet's
centre goes to (0.04, 0) and the scale puts the gun's tip 1.41 radii ahead
of it, where the muzzle flash fires (`MUZZLE` in the window's `draw.rs`).
The magenta is keyed out by how far blue and red stand above green, and
taken back out of the edge pixels. The helmet layer is the largest
grey-white blob and its dark rim, so the game can tint it and draw it again
above a packed squad. Prints each design's helmet radius and muzzle offset
across, in radii (11-19: 0.28, 0.27, 0.29; +0.11, +0.12, +0.23; the
last goes in `MUZZLE` in the window's `draw.rs`).
"""
import os
import numpy as np
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
# Sheet cells (from 0, left to right then down) and their tiles' first index.
PICK = [(0, 17), (1, 19), (5, 21)]
TILES = 25
# The fallen in slate and dark camo (tile 3 is the fallen olive).
FALLEN = [(19, 23), (21, 24)]

def key(cell):
    r, g, b = cell[..., 0], cell[..., 1], cell[..., 2]
    f = np.clip((np.minimum(r, b) - g) / 230.0, 0, 1)
    f = np.clip((f - 0.12) / 0.7, 0, 1)
    a = 1 - f
    # Take the magenta out of what is left (pixel = a * C + (1 - a) * M).
    M = np.array([255.0, 0.0, 255.0])
    c = (cell - (1 - a)[..., None] * M) / np.maximum(a, 1e-3)[..., None]
    c = np.clip(c, 0, 255)
    return c, a

def blobs(mask):
    """Largest 4-connected component of mask (no scipy)."""
    lab = np.zeros(mask.shape, int); best, bestn, n = None, 0, 0
    for y0, x0 in zip(*np.nonzero(mask)):
        if lab[y0, x0]: continue
        n += 1; stack = [(y0, x0)]; lab[y0, x0] = n; pts = []
        while stack:
            y, x = stack.pop(); pts.append((y, x))
            for dy, dx in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                yy, xx = y + dy, x + dx
                if 0 <= yy < mask.shape[0] and 0 <= xx < mask.shape[1] and mask[yy, xx] and not lab[yy, xx]:
                    lab[yy, xx] = n; stack.append((yy, xx))
        if len(pts) > bestn: best, bestn = pts, len(pts)
    return np.array(best)

def place(rgb, a, k, centre, size):
    """Premultiplied resize by k, the helmet centre to (0.04, 0) radii."""
    pm = np.dstack([rgb * a[..., None], a * 255]).astype(np.float32)
    h, w = a.shape
    nw, nh = max(1, round(w * k)), max(1, round(h * k))
    chans = [np.asarray(Image.fromarray(pm[..., i], 'F').resize((nw, nh), Image.LANCZOS)) for i in range(4)]
    small = np.dstack(chans)
    unit = size / 4.0
    ox = round((0.04 + 2) * unit - centre[0] * k); oy = round(2 * unit - centre[1] * k)
    tile = np.zeros((size, size, 4), np.float32)
    ys, xs = slice(max(0, oy), min(size, oy + nh)), slice(max(0, ox), min(size, ox + nw))
    tile[ys, xs] = small[ys.start - oy:ys.stop - oy, xs.start - ox:xs.stop - ox]
    al = np.clip(tile[..., 3], 0, 255)
    rgb2 = np.where(al[..., None] > 0, tile[..., :3] / np.maximum(al[..., None], 1e-3) * 255, 0)
    return Image.fromarray(np.dstack([np.clip(rgb2, 0, 255), al]).astype(np.uint8), 'RGBA')


def lum(c):
    return c[..., 0] * 0.3 + c[..., 1] * 0.59 + c[..., 2] * 0.11

def fallen(dead, live, olive, camo, size):
    """The fallen olive (`dead`, tile 3 at `size` px, RGBA) in another
    look: each pixel of its uniform (the greens) takes the look's colour
    (`live`, its 128 px body tile), kept as much duller than the living
    look as the fallen olive is than the living olive (`olive`), and keeps
    its own shading. With `camo`, the look's three main colours in soft
    blotches, as its camouflage. Hands, boots, rifle and outline stay."""
    d = dead.astype(float)
    rgb, a = d[..., :3], d[..., 3]
    r, g, b = rgb[..., 0], rgb[..., 1], rgb[..., 2]
    cloth = (a > 0) & (g > b + 10) & (g >= r - 4)
    def body(t):
        t = t.astype(float)
        c = t[..., :3][(t[..., 3] > 200) & (t[..., :3].max(-1) - t[..., :3].min(-1) > 8)]
        return c
    ref = np.median(lum(rgb[cloth]))
    dull = ref / np.median(lum(body(olive)))
    shade = (lum(rgb) / ref)[..., None]
    pal = body(live)
    if not camo:
        tone = np.broadcast_to(np.median(pal, 0), rgb.shape)
    else:
        # Three colours by brightness thirds of the look's own pixels.
        L = lum(pal); q = np.quantile(L, [1 / 3, 2 / 3])
        cols = [np.median(pal[L <= q[0]], 0), np.median(pal[(L > q[0]) & (L <= q[1])], 0), np.median(pal[L > q[1]], 0)]
        rng = np.random.default_rng(1119)
        n = Image.fromarray((rng.random((16, 16)) * 255).astype(np.uint8)).resize((128, 128), Image.BICUBIC)
        n = np.asarray(n.resize((size, size), Image.BILINEAR)).astype(float) / 255
        k = np.digitize(n, np.quantile(n, [0.35, 0.7]))
        tone = np.choose(k[..., None], [np.broadcast_to(c, rgb.shape) for c in cols])
    new = np.clip(tone * dull * shade, 0, 255)
    out = np.where(cloth[..., None], new, rgb)
    return Image.fromarray(np.dstack([out, a]).astype(np.uint8), 'RGBA')


def main():
    global W, H
    sheet = np.asarray(Image.open(os.path.join(HERE, 'soldiers-2026-11-19.jpg')).convert('RGB')).astype(float)
    H, W, _ = sheet.shape
    path = os.path.join(HERE, 'sprites.png')
    old = Image.open(path).convert('RGBA')
    out = Image.new('RGBA', (TILES * 128, 160), (0, 0, 0, 0))
    out.paste(old.crop((0, 0, min(old.size[0], 17 * 128), 128)), (0, 0))
    out.paste(old.crop((0, 128, 17 * 32, 160)), (0, 128))
    cw, ch = W / 3, H / 2
    for d, t in PICK:
        cx, cy = d % 3, d // 3
        m = 8
        x0, x1 = int(cx * cw) + m, int((cx + 1) * cw) - m
        y0, y1 = int(cy * ch) + m, int((cy + 1) * ch) - m
        rgb, a = key(sheet[y0:y1, x0:x1])
        mx, mn = rgb.max(-1), rgb.min(-1)
        pts = blobs((a > 0.95) & (mx - mn < 28) & (mx > 150))
        hc = (pts[:, 1].mean(), pts[:, 0].mean())
        hr = np.sqrt(len(pts) / np.pi)
        yy, xx = np.mgrid[0:a.shape[0], 0:a.shape[1]]
        disc = np.clip(1.11 * hr + 0.5 - np.hypot(xx - hc[0], yy - hc[1]), 0, 1)
        cols = np.nonzero((a > 0.5).any(0))[0]
        tx = cols.max()
        ty = np.nonzero(a[:, tx - 2:tx + 1].max(1) > 0.5)[0].mean()
        s = 1.41 / (tx - hc[0])
        print(f"cell {d + 1} -> tiles {t}, {t + 1}: helmet r {hr * s:.2f} radii, muzzle across {(ty - hc[1]) * s:+.2f}")
        for size, row in ((128, 0), (32, 128)):
            k = s * size / 4.0
            out.paste(place(rgb, a, k, hc, size), (t * size, row))
            out.paste(place(rgb, a * disc, k, hc, size), ((t + 1) * size, row))
    for live, t in FALLEN:
        body = np.asarray(out.crop((live * 128, 0, live * 128 + 128, 128)))
        olive = np.asarray(out.crop((17 * 128, 0, 18 * 128, 128)))
        for size, row in ((128, 0), (32, 128)):
            dead = np.asarray(old.crop((3 * size, row, 4 * size, row + size)).convert('RGBA'))
            out.paste(fallen(dead, body, olive, live == 21, size), (t * size, row))
    out.save(path)


if __name__ == '__main__':
    main()
