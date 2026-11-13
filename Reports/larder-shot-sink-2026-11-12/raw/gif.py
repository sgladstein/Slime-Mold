# Side-by-side GIF of a hand with the bite off (left) and on (right), with
# each side's shots and kills since the first frame. Usage: gif.py HAND FROM TO CROPX CROPY CROPW CROPH [TCUT]
import sys, re, os, subprocess
from PIL import Image, ImageDraw, ImageFont
hand, a, z = sys.argv[1], float(sys.argv[2]), float(sys.argv[3])
cx, cy, cw, ch = map(int, sys.argv[4:8])
cut = float(sys.argv[8]) if len(sys.argv) > 8 else z
times = [a + 0.5 * k for k in range(int((z - a) / 0.5) + 1)]
def lines(b):
    out = []
    for l in open(f'{hand}-b{b}.txt'):
        m = re.search(r' t \d+ s: mould (\d+), soldiers (\d+).*shots (\d+), killed (\d+)', l)
        if m: out.append(tuple(map(int, m.groups())))
    return out
L = {b: lines(b) for b in (0, 1)}
def name(b, t):
    d = f'{hand}-b{b}'
    return f'{d}/fork-{hand}-{int(t):03}s-near.png' if t == int(t) else f'{d}/fork-{hand}-{int(t*1000):05}ms-near.png'
try:
    font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 15)
    big = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf', 16)
except Exception:
    font = big = ImageFont.load_default()
s = 0.75
W, H = int(cw * s), int(ch * s)
os.makedirs(f'gif-{hand}', exist_ok=True)
for k, t in enumerate(times):
    if t > cut: break
    fr = Image.new('RGB', (2 * W + 8, H + 46), (24, 22, 20))
    d = ImageDraw.Draw(fr)
    for i, b in enumerate((0, 1)):
        im = Image.open(name(b, t)).convert('RGB').crop((cx, cy, cx + cw, cy + ch)).resize((W, H))
        fr.paste(im, (i * (W + 8), 46))
        m0, s0_, sh0, k0 = L[b][0]
        mo, so, sh, kl = L[b][k]
        n = sh - sh0
        per = f'{(kl - k0) / n:.1f} a shot' if n else '-'
        d.text((i * (W + 8) + 8, 4), ['Before: blast on the first slime', 'Now: blast sinks one radius in'][i], fill=(240, 220, 170), font=big)
        d.text((i * (W + 8) + 8, 24), f't {t:.1f} s   since {a:.0f} s: {n} shots, {kl - k0} killed, {per}', fill=(220, 220, 220), font=font)
    fr.save(f'gif-{hand}/{k:03}.png')
subprocess.run(['ffmpeg', '-y', '-loglevel', 'error', '-framerate', '4', '-i', f'gif-{hand}/%03d.png', '-vf', 'split[a][b];[a]palettegen=max_colors=192[p];[b][p]paletteuse=dither=bayer:bayer_scale=4', f'{hand}.gif'], check=True)
print(hand, os.path.getsize(f'{hand}.gif') // 1024, 'KB')
