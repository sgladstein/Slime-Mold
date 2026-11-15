"""The openings page's pictures, from frames/{before,after}/land-D (renders.sh): each planet's landing at zoom 3
at 1 s, before and after, seed 2. Run from the folder holding frames/:  python3 make_pics.py OUT_DIR"""
import sys, os
from PIL import Image
out = sys.argv[1]
os.makedirs(out, exist_ok=True)
for d in range(1, 9):
    for tag in ("before", "after"):
        im = Image.open(f"frames/{tag}/land-{d}/planet-{d}-2-idle-001s-near.png").convert("RGB")
        w, h = im.size
        im = im.crop((0, int(h * 0.05), w, int(h * 0.895)))  # the HUD's top and bottom bars
        im.thumbnail((1100, 1100), Image.LANCZOS)
        im.save(f"{out}/land-{d}-{tag}.webp", quality=84, method=6)
