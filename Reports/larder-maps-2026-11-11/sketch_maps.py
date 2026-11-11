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

# Soil kinds, as BigDish::soil holds them (SOIL_SAND 1, SOIL_CLAY 2,
# SOIL_BARREN 3); 0 is plain ground.
SOILS = {"plain": 0, "sand": 1, "clay": 2, "barren": 3}

# Version 2 (owner, 20:06Z: "Your map generation is heavily focused on
# heights. How much does height actually influence our slime mold
# behaviors?"): flat. Height is parked (R stays 0); the ground is rock,
# water (wet), soil kinds, soft rock (walls with health), crystals. Each
# map: name, the plan it is drawn for, rock polylines and discs, water
# discs, soft rock, soil patches, crystals (x, y, radius, charge), mould
# starts (x, y, radius, tier), landing. All in 1024 x 640 cells (the
# Larder world's pile_r of 6 dish cells is 24 here); crystal radii are
# 3x version 1's (7-15 were under the engine's own pile; at 2x the maps
# held 25-43% of the race world's crystal cells, Larder thread 21:32Z). Version 1 (height-led) is in git
# history (PR #76).
MAPS = [
 dict(name="rim", title="The Rim", plan="turtle (hold the gaps in the rim)",
      rock=[("ring", 512, 320, 200, 16, [(0, 14), (90, 10), (180, 14), (270, 10)])],
      wet=[("disc", 470, 360, 34)], soft=[],
      soil=[("disc", 512, 320, 184, "clay"), ("ring", 512, 320, 250, 30, "sand")],
      crystals=[(460, 280, 42, 512), (575, 360, 42, 512), (560, 250, 36, 384),
                (150, 120, 27, 256), (880, 120, 27, 256), (150, 520, 27, 256), (880, 520, 27, 256),
                (512, 50, 27, 256), (512, 595, 27, 256), (60, 320, 24, 192), (970, 320, 24, 192)],
      moulds=[(512, 320, 24, "Drifter or Sprawler")], landing=(90, 580)),
 dict(name="fork", title="The Fork", plan="lure (bait one fork, harvest the other)",
      rock=[("poly", [(440, 640), (440, 420), (200, 120)], 9), ("poly", [(584, 640), (584, 420), (824, 120)], 9),
            ("poly", [(400, 330), (512, 400), (624, 330)], 9),
            ("poly", [(240, 40), (330, 160)], 8), ("poly", [(784, 40), (694, 160)], 8)],
      wet=[("disc", 260, 80, 30)], soft=[],
      soil=[("disc", 512, 600, 80, "clay")],
      crystals=[(512, 600, 36, 384), (330, 300, 30, 384), (180, 60, 39, 512), (694, 300, 30, 384), (844, 60, 39, 512),
                (512, 170, 27, 256), (470, 250, 21, 192), (560, 240, 21, 192), (120, 450, 24, 192), (900, 450, 24, 192)],
      moulds=[(512, 590, 26, "Trunk")], landing=(512, 225)),
 dict(name="flats", title="The Flats", plan="boom (out-harvest a mould that cannot hold sand)",
      rock=[], wet=[], soft=[],
      soil=[("all", "sand"), ("disc", 760, 320, 60, "clay"), ("disc", 560, 140, 45, "clay"), ("disc", 560, 500, 45, "clay"),
            ("disc", 330, 240, 40, "clay"), ("disc", 880, 560, 40, "clay")],
      crystals="scatter", moulds=[(760, 320, 30, "Sprawler")], landing=(110, 320)),
 dict(name="nest", title="The Nest", plan="rush (strike the nest before it breaks out)",
      rock=[("disc", 300, 140, 50), ("disc", 300, 500, 50)], wet=[("disc", 860, 120, 40), ("disc", 860, 520, 40)],
      soft=[("ring", 640, 320, 135, 8, [(200, 12)])],
      soil=[("disc", 640, 320, 128, "clay")],
      crystals=[(600, 290, 45, 640), (690, 360, 45, 640), (180, 120, 24, 192), (180, 520, 24, 192),
                (380, 320, 24, 192), (420, 90, 21, 160), (420, 560, 21, 160), (930, 120, 21, 160), (930, 540, 21, 160)],
      moulds=[(640, 320, 18, "Sleeper or Hunter")], landing=(130, 320)),
 dict(name="passes", title="The Passes", plan="adapt (set two moulds on each other)",
      rock=[("poly", [(220, 215), (1024, 215)], 9), ("poly", [(0, 215), (150, 215)], 9),
            ("poly", [(0, 425), (430, 425)], 9), ("poly", [(500, 425), (1024, 425)], 9)],
      wet=[("disc", 380, 320, 36), ("disc", 700, 300, 30)],
      soft=[("poly", [(150, 215), (220, 215)], 6)],
      soil=[("rect", 0, 0, 1024, 206, "clay"), ("rect", 0, 434, 1024, 640, "sand")],
      crystals=[(300, 100, 33, 384), (650, 90, 36, 512), (900, 150, 30, 384), (300, 540, 33, 384), (650, 560, 36, 512),
                (900, 500, 30, 384), (200, 320, 24, 256), (560, 330, 27, 256), (820, 340, 27, 320)],
      moulds=[(860, 90, 26, "rival A (clay valley)"), (860, 570, 26, "rival B (sand valley)")], landing=(90, 320)),
]

def ring_mask(cx, cy, r, wd, gaps):
    ring = np.abs(np.hypot(xx - cx, yy - cy) - r) < wd
    ang = (np.degrees(np.arctan2(yy - cy, xx - cx)) + 360) % 360
    for a, half in gaps:
        ring &= np.abs(((ang - a + 180) % 360) - 180) > half
    return ring

def build(m):
    h = np.zeros((H, W), np.float32)  # height is parked: R stays 0
    rock = np.zeros((H, W), bool)
    for r in m["rock"]:
        if r[0] == "poly": rock |= poly_dist(r[1]) < r[2]
        elif r[0] == "disc": rock |= np.hypot(xx - r[1], yy - r[2]) < r[3]
        elif r[0] == "ring": rock |= ring_mask(*r[1:])
    wet = np.zeros((H, W), bool)
    for r in m["wet"]:
        wet |= np.hypot(xx - r[1], yy - r[2]) < r[3]
    soft = np.zeros((H, W), bool)
    for s_ in m["soft"]:
        if s_[0] == "poly": soft |= poly_dist(s_[1]) < s_[2]
        else: soft |= ring_mask(*s_[1:])
    soil = np.zeros((H, W), np.uint8)
    for s_ in m["soil"]:
        if s_[0] == "all": soil[:] = SOILS[s_[1]]
        elif s_[0] == "disc": soil[np.hypot(xx - s_[1], yy - s_[2]) < s_[3]] = SOILS[s_[4]]
        elif s_[0] == "rect": soil[s_[2]:s_[4], s_[1]:s_[3]] = SOILS[s_[5]]
        elif s_[0] == "ring": soil[np.abs(np.hypot(xx - s_[1], yy - s_[2]) - s_[3]) < s_[4]] = SOILS[s_[5]]
    if m["crystals"] == "scatter":
        rng = np.random.default_rng(42)
        cs = [(760, 320, 36, 384), (560, 140, 27, 256), (560, 500, 27, 256), (330, 240, 27, 256), (880, 560, 27, 256)]
        while len(cs) < 32:
            x, y = int(rng.uniform(40, 984)), int(rng.uniform(40, 600))
            if all(math.hypot(x - a, y - b) > 70 for a, b, *_ in cs) and math.hypot(x - 110, y - 320) > 60:
                cs.append((x, y, 18, 96))
        m["crystals"] = cs
    wet &= ~rock
    return h, rock, wet, soft, soil

def hexagon(cx, cy, r):
    return [(cx + r * math.cos(math.radians(60 * i + 30)), cy + r * math.sin(math.radians(60 * i + 30))) for i in range(6)]

# Preview colours, close to the graphics thread's world look (soil loam,
# grey rock plateaus, glossy water), with sand pale and clay red-brown.
SOIL_RGB = {0: (0.42, 0.33, 0.22), 1: (0.78, 0.70, 0.48), 2: (0.52, 0.27, 0.18), 3: (0.36, 0.36, 0.34)}

def render(m, h, rock, wet, soft, soil):
    img = np.zeros((H, W, 3), np.float32)
    for k, c in SOIL_RGB.items():
        img[soil == k] = c
    img *= (0.92 + noise(7, 4, 0.08)[..., None])
    img[wet] = img[wet] * 0.25 + np.array([0.10, 0.30, 0.40])
    img[rock] = (0.38, 0.39, 0.42)
    # A cast shadow below and right of rock, as the look lights its plateaus.
    sh = np.roll(np.roll(rock, 4, 0), 4, 1) & ~rock
    img[sh] *= 0.55
    img[soft] = (0.22, 0.30, 0.14)  # thicket green, so soft rock reads apart from soil
    im = Image.fromarray((np.clip(img, 0, 1) * 255).astype(np.uint8))
    d = ImageDraw.Draw(im)
    for x, y, r, c in m["crystals"]:
        d.polygon(hexagon(x, y, r), fill=(60, 220, 230), outline=(230, 255, 255))
    for x, y, r, tier in m["moulds"]:
        d.ellipse([x - r, y - r, x + r, y + r], outline=(255, 210, 40), width=4)
        d.text((x + r + 4, y - 6), tier, fill=(255, 230, 120))
    lx, ly = m["landing"]
    d.rectangle([lx - 12, ly - 12, lx + 12, ly + 12], outline=(255, 255, 255), width=3)
    d.text((lx + 16, ly - 6), "squad lands", fill=(255, 255, 255))
    return im

WALL_HP = 400  # soft rock: the engine's player-wall default health

def soft_segments(m):
    """Soft rock as wall segments (x0, y0, x1, y1, thick) for the engine's
    own build_wall; a ring becomes a polygon of 10-degree chords, leaving
    out the chords inside a gap."""
    segs = []
    for s in m["soft"]:
        if s[0] == "poly":
            segs += [(*a, *b, 2 * s[2]) for a, b in zip(s[1], s[1][1:])]
            continue
        _, cx, cy, r, wd, gaps = s
        def open_at(a):
            return all(abs(((a - g + 180) % 360) - 180) > half for g, half in gaps)
        for a in range(0, 360, 10):
            mid = a + 5
            if open_at(mid):
                p = lambda t: (round(cx + r * math.cos(math.radians(t))), round(cy + r * math.sin(math.radians(t))))
                segs.append((*p(a), *p(a + 10), 2 * wd))
    return segs

def export(m, h, rock, wet, soil):
    """What the Larder world's loader reads (agreed with the Larder thread,
    2026-11-11 19:40Z): maps/<name>.png, RGBA 1024 x 640, R = height
    (ground byte 3), G = ground bits 8-15 (wet 0x01, rock 0x08, soil kind in
    the high nibble), B = 0 (mould-made ground), A = 255; crystal and wall
    cells are left out, the engine makes them. maps/<name>.txt: one item a
    line in 1024 x 640 cells; a wall's thick is its full width."""
    os.makedirs(os.path.join(HERE, "maps"), exist_ok=True)
    g = np.zeros((H, W), np.uint8)
    g |= wet.astype(np.uint8) * 0x01
    g |= rock.astype(np.uint8) * 0x08
    g |= (soil.astype(np.uint8) & 0x0F) << 4
    rgba = np.stack([np.round(h).astype(np.uint8), g, np.zeros_like(g), np.full_like(g, 255)], -1)
    Image.fromarray(rgba, "RGBA").save(os.path.join(HERE, "maps", m["name"] + ".png"), optimize=True)
    lines = [f"crystal {x} {y} {r} {c}" for x, y, r, c in m["crystals"]]
    lines += [f"mould {x} {y}" for x, y, *_ in m["moulds"]]
    lines.append("landing {} {}".format(*m["landing"]))
    lines += [f"wall {x0} {y0} {x1} {y1} {t} {WALL_HP}" for x0, y0, x1, y1, t in soft_segments(m)]
    with open(os.path.join(HERE, "maps", m["name"] + ".txt"), "w") as f:
        f.write("\n".join(lines) + "\n")

if __name__ == "__main__":
    os.makedirs(os.path.join(HERE, "sketches"), exist_ok=True)
    stats = {}
    for m in MAPS:
        h, rock, wet, soft, soil = build(m)
        render(m, h, rock, wet, soft, soil).save(os.path.join(HERE, "sketches", m["name"] + ".png"), optimize=True)
        stats[m["name"]] = dict(wet=round(float(wet.mean()), 3), sand=round(float((soil == 1).mean()), 3),
                                clay=round(float((soil == 2).mean()), 3),
                                rock=round(float(rock.mean()), 3), soft=round(float(soft.mean()), 3),
                                crystals=len(m["crystals"]), charge=int(sum(c[3] for c in m["crystals"])))
        export(m, h, rock, wet, soil)
    with open(os.path.join(HERE, "maps.json"), "w") as f:
        json.dump(dict(world=[W, H], soils=SOILS, maps=MAPS, stats=stats), f, indent=1)
    for k, v in stats.items():
        print(k, v)
