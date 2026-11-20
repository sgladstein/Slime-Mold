#!/usr/bin/env python3
"""Soldiers' tracks over the ground (pathfinding thread, 2026-11-20).

    python3 tracks.py LOG.csv OUT.png T0 T1 [--squads 0,1] [--box X0,Y0,X1,Y1] [--zoom 4]

LOG.csv is `Game::path_rows` (dish cells); LOG.csv.pgm the ground at fine
cells (`examples/pathprobe` writes it; scale 2). Each soldier's track from
T0 to T1 game seconds is drawn in its squad's colour, darkening with time;
its end is a dot, squad anchors are white crosses, destinations rings.
"""
import sys
from collections import defaultdict

from PIL import Image, ImageDraw

COLS = [(230, 60, 60), (60, 140, 255), (40, 200, 90), (240, 170, 30), (200, 80, 220), (30, 210, 210), (250, 120, 160), (150, 150, 40)]


def main():
    a = sys.argv[1:]
    log, out, t0, t1 = a[0], a[1], float(a[2]), float(a[3])
    squads = None
    box = None
    zoom = 4
    for i, x in enumerate(a):
        if x == "--squads":
            squads = {int(v) for v in a[i + 1].split(",")}
        if x == "--box":
            box = [float(v) for v in a[i + 1].split(",")]
        if x == "--zoom":
            zoom = int(a[i + 1])
    tps = 15
    ground = Image.open(log + ".pgm").convert("RGB")
    # Fine cells to dish cells: half size.
    ground = ground.resize((ground.width // 2, ground.height // 2), Image.NEAREST)
    if box is None:
        box = [0, 0, ground.width, ground.height]
    x0, y0, x1, y1 = box
    img = ground.crop((int(x0), int(y0), int(x1), int(y1))).resize((int((x1 - x0) * zoom), int((y1 - y0) * zoom)), Image.NEAREST)
    d = ImageDraw.Draw(img)
    tr = defaultdict(list)
    anchors = defaultdict(list)
    dests = {}
    for line in open(log):
        p = line.rstrip("\n").split(",")
        if len(p) < 12 or "" in p:
            continue
        t = int(p[1]) / tps
        if t < t0 or t > t1:
            continue
        if p[0] == "s":
            q = int(p[3])
            if squads is not None and q not in squads:
                continue
            tr[int(p[2])].append((t, q, float(p[4]), float(p[5]), int(p[10])))
        else:
            q = int(p[2])
            if squads is not None and q not in squads:
                continue
            if p[3] != "NaN":
                anchors[q].append((float(p[3]), float(p[4])))
            if p[5] != "NaN":
                dests[q] = (float(p[5]), float(p[6]))
    P = lambda x, y: ((x - x0) * zoom, (y - y0) * zoom)
    for k, rows in tr.items():
        for (ta, q, xa, ya, ha), (tb, _, xb, yb, hb) in zip(rows, rows[1:]):
            f = 0.45 + 0.55 * (ta - t0) / max(t1 - t0, 1e-3)
            c = COLS[q % len(COLS)]
            c = tuple(int(v * f) for v in c)
            d.line([P(xa, ya), P(xb, yb)], fill=c, width=1)
        t, q, x, y, h = rows[-1]
        px, py = P(x, y)
        d.ellipse([px - 2, py - 2, px + 2, py + 2], fill=COLS[q % len(COLS)], outline=(0, 0, 0) if h == 0 else (255, 255, 0))
    for q, ps in anchors.items():
        for x, y in ps[:: tps]:
            px, py = P(x, y)
            d.line([px - 3, py, px + 3, py], fill=(255, 255, 255))
            d.line([px, py - 3, px, py + 3], fill=(255, 255, 255))
    for q, (x, y) in dests.items():
        px, py = P(x, y)
        d.ellipse([px - 6, py - 6, px + 6, py + 6], outline=COLS[q % len(COLS)], width=2)
    d.text((4, 4), f"{log.split('/')[-1]}  {t0:.0f}-{t1:.0f} s", fill=(0, 0, 0))
    img.save(out)


if __name__ == "__main__":
    main()
