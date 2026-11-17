#!/bin/sh
# The playtest pack's blind films (11-17, group 2): the runs behind film.py.
# Run from the repo root after
#   cargo build --release --locked --bin larder-world --example dryprobe
# with OUT the folder for the stills; on a machine without a GPU,
# VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json (llvmpipe).
OUT=${1:?out dir}
LW=./target/release/larder-world
mkdir -p "$OUT"
at() { seq -s, "$@"; }
# Crust's pull: the same idle planet with the pull at -8 (moves on) and +8
# (keeps to old ground), crust_fast 1, a still every 5 s to 5:00.
for w in 8 -8; do
  d=$OUT/$([ "$w" = 8 ] && echo keep || echo move); mkdir -p "$d"
  $LW look=$d run=1:3 hand=idle crust=1 crustlook=1 crust_weight=$w crust_fast=1 at=$(at 5 5 300) > "$d.txt" 2>&1 &
done
wait
# Setting on or off: four rifles stand at the first slime (seed 4).
for s in 0 1; do
  d=$OUT/set$s; mkdir -p "$d"
  $LW look=$d run=4:3 hand=stand gun=rifle crust=1 crustlook=1 setting=$s at=$(at 30 1 89) > "$d.txt" 2>&1 &
done
# Block or trap: eight rifles rush into the body (seed 1).
d=$OUT/block; mkdir -p "$d"
$LW look=$d run=1:3 hand=push gun=rifle crust=1 crustlook=1 setting=1 set=set_block=5 at=$(at 3 1 32) > "$d.txt" 2>&1 &
d=$OUT/trap; mkdir -p "$d"
$LW look=$d run=1:3 hand=push gun=rifle crust=1 crustlook=1 setting=1 set=set_block=5 set=set_hold=0.05 at=$(at 3 1 32) > "$d.txt" 2>&1 &
wait
# fire_fed: dryprobe's joined disc (seed 1), lit, with and without.
./target/release/examples/dryprobe seeds=1 look=$OUT/fed > "$OUT/fed.txt" 2>&1
