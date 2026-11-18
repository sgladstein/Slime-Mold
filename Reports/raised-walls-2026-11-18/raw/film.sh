#!/bin/bash
# The raised walls' and the Lure's rim's films (Reports/raised-walls-2026-11-18).
# From the repo's root after `cargo build --release --locked --bin
# larder-world`, on the software GPU, with the Trunk's fort (#473) in the
# build:
#
#   film.sh OUT OLD_WGSL     -> OUT/<name>/ frames, OUT/<name>.log
#
# OLD_WGSL is the view shader before (set slime flat, every ring drawn):
# `git show HEAD~1:src/world/view.wgsl` from this change's commit. Every
# pair is one game drawn two ways (`LARDER_VIEW_WGSL`), so the game lines
# are equal.
#
# The walls: planet 3 of run seed 1, idle, the home ring setting from 12 to
# 18 s, 30 frames a game second, at the Enemy lane's two cameras (zoom 6 on
# the home crystal, zoom 12 on a gate). The rim: planet 2, a Lure at
# (100, 168) from 30 s with three turrets, the whole planet at 40 s.
set -e
out=${1:?film.sh OUT OLD_WGSL}; old=${2:?film.sh OUT OLD_WGSL}
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
game=${GAME:-./target/release/larder-world}
film() { name=$1; shift; mkdir -p "$out/$name"; "$game" look="$out/$name" screen=1280x800 mode=squads "$@" > "$out/$name.log" 2>&1; }
A=$(python3 -c "print(','.join('%.3f' % (12 + i / 30) for i in range(181)))")
W="run=1:3 hand=idle at=$A"
film z6-after  $W centre=60,71,6 &
film z12-after $W centre=76,80,12 &
LARDER_VIEW_WGSL=$old film z6-before  $W centre=60,71,6 &
LARDER_VIEW_WGSL=$old film z12-before $W centre=76,80,12 &
wait
L="run=1:2 bank=20000 turret2=100,148,20 turret2=117,180,20 turret2=83,180,20 lure=100,168,30 hand=hold@70,140 at=40"
film rim-p2-new $L &
LARDER_VIEW_WGSL=$old film rim-p2-old $L &
wait
python3 "$(dirname "$0")/make_clips.py" "$out" "$(dirname "$0")/../page"
