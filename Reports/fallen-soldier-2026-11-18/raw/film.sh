#!/bin/bash
# The fallen soldier's films (Reports/fallen-soldier-2026-11-18). From the
# repo's root after `cargo build --release --locked --bin larder-world`,
# on the software GPU:
#
#   film.sh OUT [OLD_GAME]   -> OUT/<name>/ frames, OUT/<name>.log
#
# OLD_GAME, if given, is a larder-world built from main before this change
# (60502ed): its stills are the "before" (the corpse is a sprite tile and
# the shader's dead branch, so swapping the shader alone does not give the
# old look). The game is the same in both: only the drawing changed.
#
# The Fork held with plain rounds (`hand=hold@150,265 bank=8000
# rounds=plain`, not squad mode), where one soldier falls between 25 and
# 26 s, at zoom 12 and 6 from 25 to 28 s; planet 2 of run seed 1 with the
# greedy hand, where the squad loses soldiers through the fight at its
# first crystal, at zoom 6 and 12 from 86 to 92 s; 30 frames a game second.
set -e
out=${1:?film.sh OUT [OLD_GAME]}; old=$2
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
game=${GAME:-./target/release/larder-world}
film() { g=$1; name=$2; shift 2; mkdir -p "$out/$name"; "$g" look="$out/$name" screen=1280x800 "$@" > "$out/$name.log" 2>&1; }
at() { python3 -c "print(','.join('%.3f' % ($1 + i / 30) for i in range($2)))"; }
F="hand=hold@150,265 bank=8000 rounds=plain"
P="run=1:2 mode=squads hand=greedy"
film "$game" fork-z12 $F centre=158,268,12 at=$(at 25 91) &
film "$game" fork-z6 $F centre=158,268,6 at=$(at 25 91) &
wait
film "$game" p2-z6 $P centre=250,205,6 at=$(at 86 181) &
film "$game" p2-z12 $P centre=250,205,12 at=$(at 86 181) &
wait
if [ -n "$old" ]; then
  film "$old" fork-before $F centre=158,268,12 at=28 &
  film "$game" fork-after $F centre=158,268,12 at=28 &
  film "$old" p2-before $P at=90 &
  film "$game" p2-after $P at=90 &
  wait
fi
