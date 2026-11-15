"""Builds the flinch page's clips and prints each film's game lines.

Run from the repo root with the film folders under FILMS, one per flinch
strength (FILMS/<flinch>/, from larder-world look=FILMS/<flinch> ...):

    python3 Reports/flinch-2026-11-15/raw/make_page.py FILMS

Each film: run=1:1 mode=squads hand=squads screen=320x200 flinch=<V>,
frames every 0.1 s from 11 to 18.9 s, plus 30, 60, 90 and 120 s (the
'near' camera follows the squad). Writes page/img/fight-<letter>.mp4 and a
poster (lettered by raw/ab-key.txt, which stays off the page).
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
KEY = dict(line.split()[:2] for line in open(os.path.join(HERE, "ab-key.txt")) if line.strip() and not line.startswith("#"))


def t_of(f):
    s = os.path.basename(f).rsplit("-", 2)[-2]
    return int(s[:-2]) / 1000 if s.endswith("ms") else float(s[:-1])


def frames(folder):
    fs = [f for f in glob.glob(os.path.join(folder, "*-near.png")) if t_of(f) < 19.0]
    return sorted(fs, key=t_of)


def crop(im):
    # Leave out the HUD's bottom bar and top strip: the fight is the point.
    return im.crop((0, 34, 640, 316))


def clip(opt, letter):
    tmp = tempfile.mkdtemp()
    fs = frames(os.path.join(FILMS, opt))
    for i, f in enumerate(fs):
        crop(Image.open(f).convert("RGB")).save(os.path.join(tmp, f"{i:03}.png"))
    out = os.path.join(IMG, f"fight-{letter}.mp4")
    # 0.1 s a frame: the game's own pace.
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "10", "-i", os.path.join(tmp, "%03d.png"),
                    "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "25", "-movflags", "+faststart", out], check=True)
    # The poster: 16 s, the ring at its widest on the flinching films.
    pick = min(fs, key=lambda f: abs(t_of(f) - 16.0))
    crop(Image.open(pick).convert("RGB")).save(os.path.join(IMG, f"fight-{letter}.jpg"), quality=88)
    shutil.rmtree(tmp)
    return out, len(fs)


os.makedirs(IMG, exist_ok=True)
for letter, opt in sorted(KEY.items()):
    out, n = clip(opt, letter)
    lines = [l.strip() for l in open(os.path.join(FILMS, f"{opt}.txt")) if l.startswith("  t ") and l.split()[1] in ("30", "60", "90", "120")]
    print(f"{letter} (flinch {opt}): {n} frames, {os.path.getsize(out) // 1024} KB")
    for l in lines:
        print("   ", l)
