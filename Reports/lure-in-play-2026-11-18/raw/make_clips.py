#!/usr/bin/env python3
"""The page's clips, stills and counts from film.sh's frames
(Reports/lure-in-play-2026-11-18).

    make_clips.py OUT PAGE

Each planet's near-view frames (30 a game second; zoom 6 from 36 s, zoom
12 from 40 s) become an H.264 clip at 30 frames a second: the middle
960 x 600 of the 1280 x 800 picture at its own pixels, since the rings are
hairlines a scaled clip blurs. The whole planet at 40 s becomes a still.
Beside each, from the film's log: the walkers within 30 cells of the Lure
and from 30 to 60, at the clip's first and last second.
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


def watch(log, t):
    for l in open(log):
        m = re.match(r"\s*watch t (\d+) s: (\d+) walkers within 30 of [\d.,]+, (\d+) from 30 to 60", l)
        if m and int(m.group(1)) == t:
            return int(m.group(2)), int(m.group(3))
    return None


for p in [1, 2, 3]:
    for zoom, t0 in [(6, 36), (12, 40)]:
        name = f"p{p}-z{zoom}"
        fs = frames(os.path.join(out, name))
        with tempfile.TemporaryDirectory() as d:
            for i, f in enumerate(fs):
                Image.open(f).convert("RGB").crop(CROP).save(os.path.join(d, f"{i:04d}.png"))
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "30", "-i", os.path.join(d, "%04d.png"),
                            "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "30", "-preset", "slow",
                            "-movflags", "+faststart", os.path.join(page, f"{name}.mp4")], check=True)
        Image.open(fs[0]).convert("RGB").crop(CROP).save(os.path.join(page, f"{name}.jpg"), quality=86)
        log = os.path.join(out, f"{name}.log")
        print(name, len(fs), "frames", os.path.getsize(os.path.join(page, f"{name}.mp4")) // 1024, "KB;",
              f"near/far at {t0} s {watch(log, t0)}, at {t0 + 6} s {watch(log, t0 + 6)}")
    (f,) = glob.glob(os.path.join(out, f"p{p}-whole", "*040s-all.png"))
    Image.open(f).convert("RGB").save(os.path.join(page, f"p{p}-whole.jpg"), quality=86)
