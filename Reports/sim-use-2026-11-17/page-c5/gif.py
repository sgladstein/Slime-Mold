"""Builds flash.gif from gasprobe's stills (look=DIR, seed 1, gas_burn 0 and
0.2): python3 gif.py DIR. Two panels side by side, every still (1/3 s) for
the first 6 s, then every third."""
import sys
from PIL import Image
d = sys.argv[1]
qs = list(range(0, 18)) + list(range(18, 45, 3))
frames = []
for q in qs:
    ims = [Image.open(f"{d}/b{a}-{q:03}.png").convert("RGB") for a in ("0", "0.2")]
    w, h = ims[0].size
    c = Image.new("RGB", (2 * w + 8, h), (255, 255, 255))
    for j, im in enumerate(ims):
        c.paste(im, (j * (w + 8), 0))
    frames.append(c.quantize(colors=128, method=Image.MEDIANCUT))
frames[0].save("flash.gif", save_all=True, append_images=frames[1:], duration=[200] * 18 + [300] * (len(qs) - 18), loop=0, optimize=True)
