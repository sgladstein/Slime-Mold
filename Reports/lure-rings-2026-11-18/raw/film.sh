#!/bin/bash
# film.sh BIN ROOT OUT T0 SECS FPS [args...]: one clip's frames on software
# Vulkan (llvmpipe), run from ROOT (the checkout BIN was built from, for its
# data files). The scene is the lab's (Reports/lab-steering-2026-11-17):
# planet 1, run seed 1, the squad held east of a Lure at dish cell (275, 95)
# with three turrets round it, the Lure built at 30 s, pull 10.
#
# BIN is a build of the lab's steering prototype (branch
# claude/project-thread-4tc75h) merged onto the flinch look, since the
# game's own field was not on main yet. The view shader is swapped with
# LARDER_VIEW_WGSL, so the three films of a zoom are one game:
#
#   film.sh LAB_BIN . FILMS/before-z6 34 6 30 centre=275,95,6                      # the lab's look
#   LARDER_VIEW_WGSL=NEW film.sh LAB_BIN . FILMS/after-z6 34 6 30 centre=275,95,6   # this report's view.wgsl
#   LARDER_VIEW_WGSL=OFF film.sh LAB_BIN . FILMS/none-z6 34 6 30 centre=275,95,6    # NEW with the rings' block compiled out
#   and the same three with `37 6 30 centre=268,88,12` for zoom 12.
#
# About 3 minutes a film, 1280x800 at 30 frames a game second, four at once
# on four cores.
BIN=$1; ROOT=$2; OUT=$3; T0=$4; SECS=$5; FPS=$6; shift 6
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
AT=$(python3 -c "print(','.join(f'{$T0 + i/$FPS:.3f}' for i in range(int($SECS*$FPS))))")
mkdir -p "$OUT"
SECONDS=0
cd "$ROOT" && "$BIN" look="$OUT" run=1:1 mode=squads bank=20000 hand=hold@325,80 far_smell=-10 turret2=275,75,20 turret2=292,106,20 turret2=258,106,20 lure=275,95,30 screen=1280x800 at="$AT" "$@" > "$OUT.txt" 2>&1
echo "$(basename $OUT): $SECONDS s wall, $(ls $OUT | grep -c near) near frames"
grep -E "^  t " "$OUT.txt" | sed -n '1p;$p'
