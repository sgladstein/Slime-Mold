#!/bin/sh
# Plants look films (11-14), from the repo root on llvmpipe, after
# `cargo build --release --locked --bin larder-world`. Planet 2 seed 1
# (Barrens, the level generator's own plants), the squad idle.
#   meadow-12: a bomb into the meadow at 3 s, zoom 12, 1-45 s
#   jungle-6:  a bomb into the jungle at 3 s, zoom 6, 1-40 s
#   stills at zoom 24 (jungle edge at 2 s, the meadow fire at 20 s)
#   thumbs: every planet (1-8, seed 1) whole, at 1 s, for grey thumbnails
OUT=${1:-/tmp/plants-films}
LOG=Reports/plants-look-2026-11-14/raw/film.log
W=./target/release/larder-world
mkdir -p "$OUT"
: > "$LOG"
$W map=planet:2:1 seed=1 look="$OUT/meadow-12" hand=idle rounds=bomb boom=3,60,240 centre=60,230,12 at="$(seq -s, 1 45)" screen=960x600 >> "$LOG" 2>&1
$W map=planet:2:1 seed=1 look="$OUT/jungle-6" hand=idle rounds=bomb boom=3,180,190 centre=200,190,6 at="$(seq -s, 1 40)" screen=960x600 >> "$LOG" 2>&1
$W map=planet:2:1 seed=1 look="$OUT/z24-jungle" hand=idle centre=180,170,24 at=2 screen=960x600 >> "$LOG" 2>&1
$W map=planet:2:1 seed=1 look="$OUT/z24-fire" hand=idle rounds=bomb boom=3,60,240 centre=62,228,24 at=20 screen=960x600 >> "$LOG" 2>&1
for p in 1 2 3 4 5 6 7 8; do
  $W map=planet:$p:1 seed=1 look="$OUT/thumb-$p" hand=idle at=1 screen=960x600 >> "$LOG" 2>&1
done
python3 Reports/plants-look-2026-11-14/raw/make_page.py "$OUT" Reports/plants-look-2026-11-14/page
