#!/bin/sh
# Growth on the big dish (Larder World W0, 2026-11-11): the census behind
# Reports/growth-2026-11-11.md. From the repo root, after
# cargo build --release --locked --example bigdish. Four runs at a time,
# one thread each; raw output in raw/.
#
# Named before the runs. Deciding measure: living walkers at 30 s, the
# grass regrowing (graze_regrow 0.002) against none (0), seeds 1-8; a fed
# mould should end above its 10,000-walker start and a starved one below
# it, fed over starved on every seed (sign test). Second: living walkers at
# 60 s against regrowth 0.0005, 0.001, 0.002, on the dish at half the
# scale (512 x 320, the nose rule's scent offset 18, 2,500 walkers to start,
# 50,000 slots so they do not cap
# it, a quarter of the cost), each step up paired by seed. Holm over the
# three. A run whose output already ends at its last second is skipped.
set -e
B=${B:-target/release/examples/bigdish}
D=Reports/growth-2026-11-11/raw
G="preset=nose walkers=10000 upkeep=0.01 graze_weight=10 threads=1 report=5"
mkdir -p "$D"
{
  for s in 1 2 3 4 5 6 7 8; do
    echo "$G capacity=100000 seconds=30 graze_regrow=0.002 seeds=$s tag=fed-s$s"
    echo "$G capacity=100000 seconds=30 graze_regrow=0 seeds=$s tag=starved-s$s"
  done
  for s in 1 2 3 4 5 6 7 8; do
    for r in 0.0005 0.001 0.002; do
      echo "$G scale=2 scent_offset=18 walkers=2500 capacity=50000 seconds=60 graze_regrow=$r seeds=$s tag=regrow$r-s$s"
    done
  done
} | xargs -P 4 -I{} sh -c 't=$(echo "{}" | sed "s/.*tag=//"); s=$(echo "{}" | sed "s/.*seconds=\([0-9]*\).*/\1/"); grep -q "t  $s.0s  tube" "'"$D"'/$t.txt" 2>/dev/null || "'"$B"'" {} > "'"$D"'/$t.txt" 2>&1'
