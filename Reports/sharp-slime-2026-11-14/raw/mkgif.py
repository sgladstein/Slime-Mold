# mkgif.py OUT DIR_BEFORE DIR_AFTER CROP(x0,y0,x1,y1) LABEL: a side-by-side
# GIF (before | after), frames every 0.1 game s from 2:30, 10 a second.
import sys, os, subprocess, tempfile
from PIL import Image, ImageDraw, ImageFont
out, a, b, crop, label = sys.argv[1], sys.argv[2], sys.argv[3], tuple(int(v) for v in sys.argv[4].split(',')), sys.argv[5]
font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 15)
tmp = tempfile.mkdtemp(dir='.')
n = 0
for i in range(31):
    t = 150 + i * 0.1; ms = int(round(t * 1000))
    name = f'fork-farm-{ms//1000:03d}s-near.png' if ms % 1000 == 0 else f'fork-farm-{ms}ms-near.png'
    ims = [Image.open(f'{d}/{name}').convert('RGB').crop(crop) for d in (a, b)]
    w, h = ims[0].size
    sc = 480 / w
    ims = [im.resize((480, int(h * sc)), Image.LANCZOS) for im in ims]
    canvas = Image.new('RGB', (964, ims[0].size[1]), (17, 26, 31))
    canvas.paste(ims[0], (0, 0)); canvas.paste(ims[1], (484, 0))
    d = ImageDraw.Draw(canvas)
    for x, tag in ((0, 'before'), (484, 'now')):
        d.rectangle([x, 0, x + 190, 22], fill=(17, 26, 31))
        d.text((x + 6, 3), f'{tag} · {label}', fill=(227, 234, 236), font=font)
    canvas.save(f'{tmp}/f{n:04d}.png'); n += 1
subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-framerate', '10', '-i', f'{tmp}/f%04d.png', '-vf', 'split[a][b];[a]palettegen=max_colors=160[p];[b][p]paletteuse=dither=bayer:bayer_scale=3', '-loop', '0', out], check=True)
subprocess.run(['rm', '-r', tmp])
print(out, n, os.path.getsize(out))
