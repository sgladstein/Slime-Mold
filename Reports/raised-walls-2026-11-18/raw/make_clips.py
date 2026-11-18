#!/usr/bin/env python3
"""The page's clips and pictures from film.sh's frames
(Reports/raised-walls-2026-11-18).

    make_clips.py OUT PAGE

Each wall film's near-view frames (30 a game second, 12 to 18 s) become an
H.264 clip at 30 frames a second, 800 x 500 (small on purpose: the repo's
clone is big). The rim's frames become a pair, before and after, of the
whole planet round the Lure.
"""
import glob, os, re, subprocess, sys, tempfile
from PIL import Image

out, page = sys.argv[1], sys.argv[2]
os.makedirs(page, exist_ok=True)


def frames(folder):
    fs = []
    for f in glob.glob(os.path.join(folder, "*-near.png")):
        m = re.search(r"-(\d+)(ms|s)-near\.png$", f)
        fs.append((int(m.group(1)) / (1000 if m.group(2) == "ms" else 1), f))
    return [f for _, f in sorted(fs)]


for name in ["z6-before", "z6-after", "z12-before", "z12-after"]:
    fs = frames(os.path.join(out, name))
    with tempfile.TemporaryDirectory() as d:
        for i, p in enumerate(fs):
            os.symlink(os.path.abspath(p), os.path.join(d, f"{i:04d}.png"))
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "30", "-i", os.path.join(d, "%04d.png"),
                        "-vf", "scale=800:500:flags=lanczos", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "28",
                        "-preset", "slow", "-movflags", "+faststart", os.path.join(page, f"walls-{name}.mp4")], check=True)
    Image.open(fs[-1]).convert("RGB").resize((800, 500), Image.LANCZOS).save(os.path.join(page, f"walls-{name}.jpg"), quality=85)
    print(name, len(fs), "frames", os.path.getsize(os.path.join(page, f"walls-{name}.mp4")) // 1024, "KB")

# The rim: planet 2's Lure at 40 s, the whole planet's left half.
for side in ["old", "new"]:
    (f,) = glob.glob(os.path.join(out, f"rim-p2-{side}", "*040s-all.png"))
    Image.open(f).convert("RGB").crop((110, 140, 640, 700)).save(os.path.join(page, f"rim-{side}.jpg"), quality=88)
