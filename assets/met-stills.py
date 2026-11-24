"""Cut the Slimes met page's stills into one sheet (assets/met-stills.png).

Direction 11-24 asked for "a small still from the game" of each slime and
threat on the Slimes met page. Each is a headless film's near frame
(1280x800, software Vulkan), cropped to the thing and scaled to a cell of
240 x 150 (`canvas::STILL_W`, `STILL_H`), six a row, in the order of
`met.rs`'s `STILLS`.

    python3 assets/met-stills.py FILMS_DIR

FILMS_DIR holds the frames named below (`look=` folders); a missing frame
leaves its cell dark, and the page draws it so. How each was filmed (run
seed 1, `mode=squads screen=1280x800`, `LARDER_HINTS=off` so no hint's
words are in the picture):

- slime-*: `run=1:N hand=idle centre=HX,HY,2.5 at=45`, HX,HY the heart's
  first place (the film's `heart t 1 s` line), planet N the slime's.
- boss-KIND: `run=1:3 hand=idle LARDER_FORCE=boss-KIND@5 follow=boss
  centre=squad,8 at=18,26` (the frame each names below).
- core: `run=1:2 hand=idle LARDER_FORCE=core@3 centre=squad,8 at=5`.
- fruit: `run=1:3 hand=idle LARDER_FORCE=fruit@5 centre=82,75,8 at=16`.
- wave: `run=1:3 hand=greedy LARDER_FORCE=wave@60 centre=squad,4 at=70`.
- stand: `run=1:4 hand=idle set=origin=0 LARDER_FORCE=stand@5
  centre=98,126,6 at=20`.
- strikers: `run=1:2 hand=greedy centre=squad,5 at=100`.
- lunge: `run=1:2 hand=hold@320,149 LARDER_FORCE=lunge@8
  centre=340,149,5 at=34`: the heart's dark body reaching off its crystal
  into the squad that holds beside it.
- wake: `run=1:6 hand=hold@700,229 centre=736,229,6 at=35`: the squad
  holding beside the heart wakes it, orange among the grey sleeping slime.
- scar: `run=1:7 hand=hold@200,520 bank=3000 centre=150,520,6 at=90`: the
  squad fights from the heart's edge for a minute and a half, and the arm
  the slime sends at it runs pale and glossy where it grew back tough.
"""

import os
import sys

from PIL import Image

W, H, COLS = 240, 150, 6

# key, frame (under FILMS_DIR), crop box (left, top, right, bottom) in the
# 1280x800 frame, 1.6 wide to 1 high. Order: `met.rs` STILLS.
CUTS = [
    ("slime-drifter", "s1/planet-1-1-idle-045s-near.png", (140, 250, 780, 650)),
    ("slime-sprawler", "s2/planet-2-1-idle-045s-near.png", (560, 170, 1200, 570)),
    ("slime-trunk", "s3/planet-3-1-idle-045s-near.png", (24, 60, 664, 460)),
    ("slime-swarmer", "s4/planet-4-1-idle-045s-near.png", (0, 120, 640, 520)),
    ("slime-sporer", "s5/planet-5-1-idle-045s-near.png", (560, 260, 1200, 660)),
    ("slime-sleeper", "s6/planet-6-1-idle-045s-near.png", (660, 170, 1280, 557)),
    ("slime-grudge", "s7/planet-7-1-idle-045s-near.png", (20, 220, 660, 620)),
    ("slime-dozer", "s8/planet-8-1-idle-045s-near.png", (320, 140, 960, 540)),
    ("strikers", "t-strikers/planet-2-1-greedy-100s-near.png", (320, 100, 960, 500)),
    ("fruit", "t-fruit/planet-3-1-idle-016s-near.png", None),
    ("core", "t-core/planet-2-1-idle-005s-near.png", None),
    ("wave", "t-wave/planet-3-1-greedy-070s-near.png", (320, 45, 960, 445)),
    ("lunge", "r-lunge2/planet-2-1-hold@320,149-034s-near.png", (440, 180, 1000, 530)),
    ("stand", "t-stand/planet-4-1-idle-020s-near.png", (280, 160, 920, 560)),
    ("wake", "r-wake1/planet-6-1-hold@700,229-035s-near.png", (150, 130, 950, 630)),
    ("scar", "r-scar2/planet-7-1-hold@200,520-090s-near.png", (560, 180, 1200, 580)),
    ("boss-hunter", "b2-hunter/planet-3-1-idle-018s-near.png", None),
    ("boss-wanderer", "b2-wanderer/planet-3-1-idle-026s-near.png", None),
    ("boss-sapper", "b2-sapper/planet-3-1-idle-018s-near.png", None),
    ("boss-leech", "b2-leech/planet-3-1-idle-018s-near.png", None),
    ("boss-mortar", "b2-mortar/planet-3-1-idle-026s-near.png", None),
    ("boss-pinwheel", "b2-pinwheel/planet-3-1-idle-018s-near.png", None),
]

# The middle of a near frame, clear of the HUD's top and bottom bars: a
# film that centres on the thing.
MIDDLE = (320, 200, 960, 600)


def main(films):
    rows = (len(CUTS) + COLS - 1) // COLS
    sheet = Image.new("RGB", (W * COLS, H * rows), (18, 21, 25))
    for i, (key, frame, box) in enumerate(CUTS):
        path = os.path.join(films, frame)
        if not os.path.exists(path):
            print(f"{key}: no {frame}, left dark")
            continue
        im = Image.open(path).convert("RGB").crop(box or MIDDLE).resize((W, H), Image.LANCZOS)
        sheet.paste(im, ((i % COLS) * W, (i // COLS) * H))
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "met-stills.png")
    sheet.save(out, optimize=True)
    print(f"{out}: {sheet.size[0]}x{sheet.size[1]}, {os.path.getsize(out)} bytes")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else ".")
