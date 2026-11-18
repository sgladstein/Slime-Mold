#!/usr/bin/env python3
"""The page's clips from film.sh's frames (Reports/bite-light-2026-11-18).

    make_clips.py OUT PAGE

Each near-view film holds two windows, 23 to 27 s and 41 to 45 s; each
window becomes an H.264 clip at 30 frames a second: the middle 960 x 600
of the 1280 x 800 picture at its own pixels, with a poster from its most
changed frame. Printed beside each pair: the soldiers standing and the
mean walkers touching a soldier (from the film's log), and the share of
the picture that changed between before and after (any channel more than
12 of 255 apart).
"""
import glob, os, re, subprocess, sys, tempfile
import numpy as np
from PIL import Image

out, page = sys.argv[1], sys.argv[2]
os.makedirs(page, exist_ok=True)
CROP = (160, 100, 1120, 700)
WINDOWS = {"wave": (23, 27.1), "wade": (41, 45.1)}


def frames(folder, lo, hi):
    fs = []
    for f in glob.glob(os.path.join(folder, "*-near.png")):
        m = re.search(r"-(\d+)(ms|s)-near\.png$", f)
        t = int(m.group(1)) / (1000 if m.group(2) == "ms" else 1)
        if lo <= t < hi:
            fs.append((t, f))
    return [f for _, f in sorted(fs)]


def log(path, lo, hi):
    rows, t = [], None
    for l in open(path):
        m = re.match(r"\s*t ([\d.]+) s: mould \d+, soldiers (\d+)", l)
        if m:
            t, n = float(m.group(1)), int(m.group(2))
        m = re.match(r"\s*squad: touching ([\d.]+) mean", l)
        if m and t is not None and lo <= t < hi:
            rows.append((n, float(m.group(1))))
    return rows[0][0], rows[-1][0], np.mean([r[1] for r in rows])


for z in (12, 6):
    for scene, (lo, hi) in WINDOWS.items():
        pair = {side: frames(os.path.join(out, f"fork{z}-{side}"), lo, hi) for side in ("before", "after")}
        changed = []
        for a, b in zip(pair["before"], pair["after"]):
            x = np.asarray(Image.open(a).convert("RGB").crop(CROP)).astype(int)
            y = np.asarray(Image.open(b).convert("RGB").crop(CROP)).astype(int)
            changed.append((np.abs(x - y).max(2) > 12).mean())
        most = int(np.argmax(changed))
        for side in ("before", "after"):
            name = f"{scene}{z}-{side}"
            with tempfile.TemporaryDirectory() as d:
                for i, f in enumerate(pair[side]):
                    Image.open(f).convert("RGB").crop(CROP).save(os.path.join(d, f"{i:04d}.png"))
                subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "30", "-i", os.path.join(d, "%04d.png"),
                                "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "26", "-preset", "slow",
                                "-movflags", "+faststart", os.path.join(page, f"{name}.mp4")], check=True)
            Image.open(pair[side][most]).convert("RGB").crop(CROP).save(os.path.join(page, f"{name}.jpg"), quality=86)
        first, last, touching = log(os.path.join(out, f"fork{z}-after.log"), lo, hi)
        print(f"{scene}{z}", len(pair["after"]), f"frames; standing {first} to {last}; touching {touching:.1f} a soldier;",
              f"changed {100 * np.mean(changed):.2f}% (at most {100 * changed[most]:.2f}%, poster {os.path.basename(pair['after'][most])})")
