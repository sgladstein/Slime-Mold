# Side-by-side clip of two look runs (2026-11-13, wading): each half is the
# near view cropped clear of the HUD, with the run's own counts above it.
# Usage: clip.py OUT LEFT RIGHT "LEFT LABEL" "RIGHT LABEL" FROM TO [MODE]
# MODE walk: soldiers, health, walkers on each, distance walked; MODE fire:
# shots and kills since FROM.
import sys, re, os, subprocess, glob
from PIL import Image, ImageDraw, ImageFont
out, L, R, la, lb, a, z = sys.argv[1:8]
a, z = float(a), float(z)
mode = sys.argv[8] if len(sys.argv) > 8 else 'walk'
def rows(n):
    t = open(f'clip-{n}.txt').read().splitlines()
    out, cur = {}, None
    for l in t:
        m = re.match(r'\s+t (\d+) s: mould (\d+), soldiers (\d+).*shots (\d+), killed (\d+)(.*)', l)
        if m:
            sq = re.search(r'squad at ([\d.]+),([\d.]+)', m.group(6))
            cur = dict(mould=int(m.group(2)), n=int(m.group(3)), shots=int(m.group(4)), killed=int(m.group(5)), at=(float(sq.group(1)), float(sq.group(2))) if sq else None, touch=0.0, hp=0.0)
            out.setdefault('list', []).append(cur)
            continue
        m = re.match(r'\s+squad: touching ([\d.]+) mean, (\d+) most; health (\d+) mean', l)
        if m and cur is not None:
            cur['touch'], cur['most'], cur['hp'] = float(m.group(1)), int(m.group(2)), int(m.group(3))
    return out['list']
def frames(n):
    fs = sorted(glob.glob(f'clip-{n}/*-near.png'), key=lambda p: int(re.search(r'-(\d+)(ms|s)-near', p).group(1)) * (1 if 'ms-' in p else 1000))
    return fs
F = {n: frames(n) for n in (L, R)}
D = {n: rows(n) for n in (L, R)}
# A run that ended early (squad wiped) holds its last frame to the other's end.
k = max(min(len(F[n]), len(D[n])) for n in (L, R))
last = {n: min(len(F[n]), len(D[n])) - 1 for n in (L, R)}
try:
    font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 14)
    big = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf', 15)
except Exception:
    font = big = ImageFont.load_default()
crop = (0, 40, 1280, 630)
W, H = 640, 296
os.makedirs(f'frames-{out}', exist_ok=True)
for f in glob.glob(f'frames-{out}/*.png'): os.remove(f)
for i in range(k):
    t = a + 0.5 * i
    fr = Image.new('RGB', (2 * W + 8, H + 48), (24, 22, 20))
    d = ImageDraw.Draw(fr)
    for j, (n, lab) in enumerate(((L, la), (R, lb))):
        q = min(i, last[n])
        im = Image.open(F[n][q]).convert('RGB').crop(crop).resize((W, H))
        fr.paste(im, (j * (W + 8), 48))
        r, r0 = D[n][q], D[n][0]
        if i > last[n]:
            r = dict(r, n=0)
        if mode == 'walk':
            walked = ((r['at'][0] - r0['at'][0]) ** 2 + (r['at'][1] - r0['at'][1]) ** 2) ** 0.5 / 2 if r['at'] and r0['at'] else None
            line = f"t {t:.1f} s   soldiers {r['n']}" + (f",  health {r['hp']}, walkers on each {r['touch']:.1f}" if r['n'] else ',  squad wiped')
        else:
            sh, kl = r['shots'] - r0['shots'], r['killed'] - r0['killed']
            line = f"t {t:.1f} s   since {a:.0f} s: {sh} shots, {kl} killed" + (f", {kl / sh:.1f} a shot" if sh else '') + f";  health {r['hp']}"
        d.text((j * (W + 8) + 8, 4), lab, fill=(240, 220, 170), font=big)
        d.text((j * (W + 8) + 8, 26), line, fill=(222, 222, 222), font=font)
    fr.save(f'frames-{out}/{i:03}.png')
# Twice game speed (frames every half game second at 4 a second), and a
# poster from the last frame.
subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-framerate', '4', '-i', f'frames-{out}/%03d.png', '-c:v', 'libx264', '-pix_fmt', 'yuv420p', '-crf', '23', '-movflags', '+faststart', f'{out}.mp4'], check=True)
Image.open(f'frames-{out}/{k - 1:03}.png').convert('RGB').save(f'{out}.jpg', quality=85)
print(out, k, 'frames', os.path.getsize(f'{out}.mp4') // 1024, 'KB')
