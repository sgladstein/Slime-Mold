"""Builds the breakaway page's two time-lapse clips and posters.

Run from the repo root with the two film folders under FILMS:

    python3 Reports/breakaway-run-2026-11-15/raw/make_clips.py FILMS

Each film (raw/film.sh): larder-world run=2:2 mode=squads hand=squads
screen=960x600, look=FILMS/film-<arm> at every 0.5 s from 0.5 to 150 s,
with `roam=0` (off: main before this change) or `condense=300 roam=300`
(on: what planet 2 now plays). Writes page/img/<arm>.mp4 at 15 frames a
second (7.5 times the game's pace, 20 s a clip) and a poster at 2:00.
"""
import glob
import os
import shutil
import subprocess
import sys
import tempfile

from PIL import Image

FILMS = sys.argv[1]
HERE = os.path.dirname(os.path.abspath(__file__))
IMG = os.path.join(HERE, "..", "page", "img")


def t_of(f):
    s = os.path.basename(f).rsplit("-", 2)[-2]
    return int(s[:-2]) / 1000 if s.endswith("ms") else float(s[:-1])


def crop(im):
    # The top bar (walkers and clock) and the map; the bottom bar is left out.
    return im.crop((0, 0, 960, 516))


os.makedirs(IMG, exist_ok=True)
for arm in ("off", "on"):
    fs = sorted(glob.glob(os.path.join(FILMS, f"film-{arm}", "*-all.png")), key=t_of)
    tmp = tempfile.mkdtemp()
    for i, f in enumerate(fs):
        crop(Image.open(f).convert("RGB")).save(os.path.join(tmp, f"{i:03}.png"))
    out = os.path.join(IMG, f"{arm}.mp4")
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "15", "-i", os.path.join(tmp, "%03d.png"),
                    "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "31", "-movflags", "+faststart", out], check=True)
    pick = min(fs, key=lambda f: abs(t_of(f) - 120.0))
    crop(Image.open(pick).convert("RGB")).save(os.path.join(IMG, f"{arm}.jpg"), quality=86)
    shutil.rmtree(tmp)
    print(f"{arm}: {len(fs)} frames, {os.path.getsize(out) // 1024} KB")
