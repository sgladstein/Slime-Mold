#!/bin/sh
# The sandbox's dishes on several seeds against the CPU's raw runs, after the
# independent review (2026-11-11): fed scent on the GPU, jumps after the step.
# Software Vulkan (llvmpipe) in the cloud; 10 s each, spread start.
#   crowd1   preset 1, food off   vs ../../jenson-compare-2026-11-10/raw/crowd1-s*.txt
#   crowd0   preset 3             vs .../crowd0-s*.txt
#   jenson   preset 4             vs .../respawn0-s*.txt
#   respawn  preset 5             vs .../respawn0.01-s*.txt
#   fed32k   preset 2 (food on)   vs ../../gpu-plan-2026-11-10/scale/raw/nose-uniform-s*.txt (16 seeds)
set -e
BIN=${BIN:-target/release/mouldgpu}
OUT=${OUT:-Reports/gpu-sandbox-2026-11-11/review/raw}
PICS=${PICS:-/tmp/sandbox-review}
mkdir -p "$OUT" "$PICS"
run() { name=$1; seed=$2; shift 2; "$BIN" "$@" seed=$seed look="$PICS" seconds=10 tag="$name-s$seed" > "$OUT/$name-s$seed.txt" 2>&1; }
for s in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16; do run fed32k $s preset=2; done
for s in 1 2 3 4 5 6 7 8; do run crowd1 $s preset=1 food=0; done
for s in 1 2 3 4 5 6 7 8; do run crowd0 $s preset=3; done
for s in 1 2 3 4 5 6 7 8; do run respawn $s preset=5; done
for s in 1 2 3 4 5 6 7 8; do run jenson $s preset=4; done
