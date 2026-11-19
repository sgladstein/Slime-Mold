#!/bin/bash
# The heads-on-top films (Reports/squad-heads-2026-11-18). From the repo's
# root after `cargo build --release --locked --bin larder-world`, on the
# software GPU:
#
#   film.sh OUT OLD_GAME   -> OUT/<name>/ frames, OUT/<name>.log
#
# OLD_GAME is a larder-world built from the commit before this change (the
# heads are marks the window pushes, so swapping the shader alone does not
# give the old look). The game is the same in both: only the drawing
# changed.
#
# Planet 1 of run seed 1 with the greedy hand: 86 to 92 s, 35 soldiers
# draining the first crystal (centre 215,48), and 118 to 124 s, 42 walking
# to the next (centre 350,68); zoom 6 and 12, 30 frames a game second.
set -e
out=${1:?film.sh OUT OLD_GAME}; old=${2:?film.sh OUT OLD_GAME}
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
game=${GAME:-./target/release/larder-world}
film() { g=$1; name=$2; shift 2; mkdir -p "$out/$name"; "$g" look="$out/$name" screen=1280x800 run=1:1 mode=squads hand=greedy "$@" > "$out/$name.log" 2>&1; }
at() { python3 -c "print(','.join('%.3f' % ($1 + i / 30) for i in range(181)))"; }
for z in 6 12; do
  film "$game" drain$z-after centre=215,48,$z at=$(at 86) &
  film "$old" drain$z-before centre=215,48,$z at=$(at 86) &
  wait
done
for z in 6 12; do
  film "$game" walk$z-after centre=350,68,$z at=$(at 118) &
  film "$old" walk$z-before centre=350,68,$z at=$(at 118) &
  wait
done
