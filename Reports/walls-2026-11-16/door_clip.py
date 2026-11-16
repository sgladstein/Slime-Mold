# Clip of the `door` hand (larder-world look=DIR hand=door ...):
#   python3 door_clip.py OUT_DIR T0 T1 STEP RUN_DIR
# RUN_DIR holds the run's *-door-NNNs-near.png frames and its log.txt
# ("pen t N: wall cells X, walkers inside Y, soldiers Z, outside K" each
# second and "door t ..." events). Writes OUT_DIR/f-NNN.png; then
#   ffmpeg -framerate 4 -i OUT_DIR/f-%03d.png -pix_fmt yuv420p -vf scale=trunc(iw/2)*2:-2 door.mp4
import re, sys, glob, os
from PIL import Image, ImageDraw, ImageFont
out, t0, t1, step, run = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
box = (190, 40, 770, 515)
font = ImageFont.load_default(size=17)
counts, events = {}, []
for line in open(run + '/log.txt'):
    m = re.match(r'\s+pen t (\d+): wall cells (\d+), walkers inside (\d+), soldiers (\d+), outside (\d+)', line)
    if m:
        counts[int(m.group(1))] = tuple(map(int, m.groups()[1:]))
    m = re.match(r'\s+door t ([\d.]+): (.*)', line)
    if m:
        events.append((float(m.group(1)), m.group(2)))
door_t = next((t for t, e in events if 'door cut' in e), None)
thick_t = next((t for t, e in events if 'thickened' in e), None)
def caption(t):
    if door_t is None or t < door_t:
        return 'A ring of wall stands round the squad'
    if t < 18:
        return 'A door is cut in the ring (the timber grate)'
    if t < 34:
        return 'Ordered out: the squad walks through the door'
    if thick_t is None or t < thick_t:
        return 'Ordered back in, through the door'
    if t < 60:
        return "Thicken x2: the door's wall and two each side, 2 to 6 cells, on the inside"
    return 'The slime presses on the ring; it has to eat through, door and all'
os.makedirs(out, exist_ok=True)
n = 0
for t in range(t0, t1 + 1, step):
    f = glob.glob(f'{run}/*-{t:03d}s-near.png')
    if not f:
        continue
    im = Image.open(f[0]).convert('RGB').crop(box)
    w, h = im.size
    canvas = Image.new('RGB', (w, h + 78), (20, 22, 20))
    canvas.paste(im, (0, 78))
    d = ImageDraw.Draw(canvas)
    d.text((6, 6), f'{t // 60}:{t % 60:02d}   {caption(t)}', fill=(255, 255, 255), font=font)
    v = counts.get(t) or counts.get(t - 1)
    if v:
        wl, ins, s, outside = v
        d.text((6, 30), f'soldiers {s}, outside the ring {outside}', fill=(232, 212, 150), font=font)
        d.text((6, 52), f'slime inside the ring {ins}   ring wall cells {wl}', fill=(232, 212, 150), font=font)
    canvas.save(f'{out}/f-{n:03d}.png')
    n += 1
print(n, 'frames')
