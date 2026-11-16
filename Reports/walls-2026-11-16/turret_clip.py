# Side-by-side clip of the `turretring` hand with and without turrets
# firing over walls (larder-world look=DIR hand=turretring mode=squads
# map=planet:1:7 set=turret_over=1|0 at=0,2,...,200):
#   python3 turret_clip.py OUT_DIR ON_DIR OFF_DIR
# Each DIR holds the run's *-turretring-NNNs-near.png frames and log.txt
# ("fort t N: shots S, turret kills K, soldier kills Q" and "pen t N: wall
# cells X, ..." each second). Writes OUT_DIR/f-NNN.png; then
#   ffmpeg -framerate 6 -i OUT_DIR/f-%03d.png -pix_fmt yuv420p -vf scale=trunc(iw/2)*2:-2 turrets.mp4
import re, sys, glob, os
from PIL import Image, ImageDraw, ImageFont
out, on, off = sys.argv[1], sys.argv[2], sys.argv[3]
box = (340, 90, 940, 660)
font = ImageFont.load_default(size=17)
big = ImageFont.load_default(size=20)
def counts(run):
    c, wl = {}, {}
    for line in open(run + '/log.txt'):
        m = re.match(r'\s+fort t (\d+): shots (\d+), turret kills (\d+), soldier kills (\d+)', line)
        if m:
            c[int(m.group(1))] = (int(m.group(3)), int(m.group(4)))
        m = re.match(r'\s+pen t (\d+): wall cells (\d+)', line)
        if m:
            wl[int(m.group(1))] = int(m.group(2))
    return c, wl
runs = [(on, 'Turrets fire over walls (new)', counts(on)), (off, 'Turrets stop at walls (before)', counts(off))]
os.makedirs(out, exist_ok=True)
n = 0
for t in range(0, 201, 2):
    ims = []
    for run, title, (c, wl) in runs:
        f = glob.glob(f'{run}/*-turretring-{t:03d}s-near.png')
        if not f:
            break
        im = Image.open(f[0]).convert('RGB').crop(box)
        w, h = im.size
        cv = Image.new('RGB', (w, h + 84), (20, 22, 20))
        cv.paste(im, (0, 84))
        d = ImageDraw.Draw(cv)
        d.text((8, 6), title, fill=(255, 255, 255), font=big)
        k = c.get(t) or c.get(t - 1)
        if k:
            d.text((8, 34), f'killed by turrets {k[0]}, by soldiers {k[1]}', fill=(232, 212, 150), font=font)
        if t in wl or t - 1 in wl:
            d.text((8, 58), f'ring wall cells {wl.get(t, wl.get(t - 1))} of 646', fill=(232, 212, 150), font=font)
        ims.append(cv)
    if len(ims) < 2:
        continue
    w, h = ims[0].size
    frame = Image.new('RGB', (2 * w + 12, h + 30), (8, 8, 8))
    frame.paste(ims[0], (0, 30))
    frame.paste(ims[1], (w + 12, 30))
    d = ImageDraw.Draw(frame)
    d.text((8, 5), f'{t // 60}:{t % 60:02d}   Same seed (planet 1, seed 7): a wall ring, three turrets and nine riflemen inside', fill=(255, 255, 255), font=font)
    frame.save(f'{out}/f-{n:03d}.png')
    n += 1
print(n, 'frames')
