#!/bin/sh
# Fire round 2 films (11-14), from the repo root on llvmpipe, after
# `cargo build --release --locked --bin larder-world`. Planet 2 seed 1
# (Barrens, levelgen's own plants); bombs at 3 s into the jungle and
# meadow, at 20 s near the squad.
OUT=${1:-/tmp/fire-films}
mkdir -p "$OUT"
./target/release/larder-world map=planet:2:1 seed=1 look="$OUT" hand=idle rounds=bomb \
  boom=3,60,240 boom=3,200,190 boom=20,90,60 centre=60,230,12 at="$(seq -s, 1 60)" screen=960x600 > Reports/fire-2026-11-14/raw/film.log 2>&1
python3 Reports/fire-2026-11-14/raw/make_gifs.py "$OUT" Reports/fire-2026-11-14/page
