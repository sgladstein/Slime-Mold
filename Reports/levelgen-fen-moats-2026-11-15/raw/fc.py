"""False-colour layouts, two map folders side by side: python3 fc.py LEFT RIGHT OUT NAME... (the page used
main's maps or the rock twin on the left and the moats on the right)."""
import sys
import numpy as np
from PIL import Image, ImageDraw
def view(base, W=700):
    im = np.array(Image.open(f"{base}.png").convert("RGBA"))
    g = im[:, :, 1].astype(int)
    out = np.zeros(g.shape + (3,), np.uint8)
    out[:] = (95, 100, 60)
    out[(g & 1) != 0] = (60, 100, 140)
    out[g == 0x41] = (25, 40, 80)
    out[(g >> 4) == 5] = (170, 120, 60)
    out[(g & 8) != 0] = (150, 150, 155)
    pic = Image.fromarray(out)
    d = ImageDraw.Draw(pic)
    for line in open(f"{base}.txt"):
        w = line.split()
        if not w: continue
        if w[0] == "wall":
            a, b, c, e, t = map(float, w[1:6]); d.line([(a, b), (c, e)], fill=(220, 60, 220), width=max(2, int(t)))
        elif w[0] == "crystal":
            x, y, r = map(float, w[1:4]); d.ellipse([x - r, y - r, x + r, y + r], fill=(60, 220, 230))
        elif w[0] == "mould":
            x, y = map(float, w[1:3]); d.ellipse([x - 40, y - 40, x + 40, y + 40], outline=(255, 60, 40), width=10)
        elif w[0] == "landing":
            x, y = map(float, w[1:3]); d.rectangle([x - 30, y - 30, x + 30, y + 30], outline=(255, 230, 0), width=8)
    k = W / pic.width
    return pic.resize((W, int(pic.height * k)), Image.LANCZOS)
left, right, out = sys.argv[1:4]
for name in sys.argv[4:]:
    b, a = view(f"{left}/{name}"), view(f"{right}/{name}")
    sheet = Image.new("RGB", (b.width + a.width + 10, max(b.height, a.height)), (20, 20, 20))
    sheet.paste(b, (0, 0)); sheet.paste(a, (b.width + 10, 0))
    sheet.save(f"{out}/{name}.png")
