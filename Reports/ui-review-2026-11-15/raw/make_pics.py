"""Pictures and clips for the UI review page (2026-11-15).

Run from the scratch folder holding `before/`, `after2/` (stills) and
`clips/` (frames), as made by the commands in ../README.md; writes into
../page/.
"""
import glob, os, sys
from PIL import Image

out = sys.argv[1]
def near(d):
    return sorted(glob.glob(d + '/*-near.png'))

def still(src, name, crop=None, width=1600):
    im = Image.open(src).convert('RGB')
    if crop:
        w, h = im.size
        im = im.crop((0, int(h * crop), w, h))
    if im.size[0] > width:
        im = im.resize((width, round(im.size[1] * width / im.size[0])), Image.LANCZOS)
    im.save(os.path.join(out, name + '.webp'), quality=88)

for v, d in [('before', 'before'), ('after', 'after2')]:
    for page in ['main', 'campaign', 'options', 'keys', 'shopmid', 'shopbuild']:
        f = near(f'{d}/{page}')
        if f:
            still(f[0], f'{page}-{v}')
    for page in ['runhud', 'hudrts']:
        f = near(f'{d}/{page}')
        if f:
            still(f[0], f'{page}-{v}')
            still(f[0], f'{page}-bar-{v}', crop=0.82, width=1920)
for page in ['hud150', 'hud200', 'small', 'smallshop', 'nosel']:
    f = near(f'after2/{page}')
    if f:
        still(f[0], f'{page}-after', crop=0.80 if page.startswith('hud') or page == 'nosel' else None, width=1920)

def clip(d, name, crop=None, width=1280, ms=120):
    frames = []
    for f in near(d):
        im = Image.open(f).convert('RGB')
        if crop:
            w, h = im.size
            im = im.crop((0, int(h * crop), w, h))
        if im.size[0] > width:
            im = im.resize((width, round(im.size[1] * width / im.size[0])), Image.LANCZOS)
        frames.append(im)
    frames[0].save(os.path.join(out, name + '.webp'), save_all=True, append_images=frames[1:], duration=ms, loop=0, quality=80)

for v in ['before', 'after']:
    clip(f'clips/shop-{v}', f'clip-shop-{v}', width=1440, ms=160)
    clip(f'clips/bar-{v}', f'clip-bar-{v}', crop=0.80, width=1920)
