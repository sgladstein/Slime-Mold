#!/usr/bin/env python3
"""The page's clips and the counts beside them, from film.sh's frames
(Reports/droplets-2026-11-18).

    make_clips.py OUT PAGE

Each film's near-view frames (30 a game second) become an H.264 clip at 30
frames a second: the middle 800 x 500 of the 1280 x 800 picture at its own
pixels, since a droplet is a few pixels across and a scaled-down clip
blurs it away. Then, for each pair, the share of that crop where any
channel differs by more than 12 of 255 between the two drawings of the
same frame (mean and most over the frames), and the game lines' shots and
kills, which must be equal. Last, a close pair of stills at twice the
pixels.
"""
import glob, os, re, subprocess, sys, tempfile
import numpy as np
from PIL import Image

out, page = sys.argv[1], sys.argv[2]
os.makedirs(page, exist_ok=True)
CROP = (240, 150, 1040, 650)


def frames(folder):
    fs = []
    for f in glob.glob(os.path.join(folder, "*-near.png")):
        m = re.search(r"-(\d+)(ms|s)-near\.png$", f)
        fs.append((int(m.group(1)) / (1000 if m.group(2) == "ms" else 1), f))
    return [f for _, f in sorted(fs)]


def last_line(log):
    lines = [l.strip() for l in open(log) if l.strip().startswith("t ") and "shots" in l]
    return lines[-1] if lines else "?"


for scene in ["range", "fork12", "fork6"]:
    pair = {}
    for side in ["before", "after"]:
        name = f"{scene}-{side}"
        fs = frames(os.path.join(out, name))
        pair[side] = fs
        with tempfile.TemporaryDirectory() as d:
            for i, p in enumerate(fs):
                Image.open(p).convert("RGB").crop(CROP).save(os.path.join(d, f"{i:04d}.png"))
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "30", "-i", os.path.join(d, "%04d.png"),
                            "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "24", "-preset", "slow",
                            "-movflags", "+faststart", os.path.join(page, f"{name}.mp4")], check=True)
        Image.open(fs[len(fs) // 2]).convert("RGB").crop(CROP).save(os.path.join(page, f"{name}.jpg"), quality=88)
        print(name, len(fs), "frames", os.path.getsize(os.path.join(page, f"{name}.mp4")) // 1024, "KB;",
              last_line(os.path.join(out, f"{name}.log")))
    shares = []
    for a, b in zip(pair["before"], pair["after"]):
        x = np.asarray(Image.open(a).convert("RGB").crop(CROP)).astype(int)
        y = np.asarray(Image.open(b).convert("RGB").crop(CROP)).astype(int)
        shares.append((np.abs(x - y).max(axis=2) > 12).mean())
    print(f"{scene}: changed {100 * np.mean(shares):.2f}% mean, {100 * max(shares):.2f}% most, "
          f"{sum(s > 0 for s in shares)} of {len(shares)} frames differ")

# A close look: the Fork at zoom 12 at 31 s round the hit, at twice its
# pixels (nearest), before and after, so the bead's lit and shaded sides
# and its shadow can be seen.
CLOSE = (640, 360, 940, 560)
for side in ["before", "after"]:
    (f,) = glob.glob(os.path.join(out, f"fork12-{side}", "*-031s-near.png"))
    im = Image.open(f).convert("RGB").crop(CLOSE)
    im.resize((im.width * 2, im.height * 2), Image.NEAREST).save(os.path.join(page, f"close-{side}.png"), optimize=True)
