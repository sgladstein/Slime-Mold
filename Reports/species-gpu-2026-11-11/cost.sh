#!/bin/sh
# What the species settings cost a frame on the GPU (2026-11-11): the
# sandbox's default dish (100,000 walkers, one a cell, food), a frame of two
# ticks, headless, 5 s a run, three rounds alternating. BASE is mouldgpu
# built from main before the port (296ea40), NEW this branch's. Pairs:
#   plain: no outside rule (the plain pipelines), BASE against NEW;
#   two species: the outside rules on, BASE against NEW (the new branches);
#   own settings: two species, NEW, without and with sets equal to the
#     globals (s0.crowd=1 repeats the global limit), so only the lookups differ;
#   mixed: NEW, mix off against mix_share 0.3 with its multipliers;
#   square: NEW, two species converting, convert_r 0 against 8.
# From the repo root. Output on the software GPU: raw/cost.txt (the first
# part, run with the first version of the rows pass), raw/cost-mix.txt (the
# mix's part), raw/cost-square.txt (the square again, after the rows pass
# shared its work across a workgroup).
B=${BASE:?set BASE to mouldgpu built from main before the port}
N=${NEW:-target/release/mouldgpu}
D=${OUT:-/tmp/species-cost}
for r in 1 2 3; do
  echo "== round $r"
  echo "plain base:        $($B look=$D seconds=5 tag=c | grep '^wall ms')"
  echo "plain new:         $($N look=$D seconds=5 tag=c | grep '^wall ms')"
  echo "two species base:  $($B look=$D seconds=5 tag=c species=2 | grep '^wall ms')"
  echo "two species new:   $($N look=$D seconds=5 tag=c species=2 | grep '^wall ms')"
  echo "own settings off:  $($N look=$D seconds=5 tag=c species=2 | grep '^wall ms')"
  echo "own settings on:   $($N look=$D seconds=5 tag=c species=2 s0.crowd=1 | grep '^wall ms')"
  echo "mix off:           $($N look=$D seconds=5 tag=c | grep '^wall ms')"
  echo "mix on:            $($N look=$D seconds=5 tag=c mix_share=0.3 mix_reach=3 mix_angle=0.5 mix_turn=2 mix_step=1.5 mix_deposit=0.5 | grep '^wall ms')"
  echo "square 0:          $($N look=$D seconds=5 tag=c species=2 convert=1.2 | grep '^wall ms')"
  echo "square 8:          $($N look=$D seconds=5 tag=c species=2 convert=1.2 convert_r=8 | grep '^wall ms')"
done
# The mix's own cost, apart from the outside pipelines it switches on and
# the dish it makes: the outside rules forced on with nothing kept
# (blocked_keep with a reset every tick, as compiling_the_outside_rules_out
# does), then the second kind with every multiplier 1, then as above.
F="blocked_keep=1 blocked_reset=1"
for r in 1 2 3; do
  echo "== mix round $r"
  echo "forced, mix off:     $($N look=$D seconds=5 tag=c $F | grep '^wall ms')"
  echo "forced, mix of 1s:   $($N look=$D seconds=5 tag=c $F mix_share=0.3 | grep '^wall ms')"
  echo "forced, mix:         $($N look=$D seconds=5 tag=c $F mix_share=0.3 mix_reach=3 mix_angle=0.5 mix_turn=2 mix_step=1.5 mix_deposit=0.5 | grep '^wall ms')"
done
# The passes of the square's dishes, each tick submitted alone.
echo "== passes"
echo "square 0: $($N look=$D seconds=2 tag=c species=2 convert=1.2 | grep 'a pass')"
echo "square 8: $($N look=$D seconds=2 tag=c species=2 convert=1.2 convert_r=8 | grep 'a pass')"
