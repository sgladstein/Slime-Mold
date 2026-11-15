"""Pictures and films for the fen moats page, from frames/{before,after,after-v1} (renders.sh, renders-v2.sh) and fc2/ (fc.py).
Run from the folder holding frames/:  python3 make_pics.py FC_DIR OUT_DIR"""
import sys, subprocess, tempfile, os
from PIL import Image, ImageDraw, ImageFont
fc, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
def load(tag, name, f):
    im = Image.open(f"frames/{tag}/{name}/{f}.png").convert("RGB")
    w, h = im.size
    return im.crop((0, int(h * 0.05), w, int(h * 0.895)))  # the HUD's top and bottom bars
for s in (1, 4, 7, 15):
    for tag in ("before", "after"):
        for kind in ("nest", "close"):
            im = load(tag, f"{kind}-3-{s}", f"planet-3-{s}-idle-030s-near")
            im.thumbnail((1100, 1100), Image.LANCZOS)
            im.save(f"{out}/{kind}-3-{s}-{tag}.webp", quality=84, method=6)
    Image.open(f"{fc}/planet-3-{s}.png").convert("RGB").save(f"{out}/fc-3-{s}.webp", quality=90, method=6)
# The first try (deep water as the whole rim) at 1 s, and the second at 1 s.
for tag, name in (("after-v1", "try1"), ("after", "try2")):
    im = load(tag, "nest-3-4", "planet-3-4-idle-001s-near")
    im.thumbnail((1100, 1100), Image.LANCZOS)
    im.save(f"{out}/{name}-3-4-1s.webp", quality=84, method=6)
try:
    font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 26)
except OSError:
    font = ImageFont.load_default()
for s in (4, 7):
    with tempfile.TemporaryDirectory() as td:
        for i, t in enumerate(range(1, 61, 3)):
            im = load("after", f"film-3-{s}", f"planet-3-{s}-idle-{t:03d}s-near")
            d = ImageDraw.Draw(im)
            d.rectangle((8, 8, 120, 44), fill=(17, 22, 27)); d.text((14, 12), f"{t} s", fill=(232, 164, 58), font=font)
            im = im.resize((1280, int(1280 * im.height / im.width) // 2 * 2), Image.LANCZOS)
            im.save(f"{td}/{i:02d}.png")
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "4", "-i", f"{td}/%02d.png", "-c:v", "libx264",
                        "-pix_fmt", "yuv420p", "-r", "12", "-crf", "24", "-movflags", "+faststart", f"{out}/film-3-{s}.mp4"], check=True)
