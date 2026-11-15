"""Before/after frames to the page: make_page.py BEFORE AFTER OUT.
BEFORE and AFTER each hold near/ (zoom 12) and mid/ (zoom 4) from films.sh.
Each picture is 2 s, then 120 s before, then 120 s after, colour over grey,
cropped round the river (not scaled)."""
import glob, os, sys
from PIL import Image
import numpy as np
before, after, out = sys.argv[1:4]
def frame(d, sub, t):
    return Image.open(glob.glob(os.path.join(d, sub, f"*{t}-near.png"))[0]).convert("RGB").crop((330, 60, 630, 500))
def blue(im):
    a = np.asarray(im).astype(int)
    return (a[:, :, 2] > a[:, :, 1] + 4).mean()
for sub in ["near", "mid"]:
    ims = [frame(before, sub, "002s"), frame(before, sub, "120s"), frame(after, sub, "120s")]
    sheet = Image.new("RGB", (3 * 310 - 10, 2 * 450 - 10), (20, 20, 20))
    for n, im in enumerate(ims):
        sheet.paste(im, (n * 310, 0)); sheet.paste(im.convert("L").convert("RGB"), (n * 310, 450))
    sheet.save(os.path.join(out, f"river-{sub}.png"))
    print(sub, "blue share 2 s / 120 s before / 120 s after:", " / ".join(f"{blue(im):.3f}" for im in ims))
