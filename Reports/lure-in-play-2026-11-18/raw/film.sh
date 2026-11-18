#!/bin/bash
# The Lure in play on planets 1 to 3 (Reports/lure-in-play-2026-11-18).
# From the repo's root, after `cargo build --release --locked --bin
# larder-world`, on the software GPU, with Enemy's round field (#478):
#
#   film.sh OUT              -> OUT/<name>/ frames, OUT/<name>.log
#
# Each planet of run seed 1: three rendering turrets from 0:20 round a
# Lure built at 0:30, eight soldiers holding beside them (planet 1 is the
# lab's and the Enemy lane's scene), `watch=` counting the walkers within
# 30 and 30 to 60 cells of the Lure each second. Zoom 6 from 36 to 42 s and
# zoom 12 from 40 to 46 s, 30 frames a game second; one frame of the whole
# planet at 40 s. World units (fine cells).
set -e
out=${1:?film.sh OUT}
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
game=${GAME:-./target/release/larder-world}
film() { name=$1; shift; mkdir -p "$out/$name"; "$game" look="$out/$name" screen=1280x800 mode=squads bank=20000 "$@" > "$out/$name.log" 2>&1; }
times() { python3 -c "print(','.join('%.3f' % ($1 + i / 30) for i in range(181)))"; }
P1="run=1:1 turret2=275,75,20 turret2=292,106,20 turret2=258,106,20 lure=275,95,30 watch=275,95,30 hand=hold@325,80"
P2="run=1:2 turret2=100,148,20 turret2=117,180,20 turret2=83,180,20 lure=100,168,30 watch=100,168,30 hand=hold@70,140"
P3="run=1:3 turret2=357,162,20 turret2=374,193,20 turret2=340,193,20 lure=357,182,30 watch=357,182,30 hand=hold@400,200"
Z6=$(times 36); Z12=$(times 40)
film p1-z6  $P1 centre=262,90,6 at=$Z6 &
film p2-z6  $P2 centre=112,166,6 at=$Z6 &
film p3-z6  $P3 centre=352,178,6 at=$Z6 &
wait
film p1-z12 $P1 centre=272,92,12 at=$Z12 &
film p2-z12 $P2 centre=106,168,12 at=$Z12 &
film p3-z12 $P3 centre=354,180,12 at=$Z12 &
wait
film p1-whole $P1 at=40 &
film p2-whole $P2 at=40 &
film p3-whole $P3 at=40 &
wait
