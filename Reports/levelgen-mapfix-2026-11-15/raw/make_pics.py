"""Pictures and films for the map-fix page, from frames/{before,after} (renders.sh) and fc/ (fc.py).
Run from the folder holding frames/ and fc/:  python3 make_pics.py OUT_DIR"""
import sys, subprocess, tempfile, os
from PIL import Image, ImageDraw, ImageFont
out = sys.argv[1]
# Crop the HUD's top and bottom bars off whole-map frames; keep close views whole.
def load(tag, name, f, crop):
    im = Image.open(f"frames/{tag}/{name}/{f}.png").convert("RGB")
    if crop:
        w, h = im.size
        im = im.crop((0, int(h * 0.05), w, int(h * 0.895)))
    return im
stills = {
    "core-1": ("planet-8-1-idle-060s-all", True), "core-2": ("planet-8-2-idle-060s-all", True),
    "jungle-1": ("planet-5-1-idle-001s-all", True), "valley-2": ("planet-4-2-idle-001s-all", True),
    "bridge-4-2": ("planet-4-2-idle-001s-near", False), "bridge-4-6": ("planet-4-6-idle-001s-near", False),
    "nest-8-2": ("planet-8-2-idle-001s-near", False), "nest-5-1": ("planet-5-1-idle-001s-near", False),
}
for name, (f, crop) in stills.items():
    for tag in ("before", "after"):
        im = load(tag, name, f, crop)
        im.thumbnail((1100, 1100), Image.LANCZOS)
        im.save(f"{out}/{name}-{tag}.webp", quality=84, method=6)
for f in sorted(os.listdir("fc")):
    Image.open(f"fc/{f}").convert("RGB").save(f"{out}/fc-{f[:-4]}.webp", quality=90, method=6)
# Films: before | after side by side, 1 s to 60 s every 10 s, one frame a second of film.
try:
    font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 26)
except OSError:
    font = ImageFont.load_default()
for name, planet in (("core-1", "8-1"), ("core-2", "8-2")):
    with tempfile.TemporaryDirectory() as td:
        for i, s in enumerate((1, 10, 20, 30, 40, 50, 60)):
            a, b = (load(t, name, f"planet-{planet}-idle-{s:03d}s-all", True) for t in ("before", "after"))
            w, h = a.size
            sheet = Image.new("RGB", (w * 2 + 12, h), (17, 22, 27))
            sheet.paste(a, (0, 0)); sheet.paste(b, (w + 12, 0))
            d = ImageDraw.Draw(sheet)
            for x, label in ((14, f"before  {s} s"), (w + 26, f"after  {s} s")):
                d.rectangle((x - 6, 8, x + 190, 44), fill=(17, 22, 27)); d.text((x, 12), label, fill=(232, 164, 58), font=font)
            sheet = sheet.resize((1600, int(1600 * sheet.height / sheet.width)) , Image.LANCZOS)
            sheet = sheet.crop((0, 0, 1600, sheet.height // 2 * 2))
            sheet.save(f"{td}/{i:02d}.png")
        subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "1", "-i", f"{td}/%02d.png", "-c:v", "libx264",
                        "-pix_fmt", "yuv420p", "-r", "10", "-crf", "24", "-movflags", "+faststart", f"{out}/{name}-film.mp4"], check=True)
