#!/bin/bash
# The droplets' films (Reports/droplets-2026-11-18). From the repo's root
# after `cargo build --release --locked --bin larder-world`, on the
# software GPU:
#
#   film.sh OUT OLD_WGSL     -> OUT/<name>/ frames, OUT/<name>.log
#
# OLD_WGSL is the view shader before (droplets in a dark rim):
# `git show HEAD~1:src/world/view.wgsl` from this change's commit. Every
# pair is one game drawn two ways (`LARDER_VIEW_WGSL`), so the game lines
# are equal.
#
# The gun range (`range=7 hand=stand`) at zoom 12, 2.5 to 4.5 s, and the
# Fork held with plain rounds (`hand=hold@150,265 bank=8000 rounds=plain`)
# at zoom 12 and 6, 30 to 32 s, as the spray A/B and gun feel reports
# filmed them; 30 frames a game second.
set -e
out=${1:?film.sh OUT OLD_WGSL}; old=${2:?film.sh OUT OLD_WGSL}
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
game=${GAME:-./target/release/larder-world}
film() { name=$1; shift; mkdir -p "$out/$name"; "$game" look="$out/$name" screen=1280x800 "$@" > "$out/$name.log" 2>&1; }
at() { python3 -c "print(','.join('%.3f' % ($1 + i / 30) for i in range(61)))"; }
R="range=7 hand=stand centre=258,121,12 at=$(at 2.5)"
F="hand=hold@150,265 bank=8000 rounds=plain at=$(at 30)"
film range-after $R &
LARDER_VIEW_WGSL=$old film range-before $R &
film fork12-after $F centre=158,268,12 &
LARDER_VIEW_WGSL=$old film fork12-before $F centre=158,268,12 &
wait
film fork6-after $F centre=158,268,6 &
LARDER_VIEW_WGSL=$old film fork6-before $F centre=158,268,6 &
wait
