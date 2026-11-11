#!/bin/sh
# What growth's bookkeeping costs a tick on the CPU (2026-11-11): 100,000
# walkers on the nose dish with grass, growth off against growth on with
# nothing born or starving (upkeep 1e-9, births at 1e9), so the two dishes
# stay alike and only the checks, the eating shares and the life step
# differ. 10 s each, one thread, three rounds alternating, alone on the
# machine. From the repo root after building the bigdish example.
B=${B:-target/release/examples/bigdish}
G="preset=nose walkers=100000 graze_weight=10 seconds=10 report=10 threads=1"
for r in 1 2 3; do
  echo "round $r off: $($B $G tag=off | grep 'ms/tick')"
  echo "round $r on: $($B $G upkeep=0.000000001 birth_energy=1000000000 start_energy=1 tag=on | grep 'ms/tick')"
done
