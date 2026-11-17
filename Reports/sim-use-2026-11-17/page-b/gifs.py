"""Side-by-side GIFs of mindprobe's frames (first seed): each arm a panel."""
import sys
from PIL import Image

src, out = sys.argv[1], sys.argv[2]
sets = {"provoke": ["provoke-plain", "provoke-deaf"], "still": ["still-plain", "still-held"]}
for name, arms in sets.items():
    frames = []
    q = 0
    while True:
        try:
            ims = [Image.open(f"{src}/{a}-{q:03}.png").convert("RGB") for a in arms]
        except FileNotFoundError:
            break
        w, h = ims[0].size
        gap = 8
        sheet = Image.new("RGB", (w * len(ims) + gap * (len(ims) - 1), h), (24, 22, 18))
        for i, im in enumerate(ims):
            sheet.paste(im, (i * (w + gap), 0))
        sheet = sheet.resize((sheet.width * 7 // 10, sheet.height * 7 // 10), Image.NEAREST)
        frames.append(sheet.quantize(colors=96, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.NONE))
        q += 1
    frames[0].save(f"{out}/{name}.gif", save_all=True, append_images=frames[1:], duration=110, loop=0, optimize=True)
    print(name, len(frames))
