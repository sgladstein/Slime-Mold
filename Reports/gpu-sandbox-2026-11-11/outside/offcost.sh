#!/bin/sh
# The outside rules' cost when off (2026-11-11): main's sandbox (the looks
# build) against this one on keys 1 and 4, seed 1, 5 s, three rounds with
# the builds alternating. MAIN and NEW are the two binaries (main's built
# from `git show <main>:src/bin/mouldgpu.rs` against the same library).
# Prints each run's GPU ms a frame line and pass profile.
set -e
MAIN=${MAIN:?the main build}; NEW=${NEW:?this build}
D=Reports/gpu-sandbox-2026-11-11/outside/raw
F=${F:-/tmp/mouldgpu-offcost}; mkdir -p "$F" "$D"
for round in 1 2 3; do
  for p in 1 4; do
    for b in main new; do
      bin=$MAIN; [ $b = new ] && bin=$NEW
      "$bin" preset=$p look="$F" tag=p$p-$b-r$round seconds=5 > "$F/run.txt" 2>&1
      echo "round $round key $p $b: $(grep -E '^wall' "$F/run.txt")"
      echo "round $round key $p $b: $(grep -E '^GPU ms a pass' "$F/run.txt")"
    done
  done
done
