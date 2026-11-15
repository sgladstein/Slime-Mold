#!/bin/sh
# Planet review fixes (11-15), from the repo root on llvmpipe, after
# `cargo build --release --locked --bin larder-world`. Planet 1 seed 1
# (meadow), squad idle.
#   fire-12: a bomb into the meadow at 3 s, zoom 12, 1-30 s
#   nofire:  the same view at 20 s without the bomb (measure.py's base)
#   edge-12: the slime's edge over plants at 20 s, zoom 12
#   slime:   planet 2's thick slime body at 24 s, zoom 12 (the highlight
#            measure.py compares the fire with)
# Then measure.py prints the luma and hue percentiles, and make_page.py
# builds the page's GIF and sheets.
OUT=${1:-/tmp/review-films}
LOG=Reports/review-fixes-2026-11-15/raw/film.log
W=./target/release/larder-world
mkdir -p "$OUT"
: > "$LOG"
$W map=planet:1:1 seed=1 look="$OUT/fire-12" hand=idle rounds=bomb boom=3,230,110 centre=230,110,12 at="$(seq -s, 1 30)" screen=960x600 >> "$LOG" 2>&1
$W map=planet:1:1 seed=1 look="$OUT/nofire" hand=idle centre=230,110,12 at=20 screen=960x600 >> "$LOG" 2>&1
$W map=planet:1:1 seed=1 look="$OUT/edge-12" hand=idle centre=177,140,12 at=20 screen=960x600 >> "$LOG" 2>&1
$W map=planet:2:1 seed=1 look="$OUT/slime" hand=idle centre=380,150,12 at=24 screen=960x600 >> "$LOG" 2>&1
# Each planet's flora at zoom 12, 2 s (seed 1): a window of its most
# typical plant kind away from the nests, picked from the planets
# example's maps by raw/pick.py.
for pc in "1 350 115" "2 60 25" "3 225 160" "4 565 250" "5 105 220" "6 655 140" "7 905 170" "8 875 365"; do
  set -- $pc
  $W map=planet:$1:1 seed=1 look="$OUT/flora-$1" hand=idle centre=$2,$3,12 at=2 screen=960x600 >> "$LOG" 2>&1
done
python3 Reports/review-fixes-2026-11-15/raw/make_page.py "$OUT" Reports/review-fixes-2026-11-15/page
python3 Reports/review-fixes-2026-11-15/raw/measure.py \
  "$OUT"/fire-12/*-020s-near.png "$OUT"/nofire/*near.png "$OUT"/slime/*near.png
