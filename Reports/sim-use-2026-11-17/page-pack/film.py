"""Builds the playtest pack's blind films (11-17, group 2) from larder-world's
and dryprobe's stills. python3 film.py SRC KEY.json; SRC holds the runs
(film.sh), KEY.json says which arm is A for each film (the coin tosses; the
key is in ../pack-films.md, not on the page). Two panels side by side, A
left, B right, each labelled with its letter and the game time."""
import glob, json, subprocess, sys, tempfile
from PIL import Image, ImageDraw, ImageFont

src, key = sys.argv[1], json.load(open(sys.argv[2]))
try:
    font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 26)
except OSError:
    font = ImageFont.load_default()

# film: (arms by name, frame glob per arm, frames a second, panel scale,
# label per frame).
FILMS = {
    "setting": (("set0", "set1"), "{d}/*-{t:03}s-near.png", range(30, 90), 6, 0.75, lambda t: f"{t} s"),
    "hold": (("block", "trap"), "{d}/*-{t:03}s-near.png", range(3, 33), 4, 0.75, lambda t: f"{t} s"),
    "pull": (("keep", "move"), "{d}/*-{t:03}s-all.png", range(5, 305, 5), 6, 0.75, lambda t: f"{t // 60}:{t % 60:02}"),
    "fed": (("fed0", "fed1"), "{d}-{t:03}.png", range(0, 30), 4, 2.0, lambda t: f"{t / 3:.1f} s after the light"),
}


def panel(path, s, letter, label):
    im = Image.open(path).convert("RGB")
    im = im.resize((int(im.width * s) // 2 * 2, int(im.height * s) // 2 * 2), Image.NEAREST if s > 1 else Image.LANCZOS)
    g = ImageDraw.Draw(im)
    g.rectangle((0, im.height - 40, 300, im.height), fill=(16, 16, 20))
    g.text((10, im.height - 36), f"{letter}   {label}", fill=(255, 255, 255), font=font)
    return im


def pair(film, d_of, t):
    arms, pat, _, _, s, lab = FILMS[film]
    a = key[film]
    order = (a, [x for x in arms if x != a][0])
    ims = []
    for letter, arm in zip("AB", order):
        p = glob.glob(pat.format(d=d_of(arm), t=t))
        ims.append(panel(p[0], s, letter, lab(t)))
    w, h = ims[0].size
    c = Image.new("RGB", (2 * w + 12, h), (240, 240, 240))
    c.paste(ims[0], (0, 0))
    c.paste(ims[1], (w + 12, 0))
    return c


def main():
    for film, (arms, _, ts, fps, _, _) in FILMS.items():
        d_of = (lambda a: f"{src}/fed/seed1-{'joined-fed' if a == 'fed1' else 'joined'}") if film == "fed" else (lambda a: f"{src}/{a}")
        with tempfile.TemporaryDirectory() as tmp:
            for q, t in enumerate(ts):
                pair(film, d_of, t).save(f"{tmp}/{q:03}.png")
            subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", str(fps), "-i", f"{tmp}/%03d.png", "-vf", "scale=trunc(iw/2)*2:trunc(ih/2)*2", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "23", "-movflags", "+faststart", f"{film}.mp4"], check=True)
        print(film, len(ts), "frames")


main()
