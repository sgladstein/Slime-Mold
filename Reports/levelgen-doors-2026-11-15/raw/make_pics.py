"""Pictures and films for the doors page, from frames/{before,after} (renders.sh, renders2.sh) and fc/ (fc.py).
Run from the folder holding frames/ and fc/:  python3 make_pics.py OUT_DIR"""
import sys, subprocess, tempfile, os
from PIL import Image, ImageDraw, ImageFont
out = sys.argv[1]
os.makedirs(out, exist_ok=True)
PLANETS = ["1-2", "2-3", "3-1", "4-2", "5-1", "6-3", "7-2", "8-2"]
def load(tag, name, f, crop=False):
    im = Image.open(f"frames/{tag}/{name}/{f}.png").convert("RGB")
    if crop:  # the HUD's top and bottom bars
        w, h = im.size
        im = im.crop((0, int(h * 0.05), w, int(h * 0.895)))
    return im
for p in PLANETS:
    for tag in ("before", "after"):
        # Close (zoom 5) at 30 s, the door (zoom 8) at 1 s, the wider view (zoom 3) at 30 s.
        for name, f, crop in ((f"close-{p}", f"planet-{p}-idle-030s-near", True), (f"door-{p}", f"planet-{p}-idle-001s-near", True), (f"nest-{p}", f"planet-{p}-idle-030s-near", True)):
            im = load(tag, name, f, crop)
            im.thumbnail((1100, 1100), Image.LANCZOS)
            im.save(f"{out}/{name}-{tag}.webp", quality=84, method=6)
for f in sorted(os.listdir("fc")):
    Image.open(f"fc/{f}").convert("RGB").save(f"{out}/fc-{f[:-4]}.webp", quality=90, method=6)
# Films, after only: the slime's first minute in its bowl, every 3 s, 4 frames a second of film.
try:
    font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 26)
except OSError:
    font = ImageFont.load_default()
for p in ("1-2", "7-2"):
    with tempfile.TemporaryDirectory() as td:
        for i, s in enumerate(range(1, 61, 3)):
            im = load("after", f"film-{p}", f"planet-{p}-idle-{s:03d}s-near", True)
            d = ImageDraw.Draw(im)
            d.rectangle((8, 8, 120, 44), fill=(17, 22, 27)); d.text((14, 12), f"{s} s", fill=(232, 164, 58), font=font)
            im = im.resize((1280, int(1280 * im.height / im.width) // 2 * 2), Image.LANCZOS)
            im.save(f"{td}/{i:02d}.png")
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "4", "-i", f"{td}/%02d.png", "-c:v", "libx264",
                        "-pix_fmt", "yuv420p", "-r", "12", "-crf", "24", "-movflags", "+faststart", f"{out}/film-{p}.mp4"], check=True)
