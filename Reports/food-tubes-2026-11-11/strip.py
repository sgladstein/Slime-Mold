#!/usr/bin/env python3
"""Picture strips for the food-tubes page: python3 strip.py LOOK_DIR OUT_DIR
NAME SEED T1,T2,... Joins bigdish's `look=` stills big-NAME-sSEED-TTTs.png
side by side, 6 px apart, into OUT_DIR/NAME-sSEED.jpg."""
import sys
from PIL import Image
d, out, name, seed, ts = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5].split(',')
ims = [Image.open(f"{d}/big-{name}-s{seed}-{int(t):03d}s.png").convert('RGB') for t in ts]
w, h = ims[0].size
S = Image.new('RGB', (len(ims) * w + (len(ims) - 1) * 6, h), (12, 14, 13))
for i, im in enumerate(ims):
    S.paste(im, (i * (w + 6), 0))
S.save(f"{out}/{name}-s{seed}.jpg", quality=86)
