# make_gifs.py DIR OUTDIR: the gun range film (`larder-world look=DIR range=7
# menu=shop drag=...go at=...`) as two GIFs: the armory's frames (before the
# film's first whole second) cropped to the shop with the logged pointer
# drawn, and the round's frames after it.
import sys, glob, re
from PIL import Image, ImageDraw
d, out = sys.argv[1], sys.argv[2]
def t(f):
    m = re.search(r'-(\d+)(ms|s)-near', f); v = int(m.group(1)); return v if m.group(2) == 'ms' else v * 1000
fs = sorted(glob.glob(d + '/*-near.png'), key=t)
ptr = {}
for l in open(d + '/log.txt'):
    m = re.match(r'drag: frame (\d+) pointer (-?\d+),(-?\d+) (\w+)', l)
    if m: ptr[int(m.group(1))] = (int(m.group(2)), int(m.group(3)), m.group(4) == 'down')
def cursor(im, x, y, down):
    dr = ImageDraw.Draw(im); s = 22
    pts = [(x, y), (x, y + s), (x + s * 0.28, y + s * 0.75), (x + s * 0.5, y + s * 1.1), (x + s * 0.62, y + s * 1.04), (x + s * 0.42, y + s * 0.68), (x + s * 0.72, y + s * 0.68)]
    dr.polygon(pts, fill=(200, 230, 255) if down else (255, 255, 255), outline=(0, 0, 0))
def save(ims, path, ms, hold_first=0):
    # Each frame its own palette: one shared palette lost the cards' colour bands.
    q = [im.quantize(colors=255, method=Image.FASTOCTREE, dither=Image.NONE) for im in ims]
    durs = [ms] * len(q); durs[0] += hold_first; durs[-1] = 1800
    q[0].save(path, save_all=True, append_images=q[1:], duration=durs, loop=0, optimize=True)
    print(path, len(q), 'frames')
armory, fight = [], []
for k, f in enumerate(fs):
    im = Image.open(f).convert('RGB')
    if t(f) < 1000:
        if k in ptr: cursor(im, *ptr[k])
        im = im.crop((0, 110, 930, 900))
        armory.append(im.resize((int(im.width * 0.8), int(im.height * 0.8)), Image.LANCZOS))
    else:
        fight.append(im.resize((int(im.width * 0.45), int(im.height * 0.45)), Image.LANCZOS))
save(armory, out + '/build.gif', 67, hold_first=1000)
save(fight, out + '/fight.gif', 100)
