#!/bin/bash
# The biting-slime films (Reports/biting-slime-2026-11-18). From the repo's root
# after `cargo build --release --locked --bin larder-world`, on the software
# GPU:
#
#   film.sh OUT OLD_GAME   -> OUT/<name>/ frames, OUT/<name>.log
#
# OLD_GAME is a larder-world built from the commit before this change (the
# bite zone is pushed by the window, so swapping the shader alone does not
# give the old look). The game is the same in both: only the drawing changed.
#
# The Fork held at 150,265 with plain rounds (not squad mode): the slime
# wades into the eight soldiers at 23 to 27 s and again at 41 to 45 s,
# the two windows where the most soldiers touch it (a scan once every 2 s
# from 10 to 50 s: 3.9 touching walkers a soldier at 24 s, 6.7 at 42 s,
# 1.1 at most elsewhere). Zoom 12 and 6, 30 frames a game second.
set -e
out=${1:?film.sh OUT OLD_GAME}; old=${2:?film.sh OUT OLD_GAME}
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
game=${GAME:-./target/release/larder-world}
film() { g=$1; name=$2; shift 2; mkdir -p "$out/$name"; "$g" look="$out/$name" screen=1280x800 hand=hold@150,265 bank=8000 rounds=plain "$@" > "$out/$name.log" 2>&1; }
at() { python3 -c "print(','.join('%.3f' % ($1 + i / 30) for i in range($2)))"; }
for z in 12 6; do
  film "$game" fork$z-after centre=150,263,$z at=$(at 23 121),$(at 41 121) &
  film "$old" fork$z-before centre=150,263,$z at=$(at 23 121),$(at 41 121) &
  wait
done
