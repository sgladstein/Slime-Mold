"""Builds the Lure rings page's clips and prints the counts beside them.

Run from the repo root with the six film folders under FILMS (made by
raw/film.sh: before-, after- and none- for z6 and z12, each with its game
lines in FILMS/<name>.txt):

    python3 Reports/lure-rings-2026-11-18/raw/make_page.py FILMS

Writes page/img/<name>.mp4 and a poster for the before and after films, and
prints for each how much of the picture the Lure's sign changes: the same
frame of the `none` film (the same game with no sign drawn) is the measure,
so every pixel that differs is the sign.
"""
import glob
import os
import re
import subprocess
import sys
import tempfile

import numpy as np
from PIL import Image

FILMS = sys.argv[1]
HERE = os.path.dirname(os.path.abspath(__file__))
IMG = os.path.join(HERE, "..", "page", "img")
# Leave out the HUD's top strip and bottom bar: the fight is the point.
BOX = (0, 28, 1280, 708)
POSTER = 150
# A pixel counts as changed when any channel moves by more than this of 255.
MOVED = 12


def t_of(f):
    m = re.search(r"-(\d+)(ms|s)-near\.png$", f)
    return int(m.group(1)) * (1000 if m.group(2) == "s" else 1)


def frames(name):
    return sorted(glob.glob(os.path.join(FILMS, name, "*-near.png")), key=t_of)


def clip(name, base):
    fs, bs = frames(name), frames(base)
    assert len(fs) == len(bs) and [t_of(f) for f in fs] == [t_of(f) for f in bs]
    tmp = tempfile.mkdtemp()
    cover, slime, lift = [], [], []
    for i, (f, b) in enumerate(zip(fs, bs)):
        im = Image.open(f).convert("RGB").crop(BOX)
        im.save(os.path.join(tmp, f"{i:03}.png"))
        a = np.asarray(im).astype(int)
        n = np.asarray(Image.open(b).convert("RGB").crop(BOX)).astype(int)
        moved = np.abs(a - n).max(axis=2) > MOVED
        # Slime in the picture with no sign: gold to orange, red well over blue.
        r, g, bl = n[..., 0], n[..., 1], n[..., 2]
        is_slime = (r > 150) & (r > 1.6 * bl) & (g > 0.35 * r)
        cover.append(moved.mean())
        slime.append(moved[is_slime].mean() if is_slime.any() else 0.0)
        lift.append(np.abs(a - n).mean())
        if i == POSTER:
            im.save(os.path.join(IMG, f"{name}.jpg"), quality=86)
    out = os.path.join(IMG, f"{name}.mp4")
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "30", "-i", os.path.join(tmp, "%03d.png"),
                    "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "26", "-preset", "slow", "-movflags", "+faststart", out], check=True)
    lines = [l for l in open(os.path.join(FILMS, f"{name}.txt")) if l.startswith("  t ")]
    print(f"{name}: {len(fs)} frames, {os.path.getsize(out) // 1024} KB; picture changed mean {100 * np.mean(cover):.1f}% max {100 * max(cover):.1f}%; "
          f"slime changed mean {100 * np.mean(slime):.1f}% max {100 * max(slime):.1f}%; mean change {np.mean(lift):.2f} of 255")
    print("   ", lines[0].strip())
    print("   ", lines[-1].strip())
    return lines


os.makedirs(IMG, exist_ok=True)
for z in ("z6", "z12"):
    none = [l for l in open(os.path.join(FILMS, f"none-{z}.txt")) if l.startswith("  t ")]
    before, after = clip(f"before-{z}", f"none-{z}"), clip(f"after-{z}", f"none-{z}")
    print(f"  {z}: the three games' lines are {'the same' if before == after == none else 'DIFFERENT'} ({len(after)} lines)")
