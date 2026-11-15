#!/bin/sh
# Rivers under reeds (11-15), from the repo root on llvmpipe, after
# `cargo build --release --locked --bin larder-world`, once on main before
# the change (BEFORE) and once after (AFTER). Planet 4 seed 1 (river
# valley), squad idle, the river at 305,150; zoom 12 and 4, at 2 and 120 s.
OUT=${1:-/tmp/river-films}
W=./target/release/larder-world
mkdir -p "$OUT"
$W map=planet:4:1 seed=1 look="$OUT/near" hand=idle centre=305,150,12 at=2,120 screen=960x600
$W map=planet:4:1 seed=1 look="$OUT/mid" hand=idle centre=305,150,4 at=2,120 screen=960x600
$W map=planet:4:1 seed=1 look="$OUT/p4" hand=idle at=120 screen=960x600
# Then: python3 Reports/river-reeds-2026-11-15/raw/make_page.py BEFORE AFTER Reports/river-reeds-2026-11-15/page
