#!/bin/sh
# The explosion films (11-14): run from the repo root after
# `cargo build --release --locked --bin larder-world`, on llvmpipe.
# OUT is a scratch folder; frames go to OUT/<film>/, t lines to raw/.
OUT=${1:-/tmp/explosions}
RAW=Reports/explosions-2026-11-14/raw
mkdir -p "$OUT"
frames() { python3 -c "import sys; a,n=float(sys.argv[1]),int(sys.argv[2]); print(','.join(f'{a+i/15:.4f}' for i in range(n)))" "$@"; }
# Scene 1: each round set off once at (175, 215) in shape C's body on the
# Fork at 20 s (boom=, free; the squad idles at the larder), 2 s at zoom 6
# and 12.
for z in 6 12; do
  for r in blast bomb cluster blowout; do
    ${LW:-./target/release/larder-world} look="$OUT/z$z-$r" hand=idle rounds=$r boom=20,175,215 \
      at="$(frames 19.8 33)" centre=175,215,$z screen=960x600 > "$RAW/z$z-$r.txt" 2>&1 &
  done
  wait
done
# Scene 2: in play. Eight soldiers stand at (95, 185) by the body on free
# fire with 8,000 food, 17-27 s at zoom 4, bomb rounds against blast rounds.
for r in blast bomb cluster; do
  ${LW:-./target/release/larder-world} look="$OUT/play-$r" hand=hold@95,185 bank=8000 rounds=$r \
    at="$(frames 17 150)" centre=110,200,4 screen=960x600 > "$RAW/play-$r.txt" 2>&1 &
done
wait
