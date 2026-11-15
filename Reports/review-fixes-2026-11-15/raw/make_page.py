"""films.sh frames to the page: make_page.py FRAMES OUT. Cropped, not scaled."""
import glob, os, sys
from PIL import Image
frames, out = sys.argv[1], sys.argv[2]
def crop(im, w=720, h=450):
    x = (im.width - w) // 2
    y = max((im.height - 120 - h) // 2, 0)
    return im.crop((x, y, x + w, y + h))
paths = sorted(glob.glob(os.path.join(frames, "fire-12", "*-near.png")))
ims = [crop(Image.open(p).convert("RGB")) for p in paths]
pal = [im.quantize(colors=255, method=Image.MEDIANCUT, dither=Image.NONE) for im in ims]
pal[0].save(os.path.join(out, "fire-12.gif"), save_all=True, append_images=pal[1:], duration=250, loop=0)
# Grey sheet: the fire at 20 s and the slime's edge, colour over grey.
sheet = []
for name, pat in [("fire", "fire-12/*-020s-near.png"), ("edge", "edge-12/*-near.png")]:
    im = crop(Image.open(glob.glob(os.path.join(frames, pat))[0]).convert("RGB"))
    sheet.append(im)
grey = Image.new("RGB", (720 * 2 + 10, 450 * 2 + 10), (20, 20, 20))
for n, im in enumerate(sheet):
    grey.paste(im, (n * 730, 0)); grey.paste(im.convert("L").convert("RGB"), (n * 730, 460))
grey.save(os.path.join(out, "sheet.png"))
print(len(ims), os.path.getsize(os.path.join(out, "fire-12.gif")))

# Flora sheet: the eight planets at zoom 12, colour over grey, four a row.
names = ["1 meadow", "2 barrens", "3 fen", "4 river valley", "5 jungle", "6 highlands", "7 caves", "8 core"]
tiles = []
for p in range(1, 9):
    f = glob.glob(os.path.join(frames, f"flora-{p}", "*-near.png"))
    tiles.append(Image.open(f[0]).convert("RGB").crop((240, 60, 720, 360)) if f else Image.new("RGB", (480, 300)))
sheet = Image.new("RGB", (4 * 490, 4 * 310), (20, 20, 20))
for n, im in enumerate(tiles):
    x = (n % 4) * 490; y = (n // 4) * 620
    sheet.paste(im, (x, y)); sheet.paste(im.convert("L").convert("RGB"), (x, y + 310))
sheet.save(os.path.join(out, "flora.png"))
print("flora", len(tiles))
