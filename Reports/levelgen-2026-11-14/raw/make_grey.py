#!/usr/bin/env python3
"""Grey thumbnails for the round-2 layouts page: each planet's whole-map frame
at 1 s from round 1's films (R1) and round 2's (R2), cropped to the map as
make_gifs.py does, in grey, plus a 4 x 4 contact sheet of each round.

  python3 make_grey.py R1 R2 PAGE
"""
import sys
from pathlib import Path

from PIL import Image

r1, r2, page = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
out = page / "grey"
out.mkdir(parents=True, exist_ok=True)
CROP = (111, 47, 1172, 701)
THUMB = (266, 164)
for tag, film in (("r1", r1), ("r2", r2)):
    tiles = []
    for d in range(1, 9):
        for s in (1, 2):
            name = f"planet-{d}-{s}"
            f = film / f"p{d}-{s}" / f"{name}-idle-001s-all.png"
            im = Image.open(f).convert("RGB").crop(CROP).convert("L").resize(THUMB, Image.LANCZOS)
            im.save(out / f"{tag}-{name}.png", optimize=True)
            tiles.append(im)
    gap = 6
    sheet = Image.new("L", (4 * THUMB[0] + 3 * gap, 4 * THUMB[1] + 3 * gap), 0)
    for i, t in enumerate(tiles):
        sheet.paste(t, ((i % 4) * (THUMB[0] + gap), (i // 4) * (THUMB[1] + gap)))
    sheet.save(out / f"sheet-{tag}.png", optimize=True)
print(out)
