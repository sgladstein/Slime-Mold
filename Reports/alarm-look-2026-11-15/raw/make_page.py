"""Builds the alarm-look page's clips and the haze's lift numbers.

Run from the repo root with the film folders (larder-world look=DIR ...
alarm_look=G,S, one folder per strength) under FILMS:

    python3 Reports/alarm-look-2026-11-15/raw/make_page.py FILMS [p4clip] [siren]

FILMS/p4clip/<G_S>/   planet 4 (run=1:4, the Swarmer), hand=squads,
                      frames 39-42 s every 0.125 s, plus 0_0 (no haze)
FILMS/siren/<G_S>/    the Fork, hand=idle, siren=90,160, centre=100,165,5,
                      frames 19.9-22 s every 0.1 s, plus 0_0

Writes page/img/*.mp4 and a middle frame as *.jpg (lettered by raw/ab-key.txt, which stays off the
page) and prints each option's lift over the no-haze frames.
"""
import glob
import os
import shutil
import subprocess
import sys
import tempfile

import numpy as np
from PIL import Image

FILMS = sys.argv[1]
HERE = os.path.dirname(os.path.abspath(__file__))
IMG = os.path.join(HERE, "..", "page", "img")
KEY = dict(line.split()[:2] for line in open(os.path.join(HERE, "ab-key.txt")) if line.strip() and not line.startswith("#"))


def frames(folder):
    fs = glob.glob(os.path.join(folder, "*-near.png"))
    def t(f):
        b = os.path.basename(f).rsplit("-", 2)
        s = b[-2]
        return int(s[:-2]) / 1000 if s.endswith("ms") else float(s[:-1])
    return [f for f in sorted(fs, key=t)]


def crop(im, scene):
    # Leave out the HUD's bottom bar and top strip: the ground is the point.
    return im.crop((0, 34, 640, 316))


def lift(scene, opt):
    a = [np.asarray(crop(Image.open(f).convert("RGB"), scene), int) for f in frames(os.path.join(FILMS, scene, opt))]
    b = [np.asarray(crop(Image.open(f).convert("RGB"), scene), int) for f in frames(os.path.join(FILMS, scene, "0_0"))]
    d = np.array([(x - y).sum(2) for x, y in zip(a, b)])
    # The mean lift (sum of the three channels) and the share of pixels
    # lifted by more than 60 (20 a channel), over every frame.
    return d.mean(), (d > 60).mean() * 100, d.max()


def clip(scene, opt, letter, ms):
    # An H.264 clip at the game's own pace (a GIF of the same 25 frames
    # was 3 MB; this is a tenth of that).
    tmp = tempfile.mkdtemp()
    fs = frames(os.path.join(FILMS, scene, opt))
    for i, f in enumerate(fs):
        crop(Image.open(f).convert("RGB"), scene).save(os.path.join(tmp, f"{i:03}.png"))
    out = os.path.join(IMG, f"{scene}-{letter}.mp4")
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", f"{1000 / ms:g}", "-i", os.path.join(tmp, "%03d.png"),
                    "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "18", "-movflags", "+faststart", out], check=True)
    # A poster for a page that can't play it: the middle frame, or the
    # siren's call at its height (20.2 s).
    crop(Image.open(fs[3 if scene == "siren" else len(fs) // 2]).convert("RGB"), scene).save(os.path.join(IMG, f"{scene}-{letter}.jpg"), quality=88)
    shutil.rmtree(tmp)
    return out, len(fs)


SCENES = [("p4clip", 125), ("siren", 100)]
ONLY = sys.argv[2:] or [s for s, _ in SCENES]
for scene, ms in [s for s in SCENES if s[0] in ONLY]:
    for letter, opt in sorted(KEY.items()):
        m, share, mx = lift(scene, opt)
        out, n = clip(scene, opt, letter, ms)
        print(f"{scene} {letter} ({opt}): mean lift {m:.1f}, {share:.1f}% of pixels lifted over 60, max {mx}; {n} frames, {os.path.getsize(out) // 1024} KB")
