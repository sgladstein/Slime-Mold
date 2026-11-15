"""Before/after crops of the bottom bar for the bar-allocator page.

Run with the frames from `larder-world look=DIR` (commands in the README)
under BAR/main/<name>/ (before, main at 2f6ac3f4) and BAR/alloc/<name>/
(after); writes webp files to ../page/.
python3 make_pics.py BAR
"""
import os
import sys

from PIL import Image

BAR = sys.argv[1]
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "page")

# name -> (frame folder, the bar's height in pixels from the bottom)
PICS = {
    "sq125": ("sq125", 175),
    "run125": ("run125", 175),
    "sq150": ("sq150", 215),
    "sq200": ("sq200", 265),
    "q125": ("q125", 255),
    "w125": ("w125", 180),
}

for name, (folder, tall) in PICS.items():
    for v, tag in (("main", "before"), ("alloc", "after")):
        d = os.path.join(BAR, v, folder)
        near = [f for f in os.listdir(d) if f.endswith("-near.png")]
        im = Image.open(os.path.join(d, near[0])).convert("RGB")
        im = im.crop((0, im.height - tall, im.width, im.height))
        if im.width > 1920:
            im = im.resize((1920, round(im.height * 1920 / im.width)), Image.LANCZOS)
        im.save(os.path.join(OUT, f"{name}-{tag}.webp"), quality=90)
        print(name, tag, im.size)
