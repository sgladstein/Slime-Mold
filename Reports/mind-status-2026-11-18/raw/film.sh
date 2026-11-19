#!/bin/bash
# The Mind status films (Reports/mind-status-2026-11-18). From the repo's
# root after `cargo build --release --locked --bin larder-world`, on the
# software GPU:
#
#   film.sh OUT OLD_VIEW_WGSL   -> OUT/<name>/ frames, OUT/<name>.log
#
# OLD_VIEW_WGSL is `src/world/view.wgsl` from the commit before this change
# (`git show <commit>:src/world/view.wgsl`): the same build drawn with it
# (`LARDER_VIEW_WGSL`) is the old look, since the old shader leaves the two
# words this change fills unread. So both sides are one game.
#
# The Fork held at 150,265 with 8,000 food, firing Spin rounds, then Still
# rounds (`rounds=spin`, `rounds=still`; Mind bullets kill nothing, so the
# slime wades on in): 22 to 26 s, when the first statuses are on the slime
# coming in (a scan at 18 to 28 s: the rounds land from about 21 s). Zoom
# 12 and 6, 30 frames a game second, centred north of the squad (150,250)
# where the slime comes from.
set -e
out=${1:?film.sh OUT OLD_VIEW_WGSL}; oldv=${2:?film.sh OUT OLD_VIEW_WGSL}
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
game=${GAME:-./target/release/larder-world}
at=$(python3 -c "print(','.join('%.3f' % (22 + i / 30) for i in range(121)))")
film() { name=$1; shift; mkdir -p "$out/$name"; "$game" look="$out/$name" screen=1280x800 hand=hold@150,265 bank=8000 "$@" at=$at > "$out/$name.log" 2>&1; }
for r in spin still; do
  for z in 12 6; do
    film $r$z-after rounds=$r centre=150,250,$z &
    LARDER_VIEW_WGSL=$oldv film $r$z-before rounds=$r centre=150,250,$z &
  done
  wait
done
echo done
