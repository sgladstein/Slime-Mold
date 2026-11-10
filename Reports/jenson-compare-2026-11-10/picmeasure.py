"""Image measures for a still, the same way for Jenson's frames and ours:
tube = brightness over a cut (110 and 150 of 255); width = 2 x area / edge,
as a share of the dish's equivalent diameter; haze = share of dish pixels
between 20 and 80 of 255. Usage: picmeasure.py image [image...]"""
import sys
import numpy as np
from PIL import Image
for f in sys.argv[1:]:
    a = np.array(Image.open(f).convert('L')).astype(float)
    h, w = a.shape
    yy, xx = np.mgrid[:h, :w]
    if w == h:  # Jenson's square clips: the central disc, radius 0.45 w
        dish = ((xx - w / 2) ** 2 + (yy - h / 2) ** 2) < (0.45 * w) ** 2
    else:  # our dish: the agar ellipse, 3 dish cells (12 px) in from the edge
        rx, ry = w / 2 - 12, h / 2 - 12
        dish = ((xx + 0.5 - w / 2) / rx) ** 2 + ((yy + 0.5 - h / 2) / ry) ** 2 <= 1
    diam = np.sqrt(4 * dish.sum() / np.pi)
    out = []
    for cut in (110, 150):
        t = (a > cut) & dish
        edge = (t[:, 1:] != t[:, :-1]).sum() + (t[1:, :] != t[:-1, :]).sum()
        out.append('cut %d: width %.2f px = %.2f%%' % (cut, 2 * t.sum() / edge, 200 * t.sum() / edge / diam))
    d = a[dish]
    print('%-40s %s | haze %.2f' % (f.split('/')[-1], ' ; '.join(out), ((d >= 20) & (d < 80)).mean()))
