#!/usr/bin/env python3
"""The page's clips and stills from film.sh's frames
(Reports/fallen-soldier-2026-11-18).

    make_clips.py OUT PAGE

Each near-view film (30 frames a game second) becomes an H.264 clip at 30
frames a second: the middle 960 x 600 of the 1280 x 800 picture at its own
pixels. The before and after stills are the same crop. Beside each, from
the film's log: the soldiers standing at the clip's first and last second.
"""
import glob, os, re, subprocess, sys, tempfile
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
    s = []
    for l in open(log):
        m = re.match(r"\s*t ([\d.]+) s: mould \d+, soldiers (\d+)", l)
        if m:
            s.append((float(m.group(1)), int(m.group(2))))
    return s


for name in ["fork-z12", "fork-z6", "p2-z6", "p2-z12"]:
    fs = frames(os.path.join(out, name))
    if not fs:
        continue
    with tempfile.TemporaryDirectory() as d:
        for i, f in enumerate(fs):
            Image.open(f).convert("RGB").crop(CROP).save(os.path.join(d, f"{i:04d}.png"))
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "30", "-i", os.path.join(d, "%04d.png"),
                        "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "28", "-preset", "slow",
                        "-movflags", "+faststart", os.path.join(page, f"{name}.mp4")], check=True)
    Image.open(fs[0]).convert("RGB").crop(CROP).save(os.path.join(page, f"{name}.jpg"), quality=86)
    s = standing(os.path.join(out, f"{name}.log"))
    print(name, len(fs), "frames", os.path.getsize(os.path.join(page, f"{name}.mp4")) // 1024, "KB; standing",
          s[0] if s else None, "to", s[-1] if s else None)
for name in ["fork-before", "fork-after", "p2-before", "p2-after"]:
    fs = frames(os.path.join(out, name))
    if fs:
        Image.open(fs[-1]).convert("RGB").crop(CROP).save(os.path.join(page, f"{name}.jpg"), quality=88)
        print(name, standing(os.path.join(out, f"{name}.log"))[-1:])
