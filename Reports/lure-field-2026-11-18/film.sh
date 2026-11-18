#!/bin/bash
# The lure field's films (2026-11-18, the Enemy lane). Run from the repo
# root after `cargo build --release --locked --bin larder-world`, on the
# software GPU (CLAUDE.md, "Cloud sessions have no GPU").
#
#   film.sh OUT        -> OUT/<name>/ frames and OUT/<name>.log
#
# The scene is the lab's (Reports/lab-steering-2026-11-17/raw/film.sh):
# planet 1 of run seed 1, three rendering turrets standing from 0:20, eight
# soldiers holding beside them, the lure between the turrets from 0:30.
# World units throughout (fine cells; planet 1 is 400 x 248).
set -e
out=${1:?film.sh OUT}
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
B="run=1:1 mode=squads bank=20000 turret2=275,75,20 turret2=292,106,20 turret2=258,106,20 watch=275,95,30 hand=hold@325,80"
film() { name=$1; shift; mkdir -p "$out/$name"; ./target/release/larder-world look="$out/$name" "$@" > "$out/$name.log" 2>&1; }

# The counts in the README's table: five frames each.
T="centre=255,85,6 at=30,45,60,75,90"
film L-off  $B $T
film L-bld  $B $T lure=275,95,30
film L-mark $B $T lure_mark=30,275,95,60

# The page's clips: ten frames a game second (make_clips.py cuts them).
A6=$(python3 -c "print(','.join('%.1f' % (26 + i / 10) for i in range(641)))")
A12=$(python3 -c "print(','.join('%.1f' % (34 + i / 10) for i in range(221)))")
film P-off-z6   $B centre=255,85,6 at=$A6
film P-bld-z6   $B centre=255,85,6 at=$A6 lure=275,95,30
film P-mark-z12 $B centre=272,92,12 at=$A12 lure_mark=30,275,95,60
