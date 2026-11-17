"""Builds charge.mp4 from wallprobe's stills (seeds=1-1 watch=30 look=DIR):
python3 film.py DIR FIRST SECOND. Three panels side by side, cropped to 320
pixels round the unit: no wall, then the arms named FIRST and SECOND
(straight, field), a still every third of a game second, played at 9 a
second (3 times real speed)."""
import glob, subprocess, sys, tempfile
from PIL import Image
d, arms = sys.argv[1], ["open"] + sys.argv[2:4]
n = min(len(glob.glob(f"{d}/{a}-*.png")) for a in arms)
with tempfile.TemporaryDirectory() as t:
    for q in range(n):
        ims = [Image.open(f"{d}/{a}-{q:03}.png").convert("RGB") for a in arms]
        w, h = ims[0].size
        box = (w // 2 - 160, h // 2 - 160, w // 2 + 160, h // 2 + 160)
        c = Image.new("RGB", (3 * 320 + 16, 320), (255, 255, 255))
        for j, im in enumerate(ims):
            c.paste(im.crop(box), (j * 328, 0))
        c.resize((c.width * 2, c.height * 2), Image.NEAREST).save(f"{t}/{q:03}.png")
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "9", "-i", f"{t}/%03d.png", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "20", "-movflags", "+faststart", "charge.mp4"], check=True)
