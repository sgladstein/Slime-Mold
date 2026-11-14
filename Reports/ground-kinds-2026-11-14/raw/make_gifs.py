#!/usr/bin/env python3
"""The River page's GIFs from larder-world's headless frames.
Films (seed 1, shape C defaults):
  larder-world map=river seed=1 look=FILM/idle hand=idle at=5,10,...,150 screen=1280x800
  larder-world map=river seed=1 look=FILM/rush hand=rush at=2,4,...,60 screen=1280x800
  python3 make_gifs.py FILM PAGE"""
import sys
from pathlib import Path
from PIL import Image

film, page = Path(sys.argv[1]), Path(sys.argv[2])
CROP = (111, 47, 1172, 701)  # the map inside the 1280 x 800 window, zoom 1

def gif(frames, out, crop, size, ms):
    ims = [Image.open(f).convert("RGB") for f in frames]
    if crop:
        ims = [im.crop(crop) for im in ims]
    ims = [im.resize(size, Image.LANCZOS) for im in ims]
    pal = [im.quantize(colors=128, method=Image.MEDIANCUT, dither=Image.NONE) for im in ims]
    pal[0].save(out, save_all=True, append_images=pal[1:], duration=[ms] * (len(pal) - 1) + [2000], loop=0, optimize=True)

gif(sorted((film / "idle").glob("*-all.png")), page / "river-idle.gif", CROP, (636, 392), 350)
gif(sorted((film / "rush").glob("*-near.png"))[:8], page / "river-rush-near.gif", (0, 40, 1280, 712), (640, 336), 500)
Image.open(sorted((film / "rush").glob("*-all.png"))[2]).crop(CROP).save(page / "river-rush-006s.png")
