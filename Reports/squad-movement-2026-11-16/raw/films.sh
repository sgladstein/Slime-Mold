#!/bin/sh
# Before/after films for the squad movement report, on software Vulkan
# (llvmpipe; apt install mesa-vulkan-drivers). Run from the repo root.
# BEFORE is main with film.rs copied in (MAIN=worktree of origin/main whose
# film.rs drops the `Rules::steer` and `Soldier::was` lines; see the
# report); AFTER is this branch. Frames to OUT/<name>/fNNNN.png, then one
# side-by-side H.264 per clip: main left, this change right.
set -e
OUT=${OUT:-/tmp/film}
MAIN=${MAIN:-/tmp/main-wt}
film() { # dir name, then env
  name=$1; shift
  for side in before after; do
    tree=.; [ $side = before ] && tree=$MAIN
    (cd $tree && env FILM=$OUT/$name-$side "$@" cargo test --release --locked --bin larder-world film_moves -- --ignored --nocapture) > $OUT/$name-$side.log 2>&1
    python3 Reports/squad-movement-2026-11-16/raw/judder.py $OUT/$name-$side
  done
  ffmpeg -y -loglevel error -framerate 60 -i $OUT/$name-before/f%04d.png -framerate 60 -i $OUT/$name-after/f%04d.png \
    -filter_complex "[0:v][1:v]hstack=inputs=2" -c:v libx264 -pix_fmt yuv420p -crf 30 -preset slow -movflags +faststart $OUT/$name.mp4
}
mkdir -p $OUT
# WASD with 40 soldiers on planet 1: east 3 s, north 2.5 s, west 2.5 s, let go.
film wasd40 FILM_MAP=planet:1:1 FILM_N=40 FILM_DO=steer FILM_SECS=11 FILM_ZOOM=8
# A right click on the furthest crystal within 120 cells, planet 1, 100 soldiers.
film far100 FILM_MAP=planet:1:1 FILM_N=100 FILM_DO=far FILM_SECS=20 FILM_ZOOM=5
# The same on planet 8 with 200 soldiers.
film far200 FILM_MAP=planet:8:1 FILM_N=200 FILM_DO=far FILM_SECS=24 FILM_ZOOM=4
