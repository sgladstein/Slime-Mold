#!/usr/bin/env python3
"""The page's clips from film.sh's frames (Reports/squad-heads-2026-11-18).

    make_clips.py OUT PAGE

Each near-view film (30 frames a game second) becomes an H.264 clip at 30
frames a second: the middle 960 x 600 of the 1280 x 800 picture at its own
pixels, with its first frame as the poster. Printed beside each pair: the
soldiers standing (from the film's log) and the share of the picture that
changed between before and after (any channel more than 12 of 255 apart).
"""
import glob, os, re, subprocess, sys, tempfile
import numpy as np
from PIL import Image

out, page = sys.argv[1], sys.argv[2]
os.makedirs(page, exist_ok=True)
CROP = (160, 100, 1120, 700)


def frames(folder):
    fs = []
    for f in glob.glob(os.path.join(folder, "*-near.png")):
        m = re.search(r"-(\d+)(ms|s)-near\.png$", f)
        fs.append((int(m.group(1)) / (1000 if m.group(2) == "ms" else 1), f))
    return [f for _, f in sorted(fs)]


def standing(log):
    s = [int(m.group(1)) for l in open(log) for m in [re.match(r"\s*t [\d.]+ s: mould \d+, soldiers (\d+)", l)] if m]
    return (s[0], s[-1]) if s else None


for scene in ["drain6", "drain12", "walk6", "walk12"]:
    changed = []
    pair = {}
    for side in ["before", "after"]:
        name = f"{scene}-{side}"
        pair[side] = frames(os.path.join(out, name))
        with tempfile.TemporaryDirectory() as d:
            for i, f in enumerate(pair[side]):
                Image.open(f).convert("RGB").crop(CROP).save(os.path.join(d, f"{i:04d}.png"))
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "30", "-i", os.path.join(d, "%04d.png"),
                            "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "28", "-preset", "slow",
                            "-movflags", "+faststart", os.path.join(page, f"{name}.mp4")], check=True)
        Image.open(pair[side][0]).convert("RGB").crop(CROP).save(os.path.join(page, f"{name}.jpg"), quality=86)
    for a, b in zip(pair["before"], pair["after"]):
        x = np.asarray(Image.open(a).convert("RGB").crop(CROP)).astype(int)
        y = np.asarray(Image.open(b).convert("RGB").crop(CROP)).astype(int)
        changed.append((np.abs(x - y).max(2) > 12).mean())
    print(scene, len(pair["after"]), "frames; standing", standing(os.path.join(out, f"{scene}-after.log")),
          f"changed {100 * np.mean(changed):.2f}% (at most {100 * max(changed):.2f}%)")
