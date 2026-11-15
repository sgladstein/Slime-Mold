#!/usr/bin/env python3
"""Copies the three reviewers' chosen pictures into PAGE/pics as WebP (and
their GIFs as MP4 through ffmpeg) for the review page.

  python3 make_pics.py PAGE AESTHETICS_DIR FUN_DIR

AESTHETICS_DIR and FUN_DIR are the reviewers' working folders
(/home/user/review-aesthetics and /home/user/review-fun in the review's
container); the scripts that made the pictures are in raw/aesthetics and
raw/fun beside this file.
"""
import subprocess
import sys
from pathlib import Path

from PIL import Image

page, a, f = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
out = page / "pics"
out.mkdir(parents=True, exist_ok=True)

STILLS = [
    (a / "core-6x-fit-1280x800-zoom0.86-far-view.png", "far-view-6x"),
    (a / "core-6x-fit-1600x1000-shaded-s1.png", "shaded-6x"),
    (a / "crops/minimap-p8s1-6x-vs-p3s1-1x.png", "minimap"),
    (a / "thumbs-001s-colour.png", "thumbs-colour"),
    (a / "thumbs-001s-grey.png", "thumbs-grey"),
    (a / "ident-z12-colour.png", "ident-z12"),
    (a / "core-6x-fit-1600x1000-shaded-s2-streams.png", "core-streams"),
    (a / "frames/close/8-2-nest-z6/planet-8-2-idle-001s-near.png", "nest-ring"),
    (a / "caves-p7s1-fit-rock-equals-backdrop.png", "caves"),
    (a / "crops/p1s2-z12-clay-cracks-over-slime.png", "cracks-through-slime"),
    (a / "crops/p4s1-river-fords-zoom1-and-x3.png", "fords"),
    (a / "crops/p4s2-z24-bridge.png", "bridge"),
    (a / "crops/p3s2-z12-hedge-across-water.png", "hedge-water"),
    (a / "crops/p1s1-z24-rock-edge.png", "rock-edge"),
    (f / "pics/sheet-d1-4.png", "layouts-1-4"),
    (f / "pics/sheet-d5-8.png", "layouts-5-8"),
    (f / "pics/core-8-1-5.png", "core-nests"),
    (f / "pics/core-8-5.png", "core-rivers"),
    (f / "pics/valley-4-6.png", "valley"),
]
CLIPS = [
    (a / "gif-front-p3s1-z12.gif", "front-fen"),
    (a / "gif-fire-p1s1-z12.gif", "fire"),
]

for src, name in STILLS:
    im = Image.open(src).convert("RGB")
    im.save(out / f"{name}.webp", quality=88, method=6)
    print(name, im.size)
for src, name in CLIPS:
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-i", str(src), "-movflags", "+faststart", "-pix_fmt", "yuv420p", "-vf", "scale=trunc(iw/2)*2:trunc(ih/2)*2", "-crf", "20", str(out / f"{name}.mp4")], check=True)
    print(name, "mp4")
