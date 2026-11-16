# Side-by-side clip of `pen` runs (larder-world look=DIR hand=pen ...):
#   python3 clip.py OUT_DIR T0 T1 STEP "LABEL=DIR" ["LABEL=DIR" ...]
# Each DIR holds the run's *-near.png frames and its log.txt ("pen t N: wall
# cells X, walkers inside Y, soldiers Z"). Writes OUT_DIR/f-NNN.png; then
#   ffmpeg -framerate 4 -i OUT_DIR/f-%03d.png -pix_fmt yuv420p -vf scale=trunc(iw/2)*2:-2 clip.mp4
import re, sys, glob, os
from PIL import Image, ImageDraw, ImageFont
out, t0, t1, step = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
runs = [a.split('=', 1) for a in sys.argv[5:]]
box = (220, 30, 740, 520)
font = ImageFont.load_default(size=17)
def counts(d):
    c, first = {}, None
    for line in open(d + '/log.txt'):
        m = re.match(r'\s+pen t (\d+): wall cells (\d+), walkers inside (\d+), soldiers (\d+)', line)
        if m:
            t, w, i, s = map(int, m.groups())
            first = first or w
            c[t] = (w, i, s, first)
    return c
cs = [counts(d) for _, d in runs]
os.makedirs(out, exist_ok=True)
n = 0
for t in range(t0, t1 + 1, step):
    ims = []
    for (lab, d), c in zip(runs, cs):
        f = glob.glob(f'{d}/*-pen-{t:03d}s-near.png')
        if not f:
            break
        ims.append((lab, Image.open(f[0]).convert('RGB').crop(box), c.get(t) or c.get(t - 1)))
    if len(ims) != len(runs):
        continue
    w, h = ims[0][1].size
    canvas = Image.new('RGB', (len(ims) * (w + 8) - 8, h + 78), (20, 22, 20))
    d = ImageDraw.Draw(canvas)
    for k, (lab, im, v) in enumerate(ims):
        x = k * (w + 8)
        canvas.paste(im, (x, 78))
        d.text((x + 6, 6), f'{lab}   {t // 60}:{t % 60:02d}', fill=(255, 255, 255), font=font)
        if v:
            wl, ins, s, first = v
            d.text((x + 6, 30), f'wall left {100 * wl / first:.0f}% ({wl} of {first} cells)', fill=(232, 212, 150), font=font)
            d.text((x + 6, 52), f'slime inside {ins}   soldiers {s}', fill=(232, 212, 150), font=font)
    canvas.save(f'{out}/f-{n:03d}.png')
    n += 1
print(n, 'frames')
