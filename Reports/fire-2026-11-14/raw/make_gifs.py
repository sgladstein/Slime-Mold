"""Frames of a larder-world look= run to GIFs: make_gifs.py FRAMES OUT."""
import glob, os, sys
from PIL import Image
frames, out = sys.argv[1], sys.argv[2]
for view, size, last in [("all", (720, 450), 60), ("near", (720, 450), 45)]:
    paths = sorted(glob.glob(os.path.join(frames, f"*-{view}.png")))
    paths = [p for p in paths if int(p.rsplit("-", 2)[-2][:-1]) <= last]
    ims = [Image.open(p).convert("RGB").resize(size, Image.LANCZOS) for p in paths]
    pal = [im.quantize(colors=255, method=Image.MEDIANCUT, dither=Image.NONE) for im in ims]
    pal[0].save(os.path.join(out, f"fire-{view}.gif"), save_all=True, append_images=pal[1:], duration=250, loop=0)
    print(view, len(ims), os.path.getsize(os.path.join(out, f"fire-{view}.gif")))
