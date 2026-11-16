#!/bin/sh
# Frames for the throw-aim clips (2026-11-16), on llvmpipe in the cloud:
# the Fork, seed 1, the squad mode's camera held at zoom 12 on the squad;
# the cursor sweeps from the first point to the second over 31 frames,
# 6.0-9.0 s. Clips: ffmpeg -framerate 10, forward then back, H.264 crf 26.
set -e
OUT=${1:-throw-frames}
AT=$(python3 -c "print(','.join(f'{6+i*0.1:.1f}' for i in range(31)))")
cargo build --release --bin larder-world
./target/release/larder-world look=$OUT/blast hand=throw at=$AT hover=760,300,1250,90 centre=256,112.5,12
./target/release/larder-world look=$OUT/gas hand=throwgas at=$AT hover=1100,560,700,140 centre=256,112.5,12
