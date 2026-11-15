"""Fire and slime luma/hue percentiles: measure.py FIRE NOFIRE SLIME (near PNGs).
Fire pixels: changed by more than 40 (sum of |RGB|) from the no-fire frame
and warm (R > B + 60). Slime pixels: the slime frame's warm saturated
pixels (R > B + 80). Hue in degrees (HSV), luma Rec.601. The bottom 120 px
(HUD) is cut."""
import sys, colorsys
import numpy as np
from PIL import Image
def load(p):
    a = np.asarray(Image.open(p).convert("RGB")).astype(float)
    return a[:-120]
f, n, s = (load(p) for p in sys.argv[1:4])
def stats(px, name):
    luma = px @ np.array([0.299, 0.587, 0.114])
    hue = np.array([colorsys.rgb_to_hsv(*(c / 255))[0] * 360 for c in px[:: max(1, len(px) // 20000)]])
    q = [10, 50, 90, 99]
    print(f"{name}: {len(px)} px; luma p10/50/90/99 {np.percentile(luma, q).round(0)}; hue p10/50/90 {np.percentile(hue, [10, 50, 90]).round(0)}")
mf = (np.abs(f - n).sum(-1) > 40) & (f[..., 0] > f[..., 2] + 60)
stats(f[mf], "fire")
ms = s[..., 0] > s[..., 2] + 80
stats(s[ms], "slime")
