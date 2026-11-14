#!/bin/sh
# The fire films (11-14), from the repo root on llvmpipe, after
# `cargo build --release --locked --bin larder-world --example plantprobe`.
# The map is planet 2 seed 1 with the probe's stand-in plants, full grown
# (levelgen writes the real ones per biome).
OUT=${1:-/tmp/plants-films}
mkdir -p "$OUT"
./target/release/examples/plantprobe seed=1 depth=2 save=Reports/plants-2026-11-14/raw/maps
./target/release/larder-world map=Reports/plants-2026-11-14/raw/maps/plants-2-1 seed=1 look="$OUT" hand=idle rounds=bomb \
  boom=3,300,190 boom=3,240,110 centre=290,180,3 at="$(seq -s, 1 45)" screen=960x600 > Reports/plants-2026-11-14/raw/film5.log 2>&1
python3 Reports/plants-2026-11-14/raw/make_gifs.py "$OUT" Reports/plants-2026-11-14/page
