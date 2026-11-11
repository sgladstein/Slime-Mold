#!/usr/bin/env python3
"""Paper sketches of the five WM2 maps (Larder World), 2026-11-11.

These are drawings of a map *spec*, not the simulation: no walker has run
on them. Each map is a list of shapes (height bumps and trenches, rock,
soft rock, soil patches, crystals, mould starts, the squad's landing),
rasterised here onto the 1024 x 640 world so the layouts can be looked at.
The spec is kept as `maps.json` so WM1's loader (Larder thread) can read
it or convert it to whatever format it settles on.

Ground kinds named here (height, soil kinds, soft rock) are WM3 proposals:
see Reports/larder-maps-2026-11-11.md. Run: python3 sketch_maps.py
(numpy + Pillow), writes sketches/<map>.png and maps.json beside it.
"""
import json, math, os
import numpy as np
from PIL import Image, ImageDraw

W, H = 1024, 640
HERE = os.path.dirname(os.path.abspath(__file__))
yy, xx = np.mgrid[0:H, 0:W].astype(np.float32)

def seg_dist(ax, ay, bx, by):
    dx, dy = bx - ax, by - ay
    t = np.clip(((xx - ax) * dx + (yy - ay) * dy) / max(dx * dx + dy * dy, 1e-6), 0, 1)
    return np.hypot(xx - (ax + t * dx), yy - (ay + t * dy))

def poly_dist(pts):
    d = np.full((H, W), 1e9, np.float32)
    for (a, b) in zip(pts, pts[1:]):
        d = np.minimum(d, seg_dist(*a, *b))
    return d

def noise(seed, cell, amp):
    rng = np.random.default_rng(seed)
    gw, gh = W // cell + 2, H // cell + 2
    g = rng.standard_normal((gh, gw)).astype(np.float32)
    img = Image.fromarray(g).resize((gw * cell, gh * cell), Image.BICUBIC)
    return np.asarray(img)[:H, :W] * amp

def bump(x, y, sigma, dh):
    return dh * np.exp(-((xx - x) ** 2 + (yy - y) ** 2) / (2 * sigma * sigma))

def ridge(pts, width, dh):
    return dh * np.exp(-(poly_dist(pts) / width) ** 2)

SOILS = {"loam": 0, "sand": 1, "clay": 2, "warm": 3, "cold": 4, "barren": 5}

# Each map: name, the plan it is drawn for, height ops, rock, soft rock
# (natural walls with health), soil patches, waterline, crystals
# (x, y, radius, charge), mould starts (x, y, radius, tier), landing.
MAPS = [
 dict(name="basin", title="The Basin", plan="turtle (hold the rim)",
      height=[("base", 170), ("bump", 512, 330, 230, -150), ("ridge", [(40, 40), (984, 40)], 30, 40),
              ("bump", 150, 540, 70, 30), ("bump", 880, 120, 80, 25), ("noise", 3, 48, 6)],
      waterline=30, rock=[("poly", [(300, 70), (420, 120), (600, 120), (720, 70)], 10)],
      soft=[], soil=[],
      crystals=[(470, 300, 14, 512), (560, 360, 14, 512), (600, 260, 12, 384),
                (200, 200, 9, 256), (820, 230, 9, 256), (300, 520, 9, 256), (740, 520, 9, 256),
                (512, 80, 9, 256), (90, 600, 7, 192), (950, 80, 7, 192), (950, 600, 7, 192), (80, 70, 7, 192)],
      moulds=[(512, 250, 24, "Drifter or Sprawler")], landing=(150, 540)),
 dict(name="fork", title="The Fork", plan="lure (bait one fork, harvest the other)",
      height=[("base", 200), ("grad_y", -0.12),
              ("trench", [[(512, 640), (512, 380), (180, 40)], [(512, 380), (844, 40)]], 60, -120),
              ("noise", 5, 40, 6)],
      waterline=0,
      rock=[("poly", [(440, 600), (440, 430)], 6), ("poly", [(584, 600), (584, 430)], 6)],
      soft=[], soil=[("disc", 512, 600, 70, "clay")],
      crystals=[(512, 600, 12, 384), (380, 250, 10, 384), (260, 120, 13, 512), (644, 250, 10, 384), (764, 120, 13, 512),
                (512, 150, 9, 256), (470, 230, 7, 192), (560, 220, 7, 192), (120, 450, 8, 192), (900, 450, 8, 192)],
      moulds=[(512, 590, 26, "Trunk")], landing=(512, 180)),
 dict(name="flats", title="The Flats", plan="boom (out-harvest a mould that cannot hold sand)",
      height=[("base", 120), ("noise", 7, 64, 8)], waterline=0, rock=[], soft=[],
      soil=[("all", "sand"), ("disc", 760, 320, 60, "clay"), ("disc", 560, 140, 45, "clay"), ("disc", 560, 500, 45, "clay"),
            ("disc", 330, 240, 40, "clay"), ("disc", 880, 560, 40, "clay")],
      crystals="scatter", moulds=[(760, 320, 30, "Sprawler")], landing=(110, 320)),
 dict(name="nest", title="The Nest", plan="rush (strike the nest before it breaks out)",
      height=[("base", 110), ("noise", 11, 56, 10), ("bump", 640, 320, 150, 30)], waterline=0,
      rock=[], soft=[("ring", 640, 320, 135, 8, [(200, 12)])],
      soil=[("disc", 640, 320, 128, "warm")],
      crystals=[(600, 290, 15, 640), (690, 360, 15, 640), (180, 120, 8, 192), (180, 520, 8, 192),
                (380, 320, 8, 192), (420, 90, 7, 160), (420, 560, 7, 160), (930, 120, 7, 160), (930, 540, 7, 160)],
      moulds=[(640, 320, 18, "Sleeper or Hunter")], landing=(130, 320)),
 dict(name="passes", title="The Passes", plan="adapt (set two moulds on each other)",
      height=[("base", 140), ("ridge", [(0, 215), (1024, 215)], 40, 70), ("ridge", [(0, 425), (1024, 425)], 40, 70),
              ("noise", 13, 56, 8)], waterline=0,
      rock=[("poly", [(220, 215), (1024, 215)], 9), ("poly", [(0, 215), (150, 215)], 9),
            ("poly", [(0, 425), (430, 425)], 9), ("poly", [(500, 425), (1024, 425)], 9)],
      soft=[("poly", [(150, 215), (220, 215)], 6)],
      soil=[("rect", 0, 0, 1024, 200, "cold"), ("rect", 0, 440, 1024, 640, "warm")],
      crystals=[(300, 100, 11, 384), (650, 90, 12, 512), (900, 150, 10, 384), (300, 540, 11, 384), (650, 560, 12, 512),
                (900, 500, 10, 384), (200, 320, 8, 256), (560, 300, 9, 256), (800, 340, 9, 320)],
      moulds=[(860, 90, 26, "rival A (cold valley)"), (860, 570, 26, "rival B (warm valley)")], landing=(90, 320)),
]

def build(m):
    h = np.zeros((H, W), np.float32)
    for op in m["height"]:
        k = op[0]
        if k == "base": h += op[1]
        elif k == "bump": h += bump(*op[1:])
        elif k == "ridge": h += ridge(op[1], op[2], op[3])
        elif k == "trench":
            d = np.minimum.reduce([poly_dist(p) for p in op[1]])
            h += op[3] * np.exp(-(d / op[2]) ** 2)
        elif k == "grad_y": h += (yy - H / 2) * op[1]
        elif k == "noise": h += noise(*op[1:])
    h = np.clip(h, 0, 255)
    rock = np.zeros((H, W), bool)
    for _, pts, wd in m["rock"]:
        rock |= poly_dist(pts) < wd
    soft = np.zeros((H, W), bool)
    for s in m["soft"]:
        if s[0] == "poly":
            soft |= poly_dist(s[1]) < s[2]
        else:
            _, cx, cy, r, wd, gaps = s
            ring = np.abs(np.hypot(xx - cx, yy - cy) - r) < wd
            ang = (np.degrees(np.arctan2(yy - cy, xx - cx)) + 360) % 360
            for a, half in gaps:
                ring &= np.abs(((ang - a + 180) % 360) - 180) > half
            soft |= ring
    soil = np.zeros((H, W), np.uint8)
    for s in m["soil"]:
        if s[0] == "all": soil[:] = SOILS[s[1]]
        elif s[0] == "disc": soil[np.hypot(xx - s[1], yy - s[2]) < s[3]] = SOILS[s[4]]
        elif s[0] == "rect": soil[s[2]:s[4], s[1]:s[3]] = SOILS[s[5]]
    if m["crystals"] == "scatter":
        rng = np.random.default_rng(42)
        cs = [(760, 320, 12, 384), (560, 140, 9, 256), (560, 500, 9, 256), (330, 240, 9, 256), (880, 560, 9, 256)]
        while len(cs) < 32:
            x, y = int(rng.uniform(40, 984)), int(rng.uniform(40, 600))
            if all(math.hypot(x - a, y - b) > 70 for a, b, *_ in cs) and math.hypot(x - 110, y - 320) > 60:
                cs.append((x, y, 6, 96))
        m["crystals"] = cs
    return h, rock, soft, soil

def hexagon(cx, cy, r):
    return [(cx + r * math.cos(math.radians(60 * i + 30)), cy + r * math.sin(math.radians(60 * i + 30))) for i in range(6)]

SOIL_TINT = {1: (1.10, 1.02, 0.80), 2: (1.05, 0.85, 0.75), 3: (1.15, 0.90, 0.70), 4: (0.85, 0.95, 1.15), 5: (0.9, 0.9, 0.9)}

def render(m, h, rock, soft, soil):
    gy, gx = np.gradient(h)
    shade = np.clip(0.75 + (-gx - gy) * 0.06, 0.35, 1.2)
    lum = (0.25 + 0.55 * h / 255.0) * shade
    img = np.stack([lum * 0.75, lum * 0.80, lum * 0.62], -1)
    for k, t in SOIL_TINT.items():
        img[soil == k] *= t
    band = (np.floor(h / 24) != np.floor(np.roll(h, 1, 1) / 24)) | (np.floor(h / 24) != np.floor(np.roll(h, 1, 0) / 24))
    img[band] *= 0.75
    wet = h < m["waterline"]
    img[wet] = img[wet] * 0.4 + np.array([0.10, 0.25, 0.45])
    img[rock] = (0.12, 0.11, 0.12)
    img[soft] = (0.45, 0.32, 0.20)
    im = Image.fromarray((np.clip(img, 0, 1) * 255).astype(np.uint8))
    d = ImageDraw.Draw(im)
    for x, y, r, c in m["crystals"]:
        d.polygon(hexagon(x, y, r), fill=(60, 220, 230), outline=(230, 255, 255))
    for x, y, r, tier in m["moulds"]:
        d.ellipse([x - r, y - r, x + r, y + r], outline=(255, 210, 40), width=4)
        d.text((x + r + 4, y - 6), tier, fill=(255, 220, 90))
    lx, ly = m["landing"]
    d.rectangle([lx - 12, ly - 12, lx + 12, ly + 12], outline=(255, 255, 255), width=3)
    d.text((lx + 16, ly - 6), "squad lands", fill=(255, 255, 255))
    return im

if __name__ == "__main__":
    os.makedirs(os.path.join(HERE, "sketches"), exist_ok=True)
    stats = {}
    for m in MAPS:
        h, rock, soft, soil = build(m)
        render(m, h, rock, soft, soil).save(os.path.join(HERE, "sketches", m["name"] + ".png"), optimize=True)
        wet = (h < m["waterline"]).mean()
        stats[m["name"]] = dict(h_min=int(h.min()), h_max=int(h.max()), wet=round(float(wet), 3),
                                rock=round(float(rock.mean()), 3), soft=round(float(soft.mean()), 3),
                                crystals=len(m["crystals"]), charge=int(sum(c[3] for c in m["crystals"])))
    with open(os.path.join(HERE, "maps.json"), "w") as f:
        json.dump(dict(world=[W, H], soils=SOILS, maps=MAPS, stats=stats), f, indent=1)
    for k, v in stats.items():
        print(k, v)
