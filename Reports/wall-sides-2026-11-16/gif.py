# Before/after clip of the `wallq` hand: python3 gif.py BEFORE_DIR BEFORE_TXT AFTER_DIR AFTER_TXT OUT_DIR
# writes OUT_DIR/f-NN.png; then ffmpeg -framerate 2.5 -i OUT_DIR/f-%02d.png -pix_fmt yuv420p -vf scale=1130:-2 wall-sides.mp4
import re, sys, glob
from PIL import Image, ImageDraw, ImageFont
bd, bt, ad, at, out = sys.argv[1:6]
def counts(path):
    c = {}
    for line in open(path):
        m = re.match(r'\s+wallq t (\d+): built (\d+)%, (\d+) of (\d+) past', line)
        if m: c[int(m.group(1))] = (int(m.group(2)), int(m.group(3)), int(m.group(4)))
    return c
def key(p):
    m = re.search(r'-(\d+)(ms|s)-near', p); v = int(m.group(1)); return v / 1000 if m.group(2) == 'ms' else v
fb = sorted(glob.glob(bd + '/*-near.png'), key=key); fa = sorted(glob.glob(ad + '/*-near.png'), key=key)
cb, ca = counts(bt), counts(at)
font = ImageFont.load_default(size=18)
box = (200, 120, 760, 500)
frames = []
for b, a in zip(fb, fa):
    t = key(b)
    ib = Image.open(b).convert('RGB').crop(box); ia = Image.open(a).convert('RGB').crop(box)
    w, h = ib.size
    c = Image.new('RGB', (w * 2 + 10, h + 60), (20, 20, 24))
    c.paste(ib, (0, 60)); c.paste(ia, (w + 10, 60))
    d = ImageDraw.Draw(c)
    def lab(cs):
        v = cs.get(int(t))
        return 'wall not placed yet' if v is None else f'wall {v[0]}%   {v[1]} of {v[2]} past the wall'
    d.text((8, 6), f'BEFORE (main)   t {t:.1f} s', fill=(255, 255, 255), font=font)
    d.text((8, 32), lab(cb), fill=(230, 210, 150), font=font)
    d.text((w + 18, 6), f'AFTER   t {t:.1f} s', fill=(255, 255, 255), font=font)
    d.text((w + 18, 32), lab(ca), fill=(230, 210, 150), font=font)
    frames.append(c)
import os
os.makedirs(out, exist_ok=True)
for i, f in enumerate(frames):
    f.save(f'{out}/f-{i:02d}.png')
print(len(frames), 'frames')
