"""Builds reach.mp4 from farprobe's stills (seeds=1 depths=3 secs=150
smells=0,1 look=DIR): python3 film.py DIR FIRST SECOND, FIRST and SECOND
the arms' names (far0, far1). Two panels side by side, a still a game
second, played at 10 a second (10 times real speed)."""
import glob, subprocess, sys, tempfile
from PIL import Image
d, arms = sys.argv[1], sys.argv[2:4]
n = min(len(glob.glob(f"{d}/{a}-*.png")) for a in arms)
with tempfile.TemporaryDirectory() as t:
    for q in range(n):
        ims = [Image.open(f"{d}/{a}-{q:03}.png").convert("RGB") for a in arms]
        w, h = ims[0].size
        c = Image.new("RGB", (2 * w + 8, h), (255, 255, 255))
        for j, im in enumerate(ims):
            c.paste(im, (j * (w + 8), 0))
        c.resize((c.width * 2, c.height * 2), Image.NEAREST).save(f"{t}/{q:03}.png")
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", "10", "-i", f"{t}/%03d.png", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "20", "-movflags", "+faststart", "reach.mp4"], check=True)
