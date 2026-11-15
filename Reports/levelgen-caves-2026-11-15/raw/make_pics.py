"""The cave page's pictures, from frames/{before,after} (renders.sh, renders-films.sh) and the planets' maps (main/
and v3/, by `examples/planets out=DIR`): stills before and after, films of the slime's first minute from its first
nest on seeds 8 and 2, and the layouts in false colour (fc.py from the fen report: before left, after right).
Run from the folder holding frames/:  python3 make_pics.py OUT_DIR"""
import sys, os, subprocess, tempfile
from PIL import Image, ImageDraw, ImageFont
out = sys.argv[1]
os.makedirs(out, exist_ok=True)
def load(tag, name, f):
    im = Image.open(f"frames/{tag}/{name}/{f}.png").convert("RGB")
    w, h = im.size
    im = im.crop((0, int(h * 0.05), w, int(h * 0.895)))  # the HUD's top and bottom bars
    im.thumbnail((1100, 1100), Image.LANCZOS)
    return im
for s in (2, 8, 13):
    for tag in ("before", "after"):
        load(tag, f"land-{s}", f"planet-7-{s}-idle-001s-near").save(f"{out}/land-{s}-1s-{tag}.webp", quality=84, method=6)
        load(tag, f"mid-{s}", f"planet-7-{s}-idle-030s-near").save(f"{out}/mid-{s}-{tag}.webp", quality=84, method=6)
        load(tag, f"room-{s}", f"planet-7-{s}-idle-030s-near").save(f"{out}/room-{s}-{tag}.webp", quality=84, method=6)
        load(tag, f"far-{s}", f"planet-7-{s}-idle-030s-near").save(f"{out}/far-{s}-{tag}.webp", quality=84, method=6)
try:
    font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 26)
except OSError:
    font = ImageFont.load_default()
for s, tag in [(s, tag) for s in (8, 2) for tag in ("before", "after")]:
    with tempfile.TemporaryDirectory() as td:
        for i, t in enumerate(range(1, 61, 3)):
            im = load(tag, f"nestfilm-{s}", f"planet-7-{s}-idle-{t:03d}s-near")
            d = ImageDraw.Draw(im)
            d.rectangle((8, 8, 120, 44), fill=(17, 22, 27)); d.text((14, 12), f"{t} s", fill=(232, 164, 58), font=font)
            im.save(f"{td}/{i:02d}.png")
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "4", "-i", f"{td}/%02d.png", "-vf", "scale=trunc(iw/2)*2:trunc(ih/2)*2",
                        "-c:v", "libx264", "-pix_fmt", "yuv420p", "-r", "12", "-crf", "24", "-movflags", "+faststart",
                        f"{out}/nestfilm-{s}-{tag}.mp4"], check=True)
fc = "/home/user/Slime-Mold/Reports/levelgen-fen-moats-2026-11-15/raw/fc.py"
with tempfile.TemporaryDirectory() as td:
    subprocess.run(["python3", "-I", fc, "main", "v3", td] + [f"planet-7-{s}" for s in (2, 8, 13)], check=True)
    for s in (2, 8, 13):
        Image.open(f"{td}/planet-7-{s}.png").convert("RGB").save(f"{out}/fc-7-{s}.webp", quality=90, method=6)
