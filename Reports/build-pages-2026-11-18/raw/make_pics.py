"""The page's pictures from film.sh's frames (Reports/build-pages-2026-11-18).

    python3 make_pics.py OUT IMG

OUT is film.sh's output folder, IMG the page's `img/`. Each shot's whole
screen (the `-all` frame, 1920 x 1080) is scaled to 1600 x 900; `mark.jpg`
is the top right of the round with two experiments on and the cursor on
the mark, 960 x 300 at full size, so the mark and its tip read.
"""
import glob
import os
import sys

from PIL import Image

out, img = sys.argv[1], sys.argv[2]
os.makedirs(img, exist_ok=True)


def frame(shot):
    (path,) = glob.glob(os.path.join(out, shot, "*-all.png"))
    return Image.open(path).convert("RGB")


for shot in ["main", "news", "try", "main200", "news200", "try200", "play"]:
    frame(shot).resize((1600, 900), Image.LANCZOS).save(os.path.join(img, shot + ".jpg"), quality=86)
f = frame("play2")
f.crop((f.width - 960, 0, f.width, 300)).save(os.path.join(img, "mark.jpg"), quality=90)
