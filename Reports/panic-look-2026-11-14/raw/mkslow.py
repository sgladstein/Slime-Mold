# mkslow.py SRC OUTDIR: the slow-motion clip (frames every 0.1 game s from
# 2:34 to 2:39, crop 400,170,920,630, 5 frames a second: half speed) as a
# GIF and an MP4, from `larder-world look=SRC ... at=154.0,154.1,...,159.0`.
import os, sys, subprocess, tempfile
from PIL import Image, ImageDraw, ImageFont
src, out = sys.argv[1], sys.argv[2]
font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 18)
tmp = tempfile.mkdtemp(dir='.')
n = 0
for i in range(51):
    t = 154 + i * 0.1; ms = int(round(t * 1000))
    f = f'{src}/fork-farm-{ms//1000:03d}s-near.png' if ms % 1000 == 0 else f'{src}/fork-farm-{ms}ms-near.png'
    im = Image.open(f).convert('RGB').crop((400, 170, 920, 630))
    d = ImageDraw.Draw(im); d.rectangle([0, 0, 250, 28], fill=(17, 26, 31))
    d.text((8, 4), f'2:{int(t)-120:02d}.{int(round((t % 1) * 10)) % 10}   cut +{t-150:.1f} s', fill=(227, 234, 236), font=font)
    im.save(f'{tmp}/f{n:04d}.png'); n += 1
subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-framerate', '5', '-i', f'{tmp}/f%04d.png', '-vf', 'split[a][b];[a]palettegen=max_colors=128[p];[b][p]paletteuse=dither=bayer:bayer_scale=4', '-loop', '0', f'{out}/slow.gif'], check=True)
subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-framerate', '5', '-i', f'{tmp}/f%04d.png', '-c:v', 'libx264', '-pix_fmt', 'yuv420p', '-crf', '20', '-movflags', '+faststart', f'{out}/slow.mp4'], check=True)
subprocess.run(['rm', '-r', tmp])
