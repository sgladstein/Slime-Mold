# Join look frames into 10 fps GIFs (near view), downscaled to 640x400.
import sys, glob
from PIL import Image
R = '/home/user/review-aesthetics/'
for name in sys.argv[1:]:
    fs = sorted(glob.glob(R + f'frames/gif/{name}/*-near.png'))
    ims = [Image.open(f).convert('RGB').resize((640, 400), Image.LANCZOS) for f in fs]
    pal = [im.quantize(colors=200, method=Image.MEDIANCUT, dither=Image.NONE) for im in ims]
    pal[0].save(R + f'gif-{name}-z12.gif', save_all=True, append_images=pal[1:], duration=100, loop=0)
    # one full-size still from the middle for close reading
    Image.open(fs[len(fs) // 2]).save(R + f'crops/{name}-z12-mid.png')
    print(name, len(fs))
