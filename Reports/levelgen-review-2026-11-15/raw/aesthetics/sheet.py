# Contact sheet: sheet.py OUT COLS TILE_W TILE_H file[:label] ...
import sys
from PIL import Image, ImageDraw
out, cols, tw, th = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
items = sys.argv[5:]
rows = (len(items) + cols - 1) // cols
sheet = Image.new('RGB', (cols * tw, rows * (th + 14)), (16, 16, 16))
d = ImageDraw.Draw(sheet)
for i, it in enumerate(items):
    f, _, lab = it.partition(':')
    im = Image.open(f).convert('RGB').resize((tw, th), Image.LANCZOS)
    x, y = (i % cols) * tw, (i // cols) * (th + 14)
    sheet.paste(im, (x, y + 14)); d.text((x + 3, y + 1), lab or f.split('/')[-1], fill=(230, 230, 230))
sheet.save(out)
