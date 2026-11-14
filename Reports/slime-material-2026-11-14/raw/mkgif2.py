# mkgif2.py OUT DIR_BEFORE DIR_AFTER CROP T0 N LABEL_A LABEL_B [grey]: a side-by-side
# GIF, frames every 0.1 game s from T0, 10 a second.
import sys, os, subprocess, tempfile
from PIL import Image, ImageDraw, ImageFont, ImageOps
out, a, b = sys.argv[1:4]
crop = tuple(int(v) for v in sys.argv[4].split(','))
t0, nf, la, lb = float(sys.argv[5]), int(sys.argv[6]), sys.argv[7], sys.argv[8]
grey = len(sys.argv) > 9
font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 15)
tmp = tempfile.mkdtemp(dir='.')
for i in range(nf):
    ms = int(round((t0 + i * 0.1) * 1000))
    name = f'fork-farm-{ms//1000:03d}s-near.png' if ms % 1000 == 0 else f'fork-farm-{ms}ms-near.png'
    ims = [Image.open(f'{d}/{name}').convert('RGB').crop(crop) for d in (a, b)]
    if grey: ims = [ImageOps.grayscale(im).convert('RGB') for im in ims]
    w, h = ims[0].size; sc = 480 / w
    ims = [im.resize((480, int(h * sc)), Image.LANCZOS) for im in ims]
    cv = Image.new('RGB', (964, ims[0].size[1]), (17, 26, 31))
    cv.paste(ims[0], (0, 0)); cv.paste(ims[1], (484, 0))
    d = ImageDraw.Draw(cv)
    s = ms / 1000; tl = f'{int(s//60)}:{s%60:04.1f}'
    for x, tag in ((0, la), (484, lb)):
        d.rectangle([x, 0, x + 230, 22], fill=(17, 26, 31))
        d.text((x + 6, 3), f'{tag} · {tl}', fill=(227, 234, 236), font=font)
    cv.save(f'{tmp}/f{i:04d}.png')
subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-framerate', '10', '-i', f'{tmp}/f%04d.png', '-vf', 'split[a][b];[a]palettegen=max_colors=160[p];[b][p]paletteuse=dither=bayer:bayer_scale=3', '-loop', '0', out], check=True)
subprocess.run(['rm', '-r', tmp])
print(out, nf, os.path.getsize(out))
