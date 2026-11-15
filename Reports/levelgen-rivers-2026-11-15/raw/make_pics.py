"""The rivers page's pictures, from frames/{before,after} (renders.sh) and the planets' maps (before: main's survey,
after: this change's): the whole river valley at 1 s, films of the slime's first minute from the second nest, and the
layouts in false colour (fc.py from the fen report: before left, after right).
Run from the folder holding frames/:  python3 make_pics.py OUT_DIR BEFORE_MAPS AFTER_MAPS"""
import sys, os, subprocess, tempfile
from PIL import Image, ImageDraw, ImageFont
out, before_maps, after_maps = sys.argv[1:4]
os.makedirs(out, exist_ok=True)
def load(tag, name, f):
    im = Image.open(f"frames/{tag}/{name}/{f}.png").convert("RGB")
    w, h = im.size
    im = im.crop((0, int(h * 0.05), w, int(h * 0.895)))  # the HUD's top and bottom bars
    im.thumbnail((1100, 1100), Image.LANCZOS)
    return im
for s in (1, 5):
    for tag in ("before", "after"):
        load(tag, f"whole-4-{s}", f"planet-4-{s}-idle-001s-near").save(f"{out}/whole-4-{s}-{tag}.webp", quality=84, method=6)
try:
    font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 26)
except OSError:
    font = ImageFont.load_default()
for d, s in ((4, 1), (4, 5)):  # the core world's film (8, 11) went unused
    for tag in ("before", "after"):
        with tempfile.TemporaryDirectory() as td:
            for i, t in enumerate(range(1, 61, 3)):
                im = load(tag, f"film-{d}-{s}", f"planet-{d}-{s}-idle-{t:03d}s-near")
                dr = ImageDraw.Draw(im)
                dr.rectangle((8, 8, 120, 44), fill=(17, 22, 27)); dr.text((14, 12), f"{t} s", fill=(232, 164, 58), font=font)
                im.save(f"{td}/{i:02d}.png")
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "4", "-i", f"{td}/%02d.png", "-vf", "scale=trunc(iw/2)*2:trunc(ih/2)*2",
                            "-c:v", "libx264", "-pix_fmt", "yuv420p", "-r", "12", "-crf", "24", "-movflags", "+faststart",
                            f"{out}/film-{d}-{s}-{tag}.mp4"], check=True)
            # the last frame as a still, for the page's poster
            Image.open(f"{td}/19.png").save(f"{out}/film-{d}-{s}-{tag}.webp", quality=84, method=6)
fc = "/home/user/Slime-Mold/Reports/levelgen-fen-moats-2026-11-15/raw/fc.py"
with tempfile.TemporaryDirectory() as td:
    subprocess.run(["python3", "-I", fc, before_maps, after_maps, td] + [f"planet-4-{s}" for s in (1, 5)] + ["planet-8-11"], check=True)
    for n in ("planet-4-1", "planet-4-5", "planet-8-11"):
        Image.open(f"{td}/{n}.png").convert("RGB").save(f"{out}/fc-{n[7:]}.webp", quality=90, method=6)
