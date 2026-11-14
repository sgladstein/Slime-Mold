"""Frames of films.sh to the page's GIFs and stills: make_page.py FRAMES OUT.
GIFs are cropped, not scaled, so they stay as crisp as the game."""
import glob, os, sys
from PIL import Image
frames, out = sys.argv[1], sys.argv[2]

def crop(im, w=720, h=450):
    x = (im.width - w) // 2
    y = max((im.height - 120 - h) // 2, 0)  # keep clear of the bottom bar
    return im.crop((x, y, x + w, y + h))

for name in ["meadow-12", "jungle-6"]:
    paths = sorted(glob.glob(os.path.join(frames, name, "*-near.png")))
    ims = [crop(Image.open(p).convert("RGB")) for p in paths]
    pal = [im.quantize(colors=255, method=Image.MEDIANCUT, dither=Image.NONE) for im in ims]
    dst = os.path.join(out, f"{name}.gif")
    pal[0].save(dst, save_all=True, append_images=pal[1:], duration=250, loop=0)
    print(name, len(ims), os.path.getsize(dst))

for name in ["z24-jungle", "z24-fire"]:
    p = glob.glob(os.path.join(frames, name, "*-near.png"))[0]
    im = crop(Image.open(p).convert("RGB"))
    g = im.convert("L").convert("RGB")
    both = Image.new("RGB", (im.width, im.height * 2))
    both.paste(im, (0, 0)); both.paste(g, (0, im.height))
    both.save(os.path.join(out, f"{name}.png"))
    print(name)

# Grey thumbnails, 266 x 164, of each planet's whole map, in one strip.
th = []
for p in range(1, 9):
    f = glob.glob(os.path.join(frames, f"thumb-{p}", "*-all.png"))
    if not f:
        continue
    im = Image.open(f[0]).convert("RGB")
    th.append((im.resize((266, 164), Image.LANCZOS), im.convert("L").resize((266, 164), Image.LANCZOS).convert("RGB")))
strip = Image.new("RGB", (266 * 4 + 30, (164 * 2 + 10) * 2 + 10), (20, 20, 20))
for n, (c, g) in enumerate(th):
    x = (n % 4) * 276; y = (n // 4) * 348
    strip.paste(c, (x, y)); strip.paste(g, (x, y + 174))
strip.save(os.path.join(out, "planets.png"))
print("planets", len(th))
