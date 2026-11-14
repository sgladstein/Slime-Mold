#!/usr/bin/env python3
"""The planets page's GIFs and counts, from larder-world's headless frames.

Films (see the report, *Films*): for planet D and seed S,
  larder-world map=planet:D:S seed=S look=FILM/pD-S hand=idle \
      at=1,10,20,...,120 screen=1280x800 > FILM/pD-S.log
This script crops each whole-map frame to the map (the window's bars off),
writes one GIF a planet into PAGE/gif/ and the counts each film printed into PAGE/counts.json.

  python3 make_gifs.py FILM PAGE
"""
import json
import re
import sys
from pathlib import Path

from PIL import Image

film, page = Path(sys.argv[1]), Path(sys.argv[2])
(page / "gif").mkdir(parents=True, exist_ok=True)
# The map inside the 1280 x 800 window at zoom 1 (between the top bar and
# the command card), measured on the frames.
CROP = (111, 47, 1172, 701)
SIZE = (531, 328)
counts = {}
for d in range(1, 9):
    for s in (1, 2):
        name = f"planet-{d}-{s}"
        frames = sorted((film / f"p{d}-{s}").glob(f"{name}-idle-*s-all.png"))
        if not frames:
            continue
        ims = [Image.open(f).convert("RGB").crop(CROP).resize(SIZE, Image.LANCZOS) for f in frames]
        pal = [im.quantize(colors=96, method=Image.MEDIANCUT, dither=Image.NONE) for im in ims]
        durations = [700] * (len(pal) - 1) + [2200]
        pal[0].save(page / "gif" / f"{name}.gif", save_all=True, append_images=pal[1:], duration=durations, loop=0, optimize=True)
        rows = []
        for line in (film / f"p{d}-{s}.log").read_text().splitlines():
            m = re.search(r"t (\d+) s: mould (\d+), soldiers (\d+), bank (\d+), harvested (\d+), shots (\d+), killed (\d+), took (\d+)%, crystals left ([\d.]+)", line)
            if m:
                t, mould, _, _, _, shots, killed, _, left = m.groups()
                rows.append({"t": int(t), "mould": int(mould), "shots": int(shots), "killed": int(killed), "left": float(left)})
        counts[name] = rows
(page / "counts.json").write_text(json.dumps(counts, indent=1))
print(f"{len(counts)} films")

# The close clips: near the squad, a frame every quarter second
# (`at=40,40.25,...,46`), the window's bars cropped off.
(page / "clip").mkdir(parents=True, exist_ok=True)
for d in (3, 6):
    # Whole seconds are named NNNs, the rest NNNNNms: sort by the time.
    def secs(f):
        t = f.name.split("-idle-")[1].split("-")[0]
        return int(t[:-2]) / 1000 if t.endswith("ms") else float(t[:-1])
    frames = sorted((film / f"clip{d}").glob(f"planet-{d}-1-idle-*-near.png"), key=secs)
    if frames:
        ims = [Image.open(f).convert("RGB").crop((0, 47, 1280, 701)).resize((640, 327), Image.LANCZOS) for f in frames]
        pal = [im.quantize(colors=128, method=Image.MEDIANCUT, dither=Image.NONE) for im in ims]
        pal[0].save(page / "clip" / f"near-planet-{d}.gif", save_all=True, append_images=pal[1:], duration=250, loop=0, optimize=True)
        print(f"clip {d}: {len(frames)} frames")
