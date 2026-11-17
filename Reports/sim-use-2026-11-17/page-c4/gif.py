"""Builds fire.gif from fearprobe's stills (look=DIR, seed 1, arms 0, 50, 200):
python3 gif.py DIR. Three panels side by side, every second still (2/3 s),
scaled to 3/4."""
import sys
from PIL import Image
d = sys.argv[1]
frames = []
for q in range(0, 90, 2):
    ims = [Image.open(f"{d}/a{a}-{q:03}.png").convert("RGB") for a in (0, 50, 200)]
    w, h = ims[0].size
    c = Image.new("RGB", (3 * w + 12, h), (255, 255, 255))
    for j, im in enumerate(ims):
        c.paste(im, (j * (w + 6), 0))
    c = c.resize((c.width * 3 // 4, c.height * 3 // 4), Image.LANCZOS)
    frames.append(c.quantize(colors=128, method=Image.MEDIANCUT))
frames[0].save("fire.gif", save_all=True, append_images=frames[1:], duration=133, loop=0, optimize=True)
