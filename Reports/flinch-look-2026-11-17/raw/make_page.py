"""Builds the flinch-look page's clips and prints the counts beside them.

Run from the repo root with the four film folders under FILMS (made by
raw/film.sh: before-z6, after-z6, before-z12, after-z12, each with its
game lines in FILMS/<name>.txt):

    python3 Reports/flinch-look-2026-11-17/raw/make_page.py FILMS

Writes page/img/<name>.mp4 and a poster (the frame 5.67 s in), and prints,
for each film, the share of the picture that is tinted green and the share
that is hot, and whether the before and after games are the same game.

The Swarmer's planet is stills only: FILMS/p4-before and FILMS/p4-after,
from `larder-world look=FILMS/p4-<side> run=1:4 mode=squads hand=squads
screen=1280x800 at=20,40,60` with each build; the 40 s frame goes on the
page as page/img/p4-<side>.jpg. So is the flare on bare ground:
FILMS/fl-before and FILMS/fl-after, from `look=FILMS/fl-<side> run=1:1
mode=squads hand=squads screen=1280x800 at=16.5,18,18.8 centre=150,162,6
boom=16,165,168,flare`; the 18 s frame goes on the page as
page/img/flare-<side>.jpg.
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
POSTER = 170


def t_of(f):
    m = re.search(r"-(\d+)(ms|s)-near\.png$", f)
    return int(m.group(1)) * (1000 if m.group(2) == "s" else 1)


def shares(im):
    a = np.asarray(im).astype(int)
    r, g, b = a[..., 0], a[..., 1], a[..., 2]
    # Green: a pixel clearly greener than it is red or blue (the lime slime
    # and the mint ground; the fen's teal water stays under it).
    green = (g > r * 1.06) & (g > b * 1.15) & (g > 70)
    # Hot: red at least 1.9 times green and blue (the flush on thin slime
    # and the crater's ember; gold slime is about 1.3).
    hot = (r > g * 1.9) & (r > b * 1.9) & (r > 150)
    return green.mean(), hot.mean()


def clip(name):
    fs = sorted(glob.glob(os.path.join(FILMS, name, "*-near.png")), key=t_of)
    tmp = tempfile.mkdtemp()
    gs, hs = [], []
    for i, f in enumerate(fs):
        im = Image.open(f).convert("RGB").crop(BOX)
        im.save(os.path.join(tmp, f"{i:03}.png"))
        g, h = shares(im)
        gs.append(g)
        hs.append(h)
        if i == POSTER:
            im.save(os.path.join(IMG, f"{name}.jpg"), quality=86)
    out = os.path.join(IMG, f"{name}.mp4")
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "30", "-i", os.path.join(tmp, "%03d.png"),
                    "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "26", "-preset", "slow", "-movflags", "+faststart", out], check=True)
    lines = [l for l in open(os.path.join(FILMS, f"{name}.txt")) if l.startswith("  t ")]
    print(f"{name}: {len(fs)} frames, {os.path.getsize(out) // 1024} KB; green mean {100 * np.mean(gs):.1f}% max {100 * max(gs):.1f}%; "
          f"hot mean {100 * np.mean(hs):.2f}% max {100 * max(hs):.2f}%")
    print("   ", lines[0].strip())
    print("   ", lines[-1].strip())
    return lines


os.makedirs(IMG, exist_ok=True)
for z in ("z6", "z12"):
    before, after = clip(f"before-{z}"), clip(f"after-{z}")
    print(f"  {z}: the two games' lines are {'the same' if before == after else 'DIFFERENT'} ({len(after)} lines)")

for side in ("before", "after"):
    row = []
    for t in ("020", "040", "060"):
        im = Image.open(os.path.join(FILMS, f"p4-{side}", f"planet-4-1-squads-{t}s-near.png")).convert("RGB").crop(BOX)
        g, h = shares(im)
        row.append(f"{t[1:]} s green {100 * g:.1f}% hot {100 * h:.2f}%")
        if t == "040":
            im.save(os.path.join(IMG, f"p4-{side}.jpg"), quality=86)
    print(f"p4-{side}: " + "; ".join(row))

for side in ("before", "after"):
    im = Image.open(os.path.join(FILMS, f"fl-{side}", "planet-1-1-squads-018s-near.png")).convert("RGB").crop(BOX)
    g, h = shares(im)
    im.save(os.path.join(IMG, f"flare-{side}.jpg"), quality=86)
    print(f"flare-{side}: 18 s green {100 * g:.1f}% hot {100 * h:.2f}%")
