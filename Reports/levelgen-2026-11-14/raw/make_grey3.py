#!/usr/bin/env python3
"""The graphics lead's grey thumbnail test again once the ground thread's
river look (#292: deep water deepening to its middle, plank bridges, water
darker than ground) and the plants (#290) reached main: seed 1 of planets
1-8 filmed at 1 s in the game, cropped to the map as make_grey.py does,
266 x 164 each, in a 4 x 2 sheet in grey and in colour.

  larder-world map=planet:D:1 seed=1 look=FILM/pD-1 hand=idle at=1 (D = 1..8)
  python3 make_grey3.py FILM PAGE TAG
"""
import sys
from pathlib import Path

from PIL import Image

film, page, tag = Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3]
out = page / "grey"
out.mkdir(parents=True, exist_ok=True)
CROP = (111, 47, 1172, 701)
THUMB = (266, 164)
gap = 6
for mode, suffix in (("L", ""), ("RGB", "-colour")):
    sheet = Image.new(mode, (4 * THUMB[0] + 3 * gap, 2 * THUMB[1] + gap), 0)
    for d in range(1, 9):
        f = film / f"p{d}-1" / f"planet-{d}-1-idle-001s-all.png"
        im = Image.open(f).convert("RGB").crop(CROP).convert(mode).resize(THUMB, Image.LANCZOS)
        i = d - 1
        sheet.paste(im, ((i % 4) * (THUMB[0] + gap), (i // 4) * (THUMB[1] + gap)))
    sheet.save(out / f"sheet-{tag}{suffix}.png", optimize=True)
print(out)
