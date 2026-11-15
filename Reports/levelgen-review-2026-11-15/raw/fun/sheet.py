"""False-colour contact sheet of planet map files (ground, plants, walls,
crystals, nests, landing). Usage: sheet.py MAPDIR OUT.png SEEDS DEPTHS [width]"""
import sys, os, math
import numpy as np
from PIL import Image, ImageDraw

SOIL = {0: (46, 52, 60), 1: (150, 128, 84), 2: (110, 80, 62), 3: (128, 128, 120), 5: (170, 110, 50)}


def render(d, name, W):
    a = np.array(Image.open(os.path.join(d, name + '.png')))
    g = a[..., 1].astype(int)
    b = a[..., 2].astype(int)
    h, w = g.shape
    img = np.zeros((h, w, 3), float)
    soil = g >> 4
    for k, c in SOIL.items():
        img[soil == k] = c
    dens = (b & 63) / 63.0
    green = np.array([60, 120, 60])
    img = img * (1 - 0.6 * dens[..., None]) + green * 0.6 * dens[..., None]
    wet = (g & 1) != 0
    img[wet] = (40, 80, 140)
    img[g == 0x41] = (15, 30, 90)
    img[(g & 8) != 0] = (190, 190, 195)
    im = Image.fromarray(img.clip(0, 255).astype(np.uint8))
    k = W / w
    im = im.resize((W, int(h * k)), Image.BILINEAR)
    dr = ImageDraw.Draw(im)
    for line in open(os.path.join(d, name + '.txt')):
        p = line.split()
        if not p:
            continue
        if p[0] == 'wall':
            x0, y0, x1, y1, t = map(float, p[1:6])
            dr.line([(x0 * k, y0 * k), (x1 * k, y1 * k)], fill=(220, 60, 220), width=max(1, int(t * k)))
    for line in open(os.path.join(d, name + '.txt')):
        p = line.split()
        if not p:
            continue
        if p[0] == 'crystal':
            x, y, r, c = map(float, p[1:5])
            dr.ellipse([(x - r) * k, (y - r) * k, (x + r) * k, (y + r) * k], fill=(0, 230, 230) if c < 384 else (120, 255, 255), outline=(0, 0, 0))
        elif p[0] == 'mould':
            x, y = map(float, p[1:3])
            R = 60
            dr.ellipse([(x - R) * k, (y - R) * k, (x + R) * k, (y + R) * k], outline=(255, 40, 40), width=3)
        elif p[0] == 'landing':
            x, y = map(float, p[1:3])
            R = 45
            dr.rectangle([(x - R) * k, (y - R) * k, (x + R) * k, (y + R) * k], outline=(255, 230, 0), width=3)
    return im


def rng(v):
    a, _, b = v.partition('-')
    return list(range(int(a), int(b or a) + 1))


if __name__ == '__main__':
    d, out, seeds, depths = sys.argv[1], sys.argv[2], rng(sys.argv[3]), rng(sys.argv[4])
    W = int(sys.argv[5]) if len(sys.argv) > 5 else 400
    tiles = [[render(d, f'planet-{dp}-{s}', W) for s in seeds] for dp in depths]
    H = [max(t.size[1] for t in row) for row in tiles]
    sheet = Image.new('RGB', (W * len(seeds) + 6 * (len(seeds) + 1), sum(H) + 22 * len(depths) + 6), (12, 12, 16))
    dr = ImageDraw.Draw(sheet)
    y = 6
    for row, hh, dp in zip(tiles, H, depths):
        for i, (t, s) in enumerate(zip(row, seeds)):
            x = 6 + i * (W + 6)
            dr.text((x, y), f'planet {dp} seed {s}', fill=(230, 230, 230))
            sheet.paste(t, (x, y + 16))
        y += hh + 22
    sheet.save(out)
