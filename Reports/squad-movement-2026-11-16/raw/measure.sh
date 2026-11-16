#!/bin/sh
# The report's numbers: every move in move_tests.rs, this branch's walk
# (after) and the old walk (before, MOVE_STEER=0, same code otherwise).
# Run from the repo root: sh Reports/squad-movement-2026-11-16/raw/measure.sh after|before
set -e
side=${1:-after}
out=Reports/squad-movement-2026-11-16/raw/$side.txt
steer=1; [ "$side" = before ] && steer=0
run() { MOVE_STEER=$steer MOVE=$1 MOVE_NS=$2 MOVE_SEEDS=$3 cargo test --release --locked --lib measure_moves -- --ignored --nocapture 2>&1 | grep " n=" ; }
: > $out
run steer,send,line,wedge,ring,reverse,bridge8,door6long,door8slant 12,40,100,200 1,2 >> $out
run cross 40,100 1,2 >> $out
run planet1,planet8 40,200 1,2,3,4,5,6 >> $out
echo "done $side" >> $out
