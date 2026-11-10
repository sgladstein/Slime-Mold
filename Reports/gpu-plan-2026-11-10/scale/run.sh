#!/usr/bin/env bash
# The scaling-rule census (GPU plan, phase 1's last decision): today's grid
# against a ladder of 4x-finer arms (examples/bigdish.rs presets), on the
# engine-only dish (src/mould/big.rs), 16 seeds, 20 dish seconds, two starts:
#   blob     walkers start packed on the home pile (the game's start)
#   uniform  walkers start spread over the dish (Jones 2010's start)
# Deciding measures, named before the run:
#   pace   (blob):    piles reached at 20 s; reach at 5 s
#   shape  (uniform): loops and mean tube width at 20 s
#   strays (both):    share of walkers off the tube at 20 s
#   cost:             ms a tick, one thread
# Each arm is paired with today on the same seed (same pile layout).
#
#   BIN=path/to/bigdish OUT=raw JOBS=4 SEEDS="1 2 3 4 5 6 7 8" bash run.sh
set -euo pipefail
BIN=${BIN:-./target/release/examples/bigdish}
OUT=${OUT:-raw}
JOBS=${JOBS:-4}
SEEDS=${SEEDS:-1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16}
mkdir -p "$OUT"
jobs=()
for start in blob uniform; do
  b=1; [ "$start" = uniform ] && b=0
  # Slowest arms first so the pool drains evenly.
  for arm in turn steps smell reach naive today; do
    for seed in $SEEDS; do
      jobs+=("$arm $start $b $seed")
    done
  done
done
printf '%s\n' "${jobs[@]}" | xargs -P "$JOBS" -L 1 bash -c '
  arm=$0 start=$1 b=$2 seed=$3
  f="'"$OUT"'/$arm-$start-s$seed.txt"
  [ -s "$f" ] && grep -q hash= "$f" && exit 0
  "'"$BIN"'" preset=$arm tag=$arm-$start blob=$b seeds=$seed seconds=20 report=5 > "$f.tmp" && mv "$f.tmp" "$f"
'
