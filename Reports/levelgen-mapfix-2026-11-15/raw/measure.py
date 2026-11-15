#!/usr/bin/env python3
"""Per-planet numbers for the review's map fixes, from the planets
example's map files (`cargo run --release --example planets -- seeds=1-20
depths=1-8 out=DIR`):

  python3 measure.py BEFORE AFTER

- edge: each nest's distance to the nearest map edge, map cells;
- apart: the nearest pair of nests over the map's width (2+ nests);
- hedge on water: soft-rock pieces with a point on water (sampled every
  4 cells along each piece);
- water: the share of the map that is water (shallow or deep).
"""
import sys
from pathlib import Path

import numpy as np
from PIL import Image


def load(base):
    g = np.array(Image.open(f"{base}.png").convert("RGBA"))[:, :, 1].astype(int)
    walls, nests, crystals = [], [], []
    for line in open(f"{base}.txt"):
        w = line.split()
        if not w:
            continue
        if w[0] == "wall":
            walls.append(tuple(map(float, w[1:6])))
        elif w[0] == "mould":
            nests.append(tuple(map(float, w[1:3])))
        elif w[0] == "crystal":
            crystals.append(tuple(map(float, w[1:5])))
    return g, walls, nests, crystals


def measure(base):
    g, walls, nests, _ = load(base)
    h, w = g.shape
    edge = [min(x, y, w - x, h - y) for x, y in nests]
    apart = None
    if len(nests) > 1:
        apart = min(np.hypot(a[0] - b[0], a[1] - b[1]) for i, a in enumerate(nests) for b in nests[i + 1:]) / w
    wet = (g & 1) != 0
    on_water = 0
    for ax, ay, bx, by, _t in walls:
        n = max(2, int(np.hypot(bx - ax, by - ay) / 4) + 1)
        for k in range(n):
            t = k / (n - 1)
            x, y = int(ax + (bx - ax) * t), int(ay + (by - ay) * t)
            if 0 <= x < w and 0 <= y < h and wet[y, x]:
                on_water += 1
                break
    return {"edge": edge, "apart": apart, "on_water": on_water, "walls": len(walls), "water": wet.mean(), "w": w}


before, after = Path(sys.argv[1]), Path(sys.argv[2])
print("depth | nests within 150 map cells of an edge (before -> after) | nearest pair / width, min and median | hedge pieces on water | water share")
for d in range(1, 9):
    rows = {}
    for tag, src in (("b", before), ("a", after)):
        ms = [measure(src / f"planet-{d}-{s}") for s in range(1, 21) if (src / f"planet-{d}-{s}.png").exists()]
        near = sum(e < 150 for m in ms for e in m["edge"])
        nests = sum(len(m["edge"]) for m in ms)
        ap = [m["apart"] for m in ms if m["apart"] is not None]
        rows[tag] = (near, nests, ap, sum(m["on_water"] for m in ms), sum(m["walls"] for m in ms), np.mean([m["water"] for m in ms]))
    b, a = rows["b"], rows["a"]
    ap = lambda v: f"{min(v):.2f}/{np.median(v):.2f}" if v else "-"
    print(f"{d} | {b[0]}/{b[1]} -> {a[0]}/{a[1]} | {ap(b[2])} -> {ap(a[2])} | {b[3]}/{b[4]} -> {a[3]}/{a[4]} | {b[5]:.3f} -> {a[5]:.3f}")
