"""Stills and clips for the UI-size page (2026-11-15).

Run with the scratch folder that holds `sc/` (menus before, main's build),
`gun/r-2560x1440-200` (bar before), `fin/` (after) and `clip/` (scrolling
films), as made by the commands in ../README.md; writes into ../page/.
"""
import glob, os, sys
from PIL import Image

src, out = sys.argv[1], sys.argv[2]

def near(d):
    f = sorted(glob.glob(os.path.join(src, d, '*-near.png')))
    if not f:
        raise SystemExit('no frames in ' + d)
    return f

def still(d, name, crop=None, width=1600):
    im = Image.open(near(d)[0]).convert('RGB')
    if crop:
        w, h = im.size
        im = im.crop((0, int(h * crop), w, h))
    if im.size[0] > width:
        im = im.resize((width, round(im.size[1] * width / im.size[0])), Image.LANCZOS)
    im.save(os.path.join(out, name + '.webp'), quality=86)

for page in ['main', 'options', 'keys', 'shop', 'range']:
    still(f'sc/{page}-200', f'{page}-200-before')
    still(f'fin/{page}-1920x1080-200', f'{page}-200-after')
    still(f'fin/{page}-1920x1080-100', f'{page}-100-after')
    still(f'fin/{page}-2560x1440-200', f'{page}-1440-200-after')
still('gun/r-2560x1440-200', 'bar-1440-200-before', crop=0.70, width=1920)
for k in ['2560x1440-200', '1920x1080-200', '1920x1080-150', '1920x1080-100']:
    still(f'fin/hud-{k}', f'bar-{k}-after', crop=0.80 if k.startswith('1920') else 0.70, width=1920)

def clip(d, name, width=1280, ms=220):
    frames = []
    for f in near(d):
        im = Image.open(f).convert('RGB')
        if im.size[0] > width:
            im = im.resize((width, round(im.size[1] * width / im.size[0])), Image.LANCZOS)
        frames.append(im)
    frames += [frames[-1]] * 4
    frames[0].save(os.path.join(out, name + '.webp'), save_all=True, append_images=frames[1:], duration=ms, loop=0, quality=80)

for c in ['shop200', 'range100', 'options200']:
    clip(f'clip/{c}', f'clip-{c}')
