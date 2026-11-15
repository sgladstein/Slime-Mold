"""Before/after crops for the menu-fixes page.

Run with the frames from `larder-world look=DIR` (see the README) under
FIX/before/<name>/ and FIX/after/<name>/; writes webp files to ../page/.
python3 make_pics.py FIX
"""
import os
import sys

from PIL import Image

FIX = sys.argv[1]
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "page")

# name -> (folder, crop box in pixels or None for the whole frame, width to save)
PICS = {
    "bar-run-100": ("squads100", (0, 930, 1920, 1080), 1920),
    "top-run-100": ("squads100", (560, 0, 1360, 56), 1600),
    "bar-run-200": ("squads200", (0, 800, 1920, 1080), 1920),
    "build-run-100": ("build100h", (1000, 925, 1760, 1080), 1520),
    "tiles-100": ("sq30", (0, 930, 1020, 1080), 1530),
    "main-100": ("main100", (0, 0, 960, 900), 960),
    "options-150": ("options150", (0, 240, 1260, 1080), 1260),
}

for name, (folder, box, width) in PICS.items():
    for v in ("before", "after"):
        d = os.path.join(FIX, v, folder)
        if not os.path.isdir(d):
            continue
        near = [f for f in os.listdir(d) if f.endswith("-near.png")]
        if not near:
            continue
        im = Image.open(os.path.join(d, near[0])).convert("RGB")
        if box:
            im = im.crop(box)
        if im.width != width:
            im = im.resize((width, round(im.height * width / im.width)), Image.LANCZOS)
        im.save(os.path.join(OUT, f"{name}-{v}.webp"), quality=88)
        print(name, v, im.size)
