#!/usr/bin/env bash
# Who reaches food first: mixed droplet moulds against plain ones (2026-11-11).
# The redesign of the first crystal-mix test after the independent review
# (Reports/research-review-2026-11-11.md, section 5: the start disc covered
# 3-6 of the 12 crystals, so charge eaten measured where the start landed).
# From the repo root, after cargo build --release --example bigdish:
#   bash Reports/first-reach-2026-11-11/run.sh [JOBS] [OUT]
# 30,000 droplet walkers start in a disc of 50 dish cells at the dish
# centre; 12 food piles (pile 0 counts as reached at the start and is not
# scored). The `firsts` line gives each pile's distance from the start and
# when tube first reached it.
# Deciding measure, named before the run: the number of scored piles (pile 1
# and up, centre more than 66 dish cells from the start: the disc, a pile's
# radius and a 10-cell margin) reached by 30 s; each arm minus its base on
# paired seeds 1-16, sign test with ties dropped, Holm over the 5
# comparisons: grazers - droplets, grazescouts - grazers, dropscouts -
# droplets, dropfast - droplets, scoutsonly - droplets (every walker the
# second kind: the review's control for "the kinds sort themselves").
# Plain piles, not crystals: in the first 30 s little charge is eaten, and
# this measure is about spread. The 1M-walker network arms are left out
# (about 25 min a run on one core).
set -u
JOBS=${1:-4}; OUT=${2:-/tmp/first-reach}; mkdir -p "$OUT"
BIN=${BIN:-./target/release/examples/bigdish}
SEEDS=${SEEDS:-"$(seq -s ' ' 1 16)"}; SECS=${SECS:-60}
D="preset=dense piles=12 crowd=1 walkers=30000 deposit=5 rotate_angle=45 sensor_angle=90 sensor_offset=9 keep=0.93 blob=1 blob_r=50 blocked_deposit=1"
LIST="droplets|$D
grazers|$D graze_weight=10
grazescouts|$D graze_weight=10 mix_share=0.2 mix_reach=3 mix_deposit=0.3
dropscouts|$D mix_share=0.2 mix_reach=3 mix_deposit=0.3
dropfast|$D mix_share=0.2 mix_step=2 mix_deposit=0.3
scoutsonly|$D mix_share=1 mix_reach=3 mix_deposit=0.3"
n=0
for s in $SEEDS; do
  while IFS='|' read -r name args; do
    f="$OUT/$name-s$s.txt"
    [ -s "$f" ] && grep -q "firsts" "$f" && continue   # done before a restart
    ( $BIN $args seeds=$s tag=$name report=10 threads=1 seconds=$SECS 2>&1 | grep -E "^seed" ) > "$f" &
    n=$((n+1)); (( n % JOBS == 0 )) && wait
  done <<< "$LIST"
done
wait
grep -h firsts "$OUT"/*.txt
