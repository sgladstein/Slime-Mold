#!/bin/sh
# Growth off changes nothing (2026-11-11): three dishes, 3 s, seed 1, with
# this branch's bigdish against main's before growth (MAIN=path to that
# build). Prints each dish's measure lines without the timing and the
# parameter dump (which lists the new fields), then whether
# they match. From the repo root.
B=${B:-target/release/examples/bigdish}
M=${MAIN:?set MAIN to the bigdish built from main}
for d in "preset=nose" \
         "preset=nose graze_weight=10 graze_regrow=0.002" \
         "preset=dense species=2 respawn=0.01 explore_steps=60 explore_spread=1"; do
  a=$($B $d seconds=3 report=1 threads=2 tag=x | grep -v "^params" | sed 's/ms\/tick.*//')
  b=$($M $d seconds=3 report=1 threads=2 tag=x | grep -v "^params" | sed 's/ms\/tick.*//')
  echo "== $d"
  echo "$a"
  [ "$a" = "$b" ] && echo "same as main" || { echo "DIFFERS from main:"; echo "$b"; }
done
