"""Clips (H.264) for the command card page, from `larder-world look=DIR` frames.

python3 make_gifs.py R   (R holds guard/, bsel/, fire/ and before/, commands in the README)
"""
import glob
import os
import subprocess
import sys
import tempfile

from PIL import Image

R = sys.argv[1]
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "page")


def frames(folder):
    return sorted(glob.glob(os.path.join(R, folder, "*-near.png")), key=lambda p: os.path.basename(p).replace("s-near", "000ms-near") if "ms-" not in p else os.path.basename(p))


def key(p):
    n = os.path.basename(p).split("-")[2]
    return float(n[:-2]) / 1000 if n.endswith("ms") else float(n[:-1])


def gif(name, folder, crop, scale, ms):
    fs = sorted(glob.glob(os.path.join(R, folder, "*-near.png")), key=key)
    ims = []
    for f in fs:
        im = Image.open(f).convert("RGB")
        if crop:
            im = im.crop(crop)
        if scale != 1:
            im = im.resize((round(im.width * scale), round(im.height * scale)), Image.LANCZOS)
        ims.append(im)
    tmp = tempfile.mkdtemp()
    for i, im in enumerate(ims):
        im.crop((0, 0, im.width // 2 * 2, im.height // 2 * 2)).save(os.path.join(tmp, f"{i:03d}.png"))
    fps = 1000 / ms
    subprocess.run(["ffmpeg", "-loglevel", "error", "-y", "-framerate", f"{fps}", "-i", os.path.join(tmp, "%03d.png"), "-r", "30", "-pix_fmt", "yuv420p", "-c:v", "libx264", "-crf", "20", "-movflags", "+faststart", os.path.join(OUT, name)], check=True)
    print(name, len(ims), ims[0].size)



gif("building.mp4", "bsel", (0, 300, 1920, 1080), 0.5, 1100)
gif("card.mp4", "fire", (900, 600, 1920, 1080), 0.75, 1100)
for f, out in [("main/fork-squads-012s-near.png", "card-after.webp"), (None, None)]:
    if f:
        im = Image.open(os.path.join(R, f)).convert("RGB").crop((0, 880, 1920, 1080))
        im.save(os.path.join(OUT, out), quality=90)
b = glob.glob(os.path.join(R, "before", "*-near.png"))
if b:
    Image.open(b[0]).convert("RGB").crop((0, 880, 1920, 1080)).save(os.path.join(OUT, "card-before.webp"), quality=90)

# Seed 3 side by side: a plain move (left, GUARD_MOVE=1) and Guard (right)
# to the same crystal, frames s3move/ and s3guard/.
if os.path.isdir(os.path.join(R, "s3guard")):
    a = sorted(glob.glob(os.path.join(R, "s3move", "*-near.png")), key=key)
    b = sorted(glob.glob(os.path.join(R, "s3guard", "*-near.png")), key=key)
    tmp = tempfile.mkdtemp()
    for i, (x, y) in enumerate(zip(a, b)):
        ia = Image.open(x).convert("RGB").crop((240, 60, 1040, 620))
        ib = Image.open(y).convert("RGB").crop((240, 60, 1040, 620))
        im = Image.new("RGB", (1608, 560), (20, 24, 30))
        im.paste(ia, (0, 0))
        im.paste(ib, (808, 0))
        im.save(os.path.join(tmp, f"{i:03d}.png"))
    subprocess.run(["ffmpeg", "-loglevel", "error", "-y", "-framerate", "4", "-i", os.path.join(tmp, "%03d.png"), "-r", "30", "-pix_fmt", "yuv420p", "-c:v", "libx264", "-crf", "27", "-movflags", "+faststart", os.path.join(OUT, "guard-vs-move.mp4")], check=True)
    print("guard-vs-move.mp4", len(a))
