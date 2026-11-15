"""The cave-gates page's pictures, from frames/{before,after} (renders.sh) and the caves' maps (before: main's survey,
after: this change's): three gates at zoom 3 and zoom 6 at 1 s, and two caves' layouts in false colour (fc.py from
the fen report: before left, after right). Run from the folder holding frames/:
python3 make_pics.py OUT_DIR BEFORE_MAPS AFTER_MAPS"""
import sys, os, subprocess, tempfile
from PIL import Image
out, before_maps, after_maps = sys.argv[1:4]
os.makedirs(out, exist_ok=True)
for s in (2, 11, 16):
    for z in (3, 6):
        for tag in ("before", "after"):
            im = Image.open(f"frames/{tag}/gate-{s}-z{z}/planet-7-{s}-idle-001s-near.png").convert("RGB")
            w, h = im.size
            top, bottom = 70, 682  # below the objective line, above the squad badge and the bottom bar
            im = im.crop((0, top, w, bottom))
            if z == 6:  # the gate and its tunnel, round the screen's middle where the camera points
                cy = h // 2 - top
                im = im.crop((w // 2 - 360, cy - 225, w // 2 + 360, cy + 225))
            im.thumbnail((1100, 1100), Image.LANCZOS)
            im.save(f"{out}/gate-{s}-z{z}-{tag}.webp", quality=86, method=6)
fc = "/home/user/Slime-Mold/Reports/levelgen-fen-moats-2026-11-15/raw/fc.py"
with tempfile.TemporaryDirectory() as td:
    subprocess.run(["python3", "-I", fc, before_maps, after_maps, td, "planet-7-2", "planet-7-16"], check=True)
    for n in ("planet-7-2", "planet-7-16"):
        Image.open(f"{td}/{n}.png").convert("RGB").save(f"{out}/fc-{n[7:]}.webp", quality=90, method=6)
