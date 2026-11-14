"""A false-colour contact sheet of the round-3 biomes (planets 1-8 down,
seeds across), straight from the saved map files, each tile labelled with
the planet's dials, and the same in grey. Plants (the B byte) are mixed
over the ground by kind and density.

  cargo run --release --example planets -- seeds=1-3 depths=1-8 out=MAPS > SURVEY.txt
  python3 biome_sheet.py MAPS SURVEY.txt OUT.png      # also writes OUT-grey.png

Every tile is the same width whatever the planet's size, so planet 1 (60%
of the area) is drawn at a finer scale than planet 3.
"""
import re
import sys
from pathlib import Path

from PIL import Image, ImageDraw

src, survey, out = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
PLANT = {0: (150, 168, 84), 1: (34, 92, 52), 2: (88, 124, 104), 3: (122, 116, 76)}
COL = {5: (150, 116, 74), 0: (38, 42, 48), 1: (120, 104, 70), 2: (96, 60, 46), 3: (70, 70, 66)}
TW, TH, CAP = 384, 240, 18
label = {}
for line in survey.read_text().splitlines():
    m = re.match(r"(\d+) (\d+) (\S+) (\S+) \[([^\]]+)\]", line)
    if m:
        d, s, name, biome, dials = m.groups()
        label[(int(d), int(s))] = f"{d}-{s} {biome}: {dials.lower()}"
seeds = sorted({s for (_, s) in label})
depths = sorted({d for (d, _) in label})


def tile(d, s):
    name = f"planet-{d}-{s}"
    png = Image.open(src / f"{name}.png").convert("RGBA")
    g, b = png.getchannel(1), png.getchannel(2)
    w, h = g.size
    px, pb = g.load(), b.load()
    im = Image.new("RGB", (w, h))
    o = im.load()
    for y in range(h):
        for x in range(w):
            v = px[x, y]
            if v & 8:
                c = (140, 142, 146)
            elif v == 0x41:
                c = (14, 34, 66)  # deep water (soil kind 4 with the wet bit)
            elif v & 1:
                c = (34, 64, 96)
            else:
                c = COL.get(v >> 4, (200, 0, 200))
            # Plants (B: kind in the top two bits, density in the low six)
            # over the ground, as the game mixes them (graphics lead's
            # spec, 17:55Z): meadow pale yellow-green, jungle deep green,
            # reeds olive-teal, scrub dusty brown-green.
            pv = pb[x, y]
            if pv and not v & 8:
                k, dens = pv >> 6, (pv & 63) / 63
                pc = PLANT[k]
                t = 0.25 + 0.75 * dens
                c = tuple(int(c[j] * (1 - t) + pc[j] * t) for j in range(3))
            o[x, y] = c
    dr = ImageDraw.Draw(im)
    for line in (src / f"{name}.txt").read_text().splitlines():
        p = line.split()
        if not p or p[0].startswith("#"):
            continue
        v = list(map(float, p[1:]))
        if p[0] == "crystal":
            dr.ellipse([v[0] - v[2], v[1] - v[2], v[0] + v[2], v[1] + v[2]], fill=(63, 198, 220))
        elif p[0] == "wall":
            dr.line([v[0], v[1], v[2], v[3]], fill=(185, 180, 138), width=int(v[4]))
            r = v[4] / 2
            for (cx, cy) in ((v[0], v[1]), (v[2], v[3])):
                dr.ellipse([cx - r, cy - r, cx + r, cy + r], fill=(185, 180, 138))
        elif p[0] == "mould":
            dr.ellipse([v[0] - 16, v[1] - 16, v[0] + 16, v[1] + 16], outline=(232, 164, 58), width=6)
        elif p[0] == "landing":
            dr.rectangle([v[0] - 14, v[1] - 14, v[0] + 14, v[1] + 14], outline=(255, 255, 255), width=6)
    im = im.resize((TW, TH), Image.LANCZOS)
    full = Image.new("RGB", (TW, TH + CAP), (10, 10, 10))
    full.paste(im, (0, CAP))
    ImageDraw.Draw(full).text((4, 3), label[(d, s)], fill=(220, 220, 220))
    return full


gap = 8
sheet = Image.new("RGB", (len(seeds) * (TW + gap) - gap, len(depths) * (TH + CAP + gap) - gap), (10, 10, 10))
for row, d in enumerate(depths):
    for col, s in enumerate(seeds):
        sheet.paste(tile(d, s), (col * (TW + gap), row * (TH + CAP + gap)))
sheet.save(out)
sheet.convert("L").save(out.with_name(out.stem + "-grey.png"))
