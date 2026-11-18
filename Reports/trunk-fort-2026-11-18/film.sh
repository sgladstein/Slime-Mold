#!/bin/bash
# The Trunk's fort films (2026-11-18, the Enemy lane). Run from the repo
# root after `cargo build --release --locked --bin larder-world`, on the
# software GPU (CLAUDE.md, "Cloud sessions have no GPU").
#
#   film.sh OUT        -> OUT/<name>/ frames and OUT/<name>.log
#
# Planet 3 of run seed 1 (the Trunk). World units (fine cells) for
# `centre=`; `light=T,X,Y,R` takes dish cells, which are half of them on a
# run planet. The slime's home crystal is at about (76, 71) (the zoom-6
# camera asks for 60 and is held at the map's edge); one of its ring's four
# gates is at about (101, 81), and the south stretch of the wall passes
# (87, 96), dish (43.5, 48). The squad lands at (435, 262), beside the
# southern crystals. `crustlook=1` adds the log's "walkers set, bulwark"
# and fire lines.
set -e
out=${1:?film.sh OUT}
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
B="run=1:3 mode=squads"
film() { name=$1; shift; mkdir -p "$out/$name"; ./target/release/larder-world look="$out/$name" "$@" > "$out/$name.log" 2>&1; }

# The counts in the README's tables: the fort on (the game) and off.
film G-idle-on  $B hand=idle   at=60,120,180 centre=60,71,6 crustlook=1
film G-idle-off $B hand=idle   at=60,120,180 centre=60,71,6 crustlook=1 set=fort=0
film G-rush-on  $B hand=greedy at=60,120     centre=60,71,6 crustlook=1
film G-rush-off $B hand=greedy at=60,120     centre=60,71,6 crustlook=1 set=fort=0

# The page's clips (make_clips.py cuts them): ten frames a game second at
# the home crystal, five where the squad lands and for the fire.
A6=$(python3 -c "print(','.join('%.1f' % (2 + i / 10) for i in range(301)))")
A12=$(python3 -c "print(','.join('%.1f' % (8 + i / 10) for i in range(201)))")
AS=$(python3 -c "print(','.join('%.1f' % (60 + i / 5) for i in range(601)))")
AF=$(python3 -c "print(','.join('%.1f' % (58 + i / 5) for i in range(361)))")
film P-home-z6  $B hand=idle centre=60,71,6   at=$A6
film P-gate-z12 $B hand=idle centre=76,80,12  at=$A12
film P-south-z6 $B hand=idle centre=370,250,6 at=$AS
# A fire lit on the home ring's south stretch at 1:00.
film P-fire-z6  $B hand=idle centre=60,71,6   at=$AF light=60,43.5,48,3 crustlook=1
