"""A false-colour contact sheet of saved planet maps (planets 1-8, seeds 1-2),
straight from the map files, and the same in grey.

  cargo run --release --example planets -- seeds=1-2 depths=1-8 out=MAPS
  python3 layout_sheet.py MAPS OUT.png      # also writes OUT-grey.png
"""
import sys
from pathlib import Path
from PIL import Image, ImageDraw
src, out = Path(sys.argv[1]), Path(sys.argv[2])
COL = {0: (38, 42, 48), 1: (120, 104, 70), 2: (96, 60, 46), 3: (70, 70, 66)}
tiles = []
for d in range(1, 9):
    for s in (1, 2):
        name = f"planet-{d}-{s}"
        g = Image.open(src / f"{name}.png").getchannel(1)
        w, h = g.size
        px = g.load()
        im = Image.new("RGB", (w, h))
        o = im.load()
        for y in range(h):
            for x in range(w):
                v = px[x, y]
                if v & 8:
                    c = (140, 142, 146)
                elif v & 1:
                    c = (34, 64, 96)
                else:
                    c = COL[v >> 4]
                o[x, y] = c
        dr = ImageDraw.Draw(im)
        for line in (src / f"{name}.txt").read_text().splitlines():
            p = line.split()
            if not p: continue
            v = list(map(float, p[1:]))
            if p[0] == "crystal":
                dr.ellipse([v[0]-v[2], v[1]-v[2], v[0]+v[2], v[1]+v[2]], fill=(63, 198, 220))
            elif p[0] == "wall":
                dr.line([v[0], v[1], v[2], v[3]], fill=(185, 180, 138), width=int(v[4]))
                r = v[4] / 2
                for (cx, cy) in ((v[0], v[1]), (v[2], v[3])):
                    dr.ellipse([cx-r, cy-r, cx+r, cy+r], fill=(185, 180, 138))
            elif p[0] == "mould":
                dr.ellipse([v[0]-14, v[1]-14, v[0]+14, v[1]+14], outline=(232, 164, 58), width=5)
            elif p[0] == "landing":
                dr.rectangle([v[0]-12, v[1]-12, v[0]+12, v[1]+12], outline=(255, 255, 255), width=5)
        tiles.append(im.resize((w // 2, h // 2), Image.LANCZOS))
tw, th = tiles[0].size
sheet = Image.new("RGB", (tw * 4 + 30, th * 4 + 30), (10, 10, 10))
for i, t in enumerate(tiles):
    sheet.paste(t, ((i % 4) * (tw + 10), (i // 4) * (th + 10)))
sheet.save(out)
sheet.convert("L").save(out.with_name(out.stem + "-grey.png"))
