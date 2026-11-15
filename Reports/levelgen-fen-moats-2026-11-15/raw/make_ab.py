"""The blind A/B page's pictures: A = rock rims (frames/rock, renders-rock.sh), B = moats (frames/after, renders-v2.sh),
both with lakes 3 to 1, stills (1 s and 30 s at zoom 3, 30 s at zoom 5) and films of seeds 4 and 7 (a frame every 3 s
to 58 s; renders-rock-films.sh and renders-v2.sh); and the lakes change itself (fc-lakes/: main's map left, the rock twin's right).
Run from the folder holding frames/:  python3 make_ab.py FC_LAKES_DIR OUT_DIR"""
import sys, os, subprocess, tempfile
from PIL import Image, ImageDraw, ImageFont
fc, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
def load(tag, name, f):
    im = Image.open(f"frames/{tag}/{name}/{f}.png").convert("RGB")
    w, h = im.size
    im = im.crop((0, int(h * 0.05), w, int(h * 0.895)))  # the HUD's top and bottom bars
    im.thumbnail((1100, 1100), Image.LANCZOS)
    return im
for s in (1, 4, 7, 15):
    for key, tag in (("A", "rock"), ("B", "after")):
        load(tag, f"nest-3-{s}", f"planet-3-{s}-idle-001s-near").save(f"{out}/ab-3-{s}-z3-1s-{key}.webp", quality=84, method=6)
        load(tag, f"nest-3-{s}", f"planet-3-{s}-idle-030s-near").save(f"{out}/ab-3-{s}-z3-30s-{key}.webp", quality=84, method=6)
        load(tag, f"close-3-{s}", f"planet-3-{s}-idle-030s-near").save(f"{out}/ab-3-{s}-z5-30s-{key}.webp", quality=84, method=6)
for s in (1, 7):
    Image.open(f"{fc}/planet-3-{s}.png").convert("RGB").save(f"{out}/fc-lakes-3-{s}.webp", quality=90, method=6)
try:
    font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 26)
except OSError:
    font = ImageFont.load_default()
for s in (4, 7):
    for key, tag in (("A", "rock"), ("B", "after")):
        with tempfile.TemporaryDirectory() as td:
            for i, t in enumerate(range(1, 61, 3)):
                im = load(tag, f"film-3-{s}", f"planet-3-{s}-idle-{t:03d}s-near")
                d = ImageDraw.Draw(im)
                d.rectangle((8, 8, 120, 44), fill=(17, 22, 27)); d.text((14, 12), f"{t} s", fill=(232, 164, 58), font=font)
                im.save(f"{td}/{i:02d}.png")
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "4", "-i", f"{td}/%02d.png", "-vf", "scale=trunc(iw/2)*2:trunc(ih/2)*2",
                            "-c:v", "libx264", "-pix_fmt", "yuv420p", "-r", "12", "-crf", "24", "-movflags", "+faststart",
                            f"{out}/ab-3-{s}-film-{key}.mp4"], check=True)
