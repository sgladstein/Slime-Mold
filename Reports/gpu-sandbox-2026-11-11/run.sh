#!/usr/bin/env bash
# Pictures and frame costs for the GPU sandbox page (2026-11-11). From the
# repo root: cargo build --release --locked --bin mouldgpu, then this.
# Each preset, seed 1, 10 s from a spread start: a still at 10 s, PNG frames
# every 4 frames (15 a second) for a video, the measures, and whether the
# same seed gives the same frame twice. On the cloud this ran on llvmpipe
# (a CPU emulating a GPU), so its costs are not GPU numbers; the GTX 1070's
# costs come from the same command there.
set -u
BIN=${BIN:-./target/release/mouldgpu}
OUT=${OUT:-Reports/gpu-sandbox-2026-11-11/raw}
FRAMES=${FRAMES:-/tmp/mouldgpu-frames}
mkdir -p "$OUT" "$FRAMES"
for p in 1 2 3 4 5 6 7 8; do
  "$BIN" preset=$p look="$FRAMES" tag=p$p seconds=10 frames=1 every=4 twice=1 > "$OUT/p$p.txt" 2>&1
done
